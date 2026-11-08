// ---------- %%LEVA%%a leva: a Série C: os sete grupos da primeira fase, a segunda fase dos dezesseis e os dois grupos da terceira (bra_nat_third.cpp) ----------

unsafe fn cqs3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqs3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs3_b(a: usize) -> u8 {
    *(a as *const u8)
}

#[allow(clippy::type_complexity)]
type LeagueStageCtorFn = unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize;

/// { 0x42ee80 } (thiscall, ret 4, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_group_stage(g)`, o
/// grupo g (0 a 6) da primeira fase da Série C, da virada e do construtor (g de 0 a 6; a liga é o grupo de cima, e os
/// sete grupos são as fases 0 a 6).
/// 1) A tabela das rodadas pela virtual +0x3c (create_round_table: thiscall(this, g inteiro, &rodadas, &extra, 0), ret
///    0x10).
/// 2) Um vetor de 4 × +0x3e (malloc 0x944e46; sem memória: a caixa da linha 0x475 e volta) com os sete clubes do
///    grupo: as vagas 7(g + 1) a 7(g + 1) + 6 da lista dos 56 (+0xee; as sete primeiras são da liga).
/// 3) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, 7, o vetor, 2 (turnos: ida e volta), +4, a tabela das rodadas, as rodadas (word), os
///    pontos +0xc2 (`pv`) e +0xc3, a categoria +0xc4, &+0xc5, &+0xbe (as vagas da liga), o ano +0x40 (word), g (o
///    dword inteiro), o extra (word), +0x51, 1, 0, +0xd9 (word), −1, 0, 2; words e bytes limpos). Sem memória: 0.
/// 4) [+0xc][g] = a fase. Sem fase: a caixa da linha 0x487 e volta (sem liberar o vetor nem a tabela). Com ela: o vetor
///    e a tabela liberados (free 0x9452ca) e +0x30 = g (o byte com sinal).
unsafe fn bra_nat_third_create_group_stage_cqs3_grupo(this: usize, g: u32, pv: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let construir: LeagueStageCtorFn = core::mem::transmute(0x68b100usize);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqs3_d(cqs3_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, g, &mut rodadas, &mut extra, 0);
    let v = alocar(((cqs3_w(this + 0x3e) as i32) << 2) as u32);
    if v == 0 {
        return show_error_at(0x98aba4, 0x475);
    }
    let k = g as u8 as i8 as i32;
    for j in 0..7usize {
        core::ptr::write_unaligned((v + 4 * j) as *mut usize, cqs3_d(cqs3_d(this + 0xee) + 4 * (7 * (k + 1) as usize + j)));
    }
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(p, this, 7, v, 2, cqs3_d(this + 4), t, rodadas as u16 as u32, cqs3_b(this + pv) as u32, cqs3_b(this + 0xc3) as u32,
                  cqs3_b(this + 0xc4) as u32, this + 0xc5, this + 0xbe, cqs3_w(this + 0x40) as u16 as u32, g, extra as u16 as u32,
                  cqs3_b(this + 0x51) as u32, 1, 0, cqs3_w(this + 0xd9) as u16 as u32, -1, 0, 2)
    } else {
        0
    };
    core::ptr::write_unaligned((cqs3_d(this + 0xc) + 4 * k as usize) as *mut usize, fase);
    if cqs3_d(cqs3_d(this + 0xc) + 4 * k as usize) == 0 {
        return show_error_at(0x98aba4, 0x487);
    }
    crate::comparison::branch("cqs_3 grupo");
    soltar(v);
    soltar(t);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, k);
}

/// { 0x42ee80 } (acima): o grupo g da primeira fase da Série C.
unsafe extern "thiscall" fn bra_nat_third_create_group_stage(this: usize, g: u32) {
    bra_nat_third_create_group_stage_cqs3_grupo(this, g, 0xc2) // cqs3: os pontos por vitória nos grupos
}

/// A ordem da segunda fase: o k-ésimo melhor (pelos pontos) vai à vaga `SECOND_ROUND_SEEDS[k]` da lista dos dezesseis.
const SECOND_ROUND_SEEDS: [u8; 16] = [0, 2, 4, 6, 8, 10, 12, 14, 15, 13, 11, 9, 7, 5, 3, 1]; // cqs3: as vagas da segunda fase

