// ---------- 273a leva: a classificação da UEFA: os coeficientes das nações, as vagas de cada nação e o n-ésimo clube da nação (uefa_seeding.cpp) ----------



/// A entrada `i` da tabela das nações da classificação (registros de 0x12 bytes a partir do objeto: +0 id da nação,
/// +4 a nação, +8 o coeficiente das 5 temporadas (float), +0xc vagas na Liga dos Campeões, +0xd na Copa da UEFA,
/// +0xe na Intertoto, +0xf os pontos da temporada (short), +0x11 os clubes da nação nas copas (char)).
fn cqi1_e(this: usize, i: i32) -> usize {
    this.wrapping_add((i * 0x12) as usize)
}

/// { 0x903b50 } (thiscall, ret 0, uefa_seeding.cpp, `UefaSeeding::roll_coefficients`): na virada do ano do jogo
/// (0x9035a0), para cada nação da tabela (+0x438, char): fora de 2001, as 5 temporadas da nação (+0xe0..+0x108, double)
/// andam uma casa e a última vira os pontos da temporada / os clubes nas copas (0 sem clubes); em todo ano, o
/// coeficiente (+8, float) = a soma das 5 (na ordem +0x100, +0xf8, +0xf0, +0xe8, +0xe0, em double, arredondada para
/// float) e os pontos e os clubes zeram. Depois ordena a tabela pelo coeficiente (qsort 0x945333 com 0x905cb0, do
/// maior para o menor) e grava a posição de cada nação no +0x1df da seleção dela (0x53b3d0).
unsafe extern "thiscall" fn uefa_rodar_coeficientes(this: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let conta = || *((this + 0x438) as *const i8);
    let ano = *(0xae2c92 as *const i16);
    let mut i = 0i8;
    while i < conta() {
        let e = cqi1_e(this, i as i32);
        let p = read_u32(e + 4) as usize;
        if ano != 0x7d1 {
            for k in 0..10usize {
                core::ptr::write_unaligned((p + 0xe0 + k * 4) as *mut u32, read_u32(p + 0xe8 + k * 4));
            }
            let clubes = *((e + 0x11) as *const i8);
            if clubes != 0 {
                let pontos = core::ptr::read_unaligned((e + 0xf) as *const i16) as i32;
                core::ptr::write_unaligned((p + 0x108) as *mut f64, pontos as f64 / clubes as i32 as f64);
            } else {
                core::ptr::write_unaligned((p + 0x108) as *mut u64, 0);
            }
        }
        crate::comparacao::ramo("cqi_1 soma dos coeficientes");
        let soma = read_f64(p + 0x100) + read_f64(p + 0xf8) + read_f64(p + 0xf0) + read_f64(p + 0xe8) + read_f64(p + 0xe0); // cqi1: as 5 temporadas
        core::ptr::write_unaligned((e + 8) as *mut f32, soma as f32);
        *((e + 0x11) as *mut u8) = 0;
        core::ptr::write_unaligned((e + 0xf) as *mut u16, 0);
        i += 1;
    }
    ordenar(this, conta() as i32 as u32, 0x12, 0x905cb0);
    let mut i = 0i8;
    while i < conta() {
        let s = selecao(read_u32(cqi1_e(this, i as i32) + 4) as usize);
        *((s + 0x1df) as *mut i8) = i;
        i += 1;
    }
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn cqi1_dif(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// A tabela das nações e, de cada nação, as 5 temporadas (+0xe0..+0x110) e o +0x1df da seleção.
unsafe fn cqi1_regioes(this: usize) -> Vec<(usize, usize)> {
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let n = (*((this + 0x438) as *const i8)).max(0) as i32;
    let mut reg = vec![(this, 0x43b)];
    for i in 0..n {
        let p = read_u32(cqi1_e(this, i) + 4) as usize;
        reg.push((p + 0xe0, 0x30));
        reg.push((selecao(p) + 0x1df, 1));
    }
    reg
}

/// Lado a lado: a tabela das nações, as temporadas de cada nação e a posição gravada nas seleções.
unsafe extern "thiscall" fn comparar_uefa_rodar_coeficientes(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1658);
    let reg = cqi1_regioes(this);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || uefa_rodar_coeficientes(this));
    PLACAR.registrar(igual, || format!("uefa_rodar_coeficientes: diferenças em {:x?}", cqi1_dif(&mo, &mr)));
}

/// { 0x903cc0 } (thiscall, ret 0, uefa_seeding.cpp, `UefaSeeding::set_place_counts`): as vagas de cada nação pela
/// posição `i` na tabela já ordenada (0x903b50). Liga dos Campeões (+0xc): 1ª a 3ª 4 vagas, 4ª a 6ª 3, 7ª a 15ª 2, 16ª
/// a 48ª 1, o resto 0 (Liechtenstein antes zerado, mas a faixa grava por cima). Copa da UEFA (+0xd): Liechtenstein e
/// San Marino 0, e por cima 1ª a 6ª 3, 7ª e 8ª 4, 9ª a 15ª 2, 16ª a 21ª 3, 22ª a 48ª 2, 49ª e 50ª 1 (da 51ª em diante
/// fica o que estava). Intertoto (+0xe): Moldávia, San Marino, Andorra, Bósnia e Liechtenstein 0; Escócia 1; em 2001
/// França 4, Alemanha 3, Portugal, Noruega, Espanha e Rússia 0, Itália e Portugal 1; fora de 2001 Inglaterra 1; as
/// outras, na ordem da tabela, 2 vagas até a 16ª delas (e a Bélgica em 2000), 1 até a 43ª, depois 0. O prêmio do fair
/// play: três nações ganham uma vaga a mais na Copa da UEFA; em 2001 a Finlândia, a Eslováquia e a Bielorrússia (sem
/// uma delas na tabela, a caixa da linha 0x285 e o sorteio); fora de 2001, três sorteadas (0x90cfc0 com o número de
/// nações, de três em três) distintas e fora a Itália.
unsafe extern "thiscall" fn uefa_vagas_por_nacao(this: usize) {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let g = |a: usize| read_u32(a);
    let conta = || *((this + 0x438) as *const i8);
    let ano = *(0xae2c92 as *const i16);
    let (mut fa, mut fb, mut fc, mut cont) = (-1i8, -1i8, -1i8, 0i8);
    let mut i = 0i8;
    while i < conta() {
        let e = cqi1_e(this, i as i32);
        let id = read_u32(e);
        if id == g(0x9cf2fc) {
            fa = i;
        } else if id == g(0x9cf470) {
            fb = i;
        } else if id == g(0x9cf240) {
            fc = i;
        }
        if id == g(0x9cf3a0) {
            *((e + 0xc) as *mut u8) = 0;
        }
        *((e + 0xc) as *mut u8) = if i < 3 {
            4
        } else if i < 6 {
            3
        } else if i < 0xf {
            2
        } else if i < 0x30 {
            1
        } else {
            0
        };
        let id = read_u32(e);
        if id == g(0x9cf3a0) || id == g(0x9cf450) {
            *((e + 0xd) as *mut u8) = 0;
        }
        let uefa = if i < 6 {
            Some(3)
        } else if i < 8 {
            Some(4)
        } else if i < 0xf {
            Some(2)
        } else if i < 0x15 {
            Some(3)
        } else if i < 0x30 {
            Some(2)
        } else if i < 0x32 {
            Some(1)
        } else {
            None
        };
        if let Some(v) = uefa {
            *((e + 0xd) as *mut u8) = v;
        }
        let id = read_u32(e);
        let intertoto = if id == g(0x9cf3d4) || id == g(0x9cf450) || id == g(0x9cf20c) || id == g(0x9cf258) || id == g(0x9cf3a0) {
            0
        } else if id == g(0x9cf45c) {
            1
        } else if id == g(0x9cf300) && ano == 0x7d1 {
            crate::comparacao::ramo("cqi_1 intertoto da franca");
            4 // cqi1: a França na Intertoto em 2001
        } else if id == g(0x9cf314) && ano == 0x7d1 {
            3
        } else if (id == g(0x9cf434) || id == g(0x9cf408) || id == g(0x9cf488) || id == g(0x9cf448)) && ano == 0x7d1 {
            0
        } else if id == g(0x9cf2e4) && ano != 0x7d1 {
            1
        } else if (id == g(0x9cf360) || id == g(0x9cf434)) && ano == 0x7d1 {
            1
        } else if cont < 0x10 || (id == g(0x9cf244) && ano == 0x7d0) {
            cont += 1;
            2
        } else if cont < 0x2b {
            cont += 1;
            1
        } else {
            cont += 1;
            0
        };
        *((e + 0xe) as *mut u8) = intertoto;
        i += 1;
    }
    let italia = |k: i8| read_u32(cqi1_e(this, k as i32)) == g(0x9cf360);
    if ano == 0x7d1 {
        if fa == -1 || fb == -1 || fc == -1 {
            aviso_em(0xaa1a1c, 0x285);
        }
    } else {
        fa = -1;
    }
    while fa == -1 || fb == -1 || fc == -1 || fa == fb || fa == fc || fb == fc || italia(fa) || italia(fb) || italia(fc) {
        fa = sorteio(conta() as i32) as u8 as i8;
        fb = sorteio(conta() as i32) as u8 as i8;
        fc = sorteio(conta() as i32) as u8 as i8;
    }
    for k in [fa, fb, fc] {
        let p = cqi1_e(this, k as i32) + 0xd;
        *(p as *mut u8) = (*(p as *const u8)).wrapping_add(1);
    }
}

