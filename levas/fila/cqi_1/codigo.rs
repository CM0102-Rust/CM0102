// ---------- %%LEVA%%a leva: a classificação da UEFA: os coeficientes das nações, as vagas de cada nação e o n-ésimo clube da nação (uefa_seeding.cpp) ----------

unsafe fn cqi1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cqi1_f(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// A entrada `i` da tabela das nações da classificação (registros de 0x12 bytes a partir do objeto: +0 id da nação,
/// +4 a nação, +8 o coeficiente das 5 temporadas (float), +0xc vagas na Liga dos Campeões, +0xd na Copa da UEFA,
/// +0xe na Intertoto, +0xf os pontos da temporada (short), +0x11 os clubes da nação nas copas (char)).
fn seeding_nation_entry(this: usize, i: i32) -> usize {
    this.wrapping_add((i * 0x12) as usize)
}

/// { 0x903b50 } (thiscall, ret 0, uefa_seeding.cpp, `UefaSeeding::roll_coefficients`): na virada do ano do jogo
/// (0x9035a0), para cada nação da tabela (+0x438, char): fora de 2001, as 5 temporadas da nação (+0xe0..+0x108, double)
/// andam uma casa e a última vira os pontos da temporada / os clubes nas copas (0 sem clubes); em todo ano, o
/// coeficiente (+8, float) = a soma das 5 (na ordem +0x100, +0xf8, +0xf0, +0xe8, +0xe0, em double, arredondada para
/// float) e os pontos e os clubes zeram. Depois ordena a tabela pelo coeficiente (qsort 0x945333 com 0x905cb0, do
/// maior para o menor) e grava a posição de cada nação no +0x1df da seleção dela (0x53b3d0).
unsafe extern "thiscall" fn roll_coefficients(this: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let conta = || *((this + 0x438) as *const i8);
    let ano = *(0xae2c92 as *const i16);
    let mut i = 0i8;
    while i < conta() {
        let e = seeding_nation_entry(this, i as i32);
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
        crate::comparison::branch("cqi_1 coefficient sum");
        let soma = cqi1_f(p + 0x100) + cqi1_f(p + 0xf8) + cqi1_f(p + 0xf0) + cqi1_f(p + 0xe8) + cqi1_f(p + 0xe0); // cqi1: as 5 temporadas
        core::ptr::write_unaligned((e + 8) as *mut f32, soma as f32);
        *((e + 0x11) as *mut u8) = 0;
        core::ptr::write_unaligned((e + 0xf) as *mut u16, 0);
        i += 1;
    }
    ordenar(this, conta() as i32 as u32, 0x12, 0x905cb0);
    let mut i = 0i8;
    while i < conta() {
        let s = selecao(cqi1_d(seeding_nation_entry(this, i as i32) + 4) as usize);
        *((s + 0x1df) as *mut i8) = i;
        i += 1;
    }
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn memory_diff_addresses(mo: &crate::comparison::MemorySnapshot, mr: &crate::comparison::MemorySnapshot) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// A tabela das nações e, de cada nação, as 5 temporadas (+0xe0..+0x110) e o +0x1df da seleção.
unsafe fn seeding_regions(this: usize) -> Vec<(usize, usize)> {
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let n = (*((this + 0x438) as *const i8)).max(0) as i32;
    let mut reg = vec![(this, 0x43b)];
    for i in 0..n {
        let p = cqi1_d(seeding_nation_entry(this, i) + 4) as usize;
        reg.push((p + 0xe0, 0x30));
        reg.push((selecao(p) + 0x1df, 1));
    }
    reg
}

/// Lado a lado: a tabela das nações, as temporadas de cada nação e a posição gravada nas seleções.
unsafe extern "thiscall" fn compare_roll_coefficients(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let reg = seeding_regions(this);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || roll_coefficients(this));
    SCOREBOARD.record(igual, || format!("roll_coefficients: diferenças em {:x?}", memory_diff_addresses(&mo, &mr)));
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
unsafe extern "thiscall" fn set_place_counts(this: usize) {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let g = |a: usize| cqi1_d(a);
    let conta = || *((this + 0x438) as *const i8);
    let ano = *(0xae2c92 as *const i16);
    let (mut fa, mut fb, mut fc, mut cont) = (-1i8, -1i8, -1i8, 0i8);
    let mut i = 0i8;
    while i < conta() {
        let e = seeding_nation_entry(this, i as i32);
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
            crate::comparison::branch("cqi_1 france intertoto");
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
    let italia = |k: i8| cqi1_d(seeding_nation_entry(this, k as i32)) == g(0x9cf360);
    if ano == 0x7d1 {
        if fa == -1 || fb == -1 || fc == -1 {
            show_error_at(0xaa1a1c, 0x285);
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
        let p = seeding_nation_entry(this, k as i32) + 0xd;
        *(p as *mut u8) = (*(p as *const u8)).wrapping_add(1);
    }
}

/// Lado a lado: a tabela das nações, o sorteio (a cópia sem contar) e a marca da caixa.
unsafe extern "thiscall" fn compare_set_place_counts(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let mut reg = vec![(this, 0x43b), (0xb67a34, 4)];
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || crate::rng::without_counting(|| set_place_counts(this)));
    SCOREBOARD.record(igual, || format!("set_place_counts: diferenças em {:x?}", memory_diff_addresses(&mo, &mr)));
}

/// { 0x9050d0 } (thiscall, ret 0x18, uefa_seeding.cpp, `uefa_find_nth_unplaced_club_of_nation`; o this não é lido):
/// na tabela dos clubes europeus (registros de 12 bytes ordenados pela nação, 0x904040: +0 o clube, +4 o id da nação,
/// +8, +9, +0xb), a busca binária (bsearch 0x946464 com 0x905ce0; a chave só com a nação e +8 = 0xff, então só a nação
/// conta) acha um clube da nação; recua até o primeiro da nação; anda até o `n`-ésimo (char, 1 = o primeiro) que conta,
/// pulando, se pedido, os que já têm vaga nas copas europeias (+0x1db ≠ −1) e os de técnico humano (0x600a50(clube, 0,
/// 1, 0, 0)). O último registro da tabela nunca é examinado pela volta: se ela chega nele com a conta zerada, ele é o
/// escolhido sem os testes. Devolve o clube se for da nação, senão 0.
unsafe extern "thiscall" fn uefa_find_nth_unplaced_club_of_nation(_this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
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
                crate::comparison::branch("cqi_1 already with a place");
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
unsafe extern "thiscall" fn compare_uefa_find_nth_unplaced_club_of_nation(this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize>(%%I2%%);
    let r = o(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    let c = uefa_find_nth_unplaced_club_of_nation(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    SCOREBOARD.record(r == c, || format!("uefa_find_nth_unplaced_club_of_nation(nação {:x}, n {}, {} {}): original {:x} rust {:x}", nacao, n as u8, sem_vaga, sem_humano, r, c));
    r
}
