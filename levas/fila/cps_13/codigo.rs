// ---------- %%LEVA%%a leva: o dia a dia das copas e o risco de doença na viagem: a atualização da copa no dia, montar a rodada da copa e o risco de doença dos visitantes (Cup.cpp, fix_man.cpp) ----------

unsafe fn cps13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps13_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps13_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Uma data montada pela 0x549ef0 (dia, ano: ela lê as palavras; os dwords vão como os da original).
unsafe fn make_date(dia: u32, ano: u32) -> [u8; 8] {
    let montar: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mut d = [0u8; 8];
    montar(&mut d, dia, ano);
    d
}

/// (dia, ano) de uma data.
fn cup_date_day_year(d: &[u8; 8]) -> (u16, u16) {
    (u16::from_le_bytes([d[0], d[1]]), u16::from_le_bytes([d[2], d[3]]))
}

/// A rodada `k` da copa (0x68 bytes cada, em +0xa3).
unsafe fn round_record_at(this: usize, k: isize) -> usize {
    (cps13_d(this + 0xa3) as isize).wrapping_add(k.wrapping_mul(0x68)) as usize
}

/// A data da rodada `k` (+0 o dia, +2 o ano a partir do ano-base +0x40 da copa, byte com sinal), como nos blocos da
/// 0x51f890 em que a metade alta do dword do dia é a do ponteiro da rodada (a do ano é zero).
unsafe fn round_date(this: usize, k: isize) -> [u8; 8] {
    let r = round_record_at(this, k);
    let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
    make_date((r as u32 & 0xffff_0000) | cps13_w(r) as u32, ano)
}

/// A virtual `off` do objeto `p` (thiscall, sem argumentos).
unsafe fn cup_call_virtual_no_args(p: usize, off: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cps13_d(cps13_d(p) + off));
    f(p)
}

/// A fase `i` da copa ([+0xc][i]).
unsafe fn cup_stage(this: usize, i: usize) -> usize {
    cps13_d(cps13_d(this + 0xc) + i * 4)
}

