// ---------- %%LEVA%%a leva: o contêiner do save: empacotar e desempacotar os arquivos da pasta e limpar a pasta temporária (zipdir.cpp, utils.cpp) ----------

/// strcpy: copia o texto `origem` (com o zero) para `destino`.
unsafe fn copiar_texto_c(destino: *mut u8, origem: *const u8) {
    let t = core::ffi::CStr::from_ptr(origem as *const core::ffi::c_char).to_bytes_with_nul();
    core::ptr::copy(t.as_ptr(), destino, t.len());
}

/// strcat: acrescenta o texto `origem` ao fim de `destino`.
unsafe fn juntar_texto_c(destino: *mut u8, origem: *const u8) {
    let n = core::ffi::CStr::from_ptr(destino as *const core::ffi::c_char).to_bytes().len();
    copiar_texto_c(destino.add(n), origem);
}

/// { 0x933610 } (cdecl, zipdir.cpp): `zadd_matching_files(caminho, pasta, padrao, a4, tipo)`: cada
/// arquivo da pasta que o `FindFirstFile` acha com o padrão (0x4c0aa0/0x4c0b50, só arquivos, na
/// ordem do sistema) vira um item do contêiner: `zopen` gravando (0x9326e0, cria o contêiner com
/// `a4`/`tipo` se não existir) + `fread` em blocos de 0x30d40 no buffer de leitura do item (+0x41705)
/// + `zwrite` (0x932e10) + `zclose` (0x932b00). 1 no fim; 0 se nada casa. Arquivo que não abre ou
/// item recusado: a caixa de erro (zipdir.cpp 797 / 805) e 0, sem fechar a busca (como a original).
unsafe extern "C" fn empacotar_no_conteiner(caminho: usize, pasta: usize, padrao: usize, a4: u32, tipo: u32) -> u32 {
    let juntar_caminho: unsafe extern "C" fn(*mut u8, usize, usize, usize, usize) = core::mem::transmute(0x945b13usize);
    let primeiro: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x4c0aa0usize);
    let proximo: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x4c0b50usize);
    let encerrar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x4c0c10usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> i32 = core::mem::transmute(0x945bfeusize);
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let gravar: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(0x932e10usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparacao::ramo("save empacotar");
    // a busca (alça +0, nome +0x10, filtro +0x110) e o caminho montado
    let mut achado = [0u8; 0x114];
    let mut busca = [0u8; 0x104];
    let achado_p = achado.as_mut_ptr() as usize;
    juntar_caminho(busca.as_mut_ptr(), 0, pasta, padrao, 0);
    if primeiro(busca.as_mut_ptr() as usize, achado_p, 0) != 0 {
        return 0;
    }
    loop {
        juntar_caminho(busca.as_mut_ptr(), 0, pasta, achado_p + 0x10, 0);
        let f = abrir(busca.as_mut_ptr() as usize, 0x98f928);
        if f == 0 {
            aviso_em(0xad6ab0, 0x31d);
            return 0;
        }
        let mut z = abrir_item(caminho, achado_p + 0x10, 1, a4, tipo);
        if z == 0 {
            aviso_em(0xad6ab0, 0x325);
            fechar(f);
            return 0;
        }
        let mut n = ler(z + 0x41705, 1, 0x30d40, f);
        while n > 0 {
            gravar(z + 0x41705, 1, n, z);
            n = ler(z + 0x41705, 1, 0x30d40, f);
        }
        fechar(f);
        fechar_item(&mut z);
        if proximo(achado_p) != 0 {
            break;
        }
    }
    encerrar(achado_p);
    1
}

// ponytail: cria itens no contêiner (e o arquivo, se não existir); não roda duas vezes. A prova do
// save compara o .qst gravado com e sem a leva.
unsafe extern "C" fn comparar_empacotar_no_conteiner(caminho: usize, pasta: usize, padrao: usize, a4: u32, tipo: u32) -> u32 {
    orig::<unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32>(%%I0%%)(caminho, pasta, padrao, a4, tipo)
}

