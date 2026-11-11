use super::*;

/// Unidades da fita da lesao: a 0x68d0d0 recebe o registro montado na pilha (confere o conteudo).
pub(crate) static RECORD_INJURY: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// True while the injure comparison is recording calls on its tape (RECORD_INJURY flag set and tape recording).
pub(crate) fn is_recording_injury() -> bool {
    RECORD_INJURY.load(std::sync::atomic::Ordering::Relaxed) && is_recording()
}

/// { `register_injury` } (ret 0xc, injury.cpp) with the list copy injected (0x68d0d0): records the player sheet's injury in the
/// object's injury list (+0x28), returning the resulting type and severity.
///
/// With no list, the original runs (it creates it). Player sheet already in the list (by id): the same type adds to the
/// severity (+5, at most 100); another type keeps the more serious one. New player sheet: the record (id, type,
/// severity) goes into the list. Returns in `tipo` and `chance` what remained.
///
/// [pt] { 0x62be00 } (ret 0xc, injury.cpp) com a copia na lista injetada (0x68d0d0): registra a lesao da
/// ficha na lista de lesoes do objeto (+0x28; sem lista, a original, que a cria). Ficha ja na lista
/// (pelo id): mesmo tipo soma a gravidade (+5, no maximo 100); outro tipo fica o mais grave. Ficha nova:
/// o registro (id, tipo, gravidade) vai para a lista. Devolve em `tipo` e `chance` o que ficou.
pub(crate) unsafe fn register_injury_with(this: usize, sheet: usize, kind: *mut u8, chance: *mut u8, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let l = Llist(read_usize(this + injury_fields::MATCH_HIT_LIST));
    if l.is_null() {
        crate::comparison::branch("injury new list");
        let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(IDX_REGISTER_INJURY);
        return o(this, sheet, kind, chance);
    }
    l.set_cursor(((l.head() as u32)) as usize);
    let id = read_u32(sheet);
    let mut e = exe::llist_get_next(l);
    while e != 0 && *(e as *const u32) != id {
        e = exe::llist_get_next(l);
    }
    if e == 0 {
        crate::comparison::branch("injury recorded");
        let mut r = [0u8; 6];
        r[..4].copy_from_slice(&id.to_le_bytes());
        r[4] = *kind;
        r[5] = *chance;
        request_buffers(&[(r.as_ptr() as usize, 6)], &[]);
        call(FN_LLIST_ADD_TAIL, l.addr(), [0, 6, 0, 0, 0]);
        return;
    }
    let (t, g) = ((e + 4) as *mut u8, (e + 5) as *mut u8);
    if *t == *kind {
        crate::comparison::branch("injury worsened");
        *g = if *chance as i8 as i32 + *g as i8 as i32 > 0x64 { 0x64 } else { (*chance).wrapping_add(*g) };
    } else if *chance as i8 > *g as i8 {
        crate::comparison::branch("injury more serious");
        *g = *chance;
        *t = *kind;
    }
    *kind = *t;
    *chance = *g;
}

/// Replays a recorded list-insert call (0x68d0d0) with the last requested block as the item.
pub(crate) unsafe fn dispatch_injury_list_call(routine: usize, this: usize, a: [u32; 5]) -> u32 {
    // 0x68d0d0: the block requested last (request_buffers) is the one that goes to the list
    let p = REQUESTED_BUFFERS.lock().unwrap_or_else(|e| e.into_inner()).take().map_or(0, |v| v.0[0].0);
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(routine);
    f(this, p, a[1]);
    0
}

/// the record's injury in the injury list (+0x28 of [0xb63d28]): same injury adds the severity (up to 100), another
/// keeps the worse; a new record goes to the list (0x68d0d0, the record built on the stack checked by the tape). No
/// list: the original (creates the list)
/// Original: `register_injury` (0x62be00, injury.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x62be00 }: ver `register_injury_with`.
pub(crate) unsafe extern "thiscall" fn register_injury(this: usize, sheet: usize, kind: *mut u8, chance: *mut u8) {
    register_injury_with(this, sheet, kind, chance, &mut |r, t, a| dispatch_injury_list_call(r, t, a))
}

/// A player's injury during the match: `0x6d0980` (ret 0x10, `match_eng.cpp`) with the copy into the injury list
/// (`0x68d0d0`) injected; decides and applies the injury of player `p` (fitness loss, leaving the pitch, counters).
///
/// With more than 3 injuries in the match (`+0xf639`), non-zero random draw(injuries) and random draw(500) prevent it;
/// with more than 6, never. A valid type: records it (`0x62be00`, which returns type and severity). `+0x55` = `o` (the
/// player who caused it loses 10 in `+0x31`).
///
/// Severity above 50: removes severity^2 from stamina (`+0x4d`) and marks `+0x1c`; otherwise removes random
/// draw(severity^2 / 2), writes random draw(that × 0.5) into `+0x59` and halves `+0xd5`.
///
/// Stamina below the limit: he leaves (`+0x8eb5`, `+0x1d`, `+0x1c`); otherwise he stops (`0x7160e0` with 5 or 3)
/// according to the person's strength (`+0x2d` × 25). Stamina is kept between 1 and the ceiling; when leaving, one more
/// injury in the match.
///
/// [pt] { 0x6d0980 } (ret 0x10, match_eng.cpp) com a copia na lista de lesoes injetada (0x68d0d0): a lesao
/// do jogador `p` na partida. Com mais de 3 lesoes no jogo (+0xf639), sorteio(lesoes) e sorteio(500)
/// nao nulos a evitam; mais de 6, nunca. Tipo valido: registra (0x62be00, que devolve tipo e
/// gravidade). +0x55 = `o` (o causador perde 10 em +0x31). Gravidade acima de 50: tira gravidade^2 do
/// folego (+0x4d) e marca +0x1c; senao tira sorteio(gravidade^2 / 2), grava sorteio(isso x 0,5) em
/// +0x59 e reduz +0xd5 a metade. Folego abaixo do limite: sai (+0x8eb5, +0x1d, +0x1c); senao para
/// (0x7160e0 com 5 ou 3) conforme a forca da pessoa (+0x2d x 25). Folego entre 1 e o teto; saindo,
/// mais uma lesao no jogo.
pub(crate) unsafe fn injure_with(m: Engine, kind: u32, chance: u32, p: MatchPlayer, o: usize, call: &mut dyn FnMut(usize, usize, [u32; 5]) -> u32) {
    let n = m.injury_count();
    if n > 3 && random(n as i32) != 0 && random(0x1f4) != 0 {
        return;
    }
    if m.injury_count() > 6 {
        return;
    }
    let (mut t, mut g) = (kind as u8, chance as u8);
    if t as i8 > -1 {
        register_injury_with(read_usize(G_INJURY), p.staff().addr(), &mut t, &mut g, call);
    }
    p.set_injury_culprit(o);
    if o != 0 {
        let q = (o + match_player::MATCH_TEMPER) as *mut i16;
        *q = (*q).wrapping_sub(10);
    }
    let stamina = (p.addr() + match_player::STAMINA) as *mut f32;
    let c = g as i8 as i32;
    let mut local = 0f32;
    if g as i8 > 0x32 {
        crate::comparison::branch("injury serious");
        p.set_injury_forces_off((1) as i8);
        *stamina = (*stamina as f64 - (c * c) as f64) as f32;
    } else {
        local = random(ftol((c * c) as f64 * read_f64(rdata::F64_0_5))) as f32;
        *stamina = (*stamina as f64 - local as f64) as f32;
        let r = random(ftol(local as f64 * read_f32(rdata::F32_0_5) as f64));
        p.set_stamina_boost(r as f32);
        let d5 = (p.addr() + match_player::STAMINA_ATTRIBUTE) as *mut f32;
        *d5 = (*d5 as f64 * read_f32(rdata::F32_0_5) as f64) as f32;
    }
    if !(*stamina as f64 >= read_f64(rdata::F64_5000)) {
        crate::comparison::branch("injury out of breath");
        m.set_substitution_pending((1) as i8);
        p.set_injury_pending((1) as i8);
        p.set_injury_forces_off((1) as i8);
    } else {
        let person = p.player();
        let v = (person.injury_proneness() as i32 * 25) as f64 + read_f64(rdata::F64_7100);
        if v > *stamina as f64 {
            crate::comparison::branch("injury to 5");
            write_2b(p.addr(), 5);
            m.set_substitution_pending((1) as i8);
            p.set_injury_pending((1) as i8);
        } else {
            write_2b(p.addr(), 3);
            if local >= read_f32(rdata::F32_500) && (p.injury_forces_off() as u8) != 0 {
                crate::comparison::branch("injury to 3 and leaves");
                m.set_substitution_pending((1) as i8);
                p.set_injury_pending((1) as i8);
            }
        }
    }
    let x = *stamina as f64;
    if !(x >= read_f64(rdata::F64_1)) {
        *stamina = read_f64(rdata::F64_1) as f32;
    } else if x > read_f64(rdata::F64_10000) {
        *stamina = read_f64(rdata::F64_10000) as f32;
    }
    if (m.substitution_pending() as u8) != 0 {
        m.set_injury_count((m.injury_count()).wrapping_add(1));
    }
}

/// Memory regions of the injury list (G_INJURY +0x28): pointer, header, nodes and 6-byte records, for the comparer.
///
/// [pt] As regioes da lesao: o ponteiro da lista de lesoes, o cabecalho, os nos (o ultimo ganha o
/// proximo quando a original acrescenta) e os registros de 6 bytes.
pub(crate) unsafe fn injury_list_regions() -> Vec<(usize, usize)> {
    let mut reg = Vec::new();
    let owner = read_usize(G_INJURY);
    let list = if owner != 0 { read_usize(owner + injury_fields::MATCH_HIT_LIST) } else { 0 };
    if list != 0 {
        reg.extend([(owner + injury_fields::MATCH_HIT_LIST, 4), (list, 0xe)]);
        let mut in_the = read_usize(list + 2);
        for _ in 0..read_i16(list).max(0) {
            if in_the == 0 {
                break;
            }
            reg.push((in_the, 0xa));
            let e = read_usize(in_the);
            if e != 0 {
                reg.push((e, 6));
            }
            in_the = read_usize(in_the + 6);
        }
    }
    reg
}

/// Comparer for injure: runs the original and injure_with with call tapes over the engine, RNG and injury list regions
/// and records the result.
pub(crate) unsafe fn compare_injure_with_tape(m: usize, kind: u32, chance: u32, p: usize, o: usize) {
    use std::sync::atomic::Ordering::Relaxed;
    let f = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize)>(IDX_INJURE);
    let owner = read_usize(G_INJURY);
    if owner == 0 || read_usize(owner + injury_fields::MATCH_HIT_LIST) == 0 {
        // without a list, the original routine creates it (allocates): left without comparison
        return f(m, kind, chance, p, o);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(m, ENGINE_SIZE), (G_MOUSE_LEFT_BUTTON_DOWN, 4)]);
    if o != 0 {
        reg.push((o + match_player::MATCH_TEMPER, 2));
    }
    reg.extend(injury_list_regions());
    let write = RECORD_INJURY.swap(true, Relaxed);
    let (_, _, igual, sequencia, k) = compare_com_fita(reg, || f(m, kind, chance, p, o), |chamar| {
        injure_with(Engine(m), kind, chance, MatchPlayer(p), o, &mut |rotina, this, args| chamar(rotina, this, args, [0, u32::MAX, 0, 0, 0]))
    });
    RECORD_INJURY.store(write, Relaxed);
    SCOREBOARD.record(igual && sequencia, || format!("injure(kind {}, chance {}) {} chamadas: {}", kind as u8 as i8, chance as u8 as i8, k, tape_detail()));
}

/// can the player earn highlight `codigo` (0x1f40..0x21e4)? Not if the side already had it (table at engine +0x21e) or
/// the record is from outside; 0x219e..0x21b8 with rules on performance, goals, cards, time on the pitch and the
/// history search (`has_logged_story`, last 7 or 30 days). Compared side by side
/// Original: `player_highlight` (0x717470, thiscall, ret 4).
///
/// [pt] { 0x717470 } (ret 4): o jogador pode ganhar o destaque `codigo` (0x1f40..=0x21e4) nesta partida?
/// Nao se o lado ja o teve (tabela de dwords em +0x21e do motor, 4000 bytes por lado) ou se a ficha
/// (+0x69) e de fora da base ([0xae2364]). Os codigos 0x219e..=0x21b8 tem regras proprias (atuacao
/// +0x35 contra um sorteio ou contra o tempo em campo, gols +0xc, cartoes, e a busca no historico
/// 0x757da0 nos ultimos 7 ou 30 dias, datas da 0x54c8e0); os demais da faixa, sim.
pub(crate) unsafe extern "thiscall" fn player_highlight(p: usize, code: u32) -> u32 {
    let p = MatchPlayer(p);
    let m = p.engine();
    let today = G_CURRENT_DATE;
    let (mut d7, mut d30) = ([0u8; 8], [0u8; 8]);
    exe::date_sub_days(today, (&mut d7) as *mut [u8; 8] as usize, 7);
    exe::date_sub_days(today, (&mut d30) as *mut [u8; 8] as usize, 0x1e);
    let c = code as u16 as i16 as i32;
    if !(0x1f40..=0x21e4).contains(&c) {
        return 0;
    }
    let side = p.side() as i32;
    if *((m.addr() as isize + ((c + side * 1000) * 4) as isize - 0x7ae2) as *const u32) != 0 {
        return 0;
    }
    let sheet = p.staff();
    if !sheet.is_null() && *(sheet.addr() as *const i32) >= *(G_STAFF_COUNT as *const i32) {
        return 0;
    }
    let performance = p.match_rating() as i32;
    let b = |o: usize| read_i8(p.addr() + o);
    let minute = m.clock() as i32 / 11;
    let in_field = minute - b(0x1f) as i32;
    let limit = |t: i32| (t * 33 + 0xfa0).min(0x1964);
    // the common conditions: no goal, no card, +0x17 up to 8, +0x03 up to 2 and the performance up to the limit
    let common = || b(0xc) == 0 && b(0x10) == 0 && b(0x17) <= 8 && (b(3) as u8) <= 2 && performance <= limit(in_field);
    let search = |cod: u32, data: &[u8; 8]| {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, *const [u8; 8]) -> u32 = core::mem::transmute(FN_MEDIA_HAS_LOGGED_STORY);
        f(*(G_MEDIA as *const usize), sheet.addr(), p.club(), -1, cod, data) != 0
    };
    let algum = |of: u32, ate: u32, data: &[u8; 8]| (of..=ate).any(|k| search(k as u16 as u32, data));
    let r = match c {
        0x219e => common() && (algum(0, 0x1d, &d7) || algum(0x1e, 0x27, &d7)),
        0x219f => performance >= random(0x7d0) + 0x1b58 && algum(0, 0x1d, &d7),
        0x21a0 => {
            if exe::date_days_since(today, *((sheet.addr() + player_sheet::TACKLING) as *const u32), *((sheet.addr() + player_sheet::VERSATILITY) as *const u32)) < 0x5a || performance <= random(0x7d0) + 0x1b58 {
                false
            } else {
                let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_MEDIA_HAS_RESPONSE_TWO_FOR_PLAYER);
                f(*(G_MEDIA as *const usize), sheet.addr()) != 0
            }
        }
        0x21a1 | 0x21a2 => {
            let goals = b(0xa) as u8 as i32;
            if p.addr() != m.pass_receiver() || (m.assist_candidate() as u32) == 0 {
                false
            } else if c == 0x21a1 {
                if goals <= 4 || p.shot_quality() <= 0xfa {
                    false
                } else {
                    let r = random(6) + 12;
                    in_field / goals < r && search(0x52, &d7)
                }
            } else if goals >= 3 || goals == 0 {
                false
            } else {
                let r = random(0x19) + 0x32;
                minute > r && in_field / goals > 0x1e && search(0x52, &d7)
            }
        }
        0x21b0 => performance > random(1000) + 0x1f40 && search(0x34, &d7),
        0x21b1 => common() && search(0x34, &d7),
        0x21b2 => common() && search(0x33, &d30),
        0x21b3 => performance > random(1000) + 0x1f40 && search(0x33, &d30),
        0x21b4 => performance >= random(0x7d0) + 0x1b58 && algum(0x28, 0x3b, &d7),
        0x21b5 => performance > random(1000) + 0x1f40 && b(0x30) != 0,
        0x21b6 => {
            let yes = performance > random(0x7d0) + 0x1c52 && b(0xc) == 1 && in_field > 0x41 && b(0x30) != 0;
            if yes {
                crate::comparison::branch("highlight mark 21b5");
                *((m.addr() + 0xbf2 + side as usize * 4000) as *mut u32) = 1;
            }
            yes
        }
        0x21b7 => b(0x30) != 0 && b(0xc) == 0 && b(0x10) == 0 && b(0x17) <= 8 && (b(3) as u8) <= 2 && in_field >= 0x37 && performance <= limit(in_field),
        0x21b8 => {
            performance < 0x1964
                && (b(0x14) as u8 as i32) > random(3)
                && player_table(p.addr()) & 4 != 0
                && p.player().defender() as i32 > random(4) + 0xf
                && b(0xc) == 0
                && (b(0x17) as i32) < random(6)
                && in_field > random(0x32) + 0x1e
        }
        _ => true,
    };
    crate::comparison::branch(if r { "destaque sim" } else { "destaque nao" });
    r as u32
}

/// Returns the severity byte of injury type `tipo` from the injury types table (13 bytes per type at 0x9e7ad9); `this`
/// (the injury manager) is unused.
///
/// [pt] { 0x62ec00 } (ret 4, injury.cpp): o byte de gravidade do tipo de lesao `tipo` (tabela de 13 bytes
/// por tipo em 0x9e7ad9).
pub(crate) unsafe extern "thiscall" fn injury_severity(_injuries: usize, kind: i8) -> u8 {
    read_u8(0x9e7ad9 + kind as isize as usize * 13)
}

/// Returns the severity grade of the person's injury (31-byte record per id in the injury table `lesoes`, +0x13, at
/// least 1), or 0 without a person, without a player record (+0x61), with the id out of range or with no injury (+0x12
/// negative).
///
/// [pt] { 0x62ece0 } (ret 4, injury.cpp): o grau da lesao da pessoa (registro de 31 bytes por id na
/// tabela das lesoes, +0x13, pelo menos 1), ou 0 sem pessoa, sem ficha (+0x61), id fora da tabela
/// (0xae2364) ou sem lesao (+0x12 negativo).
pub(crate) unsafe extern "thiscall" fn injury_grade(injuries: usize, person: usize) -> u8 {
    let person = Staff(person);
    if person.is_null() {
        return 0;
    }
    let id = read_i32(person.addr());
    if id >= read_i32(G_STAFF_COUNT) || person.player().is_null() {
        return 0;
    }
    let r = PhysicalRecord(read_usize(injuries) + (id as usize).wrapping_mul(31));
    if (r.recurring_type() as i8) <= -1 {
        return 0;
    }
    let g = (r.recurrence_grade() as i8);
    if g > 1 { g as u8 } else { 1 }
}

/// The injury names of player_setup.cfg (the comparison, the characters compared, the text in .rdata and the type), in
/// the original routine's order. The last one, ARTHRITIS (8 characters, 0xa6b3a8), gives 0x4c, and none gives 0x4b.
///
/// [pt] Os nomes das lesões do player_setup.cfg (a comparação, os caracteres comparados, o texto no .rdata e o tipo), na
/// ordem da original; o último, ARTHRITIS (8 caracteres, 0xa6b3a8), dá 0x4c, e nenhum, 0x4b.
pub(crate) const INJURY_NAMES: [(u32, usize, u8); 75] = [(4, 0xa6b840, 0), (3, 0xa6b83c, 1), (5, 0xa6b834, 2), (10, 0xa6b824, 3), (10, 0xa6b80c, 4), (10, 0xa6b7fc, 5), (10, 0xa6b7ec, 6), (10, 0xa6b7dc, 7), (10, 0xa6b7d0, 8), (10, 0xa6b7c0, 9), (10, 0xa6b7b4, 10), (15, 0xa6b7a0, 11), (15, 0xa6b788, 12), (10, 0xa6b778, 80), (10, 0xa6b768, 13), (10, 0xa6b758, 14), (10, 0xa6b74c, 15), (10, 0xa6b73c, 16), (10, 0xa6b730, 17), (10, 0xa6b71c, 18), (10, 0xa6b70c, 19), (10, 0xa6b6fc, 20), (10, 0xa6b6ec, 21), (10, 0xa6b6dc, 22), (10, 0xa6b6c8, 23), (10, 0xa6b6b4, 24), (10, 0xa6b6a0, 25), (10, 0xa6b690, 26), (10, 0xa6b678, 27), (10, 0xa6b664, 28), (10, 0xa6b654, 29), (10, 0xa6b644, 30), (10, 0xa6b634, 31), (10, 0xa6b628, 32), (10, 0xa6b618, 33), (10, 0xa6b608, 34), (10, 0xa6b5f8, 35), (10, 0xa6b5e4, 36), (5, 0xa6b5d8, 37), (10, 0xa6b5c8, 38), (10, 0xa6b5bc, 39), (10, 0xa6b5ac, 40), (10, 0xa6b5a0, 41), (14, 0xa6b590, 42), (10, 0xa6b584, 43), (10, 0xa6b574, 44), (10, 0xa6b564, 45), (10, 0xa6b554, 46), (5, 0xa6b548, 47), (10, 0xa6b538, 48), (10, 0xa6b52c, 49), (10, 0xa6b51c, 50), (10, 0xa6b50c, 52), (15, 0xa6b4fc, 53), (5, 0xa6b4f0, 54), (13, 0xa6b4e0, 55), (10, 0xa6b4d0, 56), (10, 0xa6b4c4, 57), (10, 0xa6b4b0, 58), (10, 0xa6b49c, 59), (10, 0xa6b48c, 60), (10, 0xa6b47c, 61), (10, 0xa6b46c, 62), (10, 0xa6b458, 63), (10, 0xa6b448, 64), (10, 0xa6b438, 65), (5, 0xa6b42c, 66), (10, 0xa6b420, 67), (5, 0xa6b414, 68), (5, 0xa6b408, 69), (13, 0xa6b3f8, 70), (10, 0xa6b3e8, 71), (19, 0xa6b3d4, 72), (10, 0xa6b3c0, 73), (10, 0xa6b3b4, 74)];

/// Calls the CRT strncmp (`exe::crt_strncmp`, cdecl).
///
/// [pt] strncmp (0x946fa0, cdecl).
pub(crate) unsafe fn strncmp(a: usize, b: usize, n: u32) -> i32 {
    exe::crt_strncmp(a, b, n)
}

