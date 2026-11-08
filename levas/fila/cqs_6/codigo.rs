// ---------- %%LEVA%%a leva: o Troféu da Liga inglesa (LDV Vans Trophy): a final entre os campeões das duas chaves e a classificação final da copa nas estatísticas (eng_auto_cup.cpp) ----------

unsafe fn cqs6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O primeiro clube da lista de inscritos `l` (entradas de 6 bytes: +0 o clube, +5 a situação) com a situação `s`, entre
/// as `n` primeiras (n = +0x3e da copa, word com sinal); sem ele, None.
unsafe fn cqs6_vivo(l: usize, n: i32, s: u8) -> Option<usize> {
    (0..n.max(0) as usize).find(|&i| *((l + i * 6 + 5) as *const u8) == s).map(|i| cqs6_d(l + i * 6))
}

/// { 0x56bce0 } (thiscall, ret 0, eng_auto_cup.cpp; da fase seguinte, quadro SEH só para o new, sem destrutor):
/// `EngAutoCup::create_final_stage()`: a final do Troféu: um vetor de 2 (0x944e46 de 8; sem memória, a caixa da linha
/// 0x337) com o primeiro clube de situação 1 (o que segue vivo [inferência]) da lista da copa (+0xa7) e o primeiro de
/// situação 1 da lista da fase das chaves ([+0xc][0] +0xa7), ambos entre os +0x3e primeiros (da copa); as rodadas pelo
/// método +0x3c (push 0; push &extra; push &rodadas; push 1; mov ecx -> thiscall, ret 0x10); a fase (0x944cf1(0xb2);
/// Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 2, o vetor, as rodadas (word), +4, a tabela, o ano +0x40
/// (word), 1, 1, o extra (word), 0x14, 0, 0, 0, sem sementes) em [+0xc][1]; a tabela das rodadas liberada (0x9452ca). Sem
/// fase, a caixa da linha 0x35a (o vetor fica, como na original). Com ela, o sorteio da rodada (Cup::draw_round 0x51c410:
/// push 0; mov ecx, fase -> thiscall, ret 4) e o vetor liberado.
unsafe extern "thiscall" fn eng_auto_cup_create_final_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let l = alocar(8);
    if l == 0 {
        return aviso_em(0x9c4180, 0x337);
    }
    if let Some(c) = cqs6_vivo(cqs6_d(this + 0xa7), cqs6_w(this + 0x3e) as i32, 1) { // cqs6: o finalista da copa
        cqs6_pd(l, c);
    }
    if let Some(c) = cqs6_vivo(cqs6_d(cqs6_d(cqs6_d(this + 0xc)) + 0xa7), cqs6_w(this + 0x3e) as i32, 1) { // cqs6: o finalista das chaves
        cqs6_pd(l + 4, c);
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqs6_d(cqs6_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 1, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    let f = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 2, l, rodadas as u16 as u32, cqs6_d(this + 4), tabela, cqs6_w(this + 0x40) as u16 as u32, 1, 1, // cqs6: o ano da final
             extra as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqs6_pd(cqs6_d(this + 0xc) + 4, f);
    soltar(tabela);
    let f = cqs6_d(cqs6_d(this + 0xc) + 4);
    if f == 0 {
        return aviso_em(0x9c4180, 0x35a);
    }
    crate::comparacao::ramo("cqs_6 final do trofeu");
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c410usize);
    sortear(f, 0); // cqs6: o sorteio da final
    soltar(l);
}

/// Só a original (aloca a fase e sorteia), conferindo o plano: a fase em [+0xc][1] (com o vetor dos dois finalistas
/// achados antes, quando há fase).
unsafe extern "thiscall" fn comparar_eng_auto_cup_create_final_stage(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    o(this);
    let f = cqs6_d(cqs6_d(this + 0xc) + 4);
    PLACAR.registrar(f != 0, || format!("eng_auto_cup_create_final_stage({:#x}) sem a fase da final", this));
}

