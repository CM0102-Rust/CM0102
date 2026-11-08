// ---------- %%LEVA%%a leva: os contratos da comissão e as datas de decisão: achar a vaga de contrato, dar o contrato à pessoa nova e marcar a data da decisão (contract_manager.cpp) ----------

unsafe fn cpl4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpl4_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A caixa "cannot create space" do CONTRACT_MANAGER: "v%s %s.%s %d (%s)" (versão 0x90d130, nome e extensão de
/// contract_manager.cpp 0x9a8190, `linha`, `texto`) em 0xae24d0; o título "Memory Allocation Error" (0x9a8108)
/// traduzido (0x66f4e0) em 0xae28bc, o corpo 0x9a7ff8 com o texto em 0xae26c4 e a caixa 0x5e8290. O jogo segue
/// ([0xb67a34] fica como está).
unsafe fn show_cannot_create_space_box(linha: u32, texto: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// { 0x4c79a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::get_free_staff_contract_space(so_no_fim)`:
/// a vaga do vetor de contratos (registros de 0x50 bytes em [this], capacidade +4, conta +8). Sem `so_no_fim`, a
/// primeira vaga vazia (pessoa +0 = −1); senão (ou sem vaga vazia) a do fim: cheio, cresce de 200 em 200 (realloc
/// 0x945667, ou malloc 0x944e46 sem vetor; sem memória, caixa 0x175 e −1) e a conta sobe. Devolve o índice. Os dois
/// chamadores (a criação dos contratos 0x4c7560 e a 0x4c7950) passam 1.
unsafe extern "thiscall" fn get_free_staff_contract_space(this: usize, so_no_fim: u32) -> i32 {
    if so_no_fim == 0 {
        let n = cpl4_d(this + 8) as i32;
        let p = cpl4_d(this);
        let mut i = 0i32;
        while i < n {
            if cpl4_d(p.wrapping_add((i as usize).wrapping_mul(0x50))) as i32 == -1 {
                return i;
            }
            i += 1;
        }
    }
    if cpl4_d(this + 8) == cpl4_d(this + 4) {
        let cap = cpl4_d(this + 4).wrapping_add(200);
        let p = cpl4_d(this);
        cpl4_por(this + 4, cap);
        let q = if p != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            realocar(p, cap.wrapping_mul(0x50) as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            alocar(cap.wrapping_mul(0x50) as u32)
        };
        cpl4_por(this, q);
        if q == 0 {
            show_cannot_create_space_box(0x175, 0x9a81c4);
            return -1;
        }
    }
    let n = cpl4_d(this + 8).wrapping_add(1);
    cpl4_por(this + 8, n); // mais uma vaga
    (n as i32).wrapping_sub(1)
}

/// { 0x4c7950 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::add_staff_contract(pessoa, valor)`: a vaga
/// do fim (0x4c79a0 com 1); sem vaga, nada. Liga a pessoa à vaga (o índice +0 do registro de 0x4f bytes dela em
/// [0xae2d0c]), põe o id dela no contrato (+0) e gera o contrato inicial (0x85e960 com o valor). Pela contratação
/// de técnico (0x68f9c0, MANAGER_MANAGER::daily_update) e pelo regen que assina com um clube (0x7a6f10). Devolve o
/// eax da original (o da 0x85e960, ou o índice negativo); os chamadores não o leem.
unsafe extern "thiscall" fn add_staff_contract(this: usize, pessoa: usize, valor: u32) -> u32 {
    let vaga: unsafe extern "thiscall" fn(usize, u32) -> i32 = core::mem::transmute(0x4c79a0usize);
    let i = vaga(this, 1);
    if i < 0 {
        return i as u32;
    }
    crate::comparison::branch("cpl_4 contrato da pessoa");
    cpl4_por(cpl4_d(0xae2d0c).wrapping_add(cpl4_d(pessoa).wrapping_mul(0x4f)), i as usize); // a pessoa aponta a vaga
    let c = cpl4_d(this).wrapping_add((i as usize).wrapping_mul(0x50));
    cpl4_por(c, cpl4_d(pessoa));
    let gerar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x85e960usize);
    gerar(c, pessoa, valor) // o valor da pessoa
}

/// As datas dos lembretes (tipo 2) que a data de decisão de tipo 1 leva antes dela: com o registro de jogador
/// (+0x61), L = moral do contrato (+0x44) + 100 (8 bits; o contrato pela tabela [0xae2d0c], como a 0x4cf5a0
/// inline: sem contrato a original lê o endereço 0x44 e cai, como aqui); L ≤ 0x50 sem lembrete; L > 0x96 um a
/// L × 90 / 200 dias antes; e um a L / 4 dias antes (sem o registro de jogador, L = 100). As datas pela
/// Date::sub_days (0x54c8e0), na ordem da original.
unsafe fn decision_reminder_dates(this: usize, pessoa: usize, data: usize, tipo: u8) -> ([[u8; 8]; 2], usize) {
    let mut v = [[0u8; 8]; 2];
    let mut k = 0usize;
    if tipo != 1 {
        return (v, k);
    }
    let menos: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mut l: u8 = 100;
    if cpl4_d(pessoa + 0x61) != 0 {
        let id = cpl4_d(pessoa) as i32;
        let c = if pessoa == 0 || id >= cpl4_d(0xae2364) as i32 {
            0
        } else {
            let i = cpl4_d(cpl4_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(0x4f))) as i32;
            if i < 0 { 0 } else { cpl4_d(this).wrapping_add((i as usize).wrapping_mul(0x50)) }
        };
        l = (*((c + 0x44) as *const u8)).wrapping_add(100);
        if l <= 0x50 {
            return (v, k);
        }
        if l > 0x96 {
            let mut t = [0u8; 8];
            let r = menos(data, t.as_mut_ptr(), (l as u32 * 90) / 200);
            v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
            k += 1;
        }
    }
    let mut t = [0u8; 8];
    let r = menos(data, t.as_mut_ptr(), (l as u32 * 50) / 200);
    v[k] = core::ptr::read_unaligned(r as *const [u8; 8]);
    k += 1;
    (v, k)
}

