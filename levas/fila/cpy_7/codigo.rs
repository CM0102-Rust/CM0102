// ---------- %%LEVA%%a leva: o save do calendário: gravar e carregar as datas dos clubes e gravar a conferência das pernas (fix_man.cpp) ----------

unsafe fn cpy7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy7_clubes() -> u32 {
    *(0xae235c as *const u32)
}

/// Abre `nome` em CM3_TEMP (0x90bde0: cdecl(pasta, nome); fopen 0x945b00: cdecl(caminho, modo)).
unsafe fn open_temp_file(nome: usize, modo: usize) -> usize {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    abrir(caminho(0x987da0, nome), modo)
}

unsafe fn cpy7_fechar(f: usize) {
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    fechar(f);
}

/// { 0x5af040 } (cdecl, fix_man.cpp): `fix_man_save_club_dates()`, do save do calendário (FIXTURE_MANAGER::save
/// 0x5ac060, 0x5ac394): sem a tabela das datas dos clubes ([0xae2a5c]), a caixa da linha 0x1011. Grava em
/// CM3_TEMP\club_dates.tmp ("wb") uma palavra por clube ([0xae235c] delas, fwrite 0x945ce6); gravação curta: a caixa
/// da linha 0x101b; fecha. Sem abrir: nada.
unsafe extern "C" fn fix_man_save_club_dates() {
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    if cpy7_d(0xae2a5c) == 0 {
        return show_error_at(0x9cc208, 0x1011);
    }
    let f = open_temp_file(0x9cc248, 0x9a7f28);
    if f == 0 {
        return;
    }
    crate::comparacao::ramo("cpy_7 gravar datas dos clubes");
    if escrever(cpy7_d(0xae2a5c), 2, cpy7_clubes(), f) != cpy7_clubes() { // cpy7: as datas dos clubes
        show_error_at(0x9cc208, 0x101b);
    }
    cpy7_fechar(f);
}

/// Comparador da 0x5af040: só a original.
unsafe extern "C" fn comparar_fix_man_save_club_dates() {
    // ponytail: grava o arquivo do save (não pode rodar duas vezes); a cópia é provada pela prova do save (o .qst
    // gravado no modo rust com e sem a leva, item a item: club_dates.tmp)
    original_fn::<unsafe extern "C" fn()>(%%I0%%)()
}

/// { 0x5aee90 } (cdecl, fix_man.cpp): `fix_man_load_club_dates()`, do construtor do calendário (0x5a9cf0, 0x5a9e38:
/// na carreira nova e na carga do save). Com a tabela já alocada ([0xae2a5c]), a caixa da linha 0xfe4. Aloca uma
/// palavra por clube (malloc 0x944e46 de 2 × [0xae235c]; sem memória, a caixa da linha 0xfec); lê de
/// CM3_TEMP\club_dates.tmp ("rb", fread 0x945bfe; leitura curta: a caixa da linha 0xff6) e fecha; sem o arquivo (a
/// carreira nova), zera a tabela.
unsafe extern "C" fn fix_man_load_club_dates() {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    if cpy7_d(0xae2a5c) != 0 {
        return show_error_at(0x9cc208, 0xfe4);
    }
    let t = alocar(cpy7_clubes().wrapping_mul(2));
    *(0xae2a5c as *mut usize) = t;
    if t == 0 {
        return show_error_at(0x9cc208, 0xfec);
    }
    let f = open_temp_file(0x9cc248, 0x98f928);
    if f == 0 {
        crate::comparacao::ramo("cpy_7 datas dos clubes zeradas");
        core::ptr::write_bytes(cpy7_d(0xae2a5c) as *mut u8, 0, cpy7_clubes().wrapping_mul(2) as usize);
        return;
    }
    crate::comparacao::ramo("cpy_7 datas dos clubes lidas");
    if ler(cpy7_d(0xae2a5c), 2, cpy7_clubes(), f) != cpy7_clubes() { // cpy7: a leitura das datas
        show_error_at(0x9cc208, 0xff6);
    }
    cpy7_fechar(f);
}

/// Comparador da 0x5aee90: só a original.
unsafe extern "C" fn comparar_fix_man_load_club_dates() {
    // ponytail: aloca a tabela global e lê o arquivo (não pode rodar duas vezes); a cópia é provada pela carga do
    // save de férias na prova do save (levas/N/CARGA) e, sem o arquivo, pela carreira nova
    original_fn::<unsafe extern "C" fn()>(%%I1%%)()
}

/// { 0x5af4a0 } (cdecl, fix_man.cpp): `fix_man_save_fixture_leg_checks()`, do save do calendário (0x5ac399): com a
/// lista de conferência das pernas ([0xae2a48], LLIST) vazia (a palavra da conta zero), nada. Grava em
/// CM3_TEMP\check_fixture_legs.tmp ("wb") a conta (2 bytes) e, do começo da lista (o cursor +0xa ← a cabeça +2), cada
/// item de 15 bytes (0x68d1b0; gravação curta: a caixa da linha 0x10e0, e segue); fecha. Sem abrir: nada.
unsafe extern "C" fn fix_man_save_fixture_leg_checks() {
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let conta = cpy7_d(0xae2a48);
    if conta as u16 == 0 {
        return;
    }
    let f = open_temp_file(0x9cc258, 0x9a7f28);
    if f == 0 {
        return;
    }
    crate::comparacao::ramo("cpy_7 gravar conferencia das pernas");
    escrever(&conta as *const usize as usize, 2, 1, f);
    *(0xae2a52 as *mut usize) = cpy7_d(0xae2a4a);
    loop {
        let item = proximo(0xae2a48);
        if item == 0 {
            break;
        }
        if escrever(item, 0xf, 1, f) != 1 {
            show_error_at(0x9cc208, 0x10e0);
        }
    }
    cpy7_fechar(f);
}

/// Comparador da 0x5af4a0: só a original.
unsafe extern "C" fn comparar_fix_man_save_fixture_leg_checks() {
    // ponytail: grava o arquivo do save e anda o cursor da lista; a cópia é provada pela prova do save
    original_fn::<unsafe extern "C" fn()>(%%I2%%)()
}
