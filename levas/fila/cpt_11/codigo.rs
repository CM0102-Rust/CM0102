// ---------- %%LEVA%%a leva: a pré-temporada dos amistosos: o dia dos amistosos, o plano da pré-temporada da IA e as janelas da pré-temporada (friendly.cpp) ----------

unsafe fn cpt11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpt11_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpt11_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32);
}

/// A data `a` é depois da `b` (+2 o ano, sem sinal; +0 o dia, com sinal), como as comparações da original.
unsafe fn friendly_date_after(a: usize, b: usize) -> bool {
    let (aa, ab) = (cpt11_w(a + 2), cpt11_w(b + 2));
    aa > ab || (aa == ab && cpt11_w(a) as i16 > cpt11_w(b) as i16)
}

/// { 0x5c2460 } (thiscall, ret 8, friendly.cpp): `Friendly::update_for_date(hoje, ativo)`: com `ativo`, nos dois dias
/// da pré-temporada (+0x34ea, +0x34e8) e com o início do jogo [0xdd6ec8] até hoje + 8 semanas (0x54c770; depois disso,
/// nada mais no dia), guarda hoje em +0xa7, planeja a pré-temporada da IA (0x5c27a0) e cancela (0x5c8260) os torneios
/// em uso (+0x65) com data anterior a um ano atrás (0x54cab0, 0x456770). Todo dia: cria os amistosos planejados
/// (0x5c2ae0), atualiza os amistosos das seleções (0x5cbb00), tira e responde as respostas dos torneios (+0x34d0,
/// 0x5c5820) e responde as dos amistosos (+0x34d4, 0x5c5f40), tira os pedidos sem clubes ou vencidos (+0x34c8), cancela
/// os torneios fora de uso cuja data − 2 dias (0x54c8e0) já passou e zera os convites vencidos (+0x34cc, 10 de 0x19
/// bytes por item, a data em +9, a marca em +0x15). Devolve 1.
unsafe extern "thiscall" fn friendly_update_for_date(this: usize, hoje: usize, ativo: u32) -> u32 {
    if ativo == 0 {
        return 1;
    }
    let semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c770usize);
    let menos_semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54cab0usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let anterior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let cancelar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5c8260usize);
    let planejar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5c27a0usize);
    let criar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5c2ae0usize);
    let selecoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5cbb00usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let resposta_do_torneio: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x5c5820usize);
    let resposta_do_amistoso: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5c5f40usize);
    let dia = cpt11_w(hoje) as i16 as i32;
    if dia == cpt11_w(this + 0x34ea) as i32 || dia == cpt11_w(this + 0x34e8) as i32 {
        let mut t = [0u8; 8];
        let limit = semanas(hoje, &mut t, 8);
        if friendly_date_after(0xdd6ec8, limit) {
            return 1;
        }
        copiar(this + 0xa7, &mut t, hoje);
        crate::comparison::branch("cpt_11 planned the pre-season");
        planejar(this); // cpt11: planejar
        let mut ha_um_ano = [0u8; 8];
        menos_semanas(0xae2c90, &mut ha_um_ano, 0x34);
        let lo = u32::from_le_bytes([ha_um_ano[0], ha_um_ano[1], ha_um_ano[2], ha_um_ano[3]]);
        let hi = u32::from_le_bytes([ha_um_ano[4], ha_um_ano[5], ha_um_ano[6], ha_um_ano[7]]);
        for k in 0..0x7f {
            let e = this + 0xb1 + k * 0x69;
            if cpt11_d(e) != 0 && anterior(e + 4, lo, hi) != 0 && cpt11_d(e + 0x65) != 0 {
                cancelar(this, e, 0);
            }
        }
    }
    criar(this);
    selecoes(this);
    let l = cpt11_d(this + 0x34d0);
    if l != 0 {
        let conta = cpt11_w(l) as i16;
        cpt11_pd(l + 0xa, cpt11_d(l + 2));
        if conta > 0 {
            let mut n = conta as i32;
            loop {
                let item = proximo(cpt11_d(this + 0x34d0));
                if item != 0 {
                    let r = cpt11_d(item);
                    tirar(cpt11_d(this + 0x34d0), item);
                    resposta_do_torneio(this, r, 1, 0);
                }
                let l = cpt11_d(this + 0x34d0);
                n -= 1;
                cpt11_pd(l + 0xa, cpt11_d(l + 2));
                if n == 0 {
                    break;
                }
            }
        }
    }
    let l = cpt11_d(this + 0x34d4);
    if l != 0 {
        cpt11_pd(l + 0xa, cpt11_d(l + 2));
        let mut item = proximo(cpt11_d(this + 0x34d4));
        while item != 0 {
            resposta_do_amistoso(this, cpt11_d(item), 0);
            item = proximo(cpt11_d(this + 0x34d4));
        }
    }
    let l = cpt11_d(this + 0x34c8);
    if l != 0 {
        cpt11_pd(l + 0xa, cpt11_d(l + 2));
        let mut p = proximo(cpt11_d(this + 0x34c8));
        while p != 0 {
            if cpt11_d(p + 0x10) == 0 || cpt11_d(p + 0x14) == 0 {
                tirar(cpt11_d(this + 0x34c8), p);
            }
            // ponytail: como a original, a data do pedido é olhada mesmo depois de tirá-lo (o bloco já tirado)
            if friendly_date_after(hoje, p) {
                tirar(cpt11_d(this + 0x34c8), p);
            }
            p = proximo(cpt11_d(this + 0x34c8));
        }
    }
    for k in 0..0x7f {
        let e = this + 0xb1 + k * 0x69;
        if cpt11_d(e) != 0 && cpt11_d(e + 0x65) == 0 {
            let mut t = [0u8; 8];
            let fim = menos_dias(e + 4, &mut t, 2);
            if friendly_date_after(hoje, fim) {
                cancelar(this, e, 0);
            }
        }
    }
    let l = cpt11_d(this + 0x34cc);
    if l != 0 {
        cpt11_pd(l + 0xa, cpt11_d(l + 2));
        let mut item = proximo(cpt11_d(this + 0x34cc));
        while item != 0 {
            for k in 0..10 {
                let s = item + 0x15 + k * 0x19;
                let mut t = [0u8; 8];
                let fim = menos_dias(s - 0xc, &mut t, 2);
                if friendly_date_after(hoje, fim) {
                    cpt11_pd(s, 0);
                }
            }
            item = proximo(cpt11_d(this + 0x34cc));
        }
    }
    // ponytail: a original faz `mov al, 1` e deixa sobra em cima; quem chama testa o byte
    1
}