/// { 0x42e430 } (thiscall, ret 0, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_second_round_stage()`,
/// a segunda fase da Série C, do fim dos grupos.
/// 1) Uma lista de 16 (malloc 0x944e46 de 0x40; sem memória: a caixa da linha 0x2e6 e volta).
/// 2) Os dois primeiros de cada um dos oito grupos (a tabela da liga e as das fases 0 a 6, nessa ordem), em registros de
///    6 bytes (o clube e os pontos, +0x19 da linha).
/// 3) Dezesseis vezes: o registro de mais pontos (o primeiro, no empate) entre os que ainda têm clube sai (o clube vira
///    0) e o clube vai à vaga `SECOND_ROUND_SEEDS[k]`: o melhor contra o 16º, o 2º contra o 15º... (sem nenhum: a caixa da
///    linha 0x30b; a original lê então o endereço 0 e cai).
/// 4) A tabela das rodadas pela virtual +0x3c (thiscall(this, 7, &rodadas, &extra, 0), ret 0x10); a fase (operator new
///    0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00, thiscall, ret 0x3c: a liga, 16, a lista, as rodadas, +4, a
///    tabela, o ano, 7, 1, o extra, 0x14, 0, 0, 0, sementes nulas; words limpas) em [+0xc][7]. Sem fase: a caixa da
///    linha 0x31b e volta. Com ela: a tabela e a lista liberadas (free 0x9452ca) e +0x30 = 7.
unsafe extern "thiscall" fn create_second_round_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize =
        core::mem::transmute(0x522e00usize);
    let lista = alocar(0x40);
    if lista == 0 {
        return show_error_at(0x98aba4, 0x2e6);
    }
    let mut r = [(0usize, 0i16); 16];
    for (k, g) in (-1i32..7).enumerate() {
        let t = cqs3_d(if g == -1 { this } else { cqs3_d(cqs3_d(this + 0xc) + 4 * g as usize) } + 0xb1);
        r[2 * k] = (cqs3_d(t), cqs3_w(t + 0x19));
        r[2 * k + 1] = (cqs3_d(t + 0x3b), cqs3_w(t + 0x54));
    }
    for k in 0..16 {
        let mut melhor: Option<usize> = None;
        for (i, e) in r.iter().enumerate() {
            if melhor.map_or(true, |m| e.1 > r[m].1) && e.0 != 0 {
                melhor = Some(i);
            }
        }
        let c = match melhor {
            Some(m) => {
                let c = r[m].0;
                r[m].0 = 0;
                c
            }
            None => {
                show_error_at(0x98aba4, 0x30b);
                core::ptr::read_volatile(core::ptr::null::<usize>()) // ponytail: a original lê o endereço 0 aqui e cai
            }
        };
        core::ptr::write_unaligned((lista + 4 * SECOND_ROUND_SEEDS[k] as usize) as *mut usize, c);
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqs3_d(cqs3_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 7, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, 0x10, lista, rodadas as u16 as u32, cqs3_d(this + 4), t, cqs3_w(this + 0x40) as u16 as u32, 7, 1,
                  extra as u16 as u32, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((cqs3_d(this + 0xc) + 0x1c) as *mut usize, fase);
    if cqs3_d(cqs3_d(this + 0xc) + 0x1c) == 0 {
        return show_error_at(0x98aba4, 0x31b);
    }
    crate::comparison::branch("cqs_3 segunda fase");
    soltar(t);
    soltar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 7);
}

