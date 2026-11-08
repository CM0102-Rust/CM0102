// ---------- %%LEVA%%a leva: os olheiros dos técnicos humanos na partida e a sessão de edição das buscas (scout_manager.cpp) ----------

unsafe fn cqc13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x7f8c00 } (thiscall, ret 8, scout_manager.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `SCOUT_MANAGER::scout_match(partida, resultado)`, pelo fechamento da partida do dia (0x6b5f00, Rust). Para cada um
/// dos 16 últimos registros de pessoa ([0xae23bc], 0x6e bytes, do último para trás) com +0x5f = 1 (o técnico humano):
/// num vetor zerado de 10 marcas (0x945501), os 10 observadores numa ordem sorteada (0x90cfc0 de 10 até achar um não
/// marcado); o registro de cada um (0x7f6cd0) com +0x19d, numa partida com as marcas +0x40 e +0x41 e do clube do
/// técnico (+0x39 = +0x1c ou +0x20 da partida), começa a observação (0x7ae9c0 com −1, −1, −1, 0, 1); e observa a
/// partida (0x7b5be0, os relatórios na lista local). O vetor sai (0x9452ca). Por fim os relatórios vão (0x7f9890) e a
/// lista é destruída. Devolve 1.
unsafe extern "thiscall" fn scout_manager_scout_match(this: usize, p: usize, resultado: usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let zerado: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let registro: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    let comecar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) = core::mem::transmute(0x7ae9c0usize);
    let observar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x7b5be0usize);
    let enviar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7f9890usize);
    // calloc: push 4; push 0xa; add esp, 8 -> cdecl (10, 4); sorteio: push 0xa; add esp, 4 -> cdecl; free: cdecl;
    // 0x7f6cd0: push pessoa; push qual; mov ecx, [this] -> thiscall ret 8; 0x7ae9c0: 5 pushes; mov ecx, registro ->
    // thiscall ret 0x14; 0x7b5be0: push &lista; push resultado; push partida; mov ecx, registro -> thiscall ret 0xc;
    // 0x7f9890: push &lista; mov ecx, this -> thiscall ret 4; 0x68d020/0x68d040: lea ecx, lista -> thiscall
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    for k in 0..16i32 {
        let idx = (cqc13_d(0xae2364) as i32).wrapping_sub(k).wrapping_sub(1);
        let pessoa = cqc13_d(0xae23bc).wrapping_add((idx as usize).wrapping_mul(0x6e));
        if *((pessoa + 0x5f) as *const u8) != 1 {
            continue;
        }
        crate::comparison::branch("cqc_13 human scouts");
        let marcas = zerado(10, 4);
        for _ in 0..10 { // cqc13: os 10 observadores
            let q = loop {
                let q = sortear(10) as i8; // cqc13: o observador sorteado
                if cqc13_d(marcas.wrapping_add((q as isize * 4) as usize)) == 0 {
                    break q; // cqc13: um ainda não marcado
                }
            };
            core::ptr::write_unaligned(marcas.wrapping_add((q as isize * 4) as usize) as *mut u32, 1);
            // ponytail: a original empurra o dword com o byte do observador e sobra de pilha em cima; a 0x7f6cd0 lê o
            // byte
            let r = registro(this, q as u8 as u32, pessoa);
            if cqc13_d(r + 0x19d) != 0 && *((p + 0x40) as *const u8) != 0 && *((p + 0x41) as *const u8) != 0 {
                let clube = cqc13_d(pessoa + 0x39);
                if cqc13_d(p + 0x1c) == clube || cqc13_d(p + 0x20) == clube {
                    comecar(r, u32::MAX, u32::MAX, u32::MAX, 0, 1);
                }
            }
            observar(r, p, resultado, lista);
        }
        liberar(marcas);
    }
    enviar(this, lista);
    soltar(lista);
    1
}

/// Comparador: a observação (0x7b5be0, original) grava o que os olheiros sabem e os relatórios, a 0x7ae9c0 começa
/// buscas e a 0x7f9890 entrega as notícias: não pode rodar duas vezes. Só a original roda (ponytail: ela mesma só grava
/// o vetor e a lista locais, que saem); confere a devolução (1).
unsafe extern "thiscall" fn compare_scout_manager_scout_match(this: usize, p: usize, resultado: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I0%%)(this, p, resultado);
    SCOREBOARD.record(r == 1, || format!("scout_manager_scout_match(partida {:x}) original={}", p, r));
    r
}

/// O índice (0..15) do técnico `tecnico` (nulo: o humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]]) pela conta de um
/// byte da original ((id − [0xae2364]) + 16 em 8 bits); None nos casos da caixa (sem técnico ou fora).
unsafe fn scout_manager_human_manager_index(tecnico: usize) -> Option<usize> {
    let t = if tecnico != 0 {
        tecnico
    } else {
        cqc13_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)))
    };
    if t == 0 {
        return None;
    }
    let i = (*(t as *const u8)).wrapping_sub(*(0xae2364 as *const u8)).wrapping_add(0x10) as i8;
    if (0..=0xf).contains(&i) { Some(i as usize) } else { None }
}

/// { 0x7f6be0 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::get_edit_session(tecnico)`, pela montagem dos
/// conhecidos da busca (0x7afeb7). A sessão de edição do técnico humano (+0xc + 4 × índice). Sem técnico ou índice
/// fora: a caixa (linhas 0xde e 0xe8; a original) e 0.
unsafe extern "thiscall" fn get_edit_session(this: usize, tecnico: usize) -> usize {
    let Some(i) = scout_manager_human_manager_index(tecnico) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%)(this, tecnico); // a caixa
    };
    cqc13_d(this + 0xc + i * 4)
}

/// Comparador: pelo valor (só lê); com a caixa, só a original.
unsafe extern "thiscall" fn compare_get_edit_session(this: usize, tecnico: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    if scout_manager_human_manager_index(tecnico).is_none() {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = get_edit_session(this, tecnico);
    SCOREBOARD.record(r == n, || format!("get_edit_session(tecnico {:x}) original={:x} rust={:x}", tecnico, r, n));
    r
}

/// { 0x7f6af0 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::set_edit_session(sessao, tecnico)`, pelas telas
/// das buscas (0x8085bc, 0x812d88...). Grava a sessão em +0xc + 4 × índice do técnico e devolve 1. Sem técnico ou
/// índice fora: a caixa (linhas 0xb8 e 0xc2; a original) e 0.
unsafe extern "thiscall" fn set_edit_session(this: usize, sessao: usize, tecnico: usize) -> u32 {
    let Some(i) = scout_manager_human_manager_index(tecnico) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%)(this, sessao, tecnico); // a caixa
    };
    core::ptr::write_unaligned((this + 0xc + i * 4) as *mut usize, sessao);
    1
}

/// Comparador: lado a lado pelas 16 sessões e a devolução; com a caixa, só a original.
unsafe extern "thiscall" fn compare_set_edit_session(this: usize, sessao: usize, tecnico: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%);
    if scout_manager_human_manager_index(tecnico).is_none() {
        return o(this, sessao, tecnico);
    }
    let reg = [(this + 0xc, 0x40)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, sessao, tecnico),
        || set_edit_session(this, sessao, tecnico));
    SCOREBOARD.record(igual && r == n, || format!("set_edit_session(sessao {:x}, tecnico {:x}) original={} rust={} {:x?} x {:x?}",
        sessao, tecnico, r, n, mo, mr));
    r
}
