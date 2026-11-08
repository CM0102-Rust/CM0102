// ---------- %%LEVA%%a leva: as excursões da pré-temporada: marcar os jogos da excursão e as duas ordens dos candidatos do amistoso (friendly.cpp) ----------

unsafe fn cpt12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpt12_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// A data `a` (+2 o ano sem sinal, +0 o dia com sinal) comparada à `b`: -1, 0 ou 1.
unsafe fn compare_dates(a: usize, b: usize) -> i32 {
    let (aa, ab) = (cpt12_w(a + 2), cpt12_w(b + 2));
    if aa != ab {
        return if aa < ab { -1 } else { 1 };
    }
    let (da, db) = (cpt12_w(a) as i16, cpt12_w(b) as i16);
    if da < db { -1 } else if da > db { 1 } else { 0 }
}

/// Uma lista nova (0x944cf1 de 0xe bytes + 0x68d020) ou 0 sem memória.
unsafe fn alloc_list() -> usize {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let m = novo(0xe);
    if m != 0 { lista_nova(m) } else { 0 }
}

/// { 0x5c4040 } (thiscall, ret 0xc, friendly.cpp): `Friendly::schedule_tour_matches(candidato, adversários, n)`: a
/// janela é a do candidato (+0xe..+0x10) cortada pela do primeiro adversário (se ele tem uma) e encurtada em 2n + 1
/// dias; sem espaço, 0. Até 5 tentativas: o começo anda um sorteio (0x90cfc0) da largura (o começo fica andado entre
/// as tentativas, como na original), o primeiro jogo é o início da temporada (+0xa7) + começo e o último, esse + 2n + 1
/// (0x54c5b0); uma data já guardada na lista do candidato (+0x12) depois do primeiro − 2 dias ou antes do último + 2
/// estraga a tentativa (com qualquer data guardada, todas falham; na quinta, segue assim mesmo). Depois, para cada
/// adversário, com a data livre para ele (0x5c6b70) e o amistoso montado (0x5c6f00, o clube em casa), manda o pedido
/// (0x5c5570) e guarda a data na lista dos dois (criadas se faltam; 0x68d0d0); a data anda 2 dias a cada adversário.
/// Devolve 1.
unsafe extern "thiscall" fn schedule_tour_matches(this: usize, cand: usize, adv: usize, n: u32) -> u32 {
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let copiar: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let data_padrao: unsafe extern "thiscall" fn(*mut [u8; 8]) -> usize = core::mem::transmute(0x54a110usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let livre: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize, u32) -> usize = core::mem::transmute(0x5c6b70usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, *mut [u8; 8], usize, u32) -> usize = core::mem::transmute(0x5c6f00usize);
    let pedir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5c5570usize);
    let guardar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let n = n as u8 as i8;
    let mut ini = cpt12_w(cand + 0xe) as i16;
    let mut fim = cpt12_w(cand + 0x10) as i16;
    let a0 = cpt12_d(adv);
    let x = cpt12_w(a0 + 0xe) as i16;
    if x != -1 {
        if x > ini {
            ini = x;
        }
        let y = cpt12_w(a0 + 0x10) as i16;
        if y < fim {
            fim = y;
        }
    }
    let largura = (n as u8).wrapping_mul(2).wrapping_add(1) as i8 as i16; // cpt12: a largura da excursão
    fim = fim.wrapping_sub(largura);
    if ini >= fim {
        return 0;
    }
    crate::comparison::branch("cpt_12 tour window");
    let (mut d1, mut d2, mut t, mut u) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    data_padrao(&mut d1);
    data_padrao(&mut d2);
    let mut tentativas: i8 = 0;
    while tentativas < 5 {
        let mut ok = true;
        ini = ini.wrapping_add(sortear(fim as i32 - ini as i32) as i16);
        let r = mais_dias(this + 0xa7, &mut t, ini as u16 as u32);
        copiar(&mut d1, &mut u, r);
        let r = mais_dias(d1.as_ptr() as usize, &mut t, largura as u16 as u32);
        copiar(&mut d2, &mut u, r);
        let l = cpt12_d(cand + 0x12);
        if l == 0 {
            break;
        }
        core::ptr::write_unaligned((l + 0xa) as *mut u32, cpt12_d(l + 2) as u32);
        let mut item = proximo(cpt12_d(cand + 0x12));
        while item != 0 {
            let antes = menos_dias(d1.as_ptr() as usize, &mut t, 2);
            let estraga = if compare_dates(antes, item) < 0 {
                true
            } else {
                let depois = mais_dias(d2.as_ptr() as usize, &mut u, 2);
                compare_dates(depois, item) > 0
            };
            if estraga {
                ok = false;
                break;
            }
            item = proximo(cpt12_d(cand + 0x12));
        }
        tentativas += 1;
        if ok {
            break;
        }
    }
    let clube = cpt12_d(0xae23b4) + cpt12_d(cand) * 0x245;
    let mut p = adv;
    for k in 0..(n as i32).max(0) {
        let outro = cpt12_d(0xae23b4) + cpt12_d(cpt12_d(p)) * 0x245;
        if livre(this, &mut d1, outro, 0) != 0 {
            let m = acrescentar(this, clube, &mut d1, outro, 0); // cpt12: o clube em casa
            if m != 0 {
                crate::comparison::branch("cpt_12 tour match scheduled");
                pedir(this, m);
                if cpt12_d(cand + 0x12) == 0 {
                    let l = alloc_list();
                    core::ptr::write_unaligned((cand + 0x12) as *mut u32, l as u32);
                }
                if cpt12_d(cpt12_d(p) + 0x12) == 0 {
                    let l = alloc_list();
                    core::ptr::write_unaligned((cpt12_d(p) + 0x12) as *mut u32, l as u32);
                }
                guardar(cpt12_d(cand + 0x12), &mut d1, 8);
                guardar(cpt12_d(cpt12_d(p) + 0x12), &mut d1, 8);
            }
        }
        p += 4;
        if k + 1 < n as i32 {
            crate::comparison::branch("cpt_12 next match date");
        }
        let r = mais_dias(d1.as_ptr() as usize, &mut t, 2); // cpt12: o próximo jogo
        copiar(&mut d1, &mut u, r);
    }
    1
}

