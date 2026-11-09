// ---------- %%LEVA%%a leva: os registros das copas e da troca de técnico: o vice, o terceiro e a passagem do técnico pelo clube (staff_records.cpp) ----------

unsafe fn cqa12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa12_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cqa12_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

/// O ensaio dos comparadores: as cópias sem o histórico (arquivo) e sem acabar as convocações.
static mut CUP_PLACED_CLUB_DRY_RUN: bool = false;

/// A entrada `e` no fim da lista do clube `id` no arquivo do histórico dos clubes (FILE_LLIST [this+4], 0x58e340; o
/// começo e o fim em [this+0x10] + id × 8). No ensaio, nada.
unsafe fn append_club_history_by_id(this: usize, id: usize, e: &[u8; 0x40]) {
    if CUP_PLACED_CLUB_DRY_RUN {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(0x58e340usize);
    let p = cqa12_d(this + 0x10).wrapping_add(id.wrapping_mul(8));
    f(cqa12_d(this + 4), p, p + 4, e.as_ptr());
}

/// Uma nota do lado do clube na partida (palavra em `a`): soma ftol(peso × k × [0x96ff88]) (produto em 32 bits) e fica
/// no piso peso × k (16 bits) quando está abaixo dele, senão no máximo 10000 (comparações com sinal em 16 bits).
unsafe fn raise_side_rating(a: usize, peso: i16, k: i32) {
    let soma = match_pl_ftol((peso as i32).wrapping_mul(k) as f64 * read_f64(0x96ff88));
    let x = cqa12_w(a).wrapping_add(soma as i16);
    cqa12_pw(a, x);
    let piso = peso.wrapping_mul(k as i16);
    cqa12_pw(a, if piso > x { piso } else if x > 10000 { 10000 } else { x });
}

/// O vice (tipo 1: limiar 500, teto 350, notas 300/150) e o terceiro (tipo 2: 500, 200, 200/100; o limiar 500 conferido em 0x86b774) da copa: a reputação do
/// clube (+0x80) sobe até peso da competição (+0x69 de [jogo+0x14]) × limiar, no máximo `teto` de uma vez (entre 1 e
/// 10000); com técnico (+0xcf) que tem registro de não jogador (+0x69), as três notas do lado do clube na partida
/// (+0x188/+0x18a/+0x18c, lado 0x625 × visitante): peso × k, e na competição de fase 1 ([0xadadfc] +0x42 = 1) peso ×
/// metade na terceira e na segunda se o técnico for estrangeiro (0x53d980 com a nação +0x53 e 1). Os históricos (data,
/// tipo, id do clube, competição): do técnico, do presidente (+0xbf) e dos 20 da escalação do lado (+0x190, 0x49 bytes,
/// −1 vazio; pessoas válidas e fora o técnico); no terceiro, as convocações acabam nas competições guardadas (0x76d940,
/// 0x774f90 com os dois clubes); por fim o histórico do clube (tipo no +8, competição no +9).
unsafe fn update_cup_placed_club(this: usize, clube: usize, jogo: usize, partida: usize, tipo: u8) {
    let (limiar, teto, k, metade) = if tipo == 1 { (500, 350, 300, 150) } else { (500, 200, 200, 100) };
    let comp = cqa12_d(jogo + 0x14);
    let peso = cqa12_w(comp + 0x69);
    let alvo = (peso as i32).wrapping_mul(limiar); // cqa12: o limiar da reputação
    let rep = cqa12_w(clube + 0x80) as i32;
    if rep < alvo {
        let falta = alvo - rep;
        let soma = if falta > teto { teto } else { falta }; // cqa12: o teto da subida
        let v = cqa12_w(clube + 0x80).wrapping_add(soma as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if tipo == 1 {
            crate::comparison::branch("cqa_12 runner-up injury_word_0c");
        } else {
            crate::comparison::branch("cqa_12 third place injury_word_0c");
        }
        cqa12_pw(clube + 0x80, v); // cqa12: a reputação do colocado
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = tipo;
    put_i32(&mut e, 8, cqa12_i(clube));
    put_i32(&mut e, 0xd, cqa12_i(comp));
    let t = cqa12_d(clube + 0xcf);
    if t != 0 {
        if cqa12_d(t + 0x69) != 0 {
            let c = cqa12_d(cqa12_d(0xadadfc).wrapping_add(4 * cqa12_d(jogo)));
            let (k1, k2) = if *((c + 0x42) as *const i8) != 1 {
                (k, k)
            } else {
                let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
                (if estrangeiro(t, cqa12_d(clube + 0x53), 1) != 0 { metade } else { k }, metade)
            };
            let lado = partida + 0x188 + 0x625 * (cqa12_d(jogo + 0x20) == clube) as usize;
            raise_side_rating(lado, peso, k);
            raise_side_rating(lado + 2, peso, k1);
            raise_side_rating(lado + 4, peso, k2);
        }
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, cqa12_d(t), &e);
    }
    let presidente = cqa12_d(clube + 0xbf);
    if presidente != 0 {
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, cqa12_d(presidente), &e);
    }
    let escalacao = partida + 0x190 + 0x625 * (clube != cqa12_d(jogo + 0x1c)) as usize;
    for j in 0..0x14usize {
        let id = cqa12_i(escalacao + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = cqa12_d(0xae23bc).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        if p != 0 && cqa12_i(p) >= cqa12_i(0xae2364) {
            continue;
        }
        if p == cqa12_d(clube + 0xcf) {
            continue;
        }
        if p == 0 {
            show_error_at(0xa83b78, 0x1010);
            continue;
        }
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, cqa12_d(p), &e);
    }
    if tipo == 2 && !CUP_PLACED_CLUB_DRY_RUN {
        let guardada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x76d940usize);
        if guardada(cqa12_d(jogo + 0x14)) != 0 {
            let acabar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x774f90usize);
            acabar(cqa12_d(0xb74318), cqa12_d(jogo + 0x1c), cqa12_d(jogo + 0x14));
            acabar(cqa12_d(0xb74318), cqa12_d(jogo + 0x20), cqa12_d(jogo + 0x14));
        }
    }
    let mut e2 = history_entry();
    stamp_game_date(e2.as_mut_ptr());
    e2[8] = tipo;
    put_i32(&mut e2, 9, cqa12_i(comp));
    append_club_history_by_id(this, cqa12_d(clube), &e2);
}

