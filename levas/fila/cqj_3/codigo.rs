// ---------- %%LEVA%%a leva: as sedes das competições: a sede de uma edição, a próxima sede e o calendário das candidaturas (host_country.cpp) ----------

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
unsafe fn host_pair(e: usize, k: i8, a: *mut u32, b: *mut u32) -> u8 {
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
unsafe extern "thiscall" fn find_hosts(this: usize, comp: u32, ano: u32, a: *mut u32, b: *mut u32, padrao: u32) -> u32 {
    let n = cqj3_w(this + 4) as i16;
    let mut e = cqj3_d(this) as usize;
    let mut i = 0i16;
    while i < n {
        if cqj3_d(e) == comp {
            let ae = cqj3_w(e + 4);
            if ae == ano as u16 || ae == 0 {
                let k = *((e + 0x1e) as *const i8);
                if k > -1 {
                    return host_pair(e, k, a, b) as u32;
                }
                if k == -3 || k == -1 || padrao != 0 {
                    crate::comparison::branch("cqj_3 sede padrao");
                    return host_pair(e, -1, a, b) as u32; // cqj3: a sede padrão
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
unsafe extern "thiscall" fn compare_find_hosts(this: usize, comp: u32, ano: u32, a: *mut u32, b: *mut u32, padrao: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, *mut u32, *mut u32, u32) -> u32>(%%I0%%);
    let (va, vb) = (*a, *b);
    let r = o(this, comp, ano, a, b, padrao);
    let (oa, ob) = (*a, *b);
    let (mut ca, mut cb) = (va, vb);
    let c = find_hosts(this, comp, ano, &mut ca, &mut cb, padrao);
    SCOREBOARD.record(r as u8 == c as u8 && oa == ca && ob == cb, || format!("find_hosts({:x}, {}): original {} {:x} {:x} rust {} {:x} {:x}", comp, ano as u16, r as u8, oa, ob, c as u8, ca, cb));
    r
}

/// { 0x5fa7d0 } (thiscall, ret 0xc, host_country.cpp, `HostCountry::find_next_hosts`): as sedes da próxima edição da
/// competição `comp`: a primeira entrada dela com a candidatura escolhida (as duas sedes) ou com a padrão fixa (a
/// padrão). Grava as duas e devolve quantas valem; sem entrada, 0 sem gravar.
unsafe extern "thiscall" fn find_next_hosts(this: usize, comp: u32, a: *mut u32, b: *mut u32) -> u32 {
    let n = cqj3_w(this + 4) as i16;
    let mut e = cqj3_d(this) as usize;
    let mut i = 0i16;
    while i < n {
        if cqj3_d(e) == comp {
            let k = *((e + 0x1e) as *const i8);
            if k > -1 {
                return host_pair(e, k, a, b) as u32;
            }
            if k == -3 {
                return host_pair(e, -1, a, b) as u32;
            }
        }
        e += 0x22;
        i += 1;
    }
    // ponytail: a original devolve o al = 0 com os bytes altos do eax (o ponteiro da entrada); quem chama lê o al
    0
}

/// A devolução (o al) e as duas saídas (a rotina só lê).
unsafe extern "thiscall" fn compare_find_next_hosts(this: usize, comp: u32, a: *mut u32, b: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32) -> u32>(%%I1%%);
    let (va, vb) = (*a, *b);
    let r = o(this, comp, a, b);
    let (oa, ob) = (*a, *b);
    let (mut ca, mut cb) = (va, vb);
    let c = find_next_hosts(this, comp, &mut ca, &mut cb);
    SCOREBOARD.record(r as u8 == c as u8 && oa == ca && ob == cb, || format!("find_next_hosts({:x}): original {} {:x} {:x} rust {} {:x} {:x}", comp, r as u8, oa, ob, c as u8, ca, cb));
    r
}

/// A chave da busca (a 0x5fb3f0 compara o ano +4 e depois o dia +0x1f, sem sinal).
fn compare_year_day(e: usize, ano: u16, dia: u16) -> i32 {
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
unsafe fn first_entry_for_date(inicio: usize, n: i16, ano: u16, dia: u16) -> usize {
    let busca: unsafe extern "C" fn(*const u8, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let mut chave = [0u8; 0x22];
    chave[4..6].copy_from_slice(&ano.to_le_bytes());
    chave[0x1f..0x21].copy_from_slice(&dia.to_le_bytes());
    let mut p = busca(chave.as_ptr(), inicio, n as i32, 0x22, 0x5fb3f0);
    if p == 0 {
        return 0;
    }
    while p > inicio && compare_year_day(p - 0x22, ano, dia) == 0 {
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
unsafe extern "thiscall" fn host_country_daily_update(this: usize, hoje: usize) {
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
    let mut p = first_entry_for_date(inicio, n, ano.wrapping_add(8), dia);
    if p != 0 {
        while p <= fim && compare_year_day(p, ano.wrapping_add(8), dia) == 0 {
            let k = (p + 0x1e) as *mut u8;
            if *k == 0xfe {
                *k = 0xff; // cqj3: a votação abre
            }
            if *k == 0xff {
                crate::comparison::branch("cqj_3 votacao aberta");
                por(l, &p, 4);
            }
            p += 0x22;
        }
    }
    let mut p = first_entry_for_date(inicio, n, ano.wrapping_add(5), dia);
    if p != 0 {
        while p <= fim && compare_year_day(p, ano.wrapping_add(5), dia) == 0 {
            por(l, &p, 4);
            let k = (p + 0x1e) as *mut u8;
            if *k == 0xff || *k == 0xfe {
                crate::comparison::branch("cqj_3 sede sorteada");
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
                crate::comparison::branch("cqj_3 edicao seguinte");
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
unsafe extern "thiscall" fn compare_host_country_daily_update(this: usize, hoje: usize) {
    // ponytail: entrega notícias pela 0x5faec0 e mexe na lista: não dá para rodar duas vezes
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, hoje)
}