/// Lado a lado: a tabela das nações, o sorteio (a cópia sem contar) e a marca da caixa.
unsafe extern "thiscall" fn comparar_uefa_vagas_por_nacao(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1659);
    let mut reg = vec![(this, 0x43b), (0xb67a34, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| uefa_vagas_por_nacao(this)));
    PLACAR.registrar(igual, || format!("uefa_vagas_por_nacao: diferenças em {:x?}", cqi1_dif(&mo, &mr)));
}

/// { 0x9050d0 } (thiscall, ret 0x18, uefa_seeding.cpp, `uefa_find_nth_unplaced_club_of_nation`; o this não é lido):
/// na tabela dos clubes europeus (registros de 12 bytes ordenados pela nação, 0x904040: +0 o clube, +4 o id da nação,
/// +8, +9, +0xb), a busca binária (bsearch 0x946464 com 0x905ce0; a chave só com a nação e +8 = 0xff, então só a nação
/// conta) acha um clube da nação; recua até o primeiro da nação; anda até o `n`-ésimo (char, 1 = o primeiro) que conta,
/// pulando, se pedido, os que já têm vaga nas copas europeias (+0x1db ≠ −1) e os de técnico humano (0x600a50(clube, 0,
/// 1, 0, 0)). O último registro da tabela nunca é examinado pela volta: se ela chega nele com a conta zerada, ele é o
/// escolhido sem os testes. Devolve o clube se for da nação, senão 0.
unsafe extern "thiscall" fn uefa_enesimo_clube_da_nacao(_this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
    let busca: unsafe extern "C" fn(*const u8, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let k = conta as u16 as i16 as i32;
    let ultimo = clubes.wrapping_add((k * 12 - 12) as usize);
    let mut chave = [0u8; 12];
    chave[4..8].copy_from_slice(&nacao.to_le_bytes());
    chave[8] = 0xff;
    let mut p = busca(chave.as_ptr(), clubes, k, 0xc, 0x905ce0);
    if p == 0 {
        return 0;
    }
    while p > clubes && read_u32(p - 8) == nacao {
        p -= 12;
    }
    let mut falta = (n as u8).wrapping_sub(1);
    let mut achou = false;
    if p < ultimo {
        loop {
            let c = read_u32(p) as usize;
            if sem_vaga != 0 && c != 0 && read_u32(c + 0x1db) != u32::MAX {
                crate::comparacao::ramo("cqi_1 ja com vaga");
                p += 12; // cqi1: pula o clube que já tem vaga
            } else if sem_humano != 0 && c != 0 && humano(c, 0, 1, 0, 0) != 0 {
                p += 12;
            } else {
                if falta == 0 {
                    achou = true;
                    break;
                }
                p += 12;
                falta = falta.wrapping_sub(1);
            }
            if p >= ultimo {
                break;
            }
        }
    }
    if !achou && falta != 0 {
        return 0;
    }
    if read_u32(p + 4) == nacao {
        return read_u32(p) as usize;
    }
    0
}

/// A devolução das duas (a rotina só lê).
unsafe extern "thiscall" fn comparar_uefa_enesimo_clube_da_nacao(this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize>(1660);
    let r = o(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    let c = uefa_enesimo_clube_da_nacao(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    PLACAR.registrar(r == c, || format!("uefa_enesimo_clube_da_nacao(nação {:x}, n {}, {} {}): original {:x} rust {:x}", nacao, n as u8, sem_vaga, sem_humano, r, c));
    r
}

// ---------- 273a leva: a classificação da UEFA: a tabela dos clubes europeus, as vagas da Liga dos Campeões e da Intertoto (uefa_seeding.cpp) ----------



unsafe fn cqi2_mais_um(a: usize) {
    *(a as *mut u8) = read_u8(a).wrapping_add(1);
}

/// A entrada `i` da tabela das nações da classificação (registros de 0x12 bytes; +0 id da nação, +0xe vagas na
/// Intertoto, +0x11 clubes da nação nas copas).
fn cqi2_e(this: usize, i: i32) -> usize {
    this.wrapping_add((i * 0x12) as usize)
}

/// O clube `c` ganha a vaga na competição `comp`, na fase `fase`, se tem nação e ainda não tem vaga (+0x1db = −1):
/// +0x1db = comp, +0x1df = fase e um clube a mais da nação `i` (+0x11). Devolve se ganhou. (O caso da nação sem entrada,
/// −1, que a original procura pela nação do clube, não acontece: `i` vem sempre da volta pelas nações.)
unsafe fn cqi2_vaga(this: usize, c: usize, comp: u32, fase: u8, i: i32) -> bool {
    if c == 0 || read_u32(c + 0x53) == 0 || read_u32(c + 0x1db) != u32::MAX {
        return false;
    }
    *((c + 0x1df) as *mut u8) = fase;
    *((c + 0x1db) as *mut u32) = comp;
    cqi2_mais_um(cqi2_e(this, i) + 0x11);
    true
}

/// { 0x904040 } (thiscall, ret 4, uefa_seeding.cpp, `UefaSeeding::collect_european_clubs`): a tabela dos clubes
/// europeus: aloca 12 bytes por clube do jogo (0x944e46; sem memória, a caixa da linha 0x2ed e devolve 0) e grava o
/// ponteiro em `*saida`; para cada clube (registros de 0x245 em [0xae23b4], [0xae235c] clubes) com nação, que não é
/// seleção (0x53b900) e cuja nação é da Europa (o continente +0x71 com o id de [0x9cfa10]): tira a vaga europeia (+0x1db
/// = −1 e o dword da linha do clube em [0xae2d14], 6 bytes por clube, = −1) e grava o registro (+0 o clube, +4 o id da
/// nação, +8 a reputação +0x80 / 500, +9 o +0x69 da divisão +0x5b [inferência: o nível da divisão], +0xb o +0x5f [inferência: a posição nela]; sem divisão, −1 e 0x7f;
/// com a linha de uma competição sem o +0x11c, a linha e o +0x5f do clube zeram). Ordena pela nação (qsort 0x945333 com
/// 0x905ce0) e, em 2001, aplica a ordem do euro.cfg (0x9059e0, original). Devolve o número de clubes (só os 16 bits
/// baixos contam: quem chama lê o si).
unsafe extern "thiscall" fn uefa_juntar_clubes_europeus(this: usize, saida: *mut usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let euro_cfg: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x9059e0usize);
    let t = alocar((read_u32(0xae235c) as i32).wrapping_mul(12) as u32);
    *saida = t;
    if t == 0 {
        aviso_em(0xaa1a1c, 0x2ed);
        return 0;
    }
    let mut k = 0u32;
    let mut c = read_u32(0xae23b4) as usize;
    let mut i = 0u32;
    if read_u32(0xae235c) as i32 > 0 {
        loop {
            let nacao = read_u32(c + 0x53) as usize;
            if nacao != 0 && selecao(c) == 0 {
                let cont = read_u32(nacao + 0x71) as usize;
                if cont != 0 && read_u32(cont) == read_u32(0x9cfa10) {
                    *((c + 0x1db) as *mut u32) = u32::MAX;
                    core::ptr::write_unaligned((read_u32(0xae2d14) as usize).wrapping_add((read_u32(c) as usize).wrapping_mul(6)) as *mut u32, u32::MAX);
                    let r = t.wrapping_add((k as u16 as i16 as i32 * 12) as usize);
                    core::ptr::write_unaligned(r as *mut u32, c as u32);
                    core::ptr::write_unaligned((r + 4) as *mut u32, read_u32(nacao));
                    *((r + 8) as *mut u8) = (core::ptr::read_unaligned((c + 0x80) as *const i16) as i32 / 500) as u8;
                    let linha = read_u32(c + 0x5b) as usize;
                    if linha != 0 {
                        crate::comparacao::ramo("cqi_2 nivel da divisao");
                        core::ptr::write_unaligned((r + 9) as *mut u16, core::ptr::read_unaligned((linha + 0x69) as *const u16)); // cqi2: o nível da divisão
                        *((r + 0xb) as *mut u8) = read_u8(c + 0x5f);
                        let comp = read_u32(read_u32(c + 0x5b) as usize + 0x5d) as usize;
                        if comp == 0 || read_u8(comp + 0x11c) == 0 {
                            core::ptr::write_unaligned((c + 0x5b) as *mut u32, 0);
                            *((c + 0x5f) as *mut u8) = 0;
                        }
                    } else {
                        core::ptr::write_unaligned((r + 9) as *mut u16, 0xffff);
                        *((r + 0xb) as *mut u8) = 0x7f;
                    }
                    k += 1;
                }
            }
            c += 0x245;
            i += 1;
            if (i as u16 as i16 as i32) >= read_u32(0xae235c) as i32 {
                break;
            }
        }
    }
    ordenar(t, k as u16 as i16 as i32, 0xc, 0x905ce0);
    if core::ptr::read_unaligned((this + 0x439) as *const i16) == 0x7d1 {
        euro_cfg(this, t, k);
    }
    // ponytail: a original devolve o ax com os bytes altos do eax que sobraram (quem chama lê só o si); a cópia, zero
    k as u16 as u32
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn cqi2_dif(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Os clubes do jogo, a linha de cada clube em [0xae2d14], a marca da caixa e o histórico das competições ([0xae23d4],\n/// [0xae237c] linhas de 0x1a: a 0x4afd80 o reordena, e com linhas empatadas a ordem de antes muda o clube achado).
unsafe fn cqi2_regioes_dos_clubes() -> Vec<(usize, usize)> {
    let n = (read_u32(0xae235c) as i32).max(0) as usize;
    vec![(read_u32(0xae23b4) as usize, n * 0x245), (read_u32(0xae2d14) as usize, n * 6), (0xb67a34, 4), (read_u32(0xae23d4) as usize, (read_u32(0xae237c) as i32).max(0) as usize * 0x1a)]
}

/// Lado a lado: a original com a saída de quem chama, a cópia com uma saída própria (liberada depois); os clubes, a
/// linha de cada um em [0xae2d14], a devolução e os registros das duas tabelas.
unsafe extern "thiscall" fn comparar_uefa_juntar_clubes_europeus(this: usize, saida: *mut usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut usize) -> u32>(1661);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut t2 = 0usize;
    let reg = cqi2_regioes_dos_clubes();
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, saida), || uefa_juntar_clubes_europeus(this, &mut t2));
    let (n, m) = (a as u16 as i16, b as u16 as i16);
    let t1 = *saida;
    let tabela = n == m && (n <= 0 || t1 == 0 || t2 == 0 || core::slice::from_raw_parts(t1 as *const u8, n as usize * 12) == core::slice::from_raw_parts(t2 as *const u8, n as usize * 12));
    if t2 != 0 {
        soltar(t2);
    }
    PLACAR.registrar(igual && tabela, || format!("uefa_juntar_clubes_europeus: {} e {} clubes, tabela {}, diferenças em {:x?}", n, m, tabela, cqi2_dif(&mo, &mr)));
    a
}

/// { 0x904210 } (thiscall, ret 8, uefa_seeding.cpp, `UefaSeeding::assign_champions_league_places`): as vagas da Liga dos
/// Campeões ([0x9cf6e8]). O campeão da última edição (0x4afd80 com a linha da competição em [0xae23d0], 0x6b bytes, e o
/// lugar 0) vai direto à fase de grupos (fase 1; a fase de grupos fica com 15 vagas); sem ele, a caixa da linha 0x35d.
/// Depois, nação por nação na ordem do coeficiente (a posição `r`), as vagas por fase (1 = grupos, 2 = terceira
/// preliminar, 3 = segunda, 4 = primeira) pelas faixas fixas: 1ª a 3ª nação 2 nos grupos e 2 na terceira preliminar; 4ª
/// a 6ª 2 e 1; 7ª a 9ª 1 e 1; 10ª 1 nos grupos e 1 na segunda; 11ª a 15ª 1 na terceira e 1 na segunda; 16ª 1 na
/// terceira; 17ª a 29ª 1 na segunda; da 30ª em diante 1 na primeira. Para k = 1..7, o k-ésimo clube da nação
/// (0x9050d0, sem pular ninguém) vai à primeira fase com vaga pedida; fase cheia (as capacidades 16/15, 18, 19 e 18) passa
/// uma vaga para a fase seguinte; clube sem nação ou já com vaga perde a vaga.
unsafe extern "thiscall" fn uefa_vagas_liga_dos_campeoes(this: usize, clubes: usize, conta: u32) {
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x9050d0usize);
    let lc = read_u32(0x9cf6e8);
    let mut cap: [u8; 5] = [0, 0x10, 0x12, 0x13, 0x12];
    let h = campeao((read_u32(0xae23d0) as usize).wrapping_add(lc.wrapping_mul(0x6b) as usize), 0);
    if h != 0 && read_u32(h + 0x53) != 0 && read_u32(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = lc;
        let id = read_u32(read_u32(h + 0x53) as usize);
        let n = *((this + 0x438) as *const i8) as i32;
        let mut i = -1i32;
        for k in 0..n {
            if read_u32(cqi2_e(this, k)) == id {
                i = k;
                break;
            }
        }
        cqi2_mais_um(cqi2_e(this, i) + 0x11);
        cap[1] = 0xf;
    } else {
        aviso_em(0xaa1a1c, 0x35d);
    }
    let mut r = 0i8;
    while r < *((this + 0x438) as *const i8) {
        let (faixa, ultima): ([u8; 4], u8) = if r < 3 {
            crate::comparacao::ramo("cqi_2 duas nos grupos");
            ([0, 2, 2, 0], 0) // cqi2: 1ª a 3ª nação
        } else if r < 6 {
            ([0, 2, 1, 0], 0)
        } else if r < 9 {
            ([0, 1, 1, 0], 0)
        } else if r < 0xa {
            ([0, 1, 0, 1], 0)
        } else if r < 0xf {
            ([0, 0, 1, 1], 0)
        } else if r < 0x10 {
            ([0, 0, 1, 0], 0)
        } else if r < 0x1d {
            ([0, 0, 0, 1], 0)
        } else {
            ([0, 0, 0, 0], 1)
        };
        let mut vagas = [faixa[0], faixa[1], faixa[2], faixa[3], ultima];
        let e = cqi2_e(this, r as i32);
        let mut k = 1u8;
        while k < 8 {
            let c = enesimo(this, clubes, conta, read_u32(e), k as u32, 0, 0);
            let mut f = 1usize;
            while f < 5 {
                if vagas[f] != 0 {
                    if cap[f] != 0 {
                        break;
                    }
                    crate::comparacao::ramo("cqi_2 fase cheia");
                    if f + 1 < 5 {
                        vagas[f + 1] = vagas[f + 1].wrapping_add(1); // cqi2: a vaga passa para a fase seguinte
                    }
                    vagas[f] = vagas[f].wrapping_sub(1);
                }
                f += 1;
            }
            if f < 5 {
                if cqi2_vaga(this, c, lc, f as u8, r as i32) {
                    cap[f] = cap[f].wrapping_sub(1);
                }
                vagas[f] = vagas[f].wrapping_sub(1);
            }
            k += 1;
        }
        r += 1;
    }
}

/// Lado a lado: a tabela das nações, os clubes e a marca da caixa.
unsafe extern "thiscall" fn comparar_uefa_vagas_liga_dos_campeoes(this: usize, clubes: usize, conta: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1662);
    let mut reg = cqi2_regioes_dos_clubes();
    reg.push((this, 0x440));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clubes, conta), || uefa_vagas_liga_dos_campeoes(this, clubes, conta));
    PLACAR.registrar(igual, || format!("uefa_vagas_liga_dos_campeoes: diferenças em {:x?}", cqi2_dif(&mo, &mr)));
}

