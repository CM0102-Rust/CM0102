// ---------- %%LEVA%%a leva: o contêiner do save: as consultas (o padrão do nome, o id e o tamanho dos itens) (zipdir.cpp) ----------

/// { 0x933a00 } (cdecl, zipdir.cpp): `zipdir_match_filename_pattern(padrao, nome)`: 1 se o nome do
/// item casa com o padrão. "{ Empty }" (0xad6ad8, a vaga vazia da lista do desempacotar) nunca casa;
/// nome igual ao padrão, ou padrão vazio, casa; senão letra a letra sem diferenciar maiúsculas
/// (0x90c2f0, só o byte baixo). `*.` pula o nome até o último ponto; `*` seguido de outra coisa só
/// casa se for o fim do padrão. Como a original, o padrão acabar antes do nome também casa ("abc"
/// casa "abcdef").
unsafe extern "C" fn casar_nome_com_padrao(padrao: *const u8, nome: *const u8) -> u32 {
    let maiuscula: unsafe extern "C" fn(u32) -> u8 = core::mem::transmute(0x90c2f0usize);
    let achar: unsafe extern "C" fn(*const u8, i32) -> usize = core::mem::transmute(0x946540usize);
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

unsafe extern "C" fn comparar_casar_nome_com_padrao(padrao: *const u8, nome: *const u8) -> u32 {
    let r = orig::<unsafe extern "C" fn(*const u8, *const u8) -> u32>(%%I0%%)(padrao, nome);
    let n = casar_nome_com_padrao(padrao, nome);
    PLACAR.registrar(r == n, || {
        let t = |p: *const u8| String::from_utf8_lossy(core::ffi::CStr::from_ptr(p as *const core::ffi::c_char).to_bytes()).into_owned();
        format!("casar_nome_com_padrao({:?}, {:?}) {} x {}", t(padrao), t(nome), r, n)
    });
    r
}

/// O cabeçalho (12 bytes: tipo, a4, quantos) e o índice (1000 itens de 0x10c; 250 nos tipos antigos
/// 1/2, que viram 3/4) do contêiner aberto `f`, numa cópia `h` de 0x416ec bytes, como as consultas
/// do zipdir.cpp fazem (cada uma tem a sua cópia do trecho). Se o cabeçalho não vier, `h` fica
/// como estava (memória da alocação, sem zerar, como na original).
unsafe fn ler_cabecalho_avulso(h: usize, f: usize) {
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
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

/// { 0x9331c0 } (cdecl, zipdir.cpp): `zget_archive_id(caminho)`: o dword +4 do cabeçalho do
/// contêiner (0x16 `.sav`, 0x17 `.qst`), ou 0x14 se não abrir ou o tipo não for 3/4. Sem memória,
/// a caixa de erro (zipdir.cpp 592) e 0x14.
unsafe extern "C" fn id_do_conteiner(caminho: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let h = novo(0x416ec);
    if h == 0 {
        aviso_em(0xad6ab0, 0x250);
        return 0x14;
    }
    let f = abrir(caminho, 0x98f928);
    if f == 0 {
        liberar(h);
        return 0x14;
    }
    ler_cabecalho_avulso(h, f);
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
unsafe extern "C" fn comparar_id_do_conteiner(caminho: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(caminho);
    let n = id_do_conteiner(caminho);
    PLACAR.registrar(r == n, || format!("id_do_conteiner {:#x} x {:#x}", r, n));
    r
}

/// { 0x933860 } (cdecl, zipdir.cpp): `zget_matching_size(caminho, padrao)`: a soma dos tamanhos
/// (descomprimidos) dos itens cujo nome casa com o padrão (0x933a00). A soma é `int` com sinal, como
/// na original: acima de 2 GB dá negativo e a conferência de espaço do `load_game_data` passa
/// sempre (corrigir é outra decisão, como o save acima de 2 GB). Sem memória, a caixa de erro
/// (zipdir.cpp 849) e 0; contêiner que não abre ou de tipo estranho, 0.
unsafe extern "C" fn tamanho_dos_itens_do_padrao(caminho: usize, padrao: usize) -> i32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let casar: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x933a00usize);
    let h = novo(0x416ec);
    if h == 0 {
        aviso_em(0xad6ab0, 0x351);
        return 0;
    }
    let f = abrir(caminho, 0x98f928);
    if f == 0 {
        liberar(h);
        return 0;
    }
    ler_cabecalho_avulso(h, f);
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

unsafe extern "C" fn comparar_tamanho_dos_itens_do_padrao(caminho: usize, padrao: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(caminho, padrao);
    let n = tamanho_dos_itens_do_padrao(caminho, padrao);
    PLACAR.registrar(r == n, || format!("tamanho_dos_itens_do_padrao {} x {}", r, n));
    r
}
