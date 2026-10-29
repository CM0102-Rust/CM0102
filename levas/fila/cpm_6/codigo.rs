// ---------- %%LEVA%%a leva: os recordes do clube: somar o valor da transferência aos totais de compra e venda, os recordes de jogos e gols na carreira pelo clube e o recorde de nota da temporada (club_records.cpp) ----------

unsafe fn cpm6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm6_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpm6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpm6_por_w(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

/// A notícia de recorde (id do clube, tipo) já está no vetor de 6 bytes [this+0x24] (conta +0x2c)?
unsafe fn cpm6_noticia_na_fila(this: usize, id: usize, tipo: u8) -> bool {
    let n = cpm6_d(this + 0x2c) as i32;
    let v = cpm6_d(this + 0x24);
    (0..n.max(0) as usize).any(|k| cpm6_d(v + 6 * k) == id && *((v + 6 * k + 4) as *const u8) == tipo)
}

/// { 0x448ad0 } (thiscall, ret 0xc, club_records.cpp): `Club_Records::add_transfer_totals(valor, comprador,
/// vendedor)`: o vendedor soma o `valor` ao recebido na temporada (+0x36b do registro da temporada 0x4528d0; −1 =
/// ainda nada: fica o valor) e zera o ano (+0x36f); acima do recorde de sempre (0x452710, mesmo campo), o recorde passa
/// a ser o da temporada com o ano da temporada do clube (0x669950), e com o total no mínimo (+0x80 do clube + 500) ×
/// 1000 e o ano diferente do recorde, a notícia de recorde 0x1f (0x452cb0) se ainda não está na fila. O comprador, o
/// mesmo pelo gasto (+0x365, ano +0x369, notícia 0x1e), mas o mínimo é conferido com o recorde ANTIGO e sem o ano. Os
/// registros são criados se faltarem (o segundo argumento 1). Comprador ou vendedor nulo: nada, 1. Devolve 1.
unsafe extern "thiscall" fn somar_totais_de_transferencia(this: usize, valor: i32, comprador: usize, vendedor: usize) -> u32 {
    if comprador == 0 || vendedor == 0 {
        return 1;
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let ano_do_clube: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669950usize);
    let noticia: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x452cb0usize);
    let somar = |campo: usize| {
        let v = cpm6_d(campo) as i32;
        cpm6_por(campo, if v == -1 { valor } else { v.wrapping_add(valor) } as usize);
    };
    // o vendedor: o recebido
    let t = temporada(this, vendedor, 1);
    let s = sempre(this, vendedor, 1);
    somar(t + 0x36b);
    cpm6_por_w(t + 0x36f, 0);
    let ano = ano_do_clube(vendedor) as i16;
    if (cpm6_d(s + 0x36b) as i32) < cpm6_d(t + 0x36b) as i32 {
        let minimo = (cpm6_w(vendedor + 0x80) as i32 + 500).wrapping_mul(1000);
        if cpm6_d(t + 0x36b) as i32 >= minimo && ano != cpm6_w(s + 0x36f) && !cpm6_noticia_na_fila(this, cpm6_d(vendedor), 0x1f) {
            crate::comparacao::ramo("cpm_6 noticia de recorde recebido");
            noticia(this, cpm6_d(vendedor) as u32, 0x1f); // recorde de valor recebido
        }
        cpm6_por(s + 0x36b, cpm6_d(t + 0x36b));
        cpm6_por_w(s + 0x36f, ano);
    }
    // o comprador: o gasto
    let t = temporada(this, comprador, 1);
    let s = sempre(this, comprador, 1);
    somar(t + 0x365);
    cpm6_por_w(t + 0x369, 0);
    let ano = ano_do_clube(comprador) as i16;
    if (cpm6_d(t + 0x365) as i32) > cpm6_d(s + 0x365) as i32 {
        let minimo = (cpm6_w(comprador + 0x80) as i32 + 500).wrapping_mul(1000);
        // a original confere o mínimo com o recorde antigo (o ecx que ainda tem o de sempre)
        if cpm6_d(s + 0x365) as i32 >= minimo && !cpm6_noticia_na_fila(this, cpm6_d(comprador), 0x1e) {
            crate::comparacao::ramo("cpm_6 noticia de recorde pago");
            noticia(this, cpm6_d(comprador) as u32, 0x1e); // recorde de valor pago
        }
        cpm6_por(s + 0x365, cpm6_d(t + 0x365));
        cpm6_por_w(s + 0x369, ano);
    }
    1
}

