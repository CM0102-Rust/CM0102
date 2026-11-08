// ---------- %%LEVA%%a leva: os construtores das copas japonesas: a Copa do Imperador, a Copa da Liga (J League Cup) e a Supercopa (jap_emp_cup.cpp, jap_j_cup.cpp, jap_super.cpp) ----------

unsafe fn cqu8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqu8_p8(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqu8_p16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqu8_p32(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// As rodadas da copa pelo método +0x3c da vtable (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall,
/// ret 0x10): a tabela das rodadas, gravada em +0xa3.
unsafe fn cqu8_rodadas(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqu8_d(cqu8_d(this) + 0x3c));
    let t = f(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    cqu8_p32(this + 0xa3, t as u32);
}

/// As estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// this; mov ecx, novo -> thiscall, ret 4) em +8; o quadro SEH da original cobre só esse new. Devolve se criou.
unsafe fn cqu8_estatisticas(this: usize) -> bool {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    cqu8_p32(this + 8, s as u32);
    s != 0
}

/// O plano que o comparador confere depois da original: a devolução (this), a vtable, o registro (+4) e o ano (+0x40);
/// na copa nova (+0x44 = 0xff), cada campo fixo que o construtor grava e as rodadas e as estatísticas criadas.
unsafe fn cqu8_plano(this: usize, r: usize, ano: u32, dono: usize, vt: usize, campos: &[(usize, u8, u32)], rodadas: bool) -> Option<String> {
    if r != this || cqu8_d(this) != vt || cqu8_d(this + 4) != dono || core::ptr::read_unaligned((this + 0x40) as *const u16) != ano as u16 {
        return Some(format!("devolveu {:#x}, vtable {:#x}, +4 {:#x}, ano {}", r, cqu8_d(this), cqu8_d(this + 4), core::ptr::read_unaligned((this + 0x40) as *const u16)));
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
    if (rodadas && cqu8_d(this + 0xa3) == 0) || cqu8_d(this + 8) == 0 {
        return Some(format!("rodadas {:#x}, estatísticas {:#x}", cqu8_d(this + 0xa3), cqu8_d(this + 8)));
    }
    None
}

/// { 0x661d30 } (thiscall, ret 8, jap_emp_cup.cpp; da criação da carreira, n = 1): `JapEmpCup::JapEmpCup(ano, registro)`: o construtor de
/// a Copa do Imperador
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96c9a0, o registro em +4, o ano em +0x40 e
/// os da tabela CQU8_CAMPOS_COPA_DO_IMPERADOR); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os inscritos (JapEmpCup::collect_entrants) (0x6624f0, thiscall, ret 0; a caixa da linha 0x47 se devolver 0); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x55 se devolver 0); as estatísticas em +8 (a caixa da linha 0x5c sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); fundo de prêmios de 7.000.000; 5 reservas no banco e 3 substituições; regra de disciplina 20 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu8_copa_do_imperador_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu8_p32(this + 4, dono as u32);
    cqu8_p32(this, 0x96c9a0);
    cqu8_p16(this + 0x40, ano as u16);
    cqu8_p8(this + 0xab, 0x0);
    cqu8_p8(this + 0x44, 0xff);
    cqu8_p32(this + 0x30, 0xffffffff);
    cqu8_p32(this + 0x2c, 0x0);
    cqu8_p8(this + 0x42, 0x1);
    cqu8_p8(this + 0x49, 0x5);
    cqu8_p8(this + 0x4a, 0x3);
    cqu8_p8(this + 0x50, 0x14);
    cqu8_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqu_8 copa_do_imperador nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x6624f0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9eab90, 0x47);
        return this;
    }
    cqu8_p32(this + 0x28, 0x6acfc0);
    cqu8_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9eab90, 0x55);
        return this;
    }
    if !cqu8_estatisticas(this) {
        show_error_at(0x9eab90, 0x5c);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQU8_CAMPOS_COPA_DO_IMPERADOR: [(usize, u8, u32); 10] = [(0x28, 4, 0x6acfc0), (0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x49, 1, 0x5), (0x4a, 1, 0x3), (0x50, 1, 0x14), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqu8_copa_do_imperador(this: usize, ano: u32, dono: usize) -> usize {
    cqu8_copa_do_imperador_corpo(this, ano, dono) // cqu8: copa_do_imperador
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu8_plano).
unsafe extern "thiscall" fn comparar_cqu8_copa_do_imperador(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, ano, dono);
    let erro = cqu8_plano(this, r, ano, dono, 0x96c9a0, &CQU8_CAMPOS_COPA_DO_IMPERADOR, true);
    PLACAR.record(erro.is_none(), || format!("cqu8_copa_do_imperador({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x665380 } (thiscall, ret 8, jap_j_cup.cpp; da criação da carreira, n = 1): `JapJCup::JapJCup(ano, registro)`: o construtor de
/// a Copa da Liga japonesa
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96cba8, o registro em +4, o ano em +0x40 e
/// os da tabela CQU8_CAMPOS_COPA_DA_LIGA_JAPONESA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os 28 inscritos (JapJCup::collect_entrants) (0x665a00, thiscall, ret 0; a caixa da linha 0x45 se devolver 0); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x53 se devolver 0); as estatísticas em +8 (a caixa da linha 0x5a sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); fundo de prêmios de 2.000.000; 5 reservas no banco e 3 substituições; regra de disciplina 20 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu8_copa_da_liga_japonesa_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu8_p32(this + 4, dono as u32);
    cqu8_p32(this, 0x96cba8);
    cqu8_p16(this + 0x40, ano as u16);
    cqu8_p8(this + 0xab, 0x0);
    cqu8_p8(this + 0x44, 0xff);
    cqu8_p32(this + 0x30, 0xffffffff);
    cqu8_p32(this + 0x2c, 0x0);
    cqu8_p8(this + 0x42, 0x1);
    cqu8_p8(this + 0x49, 0x5);
    cqu8_p8(this + 0x4a, 0x3);
    cqu8_p8(this + 0x50, 0x14);
    cqu8_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqu_8 copa_da_liga_japonesa nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x665a00usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9eaf14, 0x45);
        return this;
    }
    cqu8_p32(this + 0x28, 0x1e8480);
    cqu8_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9eaf14, 0x53);
        return this;
    }
    if !cqu8_estatisticas(this) {
        show_error_at(0x9eaf14, 0x5a);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQU8_CAMPOS_COPA_DA_LIGA_JAPONESA: [(usize, u8, u32); 10] = [(0x28, 4, 0x1e8480), (0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x49, 1, 0x5), (0x4a, 1, 0x3), (0x50, 1, 0x14), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqu8_copa_da_liga_japonesa(this: usize, ano: u32, dono: usize) -> usize {
    cqu8_copa_da_liga_japonesa_corpo(this, ano, dono) // cqu8: copa_da_liga_japonesa
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu8_plano).
unsafe extern "thiscall" fn comparar_cqu8_copa_da_liga_japonesa(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, ano, dono);
    let erro = cqu8_plano(this, r, ano, dono, 0x96cba8, &CQU8_CAMPOS_COPA_DA_LIGA_JAPONESA, true);
    PLACAR.record(erro.is_none(), || format!("cqu8_copa_da_liga_japonesa({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x665d70 } (thiscall, ret 8, jap_super.cpp; da criação da carreira, n = 1): `JapSuper::JapSuper(ano, registro)`: o construtor de
/// a Supercopa japonesa
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96cc48, o registro em +4, o ano em +0x40 e
/// os da tabela CQU8_CAMPOS_SUPERCOPA_JAPONESA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os dois clubes (JapSuper::select_teams) (0x666160, thiscall, ret 0; a caixa da linha 0x3f se devolver negativo); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x4d se devolver negativo); as estatísticas em +8 (a caixa da linha 0x54 sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); sem fundo de prêmios (+0x28 fica como a base deixou); reservas e substituições da base; regra de disciplina 20 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu8_supercopa_japonesa_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu8_p32(this + 4, dono as u32);
    cqu8_p32(this, 0x96cc48);
    cqu8_p16(this + 0x40, ano as u16);
    cqu8_p8(this + 0xab, 0x0);
    cqu8_p8(this + 0x44, 0xff);
    cqu8_p32(this + 0x2c, 0x0);
    cqu8_p32(this + 0x30, 0xffffffff);
    cqu8_p8(this + 0x42, 0x1);
    cqu8_p8(this + 0x50, 0x14);
    cqu8_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqu_8 supercopa_japonesa nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x666160usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9eaf4c, 0x3f);
        return this;
    }
    cqu8_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0x9eaf4c, 0x4d);
        return this;
    }
    if !cqu8_estatisticas(this) {
        show_error_at(0x9eaf4c, 0x54);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQU8_CAMPOS_SUPERCOPA_JAPONESA: [(usize, u8, u32); 7] = [(0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x50, 1, 0x14), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqu8_supercopa_japonesa(this: usize, ano: u32, dono: usize) -> usize {
    cqu8_supercopa_japonesa_corpo(this, ano, dono) // cqu8: supercopa_japonesa
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu8_plano).
unsafe extern "thiscall" fn comparar_cqu8_supercopa_japonesa(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I2%%)(this, ano, dono);
    let erro = cqu8_plano(this, r, ano, dono, 0x96cc48, &CQU8_CAMPOS_SUPERCOPA_JAPONESA, true);
    PLACAR.record(erro.is_none(), || format!("cqu8_supercopa_japonesa({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}