/// { 0x904e30 } (thiscall, ret 8, uefa_seeding.cpp, `UefaSeeding::assign_intertoto_places`): as vagas da Intertoto
/// ([0x9cf6f8]) até 60 clubes: dá voltas pelas nações na ordem do coeficiente (a conta vai a `n` e volta a 0, e a fase
/// sai da conta antes de voltar: abaixo de 8 a fase 1, de 8 a 11 a 2, de 12 em diante a 3). A Inglaterra: um clube (o
/// voluntário da 0x903890; sem ele, o primeiro sem vaga e sem humano, 0x9050d0) e, com 2 vagas (+0xe), o segundo, na
/// fase seguinte; depois solta a lista dos voluntários (+0x43b, +0x43f). As outras: tantos clubes quanto as vagas +0xe,
/// o primeiro sem vaga de cada vez (humanos podem), na fase + a ordem do clube, enquanto houver menos de 60.
unsafe extern "thiscall" fn uefa_vagas_intertoto(this: usize, clubes: usize, conta: u32) {
    let voluntario: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x903890usize);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x9050d0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let it = read_u32(0x9cf6f8);
    let mut colocados = 0i32;
    let mut cl = 0u8;
    loop {
        let fase: u8 = if (cl as i8) < 8 { 1 } else if (cl as i8) >= 0xc { 3 } else { 2 };
        let mut i = cl;
        if cl == read_u8(this + 0x438) {
            i = 0;
        }
        let e = cqi2_e(this, i as i8 as i32);
        if read_u32(e) == read_u32(0x9cf2e4) {
            let mut c = voluntario(this);
            if c == 0 {
                c = enesimo(this, clubes, conta, read_u32(e), 1, 1, 1);
            }
            if cqi2_vaga(this, c, it, fase, i as i8 as i32) {
                colocados += 1;
            }
            if read_u8(e + 0xe) == 2 {
                let mut c = voluntario(this);
                if c == 0 {
                    c = enesimo(this, clubes, conta, read_u32(e), 1, 1, 1);
                }
                if cqi2_vaga(this, c, it, fase.wrapping_add(1), i as i8 as i32) {
                    colocados += 1;
                }
            }
            let lista = read_u32(this + 0x43b) as usize;
            if lista != 0 {
                soltar(lista);
                core::ptr::write_unaligned((this + 0x43b) as *mut u32, 0);
                *((this + 0x43f) as *mut u8) = 0;
            }
        } else if (read_u8(e + 0xe) as i8) > 0 {
            let mut j = 0i8;
            loop {
                if colocados as i16 >= 0x3c {
                    break;
                }
                let c = enesimo(this, clubes, conta, read_u32(e), 1, 1, 0);
                if cqi2_vaga(this, c, it, (j as u8).wrapping_add(fase), i as i8 as i32) {
                    crate::comparacao::ramo("cqi_2 clube na intertoto");
                    colocados += 1; // cqi2: mais um na Intertoto
                }
                j += 1;
                if j >= read_u8(e + 0xe) as i8 {
                    break;
                }
            }
        }
        cl = i.wrapping_add(1);
        if colocados as i16 >= 0x3c {
            break;
        }
    }
}

