// ---------- %%LEVA%%a leva: a inscrição das três copas europeias, a tabela inicial do ranking da FIFA e a lista dos estádios neutros (fifa_rankings.cpp, squad_manager.cpp, stadium.cpp) ----------

unsafe fn cqn3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As três copas europeias da inscrição, na ordem: a Liga dos Campeões, a Copa da UEFA e a Recopa [inferência] (as
/// chaves dos ids das competições).
const EUROPEAN_REGISTRATION_CUPS: [usize; 3] = [0x9cf6e8, 0x9cf6f0, 0x9cf6f8];

/// Os clubes da competição `comp` pelo método +0x58 dela (thiscall(comp, &lista, etapa), devolve a conta em ax); a
/// lista alocada é solta pela 0x9452ca depois de `cada` passar por todos.
unsafe fn for_each_comp_club(comp: usize, etapa: u32, mut cada: impl FnMut(usize)) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let metodo: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16 = core::mem::transmute(cqn3_d(cqn3_d(comp) as usize + 0x58) as usize);
    let mut lista = 0usize;
    let n = metodo(comp, &mut lista, etapa) as i16 as i32;
    for i in 0..n.max(0) as usize {
        cada(cqn3_d(lista + 4 * i) as usize);
    }
    if lista != 0 {
        soltar(lista);
    }
}

/// { 0x85a9b0 } (thiscall, ret 4, squad_manager.cpp, `SQUAD_MANAGER::process_euro_registration`): a etapa `etapa`
/// (byte) da inscrição nas copas europeias. Na etapa 1, o ano das inscrições (+0, word) fica o ano do jogo e a marca de
/// inscrição (+0x6d) de todas as pessoas com contrato (registros de 0x6e bytes em [0xae23bc]) é zerada. Depois, em cada
/// uma das três copas (`EUROPEAN_REGISTRATION_CUPS`; a competição pela tabela [0xadadfc]), cada clube da lista (o método +0x58 com a
/// etapa) que está nela (+0x1db = o id): com técnico humano (0x7875e0, ecx = [0xb7431c]) recebe a notícia da inscrição
/// (0x85b020(this, clube, id, etapa), original); senão a inscrição é feita (0x85aae0(clube, etapa, 0), com ecx =
/// [0xdd7ef0]).
unsafe extern "thiscall" fn process_euro_registration(this: usize, etapa: u32) {
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x85b020usize);
    let inscrever: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u8 = core::mem::transmute(0x85aae0usize);
    if etapa as u8 == 1 {
        *(this as *mut u16) = *(0xae2c92 as *const u16);
        let mut i = 0i32;
        while i < cqn3_d(0xae2364) as i32 {
            let m = (cqn3_d(0xae23bc) as usize + 0x6e * i as usize + 0x6d) as *mut u8;
            if *m != 0 {
                *m = 0;
            }
            i += 1;
        }
    }
    for &chave in EUROPEAN_REGISTRATION_CUPS.iter() {
        let id = cqn3_d(chave);
        let comp = cqn3_d(cqn3_d(0xadadfc) as usize + 4 * id as usize) as usize;
        if comp == 0 {
            continue;
        }
        for_each_comp_club(comp, etapa, |c| {
            if c == 0 || cqn3_d(c + 0x1db) != id {
                return;
            }
            if humano(cqn3_d(0xb7431c) as usize, c) != 0 {
                noticia(this, c, id, etapa);
            } else {
                crate::comparison::branch("cqn_3 inscreve o clube");
                inscrever(cqn3_d(0xdd7ef0) as usize, c, etapa, 0); // cqn3: inscreve o clube
            }
        });
    }
}

