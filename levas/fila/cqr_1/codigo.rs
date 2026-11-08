// ---------- %%LEVA%%a leva: os turnos do Carioca, do Mineiro, do Paranaense e da J1: a fase de pontos corridos de cada turno (bra_reg_rio.cpp, bra_reg_minas_gerais.cpp, bra_reg_parana.cpp, jap_j1.cpp) ----------

unsafe fn cqr1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqr1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqr1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O turno `k` de uma liga de dois turnos (o slot create_half_season_stage; o mesmo molde nos três estaduais e na J1,
/// conferido instrução a instrução: só mudam o fonte, a linha da caixa e o 20º argumento do construtor, 0xb nos
/// estaduais e 0xf na J1). Chamado do resultado de cada jogo (0x43af20 e as irmãs: push dl = +0x30 + 1; mov ecx ->
/// thiscall, ret 4; a original lê o argumento inteiro, usa o byte baixo e o empurra inteiro ao construtor).
/// 1) Os clubes da tabela da liga (+0xb1, linhas de 0x3b bytes, as +0x3e linhas) num vetor (malloc 0x944e46 de
///    4 × +0x3e; sem memória: a caixa e volta).
/// 2) A fase: operator new 0x944cf1(0xee) e o construtor League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos, na
///    ordem da original: a liga, +0x3e (word), o vetor, 1, +4, 0, 0, os pontos por vitória +0xc2 (`pv`), por empate +0xc3, a
///    categoria dos jogos +0xc4, &+0xc5 (os critérios de desempate), &{1,0,0,0} (uma vaga de cima: o campeão do
///    turno), o ano +0x40 (word), k, o id 0x439 (primeiro turno) ou 0x43a (segundo), 0, 1, 0, 8, `vinte`, 0, 2; a
///    original empurra os words e bytes com lixo nos bytes altos, que o construtor não lê: a cópia passa os valores
///    limpos; o k vai inteiro, como chegou). Sem memória para a fase: 0 na vaga, sem caixa.
/// 3) [+0xc][k] = a fase, +0x30 = k (o byte com sinal) e o vetor liberado (free 0x9452ca). O quadro SEH da original só
///    cobre o construtor (sem destrutor no caminho normal).
unsafe fn create_half_season_stage(this: usize, k: u32, arquivo: usize, linha: u32, vinte: u32, pv: usize, ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, u32, u32, u32, u32, u32, usize, *const u8, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(0x68b100usize);
    let vagas: [u8; 4] = [1, 0, 0, 0];
    let n = cqr1_w(this + 0x3e);
    let clubes = alocar(((n as i32) << 2) as u32);
    if clubes == 0 {
        return show_error_at(arquivo, linha);
    }
    let mut i: i16 = 0;
    while i < cqr1_w(this + 0x3e) {
        core::ptr::write_unaligned((clubes + 4 * i as usize) as *mut usize, cqr1_d(cqr1_d(this + 0xb1) + i as usize * 0x3b));
        i += 1;
    }
    let id = 0x439 + (k as u8 != 0) as u32;
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(p, this, cqr1_w(this + 0x3e) as u16 as u32, clubes, 1, cqr1_d(this + 4), 0, 0,
                  cqr1_b(this + pv) as u32,
                  cqr1_b(this + 0xc3) as u32, cqr1_b(this + 0xc4) as u32, this + 0xc5, vagas.as_ptr(),
                  cqr1_w(this + 0x40) as u16 as u32, k, id, 0, 1, 0, 8, vinte, 0, 2)
    } else {
        0
    };
    let k = k as u8 as i8 as i32;
    core::ptr::write_unaligned((cqr1_d(this + 0xc) + 4 * k as usize) as *mut usize, fase);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, k);
    crate::comparacao::ramo(ramo);
    soltar(clubes);
}

/// O comparador de um turno: só a original (ponytail: aloca o vetor e a fase, que monta a tabela do turno), com o plano:
/// depois, a vaga k tem a fase (o operator new não falha na carreira), +0x30 = k e a fase tem os +0x3e clubes da liga.
unsafe fn compare_half_season_stage(nome: &str, this: usize, k: u32, original: impl FnOnce()) {
    original();
    let kk = k as u8 as i8 as i32;
    let fase = cqr1_d(cqr1_d(this + 0xc) + 4 * kk as usize);
    let igual = fase != 0 && cqr1_d(this + 0x30) as i32 == kk && cqr1_w(fase + 0x3e) == cqr1_w(this + 0x3e);
    PLACAR.registrar(igual, || format!("{}({:#x}, {}) fase={:#x} +0x30={}", nome, this, kk, fase, cqr1_d(this + 0x30) as i32));
}

/// { 0x43afa0 } (thiscall, ret 4, bra_reg_rio.cpp): `BraRegRio::create_half_season_stage(k)`, um turno do Carioca (os
/// doze clubes; a caixa da linha 0x3d5).
unsafe extern "thiscall" fn bra_reg_rio_create_half_season_stage(this: usize, k: u32) {
    create_half_season_stage(this, k, 0x98b014, 0x3d5, 0xb, 0xc2, "cqr_1 turno do carioca") // cqr1: os pontos por vitória no Carioca
}

/// { 0x4351b0 } (thiscall, ret 4, bra_reg_minas_gerais.cpp): `BraRegMinasGerais::create_half_season_stage(k)`, um turno
/// do Mineiro (a caixa da linha 0x3d5).
unsafe extern "thiscall" fn bra_reg_minas_gerais_create_half_season_stage(this: usize, k: u32) {
    create_half_season_stage(this, k, 0x98aed0, 0x3d5, 0xb, 0xc2, "cqr_1 turno do mineiro") // cqr1: os pontos por vitória no Mineiro
}

/// { 0x438760 } (thiscall, ret 4, bra_reg_parana.cpp): `BraRegParana::create_half_stage(k)`, um turno do Paranaense (a
/// caixa da linha 0x3d5).
unsafe extern "thiscall" fn create_half_stage(this: usize, k: u32) {
    create_half_season_stage(this, k, 0x98af98, 0x3d5, 0xb, 0xc2, "cqr_1 turno do paranaense") // cqr1: os pontos por vitória no Paranaense
}

/// { 0x663c20 } (thiscall, ret 4, jap_j1.cpp): `JapJ1::create_half_season_stage(k)`, um turno da J1 (a caixa da linha
/// 0x278; o 20º argumento do construtor é 0xf).
unsafe extern "thiscall" fn jap_j1_create_half_season_stage(this: usize, k: u32) {
    create_half_season_stage(this, k, 0x9eabc8, 0x278, 0xf, 0xc2, "cqr_1 turno da j1") // cqr1: os pontos por vitória na J1
}

unsafe extern "thiscall" fn comparar_bra_reg_rio_create_half_season_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%);
    compare_half_season_stage("bra_reg_rio_create_half_season_stage", this, k, || o(this, k))
}

unsafe extern "thiscall" fn comparar_bra_reg_minas_gerais_create_half_season_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%);
    compare_half_season_stage("bra_reg_minas_gerais_create_half_season_stage", this, k, || o(this, k))
}

unsafe extern "thiscall" fn comparar_create_half_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I2%%);
    compare_half_season_stage("create_half_stage", this, k, || o(this, k))
}

unsafe extern "thiscall" fn comparar_jap_j1_create_half_season_stage(this: usize, k: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I3%%);
    compare_half_season_stage("jap_j1_create_half_season_stage", this, k, || o(this, k))
}