/// Lado a lado em 2001 (a 0x903890 devolve 0 sem efeito) e sem a lista dos voluntários (soltá-la duas vezes não dá):
/// a tabela das nações, os clubes e o sorteio (a cópia sem contar). Fora disso, só a original (a 0x903890 sorteia e
/// entrega notícia).
unsafe extern "thiscall" fn comparar_uefa_vagas_intertoto(this: usize, clubes: usize, conta: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1663);
    if *(0xae2c92 as *const i16) != 0x7d1 || read_u32(this + 0x43b) != 0 {
        return o(this, clubes, conta);
    }
    let mut reg = cqi2_regioes_dos_clubes();
    reg.push((this, 0x440));
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clubes, conta), || crate::aleatorio::sem_contar(|| uefa_vagas_intertoto(this, clubes, conta)));
    PLACAR.registrar(igual, || format!("uefa_vagas_intertoto: diferenças em {:x?}", cqi2_dif(&mo, &mr)));
}

// ---------- 273a leva: a classificação da UEFA: as vagas da Copa da UEFA, o voluntário inglês da Intertoto e a virada do ano (uefa_seeding.cpp) ----------




/// A entrada `i` da tabela das nações da classificação (registros de 0x12 bytes; +0 id da nação, +4 a nação, +0x11
/// clubes da nação nas copas).
fn cqi3_e(this: usize, i: i32) -> usize {
    this.wrapping_add((i * 0x12) as usize)
}

