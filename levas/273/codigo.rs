// ---------- 273a leva: a classificação da UEFA: os coeficientes das nações, as vagas de cada nação e o n-ésimo clube da nação (uefa_seeding.cpp) ----------

unsafe fn cqi1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cqi1_f(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

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
        let p = cqi1_d(e + 4) as usize;
        if ano != 0x7d1 {
            for k in 0..10usize {
                core::ptr::write_unaligned((p + 0xe0 + k * 4) as *mut u32, cqi1_d(p + 0xe8 + k * 4));
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
        let soma = cqi1_f(p + 0x100) + cqi1_f(p + 0xf8) + cqi1_f(p + 0xf0) + cqi1_f(p + 0xe8) + cqi1_f(p + 0xe0); // cqi1: as 5 temporadas
        core::ptr::write_unaligned((e + 8) as *mut f32, soma as f32);
        *((e + 0x11) as *mut u8) = 0;
        core::ptr::write_unaligned((e + 0xf) as *mut u16, 0);
        i += 1;
    }
    ordenar(this, conta() as i32 as u32, 0x12, 0x905cb0);
    let mut i = 0i8;
    while i < conta() {
        let s = selecao(cqi1_d(cqi1_e(this, i as i32) + 4) as usize);
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
        let p = cqi1_d(cqi1_e(this, i) + 4) as usize;
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
    let g = |a: usize| cqi1_d(a);
    let conta = || *((this + 0x438) as *const i8);
    let ano = *(0xae2c92 as *const i16);
    let (mut fa, mut fb, mut fc, mut cont) = (-1i8, -1i8, -1i8, 0i8);
    let mut i = 0i8;
    while i < conta() {
        let e = cqi1_e(this, i as i32);
        let id = cqi1_d(e);
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
        let id = cqi1_d(e);
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
        let id = cqi1_d(e);
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
    let italia = |k: i8| cqi1_d(cqi1_e(this, k as i32)) == g(0x9cf360);
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
    while p > clubes && cqi1_d(p - 8) == nacao {
        p -= 12;
    }
    let mut falta = (n as u8).wrapping_sub(1);
    let mut achou = false;
    if p < ultimo {
        loop {
            let c = cqi1_d(p) as usize;
            if sem_vaga != 0 && c != 0 && cqi1_d(c + 0x1db) != u32::MAX {
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
    if cqi1_d(p + 4) == nacao {
        return cqi1_d(p) as usize;
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

unsafe fn cqi2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cqi2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqi2_mais_um(a: usize) {
    *(a as *mut u8) = cqi2_b(a).wrapping_add(1);
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
    if c == 0 || cqi2_d(c + 0x53) == 0 || cqi2_d(c + 0x1db) != u32::MAX {
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
    let t = alocar((cqi2_d(0xae235c) as i32).wrapping_mul(12) as u32);
    *saida = t;
    if t == 0 {
        aviso_em(0xaa1a1c, 0x2ed);
        return 0;
    }
    let mut k = 0u32;
    let mut c = cqi2_d(0xae23b4) as usize;
    let mut i = 0u32;
    if cqi2_d(0xae235c) as i32 > 0 {
        loop {
            let nacao = cqi2_d(c + 0x53) as usize;
            if nacao != 0 && selecao(c) == 0 {
                let cont = cqi2_d(nacao + 0x71) as usize;
                if cont != 0 && cqi2_d(cont) == cqi2_d(0x9cfa10) {
                    *((c + 0x1db) as *mut u32) = u32::MAX;
                    core::ptr::write_unaligned((cqi2_d(0xae2d14) as usize).wrapping_add((cqi2_d(c) as usize).wrapping_mul(6)) as *mut u32, u32::MAX);
                    let r = t.wrapping_add((k as u16 as i16 as i32 * 12) as usize);
                    core::ptr::write_unaligned(r as *mut u32, c as u32);
                    core::ptr::write_unaligned((r + 4) as *mut u32, cqi2_d(nacao));
                    *((r + 8) as *mut u8) = (core::ptr::read_unaligned((c + 0x80) as *const i16) as i32 / 500) as u8;
                    let linha = cqi2_d(c + 0x5b) as usize;
                    if linha != 0 {
                        crate::comparacao::ramo("cqi_2 nivel da divisao");
                        core::ptr::write_unaligned((r + 9) as *mut u16, core::ptr::read_unaligned((linha + 0x69) as *const u16)); // cqi2: o nível da divisão
                        *((r + 0xb) as *mut u8) = cqi2_b(c + 0x5f);
                        let comp = cqi2_d(cqi2_d(c + 0x5b) as usize + 0x5d) as usize;
                        if comp == 0 || cqi2_b(comp + 0x11c) == 0 {
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
            if (i as u16 as i16 as i32) >= cqi2_d(0xae235c) as i32 {
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
    let n = (cqi2_d(0xae235c) as i32).max(0) as usize;
    vec![(cqi2_d(0xae23b4) as usize, n * 0x245), (cqi2_d(0xae2d14) as usize, n * 6), (0xb67a34, 4), (cqi2_d(0xae23d4) as usize, (cqi2_d(0xae237c) as i32).max(0) as usize * 0x1a)]
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
    let lc = cqi2_d(0x9cf6e8);
    let mut cap: [u8; 5] = [0, 0x10, 0x12, 0x13, 0x12];
    let h = campeao((cqi2_d(0xae23d0) as usize).wrapping_add(lc.wrapping_mul(0x6b) as usize), 0);
    if h != 0 && cqi2_d(h + 0x53) != 0 && cqi2_d(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = lc;
        let id = cqi2_d(cqi2_d(h + 0x53) as usize);
        let n = *((this + 0x438) as *const i8) as i32;
        let mut i = -1i32;
        for k in 0..n {
            if cqi2_d(cqi2_e(this, k)) == id {
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
            let c = enesimo(this, clubes, conta, cqi2_d(e), k as u32, 0, 0);
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
    let it = cqi2_d(0x9cf6f8);
    let mut colocados = 0i32;
    let mut cl = 0u8;
    loop {
        let fase: u8 = if (cl as i8) < 8 { 1 } else if (cl as i8) >= 0xc { 3 } else { 2 };
        let mut i = cl;
        if cl == cqi2_b(this + 0x438) {
            i = 0;
        }
        let e = cqi2_e(this, i as i8 as i32);
        if cqi2_d(e) == cqi2_d(0x9cf2e4) {
            let mut c = voluntario(this);
            if c == 0 {
                c = enesimo(this, clubes, conta, cqi2_d(e), 1, 1, 1);
            }
            if cqi2_vaga(this, c, it, fase, i as i8 as i32) {
                colocados += 1;
            }
            if cqi2_b(e + 0xe) == 2 {
                let mut c = voluntario(this);
                if c == 0 {
                    c = enesimo(this, clubes, conta, cqi2_d(e), 1, 1, 1);
                }
                if cqi2_vaga(this, c, it, fase.wrapping_add(1), i as i8 as i32) {
                    colocados += 1;
                }
            }
            let lista = cqi2_d(this + 0x43b) as usize;
            if lista != 0 {
                soltar(lista);
                core::ptr::write_unaligned((this + 0x43b) as *mut u32, 0);
                *((this + 0x43f) as *mut u8) = 0;
            }
        } else if (cqi2_b(e + 0xe) as i8) > 0 {
            let mut j = 0i8;
            loop {
                if colocados as i16 >= 0x3c {
                    break;
                }
                let c = enesimo(this, clubes, conta, cqi2_d(e), 1, 1, 0);
                if cqi2_vaga(this, c, it, (j as u8).wrapping_add(fase), i as i8 as i32) {
                    crate::comparacao::ramo("cqi_2 clube na intertoto");
                    colocados += 1; // cqi2: mais um na Intertoto
                }
                j += 1;
                if j >= cqi2_b(e + 0xe) as i8 {
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
    if *(0xae2c92 as *const i16) != 0x7d1 || cqi2_d(this + 0x43b) != 0 {
        return o(this, clubes, conta);
    }
    let mut reg = cqi2_regioes_dos_clubes();
    reg.push((this, 0x440));
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clubes, conta), || crate::aleatorio::sem_contar(|| uefa_vagas_intertoto(this, clubes, conta)));
    PLACAR.registrar(igual, || format!("uefa_vagas_intertoto: diferenças em {:x?}", cqi2_dif(&mo, &mr)));
}

// ---------- 273a leva: a classificação da UEFA: as vagas da Copa da UEFA, o voluntário inglês da Intertoto e a virada do ano (uefa_seeding.cpp) ----------

unsafe fn cqi3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cqi3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqi3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A entrada `i` da tabela das nações da classificação (registros de 0x12 bytes; +0 id da nação, +4 a nação, +0x11
/// clubes da nação nas copas).
fn cqi3_e(this: usize, i: i32) -> usize {
    this.wrapping_add((i * 0x12) as usize)
}

/// O clube `c` (com nação e ainda sem vaga) entra na Copa da UEFA na fase `fase`; um clube a mais da nação `r`. Devolve
/// se entrou. (O caso da nação −1, que a original procura pela nação do clube, não acontece: `r` vem da volta.)
unsafe fn cqi3_entra(this: usize, c: usize, fase: u8, r: i32, colocados: &mut i32) -> bool {
    if cqi3_d(c + 0x53) == 0 || cqi3_d(c + 0x1db) != u32::MAX {
        return false;
    }
    *((c + 0x1db) as *mut u32) = cqi3_d(0x9cf6f0);
    *((c + 0x1df) as *mut u8) = fase;
    *colocados += 1;
    let p = cqi3_e(this, r) + 0x11;
    *(p as *mut u8) = cqi3_b(p).wrapping_add(1);
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
    let uc = cqi3_d(0x9cf6f0);
    let mut conta_extra = 1i8;
    let mut colocados = 0i32;
    let mut nacao_do_campeao = 0usize;
    let h = campeao((cqi3_d(0xae23d0) as usize).wrapping_add(uc.wrapping_mul(0x6b) as usize), 0);
    if h != 0 && cqi3_d(h + 0x53) != 0 && cqi3_d(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = uc;
        colocados = 1;
        let id = cqi3_d(cqi3_d(h + 0x53) as usize);
        let mut i = -1i32;
        for k in 0..n8() as i32 {
            if cqi3_d(cqi3_e(this, k)) == id {
                i = k;
                break;
            }
        }
        let p = cqi3_e(this, i) + 0x11;
        *(p as *mut u8) = cqi3_b(p).wrapping_add(1);
        nacao_do_campeao = cqi3_d(h + 0x53) as usize;
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
        let nacao = cqi3_d(e + 4) as usize;
        if nacao == nacao_do_campeao {
            vagas[1] = vagas[1].wrapping_sub(1);
        }
        if r == s1 || r == s2 || r == s3 {
            crate::comparacao::ramo("cqi_3 fair play");
            vagas[2] = vagas[2].wrapping_add(1); // cqi3: a vaga do fair play
        }
        let um = || enesimo(this, clubes, conta, cqi3_d(e), 1, 1, 0);
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
                        if cqi3_b(nacao + 0x11c) == 0 {
                            um()
                        } else {
                            let copa = cqi3_d(0xb63d7e + (linha(nacao) as i32 * 0x48) as usize) as usize;
                            if copa == 0 {
                                aviso_em(0xaa1a1c, 0x418);
                                tentar_de_novo = true;
                                0
                            } else {
                                let mut c = campeao(copa, 0);
                                if c == 0 || cqi3_d(c + 0x53) == 0 {
                                    c = um();
                                }
                                if cqi3_d(c + 0x1db) == u32::MAX {
                                    c
                                } else {
                                    let mut c = campeao(copa, 1);
                                    if c == 0 || cqi3_d(c + 0x53) == 0 || cqi3_d(nacao) == cqi3_d(0x9cf244) {
                                        c = um();
                                    }
                                    if cqi3_d(c + 0x1db) == u32::MAX {
                                        c
                                    } else if cqi3_d(0xadadfc) == 0 {
                                        um()
                                    } else {
                                        let id = cqi3_d(copa);
                                        let ano = core::ptr::read_unaligned((0xb63d7b + (linha(nacao) as i32 * 0x48) as usize) as *const u16) as i32;
                                        let comp = cqi3_d((cqi3_d(0xadadfc) as usize).wrapping_add(id as usize * 4)) as usize;
                                        if cqi3_w(comp + 0x40) as i32 == ano || cqi3_d(nacao) == cqi3_d(0x9cf488) || cqi3_d(nacao) == cqi3_d(0x9cf244) {
                                            um()
                                        } else {
                                            let (mut v1, mut v2) = (-1i32, -1i32);
                                            for k in 0..0x32usize {
                                                if cqi3_d(this + 0x440 + k * 12) == id {
                                                    v1 = cqi3_d(this + 0x444 + k * 12) as i32;
                                                    v2 = cqi3_d(this + 0x448 + k * 12) as i32;
                                                    break;
                                                }
                                            }
                                            let base = cqi3_d(0xae23b4) as usize;
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
                        if cqi3_b(nacao + 0x11c) == 0 {
                            um()
                        } else {
                            let copa = cqi3_d(0xb63d82 + (linha(nacao) as i32 * 0x48) as usize) as usize;
                            if copa == 0 {
                                um()
                            } else {
                                let c = campeao(copa, 0);
                                if c == 0 || cqi3_d(c + 0x53) == 0 || cqi3_d(c + 0x1db) != u32::MAX {
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
            let (a, b) = (cqi3_d(vencedor + 0x1db) == u32::MAX, cqi3_d(vice + 0x1db) == u32::MAX);
            if !a && !b {
                let c = um();
                if c != 0 {
                    cqi3_entra(this, c, ultima_fase, r as i32, &mut colocados);
                }
            } else if a && b {
                let fase = (conta_extra as u8).wrapping_mul(10).wrapping_add(3);
                ultima_fase = fase;
                if cqi3_d(vencedor + 0x53) != 0 {
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
    let n = (cqi3_d(0xae235c) as i32).max(0) as usize;
    // o histórico das competições também: a 0x4afd80 o reordena (linhas empatadas: a ordem de antes muda o clube achado)
    let mut reg = vec![(this, 0x698), (cqi3_d(0xae23b4) as usize, n * 0x245), (0xb67a34, 4), (cqi3_d(0xae23d4) as usize, (cqi3_d(0xae237c) as i32).max(0) as usize * 0x1a)];
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
    if cqi3_w(0xae2c92) == 0x7d1 {
        return 0;
    }
    let tabela = cqi3_d(0xadadfc) as usize;
    if tabela == 0 {
        return 0;
    }
    let liga = cqi3_d(tabela.wrapping_add(cqi3_d(0x9cf5bc) as usize * 4)) as usize;
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
        let s = sorteio(cqi3_w(liga + 0x3e) as i32) as i16 as i32;
        cqi3_d((cqi3_d(liga + 0xb1) as usize).wrapping_add((s * 0x3b) as usize)) as usize
    };
    let mut tentativas = 0i8;
    while (cqi3_b(this + 0x43f) as i8) < 6 && tentativas < 0x50 {
        tentativas += 1;
        let c = clube_sorteado();
        if humano(c, 0, 1, 0, 0) == 0 && cqi3_d(c + 0x1db) == u32::MAX {
            if cqi3_d(this + 0x43b) == 0 {
                *((this + 0x43f) as *mut u8) = 0;
                let l = alocar(0x78);
                core::ptr::write_unaligned((this + 0x43b) as *mut u32, l as u32);
                if l == 0 {
                    aviso_em(0xaa1a1c, 0xf9);
                    continue;
                }
            }
            let k = cqi3_b(this + 0x43f) as i8;
            if k < 0x1e {
                core::ptr::write_unaligned((cqi3_d(this + 0x43b) as usize + k as usize * 4) as *mut u32, cqi3_d(c));
                *((this + 0x43f) as *mut u8) = (k as u8).wrapping_add(1);
            }
        }
    }
    let mut melhor = 0usize;
    let mut nivel = 0x64i8;
    let k = cqi3_b(this + 0x43f) as i8 as i32;
    if k > 0 {
        let lista = cqi3_d(this + 0x43b) as usize;
        let base = cqi3_d(0xae23b4) as usize;
        for q in 0..k as usize {
            let c = base.wrapping_add((cqi3_d(lista + q * 4) as usize).wrapping_mul(0x245));
            if cqi3_d(c + 0x1db) == u32::MAX {
                let v = cqi3_b(c + 0x5f) as i8;
                if cqi3_d(c + 0x5b) == cqi3_d(liga + 4) {
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
        if cqi3_d(c + 0x1db) == u32::MAX {
            melhor = c;
        }
    }
    let mut noticia = [0u8; 0xe8];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0x17, 0);
    gravar(n, 0, cqi3_d(melhor));
    a_divisao(cqi3_d(0xb7431c) as usize, n, cqi3_d(liga + 4) as usize);
    melhor
}

/// Em 2001 (devolve 0 sem efeito): a devolução das duas. Fora de 2001 só a original (sorteia, aloca a lista e entrega a
/// notícia).
unsafe extern "thiscall" fn comparar_uefa_voluntario_ingles(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(1665);
    if cqi3_w(0xae2c92) != 0x7d1 {
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
        let ano = cqi3_w(0xae2c92);
        if ano == cqi3_w(this + 0x439) {
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
        let tabela = cqi3_d(0xadadfc) as usize;
        let liga = cqi3_d(tabela.wrapping_add(cqi3_d(0x9cf5bc) as usize * 4)) as usize;
        if liga == 0 {
            return;
        }
        if cqi3_d(tabela.wrapping_add(cqi3_d(0x9cf6f8) as usize * 4)) != 0 {
            let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
            let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
            let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
            let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
            let mut noticia = [0u8; 0xe8];
            let p = noticia.as_mut_ptr() as usize;
            construir(p, 0x16, 0);
            gravar(p, 0, 0);
            gravar(p, 2, cqi3_w(this + 0x439) as u16 as u32);
            let mut i = 0i16;
            while i < cqi3_w(liga + 0x3e) {
                let c = cqi3_d((cqi3_d(liga + 0xb1) as usize).wrapping_add(i as i32 as usize * 0x3b)) as usize;
                if humano(cqi3_d(0xb7431c) as usize, c) != 0 {
                    gravar(p, 1, cqi3_d(c));
                    entregar(cqi3_d(0xb7431c) as usize, p, c);
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

// ---------- 273a leva: a classificação da Conmebol: as vagas da Libertadores, o n-ésimo clube da nação e os convidados da Mercosul (conmebol_seeding.cpp) ----------

unsafe fn cqj1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cqj1_clube(id: u32) -> usize {
    (cqj1_d(0xae23b4) as usize).wrapping_add((id as usize).wrapping_mul(0x245))
}

/// O clube `c` (não nulo e ainda sem vaga, +0x1db = −1) entra na Libertadores na fase `fase`. Devolve se entrou.
unsafe fn cqj1_entra(c: usize, fase: u8) -> bool {
    if c == 0 || cqj1_d(c + 0x1db) != u32::MAX {
        return false;
    }
    *((c + 0x1db) as *mut u32) = cqj1_d(0x9cf63c);
    *((c + 0x1df) as *mut u8) = fase;
    true
}

/// { 0x4c6430 } (thiscall, ret 8, conmebol_seeding.cpp, `ConmebolSeeding::assign_libertadores_places`): as vagas da
/// Libertadores ([0x9cf63c]). O campeão da última edição (0x4afd80 com a linha da competição e o lugar 0) entra na fase 1
/// (sem ele ou já com vaga, a caixa da linha 0x114). Depois, as 11 nações da tabela (registros de 10 bytes a partir do
/// objeto: +0 id da nação, +8 a cota), cada uma até a cota, na fase 2: o Brasil, até 2002, o Palmeiras e o Cruzeiro (as
/// chaves 0x9d0274 e 0x9d026c); de 2003 em diante o campeão da Copa do Brasil (ou o vice, se o campeão já tem vaga) e o
/// campeão da Copa dos Campeões (+0x71); e sempre o campeão e o vice do Brasileirão (0x9cf9ec); cada nação completa a
/// cota com os primeiros da tabela ainda sem vaga (0x4c6730(.., 1, 1, 0)), uma tentativa por vaga.
unsafe extern "thiscall" fn conmebol_vagas_libertadores(this: usize, clubes: usize, conta: u32) {
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x4c6730usize);
    let linha = |chave: usize| (cqj1_d(0xae23d0) as usize).wrapping_add(cqj1_d(chave).wrapping_mul(0x6b) as usize);
    let h = campeao(linha(0x9cf63c), 0);
    if h != 0 && cqj1_d(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = cqj1_d(0x9cf63c);
    } else {
        aviso_em(0x9a7ed8, 0x114);
    }
    for k in 0..0xbusize {
        let e = this + k * 10;
        let mut n = 0u8;
        if cqj1_d(e) == cqj1_d(0x9cf260) {
            if *(0xae2c92 as *const u16) > 0x7d2 {
                let mut c = campeao(linha(0x9cf9e8), 0);
                if c == 0 || cqj1_d(c + 0x1db) != u32::MAX {
                    c = campeao(linha(0x9cf9e8), 1);
                }
                if cqj1_entra(c, 2) {
                    n = 1;
                }
                let id = cqj1_d(this + 0x71);
                if id != u32::MAX && cqj1_entra(cqj1_clube(id), 2) {
                    n = n.wrapping_add(1);
                }
            } else {
                if cqj1_entra(cqj1_clube(cqj1_d(0x9d0274)), 2) {
                    crate::comparacao::ramo("cqj_1 brasil fixo");
                    n = 1; // cqj1: o Palmeiras
                }
                if cqj1_entra(cqj1_clube(cqj1_d(0x9d026c)), 2) {
                    n = n.wrapping_add(1);
                }
            }
            for lugar in [0u32, 1] {
                if cqj1_entra(campeao(linha(0x9cf9ec), lugar), 2) {
                    n = n.wrapping_add(1);
                }
            }
        }
        while (n as i8) < *((e + 8) as *const i8) {
            let c = enesimo(this, clubes, conta, cqj1_d(e), 1, 1, 0);
            if c != 0 && cqj1_d(c + 0x1db) == u32::MAX {
                crate::comparacao::ramo("cqj_1 cota da nacao");
                cqj1_entra(c, 2); // cqj1: a vaga da cota
            }
            n = n.wrapping_add(1);
        }
    }
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn cqj1_dif(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Lado a lado: os clubes do jogo e a marca da caixa.
unsafe extern "thiscall" fn comparar_conmebol_vagas_libertadores(this: usize, clubes: usize, conta: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1667);
    let n = (cqj1_d(0xae235c) as i32).max(0) as usize;
    // o histórico das competições também: a 0x4afd80 o reordena (linhas empatadas: a ordem de antes muda o clube achado)
    let reg = [(cqj1_d(0xae23b4) as usize, n * 0x245), (0xb67a34, 4), (cqj1_d(0xae23d4) as usize, (cqj1_d(0xae237c) as i32).max(0) as usize * 0x1a)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clubes, conta), || conmebol_vagas_libertadores(this, clubes, conta));
    PLACAR.registrar(igual, || {
        let base = cqj1_d(0xae23b4) as usize;
        let l: Vec<String> = cqj1_dif(&mo, &mr).iter().map(|&a| {
            if a < base || a - base >= mo.0[0].1.len() {
                return format!("{:x}", a);
            }
            let (q, k) = ((a - base) / 0x245, (a - base) % 0x245);
            let (x, y) = (mo.0[0].1[a - base], mr.0[0].1[a - base]);
            format!("clube {} +{:x}: {:02x}/{:02x}", q, k, x, y)
        }).collect();
        format!("conmebol_vagas_libertadores (ano {}): {}", *(0xae2c92 as *const u16), l.join(", "))
    });
}

/// { 0x4c6730 } (thiscall, ret 0x18, conmebol_seeding.cpp, `ConmebolSeeding::find_nth_unqualified_club_of_nation`; o
/// this não é lido; o mesmo código da 0x9050d0 da UEFA): na tabela dos clubes sul-americanos (registros de 12 bytes
/// ordenados pela nação: +0 o clube, +4 o id da nação), a busca binária (bsearch 0x946464 com 0x905ce0; a chave só com a
/// nação e +8 = 0xff) acha um clube da nação; recua até o primeiro da nação; anda até o `n`-ésimo (char, 1 = o primeiro)
/// que conta, pulando, se pedido, os que já têm vaga (+0x1db ≠ −1) e os de técnico humano (0x600a50(clube, 0, 1, 0, 0)).
/// O último registro da tabela nunca é examinado pela volta (se ela chega nele com a conta zerada, ele é o escolhido sem
/// os testes). Devolve o clube se for da nação, senão 0.
unsafe extern "thiscall" fn conmebol_enesimo_clube_da_nacao(_this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
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
    while p > clubes && cqj1_d(p - 8) == nacao {
        p -= 12;
    }
    let mut falta = (n as u8).wrapping_sub(1);
    let mut achou = false;
    if p < ultimo {
        loop {
            let c = cqj1_d(p) as usize;
            if sem_vaga != 0 && c != 0 && cqj1_d(c + 0x1db) != u32::MAX {
                crate::comparacao::ramo("cqj_1 ja com vaga");
                p += 12; // cqj1: pula o clube que já tem vaga
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
    if cqj1_d(p + 4) == nacao {
        return cqj1_d(p) as usize;
    }
    0
}

/// A devolução das duas (a rotina só lê).
unsafe extern "thiscall" fn comparar_conmebol_enesimo_clube_da_nacao(this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize>(1668);
    let r = o(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    let c = conmebol_enesimo_clube_da_nacao(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    PLACAR.registrar(r == c, || format!("conmebol_enesimo_clube_da_nacao(nação {:x}, n {}, {} {}): original {:x} rust {:x}", nacao, n as u8, sem_vaga, sem_humano, r, c));
    r
}

/// Os 20 clubes convidados da Copa Mercosul (as chaves fixas: River Plate, Boca Juniors, Independiente, San Lorenzo,
/// Vélez Sarsfield, Cruzeiro, São Paulo, Palmeiras, Corinthians, Flamengo, Rosario Central, Vasco, Colo-Colo,
/// Universidad de Chile, Universidad Católica, Peñarol, Nacional, Olimpia, Cerro Porteño e Grêmio), na ordem da original.
const CQJ1_MERCOSUL: [usize; 20] = [
    0x9d00ac, 0x9d00d0, 0x9d00b4, 0x9d00d8, 0x9d00b0, 0x9d026c, 0x9d0270, 0x9d0274, 0x9d0278, 0x9d0358,
    0x9d00e0, 0x9d0354, 0x9d0280, 0x9d0284, 0x9d0288, 0x9d0298, 0x9d028c, 0x9d0290, 0x9d0294, 0x9d027c,
];

/// { 0x4c6890 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::is_mercosur_invited_club`; o this não é
/// lido): 1 se o id do clube é um dos 20 convidados da Copa Mercosul (`CQJ1_MERCOSUL`), senão 0 (sem clube, devolve o 0
/// do próprio ponteiro).
unsafe extern "thiscall" fn conmebol_convidado_da_mercosul(_this: usize, clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let id = cqj1_d(clube);
    for &chave in CQJ1_MERCOSUL.iter() {
        if id == cqj1_d(chave) {
            return 1; // cqj1: convidado
        }
    }
    0
}

/// A devolução das duas (a rotina só lê).
unsafe extern "thiscall" fn comparar_conmebol_convidado_da_mercosul(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1669);
    let r = o(this, clube);
    let c = conmebol_convidado_da_mercosul(this, clube);
    PLACAR.registrar(r == c, || format!("conmebol_convidado_da_mercosul({:x}): original {} rust {}", clube, r, c));
    r
}

// ---------- 273a leva: a classificação da Conmebol: a tabela dos clubes sul-americanos, a virada do ano e o campeão da Copa dos Campeões (conmebol_seeding.cpp) ----------

unsafe fn cqj2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// { 0x4c6280 } (stdcall, ret 4, conmebol_seeding.cpp, `collect_conmebol_clubs`): a tabela dos clubes da Conmebol: aloca
/// 12 bytes por clube do jogo (0x944e46; sem memória, a caixa da linha 0xbb e devolve 0) e grava o ponteiro em `*saida`;
/// para cada clube com nação, que não é seleção (0x53b900), do México ([0x9cf3d0]) ou de nação da América do Sul (o
/// continente +0x71 com o id de [0x9cfa1c]): tira a vaga continental (+0x1db = −1) e grava o registro (+0 o clube, +4 o
/// id da nação, +8 a reputação +0x80 / 500, +9 o +0x69 da divisão +0x5b, +0xb o +0x5f; sem divisão −1 e 0x7f; com a
/// divisão de uma competição sem o +0x11c, a divisão e o +0x5f do clube zeram). Ordena pela nação (qsort 0x945333 com
/// 0x905ce0). Devolve o número de clubes (só os 16 bits baixos contam).
unsafe extern "stdcall" fn conmebol_juntar_clubes(saida: *mut usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let t = alocar((cqj2_d(0xae235c) as i32).wrapping_mul(12) as u32);
    *saida = t;
    if t == 0 {
        aviso_em(0x9a7ed8, 0xbb);
        return 0;
    }
    let mut k = 0u32;
    let mut c = cqj2_d(0xae23b4) as usize;
    let mut i = 0u32;
    if cqj2_d(0xae235c) as i32 > 0 {
        loop {
            let nacao = cqj2_d(c + 0x53) as usize;
            if nacao != 0 && selecao(c) == 0 {
                let mexico = cqj2_d(nacao) == cqj2_d(0x9cf3d0); // cqj2: o México entra
                let cont = if mexico { 0 } else { cqj2_d(nacao + 0x71) as usize };
                if mexico || (cont != 0 && cqj2_d(cont) == cqj2_d(0x9cfa1c)) {
                    if mexico {
                        crate::comparacao::ramo("cqj_2 clube mexicano");
                    }
                    crate::comparacao::ramo("cqj_2 tira a vaga");
                    *((c + 0x1db) as *mut u32) = u32::MAX; // cqj2: sem vaga continental
                    let r = t.wrapping_add((k as u16 as i16 as i32 * 12) as usize);
                    core::ptr::write_unaligned(r as *mut u32, c as u32);
                    core::ptr::write_unaligned((r + 4) as *mut u32, cqj2_d(nacao));
                    *((r + 8) as *mut u8) = (core::ptr::read_unaligned((c + 0x80) as *const i16) as i32 / 500) as u8;
                    let divisao = cqj2_d(c + 0x5b) as usize;
                    if divisao != 0 {
                        crate::comparacao::ramo("cqj_2 nivel da divisao");
                        core::ptr::write_unaligned((r + 9) as *mut u16, core::ptr::read_unaligned((divisao + 0x69) as *const u16)); // cqj2: o nível da divisão
                        *((r + 0xb) as *mut u8) = *((c + 0x5f) as *const u8);
                        let comp = cqj2_d(cqj2_d(c + 0x5b) as usize + 0x5d) as usize;
                        if comp == 0 || *((comp + 0x11c) as *const u8) == 0 {
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
            if (i as u16 as i16 as i32) >= cqj2_d(0xae235c) as i32 {
                break;
            }
        }
    }
    ordenar(t, k as u16 as i16 as i32, 0xc, 0x905ce0);
    // ponytail: a original devolve o ax com os bytes altos do eax que sobraram do qsort (quem chama lê o ax); a cópia, zero
    k as u16 as u32
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn cqj2_dif(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Os clubes do jogo, a marca da caixa e o histórico das competições (a 0x4afd80 o reordena: ver a cqj_1).
unsafe fn cqj2_regioes_dos_clubes() -> Vec<(usize, usize)> {
    let n = (cqj2_d(0xae235c) as i32).max(0) as usize;
    vec![(cqj2_d(0xae23b4) as usize, n * 0x245), (0xb67a34, 4), (cqj2_d(0xae23d4) as usize, (cqj2_d(0xae237c) as i32).max(0) as usize * 0x1a)]
}

/// Lado a lado: a original com a saída de quem chama, a cópia com uma saída própria (liberada depois); os clubes, a
/// devolução e os registros das duas tabelas.
unsafe extern "stdcall" fn comparar_conmebol_juntar_clubes(saida: *mut usize) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(*mut usize) -> u32>(1670);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut t2 = 0usize;
    let reg = cqj2_regioes_dos_clubes();
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(saida), || conmebol_juntar_clubes(&mut t2));
    let (n, m) = (a as u16 as i16, b as u16 as i16);
    let t1 = *saida;
    let tabela = n == m && (n <= 0 || t1 == 0 || t2 == 0 || core::slice::from_raw_parts(t1 as *const u8, n as usize * 12) == core::slice::from_raw_parts(t2 as *const u8, n as usize * 12));
    if t2 != 0 {
        soltar(t2);
    }
    PLACAR.registrar(igual && tabela, || format!("conmebol_juntar_clubes: {} e {} clubes, tabela {}, diferenças em {:x?}", n, m, tabela, cqj2_dif(&mo, &mr)));
    a
}

/// { 0x4c61b0 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::update_for_new_season`): chamada pela
/// Libertadores e pela Mercosul; sem `pular` e com o ano do jogo ([0xae2c92]) outro que o +0x6f: grava o ano, monta a
/// tabela dos clubes (0x4c6280; vazia, a caixa da linha 0x9b e volta sem soltar), dá as vagas da Libertadores (0x4c6430)
/// e solta a tabela.
unsafe extern "thiscall" fn conmebol_virada_do_ano(this: usize, pular: u32) {
    if pular != 0 {
        return;
    }
    let ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
    if ano == core::ptr::read_unaligned((this + 0x6f) as *const u16) {
        return;
    }
    crate::comparacao::ramo("cqj_2 virada do ano");
    core::ptr::write_unaligned((this + 0x6f) as *mut u16, ano);
    let juntar: unsafe extern "stdcall" fn(*mut usize) -> u32 = core::mem::transmute(0x4c6280usize);
    let vagas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4c6430usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut t = 0usize;
    let n = juntar(&mut t);
    if n as u16 == 0 {
        aviso_em(0x9a7ed8, 0x9b);
        return;
    }
    vagas(this, t, n);
    soltar(t);
}

/// Lado a lado: o ano gravado (+0x6f), os clubes e a marca da caixa (cada uma monta e solta a própria tabela).
unsafe extern "thiscall" fn comparar_conmebol_virada_do_ano(this: usize, pular: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1671);
    let mut reg = cqj2_regioes_dos_clubes();
    reg.push((this + 0x6f, 2));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pular), || conmebol_virada_do_ano(this, pular));
    PLACAR.registrar(igual, || format!("conmebol_virada_do_ano({}): diferenças em {:x?}", pular, cqj2_dif(&mo, &mr)));
}

/// { 0x4c6960 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::set_bra_champ_cup_winner`): guarda o id do
/// campeão da Copa dos Campeões em +0x71 (a vaga dele na Libertadores, de 2003 em diante).
unsafe extern "thiscall" fn conmebol_campeao_dos_campeoes(this: usize, clube: usize) {
    core::ptr::write_unaligned((this + 0x71) as *mut u32, cqj2_d(clube));
}

/// Lado a lado: o +0x71.
unsafe extern "thiscall" fn comparar_conmebol_campeao_dos_campeoes(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1672);
    let reg = [(this + 0x71, 4)];
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || conmebol_campeao_dos_campeoes(this, clube));
    PLACAR.registrar(igual, || format!("conmebol_campeao_dos_campeoes({:x})", clube));
}

// ---------- 273a leva: as sedes das competições: a sede de uma edição, a próxima sede e o calendário das candidaturas (host_country.cpp) ----------

unsafe fn cqj3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cqj3_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// As duas sedes da entrada `e` (registros de 0x22 bytes: +0 id da competição, +4 ano, +6 a sede padrão e a segunda
/// (+0xa), +6 + 8 × k as candidaturas k = 0..2 (sede e segunda), +0x1e a escolhida (0..2; −3 a padrão fixa, −1 a
/// votação aberta, −2 a votação ainda fechada), +0x1f o dia do ano das decisões, +0x21 o intervalo em anos) pela
/// candidatura `k` (a padrão com `k` < 0): grava as duas e devolve quantas valem (≠ −1).
unsafe fn cqj3_sedes(e: usize, k: i8, a: *mut u32, b: *mut u32) -> u8 {
    let off = if k >= 0 { k as i32 * 8 } else { 0 } as usize;
    *a = cqj3_d(e + 6 + off);
    let segunda = cqj3_d(e + 0xa + off);
    *b = segunda;
    (*a != u32::MAX) as u8 + (segunda != u32::MAX) as u8
}

/// { 0x5fa730 } (thiscall, ret 0x14, host_country.cpp, `HostCountry::find_hosts`): as sedes da competição `comp` no ano
/// `ano` (short): a primeira entrada da competição com o ano (ou ano 0) que já tem a candidatura escolhida (as duas
/// sedes dela), a padrão fixa ou a votação aberta (a sede padrão), ou, com `padrao`, a votação ainda fechada (a padrão).
/// Grava as duas e devolve quantas valem; sem entrada, 0 sem gravar.
unsafe extern "thiscall" fn sede_da_edicao(this: usize, comp: u32, ano: u32, a: *mut u32, b: *mut u32, padrao: u32) -> u32 {
    let n = cqj3_w(this + 4) as i16;
    let mut e = cqj3_d(this) as usize;
    let mut i = 0i16;
    while i < n {
        if cqj3_d(e) == comp {
            let ae = cqj3_w(e + 4);
            if ae == ano as u16 || ae == 0 {
                let k = *((e + 0x1e) as *const i8);
                if k > -1 {
                    return cqj3_sedes(e, k, a, b) as u32;
                }
                if k == -3 || k == -1 || padrao != 0 {
                    crate::comparacao::ramo("cqj_3 sede padrao");
                    return cqj3_sedes(e, -1, a, b) as u32; // cqj3: a sede padrão
                }
            }
        }
        e += 0x22;
        i += 1;
    }
    // ponytail: a original devolve o al = 0 com os bytes altos do eax (o ponteiro da entrada); quem chama lê o al
    0
}

/// A devolução (o al) e as duas saídas (a rotina só lê).
unsafe extern "thiscall" fn comparar_sede_da_edicao(this: usize, comp: u32, ano: u32, a: *mut u32, b: *mut u32, padrao: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, *mut u32, *mut u32, u32) -> u32>(1673);
    let (va, vb) = (*a, *b);
    let r = o(this, comp, ano, a, b, padrao);
    let (oa, ob) = (*a, *b);
    let (mut ca, mut cb) = (va, vb);
    let c = sede_da_edicao(this, comp, ano, &mut ca, &mut cb, padrao);
    PLACAR.registrar(r as u8 == c as u8 && oa == ca && ob == cb, || format!("sede_da_edicao({:x}, {}): original {} {:x} {:x} rust {} {:x} {:x}", comp, ano as u16, r as u8, oa, ob, c as u8, ca, cb));
    r
}

/// { 0x5fa7d0 } (thiscall, ret 0xc, host_country.cpp, `HostCountry::find_next_hosts`): as sedes da próxima edição da
/// competição `comp`: a primeira entrada dela com a candidatura escolhida (as duas sedes) ou com a padrão fixa (a
/// padrão). Grava as duas e devolve quantas valem; sem entrada, 0 sem gravar.
unsafe extern "thiscall" fn proxima_sede(this: usize, comp: u32, a: *mut u32, b: *mut u32) -> u32 {
    let n = cqj3_w(this + 4) as i16;
    let mut e = cqj3_d(this) as usize;
    let mut i = 0i16;
    while i < n {
        if cqj3_d(e) == comp {
            let k = *((e + 0x1e) as *const i8);
            if k > -1 {
                return cqj3_sedes(e, k, a, b) as u32;
            }
            if k == -3 {
                return cqj3_sedes(e, -1, a, b) as u32;
            }
        }
        e += 0x22;
        i += 1;
    }
    // ponytail: a original devolve o al = 0 com os bytes altos do eax (o ponteiro da entrada); quem chama lê o al
    0
}

/// A devolução (o al) e as duas saídas (a rotina só lê).
unsafe extern "thiscall" fn comparar_proxima_sede(this: usize, comp: u32, a: *mut u32, b: *mut u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32) -> u32>(1674);
    let (va, vb) = (*a, *b);
    let r = o(this, comp, a, b);
    let (oa, ob) = (*a, *b);
    let (mut ca, mut cb) = (va, vb);
    let c = proxima_sede(this, comp, &mut ca, &mut cb);
    PLACAR.registrar(r as u8 == c as u8 && oa == ca && ob == cb, || format!("proxima_sede({:x}): original {} {:x} {:x} rust {} {:x} {:x}", comp, r as u8, oa, ob, c as u8, ca, cb));
    r
}

/// A chave da busca (a 0x5fb3f0 compara o ano +4 e depois o dia +0x1f, sem sinal).
fn cqj3_compara(e: usize, ano: u16, dia: u16) -> i32 {
    unsafe {
        let ae = cqj3_w(e + 4);
        if ae != ano {
            ae as i32 - ano as i32
        } else {
            cqj3_w(e + 0x1f) as i32 - dia as i32
        }
    }
}

/// A primeira entrada (de `inicio` a `fim`, ordenadas por ano e dia) com o ano e o dia da chave (bsearch 0x946464 com a
/// 0x5fb3f0 e o recuo até a primeira igual), ou 0.
unsafe fn cqj3_primeira(inicio: usize, n: i16, ano: u16, dia: u16) -> usize {
    let busca: unsafe extern "C" fn(*const u8, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let mut chave = [0u8; 0x22];
    chave[4..6].copy_from_slice(&ano.to_le_bytes());
    chave[0x1f..0x21].copy_from_slice(&dia.to_le_bytes());
    let mut p = busca(chave.as_ptr(), inicio, n as i32, 0x22, 0x5fb3f0);
    if p == 0 {
        return 0;
    }
    while p > inicio && cqj3_compara(p - 0x22, ano, dia) == 0 {
        p -= 0x22;
    }
    p
}

/// { 0x5fa850 } (thiscall, ret 4, host_country.cpp, `HostCountry::daily_update`; quadro SEH pela lista local, destruída
/// no fim pela 0x68d040): o calendário das sedes, a cada dia `hoje` (+0 o dia do ano, +2 o ano). Fora de 2001, no dia
/// 1º de julho (0x549f70(1, 6, ano, −1)) as sedes das finais europeias (0x5fad20). As entradas com o dia de hoje e o ano
/// + 8: a votação abre (−2 vira −1) e vão à lista. As com o ano + 5: vão à lista e, com a votação aberta ou fechada,
/// a candidatura vencedora é sorteada (0x90cfc0(3)). No 1º de janeiro, as entradas de anos passados (≠ 0) andam o
/// intervalo (+0x21) e, fora da padrão fixa, a votação fecha (−2); a tabela é reordenada (qsort 0x945333 com 0x5fb3f0).
/// Por fim a notícia das entradas da lista (0x5faec0).
unsafe extern "thiscall" fn calendario_das_sedes(this: usize, hoje: usize) {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let por: unsafe extern "thiscall" fn(usize, *const usize, u32) = core::mem::transmute(0x68d0d0usize);
    let data: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> *const [u16; 4] = core::mem::transmute(0x549f70usize);
    let europeias: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5fad20usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5faec0usize);
    let n = cqj3_w(this + 4) as i16;
    let inicio = cqj3_d(this) as usize;
    let fim = inicio.wrapping_add((n as i32 * 0x22 - 0x22) as usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    let ano = cqj3_w(hoje + 2);
    let dia = cqj3_w(hoje);
    if ano != 0x7d1 {
        let mut d = [0u16; 4];
        let r = *data(&mut d, 1, 6, ano as u32, u32::MAX);
        if r[1] == ano && r[0] == dia {
            europeias(this);
        }
    }
    let mut p = cqj3_primeira(inicio, n, ano.wrapping_add(8), dia);
    if p != 0 {
        while p <= fim && cqj3_compara(p, ano.wrapping_add(8), dia) == 0 {
            let k = (p + 0x1e) as *mut u8;
            if *k == 0xfe {
                *k = 0xff; // cqj3: a votação abre
            }
            if *k == 0xff {
                crate::comparacao::ramo("cqj_3 votacao aberta");
                por(l, &p, 4);
            }
            p += 0x22;
        }
    }
    let mut p = cqj3_primeira(inicio, n, ano.wrapping_add(5), dia);
    if p != 0 {
        while p <= fim && cqj3_compara(p, ano.wrapping_add(5), dia) == 0 {
            por(l, &p, 4);
            let k = (p + 0x1e) as *mut u8;
            if *k == 0xff || *k == 0xfe {
                crate::comparacao::ramo("cqj_3 sede sorteada");
                *k = sorteio(3) as u8; // cqj3: a candidatura vencedora
            }
            p += 0x22;
        }
    }
    if dia == 0 {
        let mut i = 0i16;
        while i < cqj3_w(this + 4) as i16 {
            let e = (cqj3_d(this) as usize).wrapping_add(i as i32 as usize * 0x22);
            let ae = cqj3_w(e + 4);
            if ae < ano && ae != 0 {
                crate::comparacao::ramo("cqj_3 edicao seguinte");
                core::ptr::write_unaligned((e + 4) as *mut u16, ae.wrapping_add(*((e + 0x21) as *const i8) as i16 as u16)); // cqj3: o intervalo
                if *((e + 0x1e) as *const u8) != 0xfd {
                    *((e + 0x1e) as *mut u8) = 0xfe;
                }
            }
            i += 1;
        }
        ordenar(cqj3_d(this) as usize, cqj3_w(this + 4) as i16 as i32, 0x22, 0x5fb3f0);
    }
    noticia(this, l);
    destruir(l);
}

/// Só a original (sorteia, monta a lista local e entrega notícias); a cópia leva os contadores.
unsafe extern "thiscall" fn comparar_calendario_das_sedes(this: usize, hoje: usize) {
    // ponytail: entrega notícias pela 0x5faec0 e mexe na lista: não dá para rodar duas vezes
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1675)(this, hoje)
}

// ---------- 273a leva: as regras de transferência da Inglaterra, dos EUA e da Austrália: as janelas de transferência (australia_rules.cpp, england_rules.cpp, usa_rules.cpp) ----------

unsafe fn cqk1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0xff nenhum) [inferência], +3 o dia, +4 o mês (0 = janeiro), +5 1
/// no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória, a caixa da linha
/// `linha` do fonte `arquivo`).
unsafe fn cqk1_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    *((this + 8) as *mut u8) = n;
    let j = alocar((t.len() * 6) as u32);
    core::ptr::write_unaligned((this + 4) as *mut u32, j as u32);
    if j == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, r) in t.iter().enumerate() {
        core::ptr::copy_nonoverlapping(r.as_ptr(), (j + k * 6) as *mut u8, 6);
    }
}

/// O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
unsafe fn cqk1_construir(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x8e84a0usize);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqk1_janelas(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn cqk1_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqk1_d(this + 4) as usize, cqk1_d(r + 4) as usize);
    let n = s[8] as usize * 12;
    let janelas = if ja != 0 && jb != 0 {
        core::slice::from_raw_parts(ja as *const u8, n) == core::slice::from_raw_parts(jb as *const u8, n)
    } else {
        ja == jb
    };
    if jb != 0 {
        soltar(jb);
    }
    PLACAR.registrar(campos && janelas, || format!("{}: campos {} janelas {}", nome, campos, janelas));
}

/// A janela da Inglaterra: de 2 de junho (andando para a segunda-feira [inferência]) a 26 de março (andando para a quinta-feira [inferência]).
const CQK1_INGLATERRA: [[u8; 6]; 2] = [
    [0x07, 0x00, 0x01, 0x02, 0x05, 0x01], // cqk1: a Inglaterra abre em junho
    [0x07, 0x00, 0x04, 0x1a, 0x02, 0x00],
];

/// { 0x579b30 } (thiscall, ret 0x14, england_rules.cpp, `ENGLAND_RULES::ENGLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência da Inglaterra (regra 7): a base (0x8e84a0), a vtable 0x969f4c, 1 janela(s) de `CQK1_INGLATERRA`. Devolve o objeto.
unsafe extern "thiscall" fn regras_inglaterra(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_1 inglaterra");
    cqk1_construir(this, [a1, a2, a3, a4, a5], 0x969f4c, 1, &CQK1_INGLATERRA, 0x9c4498, 0x4f);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
unsafe extern "thiscall" fn comparar_regras_inglaterra(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1676);
    let mut r = 0usize;
    cqk1_comparar("regras_inglaterra", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_inglaterra(s, a1, a2, a3, a4, a5);
    });
    r
}

/// A janela dos EUA: de 7 de janeiro (andando para a quarta-feira [inferência]) a 15 de agosto.
const CQK1_EUA: [[u8; 6]; 2] = [
    [0x19, 0x00, 0x03, 0x07, 0x00, 0x01],
    [0x19, 0x00, 0xff, 0x0f, 0x07, 0x00], // cqk1: os EUA fecham em agosto
];

/// { 0x90b940 } (thiscall, ret 0x14, usa_rules.cpp, `USA_RULES::USA_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência dos EUA (regra 25): a base (0x8e84a0), a vtable 0x970a38, 1 janela(s) de `CQK1_EUA` e o +0x13 = 4 (o padrão da base é 50) [inferência: o limite de estrangeiros]. Devolve o objeto.
unsafe extern "thiscall" fn regras_eua(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_1 eua");
    cqk1_construir(this, [a1, a2, a3, a4, a5], 0x970a38, 1, &CQK1_EUA, 0xaa211c, 0x54);
    *((this + 0x13) as *mut u8) = 4;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
unsafe extern "thiscall" fn comparar_regras_eua(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1677);
    let mut r = 0usize;
    cqk1_comparar("regras_eua", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_eua(s, a1, a2, a3, a4, a5);
    });
    r
}

/// A janela da Austrália: de 11 de junho (andando para o sábado [inferência]) a 17 de abril.
const CQK1_AUSTRALIA: [[u8; 6]; 2] = [
    [0x02, 0x00, 0x06, 0x0b, 0x05, 0x01], // cqk1: a Austrália abre em junho
    [0x02, 0x00, 0xff, 0x11, 0x03, 0x00],
];

/// { 0x412c80 } (thiscall, ret 0x14, australia_rules.cpp, `AUSTRALIA_RULES::Australia_rules`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência da Austrália (regra 2): a base (0x8e84a0), a vtable 0x967830, 1 janela(s) de `CQK1_AUSTRALIA` e o +0x13 = 6 [inferência: o limite de estrangeiros]. Devolve o objeto.
unsafe extern "thiscall" fn regras_australia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_1 australia");
    cqk1_construir(this, [a1, a2, a3, a4, a5], 0x967830, 1, &CQK1_AUSTRALIA, 0x987cfc, 0x51);
    *((this + 0x13) as *mut u8) = 6;
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk1_comparar`).
unsafe extern "thiscall" fn comparar_regras_australia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1678);
    let mut r = 0usize;
    cqk1_comparar("regras_australia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_australia(s, a1, a2, a3, a4, a5);
    });
    r
}

// ---------- 273a leva: as regras de transferência da Espanha, da Itália e da França: as janelas de transferência (france_rules.cpp, italy_rules.cpp, spain_rules.cpp) ----------

unsafe fn cqk2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0xff nenhum) [inferência], +3 o dia, +4 o mês (0 = janeiro), +5 1
/// no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória, a caixa da linha
/// `linha` do fonte `arquivo`).
unsafe fn cqk2_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    *((this + 8) as *mut u8) = n;
    let j = alocar((t.len() * 6) as u32);
    core::ptr::write_unaligned((this + 4) as *mut u32, j as u32);
    if j == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, r) in t.iter().enumerate() {
        core::ptr::copy_nonoverlapping(r.as_ptr(), (j + k * 6) as *mut u8, 6);
    }
}

/// O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
unsafe fn cqk2_construir(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x8e84a0usize);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqk2_janelas(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn cqk2_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqk2_d(this + 4) as usize, cqk2_d(r + 4) as usize);
    let n = s[8] as usize * 12;
    let janelas = if ja != 0 && jb != 0 {
        core::slice::from_raw_parts(ja as *const u8, n) == core::slice::from_raw_parts(jb as *const u8, n)
    } else {
        ja == jb
    };
    if jb != 0 {
        soltar(jb);
    }
    PLACAR.registrar(campos && janelas, || format!("{}: campos {} janelas {}", nome, campos, janelas));
}

/// As janelas da Espanha: de 17 de junho a 20 de agosto e de 15 de dezembro a 31 de janeiro.
const CQK2_ESPANHA: [[u8; 6]; 4] = [
    [0x16, 0x00, 0x03, 0x11, 0x05, 0x01],
    [0x16, 0x00, 0x04, 0x14, 0x07, 0x00], // cqk2: a Espanha fecha em agosto
    [0x16, 0x01, 0xff, 0x0f, 0x0b, 0x01],
    [0x16, 0x01, 0xff, 0x1f, 0x00, 0x00],
];

/// { 0x859470 } (thiscall, ret 0, spain_rules.cpp, `SPAIN_RULES::init_transfer_windows`; chamada pelo construtor das regras da Espanha (regra 22), depois da base e da vtable): as
/// 2 janelas de transferência de `CQK2_ESPANHA`.
unsafe extern "thiscall" fn janelas_espanha(this: usize) {
    crate::comparacao::ramo("cqk_2 espanha");
    cqk2_janelas(this, 2, &CQK2_ESPANHA, 0xa82ac8, 0x54);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk2_comparar`).
unsafe extern "thiscall" fn comparar_janelas_espanha(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1679);
    cqk2_comparar("janelas_espanha", this, || o(this), |s| janelas_espanha(s));
}

/// As janelas da Itália: de 1º de julho a 27 de setembro (andando para a quinta-feira [inferência]), de 2 a 31 de janeiro, e a terceira, de 1º de julho a 30 de abril [inferência: a dos jogadores sem contrato].
const CQK2_ITALIA: [[u8; 6]; 6] = [
    [0x0e, 0x00, 0xff, 0x01, 0x06, 0x01],
    [0x0e, 0x00, 0x04, 0x1b, 0x09, 0x00], // cqk2: a Itália fecha em setembro
    [0x0e, 0x01, 0xff, 0x02, 0x00, 0x01],
    [0x0e, 0x01, 0xff, 0x1f, 0x00, 0x00],
    [0x0e, 0x02, 0xff, 0x01, 0x06, 0x01],
    [0x0e, 0x02, 0xff, 0x1e, 0x03, 0x00],
];

/// { 0x6613f0 } (thiscall, ret 0, italy_rules.cpp, `ITALY_RULES::init_transfer_windows`; chamada pelo construtor das regras da Itália (regra 14), depois da base e da vtable): as
/// 3 janelas de transferência de `CQK2_ITALIA`.
unsafe extern "thiscall" fn janelas_italia(this: usize) {
    crate::comparacao::ramo("cqk_2 italia");
    cqk2_janelas(this, 3, &CQK2_ITALIA, 0x9ea9a4, 0x52);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk2_comparar`).
unsafe extern "thiscall" fn comparar_janelas_italia(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1680);
    cqk2_comparar("janelas_italia", this, || o(this), |s| janelas_italia(s));
}

/// As janelas da França: de 30 de maio a 31 de agosto, de 1º de setembro a 20 de dezembro e de 21 de dezembro a 10 de janeiro (com os dias da semana para os quais as datas andam [inferência]).
const CQK2_FRANCA: [[u8; 6]; 6] = [
    [0x09, 0x00, 0x06, 0x1e, 0x04, 0x01],
    [0x09, 0x00, 0x03, 0x1f, 0x07, 0x00], // cqk2: a França fecha em agosto
    [0x09, 0x01, 0x04, 0x01, 0x08, 0x01],
    [0x09, 0x01, 0x04, 0x14, 0x0b, 0x00],
    [0x09, 0x02, 0x05, 0x15, 0x0b, 0x01],
    [0x09, 0x02, 0x04, 0x0a, 0x00, 0x00],
];

/// { 0x5c15e0 } (thiscall, ret 0, france_rules.cpp, `FRANCE_RULES::init_transfer_windows`; chamada pelo construtor das regras da França (regra 9), depois da base e da vtable): as
/// 3 janelas de transferência de `CQK2_FRANCA`.
unsafe extern "thiscall" fn janelas_franca(this: usize) {
    crate::comparacao::ramo("cqk_2 franca");
    cqk2_janelas(this, 3, &CQK2_FRANCA, 0x9cccf4, 0x4f);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk2_comparar`).
unsafe extern "thiscall" fn comparar_janelas_franca(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1681);
    cqk2_comparar("janelas_franca", this, || o(this), |s| janelas_franca(s));
}

// ---------- 273a leva: as regras de transferência do Brasil, da Alemanha e da Escócia: as janelas de transferência (brazil_rules.cpp, germany_rules.cpp, scotland_rules.cpp) ----------

unsafe fn cqk3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As janelas de transferência: +8 = o número de janelas; a tabela (`t`, datas de 6 bytes: +0 a regra, +1 o número da
/// janela, +2 o dia da semana para o qual a data anda (0xff nenhum) [inferência], +3 o dia, +4 o mês (0 = janeiro), +5 1
/// no início da janela e 0 no fim; duas datas por janela) alocada em +4 (0x944e46; sem memória, a caixa da linha
/// `linha` do fonte `arquivo`).
unsafe fn cqk3_janelas(this: usize, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    *((this + 8) as *mut u8) = n;
    let j = alocar((t.len() * 6) as u32);
    core::ptr::write_unaligned((this + 4) as *mut u32, j as u32);
    if j == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, r) in t.iter().enumerate() {
        core::ptr::copy_nonoverlapping(r.as_ptr(), (j + k * 6) as *mut u8, 6);
    }
}

/// O construtor das regras de uma nação: a base (0x8e84a0, original, thiscall com os 5 argumentos de quem chama), a
/// vtable da nação e as janelas.
unsafe fn cqk3_construir(this: usize, a: [u32; 5], vtable: u32, n: u8, t: &[[u8; 6]], arquivo: usize, linha: u32) {
    let base: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x8e84a0usize);
    base(this, a[0], a[1], a[2], a[3], a[4]);
    core::ptr::write_unaligned(this as *mut u32, vtable);
    cqk3_janelas(this, n, t, arquivo, linha);
}

/// Lado a lado à mão: a original no objeto de quem chama; a cópia num objeto de rascunho com o mesmo conteúdo de antes
/// (os 0x19 bytes das regras); iguais os campos (fora o ponteiro +4) e as janelas alocadas (a do rascunho é solta).
unsafe fn cqk3_comparar(nome: &str, this: usize, o: impl FnOnce(), c: impl FnOnce(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x19);
    o();
    let r = s.as_mut_ptr() as usize;
    c(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x19);
    let campos = a[..4] == s[..4] && a[8..] == s[8..0x19];
    let (ja, jb) = (cqk3_d(this + 4) as usize, cqk3_d(r + 4) as usize);
    let n = s[8] as usize * 12;
    let janelas = if ja != 0 && jb != 0 {
        core::slice::from_raw_parts(ja as *const u8, n) == core::slice::from_raw_parts(jb as *const u8, n)
    } else {
        ja == jb
    };
    if jb != 0 {
        soltar(jb);
    }
    PLACAR.registrar(campos && janelas, || format!("{}: campos {} janelas {}", nome, campos, janelas));
}

/// A janela do Brasil: de 11 de dezembro (andando para a quarta-feira [inferência]) a 8 de novembro (andando para a sexta-feira [inferência]).
const CQK3_BRASIL: [[u8; 6]; 2] = [
    [0x04, 0x00, 0x03, 0x0b, 0x0b, 0x01], // cqk3: o Brasil abre em dezembro
    [0x04, 0x00, 0x05, 0x08, 0x0a, 0x00], // cqk3: o Brasil fecha em novembro
];

/// { 0x43f0e0 } (thiscall, ret 0x14, brazil_rules.cpp, `BRAZIL_RULES::BRAZIL_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência do Brasil (regra 4): a base (0x8e84a0), a vtable 0x968838, 1 janela(s) de `CQK3_BRASIL` e o +0x13 = 3 (o padrão da base é 50) [inferência: o limite de estrangeiros contratados]. Devolve o objeto.
unsafe extern "thiscall" fn regras_brasil(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_3 brasil");
    cqk3_construir(this, [a1, a2, a3, a4, a5], 0x968838, 1, &CQK3_BRASIL, 0x98b104, 0x50);
    *((this + 0x13) as *mut u8) = 3; // cqk3: o limite do Brasil
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk3_comparar`).
unsafe extern "thiscall" fn comparar_regras_brasil(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1682);
    let mut r = 0usize;
    cqk3_comparar("regras_brasil", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_brasil(s, a1, a2, a3, a4, a5);
    });
    r
}

/// As janelas da Alemanha: de 1º de julho a 15 de agosto e de 1º de julho a 15 de janeiro.
const CQK3_ALEMANHA: [[u8; 6]; 4] = [
    [0x0a, 0x00, 0xff, 0x01, 0x06, 0x01],
    [0x0a, 0x00, 0xff, 0x0f, 0x07, 0x00],
    [0x0a, 0x01, 0xff, 0x01, 0x06, 0x01],
    [0x0a, 0x01, 0xff, 0x0f, 0x00, 0x00],
];

/// { 0x5dffe0 } (thiscall, ret 0, germany_rules.cpp, `GERMANY_RULES::init_transfer_windows`; chamada pelo construtor das regras da Alemanha (regra 10), depois da base e da vtable): as
/// 2 janelas de transferência de `CQK3_ALEMANHA`.
unsafe extern "thiscall" fn janelas_alemanha(this: usize) {
    crate::comparacao::ramo("cqk_3 alemanha");
    cqk3_janelas(this, 2, &CQK3_ALEMANHA, 0x9cd538, 0x4d);
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk3_comparar`).
unsafe extern "thiscall" fn comparar_janelas_alemanha(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1683);
    cqk3_comparar("janelas_alemanha", this, || o(this), |s| janelas_alemanha(s));
}

/// A janela da Escócia: de 30 de maio (andando para o sábado [inferência]) a 30 de março (andando para a quinta-feira [inferência]).
const CQK3_ESCOCIA: [[u8; 6]; 2] = [
    [0x15, 0x00, 0x06, 0x1e, 0x04, 0x01],
    [0x15, 0x00, 0x04, 0x1e, 0x02, 0x00],
];

/// { 0x7f6390 } (thiscall, ret 0x14, scotland_rules.cpp, `SCOTLAND_RULES::SCOTLAND_RULES`; quadro SEH só do `new`, sem destrutor no caminho normal): as regras de
/// transferência da Escócia (regra 21): a base (0x8e84a0), a vtable 0x96f838, 1 janela(s) de `CQK3_ESCOCIA` e o dword +0x15 = 1. Devolve o objeto.
unsafe extern "thiscall" fn regras_escocia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    crate::comparacao::ramo("cqk_3 escocia");
    cqk3_construir(this, [a1, a2, a3, a4, a5], 0x96f838, 1, &CQK3_ESCOCIA, 0xa7b058, 0x50);
    core::ptr::write_unaligned((this + 0x15) as *mut u32, 1);
    this
}

/// A original no objeto de quem chama, a cópia num rascunho (ver `cqk3_comparar`).
unsafe extern "thiscall" fn comparar_regras_escocia(this: usize, a1: u32, a2: u32, a3: u32, a4: u32, a5: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> usize>(1684);
    let mut r = 0usize;
    cqk3_comparar("regras_escocia", this, || r = o(this, a1, a2, a3, a4, a5), |s| {
        regras_escocia(s, a1, a2, a3, a4, a5);
    });
    r
}

// ---------- 273a leva: a Copa Intertoto e a Copa da UEFA: os três vencedores do Intertoto na Copa da UEFA e o destino de cada clube nas duas (intertoto_cup.cpp, uefa_cup.cpp) ----------

unsafe fn cqi10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqi10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqi10_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O objeto da Copa da UEFA ([[0xadadfc] + 4 × [0x9cf6f0]]), ou 0.
unsafe fn cqi10_uefa() -> usize {
    cqi10_d(cqi10_d(0xadadfc).wrapping_add(cqi10_d(0x9cf6f0).wrapping_mul(4)))
}

/// Os vencedores do Intertoto (+5 = 1 no registro de 6 bytes de +0xa7) que cabem nas três últimas vagas da UEFA, na
/// ordem em que a 0x633d40 os põe: as vagas vão de +0x3e da UEFA − 27 até + 3 (palavras com sinal).
unsafe fn cqi10_vencedores(this: usize, uefa: usize) -> ([usize; 3], usize) {
    let mut v = [0usize; 3];
    let mut k = 0usize;
    let mut vaga = cqi10_w(uefa + 0x3e).wrapping_sub(0x1b);
    let limite = vaga.wrapping_add(3);
    let mut i = 0i16;
    if cqi10_w(this + 0x3e) > 0 {
        loop {
            if vaga >= limite {
                break;
            }
            let e = cqi10_d(this + 0xa7).wrapping_add((i as isize * 6) as usize);
            if cqi10_b(e + 5) == 1 {
                v[k] = cqi10_d(e);
                k += 1;
                vaga = vaga.wrapping_add(1);
            }
            i = i.wrapping_add(1);
            if i >= cqi10_w(this + 0x3e) {
                break;
            }
        }
    }
    (v, k)
}

/// { 0x633d40 } (thiscall, ret 8, intertoto_cup.cpp; a virtual +0x14 do Intertoto): `IntertotoCup::update_for_date(hoje,
/// fase)`. Uma vez (+0xb2 = 0), quando a copa acabou (virtual +0x10 com 0, 1: push 1; push 0; mov ecx, this ->
/// thiscall, ret 8; o byte de baixo): sem a Copa da UEFA, a caixa da linha 0x29e; com ela, cada time do Intertoto com
/// +5 = 1 (os vencedores das finais) vai, em ordem, para as vagas de +0x3e − 27 em diante da tabela +0xa7 da UEFA (até
/// três: o clube, +4 e +5 zerados) e para o livro de inscritos dela (+0x14, 6 bytes: o clube e a situação 4 em +4;
/// +0x36 sobe); depois as posições previstas da UEFA (virtual +0x5c: mov ecx, uefa -> thiscall, ret 0), +0xb2 = 1 e de
/// novo as posições previstas (a original relê a UEFA). Sempre, no fim, a atualização da copa (0x51f890: push fase;
/// push hoje; mov ecx, this -> thiscall, ret 8), cuja devolução sai.
unsafe extern "thiscall" fn cqi10_intertoto_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    if cqi10_d(this + 0xb2) == 0 {
        let acabou: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(cqi10_d(cqi10_d(this) + 0x10));
        if acabou(this, 0, 1) as u8 != 0 {
            let uefa = cqi10_uefa();
            if uefa == 0 {
                aviso_em(0x9e9e14, 0x29e);
            } else {
                let mut vaga = cqi10_w(uefa + 0x3e).wrapping_sub(0x1b);
                let limite = vaga.wrapping_add(3);
                let mut i = 0i16;
                if cqi10_w(this + 0x3e) > 0 {
                    loop {
                        if vaga >= limite {
                            break;
                        }
                        let e = cqi10_d(this + 0xa7).wrapping_add((i as isize * 6) as usize);
                        if cqi10_b(e + 5) == 1 {
                            crate::comparacao::ramo("cqi_10 vencedor na uefa");
                            let s = cqi10_d(uefa + 0xa7).wrapping_add((vaga as isize * 6) as usize);
                            core::ptr::write_unaligned(s as *mut u32, cqi10_d(e) as u32);
                            *((s + 4) as *mut u8) = 0;
                            *((s + 5) as *mut u8) = 0;
                            let l = cqi10_d(uefa + 0x14).wrapping_add((cqi10_w(uefa + 0x36) as isize * 6) as usize);
                            core::ptr::write_unaligned(l as *mut u32, cqi10_d(s) as u32);
                            *((l + 4) as *mut u8) = 4; // cqi10: a situação do vencedor do Intertoto no livro da UEFA
                            core::ptr::write_unaligned((uefa + 0x36) as *mut i16, cqi10_w(uefa + 0x36).wrapping_add(1));
                            vaga = vaga.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                        if i >= cqi10_w(this + 0x3e) {
                            break;
                        }
                    }
                }
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqi10_d(cqi10_d(uefa) + 0x5c));
                prever(uefa);
                core::ptr::write_unaligned((this + 0xb2) as *mut u32, 1);
                let uefa = cqi10_uefa();
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqi10_d(cqi10_d(uefa) + 0x5c));
                prever(uefa);
                let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51f890usize);
                return base(this, hoje, fase);
            }
            core::ptr::write_unaligned((this + 0xb2) as *mut u32, 1);
        }
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51f890usize);
    base(this, hoje, fase)
}

// ponytail: a atualização da copa (0x51f890) joga e sorteia: não roda duas vezes. Só a original roda; o plano, feito
// antes: se +0xb2 passa de 0 a 1 com a Copa da UEFA, o livro dela (+0x14) ganha os vencedores (cqi10_vencedores), em
// ordem e com a situação 4, e +0x36 sobe tanto quanto; senão o livro não cresce.
unsafe extern "thiscall" fn comparar_cqi10_intertoto_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1685);
    let uefa = cqi10_uefa();
    if uefa == 0 || cqi10_d(this + 0xb2) != 0 {
        return o(this, hoje, fase);
    }
    let (v, k) = cqi10_vencedores(this, uefa);
    let n36 = cqi10_w(uefa + 0x36);
    let r = o(this, hoje, fase);
    let passou = cqi10_d(this + 0xb2) == 1;
    let previsto = if passou { n36.wrapping_add(k as i16) } else { n36 };
    let mut igual = cqi10_w(uefa + 0x36) == previsto;
    if passou && igual {
        for (j, &c) in v.iter().enumerate().take(k) {
            let l = cqi10_d(uefa + 0x14).wrapping_add(((n36 as isize) + j as isize) as usize * 6);
            igual &= cqi10_d(l) == c && cqi10_b(l + 4) == 4;
        }
    }
    PLACAR.registrar(igual, || format!("cqi10_intertoto_dia({:#x}) passou={} livro {} -> {} previsto {} vencedores {:x?}", this, passou, n36, cqi10_w(uefa + 0x36), previsto, &v[..k]));
    r
}

/// { 0x633c10 } (thiscall, ret 0x18, intertoto_cup.cpp; a virtual +0x44 do Intertoto): `IntertotoCup::
/// update_staff_records_for_status(clube, desfecho, _, _, jogo, _)`; devolve 0. Peso = 0x4bf850(baixo, alto, rodada do
/// jogo +0x34, rodadas +0x3c) (cdecl, 4 pushes; add esp, 0x10).
/// - 1 (venceu a final): a classificação (0x86bdd0) para a Copa da UEFA (a competição [0xae23d0] + 0x6b × [0x9cf6f0]),
///   fase 0, rodada 0xa, peso(0x3c, 0x64), e o clube passa à Copa da UEFA (+0x1db = [0x9cf6f0]).
/// - 0 (passou de rodada): se a rodada seguinte existe (+0x34 + 1 < +0x3c), a classificação no Intertoto (+4) com a
///   fase +0x32 do jogo, o +7 da rodada seguinte (+0xa3, 0x68 bytes) e o peso rodada × 99 / (rodadas − 1) + 1 (só o
///   byte de baixo sobe: inc al).
/// - 0xff ou 2 (caiu): a eliminação (0x86c000) no Intertoto, fase 0, rodada +0x30 do jogo, peso(0x14, 0x50), e o clube
///   sai da competição europeia (+0x1db = −1).
unsafe extern "thiscall" fn cqi10_intertoto_registros(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    match desfecho as u8 {
        1 => {
            let peso = cpz4_escala(0x3c, 0x64, cqi10_w(jogo + 0x34), cqi10_w(this + 0x3c) as u16 as u32);
            let uefa = cqi10_d(0xae23d0).wrapping_add(cqi10_d(0x9cf6f0).wrapping_mul(0x6b));
            cpz4_classificado(clube, uefa, 0, 0xa, peso);
            crate::comparacao::ramo("cqi_10 vencedor do intertoto");
            core::ptr::write_unaligned((clube + 0x1db) as *mut u32, cqi10_d(0x9cf6f0) as u32); // cqi10: o vencedor passa à UEFA
        }
        0 => {
            let r = cqi10_w(jogo + 0x34);
            let n = cqi10_w(this + 0x3c);
            let seguinte = r.wrapping_add(1);
            if seguinte >= n {
                return 0;
            }
            let div = n as i32 - 1;
            if div == 0 {
                return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1686)(this, clube, desfecho, a3, a4, jogo, a6); // ponytail: a original cai na divisão
            }
            let q = (r as i32 * 99) / div;
            let peso = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32; // inc al
            let rodada = core::ptr::read_unaligned((cqi10_d(this + 0xa3).wrapping_add((seguinte as isize * 0x68) as usize) + 7) as *const u16) as u32;
            // ponytail: a fase vai num registrador cujos 16 bits altos são os do movsx da rodada seguinte (zero); limpa
            cpz4_classificado(clube, cqi10_d(this + 4), core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32, rodada, peso);
        }
        0xff | 2 => {
            let peso = cpz4_escala(0x14, 0x50, cqi10_w(jogo + 0x34), cqi10_w(this + 0x3c) as u16 as u32);
            // ponytail: a rodada +0x30 vai num registrador com lixo nos 16 bits altos; a 0x86c000 só usa a palavra
            cpz4_eliminado(clube, cqi10_d(this + 4), 0, core::ptr::read_unaligned((jogo + 0x30) as *const u16) as u32, peso);
            crate::comparacao::ramo("cqi_10 caiu no intertoto");
            cpz4_sair(clube); // cqi10: quem cai no Intertoto sai da competição europeia
        }
        _ => {}
    }
    0
}

// ponytail: os registros vão para o histórico das pessoas (reputação, moral): não rodam duas vezes. Só a original roda;
// o plano confere o +0x1db do clube: [0x9cf6f0] para o vencedor, −1 para quem cai, intacto nos outros.
unsafe extern "thiscall" fn comparar_cqi10_intertoto_registros(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1686);
    if clube == 0 {
        return o(this, clube, desfecho, a3, a4, jogo, a6);
    }
    let antes = cqi10_d(clube + 0x1db);
    let previsto = match desfecho as u8 {
        1 => cqi10_d(0x9cf6f0),
        0xff | 2 => 0xffff_ffff,
        _ => antes,
    };
    let r = o(this, clube, desfecho, a3, a4, jogo, a6);
    let depois = cqi10_d(clube + 0x1db);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("cqi10_intertoto_registros({:#x}, desfecho {}) +0x1db {:x} -> {:x}, previsto {:x}, devolveu {}", clube, desfecho as u8, antes, depois, previsto, r));
    r
}

/// { 0x9030e0 } (thiscall, ret 0x18, uefa_cup.cpp; a virtual +0x44 da Copa da UEFA): `UefaCup::
/// update_staff_records_for_status(clube, desfecho, a3, a4, jogo, a6)`: com o desfecho (byte) 1, 2 ou 0xff (campeão,
/// vice ou eliminado) o clube sai da competição europeia (+0x1db = −1); depois os registros da base (0x519890: os seis
/// argumentos como chegaram, o desfecho como dword; mov ecx, this -> thiscall, ret 0x18), cuja devolução sai.
unsafe extern "thiscall" fn cqi10_uefa_registros(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let d = desfecho as u8;
    if d == 1 || d == 2 || d == 0xff {
        if d == 0xff {
            crate::comparacao::ramo("cqi_10 caiu na uefa");
        }
        core::ptr::write_unaligned((clube + 0x1db) as *mut i32, -1); // cqi10: quem termina na UEFA sai da competição europeia
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x519890usize);
    base(this, clube, desfecho, a3, a4, jogo, a6)
}

// ponytail: a base grava os registros (histórico, reputação, moral) e pode dar a vaga da UEFA ao campeão: não roda duas
// vezes. Só a original roda; o plano confere o +0x1db do clube (−1 nos desfechos 1, 2 e 0xff, intacto nos outros).
unsafe extern "thiscall" fn comparar_cqi10_uefa_registros(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1687);
    if clube == 0 {
        return o(this, clube, desfecho, a3, a4, jogo, a6);
    }
    let d = desfecho as u8;
    let antes = cqi10_d(clube + 0x1db);
    let previsto = if d == 1 || d == 2 || d == 0xff { 0xffff_ffff } else { antes };
    let r = o(this, clube, desfecho, a3, a4, jogo, a6);
    let depois = cqi10_d(clube + 0x1db);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("cqi10_uefa_registros({:#x}, desfecho {}) +0x1db {:x} -> {:x}, previsto {:x}, devolveu {}", clube, d, antes, depois, previsto, r));
    r
}

// ---------- 273a leva: a Copa dos Campeões da CONCACAF: os oito classificados, o dia da copa e as rodadas (con_champ.cpp) ----------

unsafe fn cqi11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqi11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqi11_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro da competição `id` no banco ([0xae23d0] + 0x6b × id).
unsafe fn cqi11_comp(id: usize) -> usize {
    cqi11_d(0xae23d0).wrapping_add(id.wrapping_mul(0x6b))
}

/// As quatro listas da 0x4c11a0, na ordem do banco de clubes ([0xae23b4], 0x245 bytes, [0xae235c] clubes; para no
/// primeiro que é seleção, 0x53b900): (mexicanos, centro-americanos, caribenhos, da MLS) com as contagens.
struct Cqi11Listas {
    mex: [usize; 200],
    centro: [usize; 200],
    caribe: [usize; 200],
    mls: [usize; 12],
    n: [u32; 4],
}

unsafe fn cqi11_listas() -> Cqi11Listas {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    // ponytail: as quatro listas começam zeradas na cópia (na original, sobra de pilha que só é lida quando falta clube;
    // o comparador manda esse caso para a original sozinha)
    let mut l = Cqi11Listas { mex: [0; 200], centro: [0; 200], caribe: [0; 200], mls: [0; 12], n: [0; 4] };
    let (mut a, mut b, mut c, mut m) = (0u8, 0u8, 0u8, 0i8);
    let mut clube = cqi11_d(0xae23b4);
    let mut i = 0i32;
    while i < cqi11_d(0xae235c) as i32 {
        if selecao(clube) != 0 {
            break;
        }
        let nacao = cqi11_d(clube + 0x53);
        if nacao != 0 {
            let id = cqi11_d(nacao);
            if id == cqi11_d(0x9cf3d0) {
                if a < 0xc8 {
                    l.mex[a as usize] = clube;
                    a += 1;
                }
            } else {
                let div = cqi11_d(clube + 0x57);
                if div != 0 && cqi11_d(div) == cqi11_d(0x9cf590) {
                    if m < 0xc {
                        l.mls[m as usize] = clube;
                        m += 1;
                    }
                } else if id != cqi11_d(0x9cf4f8) && cqi11_b(nacao + 0x75) == 7 {
                    if b < 0xc8 {
                        l.centro[b as usize] = clube;
                        b += 1;
                    }
                } else if cqi11_b(nacao + 0x75) == 6 && c < 0xc8 {
                    l.caribe[c as usize] = clube;
                    c += 1;
                }
            }
        }
        i += 1;
        clube = clube.wrapping_add(0x245);
    }
    l.n = [a as u32, b as u32, c as u32, m as u8 as u32];
    l
}

/// { 0x4c11a0 } (thiscall, ret 0, con_champ.cpp): `ConChamp::select_qualified_clubs()`; devolve 1. Oito times (+0x3e).
/// Os clubes (até a primeira seleção) em quatro listas: os da nação [0x9cf3d0] (México, até 200); os da divisão
/// [0x9cf590] (a MLS, +0x57 do clube; até 12); os de nação de região 7 (+0x75 da nação, América Central) fora da
/// [0x9cf4f8] (até 200); os de região 6 (o Caribe, até 200). Cada lista ordenada pela reputação, com os 3 × k
/// primeiros embaralhados (0x4afbd0: push k; push n; push lista; add esp, 0xc -> cdecl; k = 2, 3, 1 e 1, a contagem
/// na palavra de baixo). Os times (+0xa7, 6 bytes): 2 e 3 os dois mexicanos; 4, 5 e 6 os três centro-americanos; 7 o
/// caribenho; 0 e 1: sem o objeto da competição [0x9cf728], os dois primeiros da MLS; com ele, o último campeão da MLS
/// (0x4afd80(competição [0x9cf590], 0): push 0; push comp; add esp, 8 -> cdecl) e o da [0x9cf728] (a mesma com +4 do
/// objeto) e, se for o mesmo clube, o 0 passa ao último vice da MLS (lugar 1). Depois +4 e +5 de cada time zerados.
unsafe extern "thiscall" fn cqi11_concacaf_classificados(this: usize) -> u32 {
    let ordenar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 8);
    let outro = cqi11_d(cqi11_d(0xadadfc).wrapping_add(cqi11_d(0x9cf728).wrapping_mul(4)));
    let mut l = cqi11_listas();
    ordenar(l.mex.as_mut_ptr() as usize, l.n[0], 2);
    ordenar(l.centro.as_mut_ptr() as usize, l.n[1], 3); // cqi11: três centro-americanos
    ordenar(l.caribe.as_mut_ptr() as usize, l.n[2], 1);
    ordenar(l.mls.as_mut_ptr() as usize, l.n[3], 1);
    let t = |k: usize| cqi11_d(this + 0xa7).wrapping_add(k * 6);
    let pd = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut u32, v as u32);
    crate::comparacao::ramo("cqi_11 classificados");
    pd(t(2), l.mex[0]);
    pd(t(3), l.mex[1]); // cqi11: o segundo mexicano
    pd(t(4), l.centro[0]);
    pd(t(5), l.centro[1]);
    pd(t(6), l.centro[2]);
    pd(t(7), l.caribe[0]);
    if outro == 0 {
        pd(t(0), l.mls[0]);
        pd(t(1), l.mls[1]);
    } else {
        crate::comparacao::ramo("cqi_11 campeao da mls");
        let mls = cqi11_comp(cqi11_d(0x9cf590));
        pd(t(0), colocado(mls, 0));
        pd(t(1), colocado(cqi11_d(outro + 4), 0));
        if cqi11_d(t(0)) == cqi11_d(t(1)) { // cqi11: o campeão das duas dá a vaga ao vice da MLS
            pd(t(0), colocado(mls, 1));
        }
    }
    let mut i = 0i32;
    while i < cqi11_w(this + 0x3e) as i32 {
        *((cqi11_d(this + 0xa7) + i as usize * 6 + 4) as *mut u8) = 0;
        *((cqi11_d(this + 0xa7) + i as usize * 6 + 5) as *mut u8) = 0;
        i += 1;
    }
    1
}

/// Comparador da 0x4c11a0: lado a lado nos oito times (+0xa7, 48 bytes), em +0x3e e no estado do sorteador (a 0x4afbd0
/// embaralha; a cópia sem contar). Quando falta clube numa lista (menos de 2 mexicanos, 3 centro-americanos, 1
/// caribenho, ou 2 da MLS sem o objeto [0x9cf728]) a original lê sobra de pilha: só a original.
unsafe extern "thiscall" fn comparar_cqi11_concacaf_classificados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1688);
    let l = cqi11_listas();
    let outro = cqi11_d(cqi11_d(0xadadfc).wrapping_add(cqi11_d(0x9cf728).wrapping_mul(4)));
    if l.n[0] < 2 || l.n[1] < 3 || l.n[2] < 1 || (outro == 0 && l.n[3] < 2) || cqi11_d(this + 0xa7) == 0 {
        return o(this); // ponytail: lista curta (a original lê sobra de pilha) ou sem tabela: só a original
    }
    let mut regioes = vec![(this + 0x3e, 2), (cqi11_d(this + 0xa7), 48)];
    regioes.extend_from_slice(&crate::aleatorio::REGIOES_DO_ESTADO);
    // a 0x4afd80 reordena o histórico de competições ([0xae23d4], [0xae237c] linhas de 0x1a bytes) a cada chamada; com
    // empate, a ordem que a original deixou muda o campeão que a cópia acharia: o histórico entra nas regiões
    let historico = cqi11_d(0xae23d4);
    if historico != 0 {
        regioes.push((historico, cqi11_d(0xae237c).wrapping_mul(0x1a)));
    }
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this), || crate::aleatorio::sem_contar(|| cqi11_concacaf_classificados(this)));
    PLACAR.registrar(igual && r == rr, || format!("cqi11_concacaf_classificados({:#x}) listas {:?} original={:x?} rust={:x?}", this, l.n, mo, mr));
    r
}

/// { 0x4c1620 } (thiscall, ret 8, con_champ.cpp; a virtual +0x14 da copa): `ConChamp::update_for_date(hoje, fase)`.
/// Com a tabela de times vazia (o clube do time 0 = 0) e fase ≠ 0: os classificados (0x4c11a0: mov ecx, this ->
/// thiscall, ret 0; negativo, a caixa da linha 0x1ba e devolve 0) e as posições previstas (virtual +0x5c). Depois a
/// atualização da copa (0x51f890: push fase; push hoje; mov ecx, this -> thiscall, ret 8), cuja devolução sai.
unsafe extern "thiscall" fn cqi11_concacaf_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    if cqi11_d(cqi11_d(this + 0xa7)) == 0 && fase != 0 {
        let classificar: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x4c11a0usize);
        if classificar(this) < 0 {
            // ponytail: a 0x4c11a0 sempre devolve 1; o caso de erro fica com a caixa e o 0 da original
            aviso_em(0x9a7d30, 0x1ba);
            return 0;
        }
        crate::comparacao::ramo("cqi_11 dia da selecao");
        let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqi11_d(cqi11_d(this) + 0x5c));
        prever(this);
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51f890usize);
    base(this, hoje, fase)
}

// ponytail: a atualização da copa (0x51f890) joga e sorteia: não roda duas vezes. Só a original roda; o plano: com a
// tabela vazia e fase ≠ 0, a copa sai com 8 times e o time 0 preenchido (a 0x4c11a0 tem o seu comparador); senão +0x3e
// não muda.
unsafe extern "thiscall" fn comparar_cqi11_concacaf_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1689);
    if cqi11_d(this + 0xa7) == 0 {
        return o(this, hoje, fase);
    }
    let seleciona = cqi11_d(cqi11_d(this + 0xa7)) == 0 && fase != 0;
    let n = cqi11_w(this + 0x3e);
    let r = o(this, hoje, fase);
    let igual = if seleciona { cqi11_w(this + 0x3e) == 8 } else { cqi11_w(this + 0x3e) == n };
    PLACAR.registrar(igual, || format!("cqi11_concacaf_dia({:#x}, fase {}) seleciona={} times {} -> {}", this, fase, seleciona, n, cqi11_w(this + 0x3e)));
    r
}

/// { 0x4c15f0 } (thiscall, ret 0, con_champ.cpp; a virtual +0x8c): `ConChamp::set_up_rounds()`; devolve 1. O fundo de
/// prêmios (+0x28) de 2.000.000 e a tabela das rodadas (+0xa3) da virtual +0x3c (push 0; push &+0x3a; push &+0x3c;
/// push −1; mov ecx, this -> thiscall, ret 0x10).
unsafe extern "thiscall" fn cqi11_concacaf_rodadas(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x28) as *mut u32, 0x1e8480);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqi11_d(cqi11_d(this) + 0x3c));
    let t = tabela(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xa3) as *mut u32, t as u32);
    1
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere o fundo de
// prêmios (+0x28 = 2.000.000) e a devolução 1.
unsafe extern "thiscall" fn comparar_cqi11_concacaf_rodadas(this: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1690)(this);
    PLACAR.registrar(r == 1 && cqi11_d(this + 0x28) == 0x1e8480, || format!("cqi11_concacaf_rodadas({:#x}) devolveu {} prêmios {}", this, r, cqi11_d(this + 0x28)));
    r
}

// ---------- 273a leva: as regras da fase de grupos das copas: a Copa da Itália, a Copa da Itália da Série C e a Copa dos Campeões do Brasil (ita_cup.cpp, ita_c_cup.cpp, bra_champ_cup.cpp) ----------

/// As regras que a virtual +0x8c (set_up_rules) de uma copa com fase de grupos grava na liga do grupo, antes da tabela
/// das rodadas. Desempates: os quatro códigos de +0xc5..+0xc8 (docs/.md §2.7).
#[derive(Clone, Copy)]
struct Cqi12Regras {
    vitoria: u8,     // +0xc2: pontos por vitória
    empate: u8,      // +0xc3: pontos por empate
    categoria: u8,   // +0xc4: a categoria de cada jogo (vai para o +0x3a da partida)
    desempate: [u8; 4], // +0xc5..+0xc8
    vagas: u8,       // +0xbe: vagas de cima (classificados por grupo)
    premio: Option<u32>, // +0x28: o fundo de prêmios (a Copa dos Campeões não grava)
    bits: Option<u16>,   // +0xd9: a palavra de bits de regra (só a Copa dos Campeões grava: 8)
}

const CQI12_COPA_DA_ITALIA: Cqi12Regras = Cqi12Regras { vitoria: 3, empate: 1, categoria: 4, desempate: [4, 1, 2, 0], vagas: 1, premio: Some(0xc3500), bits: None };
const CQI12_COPA_DA_SERIE_C: Cqi12Regras = Cqi12Regras { vitoria: 3, empate: 1, categoria: 4, desempate: [4, 1, 2, 0], vagas: 1, premio: Some(0x493e0), bits: None };
const CQI12_COPA_DOS_CAMPEOES: Cqi12Regras = Cqi12Regras { vitoria: 3, empate: 1, categoria: 0x17, desempate: [3, 1, 2, 4], vagas: 2, premio: None, bits: Some(8) };

/// Grava as regras `g` na liga `this`: sobe/cai para nenhuma (+0x1c, +0x20 = −1), um turno (+0x3c = 1), o tipo 1
/// (+0x42, clubes), sem play-offs nem rebaixamento (+0xbf, +0xc0, +0xc1 = 0); depois a tabela das rodadas da virtual
/// +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx, this -> thiscall, ret 0x10) em +0xba. Devolve 1.
unsafe fn cqi12_regras(this: usize, g: &Cqi12Regras, turnos: i16) -> u32 {
    let b = |o: usize, v: u8| *((this + o) as *mut u8) = v;
    b(0xc2, g.vitoria);
    b(0xc3, g.empate);
    b(0xc4, g.categoria);
    for (k, &v) in g.desempate.iter().enumerate() {
        b(0xc5 + k, v);
    }
    b(0xbe, g.vagas);
    b(0xbf, 0);
    b(0xc0, 0);
    b(0xc1, 0);
    b(0x42, 1);
    core::ptr::write_unaligned((this + 0x1c) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x20) as *mut i32, -1);
    core::ptr::write_unaligned((this + 0x3c) as *mut i16, turnos);
    if let Some(p) = g.premio {
        core::ptr::write_unaligned((this + 0x28) as *mut u32, p);
    }
    if let Some(w) = g.bits {
        core::ptr::write_unaligned((this + 0xd9) as *mut u16, w);
    }
    let vt = core::ptr::read_unaligned(this as *const usize);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(core::ptr::read_unaligned((vt + 0x3c) as *const usize));
    let t = tabela(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xba) as *mut u32, t as u32);
    1
}

/// O plano: os campos que `cqi12_regras` grava, lidos depois da original.
unsafe fn cqi12_confere(nome: &str, this: usize, g: &Cqi12Regras, r: u32) {
    let b = |o: usize| *((this + o) as *const u8);
    let lido = (b(0xc2), b(0xc3), b(0xc4), [b(0xc5), b(0xc6), b(0xc7), b(0xc8)], b(0xbe), [b(0xbf), b(0xc0), b(0xc1)], b(0x42),
        core::ptr::read_unaligned((this + 0x1c) as *const i32), core::ptr::read_unaligned((this + 0x20) as *const i32),
        core::ptr::read_unaligned((this + 0x3c) as *const i16));
    let previsto = (g.vitoria, g.empate, g.categoria, g.desempate, g.vagas, [0u8; 3], 1u8, -1i32, -1i32, 1i16);
    let premio = g.premio.map_or(true, |p| core::ptr::read_unaligned((this + 0x28) as *const u32) == p);
    let bits = g.bits.map_or(true, |w| core::ptr::read_unaligned((this + 0xd9) as *const u16) == w);
    PLACAR.registrar(r == 1 && lido == previsto && premio && bits, || format!("{}({:#x}) devolveu {} regras {:?} previstas {:?} prêmio ok {} bits ok {}", nome, this, r, lido, previsto, premio, bits));
}

/// { 0x63f350 } (thiscall, ret 0, ita_cup.cpp; a virtual +0x8c): `ItaCup::set_up_rules()`: os grupos da Copa da Itália
/// em turno único, 3 pontos a vitória e 1 o empate, a categoria 4 (copa nacional), o desempate (4, 1, 2), um
/// classificado por grupo e o fundo de prêmios de 800.000; depois a tabela das rodadas.
unsafe extern "thiscall" fn cqi12_copa_italia_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_12 regras da copa da italia");
    let mut g = CQI12_COPA_DA_ITALIA;
    g.vitoria = 3; // cqi12: a vitória nos grupos da Copa da Itália
    g.categoria = 4; // cqi12: a categoria dos jogos da Copa da Itália
    cqi12_regras(this, &g, 1)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqi12_copa_italia_regras(this: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1691)(this);
    cqi12_confere("cqi12_copa_italia_regras", this, &CQI12_COPA_DA_ITALIA, r);
    r
}