/// { 0x5c27a0 } (thiscall, ret 0, friendly.cpp): `Friendly::plan_ai_preseason_friendlies()`: monta um candidato de
/// 0x16 bytes (+0 o id do clube, +4 zero, +5 a faixa = reputação +0x80 / 500, +6 o id da nação, +0xa e +0x12 listas
/// nulas, +0xe/+0x10 a janela −1) para cada clube ([0xae23b4], 0x245 bytes, [0xae235c] deles) que não é seleção
/// (0x53b900) e tem nação; ordena os candidatos pela faixa (qsort 0x945333 com 0x5c5270) e um vetor de ponteiros pela
/// nação e faixa (0x5c5290); põe as janelas (0x5c2d40), planeja torneios e excursões (0x5c3040) e os amistosos
/// (0x5c4770); libera as listas dos candidatos (0x68d040 + 0x944c94) e os dois vetores (0x9452ca). Sem memória para os
/// candidatos: a caixa (linha 0x18a).
unsafe extern "thiscall" fn plan_ai_preseason_friendlies(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let janelas: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(0x5c2d40usize);
    let torneios: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x5c3040usize);
    let amistosos: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x5c4770usize);
    let fim_da_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let v = alocar((cpt11_d(0xae235c) as u32).wrapping_mul(0x16));
    if v == 0 {
        return show_error_at(0x9cd0a0, 0x18a);
    }
    let mut n: i16 = 0;
    let mut clube = cpt11_d(0xae23b4);
    let mut i: i32 = 0;
    if cpt11_d(0xae235c) as i32 > 0 {
        loop {
            if is_national_team(clube) == 0 && cpt11_d(clube + 0x53) != 0 {
                let r = v + n as usize * 0x16;
                let rep = cpt11_w(clube + 0x80) as i16 as i32;
                crate::comparison::branch("cpt_11 pre-season candidate");
                let band = (rep / 500) as u8; // cpt11: a faixa
                cpt11_pd(r, cpt11_d(clube));
                *((r + 4) as *mut u8) = 0;
                *((r + 5) as *mut u8) = band;
                cpt11_pd(r + 6, cpt11_d(cpt11_d(clube + 0x53)));
                cpt11_pd(r + 0xa, 0);
                cpt11_pd(r + 0x12, 0);
                core::ptr::write_unaligned((r + 0xe) as *mut u16, 0xffff);
                core::ptr::write_unaligned((r + 0x10) as *mut u16, 0xffff);
                n += 1;
            }
            clube += 0x245;
            i += 1;
            if i as i16 as i32 >= cpt11_d(0xae235c) as i32 {
                break;
            }
        }
    }
    let ns = n as i32;
    qsort(v, ns as u32, 0x16, 0x5c5270);
    let p = alocar((ns as u32).wrapping_mul(4)); // a original não confere
    for k in 0..ns.max(0) as usize {
        cpt11_pd(p + k * 4, v + k * 0x16);
    }
    qsort(p, ns as u32, 4, 0x5c5290);
    janelas(this, p, ns);
    torneios(this, v, p, ns);
    amistosos(this, v, p, ns);
    for k in 0..ns.max(0) as usize {
        for l in [cpt11_d(v + k * 0x16 + 0xa), cpt11_d(v + k * 0x16 + 0x12)] {
            if l != 0 {
                fim_da_lista(l);
                apagar(l);
            }
        }
    }
    liberar(v);
    liberar(p);
}

