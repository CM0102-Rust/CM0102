// ---------- %%LEVA%%a leva: a fluência da pessoa num idioma, recomeçar o histórico da pessoa e a posição de uma casa da formação ----------

unsafe fn cpk7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpk7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpk7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x7c6830 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::get_language_fluency(pessoa, clube, idioma,
/// pelo_contrato)`: quanto a pessoa fala o `idioma` (0 a 10). Técnico humano (id ≥ [0xae2364]): 10 se é o idioma
/// (+0x77) da nação dela (+0x1a), senão 0. Com `pelo_contrato`, o contrato com o clube (0x4cf5f0) de quem joga num
/// clube (+0x39) da mesma nação do `clube` (+0x53): o nível do contrato (+0x4f & 0xf). 10 se a nação (+0x1a) ou a
/// segunda (+0x1e) fala o idioma da nação do `clube` ou o `idioma`. Senão o idioma aprendido: a busca binária (bsearch
/// 0x946464, comparação 0x7c6d40, registros de 6 bytes em [+0x3c], [+0x44] deles) pela chave {id, idioma}, o byte +5;
/// sem a tabela ou sem o registro, 0. Os chamadores só leem o al.
unsafe extern "thiscall" fn get_language_fluency(this: usize, pessoa: usize, clube: usize, idioma: u32, pelo_contrato: u32) -> u8 {
    let idioma = idioma as u8;
    if pessoa != 0 && cpk7_d(pessoa) as i32 >= cpk7_d(0xae2364) as i32 {
        return if cpk7_b(cpk7_d(pessoa + 0x1a) + 0x77) == idioma { 10 } else { 0 }; // o técnico humano
    }
    if pelo_contrato != 0 {
        let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
        let c = contrato(cpk7_d(0xae1050), pessoa, clube);
        if c != 0 {
            let cp = cpk7_d(pessoa + 0x39);
            if cp != 0 && cpk7_d(clube + 0x53) == cpk7_d(cp + 0x53) {
                return cpk7_b(c + 0x4f) & 0xf;
            }
        }
    }
    let fala = |l: u8| {
        if cpk7_b(cpk7_d(pessoa + 0x1a) + 0x77) == l {
            return true;
        }
        let s = cpk7_d(pessoa + 0x1e);
        s != 0 && cpk7_b(s + 0x77) == l // a segunda nação
    };
    if clube != 0 {
        let n = cpk7_d(clube + 0x53);
        if n != 0 && fala(cpk7_b(n + 0x77)) {
            return 10;
        }
    }
    if fala(idioma) {
        return 10;
    }
    let base = cpk7_d(this + 0x3c);
    if base == 0 {
        return 0;
    }
    let mut chave = [0u8; 8];
    chave[..4].copy_from_slice(&(cpk7_d(pessoa) as u32).to_le_bytes());
    chave[4] = idioma;
    let bsearch: unsafe extern "C" fn(*const u8, usize, usize, usize, usize) -> usize = core::mem::transmute(0x946464usize);
    let e = bsearch(chave.as_ptr(), base, cpk7_d(this + 0x44), 6, 0x7c6d40);
    if e == 0 {
        return 0;
    }
    cpk7_b(e + 5)
}

/// { 0x86fb80 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::rewind_staff_history(pessoa)`: a lista do
/// histórico da pessoa (o par de 8 bytes em [+0xc] + 8 × id) fica sem o último (0x58f110 sobre [this]: o segundo
/// deslocamento = −1). Pessoa nula: a original (caixa fatal: o jogo termina).
unsafe extern "thiscall" fn rewind_staff_history(this: usize, pessoa: usize) {
    if pessoa == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, pessoa); // caixa fatal: só a original
    }
    crate::comparison::branch("cpk_7 recomecar historico");
    let par = cpk7_d(this + 0xc).wrapping_add(cpk7_d(pessoa).wrapping_mul(8));
    let sem_ultimo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x58f110usize);
    sem_ultimo(cpk7_d(this), par, par + 4); // o último deslocamento
}