/// Lado a lado: o ano das inscrições (+0), a marca +0x6d de todas as pessoas com contrato (a tabela inteira) e o
/// [0xae1080] da 0x85aae0. Com algum clube das três copas com técnico humano (a notícia não roda duas vezes), só a
/// original.
unsafe extern "thiscall" fn compare_process_euro_registration(this: usize, etapa: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let mut tem_humano = false;
    for &chave in EUROPEAN_REGISTRATION_CUPS.iter() {
        let id = cqn3_d(chave);
        let comp = cqn3_d(cqn3_d(0xadadfc) as usize + 4 * id as usize) as usize;
        if comp != 0 {
            for_each_comp_club(comp, etapa, |c| {
                if c != 0 && cqn3_d(c + 0x1db) == id && humano(cqn3_d(0xb7431c) as usize, c) != 0 {
                    tem_humano = true;
                }
            });
        }
    }
    if tem_humano {
        return o(this, etapa);
    }
    let n = (cqn3_d(0xae2364) as i32).max(0) as usize;
    let reg = [(this, 2usize), (cqn3_d(0xae23bc) as usize, n * 0x6e), (0xae1080, 4)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, etapa), || process_euro_registration(this, etapa));
    SCOREBOARD.record(igual, || {
        let base = cqn3_d(0xae23bc) as usize;
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).flat_map(|(a, b)| {
            let a0 = a.0;
            a.1.iter().zip(b.1.iter()).enumerate().filter(|(_, (x, y))| x != y).map(move |(i, (x, y))| (a0 + i, *x, *y))
        }).take(12).map(|(e, x, y)| if e >= base && e < base + n * 0x6e { format!("pessoa {} +{:x}: {}/{}", (e - base) / 0x6e, (e - base) % 0x6e, x, y) } else { format!("{:x}: {}/{}", e, x, y) }).collect();
        format!("process_euro_registration(etapa {}): {}", etapa as u8, d.join(", "))
    });
}

/// { 0x58dd50 } (thiscall, ret 0, fifa_rankings.cpp, `FIFA_RANKINGS::create_default_table`): a tabela inicial do
/// ranking da FIFA (registros de 0x2c bytes, um por nação, calloc 0x945501 de [0xae2350] × 0x2c em +0; sem memória, a
/// caixa da linha 0x2da). Cada registro: +0 o índice da nação; a força da nação (+0xa8, double, nações de 0x122 bytes em
/// [0xae23a8]) × 1/35 dá os pontos (+0xc, float) e, × 1000 (truncado, _ftol), os 6 dwords +0x14..+0x28 [inferência: os
/// pontos de cada um dos últimos anos]. Depois a ordem pelos pontos (0x58d610(this, &conta), thiscall, a lista ordenada
/// das nações com continente) dá a posição: +0x12 = a posição de antes (+0x10) e +0x10 = 1, 2, 3...; a lista é solta.
unsafe extern "thiscall" fn create_default_table(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let ordem: unsafe extern "thiscall" fn(usize, *mut i32) -> usize = core::mem::transmute(0x58d610usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let t = calloc(cqn3_d(0xae2350), 0x2c);
    *(this as *mut usize) = t;
    if t == 0 {
        show_error_at(0x9c74f4, 0x2da);
        return;
    }
    let c1 = core::ptr::read_unaligned(0x96a348 as *const f64);
    let c2 = core::ptr::read_unaligned(0x968908 as *const f64);
    let mut k = 0i32;
    while k < cqn3_d(0xae2350) as i32 {
        let r = t + 0x2c * k as usize;
        *(r as *mut i32) = k;
        let a = core::ptr::read_unaligned((cqn3_d(0xae23a8) as usize + 0x122 * k as usize + 0xa8) as *const f64) * c1;
        let p = (a * c2) as i32;
        for j in 0..6 {
            *((r + 0x14 + 4 * j) as *mut i32) = p;
        }
        *((r + 0xc) as *mut f32) = a as f32;
        k += 1;
    }
    let mut n = 0i32;
    let l = ordem(this, &mut n);
    for i in 0..n.max(0) as usize {
        let r = cqn3_d(l + 4 * i) as usize;
        *((r + 0x12) as *mut u16) = *((r + 0x10) as *const u16);
        *((r + 0x10) as *mut u16) = (i + 1) as u16;
    }
    soltar(l);
}

/// Lado a lado à mão: a original no objeto de quem chama (ela aloca a tabela em +0), a cópia num objeto de rascunho
/// (aloca a dela); iguais as duas tabelas inteiras (a do rascunho é solta).
unsafe extern "thiscall" fn compare_create_default_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    o(this);
    let mut s = [0usize; 1];
    create_default_table(s.as_mut_ptr() as usize);
    let (ta, tb) = (cqn3_d(this) as usize, s[0]);
    let n = (cqn3_d(0xae2350) as i32).max(0) as usize * 0x2c;
    let igual = if ta != 0 && tb != 0 {
        core::slice::from_raw_parts(ta as *const u8, n) == core::slice::from_raw_parts(tb as *const u8, n)
    } else {
        ta == tb
    };
    let dif = if igual || ta == 0 || tb == 0 {
        None
    } else {
        (0..n).find(|&i| *((ta + i) as *const u8) != *((tb + i) as *const u8))
    };
    if tb != 0 {
        soltar(tb);
    }
    SCOREBOARD.record(igual, || format!("create_default_table: a primeira diferença na nação {:?}", dif.map(|i| (i / 0x2c, i % 0x2c))));
}