/// The comparer's rehearsal: the copy runs without delivering the news item (`send_by_scope`, which allocates) and stops
/// before the walkout (`exe::awol_start_absence`, which adds to the list and sends a news item), keeping the decision.
///
/// [pt] O ensaio do comparador: a cópia roda sem a entrega da notícia (0x786090, que aloca) e para antes da fuga (0x419840,
/// que põe na lista e manda notícia), guardando a decisão.
pub(crate) static RECOVERY_NEWS_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static RECOVERY_NEWS_WOULD_GO_AWOL: AtomicBool = AtomicBool::new(false);

/// Address of the player's 31-byte physical record in the injury table ([this] + 31 × id).
///
/// [pt] O registro de forma do jogador na tabela das lesões ([this] + 31 × id).
pub(crate) unsafe fn physical_record_of_player(this: usize, p: usize) -> usize {
    read_usize(this).wrapping_add((read_i32(p) as isize * 31) as usize)
}

/// Calls a cdecl staff preference routine (likes/dislikes manager, remove from favourites/rejected) with person and
/// manager.
///
/// [pt] Gosta/não gosta do técnico (0x547810/0x547840) e tirar dos favoritos/rejeitados (0x547c10/0x547c50), cdecl.
pub(crate) unsafe fn call_preference_routine(routine: usize, p: usize, manager: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(routine);
    f(p, manager)
}

/// Calls a cdecl routine adding the manager to the person's favourites (`staff_add_favoured_staff`) or rejected (`exe::staff_add_disliked_staff`) list.
///
/// [pt] Acrescentar aos favoritos (0x547a10) ou aos rejeitados (0x547af0), cdecl(pessoa, outra, 1).
pub(crate) unsafe fn injury_add_to_preference_list(routine: usize, p: usize, manager: usize) {
    let f: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(routine);
    f(p, manager, 1);
}

/// Removes the club manager from the player's rejected list and, with random(+0x59) > 5, adds him to the favourites.
///
/// [pt] O técnico gostar mais: tira dos rejeitados e, com sorteio(+0x59) acima de 5, põe nos favoritos.
pub(crate) unsafe fn like_manager_more(p: Staff, club: Club) {
    if club.manager().addr() != 0 && call_preference_routine(0x547840, p.addr(), club.manager().addr()) != 0 {
        call_preference_routine(0x547c50, p.addr(), club.manager().addr());
    }
    if club.manager().addr() != 0 && friendly_random(p.loyalty() as i32) > 5 {
        injury_add_to_preference_list(0x547a10, p.addr(), club.manager().addr());
    }
}

/// Posts the news of a player's return to light or full training; with a type 12 category, also resolves the outcome of
/// the treatment, the return and the holidays (contract grievances, happiness, liking for the manager, chance of going
/// absent and personality attribute changes).
///
/// `Injury::post_recovery_news(player, stage, type, force, days)`, from the returns to training (`post_due_recovery_news`) and from the
/// end of the injury (`end_player_injury`); SEH frame for the local news. Without a player: nothing; without `force`, only for a
/// club with a human manager (`news_club_has_human_manager`). News 2001 (`construct_news_item`) with the stage, the type, the id, the three names, the
/// birth date, the year and the days (`set_news_field`, data 0..8). Category type 12 (the table of 13-byte entries at
/// 0x9e7ad9): random(32000) and: type 'Q' (holidays): with a contract (`exe::contract_manager_get_contract`), player sheet and club, each contract
/// grievance (+0x45: 0x20000, 0x20, 0x40) drops (`exe::staff_contract_clear_grievances`) with random(+0x5b) > random(25); if none drops,
/// random(+0x5b) 0 leads to the chance of going absent (with the absences manager [0xad9c6c] and +0x59 below 12,
/// random(+0x59) < random(5): the absence `exe::awol_start_absence` with 4 and the news is not posted); otherwise the contract happiness
/// (+0x44) rises 15 (−100..100) and, with a dropped flag, the manager likes him more (outcome 2); then random(+0x5b +
/// +0x59, − 5 with player sheet value +0xb above 7250) < random(20) and random(4) adjusts +0x57, +0x5d, +0x44 of the
/// player sheet and +0x59. Stage 0, type 'M' (treatment): 0 gives outcome 2 (form +0x12 = 0xff, +0x13 = 0); below 5,
/// outcome 1 (+0x13 = 0); otherwise outcome 0 and, with +0x5d below 5, the manager may lose sympathy
/// (`exe::staff_is_favoured_by`/`staff_remove_favoured_staff`, random(+0x5d) 0: rejected `exe::staff_add_disliked_staff`); +0x59 (or +0x57) above 10: the manager likes him more.
/// Stage 1, types 'O' and 'N': the outcome by the random draw, the form, and the flag +0xc of the 79-byte record
/// ([0xae2d0c]) is cleared. Data 9 (outcome) and 10; delivery by the reach `force` (`send_by_scope`: the club, the nation of
/// the club or of the person, and her continent).
/// Original: `Injury::post_recovery_news` (0x630ee0, injury.cpp, thiscall).
///
/// [pt] { 0x630ee0 } (thiscall, ret 0x14, injury.cpp, quadro SEH da notícia local): `Injury::post_recovery_news(jogador,
/// estágio, tipo, forçar, dias)`, pelas voltas aos treinos (0x62e010) e pelo fim da lesão (0x62e700). Sem jogador:
/// nada; sem forçar, só com clube de técnico humano (0x7875e0). A notícia 2001 (0x77bbe0) com o estágio, o tipo, o id,
/// os três nomes, o nascimento, o ano e os dias (0x785790, dados 0..8). Tipo de categoria 12 (a tabela de 13 bytes em
/// 0x9e7ad9): sorteio(32000) e: tipo 'Q' (férias): com contrato (0x4cf5a0), ficha e clube, cada queixa do
/// contrato (+0x45: 0x20000, 0x20, 0x40) cai (0x8674e0) com sorteio(+0x5b) > sorteio(25); sem nenhuma cair, sorteio(+0x5b)
/// 0 leva à chance de fuga (com o gerente das faltas [0xad9c6c] e +0x59 abaixo de 12, sorteio(+0x59) < sorteio(5): a
/// fuga 0x419840 com 4 e a notícia não sai); senão a felicidade do contrato (+0x44) sobe 15 (−100..100) e, com marca
/// caída, o técnico gosta mais (resultado 2); depois sorteio(+0x5b + +0x59, − 5 com valor +0xb da ficha acima de 7250) <
/// sorteio(20) e o sorteio(4) acerta +0x57, +0x5d, +0x44 da ficha e +0x59. Estágio 0, tipo 'M' (tratamento): 0 dá
/// resultado 2 (forma +0x12 = 0xff, +0x13 = 0); abaixo de 5, resultado 1 (+0x13 = 0); senão resultado 0 e, com +0x5d
/// abaixo de 5, o técnico pode perder a simpatia (0x547810/0x547c10, sorteio(+0x5d) 0: rejeitado 0x547af0); +0x59 (ou
/// +0x57) acima de 10: o técnico gosta mais. Estágio 1, tipos 'O' e 'N': o resultado pelo sorteio, a forma e a marca
/// +0xc do registro de 79 bytes ([0xae2d0c]) cai. Dados 9 (resultado) e 10; a entrega pelo alcance `forcar` (0x786090:
/// o clube, a nação do clube ou da pessoa e o continente dela).
pub(crate) unsafe extern "thiscall" fn post_recovery_news(this: usize, p: usize, estagio: u32, kind: u32, force: u32, days: u32) {
    if p == 0 {
        return;
    }
    if force as u8 == 0 {
        let club = read_usize(p + staff::CLUB_JOB);
        if club == 0 {
            return;
        }
        let human: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
        if human(club) == 0 {
            return;
        }
    }
    let mut news = [0u8; 0xe8];
    let n = news.as_mut_ptr() as usize;
    exe::new_newsitem(n, 0x7d1, 0);
    let est = estagio as u8 as i8;
    let tp = kind as u8 as i8;
    exe::newsitem_set_slot(n, 0, est as i32 as u32);
    exe::newsitem_set_slot(n, 1, tp as i32 as u32);
    exe::newsitem_set_slot(n, 2, read_usize(p) as u32);
    exe::newsitem_set_slot(n, 3, read_usize(read_usize(p + staff::FIRST_NAME) + names::ID) as u32);
    exe::newsitem_set_slot(n, 4, read_usize(read_usize(p + staff::SECOND_NAME) + names::ID) as u32);
    exe::newsitem_set_slot(n, 5, read_usize(read_usize(p + staff::COMMON_NAME) + names::ID) as u32);
    exe::newsitem_set_slot(n, 6, *((p + staff::DATE_OF_BIRTH) as *const i16) as i32 as u32);
    exe::newsitem_set_slot(n, 7, *((p + staff::DATE_OF_BIRTH_YEAR) as *const u16) as u32);
    exe::newsitem_set_slot(n, 8, days as u16 as i16 as i32 as u32);
    let club = read_usize(p + staff::CLUB_JOB);
    let mut res: i8 = -1;
    let mut f1: i8 = 0;
    if *((0x9e7ad9 + tp as isize * 13) as *const u8) == 0xc {
        let s = friendly_random(32000) as i16;
        if tp == 0x51 {
            res = 0;
            let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
            if c == 0 || read_usize(p + staff::PLAYER) == 0 || club == 0 {
                f1 = 1;
            } else {
                let mut fell = false;
                for marca in [0x20000u32, 0x20] {
                    if read_usize(c + contract::GRIEVANCES) as u32 & marca != 0 {
                        let a = friendly_random(read_i8(p + staff::PROFESSIONALISM) as i32);
                        let b = friendly_random(0x19);
                        if a > b {
                            exe::staff_contract_clear_grievances(c, marca);
                            fell = true;
                        }
                    }
                }
                let mut happy = fell;
                if read_usize(c + contract::GRIEVANCES) as u32 & 0x40 != 0 {
                    let a = friendly_random(read_i8(p + staff::PROFESSIONALISM) as i32);
                    let b = friendly_random(0x19);
                    if a > b {
                        exe::staff_contract_clear_grievances(c, 0x40);
                        fell = true;
                        happy = true;
                    }
                }
                if !happy && friendly_random(read_i8(p + staff::PROFESSIONALISM) as i32) != 0 {
                    happy = true;
                }
                let mut follow = true;
                if happy {
                    let v = read_i8(c + contract::MORALE) as i32 + 0xf; // cqf9: a felicidade sobe 15
                    crate::comparison::branch("cqf_9 happiness by holidays");
                    *((c + contract::MORALE) as *mut i8) = if v > 100 { 100 } else if v < -100 { -100 } else { v as i8 };
                }
                if happy && fell {
                    res = 2;
                    like_manager_more(Staff(p), Club(club));
                } else {
                    if read_usize(G_AWOL) != 0 && read_i8(p + staff::LOYALTY) < 0xc {
                        let a = friendly_random(read_i8(p + staff::LOYALTY) as i32);
                        let b = friendly_random(5);
                        if a < b {
                            if RECOVERY_NEWS_DRY_RUN.load(Relaxed) {
                                RECOVERY_NEWS_WOULD_GO_AWOL.store(true, Relaxed);
                                return;
                            }
                            exe::awol_start_absence(read_usize(G_AWOL), p, club, 4);
                            return;
                        }
                    }
                    res = 0;
                    follow = true;
                }
                if follow {
                    let mut x = (read_i8(p + staff::PROFESSIONALISM) as i16).wrapping_add(read_i8(p + staff::LOYALTY) as i16);
                    f1 = 1;
                    if *((read_usize(p + staff::PLAYER) + player_sheet::CURRENT_REPUTATION) as *const i16) > 0x1c52 {
                        x = x.wrapping_sub(5);
                    }
                    let a = friendly_random(x as i32);
                    let b = friendly_random(0x14);
                    if a < b {
                        let r = friendly_random(4);
                        let f = read_usize(p + staff::PLAYER);
                        let subir_57 = |p: Staff| {
                            if p.ambition() < 0x14 {
                                *((p.addr() + staff::AMBITION) as *mut i8) += 1;
                            }
                        };
                        let descer_5d_44 = |p: Staff, f: usize| {
                            if p.temperament() > 1 {
                                *((p.addr() + staff::TEMPERAMENT) as *mut i8) -= 1;
                            }
                            if read_i8(f + player_sheet::WORK_RATE) > 1 {
                                *((f + player_sheet::WORK_RATE) as *mut i8) -= 1;
                            }
                        };
                        crate::comparison::branch("cqf_9 fitness by holidays");
                        if r == 0 {
                            subir_57(Staff(p)); // cqf9: random draw 0 raises +0x57
                            descer_5d_44(Staff(p), f);
                        } else if r == 1 {
                            subir_57(Staff(p));
                        } else {
                            descer_5d_44(Staff(p), f);
                        }
                        if r == 0 || r == 1 {
                            if read_i8(p + staff::LOYALTY) > 1 {
                                *((p + staff::LOYALTY) as *mut i8) -= 1;
                            }
                        }
                    }
                }
            }
        } else if est == 0 {
            if tp != 0x4d {
                res = -1;
            } else {
                let form = physical_record_of_player(this, p);
                let mut v: Option<i8> = None;
                crate::comparison::branch("cqf_9 treatment");
                if s == 0 { // cqf9: o tratamento dá certo
                    res = 2;
                    *((form + physical_record::RECURRING_TYPE) as *mut u8) = 0xff;
                    *((form + physical_record::RECURRENCE_GRADE) as *mut u8) = 0;
                    v = Some(read_i8(p + staff::LOYALTY));
                } else if s < 5 {
                    res = 1;
                    *((form + physical_record::RECURRENCE_GRADE) as *mut u8) = 0;
                    if read_i8(p + staff::LOYALTY) > 0xf {
                        v = Some(read_i8(p + staff::AMBITION));
                    }
                } else {
                    res = 0;
                    if read_i8(p + staff::TEMPERAMENT) < 5 {
                        f1 = -1;
                        if club != 0 {
                            if read_usize(club + club::MANAGER) != 0 && call_preference_routine(0x547810, p, read_usize(club + club::MANAGER)) != 0 {
                                call_preference_routine(0x547c10, p, read_usize(club + club::MANAGER));
                            }
                            if read_usize(club + club::MANAGER) != 0 && friendly_random(read_i8(p + staff::TEMPERAMENT) as i32) == 0 {
                                injury_add_to_preference_list(0x547af0, p, read_usize(club + club::MANAGER));
                            }
                        }
                    }
                }
                if let Some(v) = v {
                    if v > 10 {
                        f1 = 1;
                        if club != 0 {
                            like_manager_more(Staff(p), Club(club));
                        }
                    }
                }
            }
        } else {
            let form = physical_record_of_player(this, p);
            crate::comparison::branch("cqf_9 return from injury");
            if tp == 0x4f {
                if s < 30000 { // cqf9: a volta 'O' completa
                    res = 2;
                    *((form + physical_record::RECURRING_TYPE) as *mut u8) = 0xff;
                    *((form + physical_record::RECURRENCE_GRADE) as *mut u8) = 0;
                } else if friendly_random(5) != 0 {
                    res = 1;
                    *((form + physical_record::RECURRENCE_GRADE) as *mut u8) = 0;
                } else {
                    res = 3;
                    let a = form + physical_record::RECURRENCE_GRADE;
                    let r = friendly_random(4);
                    *(a as *mut u8) = (*(a as *const u8)).wrapping_add((r as u8).wrapping_add(1));
                }
            } else if tp == 0x4e {
                if s < 0x733c { // cqf9: a volta 'N' completa
                    res = 2;
                    *((form + physical_record::RECURRING_TYPE) as *mut u8) = 0xff;
                    *((form + physical_record::RECURRENCE_GRADE) as *mut u8) = 0;
                } else if s < 30000 {
                    res = 1;
                    *((form + physical_record::RECURRENCE_GRADE) as *mut u8) = 0;
                } else {
                    res = 0;
                }
            }
            let e = read_usize(G_STAFF_STATE_TABLE).wrapping_add((*(p as *const i32) as isize * 0x4f + (staff_state::EXTRA_FLAGS as isize)) as usize);
            *(e as *mut u8) &= 0xfe; // cqf9: a marca +0xc cai
        }
    }
    exe::newsitem_set_slot(n, 9, res as i32 as u32);
    exe::newsitem_set_slot(n, 10, f1 as i32 as u32);
    if RECOVERY_NEWS_DRY_RUN.load(Relaxed) {
        return;
    }
    let nation = Nation(if club != 0 { read_usize(club + club::NATION) } else { read_usize(p + staff::NATION) });
    let continent = if !nation.is_null() { nation.continent() } else { 0 };
    exe::news_send_by_scope(read_usize(G_NEWS), n, club, 0, nation, continent, force);
}

/// Returns the translated injury category name and the name's gender in language 3.
///
/// `injury_get_category_name(category, text, size)` (stdcall, ret 0xc): the translated name (0x66f4e0) of injury
/// category 0..12 (internal, foot, shin, knee, thigh, groin, upper body, hand, arm, shoulder, neck, head,
/// rehabilitation); otherwise the box (line 0x10e7). Returns 2 with language 3 (`exe::langlib_get_language`), otherwise 0 (the name's
/// gender). Size below 20: the box (line 0x10d0) and al zeroed.
/// Original: `injury_get_category_name` (0x630c50, injury.cpp, stdcall).
///
/// [pt] { 0x630c50 } (stdcall, ret 0xc, injury.cpp): `injury_get_category_name(categoria, texto, tamanho)`: o nome traduzido
/// (0x66f4e0) da categoria da lesão 0..12 (interna, pé, canela, joelho, coxa, virilha, tronco, mão, braço, ombro,
/// pescoço, cabeça, recuperação); fora disso a caixa (linha 0x10e7). Devolve 2 com o idioma 3 (0x66f4d0), senão 0 (o
/// gênero do nome). Tamanho abaixo de 20: a caixa (linha 0x10d0) e o al zerado.
pub(crate) unsafe extern "stdcall" fn injury_get_category_name(category: u32, text: usize, size: i32) -> u32 {
    if size < 0x14 {
        show_error_at(0x9e7f28, 0x10d0);
        return 0;
    }
    const NOMES: [usize; 13] = [0x9e89ac, 0x9e89a0, 0x9e8994, 0x9e8988, 0x9e8978, 0x9e8968, 0x9e8954, 0x9e8948, 0x9e893c, 0x9e892c, 0x9e8920, 0x9e8914, 0x9e8904];
    let k = category as u8 as i8 as i32 as u32;
    if k <= 0xc {
        let translate: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
        translate(text, NOMES[k as usize]);
    } else {
        show_error_at(0x9e7f28, 0x10e7);
    }
    if exe::langlib_get_language() == 3 { 2 } else { 0 }
}

/// Address of person id's 31-byte physical record in [this].
///
/// [pt] O registro físico de 31 bytes da pessoa `id` em [this].
pub(crate) unsafe fn physical_record(this: usize, id: usize) -> usize {
    read_usize(this).wrapping_add(id.wrapping_mul(0x1f))
}

/// Address of injury i (0x30 bytes) in the array [this+4].
///
/// [pt] A lesão `i` (0x30 bytes) do vetor [this+4].
pub(crate) unsafe fn injury_entry(this: usize, i: usize) -> usize {
    read_usize(this + injury_fields::INJURY_ENTRIES).wrapping_add(i.wrapping_mul(0x30))
}

/// Person's contract via CONTRACT_MANAGER::get_contract, also stored in G_CONTRACT_LOOKUP_RESULT.
///
/// [pt] O contrato da pessoa (0x4cf5a0: thiscall([0xae1050], pessoa), ret 4), guardado em [0xae1080].
pub(crate) unsafe fn injury_main_contract_stored(p: usize) -> Contract {
    let c = exe::contract_manager_get_contract(read_usize(G_CONTRACT_MANAGER), p);
    write_usize(G_CONTRACT_LOOKUP_RESULT, c);
    Contract(c)
}

/// Club record (0x245 bytes in G_CLUB_TABLE) of contract c (+4 = club id); 0 without contract.
///
/// [pt] O clube do contrato `c` (+4 = id no registro de 0x245 bytes em [0xae23b4]; 0 sem contrato).
pub(crate) unsafe fn injury_club_of_contract(c: usize) -> usize {
    if c == 0 { 0 } else { read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE)) }
}

/// Offset of an injury type in the 13-byte-per-type injury table (base days, variable days, return-to-training %, can
/// recur).
///
/// [pt] A tabela dos tipos de lesão (13 bytes por tipo): +0 (0x9e7adb) dias de base (i16), +2 (0x9e7add) dias variáveis
/// (i16), +4 (0x9e7adf) percentual da volta ao treino (i8), +5 (0x9e7ae0) pode voltar (i32).
pub(crate) fn injury_type_offset(kind: i8) -> usize {
    (kind as i32 * 13) as usize
}

/// Days until return to training (+0x2a): the type's percentage × duration, at most 60 days before the end.
///
/// [pt] Os dias da volta ao treino (+0x2a) pela duração `dias`: ftol(percentual × 0,01 × dias) (fild, fmul qword
/// [0x967898], fimul, _ftol 0x945640; x87 em dupla precisão), no máximo 60 dias antes do fim.
pub(crate) unsafe fn return_to_training_days(kind: i8, days: i16) -> i16 {
    let pct = read_i8_as_i32(G_INJURY_TYPE_TRAINING_RETURN_PERCENT + injury_type_offset(kind));
    let x = ftol(pct as f64 * read_f64(rdata::F64_0_01) * days as f64) as i16;
    if (x as i32) + 0x3c < days as i32 { days.wrapping_sub(0x3c) } else { x }
}

