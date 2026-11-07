// ---------- %%LEVA%%a leva: os construtores das copas francesas: a Copa da França, a Copa da Liga e a Supercopa (fra_cup.cpp, fra_lge_cup.cpp, fra_super.cpp) ----------

unsafe fn cqu7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqu7_p8(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqu7_p16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqu7_p32(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// As rodadas da copa pelo método +0x3c da vtable (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall,
/// ret 0x10): a tabela das rodadas, gravada em +0xa3.
unsafe fn cqu7_rodadas(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqu7_d(cqu7_d(this) + 0x3c));
    let t = f(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    cqu7_p32(this + 0xa3, t as u32);
}

/// As estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// this; mov ecx, novo -> thiscall, ret 4) em +8; o quadro SEH da original cobre só esse new. Devolve se criou.
unsafe fn cqu7_estatisticas(this: usize) -> bool {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    cqu7_p32(this + 8, s as u32);
    s != 0
}

/// O plano que o comparador confere depois da original: a devolução (this), a vtable, o registro (+4) e o ano (+0x40);
/// na copa nova (+0x44 = 0xff), cada campo fixo que o construtor grava e as rodadas e as estatísticas criadas.
unsafe fn cqu7_plano(this: usize, r: usize, ano: u32, dono: usize, vt: usize, campos: &[(usize, u8, u32)], rodadas: bool) -> Option<String> {
    if r != this || cqu7_d(this) != vt || cqu7_d(this + 4) != dono || core::ptr::read_unaligned((this + 0x40) as *const u16) != ano as u16 {
        return Some(format!("devolveu {:#x}, vtable {:#x}, +4 {:#x}, ano {}", r, cqu7_d(this), cqu7_d(this + 4), core::ptr::read_unaligned((this + 0x40) as *const u16)));
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
    if (rodadas && cqu7_d(this + 0xa3) == 0) || cqu7_d(this + 8) == 0 {
        return Some(format!("rodadas {:#x}, estatísticas {:#x}", cqu7_d(this + 0xa3), cqu7_d(this + 8)));
    }
    None
}

/// { 0x5ba130 } (thiscall, ret 8, fra_cup.cpp; da criação da carreira, n = 1): `FraCup::FraCup(ano, registro)`: o construtor de
/// a Copa da França
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96a864, o registro em +4, o ano em +0x40 e
/// os da tabela CQU7_CAMPOS_COPA_DA_FRANCA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os 222 inscritos (FraCup::collect_entrants) (0x5baa50, thiscall, ret 0; a caixa da linha 0x4c se devolver 0); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x5b se devolver 0); as estatísticas em +8 (a caixa da linha 0x62 sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (aviso_em) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); fundo de prêmios de 1.000.000; 3 reservas no banco e 3 substituições; regra de disciplina 12 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu7_copa_da_franca_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu7_p16(this + 0x40, ano as u16);
    cqu7_p32(this + 4, dono as u32);
    cqu7_p32(this, 0x96a864);
    cqu7_p8(this + 0xab, 0x0);
    cqu7_p8(this + 0x44, 0xff);
    cqu7_p32(this + 0x30, 0xffffffff);
    cqu7_p32(this + 0x2c, 0x0);
    cqu7_p8(this + 0x42, 0x1);
    cqu7_p8(this + 0x49, 0x3);
    cqu7_p8(this + 0x4a, 0x3);
    cqu7_p8(this + 0x50, 0xc);
    cqu7_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqu_7 copa_da_franca nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5baa50usize);
    let r = f(this);
    if r == 0 {
        aviso_em(0x9ccb28, 0x4c);
        return this;
    }
    cqu7_p32(this + 0x28, 0xf4240);
    cqu7_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        aviso_em(0x9ccb28, 0x5b);
        return this;
    }
    if !cqu7_estatisticas(this) {
        aviso_em(0x9ccb28, 0x62);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQU7_CAMPOS_COPA_DA_FRANCA: [(usize, u8, u32); 10] = [(0x28, 4, 0xf4240), (0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x49, 1, 0x3), (0x4a, 1, 0x3), (0x50, 1, 0xc), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqu7_copa_da_franca(this: usize, ano: u32, dono: usize) -> usize {
    cqu7_copa_da_franca_corpo(this, ano, dono) // cqu7: copa_da_franca
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu7_plano).
unsafe extern "thiscall" fn comparar_cqu7_copa_da_franca(this: usize, ano: u32, dono: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, ano, dono);
    let erro = cqu7_plano(this, r, ano, dono, 0x96a864, &CQU7_CAMPOS_COPA_DA_FRANCA, true);
    PLACAR.registrar(erro.is_none(), || format!("cqu7_copa_da_franca({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x5bc980 } (thiscall, ret 8, fra_lge_cup.cpp; da criação da carreira, n = 1): `FraLgeCup::FraLgeCup(ano, registro)`: o construtor de
/// a Copa da Liga francesa
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96aad8, o registro em +4, o ano em +0x40 e
/// os da tabela CQU7_CAMPOS_COPA_DA_LIGA_FRANCESA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os 54 inscritos (FraLgeCup::collect_entrants) (0x5bd160, thiscall, ret 0; a caixa da linha 0x49 se devolver 0); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x57 se devolver 0); as estatísticas em +8 (a caixa da linha 0x5e sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (aviso_em) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); fundo de prêmios de 500.000; 3 reservas no banco e 3 substituições; regra de disciplina 12 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu7_copa_da_liga_francesa_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu7_p16(this + 0x40, ano as u16);
    cqu7_p32(this + 4, dono as u32);
    cqu7_p32(this, 0x96aad8);
    cqu7_p8(this + 0xab, 0x0);
    cqu7_p8(this + 0x44, 0xff);
    cqu7_p32(this + 0x30, 0xffffffff);
    cqu7_p32(this + 0x2c, 0x0);
    cqu7_p8(this + 0x42, 0x1);
    cqu7_p8(this + 0x49, 0x3);
    cqu7_p8(this + 0x4a, 0x3);
    cqu7_p8(this + 0x50, 0xc);
    cqu7_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqu_7 copa_da_liga_francesa nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5bd160usize);
    let r = f(this);
    if r == 0 {
        aviso_em(0x9ccb98, 0x49);
        return this;
    }
    cqu7_p32(this + 0x28, 0x7a120);
    cqu7_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        aviso_em(0x9ccb98, 0x57);
        return this;
    }
    if !cqu7_estatisticas(this) {
        aviso_em(0x9ccb98, 0x5e);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQU7_CAMPOS_COPA_DA_LIGA_FRANCESA: [(usize, u8, u32); 10] = [(0x28, 4, 0x7a120), (0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x49, 1, 0x3), (0x4a, 1, 0x3), (0x50, 1, 0xc), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqu7_copa_da_liga_francesa(this: usize, ano: u32, dono: usize) -> usize {
    cqu7_copa_da_liga_francesa_corpo(this, ano, dono) // cqu7: copa_da_liga_francesa
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu7_plano).
unsafe extern "thiscall" fn comparar_cqu7_copa_da_liga_francesa(this: usize, ano: u32, dono: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, ano, dono);
    let erro = cqu7_plano(this, r, ano, dono, 0x96aad8, &CQU7_CAMPOS_COPA_DA_LIGA_FRANCESA, true);
    PLACAR.registrar(erro.is_none(), || format!("cqu7_copa_da_liga_francesa({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x5bf320 } (thiscall, ret 8, fra_super.cpp; da criação da carreira, n = 1): `FraSuper::FraSuper(ano, registro)`: o construtor de
/// a Supercopa francesa
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96ace0, o registro em +4, o ano em +0x40 e
/// os da tabela CQU7_CAMPOS_SUPERCOPA_FRANCESA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os dois clubes (FraSuper::select_teams) (0x5bf770, thiscall, ret 0; a caixa da linha 0x3f se devolver negativo); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x4d se devolver negativo); as estatísticas em +8 (a caixa da linha 0x54 sem elas); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (aviso_em) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); sem fundo de prêmios (+0x28 fica como a base deixou); reservas e substituições da base; regra de disciplina 12 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu7_supercopa_francesa_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu7_p32(this + 4, dono as u32);
    cqu7_p32(this, 0x96ace0);
    cqu7_p16(this + 0x40, ano as u16);
    cqu7_p8(this + 0xab, 0x0);
    cqu7_p8(this + 0x44, 0xff);
    cqu7_p32(this + 0x2c, 0x0);
    cqu7_p32(this + 0x30, 0xffffffff);
    cqu7_p8(this + 0x42, 0x1);
    cqu7_p8(this + 0x50, 0xc);
    cqu7_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqu_7 supercopa_francesa nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5bf770usize);
    let r = f(this);
    if r < 0 {
        aviso_em(0x9ccc48, 0x3f);
        return this;
    }
    cqu7_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        aviso_em(0x9ccc48, 0x4d);
        return this;
    }
    if !cqu7_estatisticas(this) {
        aviso_em(0x9ccc48, 0x54);
        return this;
    }
    this
}

const CQU7_CAMPOS_SUPERCOPA_FRANCESA: [(usize, u8, u32); 7] = [(0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x50, 1, 0xc), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqu7_supercopa_francesa(this: usize, ano: u32, dono: usize) -> usize {
    cqu7_supercopa_francesa_corpo(this, ano, dono) // cqu7: supercopa_francesa
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu7_plano).
unsafe extern "thiscall" fn comparar_cqu7_supercopa_francesa(this: usize, ano: u32, dono: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%)(this, ano, dono);
    let erro = cqu7_plano(this, r, ano, dono, 0x96ace0, &CQU7_CAMPOS_SUPERCOPA_FRANCESA, true);
    PLACAR.registrar(erro.is_none(), || format!("cqu7_supercopa_francesa({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}
