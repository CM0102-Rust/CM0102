// ---------- centesima quinquagesima quinta leva: o leitor dos eventos da partida (0x6d9e50) ----------

/// Como `aviso_em`, e o jogo para (como a original: 0x5cc7d0, 0x5a8f60 e sai com -1).
unsafe fn erro_em(arquivo: usize, linha: u32) -> ! {
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    aviso_em(arquivo, linha);
    fechar1();
    fechar2();
    sair(-1)
}

/// { 0x6d9e50 } (match_events.cpp, ~5,2 KB): le o `events_<idioma>.cfg` (CM3_DATA; sem ele, o
/// `events_eng.cfg`, e sem este o jogo para, erro 0xcc4) para a tabela dos 677 codigos de evento em
/// [0xb68788] (0x12 bytes: 7 bytes de opcoes, +8 os sons de 0x110 bytes, +0xc quantos, +0xe o peso).
/// Com outro idioma desde a ultima leitura, 0x6d5b50 limpa antes; ja lida ([0xb6b723]), nada.
/// Linhas: `=id,o0..o6` abre um evento (o4 entre 0 e 11; o2 sem o0 avisa 0xd40), `>peso,tipo,nome`
/// acrescenta um som (com `-verifysounds` na linha de comando, confere o arquivo em CM3_SOUNDS),
/// `G`/`R` os dois textos do ultimo som (alocados; marcas `<X` desconhecidas avisam). No fim, os
/// pesos viram (maior x 10) / peso e +0xe a soma; evento sem som ganha o som padrao (0xb68670).
/// Le pelo `fgetc` do jogo e usa o buffer [0xde1f64], como a original.
unsafe extern "C" fn ler_eventos_cfg() {
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let limpar: unsafe extern "C" fn() = core::mem::transmute(0x6d5b50usize);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let ler: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x9469e0usize);
    let numero: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x947311usize);
    let numero_curto: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x94739cusize);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let procurar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945a60usize);
    let existe: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x90c120usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let formatar_texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    const FONTE: usize = 0xa159b0; // match_events.cpp
    const TABELA: usize = 0xb68788;
    const FIM: usize = 0xb6b723;
    const BUF: usize = 0xde1f64;
    let evento = |k: i32| TABELA.wrapping_add((k as isize * 0x12) as usize);
    let sons = |e: usize| core::ptr::read_unaligned((e + 8) as *const usize);
    let quantos = |e: usize| core::ptr::read_unaligned((e + 0xc) as *const i16);
    let peso = |e: usize| (e + 0xe) as *mut i32;

    if *(0xa1599c as *const i32) != idioma() {
        limpar();
    }
    if *(FIM as *const u8) != 0 {
        return;
    }
    let mut e = TABELA;
    while e < FIM {
        *(e as *mut u8) = 0xff;
        for o in 1..=7 {
            *((e + o) as *mut u8) = 0;
        }
        core::ptr::write_unaligned((e + 8) as *mut u32, 0);
        core::ptr::write_unaligned((e + 0xc) as *mut u16, 0);
        core::ptr::write_unaligned(peso(e), 0);
        e += 0x12;
    }
    let lingua = idioma();
    *(0xa1599c as *mut i32) = lingua;
    let nome = match lingua {
        2 => 0xa15cec,
        3 => 0xa15cdc,
        4 => 0xa15ccc,
        5 => 0xa15cbc,
        6 => 0xa15c6c,
        7 => 0xa15cac,
        8 => 0xa15c9c,
        9 => 0xa15c8c,
        10 => 0xa15c7c,
        11 => 0xa15c5c,
        12 => 0xa15c4c,
        _ => 0,
    };
    let mut f = if nome != 0 { abrir(caminho(0x9b5348, nome), 0x9899fc) } else { 0 };
    if f != 0 {
        *(FIM as *mut u8) = 1;
    } else {
        // ponytail: a original testa [0xb6b723] de novo aqui; ele acabou de ser visto zerado
        *(FIM as *mut u8) = 1;
        f = abrir(caminho(0x9b5348, 0xa15c3c), 0x9899fc);
        if f == 0 {
            erro_em(FONTE, 0xcc4);
        }
    }

    let fim_de_arquivo = || *((f + 0xc) as *const u8) & 0x10 != 0;
    let pula_brancos_e_le = || {
        let mut c = ler(f);
        while (c == 0x20 || c == 0xd) && c != -1 {
            c = ler(f);
        }
        c
    };
    // um campo ate a virgula ou o fim da linha, no buffer; devolve o ultimo caractere lido
    let campo = || {
        let mut c = pula_brancos_e_le();
        let mut n = 0usize;
        while c != 0x2c && c != 0xa && c != -1 {
            *((BUF + n) as *mut u8) = c as u8;
            n += 1;
            c = ler(f);
        }
        *((BUF + n) as *mut u8) = 0;
        c
    };
    // a linha inteira (virgulas inclusive) num texto alocado; marcas `<X` desconhecidas avisam
    let texto = |linha_alocacao: u32, linha_marca: u32| -> usize {
        let mut c = pula_brancos_e_le();
        let mut n = 0usize;
        while c != 0xa && c != -1 {
            *((BUF + n) as *mut u8) = c as u8;
            n += 1;
            c = ler(f);
        }
        *((BUF + n) as *mut u8) = 0;
        let t = alocar(n as u32 + 1);
        if t == 0 {
            erro_em(FONTE, linha_alocacao);
        }
        core::ptr::copy_nonoverlapping(BUF as *const u8, t as *mut u8, n + 1);
        let mut i = 0usize;
        while *((t + i) as *const u8) != 0 {
            if *((t + i) as *const u8) == b'<' {
                let x = *((t + i + 1) as *const i8);
                if !b"CDFNOPRTYcdfnoprty".contains(&(x as u8)) {
                    formatar_texto(BUF, 0xa15b74, x as i32);
                    aviso_em(FONTE, linha_marca);
                }
            }
            i += 1;
        }
        t
    };

    let mut atual = -1i32;
    while !fim_de_arquivo() {
        let c = ler(f);
        if c == -1 {
            continue;
        }
        match c {
            0xd => {}
            0x3d => {
                // `=`: confere o evento anterior e abre o novo
                if atual != -1 {
                    let e = evento(atual);
                    let s = sons(e);
                    if s == 0 {
                        formatar(BUF, 0xa15b38, atual);
                        erro_em(FONTE, 0xcdf);
                    }
                    for j in 0..quantos(e).max(0) as usize {
                        if core::ptr::read_unaligned((s + 0x110 * j + 0x108) as *const usize) == 0 {
                            formatar(BUF, 0xa15af0, atual);
                            erro_em(FONTE, 0xcea);
                        }
                    }
                }
                campo();
                atual = numero(BUF);
                if !(0..=0x2a4).contains(&atual) {
                    formatar(BUF, 0xa15aa0, atual);
                    erro_em(FONTE, 0xcfb);
                }
                let e = evento(atual);
                let mut c = 0;
                for o in 0..7usize {
                    c = campo();
                    let v = numero_curto(BUF) as u8;
                    *((e + o) as *mut u8) = v;
                    if o == 4 {
                        let b = (e + 4) as *mut i8;
                        if *b < 0 {
                            *b = 0;
                        }
                        if *b > 0xb {
                            *b = 0xb;
                        }
                    }
                }
                while c != 0xa && c != -1 {
                    c = ler(f);
                }
                if *((e + 2) as *const u8) != 0 && *(e as *const u8) == 0 {
                    formatar(BUF, 0xa15bf0, atual);
                    aviso_em(FONTE, 0xd40);
                }
            }
            0x3e => {
                // `>`: mais um som no evento aberto
                if atual == -1 {
                    erro_em(FONTE, 0xd49);
                }
                let e = evento(atual);
                let novo = realocar(sons(e), (quantos(e) as i32 + 1) as u32 * 0x110);
                if novo == 0 {
                    erro_em(FONTE, 0xd50);
                }
                core::ptr::write_unaligned((e + 8) as *mut usize, novo);
                let s = novo + 0x110 * quantos(e) as usize;
                campo();
                *(s as *mut i16) = numero_curto(BUF) as i16;
                let p = (*(s as *const i16) as i32).max(*peso(e));
                core::ptr::write_unaligned(peso(e), p);
                campo();
                *((s + 2) as *mut i16) = numero_curto(BUF) as i16;
                let mut c = ler(f);
                *((s + 4) as *mut u8) = 0;
                while (c == 0x20 || c == 0xd) && c != -1 {
                    c = ler(f);
                }
                let mut n = 0usize;
                while c != 0xa && c != -1 {
                    n += 1;
                    *((s + 3 + n) as *mut u8) = c as u8;
                    c = ler(f);
                }
                *((s + 4 + n) as *mut u8) = 0;
                if procurar(0xae2c98, 0xa15be0) != 0 && n != 0 && existe(caminho(0x9c382c, s + 4)) == 0 {
                    formatar(BUF, 0xa15bac, caminho(0x9c382c, s + 4));
                    caixa(0x9870e0, BUF, 0);
                }
                core::ptr::write_unaligned((s + 0x108) as *mut usize, 0);
                core::ptr::write_unaligned((s + 0x10c) as *mut usize, 0);
                core::ptr::write_unaligned((e + 0xc) as *mut i16, quantos(e) + 1);
            }
            0x47 | 0x67 | 0x52 | 0x72 => {
                // `G`/`R`: os dois textos do ultimo som
                let (o, sem, cheio, ja, alocacao, marca) = if c == 0x47 || c == 0x67 {
                    (0x108usize, 0xd80, 0xd82, 0xd84, 0xd97, 0xdc3)
                } else {
                    (0x10c, 0xdcf, 0xdd1, 0xdd3, 0xde6, 0xe12)
                };
                if atual == -1 {
                    erro_em(FONTE, sem);
                }
                let e = evento(atual);
                if quantos(e) == 0 {
                    erro_em(FONTE, cheio);
                }
                let campo_texto = (sons(e) as isize + quantos(e) as isize * 0x110 - 0x110 + o as isize) as usize;
                if core::ptr::read_unaligned(campo_texto as *const usize) != 0 {
                    erro_em(FONTE, ja);
                }
                let t = texto(alocacao, marca);
                // a original grava o ponteiro antes de copiar; o resultado e o mesmo
                core::ptr::write_unaligned(campo_texto as *mut usize, t);
            }
            _ => {
                let mut c = c;
                while c != 0xa && c != -1 {
                    c = ler(f);
                }
            }
        }
    }

    // os pesos: (maior x 10) / peso de cada som, e +0xe a soma; evento sem som ganha o padrao
    for k in 0..0x2a5i32 {
        let e = evento(k);
        if sons(e) == 0 {
            core::ptr::write_unaligned((e + 8) as *mut usize, 0xb68670);
            core::ptr::write_unaligned((e + 0xc) as *mut i16, 1);
            formatar(BUF, 0xa15b38, k);
            aviso_em(FONTE, 0xe46);
        } else {
            let total = (*peso(e)).wrapping_mul(10);
            core::ptr::write_unaligned(peso(e), total);
            let mut soma = 0i32;
            for j in 0..quantos(e).max(0) as usize {
                let s = (sons(e) + 0x110 * j) as *mut i16;
                *s = (core::ptr::read_unaligned(peso(e)) / *s as i32) as i16;
                soma += *s as i32;
            }
            core::ptr::write_unaligned(peso(e), soma);
        }
    }
    fechar(f);
}

// ponytail: le arquivo, aloca os textos e grava a tabela global; sem comparacao propria, validada
// pela carreira (Rust x original) e pela comparacao da tabela no fim da leitura.
unsafe extern "C" fn comparar_ler_eventos_cfg() {
    orig::<unsafe extern "C" fn()>(342)()
}
