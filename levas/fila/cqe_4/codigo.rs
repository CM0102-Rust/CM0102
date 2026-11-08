// ---------- %%LEVA%%a leva: a vontade de uma pessoa de ir para um clube (ou de ficar) e o construtor do contrato (shortlist_manager.cpp, staff_contracts.cpp) ----------

unsafe fn cqe4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqe4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqe4_c(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqe4_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqe4_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// Uma constante f64 do exe.
unsafe fn cqe4_k(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// Uma constante f32 do exe.
unsafe fn cqe4_kf(a: usize) -> f32 {
    core::ptr::read_unaligned(a as *const f32)
}

/// A conta dos dias entre o ano e o dia de hoje ([0xae2c92], [0xae2c90]) e os de uma data (ano em `ano`, dia em
/// `dia`), como a original: (ano − ano de hoje) × 365 − dia de hoje + dia.
unsafe fn days_until_date(ano: usize, dia: usize) -> i32 {
    ((cqe4_w(ano) as i32 - cqe4_w(0xae2c92) as i32) * 365 - cqe4_h(0xae2c90) as i32) + cqe4_h(dia) as i32
}

/// Hoje menos 150 dias (0x54c8e0: push 0x96; push saída; mov ecx, 0xae2c90 -> thiscall, ret 8) já passou da data
/// guardada em +0x3e (dia) / +0x40 (ano) do técnico `t`: ano maior (sem sinal), ou o mesmo ano e o dia maior (com
/// sinal).
unsafe fn is_150_days_since(t: usize) -> bool {
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) = core::mem::transmute(0x54c8e0usize);
    let mut d = [0u8; 8];
    menos(0xae2c90, &mut d, 0x96);
    let dia = i16::from_le_bytes([d[0], d[1]]);
    let ano = u16::from_le_bytes([d[2], d[3]]);
    let ano_t = cqe4_w(t + 0x40);
    ano > ano_t || (ano == ano_t && dia > cqe4_h(t + 0x3e))
}

/// A distância entre as cidades `a` e `b` (coordenadas f64 em +0x23 e +0x2b), × 55.
unsafe fn shortlist_manager_city_distance(a: usize, b: usize) -> f64 {
    let dx = cqe4_k(a + 0x2b) - cqe4_k(b + 0x2b);
    let dy = cqe4_k(a + 0x23) - cqe4_k(b + 0x23);
    (dy * dy + dx * dx).sqrt() * cqe4_k(0x96d1b8)
}

/// O máximo da original nas contas de reputação: `x` se ele passa `y`, senão `y`.
fn max_f64(x: f64, y: f64) -> f64 {
    if x > y { x } else { y }
}

/// A média das três reputações da ficha do jogador (+9, +0xb, +0xd), truncada.
unsafe fn average_reputation3(f: usize) -> i32 {
    (cqe4_h(f + 0xd) as i32 + cqe4_h(f + 0xb) as i32 + cqe4_h(f + 9) as i32) / 3
}

/// A reputação do clube pelo estádio: sem estádio ou sem cidade (+0x69, +0x38), trunc(rep × 2/7); com a cidade de
/// importância 0 (+0x33), rep / 2; senão o maior entre rep × 2/7 e rep × importância / 15, truncado.
unsafe fn reputation_by_stadium(clube: usize) -> i32 {
    let rep = cqe4_h(clube + 0x80) as i32;
    let e = cqe4_d(clube + 0x69);
    let cidade = if e != 0 { cqe4_d(e + 0x38) } else { 0 };
    if cidade == 0 {
        return match_pl_ftol(rep as f64 * cqe4_k(0x96d5e0));
    }
    let imp = cqe4_c(cidade + 0x33);
    if imp == 0 {
        return rep / 2;
    }
    let q = (imp as i32).wrapping_mul(rep) / 15;
    let x = rep as f64 * cqe4_k(0x96d5e0);
    match_pl_ftol(max_f64(x, q as f64))
}

/// A reputação do clube pela divisão (+0x57, o +0x69 dela): sem divisão, a própria (o resto do registrador vem do
/// ponteiro; só a palavra conta); com reputação de 7500 e público acima de 35000 (+0x73), o maior entre a reputação e
/// 500 × a da divisão; senão o maior entre rep/2 + 250 × a da divisão e 500 × a da divisão.
unsafe fn reputation_by_division(clube: usize) -> i16 {
    let rep = cqe4_h(clube + 0x80) as i32;
    let comp = cqe4_d(clube + 0x57);
    if comp == 0 {
        return rep as i16;
    }
    let cv = cqe4_h(comp + 0x69) as i32;
    let t = cv.wrapping_mul(500);
    if rep >= 0x1d4c && cqe4_d(clube + 0x73) as i32 > 0x88b8 {
        (if t > rep { t } else { rep }) as i16
    } else {
        let h = rep / 2 + cv.wrapping_mul(250);
        (if t > h { t } else { h }) as i16
    }
}

/// A reputação pela competição do clube (+0x1db; a da lista [0xae2d14] + id × 6 quando ela é negativa ou é a
/// [0x9cf6f8] com reputação de 5000): o registro de [0xae23d0] (0x6b bytes; 15000 para a [0x9cf6e8], senão 600 × +0x69
/// em 16 bits) e, com o clube na tabela dela (0x4a4850) em 1º a 16º sem a marca +4, (1,32 − 0,02 × posição) × isso.
/// Sem registro: o mínimo entre 0,8 × `pela_divisao` e 7000. Devolve (valor, registro).
unsafe fn reputation_by_competition(clube: usize, pela_divisao: i16) -> (i32, usize) {
    let rep = cqe4_h(clube + 0x80);
    let mut idx = cqe4_d(clube + 0x1db) as i32;
    let mut tem = true;
    if idx <= -1 || (idx == cqe4_d(0x9cf6f8) as i32 && rep >= 0x1388) {
        idx = cqe4_d(cqe4_d(0xae2d14).wrapping_add(cqe4_d(clube).wrapping_mul(6))) as i32;
        if idx <= -1 {
            tem = false;
        }
    }
    let mut reg = 0usize;
    if tem {
        reg = cqe4_d(0xae23d0).wrapping_add((idx as usize).wrapping_mul(0x6b));
        if reg != 0 {
            let id = cqe4_d(reg);
            let s: i16 = if id == cqe4_d(0x9cf6e8) { 15000 } else { cqe4_h(reg + 0x69).wrapping_mul(600) };
            // 0x4a4850: push clube; mov ecx, [[0xadadfc] + 4 × id] + 8 -> thiscall, ret 4
            let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
            let r = achar(cqe4_d(cqe4_d(cqe4_d(0xadadfc).wrapping_add(id.wrapping_mul(4))) + 8), clube);
            if r != 0 && cqe4_b(r + 4) == 0 && (1..=0x10).contains(&cqe4_b(r + 0x72)) {
                let p = cqe4_b(r + 0x72) as f64;
                return (match_pl_ftol((cqe4_k(0x96af38) - p * cqe4_k(0x968a98)) * s as f64), reg);
            }
            return (s as i32, reg);
        }
    }
    let mut x = pela_divisao as f64 * cqe4_k(0x9689b0);
    if !(x < cqe4_k(0x96d470)) {
        x = cqe4_k(0x96d470);
    }
    (match_pl_ftol(x), reg)
}

/// A reputação vista pelo técnico ([0xb67a44] + id × 0x49: (+0xa + +8) × 0,25 + +6 × 0,5): abaixo de 5000, o maior
/// entre r / (5 − 0,0008 v) e r − 1000 × (5 − 0,0008 v); senão o menor entre r × (0,5 + 0,0001 v) e r + 1000 × isso.
unsafe fn reputation_by_manager(clube: usize, r: i16) -> i32 {
    let t = cqe4_d(cqe4_d(0xb67a44)).wrapping_add(cqe4_d(clube).wrapping_mul(0x49));
    let v = match_pl_ftol((cqe4_h(t + 0xa) as f64 + cqe4_h(t + 8) as f64) * cqe4_k(0x9689e0) + cqe4_h(t + 6) as f64 * cqe4_k(0x968968)) as i16;
    let dd = r as f64;
    if v < 0x1388 {
        let t = cqe4_k(0x9688a0) - v as f64 * cqe4_k(0x96a618);
        let q1 = dd / t;
        let m = dd - t * cqe4_k(0x968908);
        match_pl_ftol(if q1 > m { q1 } else { m })
    } else {
        let t = v as f64 * cqe4_k(0x96a5c0) + cqe4_k(0x968968);
        let q1 = t * dd;
        let m = t * cqe4_k(0x968908) + dd;
        match_pl_ftol(if q1 < m { q1 } else { m })
    }
}

/// O teto da reputação pelas finanças (0x5a0590 e 0x5986b0): faixa 1, o maior entre 1,25 r e r + 750; faixa 2, o maior
/// entre 1,5 r e r + 1500.
fn reputation_cap_by_finances(band: i8, r: i16) -> i16 {
    match band {
        1 => match_pl_ftol(max_f64(r as f64 * 1.25, (r as i32 + 0x2ee) as f64)) as i16,
        2 => match_pl_ftol(max_f64(r as f64 * 1.5, (r as i32 + 0x5dc) as f64)) as i16,
        _ => r,
    }
}

/// min(h / 2, h − 2000) sobre a palavra, a correção pelo +0x64 do clube (diferente de 1; e mais uma vez com 3).
fn half_or_minus_2000(h: i16) -> i16 {
    let a = h as i32 / 2;
    let e = h as i32 - 2000;
    (if a < e { a } else { e }) as i16
}

/// A fluência: max(F × 0,1, L / (12000 − 200 F)) + 0,2, com L = (s + 5000)² × 1/15000 e F a fluência da pessoa no
/// idioma da nação do clube (0x7c6830: push 1; push idioma; push clube; push pessoa; mov ecx, [0xb74350] -> thiscall,
/// ret 0x10; o idioma vai num registrador com lixo acima, a cópia passa só o byte). Devolve (fator, F).
unsafe fn language_fluency_factor(p1: usize, clube: usize, s: i16) -> (f64, f64) {
    let q = (s as i32 + 0x1388) as f64;
    let l = q * cqe4_k(0x968d78) * q;
    let fluencia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8 = core::mem::transmute(0x7c6830usize);
    let f = fluencia(cqe4_d(0xb74350), p1, clube, cqe4_b(cqe4_d(clube + 0x53) + 0x77) as u32, 1) as i8 as i32 as f64;
    let a = f * cqe4_k(0x967880);
    let b = l / (cqe4_k(0x96f938) - f * cqe4_k(0x96a648));
    (max_f64(a, b) + cqe4_k(0x968918), f)
}

/// { 0x846920 } (thiscall, ret 0x18, shortlist_manager.cpp): `Shortlist_Manager::staff_interest_in_joining_club(pessoa,
/// clube, salário, empréstimo, situação, motivos)`: a nota de vontade da pessoa de ir para o clube (ou de ficar, quando
/// o clube é o dela), x87 transcrito em f64 na ordem da original. Os motivos (bits) só são marcados com o ponteiro. A
/// nota parte de 2,75 (3,5 para o próprio clube; com contrato acabando, 1,1 − dias × 0,002; sem clube, a idade fora do
/// clube), e passa pelos fatores: reputação dos clubes no empréstimo, gostar ou não do clube e do técnico, rivais, as
/// nações e a distância, a felicidade no contrato (+0x44), a situação no elenco (+0x4f) e a pedida, a reputação comparada (a do clube
/// novo e a do atual, pela divisão, pela competição, pelo técnico e pelas finanças), a fluência no idioma, os jogos
/// de seleção e o tempo sem clube; e, pedido o salário (salário ≠ 0, sem empréstimo e nota de 1,75), o salário padrão
/// (0x864550 num STAFF_CONTRACT local) contra o teto do clube (0x5968d0). O resto das contas está nos comentários.
unsafe extern "thiscall" fn staff_interest_in_joining_club(this: usize, p1: usize, p2: usize, p3: u32, p4: u32, p5: u32, p6: *mut u32) -> f64 {
    use crate::comparison::branch;
    let k = |a: usize| cqe4_k(a);
    let zero = k(0x9689a0);
    // as chamadas por endereço, com as convenções conferidas nos pontos de chamada da original
    // 0x54a110: lea ecx, data -> thiscall, ret 0
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    // 0x8cca10: push 0; push pessoa; mov ecx, [0xde1f54] -> thiscall, ret 8
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
    // 0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    // 0x4cf5f0: push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let contrato_com: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    // 0x54cdb0: push [0xae2c94]; push [0xae2c90]; lea ecx, data -> thiscall, ret 8
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    // 0x53d980: push 1; push nação; push pessoa; add esp, 0xc -> cdecl
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
    // 0x544590 / 0x544520 / 0x5444a0: push clube; push pessoa; add esp, 8 -> cdecl
    let na_lista_emp: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
    let na_lista_transf: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let in_list: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    // 0x547d30 / 0x547d60: push clube; push pessoa; add esp, 8 -> cdecl
    let dislikes_club: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let likes_club: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    // 0x547810 / 0x547840: push pessoa; push técnico; add esp, 8 -> cdecl
    let likes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let dislikes: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    // 0x4b02e0: push atual; push clube; add esp, 8 -> cdecl
    let rival: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b02e0usize);
    // 0x4ca470: push contrato; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let categoria: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4ca470usize);
    // 0x600a50: push 0; push 0; push 1 (ou 1); push 0 (ou 1); push clube; add esp, 0x14 -> cdecl
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    // 0x5407e0: push 1; push pessoa; push técnico; add esp, 0xc -> cdecl (só a palavra conta)
    let vista: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(0x5407e0usize);
    // 0x5a0590: push clube; mov ecx, [0xae2a38] -> thiscall, ret 4; 0x5986b0: mov ecx, finanças -> thiscall, ret 0
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    // 0x4cbb60: push clube; push pessoa; mov ecx, [0xae1050] -> thiscall, ret 8
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4cbb60usize);
    // 0x669bd0: push &data b; push &data a; push clube; add esp, 0xc -> cdecl
    let na_temporada: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x669bd0usize);

    let mut data_a = [0u8; 8];
    let mut data_b = [0u8; 8];
    data_padrao(data_a.as_mut_ptr() as usize);
    data_padrao(data_b.as_mut_ptr() as usize);
    let p4b = p4 as u8 != 0;
    let p5b = p5 as u8 as i8;
    let mut l80 = 1.0f64; // [base+0x84]
    let mut l8c = 0usize; // [base+0x78]: o registro da competição do clube atual
    let mut l84 = 0usize; // [base+0x80]: o registro da competição do clube novo
    let mut ad: i8 = 0; // [base+0x57]
    let fl = move |b: u32| {
        if !p6.is_null() {
            *p6 |= b;
        }
    };
    if !p6.is_null() {
        *p6 = 0;
    }
    if cqe4_d(p1 + 0x61) == 0 || cqe4_d(p2 + 0x53) == 0 || cqe4_d(p1 + 0x1a) == 0 {
        return zero;
    }
    if futura(cqe4_d(0xde1f54), p1, 0) != 0 {
        return zero;
    }
    if cqe4_b(cqe4_d(0xae2d0c).wrapping_add(cqe4_d(p1).wrapping_mul(0x4f)) + 0xb) & 0x30 != 0 {
        return zero;
    }
    let a57 = cqe4_c(p1 + 0x57) as i32 as f64;
    let idade = cqe4_c(p1 + 0x18);
    let aa8: f32 = ((k(0x968dc0) - a57 * k(0x967878)) - cqe4_c(p1 + 0x5b) as i32 as f64 * k(0x967878)) as f32;
    let mut a88: f32 = (a57 * k(0x968a98) + k(0x9689b0)) as f32;
    let mut b4: f32 = (a57 * k(0x967888) + k(0x968968)) as f32;
    let c0 = contract_of(cqe4_d(0xae1050), p1);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c0);
    let mut d4 = 0usize;
    if c0 != 0 {
        d4 = cqe4_d(0xae23b4).wrapping_add(cqe4_d(c0 + 4).wrapping_mul(0x245));
        if d4 != 0 {
            if cqe4_d(d4 + 0x53) == 0 {
                return k(0x968e38);
            }
            if cqe4_b(contract_of(cqe4_d(0xae1050), p1) + 0x35) & 0x3f == 5 {
                d4 = 0;
            }
        }
    }
    let c = contrato_com(cqe4_d(0xae1050), p1, d4);
    let a0: u32 = if c != 0 { cqe4_d(c + 0x45) as u32 } else { 0 };
    let r61 = cqe4_d(p1 + 0x61);
    // o próprio clube com contrato: fica (3,5) quando o contrato não acaba logo e a reputação dele não passa a do clube
    if p2 != 0 && p2 == d4 && c != 0 {
        let dias = dias_ate(c + 0x2d, cqe4_d(0xae2c90) as u32, cqe4_d(0xae2c94) as u32);
        let r1 = cqe4_h(r61 + 0xb) as i32;
        let mut s = match_pl_ftol((cqe4_h(p2 + 0x80) as i32 - r1) as f64 * k(0x968968)) as i16;
        if a0 & 0x10 != 0 {
            s = match_pl_ftol(s as f64 * k(0x968968)) as i16;
        }
        let lim = cqe4_c(p1 + 0x59) as i32 * 10 + 0x465;
        let s: i32 = if s < 0 { 0 } else if s as i32 > lim { lim as i16 as i32 } else { s as i32 };
        if a0 & 0x161a8 == 0 {
            let id = cqe4_d(p1) as i32;
            if dias < id % 0x23 + 0x14
                && cqe4_b(cqe4_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(0x4f)) + 0xb) & 0xf == 0
                && r1 + cqe4_c(p1 + 0x57) as i32 * 20 < (cqe4_c(p1 + 0x59) as i32 + 0xaa) * 25 + s
            {
                return k(0x968e38);
            }
        }
    }
    let mut f8: f64;
    if p2 == d4 {
        if c != 0 && a0 != 0 && a0 != 2 {
            return zero;
        }
        f8 = 3.5;
    } else {
        if c != 0 && d4 != 0 && cqe4_d(d4 + 0x53) != 0 && cqe4_d(p2 + 0x53) != 0
            && cqe4_b(cqe4_d(p2 + 0x53) + 0x77) == cqe4_b(cqe4_d(d4 + 0x53) + 0x77) && a0 & 0x20 != 0
        {
            return zero;
        }
        if idade < 0x10 && estrangeira(p1, cqe4_d(p2 + 0x53), 1) != 0 {
            return zero;
        }
        ramo("cqe_4 base de outro clube");
        f8 = 2.75; // cqe4: a base para outro clube
        if c != 0 && cqe4_b(c + 0x49) != 0 {
            let dd = days_until_date(c + 0x2f, c + 0x2d);
            let v = k(0x9689c0) - dd as f64 * k(0x969048);
            if !p6.is_null() && v < cqe4_c(p1 + 0x5a) as i32 as f64 * k(0x968a98) + k(0x968e48) {
                *p6 |= 1;
            }
            f8 = v * k(0x9690a8);
        } else if d4 == 0 {
            let dd =((cqe4_w(0xae2c92) as i32 - cqe4_w(p1 + 0x40) as i32) * 365 - cqe4_h(p1 + 0x3e) as i32) + cqe4_h(0xae2c90) as i32;
            let mut x = dd.wrapping_mul(dd) as f64 * k(0x96f9b8);
            if k(0x96d6a0) < x {
                x = k(0x96d6a0);
            }
            f8 = x + k(0x9690a8);
        }
    }
    // a reputação dos dois clubes num pedido de empréstimo
    if p4b && r61 != 0 && cqe4_h(r61 + 0xd) < 0x128e && p2 != 0 && d4 != 0 && p2 != d4 && a0 & 0x10 != 0 {
        let ax = cqe4_h(p2 + 0x80);
        let mesmo = cqe4_d(p2 + 0x53) == cqe4_d(d4 + 0x53);
        let rb = cqe4_h(r61 + 0xb);
        if ax > cqe4_h(d4 + 0x80) && (mesmo || ax > 0x1676) {
            if rb <= 0x1c52 && na_lista_emp(p1, 0) != 0 {
                f8 *= k(0x968940);
            } else {
                f8 *= k(0x968928);
            }
        } else if ax > 0xabe && (mesmo || ax > 0x1676) && ax > rb {
            if rb <= 0x1c52 && na_lista_emp(p1, 0) != 0 {
                f8 *= k(0x969588);
            } else {
                f8 *= k(0x968980);
            }
        } else if ax > 0x109a && mesmo {
            if rb > 0x186a && ax < 0x1482 {
                f8 *= k(0x968fa0);
            } else if rb <= 0x1c52 && na_lista_emp(p1, 0) != 0 {
                f8 *= k(0x968928);
            } else {
                f8 *= k(0x9689c0);
            }
        }
    }
    if dislikes_club(p1, p2) != 0 {
        f8 *= k(0x968f78);
        fl(2);
    }
    if dislikes_club(p1, d4) != 0 {
        let s = cqe4_b(p1 + 0x5a).wrapping_add(cqe4_b(p1 + 0x59)).wrapping_add(cqe4_b(p1 + 0x5b)) as i8;
        if s > 0x2d {
            f8 *= k(0x968928);
        } else if s > 0x19 {
            f8 *= k(0x968940);
        } else {
            f8 = f8 + f8;
        }
        fl(4);
    }
    if likes_club(p1, d4) != 0 && na_lista_transf(p1, d4) == 0 && cqe4_b(c + 0x49) == 0 {
        f8 *= k(0x968968);
        fl(8);
    }
    if likes_club(p1, p2) != 0 {
        fl(0x10);
        f8 *= k(0x968930);
    }
    if idade > 0x22 && d4 != p2 && cqe4_d(p2 + 0x53) != cqe4_d(p1 + 0x1a) {
        f8 *= k(0x969040);
    }
    let max1 = |n: i8| if n < 1 { 1 } else { n as i32 };
    if d4 != 0 && d4 != p2 {
        let (e1, e2) = (cqe4_d(0x9d0118), cqe4_d(0x9d011c));
        let i2 = cqe4_d(p2);
        let i4 = cqe4_d(d4);
        if (i2 == e1 || i2 == e2) && (i4 == e1 || i4 == e2) {
            f8 = (k(0x967878) / max1(cqe4_c(p1 + 0x59)) as f64) * f8;
            fl(0x20);
        } else {
            let n2 = cqe4_d(cqe4_d(p2 + 0x53));
            let (na, nb) = (cqe4_d(0x9cf2ac), cqe4_d(0x9cf51c));
            if n2 == nb && cqe4_d(cqe4_d(d4 + 0x53)) == na && cqe4_d(cqe4_d(p1 + 0x1a)) == na {
                f8 *= k(0x967888);
                fl(2);
            } else if n2 == na && cqe4_d(cqe4_d(d4 + 0x53)) == nb && cqe4_d(cqe4_d(p1 + 0x1a)) == nb {
                f8 *= k(0x967880);
                fl(2);
            } else {
                if p2 == cqe4_d(d4 + 0xb3) || p2 == cqe4_d(d4 + 0xb7) || p2 == cqe4_d(d4 + 0xbb) {
                    let m = if n2 == cqe4_d(0x9cf360) {
                        k(0x967890) - cqe4_c(p1 + 0x59) as i32 as f64 * k(0x968a98)
                    } else {
                        k(0x968918) / max1(cqe4_c(p1 + 0x59)) as f64
                    };
                    f8 = m * f8;
                    if !p6.is_null() && m < k(0x968e40) {
                        *p6 |= 0x20;
                    }
                }
                if rival(p2, d4) != 0 {
                    let m = if cqe4_d(cqe4_d(p2 + 0x53)) == cqe4_d(0x9cf360) {
                        k(0x967890) - cqe4_c(p1 + 0x59) as i32 as f64 * k(0x968a98)
                    } else {
                        k(0x969040) / max1(cqe4_c(p1 + 0x59)) as f64
                    };
                    f8 = m * f8;
                    if !p6.is_null() && m < k(0x968e40) {
                        *p6 |= 0x20;
                    }
                }
            }
        }
    }
    let t2 = cqe4_d(p2 + 0xcf);
    if likes(t2, p1) != 0 {
        f8 = f8 + f8;
        fl(0x40);
    }
    if dislikes(t2, p1) != 0 {
        f8 *= k(0x968f78);
        fl(0x80);
    }
    if d4 != 0 {
        let t4 = cqe4_d(d4 + 0xcf);
        if dislikes(t4, p1) != 0 {
            f8 = f8 + f8;
            fl(0x100);
        }
        if likes(t4, p1) != 0 {
            f8 *= k(0x968968);
            if !p6.is_null() && cqe4_b(c + 0x4e) & 8 == 0 {
                *p6 |= 0x200;
            }
        }
    }
    // a felicidade no contrato (+0x44), a situação no elenco (ea) e a pedida do contrato (c1 = +0x3d)
    let mut ea: i8;
    let c1: i8;
    if c == 0 {
        if p2 == d4 {
            c1 = 0;
            ea = p5b;
        } else {
            ea = 7;
            c1 = -100;
        }
    } else {
        let e44 = cqe4_c(c + 0x44) as i32;
        if p2 == d4 {
            f8 = (e44 as f64 * k(0x969028) + k(0x967890)) * f8;
            ea = p5b;
            if cqe4_d(p2 + 0xcf) != 0 && is_150_days_since(cqe4_d(p2 + 0xcf)) {
                ea = categoria(cqe4_d(0xae1050), p1, c) as i8;
            }
        } else {
            let v = k(0x967890) - e44 as f64 * k(0x969028);
            f8 = v * f8;
            if v > k(0x96afa8) {
                f8 = v * f8;
            }
            if a0 & 0x10 != 0 {
                a88 = (a88 as f64 * k(0x968940)) as f32;
            }
            if a0 & 0x100 != 0 {
                a88 = (a88 as f64 * k(0x968940)) as f32;
            }
            if a0 & 1 != 0 {
                b4 = (b4 as f64 * k(0x968940)) as f32;
            }
            if !p6.is_null() {
                if v > k(0x968940) && cqe4_b(c + 0x4e) & 8 == 0 {
                    *p6 |= 0x400;
                } else if v < k(0x968968) {
                    *p6 |= 0x800;
                }
            }
            if d4 != 0 && cqe4_d(d4 + 0xcf) != 0 && is_150_days_since(cqe4_d(d4 + 0xcf)) {
                ea = categoria(cqe4_d(0xae1050), p1, c) as i8;
            } else {
                ea = (cqe4_b(c + 0x4f) >> 4) as i8;
            }
            if cqe4_b(c + 0x4e) & 8 != 0 {
                ea = if ea == 1 { 3 } else if ea < 1 || ea > 3 { 7 } else { 4 };
            }
        }
        c1 = cqe4_c(c + 0x3d);
    }
    let pos20 = || cqe4_b(cqe4_d(p1 + 0x61) + 0xf) == 0x14;
    let jovem = idade < 0x20 && cqe4_c(p1 + 0x22) != 0;
    let mut signed_byte_at: i16; // ebx: a reputação do clube atual (ou a da pessoa sem clube) na escala da comparação
    let mut e8: i32 = 0; // [base+0x1c]: a do clube novo
    let mut h68: i16 = 0;
    let mut h6c4: i32 = 0;
    let mut ae: i8 = 0;
    let mut den: f32 = 1.0; // [base+0x58]
    if p2 != d4 {
        if humano(p2, 0, 1, 0, 0) != 0 {
            let v = cqe4_c(cqe4_d(p2 + 0xcf) + 0x59) as i32 as f64 * k(0x967878) + k(0x968968);
            f8 = v * f8;
            if !p6.is_null() && v < k(0x96a698) {
                *p6 |= 0x20000000;
            }
        }
        match ea {
            1 => {
                f8 *= k(0x96a5d0);
                if !p6.is_null() && cqe4_d(c + 0x45) == 0 {
                    *p6 |= 0x1000;
                }
            }
            2 => {
                f8 *= k(0x969040);
                if !p6.is_null() && cqe4_d(c + 0x45) == 0 {
                    *p6 |= 0x1000;
                }
            }
            3 => f8 *= k(0x9689d8),
            4 => {
                f8 *= k(0x96d640);
                fl(0x2000);
                if idade < 0x20 {
                    if idade > 0x17 {
                        f8 = f8 + f8;
                    }
                    if cqe4_c(p1 + 0x22) != 0 {
                        f8 = f8 + f8;
                    }
                }
            }
            5 => {
                if !p4b {
                    f8 *= k(0x969040);
                }
                if jovem {
                    f8 = f8 + f8;
                }
            }
            6 => {
                f8 *= k(0x9689c0);
                if jovem {
                    f8 *= k(0x968938);
                }
            }
            7 => {
                f8 *= k(0x968988);
                fl(0x2000);
            }
            _ => {}
        }
        let cf10 = c != 0 && cqe4_b(c + 0x45) & 0x10 != 0;
        match p5b {
            1 => {
                if cf10 {
                    f8 = f8 + f8;
                    fl(0x4000);
                } else {
                    f8 *= k(0x96f9b0);
                }
                b4 *= cqe4_kf(0x96a33c);
            }
            2 => {
                if cf10 {
                    f8 *= k(0x96f9a8);
                    fl(0x4000);
                } else {
                    f8 *= k(0x96f9a0);
                }
                b4 *= cqe4_kf(0x96d3b8);
            }
            3 => {
                if !cf10 {
                    f8 *= k(0x96f998);
                }
                if pos20() {
                    f8 *= k(0x9689b0);
                }
            }
            4 => {
                if a0 & 0x10 != 0 {
                    f8 *= k(0x96f990);
                    if cqe4_d(p1 + 0x69) != 0 {
                        f8 *= k(0x96f988);
                    } else {
                        fl(0x8000);
                    }
                } else {
                    f8 *= if pos20() { k(0x968f20) } else { k(0x968fb0) };
                }
                if idade < 0x20 {
                    if idade > 0x17 {
                        f8 *= k(0x968968);
                    }
                    if cqe4_c(p1 + 0x22) != 0 {
                        f8 *= k(0x968968);
                    }
                }
                b4 *= cqe4_kf(0x96d38c);
            }
            5 => {
                if a0 & 0x10 != 0 {
                    f8 *= k(0x969020);
                    fl(0x8000);
                } else {
                    f8 *= if pos20() { k(0x968918) } else { k(0x968e00) };
                }
                if jovem {
                    f8 *= k(0x968968);
                }
                b4 *= cqe4_kf(0x96f980);
            }
            6 => {
                if a0 & 0x10 != 0 {
                    f8 *= k(0x968a98);
                    fl(0x8000);
                } else {
                    f8 *= if pos20() { k(0x967880) } else { k(0x968f20) };
                }
                if jovem {
                    f8 *= k(0x9689e0);
                }
                b4 *= cqe4_kf(0x968f18);
            }
            7 => f8 *= k(0x969048),
            _ => {}
        }
        // a reputação do clube novo, em várias escalas, contra a da pessoa
        let mut adv = cqe4_c(cqe4_d(p2 + 0x53) + 0x7e);
        if adv < 1 || adv > 3 {
            adv = 3;
        }
        ad = adv.wrapping_mul(adv);
        let h34 = reputation_by_stadium(p2);
        h68 = reputation_by_division(p2);
        let (h6c, reg) = reputation_by_competition(p2, h68);
        l84 = reg;
        let t = cqe4_d(p2 + 0xcf);
        let mut rv: i16 = 0;
        if t != 0 && cqe4_d(t + 0x69) != 0 {
            rv = vista(t, p1, 1);
            if cqe4_d(0xb67a44) != 0 {
                rv = reputation_by_manager(p2, rv) as i16;
            }
        }
        let mut si: i16 = cqe4_h(p2 + 0x80);
        let fin = financas(cqe4_d(0xae2a38), p2);
        if fin != 0 && ad == 1 {
            si = reputation_cap_by_finances(band(fin) as i8, si);
        }
        let ax = cqe4_h(cqe4_d(this + 0xdc).wrapping_add(cqe4_d(p2).wrapping_mul(9)) + 5);
        let m = if si > ax { si } else { ax };
        let cx = match_pl_ftol(si as f64 * k(0x96a5d0) + m as f64 * k(0x969510)) as i16;
        let den0 = ((b4 as f64 + a88 as f64) + aa8 as f64) + cqe4_kf(0x9689a8) as f64;
        den = if den0 > zero { den0 as f32 } else { 0.5 };
        let s = (((h6c as i16 as f64 * b4 as f64 + h68 as f64 * a88 as f64) + h34 as i16 as f64 * aa8 as f64) + cx as f64) + rv as f64;
        let mut w = match_pl_ftol(s / den as f64) as i16;
        let cl = cqe4_b(p2 + 0x64);
        if cl != 1 {
            if !(cf10 && cqe4_b(p1 + 0x22) == 0) {
                w = half_or_minus_2000(w);
            }
            if cl == 3 {
                w = half_or_minus_2000(w);
            }
        }
        e8 = match_pl_ftol(w as f64 * k(0x968940) - k(0x9688e8));
        if cqe4_d(p2 + 0x53) != 0 {
            let (m, f) = language_fluency_factor(p1, p2, e8 as i16);
            if idade < 0x12 && f < k(0x9688a0) && cqe4_d(cqe4_d(p1 + 0x1a) + 0x71) != cqe4_d(cqe4_d(p2 + 0x53) + 0x71) {
                return zero;
            }
            f8 = m * f8;
            if !p6.is_null() && m < k(0x968e48) {
                *p6 |= 0x10000;
            }
        }
        // as nações, a distância e o que a pessoa quer (situação pedida)
        #[derive(PartialEq)]
        enum Segue {
            Padrao,
            Fatores,
        }
        let fav = cqe4_d(p1 + 0x39);
        let segue = 'n: {
            let cid2 = if cqe4_d(p2 + 0x69) != 0 { cqe4_d(cqe4_d(p2 + 0x69) + 0x38) } else { 0 };
            let cid4 = if d4 != 0 && cqe4_d(d4 + 0x69) != 0 { cqe4_d(cqe4_d(d4 + 0x69) + 0x38) } else { 0 };
            if fav != 0 && cqe4_d(fav + 0x53) == cqe4_d(p2 + 0x53) {
                if !p4b {
                    break 'n Segue::Padrao;
                }
                let mut g = f8;
                if na_lista_emp(p1, 0) != 0 && idade < 0x18 && r61 != 0 && cqe4_h(r61 + 0xb) < 0x109a && cqe4_c(p1 + 0x5b) > 0xf
                    && cqe4_c(p1 + 0x5a) > 0xa && c != 0 && !matches!(ea, 1 | 2 | 3 | 4)
                {
                    g *= k(0x968e18);
                }
                if d4 != 0 && cqe4_d(p2 + 0x53) != 0 && cqe4_d(cqe4_d(p2 + 0x53)) == cqe4_d(0x9cf360) && idade < 0x18
                    && cqe4_d(d4 + 0x57) != 0 && cqe4_d(cqe4_d(d4 + 0x57)) == cqe4_d(0x9cf570)
                {
                    g *= k(0x968930);
                }
                if cid2 != 0 && cid4 != 0 {
                    let dd = shortlist_manager_city_distance(cid2, cid4);
                    if dd < k(0x968e30) {
                        f8 = g * k(0x96f978);
                        fl(0x20000);
                    } else if dd < k(0x96d6e0) {
                        let m = k(0x96afa0) - dd * k(0x968a90);
                        f8 = m * g;
                        if !p6.is_null() && m > k(0x969018) {
                            *p6 |= 0x20000;
                        }
                    } else {
                        f8 = g * k(0x968de8);
                    }
                } else {
                    f8 = g * k(0x968de8);
                }
                break 'n Segue::Fatores;
            }
            if cqe4_d(p2 + 0x53) != 0 && estrangeira(p1, cqe4_d(p2 + 0x53), 1) == 0 {
                if !p4b {
                    break 'n Segue::Padrao;
                }
                if d4 == 0 {
                    break 'n Segue::Fatores;
                }
                if cid2 != 0 && cid4 != 0 {
                    let dd = shortlist_manager_city_distance(cid2, cid4);
                    if dd < k(0x968e30) {
                        f8 *= k(0x96f978);
                        fl(0x20000);
                    } else if dd < k(0x96d6e0) {
                        let m = k(0x96afa0) - dd * k(0x968a90);
                        f8 = m * f8;
                        if !p6.is_null() && m > k(0x969018) {
                            *p6 |= 0x20000;
                        }
                    } else {
                        f8 *= k(0x968de8);
                    }
                } else {
                    f8 *= k(0x968de8);
                }
                break 'n Segue::Fatores;
            }
            let t = cqe4_d(p2 + 0xcf);
            let mesmo_tecnico = t != 0 && cqe4_d(t + 0x1a) != 0 && estrangeira(p1, cqe4_d(t + 0x1a), 1) == 0;
            if cqe4_d(p2 + 0x53) != 0 && cqe4_d(cqe4_d(p2 + 0x53)) == cqe4_d(0x9cf314) {
                f8 *= k(0x969040);
            }
            if p4b {
                f8 *= if mesmo_tecnico { k(0x96a6a8) } else { k(0x968920) };
                fl(0x40000);
                break 'n Segue::Fatores;
            }
            let c56 = cqe4_c(p1 + 0x56) as i32;
            let c57 = cqe4_c(p1 + 0x57) as i32;
            if mesmo_tecnico {
                match p5b {
                    2 => f8 = ((c56 as f64 * k(0x968ef0) + c57 as f64 * k(0x968a98)) + k(0x9689b0)) * f8,
                    3 => f8 = ((c56 as f64 * k(0x968ef0) + c57 as f64 * k(0x968a98)) + k(0x968e40)) * f8,
                    4 => {
                        f8 *= k(0x968968);
                        fl(0x80000);
                    }
                    5 => {
                        f8 *= k(0x968e00);
                        fl(0x80000);
                    }
                    6 => {
                        f8 *= k(0x968918);
                        fl(0x80000);
                    }
                    _ => {}
                }
            } else {
                match p5b {
                    2 => f8 = ((c56 + c57) as f64 * k(0x967898) + k(0x968e40)) * f8,
                    3 => f8 = ((c56 + c57) as f64 * k(0x967898) + k(0x968e48)) * f8,
                    4 => {
                        f8 *= k(0x9689e0);
                        fl(0x80000);
                    }
                    5 => {
                        f8 = ((c56 + c57) as f64 * k(0x968d60) + k(0x9689e0)) * f8;
                        fl(0x80000);
                    }
                    6 => {
                        f8 *= k(0x967878);
                        fl(0x80000);
                    }
                    _ => {}
                }
            }
            Segue::Padrao
        };
        if segue == Segue::Padrao {
            if (c != 0 && cqe4_b(c + 0x49) != 0) || in_list(p1, 0) != 0 {
                f8 *= k(0x968e18);
                if f8 < k(0x968950) && (cqe4_h(p2 + 0x80) > 0x1a5e || cqe4_h(p2 + 0x80) > cqe4_h(d4 + 0x80)) {
                    let t = f8 + k(0x967890);
                    f8 = if k(0x968950) < t { 2.0 } else { t };
                }
            }
        } else {
            if na_lista_emp(p1, 0) != 0 {
                f8 = (k(0x968940) - c1 as f64 * k(0x967898)) * f8;
            }
            let mut m = k(0x967890) - c1 as f64 * k(0x96d370);
            if m < k(0x967880) {
                m = k(0x967880);
            }
            f8 = m * f8;
            if !p6.is_null() && m < k(0x968e48) {
                *p6 |= 0x40000;
            }
            let a = if pos20() {
                (0x21 - idade as i32) as f64
            } else {
                let a = (0x1c - idade as i32) as f64;
                if a < zero { a * k(0x968e00) } else { a }
            };
            let a = a.abs() * k(0x968f00) + k(0x9689e0);
            f8 = a * f8;
            if !p6.is_null() && a < k(0x968e48) {
                *p6 |= 0x40000;
            }
        }
        if d4 == 0 {
            f8 *= k(0x968f98);
        } else if na_lista_transf(p1, 0) == 0 && na_lista_emp(p1, 0) == 0 && (c == 0 || cqe4_b(c + 0x49) == 0) {
            let dd = ((cqe4_w(0xae2c92) as i32 - cqe4_w(p1 + 0x40) as i32) * 365 - cqe4_h(p1 + 0x3e) as i32) + cqe4_h(0xae2c90) as i32;
            let e: i32 = if p4b {
                (20 - c1 as i32 / 5).wrapping_mul(dd)
            } else {
                if cqe4_d(c + 0x45) == 0 && dd < (cqe4_c(p1 + 0x59) as i32 + 8) * 15 {
                    return zero;
                }
                let dv = cqe4_c(p1 + 0x59) as i32 + 10;
                dd.wrapping_mul(20) / if dv < 1 { 1 } else { dv }
            };
            if e < 0 {
                return zero;
            }
            if e > 0x190 {
                l80 = 400.0 * k(0x969028);
            } else {
                if e == 0 {
                    return zero;
                }
                l80 = e as f64 * k(0x969028);
            }
        }
    } else {
        // o próprio clube: a situação no elenco pela pedida, com o técnico há mais de 150 dias
        if humano(p2, 0, 1, 0, 0) != 0 && is_150_days_since(cqe4_d(p2 + 0xcf)) {
            let lim = 0xceu8.wrapping_sub(cqe4_b(p1 + 0x59)) as i8;
            if c1 < lim && ea != 5 && ea != 6 {
                ea = 7;
            } else if c1 < 0 {
                if ea < 4 {
                    ea = 4;
                }
            } else if c1 < 0x32 {
                if ea < 3 {
                    ea = 3;
                }
            } else if c1 < 0x4b {
                if ea < 2 {
                    ea = 2;
                }
            } else if ea < 1 {
                ea = 1;
            }
        }
        match ea {
            1 => f8 *= k(0x968978),
            3 => f8 *= k(0x9689d8),
            4 => {
                f8 *= k(0x969520);
                if idade < 0x20 {
                    if idade > 0x17 {
                        f8 *= k(0x968968);
                    }
                    if cqe4_c(p1 + 0x22) != 0 {
                        f8 *= k(0x968968);
                    }
                }
            }
            5 => {
                f8 *= k(0x969040);
                if jovem {
                    f8 *= k(0x968968);
                }
            }
            6 => {
                f8 *= k(0x968e40);
                if jovem {
                    f8 *= k(0x9689e0);
                }
            }
            7 => f8 *= k(0x967898),
            _ => {}
        }
    }
    // a reputação do clube atual (ou da pessoa, sem clube) na mesma escala
    if d4 == 0 {
        let mut aev = cqe4_c(cqe4_d(p1 + 0x1a) + 0x7e);
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        signed_byte_at = match_pl_ftol(average_reputation3(cqe4_d(p1 + 0x61)) as i16 as f64 * k(0x968940) - k(0x9688e8)) as i16;
    } else {
        let n4 = cqe4_d(d4 + 0x53);
        let mut aev = if n4 != 0 { cqe4_c(n4 + 0x7e) } else { 3 };
        if aev < 1 || aev > 3 {
            aev = 3;
        }
        ae = aev.wrapping_mul(aev);
        let h44 = reputation_by_stadium(d4);
        h6c4 = reputation_by_division(d4) as i32;
        let (bx, reg) = reputation_by_competition(d4, h6c4 as i16);
        l8c = reg;
        let t4 = cqe4_d(d4 + 0xcf);
        let si: i16 = if t4 == 0 || cqe4_d(t4 + 0x69) == 0 {
            match_pl_ftol(h6c4 as i16 as f64 * k(0x969040)) as i16
        } else if cqe4_d(0xb67a44) == 0 {
            cqe4_h(cqe4_d(t4 + 0x69) + 0xa)
        } else {
            reputation_by_manager(d4, cqe4_h(cqe4_d(t4 + 0x69) + 0xa)) as i16
        };
        let mut di: i16 = cqe4_h(d4 + 0x80);
        let fin = financas(cqe4_d(0xae2a38), d4);
        if fin != 0 && ae == 1 && c != 0 && cqe4_d(c + 0x45) == 0 {
            let m = band(fin);
            if m == 0xfe {
                fl(0x40000000);
            } else {
                di = reputation_cap_by_finances(m as i8, di);
            }
        }
        let ax = cqe4_h(cqe4_d(this + 0xdc).wrapping_add(cqe4_d(d4).wrapping_mul(9)) + 5);
        let m = if di > ax { di } else { ax };
        let axv = match_pl_ftol(di as f64 * k(0x96a5d0) + m as f64 * k(0x969510)) as i16;
        // ponytail: no próprio clube (p2 == d4) a original divide pelo float [base+0x58] que ninguém gravou (sobra de
        // pilha); o resultado não é usado depois (o caminho vai direto ao fim), e a cópia divide por 1.
        let s = (((bx as i16 as f64 * b4 as f64 + h6c4 as i16 as f64 * a88 as f64) + h44 as i16 as f64 * aa8 as f64) + axv as f64) + si as f64;
        let mut di = match_pl_ftol(s / den as f64) as i16;
        if !p4b && in_list(p1, 0) != 0 {
            di = match_pl_ftol(di as f64 * k(0x9689d8)) as i16;
        }
        let cl = cqe4_b(d4 + 0x64);
        if cl != 1 {
            if !(c != 0 && cqe4_b(c + 0x45) & 0x10 != 0 && cqe4_b(p1 + 0x22) == 0) {
                di = half_or_minus_2000(di);
            }
            if cl == 3 {
                di = half_or_minus_2000(di);
            }
        }
        if lugar(cqe4_d(0xae1050), p1, d4) == 3 {
            di = match_pl_ftol(di as f64 * k(0x968e40)) as i16;
        }
        signed_byte_at = match_pl_ftol(di as f64 * k(0x968940) - k(0x9688e8)) as i16;
        if n4 != 0 && p2 != d4 {
            let (m, _) = language_fluency_factor(p1, d4, signed_byte_at);
            f8 = f8 / m;
        }
    }
    if p2 != d4 {
        let comp2 = cqe4_d(p2 + 0x57);
        if comp2 == 0 || cqe4_d(comp2) == cqe4_d(0x9cf764) {
            f8 *= k(0x967880);
        } else if cqe4_d(p1 + 0x39) == p2 && r61 != 0 && cqe4_c(r61 + 0x45) > 0xa {
            f8 = cqe4_c(r61 + 0x45) as i32 as f64 * f8 * k(0x967880);
        }
        let mut a8 = (cqe4_h(p2 + 0x80) as i32 - average_reputation3(r61)) as f64 * k(0x96a6b8) + k(0x967890);
        let e8s = e8 as i16 as i32;
        let sbs = signed_byte_at as i32;
        let cf10 = c != 0 && cqe4_b(c + 0x45) & 0x10 != 0;
        let mut longo = false;
        if p4b && (cf10 || na_lista_emp(p1, 0) != 0) {
            let f = cqe4_d(p1 + 0x61);
            longo = (cqe4_h(f + 5) as i32 * 50) < cqe4_h(d4 + 0x80) as i32
                && cqe4_c(p1 + 0x22) == 0
                && !(idade >= 0x18 && idade <= 0x20)
                && cqe4_h(p2 + 0x80) > 0xdac
                && cqe4_h(f + 0xb) < 0x157c
                && cqe4_d(p2 + 0x53) != 0
                && cqe4_d(p2 + 0x57) != 0
                && cqe4_d(d4 + 0x57) != 0
                && !(cqe4_h(cqe4_d(p2 + 0x57) + 0x69) <= 6 && cqe4_h(cqe4_d(p2 + 0x57) + 0x69) as i32 <= cqe4_h(cqe4_d(d4 + 0x57) + 0x69) as i32 - 3)
                && cqe4_d(d4 + 0x53) != 0
                && !(cqe4_c(cqe4_d(p2 + 0x53) + 0x85) < 0xa && cqe4_c(cqe4_d(p2 + 0x53) + 0x85) < cqe4_c(cqe4_d(d4 + 0x53) + 0x85));
        }
        let dv: f64 = if longo {
            (e8s - sbs) as f64 * k(0x968968)
        } else if cf10 && e8s < sbs && sbs - e8s < 0x1388 {
            let d = (sbs - e8s) as f64;
            d * k(0x96f970) * d
        } else if e8s > sbs {
            if c != 0 && cqe4_b(c + 0x45) & 1 != 0 {
                (e8s - sbs) as f64 * k(0x968e18)
            } else {
                (cqe4_c(p1 + 0x57) as i32 as f64 * k(0x96f930) + k(0x969040)) * (e8s - sbs) as f64
            }
        } else if p5b > 3 {
            (e8s - sbs) as f64 * k(0x968940)
        } else if !(f8 > k(0x9688b0)) {
            (e8s - sbs) as f64
        } else {
            let v = if sbs > 0x1b58 && e8s < 0x7d0 && p4b {
                let z = cqe4_kf(0x9689a8) as f64 - (sbs - e8s) as f64 * cqe4_kf(0x96f968) as f64;
                let mn = if f8 < k(0x968ff8) { f8 } else { k(0x968ff8) };
                let zm = z * mn;
                if k(0x9688b0) > zm {
                    k(0x9688b0) * k(0x967898)
                } else {
                    zm * k(0x967898)
                }
            } else {
                f8 * k(0x967898)
            };
            let mut w = k(0x9689c0) - v;
            if w < k(0x967880) {
                w = k(0x967880);
            } else if !(w <= k(0x967890)) {
                w = k(0x967890);
            }
            w * (e8s - sbs) as f64
        };
        if dv > zero && l80 < k(0x967890) {
            let mut t = dv * k(0x968a98);
            if k(0x9688a8) < t {
                t = k(0x9688a8);
            }
            let r = ((k(0x9688a8) - t) * l80 + t) * k(0x967898);
            f8 = f8 * r;
            if !p6.is_null() && r < k(0x968e48) {
                *p6 |= 0x100000;
            }
        }
        ramo("cqe_4 fator da injury_word_0c");
        let mut e0 = dv * k(0x96f960) + k(0x967890); // cqe4: o fator da reputação comparada
        if e0 < k(0x96a350) {
            e0 = f64::from_bits(0x3f50624dd2f1a9fc);
        }
        if c != 0 && cqe4_b(c + 0x46) & 1 != 0 {
            e0 *= k(0x968940);
        }
        let mut x = e0 * f8;
        let sem_pedida = |c: usize| c == 0 || cqe4_b(c + 0x49) == 0;
        if !p6.is_null() {
            if e0 < k(0x968e48) {
                let ok = b4 as f64 > k(0x968978) && l8c != 0 && l84 == 0 && x > k(0x968928) && ea < 4 && ea > 0 && sem_pedida(c);
                *p6 |= if ok { 0x400000 } else { 0x800000 };
            } else if e0 > k(0x968940) && b4 as f64 > k(0x968978) && l8c == 0 && l84 != 0
                && !(d4 != 0 && cqe4_h(d4 + 0x80) >= 0x1d4c) && p5b < 4 && p5b > 0 && sem_pedida(c)
            {
                *p6 |= 0x200000;
            }
        }
        if !p4b {
            x *= e0;
        }
        if c != 0 && cqe4_b(c + 0x45) & 1 != 0 {
            x *= e0;
            if !p6.is_null() {
                if e0 < k(0x9689b0) {
                    let ok = b4 as f64 > k(0x968978) && l8c != 0 && l84 == 0 && x > k(0x968928) && ea < 4 && ea > 0 && cqe4_b(c + 0x49) == 0;
                    *p6 |= if ok { 0x400000 } else { 0x800000 };
                } else if e0 > k(0x968950) && b4 as f64 > k(0x968978) && l8c == 0 && l84 != 0
                    && !(d4 != 0 && cqe4_h(d4 + 0x80) >= 0x1d4c) && p5b < 4 && p5b > 0 && cqe4_b(c + 0x49) == 0
                {
                    *p6 |= 0x200000;
                }
            }
        }
        if p4b {
            if cqe4_h(r61 + 0xb) > signed_byte_at && sbs > e8s {
                a8 = ((e8s - sbs) as f64 * k(0x96a600) + k(0x967890)) * a8;
                if a8 < k(0x967880) {
                    a8 = f64::from_bits(0x3fb999999999999a);
                }
            }
            let r = ae as i32 as f64 / ad as i32 as f64;
            x = x * r;
            if !p6.is_null() && r < k(0x968e48) {
                *p6 |= 0x1000000;
            }
        } else if d4 != 0 {
            if ae == ad {
                let t = cqe4_d(cqe4_d(p1 + 0x1a) + 0x71);
                if t != 0 && cqe4_d(t) == cqe4_d(0x9cfa10) {
                    let u = cqe4_d(cqe4_d(p2 + 0x53) + 0x71);
                    if u != 0 && cqe4_d(u) != cqe4_d(0x9cfa10) && u != cqe4_d(cqe4_d(d4 + 0x53) + 0x71) {
                        x *= k(0x968e00);
                        fl(0x2000000);
                    }
                }
            } else {
                let v6c = h6c4 as i16;
                let wv = if v6c > 0x1f4 { v6c as i32 } else { 500 };
                let mut q = (ae as i32 as f64 * h68 as f64) / (wv as f64 * ad as i32 as f64);
                if !(k(0x967898) <= q) {
                    q = k(0x967898);
                } else if k(0x9688b0) < q {
                    q = k(0x9688b0);
                }
                x = x * q;
                if cqe4_d(cqe4_d(p2 + 0x53) + 0x71) != cqe4_d(cqe4_d(d4 + 0x53) + 0x71) && q < k(0x967890) {
                    x *= k(0x968918);
                    fl(0x2000000);
                }
            }
        }
        f8 = a8 * x;
        if !p6.is_null() && a8 < k(0x968e48) {
            *p6 |= 0x800000;
        }
    }
    // jovem com o fim do contrato antes do fim da temporada do clube atual: não sai
    if d4 != 0 && d4 != p2 {
        let s7 = cqe4_h(cqe4_d(p1 + 0x61) + 7);
        if s7 > 0x82 && s7 as i32 > cqe4_h(d4 + 0x80) as i32 / 60 && idade < 0x1a
            && na_temporada(d4, data_a.as_mut_ptr() as usize, data_b.as_mut_ptr() as usize) != 0 && c1 > 0 && c != 0
        {
            let ano = u16::from_le_bytes([data_b[2], data_b[3]]);
            let dia = i16::from_le_bytes([data_b[0], data_b[1]]);
            let ano_c = cqe4_w(c + 0x2f);
            let antes = ano < ano_c || (ano == ano_c && dia < cqe4_h(c + 0x2d));
            if antes && cqe4_d(c + 0x45) == 0 && cqe4_b(c + 0x49) == 0
                && !(humano(p2, 1, 1, 0, 0) != 0 && cqe4_c(p1 + 0x5a) <= 0xf)
                && dias_ate(data_b.as_mut_ptr() as usize, cqe4_d(0xae2c90) as u32, cqe4_d(0xae2c94) as u32) < 0x96
            {
                fl(0x80000000);
                return zero;
            }
        }
    }
    if p3 == 0 || p4b || f8 < k(0x968f98) {
        return f8;
    }
    // o salário: o padrão (num STAFF_CONTRACT local) contra o teto do clube
    ramo("cqe_4 salario");
    let mut contrato_local = [0u32; 0x14];
    // 0x85e410: lea ecx, [base+0xa0] -> thiscall, ret 0
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x85e410usize);
    construir(contrato_local.as_mut_ptr() as usize);
    let fin = financas(cqe4_d(0xae2a38), p2);
    if fin == 0 {
        return zero;
    }
    // 0x864550: push 0; push 0xb; push situação; push 0; push clube; push pessoa; lea ecx, contrato -> thiscall, ret 0x18
    let padrao: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(0x864550usize);
    let w0 = padrao(contrato_local.as_mut_ptr() as usize, p1, p2, 0, p5, 0xb, 0);
    // 0x5968d0: push situação; push pessoa; mov ecx, finanças -> thiscall, ret 8
    let teto: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x5968d0usize);
    let a8w = teto(fin, p1, p5) as f64;
    let mut e8w = 0.0f64;
    let base = if !(k(0x967880) <= f8) { k(0x967880) } else { f8 };
    let mut q = k(0x968e38) / base; // cqe4: a pedida pelo interesse
    if q < k(0x9689d8) {
        q = f64::from_bits(0x3feccccccccccccd);
    }
    let mut wv = q * w0;
    if c != 0 {
        if a0 & 0x100 != 0 {
            e8w = 10.0;
        }
        if na_lista_transf(p1, d4) != 0 {
            e8w = e8w + k(0x9688b0);
        }
        if cqe4_c(c + 0x3d) < 0 {
            e8w = e8w - cqe4_c(c + 0x3d) as i32 as f64 * k(0x968918);
        }
        if a0 != 0 {
            e8w = e8w - cqe4_c(c + 0x44) as i32 as f64 * k(0x968918);
        }
        if !(zero <= e8w) {
            e8w = 0.0;
        } else if k(0x968ea8) < e8w {
            e8w = 20.0;
        }
    }
    let mut f = f8;
    if !(wv * k(0x96a5d0) <= a8w) {
        return zero;
    }
    if !(wv <= a8w) {
        let mut t = wv * k(0x968968);
        if !(a8w <= t) {
            t = a8w;
        }
        wv = (wv * (k(0x968ea8) - e8w) + t * e8w) * k(0x967888);
    }
    if !(wv <= a8w) && !(wv <= zero) {
        let r = a8w / wv;
        if r < k(0x968e40) {
            return k(0x967880);
        }
        f = f * r;
        wv = a8w;
    }
    if c == 0 {
        return f;
    }
    let mut g = cqe4_d(c + 0xc) as i32 as f64;
    if g < k(0x9688b0) {
        g = k(0x9688b0);
    }
    let mut h = if d4 == p2 { g / wv } else { wv / g };
    if !(k(0x967880) <= h) {
        h = k(0x967880);
    } else if k(0x9688b0) < h {
        return f * k(0x9688b0);
    } else if !(h < k(0x967890)) {
        return f * h;
    }
    f * ((((k(0x968ea8) - e8w) * h + e8w) * k(0x967888)) / q)
}

