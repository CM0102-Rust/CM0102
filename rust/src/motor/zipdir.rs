use super::*;

/// matches a container item name against a pattern with `*` (`*.` goes to the last dot), case-insensitively; the "{
/// Empty }" slot never matches
/// Original: `zipdir_match_filename_pattern` (0x933a00, zipdir.cpp, cdecl).
///
/// [pt] { 0x933a00 } (cdecl, zipdir.cpp): `zipdir_match_filename_pattern(padrao, nome)`: 1 se o nome do
/// item casa com o padrão. "{ Empty }" (0xad6ad8, a vaga vazia da lista do desempacotar) nunca casa;
/// nome igual ao padrão, ou padrão vazio, casa; senão letra a letra sem diferenciar maiúsculas
/// (0x90c2f0, só o byte baixo). `*.` pula o nome até o último ponto; `*` seguido de outra coisa só
/// casa se for o fim do padrão. Como a original, o padrão acabar antes do nome também casa ("abc"
/// casa "abcdef").
pub(crate) unsafe extern "C" fn zipdir_match_filename_pattern(padrao: *const u8, nome: *const u8) -> u32 {
    let maiuscula: unsafe extern "C" fn(u32) -> u8 = core::mem::transmute(FN_UTILS_CHAR_TO_LOWER);
    let achar: unsafe extern "C" fn(*const u8, i32) -> usize = core::mem::transmute(FN_946540);
    let texto = |p: *const u8| core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes();
    if texto(nome) == texto(0xad6ad8 as *const u8) {
        return 0;
    }
    if texto(nome) == texto(padrao) || *padrao == 0 {
        return 1;
    }
    let (mut i, mut j) = (0usize, 0usize);
    loop {
        let c = *padrao.add(i);
        if c == b'*' {
            if *padrao.add(i + 1) != b'.' {
                return (*padrao.add(i + 1) == 0) as u32;
            }
            // o último ponto do nome (strchr 0x946540 sem outro ponto depois)
            loop {
                if *nome.add(j) == b'.' && achar(nome.add(j + 1), b'.' as i32) == 0 {
                    break;
                }
                if *nome.add(j) == 0 {
                    return 0;
                }
                j += 1;
            }
            i += 1;
        } else {
            let n = maiuscula(*nome.add(j) as u32);
            if maiuscula(c as u32) != n {
                return 0;
            }
        }
        i += 1;
        j += 1;
        if *padrao.add(i) == 0 {
            return 1;
        }
    }
}


