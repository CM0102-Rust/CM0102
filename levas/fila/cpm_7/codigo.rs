// ---------- %%LEVA%%a leva: os amistosos: se a competição vale na data, se o clube já tem amistoso perto da data e acrescentar o amistoso planejado (friendly.cpp) ----------

unsafe fn cpm7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm7_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// (dia, ano) de uma data (+0 dia, palavra com sinal; +2 ano, palavra sem sinal).
unsafe fn friendly_date_day_year(a: usize) -> (i16, u16) {
    (core::ptr::read_unaligned(a as *const i16), core::ptr::read_unaligned((a + 2) as *const u16))
}

/// { 0x5c29b0 } (thiscall, ret 8, friendly.cpp): `Friendly::is_comp_current_on_date(comp, data)`: competição que não
/// é a do amistoso ([this+4]) e de id abaixo de [0xae2378] − 0x7f: 1. Senão, a vaga do torneio (this + 0x69 × (byte do
/// id − byte +0xaf), +0xb1 ocupada, +0xb5 a data): 1 se a `data` está depois da data da vaga − 7 dias (0x54c8e0) e
/// antes dela + 7 (0x54c5b0), pelo ano (sem sinal) e depois o dia (com sinal); vaga vazia: 0. O quadro SEH só guarda
/// as datas locais (os destrutores são vazios).
unsafe extern "thiscall" fn is_comp_current_on_date(this: usize, comp: usize, data: usize) -> u32 {
    if comp != cpm7_d(this + 4) && (cpm7_d(comp) as i32) < (cpm7_d(0xae2378) as i32).wrapping_sub(0x7f) {
        return 1;
    }
    let k = (*(comp as *const u8)).wrapping_sub(*((this + 0xaf) as *const u8)) as i8 as isize;
    let e = this.wrapping_add((k * 0x69) as usize);
    if cpm7_d(e + 0xb1) == 0 {
        return 0;
    }
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let (dia, ano) = friendly_date_day_year(data);
    let mut a = [0u8; 8];
    let (adia, aano) = friendly_date_day_year(menos(e + 0xb5, &mut a, 7));
    if ano < aano || (ano == aano && dia <= adia) {
        return 0;
    }
    let (dia, ano) = friendly_date_day_year(data);
    let mut b = [0u8; 8];
    let (bdia, bano) = friendly_date_day_year(mais(e + 0xb5, &mut b, 7));
    (ano < bano || (ano == bano && dia < bdia)) as u32
}