/// Records an injury: replaces the previous one only for some types, finds or grows the slot, and records the match,
/// the length, the return to training, the date, the news, the recurrence, the national team, the relapse, the
/// retirement and the outbreak.
///
/// `Injury::add_injury(type, retroactive, person, match, outbreak, news arg, recurrence, recurrence_only)`, from the
/// injuries day (`injury_daily_update`), the physical effects of the match (`apply_match_effects`), the initial records (`init_records`), the relapse
/// (`roll_recurring_injury`), the illness outbreaks (`spread_illness_outbreaks`) and two original routines; frame of 0x220 bytes without SEH. Null
/// person, no player sheet (+0x61) or type outside 0..0x51: as in the original (box and −1); id beyond [0xae2364]: −1.
/// The day's fitness (`update_player_fitness`) and the training ([0xde1f44], `update_player_training` with 0). Recurrence only: with a recurrence and
/// a type that can recur, the record's relapse flag (+0x12 type, +0x13: 1 if random(rec / 10) and random(rec) both give
/// zero with a negative rec; rec if positive); −1. Otherwise, with an injury (+0xe ≠ −1): only types 0x4d, 0x4e, 0x4f
/// and 0x51 replace it (the remaining days +0x2c are kept, the old one goes to the history 0x58e340 and the slot is
/// freed). +0x1d = type; the first free slot (byte 0xff) of the vector [this+4] (count [this+8]) or 100 more slots
/// (realloc; out of memory, the box for line 0x909 and −1). The type in the slot; the match (through the home club
/// +0x1c or the away club +0x20 that has the person among the 50 of the squad +0xd7) writes the match record at +1
/// (`fill_match_record_from_fixture`), otherwise +0x1a = 0xff. The club's best physio (`exe::non_player_attribute_rating` of the three +0x1cf with +0x69). Days = base +
/// variable × (35 − best) / 20 (minus a random draw of them if random(best + 5) > random(20); 0 for type 0x51) + the
/// remaining days of the old one (+ random draws by age and by proneness +0x2d of the player sheet, with base above
/// 10). The +0xa of the 79-byte record goes up 1 for each zero of random(5 × natural + 12), per day, up to 100. A
/// current injury (retroactive 0): of 100 days, outside the human club and outside the list, the club's flag 100 in the
/// watch list; days ≥ random(50) + 30, the clubs targeting the player suspend it (`suspend_clubs_shortlisting_player`). +0x28 = +0x2c = days,
/// +0x2a = +0x2e = the return to training (the type's percentage, at most 60 days before the end); +0x20 = the date
/// (today, or today − random(days) − 1 when retroactive). The fitness again, the news (`injury_post_injury_news`), +0xe = the slot, the
/// recurrence, the national team (`withdraw_injured_player`), +0x1c = 0, the relapse (`roll_recurring_injury`), the relapse count (from two: retirement
/// `schedule_injury_retirement` by random draw), the retirement by age and by a long return to training, and the outbreak (`add_illness_outbreak`) for
/// types 0, 1 and 2. Returns the slot.
/// Original: `Injury::add_injury` (0x62d250, injury.cpp, thiscall, ret 0x20).
///
/// [pt] { 0x62d250 } (thiscall, ret 0x20, injury.cpp, quadro de 0x220 bytes sem SEH): `Injury::add_injury(tipo, retroativa,
/// pessoa, partida, surto, arg da notícia, recorrência, só_recorrência)`, pelo dia das lesões (0x62b7b0), pelos efeitos
/// físicos da partida (0x62c080), pelos registros iniciais (0x62cf70), pela recaída (0x62e500), pelos surtos de doença
/// (0x62e9e0) e por duas originais. Pessoa nula, sem ficha (+0x61) ou tipo fora de 0..0x51: a original (caixa e −1); id
/// além de [0xae2364]: −1. O físico do dia (0x62c680) e o treino ([0xde1f44], 0x8b1750 com 0). Só recorrência: com
/// recorrência e o tipo que pode voltar, a marca de recaída do registro (+0x12 tipo, +0x13: 1 se sorteio(rec / 10) e
/// sorteio(rec) derem zero com rec negativo; rec se positivo); −1. Senão, com lesão (+0xe ≠ −1): só os tipos 0x4d, 0x4e,
/// 0x4f e 0x51 a trocam (os dias restantes +0x2c ficam, a velha vai ao histórico 0x58e340 e a vaga fica livre). +0x1d
/// = tipo; a primeira vaga livre (byte 0xff) do vetor [this+4] (conta [this+8]) ou mais 100 vagas (realloc; sem
/// memória, a caixa da linha 0x909 e −1). O tipo na vaga; a partida (pelo clube da casa +0x1c ou de fora +0x20 que tem
/// a pessoa nos 50 do elenco +0xd7) grava o registro da partida em +1 (0x7e4750), senão +0x1a = 0xff. O melhor
/// fisioterapeuta do clube (0x544410 dos três +0x1cf com +0x69). Dias = base + variáveis × (35 − melhor) / 20 (menos
/// sorteio deles se sorteio(melhor + 5) > sorteio(20); 0 no tipo 0x51) + os restantes da velha (+ sorteios pela idade
/// e pela propensão +0x2d da ficha, com base acima de 10). O +0xa do registro de 79 bytes sobe 1 a cada sorteio(5 ×
/// natural + 12) zero, por dia, até 100. Lesão de agora (retroativa 0): de 100 dias, fora do clube humano e fora da
/// lista, a marca 100 do clube na lista de observação; dias ≥ sorteio(50) + 30, os clubes que miram o jogador o
/// suspendem (0x842b00). +0x28 = +0x2c = dias, +0x2a = +0x2e = a volta ao treino; +0x20 = a data (hoje, ou hoje −
/// sorteio(dias) − 1 na retroativa). O físico de novo, a notícia (0x62dca0), +0xe = a vaga, a recorrência, a seleção
/// (0x775cd0), +0x1c = 0, a recaída (0x62e500), a contagem das recaídas (de duas: aposentadoria 0x7a7f20 por sorteio),
/// a aposentadoria pela idade e pela volta ao treino longa e o surto (0x62e820) nos tipos 0, 1 e 2. Devolve a vaga.
pub(crate) unsafe extern "thiscall" fn add_injury(this: usize, kind_arg: u32, retroativa: u32, p: usize, game: usize, surto: u32, news: u32, rec_arg: u32, so_recorrencia: u32) -> i32 {
    let game = Fixture(game);
    let p = Staff(p);
    let kind = kind_arg as u8 as i8;
    let original = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, u32, u32, u32) -> i32>(IDX_ADD_INJURY);
    if p.is_null() {
        return original(this, kind_arg, retroativa, p.addr(), game.addr(), surto, news, rec_arg, so_recorrencia); // the message box: original only
    }
    if read_usize(p.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return -1;
    }
    if p.player().is_null() || !(0..0x52).contains(&kind) {
        return original(this, kind_arg, retroativa, p.addr(), game.addr(), surto, news, rec_arg, so_recorrencia); // the boxes: original only
    }
    exe::injury_update_player_fitness(this, p);
    if read_usize(G_TRAINING_MANAGER) != 0 {
        exe::training_manager_update_player_training(read_usize(G_TRAINING_MANAGER), p, 0);
    }
    let t = injury_type_offset(kind);
    let rec = rec_arg as u8 as i8;
    if so_recorrencia != 0 {
        if rec == 0 || read_usize(G_INJURY_TYPE_CAN_RECUR + t) == 0 {
            return -1;
        }
        if rec < 0 {
            if friendly_random(rec as i32 / 10) == 0 {
                let r = physical_record(this, read_usize(p.addr()));
                write_u8(r + physical_record::RECURRING_TYPE, kind as u8);
                write_u8(r + physical_record::RECURRENCE_GRADE, 1);
            }
            return -1;
        }
        let r = physical_record(this, read_usize(p.addr()));
        write_u8(r + physical_record::RECURRING_TYPE, kind as u8);
        write_u8(r + physical_record::RECURRENCE_GRADE, rec as u8);
        return -1;
    }
    let antiga = read_usize(physical_record(this, read_usize(p.addr())) + physical_record::CURRENT_INJURY) as i32;
    let mut remaining: u16 = 0; // the local’s low word (the high one is 0)
    if antiga != -1 {
        if !(kind == 0x4d || kind == 0x4e || kind == 0x4f || kind == 0x51) {
            return -1;
        }
        crate::comparison::branch("cqb_5 swaps the injury");
        let r = physical_record(this, read_usize(p.addr()));
        let l = InjuryEntry(injury_entry(this, antiga as usize));
        remaining = l.days_left() as u16;
        exe::file_llist_add_node(read_usize(this + injury_fields::INJURY_HISTORY), r + physical_record::HISTORY_FIRST, r + physical_record::HISTORY_LAST, (l.addr()) as usize as *const u8);
        write_u8(l.addr(), 0xff);
        write_i32(physical_record(this, read_usize(p.addr())) + physical_record::CURRENT_INJURY, -1);
    }
    write_u8(physical_record(this, read_usize(p.addr())) + physical_record::LAST_INJURY_TYPE, kind as u8);
    // the place
    let mut slot: i32 = -1;
    let array = read_usize(this + injury_fields::INJURY_ENTRIES);
    if array != 0 {
        let n = read_usize(this + injury_fields::INJURY_ENTRY_COUNT) as i32;
        let mut i = 0;
        while i < n {
            if read_u8(array + i as usize * 0x30) == 0xff {
                slot = i;
                break;
            }
            i += 1;
        }
    }
    if slot == -1 {
        let new = (read_usize(this + injury_fields::INJURY_ENTRY_COUNT) as i32).wrapping_add(100);
        let size = (new as usize).wrapping_mul(0x30);
        let v = if array != 0 {
            exe::crt_realloc(array, (size) as u32)
        } else {
            exe::crt_malloc((size) as u32)
        };
        write_usize(this + injury_fields::INJURY_ENTRIES, v);
        if v == 0 {
            show_error_at(0x9e7f28, 0x909); // injury.cpp
            return -1;
        }
        let old = read_usize(this + injury_fields::INJURY_ENTRY_COUNT) as i32;
        let mut i = old;
        while i < new {
            write_u8(read_usize(this + injury_fields::INJURY_ENTRIES) + i as usize * 0x30, 0xff);
            i += 1;
        }
        write_i32(this + injury_fields::INJURY_ENTRY_COUNT, new);
        slot = old;
    }
    let off = slot as usize * 0x30;
    write_u8(read_usize(this + injury_fields::INJURY_ENTRIES) + off, kind as u8);
    let mut of_match = false;
    if !game.is_null() {
        for k in 0..50usize {
            let mut club = game.home_club();
            let found = if club.squad(k) == p.addr() {
                true
            } else if read_usize(game.away_club().addr() + club::SQUAD + 4 * k) == p.addr() {
                club = game.away_club();
                true
            } else {
                false
            };
            if found {
                if !club.is_null() {
                    exe::fill_match_record_from_fixture(game, club, read_usize(this + injury_fields::INJURY_ENTRIES) + off + 1);
                    of_match = true;
                }
                break;
            }
        }
    }
    if !of_match {
        write_u8(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::MATCH_MARK, 0xff);
    }
    // o melhor fisioterapeuta
    let mut best: i8 = 0;
    if !p.club_job().is_null() {
        for k in 0..3usize {
            let f = Staff(read_usize(p.club_job().addr() + club::PHYSIOS + 4 * k));
            if f.is_null() {
                continue;
            }
            let np = f.non_player();
            if np.is_null() {
                continue;
            }
            if exe::non_player_attribute_rating(np, np.physiotherapy() as i8) as i8 > best {
                let np = read_usize(read_usize(p.club_job().addr() + club::PHYSIOS + 4 * k) + staff::NON_PLAYER);
                best = exe::non_player_attribute_rating(np, read_u8(np + non_player::PHYSIOTHERAPY) as i8) as i8;
            }
        }
    }
    // the days
    crate::comparison::branch("cqb_5 duration");
    let mut extra = (read_i16(G_INJURY_TYPE_VARIABLE_DAYS + t) as i32).wrapping_mul(0x23 - best as i32) / 20; // cqb5: o fisioterapeuta encurta
    let s1 = friendly_random(best as i32 + 5);
    let s2 = friendly_random(0x14);
    if s1 > s2 {
        extra = extra.wrapping_sub(friendly_random(extra as i16 as i32));
    }
    if kind == 0x51 {
        extra = 0;
    }
    extra = extra.wrapping_add(remaining as i32);
    let age = (p.age() as u8) as i8;
    if kind != 0x51 && read_i16(G_INJURY_TYPE_BASE_DAYS + t) > 10 {
        if age > 0x1e {
            let ax = ((p.player().injury_proneness() as i32) as i16).wrapping_add(age as i16).wrapping_sub(0x23);
            if ax > 0 {
                extra = extra.wrapping_add(friendly_random(ax as i32));
            }
        }
        let al = (p.player().injury_proneness() as u8) as i8;
        if al > 10 {
            extra = extra.wrapping_add(friendly_random((al as i16 - 10) as i32));
        }
    }
    let days = read_i16(G_INJURY_TYPE_BASE_DAYS + t).wrapping_add(extra as i16);
    let n = days as i32;
    let mut i = 0;
    while i < n {
        let g = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p.addr()).wrapping_mul(0x4f)) + staff_state::INJURY_COUNTER;
        if read_u8(g) >= 100 {
            break;
        }
        if friendly_random((p.player().natural_fitness() as i32) * 5 + 0xc) == 0 {
            let g = read_usize(G_STAFF_STATE_TABLE).wrapping_add(read_usize(p.addr()).wrapping_mul(0x4f)) + staff_state::INJURY_COUNTER;
            write_u8(g, read_u8(g).wrapping_add(1));
        }
        i += 1;
    }
    if retroativa == 0 {
        let club = p.club_job();
        if !club.is_null() {
            if exe::human_manager_is_human_club(club, 0, (1) as u32, (0) as u32, 0) == 0 && days >= 100 {
                if exe::staff_is_transfer_listed(p, 0) == 0 && read_usize(G_TRANSFER_MANAGER) != 0 {
                    let tm = TransferManagerFields(read_usize(G_TRANSFER_MANAGER));
                    let m = read_usize(tm.shortlist_manager() + shortlist_manager_fields::CLUB_SHORTLISTS).wrapping_add(read_usize(p.club_job().addr()).wrapping_mul(9)) + 8;
                    write_u8(m, 100);
                }
            }
        }
        let s = friendly_random(0x32);
        if n >= s + 0x1e && read_usize(G_TRANSFER_MANAGER) != 0 && read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER) != 0 {
            crate::comparison::branch("cqb_5 suspends the targets");
            exe::shortlist_manager_suspend_clubs_shortlisting_player(read_usize(read_usize(G_TRANSFER_MANAGER) + transfer_manager_fields::SHORTLIST_MANAGER), p, (0) as u32); // cqb5: the clubs targeting him suspend
        }
    }
    write_i16(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::DAYS_OUT, days);
    write_i16(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::DAYS_LEFT, days);
    crate::comparison::branch("cqb_5 back to training");
    let volta = return_to_training_days(kind, days); // cqb5: the return to training by percentage
    write_i16(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::TRAINING_RETURN_DAYS, volta);
    write_i16(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::TRAINING_DAYS_LEFT, read_i16(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::TRAINING_RETURN_DAYS));
    let mut out = [0u8; 8];
    if retroativa != 0 {
        let s = friendly_random(n);
        let mut d = [0u8; 8];
        let r = exe::date_sub_days(G_CURRENT_DATE, (d.as_mut_ptr()) as *mut u8 as usize, (s + 1) as u32);
        exe::date_operator_3d(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::DATE, (out.as_mut_ptr()) as usize, r);
    } else {
        exe::date_operator_3d(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::DATE, (out.as_mut_ptr()) as usize, G_CURRENT_DATE);
    }
    exe::injury_update_player_fitness(this, p);
    exe::injury_post_injury_news(this, p, read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr(), news);
    write_i32(physical_record(this, read_usize(p.addr())) + physical_record::CURRENT_INJURY, slot);
    if rec != 0 && read_usize(G_INJURY_TYPE_CAN_RECUR + t) != 0 {
        if rec < 0 {
            if friendly_random(rec as i32 / 10) == 0 {
                write_u8(physical_record(this, read_usize(p.addr())) + physical_record::RECURRING_TYPE, kind as u8);
                if friendly_random(rec as i32) == 0 {
                    write_u8(physical_record(this, read_usize(p.addr())) + physical_record::RECURRENCE_GRADE, 1);
                }
            }
        } else {
            write_u8(physical_record(this, read_usize(p.addr())) + physical_record::RECURRING_TYPE, kind as u8);
            write_u8(physical_record(this, read_usize(p.addr())) + physical_record::RECURRENCE_GRADE, rec as u8);
        }
    }
    if !p.national_job().is_null() && read_usize(G_NATIONAL_TEAMS) != 0 {
        exe::nationalteams_withdraw_injured_player(read_usize(G_NATIONAL_TEAMS), p, read_i16(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::DAYS_OUT) as u16 as u32);
    }
    write_u8(physical_record(this, read_usize(p.addr())) + physical_record::TRAINING_NEWS_SENT, 0);
    exe::injury_roll_recurring_injury(this, physical_record(this, read_usize(p.addr())), read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr());
    let r = physical_record(this, read_usize(p.addr()));
    if read_u8(r + physical_record::RECURRING_TYPE) as i8 == kind && rec >= 0 {
        write_u8(r + physical_record::RECURRENCE_GRADE, read_u8(r + physical_record::RECURRENCE_GRADE).wrapping_add(1));
        let c = read_u8(physical_record(this, read_usize(p.addr())) + physical_record::RECURRENCE_GRADE) as i8;
        if c > 1 {
            let mut e = 200 - read_i16(G_INJURY_TYPE_BASE_DAYS + t) as i32;
            if e <= 5 {
                e = 5;
            }
            if rec > 0 {
                e /= rec as i32;
            }
            let s1 = friendly_random(c as i32);
            let s2 = friendly_random(e);
            if s1 > s2 && read_usize(G_PLAYER_REGEN) != 0 {
                crate::comparison::branch("cqb_5 retires through relapses");
                exe::player_regen_schedule_injury_retirement(read_usize(G_PLAYER_REGEN), p, read_i16(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::TRAINING_RETURN_DAYS));
            }
        }
    }
    // retirement by age and by a long return to training
    let volta = || read_i16(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::TRAINING_RETURN_DAYS) as i32;
    let propensao = || p.player().injury_proneness() as i32;
    let years = || (p.age() as i8) as i32;
    let encerra = (age > 0x24 && volta() > 0x78 - propensao() - years() && friendly_random(volta()) > 0x5a - years())
        || (age > 0x21 && volta() > 0x96 - propensao() - years() && friendly_random(volta()) > 0x5a - years())
        || (age > 0x1f && volta() > 0x12c - years() && friendly_random(volta()) > 0xc8 - years())
        || (volta() > 0x1f4 && friendly_random((p.player().natural_fitness() as i32) + 5) == 0);
    if encerra && read_usize(G_PLAYER_REGEN) != 0 {
        crate::comparison::branch("cqb_5 retires through injury");
        exe::player_regen_schedule_injury_retirement(read_usize(G_PLAYER_REGEN), p, read_i16(read_usize(this + injury_fields::INJURY_ENTRIES) + off.addr() + injury_entry::TRAINING_RETURN_DAYS));
    }
    if surto != 0 && (kind == 1 || kind == 0 || kind == 2) {
        let club = p.club_job();
        if !club.is_null() {
            exe::injury_add_illness_outbreak(this, (G_CURRENT_DATE) as usize as *mut [u8; 8], kind_arg, (read_usize(club.addr()) as i32) as i32 as usize, 0);
        }
    }
    slot
}

/// Posts injury news 2000 (the person, type, date, days, birth date and match): over 100 days, to everyone if the
/// potential is above 8000, otherwise to the interested clubs; up to 100, to the human club, the contract club and the
/// national team.
///
/// `injury_post_injury_news(person, injury, arg)`. The function map gives it as stdcall: the original replaces ecx with
/// [0xde1f54] right at entry; the only call site, in `add_injury`, puts the Injury in ecx: thiscall with this unused; ret
/// 0xc; SEH frame without destructor. The interested clubs (`exe::transfer_manager_collect_player_news_recipients` on [0xde1f54] with 0x5a, up to 30) and the
/// national team (+0x24) if it fits; news 0x7d0 (local): 0..3 the person (id and the +0x33 of +4/+8/+0xc, 0 if
/// missing), 4 = the type, 5/6 = the injury date (+0x20 signed, +0x22 unsigned), 7 = the days (+0x28), 8/9 = the birth
/// date (+0x10, +0x12), 0xa..0xc = the match record (+0x12, +0x16, +9) or −1 with no match (+0x1a = 0xff), 0xd = arg,
/// 0xe = the club id or −1, 0xf = 0x420e9; the match in the news (0x7e4de0 with byte 0x10). Over 100 days: with
/// potential (+0xd of the player sheet) above 8000, to everyone (`send_to_all_managers`); otherwise to the interested clubs
/// (`send_to_club_manager`). Up to 100: a human club (`news_club_has_human_manager`), to the club; the contract club (`exe::contract_manager_get_contract`) if it is a different one;
/// the national team.
/// Original: `injury_post_injury_news` (0x62dca0, injury.cpp, thiscall (this sem uso), ret 0xc).
///
/// [pt] { 0x62dca0 } (o mapa a dá como stdcall: a original troca o ecx por [0xde1f54] logo na entrada; o único ponto de
/// chamada, na 0x62d250, põe a Injury em ecx: thiscall com o this sem uso; ret 0xc; quadro SEH sem destrutor):
/// `injury_post_injury_news(pessoa, lesão, arg)`. Os clubes interessados (0x8d1da0 em [0xde1f54] com 0x5a, até 30) e a
/// seleção (+0x24) se couber; a notícia 0x7d0 (local): 0..3 a pessoa (id e os +0x33 de +4/+8/+0xc, 0 se faltam), 4 =
/// o tipo, 5/6 = a data da lesão (+0x20 com sinal, +0x22 sem), 7 = os dias (+0x28), 8/9 = o nascimento (+0x10, +0x12),
/// 0xa..0xc = o registro da partida (+0x12, +0x16, +9) ou −1 sem partida (+0x1a = 0xff), 0xd = arg, 0xe = id do clube
/// ou −1, 0xf = 0x420e9; a partida na notícia (0x7e4de0 com o byte 0x10). Mais de 100 dias: com potencial (+0xd da
/// ficha) acima de 8000, a todos (0x786780); senão aos interessados (0x7861e0). Até 100: clube humano (0x7875e0), ao
/// clube; o clube do contrato (0x4cf5a0) se for outro; a seleção.
pub(crate) unsafe extern "thiscall" fn injury_post_injury_news(_this: usize, p: usize, l: usize, arg: u32) {
    let l = InjuryEntry(l);
    let p = Staff(p);
    crate::comparison::branch("cqb_5 injury news");
    let tm = read_usize(G_TRANSFER_MANAGER);
    let mut clubs = [0usize; 30];
    let mut n: i8 = 0;
    if tm != 0 {
        n = exe::transfer_manager_collect_player_news_recipients(tm, p, 0x5a, clubs.as_mut_ptr(), 0x1e) as i8;
    }
    let sel = p.national_job();
    if !sel.is_null() && (0..0x1e).contains(&n) {
        clubs[n as usize] = sel.addr();
        n += 1;
    }
    let mut local = [0u32; 0x38]; // the local news item (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    exe::new_newsitem(nw, 0x7d0, 0);
    exe::newsitem_set_slot(nw, 0, read_usize(p.addr()) as u32);
    for k in 1..4usize {
        let q = read_usize(p.addr() + staff::FIRST_NAME * k);
        exe::newsitem_set_slot(nw, k as i8, if q != 0 { read_usize(q + names::ID) as u32 } else { 0 });
    }
    exe::newsitem_set_slot(nw, 4, read_i8_as_i32(l.addr()) as u32);
    exe::newsitem_set_slot(nw, 5, l.date() as i32 as u32);
    exe::newsitem_set_slot(nw, 6, l.date_year() as u16 as u32);
    exe::newsitem_set_slot(nw, 7, l.days_out() as i32 as u32);
    exe::newsitem_set_slot(nw, 8, p.date_of_birth() as i32 as u32);
    exe::newsitem_set_slot(nw, 9, (p.date_of_birth_year() as i16) as u16 as u32);
    if l.match_mark() != 0xff {
        exe::newsitem_set_slot(nw, 0xa, l.match_club_id() as u32);
        exe::newsitem_set_slot(nw, 0xb, l.match_opponent_id() as u32);
        exe::newsitem_set_slot(nw, 0xc, read_usize(l.addr() + 9) as u32);
    } else {
        exe::newsitem_set_slot(nw, 0xa, u32::MAX);
        exe::newsitem_set_slot(nw, 0xb, u32::MAX);
        exe::newsitem_set_slot(nw, 0xc, u32::MAX);
    }
    exe::newsitem_set_slot(nw, 0xd, arg);
    let club = p.club_job();
    exe::newsitem_set_slot(nw, 0xe, if !club.is_null() { read_usize(club.addr()) as u32 } else { u32::MAX });
    exe::newsitem_set_slot(nw, 0xf, 0x420e9);
    let mut mark: u8 = 0x10;
    exe::write_match_record_to_news(l.addr() + 1, nw, &mut mark);
    let news = read_usize(G_NEWS);
    if l.days_out() > 100 {
        let f = p.player();
        if !f.is_null() && f.world_reputation() > 0x1f40 {
            exe::news_send_to_all_managers(read_usize(G_NEWS), nw);
        } else {
            let mut i: i8 = 0;
            while i < n {
                exe::news_send_to_club_manager(read_usize(G_NEWS), nw, clubs[i as usize]);
                i += 1;
            }
        }
        return;
    }
    if p.club_job().is_null() {
        return;
    }
    if exe::news_club_has_human_manager(news, p.club_job()) == 0 {
        return;
    }
    exe::news_send_to_club_manager(read_usize(G_NEWS), nw, p.club_job());
    let c = injury_main_contract_stored(p.addr());
    if !c.is_null() && injury_club_of_contract(c.addr()) != 0 {
        let x = injury_club_of_contract((injury_main_contract_stored(p.addr())).addr());
        if x != p.club_job().addr() {
            let y = injury_club_of_contract((injury_main_contract_stored(p.addr())).addr());
            exe::news_send_to_club_manager(read_usize(G_NEWS), nw, y);
        }
    }
    let sel = p.national_job();
    if !sel.is_null() {
        exe::news_send_to_club_manager(read_usize(G_NEWS), nw, sel);
    }
}