/// Reads the container's 12-byte header and index (1000 items, or 250 for old types 1/2 converted to 3/4) into the copy
/// h.
///
/// [pt] O cabeçalho (12 bytes: tipo, a4, quantos) e o índice (1000 itens de 0x10c; 250 nos tipos antigos
/// 1/2, que viram 3/4) do contêiner aberto `f`, numa cópia `h` de 0x416ec bytes, como as consultas
/// do zipdir.cpp fazem (cada uma tem a sua cópia do trecho). Se o cabeçalho não vier, `h` fica
/// como estava (memória da alocação, sem zerar, como na original).
pub(crate) unsafe fn read_header_copy(h: usize, f: usize) {
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
    if ler(h, 0xc, 1, f) == 0 {
        return;
    }
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

/// reads the container header and returns dword +4 (0x16 .sav, 0x17 .qst; 0x14 on failure)
/// Original: `zget_archive_id` (0x9331c0, zipdir.cpp, cdecl).
///
/// [pt] { 0x9331c0 } (cdecl, zipdir.cpp): `zget_archive_id(caminho)`: o dword +4 do cabeçalho do
/// contêiner (0x16 `.sav`, 0x17 `.qst`), ou 0x14 se não abrir ou o tipo não for 3/4. Sem memória,
/// a caixa de erro (zipdir.cpp 592) e 0x14.
pub(crate) unsafe extern "C" fn zget_archive_id(caminho: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let h = novo(0x416ec);
    if h == 0 {
        show_error_at(0xad6ab0, 0x250);
        return 0x14;
    }
    let f = abrir(caminho, 0x98f928);
    if f == 0 {
        liberar(h);
        return 0x14;
    }
    read_header_copy(h, f);
    fechar(f);
    let tipo = *(h as *const u32);
    if tipo != 3 && tipo != 4 {
        liberar(h);
        return 0x14;
    }
    let id = *((h + 4) as *const u32);
    liberar(h);
    id
}

// O id e o tamanho só leem o arquivo e liberam o que alocam: as duas rodam lado a lado.

/// adds up the uncompressed sizes of the items matching the pattern (signed int: negative above 2 GB, as in the
/// original)
/// Original: `zget_matching_size` (0x933860, zipdir.cpp, cdecl).
///
/// [pt] { 0x933860 } (cdecl, zipdir.cpp): `zget_matching_size(caminho, padrao)`: a soma dos tamanhos
/// (descomprimidos) dos itens cujo nome casa com o padrão (0x933a00). A soma é `int` com sinal, como
/// na original: acima de 2 GB dá negativo e a conferência de espaço do `load_game_data` passa
/// sempre (corrigir é outra decisão, como o save acima de 2 GB). Sem memória, a caixa de erro
/// (zipdir.cpp 849) e 0; contêiner que não abre ou de tipo estranho, 0.
pub(crate) unsafe extern "C" fn zget_matching_size(caminho: usize, padrao: usize) -> i32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let casar: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_ZIPDIR_MATCH_FILENAME_PATTERN);
    let h = novo(0x416ec);
    if h == 0 {
        show_error_at(0xad6ab0, 0x351);
        return 0;
    }
    let f = abrir(caminho, 0x98f928);
    if f == 0 {
        liberar(h);
        return 0;
    }
    read_header_copy(h, f);
    fechar(f);
    let tipo = *(h as *const u32);
    if tipo != 3 && tipo != 4 {
        liberar(h);
        return 0;
    }
    let mut soma = 0i32;
    let mut i = 0i32;
    while i < *((h + 8) as *const i32) {
        let item = h + 0xc + 0x10c * i as usize;
        if casar(padrao, item + 8) != 0 {
            soma = soma.wrapping_add(*((item + 4) as *const i32));
        }
        i += 1;
    }
    liberar(h);
    soma
}


/// strcpy: copies C string origem (with terminator) to destino.
///
/// [pt] strcpy: copia o texto `origem` (com o zero) para `destino`.
pub(crate) unsafe fn zipdir_c_strcpy(destino: *mut u8, origem: *const u8) {
    let t = core::ffi::CStr::from_ptr(origem as *const core::ffi::c_char).to_bytes_with_nul();
    core::ptr::copy(t.as_ptr(), destino, t.len());
}

/// strcat: appends C string origem to destino.
///
/// [pt] strcat: acrescenta o texto `origem` ao fim de `destino`.
pub(crate) unsafe fn zipdir_c_strcat(destino: *mut u8, origem: *const u8) {
    let n = core::ffi::CStr::from_ptr(destino as *const core::ffi::c_char).to_bytes().len();
    zipdir_c_strcpy(destino.add(n), origem);
}