/// { 0x63d6e0 } (thiscall, ret 0, ita_c_cup.cpp; a virtual +0x8c): `ItaCCup::set_up_rules()`: os grupos da Copa da
/// Itália da Série C, com as mesmas regras da Copa da Itália e o fundo de prêmios de 300.000.
unsafe extern "thiscall" fn cqi12_copa_c_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_12 regras da copa da serie c");
    let turnos = 1i16; // cqi12: o turno único dos grupos da Série C
    cqi12_regras(this, &CQI12_COPA_DA_SERIE_C, turnos)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqi12_copa_c_regras(this: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1692)(this);
    cqi12_confere("cqi12_copa_c_regras", this, &CQI12_COPA_DA_SERIE_C, r);
    r
}

/// { 0x426230 } (thiscall, ret 0, bra_champ_cup.cpp; a virtual +0x8c): `BraChampCup::set_up_rules()`: o grupo da
/// Copa dos Campeões do Brasil em turno único, 3 pontos a vitória e 1 o empate, a categoria 0x17 (23, a das finais de
/// campeões), o desempate (3, 1, 2, 4), dois classificados e os bits de regra 8 (quem garante as vagas vai à fase
/// seguinte); sem fundo de prêmios. Depois a tabela das rodadas.
unsafe extern "thiscall" fn cqi12_copa_dos_campeoes_regras(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_12 regras da copa dos campeoes");
    cqi12_regras(this, &CQI12_COPA_DOS_CAMPEOES, 1)
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere as regras.
unsafe extern "thiscall" fn comparar_cqi12_copa_dos_campeoes_regras(this: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1693)(this);
    cqi12_confere("cqi12_copa_dos_campeoes_regras", this, &CQI12_COPA_DOS_CAMPEOES, r);
    r
}

