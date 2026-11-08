// ---------- %%LEVA%%a leva: os construtores da Supercopa Asiática, da Copa Interamericana, da Copa Mercosul e do Jogo das Estrelas da MLS (asia_super_cup.cpp, con_merc_cup.cpp, inter_amer_cup.cpp, usa_mls_all_stars.cpp) ----------

unsafe fn cqw5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqw5_p8(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqw5_p16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqw5_p32(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// As rodadas da copa pelo método +0x3c da vtable (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall,
/// ret 0x10): a tabela das rodadas, gravada em +0xa3.
unsafe fn cqw5_rodadas(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqw5_d(cqw5_d(this) + 0x3c));
    let t = f(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    cqw5_p32(this + 0xa3, t as u32);
}

/// As estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// this; mov ecx, novo -> thiscall, ret 4) em +8; o quadro SEH da original cobre só esse new. Devolve se criou.
unsafe fn cqw5_estatisticas(this: usize) -> bool {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    cqw5_p32(this + 8, s as u32);
    s != 0
}

/// O plano que o comparador confere depois da original: a devolução (this), a vtable, o registro (+4) e o ano (+0x40);
/// na copa nova (+0x44 = 0xff), cada campo fixo que o construtor grava e as rodadas e as estatísticas criadas.
unsafe fn cqw5_plano(this: usize, r: usize, ano: u32, dono: usize, vt: usize, campos: &[(usize, u8, u32)], rodadas: bool) -> Option<String> {
    if r != this || cqw5_d(this) != vt || cqw5_d(this + 4) != dono || core::ptr::read_unaligned((this + 0x40) as *const u16) != ano as u16 {
        return Some(format!("devolveu {:#x}, vtable {:#x}, +4 {:#x}, ano {}", r, cqw5_d(this), cqw5_d(this + 4), core::ptr::read_unaligned((this + 0x40) as *const u16)));
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
    if (rodadas && cqw5_d(this + 0xa3) == 0) || cqw5_d(this + 8) == 0 {
        return Some(format!("rodadas {:#x}, estatísticas {:#x}", cqw5_d(this + 0xa3), cqw5_d(this + 8)));
    }
    None
}

/// { 0x410650 } (thiscall, ret 8, asia_super_cup.cpp; da criação da carreira, n = 1): `AsiaSuperCup::AsiaSuperCup(ano, registro)`: o construtor de
/// a Supercopa Asiática
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x9676c8, o registro em +4, o ano em +0x40 e
/// os da tabela CQW5_CAMPOS_SUPERCOPA_ASIATICA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os dois clubes (AsiaSuperCup::select_teams) (0x410a50, thiscall, ret 0; a caixa da linha 0x3e se devolver negativo); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x4c se devolver negativo); as estatísticas em +8 (a caixa da linha 0x53 sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 2 (clubes de várias nações); sem fundo de prêmios (+0x28 fica como a base deixou); 7 reservas no banco e 3 substituições; regra de disciplina 41 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqw5_supercopa_asiatica_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqw5_p32(this + 4, dono as u32);
    cqw5_p32(this, 0x9676c8);
    cqw5_p16(this + 0x40, ano as u16);
    cqw5_p8(this + 0xab, 0x0);
    cqw5_p8(this + 0x44, 0xff);
    cqw5_p32(this + 0x2c, 0x0);
    cqw5_p32(this + 0x30, 0xffffffff);
    cqw5_p8(this + 0x42, 0x2);
    cqw5_p8(this + 0x49, 0x7);
    cqw5_p8(this + 0x4a, 0x3);
    cqw5_p8(this + 0x50, 0x29);
    cqw5_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqw_5 supercopa_asiatica nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x410a50usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x987be0, 0x3e);
        return this;
    }
    cqw5_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x987be0, 0x4c);
        return this;
    }
    if !cqw5_estatisticas(this) {
        show_error_at(0x987be0, 0x53);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQW5_CAMPOS_SUPERCOPA_ASIATICA: [(usize, u8, u32); 9] = [(0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x2), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x29), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqw5_supercopa_asiatica(this: usize, ano: u32, dono: usize) -> usize {
    cqw5_supercopa_asiatica_corpo(this, ano, dono) // cqw5: supercopa_asiatica
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqw5_plano).
unsafe extern "thiscall" fn comparar_cqw5_supercopa_asiatica(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, ano, dono);
    let erro = cqw5_plano(this, r, ano, dono, 0x9676c8, &CQW5_CAMPOS_SUPERCOPA_ASIATICA, true);
    PLACAR.record(erro.is_none(), || format!("cqw5_supercopa_asiatica({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x632080 } (thiscall, ret 8, inter_amer_cup.cpp; da criação da carreira, n = 1): `InterAmerCup::InterAmerCup(ano, registro)`: o construtor de
/// a Copa Interamericana
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96bad8, o registro em +4, o ano em +0x40 e
/// os da tabela CQW5_CAMPOS_COPA_INTERAMERICANA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os dois finalistas (InterAmerCup::select_teams) (0x632480, thiscall, ret 0; a caixa da linha 0x41 se devolver negativo); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x4f se devolver negativo); as estatísticas em +8 (a caixa da linha 0x56 sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 2 (clubes de várias nações); sem fundo de prêmios (+0x28 fica como a base deixou); 7 reservas no banco e 3 substituições; regra de disciplina 255 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqw5_copa_interamericana_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqw5_p16(this + 0x40, ano as u16);
    cqw5_p32(this + 4, dono as u32);
    cqw5_p32(this, 0x96bad8);
    cqw5_p8(this + 0xab, 0x0);
    cqw5_p8(this + 0x44, 0xff);
    cqw5_p32(this + 0x2c, 0x0);
    cqw5_p32(this + 0x30, 0xffffffff);
    cqw5_p8(this + 0x42, 0x2);
    cqw5_p8(this + 0x52, 0x2);
    cqw5_p8(this + 0x49, 0x7);
    cqw5_p8(this + 0x4a, 0x3);
    cqw5_p8(this + 0x50, 0xff);
    cqw5_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqw_5 copa_interamericana nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x632480usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9e9d94, 0x41);
        return this;
    }
    cqw5_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9e9d94, 0x4f);
        return this;
    }
    if !cqw5_estatisticas(this) {
        show_error_at(0x9e9d94, 0x56);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQW5_CAMPOS_COPA_INTERAMERICANA: [(usize, u8, u32); 10] = [(0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x2), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0xff), (0x52, 1, 0x2), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqw5_copa_interamericana(this: usize, ano: u32, dono: usize) -> usize {
    cqw5_copa_interamericana_corpo(this, ano, dono) // cqw5: copa_interamericana
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqw5_plano).
unsafe extern "thiscall" fn comparar_cqw5_copa_interamericana(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, ano, dono);
    let erro = cqw5_plano(this, r, ano, dono, 0x96bad8, &CQW5_CAMPOS_COPA_INTERAMERICANA, true);
    PLACAR.record(erro.is_none(), || format!("cqw5_copa_interamericana({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x4c16f0 } (thiscall, ret 8, con_merc_cup.cpp; da criação da carreira, n = 1): `ConMercCup::ConMercCup(ano, registro)`: o construtor de
/// a Copa Mercosul
/// sobre a base League::League (0x682200, thiscall, ret 0); os campos fixos (a vtable 0x968b40, o registro em +4, o ano em +0x40 e
/// os da tabela CQW5_CAMPOS_COPA_MERCOSUL); se o estado salvo carrega (League::load_from_temp_file 0x687b10: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: o vetor das fases em +0xc (malloc 0x944e46 de 0x10: push; add esp, 4 -> cdecl, sem conferir); as regras (ConMercCup::set_up_rules) (0x4c1ed0, thiscall, ret 0; a caixa da linha 0x4d se devolver 0); as estatísticas em +8 (a caixa da linha 0x59 sem elas); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: sem fundo de prêmios (+0x28 fica como a base deixou); reservas e substituições da base; regra de disciplina 255 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqw5_copa_mercosul_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    base(this);
    cqw5_p32(this + 4, dono as u32);
    cqw5_p32(this, 0x968b40);
    cqw5_p16(this + 0x40, ano as u16);
    cqw5_p8(this + 0x50, 0xff);
    cqw5_p32(this + 0xee, 0x0);
    cqw5_p8(this + 0x51, 0xf);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqw_5 copa_mercosul nova");
    cqw5_p16(this + 0xd9, 0x28);
    cqw5_p8(this + 0x44, 0xff);
    cqw5_p32(this + 0x30, 0xffffffff);
    cqw5_p32(this + 0x2c, 0x4);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    cqw5_p32(this + 0xc, alocar(0x10) as u32);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x4c1ed0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9a7d6c, 0x4d);
        return this;
    }
    cqw5_p16(this + 0x3e, 0x0);
    cqw5_p32(this + 0xb1, 0x0);
    if !cqw5_estatisticas(this) {
        show_error_at(0x9a7d6c, 0x59);
        return this;
    }
    cqw5_p32(this + 0x45, 0x0);
    this
}

