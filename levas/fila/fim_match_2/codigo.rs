// ---------- %%LEVA%%a leva: o fim das estatísticas da partida: o construtor (com a conferência do CD) e o destrutor (match_stats.cpp) ----------

/// { 0x729000 } (thiscall, ret 8, match_stats.cpp): o construtor das estatísticas da partida.
/// Confere o CD: entre as unidades (GetLogicalDriveStringsA, 200) a primeira de CD (GetDriveTypeA 5)
/// com "<raiz>cm0102.exe" (0x4c0aa0; nas outras fecha a busca, 0x4c0c10); sem ela, a caixa "Insert
/// CD" e tenta de novo, para sempre. Com `zip` e a carga do save (0x72aa80) pronto; senão o índice de
/// `n` entradas (+4, [this] com n × 12 bytes; sem memória, erro 0x62 e o jogo fecha), +0x1c = n, os
/// quatro cursores dos baldes (+8: k × n / 4) e o "match_stats_matches.tmp" (CM3_TEMP, "wb") com `n`
/// registros zerados de 0xd7a bytes (erro 0x71 sem abrir, 0x7e se a escrita falha), +0x18 = 0. O
/// registro local nasce e morre pelos iteradores de vetor do runtime (0x9463af/0x9462bb) como na
/// original. Devolve `this`.
unsafe extern "thiscall" fn montar_estatisticas_da_partida(estat: usize, zip: usize, n: i32) -> usize {
    crate::comparacao::ramo("estatisticas montar");
    let vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let fim_do_vetor: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    let unidades: unsafe extern "system" fn(u32, *mut u8) -> u32 = core::mem::transmute(*(0x9670f8 as *const usize));
    let tipo: unsafe extern "system" fn(usize) -> u32 = core::mem::transmute(*(0x967100 as *const usize));
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let procurar: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x4c0aa0usize);
    let fechar_busca: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x4c0c10usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let carregar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x72aa80usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let d = |o: usize| (estat + o) as *mut i32;

    let mut registro = [0u8; 0xd7a];
    let r = registro.as_mut_ptr() as usize;
    vetor(r + 0x11f, 0x625, 2, 0x453c30, 0x453c10);

    // o exe de trabalho (cm0102-1080) vem sem a exigência do CD: o jne do tipo da unidade (0x729089)
    // virou nop e o test do achado (0x7290cb, 85 c0) virou xor (31 c0 no cm0102-1080; 33 c0 é a outra
    // codificação); a cópia lê os bytes do próprio exe
    let qualquer_unidade = *(0x729089 as *const u8) == 0x90;
    let sempre_achou = *(0x7290cb as *const u8) != 0x85; // o original tem `test eax, eax` (85 c0); qualquer outra coisa é patch sem CD (xor 31 c0 ou 33 c0)
    'cd: loop {
        // ponytail: o buffer nasce zerado; a original só leria lixo com mais de 200 bytes de unidades
        // (mais de 50 letras), impossível
        let mut lista = [0u8; 0x100];
        if unidades(200, lista.as_mut_ptr()) != 0 && lista[0] != 0 {
            let mut i = 0usize;
            loop {
                let raiz = lista.as_ptr() as usize + i;
                if qualquer_unidade || tipo(raiz) == 5 {
                    let mut nome = [0u8; 0x104];
                    let mut achado = [0u8; 0x114];
                    let nm = nome.as_mut_ptr() as usize;
                    formatar(nm, 0x987e00, raiz, 0x63u32, 0x6du32, 0x30u32, 0x31u32, 0x30u32, 0x32u32, 0x2eu32, 0x65u32, 0x78u32, 0x65u32);
                    let ach = achado.as_mut_ptr() as usize;
                    if procurar(nm, ach, 0) == 0 || sempre_achou {
                        break 'cd;
                    }
                    fechar_busca(ach);
                }
                // ponytail: a original anda com um i8 (passos de 4): só passaria de 127 com mais de 32 unidades
                i += 4;
                if lista[i] == 0 {
                    break;
                }
            }
        }
        let mut titulo = [0u8; 0x100];
        let mut msg = [0u8; 0x100];
        texto(titulo.as_mut_ptr() as usize, 0x987df4);
        texto(msg.as_mut_ptr() as usize, 0x987dc4);
        caixa(titulo.as_ptr() as usize, msg.as_ptr() as usize, 1);
    }

    if zip != 0 && carregar(estat, zip) != 0 {
        fim_do_vetor(r + 0x11f, 0x625, 2, 0x453c10);
        return estat;
    }
    *d(4) = n;
    let t = novo((n as u32).wrapping_mul(12));
    *(estat as *mut usize) = t;
    if t == 0 {
        erro_em(0xa17ba0, 0x62);
    }
    *d(0x1c) = n;
    let mut soma = 0i32;
    for k in 0..4 {
        *d(8 + 4 * k) = soma / 4;
        soma = soma.wrapping_add(n);
    }
    let f = abrir(caminho(0x987da0, 0xa17b88), 0x9a7f28);
    if f == 0 {
        aviso_em(0xa17ba0, 0x71);
    } else {
        core::ptr::write_bytes(r as *mut u8, 0, 0xd7a);
        let mut ok = true;
        let mut i = 0;
        while i < n {
            if escrever(r, 0xd7a, 1, f) != 1 {
                aviso_em(0xa17ba0, 0x7e);
                fechar(f);
                ok = false;
                break;
            }
            i += 1;
        }
        if ok {
            fechar(f);
            *d(0x18) = 0;
        }
    }
    fim_do_vetor(r + 0x11f, 0x625, 2, 0x453c10);
    estat
}

/// { 0x7293c0 } (thiscall, match_stats.cpp): o destrutor das estatísticas da partida: libera o índice
/// ([this]) e zera o ponteiro e a quantidade (+4).
unsafe extern "thiscall" fn desmontar_estatisticas_da_partida(estat: usize) {
    crate::comparacao::ramo("estatisticas desmontar");
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let t = *(estat as *const usize);
    if t != 0 {
        liberar(t);
        *(estat as *mut usize) = 0;
        *((estat + 4) as *mut u32) = 0;
    }
}

// ponytail: o construtor confere o CD (caixa que espera o clique), aloca o índice e recria o arquivo
// temporário; o destrutor libera o índice: rodar duas vezes não dá, então os comparadores só chamam a
// original; as cópias se provam no modo Rust (os ramos: a carreira automática cria o jogo e passa pelo
// construtor).
unsafe extern "thiscall" fn comparar_montar_estatisticas_da_partida(estat: usize, zip: usize, n: i32) -> usize {
    orig::<unsafe extern "thiscall" fn(usize, usize, i32) -> usize>(%%I0%%)(estat, zip, n)
}

unsafe extern "thiscall" fn comparar_desmontar_estatisticas_da_partida(estat: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(estat)
}
