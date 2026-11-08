// ---------- %%LEVA%%a leva: os amistosos da pré-temporada da IA: os amistosos dos clubes, o torneio acrescentado e o convite aberto do clube humano (friendly.cpp) ----------

unsafe fn cqb4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqb4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqb4_c(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqb4_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O registro de clube (0x245 bytes) do id `id`.
unsafe fn cqb4_clube(id: i32) -> usize {
    cqb4_d(0xae23b4).wrapping_add((id as usize).wrapping_mul(0x245))
}

/// A data de 8 bytes em `a` está antes de hoje ([0xae2c92] o ano, sem sinal; [0xae2c90] o dia, com sinal).
unsafe fn is_before_today(a: usize) -> bool {
    let (ano, hoje) = (cqb4_w(0xae2c92) as u16, cqb4_w(0xae2c90));
    let y = cqb4_w(a + 2) as u16;
    y < ano || (y == ano && cqb4_w(a) < hoje)
}

/// A data de 8 bytes em `a` está depois de hoje.
unsafe fn is_after_today(a: usize) -> bool {
    let (ano, hoje) = (cqb4_w(0xae2c92) as u16, cqb4_w(0xae2c90));
    let y = cqb4_w(a + 2) as u16;
    y > ano || (y == ano && cqb4_w(a) > hoje)
}

/// { 0x5c4770 } (thiscall, ret 0xc, friendly.cpp): `Friendly::plan_club_friendlies(entradas, ordenados, conta)`, pelo
/// plano da pré-temporada da IA (0x5c27a0). Libera o plano antigo (+0x34d8; free 0x9452ca, cdecl), aloca o novo (malloc
/// 0x944e46 de 2000 bytes, cdecl; sem memória a caixa da linha 0x6f1), capacidade +0x34e4 = 100, conta +0x34e0 e
/// +0x34dc zerados. Para cada uma das 35 ligas ativas (0xb63d60, 0x48 bytes) de nação com +0x11c, cuja temporada (+0x2a)
/// não começou e cuja data +0x39 menos 0x34 (0x54cab0: lea ecx, [liga + 0x39]; 2 pushes; sem add esp -> thiscall) já
/// chegou: os candidatos da nação (bsearch 0x946464: 5 pushes; add esp, 0x14 -> cdecl, pela 0x5c5290; recua até o
/// primeiro da nação; o último da lista nunca entra) até o primeiro de outra nação. Cada clube que não é humano
/// (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, 1, 1, 0, 0)), não é o de +0x1db = [0x9cf6f8] e tem liga (+0x57)
/// registrada em [0xadadfc]: faixa = reputação / 500 + (sorteio(3) + 1, 3 vira 2 com sorteio(4) zero); até sorteio(3) +
/// 4 amistosos (pela conta +4 do candidato): o adversário (0x5c4cb0: 5 pushes; ecx = this -> thiscall) e, achado, o
/// amistoso em data livre (0x5c4380: 3 pushes; ecx = this -> thiscall) com o de faixa (+5) maior como anfitrião e
/// mandante; sem adversário ou com o amistoso marcado, a faixa × 0,8 (no mínimo 1). No fim, o qsort do plano pela data
/// (0x945333: 4 pushes; add esp, 0x10 -> cdecl, pela 0x778e90).
unsafe extern "thiscall" fn plan_club_friendlies(this: usize, entradas: usize, ordenados: usize, conta: u32) {
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    if cqb4_d(this + 0x34d8) != 0 {
        free(cqb4_d(this + 0x34d8));
    }
    *((this + 0x34e4) as *mut i32) = 100;
    *((this + 0x34e0) as *mut i32) = 0;
    *((this + 0x34dc) as *mut i32) = 0;
    let p = malloc(0x7d0);
    *((this + 0x34d8) as *mut usize) = p;
    if p == 0 {
        show_error_at(0x9cd0a0, 0x6f1);
        return;
    }
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54cab0usize);
    let busca: unsafe extern "C" fn(*const usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let adversario: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x5c4cb0usize);
    let planejar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x5c4380usize);
    for k in 0..0x23usize {
        let liga = 0xb63d60 + k * 0x48;
        let nacao = cqb4_d(liga);
        if nacao == 0 || *((nacao + 0x11c) as *const u8) == 0 {
            continue;
        }
        if is_before_today(liga + 0x2a) {
            continue;
        }
        let mut d = [0u8; 8];
        let r = menos(liga + 0x39, &mut d, 0x34);
        if is_after_today(r) {
            continue;
        }
        // a chave da busca: um candidato de 0x16 bytes com a nação (+6) e a faixa 0xff (+5)
        let mut chave = [0u8; 0x16];
        chave[5] = 0xff;
        chave[6..10].copy_from_slice(&cqb4_i(nacao).to_le_bytes());
        let pchave = chave.as_ptr() as usize;
        let mut q = busca(&pchave, ordenados, conta, 4, 0x5c5290);
        if q == 0 {
            continue;
        }
        let id = cqb4_i(cqb4_d(liga));
        while q > ordenados {
            if cqb4_i(cqb4_d(q - 4) + 6) != id {
                break;
            }
            q -= 4;
        }
        let ultimo = ordenados.wrapping_add((conta as usize).wrapping_mul(4)).wrapping_sub(4);
        while q < ultimo {
            let cand = cqb4_d(q);
            let clube = cqb4_clube(cqb4_i(cand));
            if cqb4_d(clube + 0x53) != cqb4_d(liga) {
                break;
            }
            q += 4;
            if humano(clube, 1, 1, 0, 0) != 0 {
                continue;
            }
            if cqb4_i(clube + 0x1db) == cqb4_i(0x9cf6f8) {
                continue;
            }
            let lg = cqb4_d(clube + 0x57);
            if lg == 0 || cqb4_d(cqb4_d(0xadadfc) + (cqb4_i(lg) as usize).wrapping_mul(4)) == 0 {
                continue;
            }
            crate::comparison::branch("cqb_4 clube planejado");
            let mut b = (cqb4_sortear(3) as u8).wrapping_add(1); // cqb4: o acréscimo da faixa
            if b == 3 && cqb4_sortear(4) == 0 {
                b = 2;
            }
            let mut band = ((cqb4_w(clube + 0x80) as i32 / 500) as u8).wrapping_add(b);
            let quantos = (cqb4_sortear(3) as u8).wrapping_add(4) as i8; // cqb4: quantos amistosos
            let mut j = 0i8;
            while cqb4_c(cand + 4) < quantos && j < quantos {
                // ponytail: a original empurra o dword do local da faixa (os bytes altos são sobra); a 0x5c4cb0 só lê o byte
                let opp = adversario(this, cand, entradas, conta, band as u32, 0);
                let marcado = if opp != 0 {
                    crate::comparison::branch("cqb_4 adversario achado");
                    if cqb4_c(cand + 5) > cqb4_c(opp + 5) { // cqb4: o de faixa maior em casa
                        planejar(this, cand, opp, cand)
                    } else {
                        planejar(this, cand, cand, opp)
                    }
                } else {
                    1
                };
                if marcado != 0 {
                    let f = match_pl_ftol(band as i8 as i32 as f64 * read_f64_at(0x9689b0)) as u8;
                    band = if (f as i8) < 1 { 1 } else { f };
                }
                j = j.wrapping_add(1);
            }
        }
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cqb4_d(this + 0x34d8), cqb4_i(this + 0x34e0), 0x14, 0x778e90);
}

