// ---------- %%LEVA%%a leva: o save: as ligações com o banco (gravar e carregar) e liberar os dados do jogo (Database.cpp, Setup.cpp) ----------

/// { 0x524d30 } (cdecl, Database.cpp): `save_database_for_game(save, a4, tipo)`: o passo 1 do
/// `save_game_data`: `save_database(save, 1, a4, tipo)` (0x52dad0, para o contêiner), devolvendo a
/// resposta dela. DESLIGADA (DESLIGADAS no motor.rs): o save_database grava no index.dat 408 bytes de
/// locais sem valor (lixo de ponteiro, a partir de +0x5cb); chamado desta cópia ele roda na pilha
/// particular e o lixo gravado muda (prova do save da 219a). Volta quando a cópia reproduzir o resíduo.
unsafe extern "C" fn save_database_for_game(save: usize, a4: u32, tipo: u32) -> u32 {
    let gravar_banco: unsafe extern "C" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(0x52dad0usize);
    crate::comparacao::ramo("save gravar banco");
    gravar_banco(save, 1, a4, tipo)
}

// ponytail: grava as tabelas do banco no contêiner; não roda duas vezes. Só a original; a cópia
// leva o ramo e é provada pela prova do save.
unsafe extern "C" fn comparar_save_database_for_game(save: usize, a4: u32, tipo: u32) -> u32 {
    orig::<unsafe extern "C" fn(usize, u32, u32) -> u32>(%%I0%%)(save, a4, tipo)
}

/// { 0x524d10 } (cdecl, Database.cpp): `load_database_from_stream(save, compartilhado)`: o passo 2
/// do `load_game_data`: `load_database(save, compartilhado, 1)` (0x528540, do contêiner), devolvendo
/// a resposta dela (1 = certo).
unsafe extern "C" fn load_database_from_stream(save: usize, compartilhado: u32) -> u32 {
    let carregar_banco: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x528540usize);
    crate::comparacao::ramo("save carregar banco");
    carregar_banco(save, compartilhado, 1)
}

// ponytail: carrega o banco inteiro do contêiner (aloca as tabelas); não roda duas vezes. Só a
// original; a cópia leva o ramo (a etapa de carregar da prova do save a exercita).
unsafe extern "C" fn comparar_load_database_from_stream(save: usize, compartilhado: u32) -> u32 {
    orig::<unsafe extern "C" fn(usize, u32) -> u32>(%%I1%%)(save, compartilhado)
}