/// { 0x85dfc0 } (thiscall, ret 0, stadium.cpp, `Stadium::build_venue_list`): refaz a lista dos estádios para jogos
/// em campo neutro (+0xc a lista de registros de 16 bytes, +0x10 a conta, +0x14 a capacidade; a velha é solta). As
/// sedes das próximas edições de três competições (as chaves 0x9cf79c, 0x9cf7a4, 0x9cf7ac; 0x5fa7d0 no HostCountry
/// [0xb5e134], sem ele a caixa da linha 0x858) são achadas, mas não mudam nada (o laço delas termina igual nos dois
/// caminhos). Entram os estádios (registros de 0x4e bytes em [0xae23b0], conta [0xae2358]) com cidade (+0x38), nação da
/// cidade (+0x1f) e continente da nação (+0x71), fora o "City of Manchester Stadium" (ainda em obras em 2001) e, nas
/// nações com competições (+0x11c), os de menos de 5000 lugares (+0x3c). Cada registro: +0 o id da nação, +4 o id do
/// continente, +8 se é o estádio nacional (+0x80 da nação), +0xc o estádio. A lista cresce de 30 em 30 (0x944e46,
/// realloc 0x945667; sem memória, as caixas das linhas 0x89c e 0x8ac, e para). No fim é ordenada (qsort com a
/// 0x85e2f0) e +0x18 = 1.
unsafe extern "thiscall" fn build_venue_list(this: usize) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let sedes: unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32) -> u8 = core::mem::transmute(0x5fa7d0usize);
    let (lista, conta, cap) = ((this + 0xc) as *mut usize, (this + 0x10) as *mut i32, (this + 0x14) as *mut i32);
    if *lista != 0 {
        soltar(*lista);
        *lista = 0;
        *cap = 0;
        *conta = 0;
    }
    let hc = cqn3_d(0xb5e134) as usize;
    if hc == 0 {
        show_error_at(0xa835b0, 0x858);
    } else {
        // as sedes: só achadas (a original não as usa depois); as saídas se sobrepõem como na original
        let mut s = [0u32; 8];
        let mut b = sedes(hc, cqn3_d(0x9cf79c), s.as_mut_ptr(), s.as_mut_ptr().add(1)) as i8;
        b = b.wrapping_add(sedes(hc, cqn3_d(0x9cf7a4), s.as_mut_ptr().add(b as usize), s.as_mut_ptr().add(b as usize + 1)) as i8);
        sedes(hc, cqn3_d(0x9cf7ac), s.as_mut_ptr().add(b as usize), s.as_mut_ptr().add(b as usize + 1));
    }
    let mut k = 0i32;
    while k < cqn3_d(0xae2358) as i32 {
        let e = cqn3_d(0xae23b0) as usize + 0x4e * k as usize;
        k = (k as i16).wrapping_add(1) as i32;
        let cidade = cqn3_d(e + 0x38) as usize;
        if cidade == 0 {
            continue;
        }
        let nacao = cqn3_d(cidade + 0x1f) as usize;
        if nacao == 0 || cqn3_d(nacao + 0x71) == 0 {
            continue;
        }
        if core::ffi::CStr::from_ptr((e + 4) as *const core::ffi::c_char).to_bytes() == b"City of Manchester Stadium" {
            continue;
        }
        if *((nacao + 0x11c) as *const u8) != 0 && (cqn3_d(e + 0x3c) as i32) < 0x1388 { // cqn3: menos de 5000 lugares
            crate::comparison::branch("cqn_3 estadio pequeno");
            continue; // cqn3: o estádio pequeno fica de fora
        }
        if *lista == 0 {
            *lista = alocar(0x1e0);
            if *lista == 0 {
                show_error_at(0xa835b0, 0x89c);
                return;
            }
            *cap = 0x1e;
        } else if *conta == *cap {
            *lista = realocar(*lista, ((*cap + 0x1e) << 4) as u32);
            if *lista == 0 {
                show_error_at(0xa835b0, 0x8ac);
                return;
            }
            *cap += 0x1e;
        }
        let r = *lista + 16 * *conta as usize;
        *(r as *mut u32) = cqn3_d(nacao);
        *((r + 4) as *mut u32) = cqn3_d(cqn3_d(nacao + 0x71) as usize);
        *((r + 0xc) as *mut usize) = e;
        crate::comparison::branch("cqn_3 entra na lista");
        *((r + 8) as *mut u32) = (cqn3_d(nacao + 0x80) as usize == e) as u32; // cqn3: o estádio nacional
        *conta += 1;
    }
    ordenar(*lista, *conta as u32, 0x10, 0x85e2f0);
    *((this + 0x18) as *mut u8) = 1;
}