/// { 0x42e700 } (thiscall, ret 0, bra_nat_third.cpp; quadro SEH só para o new): `BraNatThird::create_third_round_groups()`,
/// a terceira fase da Série C, do fim da segunda: os vencedores da segunda fase ([+0xc][7], os registros de 6 bytes de
/// +0xa7 com +5 = 1, na ordem) numa lista local de 8, embaralhada (comp_util_shuffle_pointer_array 0x4afb40, cdecl(v,
/// 8)); os quatro primeiros formam o grupo A (fase 8) e os outros quatro o grupo B (fase 9). Cada grupo: a tabela das
/// rodadas pela virtual +0x3c (thiscall(this, 8 ou 9, &rodadas, &extra, 0)) e a fase (operator new 0x944cf1(0xee) e o
/// construtor League_Stage 0x68b100: a liga, 4, a lista (ou a lista + 4), 2 (ida e volta), +4, a tabela, as rodadas
/// (word), 3 pontos por vitória, 1 por empate, a categoria 5, &{1,2,4,0} (os critérios de desempate), &{2,0,0,0} (duas
/// vagas de cima), o ano, 8 ou 9, o extra (word), 0x14, 1, 0, 2 (os bits de regra), −1, 0, 2) em [+0xc][8] ou [9];
/// sem fase: a caixa da linha 0x353 ou 0x363 e volta; com ela, a tabela liberada (free 0x9452ca). Depois do segundo,
/// +0x30 = 9. [inferência] Os dois primeiros de cada grupo sobem à Série B; os campeões dos grupos fazem a final.
unsafe extern "thiscall" fn create_third_round_groups(this: usize) {
    let embaralhar: unsafe extern "C" fn(*mut usize, u32) = core::mem::transmute(0x4afb40usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let vagas: [u8; 4] = [2, 0, 0, 0];
    let desempate: [u8; 4] = [1, 2, 4, 0];
    let mut v = [0usize; 8];
    let segunda = cqs3_d(cqs3_d(this + 0xc) + 0x1c);
    let mut n = 0usize;
    let mut i = 0i32;
    while i < cqs3_w(segunda + 0x3e) as i32 {
        let e = cqs3_d(segunda + 0xa7) + 6 * i as usize;
        // ponytail: a original grava sem limite numa lista de 8 na pilha; a fase tem 8 vencedores (a cópia para no 8º).
        if cqs3_b(e + 5) == 1 && n < 8 {
            v[n] = cqs3_d(e);
            n += 1;
        }
        i += 1;
    }
    embaralhar(v.as_mut_ptr(), 8); // cqs3: os oito vencedores embaralhados
    for (k, f) in [(8u32, 0x353u32), (9, 0x363)] {
        let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqs3_d(cqs3_d(this) + 0x3c));
        let (mut rodadas, mut extra) = (0i16, 0i16);
        let t = tabela(this, k, &mut rodadas, &mut extra, 0);
        let p = novo(0xee);
        let fase = if p != 0 {
            construir(p, this, 4, v[4 * (k as usize - 8)..].as_ptr(), 2, cqs3_d(this + 4), t, rodadas as u16 as u32, 3, 1, 5,
                      desempate.as_ptr(), vagas.as_ptr(), cqs3_w(this + 0x40) as u16 as u32, k, extra as u16 as u32, 0x14, 1, 0, 2, -1, 0, 2)
        } else {
            0
        };
        core::ptr::write_unaligned((cqs3_d(this + 0xc) + 4 * k as usize) as *mut usize, fase);
        if cqs3_d(cqs3_d(this + 0xc) + 4 * k as usize) == 0 {
            return show_error_at(0x98aba4, f);
        }
        soltar(t);
    }
    crate::comparison::branch("cqs_3 terceira fase");
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 9);
}

/// O comparador de um grupo: só a original (aloca o vetor, a tabela das rodadas e a fase), com o plano feito antes: os
/// sete clubes das vagas 7(g + 1).. da lista dos 56. Depois, a fase na vaga g com +0x3e = 7, os sete na tabela dela
/// (como conjunto) e +0x30 = g.
unsafe extern "thiscall" fn comparar_bra_nat_third_create_group_stage(this: usize, g: u32) {
    let k = g as u8 as i8 as i32;
    let mut plano = [0usize; 7];
    for (j, c) in plano.iter_mut().enumerate() {
        *c = cqs3_d(cqs3_d(this + 0xee) + 4 * (7 * (k + 1) as usize + j));
    }
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, g);
    let fase = cqs3_d(cqs3_d(this + 0xc) + 4 * k as usize);
    let igual = fase != 0 && cqs3_w(fase + 0x3e) == 7 && cqs3_d(this + 0x30) as i32 == k && {
        let mut nela = [0usize; 7];
        for (j, c) in nela.iter_mut().enumerate() {
            *c = cqs3_d(cqs3_d(fase + 0xb1) + j * 0x3b);
        }
        let mut a = plano;
        a.sort_unstable();
        nela.sort_unstable();
        a == nela
    };
    SCOREBOARD.record(igual, || format!("bra_nat_third_create_group_stage({:#x}, {}) fase={:#x} plano={:x?}", this, k, fase, plano));
}

