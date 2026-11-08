// ---------- %%LEVA%%a leva: os registros das transferências: guardar a transferência passada, dar a permissão de trabalho e guardar o prêmio por jogo no time (transfer_manager.cpp) ----------

unsafe fn cpl3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpl3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A caixa "Memory Allocation Error" de transfer_manager.cpp (0xa8d514): "v%s %s.%s %d (%s)" (versão 0x90d130, nome
/// e extensão, `linha`, `texto`) em 0xae24d0, o título traduzido (0x66f4e0) em 0xae28bc, o corpo (0x9a7ff8 com
/// 0xae24d0) em 0xae26c4 e a caixa 0x5e8290. O jogo segue.
unsafe fn cpl3_sem_memoria(linha: u32, texto: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa8d514, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// Cresce o vetor [this+v] (capacidade em +v+`c`, conta em +v+12−`c`: `c` = 4 nas passadas e nos prêmios, 8 nas
/// permissões) de `passo` registros de `tam` bytes quando cheio: malloc (0x944e46) sem vetor, senão realloc
/// (0x945667). Devolve se há vetor (sem memória: a caixa já saiu).
unsafe fn cpl3_crescer(this: usize, v: usize, c: usize, passo: usize, tam: usize, linha: u32, texto: usize) -> bool {
    if cpl3_d(this + v + 12 - c) != cpl3_d(this + v + c) {
        return true;
    }
    let cap = cpl3_d(this + v + c).wrapping_add(passo);
    cpl3_por(this + v + c, cap);
    let p = if cpl3_d(this + v) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
        alocar(cap.wrapping_mul(tam) as u32)
    } else {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        realocar(cpl3_d(this + v), cap.wrapping_mul(tam) as u32)
    };
    cpl3_por(this + v, p);
    if p == 0 {
        cpl3_sem_memoria(linha, texto);
        return false;
    }
    true
}

/// { 0x8cb810 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_past_transfer(registro)`: acrescenta
/// a transferência passada (0x25 bytes, cópia 0x8cb940) ao vetor [this+0x862] (capacidade +0x866, conta +0x86a, que
/// sobe antes da cópia); cheio, cresce de 100 em 100 (sem memória: caixa 0x2794 e nada). Devolve o registro novo.
unsafe extern "thiscall" fn add_past_transfer(this: usize, registro: usize) -> usize {
    crate::comparacao::ramo("cpl_3 transferencia passada");
    // ponytail: sem memória a original devolve o que a caixa deixou em eax; aqui 0 (o jogo não lê)
    if !cpl3_crescer(this, 0x862, 4, 100, 0x25, 0x2794, 0xa8ec34) {
        return 0;
    }
    let n = cpl3_d(this + 0x86a);
    let novo = cpl3_d(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    cpl3_por(this + 0x86a, n.wrapping_add(1)); // a conta das transferências passadas
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8cb940usize);
    copiar(novo, registro)
}

/// { 0x8c7070 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::add_work_permit(pessoa, nacao, ano, _,
/// ordenar)`: acrescenta a permissão de trabalho (14 bytes: +0 id da pessoa, +4 e +6 zerados, +8 ano, +0xa id da
/// nação) ao vetor [this+0x93a] (capacidade +0x942, conta +0x93e); cheio, cresce de 30 em 30 (sem memória: caixa
/// 0x1b1b e nada). Liga a marca da permissão (bit 0x20 de [this+0x86e] + id × 8) e, com `ordenar`, reordena (qsort
/// 0x945333 com 0x8e55c0, a ordem da busca 0x8c7560).
unsafe extern "thiscall" fn add_work_permit(this: usize, pessoa: usize, nacao: usize, ano: u32, _nao_usado: u32, ordenar: u32) {
    crate::comparacao::ramo("cpl_3 permissao de trabalho");
    if !cpl3_crescer(this, 0x93a, 8, 30, 0xe, 0x1b1b, 0xa8eb04) {
        return;
    }
    let n = cpl3_d(this + 0x93e);
    let r = cpl3_d(this + 0x93a).wrapping_add(n.wrapping_mul(0xe));
    cpl3_por(r, cpl3_d(pessoa));
    cpl3_por(r + 0xa, cpl3_d(nacao));
    core::ptr::write_unaligned((r + 8) as *mut u16, ano as u16);
    core::ptr::write_unaligned((r + 6) as *mut u16, 0);
    core::ptr::write_unaligned((r + 4) as *mut u16, 0);
    cpl3_por(this + 0x93e, n.wrapping_add(1));
    let marca = (cpl3_d(this + 0x86e).wrapping_add(cpl3_d(pessoa).wrapping_mul(8))) as *mut u8;
    *marca |= 0x20; // a marca da permissão
    if ordenar != 0 {
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
        qsort(cpl3_d(this + 0x93a), cpl3_d(this + 0x93e) as u32, 0xe, 0x8e55c0);
    }
}

/// { 0x8cac40 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_first_team_appearance_money(registro)`:
/// acrescenta o prêmio por jogo no time principal (13 bytes copiados de `registro`) ao vetor [this+0x88c] (capacidade
/// +0x890, conta +0x894); cheio, cresce de 50 em 50 (sem memória: caixa 0x2581 e nada). Reordena pela pessoa (qsort
/// 0x945333 com 0x8b4e00).
unsafe extern "thiscall" fn add_first_team_appearance_money(this: usize, registro: usize) {
    crate::comparacao::ramo("cpl_3 premio por jogo");
    if !cpl3_crescer(this, 0x88c, 4, 50, 0xd, 0x2581, 0xa8eb94) {
        return;
    }
    let n = cpl3_d(this + 0x894);
    core::ptr::copy_nonoverlapping(registro as *const u8, cpl3_d(this + 0x88c).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    cpl3_por(this + 0x894, n.wrapping_add(1)); // mais um prêmio
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpl3_d(this + 0x88c), cpl3_d(this + 0x894) as u32, 0xd, 0x8b4e00);
}

/// Algum dos `n` registros de `tam` bytes do vetor `v` é igual a `alvo`.
unsafe fn cpl3_tem(v: usize, n: usize, tam: usize, alvo: &[u8]) -> bool {
    (0..n).any(|i| core::slice::from_raw_parts((v + i * tam) as *const u8, tam) == alvo)
}

/// Comparador: a original acrescenta (pode realocar) e não pode rodar duas vezes: o plano antes (a conta, o registro),
/// a original roda, confere a conta, o registro novo (as duplas e o byte +0x1c; a data é da 0x8cb940) e a devolução.
unsafe extern "thiscall" fn comparar_add_past_transfer(this: usize, registro: usize) -> usize {
    let n = cpl3_d(this + 0x86a);
    let fonte = core::slice::from_raw_parts(registro as *const u8, 0x1d).to_vec();
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%)(this, registro);
    if cpl3_d(this + 0x862) == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let novo = cpl3_d(this + 0x862).wrapping_add(n.wrapping_mul(0x25));
    let gravado = core::slice::from_raw_parts(novo as *const u8, 0x1d);
    PLACAR.registrar(r == novo && cpl3_d(this + 0x86a) == n.wrapping_add(1) && gravado == &fonte[..], || {
        format!("add_past_transfer: conta {} -> {}, devolveu {:x} (esperava {:x}), registro {:x?} x {:x?}", n, cpl3_d(this + 0x86a), r, novo, fonte, gravado)
    });
    r
}