/// each folder file matching the pattern (FindFirstFile) becomes a container item: zopen for writing, fread/zwrite in
/// 0x30d40 blocks, zclose
/// Original: `zadd_matching_files` (0x933610, zipdir.cpp, cdecl).
///
/// [pt] { 0x933610 } (cdecl, zipdir.cpp): `zadd_matching_files(caminho, pasta, padrao, a4, tipo)`: cada
/// arquivo da pasta que o `FindFirstFile` acha com o padrão (0x4c0aa0/0x4c0b50, só arquivos, na
/// ordem do sistema) vira um item do contêiner: `zopen` gravando (0x9326e0, cria o contêiner com
/// `a4`/`tipo` se não existir) + `fread` em blocos de 0x30d40 no buffer de leitura do item (+0x41705)
/// + `zwrite` (0x932e10) + `zclose` (0x932b00). 1 no fim; 0 se nada casa. Arquivo que não abre ou
/// item recusado: a caixa de erro (zipdir.cpp 797 / 805) e 0, sem fechar a busca (como a original).
pub(crate) unsafe extern "C" fn zadd_matching_files(caminho: usize, pasta: usize, padrao: usize, a4: u32, tipo: u32) -> u32 {
    let juntar_caminho: unsafe extern "C" fn(*mut u8, usize, usize, usize, usize) = core::mem::transmute(FN_CRT_MAKEPATH);
    let primeiro: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(FN_FILE_FIND_FIRST);
    let proximo: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_FILE_FIND_NEXT);
    let encerrar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_FILE_FIND_CLOSE);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> i32 = core::mem::transmute(FN_CRT_FREAD);
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let gravar: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(FN_ZWRITE);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
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
            show_error_at(0xad6ab0, 0x31d);
            return 0;
        }
        let mut z = abrir_item(caminho, achado_p + 0x10, 1, a4, tipo);
        if z == 0 {
            show_error_at(0xad6ab0, 0x325);
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

// simplification: cria itens no contêiner (e o arquivo, se não existir); não roda duas vezes. A prova do
// save compara o .qst gravado com e sem a leva.

/// each container item whose name matches the pattern becomes a folder file (zopen for reading, zread in 0x30d40
/// blocks, fwrite)
/// Original: `zextract_matching_files` (0x933320, zipdir.cpp, cdecl).
///
/// [pt] { 0x933320 } (cdecl, zipdir.cpp): `zextract_matching_files(caminho, pasta, padrao)`: copia os nomes
/// das 1000 vagas do índice (as vazias como "{ Empty }") numa lista de 0x104 por nome (malloc
/// 0x944e46); cada nome que casa com o padrão (0x933a00) e abre (`zopen` lendo, a4 0x15, tipo 4)
/// vira um arquivo da pasta ("wb"), em blocos de 0x30d40 lidos pelo `zread` (0x932c30) no buffer de
/// escrita do item (+0x72445). 1 se extraiu algum. Arquivo que não cria: 0 (fecha o item).
/// Lixo de pilha na original: nos caminhos de erro (sem memória para a lista, contêiner que não abre
/// ou de tipo estranho) ela fecha o item de uma variável local nunca escrita; aqui ela começa em 0.
/// Sem memória para o índice, a caixa de erro (zipdir.cpp 642) escreve o nome "zipdir" por cima da
/// resposta (a mesma casa da pilha) e a lista segue sem os nomes (memória da alocação).
pub(crate) unsafe extern "C" fn zextract_matching_files(caminho: usize, pasta: usize, padrao: usize) -> u32 {
    let reservar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
    let escrever: unsafe extern "C" fn(usize, u32, i32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
    let juntar_caminho: unsafe extern "C" fn(*mut u8, usize, usize, usize, usize) = core::mem::transmute(FN_CRT_MAKEPATH);
    let casar: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_ZIPDIR_MATCH_FILENAME_PATTERN);
    let abrir_item: unsafe extern "C" fn(usize, usize, u32, u32, u32) -> usize = core::mem::transmute(FN_ZOPEN);
    let ler_item: unsafe extern "C" fn(usize, i32, i32, usize) -> i32 = core::mem::transmute(FN_ZREAD);
    let fechar_item: unsafe extern "C" fn(*mut usize) = core::mem::transmute(FN_ZCLOSE);
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
        show_error_at(0xad6ab0, 0x282);
        // simplification: o nome do fonte que a 0x944cff separa cai na casa da resposta
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
            zipdir_c_strcpy((nomes + 0x104 * i) as *mut u8, origem as *const u8);
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

// simplification: cria arquivos na pasta temporária e aloca; não roda duas vezes. A carga da prova do save
// (--sav) passa por aqui; a gravação não.

/// builds the CM3_TEMP folder path in [0xde279c] and deletes its files matching the pattern, in groups of 100
/// Original: `utils_clear_temp_folder` (0x90cd50, utils.cpp, cdecl).
///
/// [pt] { 0x90cd50 } (cdecl, utils.cpp): `utils_clear_temp_folder(padrao)`: monta em [0xde279c] a pasta
/// temporária (a variável `CM3_TEMP` pela 0x90bc50 + "\", se houver, e o texto de [0xad9c64], copiado
/// com limite pela 0x90cf20) e apaga (`remove` 0x94643a) os arquivos dela que casam com o padrão,
/// em grupos de 100 nomes (a busca é fechada antes de apagar e refeita enquanto o grupo enche).
pub(crate) unsafe extern "C" fn utils_clear_temp_folder(padrao: usize) {
    let ambiente: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_UTILS_GET_FOLDER_FOR_KEY);
    let copiar_limitado: unsafe extern "C" fn(usize, *mut u8) = core::mem::transmute(FN_UTILS_COPY_PATH_LIMITED);
    let juntar_caminho: unsafe extern "C" fn(*mut u8, usize, *const u8, usize, usize) = core::mem::transmute(FN_CRT_MAKEPATH);
    let primeiro: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> i32 = core::mem::transmute(FN_FILE_FIND_FIRST);
    let proximo: unsafe extern "C" fn(*mut u8) -> i32 = core::mem::transmute(FN_FILE_FIND_NEXT);
    let encerrar: unsafe extern "C" fn(*mut u8) -> i32 = core::mem::transmute(FN_FILE_FIND_CLOSE);
    let apagar: unsafe extern "C" fn(*mut u8) -> i32 = core::mem::transmute(FN_CRT_REMOVE);
    crate::comparacao::ramo("save limpar temporarios");
    let global = 0xde279c as *mut u8;
    *global = 0;
    let variavel = ambiente(0x987da0);
    let mut sufixo = [0u8; 0x100];
    copiar_limitado(0xad9c64, sufixo.as_mut_ptr());
    if variavel != 0 && *(variavel as *const u8) != 0 {
        zipdir_c_strcpy(global, variavel as *const u8);
        zipdir_c_strcat(global, 0x9c3730 as *const u8);
    }
    zipdir_c_strcat(global, sufixo.as_ptr());
    let mut pasta = [0u8; 0x104];
    zipdir_c_strcpy(pasta.as_mut_ptr(), global);
    let mut busca = [0u8; 0x104];
    let mut achado = [0u8; 0x114];
    let mut nomes = [0u8; 0x6400];
    loop {
        juntar_caminho(busca.as_mut_ptr(), 0, pasta.as_ptr(), padrao, 0);
        let mut n = 0usize;
        if primeiro(busca.as_mut_ptr(), achado.as_mut_ptr(), 0) == 0 {
            loop {
                zipdir_c_strcpy(nomes.as_mut_ptr().add(0x100 * n), achado.as_ptr().add(0x10));
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

// simplification: apaga arquivos; não roda duas vezes. A gravação do .qst passa por aqui (create_game).

//
// O ZFILE (0xa3191 bytes, do zopen 0x9326e0): +0 o FILE da CRT, +4 modo (1 gravando), +8 posição
// lógica (bytes descomprimidos), +0xc o item, +0x10/+0x14 repetição do RLE (entregues/contagem),
// +0x18 o byte da repetição, +0x19 cabeçalho (tipo +0x19, a4 +0x1d, quantos +0x21) e índice
// (+0x25, 0x10c por item: posição, tamanho descomprimido, nome), buffer de leitura +0x41705 e de
// escrita +0x72445 (0x30d40 cada), cursores +0xa3185 (leitura), +0xa3189 (escrita), +0xa318d (lidos).
// RLE: byte <= 0x80 sai como está; > 0x80 = (byte − 0x80) cópias do byte seguinte (1 a 0x7f).

/// Reads an unaligned i32 at offset o of the ZFILE.
///
/// [pt] Lê um dword sem alinhamento no ZFILE.
pub(crate) unsafe fn zf_read_i32(z: usize, o: usize) -> i32 {
    core::ptr::read_unaligned((z + o) as *const i32)
}

/// Writes an unaligned i32 at offset o of the ZFILE.
///
/// [pt] Grava um dword sem alinhamento no ZFILE.
pub(crate) unsafe fn zf_write_i32(z: usize, o: usize, v: i32) {
    core::ptr::write_unaligned((z + o) as *mut i32, v)
}

/// Next compressed byte from the read buffer, refilling it with an fread of 0x30d40 bytes when exhausted.
///
/// [pt] O próximo byte comprimido do item: do buffer de leitura (+0x41705, cursor +0xa3185 até +0xa318d)
/// ou, acabado, de um `fread` de 0x30d40 bytes (o primeiro byte do bloco novo, mesmo se veio vazio).
pub(crate) unsafe fn zf_next_byte(z: usize) -> u8 {
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> i32 = core::mem::transmute(FN_CRT_FREAD);
    let c = zf_read_i32(z, 0xa3185);
    if c < zf_read_i32(z, 0xa318d) {
        let b = *((z + 0x41705 + c as usize) as *const u8);
        zf_write_i32(z, 0xa3185, c + 1);
        return b;
    }
    // simplification: o ramo marca o bloco, nao cada chamada (a zread roda milhoes de vezes na carga)
    crate::comparacao::ramo("save zread bloco");
    zf_write_i32(z, 0xa318d, ler(z + 0x41705, 1, 0x30d40, zf_read_i32(z, 0) as usize));
    let b = *((z + 0x41705) as *const u8);
    zf_write_i32(z, 0xa3185, 1);
    b
}

/// reads from the item like fread, clipped at the item end; on type 4 decodes the RLE (a run may span calls)
/// Original: `zread` (0x932c30, zipdir.cpp, cdecl).
///
/// [pt] { 0x932c30 } (cdecl, zipdir.cpp): `zread(buf, tam, quantos, z)` como o `fread`, dentro do item
/// aberto para leitura (senão 0): o pedido (`tam × quantos`, int) é cortado no fim do item e soma à
/// posição lógica. Tipo 3: `fread` direto de `pedido / tam` itens. Tipo 4: decodifica o RLE; uma
/// repetição pode atravessar chamadas (+0x10 de +0x14). Devolve `pedido / tam` (divisão com sinal,
/// como a original: `tam` 0 derruba as duas).
pub(crate) unsafe extern "C" fn zipdir_zread(buf: *mut u8, tam: i32, quantos: i32, z: usize) -> i32 {
    let ler: unsafe extern "C" fn(*mut u8, i32, i32, usize) -> i32 = core::mem::transmute(FN_CRT_FREAD);
    if z == 0 || zf_read_i32(z, 4) != 0 {
        return 0;
    }
    let tamanho_do_item = zf_read_i32(z, 0x29 + 0x10c * zf_read_i32(z, 0xc) as usize);
    let mut pedido = tam.wrapping_mul(quantos);
    let pos = zf_read_i32(z, 8);
    if pos > tamanho_do_item.wrapping_sub(pedido) {
        pedido = tamanho_do_item.wrapping_sub(pos);
        if pedido <= 0 {
            return 0;
        }
    }
    let fim = zf_read_i32(z, 8).wrapping_add(pedido);
    zf_write_i32(z, 8, fim);
    if zf_read_i32(z, 0x19) != 4 {
        crate::comparacao::ramo("save zread tipo 3");
        let q = pedido / tam;
        zf_write_i32(z, 8, tam.wrapping_mul(q).wrapping_sub(pedido).wrapping_add(fim));
        return ler(buf, tam, q, zf_read_i32(z, 0) as usize);
    }
    let mut k = 0i32;
    if pedido > 0 {
        'fora: loop {
            // o resto da repetição pendente
            let n = zf_read_i32(z, 0x14);
            if n != 0 && zf_read_i32(z, 0x10) < n {
                loop {
                    if k >= pedido {
                        break 'fora;
                    }
                    *buf.add(k as usize) = *((z + 0x18) as *const u8);
                    k += 1;
                    let e = zf_read_i32(z, 0x10) + 1;
                    zf_write_i32(z, 0x10, e);
                    if e >= zf_read_i32(z, 0x14) {
                        break;
                    }
                }
            }
            if k >= pedido {
                break;
            }
            zf_write_i32(z, 0x14, 0);
            zf_write_i32(z, 0x10, 0);
            loop {
                let b = zf_next_byte(z);
                if b <= 0x80 {
                    *buf.add(k as usize) = b;
                    k += 1;
                } else {
                    zf_write_i32(z, 0x14, b as i32 - 0x80);
                    *((z + 0x18) as *mut u8) = zf_next_byte(z);
                }
                if zf_read_i32(z, 0x14) == 0 {
                    if k >= pedido {
                        break 'fora;
                    }
                    continue;
                }
                if k < pedido {
                    continue 'fora;
                }
                break 'fora;
            }
        }
    }
    pedido / tam
}

// simplification: lê do arquivo e anda o cursor do FILE e do item; não roda duas vezes. A prova do save
// (carga, --sav) e a gravação (o zseek relê pela zread) passam por aqui.

/// Puts a compressed byte in the write buffer, flushing it with fwrite when full.
///
/// [pt] Um byte comprimido para o buffer de escrita (+0x72445, cursor +0xa3189); cheio, grava o buffer
/// inteiro (`fwrite`) e o byte vai para o começo.
pub(crate) unsafe fn zf_put_byte(z: usize, b: u8) {
    let escrever: unsafe extern "C" fn(usize, u32, i32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
    let p = zf_read_i32(z, 0xa3189);
    if p < 0x30d40 {
        *((z + 0x72445 + p as usize) as *mut u8) = b;
        zf_write_i32(z, 0xa3189, p + 1);
        return;
    }
    // simplification: o ramo marca o bloco, nao cada chamada (a zwrite roda milhoes de vezes no save)
    crate::comparacao::ramo("save zwrite bloco");
    escrever(z + 0x72445, 1, p, zf_read_i32(z, 0) as usize);
    zf_write_i32(z, 0xa3189, 1);
    *((z + 0x72445) as *mut u8) = b;
}

/// writes to the item like fwrite and adds to the size in the index; on type 4 encodes the RLE (runs of up to 0x7f kept
/// across calls)
/// Original: `zwrite` (0x932e10, zipdir.cpp, cdecl).
///
/// [pt] { 0x932e10 } (cdecl, zipdir.cpp): `zwrite(buf, tam, quantos, z)` como o `fwrite`, no item aberto
/// para gravar (senão 0): soma `tam × quantos` ao tamanho do item no índice e à posição lógica.
/// Tipo 3: `fwrite` direto (devolve o que ele devolver). Tipo 4: codifica o RLE byte a byte,
/// guardando a repetição em +0x18/+0x14 entre as chamadas; byte diferente fecha a repetição (1 com
/// byte <= 0x80 sai literal, senão o par 0x80 + n, byte); a 0x7f sai o par e recomeça. Devolve `quantos`.
pub(crate) unsafe extern "C" fn zipdir_zwrite(buf: *const u8, tam: i32, quantos: i32, z: usize) -> i32 {
    let escrever: unsafe extern "C" fn(*const u8, i32, i32, usize) -> i32 = core::mem::transmute(FN_CRT_FWRITE);
    if z == 0 || zf_read_i32(z, 4) != 1 {
        return 0;
    }
    let item = 0x29 + 0x10c * zf_read_i32(z, 0xc) as usize;
    let total = tam.wrapping_mul(quantos);
    zf_write_i32(z, item, zf_read_i32(z, item).wrapping_add(total));
    zf_write_i32(z, 8, zf_read_i32(z, 8).wrapping_add(total));
    if zf_read_i32(z, 0x19) != 4 {
        crate::comparacao::ramo("save zwrite tipo 3");
        return escrever(buf, tam, quantos, zf_read_i32(z, 0) as usize);
    }
    let repetido = (z + 0x18) as *mut u8;
    let mut k = 0i32;
    while k < total {
        let n = zf_read_i32(z, 0x14);
        if n != 0 {
            let c = *repetido;
            if *buf.add(k as usize) == c {
                if n.wrapping_add(0x80) < 0xff {
                    zf_write_i32(z, 0x14, n + 1);
                    k += 1;
                    continue;
                }
                // 0x7f cópias: sai o par e recomeça
                crate::comparacao::ramo("save zwrite par 0x7f");
                zf_put_byte(z, (n as u8).wrapping_add(0x80));
                zf_put_byte(z, *repetido);
            } else if n <= 1 && c <= 0x80 {
                zf_put_byte(z, c);
            } else {
                let contagem = (n as u8).wrapping_add(0x80);
                zf_put_byte(z, contagem);
                zf_put_byte(z, *repetido);
            }
        }
        zf_write_i32(z, 0x14, 1);
        *repetido = *buf.add(k as usize);
        k += 1;
    }
    quantos
}

// simplification: grava no arquivo (buffer e fwrite) e muda o índice; não roda duas vezes. A prova do save
// compara o .qst tipo 4 gravado com e sem a leva.

/// when writing: flushes the buffer, closes the pending RLE run and rewrites header and index at the start; closes the
/// file and frees the item
/// Original: `zclose` (0x932b00, zipdir.cpp, cdecl).
///
/// [pt] { 0x932b00 } (cdecl, zipdir.cpp): `zclose(&z)`. Gravando: esvazia o buffer de escrita; se o item
/// tem tamanho, no tipo 4 fecha a repetição pendente direto no arquivo (literal ou par, como no
/// zwrite) e regrava cabeçalho + índice inteiros (0x416ec bytes) no começo (`fseek` 0). Lendo: zera
/// os cursores de leitura. Fecha o FILE, libera o ZFILE e zera o ponteiro.
pub(crate) unsafe extern "C" fn zclose(zz: *mut usize) {
    let escrever: unsafe extern "C" fn(usize, u32, i32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
    let buscar: unsafe extern "C" fn(usize, i32, i32) -> i32 = core::mem::transmute(FN_CRT_FSEEK);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let z = *zz;
    if z == 0 {
        return;
    }
    crate::comparacao::ramo("save zclose");
    let f = || zf_read_i32(z, 0) as usize;
    if zf_read_i32(z, 4) == 1 {
        let n = zf_read_i32(z, 0xa3189);
        if n > 0 {
            escrever(z + 0x72445, 1, n, f());
            zf_write_i32(z, 0xa3189, 0);
        }
    } else {
        zf_write_i32(z, 0xa318d, 0);
        zf_write_i32(z, 0xa3185, 0);
    }
    if zf_read_i32(z, 4) == 1 && zf_read_i32(z, 0x29 + 0x10c * zf_read_i32(z, 0xc) as usize) != 0 {
        let n = zf_read_i32(z, 0x14);
        if zf_read_i32(z, 0x19) == 4 && n != 0 {
            if n <= 1 && *((z + 0x18) as *const u8) <= 0x80 {
                escrever(z + 0x18, 1, 1, f());
            } else {
                let marca = (n as u8).wrapping_add(0x80);
                escrever(&marca as *const u8 as usize, 1, 1, f());
                escrever(z + 0x18, 1, 1, f());
            }
        }
        buscar(f(), 0, 0);
        escrever(z + 0x19, 0x416ec, 1, f());
    }
    fechar(f());
    zf_write_i32(z, 0, 0);
    liberar(z);
    *zz = 0;
}

// simplification: grava, fecha o arquivo e libera; efeito único. A prova do save compara o .qst gravado.

