// ---------- %%LEVA%%a leva: os construtores das copas inglesas: a Copa da Inglaterra (FA Cup), a Copa da Liga e a Supercopa (Charity Shield) (eng_cc_cup.cpp, eng_charity.cpp, eng_fa_cup.cpp) ----------

unsafe fn cqu5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqu5_p8(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqu5_p16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqu5_p32(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// As rodadas da copa pelo método +0x3c da vtable (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall,
/// ret 0x10): a tabela das rodadas, gravada em +0xa3.
unsafe fn cqu5_rodadas(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqu5_d(cqu5_d(this) + 0x3c));
    let t = f(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    cqu5_p32(this + 0xa3, t as u32);
}

/// As estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// this; mov ecx, novo -> thiscall, ret 4) em +8; o quadro SEH da original cobre só esse new. Devolve se criou.
unsafe fn cqu5_estatisticas(this: usize) -> bool {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    cqu5_p32(this + 8, s as u32);
    s != 0
}

/// O plano que o comparador confere depois da original: a devolução (this), a vtable, o registro (+4) e o ano (+0x40);
/// na copa nova (+0x44 = 0xff), cada campo fixo que o construtor grava e as rodadas e as estatísticas criadas.
unsafe fn cqu5_plano(this: usize, r: usize, ano: u32, dono: usize, vt: usize, campos: &[(usize, u8, u32)], rodadas: bool) -> Option<String> {
    if r != this || cqu5_d(this) != vt || cqu5_d(this + 4) != dono || core::ptr::read_unaligned((this + 0x40) as *const u16) != ano as u16 {
        return Some(format!("devolveu {:#x}, vtable {:#x}, +4 {:#x}, ano {}", r, cqu5_d(this), cqu5_d(this + 4), core::ptr::read_unaligned((this + 0x40) as *const u16)));
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
    if (rodadas && cqu5_d(this + 0xa3) == 0) || cqu5_d(this + 8) == 0 {
        return Some(format!("rodadas {:#x}, estatísticas {:#x}", cqu5_d(this + 0xa3), cqu5_d(this + 8)));
    }
    None
}

/// { 0x56f040 } (thiscall, ret 8, eng_fa_cup.cpp; da criação da carreira, n = 1): `EngFaCup::EngFaCup(ano, registro)`: o construtor de
/// a Copa da Inglaterra (FA Cup)
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x969b28, o registro em +4, o ano em +0x40 e
/// os da tabela CQU5_CAMPOS_COPA_DA_INGLATERRA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x71 se devolver negativo); as estatísticas em +8 (a caixa da linha 0x79 sem elas); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); 152 times; fundo de prêmios de 2.000.000; reservas e substituições da base; regra de disciplina 9 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu5_copa_da_inglaterra_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu5_p32(this + 4, dono as u32);
    cqu5_p32(this, 0x969b28);
    cqu5_p16(this + 0x40, ano as u16);
    cqu5_p8(this + 0xab, 0x0);
    cqu5_p8(this + 0x44, 0xff);
    cqu5_p32(this + 0x2c, 0x0);
    cqu5_p32(this + 0x30, 0xffffffff);
    cqu5_p8(this + 0x42, 0x1);
    cqu5_p8(this + 0x50, 0x9);
    cqu5_p8(this + 0xb1, 0x0);
    cqu5_p32(this + 0x45, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqu_5 copa_da_inglaterra nova");
    cqu5_p8(this + 0xb2, 0x0);
    cqu5_p8(this + 0xb4, 0x1);
    cqu5_p8(this + 0xb3, 0x0);
    cqu5_p16(this + 0x3e, 0x98);
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let v = zerada(0x98, 6);
    cqu5_p32(this + 0xa7, v as u32);
    if v == 0 {
        show_error_at(0x9c42cc, 0x63);
        return this;
    }
    cqu5_p32(this + 0x28, 0x1e8480);
    cqu5_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9c42cc, 0x71);
        return this;
    }
    if !cqu5_estatisticas(this) {
        show_error_at(0x9c42cc, 0x79);
        return this;
    }
    this
}

const CQU5_CAMPOS_COPA_DA_INGLATERRA: [(usize, u8, u32); 13] = [(0x28, 4, 0x1e8480), (0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x3e, 2, 0x98), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x45, 4, 0x0), (0x50, 1, 0x9), (0xab, 1, 0x0), (0xb1, 1, 0x0), (0xb2, 1, 0x0), (0xb3, 1, 0x0), (0xb4, 1, 0x1)];