/// { 0x933320 } (cdecl, zipdir.cpp): `zextract_matching_files(caminho, pasta, padrao)`: copia os nomes
/// das 1000 vagas do índice (as vazias como "{ Empty }") numa lista de 0x104 por nome (malloc
/// 0x944e46); cada nome que casa com o padrão (0x933a00) e abre (`zopen` lendo, a4 0x15, tipo 4)
/// vira um arquivo da pasta ("wb"), em blocos de 0x30d40 lidos pelo `zread` (0x932c30) no buffer de
/// escrita do item (+0x72445). 1 se extraiu algum. Arquivo que não cria: 0 (fecha o item).
/// Lixo de pilha na original: nos caminhos de erro (sem memória para a lista, contêiner que não abre
/// ou de tipo estranho) ela fecha o item de uma variável local nunca escrita; aqui ela começa em 0.
/// Sem memória para o índice, a caixa de erro (zipdir.cpp 642) escreve o nome "zipdir" por cima da
/// resposta (a mesma casa da pilha) e a lista segue sem os nomes (memória da alocação).
unsafe extern "C" fn desempacotar_do_conteiner(caminho: usize, pasta: usize, padrao: usize) -> u32 {
    let reservar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    let escrever: unsafe extern "C" fn(usize, u32, i32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    let juntar_caminho: unsafe extern "C" fn(*mut u8, usize, usize, usize, usize) = core::mem::transmute(0x945b13usize);
    let casar: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x933a00usize);
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(0x9326e0usize);
    let ler_item: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(0x932c30usize);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(0x932b00usize);
    crate::comparacao::ramo("save desempacotar");
    let mut z = 0usize;
    let mut resposta = 0u32;
    let erro = |nomes: usize, z: &mut usize| {
        if nomes != 0 {
            soltar(nomes);
        }
        if *z != 0 {
            fechar_item(z);
        }
        0u32
    };
    let nomes = reservar(0x3f7a0);
    if nomes == 0 {
        return erro(0, &mut z);
    }
    let h = novo(0x416ec);
    if h == 0 {
        aviso_em(0xad6ab0, 0x282);
        // ponytail: o nome do fonte que a 0x944cff separa cai na casa da resposta
        resposta = u32::from_le_bytes(*b"zipd");
    } else {
        let f = abrir(caminho, 0x98f928);
        if f == 0 {
            liberar(h);
            return erro(nomes, &mut z);
        }
        if ler(h, 0xc, 1, f) != 0 {
            let tipo = h as *mut u32;
            let mut antigo = false;
            if *tipo == 1 {
                *tipo = 3;
                antigo = true;
            }
            if *tipo == 2 {
                *tipo = 4;
                antigo = true;
            }
            ler(h + 0xc, 0x10c, if antigo { 0xfa } else { 0x3e8 }, f);
        }
        fechar(f);
        let tipo = *(h as *const u32);
        if tipo != 3 && tipo != 4 {
            liberar(h);
            return erro(nomes, &mut z);
        }
        for i in 0..0x3e8usize {
            let origem = if (i as i32) < *((h + 8) as *const i32) { h + 0x14 + 0x10c * i } else { 0xad6ad8 };
            copiar_texto_c((nomes + 0x104 * i) as *mut u8, origem as *const u8);
        }
        liberar(h);
    }
    let mut destino = [0u8; 0x104];
    for i in 0..0x3e8usize {
        let nome = nomes + 0x104 * i;
        if casar(padrao, nome) == 0 {
            continue;
        }
        z = abrir_item(caminho, nome, 0, 0x15, 4);
        if z == 0 {
            continue;
        }
        resposta = 1;
        juntar_caminho(destino.as_mut_ptr(), 0, pasta, nome, 0);
        let g = abrir(destino.as_mut_ptr() as usize, 0x9a7f28);
        if g == 0 {
            return erro(nomes, &mut z);
        }
        let mut n = ler_item(z + 0x72445, 1, 0x30d40, z);
        while n > 0 {
            escrever(z + 0x72445, 1, n, g);
            n = ler_item(z + 0x72445, 1, 0x30d40, z);
        }
        fechar(g);
        fechar_item(&mut z);
    }
    soltar(nomes);
    resposta
}