/// { 0x448cb0 } (thiscall, ret 0x10, club_records.cpp): `club_records_update_career_records(jogador, clube, jogos,
/// gols)`: nos recordes de sempre do clube (0x452710, criados se faltarem), se o tipo 0x17 vale para o clube (0x449590)
/// e os `jogos` passam o recorde (+0x247; −1 = sem recorde), o recorde = `jogos` e a referência do jogador em +0x249
/// (0x7e4920); o mesmo com o tipo 0x18 e os `gols` (+0x261, referência +0x263). Devolve 1. Jogador ou clube nulo, ou
/// sem registro: a caixa "Error" (linhas 0x6fa e 0x702) e 0.
unsafe extern "thiscall" fn recordes_de_carreira_no_clube(this: usize, jogador: usize, clube: usize, jogos: u32, gols: u32) -> u32 {
    if jogador == 0 || clube == 0 {
        aviso_em(0x98b6a0, 0x6fa);
        return 0;
    }
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let r = sempre(this, clube, 1);
    if r == 0 {
        aviso_em(0x98b6a0, 0x702);
        return 0;
    }
    let vale: unsafe extern "stdcall" fn(u32, usize, u32) -> u32 = core::mem::transmute(0x449590usize);
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
    for (tipo, campo, n) in [(0x17u32, 0x247usize, jogos as i16), (0x18, 0x261, gols as i16)] {
        if vale(tipo, clube, 1) != 0 {
            let v = cpm6_w(r + campo);
            if v == -1 || n > v {
                cpm6_por_w(r + campo, n);
                referencia(jogador, r + campo + 2);
            }
        }
    }
    1
}