/// { 0x51f890 } (thiscall, ret 8, Cup.cpp; a atualização diária de cada copa): `Cup::update_for_date(hoje, fase)`. Copa
/// sem pai (+0x44 = 0xff): com +0xb1, se as fases 0..=+0x30 estão completas (virtual +0x10 com 0, 1), passa à fase
/// seguinte (virtual +0x28, se +0x30 < +0x2c − 1) ou encerra uma vez (+0x4c: o histórico 0x442e80 de [0xad9da8], a
/// virtual +0x30, os recordes 0x4a19f0 do Comp_Stats +8); depois as notícias dos recordes (0x4a2450). Ainda sem pai,
/// cada fase recebe o dia (virtual +0x14 com 0xae2c90 e a fase). Fase 0 (com [0xae2c88] = 1): a última rodada final
/// (+7 = 0x8c) sorteia na data dela (0x51c410); sem +0xb1, sorteia as rodadas a partir de +0xac cuja data chegou, até
/// não avançar ou acabar. Fase ≠ 0 (com +0x45): o mesmo com a montagem das rodadas (0x51c800) a partir de +0xab, que
/// também monta na data pendente da rodada (+0x34 da fase da rodada). Devolve 0.
unsafe extern "thiscall" fn cup_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    let sortear: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51c410usize);
    let montar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51c800usize);
    if cps13_b(this + 0x44) == 0xff {
        let mut completas = cps13_b(this + 0xb1) != 0;
        if completas {
            let mut i = 0usize;
            while (i as i32) <= cps13_d(this + 0x30) as i32 {
                let p = cup_stage(this, i);
                let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(cps13_d(cps13_d(p) + 0x10));
                if completa(p, 0, 1) == 0 {
                    completas = false;
                    break;
                }
                i += 1;
            }
        }
        if completas {
            if (cps13_d(this + 0x30) as i32) < (cps13_d(this + 0x2c) as i32).wrapping_sub(1) {
                cup_call_virtual_no_args(this, 0x28); // a fase seguinte
            } else if cps13_d(this + 0x4c) == 0 {
                let historico: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x442e80usize);
                historico(cps13_d(0xad9da8), this);
                cup_call_virtual_no_args(this, 0x30);
                if cps13_d(this + 8) != 0 {
                    let recordes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4a19f0usize);
                    recordes(cps13_d(this + 8));
                }
                core::ptr::write_unaligned((this + 0x4c) as *mut u32, 1);
            }
        }
        if cps13_d(this + 8) != 0 {
            let noticias: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4a2450usize);
            noticias(cps13_d(this + 8));
        }
    }
    if cps13_b(this + 0x44) == 0xff {
        let mut i = 0usize;
        while (i as i32) <= cps13_d(this + 0x30) as i32 {
            let p = cup_stage(this, i);
            let dia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(cps13_d(cps13_d(p) + 0x14));
            dia(p, 0xae2c90, fase);
            i += 1;
        }
    }
    let (hdia, hano) = (cps13_w(hoje), cps13_w(hoje + 2));
    let n = cps13_w(this + 0x3c) as i16;
    if fase == 0 {
        if cps13_b(0xae2c88) != 1 {
            return 0;
        }
        if n > 0 && cps13_w(round_record_at(this, n as isize - 1) + 7) == 0x8c {
            let r = round_record_at(this, n as isize - 1);
            let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
            let d = make_date(cps13_w(r) as u32, ano);
            if cup_date_day_year(&d) == (hdia, hano) {
                sortear(this, (n as i32 - 1) as u32); // a final na data dela
            }
        }
        if cps13_b(this + 0xb1) != 0 {
            return 0;
        }
        let mut k = cps13_b(this + 0xac) as i8;
        if k as i16 == cps13_w(this + 0x3c) as i16 {
            return 0;
        }
        loop {
            let (dia, ano) = cup_date_day_year(&round_date(this, k as isize));
            // o ano sem sinal, o dia com sinal
            if ano > hano || (ano == hano && dia as i16 > hdia as i16) { // cps13: a data do sorteio
                return 0;
            }
            crate::comparison::branch("cps_13 sorteio da rodada");
            sortear(this, cps13_b(this + 0xac) as i8 as i32 as u32);
            let novo = cps13_b(this + 0xac) as i8;
            if novo == k || novo as i16 == cps13_w(this + 0x3c) as i16 {
                return 0;
            }
            k = novo;
        }
    }
    if cps13_d(this + 0x45) == 0 {
        return 0;
    }
    if n > 0 && cps13_w(round_record_at(this, n as isize - 1) + 7) == 0x8c {
        let r = round_record_at(this, n as isize - 1);
        let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
        let d = make_date(cps13_w(r) as u32, ano);
        if cup_date_day_year(&d) == (hdia, hano) {
            montar(this, (n as i32 - 1) as u32); // a final na data dela
        }
    }
    if cps13_b(this + 0xb1) != 0 || cps13_d(this + 0x45) == 0 {
        return 0;
    }
    let mut k = cps13_b(this + 0xab) as i8;
    if k as i16 == cps13_w(this + 0x3c) as i16 {
        return 0;
    }
    loop {
        let (dia, ano) = cup_date_day_year(&round_date(this, k as isize));
        if (ano, dia) != (hano, hdia) { // cps13: a data da montagem
            let atual = cps13_b(this + 0xab) as i8 as isize;
            let p = cps13_d((cps13_d(this + 0xad) as isize + atual * 4) as usize);
            if cps13_b(p + 0x34) == 0 {
                return 0; // nem a data nem pendente
            }
        }
        crate::comparison::branch("cps_13 montagem da rodada");
        montar(this, cps13_b(this + 0xab) as i8 as i32 as u32);
        let novo = cps13_b(this + 0xab) as i8;
        if novo == k || novo as i16 == cps13_w(this + 0x3c) as i16 {
            return 0;
        }
        k = novo;
    }
}