/// { 0x4cd6d0 } (thiscall, ret 0x10, contract_manager.cpp): `CONTRACT_MANAGER::add_decision_date(pessoa, data, fatia,
/// tipo)`: acrescenta ao vetor de 14 bytes [this+0xc] (capacidade +0x10, conta +0x14; cresce de 100 em 100 quando a
/// conta passa da capacidade − 4, realloc 0x945667 ou malloc 0x944e46; sem memória, caixa 0x103e e nada) a entrada
/// (id da pessoa +0, data +4, fatia do dia +0xc, tipo +0xd). O tipo 1 leva antes os lembretes de tipo 2
/// (`decision_reminder_dates`), sem fatia (+0xc fica como está). Pelas propostas: a decisão da pessoa (0x4cd230, 0x4ce4e0).
/// As cópias das datas (0x417c10, sem outro efeito) viram cópias de 8 bytes.
unsafe extern "thiscall" fn add_decision_date(this: usize, pessoa: usize, data: usize, fatia: u32, tipo: u32) {
    crate::comparison::branch("cpl_4 data de decisao");
    if cpl4_d(this + 0x14) as i32 > (cpl4_d(this + 0x10) as i32).wrapping_sub(4) {
        let cap = cpl4_d(this + 0x10).wrapping_add(100);
        cpl4_por(this + 0x10, cap);
        let p = cpl4_d(this + 0xc);
        let q = if p != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            realocar(p, cap.wrapping_mul(14) as u32)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            alocar(cap.wrapping_mul(14) as u32)
        };
        cpl4_por(this + 0xc, q);
        if q == 0 {
            show_cannot_create_space_box(0x103e, 0x9a83e0);
            return;
        }
    }
    let (lembretes, k) = decision_reminder_dates(this, pessoa, data, tipo as u8);
    for d in &lembretes[..k] {
        let n = cpl4_d(this + 0x14);
        let e = cpl4_d(this + 0xc).wrapping_add(n.wrapping_mul(14));
        cpl4_por(e, cpl4_d(pessoa));
        core::ptr::write_unaligned((e + 4) as *mut [u8; 8], *d);
        *((e + 0xd) as *mut u8) = 2;
        cpl4_por(this + 0x14, n.wrapping_add(1));
    }
    let n = cpl4_d(this + 0x14);
    let e = cpl4_d(this + 0xc).wrapping_add(n.wrapping_mul(14));
    cpl4_por(e, cpl4_d(pessoa));
    *((e + 0xc) as *mut u8) = fatia as u8;
    core::ptr::write_unaligned((e + 4) as *mut [u8; 8], core::ptr::read_unaligned(data as *const [u8; 8]));
    *((e + 0xd) as *mut u8) = tipo as u8;
    cpl4_por(this + 0x14, n.wrapping_add(1)); // a data da decisão
}

