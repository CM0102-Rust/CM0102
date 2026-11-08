// ---------- %%LEVA%%a leva: as estatísticas dos jogadores e a aposentadoria marcada: o registro da categoria, criar o registro e se a pessoa está na lista de aposentadorias (player_stats.cpp, player_regen.cpp) ----------

unsafe fn cpq10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq10_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A vaga da categoria `categoria` (1..6, byte com sinal) no índice do jogador: o registro de 39 bytes do id dele
/// (+0) em [this+0x30], dword 4 × (categoria − 1). None sem jogador ou fora de 1..6.
unsafe fn category_slot(this: usize, jogador: usize, categoria: u32) -> Option<usize> {
    let k = (categoria as u8 as i8 as i32).wrapping_sub(1);
    if jogador == 0 || k as u32 > 5 {
        return None;
    }
    Some(cpq10_d(this + 0x30).wrapping_add(cpq10_d(jogador).wrapping_mul(39)) + 4 * k as usize)
}

/// A caixa "Memory Allocation Error" de player_stats.cpp (0xa78310): "v%s %s.%s %d (%s)" (versão 0x90d130, nome e
/// extensão, `linha`, `texto`) em 0xae24d0, o título traduzido (0x66f4e0) em 0xae28bc, o corpo (0x9a7ff8 com
/// 0xae24d0) em 0xae26c4 e a caixa 0x5e8290. O jogo segue.
unsafe fn show_stats_memory_error(linha: u32, texto: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa78310, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// { 0x7c3920 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::get_category_record(jogador, categoria)`: o
/// registro de 0x20 bytes ([this+4] + 32 × índice) da categoria do jogador (category_slot), ou 0 quando o índice é −1
/// ou menor. Sem jogador ou fora de 1..6, a caixa "Error" e 0: a original. Pela PlayerStats::record_match 0x7c10e6 e
/// pela PlayerStats::get_stat 0x7c1e30 (as médias do motor de partida). Quente: sem contador.
unsafe extern "thiscall" fn get_category_record(this: usize, jogador: usize, categoria: u32) -> usize {
    let vaga = match category_slot(this, jogador, categoria) {
        Some(v) => v,
        _ => return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I0%%)(this, jogador, categoria),
    };
    let i = cpq10_d(vaga) as i32; // o índice da categoria
    if i <= -1 {
        return 0;
    }
    cpq10_d(this + 4).wrapping_add((i as usize) << 5)
}

/// { 0x7c5d80 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::create_stats(jogador, categoria)`: o registro da
/// categoria do jogador (category_slot); sem índice (−1), com registros soltos ([this+0x38]), o primeiro registro de
/// [this+4] (conta +0xc) com +0xe = 0xffff vira o dele (zerado; os soltos descem); senão um no fim (zerado; a conta
/// sobe): cheio (capacidade +8), cresce de 100 em 100 (realloc 0x945667; sem memória, a caixa 0xef4 e 0). Sem
/// jogador ou fora de 1..6, a caixa "Error" e 0: a original. Pela PlayerStats::record_match 0x7c10f9.
unsafe extern "thiscall" fn create_stats(this: usize, jogador: usize, categoria: u32) -> usize {
    let vaga = match category_slot(this, jogador, categoria) {
        Some(v) => v,
        _ => return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I1%%)(this, jogador, categoria),
    };
    if cpq10_d(vaga) as i32 != -1 {
        return cpq10_d(this + 4).wrapping_add(cpq10_d(vaga) << 5);
    }
    if cpq10_d(this + 0x38) != 0 {
        let n = cpq10_d(this + 0xc) as i32;
        let mut i = 0i32;
        while i < n {
            let r = cpq10_d(this + 4).wrapping_add((i as usize) << 5);
            if core::ptr::read_unaligned((r + 0xe) as *const u16) == 0xffff {
                crate::comparison::branch("cpq_10 record freed");
                core::ptr::write_bytes(r as *mut u8, 0, 0x20);
                cpq10_por(vaga, i as usize);
                cpq10_por(this + 0x38, cpq10_d(this + 0x38).wrapping_sub(1));
                return r;
            }
            i += 1;
        }
    }
    let n = cpq10_d(this + 0xc) as i32;
    if n >= cpq10_d(this + 8) as i32 {
        let cap = cpq10_d(this + 8).wrapping_add(100);
        cpq10_por(this + 8, cap);
        let realocar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945667usize);
        let p = realocar(cpq10_d(this + 4), cap << 5);
        cpq10_por(this + 4, p);
        if p == 0 {
            show_stats_memory_error(0xef4, 0xa784fc); // "PLAYER_STATS::create_stats() : Unable to allocate enough memory"
            return 0;
        }
    }
    crate::comparison::branch("cpq_10 new record");
    cpq10_por(vaga, cpq10_d(this + 0xc)); // o índice novo
    cpq10_por(this + 0xc, cpq10_d(this + 0xc).wrapping_add(1));
    let r = cpq10_d(this + 4).wrapping_add(cpq10_d(vaga) << 5);
    core::ptr::write_bytes(r as *mut u8, 0, 0x20);
    r
}

