// ---------- %%LEVA%%a leva: as fichas de treino iniciais dos jogadores, o banco de jogadores livres do regen e se há reserva de não jogador (training_manager.cpp, player_regen.cpp) ----------

unsafe fn cpu9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpu9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// { 0x8b3c50 } (thiscall, ret 0, training_manager.cpp): `TRAINING_MANAGER::init_player_entries()`, pelo construtor
/// (0x8b0ec0) num jogo novo: aloca (calloc 0x945501) as fichas de treino de 6 bytes, uma por ficha de jogador
/// ([0xae236c] fichas de 0x46 bytes em [0xae23c4], a ficha pelo id +0), em this + 0x651d; sem memória, a caixa (linha
/// 0x6e3) e volta. Em cada ficha apaga o bit 0x20 do +5 e põe a vaga do programa (+0): 6 (goleiros) com a nota de
/// goleiro (+0xf, com sinal) de 15 para cima, senão 1 (geral). A primeira atualização do treino de cada jogador
/// (0x8b1750, que só depois refaz a vaga dos jogadores do computador) e todo o treino do clube humano usam essa vaga.
unsafe extern "thiscall" fn init_player_entries(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let p = calloc(cpu9_d(0xae236c) as u32, 6);
    *((this + 0x651d) as *mut u32) = p as u32;
    if p == 0 {
        show_error_at(0xa8ccd0, 0x6e3);
        return;
    }
    let mut f = cpu9_d(0xae23c4);
    let mut i = 0i32;
    while i < cpu9_i(0xae236c) {
        let ficha = || cpu9_d(this + 0x651d).wrapping_add((cpu9_i(f) as usize).wrapping_mul(6));
        *((ficha() + 5) as *mut u8) &= 0xdf;
        let gol = *((f + 0xf) as *const i8);
        if gol >= 0xf { // cpu9: goleiro de 15 para cima
            crate::comparison::branch("cpu_9 goalkeeper");
            *(ficha() as *mut u8) = 6; // cpu9: o programa dos goleiros
        } else {
            crate::comparison::branch("cpu_9 outfield");
            *(ficha() as *mut u8) = 1; // cpu9: o programa geral
        }
        i += 1;
        f += 0x46;
    }
}

/// Comparador: a original aloca (não pode rodar duas vezes): roda só ela e confere cada ficha pela regra (a vaga 6
/// ou 1 pela nota de goleiro e o bit 0x20 do +5 apagado).
unsafe extern "thiscall" fn compare_init_player_entries(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this);
    let base = cpu9_d(this + 0x651d);
    if base == 0 {
        return;
    }
    let mut erradas = 0usize;
    let mut primeira = None;
    for k in 0..cpu9_i(0xae236c).max(0) as usize {
        let f = cpu9_d(0xae23c4) + 0x46 * k;
        let e = base.wrapping_add((cpu9_i(f) as usize).wrapping_mul(6));
        let vaga = if *((f + 0xf) as *const i8) >= 0xf { 6 } else { 1 };
        if *(e as *const u8) != vaga || *((e + 5) as *const u8) & 0x20 != 0 {
            erradas += 1;
            primeira.get_or_insert(k);
        }
    }
    SCOREBOARD.record(erradas == 0, || format!("init_player_entries: {} fichas fora da regra (a primeira {:?})", erradas, primeira));
}

/// { 0x7a8350 } (thiscall, ret 0xc, player_regen.cpp): `PLAYER_REGEN::add_player_data_to_pool(pessoa, nacao,
/// segunda)`: acrescenta ao banco de jogadores livres (16 bytes em [this+0x3c], conta +0x40, capacidade +0x44) o id da
/// ficha de jogador (+0x61) da pessoa de índice `pessoa` (registro de 0x6e bytes em [0xae23bc]) em +0, a nação em +4 e
/// a segunda nação em +8 (+0xc fica como está). Conta = capacidade − 1: cresce 250 (realloc 0x945667; sem memória,
/// caixa 0xa0a e fim). Pela process_retirements (0x7a59e0), quando um jogador que também tem ficha de não jogador
/// deixa de jogar.
unsafe extern "thiscall" fn add_player_data_to_pool(this: usize, pessoa: i32, nacao: u32, segunda: u32) {
    if cpk2_d(this + 0x40) == cpk2_d(this + 0x44).wrapping_sub(1) {
        let p = crt_realloc(cpk2_d(this + 0x3c), cpk2_d(this + 0x44).wrapping_add(250) << 4);
        cpk2_por(this + 0x3c, p);
        cpk2_por(this + 0x44, cpk2_d(this + 0x44).wrapping_add(250));
        if p == 0 {
            regen_out_of_memory(0xa0a);
        }
    }
    crate::comparison::branch("cpu_9 player bench");
    let registro = cpk2_d(0xae23bc).wrapping_add((pessoa as u32).wrapping_mul(0x6e) as usize);
    let item = cpk2_d(this + 0x3c).wrapping_add(cpk2_d(this + 0x40) << 4);
    cpk2_por(item, cpk2_d(cpk2_d(registro + 0x61)));
    cpk2_por(item + 4, nacao as usize);
    cpk2_por(item + 8, segunda as usize);
    cpk2_por(this + 0x40, cpk2_d(this + 0x40).wrapping_add(1));
}

/// Comparador: o id da ficha antes; a original acrescenta (pode realocar); confere a conta e o item novo.
unsafe extern "thiscall" fn compare_add_player_data_to_pool(this: usize, pessoa: i32, nacao: u32, segunda: u32) {
    let n = cpk2_d(this + 0x40);
    let id = cpk2_d(cpk2_d(cpk2_d(0xae23bc).wrapping_add((pessoa as u32).wrapping_mul(0x6e) as usize) + 0x61)) as u32;
    original_fn::<unsafe extern "thiscall" fn(usize, i32, u32, u32)>(%%I1%%)(this, pessoa, nacao, segunda);
    let item = cpk2_d(this + 0x3c).wrapping_add(n << 4);
    let g = [cpk2_d(item) as u32, cpk2_d(item + 4) as u32, cpk2_d(item + 8) as u32];
    SCOREBOARD.record(cpk2_d(this + 0x40) == n.wrapping_add(1) && g == [id, nacao, segunda], || {
        format!("add_player_data_to_pool({}, {}, {}) conta {} -> {}, item {:?} (id {})", pessoa, nacao, segunda, n, cpk2_d(this + 0x40), g, id)
    });
}

/// { 0x7a6e20 } (thiscall, ret 0, player_regen.cpp): `PLAYER_REGEN::has_spare_non_player_record()`: se a reserva de
/// fichas de não jogador (conta em +0x20) tem alguma. O eax leva os 24 bits altos da conta (setne no al), como na
/// original.
unsafe extern "thiscall" fn has_spare_non_player_record(this: usize) -> u32 {
    let c = cpu9_d(this + 0x20) as u32;
    (c & 0xffff_ff00) | (c != 0) as u32
}

/// Comparador: a devolução.
unsafe extern "thiscall" fn compare_has_spare_non_player_record(this: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    let n = has_spare_non_player_record(this);
    SCOREBOARD.record(r == n, || format!("has_spare_non_player_record({:x}) original={:x} rust={:x}", this, r, n));
    r
}
