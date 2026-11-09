// ---------- %%LEVA%%a leva: a força dos treinadores no treino: o fator do treinador por categoria, a comissão de treino do técnico humano e a carga do programa de treino (training_manager.cpp) ----------

unsafe fn cpt8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpt8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Os seis divisores da média dos treinadores (floats imediatos da 0x8b41d0: 1; 1,45; 1,7; 1,9; 2,05; 2,2), pela
/// quantidade de treinadores (1 a 6).
fn coach_average_divisor(n: i8) -> f64 {
    const T: [u32; 6] = [0x3f800000, 0x3fb9999a, 0x3fd9999a, 0x3ff33333, 0x40033333, 0x400ccccd];
    f32::from_bits(T[(n - 1) as usize]) as f64
}

/// { 0x8b41d0 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::get_coaching_factor(clube, categoria)`,
/// pela TRAINING_MANAGER::update_player_training (0x8b1750, cinco vezes por jogador em cada atualização do treino; o
/// valor entra como (fator − 100) / 5 no peso do sorteio do progresso). Clube nulo: 0. Sem técnico (+0xcf) ou com técnico
/// do computador (id abaixo de [0xae2364] − 16): 100 (o eax leva os 16 bits altos do ponteiro do técnico, como na
/// original). Técnico humano: a comissão dele (0x8b4cd0, seis vagas de 9 bytes: id −1 é vaga vazia, a nota da categoria
/// em +4 + categoria); com n treinadores de nota, a soma (16 bits) dividida pelo n-ésimo divisor e truncada (_ftol
/// 0x945640), × 40/7 (0x970400) e truncada; abaixo de 50, 50. Quente no ramo do computador: sem contador ali.
unsafe extern "thiscall" fn get_coaching_factor(this: usize, clube: usize, categoria: i8) -> u32 {
    let _ = this;
    if clube == 0 {
        return 0;
    }
    let t = cpt8_d(clube + 0xcf);
    if t == 0 {
        crate::comparison::branch("cpt_8 without manager");
        return 100; // cpt8: sem técnico
    }
    if cpt8_i(t) < cpt8_i(0xae2364).wrapping_sub(0x10) {
        return (t as u32 & 0xffff_0000) | 100; // cpt8: técnico do computador
    }
    let comissao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8b4cd0usize);
    let s = comissao(this, t);
    let (mut n, mut soma) = (0i8, 0i16);
    for k in 0..6usize {
        let e = s + 9 * k;
        if cpt8_i(e) == -1 {
            continue;
        }
        let nota = *((e as isize + 4 + categoria as isize) as *const i8);
        if nota != 0 {
            n += 1;
            soma = soma.wrapping_add(nota as i16);
        }
    }
    if n != 0 {
        crate::comparison::branch("cpt_8 coaches");
        soma = match_pl_ftol(soma as f64 / coach_average_divisor(n)) as i16; // cpt8: a média dos treinadores
    }
    let r = match_pl_ftol(soma as f64 * read_f64(0x970400));
    if (r as i16) < 0x32 {
        return 0x32;
    }
    r as u32
}

/// Comparador: a devolução inteira (a consulta só lê; a comissão do humano é lida pelas duas).
unsafe extern "thiscall" fn compare_get_coaching_factor(this: usize, clube: usize, categoria: i8) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, i8) -> u32>(%%I0%%)(this, clube, categoria);
    let n = get_coaching_factor(this, clube, categoria);
    SCOREBOARD.record(r == n, || format!("get_coaching_factor(clube {:x}, categoria {}) original={:x} rust={:x}", clube, categoria, r, n));
    r
}

/// { 0x8b4cd0 } (thiscall, ret 4, training_manager.cpp): `TRAINING_MANAGER::get_coach_assignments(tecnico)`: a comissão
/// de treino (seis vagas de 9 bytes) de um dos 16 técnicos humanos, em this + 0x652d + 54 × ([0xae2364] − id − 1).
/// Técnico nulo: a caixa "Error" (linha 0xa20) e 0; fora dos 16 humanos: a caixa (linha 0xa2a) e 0.
unsafe extern "thiscall" fn get_coach_assignments(this: usize, tecnico: usize) -> usize {
    if tecnico == 0 {
        show_error_at(0xa8ccd0, 0xa20);
        return 0;
    }
    let v = cpt8_i(0xae2364).wrapping_sub(cpt8_i(tecnico)).wrapping_sub(1);
    if !(0..0x10).contains(&v) {
        show_error_at(0xa8ccd0, 0xa2a);
        return 0;
    }
    this + 0x652d + 54 * v as usize
}

/// Comparador: a devolução, sem os casos de erro (só a original).
unsafe extern "thiscall" fn compare_get_coach_assignments(this: usize, tecnico: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    if tecnico == 0 || !(0..0x10).contains(&cpt8_i(0xae2364).wrapping_sub(cpt8_i(tecnico)).wrapping_sub(1)) {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = get_coach_assignments(this, tecnico);
    SCOREBOARD.record(r == n, || format!("get_coach_assignments(tecnico {:x}) original={:x} rust={:x}", tecnico, r, n));
    r
}

/// { 0x8b4f60 } (thiscall, ret 0, training_manager.cpp): `TRAINING_SCHEDULE::get_training_load()`: a carga do
/// programa pelos cinco níveis (+4..+8, com sinal): trunc(+8 × 0,5) − trunc(+4 × −1,3) − trunc(+7 × −0,5) −
/// trunc(+6 × −0,5) − trunc(+5 × −0,5) (constantes 0x968968, 0x96f878, 0x969540; _ftol 0x945640), em 16 bits; de 100
/// para cima, 100. Pela update_player_training (0x8b1750, o jogador do clube humano), pela set_player_schedule
/// (0x8b2250) e pela reassign_squad_schedules (0x8b2470); vai para a ficha de forma (Injury::set_training_load 0x62c660).
unsafe extern "thiscall" fn get_training_load(this: usize) -> i32 {
    let nivel = |k: usize| *((this + k) as *const i8) as f64;
    let a = match_pl_ftol(nivel(4) * read_f64(0x96f878)) as i16;
    let mut s = match_pl_ftol(nivel(8) * read_f64(0x968968)) as i16;
    s = s.wrapping_sub(a);
    s = s.wrapping_sub(match_pl_ftol(nivel(7) * read_f64(0x969540)) as i16);
    s = s.wrapping_sub(match_pl_ftol(nivel(6) * read_f64(0x969540)) as i16);
    s = s.wrapping_sub(match_pl_ftol(nivel(5) * read_f64(0x969540)) as i16);
    if s >= 0x64 {
        return 0x64;
    }
    s as i32
}

/// Comparador: a devolução.
unsafe extern "thiscall" fn compare_get_training_load(this: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%)(this);
    let n = get_training_load(this);
    SCOREBOARD.record(r == n, || format!("get_training_load({:x}) original={} rust={}", this, r, n));
    r
}
