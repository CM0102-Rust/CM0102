// ---------- %%LEVA%%a leva: a Copa Africana de Nações: a fase seguinte, a tabela do grupo A e o histórico da temporada (african_nations.cpp) ----------

unsafe fn cqk12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqk12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x401e10 } (thiscall, ret 0, african_nations.cpp; a virtual +0x28): `AfricanNations::advance_stage()`: na última
/// fase (+0x30 ≥ +0x2c − 1), a caixa da linha 0x2ba; senão +0x30 sobe e, chegando a 3 (depois dos grupos), o mata-mata
/// (0x402090: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn african_nations_advance_stage(this: usize) {
    let f = cqk12_d(this + 0x30) as i32;
    if f >= (cqk12_d(this + 0x2c) as i32).wrapping_sub(1) {
        aviso_em(0x9870f8, 0x2ba);
        return;
    }
    let f = f + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    if f == 3 { // cqk12: o mata-mata da Copa Africana depois dos grupos
        crate::comparacao::ramo("cqk_12 mata-mata da copa africana");
        let mata: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x402090usize);
        mata(this);
    }
}

// ponytail: o mata-mata (0x402090) aloca a fase e sorteia: não roda duas vezes. Só a original; o plano: +0x30 sobe um
// (fora da última fase, que dá a caixa).
unsafe extern "thiscall" fn comparar_african_nations_advance_stage(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let f = cqk12_d(this + 0x30) as i32;
    if f >= (cqk12_d(this + 0x2c) as i32).wrapping_sub(1) {
        return o(this);
    }
    o(this);
    let depois = cqk12_d(this + 0x30) as i32;
    PLACAR.registrar(depois == f + 1, || format!("african_nations_advance_stage({:#x}) fase {} -> {}", this, f, depois));
}

/// { 0x4019c0 } (thiscall, ret 0, african_nations.cpp): `AfricanNations::init_group_a_table()`: o grupo A com 4 times
/// (+0x3e), a tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa
/// da linha 0x1f1 e devolve 0), cada linha i com a seleção i da lista dos sorteados (+0xee) (0x687430: push 0; push 0;
/// push seleção; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this
/// -> thiscall, ret 0) e devolve 1.
unsafe extern "thiscall" fn init_group_a_table(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    let times = 4i16; // cqk12: os quatro do grupo A
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, times);
    let t = novo(0xec);
    core::ptr::write_unaligned((this + 0xb1) as *mut u32, t as u32);
    if t == 0 {
        aviso_em(0x9870f8, 0x1f1);
        return 0;
    }
    crate::comparacao::ramo("cqk_12 grupo a da copa africana");
    let mut i = 0i8;
    while (i as i16) < cqk12_w(this + 0x3e) {
        let selecao = cqk12_d(cqk12_d(this + 0xee).wrapping_add((i as isize * 4) as usize)); // cqk12: a seleção da linha
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, selecao, 0, 0);
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico de posições) e a notícia vai aos técnicos: não roda duas
// vezes. Só a original; o plano: 4 times, a tabela, e em cada linha i a seleção i dos sorteados (+0xee) com a posição i
// (+4 da linha, 0x3b bytes).
unsafe extern "thiscall" fn comparar_init_group_a_table(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let lista = cqk12_d(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| cqk12_d(lista + i * 4));
    let r = o(this);
    let t = cqk12_d(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (cqk12_d(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && cqk12_w(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("init_group_a_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x401b90 } (thiscall, ret 0, african_nations.cpp; a virtual +0x30): `AfricanNations::record_season_honours()`: o
/// histórico vem do mata-mata: salta para a virtual +0x30 da fase 3 ([+0xc] + 0xc; jmp, sem quadro).
unsafe extern "thiscall" fn african_nations_record_season_honours(this: usize) {
    let fase = cqk12_d(cqk12_d(this + 0xc) + 0xc);
    let h: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqk12_d(cqk12_d(fase) + 0x30));
    h(fase);
}

// ponytail: o histórico (a virtual +0x30 do mata-mata, a 0x519a90 já em Rust com o seu comparador) não roda duas vezes;
// a rotina não grava nada além do salto. Só a original.
unsafe extern "thiscall" fn comparar_african_nations_record_season_honours(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