// ponytail: as três realocam (e a 0x4c7950 gera o contrato pela 0x85e960); não rodam duas vezes. O comparador monta
// o plano antes, roda só a original e confere; o modo Rust e a carreira provam as cópias.

/// Comparador: a vaga esperada (a primeira vazia, ou a do fim com a conta + 1 e a capacidade + 200 se cheio).
unsafe extern "thiscall" fn comparar_get_free_staff_contract_space(this: usize, so_no_fim: u32) -> i32 {
    let (n, cap, p) = (cpl4_d(this + 8), cpl4_d(this + 4), cpl4_d(this));
    let mut livre = -1i32;
    if so_no_fim == 0 {
        for i in 0..(n as i32).max(0) {
            if cpl4_d(p.wrapping_add((i as usize).wrapping_mul(0x50))) as i32 == -1 {
                livre = i;
                break;
            }
        }
    }
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i32>(%%I0%%)(this, so_no_fim);
    if r == -1 && cpl4_d(this) == 0 {
        return r; // sem memória: a caixa já saiu
    }
    let (er, en, ecap) = if livre >= 0 { (livre, n, cap) } else { (n as i32, n.wrapping_add(1), if n == cap { cap.wrapping_add(200) } else { cap }) };
    SCOREBOARD.record(r == er && cpl4_d(this + 8) == en && cpl4_d(this + 4) == ecap, || {
        format!("get_free_staff_contract_space({}) devolveu {} (esperava {}), conta {} -> {} (esperava {}), capacidade {} -> {} (esperava {})",
            so_no_fim, r, er, n, cpl4_d(this + 8), en, cap, cpl4_d(this + 4), ecap)
    });
    r
}

/// Comparador: a vaga é a do fim (a conta de antes); confere a ligação da pessoa, o id no contrato e a conta.
unsafe extern "thiscall" fn comparar_add_staff_contract(this: usize, pessoa: usize, valor: u32) -> u32 {
    let n = cpl4_d(this + 8);
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%)(this, pessoa, valor);
    if cpl4_d(this) == 0 {
        return r; // sem memória
    }
    let id = cpl4_d(pessoa);
    let ligada = cpl4_d(cpl4_d(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f)));
    let no_contrato = cpl4_d(cpl4_d(this).wrapping_add(n.wrapping_mul(0x50)));
    SCOREBOARD.record(ligada == n && no_contrato == id && cpl4_d(this + 8) == n.wrapping_add(1), || {
        format!("add_staff_contract(pessoa {}, valor {}) vaga {} (esperava {}), id no contrato {}, conta {} -> {}",
            id, valor, ligada, n, no_contrato, n, cpl4_d(this + 8))
    });
    r
}

/// Comparador: os lembretes e a entrada esperados antes da original; confere a conta e cada entrada nova.
unsafe extern "thiscall" fn comparar_add_decision_date(this: usize, pessoa: usize, data: usize, fatia: u32, tipo: u32) {
    let n = cpl4_d(this + 0x14);
    let (lembretes, k) = decision_reminder_dates(this, pessoa, data, tipo as u8);
    let d = core::ptr::read_unaligned(data as *const [u8; 8]);
    let id = cpl4_d(pessoa);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I2%%)(this, pessoa, data, fatia, tipo);
    if cpl4_d(this + 0xc) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let v = cpl4_d(this + 0xc);
    let e = |j: usize| v.wrapping_add(n.wrapping_add(j).wrapping_mul(14));
    let data_de = |j: usize| core::ptr::read_unaligned((e(j) + 4) as *const [u8; 8]);
    let mut certo = cpl4_d(this + 0x14) == n.wrapping_add(k + 1);
    for (j, l) in lembretes[..k].iter().enumerate() {
        certo &= cpl4_d(e(j)) == id && data_de(j) == *l && *((e(j) + 0xd) as *const u8) == 2;
    }
    certo &= cpl4_d(e(k)) == id && data_de(k) == d && *((e(k) + 0xc) as *const u8) == fatia as u8 && *((e(k) + 0xd) as *const u8) == tipo as u8;
    SCOREBOARD.record(certo, || {
        format!("add_decision_date(pessoa {}, fatia {}, tipo {}) conta {} -> {} (esperava {}), lembretes {:?}, entradas {:?}",
            id, fatia as u8, tipo as u8, n, cpl4_d(this + 0x14), n + k + 1, &lembretes[..k],
            (0..=k).map(|j| core::ptr::read_unaligned(e(j) as *const [u8; 14])).collect::<Vec<_>>())
    });
}
