// ---------- %%LEVA%%a leva: os construtores do Troféu da Liga inglesa e da Liga dos Campeões (eng_auto_cup.cpp, european_cup.cpp) ----------

unsafe fn cqv9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqv9_p8(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqv9_p16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqv9_p32(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// As rodadas da copa pelo método +0x3c da vtable (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall,
/// ret 0x10): a tabela das rodadas, gravada em +0xa3.
unsafe fn cqv9_rodadas(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqv9_d(cqv9_d(this) + 0x3c));
    let t = f(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    cqv9_p32(this + 0xa3, t as u32);
}

/// As estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// this; mov ecx, novo -> thiscall, ret 4) em +8; o quadro SEH da original cobre só esse new. Devolve se criou.
unsafe fn cqv9_estatisticas(this: usize) -> bool {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    cqv9_p32(this + 8, s as u32);
    s != 0
}

/// O plano que o comparador confere depois da original: a devolução (this), a vtable, o registro (+4) e o ano (+0x40);
/// na copa nova (+0x44 = 0xff), cada campo fixo que o construtor grava e as rodadas e as estatísticas criadas.
unsafe fn cqv9_plano(this: usize, r: usize, ano: u32, dono: usize, vt: usize, campos: &[(usize, u8, u32)], rodadas: bool) -> Option<String> {
    if r != this || cqv9_d(this) != vt || cqv9_d(this + 4) != dono || core::ptr::read_unaligned((this + 0x40) as *const u16) != ano as u16 {
        return Some(format!("devolveu {:#x}, vtable {:#x}, +4 {:#x}, ano {}", r, cqv9_d(this), cqv9_d(this + 4), core::ptr::read_unaligned((this + 0x40) as *const u16)));
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
    if (rodadas && cqv9_d(this + 0xa3) == 0) || cqv9_d(this + 8) == 0 {
        return Some(format!("rodadas {:#x}, estatísticas {:#x}", cqv9_d(this + 0xa3), cqv9_d(this + 8)));
    }
    None
}

/// { 0x56aa30 } (thiscall, ret 8, eng_auto_cup.cpp; da criação da carreira, n = 1): `EngAutoCup::EngAutoCup(ano, registro)`: o construtor de
/// o Troféu da Liga inglesa
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x969894, o registro em +4, o ano em +0x40 e
/// os da tabela CQV9_CAMPOS_TROFEU_DA_LIGA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: o vetor das fases em +0xc (malloc 0x944e46 de 4 vezes +0x2c, ou 0 sem fases); os 56 clubes das chaves (EngAutoCup::select_section_clubs) (0x56b730, thiscall, ret 0; a caixa da linha 0x52 se devolver 0); os inscritos (EngAutoCup::collect_entrants) (0x56b3c0, thiscall, ret 0; a caixa da linha 0x59 se devolver 0); as rodadas (o método +0x3c da vtable, em +0xa3); as estatísticas em +8 (a caixa da linha 0x68 sem elas); a fase das chaves (EngAutoCup::create_section_stage) (0x56bab0, thiscall, ret 0, sem conferir); a lista +0xb2 liberada (free 0x9452ca) e zerada; a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x78 se devolver 0); a classificação final (EngAutoCup::rank_teams) (0x56c010, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: fundo de prêmios de 400.000; 5 reservas no banco e 3 substituições; regra de disciplina 9 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqv9_trofeu_da_liga_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqv9_p32(this + 4, dono as u32);
    cqv9_p32(this, 0x969894);
    cqv9_p16(this + 0x40, ano as u16);
    cqv9_p8(this + 0xab, 0x0);
    cqv9_p8(this + 0x44, 0xff);
    cqv9_p32(this + 0x30, 0xffffffff);
    cqv9_p32(this + 0x2c, 0x2);
    cqv9_p8(this + 0x43, 0x1);
    cqv9_p32(this + 0x1c, 0xffffffff);
    cqv9_p32(this + 0x20, 0xffffffff);
    cqv9_p32(this + 0xb2, 0x0);
    cqv9_p8(this + 0x50, 0x9);
    cqv9_p8(this + 0x49, 0x5);
    cqv9_p8(this + 0x4a, 0x3);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqv_9 trofeu_da_liga nova");
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let fases = cqv9_d(this + 0x2c) as i32;
    cqv9_p32(this + 0xc, if fases > 0 { alocar((fases << 2) as u32) as u32 } else { 0 });
    cqv9_p8(this + 0x51, 0xf);
    cqv9_p8(this + 0xb1, 0x0);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56b730usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4180, 0x52);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56b3c0usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4180, 0x59);
        return this;
    }
    cqv9_p32(this + 0x28, 0x61a80);
    cqv9_rodadas(this);
    if !cqv9_estatisticas(this) {
        show_error_at(0x9c4180, 0x68);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56bab0usize);
    f(this);
    let l = cqv9_d(this + 0xb2);
    if l != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(l);
        cqv9_p32(this + 0xb2, 0);
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4180, 0x78);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56c010usize);
    f(this);
    this
}

