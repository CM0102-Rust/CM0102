// ---------- %%LEVA%%a leva: a disciplina: contar o jogo cumprido da suspensão, soltar a suspensão e apagar o histórico de suspensões da pessoa (discipline.cpp) ----------

unsafe fn cpm4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpm4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de 29 bytes da pessoa `id` em [this+0x14] (+0x14/+0x18: o começo e o último deslocamento do histórico).
unsafe fn person_discipline_record(this: usize, id: usize) -> usize {
    cpm4_d(this + 0x14).wrapping_add(id.wrapping_mul(29))
}

/// O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
unsafe fn ruling_body_for_rule(this: usize, regra: u8) -> usize {
    cpm4_d(cpm4_d(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)))
}

/// A entrada (14 bytes; +6 o índice da suspensão, −1 sem) da pessoa no órgão: 0x7e82e0 sem criar (só lê).
unsafe fn ruling_body_entry(orgao: usize, id: u32) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x7e82e0usize);
    achar(orgao, id, 0)
}

/// O item (id, nação) da pessoa no índice das nações (+0xc, 8 bytes por item, conta +0x12 com sinal): bsearch
/// (0x946464 com 0x55e5f0) pela nação (+0x1a) e, sem achar, pela segunda nação (+0x1e, se houver). 0 sem achar.
unsafe fn find_in_nation_index(this: usize, id: u32) -> usize {
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let pessoa = cpm4_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    let mut chave = [id, cpm4_d(cpm4_d(pessoa + 0x1a)) as u32];
    let e = bsearch(chave.as_ptr(), cpm4_d(this + 0xc), cpm4_w(this + 0x12) as i32, 8, 0x55e5f0);
    if e != 0 {
        return e;
    }
    let segunda = cpm4_d(cpm4_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e)) + 0x1e);
    if segunda == 0 {
        return 0;
    }
    chave[1] = cpm4_d(segunda) as u32;
    bsearch(chave.as_ptr(), cpm4_d(this + 0xc), cpm4_w(this + 0x12) as i32, 8, 0x55e5f0)
}

/// O que a 0x556e90 faz com o jogo: a entrada no órgão, a suspensão (20 bytes em [this+4]), o contador novo de jogos
/// cumpridos (+1) e se a suspensão termina.
struct SuspensionMatchUpdate {
    entrada: usize,
    suspensao: usize,
    contador: Option<u8>,
    cumprida: bool,
}

/// O plano do jogo (só lê; a 0x7e82e0 sem criar e a 0x54c5b0 só gravam a data de rascunho dela): a suspensão da
/// pessoa no órgão da `regra`; por jogos (+0 ≠ 0xff): a partir do começo (dia +4, ano +6; hoje [0xae2c90]: dia +0,
/// ano +2), com `conta`, +1 no contador (+1), e termina ao chegar em +0; por dias (+0 = 0xff): termina quando hoje
/// chega ao começo + dias (+2) (0x54c5b0).
unsafe fn serve_ban_match_plan(this: usize, id: u32, regra: u32, conta: u32) -> SuspensionMatchUpdate {
    let mut j = SuspensionMatchUpdate { entrada: 0, suspensao: 0, contador: None, cumprida: false };
    let e = ruling_body_entry(ruling_body_for_rule(this, regra as u8), id);
    if e == 0 || cpm4_w(e + 6) == -1 {
        return j;
    }
    j.entrada = e;
    let cinco = (cpm4_w(e + 6) as i32 as u32).wrapping_mul(5); // o lea da original (o alto vai no edx abaixo)
    let s = cpm4_d(this + 4).wrapping_add((cinco as usize).wrapping_mul(4));
    j.suspensao = s;
    let hoje = cpm4_d(0xae2c90) as u32;
    let (dia, ano) = (hoje as u16 as i16, (hoje >> 16) as u16);
    let jogos = *(s as *const u8);
    if jogos != 0xff {
        let (dia0, ano0) = (cpm4_w(s + 4), cpm4_w(s + 6) as u16);
        if ano < ano0 || (ano == ano0 && dia < dia0) { // ainda não começou
            crate::comparison::branch("cpm_4 antes do inicio");
            return j;
        }
        if conta == 0 {
            return j;
        }
        crate::comparison::branch("cpm_4 jogo cumprido");
        let n = (*((s + 1) as *const u8)).wrapping_add(1); // mais um jogo cumprido
        j.contador = Some(n);
        j.cumprida = n == jogos;
    } else {
        let mut fim = [0u8; 8];
        let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        // a original empurra o edx com os dias no dx e, no alto, o do índice × 5 (zero abaixo de 13108 suspensões)
        let f = mais_dias(s + 4, fim.as_mut_ptr(), (cinco & 0xffff_0000) | cpm4_w(s + 2) as u16 as u32);
        let (dia1, ano1) = (cpm4_w(f), cpm4_w(f + 2) as u16);
        if ano < ano1 || (ano == ano1 && dia < dia1) {
            return j;
        }
        j.cumprida = true;
    }
    j
}