/// { 0x5c5270 } (cdecl, friendly.cpp): o comparador do qsort dos candidatos do amistoso pela faixa (+5, com sinal),
/// a maior primeiro: faixa de b − faixa de a.
unsafe extern "C" fn compare_friendly_candidates_by_band(a: usize, b: usize) -> i32 {
    *((b + 5) as *const i8) as i32 - *((a + 5) as *const i8) as i32
}

/// { 0x5c5290 } (cdecl, friendly.cpp): o comparador (qsort e bsearch) dos ponteiros para candidatos: pela nação (+6,
/// a diferença em 32 bits); na mesma nação, pela faixa (+5, a maior primeiro), salvo faixa 0xff em um dos dois (0).
unsafe extern "C" fn compare_friendly_candidate_ptrs_by_nation(a: usize, b: usize) -> i32 {
    let (ca, cb) = (cpt12_d(a), cpt12_d(b));
    let dif = (cpt12_d(ca + 6) as u32).wrapping_sub(cpt12_d(cb + 6) as u32) as i32;
    if dif != 0 {
        return dif;
    }
    let (fa, fb) = (*((ca + 5) as *const u8), *((cb + 5) as *const u8));
    if fa == 0xff || fb == 0xff {
        return 0;
    }
    fb as i8 as i32 - fa as i8 as i32
}

/// Comparador: sorteia, monta amistosos e cria listas (não roda duas vezes): só a original.
unsafe extern "thiscall" fn compare_schedule_tour_matches(this: usize, cand: usize, adv: usize, n: u32) -> u32 {
    // ponytail: o sorteio e os amistosos novos não rodam duas vezes; a cópia se prova pela carreira
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(%%I0%%)(this, cand, adv, n)
}

/// Comparador: rotina pura, pela devolução.
unsafe extern "C" fn compare_compare_friendly_candidates_by_band(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = compare_friendly_candidates_by_band(a, b);
    SCOREBOARD.record(r == n, || format!("compare_friendly_candidates_by_band({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: rotina pura, pela devolução.
unsafe extern "C" fn compare_compare_friendly_candidate_ptrs_by_nation(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = compare_friendly_candidate_ptrs_by_nation(a, b);
    SCOREBOARD.record(r == n, || format!("compare_friendly_candidate_ptrs_by_nation({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
