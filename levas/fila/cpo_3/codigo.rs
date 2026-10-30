// ---------- %%LEVA%%a leva: os pontos dos técnicos: a virada do mês e do ano, o fim da temporada da competição e o técnico do mês (staff_records.cpp) ----------

unsafe fn cpo3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O registro de pontos de cada técnico (Staff_Records +8): 0xd2 bytes por pessoa que não é jogador ([0xae2368]) e
/// pelos 16 humanos. +0x86 id da pessoa (−1 vazio), +0xb6 pontos do ano, +0xba/+0xbe pontos na liga e na copa (+0xc2 e
/// +0xc6 as competições deles, −1 sem), +0xca/+0xce os do mês.
const CPO3_REGISTRO: usize = 0xd2;

unsafe fn cpo3_quantos() -> i32 {
    cpo3_i(0xae2368).wrapping_add(0x10)
}

unsafe fn cpo3_registro(this: usize, k: i32) -> usize {
    cpo3_d(this + 8).wrapping_add((k as usize).wrapping_mul(CPO3_REGISTRO))
}

/// A pessoa pelo id (registro de 0x6e bytes em [0xae23bc]; −1 dá o registro antes da tabela, como na original).
unsafe fn cpo3_pessoa(id: usize) -> usize {
    cpo3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O mês da data (0x54cfb0, o al com sinal).
unsafe fn cpo3_mes(data: usize) -> i16 {
    let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cfb0usize);
    mes(data) as u8 as i8 as i16
}

/// { 0x86dda0 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::daily_update(data)`: se o mês (0x54cfb0) ou o ano
/// (+2) da data não são os guardados (+0x14 mês, +0x16 ano), zera os pontos do mês (+0xca, +0xce) de todos os
/// registros e guarda o mês e o ano novos. No primeiro dia do ano (dia +0 = 0) passa os pontos do ano (+0xb6) de cada
/// registro com pessoa (+0x86) ao hall da fama (0x5ef270 em [0xb5ded8]) e os zera.
unsafe extern "thiscall" fn pontos_dos_tecnicos_no_dia(this: usize, data: usize) {
    if cpo3_mes(data) as i32 != cpo3_w(this + 0x14) as u16 as i32 || cpo3_w(data + 2) != cpo3_w(this + 0x16) {
        crate::comparacao::ramo("cpo_3 virada do mes");
        let base = cpo3_d(this + 8);
        let n = cpo3_quantos();
        for k in 0..n.max(0) as usize {
            let e = base.wrapping_add(k * CPO3_REGISTRO);
            cpo3_por(e + 0xca, 0);
            cpo3_por(e + 0xce, 0); // os pontos do mês
        }
        let m = cpo3_mes(data);
        core::ptr::write_unaligned((this + 0x14) as *mut i16, m);
        core::ptr::write_unaligned((this + 0x16) as *mut i16, cpo3_w(data + 2));
    }
    if cpo3_w(data) == 0 {
        let base = cpo3_d(this + 8);
        let hall: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x5ef270usize);
        let mut k = 0i32;
        while k < cpo3_quantos() {
            let e = base.wrapping_add((k as usize).wrapping_mul(CPO3_REGISTRO));
            let id = cpo3_d(e + 0x86);
            if id as i32 != -1 {
                hall(cpo3_d(0xb5ded8), cpo3_pessoa(id), cpo3_d(e + 0xb6));
            }
            cpo3_por(e + 0xb6, 0);
            k += 1;
        }
    }
}