// ---------- 273a leva: o destino das seleções nos torneios: a Copa Africana de Nações, a Copa América e a eliminatória da Oceania (african_nations.cpp, goldcup.cpp, wc_oceania_league.cpp) ----------

unsafe fn cqj10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqj10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Os registros de final (0x86a800 campeão, 0x86b0b0 vice, 0x86b710 terceiro: push a6; push jogo; push clube; mov ecx,
/// [0xdd7f00] -> thiscall, ret 0xc).
unsafe fn cqj10_final(f: usize, clube: usize, jogo: usize, a6: u32) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(cpz4_registros(), clube, jogo, a6);
}

/// A virtual +0x44 de um torneio de seleções com grupos e mata-mata (a Copa Africana e a Copa América): `fase` (byte com
/// sinal) abaixo de `grupos` é a fase de grupos: 6, a classificação (0x86bdd0: fase 0, rodada 0x78, peso 0x19); 0xfe,
/// a não classificação (0x86c1d0: fase 0x41c, peso 0x19), que encerra as convocações. Na fase `grupos` (o mata-mata,
/// a fase [+0xc][grupos]): 0, a classificação com a fase +0x32 do jogo, o +0x6f da rodada do jogo (+0x34) na tabela de
/// rodadas da fase (+0xa3, 0x68 bytes) e o peso 0x4bf850(0x28, 0x64, rodada, 3); 0xff, a eliminação (0x86c000) com
/// +0x32, +0x30 e o mesmo peso; 1, 2 e 3, os registros de campeão, vice e terceiro. Devolve 0.
unsafe fn cqj10_selecao(nome: u8, this: usize, clube: usize, desfecho: u32, fase: u32, jogo: usize, a6: u32, grupos: i8) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = cqj10_d(this + 4);
    if f < grupos {
        if d == 6 {
            cpz4_classificado(clube, comp, 0, 0x78, 0x19);
        } else if d == 0xfe {
            if nome == 0 {
                crate::comparacao::ramo("cqj_10 africana fora nos grupos");
                cpz4_nao_classificado(clube, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa Africana
            } else {
                crate::comparacao::ramo("cqj_10 copa america fora nos grupos");
                cpz4_nao_classificado(clube, comp, 0x41c, 0x19); // cqj10: quem cai nos grupos da Copa América
            }
        }
    } else if f == grupos {
        match d {
            0 => {
                let r = cqj10_w(jogo + 0x34);
                let peso = cpz4_escala(0x28, 0x64, r, 3);
                let mata = cqj10_d(cqj10_d(this + 0xc) + grupos as usize * 4);
                let rodada = cqj10_w(cqj10_d(mata + 0xa3).wrapping_add((r as isize * 0x68) as usize) + 0x6f) as u16 as u32;
                cpz4_classificado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, rodada, peso);
            }
            0xff => {
                let peso = cpz4_escala(0x28, 0x64, cqj10_w(jogo + 0x34), 3);
                // ponytail: +0x32 e +0x30 vão em registradores com lixo nos 16 bits altos; a 0x86c000 só usa as palavras
                if nome == 0 {
                    crate::comparacao::ramo("cqj_10 africana fora no mata-mata");
                    cpz4_eliminado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, cqj10_w(jogo + 0x30) as u16 as u32, peso); // cqj10: quem cai no mata-mata da Copa Africana
                } else {
                    cpz4_eliminado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, cqj10_w(jogo + 0x30) as u16 as u32, peso);
                }
            }
            1 => cqj10_final(0x86a800, clube, jogo, a6),
            2 => cqj10_final(0x86b0b0, clube, jogo, a6),
            3 => cqj10_final(0x86b710, clube, jogo, a6),
            _ => {}
        }
    }
    0
}