/// Adds delta to the byte at a (8-bit wrapping, like the original's add/dec).
///
/// [pt] Soma `delta` ao byte `a` (o `add`/`dec` de 8 bits da original).
pub(crate) unsafe fn add_to_byte(a: usize, delta: u8) {
    write_u8(a, read_u8(a).wrapping_add(delta));
}

/// Address of the 31-byte physical record of person index idx in [this].
///
/// [pt] O registro de 31 bytes da pessoa de índice `idx` em [this].
pub(crate) unsafe fn physical_record_by_index(this: usize, idx: i32) -> usize {
    read_usize(this).wrapping_add((idx as u32).wrapping_mul(31) as usize)
}

/// True when the record's recurring injury type (+0x12) has the given category in the type table (0x9e7ad9).
///
/// [pt] A categoria da recorrência (+0x12 do registro, com sinal) é a da tabela de tipos (0x9e7ad9, 13 bytes por tipo)?
pub(crate) unsafe fn is_recurring_category(this: usize, idx: i32, category: u8) -> bool {
    let r = read_i8(physical_record_by_index(this, idx) + physical_record::RECURRING_TYPE);
    r > -1 && *((0x9e7ad9 + (r as isize * 13) as usize) as *const u8) == category
}

/// The comparer's dry run: the copy runs only the training phase (without freeing the knock list, without the day-180
/// fitness, without the new injury or the end routines); `INJURY_DAILY_STOPPED` marks that it has reached a new injury
/// (`add_injury`, which allocates and sends a news item) or the fitness update for everyone.
///
/// [pt] O ensaio do comparador: a cópia roda só a fase do treino (sem soltar a lista das batidas, sem o físico do dia 180,
/// sem a lesão nova nem as rotinas do fim); `INJURY_DAILY_STOPPED` marca que ela chegou a uma lesão nova (0x62d250, que aloca e
/// manda notícia) ou ao físico de todos.
pub(crate) static INJURY_DAILY_DRY_RUN: AtomicBool = AtomicBool::new(false);
pub(crate) static INJURY_DAILY_STOPPED: AtomicBool = AtomicBool::new(false);
/// What the copy left in the regions of the training phase, waiting for the original routine to reach the comeback news
/// items (`post_due_recovery_news`).
///
/// [pt] O que a cópia deixou nas regiões da fase do treino, à espera de a original chegar às notícias da volta (0x62e010).
pub(crate) static mut INJURY_DAILY_PENDING_REGIONS: Option<(Vec<(usize, usize)>, crate::comparison::MemorySnapshot)> = None;

/// O switch dos tipos de lesão (tipo − 0xc, byte da tabela 0x62bcd8): 0 os do +0x1b/+0x36, 1 nenhuma lesão (0x14), 2 o
/// 0x1c, 3 o 0x2d, 4 os 0x3b e 0x3c, 5 só a lesão.
pub(crate) const INJURY_TYPE_CASES: [u8; 0x31] = [
    0, 5, 5, 5, 5, 5, 5, 5, 1, 5, 5, 0, 5, 5, 0, 0, 2, 5, 5, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 3, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 4, 4,
];

/// Daily injury update: frees the match knock list, lowers the injury counters, on day 180 of the first season updates
/// everyone's fitness, draws the training injuries on weekdays, then posts the return news and spreads the outbreaks.
///
/// `Injury::daily_update()`, from the day loop (0x5cccd0) and from advancing to the start date (0x5cc850), every day:
/// frees the match knock list (+0x28, `exe::drop_llist` and free); the two counters (+0x2c, +0x2e) of each injury in the table
/// (0x30 bytes at [+4], count +8) go down by one; on day 180 of the first season (`exe::date_operator_3c` with the start date),
/// everyone's fitness (`update_player_fitness`). Except on Saturday and Sunday (`exe::date_get_weekday`: 5 and 6), the training injuries: [0xae2364]
/// / 150 random draws (at least 10) of a person: a player (+0x61) — otherwise the draw does not count — in jobs 11, 13
/// and 15, present (`is_player_awol`), uninjured (+0xe = −1), passing the risk draws (the player sheet's +0x2d, the proneness
/// +0x1e and the recurrence +0x12/+0x13 of the 31-byte record; rating 20: random(5) against random(2)), one in three
/// and proneness of 20: the category (the recurrence's, or drawn by the bands 0x9e7f05/0x9e7f06), the type (`choose_injury_type`,
/// serious in the first 1/14 of the draws), the attributes the injury wears (+0x1b, +0x36, +0x1d of the player sheet)
/// and the new injury (`add_injury`); outside the season (`injury_player_in_close_season`) nothing; category 7 with a rating below 15 does not
/// count. At the end the return news (`post_due_recovery_news`) and the illness outbreaks (`spread_illness_outbreaks`).
/// Original: `Injury::daily_update` (0x62b7b0, injury.cpp, thiscall, ret 0).
///
/// [pt] { 0x62b7b0 } (thiscall, ret 0, injury.cpp): `Injury::daily_update()`, pelo laço do dia (0x5cccd0) e pela ida até a
/// data do começo (0x5cc850), todo dia: solta a lista das batidas da partida (+0x28, 0x68d040 e free); os dois contadores
/// (+0x2c, +0x2e) de cada lesão da tabela (0x30 bytes em [+4], conta +8) descem um; no dia 180 da primeira temporada
/// (0x456770 com a data do começo), o físico de todos (0x62c680). Fora do sábado e do domingo (0x54a3f0: 5 e 6), as lesões
/// no treino: [0xae2364] / 150 sorteios (pelo menos 10) de uma pessoa: jogador (+0x61) — senão o sorteio não conta —
/// nos cargos 11, 13 e 15, presente (0x418f60), sem lesão (+0xe = −1), passando pelos sorteios de risco (o +0x2d da ficha,
/// a propensão +0x1e e a recorrência +0x12/+0x13 do registro de 31 bytes; nota 20: sorteio(5) contra sorteio(2)), um em
/// três e propensão de 20: a categoria (a da recorrência ou sorteada pelas faixas 0x9e7f05/0x9e7f06), o tipo (0x62d140,
/// grave nos primeiros 1/14 dos sorteios), os atributos que a lesão gasta (+0x1b, +0x36, +0x1d da ficha) e a lesão nova
/// (0x62d250); fora da temporada (0x62ce30) nada; categoria 7 com nota abaixo de 15 não conta. No fim as notícias da volta
/// (0x62e010) e os surtos de doença (0x62e9e0).
pub(crate) unsafe extern "thiscall" fn injury_daily_update(this: usize) {
    let list = read_usize(this + injury_fields::MATCH_HIT_LIST);
    if list != 0 {
        if !INJURY_DAILY_DRY_RUN.load(Relaxed) {
            exe::drop_llist(list);
            exe::crt_operator_delete(list);
        }
        write_u32(this + injury_fields::MATCH_HIT_LIST, 0);
    }
    let base = read_usize(this + injury_fields::INJURY_ENTRIES);
    let mut k = 0i32;
    while k < read_i32(this + injury_fields::INJURY_ENTRY_COUNT) {
        let e = base.wrapping_add((k as u32).wrapping_mul(0x30) as usize) + injury_entry::TRAINING_DAYS_LEFT;
        write_u16(e - 2, read_u16(e - 2).wrapping_sub(1));
        write_u16(e, read_u16(e).wrapping_sub(1));
        k += 1;
    }
    if exe::date_operator_3c(G_CURRENT_DATE, read_usize(G_GAME_START_DATE) as u32, read_usize(G_GAME_START_DATE_FLAGS) as u32) != 0 && read_i16(G_CURRENT_DATE) == 0xb4 {
        if INJURY_DAILY_DRY_RUN.load(Relaxed) {
            INJURY_DAILY_STOPPED.store(true, Relaxed);
            return;
        }
        let mut p = read_usize(G_STAFF_TABLE);
        let mut i = 0i32;
        while i < read_i32(G_STAFF_COUNT) {
            exe::injury_update_player_fitness(this, p);
            i += 1;
            p = p.wrapping_add(0x6e);
        }
    }
    if exe::date_get_weekday(G_CURRENT_DATE) as i16 != 6 && exe::date_get_weekday(G_CURRENT_DATE) as i16 != 5 { // cpw9: no training on Saturday and Sunday
        crate::comparison::branch("cpw_9 training day");
        roll_training_injuries(this);
    }
    if INJURY_DAILY_DRY_RUN.load(Relaxed) {
        return;
    }
    exe::injury_post_due_recovery_news(this);
    exe::injury_spread_illness_outbreaks(this);
}

/// The day's training injuries (middle of `injury_daily_update`): draws players and gives them injuries.
///
/// [pt] As lesões no treino do dia (o meio da 0x62b7b0).
pub(crate) unsafe fn roll_training_injuries(this: usize) {
    let mut how_many = (read_i32(G_STAFF_COUNT) / 150) as i16; // cpw9: the training random draws
    if how_many < 10 {
        how_many = 10;
    }
    let total = how_many as i32;
    let graves = (total / 14) as i16 as i32;
    let mut i = 0i32;
    while i < total {
        let idx = friendly_random(read_i32(G_STAFF_COUNT));
        let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add((idx as u32).wrapping_mul(0x6e) as usize));
        'proximo: {
            if p.player().is_null() {
                i -= 1; // not a player: the random roll does not count
                break 'proximo;
            }
            let job = p.job_for_club();
            if job != 0xb && job != 0xf && job != 0xd {
                break 'proximo;
            }
            if exe::awol_is_player_awol(read_usize(G_AWOL), p, 0) != 0 {
                break 'proximo;
            }
            if p.player().goalkeeper() == 0x14 {
                let r5 = friendly_random(5);
                let r2 = friendly_random(2);
                if r5 > r2 {
                    break 'proximo;
                }
            }
            let rec = physical_record_by_index(this, idx);
            if read_i32(rec + physical_record::CURRENT_INJURY) != -1 {
                break 'proximo;
            }
            if read_i8(rec + physical_record::RECURRING_TYPE) > -1 {
                let rec = physical_record_by_index(this, idx);
                let r = friendly_random(100);
                let lim = ((p.player().injury_proneness() as i32 - 3) * 5 + read_i8(rec + physical_record::PRONENESS) as i32 + read_i8(rec + physical_record::RECURRENCE_GRADE) as i32) * 2;
                if r > lim {
                    break 'proximo;
                }
            } else {
                let r = friendly_random(0xa0);
                let lim = ((p.player().injury_proneness() as i32 - 3) * 5 + read_i8(physical_record_by_index(this, idx) + physical_record::PRONENESS) as i32) * 2; // cpw9: the risk without recurrence
                if r > lim {
                    break 'proximo;
                }
                crate::comparison::branch("cpw_9 risk without recurrence");
            }
            if friendly_random(3) != 0 {
                break 'proximo;
            }
            let rec = physical_record_by_index(this, idx);
            if read_i8(rec + physical_record::PRONENESS) < 0x14 {
                break 'proximo;
            }
            let mut category: u8 = 0;
            let mut draw_category = true;
            if read_i8(rec + physical_record::RECURRING_TYPE) > -1 {
                let r1 = friendly_random(read_i8(rec + physical_record::RECURRENCE_GRADE) as i32 + 2);
                let r2 = friendly_random(3);
                if r1 > r2 {
                    let r = read_i8(physical_record_by_index(this, idx) + physical_record::RECURRING_TYPE) as isize;
                    category = read_u8(0x9e7ad9 + (r * 13) as usize);
                    draw_category = false;
                }
            }
            if draw_category {
                loop {
                    category = friendly_random(0xc) as u8;
                    if category == 0 && friendly_random(100) != 0 {
                        continue;
                    }
                    let c = category as i8 as isize * 3;
                    let n = read_i8(0x9e7f06 + c as usize) as i32 - read_i8(0x9e7f05 + c as usize) as i32 + 2;
                    if friendly_random(n) != 0 {
                        break;
                    }
                }
            }
            if p.player().goalkeeper() < 0xf && category == 7 {
                i -= 1;
                break 'proximo;
            }
            if exe::injury_player_in_close_season(p) != 0 {
                break 'proximo;
            }
            let grave: i8 = if i < graves { -1 } else { 0 };
            let kind = exe::injury_choose_injury_type(this, category as i8, grave, p);
            let k = (kind as i8 as i32).wrapping_sub(0xc);
            let case = if (k as u32) > 0x30 { 5 } else { INJURY_TYPE_CASES[k as usize] };
            if case == 1 {
                break 'proximo; // type 0x14: no injury
            }
            if INJURY_DAILY_DRY_RUN.load(Relaxed) {
                INJURY_DAILY_STOPPED.store(true, Relaxed);
                return;
            }
            apply_injury_attribute_loss(this, idx, p, category, case);
            // simplification: the original pushes the type in a dword whose upper bytes have no value; 0x62d250 reads a char
            (exe::injury_add_injury(this, kind as u32, 0, p, 0, 1, (0xffff_ffff) as u32 as i32, 0, 0) as i32);
        }
        i += 1;
    }
}

/// Applies the attribute losses an injury causes (cases 0, 2, 3, 4 of `injury_daily_update`'s switch) to the player record (+0x61).
///
/// [pt] Os atributos que a lesão gasta (os casos 0, 2, 3 e 4 do switch da 0x62b7b0): a ficha (+0x61) é relida a cada vez.
pub(crate) unsafe fn apply_injury_attribute_loss(this: usize, idx: i32, p: Staff, category: u8, case: u8) {
    let sheet = || p.player();
    match case {
        0 => {
            let a = sheet().acceleration();
            if a == 0x14 {
                if is_recurring_category(this, idx, category) {
                    let f = sheet();
                    let r = friendly_random(3) as u8;
                    add_to_byte(f.addr() + player_sheet::ACCELERATION, 0xffu8.wrapping_sub(r));
                } else if friendly_random(3) != 0 {
                    let f = sheet();
                    let r = friendly_random(2) as u8;
                    add_to_byte(f.addr() + player_sheet::ACCELERATION, 0xffu8.wrapping_sub(r));
                }
            } else if a > 0xf && friendly_random(3) == 0 {
                add_to_byte(sheet().addr() + player_sheet::ACCELERATION, 0xff);
            }
            if sheet().player_pace() == 0x14 {
                if is_recurring_category(this, idx, category) {
                    let f = sheet();
                    let r = friendly_random(3) as u8;
                    add_to_byte(f.addr() + player_sheet::PLAYER_PACE, 0xffu8.wrapping_sub(r));
                } else if friendly_random(3) != 0 {
                    add_to_byte(sheet().addr() + player_sheet::PLAYER_PACE, 0xff);
                }
            } else if sheet().acceleration() > 0x11 && friendly_random(3) == 0 {
                add_to_byte(sheet().addr() + player_sheet::ACCELERATION, 0xff);
            }
        }
        2 => {
            if is_recurring_category(this, idx, category) && sheet().acceleration() == 0x14 && friendly_random(3) == 0 {
                add_to_byte(sheet().addr() + player_sheet::ACCELERATION, 0xff);
            }
        }
        3 => {
            let c = sheet().agility();
            if c == 0x14 {
                write_u8(sheet().addr() + player_sheet::AGILITY, 0x13);
            } else if c > 0xf && friendly_random(3) == 0 {
                add_to_byte(sheet().addr() + player_sheet::AGILITY, 0xff);
            }
        }
        4 => {
            let c = sheet().agility();
            if c == 0x14 {
                if friendly_random(3) != 0 {
                    add_to_byte(sheet().addr() + player_sheet::AGILITY, 0xff);
                }
            } else if c > 0xf && friendly_random(3) == 0 {
                add_to_byte(sheet().addr() + player_sheet::AGILITY, 0xff);
            }
        }
        _ => {}
    }
}

/// At the human managers' clubs, posts the light training return news and updates the daily fitness of injured players;
/// then processes the return queue: entries due today get the news and past entries leave the front.
///
/// `Injury::post_due_recovery_news()`, from the injuries day (`injury_daily_update`), every day; SEH frame only for the local dates.
/// At the clubs of the 16 human managers (the last 16 persons of [0xae23bc]), each squad player (50 slots +0xd7) who is
/// injured (+0xe): on the day when start (+0x20) + days out (+0x2a, more than 5) is reached, the light training return
/// news (`post_recovery_news`, once: +0x1c of the 31-byte record) and the day's fitness (`update_player_fitness`). Then the return queue ([+0x10],
/// 22 bytes: +0 date A, +8 date B, +0x10 id, +0x14 and +0x15 what the news carries, count +0x18): those with date A
/// today (bsearch `exe::crt_bsearch` with `compare_call_up_records_by_date`, stepping back while not before today, `exe::date_operator_3e`) give the news with the days
/// between B and A (`exe::date_get_difference`); those with date B today (bsearch with `injury_compare_entry_dates`) the full return news; those with date B
/// already past leave the front of the queue.
/// Original: `Injury::post_due_recovery_news` (0x62e010, injury.cpp, thiscall, ret 0).
///
/// [pt] { 0x62e010 } (thiscall, ret 0, injury.cpp, quadro SEH só das datas locais): `Injury::post_due_recovery_news()`, pelo
/// dia das lesões (0x62b7b0), todo dia. Nos clubes dos 16 técnicos humanos (as últimas 16 pessoas de [0xae23bc]), cada
/// jogador do elenco (50 vagas +0xd7) lesionado (+0xe): no dia em que começo (+0x20) + dias parados (+0x2a, mais de 5)
/// chega, a notícia da volta ao treino leve (0x630ee0, uma vez: +0x1c do registro de 31 bytes) e o físico do dia
/// (0x62c680). Depois a fila das voltas ([+0x10], 22 bytes: +0 data A, +8 data B, +0x10 id, +0x14 e +0x15 o que a
/// notícia leva, conta +0x18): as de data A hoje (bsearch 0x946464 com 0x778e90, voltando enquanto não for antes de hoje,
/// 0x456740) dão a notícia com os dias entre B e A (0x54cc30); as de data B hoje (bsearch com 0x632040) a notícia da volta
/// completa; as de data B já passada saem do começo da fila.
pub(crate) unsafe extern "thiscall" fn post_due_recovery_news(this: usize) {
    crate::comparison::branch("cpw_9 news back");
    for k in 0..0x10i32 {
        let n = read_i32(G_STAFF_COUNT);
        let s = Staff(read_usize(G_STAFF_TABLE).wrapping_add(((n - k - 1) as u32).wrapping_mul(0x6e) as usize));
        let club = s.club_job();
        if club.is_null() {
            continue;
        }
        for j in 0..50usize {
            let p = club.squad(j);
            if p == 0 || read_i32(p) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            let i = read_i32(physical_record_by_index(this, read_i32(p)) + physical_record::CURRENT_INJURY);
            if i == -1 {
                continue;
            }
            let inj = InjuryEntry(read_usize(this + injury_fields::INJURY_ENTRIES).wrapping_add((i as u32).wrapping_mul(0x30) as usize));
            let (dia, year) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
            let mut t = [0u8; 8];
            let r = exe::date_add_days(inj.addr() + injury_entry::DATE, (t.as_mut_ptr()) as usize, (inj.training_return_days() as u16) as u32);
            if year == read_u16(r + 2) && dia == read_i16(r) && inj.training_return_days() > 5 {
                let mark = physical_record_by_index(this, read_i32(p)) + physical_record::TRAINING_NEWS_SENT;
                if read_u8(mark) == 0 {
                    write_u8(mark, 1);
                    let days = (inj.days_out() as u16).wrapping_sub((inj.training_return_days() as u16));
                    // simplification: the upper bytes of the 8- and 16-bit arguments are uninitialized in the original; 0x630ee0 reads char/short
                    exe::injury_post_recovery_news(this, p, 0, read_u8(inj.addr()) as u32, 0, days as u32);
                }
            }
            exe::injury_update_player_fitness(this, p);
        }
    }
    if read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE) == 0 {
        return;
    }
    let (mut d1, mut d2, mut t) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    exe::date_date_54a110((d1.as_mut_ptr()) as usize);
    exe::date_date_54a110((d2.as_mut_ptr()) as usize);
    exe::date_operator_3d((d1.as_mut_ptr()) as usize, (t.as_mut_ptr()) as usize, G_CURRENT_DATE);
    exe::date_operator_3d((d2.as_mut_ptr()) as usize, (t.as_mut_ptr()) as usize, G_CURRENT_DATE);
    let pd1 = d1.as_ptr() as usize;
    let (d1dia, d1ano) = (read_i16(pd1), read_u16(pd1 + 2));
    let pd2 = d2.as_ptr() as usize;
    let (d2dia, d2ano) = (read_i16(pd2), read_u16(pd2 + 2));
    let last = || read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE).wrapping_add((read_i32(this + injury_fields::RECOVERY_NEWS_COUNT) as u32).wrapping_mul(0x16) as usize).wrapping_sub(0x16);
    let person = |e: usize| read_usize(G_STAFF_TABLE).wrapping_add((read_usize(e) as u32).wrapping_mul(0x6e) as usize);
    // those with date A today
    let mut e = exe::crt_bsearch((d1.as_ptr()) as usize, read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE), read_i32(this + injury_fields::RECOVERY_NEWS_COUNT) as u32, 0x16, 0x778e90);
    if e != 0 {
        if e > read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE) {
            loop {
                e -= 0x16;
                let (dia, year) = (read_i16(e), read_u16(e + 2));
                if year > d1ano || (year == d1ano && dia > d1dia) || exe::date_operator_3e((d1.as_ptr()) as usize, read_usize(e) as u32, read_usize(e + 4) as u32) != 0 {
                    e += 0x16;
                    break;
                }
                if e <= read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE) {
                    break;
                }
            }
        }
        while e <= last() {
            let (dia, year) = (read_i16(e), read_u16(e + 2));
            if year > d1ano || (year == d1ano && dia > d1dia) {
                break;
            }
            if exe::date_operator_3e((d1.as_ptr()) as usize, read_usize(e) as u32, read_usize(e + 4) as u32) != 0 {
                break;
            }
            let p = person(e + recovery_news_entry::PERSON_ID);
            let mut dif = [0u8; 8];
            exe::date_get_difference(e + 8, dif.as_mut_ptr(), read_usize(e) as u32, read_usize(e + 4) as u32);
            let pd = dif.as_ptr() as usize;
            let days = (read_u16(pd + 2) as u32).wrapping_mul(365).wrapping_add(read_u16(pd) as u32) as u16;
            exe::injury_post_recovery_news(this, p, 0, read_u8(e + recovery_news_entry::INJURY_TYPE) as u32, read_u8(e + recovery_news_entry::FORCE_DELIVERY) as u32, days as u32);
            e += 0x16;
        }
    }
    // those with date B today: the full loop
    let mut e = exe::crt_bsearch((d1.as_ptr()) as usize, read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE), read_i32(this + injury_fields::RECOVERY_NEWS_COUNT) as u32, 0x16, 0x632040);
    if e == 0 {
        return;
    }
    let diferente = |b: usize| {
        let (dia, year) = (read_i16(b), read_u16(b + 2));
        year != d2ano || dia != d2dia
    };
    if e > read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE) {
        loop {
            e -= 0x16;
            if diferente(e + 8) {
                e += 0x16;
                break;
            }
            if e <= read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE) {
                break;
            }
        }
    }
    while e <= last() {
        if diferente(e + 8) {
            break;
        }
        let p = person(e + recovery_news_entry::PERSON_ID);
        exe::injury_post_recovery_news(this, p, 1, read_u8(e + recovery_news_entry::INJURY_TYPE) as u32, read_u8(e + recovery_news_entry::FORCE_DELIVERY) as u32, 0);
        e += 0x16;
    }
    // those whose date B has already passed leave the front of the queue
    let n = read_i32(this + injury_fields::RECOVERY_NEWS_COUNT);
    let mut remove: i16 = 0;
    if n > 0 {
        let (hd, ha) = (read_i16(G_CURRENT_DATE), read_u16(G_CURRENT_DATE_YEAR));
        let mut q = read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE) + recovery_news_entry::FULL_RETURN_DATE_YEAR;
        let mut i = 0i32;
        while i < n {
            let year = read_u16(q);
            if ha < year || (ha == year && !(hd > read_i16(q - 2))) {
                break;
            }
            remove = remove.wrapping_add(1);
            i += 1;
            q += 0x16;
        }
        if remove != 0 && remove as i32 != n {
            let base = read_usize(this + injury_fields::RECOVERY_NEWS_QUEUE);
            let t = remove as i32 as usize;
            core::ptr::copy((base + t * 0x16) as *const u8, base as *mut u8, (n as usize - t) * 0x16);
        }
    }
    write_i32(this + injury_fields::RECOVERY_NEWS_COUNT, read_i32(this + injury_fields::RECOVERY_NEWS_COUNT).wrapping_sub(remove as i32));
}