/// { 0x86d6e0 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::end_season_for_nation(entrada)`: com a competição
/// da entrada (+0, id em +0 dela; sem competição −1; sem competição e sem +4 igual a [0x9cfa10], nada), para cada
/// registro: liga (+0xc2) da competição: soma a copa (+0xbe) se ela também é dessa competição (+0xc6 = −1, +0xbe = 0),
/// passa os pontos (+0xba) ao hall da fama (0x5ef330) se a competição guarda (+0x11c), zera e +0xc2 = −1; só a copa da
/// competição: passa +0xbe, zera e +0xc6 = −1.
unsafe extern "thiscall" fn pontos_dos_tecnicos_no_fim_da_competicao(this: usize, entrada: usize) {
    let p = cpo3_d(entrada + 4);
    let pela_global = p != 0 && cpo3_d(p) == cpo3_d(0x9cfa10);
    if !pela_global && cpo3_d(entrada) == 0 {
        return;
    }
    let comp = cpo3_d(entrada);
    let id = if comp != 0 { cpo3_i(comp) } else { -1 };
    let hall: unsafe extern "thiscall" fn(usize, usize, i32, usize) = core::mem::transmute(0x5ef330usize);
    let guarda = |c: usize| c != 0 && *((c + 0x11c) as *const u8) != 0;
    let base = cpo3_d(this + 8);
    let mut k = 0i32;
    while k < cpo3_quantos() {
        let s = base.wrapping_add((k as usize).wrapping_mul(CPO3_REGISTRO)) + 0xba;
        let copa = cpo3_i(s + 0xc);
        if cpo3_i(s + 8) == id {
            crate::comparacao::ramo("cpo_3 fim da liga");
            if copa == id {
                cpo3_por(s, cpo3_d(s).wrapping_add(cpo3_d(s + 4)));
                cpo3_por(s + 0xc, usize::MAX);
                cpo3_por(s + 4, 0);
            }
            if guarda(cpo3_d(entrada)) {
                hall(cpo3_d(0xb5ded8), cpo3_pessoa(cpo3_d(s - 0x34)), id, cpo3_d(s));
            }
            cpo3_por(s, 0);
            cpo3_por(s + 8, usize::MAX);
        } else if copa == id {
            if guarda(cpo3_d(entrada)) {
                hall(cpo3_d(0xb5ded8), cpo3_pessoa(cpo3_d(s - 0x34)), id, cpo3_d(s + 4));
            }
            cpo3_por(s + 4, 0);
            cpo3_por(s + 0xc, usize::MAX);
        }
        k += 1;
    }
}

/// { 0x86f950 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::find_manager_of_the_month(indice)`: na liga
/// ([0xadadfc] pelo índice; +0x43 tem de ser 2, senão a caixa "Error" e 0), entre os clubes da tabela (+0xb1, 0x3b
/// bytes, conta +0x3e) com técnico (+0xcf) no clube (+0x39) desde duas semanas atrás (0x54cab0 da data do jogo; ano
/// +0x40 sem sinal, dia +0x3e com sinal), o de mais pontos no mês (+0xce do registro 0x86f5b0, criado se falta), só
/// acima de 0. Devolve o técnico ou 0.
unsafe extern "thiscall" fn tecnico_do_mes_na_liga(this: usize, indice: i32) -> usize {
    let liga = cpo3_d(cpo3_d(0xadadfc).wrapping_add((indice as usize).wrapping_mul(4)));
    if *((liga + 0x43) as *const u8) != 2 {
        aviso_em(0xa83b78, 0x104e);
        return 0;
    }
    let antes: unsafe extern "thiscall" fn(usize, *mut u32, i32) -> usize = core::mem::transmute(0x54cab0usize);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    let mut melhor_pontos = 0i32;
    let mut melhor = 0usize;
    let mut k = 0i32;
    while k < cpo3_w(liga + 0x3e) as i32 {
        let clube = cpo3_d(cpo3_d(liga + 0xb1).wrapping_add((k as usize).wrapping_mul(0x3b)));
        let tecnico = cpo3_d(clube + 0xcf);
        if tecnico != 0 && cpo3_d(tecnico + 0x39) != 0 {
            let mut d = [0u32; 2];
            let q = antes(0xae2c90, d.as_mut_ptr(), 2); // duas semanas atrás
            let lo = cpo3_d(q) as u32;
            let ano = (lo >> 16) as u16;
            let ano_tecnico = cpo3_w(tecnico + 0x40) as u16;
            let recente = ano < ano_tecnico || (ano == ano_tecnico && (lo as u16 as i16) < cpo3_w(tecnico + 0x3e));
            if recente {
                crate::comparacao::ramo("cpo_3 tecnico recente");
            } else {
                let r = registro(this, tecnico, 1);
                if r != 0 {
                    let pontos = cpo3_i(r + 0xce);
                    if pontos > melhor_pontos { // o melhor do mês
                        crate::comparacao::ramo("cpo_3 melhor do mes");
                        melhor_pontos = pontos;
                        melhor = cpo3_d(clube + 0xcf);
                    }
                }
            }
        }
        k += 1;
    }
    melhor
}