/// { 0x82ff40 } (cdecl, Setup.cpp): `free_game_data(tela)`: o gerente de telas (0xb74450) solta as
/// telas (0x7ff5f0) e, se `tela` ≠ 0, mais 0x7fc5e0; 0x6d5b50; depois cada subsistema guardado num
/// global, na ordem da original: destrutor (thiscall) + `delete` (0x944c94) e o global zerado; os
/// vetores soltos pelo `free` (0x9452ca); dois objetos pelo destrutor virtual (vtable[0](1)); os
/// `*.tmp` da pasta temporária (0x90cd50), [0xae2338], as tabelas do banco (0x528080), a data do
/// jogo (0x54a130: 1, 0, 2001, −1) e três marcas zeradas. Devolve 1.
unsafe extern "C" fn free_game_data(tela: u32) -> u32 {
    let soltar_telas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7ff5f0usize);
    let soltar_mais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x7fc5e0usize);
    let antes: unsafe extern "C" fn() = core::mem::transmute(0x6d5b50usize);
    let competicoes: unsafe extern "C" fn() = core::mem::transmute(0x669290usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let livre: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let limpar_temporarios: unsafe extern "C" fn(usize) = core::mem::transmute(0x90cd50usize);
    let liberar_tabelas: unsafe extern "C" fn() = core::mem::transmute(0x528080usize);
    let acertar_data: unsafe extern "thiscall" fn(usize, u32, u32, u32, i32) -> usize = core::mem::transmute(0x54a130usize);
    crate::comparacao::ramo("save liberar dados do jogo");
    // destrutor + delete + global zerado
    let objetos = |lista: &[(usize, usize)]| {
        for &(g, destrutor) in lista {
            let p = *(g as *const usize);
            if p != 0 {
                let d: unsafe extern "thiscall" fn(usize) = core::mem::transmute(destrutor);
                d(p);
                apagar(p);
                *(g as *mut usize) = 0;
            }
        }
    };
    // free + global zerado
    let soltos = |lista: &[usize]| {
        for &g in lista {
            let p = *(g as *const usize);
            if p != 0 {
                livre(p);
                *(g as *mut usize) = 0;
            }
        }
    };
    // destrutor virtual (vtable[0], com 1 = apagar) + global zerado
    let virtual_ = |g: usize| {
        let p = *(g as *const usize);
        if p != 0 {
            let d: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(**(p as *const *const usize));
            d(p, 1);
            *(g as *mut usize) = 0;
        }
    };
    soltar_telas(0xb74450);
    if tela != 0 {
        soltar_mais(0xb74450);
    }
    antes();
    objetos(&[(0xb67a50, 0x6b4190)]);
    let p = *(0xadadfc as *const usize);
    if p != 0 {
        livre(p);
        *(0xadadfc as *mut usize) = 0;
        *(0xadadf8 as *mut usize) = 0;
    }
    competicoes();
    objetos(&[(0xb6b7c8, 0x72b200), (0xae2a60, 0x5af6a0), (0xb67a44, 0x68e0e0), (0xad9da8, 0x442e20),
              (0xad9dac, 0x4448a0), (0xde1d08, 0x8961e0), (0xae28f0, 0x555790), (0xde1f44, 0x8b1070),
              (0xad9c60, 0x413410), (0xde1f54, 0x8bbe20), (0xb63d28, 0x62a8a0), (0xae2a3c, 0x5a5120),
              (0xad9c6c, 0x4185f0), (0xae2a30, 0x5698d0), (0xb7433c, 0x794ba0), (0xdd7ef4, 0x85b7f0)]);
    virtual_(0xae2c74);
    objetos(&[(0xb74318, 0x76a710), (0xde2f70, 0x90d2e0), (0xde1f58, 0x903570), (0xae1000, 0x5a8f60),
              (0xdd7f00, 0x8687c0), (0xb6b7c0, 0x7293c0), (0xb74350, 0x7c0d00), (0xb5e134, 0x58e330),
              (0xae2a58, 0x5aa0d0), (0xb63c98, 0x5fb730), (0xb74448, 0x7f6aa0), (0xb7431c, 0x785d00)]);
    // [0xdd7ef0]: só o delete, sem destrutor
    let p = *(0xdd7ef0 as *const usize);
    if p != 0 {
        apagar(p);
        *(0xdd7ef0 as *mut usize) = 0;
    }
    virtual_(0xb74434);
    objetos(&[(0xae1050, 0x4c74f0), (0xae2a38, 0x59a5a0)]);
    soltos(&[0xae2d0c, 0xae2d14, 0xae2d1c, 0xae2d20, 0xae2d24]);
    objetos(&[(0xb7434c, 0x7a56f0), (0xb74340, 0x79b800), (0xdd7ee4, 0x84be10), (0xde2f74, 0x9227d0),
              (0xb74430, 0x7da690)]);
    limpar_temporarios(0xa813ac);
    soltos(&[0xae2338]);
    liberar_tabelas();
    acertar_data(0xae2c90, 1, 0, 0x7d1, -1);
    *(0xae2c88 as *mut u8) = 0;
    *(0xdd711c as *mut u32) = 0;
    *(0xdd7130 as *mut u32) = 0;
    1
}

// ponytail: destrói todos os subsistemas e apaga os *.tmp; não roda duas vezes. Só a original; a
// cópia leva o ramo e roda no modo Rust (abertura da carreira, fim do início rápido).
unsafe extern "C" fn comparar_free_game_data(tela: u32) -> u32 {
    orig::<unsafe extern "C" fn(u32) -> u32>(%%I2%%)(tela)
}