/// { 0x452a90 } (thiscall, ret 4, club_records.cpp): `Club_Records::update_season_rating_record(clube)`: com jogos
/// na temporada (o byte +0x10 da entrada do clube, [this+0x1c] + id × 0x6a) e o registro da temporada (0x4528d0, sem
/// criar), a nota do recorde (+0x1f1) volta a −1.0 e, pelas 50 vagas do elenco (+0xd7), o jogador (não técnico humano:
/// id < [0xae2364]) com estatísticas (+0x61) no clube (+0x39 = clube ou o ligado a ele, 0x540a50) e registro de
/// estatísticas da temporada (0x7c3920, tipo 6) que jogou ao menos metade dos jogos (estatística 1, truncada, 0x7c2150 com
/// 6.5) e tem a maior média (estatística 0x11) vira o recorde: a nota (+0x1f1, float), os jogos (+0x20d), a referência
/// (+0x1f5, 0x7e4920) e +0x20e = 0. Devolve 1 se alguém virou recorde. Sem jogos ou sem registro: 0. Clube nulo: a
/// original (caixa 0x16f2, 0).
unsafe extern "thiscall" fn recorde_de_nota_da_temporada(this: usize, clube: usize) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, clube); // caixa: só a original
    }
    let entrada = cpm6_d(this + 0x1c).wrapping_add(cpm6_d(clube).wrapping_mul(0x6a));
    if *((entrada + 0x10) as *const u8) == 0 {
        return 0;
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let r = temporada(this, clube, 0);
    if r == 0 {
        return 0;
    }
    let ligado: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    let estatistica: unsafe extern "stdcall" fn(usize, u32, f32) -> f64 = core::mem::transmute(0x7c2150usize);
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
    let nota = (r + 0x1f1) as *mut f32;
    cpm6_por(r + 0x1f1, 0xbf800000); // −1.0f
    let mut achou = 0;
    for k in 0..50usize {
        let p = cpm6_d(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        let est = cpm6_d(p + 0x61);
        if est == 0 || cpm6_d(p) as i32 >= cpm6_d(0xae2364) as i32 {
            continue;
        }
        if cpm6_d(p + 0x39) != clube && cpm6_d(p + 0x39) != ligado(clube, 0, 1) {
            continue;
        }
        let reg = registro(cpm6_d(0xb74350), est, 6);
        if reg == 0 {
            continue;
        }
        let jogou = ftol(estatistica(reg, 1, 6.5)) as u8;
        if (jogou as i32) < *((entrada + 0x10) as *const i8) as i32 / 2 { // menos da metade dos jogos
            continue;
        }
        let media = estatistica(reg, 0x11, 6.5);
        // fcomp com o −1.0 (double) e fcom com o float guardado: passa só com a média maior
        if *nota as f64 == f64_em(0x968948) || media > *nota as f64 {
            crate::comparacao::ramo("cpm_6 recorde de nota da temporada");
            *nota = media as f32;
            *((r + 0x20d) as *mut u8) = jogou;
            referencia(p, r + 0x1f5);
            cpm6_por_w(r + 0x20e, 0);
            achou = 1;
        }
    }
    achou
}

/// Comparador: os registros podem ser criados (realloc) e a notícia realoca: os quatro registros são pedidos antes, na
/// ordem da original (vendedor: temporada, sempre; comprador: temporada, sempre; a segunda vez só acha), e com a fila
/// de notícias sem folga para duas, só a original. Lado a lado pelos quatro registros (+0x365, 12 bytes), a conta das
/// notícias e as duas vagas seguintes.
unsafe extern "thiscall" fn comparar_somar_totais_de_transferencia(this: usize, valor: i32, comprador: usize, vendedor: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32>(%%I0%%);
    if comprador == 0 || vendedor == 0 || (cpm6_d(this + 0x28).wrapping_sub(cpm6_d(this + 0x2c)) as i32) < 2 {
        return o(this, valor, comprador, vendedor); // ponytail: com realloc da fila, só a original
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let regs = [temporada(this, vendedor, 1), sempre(this, vendedor, 1), temporada(this, comprador, 1), sempre(this, comprador, 1)];
    let fila = cpm6_d(this + 0x24).wrapping_add(cpm6_d(this + 0x2c).wrapping_mul(6));
    let reg = [(regs[0] + 0x365, 12), (regs[1] + 0x365, 12), (regs[2] + 0x365, 12), (regs[3] + 0x365, 12), (this + 0x2c, 4), (fila, 12)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, valor, comprador, vendedor), || somar_totais_de_transferencia(this, valor, comprador, vendedor));
    PLACAR.registrar(igual && r == n, || format!("somar_totais_de_transferencia({}, {}, {}) {:x?} x {:x?}", valor, cpm6_d(comprador), cpm6_d(vendedor), mo, mr));
    r
}

/// Comparador: o registro de sempre é pedido antes (pode ser criado; a segunda vez só acha); lado a lado pelos dois
/// recordes e as referências (+0x247 a +0x27b). Nulos ou sem registro: só a original (a caixa).
unsafe extern "thiscall" fn comparar_recordes_de_carreira_no_clube(this: usize, jogador: usize, clube: usize, jogos: u32, gols: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32>(%%I1%%);
    if jogador == 0 || clube == 0 {
        return o(this, jogador, clube, jogos, gols);
    }
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let reg = sempre(this, clube, 1);
    if reg == 0 {
        return o(this, jogador, clube, jogos, gols);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(reg + 0x247, 0x34)], || o(this, jogador, clube, jogos, gols), || recordes_de_carreira_no_clube(this, jogador, clube, jogos, gols));
    PLACAR.registrar(igual && r == n, || format!("recordes_de_carreira_no_clube({}, {}, {}, {}) {:x?} x {:x?}", cpm6_d(jogador), cpm6_d(clube), jogos as i16, gols as i16, mo, mr));
    r
}

/// Comparador: lado a lado pelo recorde de nota (+0x1f1 a +0x210 do registro da temporada, pedido antes sem criar).
unsafe extern "thiscall" fn comparar_recorde_de_nota_da_temporada(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    if clube == 0 {
        return o(this, clube);
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let t = temporada(this, clube, 0);
    let reg: &[(usize, usize)] = if t == 0 { &[] } else { &[(t + 0x1f1, 0x1f)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, clube), || recorde_de_nota_da_temporada(this, clube));
    PLACAR.registrar(igual && r == n, || format!("recorde_de_nota_da_temporada({}) original={} rust={} {:x?} x {:x?}", cpm6_d(clube), r, n, mo, mr));
    r
}