/// { 0x51c800 } (thiscall, ret 4, Cup.cpp): `Cup::set_up_round(rodada)`: sem +0x45: 1. Rodada fora de 0..+0x3c − 1: a
/// caixa (linha 0xa64) e −1. Sorteia a rodada se a fase dela não tem +0x34 (0x51c410). Na rodada atual (+0xab): se um
/// confronto (0x35 bytes na fase, +0/+2 os times ou 0xffff) tem time sem clube (+0xa7, 6 bytes), adia a rodada pedida e
/// as seguintes um dia (0x54c5b0, 0x417c10; as seguintes só se ficaram antes) e devolve 1. Senão cria os jogos de cada
/// confronto marcado (+0x34 = 1 adiado, 2 já: 0x51cb90; depois do 2, sem a copa em [0xadadfc][id]: 1), leva os jogos às
/// datas da rodada (data +3/+5 num bloco de 8 bytes, 0x51ebe0, liberado depois) e, com a copa em [0xadadfc][id], passa à
/// rodada seguinte (fora da final 0x8c) e leva +0xac junto. Devolve 1.
unsafe extern "thiscall" fn set_up_round(this: usize, rodada: u32) -> u32 {
    let id = cps13_d(cps13_d(this + 4));
    let ativa = || cps13_d(cps13_d(0xadadfc) + id * 4) != 0;
    if cps13_d(this + 0x45) == 0 {
        return 1;
    }
    let ri = rodada as i32;
    if ri > cps13_w(this + 0x3c) as i16 as i32 - 1 || ri < 0 {
        show_error_at(0x9b55ac, 0xa64);
        return u32::MAX;
    }
    let ad = cps13_d(this + 0xad);
    if cps13_b(cps13_d(ad + rodada as usize * 4) + 0x34) == 0 {
        let sortear: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51c410usize);
        sortear(this, rodada);
    }
    let atual = cps13_b(this + 0xab) as i8 as isize;
    let base = cps13_d(this + 0xa3);
    let off = atual * 0x68;
    let n = *((base as isize + off + 0x1a) as *const i16) as i32;
    let mut sem_clube = false;
    for i in 0..n.max(0) as usize {
        let t = cps13_d((cps13_d(this + 0xad) as isize + atual * 4) as usize) + i * 0x35;
        for lado in [0usize, 2] {
            let time = cps13_w(t + lado);
            if time != 0xffff && cps13_d(cps13_d(this + 0xa7) + (time as i16 as isize * 6) as usize) == 0 {
                sem_clube = true;
                break;
            }
        }
        if sem_clube {
            break;
        }
    }
    if sem_clube {
        // a rodada pedida (não a atual) e as seguintes ficam um dia depois
        let mais_dias: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let atribuir: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
        let r = round_record_at(this, ri as isize);
        let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
        let mut d = make_date((cps13_d(this + 0xa7) as u32 & 0xffff_0000) | cps13_w(r) as u32, ano);
        let (mut amanha, mut copia) = ([0u8; 8], [0u8; 8]);
        let p = mais_dias(&mut d, &mut amanha, 1);
        atribuir(&mut d, &mut copia, p);
        let (dia, ano) = cup_date_day_year(&d);
        crate::comparison::branch("cps_13 rodada adiada");
        let r = round_record_at(this, ri as isize);
        core::ptr::write_unaligned(r as *mut u16, dia);
        *((r + 2) as *mut u8) = (ano as u8).wrapping_sub(cps13_b(this + 0x40));
        let mut k = ri + 1;
        while (k as i16) < cps13_w(this + 0x3c) as i16 {
            let r = round_record_at(this, k as i16 as isize);
            let rano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
            let (odia, oano) = cup_date_day_year(&make_date((cps13_d(this + 0xa3) as u32 & 0xffff_0000) | cps13_w(r) as u32, rano));
            if ano > oano || (ano == oano && dia as i16 > odia as i16) {
                let r = round_record_at(this, k as i16 as isize);
                core::ptr::write_unaligned(r as *mut u16, dia);
                *((r + 2) as *mut u8) = (ano as u8).wrapping_sub(cps13_b(this + 0x40));
            }
            k += 1;
        }
        return 1;
    }
    let criar: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(0x51cb90usize);
    let mut i = 0i32;
    while n > 0 && i < *((cps13_d(this + 0xa3) as isize + off + 0x1a) as *const i16) as i32 {
        let marca = |i: i32| cps13_b(cps13_d((cps13_d(this + 0xad) as isize + atual * 4) as usize) + i as usize * 0x35 + 0x34);
        if marca(i) == 1 {
            criar(this, atual as i32 as u32, i as u32, 1);
        }
        if marca(i) == 2 {
            criar(this, atual as i32 as u32, i as u32, 0);
            if !ativa() {
                return 1;
            }
        }
        i += 1;
    }
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let montar_data: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let p = novo(8);
    let data = if p != 0 {
        let base = cps13_d(this + 0xa3);
        let r = (base as isize + off) as usize;
        let ano = (cps13_b(r + 5) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
        let dia = cps13_w(r + 3) as u32;
        montar_data(p, (r as u32 & 0xffff_0000) | dia, (base as u32 & 0xffff_0000) | ano)
    } else {
        0
    };
    let levar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51ebe0usize);
    crate::comparison::branch("cps_13 jogos da rodada");
    levar(this, data, atual as i32 as u32);
    if data != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        liberar(data);
    }
    if !ativa() {
        return 1;
    }
    if cps13_w((cps13_d(this + 0xa3) as isize + off + 7) as usize) != 0x8c {
        *((this + 0xab) as *mut u8) = cps13_b(this + 0xab).wrapping_add(1);
    }
    if (cps13_b(this + 0xac) as i8) < cps13_b(this + 0xab) as i8 {
        *((this + 0xac) as *mut u8) = cps13_b(this + 0xab);
    }
    1
}