/// O clube `c` (com nação e ainda sem vaga) entra na Copa da UEFA na fase `fase`; um clube a mais da nação `r`. Devolve
/// se entrou. (O caso da nação −1, que a original procura pela nação do clube, não acontece: `r` vem da volta.)
unsafe fn cqi3_entra(this: usize, c: usize, fase: u8, r: i32, colocados: &mut i32) -> bool {
    if read_u32(c + 0x53) == 0 || read_u32(c + 0x1db) != u32::MAX {
        return false;
    }
    *((c + 0x1db) as *mut u32) = read_u32(0x9cf6f0);
    *((c + 0x1df) as *mut u8) = fase;
    *colocados += 1;
    let p = cqi3_e(this, r) + 0x11;
    *(p as *mut u8) = read_u8(p).wrapping_add(1);
    true
}

/// { 0x904580 } (thiscall, ret 8, uefa_seeding.cpp, `UefaSeeding::assign_uefa_cup_places`): as vagas da Copa da UEFA
/// ([0x9cf6f0]). O campeão da última edição (0x4afd80 com a linha da competição e o lugar 0) na fase 1, e a nação dele
/// com uma vaga a menos na fase 1. Três nações sorteadas (0x90cfc0 com o número de nações; a 2ª e a 3ª de novo até as
/// três serem distintas) ganham uma vaga a mais na preliminar (o fair play). Por posição da nação no coeficiente, as
/// vagas na fase 1 e na preliminar (fase 2): 1ª a 6ª 3 e 0, 7ª e 8ª 4 e 0, 9ª a 15ª 1 e 1, 16ª e 17ª 1 e 2, 18ª a 21ª 0
/// e 3, 22ª a 48ª 0 e 2, da 49ª em diante 0 e 1. Os clubes de cada nação, na ordem: 1º o campeão da copa nacional (a
/// copa da nação em 0xb63d7e + linha × 0x48, linha = 0x669340), ou o vice se o campeão já tem vaga (a Bélgica e o vice
/// sem nação: o seguinte da tabela), e, com os dois já com vaga, a final guardada em +0x440 (50 registros: copa,
/// campeão, vice) quando a copa da temporada ainda não foi disputada (o ano +0x40 da copa ≠ o da nação em 0xb63d7b;
/// fora Espanha e Bélgica); 2º o campeão da copa da liga (0xb63d82), se ainda sem vaga; os outros, o primeiro sem vaga da
/// tabela (0x9050d0(.., 1, 1, 0)); sem copa nacional na linha, a caixa da linha 0x418 e a vaga é tentada de novo.
/// Com a final guardada e os dois finalistas: uma vaga a mais para quem dos dois ainda não tem (os dois sem vaga entram
/// na fase 10 × a conta + 3; os dois com vaga, o primeiro sem vaga da tabela). Para quando os colocados menos a conta
/// passam de 116.
unsafe extern "thiscall" fn uefa_vagas_copa_da_uefa(this: usize, clubes: usize, conta: u32) {
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x9050d0usize);
    let linha: unsafe extern "C" fn(usize) -> i8 = core::mem::transmute(0x669340usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let n8 = || *((this + 0x438) as *const i8);
    let uc = read_u32(0x9cf6f0);
    let mut conta_extra = 1i8;
    let mut colocados = 0i32;
    let mut nacao_do_campeao = 0usize;
    let h = campeao((read_u32(0xae23d0) as usize).wrapping_add(uc.wrapping_mul(0x6b) as usize), 0);
    if h != 0 && read_u32(h + 0x53) != 0 && read_u32(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = uc;
        colocados = 1;
        let id = read_u32(read_u32(h + 0x53) as usize);
        let mut i = -1i32;
        for k in 0..n8() as i32 {
            if read_u32(cqi3_e(this, k)) == id {
                i = k;
                break;
            }
        }
        let p = cqi3_e(this, i) + 0x11;
        *(p as *mut u8) = read_u8(p).wrapping_add(1);
        nacao_do_campeao = read_u32(h + 0x53) as usize;
    }
    let s1 = sorteio(n8() as i32) as u8 as i8;
    let mut s2 = sorteio(n8() as i32) as u8 as i8;
    let mut s3 = sorteio(n8() as i32) as u8 as i8;
    while s1 == s2 || s1 == s3 || s2 == s3 {
        s2 = sorteio(n8() as i32) as u8 as i8;
        s3 = sorteio(n8() as i32) as u8 as i8;
    }
    let mut ultima_fase = 0u8;
    let mut r = 0i8;
    while r < n8() {
        let (f1, f2): (u8, u8) = if r < 6 {
            crate::comparacao::ramo("cqi_3 tres na primeira fase");
            (3, 0) // cqi3: 1ª a 6ª nação
        } else if r < 8 {
            (4, 0)
        } else if r < 0xf {
            (1, 1)
        } else if r < 0x11 {
            (1, 2)
        } else if r < 0x15 {
            (0, 3)
        } else if r < 0x30 {
            (0, 2)
        } else {
            (0, 1)
        };
        let mut vagas = [0u8, f1, f2];
        let e = cqi3_e(this, r as i32);
        let nacao = read_u32(e + 4) as usize;
        if nacao == nacao_do_campeao {
            vagas[1] = vagas[1].wrapping_sub(1);
        }
        if r == s1 || r == s2 || r == s3 {
            crate::comparacao::ramo("cqi_3 fair play");
            vagas[2] = vagas[2].wrapping_add(1); // cqi3: a vaga do fair play
        }
        let um = || enesimo(this, clubes, conta, read_u32(e), 1, 1, 0);
        let mut escolhidos = 0u8;
        let (mut vencedor, mut vice) = (0usize, 0usize);
        let mut f = 1u8;
        while f < 3 {
            let lim = vagas[f as usize] as i8;
            if lim > 0 {
                ultima_fase = f;
                let mut j = 0i8;
                loop {
                    let mut tentar_de_novo = false;
                    let c = if escolhidos == 0 {
                        if read_u8(nacao + 0x11c) == 0 {
                            um()
                        } else {
                            let copa = read_u32(0xb63d7e + (linha(nacao) as i32 * 0x48) as usize) as usize;
                            if copa == 0 {
                                aviso_em(0xaa1a1c, 0x418);
                                tentar_de_novo = true;
                                0
                            } else {
                                let mut c = campeao(copa, 0);
                                if c == 0 || read_u32(c + 0x53) == 0 {
                                    c = um();
                                }
                                if read_u32(c + 0x1db) == u32::MAX {
                                    c
                                } else {
                                    let mut c = campeao(copa, 1);
                                    if c == 0 || read_u32(c + 0x53) == 0 || read_u32(nacao) == read_u32(0x9cf244) {
                                        c = um();
                                    }
                                    if read_u32(c + 0x1db) == u32::MAX {
                                        c
                                    } else if read_u32(0xadadfc) == 0 {
                                        um()
                                    } else {
                                        let id = read_u32(copa);
                                        let ano = core::ptr::read_unaligned((0xb63d7b + (linha(nacao) as i32 * 0x48) as usize) as *const u16) as i32;
                                        let comp = read_u32((read_u32(0xadadfc) as usize).wrapping_add(id as usize * 4)) as usize;
                                        if read_i16(comp + 0x40) as i32 == ano || read_u32(nacao) == read_u32(0x9cf488) || read_u32(nacao) == read_u32(0x9cf244) {
                                            um()
                                        } else {
                                            let (mut v1, mut v2) = (-1i32, -1i32);
                                            for k in 0..0x32usize {
                                                if read_u32(this + 0x440 + k * 12) == id {
                                                    v1 = read_u32(this + 0x444 + k * 12) as i32;
                                                    v2 = read_u32(this + 0x448 + k * 12) as i32;
                                                    break;
                                                }
                                            }
                                            let base = read_u32(0xae23b4) as usize;
                                            vencedor = if v1 != -1 { base.wrapping_add((v1 as usize).wrapping_mul(0x245)) } else { 0 };
                                            vice = if v2 != -1 { base.wrapping_add((v2 as usize).wrapping_mul(0x245)) } else { 0 };
                                            if vencedor == 0 {
                                                um()
                                            } else if vice != 0 {
                                                0
                                            } else {
                                                c
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if escolhidos == 1 {
                        if read_u8(nacao + 0x11c) == 0 {
                            um()
                        } else {
                            let copa = read_u32(0xb63d82 + (linha(nacao) as i32 * 0x48) as usize) as usize;
                            if copa == 0 {
                                um()
                            } else {
                                let c = campeao(copa, 0);
                                if c == 0 || read_u32(c + 0x53) == 0 || read_u32(c + 0x1db) != u32::MAX {
                                    um()
                                } else {
                                    c
                                }
                            }
                        }
                    } else {
                        um()
                    };
                    if tentar_de_novo {
                        j -= 1;
                    } else {
                        escolhidos = escolhidos.wrapping_add(1);
                        if c != 0 {
                            cqi3_entra(this, c, f, r as i32, &mut colocados);
                            if colocados as i16 as i32 - conta_extra as i32 + 1 >= 0x76 {
                                return;
                            }
                        }
                    }
                    j += 1;
                    if j >= lim {
                        break;
                    }
                }
            }
            f += 1;
        }
        if vencedor != 0 && vice != 0 {
            conta_extra += 1;
            let (a, b) = (read_u32(vencedor + 0x1db) == u32::MAX, read_u32(vice + 0x1db) == u32::MAX);
            if !a && !b {
                let c = um();
                if c != 0 {
                    cqi3_entra(this, c, ultima_fase, r as i32, &mut colocados);
                }
            } else if a && b {
                let fase = (conta_extra as u8).wrapping_mul(10).wrapping_add(3);
                ultima_fase = fase;
                if read_u32(vencedor + 0x53) != 0 {
                    cqi3_entra(this, vencedor, fase, r as i32, &mut colocados);
                }
                cqi3_entra(this, vice, fase, r as i32, &mut colocados);
            } else {
                cqi3_entra(this, if a { vencedor } else { vice }, ultima_fase, r as i32, &mut colocados);
            }
        }
        r += 1;
    }
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn cqi3_dif(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Lado a lado: a classificação (com a tabela das finais em +0x440), os clubes, o sorteio (a cópia sem contar) e a marca
/// da caixa.
unsafe extern "thiscall" fn comparar_uefa_vagas_copa_da_uefa(this: usize, clubes: usize, conta: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1664);
    let n = (read_u32(0xae235c) as i32).max(0) as usize;
    // o histórico das competições também: a 0x4afd80 o reordena (linhas empatadas: a ordem de antes muda o clube achado)
    let mut reg = vec![(this, 0x698), (read_u32(0xae23b4) as usize, n * 0x245), (0xb67a34, 4), (read_u32(0xae23d4) as usize, (read_u32(0xae237c) as i32).max(0) as usize * 0x1a)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clubes, conta), || crate::aleatorio::sem_contar(|| uefa_vagas_copa_da_uefa(this, clubes, conta)));
    PLACAR.registrar(igual, || format!("uefa_vagas_copa_da_uefa: diferenças em {:x?}", cqi3_dif(&mo, &mr)));
}

/// { 0x903890 } (thiscall, ret 0, uefa_seeding.cpp, `UefaSeeding::pick_english_intertoto_club`; quadro SEH pela notícia
/// local, sem destrutor no caminho normal): o clube inglês da Intertoto. Em 2001, ou sem a Premier ([0xadadfc] +
/// [0x9cf5bc] × 4), devolve 0. Senão completa a lista dos voluntários (+0x43b, 30 ids, alocada com 0x78 bytes; +0x43f a
/// conta) até 6, sorteando clubes da Premier (0x90cfc0 com +0x3e, até 80 sorteios) sem técnico humano e sem vaga
/// europeia; escolhe o voluntário sem vaga da divisão mais alta (a da Premier +4 conta o +0x5f; as outras +30); sem
/// nenhum, sorteia clubes da Premier até um sem vaga. Entrega a notícia 0x17 com o clube (0x77bbe0, 0x785790, 0x7862d0
/// à divisão da Premier) e devolve o clube.
unsafe extern "thiscall" fn uefa_voluntario_ingles(this: usize) -> usize {
    if read_i16(0xae2c92) == 0x7d1 {
        return 0;
    }
    let tabela = read_u32(0xadadfc) as usize;
    if tabela == 0 {
        return 0;
    }
    let liga = read_u32(tabela.wrapping_add(read_u32(0x9cf5bc) as usize * 4)) as usize;
    if liga == 0 {
        return 0;
    }
    crate::comparacao::ramo("cqi_3 voluntario ingles");
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
    let a_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7862d0usize);
    let clube_sorteado = || {
        let s = sorteio(read_i16(liga + 0x3e) as i32) as i16 as i32;
        read_u32((read_u32(liga + 0xb1) as usize).wrapping_add((s * 0x3b) as usize)) as usize
    };
    let mut tentativas = 0i8;
    while (read_u8(this + 0x43f) as i8) < 6 && tentativas < 0x50 {
        tentativas += 1;
        let c = clube_sorteado();
        if humano(c, 0, 1, 0, 0) == 0 && read_u32(c + 0x1db) == u32::MAX {
            if read_u32(this + 0x43b) == 0 {
                *((this + 0x43f) as *mut u8) = 0;
                let l = alocar(0x78);
                core::ptr::write_unaligned((this + 0x43b) as *mut u32, l as u32);
                if l == 0 {
                    aviso_em(0xaa1a1c, 0xf9);
                    continue;
                }
            }
            let k = read_u8(this + 0x43f) as i8;
            if k < 0x1e {
                core::ptr::write_unaligned((read_u32(this + 0x43b) as usize + k as usize * 4) as *mut u32, read_u32(c));
                *((this + 0x43f) as *mut u8) = (k as u8).wrapping_add(1);
            }
        }
    }
    let mut melhor = 0usize;
    let mut nivel = 0x64i8;
    let k = read_u8(this + 0x43f) as i8 as i32;
    if k > 0 {
        let lista = read_u32(this + 0x43b) as usize;
        let base = read_u32(0xae23b4) as usize;
        for q in 0..k as usize {
            let c = base.wrapping_add((read_u32(lista + q * 4) as usize).wrapping_mul(0x245));
            if read_u32(c + 0x1db) == u32::MAX {
                let v = read_u8(c + 0x5f) as i8;
                if read_u32(c + 0x5b) == read_u32(liga + 4) {
                    if v < nivel {
                        melhor = c;
                        nivel = v;
                    }
                } else if v as i32 + 0x1e < nivel as i32 {
                    melhor = c;
                    nivel = v.wrapping_add(0x1e);
                }
            }
        }
    }
    while melhor == 0 {
        let c = clube_sorteado();
        if read_u32(c + 0x1db) == u32::MAX {
            melhor = c;
        }
    }
    let mut noticia = [0u8; 0xe8];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0x17, 0);
    gravar(n, 0, read_u32(melhor));
    a_divisao(read_u32(0xb7431c) as usize, n, read_u32(liga + 4) as usize);
    melhor
}

/// Em 2001 (devolve 0 sem efeito): a devolução das duas. Fora de 2001 só a original (sorteia, aloca a lista e entrega a
/// notícia).
unsafe extern "thiscall" fn comparar_uefa_voluntario_ingles(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(1665);
    if read_i16(0xae2c92) != 0x7d1 {
        return o(this);
    }
    let r = o(this);
    let c = uefa_voluntario_ingles(this);
    PLACAR.registrar(r == c, || format!("uefa_voluntario_ingles: original {:x} rust {:x}", r, c));
    r
}

/// { 0x9035a0 } (thiscall, ret 4, uefa_seeding.cpp, `UefaSeeding::update_season`; quadro SEH pela notícia local, sem
/// destrutor no caminho normal): chamada pelas copas europeias. Sem `diario`: quando o ano do jogo ([0xae2c92]) é outro
/// que o +0x439, grava o ano e refaz a classificação: os coeficientes (0x903b50), as vagas por nação (0x903cc0), a tabela
/// dos clubes (0x904040; vazia, a caixa da linha 0x2b7), as vagas da Liga dos Campeões (0x904210), da Copa da UEFA
/// (0x904580) e da Intertoto (0x904e30), solta a tabela e entrega a notícia dos classificados (0x905f00). Com `diario`:
/// no dia 100 do ano, com a Premier e a Intertoto no jogo, a notícia 0x16 do pedido de voluntários (com o ano) a cada
/// técnico humano de clube da Premier (sem a Premier, volta sem mais nada); e o prazo das inscrições europeias (0x9058e0).
unsafe extern "thiscall" fn uefa_virada_da_classificacao(this: usize, diario: u32) {
    if diario == 0 {
        let ano = read_i16(0xae2c92);
        if ano == read_i16(this + 0x439) {
            return;
        }
        crate::comparacao::ramo("cqi_3 virada da classificacao");
        core::ptr::write_unaligned((this + 0x439) as *mut i16, ano);
        let coeficientes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x903b50usize);
        let vagas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x903cc0usize);
        let juntar: unsafe extern "thiscall" fn(usize, *mut usize) -> u32 = core::mem::transmute(0x904040usize);
        let campeoes: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x904210usize);
        let copa_uefa: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x904580usize);
        let intertoto: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x904e30usize);
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        let classificados: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x905f00usize);
        coeficientes(this);
        vagas(this);
        let mut t = 0usize;
        let n = juntar(this, &mut t);
        if n as u16 == 0 {
            aviso_em(0xaa1a1c, 0x2b7);
        } else {
            campeoes(this, t, n);
            copa_uefa(this, t, n);
            intertoto(this, t, n); // cqi3: as vagas da Intertoto
            soltar(t);
        }
        classificados(this);
        return;
    }
    if *(0xae2c90 as *const i16) == 100 {
        let tabela = read_u32(0xadadfc) as usize;
        let liga = read_u32(tabela.wrapping_add(read_u32(0x9cf5bc) as usize * 4)) as usize;
        if liga == 0 {
            return;
        }
        if read_u32(tabela.wrapping_add(read_u32(0x9cf6f8) as usize * 4)) != 0 {
            let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
            let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
            let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
            let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
            let mut noticia = [0u8; 0xe8];
            let p = noticia.as_mut_ptr() as usize;
            construir(p, 0x16, 0);
            gravar(p, 0, 0);
            gravar(p, 2, read_i16(this + 0x439) as u16 as u32);
            let mut i = 0i16;
            while i < read_i16(liga + 0x3e) {
                let c = read_u32((read_u32(liga + 0xb1) as usize).wrapping_add(i as i32 as usize * 0x3b)) as usize;
                if humano(read_u32(0xb7431c) as usize, c) != 0 {
                    gravar(p, 1, read_u32(c));
                    entregar(read_u32(0xb7431c) as usize, p, c);
                }
                i += 1;
            }
        }
    }
    let prazos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x9058e0usize);
    prazos(this);
}