/// { 0x5b2ba0 } (thiscall, ret 0xc, formation.cpp): `Formation::set_slot_position(casa, posicao, extra)`: sem `extra`,
/// a posição geral da casa (+0x12d, palavras) muda para `posicao` (se mudar: a máscara 0x5b59a0, a marca +0x143 da casa
/// = 1 e a das outras casas com a mesma posição = 0); com ou sem, a posição extra (+0x14e) = `posicao`. Depois, para
/// cada faixa 0..2 e altura 0..3 (a palavra baixa e a alta do argumento da 0x5b7e60), as casas no campo sem e com a
/// bola (+0x164 e +0x194 + 4 × (altura + 4 × faixa + 24 × casa)): com geral = extra, as duas da geral; senão, pelos
/// setores (0x5b73a0): extra de setor abaixo do da geral, sem a bola a extra e com a bola a geral; senão o contrário.
/// Por fim +0x187f = 0. Devolve 1. Casa fora de 0..10: a original (caixa "Error" e 0).
unsafe extern "thiscall" fn set_slot_position(this: usize, casa: u32, posicao: u32, extra: u32) -> u32 {
    let k = casa as u8 as i8;
    if !(0..=10).contains(&k) {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(%%I2%%)(this, casa, posicao, extra); // caixa: só a original
    }
    let s = k as usize;
    let p = posicao as u16 as i16;
    let geral = this + 0x12d + 2 * s;
    let extra_da_casa = this + 0x14e + 2 * s;
    if extra == 0 && cpk7_w(geral) != p {
        core::ptr::write_unaligned(geral as *mut i16, p);
        let mascara: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5b59a0usize);
        mascara(this);
        *((this + 0x143 + s) as *mut u8) = 1;
        for j in 0..11usize {
            if j != s && cpk7_w(this + 0x12d + 2 * j) == p {
                *((this + 0x143 + j) as *mut u8) = 0; // a outra casa com a mesma posição
            }
        }
    }
    core::ptr::write_unaligned(extra_da_casa as *mut i16, p);
    // a 0x5b7e60 e a 0x5b73a0 só leem a palavra baixa da máscara (a original empurra o eax com o alto que estiver)
    let pitch_cell: unsafe extern "C" fn(u32, u32, u32, *mut u32) -> u32 = core::mem::transmute(0x5b7e60usize);
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(0x5b73a0usize);
    // as saídas valem de uma volta para a outra, como as locais da original (sem valor só se a 0x5b73a0 falhar
    // logo na primeira: zero aqui)
    let (mut setor_geral, mut setor_extra, mut lado, mut saida) = (0u16, 0u16, 0u16, 0u32);
    if cpk7_w(geral) == cpk7_w(extra_da_casa) {
        crate::comparison::branch("cpk_7 formacao mesma posicao");
    } else {
        crate::comparison::branch("cpk_7 formacao posicao extra");
    }
    for band in 0..3usize {
        for altura in 0..4usize {
            let arg = band as u32 | (altura as u32) << 16;
            let i = altura + 4 * (band + 6 * s);
            let (sem_bola, com_bola) = (this + 0x164 + 4 * i, this + 0x194 + 4 * i);
            let (g, e) = (cpk7_w(geral) as u16 as u32, cpk7_w(extra_da_casa) as u16 as u32);
            let (a, b) = if g == e {
                (g, g) // a mesma posição com e sem a bola
            } else {
                separar(g, &mut setor_geral, &mut lado);
                separar(e, &mut setor_extra, &mut lado);
                if (setor_extra as i16) < (setor_geral as i16) { // a extra mais atrás
                    (e, g)
                } else {
                    (g, e)
                }
            };
            pitch_cell(a, 0, arg, &mut saida);
            core::ptr::write_unaligned(sem_bola as *mut u32, saida);
            pitch_cell(b, 1, arg, &mut saida);
            core::ptr::write_unaligned(com_bola as *mut u32, saida);
        }
    }
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    1
}

/// Comparador: pelo valor (a 0x4cf5f0 e a busca só leem).
unsafe extern "thiscall" fn comparar_get_language_fluency(this: usize, pessoa: usize, clube: usize, idioma: u32, pelo_contrato: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8>(%%I0%%)(this, pessoa, clube, idioma, pelo_contrato);
    let n = get_language_fluency(this, pessoa, clube, idioma, pelo_contrato);
    SCOREBOARD.record(r == n, || format!("get_language_fluency({:x}, {:x}, {}, {}) original={} rust={}", pessoa, clube, idioma as u8, pelo_contrato, r, n));
    r
}

/// Comparador: lado a lado pelo par da pessoa (8 bytes; a 0x58f110 só grava o segundo).
unsafe extern "thiscall" fn comparar_rewind_staff_history(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let par = cpk7_d(this + 0xc).wrapping_add(cpk7_d(pessoa).wrapping_mul(8));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side_light(&[(par, 8)], || o(this, pessoa), || rewind_staff_history(this, pessoa));
    SCOREBOARD.record(igual, || format!("rewind_staff_history({}) {:x?} x {:x?}", cpk7_d(pessoa), mo, mr));
}

/// Comparador: lado a lado pela formação (+0x12d a +0x584: posições, marcas e casas no campo; a máscara +0x187b e
/// +0x187f) e [0xb67a34] (a 0x5b73a0 zera quando falha).
unsafe extern "thiscall" fn comparar_set_slot_position(this: usize, casa: u32, posicao: u32, extra: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(%%I2%%);
    if !(0..=10).contains(&(casa as u8 as i8)) {
        return o(this, casa, posicao, extra);
    }
    let reg = [(this + 0x12d, 0x584 - 0x12d), (this + 0x187b, 8), (0xb67a34, 4)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, casa, posicao, extra), || set_slot_position(this, casa, posicao, extra));
    SCOREBOARD.record(igual && r == n, || format!("set_slot_position({}, {:#x}, {}) {:x?} x {:x?}", casa as u8 as i8, posicao as u16, extra, mo, mr));
    r
}