/// O trabalho da 0x5c2d40 sobre os `n` ponteiros para candidatos `ptrs` (ordenados pela nação e faixa): para cada uma
/// das 35 entradas de [0xb63d60] (0x48 bytes: +0 a nação, +0x26 a liga, +0x2a o início) com nação em uso (+0x11c):
/// o fim da janela é o início − 1 semana (0x54cab0) em dias desde hoje e o começo, 18 dias antes (no mínimo 3); acha
/// (bsearch 0x946464 com 0x5c5290) o primeiro candidato da nação e grava a janela (+0xe, +0x10) nos seguintes enquanto
/// o clube for da nação (sem o último do vetor, como a original); o primeiro e o último clube da tabela da liga (+0xa7
/// da competição de tipo 1) ganham uma lista nova (0x944cf1 + 0x68d020) em +0x12. Com `gravar` falso só monta a lista
/// (candidato, começo, fim, lista nova). A original calcula a data de início da liga (e + dias, tipo 2) e não usa.
unsafe fn set_friendly_windows(ptrs: usize, n: i32, gravar: bool) -> Vec<(usize, i16, i16, bool)> {
    let menos_semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54cab0usize);
    let data_nova: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let bsearch: unsafe extern "C" fn(*const usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let mut plano = Vec::new();
    for k in 0..0x23 {
        let ent = 0xb63d60 + k * 0x48;
        let nac = cpt11_d(ent);
        if nac == 0 || *((nac + 0x11c) as *const u8) == 0 {
            continue;
        }
        let (mut prim, mut ult) = (u32::MAX as usize, u32::MAX as usize);
        let lg = cpt11_d(ent + 0x26);
        let comp = if lg != 0 { cpt11_d(cpt11_d(0xadadfc) + cpt11_d(lg) * 4) } else { 0 };
        if comp != 0 && *((comp + 0x43) as *const u8) == 1 {
            if gravar {
                // as datas de início da liga que a original monta e não usa (só locais)
                let (mut t1, mut t2, mut t3, mut t4, mut t5, mut t6) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
                let x = cpt11_d(comp + 0xa3);
                let ano = *((x + 5) as *const i8) as i16 as u16 as u32;
                let r = data_nova(&mut t1, cpt11_w(x + 3) as u32, ano);
                copiar(&mut t2, &mut t3, r);
                if *((x + 0x21) as *const u8) == 2 {
                    let r = data_nova(&mut t4, cpt11_w(x + 3) as u32, ano);
                    let r = mais_dias(r, &mut t5, *((x + 0x22) as *const i8) as i16 as u16 as u32);
                    copiar(&mut t6, &mut t3, r);
                }
            }
            let t = cpt11_d(comp + 0xa7);
            let a = cpt11_d(t);
            prim = if a != 0 { cpt11_d(a) } else { u32::MAX as usize };
            let b = cpt11_d(t + 6);
            ult = if b != 0 { cpt11_d(b) } else { u32::MAX as usize };
        }
        let mut l = [0u8; 8];
        menos_semanas(ent + 0x2a, &mut l, 1);
        let dano = u16::from_le_bytes([l[2], l[3]]).wrapping_sub(cpt11_w(0xae2c92));
        let fim = (dano as u32).wrapping_mul(365).wrapping_sub(cpt11_d(0xae2c90) as u32)
            .wrapping_add(u32::from_le_bytes([l[0], l[1], l[2], l[3]])) as u16 as i16;
        let mut ini = fim.wrapping_sub(18); // cpt11: o início da janela
        if ini < 3 {
            ini = 3;
        }
        let mut chave = [0u8; 0x16];
        chave[5] = 0xff;
        chave[6..10].copy_from_slice(&(cpt11_d(nac) as u32).to_le_bytes());
        let pchave = chave.as_ptr() as usize;
        let mut q = bsearch(&pchave, ptrs, n as u32, 4, 0x5c5290);
        if q == 0 {
            continue;
        }
        while q > ptrs {
            q -= 4;
            if cpt11_d(cpt11_d(q) + 6) != cpt11_d(nac) {
                q += 4;
                break;
            }
        }
        let ate = ptrs.wrapping_add((n as usize).wrapping_mul(4)).wrapping_sub(4);
        while q < ate {
            let c = cpt11_d(q);
            let id = cpt11_d(c);
            if cpt11_d(cpt11_d(0xae23b4) + id * 0x245 + 0x53) != nac {
                break;
            }
            let nova = id == prim || id == ult;
            if gravar {
                if nova {
                    let m = novo(0xe);
                    cpt11_pd(c + 0x12, if m != 0 { lista_nova(m) } else { 0 });
                }
                crate::comparison::branch("cpt_11 window recorded");
                core::ptr::write_unaligned((c + 0xe) as *mut i16, ini);
                core::ptr::write_unaligned((c + 0x10) as *mut i16, fim);
            }
            plano.push((c, ini, fim, nova));
            q += 4;
        }
    }
    plano
}