/// Só a original (refaz as vagas pelas rotinas de dentro, solta a tabela, entrega notícias); a cópia leva o contador.
unsafe extern "thiscall" fn comparar_uefa_virada_da_classificacao(this: usize, diario: u32) {
    // ponytail: refaz a classificação inteira e entrega notícias: não dá para rodar duas vezes
    orig::<unsafe extern "thiscall" fn(usize, u32)>(1666)(this, diario)
}

// ---------- 265a leva: os temporários do save: as sementes da UEFA e da CONMEBOL e os países anfitriões (uefa_seeding.cpp, conmebol_seeding.cpp, host_country.cpp) ----------


/// A contagem de um byte com sinal (a original usa `movsx`).
unsafe fn cpx6_n8(a: usize) -> u32 {
    *(a as *const i8) as i32 as u32
}

/// Abre `CM3_TEMP\<nome>` para gravar ("wb"): o caminho pela 0x90bde0 e o fopen do jogo (0x945b00).
unsafe fn cpx6_abrir(nome: usize) -> usize {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    abrir(caminho(0x987da0, nome), 0x9a7f28)
}

unsafe fn cpx6_escrever(dados: usize, tamanho: u32, quantos: u32, f: usize) -> u32 {
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    escrever(dados, tamanho, quantos, f)
}