/// Põe o risco de doença (0x62e820 de [0xb63d28], tipo 3) na véspera da data do jogo `f` (+0x2a o dia, +0x28 o ano;
/// 0x549ef0, 0x54c8e0, 0x417c10) para o clube `id` com a nação do estádio. `alto_dia`/`alto_ano`: as metades altas dos
/// dwords que a original empurra à 0x549ef0.
unsafe fn add_illness_risk_before_match(f: usize, id: usize, nacao: usize, alto_dia: u32, alto_ano: u32) {
    let menos_dias: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let atribuir: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let doenca: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32, usize, usize) -> u32 = core::mem::transmute(0x62e820usize);
    let mut d = make_date((alto_dia & 0xffff_0000) | cps13_w(f + 0x2a) as u32, (alto_ano & 0xffff_0000) | cps13_w(f + 0x28) as u32);
    let (mut vespera, mut copia) = ([0u8; 8], [0u8; 8]);
    let p = menos_dias(&mut d, &mut vespera, 1);
    atribuir(&mut d, &mut copia, p);
    doenca(cps13_d(0xb63d28), &mut d, 3, id, nacao);
}

/// { 0x5ab3c0 } (thiscall, ret 4, fix_man.cpp; o `this` não é usado): `FIXTURE_MANAGER::add_away_trip_illness_risk(jogo)`:
/// com o estádio do jogo (+0x18), a cidade (+0x38) e a nação dela (+0x1f) num país do terceiro mundo (+0x7e = 3), cada
/// clube visitante de outra nação (+0x53) — o mandante (+0x1c, id +0xc) e o visitante (+0x20, id +0x10) — ganha o risco
/// de doença na véspera do jogo.
unsafe extern "thiscall" fn add_away_trip_illness_risk(_this: usize, f: usize) {
    let estadio = cps13_d(f + 0x18);
    if estadio == 0 {
        return;
    }
    let cidade = cps13_d(estadio + 0x38);
    if cidade == 0 {
        return;
    }
    let nacao = cps13_d(cidade + 0x1f);
    if nacao == 0 {
        return;
    }
    if cps13_b(nacao + 0x7e) != 3 { // cps13: só o terceiro mundo
        return;
    }
    let casa = cps13_d(f + 0x1c);
    if casa != 0 && cps13_d(casa + 0x53) != nacao {
        crate::comparison::branch("cps_13 doenca do mandante");
        // a original empurra o ano no eax do clube e o dia no ecx da nação (metades altas dos ponteiros)
        add_illness_risk_before_match(f, cps13_d(f + 0xc), cps13_d(cps13_d(cps13_d(f + 0x18) + 0x38) + 0x1f), nacao as u32, casa as u32);
    }
    let fora = cps13_d(f + 0x20);
    if fora != 0 && cps13_d(fora + 0x53) != cps13_d(cps13_d(cps13_d(f + 0x18) + 0x38) + 0x1f) {
        crate::comparison::branch("cps_13 doenca do visitante");
        // o ano no ecx do estádio e o dia no edx da cidade
        let (e, c) = (cps13_d(f + 0x18), cps13_d(cps13_d(f + 0x18) + 0x38));
        add_illness_risk_before_match(f, cps13_d(f + 0x10), cps13_d(c + 0x1f), c as u32, e as u32);
    }
}

// ponytail: sorteia e monta rodadas (aloca jogos): não roda duas vezes. Só a original; a cópia leva os ramos (sorteio e
// montagem; a atualização é diária em cada copa, 29.934 chamadas até 2002-06-01: sem ramo no caminho comum).
unsafe extern "thiscall" fn comparar_cup_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%)(this, hoje, fase)
}

// ponytail: sorteia, cria os jogos e passa a rodada: não roda duas vezes. Só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_set_up_round(this: usize, rodada: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%)(this, rodada)
}

// ponytail: acrescenta o risco de doença (não roda duas vezes; o caminho comum não faz nada). Só a original; a cópia leva
// os ramos.
unsafe extern "thiscall" fn comparar_add_away_trip_illness_risk(this: usize, f: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, f)
}