/// { 0x401ba0 } (thiscall, ret 0x18, african_nations.cpp; a virtual +0x44): `AfricanNations::
/// update_staff_records_for_status(seleção, desfecho, fase, _, jogo, a6)`: cqj10_selecao com 3 fases de grupos (o
/// mata-mata na fase 3); o desfecho 4 não faz nada.
unsafe extern "thiscall" fn cqj10_copa_africana_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    cqj10_selecao(0, this, clube, desfecho, fase, jogo, a6, 3)
}

// ponytail: os registros vão para o histórico dos técnicos e encerram as convocações: não rodam duas vezes, e a rotina
// não grava nada além deles. Só a original; o plano confere a devolução 0.
unsafe extern "thiscall" fn comparar_cqj10_copa_africana_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1694)(this, clube, desfecho, fase, a4, jogo, a6);
    PLACAR.registrar(r == 0, || format!("cqj10_copa_africana_registros({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}

/// { 0x5e13b0 } (thiscall, ret 0x18, goldcup.cpp; a virtual +0x44 da Copa América): `GoldCup::
/// update_staff_records_for_status(seleção, desfecho, fase, _, jogo, a6)`: cqj10_selecao com 2 fases de grupos (o
/// mata-mata na fase 2).
unsafe extern "thiscall" fn cqj10_copa_america_registros(this: usize, clube: usize, desfecho: u32, fase: u32, _a4: u32, jogo: usize, a6: u32) -> u32 {
    cqj10_selecao(1, this, clube, desfecho, fase, jogo, a6, 2)
}

// ponytail: como a da Copa Africana: só a original, a devolução 0.
unsafe extern "thiscall" fn comparar_cqj10_copa_america_registros(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1695)(this, clube, desfecho, fase, a4, jogo, a6);
    PLACAR.registrar(r == 0, || format!("cqj10_copa_america_registros({:#x}, {}, {}) devolveu {}", clube, desfecho as u8, fase as u8, r));
    r
}

/// A competição atual da seleção acaba (0x775000 em [0xb74318]: push nação; mov ecx -> thiscall, ret 4).
unsafe fn cqj10_fim_da_competicao(clube: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x775000usize);
    f(cqj10_d(0xb74318), cqj10_d(clube + 0x53));
}