/// { 0x86b0b0 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_cup_runners_up_record(clube, jogo, partida)`, pela final das copas (Cup 0x519890, Liga dos Campeões 0x584ae0 e
/// as copas de cada nação): `update_cup_placed_club` com o tipo 1. Clube, jogo ou partida nulos: a caixa (linha 0x5f6).
unsafe extern "thiscall" fn add_cup_runners_up_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return show_error_at(0xa83b78, 0x5f6);
    }
    update_cup_placed_club(this, clube, jogo, partida, 1);
}

/// { 0x86b710 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_cup_third_place_record(clube, jogo, partida)`, pelas copas com disputa de terceiro: `update_cup_placed_club` com o tipo
/// 2. Nulos: a caixa (linha 0x67b).
unsafe extern "thiscall" fn add_cup_third_place_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return show_error_at(0xa83b78, 0x67b);
    }
    update_cup_placed_club(this, clube, jogo, partida, 2);
}

/// Comparador em ensaio dos colocados: a cópia sem os históricos e sem acabar as convocações, e a original a partir do
/// mesmo estado (a reputação do clube e as três notas do lado dele na partida). Nulos: só a original.
unsafe fn compare_cup_placed(nome: &str, clube: usize, jogo: usize, partida: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return original();
    }
    let reg = [(clube + 0x80, 2), (partida + 0x188 + 0x625 * (cqa12_d(jogo + 0x20) == clube) as usize, 6)];
    let antes = crate::comparison::MemorySnapshot::read(&reg);
    CUP_PLACED_CLUB_DRY_RUN = true;
    copia();
    CUP_PLACED_CLUB_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    antes.write_back();
    original();
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || format!("{}(clube {:#x}, jogo {:#x}) original={:x?} rust={:x?}", nome, clube, jogo, mo, mr));
}

