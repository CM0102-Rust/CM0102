// ---------- centesima quinquagesima setima leva: a narracao no painel (0x6d7b00) ----------

/// { 0x6d7b00 } (thiscall, ret 0x20, match_events.cpp, ~4,4 KB): o narrador `nar` escreve o evento `ev`
/// da partida `ctx` no `painel` (0xb74450) e toca o som dele. Antes, o ruido da torcida do humano da
/// vez ([0xb77c7a]; canal +0x20, "fxloop2.wav") e reposto se parou. Com o codigo valido e a opcao 1
/// zerada, monta o texto `G` (+0x108) do som escolhido (o trecho +0xd entre `|`) como a 0x6d8d10, mas
/// numa linha propria e com cores: comeca nas do clube do lance (0x53b640 com o uniforme do lado),
/// `<O>` passa as do outro, `<Y>` e `<R>`/`<r>` sem `f` as padrao ([0xaebdea]/[0xaebdec] ou
/// [0xae31a8]); `<N>` e o numero, `<n>` por extenso; o nome do jogador e curto se o texto inteiro
/// cabe em menos de 300 (0x5e5ad0 com a `fonte`). Larga demais para `largura` - 16: quebra (0x90cb20)
/// ou corta numa palavra com "...". Desenha (0x801040, 0x801660, 0x801960, 0x801c70, 0x800d30,
/// 0x800580 com a opcao 3). Por fim, sem trecho (+0xd) e com volume (+0x2db0 >= 101), toca o som do
/// evento (0x8025e0 no canal pela opcao 6, lado pela opcao 5), e marca a hora e a duracao do som
/// (+0x2d6c, +0x2dac = peso x volume / 100).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn narrar_no_painel(nar: usize, ev: usize, ctx: usize, comp: i32, painel: u32, largura: i32, quebrar: u32, fonte: usize, modo: u32) -> u32 {
    let tocando: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x8029f0usize);
    let parar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x802a50usize);
    let tocar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(0x8025e0usize);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let largura_do_texto: unsafe extern "C" fn(usize, *const u8) -> i32 = core::mem::transmute(0x5e5ad0usize);
    let cores_do_clube: unsafe extern "C" fn(usize, u32, *mut u8, *mut u8) = core::mem::transmute(0x53b640usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let formatar_texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let nome_do_jogador: unsafe extern "C" fn(usize, i32, i32, *mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(0x6db2d0usize);
    let compor: unsafe extern "C" fn(usize, *const u8, *const u8, u32, u32) -> usize = core::mem::transmute(0x6701f0usize);
    let maiuscula: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(0x90c2d0usize);
    let por_extenso: unsafe extern "C" fn(i32, i32) -> usize = core::mem::transmute(0x90c310usize);
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    let nome_do_arbitro: unsafe extern "C" fn(usize, *mut u8, u32, u32) = core::mem::transmute(0x53eab0usize);
    let fase: unsafe extern "C" fn(usize, u32, u32, u32, *mut u8, u32) = core::mem::transmute(0x4bfa30usize);
    let criterio: unsafe extern "C" fn(u32, u32, u32, u32) -> u8 = core::mem::transmute(0x4bff70usize);
    let quebrar_linha: unsafe extern "C" fn(*mut u8, u32, u32) = core::mem::transmute(0x90cb20usize);
    let cor_de_fundo: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801040usize);
    let fonte_do_painel: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x801660usize);
    let cor_do_texto: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x801960usize);
    let texto_do_painel: unsafe extern "thiscall" fn(usize, u32, *const u8) = core::mem::transmute(0x801c70usize);
    let margem: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x800d30usize);
    let mostrar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x800580usize);
    let relogio: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x90c510usize);
    const SOM: usize = 0xb74450;
    const TABELA: isize = 0xb68788;

    if ev == 0 || ctx == 0 || painel as u16 == 0xffff {
        return 0;
    }
    let eu = || *(0xb77c7a as *const i16) as isize;
    let torcida = |n: usize| ((n as isize) + eu() + 0x20) as *mut u8;
    if *torcida(nar) != 0xff && tocando(SOM, *torcida(nar) as u32) == 0 {
        parar(SOM, *torcida(nar) as u32);
        *torcida(nar) = 0xff;
    }
    if *torcida(nar) == 0xff {
        let p = caminho(0x9c382c, 0xa159a0);
        tocar(SOM, torcida(nar) as usize, 2, p, 1);
    }
    let b = |o: usize| *((ev + o) as *const u8);
    let k = ((*(ev as *const u16) & 0x7fff) as i32 - 0x1f40) as i16;
    let e = (TABELA + k as isize * 18) as usize;
    let qual = || *((ev + 2) as *const i16) as isize;
    let sons = || core::ptr::read_unaligned((e + 8) as *const usize);
    let mut cor1 = [0u8; 4];
    let mut cor2 = [0u8; 4];

    let valido = (0..0x2a5).contains(&k) && *((e + 1) as *const u8) == 0;
    if valido {
        crate::comparacao::ramo("narracao no painel");
        if sons() == 0 {
            return 0;
        }
        if core::ptr::read_unaligned((sons() as isize + qual() * 0x110 + 0x108) as *const usize) == 0 {
            return 0;
        }
        // as cores do clube do lance (uniforme do lado)
        let lado = b(6);
        let mut clube_id = if lado != 0 { *((ctx + 4) as *const usize) } else { *(ctx as *const usize) };
        let uniforme = *((ctx as isize + lado as i8 as isize * 1573 + 0x152) as *const u8) as u32;
        cores_do_clube((*(0xae23b4 as *const usize)).wrapping_add(clube_id.wrapping_mul(0x245)), uniforme, cor1.as_mut_ptr(), cor2.as_mut_ptr());
        let mut s = core::ptr::read_unaligned((sons() as isize + qual() * 0x110 + 0x108) as *const usize);
        let v = b(0xd) as i8;
        if v > 0 {
            let mut resta = v;
            let mut c = *(s as *const u8);
            if c != 0 {
                loop {
                    if c == b'|' {
                        resta -= 1;
                        if resta <= 0 {
                            s += 1;
                            break;
                        }
                    }
                    c = *((s + 1) as *const u8);
                    s += 1;
                    if c == 0 {
                        break;
                    }
                }
            }
            if *(s as *const u8) == 0 {
                return 0;
            }
        }
        let t = |i: i16| *((s as isize + i as isize) as *const u8);
        let mut linha = [0u8; 0x800];
        let l = linha.as_mut_ptr() as usize;
        let dest = |o: i16| (l as isize + o as isize) as usize;
        let mut pos: i16 = 0;
        let mut out: i16 = 0;
        let prefixo = |pos: i16, p: &mut [u8; 0x100]| {
            p[0] = 0;
            if pos > 1 && (t(pos - 1) == b'}' || t(pos - 1) == b']') {
                let mut a = pos - 2;
                while a >= 0 && t(a) != b'{' && t(a) != b'[' {
                    a -= 1;
                }
                let j = a as i32;
                let fim = pos as i32 - j - 1;
                let mut c = 1i32;
                while c < fim {
                    p[(c - 1) as usize] = t((j + c) as i16);
                    c += 1;
                }
                p[(c - 1) as usize] = 0;
            }
        };
        let sufixo = |inicio: i16, x: &mut [u8; 0x100]| {
            x[0] = 0;
            let mut fe = inicio;
            while t(fe) != b'>' && t(fe) != 0 {
                fe += 1;
            }
            if t(fe + 1) == b'{' || t(fe + 1) == b'[' {
                let mut j = fe + 2;
                while t(j) != b'}' && t(j) != 0 {
                    x[(j - fe - 2) as usize] = t(j);
                    j += 1;
                }
                x[(j - fe - 2) as usize] = 0;
            }
        };
        let escrever = |out: i16, r: usize| {
            formatar(dest(out), 0x991944, r);
        };
        let em_maiusculas = |out: i16, r: usize| {
            let mut j: i16 = 0;
            while *((r as isize + j as isize) as *const u8) != 0 {
                let c = maiuscula(*((r as isize + j as isize) as *const u8) as i32);
                *(dest(out + j) as *mut u8) = c;
                j += 1;
            }
            *(dest(out + j) as *mut u8) = 0;
        };
        let mut pre = [0u8; 0x100];
        let mut suf = [0u8; 0x100];
        let mut nome = [0u8; 0xd0];
        let mut inteiro = [0u8; 0x800];
        let mut arb = [0u8; 0xd0];
        let mut texto_fase = [0u8; 0x800];
        let cores_padrao = |c1: &mut [u8; 4], c2: &mut [u8; 4], segunda: usize| {
            core::ptr::write_unaligned(c1.as_mut_ptr() as *mut u16, *(0xaebdea as *const u16));
            core::ptr::write_unaligned(c2.as_mut_ptr() as *mut u16, *(segunda as *const u16));
        };

        if t(0) != 0 {
            loop {
                let c = t(pos);
                if c == b'|' {
                    break;
                }
                if c != b'<' {
                    if c != b'{' && c != b'[' {
                        *(dest(out) as *mut u8) = c;
                        out += 1;
                    } else {
                        let mut j = pos;
                        let mut a = c;
                        loop {
                            if a == b']' || a == 0 {
                                break;
                            }
                            j += 1;
                            a = t(j);
                            if a == b'}' {
                                break;
                            }
                        }
                        pos = j;
                    }
                } else {
                    let x = t(pos + 1);
                    match x {
                        b'P' | b'p' => {
                            let (lado_j, idx) = if t(pos + 2) == b'1' && b(5) != 0xff { (b(5), b(6)) } else { (b(7), b(8)) };
                            if lado_j != 0xff {
                                // a largura do texto inteiro (ate o `|`) decide o nome curto
                                let mut n = 0usize;
                                while *((s + n) as *const u8) != 0 && *((s + n) as *const u8) != b'|' {
                                    inteiro[n] = *((s + n) as *const u8);
                                    n += 1;
                                }
                                inteiro[n] = 0;
                                let curto = (largura_do_texto(fonte, inteiro.as_ptr()) as i16) < 0x12c;
                                nome_do_jogador(ctx, lado_j as i8 as i32, idx as i8 as i32, nome.as_mut_ptr(), 0xc8, curto as u32, modo, 1);
                                sufixo(pos + 3, &mut suf);
                                let r = compor(nome.as_ptr() as usize, 0xad9c64 as *const u8, suf.as_ptr(), 0x64, 0x64);
                                if t(pos + 3) == b'C' || t(pos + 3) == b'c' {
                                    em_maiusculas(out, r);
                                } else {
                                    escrever(out, r);
                                }
                            }
                        }
                        b'T' | b't' => {
                            let dois = t(pos + 2) == b'2';
                            let casa = b(6) != 0;
                            let id = if dois == casa { *(ctx as *const usize) } else { *((ctx + 4) as *const usize) };
                            let clube = (*(0xae23b4 as *const usize)).wrapping_add(id.wrapping_mul(0x245));
                            prefixo(pos, &mut pre);
                            sufixo(pos + 2, &mut suf);
                            let g = *((clube + 0x52) as *const u8) as u32;
                            let r = compor(clube + 0x38, pre.as_ptr(), suf.as_ptr(), g, g);
                            if t(pos + 2) == b'C' || t(pos + 2) == b'c' {
                                em_maiusculas(out, r);
                            } else {
                                escrever(out, r);
                            }
                        }
                        b'C' | b'c' | b'F' | b'f' => {
                            if comp >= 0 {
                                let cp = (*(0xae23d0 as *const usize)).wrapping_add((comp as usize).wrapping_mul(0x6b));
                                prefixo(pos, &mut pre);
                                sufixo(pos + 2, &mut suf);
                                let r = if x == b'C' || x == b'c' {
                                    let g = *((cp + 0x52) as *const u8) as u32;
                                    compor(cp + 0x38, pre.as_ptr(), suf.as_ptr(), g, g)
                                } else {
                                    fase(cp, 0, 0x96, 3, texto_fase.as_mut_ptr(), 0x7d0);
                                    let g = criterio(0, 0, 0x96, 3) as u32;
                                    compor(texto_fase.as_ptr() as usize, pre.as_ptr(), suf.as_ptr(), g, g)
                                };
                                escrever(out, r);
                            }
                        }
                        b'D' | b'd' | b'N' => {
                            let numero = core::ptr::read_unaligned((ev + 9) as *const i32);
                            let v = if x != b'N' && t(pos + 2) == b'1' { numero.wrapping_add(1) } else { numero };
                            formatar(dest(out), 0x98806c, v);
                        }
                        b'n' => {
                            let r = por_extenso(core::ptr::read_unaligned((ev + 9) as *const i32), -1);
                            escrever(out, r);
                        }
                        b'O' | b'o' => {
                            // as cores do outro clube
                            let lado = b(6);
                            clube_id = if lado != 0 { *(ctx as *const usize) } else { *((ctx + 4) as *const usize) };
                            let a = (lado == 0) as isize;
                            let uniforme = *((ctx as isize + a * 1573 + 0x152) as *const u8) as u32;
                            cores_do_clube((*(0xae23b4 as *const usize)).wrapping_add(clube_id.wrapping_mul(0x245)), uniforme, cor1.as_mut_ptr(), cor2.as_mut_ptr());
                            *(dest(out) as *mut u8) = 0;
                        }
                        b'Y' | b'y' => {
                            cores_padrao(&mut cor1, &mut cor2, 0xaebdec);
                            *(dest(out) as *mut u8) = 0;
                        }
                        b'R' | b'r' => {
                            if t(pos + 2) != b'f' {
                                cores_padrao(&mut cor1, &mut cor2, 0xae31a8);
                                *(dest(out) as *mut u8) = 0;
                            } else {
                                let fixo = if x == b'R' { 0xa15a14usize } else { 0xa159e0 };
                                let arbitro = core::ptr::read_unaligned((ctx + 0x40) as *const i32);
                                if arbitro <= -1 {
                                    formatar_texto(dest(out), fixo);
                                } else {
                                    let soma = (b(7) as i8 as i32)
                                        .wrapping_add(b(5) as i8 as i32)
                                        .wrapping_add(core::ptr::read_unaligned((ctx + 4) as *const i32))
                                        .wrapping_add(clube_id as i32);
                                    let resto = (soma % 5) as i8;
                                    let ficha = (*(0xae23e8 as *const usize)).wrapping_add((arbitro as usize).wrapping_mul(0x2b));
                                    let modo_nome = if idioma() == 4 && resto == 0 {
                                        1
                                    } else if resto == 0 {
                                        5
                                    } else if resto > 0 && resto <= 2 {
                                        1
                                    } else {
                                        0
                                    };
                                    match modo_nome {
                                        1 => {
                                            nome_do_arbitro(ficha, arb.as_mut_ptr(), 0xc8, 1);
                                            formatar_texto(0xb67dd8, 0xa15a20, arb.as_ptr());
                                        }
                                        5 => {
                                            nome_do_arbitro(ficha, arb.as_mut_ptr(), 0xc8, 5);
                                            formatar_texto(0xb67dd8, 0xa159ec, arb.as_ptr());
                                        }
                                        _ => {
                                            formatar_texto(0xb67dd8, fixo);
                                        }
                                    }
                                    formatar(dest(out), 0xb67dd8);
                                }
                            }
                        }
                        _ => aviso_em(0xa159b0, 0x89d),
                    }
                    if t(pos) != b'>' {
                        let mut j = pos;
                        loop {
                            if t(j) == 0 {
                                break;
                            }
                            j += 1;
                            if t(j) == b'>' {
                                break;
                            }
                        }
                        pos = j;
                    }
                    if t(pos) == 0 {
                        pos -= 1;
                    }
                    let mut n = 0usize;
                    while linha[n] != 0 {
                        n += 1;
                    }
                    out = n as i16;
                }
                pos += 1;
                if t(pos) == 0 {
                    break;
                }
            }
        }
        *(dest(out) as *mut u8) = 0;
        // larga demais: quebra ou corta numa palavra com "..."
        if largura_do_texto(fonte, linha.as_ptr()) > largura - 0x10 {
            let mut n = 0usize;
            while linha[n] != 0 {
                n += 1;
            }
            let mut si = n as i16;
            if quebrar as u8 != 0 {
                quebrar_linha(dest(si / 5) as *mut u8, 0, 1);
            } else {
                loop {
                    si -= 1;
                    *(dest(si) as *mut u8) = 0;
                    if largura_do_texto(fonte, linha.as_ptr()) <= largura - 0x10 {
                        if si > 0 {
                            loop {
                                if *(dest(si - 1) as *const u8) == b' ' {
                                    if si > 0 && *(dest(si - 1) as *const u8) == b' ' {
                                        si -= 1;
                                        *(dest(si) as *mut u8) = 0;
                                    }
                                    break;
                                }
                                si -= 1;
                                *(dest(si) as *mut u8) = 0;
                                if si <= 0 {
                                    break;
                                }
                            }
                        }
                        break;
                    }
                    if si <= 0 {
                        break;
                    }
                }
                let mut n = 0usize;
                while linha[n] != 0 {
                    n += 1;
                }
                for (i, c) in b"...\0".iter().enumerate() {
                    linha[n + i] = *c;
                }
            }
        }
        cor_de_fundo(SOM, painel, core::ptr::read_unaligned(cor2.as_ptr() as *const u32));
        fonte_do_painel(SOM, painel, fonte);
        cor_do_texto(SOM, painel, core::ptr::read_unaligned(cor1.as_ptr() as *const u32));
        texto_do_painel(SOM, painel, linha.as_ptr());
        margem(SOM, painel, 0x30);
        mostrar(SOM, painel, if *((e + 3) as *const u8) != 0 { 0x1001 } else { 1 });
    }

    // o som do evento
    *(((nar as isize) + eu() + 0x10) as *mut u8) = 0xff;
    if b(0xd) as i8 <= 0 && *((nar + 0x2db0) as *const u8) >= 0x65 {
        let som = (sons() as isize + qual() * 0x110) as usize;
        if *((som + 4) as *const u8) != 0 {
            let canal = if *((e + 6) as *const u8) == 0 { 0xb6b722usize } else { ((nar as isize) + eu() + 0x10) as usize };
            *(0xb68780 as *mut usize) = canal;
            if canal != 0 {
                *(canal as *mut u8) = 0xfe;
                let lado = match *((e + 5) as *const u8) {
                    0 => (b(6) != 0) as u32,
                    2 => (b(6) == 0) as u32,
                    _ => 2,
                };
                let p = caminho(0x9c382c, som + 4);
                tocar(SOM, *(0xb68780 as *const usize), lado, p, 0);
            }
        }
    }
    let agora = relogio();
    core::ptr::write_unaligned(((nar as isize) + eu() * 4 + 0x2d6c) as *mut u32, agora);
    let peso = *((sons() as isize + qual() * 0x110 + 2) as *const i16) as i32;
    let volume = *((nar + 0x2db0) as *const u8) as i32;
    core::ptr::write_unaligned((nar + 0x2dac) as *mut i32, (peso.wrapping_mul(volume) / 100) as i16 as i32);
    0
}

// ponytail: desenha no painel e toca som; sem comparacao propria (so roda com a tela da partida).
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_narrar_no_painel(nar: usize, ev: usize, ctx: usize, comp: i32, painel: u32, largura: i32, quebrar: u32, fonte: usize, modo: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, i32, u32, usize, u32) -> u32>(345)(nar, ev, ctx, comp, painel, largura, quebrar, fonte, modo)
}