// simplification: the second-leg news (0x630ee0) allocates and the day’s fitness touches the random roll: original only; the copy takes the
// branch. It is also the checkpoint of the training stage on the injuries day (compare_injury_daily_update).

/// (id, person) of a 0x49-byte match slot (id at -0x30), or None when no id or out of range.
///
/// [pt] A pessoa da vaga de 0x49 bytes da partida (id em −0x30): sem id (−1) ou com id fora de [0xae2364], nenhuma. A
/// original confere o ponteiro nulo antes do id (ponteiro nulo passa).
pub(crate) unsafe fn staff_of_match_slot(slot: usize) -> Option<(i32, usize)> {
    let id = read_i32(slot - 0x30);
    if id == -1 {
        return None;
    }
    let person = (read_i32(G_STAFF_TABLE) as u32).wrapping_add((id as u32).wrapping_mul(0x6e)) as usize;
    if person != 0 && read_i32(person) >= read_i32(G_STAFF_COUNT) {
        return None;
    }
    Some((id, person))
}

/// Slot level used for match rhythm: +4 if not negative, else +0, else the match default (+0x4d) [inference].
///
/// [pt] O nível da vaga para o ritmo: +4 se não for negativo, senão +0, senão o padrão da partida (+0x4d).
pub(crate) unsafe fn slot_rhythm_level(fixture: MatchRecord, slot: usize) -> i8 {
    let b = read_i8(slot + 4);
    if b >= 0 {
        return b;
    }
    let b = read_i8(slot);
    if b >= 0 {
        b
    } else {
        fixture.minute() as i8 
    }
}

/// Applies a match's physical effects: for the 20 slots of each side, the recovery at a rate based on the slot's level
/// and condition; the person's knocks in the match list become injuries.
///
/// `Injury::apply_match_effects(game, match)`, from close_day_match (`close_day_match`) and from the simulated statistics
/// (0x84be20), for every match: for both sides of the match (0x625 bytes from +0x1c0), the 20 slots of 0x49 bytes with
/// a person (id at −0x30). The rate: the level (+0x5a in the first eleven with the match flag +0x4c; otherwise the
/// slot's, minus +2 for the substitutes, which with 0xff gives 0) plus (10000 − condition +6) / 100, in 32 bits (the
/// level enters only in the low 16). A game without the flag +0x40: half (16 bits). With it: above 33 + random(3) years
/// (+0x18), × 1.25 (0x968928, _ftol); otherwise below 23 − random(5), double. Before the start of the game ([0xdd6ec8]
/// after today, `exe::date_operator_3e`): 0. The recovery (`recover_injury`) with the condition and the rate. With the knock (−0x11) and the
/// match knock list (+0x28): each knock of the person (type 7 only with player sheet +0xf of 15 or more) has the
/// severity by +5 (above 95: 2; above 66: 1; above 51: 0; otherwise the previous knock's) and the injury type
/// (`choose_injury_type`); a valid type: end of the current injury (`end_player_injury`) and the new injury (`add_injury` with the game, or 0 for
/// game −2).
/// Original: `Injury::apply_match_effects` (0x62c080, injury.cpp, thiscall, ret 8).
///
/// [pt] { 0x62c080 } (thiscall, ret 8, injury.cpp): `Injury::apply_match_effects(jogo, partida)`, pela close_day_match
/// (0x6b5f00) e pelas estatísticas simuladas (0x84be20), em toda partida: para os dois lados da partida (0x625 bytes a
/// partir de +0x1c0), as 20 vagas de 0x49 bytes com pessoa (id em −0x30). O ritmo: o nível (+0x5a nas onze primeiras
/// com a marca +0x4c da partida; senão o da vaga, menos o +2 nas reservas, que com 0xff dá 0) mais (10000 − condição
/// +6) / 100, em 32 bits (o nível entra só nos 16 de baixo). Jogo sem a marca +0x40: a metade (16 bits). Com ela: acima
/// de 33 + exe::utils_random(3) anos (+0x18), × 1,25 (0x968928, _ftol); senão abaixo de 23 − exe::utils_random(5), o dobro. Antes do começo
/// do jogo ([0xdd6ec8] depois de hoje, 0x456740): 0. A recuperação (0x62bd10) com a condição e o ritmo. Com a batida
/// (−0x11) e a lista das batidas da partida (+0x28): cada batida da pessoa (o tipo 7 só com a ficha +0xf de 15 ou
/// mais) tem a gravidade por +5 (acima de 95: 2; de 66: 1; de 51: 0; senão a da batida anterior) e o tipo da lesão
/// (0x62d140); tipo válido: fim da lesão atual (0x62e700) e a lesão nova (0x62d250 com o jogo, ou 0 no jogo −2).
pub(crate) unsafe extern "thiscall" fn apply_match_effects(this: usize, game: usize, fixture: usize) {
    let fixture = MatchRecord(fixture);
    let game = Fixture(game);
    // simplification: the severity is a local of the original that carries over from one hit to the next; with no hit above 51 before,
    // the original reads a stack leftover and the copy uses 0 (the comparer leaves every match with a clash to the original).
    let mut severity: i8 = 0;
    for lado in 0..2usize {
        let base = fixture.addr() + match_record::ROW_MINUTE_OFF + match_record::SIDE_SIZE * lado;
        for k in 0..0x14i8 {
            let slot = base + 0x49 * k as usize;
            let Some((id, pessoa)) = staff_of_match_slot(slot) else { continue };
            let mark = (fixture.no_statistics() as i8) != 0;
            let level: i8 = if k < 0xb {
                if mark { 0x5a } else { slot_rhythm_level(fixture, slot) }
            } else if read_u8(slot + 2) == 0xff {
                0
            } else {
                slot_rhythm_level(fixture, slot).wrapping_sub(read_i8(slot + 2))
            };
            let c = 0x2710i32.wrapping_sub(read_i16(slot + 6) as i32);
            // simplification: the original adds the whole ecx: the level in the low 16, the upper 16 of 10000 − condition
            let mut v = (c / 100).wrapping_add(((c as u32 & 0xffff_0000) | level as i16 as u16 as u32) as i32); // cpv8: o ritmo
            crate::comparison::branch("cpv_8 pace");
            if (game.home_side_mark() as i8) == 0 {
                crate::comparison::branch("cpv_8 half");
                v = (v as i16 as i32) / 2; // cpv8: without the match mark, half
            } else if read_i8(pessoa + staff::AGE) as i32 > exe::utils_random(3) + 0x21 {
                v = ((v as i16 as i32) as f64 * read_f64(rdata::F64_1_25)) as i32;
            } else if (read_i8(pessoa + staff::AGE) as i32) < 0x17 - exe::utils_random(5) {
                crate::comparison::branch("cpv_8 youngster");
                v = v.wrapping_add(v); // cpv8: the young player counts double
            }
            if exe::date_operator_3e(G_GAME_START_DATE, read_i32(G_CURRENT_DATE) as u32, read_i32(G_CURRENT_DATE_LEAP) as u32) != 0 {
                v = 0;
            }
            exe::injury_set_match_condition(this, pessoa, read_i16(slot + 6), v as i16);
            if read_u8(slot - 0x11) == 0 || read_i32(this + injury_fields::MATCH_HIT_LIST) == 0 {
                continue;
            }
            let list = Llist(read_i32(this + injury_fields::MATCH_HIT_LIST) as usize);
            list.set_cursor(((list.head() as u32)) as usize);
            let mut e = exe::llist_get_next(read_i32(this + injury_fields::MATCH_HIT_LIST) as usize);
            while e != 0 {
                let kind = read_i8(e + 4);
                if read_i32(e) == id && (kind != 7 || read_i8(read_i32(pessoa + staff::PLAYER) as usize + player_sheet::GOALKEEPER) >= 0xf) {
                    let g = read_i8(e + 5);
                    if g > 0x5f {
                        severity = 2;
                    } else if g > 0x41 {
                        severity = 1;
                    } else if g > 0x32 {
                        severity = 0;
                    }
                    let t = exe::injury_choose_injury_type(this, kind, severity, pessoa);
                    if (t as i8) >= 0 {
                        crate::comparison::branch("cpv_8 collision becomes injury");
                        exe::injury_end_player_injury(this, pessoa, 0);
                        let j = if read_i32(game.addr()) == -2 { 0 } else { game.addr() };
                        exe::injury_add_injury(this, t as u32, 0, pessoa, j as u32, 1, -1, 0, 0);
                    }
                }
                e = exe::llist_get_next(read_i32(this + injury_fields::MATCH_HIT_LIST) as usize);
            }
        }
    }
}

/// True when the person has an entry in the match's injury list that reaches the injury (same check as `apply_match_effects`);
/// cursor restored.
///
/// [pt] A pessoa tem batida na lista da partida que chega à lesão (a mesma conferência da 0x62c080)? O cursor (+0xa) volta
/// ao que era.
pub(crate) unsafe fn has_injury_hit(this: usize, id: i32, person: Staff) -> bool {
    let list = Llist(read_i32(this + injury_fields::MATCH_HIT_LIST) as usize);
    let cursor = list.cursor() as u32;
    list.set_cursor(((list.head() as u32)) as usize);
    let mut found = false;
    let mut e = exe::llist_get_next(list);
    while e != 0 && !found {
        found = read_i32(e) == id && (read_i8(e + 4) != 7 || read_i8((person.player().addr() as i32) as usize + player_sheet::GOALKEEPER) >= 0xf);
        e = exe::llist_get_next(list);
    }
    list.set_cursor((cursor) as usize);
    found
}

/// Returns the match knock list (+0x28) item with the person's id, or 0.
///
/// `Injury::find_match_injury(person)`: the item of the match knock list (+0x28; cursor +0xa restarts from +2,
/// `next_list_item`) with the person's id, or 0 (also with no list). Used by the staff screens (0x870650, 0x885820).
/// Original: `Injury::find_match_injury` (0x62bf60, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x62bf60 } (thiscall, ret 4, injury.cpp): `Injury::find_match_injury(pessoa)`: o item da lista das batidas da
/// partida (+0x28; cursor +0xa recomeça de +2, 0x68d1b0) com o id da pessoa, ou 0 (sem lista também). Pelas telas do
/// pessoal (0x870650, 0x885820).
pub(crate) unsafe extern "thiscall" fn find_match_injury(this: usize, person: usize) -> usize {
    let list = Llist(read_i32(this + injury_fields::MATCH_HIT_LIST) as usize);
    if list.is_null() {
        return 0;
    }
    list.set_cursor(((list.head() as u32)) as usize);
    let mut e = exe::llist_get_next(read_i32(this + injury_fields::MATCH_HIT_LIST) as usize);
    while e != 0 {
        if read_i32(e) == read_i32(person) {
            return e;
        }
        e = exe::llist_get_next(read_i32(this + injury_fields::MATCH_HIT_LIST) as usize);
    }
    0
}

/// 1 if the person's current injury (`exe::injury_get_player_injury` on [0xb63d28]) is a holiday (type 0x51)
/// Original: `Injury::is_player_on_holiday` (0x631550, injury.cpp, thiscall, ret 4 (this não é lido)).
///
/// [pt] { 0x631550 } (ret 4, injury.cpp; `this` não é lido): `Injury::is_player_on_holiday(pessoa)`: 1 se a lesão atual
/// da pessoa (0x62ed70 no [0xb63d28]) é de férias (tipo 0x51). Pela tela do clube (0x4849f0).
pub(crate) unsafe extern "thiscall" fn is_player_on_holiday(_this: usize, person: usize) -> u8 {
    let e = exe::injury_get_player_injury(read_i32(G_INJURY) as usize, person);
    (e != 0 && *(e as *const u8) == 0x51) as u8
}

/// Address of person id's 31-byte physical record in [lesoes] (current injury at +0xe).
///
/// [pt] O registro de 31 bytes da pessoa `id` em [lesoes] (o físico e a lesão atual em +0xe).
pub(crate) unsafe fn physical_record_of_id(injuries: usize, id: i32) -> usize {
    (read_i32(injuries) as u32).wrapping_add((id as u32).wrapping_mul(31)) as usize
}

/// Ends the person's current injury: the history (except holidays), the return news at a human club (when asked), entry
/// type 0xff and the record's +0xe set to −1.
///
/// `Injury::end_player_injury(person, news)`, from the day's fitness (`update_player_fitness`, with news), from the match knocks
/// (`apply_match_effects`, without) and from the fitness restart (`injury_reset_player`, without): with a current injury (+0xe of the record,
/// index into the 0x30-byte table at [this+4]; −1 none): except holidays (type 0x51), the history ([this+0xc], 0x58e340
/// with +0x14 and +0x18 of the record); with `news` and the person's club (+0x39) human (`exe::human_manager_is_human_club`), the return news
/// (`post_recovery_news` with 1 and the type). The type becomes 0xff and +0xe −1. Without a person: the error box (line 0xc87).
/// Original: `Injury::end_player_injury` (0x62e700, injury.cpp, thiscall, ret 8).
///
/// [pt] { 0x62e700 } (thiscall, ret 8, injury.cpp): `Injury::end_player_injury(pessoa, noticia)`, pelo físico do dia
/// (0x62c680, com notícia), pelas batidas da partida (0x62c080, sem) e pelo recomeço do físico (0x62bfa0, sem): com a
/// lesão atual (+0xe do registro, índice na tabela de 0x30 bytes em [this+4]; −1 nenhuma): fora das férias (tipo
/// 0x51), o histórico ([this+0xc], 0x58e340 com +0x14 e +0x18 do registro); com `noticia` e o clube da pessoa (+0x39)
/// humano (0x600a50), a notícia da volta (0x630ee0 com 1 e o tipo). O tipo vira 0xff e o +0xe −1. Sem pessoa: a caixa
/// de erro (linha 0xc87).
pub(crate) unsafe extern "thiscall" fn end_player_injury(this: usize, person: usize, news: u32) {
    let person = Staff(person);
    if person.is_null() {
        show_error_at(0x9e7f28, 0xc87);
        return;
    }
    let id = read_i32(person.addr());
    if id >= read_i32(G_STAFF_COUNT) { // cpv10: the person id
        return;
    }
    if id >= read_i32(G_STAFF_COUNT) / 2 {
        crate::comparison::branch("cpv_10 top half");
    }
    let r = PhysicalRecord(physical_record_of_id(this, id));
    let i = r.current_injury();
    if i == -1 { // cpv10: the current injury
        return;
    }
    if i == 0 {
        crate::comparison::branch("cpv_10 zero entry");
    }
    let e = (read_i32(this + injury_fields::INJURY_ENTRIES) as u32).wrapping_add((i as u32).wrapping_mul(0x30)) as usize;
    if read_u8(e) != 0x51 {
        exe::file_llist_add_node(read_i32(this + injury_fields::INJURY_HISTORY) as usize, r.addr() + physical_record::HISTORY_FIRST, r.addr() + physical_record::HISTORY_LAST, e as *const u8);
    }
    let club = (person.club_job().addr() as i32) as usize;
    if news != 0 && club != 0 {
        if exe::human_manager_is_human_club(club, 0, 1, 0, 0) != 0 {
            // simplification: the original pushes the whole ecx with the type in the low byte; 0x630ee0 reads the byte
            exe::injury_post_recovery_news(this, person, 1, read_u8(e) as u32, 0, 0);
        }
    }
    crate::comparison::branch("cpv_10 end of injury");
    write_u8(e, 0xff); // cpv10: a entrada fica livre
    write_i32(physical_record_of_id(this, read_i32(person.addr())) + physical_record::CURRENT_INJURY, -1); // cpv10: no current injury
}

/// Memory regions `clear_inbox` writes: the box and, with a first news item, +0xd6..+0xde of the 100 news items.
///
/// [pt] As regiões que a 0x787630 grava (a caixa e, com a primeira notícia, +0xd6..+0xde das 100 notícias).
pub(crate) unsafe fn news_box_regions(this: usize, caixa: usize) -> Vec<(usize, usize)> {
    let mut reg = vec![(caixa, 12)];
    let first = read_i32(caixa);
    if first != -1 {
        for k in 0..0x64u32 {
            reg.push(((read_i32(this) as u32).wrapping_add(k.wrapping_add(first as u32).wrapping_mul(0xdf)) as usize + news_item::ID, 8));
        }
    }
    reg
}

/// The person's injury status from the age of the current injury: 2 injured, 1 regaining fitness, 0 fit.
///
/// The current injury is the index at [this] + id × 31 + 0xe (−1 without an injury). The difference from the game date
/// to the injury date (+0x20 of the 48-byte record at [this+4], `exe::date_get_difference`) in years × 365 + days: below +0x2a, 2
/// (injured); below +0x28, 1 (regaining fitness); otherwise 0.
///
/// [pt] O estado pela lesão atual da pessoa (o índice em [this] + id × 31 + 0xe; −1 sem lesão): a diferença da data do jogo
/// para a da lesão (+0x20 do registro de 48 bytes em [this+4], 0x54cc30) em anos × 365 + dias; abaixo de +0x2a, 2
/// (lesionado); abaixo de +0x28, 1 (recuperando a forma); senão 0.
pub(crate) unsafe fn injury_status(this: usize, person: usize) -> u8 {
    let id = read_usize(person);
    let k = read_usize(read_usize(this).wrapping_add(id.wrapping_mul(31)) + physical_record::CURRENT_INJURY) as i32;
    if k == -1 {
        return 0;
    }
    let l = InjuryEntry(read_usize(this + injury_fields::INJURY_ENTRIES).wrapping_add((k as usize).wrapping_mul(48)));
    let mut t = [0u8; 8];
    exe::date_get_difference(G_CURRENT_DATE, t.as_mut_ptr(), read_usize(l.addr() + injury_entry::DATE) as u32, l.date_leap_year() as u32);
    let days = (read_u16(t.as_ptr() as usize + 2) as i32).wrapping_mul(365).wrapping_add(read_i16(t.as_ptr() as usize) as i32);
    if days < l.training_return_days() as i32 {
        return 2; // cpr10: lesionado
    }
    (days < l.days_out() as i32) as u8 // cpr10: recuperando a forma
}

/// updates the daily fitness (`update_player_fitness`) and returns 2 while the injury lasts, 1 while recovering match fitness and 0
/// when fit, by the gap from the injury date to the game date
/// Original: `Injury::get_player_injury_status` (0x62c400, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x62c400 } (thiscall, ret 4, injury.cpp): `Injury::get_player_injury_status(pessoa)`: pessoa nula, a caixa
/// "Error" (linha 0x556) e 0; id a partir de [0xae2364], 0. Senão a forma do dia (0x62c680) e o estado
/// (injury_status). Quente (escalação automática): sem contador.
pub(crate) unsafe extern "thiscall" fn get_player_injury_status(this: usize, person: usize) -> u8 {
    if person == 0 {
        show_error_at(0x9e7f28, 0x556);
        return 0;
    }
    if read_usize(person) as i32 >= read_usize(G_STAFF_COUNT) as i32 {
        return 0;
    }
    exe::injury_update_player_fitness(this, person);
    injury_status(this, person)
}