unsafe extern "thiscall" fn compare_add_cup_runners_up_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I0%%);
    compare_cup_placed("add_cup_runners_up_record", clube, jogo, partida, || o(this, clube, jogo, partida),
        || add_cup_runners_up_record(this, clube, jogo, partida))
}

unsafe extern "thiscall" fn compare_add_cup_third_place_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I1%%);
    compare_cup_placed("add_cup_third_place_record", clube, jogo, partida, || o(this, clube, jogo, partida),
        || add_cup_third_place_record(this, clube, jogo, partida))
}

/// A data em `src` copiada para `dst` (0x417c10; a cópia de retorno vai para uma data local).
unsafe fn staff_records_copy_date(dst: usize, src: usize) {
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copia(dst, t.as_mut_ptr(), src);
}

/// Os dias da data `a` menos a `b` (0x54cdb0, `b` por valor; grava a data estática 0xae2470).
unsafe fn days_between(a: usize, b: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    f(a, cqa12_d(b) as u32, cqa12_d(b + 4) as u32)
}

/// { 0x869aa0 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// record_manager_job_change(técnico, clube, contratado)`, pela saída do técnico (0x69b790) e pelo técnico novo
/// (0x694c80). Fora da seleção (0x53b900), no registro do técnico (0x86f5b0 com criar), quando o começo do jogo
/// ([0xdd6ec8]) não é depois de hoje: a passagem atual (+0x4a clube, +0x4e começo) fecha; a mais curta (+0x72, +0x76
/// começo, +0x7e fim) é trocada pela atual se não houver ou se a atual for mais curta (0x54cdb0), ou só tem o fim
/// renovado se for a mesma; a mais longa (+0x5e, +0x62, +0x6a) é trocada se a atual for mais longa (o fim vira o +0x56)
/// ou se não houver (o fim vira hoje). Depois +0x4a = −1 e, contratado = 1, a passagem nova (o id do clube, hoje,
/// +0x56 = 13/11/1975 da 0x549f70) e +1 em +0xb4. Na seleção, contratado = 1: +1 em +0xb5. Os históricos: do técnico
/// (tipo 0x96: clube, contratado), do presidente do clube (+0xbf; tipo 0x97 com o técnico e as três chaves) e do clube
/// (tipo 0x96 no +8, o técnico, as chaves e contratado). Técnico ou clube nulos: a caixa (linha 0x3b1).
unsafe extern "thiscall" fn record_manager_job_change(this: usize, tecnico: usize, clube: usize, contratado: u32) {
    if clube == 0 || tecnico == 0 {
        return show_error_at(0xa83b78, 0x3b1);
    }
    let contratado = contratado as u8 as i8;
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    if is_national_team(clube) == 0 {
        let r = registro(this, tecnico, 1);
        if r != 0 {
            let hoje = 0xae2c90usize;
            let (ai, ah) = (*((0xdd6ec8 + 2) as *const u16), *((hoje + 2) as *const u16));
            let depois = ai > ah || (ai == ah && cqa12_w(0xdd6ec8) > cqa12_w(hoje));
            if !depois {
                crate::comparison::branch("cqa_12 spell closed");
                if cqa12_i(r + 0x72) != -1 && cqa12_i(r + 0x4a) != -1 {
                    if cqa12_i(r + 0x72) == cqa12_i(r + 0x4a) && cqa12_w(r + 0x50) == cqa12_w(r + 0x78) && cqa12_w(r + 0x4e) == cqa12_w(r + 0x76) {
                        staff_records_copy_date(r + 0x7e, hoje);
                    } else {
                        let atual = days_between(hoje, r + 0x4e);
                        if atual < days_between(r + 0x7e, r + 0x76) {
                            cqa12_pi(r + 0x72, cqa12_i(r + 0x4a));
                            staff_records_copy_date(r + 0x76, r + 0x4e);
                            staff_records_copy_date(r + 0x7e, hoje);
                        }
                    }
                } else {
                    cqa12_pi(r + 0x72, cqa12_i(r + 0x4a));
                    staff_records_copy_date(r + 0x76, r + 0x4e);
                    staff_records_copy_date(r + 0x7e, hoje);
                }
                if cqa12_i(r + 0x5e) != -1 && cqa12_i(r + 0x4a) != -1 {
                    let atual = days_between(hoje, r + 0x4e);
                    if atual > days_between(r + 0x6a, r + 0x62) {
                        cqa12_pi(r + 0x5e, cqa12_i(r + 0x4a));
                        staff_records_copy_date(r + 0x62, r + 0x4e);
                        staff_records_copy_date(r + 0x6a, r + 0x56);
                    }
                } else {
                    cqa12_pi(r + 0x5e, cqa12_i(r + 0x4a));
                    staff_records_copy_date(r + 0x62, r + 0x4e);
                    staff_records_copy_date(r + 0x6a, hoje);
                }
            }
            cqa12_pi(r + 0x4a, -1);
            if contratado == 1 {
                crate::comparison::branch("cqa_12 new spell");
                cqa12_pi(r + 0x4a, cqa12_i(clube));
                staff_records_copy_date(r + 0x4e, hoje);
                let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
                let mut t = [0u8; 8];
                let d = data(t.as_mut_ptr(), 0xd, 0xb, 0x7b7, u32::MAX);
                staff_records_copy_date(r + 0x56, d);
                *((r + 0xb4) as *mut u8) = (*((r + 0xb4) as *const u8)).wrapping_add(1); // cqa12: as passagens
            }
        }
    } else if contratado == 1 {
        let r = registro(this, tecnico, 1);
        if r != 0 {
            *((r + 0xb5) as *mut u8) = (*((r + 0xb5) as *const u8)).wrapping_add(1);
        }
    }
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x96;
    put_i32(&mut e, 8, cqa12_i(clube));
    put_i32(&mut e, 0xd, contratado as i32);
    append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, cqa12_d(tecnico), &e);
    if cqa12_d(clube + 0xbf) != 0 {
        let mut e2 = history_entry();
        stamp_game_date(e2.as_mut_ptr());
        e2[0xc] = 0x97;
        put_i32(&mut e2, 8, cqa12_i(clube));
        put_i32(&mut e2, 0xd, contratado as i32);
        put_person_keys(&mut e2, 0x11, tecnico);
        append_person_history_by_id(!CUP_PLACED_CLUB_DRY_RUN, this, cqa12_d(cqa12_d(clube + 0xbf)), &e2);
    }
    let mut e3 = history_entry();
    stamp_game_date(e3.as_mut_ptr());
    e3[8] = 0x96;
    put_person_keys(&mut e3, 9, tecnico);
    put_i32(&mut e3, 0x19, contratado as i32);
    append_club_history_by_id(this, cqa12_d(clube), &e3);
}