/// { 0x91e840 } (thiscall, ret 0x18, wc_oceania_league.cpp; a virtual +0x44): `WcOceaniaLeague::
/// update_staff_records_for_status(seleção, desfecho, fase, saida, jogo, _)`; devolve 0. Fase (byte com sinal) 0 ou
/// menos, os grupos: 6, a classificação (fase 0, rodada 0x82, peso 0x14); 0xfe, a não classificação (0x41c, 0x14) e a
/// competição atual da seleção acaba. Fase 1 (a final da Oceania): 1, a classificação (0xa0, 0, 0x1e); 2 ou 0xff, a
/// eliminação (+0x32 e +0x30 do jogo, peso 0x1e) e a competição acaba. Fase 2 (a repescagem): 1, a classificação para
/// a Copa do Mundo (a competição [0xae23d0] + 0x6b × [0x9cf79c], 0, 0, 0x64), a vaga nela (0x92e740 no objeto
/// [[0xadadfc] + 4 × [0x9cf79c]]: push seleção -> thiscall, ret 4) e *saida = 4; 2, a eliminação (peso 0x64) e a
/// competição acaba.
unsafe extern "thiscall" fn cqj10_eliminatoria_oceania_registros(this: usize, clube: usize, desfecho: u32, fase: u32, saida: usize, jogo: usize, _a6: u32) -> u32 {
    let (d, f) = (desfecho as u8, fase as u8 as i8);
    let comp = cqj10_d(this + 4);
    let eliminar = |peso: u32| {
        // ponytail: +0x30 e +0x32 vão em registradores com lixo nos 16 bits altos; a 0x86c000 só usa as palavras
        cpz4_eliminado(clube, comp, cqj10_w(jogo + 0x32) as u16 as u32, cqj10_w(jogo + 0x30) as u16 as u32, peso);
        cqj10_fim_da_competicao(clube);
    };
    if f < 1 {
        if d == 6 {
            cpz4_classificado(clube, comp, 0, 0x82, 0x14);
        } else if d == 0xfe {
            cpz4_nao_classificado(clube, comp, 0x41c, 0x14);
            cqj10_fim_da_competicao(clube);
        }
    } else if f == 1 {
        if d == 1 {
            cpz4_classificado(clube, comp, 0xa0, 0, 0x1e);
        } else if d == 2 || d == 0xff {
            eliminar(0x1e);
        }
    } else if f == 2 {
        if d == 1 {
            crate::comparacao::ramo("cqj_10 oceania na copa do mundo");
            let id = cqj10_d(0x9cf79c);
            cpz4_classificado(clube, cqj10_d(0xae23d0).wrapping_add(id.wrapping_mul(0x6b)), 0, 0, 0x64);
            let vaga: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x92e740usize);
            vaga(cqj10_d(cqj10_d(0xadadfc).wrapping_add(id.wrapping_mul(4))), clube);
            *(saida as *mut u8) = 4;
        } else if d == 2 {
            eliminar(0x64);
        }
    }
    0
}

