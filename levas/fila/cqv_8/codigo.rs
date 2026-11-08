// ---------- %%LEVA%%a leva: os construtores da Copa da Itália e da Copa da Série C (ita_c_cup.cpp, ita_cup.cpp) ----------

unsafe fn cqv8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqv8_p8(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqv8_p16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqv8_p32(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// As rodadas da copa pelo método +0x3c da vtable (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall,
/// ret 0x10): a tabela das rodadas, gravada em +0xa3.
unsafe fn cqv8_rodadas(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqv8_d(cqv8_d(this) + 0x3c));
    let t = f(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    cqv8_p32(this + 0xa3, t as u32);
}

/// As estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// this; mov ecx, novo -> thiscall, ret 4) em +8; o quadro SEH da original cobre só esse new. Devolve se criou.
unsafe fn cqv8_estatisticas(this: usize) -> bool {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    cqv8_p32(this + 8, s as u32);
    s != 0
}

/// O plano que o comparador confere depois da original: a devolução (this), a vtable, o registro (+4) e o ano (+0x40);
/// na copa nova (+0x44 = 0xff), cada campo fixo que o construtor grava e as rodadas e as estatísticas criadas.
unsafe fn cqv8_plano(this: usize, r: usize, ano: u32, dono: usize, vt: usize, campos: &[(usize, u8, u32)], rodadas: bool) -> Option<String> {
    if r != this || cqv8_d(this) != vt || cqv8_d(this + 4) != dono || core::ptr::read_unaligned((this + 0x40) as *const u16) != ano as u16 {
        return Some(format!("devolveu {:#x}, vtable {:#x}, +4 {:#x}, ano {}", r, cqv8_d(this), cqv8_d(this + 4), core::ptr::read_unaligned((this + 0x40) as *const u16)));
    }
    if *((this + 0x44) as *const u8) != 0xff {
        return None;
    }
    for &(o, t, v) in campos {
        let x = match t {
            1 => *((this + o) as *const u8) as u32,
            2 => core::ptr::read_unaligned((this + o) as *const u16) as u32,
            _ => core::ptr::read_unaligned((this + o) as *const u32),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, o construtor grava {:#x}", o, x, v));
        }
    }
    if (rodadas && cqv8_d(this + 0xa3) == 0) || cqv8_d(this + 8) == 0 {
        return Some(format!("rodadas {:#x}, estatísticas {:#x}", cqv8_d(this + 0xa3), cqv8_d(this + 8)));
    }
    None
}

/// { 0x63e900 } (thiscall, ret 8, ita_cup.cpp; da criação da carreira, n = 1): `ItaCup::ItaCup(ano, registro)`: o construtor de
/// a Copa da Itália
/// sobre a base League::League (0x682200, thiscall, ret 0); os campos fixos (a vtable 0x96c318, o registro em +4, o ano em +0x40 e
/// os da tabela CQV8_CAMPOS_COPA_DA_ITALIA); se o estado salvo carrega (League::load_from_temp_file 0x687b10: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: o vetor das fases em +0xc (malloc 0x944e46 de 0x20: push; add esp, 4 -> cdecl, sem conferir); os 48 inscritos (ItaCup::select_entrants) (0x63faf0, thiscall, ret 0, sem conferir); as estatísticas em +8 (a caixa da linha 0x54 sem elas); as posições previstas (ItaCup::set_predicted_positions) (0x640210, thiscall, ret 0, sem conferir); as regras (ItaCup::set_up_rules) (0x63f350, thiscall, ret 0; a caixa da linha 0x5e se devolver 0); a tabela do grupo 1 (ItaCup::create_first_group_table) (0x63f250, thiscall, ret 0; a caixa da linha 0x65 se devolver 0); os jogos (League::create_fixtures) (0x6835c0, thiscall, ret 0; a caixa da linha 0x6b se devolver 0); a ordem da tabela (League::sort_table) (0x6827d0, thiscall, push 0, ret 4; a caixa da linha 0x72 se devolver 0); a fase de cada grupo (ItaCup::create_group_stage) para os grupos 0 a 6 (0x63f640, push i; mov ecx, this -> thiscall, ret 4); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: sem fundo de prêmios (+0x28 fica como a base deixou); 7 reservas no banco e 3 substituições; regra de disciplina 18 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqv8_copa_da_italia_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    base(this);
    cqv8_p32(this + 4, dono as u32);
    cqv8_p32(this, 0x96c318);
    cqv8_p16(this + 0x40, ano as u16);
    cqv8_p8(this + 0x4a, 0x3);
    cqv8_p8(this + 0x49, 0x7);
    cqv8_p8(this + 0x50, 0x12);
    cqv8_p8(this + 0x51, 0xc);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqv_8 copa_da_italia nova");
    cqv8_p16(this + 0xd9, 0x28);
    cqv8_p8(this + 0x44, 0xff);
    cqv8_p32(this + 0x30, 0xffffffff);
    cqv8_p32(this + 0x2c, 0x8);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    cqv8_p32(this + 0xc, alocar(0x20) as u32);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63faf0usize);
    f(this);
    if !cqv8_estatisticas(this) {
        show_error_at(0x9ea23c, 0x54);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x640210usize);
    f(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63f350usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea23c, 0x5e);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63f250usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea23c, 0x65);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x6835c0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea23c, 0x6b);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x6827d0usize);
    let r = f(this, 0);
    if r == 0 {
        show_error_at(0x9ea23c, 0x72);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x63f640usize);
    for i in 0..7 {
        f(this, i);
    }
    this
}

