// ---------- %%LEVA%%a leva: a comissão de treino do técnico humano: as notas dos treinadores por categoria, os programas do elenco na troca de clube e os nomes padrão dos programas (training_manager.cpp) ----------

unsafe fn cqf7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqf7_sb(a: usize) -> i8 {
    *(a as *const i8)
}

/// O registro da pessoa `id` (tabela [0xae23bc], 0x6e bytes).
unsafe fn training_manager_staff_by_id(id: i32) -> usize {
    cqf7_d(0xae23bc).wrapping_add((id as isize * 0x6e) as usize)
}

/// A vaga de 9 bytes da comissão (+0 id do treinador, +4..+8 as notas das cinco categorias) volta a vazia: −1, 0, 0.
unsafe fn clear_coach_slot(e: usize) {
    core::ptr::write_unaligned(e as *mut i32, -1);
    core::ptr::write_unaligned((e + 4) as *mut u32, 0);
    *((e + 8) as *mut u8) = 0;
}

/// Os divisores da nota pela quantidade de categorias do treinador (1 a 5), floats montados na pilha pela original:
/// 1,0; 1,2; 1,4; 1,6; 2,0.
const RATING_DIVISORS: [u32; 5] = [0x3f80_0000, 0x3f99_999a, 0x3fb3_3333, 0x3fcc_cccd, 0x4000_0000];

/// A chance de conhecer (0x544410, cdecl: a ficha de não jogador e o byte do atributo; a original empurra o
/// registrador com lixo em cima: a chamada lê o byte).
unsafe fn non_player_attribute_rating(f: usize, k: usize) -> i32 {
    let c: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    c(f, cqf7_sb(f + k)) as i8 as i32 // a original estende o al (movsx)
}

