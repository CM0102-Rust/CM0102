// ---------- %%LEVA%%a leva: os programas de treino padrão: os sete do computador e os dez de cada técnico humano no começo do jogo, o tipo do programa e o programa do humano por vaga (training_manager.cpp) ----------

unsafe fn cpu8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Os cinco níveis (+4..+8) de um programa de treino.
unsafe fn set_schedule_levels(this: usize, n: [u8; 5]) {
    for (k, v) in n.iter().enumerate() {
        *((this + 4 + k) as *mut u8) = *v;
    }
}

/// { 0x8b4e50 } (thiscall, ret 8, training_manager.cpp): `TRAINING_SCHEDULE::set_type(vaga, tipo)`: a vaga (com sinal,
/// em dword) em +0, o tipo em +0x15 e os cinco níveis das categorias (+4..+8) pela mistura do tipo (tabela de saltos
/// 0x8b4f08): 1 geral (25 nas quatro primeiras), 2 a 5 uma categoria com 50 e as outras com 10, 6 goleiros (50 na quinta,
/// 0 na terceira), 0 e 7 sem treino (tudo 0); fora de 0..=7 (o −1 da primeira vaga do humano) só a vaga e o tipo. Pela
/// set_default_schedules (0x8b3d40, 167 vezes no começo do jogo) e pelas telas do treino.
unsafe extern "thiscall" fn set_type(this: usize, vaga: i8, tipo: i8) {
    *(this as *mut i32) = vaga as i32;
    *((this + 0x15) as *mut i8) = tipo;
    match tipo {
        1 => {
            crate::comparacao::ramo("cpu_8 geral");
            set_schedule_levels(this, [0x19, 0x19, 0x19, 0x19, 0]); // cpu8: o treino geral
        }
        2 => set_schedule_levels(this, [0x32, 0xa, 0xa, 0xa, 0]),
        3 => set_schedule_levels(this, [0xa, 0xa, 0x32, 0xa, 0]),
        4 => set_schedule_levels(this, [0xa, 0xa, 0xa, 0x32, 0]),
        5 => set_schedule_levels(this, [0xa, 0x32, 0xa, 0xa, 0]),
        6 => {
            crate::comparacao::ramo("cpu_8 goleiros");
            set_schedule_levels(this, [0xa, 0xa, 0, 0xa, 0x32]); // cpu8: o treino dos goleiros
        }
        0 | 7 => set_schedule_levels(this, [0; 5]),
        _ => {}
    }
}

/// Comparador: lado a lado nos 0x16 bytes do programa (a vaga, os níveis e o tipo).
unsafe extern "thiscall" fn comparar_set_type(this: usize, vaga: i8, tipo: i8) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8, i8)>(%%I1%%);
    let regioes = [(this, 0x16)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, vaga, tipo), || set_type(this, vaga, tipo));
    PLACAR.registrar(igual, || format!("set_type({:x}, {}, {}) {:x?} x {:x?}", this, vaga, tipo, mo, mr));
}