unsafe fn cpx6_fechar(f: usize) {
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    fechar(f);
}

/// { 0x905510 } (thiscall, ret 0, uefa_seeding.cpp): `UefaSeeding::save_to_temp_file`: grava `CM3_TEMP\uefa_seeding.tmp`:
/// a quantidade de países (+0x438, i8), os registros de 0x12 bytes de +0, +0x439 (u16), a quantidade +0x43f (i8) e, se
/// não for zero, essa quantidade de u32 de [+0x43b], e os 50 registros fixos de 0xc bytes de +0x440. Erro ao abrir ou
/// gravar: a caixa "Error" (uefa_seeding.cpp 0x64f, 0x657, 0x660, 0x668, 0x66f, 0x679, 0x682), sem fechar o arquivo
/// (como a original).
unsafe extern "thiscall" fn gravar_sementes_da_uefa(this: usize) {
    const FONTE: usize = 0xaa1a1c;
    crate::comparacao::ramo("cpx_6 gravar sementes da uefa");
    let f = cpx6_abrir(0xaa1a50);
    if f == 0 {
        aviso_em(FONTE, 0x64f);
        return;
    }
    if cpx6_escrever(this + 0x438, 1, 1, f) != 1 {
        aviso_em(FONTE, 0x657);
        return;
    }
    if cpx6_escrever(this, 0x12, cpx6_n8(this + 0x438), f) != cpx6_n8(this + 0x438) {
        aviso_em(FONTE, 0x660);
        return;
    }
    if cpx6_escrever(this + 0x439, 2, 1, f) != 1 {
        aviso_em(FONTE, 0x668);
        return;
    }
    if cpx6_escrever(this + 0x43f, 1, 1, f) != 1 {
        aviso_em(FONTE, 0x66f);
        return;
    }
    if *((this + 0x43f) as *const u8) != 0 && cpx6_escrever(read_usize(this + 0x43b), 4, cpx6_n8(this + 0x43f), f) != cpx6_n8(this + 0x43f) {
        aviso_em(FONTE, 0x679);
        return;
    }
    if cpx6_escrever(this + 0x440, 0xc, 0x32, f) != 0x32 { // cpx6: os 50 registros fixos
        aviso_em(FONTE, 0x682);
        return;
    }
    cpx6_fechar(f);
}

