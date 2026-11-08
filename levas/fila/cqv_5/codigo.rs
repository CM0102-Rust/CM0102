// ---------- %%LEVA%%a leva: os construtores das supercopas italianas e da Copa dos EUA (ita_c1_super.cpp, ita_super.cpp, usa_open_cup.cpp) ----------

unsafe fn cqv5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqv5_p8(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqv5_p16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqv5_p32(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// As rodadas da copa pelo método +0x3c da vtable (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall,
/// ret 0x10): a tabela das rodadas, gravada em +0xa3.
unsafe fn cqv5_rodadas(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqv5_d(cqv5_d(this) + 0x3c));
    let t = f(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    cqv5_p32(this + 0xa3, t as u32);
}

/// As estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// this; mov ecx, novo -> thiscall, ret 4) em +8; o quadro SEH da original cobre só esse new. Devolve se criou.
unsafe fn cqv5_estatisticas(this: usize) -> bool {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    cqv5_p32(this + 8, s as u32);
    s != 0
}

/// O plano que o comparador confere depois da original: a devolução (this), a vtable, o registro (+4) e o ano (+0x40);
/// na copa nova (+0x44 = 0xff), cada campo fixo que o construtor grava e as rodadas e as estatísticas criadas.
unsafe fn cqv5_plano(this: usize, r: usize, ano: u32, dono: usize, vt: usize, campos: &[(usize, u8, u32)], rodadas: bool) -> Option<String> {
    if r != this || cqv5_d(this) != vt || cqv5_d(this + 4) != dono || core::ptr::read_unaligned((this + 0x40) as *const u16) != ano as u16 {
        return Some(format!("devolveu {:#x}, vtable {:#x}, +4 {:#x}, ano {}", r, cqv5_d(this), cqv5_d(this + 4), core::ptr::read_unaligned((this + 0x40) as *const u16)));
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
    if (rodadas && cqv5_d(this + 0xa3) == 0) || cqv5_d(this + 8) == 0 {
        return Some(format!("rodadas {:#x}, estatísticas {:#x}", cqv5_d(this + 0xa3), cqv5_d(this + 8)));
    }
    None
}

/// { 0x65f670 } (thiscall, ret 8, ita_super.cpp; da criação da carreira, n = 1): `ItaSuper::ItaSuper(ano, registro)`: o construtor de
/// a Supercopa italiana
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96c8b8, o registro em +4, o ano em +0x40 e
/// os da tabela CQV5_CAMPOS_SUPERCOPA_ITALIANA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os dois clubes (ItaSuper::init_team_list) (0x65fa60, thiscall, ret 0; a caixa da linha 0x43 se devolver negativo); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x51 se devolver negativo); as estatísticas em +8 (a caixa da linha 0x58 sem elas); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); sem fundo de prêmios (+0x28 fica como a base deixou); 7 reservas no banco e 3 substituições; regra de disciplina 17 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqv5_supercopa_italiana_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqv5_p32(this + 4, dono as u32);
    cqv5_p32(this, 0x96c8b8);
    cqv5_p16(this + 0x40, ano as u16);
    cqv5_p8(this + 0xab, 0x0);
    cqv5_p8(this + 0x44, 0xff);
    cqv5_p32(this + 0x2c, 0x0);
    cqv5_p32(this + 0x30, 0xffffffff);
    cqv5_p8(this + 0x42, 0x1);
    cqv5_p8(this + 0x4a, 0x3);
    cqv5_p8(this + 0x49, 0x7);
    cqv5_p8(this + 0x50, 0x11);
    cqv5_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqv_5 supercopa_italiana nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x65fa60usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9ea934, 0x43);
        return this;
    }
    cqv5_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9ea934, 0x51);
        return this;
    }
    if !cqv5_estatisticas(this) {
        show_error_at(0x9ea934, 0x58);
        return this;
    }
    this
}