/// { 0x56c010 } (thiscall, ret 0, eng_auto_cup.cpp; a virtual da classificação final, n = 1 até 2002-06-01): `EngAutoCup::
/// rank_teams()`: a posição final de cada clube nas estatísticas da copa (+8, Comp_Stats; sem elas, a caixa da linha 0x3bb).
/// Um vetor de 2 × +0x3e ponteiros (0x944e46; sem memória, a caixa da linha 0x3c3): na primeira metade os clubes da lista
/// da copa (+0xa7, entradas de 6 bytes), na segunda os da fase das chaves ([+0xc][0] +0xa7); cada metade ordenada pela
/// reputação (+0x80, word), da maior para a menor (qsort 0x945333 com 0x4b07d0). Os dois primeiros (o de maior reputação
/// de cada metade): o de reputação maior (com sinal, estritamente) fica em 1º e o outro em 2º (Comp_Stats::
/// set_club_final_position 0x4a2540: push posição; push clube; mov ecx, +8 -> thiscall, ret 8). Depois, para cada metade,
/// da última rodada (+0x3c − 1) para a primeira, tantos clubes (os seguintes da metade, a partir do 2º) quantos a rodada
/// tem em +0x1a da tabela das rodadas (+0xa3, 0x68 bytes por rodada), com as posições 3, 5, 7... (a original empurra o
/// dword do byte da posição com lixo da pilha em cima; a 0x4a2540 só lê o byte). O vetor é liberado.
unsafe extern "thiscall" fn eng_auto_cup_rank_teams(this: usize) {
    if cqs6_d(this + 8) == 0 {
        return aviso_em(0x9c4180, 0x3bb);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(((cqs6_w(this + 0x3e) as i32) << 3) as u32);
    if v == 0 {
        return aviso_em(0x9c4180, 0x3c3);
    }
    let mut d: i16 = 0;
    while d < cqs6_w(this + 0x3e) {
        let e = d as i32 as usize * 6;
        cqs6_pd(v + d as i32 as usize * 4, cqs6_d(cqs6_d(this + 0xa7) + e));
        let n = cqs6_w(this + 0x3e) as i32;
        cqs6_pd(v + (n + d as i32) as usize * 4, cqs6_d(cqs6_d(cqs6_d(cqs6_d(this + 0xc)) + 0xa7) + e));
        d = d.wrapping_add(1);
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let n = cqs6_w(this + 0x3e) as i32;
    ordenar(v, n as u32, 4, 0x4b07d0);
    let n = cqs6_w(this + 0x3e) as i32;
    ordenar(v + n as usize * 4, n as u32, 4, 0x4b07d0);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4a2540usize);
    let a = cqs6_d(v);
    let b = cqs6_d(v + cqs6_w(this + 0x3e) as i32 as usize * 4);
    crate::comparacao::ramo("cqs_6 classificacao do trofeu");
    if cqs6_w(a + 0x80) > cqs6_w(b + 0x80) {
        posicao(cqs6_d(this + 8), a, 1);
        posicao(cqs6_d(this + 8), cqs6_d(v + cqs6_w(this + 0x3e) as i32 as usize * 4), 2);
    } else {
        posicao(cqs6_d(this + 8), a, 2);
        posicao(cqs6_d(this + 8), cqs6_d(v + cqs6_w(this + 0x3e) as i32 as usize * 4), 1);
    }
    for lado in 0..2u8 {
        let mut k: i16 = if lado == 0 { 1 } else { cqs6_w(this + 0x3e).wrapping_add(1) };
        let mut pos: u8 = 3;
        let ultima = cqs6_w(this + 0x3c).wrapping_sub(1);
        if ultima < 0 {
            continue;
        }
        let mut r = ultima as i32;
        while r >= 0 {
            let p = pos;
            let mut j: i16 = 0;
            while j < cqs6_w(cqs6_d(this + 0xa3) + r as usize * 0x68 + 0x1a) {
                let c = cqs6_d(v + k as i32 as usize * 4);
                k = k.wrapping_add(1);
                posicao(cqs6_d(this + 8), c, p as u32);
                pos = pos.wrapping_add(2);
                j = j.wrapping_add(1);
            }
            r -= 1;
        }
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    soltar(v);
}

// ponytail: grava nas estatísticas da copa (Comp_Stats, que acha ou acrescenta a entrada do clube: aloca); só a original
unsafe extern "thiscall" fn comparar_eng_auto_cup_rank_teams(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}