/// Comparador em ensaio: o registro do técnico criado antes (0x86f5b0, como a original faria nos dois caminhos que o
/// pedem), a cópia sem os históricos e a original a partir do mesmo estado (o registro, 0xd2 bytes, e a data estática da
/// 0x54cdb0). Nulos: só a original.
unsafe extern "thiscall" fn compare_record_manager_job_change(this: usize, tecnico: usize, clube: usize, contratado: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I2%%);
    if clube == 0 || tecnico == 0 {
        return o(this, tecnico, clube, contratado);
    }
    let mut reg = vec![(0xae2470usize, 8usize)];
    if is_national_team(clube) == 0 || contratado as u8 == 1 {
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
        let r = registro(this, tecnico, 1);
        if r != 0 {
            reg.push((r, 0xd2));
        }
    }
    let antes = crate::comparison::MemorySnapshot::read(&reg);
    CUP_PLACED_CLUB_DRY_RUN = true;
    record_manager_job_change(this, tecnico, clube, contratado);
    CUP_PLACED_CLUB_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&reg);
    antes.write_back();
    o(this, tecnico, clube, contratado);
    let mo = crate::comparison::MemorySnapshot::read(&reg);
    SCOREBOARD.record(mo == mr, || format!("record_manager_job_change(tecnico {:#x}, clube {:#x}, {}) original={:x?} rust={:x?}", tecnico, clube, contratado as u8, mo, mr));
}