/// Comparador da 0x905510: só a original.
unsafe extern "thiscall" fn comparar_gravar_sementes_da_uefa(this: usize) {
    // ponytail: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    // (o .tmp vai no .qst pelo zadd_matching_files)
    orig::<unsafe extern "thiscall" fn(usize)>(1500)(this)
}

/// { 0x4c6b70 } (thiscall, ret 0, conmebol_seeding.cpp): `ConmebolSeeding::save_to_temp_file`: grava
/// `CM3_TEMP\conmebol_seeding.tmp`: a quantidade de países (+0x6e, i8), os registros de 0xa bytes de +0, +0x6f (u16) e
/// +0x71 (u32). Erro: a caixa "Error" (conmebol_seeding.cpp 0x2ee, 0x2f6, 0x2ff, 0x307, 0x30f), sem fechar o arquivo.
unsafe extern "thiscall" fn gravar_sementes_da_conmebol(this: usize) {
    const FONTE: usize = 0x9a7ed8;
    crate::comparacao::ramo("cpx_6 gravar sementes da conmebol");
    let f = cpx6_abrir(0x9a7f10);
    if f == 0 {
        aviso_em(FONTE, 0x2ee);
        return;
    }
    if cpx6_escrever(this + 0x6e, 1, 1, f) != 1 {
        aviso_em(FONTE, 0x2f6);
        return;
    }
    if cpx6_escrever(this, 0xa, cpx6_n8(this + 0x6e), f) != cpx6_n8(this + 0x6e) { // cpx6: os registros dos países
        aviso_em(FONTE, 0x2ff);
        return;
    }
    if cpx6_escrever(this + 0x6f, 2, 1, f) != 1 {
        aviso_em(FONTE, 0x307);
        return;
    }
    if cpx6_escrever(this + 0x71, 4, 1, f) != 1 {
        aviso_em(FONTE, 0x30f);
        return;
    }
    cpx6_fechar(f);
}

/// Comparador da 0x4c6b70: só a original.
unsafe extern "thiscall" fn comparar_gravar_sementes_da_conmebol(this: usize) {
    // ponytail: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize)>(1501)(this)
}

/// { 0x5fb320 } (thiscall, ret 0, host_country.cpp): `HostCountry::save_temp_file`: grava `CM3_TEMP\HostCountry.tmp`:
/// a quantidade (+4, i16) e os registros de 0x22 bytes de [+0], sem conferir a gravação. Erro ao abrir: a caixa
/// "Error" (host_country.cpp 0x475).
unsafe extern "thiscall" fn gravar_paises_anfitrioes(this: usize) {
    crate::comparacao::ramo("cpx_6 gravar paises anfitrioes");
    let f = cpx6_abrir(0x9cf10c);
    if f == 0 {
        aviso_em(0x9cf0d8, 0x475);
        return;
    }
    cpx6_escrever(this + 4, 2, 1, f);
    cpx6_escrever(read_usize(this), 0x22, *((this + 4) as *const i16) as i32 as u32, f); // cpx6: os anfitriões
    cpx6_fechar(f);
}

/// Comparador da 0x5fb320: só a original.
unsafe extern "thiscall" fn comparar_gravar_paises_anfitrioes(this: usize) {
    // ponytail: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize)>(1502)(this)
}