/// Lado a lado à mão: a original no objeto de quem chama, a cópia num rascunho com os mesmos 0x20 bytes de antes e a
/// lista velha zerada (a original já a soltou); iguais os campos (fora o ponteiro +0xc) e as listas (a do rascunho é
/// solta). Sem o HostCountry (a caixa), só a original.
unsafe extern "thiscall" fn compare_build_venue_list(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cqn3_d(0xb5e134) == 0 {
        return o(this);
    }
    let mut s = [0u8; 0x20];
    core::ptr::copy_nonoverlapping(this as *const u8, s.as_mut_ptr(), 0x20);
    s[0xc..0x18].fill(0);
    o(this);
    let r = s.as_mut_ptr() as usize;
    build_venue_list(r);
    let a = core::slice::from_raw_parts(this as *const u8, 0x20);
    let campos = a[..0xc] == s[..0xc] && a[0x10..0x20] == s[0x10..0x20];
    let (la, lb) = (cqn3_d(this + 0xc) as usize, cqn3_d(r + 0xc) as usize);
    let n = (cqn3_d(this + 0x10) as i32).max(0) as usize * 16;
    let listas = if la != 0 && lb != 0 {
        core::slice::from_raw_parts(la as *const u8, n) == core::slice::from_raw_parts(lb as *const u8, n)
    } else {
        la == lb
    };
    if lb != 0 {
        soltar(lb);
    }
    SCOREBOARD.record(campos && listas, || format!("build_venue_list: campos {} listas {} (contas {} e {})", campos, listas, cqn3_d(this + 0x10) as i32, cqn3_d(r + 0x10) as i32));
}