// ponytail: cria arquivos na pasta temporária e aloca; não roda duas vezes. A carga da prova do save
// (--sav) passa por aqui; a gravação não.
unsafe extern "C" fn comparar_desempacotar_do_conteiner(caminho: usize, pasta: usize, padrao: usize) -> u32 {
    orig::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I1%%)(caminho, pasta, padrao)
}

/// { 0x90cd50 } (cdecl, utils.cpp): `utils_clear_temp_folder(padrao)`: monta em [0xde279c] a pasta
/// temporária (a variável `CM3_TEMP` pela 0x90bc50 + "\", se houver, e o texto de [0xad9c64], copiado
/// com limite pela 0x90cf20) e apaga (`remove` 0x94643a) os arquivos dela que casam com o padrão,
/// em grupos de 100 nomes (a busca é fechada antes de apagar e refeita enquanto o grupo enche).
unsafe extern "C" fn limpar_pasta_temporaria(padrao: usize) {
    let ambiente: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x90bc50usize);
    let copiar_limitado: unsafe extern "C" fn(usize, *mut u8) = core::mem::transmute(0x90cf20usize);
    let juntar_caminho: unsafe extern "C" fn(*mut u8, usize, *const u8, usize, usize) = core::mem::transmute(0x945b13usize);
    let primeiro: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> i32 = core::mem::transmute(0x4c0aa0usize);
    let proximo: unsafe extern "C" fn(*mut u8) -> i32 = core::mem::transmute(0x4c0b50usize);
    let encerrar: unsafe extern "C" fn(*mut u8) -> i32 = core::mem::transmute(0x4c0c10usize);
    let apagar: unsafe extern "C" fn(*mut u8) -> i32 = core::mem::transmute(0x94643ausize);
    crate::comparacao::ramo("save limpar temporarios");
    let global = 0xde279c as *mut u8;
    *global = 0;
    let variavel = ambiente(0x987da0);
    let mut sufixo = [0u8; 0x100];
    copiar_limitado(0xad9c64, sufixo.as_mut_ptr());
    if variavel != 0 && *(variavel as *const u8) != 0 {
        copiar_texto_c(global, variavel as *const u8);
        juntar_texto_c(global, 0x9c3730 as *const u8);
    }
    juntar_texto_c(global, sufixo.as_ptr());
    let mut pasta = [0u8; 0x104];
    copiar_texto_c(pasta.as_mut_ptr(), global);
    let mut busca = [0u8; 0x104];
    let mut achado = [0u8; 0x114];
    let mut nomes = [0u8; 0x6400];
    loop {
        juntar_caminho(busca.as_mut_ptr(), 0, pasta.as_ptr(), padrao, 0);
        let mut n = 0usize;
        if primeiro(busca.as_mut_ptr(), achado.as_mut_ptr(), 0) == 0 {
            loop {
                copiar_texto_c(nomes.as_mut_ptr().add(0x100 * n), achado.as_ptr().add(0x10));
                n += 1;
                if n == 100 || proximo(achado.as_mut_ptr()) != 0 {
                    break;
                }
            }
        }
        encerrar(achado.as_mut_ptr());
        for k in 0..n {
            juntar_caminho(busca.as_mut_ptr(), 0, pasta.as_ptr(), nomes.as_ptr().add(0x100 * k) as usize, 0);
            apagar(busca.as_mut_ptr());
        }
        if n != 100 {
            break;
        }
    }
}

// ponytail: apaga arquivos; não roda duas vezes. A gravação do .qst passa por aqui (create_game).
unsafe extern "C" fn comparar_limpar_pasta_temporaria(padrao: usize) {
    orig::<unsafe extern "C" fn(usize)>(%%I2%%)(padrao)
}