/// { 0x5c45a0 } (thiscall sem usar o this, ret 8, friendly.cpp): `Friendly::candidate_has_nearby_date(clube, data)`:
/// 1 se a lista de amistosos planejados do candidato (+0x12; itens com dia +0 e ano +2) tem a `data`, o dia antes
/// (0x54c8e0) ou o dia depois (0x54c5b0). Percorre do começo (o cursor +0xa = a cabeça +2; 0x68d1b0). Sem lista: 0. As
/// datas locais vêm antes (como a original); o quadro SEH é delas (destrutores vazios).
unsafe extern "thiscall" fn candidate_has_nearby_date(_this: usize, clube: usize, data: usize) -> u32 {
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let (mut antes, mut depois) = ([0u8; 8], [0u8; 8]);
    menos(data, &mut antes, 1);
    mais(data, &mut depois, 1);
    let l = cpm7_d(clube + 0x12);
    if l == 0 {
        return 0;
    }
    cpm7_por(l + 0xa, cpm7_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let antes = friendly_date_day_year(antes.as_ptr() as usize);
    let depois = friendly_date_day_year(depois.as_ptr() as usize);
    let mut e = proximo(l);
    while e != 0 {
        let item = friendly_date_day_year(e);
        if item == friendly_date_day_year(data) {
            return 1;
        }
        if item == antes {
            crate::comparacao::ramo("cpm_7 amistoso no dia antes");
            return 1;
        }
        if item == depois {
            crate::comparacao::ramo("cpm_7 amistoso no dia depois");
            return 1;
        }
        e = proximo(l);
    }
    0
}

/// { 0x5c4b60 } (thiscall, ret 0x10, friendly.cpp): `Friendly::add_planned_friendly(anfitriao, mandante, visitante,
/// data)`: acrescenta ao vetor de 0x14 bytes [this+0x34d8] (conta +0x34e0, capacidade +0x34e4; cheio: cresce de 100
/// em 100, realloc 0x945667; sem memória, a caixa "Error" 0x797 e volta) o amistoso: a data em +0 (0x417c10, a cópia
/// devolvida fica na pilha) e os ids (o dword apontado por cada argumento) em +8, +0xc e +0x10; a conta sobe.
unsafe extern "thiscall" fn add_planned_friendly(this: usize, anfitriao: usize, mandante: usize, visitante: usize, data: usize) {
    if cpm7_d(this + 0x34e0) == cpm7_d(this + 0x34e4) {
        crate::comparacao::ramo("cpm_7 amistosos crescem");
        let cap = cpm7_d(this + 0x34e4).wrapping_add(100);
        cpm7_por(this + 0x34e4, cap);
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realocar(cpm7_d(this + 0x34d8), cap.wrapping_mul(0x14) as u32);
        cpm7_por(this + 0x34d8, p);
        if p == 0 {
            show_error_at(0x9cd0a0, 0x797);
            return;
        }
    }
    crate::comparacao::ramo("cpm_7 amistoso planejado");
    let n = cpm7_d(this + 0x34e0);
    let item = cpm7_d(this + 0x34d8).wrapping_add(n.wrapping_mul(0x14));
    cpm7_por(item + 8, cpm7_d(anfitriao));
    cpm7_por(item + 0xc, cpm7_d(mandante)); // o mandante
    cpm7_por(item + 0x10, cpm7_d(visitante)); // o visitante
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut copia = [0u8; 8];
    copiar(item, &mut copia, data);
    cpm7_por(this + 0x34e0, n.wrapping_add(1)); // mais um amistoso
}

/// Comparador: pelo valor (as datas e a vaga só são lidas). Chamada pelas copas e ligas a cada data: sem contador.
unsafe extern "thiscall" fn comparar_is_comp_current_on_date(this: usize, comp: usize, data: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, comp, data);
    let n = is_comp_current_on_date(this, comp, data);
    PLACAR.registrar(r == n, || format!("is_comp_current_on_date({}, {:?}) original={} rust={}", cpm7_d(comp), friendly_date_day_year(data), r, n));
    r
}

/// Comparador: lado a lado pelo cursor da lista (+0xa) e o valor.
unsafe extern "thiscall" fn comparar_candidate_has_nearby_date(this: usize, clube: usize, data: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I1%%);
    let l = cpm7_d(clube + 0x12);
    let reg: &[(usize, usize)] = if l == 0 { &[] } else { &[(l + 0xa, 4)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, clube, data), || candidate_has_nearby_date(this, clube, data));
    PLACAR.registrar(igual && r == n, || format!("candidate_has_nearby_date({:?}) original={} rust={} {:x?} x {:x?}", friendly_date_day_year(data), r, n, mo, mr));
    r
}

/// Comparador: a original acrescenta (pode realocar, não roda duas vezes): o plano antes (a conta, os ids e a data),
/// a original roda, confere o item novo e a conta.
unsafe extern "thiscall" fn comparar_add_planned_friendly(this: usize, anfitriao: usize, mandante: usize, visitante: usize, data: usize) {
    let n = cpm7_d(this + 0x34e0);
    let esperado = [cpm7_d(data), cpm7_d(data + 4), 0, cpm7_d(anfitriao), cpm7_d(mandante), cpm7_d(visitante)];
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(%%I2%%)(this, anfitriao, mandante, visitante, data);
    let v = cpm7_d(this + 0x34d8);
    if v == 0 {
        return; // sem memória: a caixa já saiu
    }
    let item = v.wrapping_add(n.wrapping_mul(0x14));
    let feito = [cpm7_d(item), cpm7_d(item + 4), 0, cpm7_d(item + 8), cpm7_d(item + 0xc), cpm7_d(item + 0x10)];
    PLACAR.registrar(feito == esperado && cpm7_d(this + 0x34e0) == n.wrapping_add(1), || {
        format!("add_planned_friendly conta {} -> {}, item {:x?} (esperava {:x?})", n, cpm7_d(this + 0x34e0), feito, esperado)
    });
}