/// Comparador: lado a lado na nota (os bits do f64), nos motivos e em [0xae1080] (que a original e o salário padrão
/// 0x864550 gravam); o resto das chamadas só lê, e o STAFF_CONTRACT do salário é local.
unsafe extern "thiscall" fn comparar_staff_interest_in_joining_club(this: usize, p1: usize, p2: usize, p3: u32, p4: u32, p5: u32, p6: *mut u32) -> f64 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, *mut u32) -> f64>(%%I0%%);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    if !p6.is_null() {
        regioes.push((p6 as usize, 4));
    }
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&regioes, || o(this, p1, p2, p3, p4, p5, p6), || staff_interest_in_joining_club(this, p1, p2, p3, p4, p5, p6));
    PLACAR.record(igual && ro.to_bits() == rr.to_bits(), || format!("staff_interest_in_joining_club(pessoa {:#x}, clube {:#x}, {}, {}, {}) original={} {:x?} rust={} {:x?}", p1, p2, p3, p4 as u8, p5 as u8, ro, mo, rr, mr));
    ro
}

/// { 0x85e410 } (thiscall, ret 0, staff_contracts.cpp): `STAFF_CONTRACT::STAFF_CONTRACT()`: as duas datas (+0x25,
/// +0x2d) padrão (0x54a110), +4 e +0x4a = −1, +0x49, +0x3b, +0x4e, +0x40, +0x45 e +8 = 0. Devolve o próprio contrato.
unsafe extern "thiscall" fn construct_staff_contract(this: usize) -> usize {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    data_padrao(this + 0x25);
    data_padrao(this + 0x2d);
    core::ptr::write_unaligned((this + 4) as *mut u32, u32::MAX);
    core::ptr::write_unaligned((this + 0x4a) as *mut u32, u32::MAX);
    *((this + 0x49) as *mut u8) = 0;
    core::ptr::write_unaligned((this + 0x3b) as *mut u16, 0);
    *((this + 0x4e) as *mut u8) = 0;
    core::ptr::write_unaligned((this + 0x40) as *mut u32, 0);
    core::ptr::write_unaligned((this + 0x45) as *mut u32, 0);
    core::ptr::write_unaligned((this + 8) as *mut u32, 0);
    this
}

/// Comparador: lado a lado nos 0x4f bytes do contrato e na devolução.
unsafe extern "thiscall" fn comparar_construct_staff_contract(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I1%%);
    let (ro, rr, igual, mo, mr) = crate::comparison::side_by_side(&[(this, 0x4f)], || o(this), || construct_staff_contract(this));
    PLACAR.record(igual && ro == rr, || format!("construct_staff_contract({:#x}) original={:x?} rust={:x?}", this, mo, mr));
    ro
}