/// Date d + n days into t (Date::add_days); returns the result.
pub(crate) unsafe fn injury_date_add_days(d: usize, n: u32, t: &mut [u8; 8]) -> usize {
    exe::date_add_days(d, (t.as_mut_ptr()) as usize, n)
}

/// Assigns the date at p to date d (`exe::date_operator_3d`).
///
/// [pt] A data `d` = a data em `p` (0x417c10).
pub(crate) unsafe fn injury_assign_date(d: usize, p: usize) {
    let mut t = [0u8; 8];
    exe::date_operator_3d(d, (t.as_mut_ptr()) as usize, p);
}

/// Sets date d = game date + n days.
///
/// [pt] A data `d` = a data do jogo + `n` dias (0x54c5b0, 0x417c10).
pub(crate) unsafe fn set_today_plus_days(d: usize, n: u32) {
    let mut t = [0u8; 8];
    injury_assign_date(d, injury_date_add_days(G_CURRENT_DATE, n, &mut t));
}

/// Stores slice f in entry +0xc; above 2 subtracts 3 and moves the entry date (+4) to the next day.
///
/// [pt] A data `d` passa para o dia seguinte quando a fatia (+0xc da entrada, com sinal) passa de 2 (menos 3).
pub(crate) unsafe fn advance_time_slice(e: usize, f: i8) {
    write_i8(e + decision_date::SLICE, f);
    if f > 2 {
        write_i8(e + decision_date::SLICE, f.wrapping_sub(3));
        let mut t = [0u8; 8];
        injury_assign_date(e + 4, injury_date_add_days(e + 4, 1, &mut t));
    }
}

/// Finds the person's type-3 decision entry in the 14-byte decision array ([this+0xc], count +0x14), optionally voiding
/// it; returns its address or 0.
///
/// From position +0x18, among the entries up to `limite` (year unsigned, day signed), the person's first type-3 entry
/// is voided (id −1; without `alterar` it is only skipped) and the search stops; then, from that position (or from
/// count − [this+0x1c], if larger), it finds the person's type-3 entry. Returns its address or 0.
///
/// [pt] A entrada de decisão de tipo 3 da pessoa no vetor de 14 bytes ([this+0xc], conta +0x14): da posição +0x18, nas
/// entradas até `limite` (ano sem sinal, dia com sinal), a primeira de tipo 3 da pessoa é anulada (id −1; sem
/// `alterar` só fica de fora) e a busca para; depois, a partir dessa posição (ou da conta − [this+0x1c], se maior),
/// a de tipo 3 da pessoa. Devolve o endereço dela ou 0.
pub(crate) unsafe fn find_type3_decision(this: usize, id: i32, limit: &[u8; 8], alterar: bool) -> usize {
    let count = read_usize(this + contract_manager_fields::DECISION_COUNT) as i32;
    let base = read_usize(this + contract_manager_fields::DECISION_DATES);
    let (la, ld) = (u16::from_le_bytes([limit[2], limit[3]]), i16::from_le_bytes([limit[0], limit[1]]));
    let mut i = read_usize(this + contract_manager_fields::DECISION_NEXT_INDEX) as i32;
    let mut cancelled = -1i32;
    while i < count {
        let r = base.wrapping_add((i as usize).wrapping_mul(14));
        let ra = read_u16(r + 6);
        if la < ra || (la == ra && ld < read_i16(r + decision_date::DATE)) {
            break;
        }
        if read_u8(r + decision_date::KIND) == 3 && read_usize(r) as i32 == id {
            if alterar {
                write_u32(r, u32::MAX);
            }
            cancelled = i;
            break;
        }
        i += 1;
    }
    let count = read_usize(this + contract_manager_fields::DECISION_COUNT) as i32;
    i = i.max(count.wrapping_sub(read_usize(this + contract_manager_fields::DECISIONS_ADDED) as i32));
    while i < count {
        let r = read_usize(this + contract_manager_fields::DECISION_DATES).wrapping_add((i as usize).wrapping_mul(14));
        if read_u8(r + decision_date::KIND) == 3 && read_usize(r) as i32 == id && i != cancelled {
            return r;
        }
        i += 1;
    }
    0
}

/// As datas estáticas da `exe::date_add_days`, da 0x54c8e0 e da `exe::date_days_since` (valor e guarda).
pub(crate) const INJURY_DATE_STATICS: [(usize, usize); 6] = [(G_ADD_DAYS_RESULT_DATE, 8), (0xae2469, 1), (G_SUB_DAYS_RESULT_DATE, 8), (0xae249e, 1), (G_DAYS_SINCE_SCRATCH_DATE, 8), (0xae2418, 1)];

/// Club record (0x245 bytes) of index i in G_CLUB_TABLE.
///
/// [pt] O clube pelo índice (registro de 0x245 bytes em [0xae23b4]).
pub(crate) unsafe fn injury_club_by_index(i: i32) -> Club {
    Club(read_usize(G_CLUB_TABLE).wrapping_add((i as usize).wrapping_mul(club::SIZE)))
}

/// Calls Database is_national_team for the club.
pub(crate) unsafe fn injury_is_national_team(club: Club) -> bool {
    exe::database_is_national_team(club) != 0
}

/// Um surto (17 bytes, vetor [this+0x1c], capacidade +0x20, conta +0x24): +0 a data (8 bytes), +8 o tipo da doença,
/// +9 o índice do clube, +0xd o id da partida (−1 sem).
pub(crate) const OUTBREAK_SIZE: usize = 0x11;

/// Whether an illness outbreak of a type takes hold at the club (random draws as `add_illness_outbreak`; never type 4 or national
/// teams).
///
/// [pt] Se o surto entra (os sorteios da 0x62e820, na ordem): tipo 4 nunca; tipo 3 só com 1 em 15 (sorteio de 15 = 0); os
/// outros com 3 em 4 (sorteio de 4 ≠ 0); e nunca numa seleção (0x53b900).
pub(crate) unsafe fn outbreak_takes_hold(kind: u8, club_i: i32) -> bool {
    if kind == 4 {
        return false;
    }
    if kind == 3 {
        if friendly_random(0xf) != 0 {
            return false;
        }
    } else if friendly_random(4) == 0 { // 1 in 4 is left out
        return false;
    }
    !injury_is_national_team(Club((injury_club_by_index(club_i)).addr()))
}

/// Stores an illness outbreak (date, type, club, fixture) if it takes hold (type 4 never; type 3 with 1 in 15, the
/// others with 3 in 4; outside national teams) and re-sorts the array by date.
///
/// `Injury::add_illness_outbreak(date, type, club, match)`: the type is the low byte; if the outbreak takes hold
/// (`outbreak_takes_hold`), the vector grows by 30 when full (`exe::crt_realloc`, or `exe::crt_malloc` with no vector; out of memory, the
/// "Error" box for line 0xd01 and nothing), the outbreak is written at the end (date through `exe::date_operator_3d`) and the vector
/// is re-sorted by date (qsort `exe::crt_qsort` with `compare_call_up_records_by_date`).
/// Original: `Injury::add_illness_outbreak` (0x62e820, injury.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x62e820 } (thiscall, ret 0x10, injury.cpp): `Injury::add_illness_outbreak(data, tipo, clube, partida)`: o tipo
/// é o byte baixo; se o surto entra (`outbreak_takes_hold`), cresce o vetor de 30 em 30 quando cheio (0x945667, ou
/// 0x944e46 sem vetor; sem memória, a caixa "Error" da linha 0xd01 e nada), grava o surto no fim (data pela 0x417c10)
/// e reordena pela data (qsort 0x945333 com 0x778e90).
pub(crate) unsafe extern "thiscall" fn add_illness_outbreak(this: usize, data: usize, kind: u32, club_i: i32, fixture: usize) {
    let kind = kind as u8;
    if !outbreak_takes_hold(kind, club_i) {
        return;
    }
    crate::comparison::branch("cpp_8 new outbreak");
    let cap = read_i32(this + injury_fields::OUTBREAK_CAPACITY);
    if read_i32(this + injury_fields::OUTBREAK_COUNT) >= cap {
        let cap = cap.wrapping_add(30);
        write_i32(this + injury_fields::OUTBREAK_CAPACITY, cap);
        let size = (cap as u32).wrapping_mul(OUTBREAK_SIZE as u32);
        let v = read_usize(this + injury_fields::OUTBREAKS);
        let new = if v != 0 {
            exe::crt_realloc(v, size)
        } else {
            exe::crt_malloc(size)
        };
        write_usize(this + injury_fields::OUTBREAKS, new);
        if new == 0 {
            show_error_at(0x9e7f28, 0xd01);
            return;
        }
    }
    let e = read_usize(this + injury_fields::OUTBREAKS).wrapping_add((read_i32(this + injury_fields::OUTBREAK_COUNT) as usize).wrapping_mul(OUTBREAK_SIZE));
    let mut draft = [0u8; 8];
    exe::date_operator_3d(e, (draft.as_mut_ptr()) as usize, data);
    write_u8(e + 8, kind);
    write_i32(e + 9, club_i);
    let id = if fixture != 0 { read_i32(fixture) } else { -1 };
    write_i32(e + illness_outbreak::CLUB_ID, id);
    let n = read_i32(this + injury_fields::OUTBREAK_COUNT).wrapping_add(1);
    write_i32(this + injury_fields::OUTBREAK_COUNT, n);
    exe::crt_qsort(read_usize(this + injury_fields::OUTBREAKS), n as u32, OUTBREAK_SIZE as u32, 0x778e90);
}

/// Game date minus five days (Date::sub_days).
///
/// [pt] A data de cinco dias atrás (0x54c8e0 sobre a data do jogo).
pub(crate) unsafe fn five_days_ago() -> [u8; 8] {
    let mut d = [0u8; 8];
    exe::date_sub_days(G_CURRENT_DATE, (&mut d) as *mut [u8; 8] as usize, 5); // five days ago
    d
}

/// True when date limit is after outbreak e's date (unsigned year, then signed day).
///
/// [pt] A data `limite` (dia com sinal em +0, ano sem sinal em +2) é depois da do surto `e`?
pub(crate) unsafe fn is_after_outbreak(limit: &[u8; 8], e: usize) -> bool {
    let day = i16::from_le_bytes([limit[0], limit[1]]);
    let year = u16::from_le_bytes([limit[2], limit[3]]);
    let year_and = read_i16(e + 2) as u16;
    year > year_and || (year == year_and && day > read_i16(e))
}

/// Spreads the illness outbreaks up to the first one after today: one older than five days is removed; the others make
/// random squad players of the club ill.
///
/// `Injury::spread_illness_outbreaks()`; SEH frame without destructor. It walks the outbreaks (sorted by date) up to
/// the first one after today. One older than five days (0x54c8e0) is marked to leave; the others spread: type 3 only on
/// the day itself, with 1 to 4 ill (random of 4, +1); the others 0 to 2 (random of 3). Outside national teams
/// (`exe::database_is_national_team`), up to 20 tries: one of the 50 squad slots (random of 50, +0xd7), a player (id below [0xae2364]) without
/// that illness (+0x1d of the 31-byte record at [this]) becomes ill (`add_injury`: the type in the low byte of the
/// outbreak's offset dword, as in the original; the outbreak's fixture). At the end, those leaving go from the front of
/// the vector (the others move down) and the count drops.
/// Original: `Injury::spread_illness_outbreaks` (0x62e9e0, injury.cpp, thiscall).
///
/// [pt] { 0x62e9e0 } (thiscall, ret 0, injury.cpp, quadro SEH sem destrutor): `Injury::spread_illness_outbreaks()`: percorre
/// os surtos (ordenados pela data) até o primeiro depois de hoje. O de mais de cinco dias (0x54c8e0) conta para sair;
/// os outros espalham: tipo 3 só no próprio dia, com 1 a 4 doentes (sorteio de 4, +1); os outros 0 a 2 (sorteio de 3).
/// Fora das seleções (0x53b900), até 20 tentativas: uma das 50 vagas do elenco (sorteio de 50, +0xd7), jogador (id
/// abaixo de [0xae2364]) sem essa doença (+0x1d do registro de 31 bytes em [this]) fica doente (0x62d250: o tipo no
/// byte baixo da dword do deslocamento do surto, como a original; a partida do surto). No fim, os que saem vão
/// embora do começo do vetor (os outros descem) e a conta cai.
pub(crate) unsafe extern "thiscall" fn spread_illness_outbreaks(this: usize) {
    let five = five_days_ago();
    let mut removidos: i32 = 0;
    let mut k: i32 = 0;
    let mut off: usize = 0;
    while k < read_i32(this + injury_fields::OUTBREAK_COUNT) {
        let e = read_usize(this + injury_fields::OUTBREAKS).wrapping_add(off);
        let hoje4 = read_usize(G_CURRENT_DATE) as u32;
        let (ano_e, ano_h) = (read_i16(e + 2) as u16, (hoje4 >> 16) as u16);
        if ano_h < ano_e || (ano_h == ano_e && (hoje4 as u16 as i16) < read_i16(e)) {
            break; // the first outbreak after today
        }
        if is_after_outbreak(&five, e) {
            crate::comparison::branch("cpp_8 old outbreak");
            removidos += 1;
        } else {
            let kind = read_u8(e + 8);
            let how_many = if kind == 3 {
                if read_usize(G_CURRENT_DATE) as u32 == read_usize(e) as u32 { Some((friendly_random(4) as u8).wrapping_add(1)) } else { None }
            } else {
                Some(friendly_random(3) as u8) // 0 a 2 doentes
            };
            if let Some(mut how_many) = how_many {
                crate::comparison::branch("cpp_8 spread");
                let club = Club((injury_club_by_index(read_i32(read_usize(this + injury_fields::OUTBREAKS).wrapping_add(off) + 9))).addr());
                if !injury_is_national_team(club) {
                    let mut t = 0;
                    while t < 0x14 && (how_many as i8) > 0 {
                        let slot = friendly_random(0x32) as i8 as isize;
                        let p = read_usize((club.addr() + club::SQUAD).wrapping_add((slot * 4) as usize));
                        if p != 0 && read_i32(p) < read_i32(G_STAFF_COUNT) {
                            let e = read_usize(this + injury_fields::OUTBREAKS).wrapping_add(off);
                            let doenca = read_u8(e + 8);
                            let record = PhysicalRecord(read_usize(this).wrapping_add((read_i32(p) as usize).wrapping_mul(31)));
                            if record.last_injury_type() != doenca {
                                exe::injury_add_injury(this, (off as u32 & 0xffff_ff00) | doenca as u32, 0, p, 0, 0, read_i32(e + illness_outbreak::CLUB_ID), 0, 0);
                                how_many = how_many.wrapping_sub(1);
                            }
                        }
                        t += 1;
                    }
                }
            }
        }
        k += 1;
        off += OUTBREAK_SIZE;
    }
    let n = read_i32(this + injury_fields::OUTBREAK_COUNT);
    if removidos != 0 && removidos != n {
        let base = read_usize(this + injury_fields::OUTBREAKS);
        core::ptr::copy((base + removidos as usize * OUTBREAK_SIZE) as *const u8, base as *mut u8, (n - removidos) as usize * OUTBREAK_SIZE);
    }
    write_i32(this + injury_fields::OUTBREAK_COUNT, n - removidos);
}

/// Tells whether the person is in the close season: 1 without a person; 1 when the club (without the +0x1db mark) is
/// out of its nation's season with less than 40 days to go and there is no future national call-up; otherwise 0.
///
/// `injury_player_in_close_season(person)` (stdcall, ret 4; SEH frame without destructor): without a person, 1. With a
/// national team (+0x24), its date (`exe::nationalteams_get_squad_slot_record` on [0xb74318], the national team's nation +0x53, the class from `exe::national_teams_get_squad_slot`
/// by bit 3 of +0x5e) after today counts as called up. Without a club (+0x39), called up, or a club with +0x1db ≠ −1:
/// 0. Otherwise, out of the season of the club's nation (`key_nation_club_in_season` with the two dates) and with less than 40 days to go
/// (`exe::date_days_since` from today to the second date, 16 bits unsigned): 1; otherwise 0.
/// Original: `injury_player_in_close_season` (0x62ce30, injury.cpp, stdcall, ret 4).
///
/// [pt] { 0x62ce30 } (stdcall, ret 4, injury.cpp, quadro SEH sem destrutor): `injury_player_in_close_season(pessoa)`: sem
/// pessoa, 1. Com seleção (+0x24), a data dela (0x76e040 sobre [0xb74318], a nação +0x53 da seleção, a classe da
/// 0x76d780 pelo bit 3 do +0x5e) depois de hoje conta como convocado. Sem clube (+0x39), convocado ou clube com +0x1db
/// ≠ −1: 0. Senão, fora da temporada da nação do clube (0x669bd0 com as duas datas) e faltando menos de 40 dias
/// (0x54cdb0 de hoje até a segunda data, 16 bits sem sinal): 1; senão 0.
pub(crate) unsafe extern "stdcall" fn injury_player_in_close_season(person: usize) -> u32 {
    let person = Staff(person);
    let mut start = [0u32; 2];
    let mut end = [0u32; 2];
    exe::date_date_54a110((start.as_mut_ptr()) as usize);
    exe::date_date_54a110((end.as_mut_ptr()) as usize);
    if person.is_null() {
        return 1;
    }
    let mut called_up = false;
    let national_team = person.national_job();
    if !national_team.is_null() {
        let k = exe::national_teams_get_squad_slot(national_team, (person.playing_squad() & 8) as u32);
        // simplification: the original pushes the argument’s dword with only the low byte replaced by al; 0x76e040 reads only the byte
        let q = exe::nationalteams_get_squad_slot_record(read_usize(G_NATIONAL_TEAMS), national_team.nation().addr(), k as u32);
        let lo = read_usize(q) as u32;
        let (ano, ano_h) = ((lo >> 16) as u16, read_i16(G_CURRENT_DATE_YEAR) as u16);
        called_up = ano > ano_h || (ano == ano_h && (lo as u16 as i16) > read_i16(G_CURRENT_DATE));
    }
    let club = person.club_job();
    if club.is_null() || called_up || (club.continental_comp_id() as i32) != -1 {
        return 0;
    }
    if exe::key_nation_club_in_season(club, (start.as_mut_ptr()) as *mut u32 as usize, (end.as_mut_ptr()) as *mut u32 as usize) != 0 {
        return 0;
    }
    if (exe::date_days_since(G_CURRENT_DATE, end[0], end[1]) as u16) < 0x28 {
        return 1;
    }
    0
}

/// Club count without the national teams at the end of the array: [0xae235c] - 2 × [0xae2350].
///
/// [pt] Os clubes sem as seleções do fim do vetor: [0xae235c] − 2 × [0xae2350].
pub(crate) unsafe fn injury_club_count_without_nations() -> i32 {
    read_i32(G_CLUB_COUNT).wrapping_sub(read_i32(G_NATION_COUNT).wrapping_mul(2))
}

/// Preseason days counting for initial fitness: 0 within the nation's season, else days since its start capped/folded
/// (40, 80 - days).
///
/// [pt] Os dias de pré-temporada que contam na forma inicial: dentro da temporada da nação do clube (0x669bd0), 0; fora,
/// os dias do começo dela até hoje (0x54cdb0 sobre a primeira data): acima de 80, 40; de 41 a 80, 80 − dias (no byte,
/// com sinal); senão os dias.
pub(crate) unsafe fn preseason_days(club: Club, start: &mut [u32; 2], end: &mut [u32; 2]) -> i32 {
    if exe::key_nation_club_in_season(club, (start.as_mut_ptr()) as *mut u32 as usize, (end.as_mut_ptr()) as *mut u32 as usize) != 0 {
        return 0;
    }
    let d = exe::date_days_since((start.as_mut_ptr()) as usize, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32);
    if d > 0x50 {
        0x28
    } else if d > 0x28 {
        (0x50u8.wrapping_sub(d as u8)) as i8 as i32
    } else {
        d
    }
}

/// Sets each squad player's initial fitness from the club's pre-season days; non-players leave the squad slot.
///
/// `Injury::init_squad_fitness()`; SEH frame without destructor. For each club (except national teams), the pre-season
/// days (`preseason_days`; 0 in season, up to 40); each of the 50 slots (+0xd7) with a person whose id is below
/// [0xae2364]: a player (+0x61): v = days − (player's +0x34, natural fitness)/2 − (person's +0x5b)/4 (signed,
/// truncated), between 1 and 40, and the fitness (+8 of the 31-byte record at [this]) = 10000 − 75 × v; anyone who is
/// not a player leaves the slot (0), and if their club (+0x39) is this one: job (+0x3d) 0xb becomes clubless and 8, 0xf
/// becomes 8, 0xd becomes 6.
/// Original: `Injury::init_squad_fitness` (0x62a920, injury.cpp, thiscall).
///
/// [pt] { 0x62a920 } (thiscall, ret 0, injury.cpp, quadro SEH sem destrutor): `Injury::init_squad_fitness()`: para cada
/// clube (sem as seleções), os dias de preparo (`preseason_days`); cada uma das 50 vagas (+0xd7) com pessoa de
/// id abaixo de [0xae2364]: jogador (+0x61): v = dias − (+0x34 do jogador)/2 − (+0x5b da pessoa)/4 (com sinal,
/// truncado), entre 1 e 40, e a forma (+8 do registro de 31 bytes em [this]) = 10000 − 75 × v; quem não é jogador sai
/// da vaga (0), e se o clube dele (+0x39) é este: cargo (+0x3d) 0xb fica sem clube e 8, 0xf vira 8, 0xd vira 6.
pub(crate) unsafe extern "thiscall" fn init_squad_fitness(this: usize) {
    let mut start = [0u32; 2];
    let mut end = [0u32; 2];
    exe::date_date_54a110((start.as_mut_ptr()) as usize);
    exe::date_date_54a110((end.as_mut_ptr()) as usize);
    let base = read_usize(G_CLUB_TABLE);
    let mut i = 0i32;
    while i < injury_club_count_without_nations() {
        let club = Club(base.wrapping_add((i as usize).wrapping_mul(club::SIZE)));
        let days = preseason_days(club, &mut start, &mut end);
        for s in 0..50usize {
            let slot = club.addr() + club::SQUAD + 4 * s;
            let p = Staff(read_usize(slot));
            if p.is_null() || read_i32(p.addr()) >= read_i32(G_STAFF_COUNT) {
                continue;
            }
            let player = p.player();
            if !player.is_null() {
                crate::comparison::branch("cpp_9 initial form");
                let v = days - ((player.natural_fitness() as u8) as i8 as i32) / 2 - ((p.professionalism() as u8) as i8 as i32) / 4;
                let v = v.clamp(1, 0x28);
                let r = read_usize(this).wrapping_add((read_i32(p.addr()) as usize).wrapping_mul(31));
                write_u16(r + 8, (10000 - v * 75) as u16); // a forma inicial
            } else {
                if p.club_job() == club {
                    match p.job_for_club() {
                        0xb => {
                            p.set_club_job(Club(0));
                            p.set_job_for_club(8);
                        }
                        0xf => *((p.addr() + staff::JOB_FOR_CLUB) as *mut u8) = 8,
                        0xd => *((p.addr() + staff::JOB_FOR_CLUB) as *mut u8) = 6,
                        _ => {}
                    }
                }
                write_usize(slot, 0);
            }
        }
        i += 1;
    }
}