/// Comparador: o plano antes (a conta e o registro esperado), a original roda; confere a conta, a marca 0x20 e que o
/// registro está no vetor (com `ordenar`, em qualquer lugar).
unsafe extern "thiscall" fn comparar_add_work_permit(this: usize, pessoa: usize, nacao: usize, ano: u32, nao_usado: u32, ordenar: u32) {
    let n = cpl3_d(this + 0x93e);
    let mut alvo = [0u8; 0xe];
    alvo[0..4].copy_from_slice(&(cpl3_d(pessoa) as u32).to_le_bytes());
    alvo[8..10].copy_from_slice(&(ano as u16).to_le_bytes());
    alvo[0xa..0xe].copy_from_slice(&(cpl3_d(nacao) as u32).to_le_bytes());
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(%%I1%%)(this, pessoa, nacao, ano, nao_usado, ordenar);
    if cpl3_d(this + 0x93a) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpl3_d(this + 0x93e);
    let marca = *((cpl3_d(this + 0x86e).wrapping_add(cpl3_d(pessoa).wrapping_mul(8))) as *const u8);
    let ok = conta == n.wrapping_add(1) && marca & 0x20 != 0 && cpl3_tem(cpl3_d(this + 0x93a), conta, 0xe, &alvo);
    PLACAR.registrar(ok, || format!("add_work_permit(pessoa {}, nação {}, ano {}): conta {} -> {}, marca {:#x}", cpl3_d(pessoa), cpl3_d(nacao), ano as u16, n, conta, marca));
}

/// Comparador: o plano antes (a conta e os 13 bytes), a original roda; confere a conta e que o registro está no vetor
/// (reordenado).
unsafe extern "thiscall" fn comparar_add_first_team_appearance_money(this: usize, registro: usize) {
    let n = cpl3_d(this + 0x894);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, registro);
    if cpl3_d(this + 0x88c) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpl3_d(this + 0x894);
    let ok = conta == n.wrapping_add(1) && cpl3_tem(cpl3_d(this + 0x88c), conta, 0xd, &alvo);
    PLACAR.registrar(ok, || format!("add_first_team_appearance_money: conta {} -> {}, registro {:x?}", n, conta, alvo));
}