/// Comparador: os pontos vão ao hall da fama (0x5ef270, soma): roda só a original e confere o plano: na virada, os pontos
/// do mês zerados e o mês e o ano guardados; no primeiro dia do ano, os pontos do ano zerados.
unsafe extern "thiscall" fn comparar_pontos_dos_tecnicos_no_dia(this: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let virada = cpo3_mes(data) as i32 != cpo3_w(this + 0x14) as u16 as i32 || cpo3_w(data + 2) != cpo3_w(this + 0x16);
    let ano = cpo3_w(data) == 0;
    let (m, a) = (cpo3_mes(data), cpo3_w(data + 2));
    o(this, data);
    if !virada && !ano {
        return;
    }
    let mut ok = !virada || (cpo3_w(this + 0x14) == m && cpo3_w(this + 0x16) == a);
    let mut primeiro = -1i32;
    for k in 0..cpo3_quantos().max(0) {
        let e = cpo3_registro(this, k);
        let certo = (!virada || (cpo3_d(e + 0xca) == 0 && cpo3_d(e + 0xce) == 0)) && (!ano || cpo3_d(e + 0xb6) == 0);
        if !certo && primeiro < 0 {
            primeiro = k;
        }
        ok &= certo;
    }
    PLACAR.registrar(ok, || format!("pontos_dos_tecnicos_no_dia(mês {}, ano {}) virada {} ano novo {}: registro {} não zerado", m, a, virada, ano, primeiro));
}

/// Comparador: os pontos vão ao hall da fama (0x5ef330, soma): roda só a original e confere o plano feito antes: os
/// quatro campos (+0xba, +0xbe, +0xc2, +0xc6) de cada registro.
unsafe extern "thiscall" fn comparar_pontos_dos_tecnicos_no_fim_da_competicao(this: usize, entrada: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let p = cpo3_d(entrada + 4);
    if !(p != 0 && cpo3_d(p) == cpo3_d(0x9cfa10)) && cpo3_d(entrada) == 0 {
        return o(this, entrada);
    }
    let comp = cpo3_d(entrada);
    let id = if comp != 0 { cpo3_i(comp) } else { -1 };
    let n = cpo3_quantos().max(0);
    let mut plano = Vec::with_capacity(n as usize);
    for k in 0..n {
        let s = cpo3_registro(this, k) + 0xba;
        let (mut liga, mut copa, mut cl, mut cc) = (cpo3_d(s), cpo3_d(s + 4), cpo3_i(s + 8), cpo3_i(s + 0xc));
        if cl == id {
            if cc == id {
                cc = -1; // a copa somada à liga, que vai ao hall da fama e zera
                copa = 0;
            }
            liga = 0;
            cl = -1;
        } else if cc == id {
            copa = 0;
            cc = -1;
        }
        plano.push((liga, copa, cl, cc));
    }
    o(this, entrada);
    let errado = (0..n).find(|&k| {
        let s = cpo3_registro(this, k) + 0xba;
        plano[k as usize] != (cpo3_d(s), cpo3_d(s + 4), cpo3_i(s + 8), cpo3_i(s + 0xc))
    });
    PLACAR.registrar(errado.is_none(), || format!("pontos_dos_tecnicos_no_fim_da_competicao(competição {}) registro {:?} difere do plano", id, errado));
}

/// Comparador: pelo valor (o registro 0x86f5b0 criado pela original já existe para a cópia).
unsafe extern "thiscall" fn comparar_tecnico_do_mes_na_liga(this: usize, indice: i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(%%I2%%);
    let liga = cpo3_d(cpo3_d(0xadadfc).wrapping_add((indice as usize).wrapping_mul(4)));
    if *((liga + 0x43) as *const u8) != 2 {
        return o(this, indice);
    }
    let r = o(this, indice);
    let n = tecnico_do_mes_na_liga(this, indice);
    PLACAR.registrar(r == n, || format!("tecnico_do_mes_na_liga({}) original={:x} rust={:x}", indice, r, n));
    r
}