const CQV8_CAMPOS_COPA_DA_ITALIA: [(usize, u8, u32); 7] = [(0x2c, 4, 0x8), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x12), (0x51, 1, 0xc), (0xd9, 2, 0x28)];

unsafe extern "thiscall" fn cqv8_copa_da_italia(this: usize, ano: u32, dono: usize) -> usize {
    cqv8_copa_da_italia_corpo(this, ano, dono) // cqv8: copa_da_italia
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqv8_plano).
unsafe extern "thiscall" fn comparar_cqv8_copa_da_italia(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, ano, dono);
    let erro = cqv8_plano(this, r, ano, dono, 0x96c318, &CQV8_CAMPOS_COPA_DA_ITALIA, false);
    PLACAR.record(erro.is_none(), || format!("cqv8_copa_da_italia({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x63cc40 } (thiscall, ret 8, ita_c_cup.cpp; da criação da carreira, n = 1): `ItaCCup::ItaCCup(ano, registro)`: o construtor de
/// a Copa da Série C
/// sobre a base League::League (0x682200, thiscall, ret 0); os campos fixos (a vtable 0x96c264, o registro em +4, o ano em +0x40 e
/// os da tabela CQV8_CAMPOS_COPA_DA_SERIE_C); se o estado salvo carrega (League::load_from_temp_file 0x687b10: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: o vetor das fases em +0xc (malloc 0x944e46 de 0x40: push; add esp, 4 -> cdecl, sem conferir); os 90 inscritos (ItaCCup::select_entrants) (0x63dfe0, thiscall, ret 0, sem conferir); as estatísticas em +8 (a caixa da linha 0x4f sem elas); as posições previstas (ItaCCup::set_predicted_positions) (0x63e580, thiscall, ret 0, sem conferir); as regras (ItaCCup::set_up_rules) (0x63d6e0, thiscall, ret 0; a caixa da linha 0x59 se devolver 0); a tabela do grupo 1 (ItaCCup::init_first_group) (0x63d5e0, thiscall, ret 0; a caixa da linha 0x60 se devolver 0); os jogos (League::create_fixtures) (0x6835c0, thiscall, ret 0; a caixa da linha 0x66 se devolver 0); a ordem da tabela (League::sort_table) (0x6827d0, thiscall, push 0, ret 4; a caixa da linha 0x6d se devolver 0); a fase de cada grupo (ItaCCup::create_group_stage) para os grupos 0 a 14 (0x63d9e0, push i; mov ecx, this -> thiscall, ret 4); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: sem fundo de prêmios (+0x28 fica como a base deixou); 7 reservas no banco e 3 substituições; regra de disciplina 18 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqv8_copa_da_serie_c_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    base(this);
    cqv8_p32(this + 4, dono as u32);
    cqv8_p32(this, 0x96c264);
    cqv8_p16(this + 0x40, ano as u16);
    cqv8_p8(this + 0x4a, 0x3);
    cqv8_p8(this + 0x49, 0x7);
    cqv8_p8(this + 0x50, 0x12);
    cqv8_p8(this + 0x51, 0xc);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqv_8 copa_da_serie_c nova");
    cqv8_p16(this + 0xd9, 0x28);
    cqv8_p8(this + 0x44, 0xff);
    cqv8_p32(this + 0x30, 0xffffffff);
    cqv8_p32(this + 0x2c, 0x10);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    cqv8_p32(this + 0xc, alocar(0x40) as u32);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63dfe0usize);
    f(this);
    if !cqv8_estatisticas(this) {
        show_error_at(0x9ea204, 0x4f);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63e580usize);
    f(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63d6e0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea204, 0x59);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63d5e0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea204, 0x60);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x6835c0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9ea204, 0x66);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x6827d0usize);
    let r = f(this, 0);
    if r == 0 {
        show_error_at(0x9ea204, 0x6d);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x63d9e0usize);
    for i in 0..15 {
        f(this, i);
    }
    this
}

const CQV8_CAMPOS_COPA_DA_SERIE_C: [(usize, u8, u32); 7] = [(0x2c, 4, 0x10), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x12), (0x51, 1, 0xc), (0xd9, 2, 0x28)];

unsafe extern "thiscall" fn cqv8_copa_da_serie_c(this: usize, ano: u32, dono: usize) -> usize {
    cqv8_copa_da_serie_c_corpo(this, ano, dono) // cqv8: copa_da_serie_c
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqv8_plano).
unsafe extern "thiscall" fn comparar_cqv8_copa_da_serie_c(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, ano, dono);
    let erro = cqv8_plano(this, r, ano, dono, 0x96c264, &CQV8_CAMPOS_COPA_DA_SERIE_C, false);
    PLACAR.record(erro.is_none(), || format!("cqv8_copa_da_serie_c({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}