/// Comparador da 0x5c4770: só a original (ponytail: ela libera e aloca o plano, e a 0x5c4380 aloca as listas de datas e
/// de adversários dos candidatos; não roda duas vezes), conferindo o plano que ela deixa: o plano alocado, a capacidade
/// (+0x34e4) não menor que a conta (+0x34e0), o +0x34dc zerado e o plano em ordem de data (a 0x778e90 entre vizinhos).
unsafe extern "thiscall" fn comparar_plan_club_friendlies(this: usize, entradas: usize, ordenados: usize, conta: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%)(this, entradas, ordenados, conta);
    let (p, n, cap, z) = (cqb4_d(this + 0x34d8), cqb4_i(this + 0x34e0), cqb4_i(this + 0x34e4), cqb4_i(this + 0x34dc));
    if p == 0 {
        return; // sem memória: a caixa
    }
    let ordem: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(0x778e90usize);
    let ordenado = (1..n.max(0) as usize).all(|i| ordem(p + (i - 1) * 0x14, p + i * 0x14) <= 0);
    SCOREBOARD.record(n >= 0 && cap >= n && z == 0 && ordenado, || format!("plan_club_friendlies: plano {:#x} conta {} capacidade {} +0x34dc {} em ordem {}", p, n, cap, z, ordenado));
}

/// O torneio livre: o primeiro dos 127 (0x69 bytes a partir de +0xb1) sem anfitrião (+0), ou 0.
unsafe fn free_tournament_slot(this: usize) -> usize {
    (0..0x7fusize).map(|i| this + 0x69 * i + 0xb1).find(|&t| cqb4_d(t) == 0).unwrap_or(0)
}

