// ---------- %%LEVA%%a leva: a Liga dos Campeões asiática: a fase seguinte, os dois grupos e as semifinais (asia_club_champ.cpp) ----------

unsafe fn cqt12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqt12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqt12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// { 0x40c070 } (thiscall, ret 0, asia_club_champ.cpp; a virtual +0x28): `AsiaClubChamp::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x355; senão +0x30 sobe e, chegando a 1 (depois dos mata-matas
/// das duas zonas), os dois grupos (0x40c110), chegando a 3 (depois dos grupos), as semifinais (0x40c3d0); as duas mov
/// ecx, this -> thiscall, ret 0.
unsafe extern "thiscall" fn asia_club_champ_advance_stage(this: usize) {
    let f = cqt12_d(this + 0x30) as i32;
    if f >= (cqt12_d(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0x987a40, 0x355);
    }
    let f = f + 1;
    cqt12_pd(this + 0x30, f as usize);
    if f == 1 {
        crate::comparison::branch("cqt_12 grupos da asia");
        let grupos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x40c110usize);
        grupos(this);
    } else if f == 3 {
        let semifinais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x40c3d0usize);
        semifinais(this);
    }
}

// ponytail: os grupos e as semifinais alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um
// (os grupos gravam 2, o mesmo valor) fora da última fase, que dá a caixa e não muda nada.
unsafe extern "thiscall" fn comparar_asia_club_champ_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let f = cqt12_d(this + 0x30) as i32;
    let ultima = f >= (cqt12_d(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = cqt12_d(this + 0x30) as i32;
    let esperado = if ultima { f } else if f + 1 == 1 { 2 } else { f + 1 };
    SCOREBOARD.record(depois == esperado, || format!("asia_club_champ_advance_stage({:#x}) fase {} -> {}", this, f, depois));
}

/// Os 8 dos grupos: os vivos (+5 = 1 na tabela de 6 bytes) do mata-mata da primeira zona (a tabela da copa, +0xa7) até 4
/// e depois os do mata-mata da segunda zona (a tabela +0xa7 da fase [+0xc][0]) até 8; as duas buscas vão até o +0x3e da
/// copa (palavra com sinal). Devolve os 8 (as vagas que sobrarem ficam 0: a original as deixa com sobra da pilha).
unsafe fn group_stage_qualifiers(this: usize) -> [usize; 8] {
    let mut v = [0usize; 8];
    let mut n = 0usize;
    let times = cqt12_w(this + 0x3e);
    let mut i = 0i16;
    while i < times && n < 4 {
        let e = cqt12_d(this + 0xa7) + i as usize * 6;
        if *((e + 5) as *const u8) == 1 {
            v[n] = cqt12_d(e); // cqt12: um vivo da primeira zona
            n += 1;
        }
        i += 1;
    }
    let mut i = 0i16;
    while i < times && n < 8 {
        let e = cqt12_d(cqt12_d(cqt12_d(this + 0xc)) + 0xa7) + i as usize * 6;
        if *((e + 5) as *const u8) == 1 {
            v[n] = cqt12_d(e);
            n += 1;
        }
        i += 1;
    }
    v
}

/// Um grupo: a tabela das rodadas pela virtual +0x3c (push 0; push &id; push &rodadas; push `g`; mov ecx, this ->
/// thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall,
/// ret 0x58, 22 argumentos: a copa, 4, os 4 times, 1, +4, a tabela das rodadas, as rodadas (word), 3, 1, 8, &{1, 2, 0,
/// 0}, &{2, 0, 0, 0}, o ano +0x40 (word), `g`, o id (word), +0x51, `a17`, 0, 0x28, −1, 0, 2; a original empurra os words
/// e bytes com lixo nos bytes altos, que o construtor não lê) em [+0xc][`g`]; a tabela liberada (0x9452ca). Devolve a fase.
unsafe fn create_asia_group(this: usize, g: u32, times: *const usize, a17: u32) -> usize {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, *const usize, u32, usize, usize, u32, u32, u32, u32, *const u8, *const u8, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let regras_a: [u8; 4] = [1, 2, 0, 0];
    let regras_b: [u8; 4] = [2, 0, 0, 0];
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt12_d(cqt12_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(
            p,
            this,
            4,
            times,
            1,
            cqt12_d(this + 4),
            tabela,
            rodadas as u16 as u32,
            3,
            1,
            8,
            regras_a.as_ptr(),
            regras_b.as_ptr(),
            cqt12_w(this + 0x40) as u16 as u32,
            g,
            id as u16 as u32,
            *((this + 0x51) as *const u8) as u32,
            a17,
            0,
            0x28,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    cqt12_pd(cqt12_d(this + 0xc) + g as usize * 4, fase);
    soltar(tabela);
    cqt12_d(cqt12_d(this + 0xc) + g as usize * 4)
}

/// { 0x40c110 } (thiscall, ret 0, asia_club_champ.cpp; quadro SEH só para o new das fases: sem destrutor no caminho
/// normal): `AsiaClubChamp::create_group_stages()`: os 8 vivos das duas zonas (group_stage_qualifiers); o grupo 1 (fase 1, os 4
/// primeiros, o 17º argumento 2; sem fase, a caixa da linha 0x39e e volta) e a notícia dele (0x684230 na fase, thiscall,
/// ret 0); o grupo 2 (fase 2, os 4 seguintes, o 17º argumento 1; sem fase, a caixa da linha 0x3b4 e volta), a notícia e
/// +0x30 = 2.
unsafe extern "thiscall" fn create_group_stages(this: usize) {
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    let v = group_stage_qualifiers(this);
    let grupo_1 = [v[0], v[1], v[2], v[3]]; // cqt12: o grupo 1
    let f = create_asia_group(this, 1, grupo_1.as_ptr(), 2);
    if f == 0 {
        return show_error_at(0x987a40, 0x39e);
    }
    noticia(f);
    crate::comparison::branch("cqt_12 dois grupos da asia");
    let grupo_2 = [v[4], v[5], v[6], v[7]]; // cqt12: o grupo 2
    let f = create_asia_group(this, 2, grupo_2.as_ptr(), 1);
    if f == 0 {
        return show_error_at(0x987a40, 0x3b4);
    }
    noticia(f);
    cqt12_pd(this + 0x30, 2);
}

// ponytail: as fases são alocadas e as notícias vão aos técnicos: não roda duas vezes. Só a original; o plano: os vivos
// lidos antes; depois as fases [+0xc][1] e [+0xc][2] com 4 times, o ano da copa e os 4 vivos de cada (em qualquer ordem:
// o construtor ordena a tabela) e +0x30 = 2.
unsafe extern "thiscall" fn comparar_create_group_stages(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    if cqt12_d(this + 0xc) == 0 || cqt12_d(cqt12_d(this + 0xc)) == 0 {
        return o(this);
    }
    let v = group_stage_qualifiers(this);
    o(this);
    let confere = |g: usize, de: &[usize]| {
        let f = cqt12_d(cqt12_d(this + 0xc) + g * 4);
        let t = if f != 0 { cqt12_d(f + 0xb1) } else { 0 };
        if t == 0 || cqt12_w(f + 0x3e) != 4 || cqt12_w(f + 0x40) != cqt12_w(this + 0x40) {
            return false;
        }
        let mut a: Vec<usize> = (0..4).map(|k| cqt12_d(t + k * 0x3b)).collect();
        let mut b = de.to_vec();
        a.sort_unstable();
        b.sort_unstable();
        a == b
    };
    let ok = confere(1, &v[..4]) && confere(2, &v[4..]) && cqt12_d(this + 0x30) == 2;
    SCOREBOARD.record(ok, || format!("create_group_stages({:#x}): os grupos não batem com os vivos {:x?}", this, v));
}

/// Os 4 das semifinais, na ordem da lista: o 1º do grupo 1 ([+0xc][1], tabela +0xb1), o 2º do grupo 2 ([+0xc][2]), o
/// 1º do grupo 2 e o 2º do grupo 1 (os cruzamentos: as chaves são os pares da lista).
unsafe fn asia_semi_finalists(this: usize) -> [usize; 4] {
    let l = |g: usize, p: usize| cqt12_d(cqt12_d(cqt12_d(cqt12_d(this + 0xc) + g * 4) + 0xb1) + p * 0x3b);
    [l(1, 0), l(2, 1), l(2, 0), l(1, 1)] // cqt12: 1º do 1 × 2º do 2, 1º do 2 × 2º do 1
}

/// { 0x40c3d0 } (thiscall, ret 0, asia_club_champ.cpp, quadro SEH só para o new da fase: sem destrutor no caminho normal):
/// `AsiaClubChamp::create_semi_final_stage()`: a lista dos 4 (0x944e46(0x10): push; add esp, 4 -> cdecl; sem memória, a
/// caixa da linha 0x3d1 e volta), os cruzamentos de asia_semi_finalists; a tabela das rodadas pela virtual +0x3c (push
/// 0; push &id; push &rodadas; push 3; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2) push; add esp, 4 ->
/// cdecl; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 4, a lista, as rodadas, +4, a tabela, o ano +0x40,
/// 3, 2, o id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo nos 16 bits altos,
/// que o construtor não lê) em [+0xc][3]; a tabela liberada (0x9452ca). Sem fase: a caixa da linha 0x3e6 e volta (a lista
/// não é liberada, como na original). Com ela: o sorteio da rodada 0 (0x51c410 Cup::draw_round: push 0; mov ecx, fase ->
/// thiscall, ret 4) e a lista liberada.
unsafe extern "thiscall" fn asia_club_champ_create_semi_final_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c410usize);
    let lista = alocar(0x10);
    if lista == 0 {
        return show_error_at(0x987a40, 0x3d1);
    }
    crate::comparison::branch("cqt_12 cruzamentos da asia");
    for (k, &c) in asia_semi_finalists(this).iter().enumerate() {
        cqt12_pd(lista + k * 4, c);
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt12_d(cqt12_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 3, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 4, lista, rodadas as u16 as u32, cqt12_d(this + 4), tabela, cqt12_w(this + 0x40) as u16 as u32, 3, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqt12_pd(cqt12_d(this + 0xc) + 0xc, fase);
    soltar(tabela);
    let f = cqt12_d(cqt12_d(this + 0xc) + 0xc);
    if f == 0 {
        return show_error_at(0x987a40, 0x3e6);
    }
    sortear(f, 0);
    soltar(lista);
}

// ponytail: aloca a fase e sorteia a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][3] com 4
// times, o +4 da copa e os 4 cruzados (lidos antes) na lista dela (+0xa7, 6 bytes; como conjunto: o sorteio pode
// reordenar).
unsafe extern "thiscall" fn comparar_asia_club_champ_create_semi_final_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let fases = cqt12_d(this + 0xc);
    if fases == 0 || cqt12_d(fases + 4) == 0 || cqt12_d(fases + 8) == 0 {
        return o(this);
    }
    let mut plano = asia_semi_finalists(this).to_vec();
    o(this);
    let fase = cqt12_d(cqt12_d(this + 0xc) + 0xc);
    if fase == 0 {
        SCOREBOARD.record(false, || format!("asia_club_champ_create_semi_final_stage({:#x}) sem fase", this));
        return;
    }
    let lista = cqt12_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| cqt12_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqt12_w(fase + 0x3e) == 4 && cqt12_d(fase + 4) == cqt12_d(this + 4);
    SCOREBOARD.record(igual, || format!("asia_club_champ_create_semi_final_stage({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}