/// { 0x556e90 } (thiscall, ret 0xc, discipline.cpp): `Discipline::update_ban_progress(id, regra, conta)`: a suspensão
/// da pessoa no órgão da `regra` (plano em `serve_ban_match_plan`): por jogos, grava o contador novo (+1); cumprida, vai para o
/// histórico (0x58e340 sobre [this+0x18] com o começo e o último deslocamento do registro de 29 bytes) e é solta
/// (0x556d00). Sem SEH, sem sorteio, sem caixa.
unsafe extern "thiscall" fn update_ban_progress(this: usize, id: u32, regra: u32, conta: u32) {
    let j = serve_ban_match_plan(this, id, regra, conta);
    if let Some(n) = j.contador {
        *((j.suspensao + 1) as *mut u8) = n;
    }
    if !j.cumprida {
        return;
    }
    let r = person_discipline_record(this, id as usize);
    let historico: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(0x58e340usize);
    let g = historico(cpm4_d(this + 0x18), r + 0x14, r + 0x18, j.suspensao);
    let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x556d00usize);
    // o índice vai no ax; o alto do eax é o da devolução da 0x58e340 (a 0x556d00 só lê a palavra)
    soltar(this, id, (g as u32 & 0xffff_0000) | cpm4_w(j.entrada + 6) as u16 as u32);
}

/// { 0x556d00 } (thiscall, ret 8, discipline.cpp): `Discipline::release_ban(id, indice)`: a suspensão `indice` (20
/// bytes em [this+4]; a palavra com sinal): para cada uma das 4 regras (+0xc..+0xf, 0xff = sem), o órgão a esquece
/// (0x7e8590); na quarta, o item da pessoa sai do índice das nações (o último vem para o lugar, conta +0x12 − 1, qsort
/// 0x945333 com 0x55e5f0; sem achar, caixa "Error" e segue). Por fim a suspensão fica livre (+0 = 0xff, +2 = −1).
/// Devolve a suspensão.
unsafe extern "thiscall" fn release_ban(this: usize, id: u32, indice: u32) -> usize {
    let s = cpm4_d(this + 4).wrapping_add((indice as u16 as i16 as i32 as usize).wrapping_mul(20));
    let esquecer: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x7e8590usize);
    for k in 0..4usize {
        let regra = *((s + 0xc + k) as *const u8);
        if regra == 0xff {
            continue;
        }
        crate::comparison::branch("cpm_4 soltar do orgao");
        esquecer(ruling_body_for_rule(this, regra), id); // o órgão esquece a suspensão
        if k != 3 {
            continue;
        }
        let e = find_in_nation_index(this, id);
        if e == 0 {
            show_error_at(0x9b8f18, 0x47d);
            continue;
        }
        let n = cpm4_w(this + 0x12).wrapping_sub(1);
        core::ptr::write_unaligned((this + 0x12) as *mut i16, n);
        let ultimo = cpm4_d(this + 0xc).wrapping_add((n as i32 as usize).wrapping_mul(8));
        core::ptr::write_unaligned(e as *mut u32, cpm4_d(ultimo) as u32);
        core::ptr::write_unaligned((e + 4) as *mut u32, cpm4_d(ultimo + 4) as u32);
        let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
        qsort(cpm4_d(this + 0xc), cpm4_w(this + 0x12) as i32, 8, 0x55e5f0);
    }
    *(s as *mut u8) = 0xff;
    core::ptr::write_unaligned((s + 2) as *mut u16, 0xffff);
    s
}