// ponytail: os registros, o fim da competição da seleção e a vaga na Copa do Mundo não rodam duas vezes. Só a original;
// o plano: a devolução 0 e, na repescagem ganha, *saida = 4 e a lista da Copa do Mundo (+0x36) um a mais (se cabia).
unsafe extern "thiscall" fn comparar_cqj10_eliminatoria_oceania_registros(this: usize, clube: usize, desfecho: u32, fase: u32, saida: usize, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, usize, u32) -> u32>(1696);
    let vaga = fase as u8 == 2 && desfecho as u8 == 1 && saida != 0;
    let mundial = cqj10_d(cqj10_d(0xadadfc).wrapping_add(cqj10_d(0x9cf79c).wrapping_mul(4)));
    if vaga && mundial == 0 {
        return o(this, clube, desfecho, fase, saida, jogo, a6);
    }
    let n = if vaga { cqj10_w(mundial + 0x36) } else { 0 };
    let cabe = vaga && n < cqj10_w(mundial + 0x38);
    let r = o(this, clube, desfecho, fase, saida, jogo, a6);
    let igual = r == 0 && (!vaga || (*(saida as *const u8) == 4 && cqj10_w(mundial + 0x36) == if cabe { n + 1 } else { n }));
    PLACAR.registrar(igual, || format!("cqj10_eliminatoria_oceania_registros({:#x}, {}, {}) devolveu {} vaga={} lista {}", clube, desfecho as u8, fase as u8, r, vaga, n));
    r
}