/// { 0x5c6e00 } (thiscall, ret 0x28, friendly.cpp): `Friendly::add_tournament(anfitrião, tipo, início, a, b, c, d, e,
/// convidados, nome)`, pelo torneio da IA (0x5c3be0) e pelas telas do clube (0x4701f0). O primeiro torneio livre (sem
/// livre: 0) ganha o anfitrião (+0), o tipo (+0xc), os bytes a e b (+0xd, +0xe), a data de início (+4, pela 0x417c10:
/// lea ecx, [t + 4]; 2 pushes; sem add esp -> thiscall), as palavras c, d, e (+0xf, +0x11, +0x13), +0x29, +0x2d e
/// +0x65 zerados, o nome (+0x31, com o zero) e os 4 convidados (+0x15, dwords) com as marcas +0x25..+0x28 em 1.
unsafe extern "thiscall" fn add_tournament(this: usize, anfitriao: usize, tipo: u32, inicio: usize, a: u32, b: u32, c: u32, d: u32, e: u32, convidados: usize, nome: usize) -> usize {
    let t = free_tournament_slot(this);
    if t == 0 {
        return 0;
    }
    *((t + 0xc) as *mut u8) = tipo as u8;
    *((t + 0xd) as *mut u8) = a as u8;
    core::ptr::write_unaligned(t as *mut usize, anfitriao);
    *((t + 0xe) as *mut u8) = b as u8;
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut tmp = [0u8; 8];
    copiar(t + 4, &mut tmp, inicio);
    core::ptr::write_unaligned((t + 0x11) as *mut u16, d as u16);
    core::ptr::write_unaligned((t + 0x13) as *mut u16, e as u16);
    core::ptr::write_unaligned((t + 0x65) as *mut u32, 0);
    core::ptr::write_unaligned((t + 0x29) as *mut u32, 0);
    core::ptr::write_unaligned((t + 0x2d) as *mut u32, 0);
    core::ptr::write_unaligned((t + 0xf) as *mut u16, c as u16);
    let mut k = 0usize;
    loop {
        let x = *((nome + k) as *const u8);
        *((t + 0x31 + k) as *mut u8) = x;
        if x == 0 {
            break;
        }
        k += 1;
    }
    crate::comparison::branch("cqb_4 torneio acrescentado");
    for k in 0..4usize {
        core::ptr::write_unaligned((t + 0x15 + 4 * k) as *mut u32, cqb4_d(convidados + 4 * k) as u32);
        *((t + 0x25 + k) as *mut u8) = 1;
    }
    t
}

/// Comparador da 0x5c6e00: lado a lado no torneio livre (0x69 bytes, mais o que um nome comprido passar) e a devolução.
unsafe extern "thiscall" fn comparar_add_tournament(this: usize, anfitriao: usize, tipo: u32, inicio: usize, a: u32, b: u32, c: u32, d: u32, e: u32, convidados: usize, nome: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32, u32, usize, usize) -> usize>(%%I1%%);
    let t = free_tournament_slot(this);
    let mut tam = 0usize;
    while nome != 0 && *((nome + tam) as *const u8) != 0 {
        tam += 1;
    }
    let reg = if t != 0 { vec![(t, 0x69usize.max(0x31 + tam + 1))] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, anfitriao, tipo, inicio, a, b, c, d, e, convidados, nome), || add_tournament(this, anfitriao, tipo, inicio, a, b, c, d, e, convidados, nome));
    SCOREBOARD.record(igual && r == n, || format!("add_tournament(anfitriao {:x}) original={:#x} {:x?} rust={:#x} {:x?}", anfitriao, r, mo, n, mr));
    r
}

/// { 0x5c68b0 } (thiscall, ret 0, friendly.cpp): `Friendly::find_human_club_open_invitation()`, pelas telas dos
/// amistosos. Com a lista de amistosos (+0x34c8) e o clube (+0x39) do técnico humano da vez ([0xb74c26 + 0x300 ×
/// [0xb77c7a]]): o cursor da lista (+0xa) volta ao começo (+2) e o primeiro amistoso desse clube (+0xc) ainda sem
/// data (+8 ≠ 0) pelo próximo da lista (0x68d1b0: ecx = a lista; sem push -> thiscall); senão 0.
unsafe extern "thiscall" fn find_human_club_open_invitation(this: usize) -> usize {
    let lista = cqb4_d(this + 0x34c8);
    if lista == 0 {
        return 0;
    }
    let tecnico = cqb4_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if tecnico == 0 {
        return 0;
    }
    let clube = cqb4_d(tecnico + 0x39);
    if clube == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqb4_d(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    loop {
        let m = proximo(cqb4_d(this + 0x34c8));
        if m == 0 {
            return 0;
        }
        if cqb4_d(m + 0xc) == clube && cqb4_d(m + 8) != 0 {
            return m;
        }
    }
}

/// Comparador da 0x5c68b0: lado a lado no cursor da lista (+0xa) e a devolução.
unsafe extern "thiscall" fn comparar_find_human_club_open_invitation(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I2%%);
    let lista = cqb4_d(this + 0x34c8);
    let reg = if lista != 0 { vec![(lista + 0xa, 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || find_human_club_open_invitation(this));
    SCOREBOARD.record(igual && r == n, || format!("find_human_club_open_invitation original={:#x} {:x?} rust={:#x} {:x?}", r, mo, n, mr));
    r
}