/// O programa `vaga` (0..=9) do técnico humano `pessoa` como a 0x8b3d40 o calcula sem chamar a 0x8b3930 (vagas 0 a
/// 5): this + 0x43d + 0x9b × vaga + 0x60e × humano; fora dos 16 humanos, a caixa (linha 0x68f) e 0.
unsafe fn human_schedule_slot(this: usize, pessoa: usize, vaga: usize) -> usize {
    let v = cpu8_i(0xae2364).wrapping_sub(cpu8_i(pessoa)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    this + 0x43d + 0x9b * vaga + 0x60e * v as usize
}

/// { 0x8b3d40 } (thiscall, ret 0, training_manager.cpp): `TRAINING_MANAGER::set_default_schedules()`, pelo construtor
/// (0x8b0ec0) num jogo novo (sem training.dat): os sete programas do computador (0x9b bytes cada a partir de this; o
/// tipo k na vaga k: o computador treina pela vaga 1, geral, e pela 6, goleiros, conforme a ficha de treino do
/// jogador) e, para cada um dos 16 técnicos humanos (pessoa [0xae2364] − h − 1, registro de 0x6e bytes em
/// [0xae23bc]), os dez programas: vagas 0 a 5 calculadas aqui (tipos −1, 0, 1, 2, 3, 4), 6 a 9 pela 0x8b3930 (tipos
/// 5, 6, 7, 7). No fim os nomes padrão (0x8b2230). Chamadas pelo endereço (0x8b4e50 e 0x8b3930 nesta leva).
unsafe extern "thiscall" fn set_default_schedules(this: usize) {
    let tipo: unsafe extern "thiscall" fn(usize, i8, i8) = core::mem::transmute(0x8b4e50usize);
    let humano: unsafe extern "thiscall" fn(usize, usize, i8) -> usize = core::mem::transmute(0x8b3930usize);
    tipo(this, 0, 0);
    crate::comparacao::ramo("cpu_8 programas padrao");
    tipo(this + 0x9b, 1, 1); // cpu8: o programa geral do computador
    for k in 2..7i8 {
        tipo(this + 0x9b * k as usize, k, k);
    }
    for h in 0..0x10i32 {
        let id = cpu8_i(0xae2364).wrapping_sub(h).wrapping_sub(1);
        let pessoa = (*(0xae23bc as *const usize)).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        for (vaga, t) in [(0usize, -1i8), (1, 0), (2, 1), (3, 2), (4, 3), (5, 4)] {
            tipo(human_schedule_slot(this, pessoa, vaga), vaga as i8, t);
        }
        // ponytail: a original empurra a vaga 8 e 9 num dword só com o byte baixo gravado; as duas chamadas leem o byte.
        for (vaga, t) in [(6i8, 5i8), (7, 6), (8, 7), (9, 7)] {
            tipo(humano(this, pessoa, vaga), vaga, t);
        }
    }
    let nomes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8b2230usize);
    nomes(this);
}

/// Comparador: lado a lado nos 167 programas (0x651d bytes a partir de this; nomes incluídos) e no marcador de erro.
unsafe extern "thiscall" fn comparar_set_default_schedules(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let regioes = [(this, 0x651d), (0xb67a34, 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this), || set_default_schedules(this));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("set_default_schedules({:x}) diferencas em {:x?}", this, dif)
    });
}

/// { 0x8b3930 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_manager_schedule(tecnico, vaga)`: o
/// programa da vaga (0..=9; fora disso, 1) do técnico humano: this + 0x9b × (vaga + 10 × humano + 7), humano =
/// [0xae2364] − id − 1. Fora dos 16 humanos: a caixa (linha 0x68f) e 0. Pela 0x8b3d40 (vagas 6 a 9) e pelas telas.
unsafe extern "thiscall" fn get_manager_schedule(this: usize, tecnico: usize, vaga: i8) -> usize {
    let v = cpu8_i(0xae2364).wrapping_sub(cpu8_i(tecnico)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    let vaga = if (0..0xa).contains(&vaga) { vaga } else { 1 };
    this.wrapping_add(0x9b * (vaga as i32 + 10 * v + 7) as usize)
}

/// Comparador: a devolução, sem os casos de erro (só a original).
unsafe extern "thiscall" fn comparar_get_manager_schedule(this: usize, tecnico: usize, vaga: i8) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> usize>(%%I2%%);
    if !(0..0x10).contains(&cpu8_i(0xae2364).wrapping_sub(cpu8_i(tecnico)).wrapping_sub(1)) {
        return o(this, tecnico, vaga);
    }
    let r = o(this, tecnico, vaga);
    let n = get_manager_schedule(this, tecnico, vaga);
    PLACAR.registrar(r == n, || format!("get_manager_schedule(tecnico {:x}, vaga {}) original={:x} rust={:x}", tecnico, vaga, r, n));
    r
}