/// O comparador da segunda fase: só a original (aloca a lista, a tabela e a fase e sorteia a rodada), com o plano feito
/// antes pela mesma escolha numa cópia dos registros: a fase da vaga 7 existe, com +0x3e = 16 e os dezesseis na ordem
/// das vagas na lista dela (+0xa7, 6 bytes), e +0x30 = 7.
unsafe extern "thiscall" fn comparar_create_second_round_stage(this: usize) {
    let mut r = [(0usize, 0i16); 16];
    for (k, g) in (-1i32..7).enumerate() {
        let t = cqs3_d(if g == -1 { this } else { cqs3_d(cqs3_d(this + 0xc) + 4 * g as usize) } + 0xb1);
        r[2 * k] = (cqs3_d(t), cqs3_w(t + 0x19));
        r[2 * k + 1] = (cqs3_d(t + 0x3b), cqs3_w(t + 0x54));
    }
    let mut plano = [0usize; 16];
    for k in 0..16 {
        let mut melhor: Option<usize> = None;
        for (i, e) in r.iter().enumerate() {
            if melhor.map_or(true, |m| e.1 > r[m].1) && e.0 != 0 {
                melhor = Some(i);
            }
        }
        if let Some(m) = melhor {
            plano[SECOND_ROUND_SEEDS[k] as usize] = r[m].0;
            r[m].0 = 0;
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    let fase = cqs3_d(cqs3_d(this + 0xc) + 0x1c);
    let igual = fase != 0 && cqs3_w(fase + 0x3e) == 0x10 && cqs3_d(this + 0x30) == 7 && {
        let mut nela = [0usize; 16];
        for (j, c) in nela.iter_mut().enumerate() {
            *c = cqs3_d(cqs3_d(fase + 0xa7) + 6 * j);
        }
        let (mut a, mut b) = (plano, nela);
        a.sort_unstable();
        b.sort_unstable();
        a == b
    };
    SCOREBOARD.record(igual, || format!("create_second_round_stage({:#x}) fase={:#x} plano={:x?}", this, fase, plano));
}

/// O comparador da terceira fase: só a original (embaralha: sorteia; aloca as tabelas e as fases), com o plano: os
/// vencedores da segunda fase; depois, as fases 8 e 9 com 4 clubes cada, os oito juntos (como conjunto) e +0x30 = 9.
unsafe extern "thiscall" fn comparar_create_third_round_groups(this: usize) {
    let segunda = cqs3_d(cqs3_d(this + 0xc) + 0x1c);
    let mut plano = [0usize; 8];
    let mut n = 0usize;
    let mut i = 0i32;
    while i < cqs3_w(segunda + 0x3e) as i32 && n < 8 {
        let e = cqs3_d(segunda + 0xa7) + 6 * i as usize;
        if cqs3_b(e + 5) == 1 {
            plano[n] = cqs3_d(e);
            n += 1;
        }
        i += 1;
    }
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let (a, b) = (cqs3_d(cqs3_d(this + 0xc) + 0x20), cqs3_d(cqs3_d(this + 0xc) + 0x24));
    let igual = a != 0 && b != 0 && cqs3_w(a + 0x3e) == 4 && cqs3_w(b + 0x3e) == 4 && cqs3_d(this + 0x30) == 9 && {
        let mut nela = [0usize; 8];
        for j in 0..4 {
            nela[j] = cqs3_d(cqs3_d(a + 0xb1) + j * 0x3b);
            nela[4 + j] = cqs3_d(cqs3_d(b + 0xb1) + j * 0x3b);
        }
        let mut p = plano;
        p.sort_unstable();
        nela.sort_unstable();
        p == nela
    };
    SCOREBOARD.record(igual, || format!("create_third_round_groups({:#x}) n={} plano={:x?}", this, n, plano));
}