/// { 0x8b2880 } (thiscall, ret 8, training_manager.cpp): `TRAINING_MANAGER::update_coach_assignments(clube, técnico)`,
/// pelas contratações e saídas de treinadores (0x4ce4e0, 0x4cf720, 0x8c9720, a regeneração 0x7aaeb0) e pela troca de
/// clube (0x8b2470). Sem técnico e o clube sem humano (0x600a50 com 1, 1, 0, 0) ou clube de seleção (0x53b900): nada.
/// Técnico nulo: o do clube (+0xcf). A comissão do humano (0x8b4cd0, seis vagas de 9 bytes): (1) a vaga cujo
/// treinador não tem ficha de não jogador (+0x69) ou não é o auxiliar (0x53cb80) nem um dos 5 treinadores (0x53cc30)
/// fica vazia; (2) o auxiliar e os treinadores com ficha que não estão nas vagas entram na primeira vazia com as cinco
/// categorias (0x01010101 e 1); (3) cada categoria marcada recebe a nota: 0 (goleiros, ficha +0x10) (chance − ficha
/// +0x12 + 21) / 2; 1 (tática, +0x21 e +0x10) (chance + chance + +0x12) / 3; 2 (+0x10) (chance + +0xe + +0x12) / 3;
/// 3 (+0x10) (chance + +0x12) / 2; 4 (+0x11) a chance (divisões truncadas para zero; a chance 0x544410), o byte com
/// sinal dividido pelo divisor da quantidade de categorias marcadas (1,0 a 2,0, float), truncado e posto em 1..20.
unsafe extern "thiscall" fn update_coach_assignments(this: usize, clube: usize, tecnico: usize) {
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if tecnico == 0 && humano(clube, 1, 1, 0, 0) == 0 {
        return;
    }
    if selecao(clube) != 0 {
        return;
    }
    let tec = if tecnico == 0 { cqf7_d(clube + 0xcf) } else { tecnico };
    let comissao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8b4cd0usize);
    let auxiliar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53cb80usize);
    let treinadores: unsafe extern "C" fn(usize, *mut usize) -> u32 = core::mem::transmute(0x53cc30usize);
    let t = comissao(this, tec);
    let aux = auxiliar(clube);
    let mut tr = [0usize; 5];
    treinadores(clube, tr.as_mut_ptr());
    let candidato = |k: i32| if k < 0 { aux } else { tr[k as usize] };
    for v in 0..6 {
        let e = t + 9 * v;
        let id = cqf7_i(e);
        if id == -1 {
            continue;
        }
        if cqf7_d(training_manager_staff_by_id(id) + 0x69) == 0 {
            clear_coach_slot(e);
            continue;
        }
        if !(-1..5).any(|k| {
            let p = candidato(k);
            p != 0 && cqf7_i(p) == id
        }) {
            clear_coach_slot(e);
        }
    }
    for k in -1..5 {
        let p = candidato(k);
        if p == 0 || cqf7_d(p + 0x69) == 0 {
            continue;
        }
        if (0..6).any(|v| cqf7_i(t + 9 * v) == cqf7_i(p)) {
            continue;
        }
        if let Some(v) = (0..6).find(|&v| cqf7_i(t + 9 * v) == -1) {
            let e = t + 9 * v;
            core::ptr::write_unaligned(e as *mut i32, cqf7_i(p));
            core::ptr::write_unaligned((e + 4) as *mut u32, 0x0101_0101);
            *((e + 8) as *mut u8) = 1;
        }
    }
    for v in 0..6 {
        let e = t + 9 * v;
        let id = cqf7_i(e);
        if id == -1 {
            continue;
        }
        let f = cqf7_d(training_manager_staff_by_id(id) + 0x69);
        if f == 0 {
            clear_coach_slot(e);
            continue;
        }
        let n = (0..5).filter(|&j| *((e + 4 + j) as *const u8) != 0).count();
        for j in 0..5 {
            if *((e + 4 + j) as *const u8) == 0 {
                continue;
            }
            let b = |k: usize| cqf7_sb(f + k) as i32;
            let x = match j {
                0 => (non_player_attribute_rating(f, 0x10) - b(0x12) + 0x15) / 2, // cqf7: a nota de goleiros
                1 => {
                    let a = non_player_attribute_rating(f, 0x21);
                    (a + non_player_attribute_rating(f, 0x10) + b(0x12)) / 3 // cqf7: a nota de tática
                }
                2 => (non_player_attribute_rating(f, 0x10) + b(0xe) + b(0x12)) / 3,
                3 => (non_player_attribute_rating(f, 0x10) + b(0x12)) / 2,
                _ => non_player_attribute_rating(f, 0x11),
            };
            // o byte de baixo, com sinal, em float dividido pelo float da quantidade (precisão dupla do x87)
            let q = match_pl_ftol(x as i8 as f64 / f32::from_bits(RATING_DIVISORS[n - 1]) as f64) as i8; // cqf7: o divisor da quantidade
            crate::comparison::branch("cqf_7 nota do treinador");
            *((e + 4 + j) as *mut i8) = q.clamp(1, 20);
        }
    }
}

/// Comparador: lado a lado na comissão do humano (54 bytes) e na marca +0x244 do clube (a 0x53cb80/0x53cc30 pela
/// 0x540a50 podem zerá-la). Sem humano ou clube de seleção: lado a lado só na marca. Técnico fora dos 16 humanos
/// (a 0x8b4cd0 mostra a caixa e a original escreveria no nulo): só a original.
unsafe extern "thiscall" fn comparar_update_coach_assignments(this: usize, clube: usize, tecnico: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    if clube == 0 {
        return o(this, clube, tecnico);
    }
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let mut reg = vec![(clube + 0x244, 1)];
    if (tecnico != 0 || humano(clube, 1, 1, 0, 0) != 0) && selecao(clube) == 0 {
        let tec = if tecnico == 0 { cqf7_d(clube + 0xcf) } else { tecnico };
        if tec == 0 || !(0..0x10).contains(&cqf7_i(0xae2364).wrapping_sub(cqf7_i(tec)).wrapping_sub(1)) {
            return o(this, clube, tecnico);
        }
        reg.push((this + 0x652d + 54 * cqf7_i(0xae2364).wrapping_sub(cqf7_i(tec)).wrapping_sub(1) as usize, 54));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clube, tecnico), || update_coach_assignments(this, clube, tecnico));
    SCOREBOARD.record(igual, || format!("update_coach_assignments(clube {:x}, tecnico {:x}) {:x?} x {:x?}", clube, tecnico, mo.0, mr.0));
}