/// { 0x7ae350 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::is_retirement_pending(pessoa)`: 1 quando o id da
/// pessoa (+0) está na lista de aposentadorias ([this+4], registros de 0x1a bytes, conta +8 com sinal), senão 0 (e 0
/// sem pessoa). Pela MANAGER_MANAGER::is_staff_suitable_for_job 0x69fa4b: quem não é jogador e vai se aposentar não
/// serve para técnico.
unsafe extern "thiscall" fn is_retirement_pending(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        return 0;
    }
    let n = cpq10_d(this + 8) as i32;
    let id = cpq10_d(pessoa);
    let mut i = 0i32;
    while i < n {
        if cpq10_d(cpq10_d(this + 4) + 0x1a * i as usize) == id {
            crate::comparison::branch("cpq_10 retirement scheduled");
            return 1; // está na lista
        }
        i += 1;
    }
    0
}

/// Comparador: pelo valor (só lê); os casos de erro só na original.
unsafe extern "thiscall" fn compare_get_category_record(this: usize, jogador: usize, categoria: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I0%%)(this, jogador, categoria);
    if category_slot(this, jogador, categoria).is_none() {
        return r;
    }
    let n = get_category_record(this, jogador, categoria);
    SCOREBOARD.record(r == n, || format!("get_category_record({:x}, {:x}, {}) original={:x} rust={:x}", this, jogador, categoria as u8 as i8, r, n));
    r
}

/// Comparador: a original realoca o vetor, então só ela roda; antes, o plano (o índice que o jogador vai ter, a
/// conta e os soltos depois); depois, o registro devolvido, o índice na vaga, a conta, os soltos e o registro
/// zerado quando é novo. ponytail: os casos de erro e a falta de memória só na original, sem conferir.
unsafe extern "thiscall" fn compare_create_stats(this: usize, jogador: usize, categoria: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(%%I1%%);
    let vaga = match category_slot(this, jogador, categoria) {
        Some(v) => v,
        _ => return o(this, jogador, categoria),
    };
    let atual = cpq10_d(vaga) as i32;
    let (n, soltos) = (cpq10_d(this + 0xc), cpq10_d(this + 0x38));
    let solto = if atual == -1 && soltos != 0 {
        (0..n as i32).find(|&i| core::ptr::read_unaligned((cpq10_d(this + 4) + ((i as usize) << 5) + 0xe) as *const u16) == 0xffff)
    } else {
        None
    };
    let (indice, n2, soltos2) = match (atual, solto) {
        (-1, Some(i)) => (i, n, soltos.wrapping_sub(1)),
        (-1, None) => (n as i32, n.wrapping_add(1), soltos),
        (i, _) => (i, n, soltos),
    };
    let r = o(this, jogador, categoria);
    if r == 0 {
        return r;
    }
    let esperado = cpq10_d(this + 4).wrapping_add((indice as usize) << 5);
    let zerado = atual != -1 || (0..0x20).all(|k| *((r + k) as *const u8) == 0);
    let igual = r == esperado && cpq10_d(vaga) as i32 == indice && cpq10_d(this + 0xc) == n2 && cpq10_d(this + 0x38) == soltos2 && zerado;
    SCOREBOARD.record(igual, || format!("create_stats({:x}, {:x}, {}) original={:x} plano={:x} índice {} x {} conta {} x {} soltos {} x {} zerado={}",
        this, jogador, categoria as u8 as i8, r, esperado, cpq10_d(vaga) as i32, indice, cpq10_d(this + 0xc), n2, cpq10_d(this + 0x38), soltos2, zerado));
    r
}

compare_value!(compare_is_retirement_pending, is_retirement_pending, %%I2%%, (pessoa: usize) -> u8);
