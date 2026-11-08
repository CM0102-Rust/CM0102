// ---------- %%LEVA%%a leva: as posições previstas dos estaduais: Mineiro, Norte, Nordeste e Paranaense (a ordem pela força e os lugares do mata-mata) (bra_reg_minas_gerais.cpp, bra_reg_north.cpp, bra_reg_northeast.cpp, bra_reg_parana.cpp) ----------

unsafe fn cqv1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// Um estadual: o fonte, as linhas das duas caixas (sem estatísticas, sem memória), os lugares fixos dos primeiros da
/// ordem (o mata-mata) e o contador.
struct Cqv1Estadual {
    arquivo: usize,
    linhas: [u32; 2],
    lugares: &'static [u8],
    ramo: &'static str,
}

/// O lugar previsto do clube na vaga `i` da ordem: os fixos do mata-mata e, depois deles, i + 1.
fn cqv1_lugar(e: &Cqv1Estadual, i: usize) -> u32 {
    if i < e.lugares.len() { e.lugares[i] as u32 } else { i as u32 + 1 }
}

/// As posições previstas de um estadual (o mesmo molde de 336 a 432 bytes nos doze, conferido instrução a instrução:
/// só mudam o fonte, as linhas e os lugares fixos do mata-mata). Sem estatísticas (+8): a caixa da primeira linha e
/// volta. Os clubes das +0x3e linhas da tabela (+0xb1, 0x3b bytes cada) vão para um vetor (malloc 0x944e46; sem memória:
/// a caixa da segunda linha e volta), ordenado pela força (0x4c08f0, cdecl, a maior primeiro); os primeiros ganham os
/// lugares fixos (sem olhar quantos clubes há) e os seguintes, enquanto houver clube, o lugar i + 1
/// (Comp_Stats::set_club_final_position 0x4a2540, thiscall(+8, clube, lugar), ret 8; a original empurra o lugar limpo);
/// o vetor é liberado (free 0x9452ca).
unsafe fn cqv1_previstas(this: usize, e: &Cqv1Estadual) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x4c08f0usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    if cqv1_d(this + 8) == 0 {
        return show_error_at(e.arquivo, e.linhas[0]);
    }
    let n = core::ptr::read_unaligned((this + 0x3e) as *const i16);
    let v = alocar(((n as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(e.arquivo, e.linhas[1]);
    }
    let mut i = 0i16;
    while i < n {
        core::ptr::write_unaligned((v + 4 * i as usize) as *mut usize, cqv1_d(cqv1_d(this + 0xb1) + 0x3b * i as usize));
        i += 1;
    }
    ordenar(v, n as i32); // cqv1: a ordem pela força
    for (k, &l) in e.lugares.iter().enumerate() {
        gravar(cqv1_d(this + 8), cqv1_d(v + 4 * k), l as u32);
    }
    crate::comparison::branch(e.ramo);
    let mut k = e.lugares.len() as i32 + 1;
    while k <= core::ptr::read_unaligned((this + 0x3e) as *const i16) as i32 {
        gravar(cqv1_d(this + 8), cqv1_d(v + 4 * (k - 1) as usize), k as u32);
        k += 1;
    }
    liberar(v);
}

/// O comparador: só a original (a 0x4a2540 acrescenta o registro de um clube que não tem; não pode rodar duas vezes),
/// com o plano feito antes: os clubes da tabela ordenados pelo núcleo da 0x4c08f0 (sort_clubs_by_strength); depois, o
/// registro de cada clube (a última vez que ele aparece) tem de ter em +0x72 o lugar previsto.
unsafe fn cqv1_comparar(nome: &str, this: usize, e: &Cqv1Estadual, original: impl FnOnce()) {
    let n = core::ptr::read_unaligned((this + 0x3e) as *const i16) as i32;
    if cqv1_d(this + 8) == 0 || n < e.lugares.len() as i32 {
        return original(); // ponytail: a caixa, ou menos clubes que os lugares fixos (a original lê além do vetor): só a original
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| cqv1_d(cqv1_d(this + 0xb1) + 0x3b * i) as u32).collect();
    let mut k = vec![0i32; n as usize];
    sort_clubs_by_strength(v.as_mut_ptr(), k.as_mut_ptr(), n);
    original();
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut erros = Vec::new();
    for (i, &c) in v.iter().enumerate() {
        if c == 0 || v[i + 1..].contains(&c) {
            continue;
        }
        let p = cqv1_lugar(e, i) as u8;
        let r = buscar(cqv1_d(this + 8), c as usize);
        if r == 0 || *((r + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if r == 0 { -1 } else { *((r + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.record(erros.is_empty(), || format!("{}({:#x}): {}", nome, this, erros.join("; ")));
}

const CQV1_MINEIRO: Cqv1Estadual = Cqv1Estadual { arquivo: 0x98aed0, linhas: [0x33f, 0x347], lugares: &[1, 2], ramo: "cqv_1 mineiro" }; // cqv1: os lugares do Mineiro

/// { 0x434f80 } (thiscall, ret 0, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::set_predicted_positions()`: as posições previstas do Mineiro (os lugares fixos 1, 2).
unsafe extern "thiscall" fn cqv1_previstas_mineiro(this: usize) {
    cqv1_previstas(this, &CQV1_MINEIRO)
}

unsafe extern "thiscall" fn comparar_cqv1_previstas_mineiro(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    cqv1_comparar("cqv1_previstas_mineiro", this, &CQV1_MINEIRO, || o(this));
}

const CQV1_NORTE: Cqv1Estadual = Cqv1Estadual { arquivo: 0x98af14, linhas: [0x307, 0x30f], lugares: &[1, 2, 3, 3], ramo: "cqv_1 norte" }; // cqv1: os lugares do Norte

/// { 0x436280 } (thiscall, ret 0, bra_reg_north.cpp): `BraRegNorth::set_predicted_positions()`: as posições previstas do Norte (os lugares fixos 1, 2, 3, 3).
unsafe extern "thiscall" fn cqv1_previstas_norte(this: usize) {
    cqv1_previstas(this, &CQV1_NORTE)
}

unsafe extern "thiscall" fn comparar_cqv1_previstas_norte(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqv1_comparar("cqv1_previstas_norte", this, &CQV1_NORTE, || o(this));
}

const CQV1_NORDESTE: Cqv1Estadual = Cqv1Estadual { arquivo: 0x98af54, linhas: [0x2f9, 0x301], lugares: &[1, 2, 3, 3], ramo: "cqv_1 nordeste" }; // cqv1: os lugares do Nordeste

/// { 0x437290 } (thiscall, ret 0, bra_reg_northeast.cpp): `BraRegNortheast::set_predicted_positions()`: as posições previstas do Nordeste (os lugares fixos 1, 2, 3, 3).
unsafe extern "thiscall" fn cqv1_previstas_nordeste(this: usize) {
    cqv1_previstas(this, &CQV1_NORDESTE)
}

unsafe extern "thiscall" fn comparar_cqv1_previstas_nordeste(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    cqv1_comparar("cqv1_previstas_nordeste", this, &CQV1_NORDESTE, || o(this));
}

const CQV1_PARANAENSE: Cqv1Estadual = Cqv1Estadual { arquivo: 0x98af98, linhas: [0x33f, 0x347], lugares: &[1, 2], ramo: "cqv_1 paranaense" }; // cqv1: os lugares do Paranaense

/// { 0x438590 } (thiscall, ret 0, bra_reg_parana.cpp): `BraRegParana::set_predicted_positions()`: as posições previstas do Paranaense (os lugares fixos 1, 2).
unsafe extern "thiscall" fn cqv1_previstas_paranaense(this: usize) {
    cqv1_previstas(this, &CQV1_PARANAENSE)
}

unsafe extern "thiscall" fn comparar_cqv1_previstas_paranaense(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%);
    cqv1_comparar("cqv1_previstas_paranaense", this, &CQV1_PARANAENSE, || o(this));
}