/// Allocates every person's 31-byte injury record fully fit; players may get a recurring injury (1 in 10) or start
/// injured (1 in 100); the totals go to 0xde1f64.
///
/// `Injury::init_records()`: [this] = 31 bytes per person of [0xae2364] (`exe::crt_malloc`; out of memory, the "Error" box for
/// line 0x7d0 and nothing). Each record: today's date (`exe::date_operator_3d`), form and condition 10000 (+8, +0xa), wear 0 (+0xc),
/// no injury (+0xe −1), no recurring injury (+0x12 0xff), history −1 (+0x14, +0x18), +0x1d 0xff, +0x1e 50. A player
/// (+0x61): the type drawn from 82 with weight (+8 of the 13-byte table at 0x9e7ad8) and 1 in 10 becomes recurring; 1
/// in 100 starts injured: category drawn from 12, type through `choose_injury_type` (severity −1) and the new injury `add_injury`. At
/// the end the two totals in 0xde1f64 ("%d players with ...").
/// Original: `Injury::init_records` (0x62cf70, injury.cpp, thiscall).
///
/// [pt] { 0x62cf70 } (thiscall, ret 0, injury.cpp): `Injury::init_records()`: [this] = 31 bytes por pessoa de [0xae2364]
/// (0x944e46; sem memória, a caixa "Error" da linha 0x7d0 e nada). Cada registro: a data de hoje (0x417c10), forma e
/// condição 10000 (+8, +0xa), desgaste 0 (+0xc), sem lesão (+0xe −1), sem recorrente (+0x12 0xff), histórico −1
/// (+0x14, +0x18), +0x1d 0xff, +0x1e 50. Jogador (+0x61): o tipo sorteado de 82 com peso (+8 da tabela de 13 bytes em
/// 0x9e7ad8) e 1 em 10 fica recorrente; 1 em 100 começa lesionado: categoria sorteada de 12, tipo pela 0x62d140
/// (gravidade −1) e a lesão nova 0x62d250. No fim os dois totais em 0xde1f64 ("%d players with ...").
pub(crate) unsafe extern "thiscall" fn init_records(this: usize) {
    let n = read_i32(G_STAFF_COUNT);
    let v = exe::crt_malloc((n as u32).wrapping_mul(31));
    write_usize(this, v);
    if v == 0 {
        show_error_at(0x9e7f28, 0x7d0);
        return;
    }
    let (mut recorrentes, mut iniciais) = (0i32, 0i32);
    let mut r = v;
    let mut person = read_usize(G_STAFF_TABLE);
    let mut i = 0i32;
    while i < read_i32(G_STAFF_COUNT) {
        let mut draft = [0u8; 8];
        exe::date_operator_3d(r, (draft.as_mut_ptr()) as usize, G_CURRENT_DATE);
        write_u16(r + physical_record::FATIGUE, 0);
        write_u16(r + 8, 10000);
        write_u16(r + physical_record::CONDITION, 10000);
        write_u8(r + physical_record::RECURRING_TYPE, 0xff);
        write_i32(r + physical_record::CURRENT_INJURY, -1);
        write_i32(r + physical_record::HISTORY_FIRST, -1);
        write_i32(r + physical_record::HISTORY_LAST, -1);
        write_u8(r + physical_record::LAST_INJURY_TYPE, 0xff);
        write_u8(r + physical_record::PRONENESS, 0x32);
        if read_usize(person + staff::PLAYER) != 0 {
            let t = friendly_random(0x52) as u8;
            if read_usize((0x9e7ae0usize).wrapping_add((t as i8 as isize * 13) as usize)) != 0 && friendly_random(10) == 0 {
                recorrentes += 1;
                write_u8(r + physical_record::RECURRING_TYPE, t);
            }
            if friendly_random(100) == 0 { // 1 in 100 starts injured
                crate::comparison::branch("cpp_9 initial injury");
                // simplification: the original pushes stack dwords with only the low byte written (the rest is garbage); 0x62d140
                // and 0x62d250 only read the byte: here they go clean
                let category = friendly_random(0xc) as u8;
                let kind = (exe::injury_choose_injury_type(this, (category as u32) as u32 as i8, (0xffff_ffff) as u32 as i8, person) as u32) as u8;
                exe::injury_add_injury(this, kind as u32, 1, person, 0, 1, -1, 0, 0);
                iniciais += 1;
            }
        }
        i += 1;
        person = person.wrapping_add(0x6e);
        r = r.wrapping_add(31);
    }
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    format(0xde1f64, 0x9e7f98, recorrentes);
    format(0xde1f64, 0x9e7f74, iniciais);
}

/// Nation whose youth list the club joins (`build_nation_club_lists` rules), or 0 for national teams, no nation or excluded clubs.
///
/// [pt] A nação em cuja lista dos jovens o clube entra (0x7acfc0): nenhuma para seleção (0x53b900), sem nação (+0x53),
/// nação sem +0x71 ou os dois clubes de 0x9d00a4/0x9d00a8. Nação com +0x11c: só com +0x80 até 2 e divisão (+0x57);
/// sem: com divisão.
pub(crate) unsafe fn injury_youth_list_nation(club: Club) -> usize {
    if injury_is_national_team(club) {
        return 0;
    }
    let nation = club.nation();
    if nation.is_null() || nation.continent() == 0 {
        return 0;
    }
    if nation.league_selected() != 0 {
        crate::comparison::branch("cpp_9 key nation");
        if !(club.reputation() <= 2 && club.division().addr() != 0) { // nation with +0x11c
            return 0;
        }
    } else if club.division().addr() == 0 {
        return 0;
    }
    let id = read_usize(club.addr());
    if id == read_usize(G_KEY_CLUB_MLS_ALL_STARS_EAST) || id == read_usize(G_KEY_CLUB_MLS_ALL_STARS_WEST) {
        return 0;
    }
    nation.addr()
}

/// Address of person id's 31-byte physical record (+0x12 recurring type, +0x13 level, +0x14/+0x18 injury history
/// offsets).
///
/// [pt] O registro de 31 bytes do físico da pessoa `id` em [this] (+0x12 o tipo da lesão recorrente, 0xff sem; +0x13 o
/// nível; +0x14/+0x18 o começo e o último deslocamento do histórico de lesões).
pub(crate) unsafe fn injury_history_record(this: usize, id: usize) -> usize {
    read_usize(this).wrapping_add(id.wrapping_mul(31))
}

/// O que a `roll_recurring_injury` decide depois de sortear.
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum RecurringInjuryOutcome {
    Nada,
    Marcada,
    Limpar(usize),
    Tratar(usize),
}

/// The plan of `0x62e500`: the random draws and the record marks for a recurring injury; the rest is left to the caller
/// (RecurringInjuryOutcome).
///
/// 1 in 20 (random draw 0x14) and injury `tipo` able to recur ([`0x9e7ae0` + 13 × `tipo`]): the record without a
/// recurring injury takes it (`+0x12` = `tipo`, `+0x13` = 0).
///
/// Of the records with a recurring injury (`+0x12` ≠ 0xff), the one with the highest score (random draw 0x65 − 5 ×
/// level − the player's person `+0x2d`; 100 without a player, signed bytes), stopping at the first with 95 or more.
///
/// Found: a club player (`+0x39`) with reputation (`+0x80`) above 4250, a club without a human (`0x600a50`) and 1 in 3
/// (random draw 3): treat; otherwise clear its mark.
///
/// [pt] O plano da 0x62e500 (os sorteios e as marcas do registro; o resto fica para quem chama): 1 em 20 (sorteio 0x14) e
/// a lesão `tipo` podendo voltar ([0x9e7ae0 + 13 × tipo]), o registro sem recorrente fica com ela (+0x12 = tipo,
/// +0x13 = 0). Dos registros com recorrente (+0x12 ≠ 0xff), o de maior nota (sorteio 0x65 − 5 × nível − pessoa +0x2d
/// do jogador; 100 sem jogador, bytes com sinal), parando no primeiro com 95 ou mais. Achado: jogador de clube (+0x39)
/// de reputação (+0x80) acima de 4250, clube sem humano (0x600a50) e 1 em 3 (sorteio 3): tratar; senão limpar a marca
/// dele.
pub(crate) unsafe fn plan_recurring_injury(this: usize, record: usize, injury: *const u8) -> RecurringInjuryOutcome {
    let record = PhysicalRecord(record);
    if record.recurring_type() != 0xff || read_usize((0x9e7ae0isize + *injury as i8 as isize * 13) as usize) == 0 {
        return RecurringInjuryOutcome::Nada;
    }
    let s = friendly_random(0x14);
    if s != 0 { // 1 in 20
        return RecurringInjuryOutcome::Nada;
    }
    crate::comparison::branch("cpm_5 recurring injury");
    record.set_recurrence_grade(s as u8);
    record.set_recurring_type(*injury);
    if read_i32(G_STAFF_COUNT) <= 0 {
        return RecurringInjuryOutcome::Marcada;
    }
    let (mut melhor, mut escolhido, mut pessoa) = (0i8, 0usize, 0usize);
    let mut r = read_usize(this);
    let mut i = 0i32;
    while i < read_i32(G_STAFF_COUNT) {
        if read_u8(r + physical_record::RECURRING_TYPE) != 0xff {
            let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add(i as usize * staff::SIZE));
            let v = friendly_random(0x65) as u8;
            let player = p.player();
            let rating = if player.is_null() { 0x64 } else { v.wrapping_add(read_u8(r + physical_record::RECURRENCE_GRADE).wrapping_mul(0xfb)).wrapping_sub(player.injury_proneness() as u8) };
            if rating as i8 > melhor {
                (melhor, escolhido, pessoa) = (rating as i8, r, p.addr());
            }
            if melhor >= 0x5f { // strong enough: stop
                crate::comparison::branch("cpm_5 strong recurring");
                break;
            }
        }
        i += 1;
        r += 0x1f;
    }
    if escolhido == 0 {
        return RecurringInjuryOutcome::Marcada;
    }
    let club = Club(read_usize(pessoa + staff::CLUB_JOB));
    if read_usize(pessoa + staff::PLAYER) != 0 && !club.is_null() && club.reputation() > 0x109a {
        if exe::human_manager_is_human_club(club, 0, 1, 0, 0) == 0 && friendly_random(3) == 0 && pessoa != 0 {
            return RecurringInjuryOutcome::Tratar(pessoa);
        }
    }
    RecurringInjuryOutcome::Limpar(escolhido)
}

/// with 1 in 20, a recurring-capable injury becomes recurring in the record (+0x12 type, +0x13 = 0); among the
/// recurring records, the best score (roll 0x65 − 5 × level − proneness, stops at 95): a player at a big club
/// (reputation > 4250) without a human and 1 in 3 gets a new injury 0x4d/0x4e/0x4f (`add_injury`); otherwise his mark is
/// cleared; turned off (DESLIGADAS) from 2026-10-29 to 2026-10-31, when the test criterion changed and the list was emptied (the detour, even passing through, changes the stack residue a later
/// original reads: the career left be54a138); the copy stays in the code and does not run
/// Original: `Injury::roll_recurring_injury` (0x62e500, injury.cpp, thiscall, ret 8).
///
/// [pt] { 0x62e500 } (thiscall, ret 8, injury.cpp): `Injury::roll_recurring_injury(registro, lesao)`: o plano
/// (`plan_recurring_injury`); tratar: a lesão nova da pessoa (0x62d250 com 0x4e, 0x4f ou 0x4d pelo tipo recorrente do registro
/// dela, 0xff fora das pessoas ou sem jogador; o tipo vai no byte baixo do argumento `registro`, como a original);
/// limpar: o registro escolhido sem recorrente (+0x12 = 0xff, +0x13 = 0).
pub(crate) unsafe extern "thiscall" fn roll_recurring_injury(this: usize, record: usize, injury: *const u8) {
    match plan_recurring_injury(this, record, injury) {
        RecurringInjuryOutcome::Tratar(p) => {
            let id = read_usize(p) as i32;
            let kind = if id < read_i32(G_STAFF_COUNT) && read_usize(p + staff::PLAYER) != 0 {
                read_u8(injury_history_record(this, id as usize) + physical_record::RECURRING_TYPE)
            } else {
                0xff
            };
            let new: u8 = match kind {
                0x0b | 0x11 | 0x17 | 0x1e | 0x23 | 0x29 => 0x4e,
                0x0c | 0x0d | 0x12 | 0x14 | 0x19..=0x1c | 0x1f | 0x24 | 0x25 | 0x2c | 0x2d | 0x3b => 0x4f,
                _ => 0x4d,
            };
            exe::injury_add_injury(this, (record as u32 & 0xffff_ff00) | new as u32, 0, p, 0, 1, -1, 0, 0);
        }
        RecurringInjuryOutcome::Limpar(r) => {
            write_u8(r + physical_record::RECURRING_TYPE, 0xff);
            write_u8(r + physical_record::RECURRENCE_GRADE, 0);
        }
        _ => {}
    }
}

/// Clears the last entry of the person's injury history (+0x18 of the 31-byte record = −1, through 0x58f110).
///
/// `Injury::reset_player_history(person)`: the person's injury history (31-byte record of the id at [this]) loses its
/// last entry (0x58f110 on [this+0xc]: +0x18 = −1). Returns what 0x58f110 returns. Null person: as in the original
/// (fatal box).
/// Original: `Injury::reset_player_history` (0x631f90, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x631f90 } (thiscall, ret 4, injury.cpp): `Injury::reset_player_history(pessoa)`: o histórico de lesões da pessoa
/// (registro de 31 bytes do id em [this]) fica sem o último (0x58f110 sobre [this+0xc]: +0x18 = −1). Devolve o que a
/// 0x58f110 devolve. Pessoa nula: a original (caixa fatal).
pub(crate) unsafe extern "thiscall" fn reset_player_history(this: usize, person: usize) -> usize {
    if person == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_RESET_PLAYER_HISTORY)(this, person); // fatal box: original only
    }
    let r = PhysicalRecord(injury_history_record(this, read_usize(person)));
    exe::file_llist_reset_last_offset(read_usize(this + injury_fields::INJURY_HISTORY), r.addr() + physical_record::HISTORY_FIRST, r.addr() + physical_record::HISTORY_LAST)
}

/// Resets a person's injury state: removes their items from the match knock list, ends the injury, rewinds the history
/// and restarts the 31-byte record; bit 0 of +0xc of the 79-byte record is cleared.
///
/// `Injury::reset_player(id)`: removes from list [this+0x28] (from the start: cursor +0xa back to +2) each item with
/// the id (`manager_manager_remove`); with an injury (+0xe ≠ −1), ends it (`end_player_injury`, no news); restarts the history (`reset_player_history`); the
/// 31-byte record restarts: today's date (`exe::date_operator_3d` of [0xae2c90]), condition and form 10000 (+8, +0xa), wear 0 (+0xc),
/// no injury (+0xe −1), no recurring injury (+0x12 0xff), history −1 (+0x14, +0x18), +0x1d 0xff. Bit 0 of +0xc of the
/// 79-byte record ([0xae2d0c]) is cleared. Returns the address of that byte.
/// Original: `Injury::reset_player` (0x62bfa0, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x62bfa0 } (thiscall, ret 4, injury.cpp): `Injury::reset_player(id)`: tira da lista [this+0x28] (do começo: o
/// cursor +0xa volta a +2) cada item do id (0x68d1f0); com lesão (+0xe ≠ −1), termina (0x62e700, sem notícia);
/// recomeça o histórico (0x631f90); o registro de 31 bytes recomeça: data de hoje (0x417c10 de [0xae2c90]), condição
/// e forma 10000 (+8, +0xa), desgaste 0 (+0xc), sem lesão (+0xe −1), sem recorrente (+0x12 0xff), histórico −1 (+0x14,
/// +0x18), +0x1d 0xff. O bit 0 do +0xc do registro de 79 bytes ([0xae2d0c]) cai. Devolve o endereço desse byte.
pub(crate) unsafe extern "thiscall" fn injury_reset_player(this: usize, id: u32) -> usize {
    crate::comparison::branch("cpm_5 reset player");
    if read_usize(this + injury_fields::MATCH_HIT_LIST) != 0 {
        let l = Llist(read_usize(this + injury_fields::MATCH_HIT_LIST));
        l.set_cursor(l.head());
        let mut e = exe::llist_get_next(read_usize(this + injury_fields::MATCH_HIT_LIST));
        while e != 0 {
            if read_usize(e) as u32 == id {
                exe::llist_remove(read_usize(this + injury_fields::MATCH_HIT_LIST), e);
            }
            e = exe::llist_get_next(read_usize(this + injury_fields::MATCH_HIT_LIST));
        }
    }
    let r = PhysicalRecord(injury_history_record(this, id as usize));
    let person = read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE));
    if (r.current_injury() as usize) as i32 != -1 {
        exe::injury_end_player_injury(this, person, 0);
    }
    exe::injury_reset_player_history(this, read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    let mut draft = [0u8; 8];
    exe::date_operator_3d(r.addr(), (draft.as_mut_ptr()) as usize, G_CURRENT_DATE); // today’s date
    write_u16(r.addr() + 8, 10000);
    r.set_condition(10000u16 as i16);
    r.set_fatigue(0); // no fatigue
    r.set_current_injury(-1);
    r.set_recurring_type(0xff);
    r.set_history_first(-1);
    r.set_history_last(-1);
    r.set_last_injury_type(0xff);
    let g = read_usize(G_STAFF_STATE_TABLE).wrapping_add((id as usize).wrapping_mul(79)) + staff_state::EXTRA_FLAGS;
    *(g as *mut u8) &= 0xfe;
    g
}

/// Applies a recovery day to person `p`'s 31-byte record (below −1000 in x87 with −0.0035 and 2.5; otherwise the rate
/// or a random draw).
///
/// In the 31-byte record (from [this]): +0xa receives `mark`; the word +0xc, below −1000, rises by trunc((v × −0.0035 −
/// 2.5) × `rate`); otherwise, below 1200 − (player sheet +0x44 + player sheet +0x3c + person +0x5b, 16 bits), it rises
/// by `rate`; otherwise, below 1250, it rises by random(`rate`) + 1. Null person or id outside [0xae2364]: nothing.
/// Original: `recover_injury` (0x62bd10, injury.cpp, thiscall).
///
/// [pt] { 0x62bd10 } (thiscall, ret 0xc, injury.cpp): o dia de recuperação da pessoa `p` no registro de
/// 31 bytes (a partir de [this]): +0xa recebe `marca`; a palavra +0xc, abaixo de −1000, sobe
/// trunc((v × −0.0035 − 2.5) × `ritmo`); senão, abaixo de 1200 − (ficha +0x44 + ficha +0x3c +
/// pessoa +0x5b, 16 bits), sobe `ritmo`; senão, abaixo de 1250, sobe exe::utils_random(`ritmo`) + 1.
/// Pessoa nula ou id fora de [0xae2364]: nada.
pub(crate) unsafe extern "thiscall" fn recover_injury(t: usize, p: *const i32, mark: i16, ritmo: i16) {
    if p.is_null() || *p >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let r = PhysicalRecord(read_usize(t).wrapping_add((*p as usize).wrapping_mul(31)));
    r.set_condition(mark);
    let e = (r.addr() + physical_record::FATIGUE) as *mut i16;
    let v = core::ptr::read_unaligned(e);
    if v < -1000 {
        let x = (v as f64 * read_f64(rdata::F64_NEG_0_0035) - read_f64(rdata::F64_2_5)) * ritmo as f64;
        core::ptr::write_unaligned(e, (x as i32).wrapping_add(v as i32) as i16);
        return;
    }
    let sheet = PlayerSheet(read_usize(p as usize + staff::PLAYER));
    let b = |a: usize| read_i8(a) as i16;
    let sum = b(sheet.addr() + player_sheet::WORK_RATE).wrapping_add(b(sheet.addr() + player_sheet::STAMINA)).wrapping_add(b(p as usize + staff::PROFESSIONALISM));
    if (v as i32) < 0x4b0 - sum as i32 {
        core::ptr::write_unaligned(e, v.wrapping_add(ritmo));
    } else if v < 0x4e2 {
        let s = exe::utils_random(ritmo as i32).wrapping_add(1) as i16;
        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_add(s));
    }
}

compare_value!(compare_table_penalty, table_penalty, IDX_TABLE_PENALTY, (linha: i32, coluna: i8, tipo: i8) -> i32);

/// Returns the player's condition: fitness × match condition / 10,000 (minimum 100, maximum 9000 while recovering), or
/// as a percentage (maximum 100).
///
/// `Injury::get_condition(player, as_percent)`: the player's condition: form (+0xa) × match condition (+8) / 10,000
/// from the id's 31-byte record [this], at least 100 and at most 9000 while he is recovering (0x62c400 = 1); as a
/// percentage (× 0x96bac8 + 0x968968, _ftol), at most 100. A person outside the players ([0xae2364]): 100 or 10,000.
/// Null player: 0.
/// Original: `Injury::get_condition` (0x62c590, injury.cpp, thiscall, ret 8).
///
/// [pt] { 0x62c590 } (thiscall, ret 8, injury.cpp): `Injury::get_condition(jogador, em_porcento)`: a condição do jogador:
/// forma (+0xa) × condição de jogo (+8) / 10.000 do registro de 31 bytes [this] do id, no mínimo 100 e no máximo 9000
/// enquanto ele se recupera (0x62c400 = 1); em porcentagem (× 0x96bac8 + 0x968968, _ftol), no máximo 100. Pessoa fora
/// dos jogadores ([0xae2364]): 100 ou 10.000. Jogador nulo: 0.
pub(crate) unsafe extern "thiscall" fn get_condition(this: usize, player: usize, in_percent: u32) -> u16 {
    if player == 0 {
        return 0;
    }
    let id = read_usize(player) as i32;
    if id >= read_usize(G_STAFF_COUNT) as i32 {
        return if in_percent != 0 { 100 } else { 10_000 };
    }
    let recuperando = exe::injury_get_player_injury_status(this, player) == 1;
    let r = PhysicalRecord(read_usize(this).wrapping_add((id as usize).wrapping_mul(31)));
    let v = r.condition() as i32 * read_i16(r.addr() + 8) as i32 / 10_000;
    let mut c = if recuperando && v as i16 >= 9000 {
        9000 // the ceiling for those recovering
    } else if v as i16 > 100 {
        v
    } else {
        100
    };
    if in_percent != 0 {
        c = ftol(c as i16 as f64 * read_f32(rdata::F32_0_009999999776) as f64 + read_f64(rdata::F64_0_5));
        if c as i16 > 100 {
            c = 100;
        }
    }
    c as u16
}