/// { 0x5c2d40 } (thiscall, ret 8, friendly.cpp): `Friendly::set_preseason_windows(candidatos, n)`: a janela da
/// pré-temporada de cada nação nos candidatos dela (ver `set_friendly_windows`); o `this` não é lido. A original constrói e
/// destrói duas datas locais que não usa (0x9463af / 0x9462bb com 0x54a110 / 0x5a53c0).
unsafe extern "thiscall" fn set_preseason_windows(_this: usize, ptrs: usize, n: i32) {
    set_friendly_windows(ptrs, n, true);
}

/// Comparador: anda o calendário e planeja (aloca, sorteia): só a original.
unsafe extern "thiscall" fn compare_friendly_update_for_date(this: usize, hoje: usize, ativo: u32) -> u32 {
    // ponytail: cancela torneios, tira pedidos e cria amistosos (não roda duas vezes); a cópia se prova pela carreira
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%)(this, hoje, ativo)
}

/// Comparador: monta e planeja os amistosos da IA (aloca, sorteia): só a original.
unsafe extern "thiscall" fn compare_plan_ai_preseason_friendlies(this: usize) {
    // ponytail: os torneios, as excursões e os amistosos sorteiam e alocam; a cópia se prova pela carreira
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// Comparador: grava as janelas e cria listas (não roda duas vezes): o plano antes (sem gravar), a original grava;
/// cada candidato planejado tem de ter a janela e, quando é a ponta da tabela, uma lista em +0x12.
unsafe extern "thiscall" fn compare_set_preseason_windows(this: usize, ptrs: usize, n: i32) {
    let plano = set_friendly_windows(ptrs, n, false);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(%%I2%%)(this, ptrs, n);
    let erros: Vec<String> = plano.iter()
        .filter(|&&(c, ini, fim, nova)| {
            core::ptr::read_unaligned((c + 0xe) as *const i16) != ini || core::ptr::read_unaligned((c + 0x10) as *const i16) != fim
                || (nova && cpt11_d(c + 0x12) == 0)
        })
        .map(|&(c, ini, fim, nova)| format!("candidato {:x}: esperado {}..{} lista {} obtido {}..{} lista {:x}", c, ini, fim, nova,
            core::ptr::read_unaligned((c + 0xe) as *const i16), core::ptr::read_unaligned((c + 0x10) as *const i16), cpt11_d(c + 0x12)))
        .collect();
    SCOREBOARD.record(erros.is_empty(), || format!("set_preseason_windows(n {}) {} candidatos: {}", n, plano.len(), erros.join("; ")));
}