const CQV9_CAMPOS_TROFEU_DA_LIGA: [(usize, u8, u32); 13] = [(0x1c, 4, 0xffffffff), (0x20, 4, 0xffffffff), (0x28, 4, 0x61a80), (0x2c, 4, 0x2), (0x43, 1, 0x1), (0x44, 1, 0xff), (0x49, 1, 0x5), (0x4a, 1, 0x3), (0x50, 1, 0x9), (0x51, 1, 0xf), (0xab, 1, 0x0), (0xb1, 1, 0x0), (0xb2, 4, 0x0)];

unsafe extern "thiscall" fn cqv9_trofeu_da_liga(this: usize, ano: u32, dono: usize) -> usize {
    cqv9_trofeu_da_liga_corpo(this, ano, dono) // cqv9: trofeu_da_liga
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqv9_plano).
unsafe extern "thiscall" fn comparar_cqv9_trofeu_da_liga(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, ano, dono);
    let erro = cqv9_plano(this, r, ano, dono, 0x969894, &CQV9_CAMPOS_TROFEU_DA_LIGA, true);
    PLACAR.record(erro.is_none(), || format!("cqv9_trofeu_da_liga({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x5837c0 } (thiscall, ret 8, european_cup.cpp; da criação da carreira, n = 1): `EuropeanCup::EuropeanCup(ano, registro)`: o construtor de
/// a Liga dos Campeões
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96a1d0, o registro em +4, o ano em +0x40 e
/// os da tabela CQV9_CAMPOS_LIGA_DOS_CAMPEOES); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: o vetor das fases em +0xc (malloc 0x944e46 de 0x34: push; add esp, 4 -> cdecl, sem conferir); a semeadura europeia do ano (UefaSeeding::update_season 0x9035a0 em [0xde1f58], se houver; thiscall, ret 4); os 71 inscritos (EuropeanCup::collect_entrants) (0x584fa0, thiscall, ret 0, sem conferir); a tabela das preliminares (EuropeanCup::build_qualifying_entry_table) (0x584990, thiscall, ret 0; a caixa da linha 0x74 se devolver 0); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x82 se devolver 0); as estatísticas em +8 (a caixa da linha 0x89 sem elas); a classificação final (EuropeanCup::rank_teams) (0x586900, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 2 (clubes de várias nações); sem fundo de prêmios (+0x28 fica como a base deixou); 7 reservas no banco e 3 substituições; regra de disciplina 1 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqv9_liga_dos_campeoes_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqv9_p16(this + 0x40, ano as u16);
    cqv9_p32(this, 0x96a1d0);
    cqv9_p8(this + 0xab, 0x0);
    cqv9_p8(this + 0x44, 0xff);
    cqv9_p32(this + 0x30, 0xffffffff);
    cqv9_p32(this + 0x2c, 0xd);
    cqv9_p32(this + 4, dono as u32);
    cqv9_p8(this + 0x42, 0x2);
    cqv9_p32(this + 0x1c, 0xffffffff);
    cqv9_p32(this + 0x20, 0xffffffff);
    cqv9_p8(this + 0x52, 0x3);
    cqv9_p8(this + 0x49, 0x7);
    cqv9_p8(this + 0x4a, 0x3);
    cqv9_p8(this + 0x51, 0xa);
    cqv9_p8(this + 0x50, 0x1);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    cqv9_p32(this + 0xc, alocar(0x34) as u32);
    cqv9_p16(this + 0x3a, 0x0);
    cqv9_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqv_9 liga_dos_campeoes nova");
    // a semeadura europeia do ano (UefaSeeding::update_season 0x9035a0: push 0; mov ecx, [0xde1f58] -> thiscall, ret 4)
    let s = cqv9_d(0xde1f58);
    if s != 0 {
        let f: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x9035a0usize);
        f(s, 0);
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x584fa0usize);
    f(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x584990usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4900, 0x74);
        return this;
    }
    cqv9_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x9c4900, 0x82);
        return this;
    }
    if !cqv9_estatisticas(this) {
        show_error_at(0x9c4900, 0x89);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x586900usize);
    f(this);
    this
}

const CQV9_CAMPOS_LIGA_DOS_CAMPEOES: [(usize, u8, u32); 13] = [(0x1c, 4, 0xffffffff), (0x20, 4, 0xffffffff), (0x2c, 4, 0xd), (0x30, 4, 0xffffffff), (0x42, 1, 0x2), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x1), (0x51, 1, 0xa), (0x52, 1, 0x3), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqv9_liga_dos_campeoes(this: usize, ano: u32, dono: usize) -> usize {
    cqv9_liga_dos_campeoes_corpo(this, ano, dono) // cqv9: liga_dos_campeoes
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqv9_plano).
unsafe extern "thiscall" fn comparar_cqv9_liga_dos_campeoes(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, ano, dono);
    let erro = cqv9_plano(this, r, ano, dono, 0x96a1d0, &CQV9_CAMPOS_LIGA_DOS_CAMPEOES, true);
    PLACAR.record(erro.is_none(), || format!("cqv9_liga_dos_campeoes({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}