/// O programa `vaga` (0..9; fora disso o 1) do técnico humano `tec`: this + (vaga + 7 + 10 × ([0xae2364] − id − 1)) ×
/// 0x9b; fora dos 16 humanos, a caixa "Error" (linha 0x68f) e 0 (a original lê então o byte +0x15 do nulo).
unsafe fn human_training_schedule(this: usize, tec: usize, vaga: i8) -> usize {
    let k = cqf7_i(0xae2364).wrapping_sub(cqf7_i(tec)).wrapping_sub(1);
    if !(0..0x10).contains(&k) {
        show_error_at(0xa8ccd0, 0x68f);
        return 0;
    }
    let v = if (0..10).contains(&vaga) { vaga as i32 } else { 1 };
    this.wrapping_add(((v + 7 + k * 10) * 0x9b) as usize)
}

/// A entrada de 6 bytes do treino da ficha `f` (+0 a vaga do programa).
unsafe fn training_entry_of_player_data(this: usize, f: usize) -> usize {
    cqf7_d(this + 0x651d).wrapping_add((cqf7_i(f) as isize * 6) as usize)
}

/// { 0x8b2470 } (thiscall, ret 0xc, training_manager.cpp): `TRAINING_MANAGER::reassign_squad_schedules(técnico, antigo,
/// novo)`, quando o técnico humano troca de clube (0x690890 e 0x68f9c0/0x69ebc0, manager_manager.cpp). Elenco do clube
/// antigo (0x541f90, 50): cada pessoa válida (id abaixo de [0xae2364]) leva carga 50 (0x62c660) e a vaga 6 (goleiro de
/// 15 para cima, ficha +0xf) ou 1 na entrada do treino; sem ficha, a caixa (linha 0x34c). Com clube novo: dos dez
/// programas do técnico, o último de tipo 6 (+0x15) é o dos goleiros e o último de tipo 1 o geral; cada jogador do
/// elenco novo com ficha vai para o dos goleiros (goleiro de 15 para cima e havendo um), senão o geral (havendo), senão
/// a vaga 1, com a carga do programa (0x8b4f60) na ficha de forma; por fim a comissão (0x8b2880 com o clube novo e o
/// técnico).
unsafe extern "thiscall" fn reassign_squad_schedules(this: usize, tec: usize, antigo: usize, novo: usize) {
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let gravar_carga: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x62c660usize);
    let (mut gol, mut geral) = (-1i8, -1i8);
    if antigo != 0 {
        let mut v = [0usize; 50];
        elenco(antigo, v.as_mut_ptr(), 0);
        for &p in &v {
            if p == 0 || cqf7_i(p) >= cqf7_i(0xae2364) {
                continue;
            }
            gravar_carga(cqf7_d(0xb63d28), p, 0x32);
            let f = cqf7_d(p + 0x61);
            if f == 0 {
                show_error_at(0xa8ccd0, 0x34c);
                continue;
            }
            *(training_entry_of_player_data(this, f) as *mut u8) = if cqf7_sb(f + 0xf) >= 0xf { 6 } else { 1 };
        }
    }
    if novo == 0 {
        return;
    }
    for vaga in 0..10i8 {
        if *((human_training_schedule(this, tec, vaga) + 0x15) as *const u8) == 6 {
            gol = vaga;
        } else if *((human_training_schedule(this, tec, vaga) + 0x15) as *const u8) == 1 {
            geral = vaga;
        }
    }
    let mut v = [0usize; 50];
    elenco(novo, v.as_mut_ptr(), 0);
    let carga: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x8b4f60usize);
    for &p in &v {
        if p == 0 {
            continue;
        }
        let f = cqf7_d(p + 0x61);
        if f == 0 || cqf7_i(p) >= cqf7_i(0xae2364) {
            continue;
        }
        let e = training_entry_of_player_data(this, f) as *mut i8;
        *e = if cqf7_sb(f + 0xf) >= 0xf && gol != -1 {
            gol // cqf7: o programa dos goleiros
        } else if geral != -1 {
            geral
        } else {
            1
        };
        crate::comparison::branch("cqf_7 programa do elenco novo");
        let c = carga(human_training_schedule(this, tec, *e));
        gravar_carga(cqf7_d(0xb63d28), p, c as u32);
    }
    let comissao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x8b2880usize);
    comissao(this, novo, tec);
}