/// Updates the day's fitness (`update_player_fitness`) and returns the match condition (out of 10000, or as a percentage).
///
/// `Injury::get_match_condition(player, as_percent)`: without a player, 0; an id outside the persons ([0xae2364]):
/// 10000 (100 as a percentage); otherwise it updates the day's fitness (`update_player_fitness`, once a day) and returns the condition
/// (31-byte record at [this], +0xa), as a percentage × 0.01f truncated. Only ax is the return value.
/// Original: `Injury::get_match_condition` (0x62c510, injury.cpp, thiscall, ret 8).
///
/// [pt] { 0x62c510 } (thiscall, ret 8, injury.cpp): `Injury::get_match_condition(jogador, em_porcento)`: sem jogador, 0; id
/// fora das pessoas ([0xae2364]): 10000 (100 em porcento); senão atualiza o físico do dia (0x62c680, uma vez por dia)
/// e devolve a condição (registro de 31 bytes em [this], +0xa), em porcento × 0,01f truncado. Só o ax é o retorno.
pub(crate) unsafe extern "thiscall" fn get_match_condition(this: usize, player: usize, in_percent: u32) -> u16 {
    if player == 0 {
        return 0;
    }
    let id = read_i32(player);
    if id >= read_i32(G_STAFF_COUNT) {
        return if in_percent != 0 { 100 } else { 10000 };
    }
    exe::injury_update_player_fitness(this, player);
    let c = read_i16(read_usize(this).wrapping_add((read_i32(player) as usize).wrapping_mul(31)).wrapping_add(0xa));
    if in_percent != 0 {
        return ftol(c as f64 * read_f32(rdata::F32_0_009999999776) as f64) as u16; // in percent
    }
    c as u16
}

/// stores the player's training load in the fitness record
/// Original: `Injury::set_training_load` (0x62c660, injury.cpp, thiscall).
///
/// [pt] { 0x62c660 } (thiscall, ret 8, injury.cpp): grava a carga de treino `valor` do jogador `*id`.
pub(crate) unsafe extern "thiscall" fn set_training_load(injuries: usize, id: *const i32, value: u8) {
    *training_load_byte(injuries, id) = value;
}

/// Updates a person's fitness once a day: the days since the last update become free, recovery, stopped (injury) or
/// holiday days, which raise or lower fitness (+8) and match condition (+0xa); stores today's date.
///
/// `Injury::update_player_fitness(person)`; SEH frame only for the local statics. Once a day (if date +0 of the
/// person's 31-byte record at [this] is already today: nothing), called from the match condition (`get_match_condition`), the injury
/// state (`get_player_injury_status`), the injuries day (`injury_daily_update`) and the return news (`post_due_recovery_news`): every person who plays, every day
/// someone asks. T = days since the last update (`exe::date_get_difference`). Injured (+0xe in the 0x30-byte table at [this+4]): by the
/// days since the start (+0x20) against +0x2a and +0x28, the stopped days (loss), recovery days or free days, with the
/// end of the injury (`end_player_injury`) when it passes +0x28. Not injured: free days, except with the national team with a
/// future date (`exe::national_teams_get_squad_slot`/`exe::nationalteams_get_squad_slot_record`) or at a club out of season (`key_nation_club_in_season`, cached in 0xb63d2c/0xb63d30 with the dates
/// 0xb63d18/0xb63d20): holidays (byte +0xa of the 79-byte record rises when random(+0x5b / 3 + 2) is zero, up to 50)
/// and the pre-season return (15 days, or the middle of a random(2) with form × condition below 80%). The fitness (+8):
/// recovery of 1000 − 12 × (+0x1e − natural (+0x34 of the player sheet), 0..100) per free day, loss of 66 per stopped
/// day, capped by natural fitness, stretches above 7000/9000/9850 shrunk (0x96af30, 0x96bad0, 0x968968, 0x968918,
/// 0x967880), and −55 per holiday day; the match condition (+0xa) rises 500 × free days and 250 × recovery days times
/// the tiredness factor (+0xc).
///
/// Writes both and today's date.
/// Original: `Injury::update_player_fitness` (0x62c680, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x62c680 } (thiscall, ret 4, injury.cpp, quadro SEH só das estáticas locais): `Injury::update_player_fitness(
/// pessoa)`: uma vez por dia (a data +0 do registro de 31 bytes da pessoa em [this] já é hoje: nada), pela condição de
/// jogo (0x62c510), pelo estado da lesão (0x62c400), pelo dia das lesões (0x62b7b0) e pelas notícias da volta (0x62e010):
/// toda pessoa que joga, todo dia em que alguém pergunta. T = dias desde a última atualização (0x54cc30). Lesionada
/// (+0xe na tabela de 0x30 bytes em [this+4]): pelos dias desde o começo (+0x20) contra +0x2a e +0x28, os dias parados
/// (perda), em recuperação ou livres, com o fim da lesão (0x62e700) quando passa de +0x28. Sem lesão: dias livres, salvo
/// na seleção com data futura (0x76d780/0x76e040) ou no clube fora da temporada (0x669bd0, guardado em 0xb63d2c/0xb63d30
/// com as datas 0xb63d18/0xb63d20): férias (o byte +0xa do registro de 79 bytes sobe com sorteio(+0x5b / 3 + 2) zero, até
/// 50) e a volta da pré-temporada (15 dias, ou o meio de um sorteio(2) com forma × condição abaixo de 80%). A forma (+8):
/// recuperação de 1000 − 12 × (+0x1e − natural (+0x34 da ficha), 0..100) por dia livre, perda de 66 por dia parado, teto
/// pela forma natural, trechos acima de 7000/9000/9850 encolhidos (0x96af30, 0x96bad0, 0x968968, 0x968918, 0x967880) e −55
/// por dia de férias; a condição de jogo (+0xa) sobe 500 × dias livres e 250 × em recuperação pelo fator do cansaço
/// (+0xc).
/// Grava as duas e a data de hoje.
pub(crate) unsafe extern "thiscall" fn update_player_fitness(this: usize, person: usize) {
    let person = Staff(person);
    init_static_date(1, G_FITNESS_UPDATE_CACHE_DATE, 0x62ce20);
    init_static_date(2, G_FITNESS_UPDATE_SEASON_START_DATE, 0x62ce10);
    init_static_date(4, G_FITNESS_UPDATE_SEASON_END_DATE, 0x62ce00);
    if person.is_null() {
        return;
    }
    let id = read_i32(person.addr());
    if id >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let sheet = person.player();
    if sheet.is_null() {
        return;
    }
    let rec = PhysicalRecord(read_usize(this).wrapping_add((id as u32).wrapping_mul(31) as usize));
    if read_u16(rec.addr() + 2) == read_u16(G_CURRENT_DATE_YEAR) && read_i16(rec.addr()) == read_i16(G_CURRENT_DATE) {
        return; // already updated today
    }
    let t = days_since(rec.addr());
    let current = rec.current_injury();
    if current > read_i32(this + injury_fields::INJURY_ENTRY_COUNT) || current < -1 {
        rec.set_current_injury(-1);
    }
    // the free days (l38), recovering (l3c), stopped (l34) and on holiday (l30)
    let (mut l3c, mut l38, mut l34, mut l30) = (0i32, 0i32, 0i32, 0i32);
    let i = rec.current_injury();
    if i != -1 {
        let inj = InjuryEntry(read_usize(this + injury_fields::INJURY_ENTRIES).wrapping_add((i as u32).wrapping_mul(0x30) as usize));
        let e = days_since(inj.addr() + injury_entry::DATE);
        let s = inj.training_return_days() as i32;
        if e < s {
            l34 = t;
        } else {
            let f = inj.days_out() as i32;
            if e < f {
                if t < s.wrapping_add(e) {
                    l3c = e - s;
                    l34 = t - (e - s);
                } else {
                    l3c = t;
                }
            } else if t > e - s {
                l38 = t - f;
                l3c = (e - s) - l38;
                l34 = f - l3c;
                end_injury(this, person);
            } else {
                l38 = t;
                end_injury(this, person);
            }
        }
    } else {
        l38 = t;
        let national_team = person.national_job();
        let mut occupied = false;
        if !national_team.is_null() {
            let k = exe::national_teams_get_squad_slot(national_team, (person.playing_squad() & 8) as u32);
            // simplification: the original pushes the argument with al on top (the rest is the person pointer); only the byte counts
            let d = exe::nationalteams_get_squad_slot_record(read_usize(G_NATIONAL_TEAMS), national_team.nation().addr(), (person.addr() as u32 & 0xffff_ff00) | k as u32);
            let (ano, hoje) = (read_u16(d + 2), read_u16(G_CURRENT_DATE_YEAR));
            occupied = if ano > hoje {
                true
            } else if ano < hoje {
                false
            } else {
                read_i16(d) > read_i16(G_CURRENT_DATE)
            };
        }
        let club = person.club_job();
        if !club.is_null() && !occupied {
            if club != Club(read_usize(G_FITNESS_UPDATE_CACHED_CLUB)) || read_u16(G_FITNESS_UPDATE_CACHE_DATE_YEAR) != read_u16(G_CURRENT_DATE_YEAR) || read_i16(G_FITNESS_UPDATE_CACHE_DATE) != read_i16(G_CURRENT_DATE) {
                let r = exe::key_nation_club_in_season(club, G_FITNESS_UPDATE_SEASON_START_DATE, G_FITNESS_UPDATE_SEASON_END_DATE);
                write_u32(G_FITNESS_UPDATE_CACHED_SEASON_FLAG, (r == 0) as u32);
                write_usize(G_FITNESS_UPDATE_CACHED_CLUB, person.club_job().addr());
                manager_manager_copy_date(G_FITNESS_UPDATE_CACHE_DATE, G_CURRENT_DATE);
            }
            if read_i32(G_FITNESS_UPDATE_CACHED_SEASON_FLAG) != 0 {
                l38 = 0;
                let u6 = exe::date_days_since(G_CURRENT_DATE, read_usize(G_FITNESS_UPDATE_SEASON_END_DATE) as u32, read_usize(G_FITNESS_UPDATE_SEASON_END_DATE_LEAP) as u32) as u16;
                let u7 = exe::date_days_since(G_FITNESS_UPDATE_SEASON_START_DATE, read_usize(G_CURRENT_DATE) as u32, read_usize(G_CURRENT_DATE_LEAP) as u32) as u16;
                if u6 < u7 {
                    let lim = if (u6 as i32) < t { u6 as i32 } else { t };
                    l30 = lim;
                    l3c = t - lim;
                    for _ in 0..lim {
                        let g = read_usize(G_STAFF_STATE_TABLE).wrapping_add((read_i32(person.addr()) as u32).wrapping_mul(0x4f) as usize) + staff_state::INJURY_COUNTER;
                        if read_u8(g) >= 0x32 {
                            break;
                        }
                        if friendly_random(person.professionalism() as i32 / 3 + 2) == 0 {
                            write_u8(g, read_u8(g).wrapping_add(1));
                        }
                    }
                } else {
                    let mut x = u6 as i32 - (u7 as i32 + u6 as i32) / 2;
                    if x < 0 {
                        x = 0;
                    }
                    if t < x {
                        x = t;
                    }
                    l3c = x;
                    l34 = t - x;
                }
                if u7 < 0xf {
                    l38 = 0xf - u7 as i32;
                    l3c -= l38;
                    if l3c < 0 {
                        l3c = 0;
                    }
                } else {
                    let (f, mc) = (read_i16(rec.addr() + 8) as i32, rec.condition() as i32);
                    if mc.wrapping_mul(f) / 10000 < 8000 { // cpw8: form below 80% in pre-season
                        crate::comparison::branch("cpw_8 form draw");
                        let r = friendly_random(2);
                        l38 = (r + l3c) / 2;
                        l3c -= l38;
                    }
                }
            }
        }
    }
    // a forma
    let f = read_i16(rec.addr() + 8) as i32;
    let natural = sheet.natural_fitness();
    let mut v = rec.proneness().wrapping_sub(natural);
    if v < 0 {
        v = 0;
    } else if v > 100 {
        v = 100;
    }
    let s = (1000 - 12 * v as i32) as i16 as i32; // cpw8: the recovery per free day
    let half = s / 2;
    let n = natural as i32;
    let floor = ((n * 25 / 20 + 15) * 100) as i16 as i32;
    if l34 != 0 {
        crate::comparison::branch("cpw_8 loss");
    }
    let mut x = f.wrapping_sub(66i32.wrapping_mul(l34)); // cpw8: a perda por dia parado
    if x < floor {
        x = 4000;
    }
    let m = n * 16 / 20 + 0x54;
    let mut ceiling = m.wrapping_mul(100);
    if (ceiling as i16 as i32) < x {
        ceiling = x;
    }
    x = x.wrapping_add((half as i16 as i32).wrapping_mul(l3c));
    let teto16 = ceiling as i16 as i32;
    if x > teto16 {
        x = teto16;
    }
    if l38 != 0 {
        crate::comparison::branch("cpw_8 recovery");
    }
    x = x.wrapping_add(s.wrapping_mul(l38));
    if f > 0x267a {
        x = scale_above_base(x, f, 0x967880);
    } else if f > 0x2328 {
        x = scale_above_base(x, f, 0x968918);
        if x > 0x267a {
            x = scale_above_base(x, 0x267a, 0x968968);
        }
    } else if x > 0x1b58 {
        x = scale_above_base(x, 0x1b58, 0x96af30);
        if x > 0x2328 {
            x = scale_above_base(x, 0x2328, 0x96bad0);
        }
        if x > 0x267a {
            x = scale_above_base(x, 0x267a, 0x968968);
        }
    }
    if x > 10000 {
        x = 10000;
    }
    let mut form = x.wrapping_sub(55i32.wrapping_mul(l30));
    if form < floor {
        form = 5000;
    }
    let mut e2 = m;
    if (m as i16 as i32) < form {
        e2 = form;
    }
    let e16 = e2 as i16 as i32;
    if form > e16 {
        form = e16;
    }
    // the match condition, by the fatigue factor (+0xc)
    let cansaco = rec.fatigue();
    let mc = rec.condition() as i32;
    let ceiling_k = read_f64(rdata::F64_0_95);
    let k = if cansaco > 1000 {
        let kv = read_f64(rdata::F64_1_4) - cansaco as f64 * read_f64(rdata::F64_0_0004);
        if !(kv > ceiling_k) {
            kv
        } else if l3c > 0 {
            ceiling_k
        } else {
            kv
        }
    } else if l3c > 0 {
        ceiling_k
    } else {
        read_f64(rdata::F64_1)
    };
    let r1 = ftol(l38.wrapping_mul(500) as f64 * k);
    let r2 = ftol(l3c.wrapping_mul(250) as f64 * k);
    let mut cond = r2.wrapping_add(mc).wrapping_add(r1);
    if cond > 10000 {
        cond = 10000;
    }
    write_i16(rec.addr() + 8, form as i16);
    rec.set_condition(cond as i16);
    manager_manager_copy_date(rec.addr(), G_CURRENT_DATE);
}

/// Applies the day's wear to word +0xc of person `id`'s 31-byte record (age, draws of 25 and 13, goalkeeper cap).
///
/// From [this]: below −1000 it goes down 1; under 19 years old (+0x18 of the person in [0xae23bc]) and negative, it is
/// zeroed; otherwise, from −500 up and with age ≥ random(25), it goes down 13, and in the other cases it goes down
/// random(13); a goalkeeper (player sheet position +0xf = 20) stays up to 1050. Id outside [0xae2364]: nothing.
/// Original: `daily_wear` (0x62c2f0, injury.cpp, thiscall).
///
/// [pt] { 0x62c2f0 } (thiscall, ret 4, injury.cpp): o desgaste do dia na palavra +0xc do registro de 31
/// bytes da pessoa `id` (a partir de [this]): abaixo de −1000 desce 1; abaixo de 19 anos (+0x18 da
/// pessoa em [0xae23bc]) e negativo, zera; senão, de −500 para cima e com idade ≥ exe::utils_random(25), desce
/// 13, e nos outros casos desce exe::utils_random(13); goleiro (posição +0xf da ficha = 20) fica até 1050.
/// Id fora de [0xae2364]: nada.
pub(crate) unsafe extern "thiscall" fn daily_wear(t: usize, id: i32) {
    if id >= read_i32(G_STAFF_COUNT) {
        return;
    }
    let e = read_usize(t).wrapping_add((id as usize).wrapping_mul(31)).wrapping_add(0xc) as *mut i16;
    let v = core::ptr::read_unaligned(e);
    if v < -1000 {
        core::ptr::write_unaligned(e, v.wrapping_sub(1));
    }
    let p = Staff(read_usize(G_STAFF_TABLE).wrapping_add((id as usize).wrapping_mul(staff::SIZE)));
    let age = p.age() as i8;
    if age < 0x13 && core::ptr::read_unaligned(e) < 0 {
        core::ptr::write_unaligned(e, 0);
    } else if core::ptr::read_unaligned(e) >= -500 && age as i32 >= exe::utils_random(0x19) {
        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(0xd));
    } else {
        let r = exe::utils_random(0xd) as i16;
        core::ptr::write_unaligned(e, core::ptr::read_unaligned(e).wrapping_sub(r));
    }
    let sheet = p.player();
    if !sheet.is_null() && (sheet.goalkeeper() as u8) == 0x14 && core::ptr::read_unaligned(e) >= 0x41a {
        core::ptr::write_unaligned(e, 0x41a);
    }
}

/// draws the injury type for the body part and severity, keeping the current injury or preferring the recurring one
/// Original: `Injury::choose_injury_type` (0x62d140, injury.cpp, thiscall).
///
/// [pt] sorteia o tipo de lesão da parte do corpo e da gravidade, mantendo a lesão atual ou preferindo a recorrente
pub(crate) unsafe extern "thiscall" fn choose_injury_type(injuries: usize, category: i8, severity: i8, player: usize) -> u8 {
    let mut best: i8 = 0x64;
    let mut chosen: u8 = 0xff;
    let mut recorrente: u8 = 0xff;
    let a = exe::injury_get_player_injury(injuries, player);
    if a != 0 {
        let t = *(a as *const u8);
        if !matches!(t, 0 | 1 | 2 | 3 | 4 | 0x4c | 0x4d | 0x4e | 0x4f | 0x50 | 0x51) {
            return t;
        }
    }
    let kind = |k: i8, o: usize| *((0x9e7ad8 + o) as *const u8).offset(k as isize * 13);
    if player != 0 {
        let id = *(player as *const u32);
        let r = *((*(injuries as *const u32)).wrapping_add(id.wrapping_mul(31)).wrapping_add(0x12) as usize as *const u8);
        if r != 0xff && kind(r as i8, 1) as i8 == category {
            recorrente = r;
        }
    }
    let c = category as isize * 3;
    let mut bl = *((0x9e7f05 + c) as *const i8);
    let last = (0x9e7f06 + c) as *const i8;
    if bl > *last {
        return chosen;
    }
    loop {
        let g = kind(bl, 0xc) as i8;
        let serve = (severity == -1 || g == severity || g == -1) && severity != 3;
        if serve {
            let r = if bl as u8 == recorrente {
                (exe::utils_random(3) as u8).wrapping_sub(1) as i8
            } else {
                exe::utils_random(kind(bl, 2) as i8 as i32) as i8
            };
            if r < best {
                best = r;
                chosen = bl as u8;
            }
        }
        bl = bl.wrapping_add(1);
        if bl > *last {
            return chosen;
        }
    }
}

/// Compares the key date with that of a 0x16-byte return to training queue entry, for bsearch.
///
/// `injury_compare_entry_dates(key, item)` (cdecl), the bsearch (`exe::crt_bsearch`) order of the return to training queue
/// (0x16-byte entries) in Injury::post_due_recovery_news (`post_due_recovery_news`, every day): the date at +8 of the key before the
/// item's (`exe::date_operator_3c`), −1; after (`exe::date_operator_3e`), 1; equal, 0.
/// Original: `injury_compare_entry_dates` (0x632040, injury.cpp, cdecl).
///
/// [pt] { 0x632040 } (cdecl, injury.cpp): `injury_compare_entry_dates(chave, item)`, a ordem do bsearch (0x946464) da fila
/// das voltas aos treinos (entradas de 0x16 bytes) em Injury::post_due_recovery_news (0x62e010, todo dia): a data em
/// +8 da chave antes da do item (0x456770), −1; depois (0x456740), 1; igual, 0.
pub(crate) unsafe extern "C" fn injury_compare_entry_dates(key: usize, item: usize) -> i32 {
    if exe::date_operator_3c(key + 8, read_usize(item + 8) as u32, read_usize(item + recovery_news_entry::FULL_RETURN_DATE_LEAP_YEAR) as u32) != 0 {
        return -1;
    }
    if exe::date_operator_3e(key + 8, read_usize(item + 8) as u32, read_usize(item + recovery_news_entry::FULL_RETURN_DATE_LEAP_YEAR) as u32) != 0 {
        return 1;
    }
    crate::comparison::branch("cps_9 same date");
    0 // cps9: the return of today found
}

/// Returns the player's recurring injury type, or 0xff when he has none or is not a player.
///
/// `Injury::get_recurring_injury_type(person)`, from the news (0x77bc80), the physio's assessment (0x7a2650) and the
/// screens: the type of the recurring injury (+0x12 of the person's 31-byte fitness record, at [this]); null person, id
/// from [0xae2364] on, or no player sheet (+0x61): 0xff. The callers only read al.
/// Original: `Injury::get_recurring_injury_type` (0x62ed30, injury.cpp, thiscall, ret 4).
///
/// [pt] { 0x62ed30 } (thiscall, ret 4, injury.cpp): `Injury::get_recurring_injury_type(pessoa)`, pelas notícias (0x77bc80),
/// pela avaliação do fisioterapeuta (0x7a2650) e pelas telas: o tipo da lesão que volta (+0x12 da ficha de forma de 31
/// bytes da pessoa, em [this]); pessoa nula, id a partir de [0xae2364] ou sem a ficha de jogador (+0x61): 0xff. Os
/// chamadores leem só o al.
pub(crate) unsafe extern "thiscall" fn get_recurring_injury_type(this: usize, person: usize) -> u8 {
    let person = Staff(person);
    if person.is_null() || read_usize(person.addr()) as i32 >= read_usize(G_STAFF_COUNT) as i32 || person.player().is_null() {
        return 0xff;
    }
    *((read_usize(this) + 31 * read_usize(person.addr())) as *const u8).add(0x12)
}