const CQV5_CAMPOS_SUPERCOPA_ITALIANA: [(usize, u8, u32); 9] = [(0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x11), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqv5_supercopa_italiana(this: usize, ano: u32, dono: usize) -> usize {
    cqv5_supercopa_italiana_corpo(this, ano, dono) // cqv5: supercopa_italiana
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqv5_plano).
unsafe extern "thiscall" fn comparar_cqv5_supercopa_italiana(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, ano, dono);
    let erro = cqv5_plano(this, r, ano, dono, 0x96c8b8, &CQV5_CAMPOS_SUPERCOPA_ITALIANA, true);
    PLACAR.record(erro.is_none(), || format!("cqv5_supercopa_italiana({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x63c5e0 } (thiscall, ret 8, ita_c1_super.cpp; da criação da carreira, n = 1): `ItaC1Super::ItaC1Super(ano, registro)`: o construtor de
/// a Supercopa da Série C1
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96c1c4, o registro em +4, o ano em +0x40 e
/// os da tabela CQV5_CAMPOS_SUPERCOPA_DA_SERIE_C1); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: a lista dos dois clubes (ItaC1Super::init_team_list) (0x63c9d0, thiscall, ret 0; a caixa da linha 0x43 se devolver negativo); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x51 se devolver negativo); as estatísticas em +8 (a caixa da linha 0x58 sem elas); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); sem fundo de prêmios (+0x28 fica como a base deixou); 7 reservas no banco e 3 substituições; regra de disciplina 17 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqv5_supercopa_da_serie_c1_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqv5_p32(this + 4, dono as u32);
    cqv5_p32(this, 0x96c1c4);
    cqv5_p16(this + 0x40, ano as u16);
    cqv5_p8(this + 0xab, 0x0);
    cqv5_p8(this + 0x44, 0xff);
    cqv5_p32(this + 0x2c, 0x0);
    cqv5_p32(this + 0x30, 0xffffffff);
    cqv5_p8(this + 0x42, 0x1);
    cqv5_p8(this + 0x4a, 0x3);
    cqv5_p8(this + 0x49, 0x7);
    cqv5_p8(this + 0x50, 0x11);
    cqv5_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqv_5 supercopa_da_serie_c1 nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x63c9d0usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9ea1c8, 0x43);
        return this;
    }
    cqv5_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9ea1c8, 0x51);
        return this;
    }
    if !cqv5_estatisticas(this) {
        show_error_at(0x9ea1c8, 0x58);
        return this;
    }
    this
}

const CQV5_CAMPOS_SUPERCOPA_DA_SERIE_C1: [(usize, u8, u32); 9] = [(0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x11), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqv5_supercopa_da_serie_c1(this: usize, ano: u32, dono: usize) -> usize {
    cqv5_supercopa_da_serie_c1_corpo(this, ano, dono) // cqv5: supercopa_da_serie_c1
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqv5_plano).
unsafe extern "thiscall" fn comparar_cqv5_supercopa_da_serie_c1(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, ano, dono);
    let erro = cqv5_plano(this, r, ano, dono, 0x96c1c4, &CQV5_CAMPOS_SUPERCOPA_DA_SERIE_C1, true);
    PLACAR.record(erro.is_none(), || format!("cqv5_supercopa_da_serie_c1({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x90aeb0 } (thiscall, ret 8, usa_open_cup.cpp; da criação da carreira, n = 1): `UsaOpenCup::UsaOpenCup(ano, registro)`: o construtor de
/// a Copa dos EUA (US Open Cup)
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x970998, o registro em +4, o ano em +0x40 e
/// os da tabela CQV5_CAMPOS_COPA_DOS_EUA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os inscritos (UsaOpenCup::collect_entrants) (0x90b530, thiscall, ret 0; a caixa da linha 0x43 se devolver negativo); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x51 se devolver negativo); as estatísticas em +8 (a caixa da linha 0x58 sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); fundo de prêmios de 1.000.000; reservas e substituições da base; regra de disciplina 30 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqv5_copa_dos_eua_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqv5_p32(this + 4, dono as u32);
    cqv5_p32(this, 0x970998);
    cqv5_p16(this + 0x40, ano as u16);
    cqv5_p8(this + 0xab, 0x0);
    cqv5_p8(this + 0x44, 0xff);
    cqv5_p32(this + 0x2c, 0x0);
    cqv5_p32(this + 0x30, 0xffffffff);
    cqv5_p8(this + 0x42, 0x1);
    cqv5_p8(this + 0x50, 0x1e);
    cqv5_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqv_5 copa_dos_eua nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x90b530usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0xaa20e0, 0x43);
        return this;
    }
    cqv5_p32(this + 0x28, 0xf4240);
    cqv5_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0xaa20e0, 0x51);
        return this;
    }
    if !cqv5_estatisticas(this) {
        show_error_at(0xaa20e0, 0x58);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQV5_CAMPOS_COPA_DOS_EUA: [(usize, u8, u32); 8] = [(0x28, 4, 0xf4240), (0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x50, 1, 0x1e), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqv5_copa_dos_eua(this: usize, ano: u32, dono: usize) -> usize {
    cqv5_copa_dos_eua_corpo(this, ano, dono) // cqv5: copa_dos_eua
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqv5_plano).
unsafe extern "thiscall" fn comparar_cqv5_copa_dos_eua(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%)(this, ano, dono);
    let erro = cqv5_plano(this, r, ano, dono, 0x970998, &CQV5_CAMPOS_COPA_DOS_EUA, true);
    PLACAR.record(erro.is_none(), || format!("cqv5_copa_dos_eua({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}
