// ---------- %%LEVA%%a leva: o save das finanças e do gerente de elencos: gravar e carregar o finance.dat e gravar o squad_manager.dat (fin_prm.cpp, squad_manager.cpp) ----------

unsafe fn cpx5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn club_count() -> u32 {
    *(0xae235c as *const u32)
}

/// { 0x59b3b0 } (thiscall, ret 4, fin_prm.cpp; o fonte se chama finance.CPP): `FinanceTable::save_finance_dat(save)`:
/// grava no item `finance.dat` (0x9326e0 gravando) as finanças de todos os clubes, 0x167 bytes × [0xae235c] a partir
/// de [this]. Sem contêiner, nada; erro ao abrir ou ao gravar: a caixa "Error" (finance.CPP 0x115e, 0x1166), sem
/// fechar o item (como a original).
unsafe extern "thiscall" fn save_finance_dat(this: usize, save: usize) {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let gravar_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparacao::ramo("cpx_5 gravar financas");
    if save == 0 {
        return;
    }
    let mut z = abrir_item(save, 0x9c87b0, 1, 0x16, 4);
    if z == 0 {
        show_error_at(0x9c8784, 0x115e);
        return;
    }
    if gravar_item(cpx5_d(this), 0x167, club_count(), z) != club_count() { // cpx5: os registros das finanças
        show_error_at(0x9c8784, 0x1166);
        return;
    }
    fechar_item(&mut z);
}

/// Comparador da 0x59b3b0: só a original.
unsafe extern "thiscall" fn comparar_save_finance_dat(this: usize, save: usize) {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    // (o .qst gravado no modo rust com e sem a leva, item a item)
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, save)
}

/// { 0x59b2a0 } (thiscall, ret 4, fin_prm.cpp): `FinanceTable::load_from_save(save)`: lê do item `finance.dat`
/// (0x9326e0 lendo) as finanças de todos os clubes, 0x167 bytes × [0xae235c] em [this], e devolve 1. Sem contêiner,
/// 0 sem caixa; erro ao abrir ou leitura curta: a caixa "Error" (finance.CPP 0x1137, 0x113f), 0 e o item fica aberto
/// (como a original). Com 0, o construtor (0x59a55c) refaz as finanças de cada clube do zero (0x5961d0).
unsafe extern "thiscall" fn load_from_save(this: usize, save: usize) -> u32 {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let ler_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932c30usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparacao::ramo("cpx_5 carregar financas");
    if save == 0 {
        return 0;
    }
    let mut z = abrir_item(save, 0x9c87b0, 0, 0x16, 4);
    if z == 0 {
        show_error_at(0x9c8784, 0x1137);
        return 0;
    }
    if ler_item(cpx5_d(this), 0x167, club_count(), z) != club_count() {
        show_error_at(0x9c8784, 0x113f);
        return 0;
    }
    fechar_item(&mut z);
    1 // cpx5: carregou (o construtor não refaz as finanças)
}

/// Comparador da 0x59b2a0: só a original.
unsafe extern "thiscall" fn comparar_load_from_save(this: usize, save: usize) -> u32 {
    // ponytail: lê do contêiner (a posição do item anda; não pode rodar duas vezes); a cópia é provada pela carga
    // do save de férias na prova do save (levas/N/CARGA)
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%)(this, save)
}

/// { 0x859cc0 } (thiscall, ret 4, squad_manager.cpp): `SQUAD_MANAGER::save(save)`: grava no item `squad_manager.dat`
/// o u16 em +0 do gerente de elencos, estendido a 4 bytes (a original zera eax antes do `mov ax`), e devolve 1. Erro
/// ao abrir ou ao gravar: a caixa "Error" (squad_manager.cpp 0x68, 0x6f) e 0, sem fechar o item. Não confere o
/// contêiner nulo (o 0x9326e0 recebe como vier).
unsafe extern "thiscall" fn squad_manager_save(this: usize, save: usize) -> u32 {
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let gravar_item: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x932e10usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparacao::ramo("cpx_5 gravar gerente de elencos");
    let mut v = *(this as *const u16) as u32; // cpx5: o valor do gerente de elencos
    let mut z = abrir_item(save, 0xa82bc0, 1, 0x16, 4);
    if z == 0 {
        show_error_at(0xa82b88, 0x68);
        return 0;
    }
    if gravar_item(&mut v as *mut u32 as usize, 4, 1, z) != 1 {
        show_error_at(0xa82b88, 0x6f);
        return 0;
    }
    fechar_item(&mut z);
    1
}

/// Comparador da 0x859cc0: só a original.
unsafe extern "thiscall" fn comparar_squad_manager_save(this: usize, save: usize) -> u32 {
    // ponytail: grava no contêiner do save (não pode rodar duas vezes); a cópia é provada pela prova do save
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%)(this, save)
}