const CQW5_CAMPOS_COPA_MERCOSUL: [(usize, u8, u32); 10] = [(0x2c, 4, 0x4), (0x30, 4, 0xffffffff), (0x3e, 2, 0x0), (0x44, 1, 0xff), (0x45, 4, 0x0), (0x50, 1, 0xff), (0x51, 1, 0xf), (0xb1, 4, 0x0), (0xd9, 2, 0x28), (0xee, 4, 0x0)];

unsafe extern "thiscall" fn cqw5_copa_mercosul(this: usize, ano: u32, dono: usize) -> usize {
    cqw5_copa_mercosul_corpo(this, ano, dono) // cqw5: copa_mercosul
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqw5_plano).
unsafe extern "thiscall" fn comparar_cqw5_copa_mercosul(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%)(this, ano, dono);
    let erro = cqw5_plano(this, r, ano, dono, 0x968b40, &CQW5_CAMPOS_COPA_MERCOSUL, false);
    PLACAR.record(erro.is_none(), || format!("cqw5_copa_mercosul({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x90a150 } (thiscall, ret 8, usa_mls_all_stars.cpp; da criação da carreira, n = 1): `UsaMlsAllStars::UsaMlsAllStars(ano, registro)`: o construtor de
/// o Jogo das Estrelas da MLS
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x9708f8, o registro em +4, o ano em +0x40 e
/// os da tabela CQW5_CAMPOS_JOGO_DAS_ESTRELAS); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os dois times das estrelas (UsaMlsAllStars::set_up_teams) (0x90a540, thiscall, ret 0; a caixa da linha 0x44 se devolver negativo); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x52 se devolver negativo); as estatísticas em +8 (a caixa da linha 0x59 sem elas); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); sem fundo de prêmios (+0x28 fica como a base deixou); reservas e substituições da base; regra de disciplina 255 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqw5_jogo_das_estrelas_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqw5_p32(this + 4, dono as u32);
    cqw5_p32(this, 0x9708f8);
    cqw5_p16(this + 0x40, ano as u16);
    cqw5_p8(this + 0xab, 0x0);
    cqw5_p8(this + 0x44, 0xff);
    cqw5_p32(this + 0x2c, 0x0);
    cqw5_p32(this + 0x30, 0xffffffff);
    cqw5_p8(this + 0x42, 0x1);
    cqw5_p8(this + 0x50, 0xff);
    cqw5_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqw_5 jogo_das_estrelas nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x90a540usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0xaa20a0, 0x44);
        return this;
    }
    cqw5_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0xaa20a0, 0x52);
        return this;
    }
    if !cqw5_estatisticas(this) {
        show_error_at(0xaa20a0, 0x59);
        return this;
    }
    this
}

const CQW5_CAMPOS_JOGO_DAS_ESTRELAS: [(usize, u8, u32); 7] = [(0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x50, 1, 0xff), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqw5_jogo_das_estrelas(this: usize, ano: u32, dono: usize) -> usize {
    cqw5_jogo_das_estrelas_corpo(this, ano, dono) // cqw5: jogo_das_estrelas
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqw5_plano).
unsafe extern "thiscall" fn comparar_cqw5_jogo_das_estrelas(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I3%%)(this, ano, dono);
    let erro = cqw5_plano(this, r, ano, dono, 0x9708f8, &CQW5_CAMPOS_JOGO_DAS_ESTRELAS, true);
    PLACAR.record(erro.is_none(), || format!("cqw5_jogo_das_estrelas({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}
