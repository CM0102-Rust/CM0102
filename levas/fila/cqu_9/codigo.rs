// ---------- %%LEVA%%a leva: os construtores das copas europeias: a Copa da UEFA, a Supercopa Europeia e a Intertoto (eur_super_cup.cpp, intertoto_cup.cpp, uefa_cup.cpp) ----------

unsafe fn cqu9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqu9_p8(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqu9_p16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqu9_p32(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// As rodadas da copa pelo método +0x3c da vtable (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall,
/// ret 0x10): a tabela das rodadas, gravada em +0xa3.
unsafe fn cqu9_rodadas(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqu9_d(cqu9_d(this) + 0x3c));
    let t = f(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    cqu9_p32(this + 0xa3, t as u32);
}

/// As estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// this; mov ecx, novo -> thiscall, ret 4) em +8; o quadro SEH da original cobre só esse new. Devolve se criou.
unsafe fn cqu9_estatisticas(this: usize) -> bool {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    cqu9_p32(this + 8, s as u32);
    s != 0
}

/// O plano que o comparador confere depois da original: a devolução (this), a vtable, o registro (+4) e o ano (+0x40);
/// na copa nova (+0x44 = 0xff), cada campo fixo que o construtor grava e as rodadas e as estatísticas criadas.
unsafe fn cqu9_plano(this: usize, r: usize, ano: u32, dono: usize, vt: usize, campos: &[(usize, u8, u32)], rodadas: bool) -> Option<String> {
    if r != this || cqu9_d(this) != vt || cqu9_d(this + 4) != dono || core::ptr::read_unaligned((this + 0x40) as *const u16) != ano as u16 {
        return Some(format!("devolveu {:#x}, vtable {:#x}, +4 {:#x}, ano {}", r, cqu9_d(this), cqu9_d(this + 4), core::ptr::read_unaligned((this + 0x40) as *const u16)));
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
    if (rodadas && cqu9_d(this + 0xa3) == 0) || cqu9_d(this + 8) == 0 {
        return Some(format!("rodadas {:#x}, estatísticas {:#x}", cqu9_d(this + 0xa3), cqu9_d(this + 8)));
    }
    None
}

/// { 0x9022e0 } (thiscall, ret 8, uefa_cup.cpp; da criação da carreira, n = 1): `UefaCup::UefaCup(ano, registro)`: o construtor de
/// a Copa da UEFA
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x970790, o registro em +4, o ano em +0x40 e
/// os da tabela CQU9_CAMPOS_COPA_DA_UEFA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: a semeadura europeia do ano (UefaSeeding::update_season 0x9035a0 em [0xde1f58], se houver; thiscall, ret 4); os inscritos (UefaCup::build_entrants) (0x902b90, thiscall, ret 0; a caixa da linha 0x54 se devolver 0); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x62 se devolver 0); as estatísticas em +8 (a caixa da linha 0x69 sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 2 (clubes de várias nações); fundo de prêmios de 3.000.000; 7 reservas no banco e 3 substituições; regra de disciplina 1 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu9_copa_da_uefa_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu9_p16(this + 0x40, ano as u16);
    cqu9_p32(this + 4, dono as u32);
    cqu9_p32(this, 0x970790);
    cqu9_p8(this + 0xab, 0x0);
    cqu9_p8(this + 0x44, 0xff);
    cqu9_p32(this + 0x30, 0xffffffff);
    cqu9_p32(this + 0x2c, 0x0);
    cqu9_p8(this + 0x42, 0x2);
    cqu9_p8(this + 0x52, 0x3);
    cqu9_p8(this + 0x49, 0x7);
    cqu9_p8(this + 0x4a, 0x3);
    cqu9_p8(this + 0x50, 0x1);
    cqu9_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqu_9 copa_da_uefa nova");
    // a semeadura europeia do ano (UefaSeeding::update_season 0x9035a0: push 0; mov ecx, [0xde1f58] -> thiscall, ret 4)
    let s = cqu9_d(0xde1f58);
    if s != 0 {
        let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x9035a0usize);
        f(s, 0);
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x902b90usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0xaa19e0, 0x54);
        return this;
    }
    cqu9_p32(this + 0x28, 0x2dc6c0);
    cqu9_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0xaa19e0, 0x62);
        return this;
    }
    if !cqu9_estatisticas(this) {
        show_error_at(0xaa19e0, 0x69);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQU9_CAMPOS_COPA_DA_UEFA: [(usize, u8, u32); 11] = [(0x28, 4, 0x2dc6c0), (0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x2), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x1), (0x52, 1, 0x3), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqu9_copa_da_uefa(this: usize, ano: u32, dono: usize) -> usize {
    cqu9_copa_da_uefa_corpo(this, ano, dono) // cqu9: copa_da_uefa
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu9_plano).
unsafe extern "thiscall" fn compare_cqu9_copa_da_uefa(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, ano, dono);
    let erro = cqu9_plano(this, r, ano, dono, 0x970790, &CQU9_CAMPOS_COPA_DA_UEFA, true);
    SCOREBOARD.record(erro.is_none(), || format!("cqu9_copa_da_uefa({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x57a100 } (thiscall, ret 8, eur_super_cup.cpp; da criação da carreira, n = 1): `EurSuperCup::EurSuperCup(ano, registro)`: o construtor de
/// a Supercopa Europeia
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x969f80, o registro em +4, o ano em +0x40 e
/// os da tabela CQU9_CAMPOS_SUPERCOPA_EUROPEIA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os dois clubes (EurSuperCup::select_teams) (0x57a500, thiscall, ret 0; a caixa da linha 0x3f se devolver negativo); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x4d se devolver negativo); as estatísticas em +8 (a caixa da linha 0x54 sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 2 (clubes de várias nações); sem fundo de prêmios (+0x28 fica como a base deixou); 7 reservas no banco e 3 substituições; regra de disciplina 1 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu9_supercopa_europeia_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu9_p16(this + 0x40, ano as u16);
    cqu9_p32(this + 4, dono as u32);
    cqu9_p32(this, 0x969f80);
    cqu9_p8(this + 0xab, 0x0);
    cqu9_p8(this + 0x44, 0xff);
    cqu9_p32(this + 0x2c, 0x0);
    cqu9_p32(this + 0x30, 0xffffffff);
    cqu9_p8(this + 0x42, 0x2);
    cqu9_p8(this + 0x52, 0x3);
    cqu9_p8(this + 0x49, 0x7);
    cqu9_p8(this + 0x4a, 0x3);
    cqu9_p8(this + 0x50, 0x1);
    cqu9_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqu_9 supercopa_europeia nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x57a500usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9c45f8, 0x3f);
        return this;
    }
    cqu9_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9c45f8, 0x4d);
        return this;
    }
    if !cqu9_estatisticas(this) {
        show_error_at(0x9c45f8, 0x54);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQU9_CAMPOS_SUPERCOPA_EUROPEIA: [(usize, u8, u32); 10] = [(0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x2), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x1), (0x52, 1, 0x3), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqu9_supercopa_europeia(this: usize, ano: u32, dono: usize) -> usize {
    cqu9_supercopa_europeia_corpo(this, ano, dono) // cqu9: supercopa_europeia
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu9_plano).
unsafe extern "thiscall" fn compare_cqu9_supercopa_europeia(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, ano, dono);
    let erro = cqu9_plano(this, r, ano, dono, 0x969f80, &CQU9_CAMPOS_SUPERCOPA_EUROPEIA, true);
    SCOREBOARD.record(erro.is_none(), || format!("cqu9_supercopa_europeia({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x632fb0 } (thiscall, ret 8, intertoto_cup.cpp; da criação da carreira, n = 1): `IntertotoCup::IntertotoCup(ano, registro)`: o construtor de
/// a Intertoto
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96bb8c, o registro em +4, o ano em +0x40 e
/// os da tabela CQU9_CAMPOS_INTERTOTO); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: a semeadura europeia do ano (UefaSeeding::update_season 0x9035a0 em [0xde1f58], se houver; thiscall, ret 4); as rodadas (o método +0x3c da vtable, em +0xa3); os 60 inscritos (IntertotoCup::collect_entrants) (0x6336c0, thiscall, ret 0; a caixa da linha 0x60 se devolver 0); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x67 se devolver 0); as estatísticas em +8 (a caixa da linha 0x6e sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 2 (clubes de várias nações); sem fundo de prêmios (+0x28 fica como a base deixou); 7 reservas no banco e 3 substituições; regra de disciplina 1 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu9_intertoto_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu9_p16(this + 0x40, ano as u16);
    cqu9_p32(this + 4, dono as u32);
    cqu9_p32(this, 0x96bb8c);
    cqu9_p8(this + 0xab, 0x0);
    cqu9_p8(this + 0x44, 0xff);
    cqu9_p32(this + 0x30, 0xffffffff);
    cqu9_p32(this + 0x2c, 0x0);
    cqu9_p8(this + 0x42, 0x2);
    cqu9_p8(this + 0x52, 0x3);
    cqu9_p8(this + 0x49, 0x7);
    cqu9_p8(this + 0x4a, 0x3);
    cqu9_p32(this + 0xb2, 0x0);
    cqu9_p8(this + 0x50, 0x1);
    cqu9_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqu_9 intertoto nova");
    // a semeadura europeia do ano (UefaSeeding::update_season 0x9035a0: push 0; mov ecx, [0xde1f58] -> thiscall, ret 4)
    let s = cqu9_d(0xde1f58);
    if s != 0 {
        let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x9035a0usize);
        f(s, 0);
    }
    cqu9_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x6336c0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9e9e14, 0x60);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9e9e14, 0x67);
        return this;
    }
    if !cqu9_estatisticas(this) {
        show_error_at(0x9e9e14, 0x6e);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQU9_CAMPOS_INTERTOTO: [(usize, u8, u32); 11] = [(0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x2), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x1), (0x52, 1, 0x3), (0xab, 1, 0x0), (0xb1, 1, 0x0), (0xb2, 4, 0x0)];

unsafe extern "thiscall" fn cqu9_intertoto(this: usize, ano: u32, dono: usize) -> usize {
    cqu9_intertoto_corpo(this, ano, dono) // cqu9: intertoto
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu9_plano).
unsafe extern "thiscall" fn compare_cqu9_intertoto(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%)(this, ano, dono);
    let erro = cqu9_plano(this, r, ano, dono, 0x96bb8c, &CQU9_CAMPOS_INTERTOTO, true);
    SCOREBOARD.record(erro.is_none(), || format!("cqu9_intertoto({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}