/// { 0x55e1d0 } (thiscall, ret 4, discipline.cpp): `Discipline::clear_ban_history(pessoa)`: o histórico de suspensões
/// da pessoa (registro de 29 bytes do id em [this+0x14]) fica sem o último (0x58f110 sobre [this+0x18]: +0x18 = −1).
/// Devolve o que a 0x58f110 devolve. Pessoa nula ou não jogadora (+0x61): a original (caixa fatal).
unsafe extern "thiscall" fn clear_ban_history(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 || cpm4_d(pessoa + 0x61) == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%)(this, pessoa); // caixa fatal: só a original
    }
    let r = person_discipline_record(this, cpm4_d(pessoa));
    let sem_ultimo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x58f110usize);
    sem_ultimo(cpm4_d(this + 0x18), r + 0x14, r + 0x18)
}

/// Comparador: a 0x58e340 grava o histórico no arquivo (não pode rodar duas vezes): o plano antes, a original roda;
/// confere o contador, ou a suspensão solta (livre e a entrada do órgão sem índice), ou nada mudado.
unsafe extern "thiscall" fn compare_update_ban_progress(this: usize, id: u32, regra: u32, conta: u32) {
    let j = serve_ban_match_plan(this, id, regra, conta);
    let antes = if j.suspensao != 0 { *((j.suspensao + 1) as *const u8) } else { 0 };
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32)>(%%I0%%)(this, id, regra, conta);
    if j.suspensao == 0 {
        return;
    }
    let s = j.suspensao;
    let (jogos, contador, dias) = (*(s as *const u8), *((s + 1) as *const u8), cpm4_w(s + 2));
    let ok = if j.cumprida {
        jogos == 0xff && dias == -1 && cpm4_w(j.entrada + 6) == -1
    } else {
        contador == j.contador.unwrap_or(antes)
    };
    SCOREBOARD.record(ok, || {
        format!("update_ban_progress({}, {}, {}) contador {} (esperava {:?}, antes {}), cumprida={} suspensão {:#x}/{} entrada {}",
            id, regra as u8, conta, contador, j.contador, antes, j.cumprida, jogos, dias, cpm4_w(j.entrada + 6))
    });
}

/// Comparador: lado a lado pela suspensão, o índice das nações (conta e itens) e as entradas da pessoa nos órgãos das
/// regras (achadas antes, sem criar). Quarta regra sem o item no índice: só a original (caixa).
unsafe extern "thiscall" fn compare_release_ban(this: usize, id: u32, indice: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(%%I1%%);
    let s = cpm4_d(this + 4).wrapping_add((indice as u16 as i16 as i32 as usize).wrapping_mul(20));
    if *((s + 0xf) as *const u8) != 0xff && find_in_nation_index(this, id) == 0 {
        return o(this, id, indice);
    }
    let mut reg = vec![(s, 20), (this + 0x10, 4)];
    let n = cpm4_w(this + 0x12);
    if n > 0 {
        reg.push((cpm4_d(this + 0xc), n as usize * 8));
    }
    for k in 0..4usize {
        let regra = *((s + 0xc + k) as *const u8);
        if regra != 0xff {
            let e = ruling_body_entry(ruling_body_for_rule(this, regra), id);
            if e != 0 {
                reg.push((e, 14));
            }
        }
    }
    let (r, m, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, id, indice), || release_ban(this, id, indice));
    SCOREBOARD.record(igual && r == m, || format!("release_ban({}, {}) {:x?} x {:x?}", id, indice as u16 as i16, mo, mr));
    r
}

/// Comparador: lado a lado pelo começo e o último deslocamento do histórico (8 bytes; a 0x58f110 só grava o segundo).
unsafe extern "thiscall" fn compare_clear_ban_history(this: usize, pessoa: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I2%%);
    if pessoa == 0 || cpm4_d(pessoa + 0x61) == 0 {
        return o(this, pessoa);
    }
    let r = person_discipline_record(this, cpm4_d(pessoa));
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side_light(&[(r + 0x14, 8)], || o(this, pessoa), || clear_ban_history(this, pessoa));
    SCOREBOARD.record(igual && a == b, || format!("clear_ban_history({}) {:x?} x {:x?}", cpm4_d(pessoa), mo, mr));
    a
}