/// Comparador: lado a lado na entrada do treino e na carga de forma de cada jogador dos dois elencos, na comissão do
/// técnico (a 0x8b2880 do fim) e na marca +0x244 dos clubes (a 0x541f90 pela 0x540a50). Com caixa (jogador sem
/// ficha no elenco antigo ou técnico fora dos 16 humanos com clube novo): só a original.
unsafe extern "thiscall" fn comparar_reassign_squad_schedules(this: usize, tec: usize, antigo: usize, novo: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I1%%);
    let k = if tec == 0 { -1 } else { cqf7_i(0xae2364).wrapping_sub(cqf7_i(tec)).wrapping_sub(1) };
    if novo != 0 && !(0..0x10).contains(&k) {
        return o(this, tec, antigo, novo);
    }
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let les = cqf7_d(cqf7_d(0xb63d28));
    let mut reg = Vec::new();
    for (clube, antigo_) in [(antigo, true), (novo, false)] {
        if clube == 0 {
            continue;
        }
        reg.push((clube + 0x244, 1));
        let mut v = [0usize; 50];
        elenco(clube, v.as_mut_ptr(), 0);
        for &p in &v {
            if p == 0 || cqf7_i(p) >= cqf7_i(0xae2364) {
                continue;
            }
            let f = cqf7_d(p + 0x61);
            if f == 0 {
                if antigo_ {
                    return o(this, tec, antigo, novo);
                }
                continue;
            }
            reg.push((training_entry_of_player_data(this, f), 6));
            reg.push((les.wrapping_add((cqf7_i(p) as isize * 31 + 0x1e) as usize), 1));
        }
    }
    if novo != 0 {
        reg.push((this + 0x652d + 54 * k as usize, 54));
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, tec, antigo, novo), || reassign_squad_schedules(this, tec, antigo, novo));
    SCOREBOARD.record(igual, || format!("reassign_squad_schedules(tecnico {:x}, {:x} -> {:x}) {:x?} x {:x?}", tec, antigo, novo, mo.0, mr.0));
}

/// { 0x8b2230 } (thiscall, ret 0, training_manager.cpp): os nomes padrão dos 167 programas de treino (0x8b50f0 em cada
/// um, de 0x9b em 0x9b bytes), pela criação dos programas padrão (0x8b3930).
unsafe extern "thiscall" fn training_manager_cpp_0x8b2230(this: usize) {
    let nome: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8b50f0usize);
    for k in 0..0xa7 {
        nome(this + k * 0x9b);
    }
}

/// Comparador: lado a lado nos 167 programas (0x9b bytes cada).
unsafe extern "thiscall" fn comparar_training_manager_cpp_0x8b2230(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&[(this, 0xa7 * 0x9b)], || o(this), || training_manager_cpp_0x8b2230(this));
    SCOREBOARD.record(igual, || format!("training_manager_cpp_0x8b2230({:x})", this));
}