unsafe extern "thiscall" fn cqu5_copa_da_inglaterra(this: usize, ano: u32, dono: usize) -> usize {
    cqu5_copa_da_inglaterra_corpo(this, ano, dono) // cqu5: copa_da_inglaterra
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu5_plano).
unsafe extern "thiscall" fn compare_cqu5_copa_da_inglaterra(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, ano, dono);
    let erro = cqu5_plano(this, r, ano, dono, 0x969b28, &CQU5_CAMPOS_COPA_DA_INGLATERRA, true);
    SCOREBOARD.record(erro.is_none(), || format!("cqu5_copa_da_inglaterra({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x56c2a0 } (thiscall, ret 8, eng_cc_cup.cpp; da criação da carreira, n = 1): `EngCcCup::EngCcCup(ano, registro)`: o construtor de
/// a Copa da Liga inglesa
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x969934, o registro em +4, o ano em +0x40 e
/// os da tabela CQU5_CAMPOS_COPA_DA_LIGA_INGLESA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x5e se devolver negativo); as estatísticas em +8 (a caixa da linha 0x65 sem elas); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); 92 times; fundo de prêmios de 1.000.000; reservas e substituições da base; regra de disciplina 9 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu5_copa_da_liga_inglesa_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu5_p32(this + 4, dono as u32);
    cqu5_p32(this, 0x969934);
    cqu5_p16(this + 0x40, ano as u16);
    cqu5_p8(this + 0xab, 0x0);
    cqu5_p8(this + 0x44, 0xff);
    cqu5_p32(this + 0x2c, 0x0);
    cqu5_p32(this + 0x30, 0xffffffff);
    cqu5_p8(this + 0x42, 0x1);
    cqu5_p8(this + 0x50, 0x9);
    cqu5_p8(this + 0xb1, 0x0);
    cqu5_p32(this + 0x45, 0x0);
    cqu5_p8(this + 0xb2, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqu_5 copa_da_liga_inglesa nova");
    cqu5_p16(this + 0x3e, 0x5c);
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let v = zerada(0x5c, 6);
    cqu5_p32(this + 0xa7, v as u32);
    if v == 0 {
        show_error_at(0x9c4224, 0x50);
        return this;
    }
    cqu5_p32(this + 0x28, 0xf4240);
    cqu5_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9c4224, 0x5e);
        return this;
    }
    if !cqu5_estatisticas(this) {
        show_error_at(0x9c4224, 0x65);
        return this;
    }
    this
}

const CQU5_CAMPOS_COPA_DA_LIGA_INGLESA: [(usize, u8, u32); 11] = [(0x28, 4, 0xf4240), (0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x3e, 2, 0x5c), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x45, 4, 0x0), (0x50, 1, 0x9), (0xab, 1, 0x0), (0xb1, 1, 0x0), (0xb2, 1, 0x0)];

unsafe extern "thiscall" fn cqu5_copa_da_liga_inglesa(this: usize, ano: u32, dono: usize) -> usize {
    cqu5_copa_da_liga_inglesa_corpo(this, ano, dono) // cqu5: copa_da_liga_inglesa
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu5_plano).
unsafe extern "thiscall" fn compare_cqu5_copa_da_liga_inglesa(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, ano, dono);
    let erro = cqu5_plano(this, r, ano, dono, 0x969934, &CQU5_CAMPOS_COPA_DA_LIGA_INGLESA, true);
    SCOREBOARD.record(erro.is_none(), || format!("cqu5_copa_da_liga_inglesa({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x56d380 } (thiscall, ret 8, eng_charity.cpp; da criação da carreira, n = 1): `EngCharity::EngCharity(ano, registro)`: o construtor de
/// a Supercopa inglesa (Charity Shield)
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x9699d4, o registro em +4, o ano em +0x40 e
/// os da tabela CQU5_CAMPOS_SUPERCOPA_INGLESA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os dois clubes (EngCharity::select_teams) (0x56d780, thiscall, ret 0; a caixa da linha 0x43 se devolver negativo); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x51 se devolver negativo); as estatísticas em +8 (a caixa da linha 0x58 sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); fundo de prêmios de 300.000; 7 reservas no banco e 5 substituições; regra de disciplina 9 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu5_supercopa_inglesa_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu5_p32(this + 4, dono as u32);
    cqu5_p32(this, 0x9699d4);
    cqu5_p16(this + 0x40, ano as u16);
    cqu5_p8(this + 0xab, 0x0);
    cqu5_p8(this + 0x44, 0xff);
    cqu5_p32(this + 0x2c, 0x0);
    cqu5_p32(this + 0x30, 0xffffffff);
    cqu5_p8(this + 0x42, 0x1);
    cqu5_p8(this + 0x49, 0x7);
    cqu5_p8(this + 0x4a, 0x5);
    cqu5_p8(this + 0x50, 0x9);
    cqu5_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqu_5 supercopa_inglesa nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56d780usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9c425c, 0x43);
        return this;
    }
    cqu5_p32(this + 0x28, 0x493e0);
    cqu5_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9c425c, 0x51);
        return this;
    }
    if !cqu5_estatisticas(this) {
        show_error_at(0x9c425c, 0x58);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQU5_CAMPOS_SUPERCOPA_INGLESA: [(usize, u8, u32); 10] = [(0x28, 4, 0x493e0), (0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x5), (0x50, 1, 0x9), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqu5_supercopa_inglesa(this: usize, ano: u32, dono: usize) -> usize {
    cqu5_supercopa_inglesa_corpo(this, ano, dono) // cqu5: supercopa_inglesa
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu5_plano).
unsafe extern "thiscall" fn compare_cqu5_supercopa_inglesa(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%)(this, ano, dono);
    let erro = cqu5_plano(this, r, ano, dono, 0x9699d4, &CQU5_CAMPOS_SUPERCOPA_INGLESA, true);
    SCOREBOARD.record(erro.is_none(), || format!("cqu5_supercopa_inglesa({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}
