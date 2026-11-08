// ---------- %%LEVA%%a leva: a projeção da liga para a diretoria: as posições projetadas do clube e as duas ordens da projeção (league.cpp) ----------

unsafe fn cpr11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A entrada da projeção (0xe bytes, no bloco da 0x944cf1): +0 o clube, +4 os pontos por jogo (float), +8 os pontos,
/// +0xa os pontos projetados, +0xc os jogos.
unsafe fn projection_entry(buf: usize, i: i32) -> usize {
    buf.wrapping_add((i as isize * 0xe) as usize)
}

/// { 0x68afb0 } (cdecl, league.cpp): `compare_projection_by_reputation_desc(a, b)`: o qsort das entradas da projeção
/// pela reputação do clube (+0 → +0x80, palavra com sinal), a maior primeiro. Quente (qsort): sem contador.
unsafe extern "C" fn compare_projection_by_reputation_desc(a: usize, b: usize) -> i32 {
    let ra = cpr11_w(cpr11_d(a) + 0x80) as i32;
    let rb = cpr11_w(cpr11_d(b) + 0x80) as i32;
    rb - ra // o clube de maior reputação primeiro
}

/// { 0x68afd0 } (cdecl, league.cpp): `compare_projection_by_points_desc(a, b)`: o qsort das entradas da projeção pelos
/// pontos projetados (+0xa, palavra com sinal), o maior primeiro. Quente (qsort): sem contador.
unsafe extern "C" fn compare_projection_by_points_desc(a: usize, b: usize) -> i32 {
    cpr11_w(b + 0xa) as i32 - cpr11_w(a + 0xa) as i32 // os mais pontos primeiro
}

/// A posição (1..) do clube na projeção já ordenada (a primeira entrada dele; `None` sem ela).
unsafe fn projection_position(buf: usize, n: i16, clube: usize) -> Option<i32> {
    (0..n as i32).find(|&i| cpr11_d(projection_entry(buf, i)) == clube)
}

/// O clube com outra forma: os pontos por jogo (+4) vezes `fator` (o float guardado; o x87 segue com o de 53 bits) e
/// os pontos projetados de novo com ele.
unsafe fn apply_form_factor(this: usize, e: usize, fator: f64) -> i32 {
    let r = *((e + 4) as *const f32) as f64 * fator;
    *((e + 4) as *mut f32) = r as f32; // fst: guarda o float e segue com o de 53 bits
    let resta = cpr11_w(this + 0xa7) as i32 - cpr11_w(e + 0xc) as i32;
    *((e + 0xa) as *mut i16) = (match_pl_ftol(resta as f64 * r) as i16).wrapping_add(cpr11_w(e + 8));
    resta
}

/// { 0x68ac40 } (thiscall, ret 4, league.cpp; virtual +0x84 das ligas, chamada só por Comp_Stats 0x4ae300, que a
/// diretoria usa na confiança semanal 0x697c40 e no motivo da saída do técnico 0x69b350):
/// `League::update_projected_positions(clube)`: com o clube na tabela (+0xb1, 0x3b bytes, conta +0x3e: +0 o clube,
/// +0x11 os jogos, +0x19 os pontos), monta num bloco (0x944cf1) uma entrada de 0xe bytes por linha com os pontos por
/// jogo; no primeiro terço da temporada (jogos do clube × 3 < +0xa7), troca-os por uma escala pela reputação
/// (0,83 × +0xc2 do primeiro ao 0,23 × +0xc2 abaixo do último). Projeta os pontos no fim da temporada, ordena e acha a
/// posição do clube na forma normal, melhor (× 1,5) e pior (× 0,444); libera o bloco e entrega as três à virtual
/// +0x68 (`set_club_progress(clube, competição +0x44, normal, melhor, pior)`, que grava os bytes +0x73..+0x75 do
/// registro do clube no Comp_Stats). Devolve 1; fora da tabela, 0.
unsafe extern "thiscall" fn update_projected_positions(this: usize, clube: usize) -> u32 {
    let n = cpr11_w(this + 0x3e);
    let tabela = cpr11_d(this + 0xb1);
    let linha = match (0..n as i32).map(|i| tabela.wrapping_add((i as isize * 0x3b) as usize)).find(|&l| cpr11_d(l) == clube) {
        Some(l) => l,
        None => return 0,
    };
    if linha == 0 || n == 0 {
        return 0;
    }
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let buf = novo((n as i32 * 0xe) as u32);
    let mut i: i16 = 0;
    while i < cpr11_w(this + 0x3e) {
        let l = cpr11_d(this + 0xb1).wrapping_add((i as isize * 0x3b) as usize);
        let e = projection_entry(buf, i as i32);
        core::ptr::write_unaligned(e as *mut usize, cpr11_d(l));
        *((e + 8) as *mut i16) = cpr11_w(l + 0x19);
        let jogos = *((l + 0x11) as *const i8);
        *((e + 0xc) as *mut i16) = jogos as i16;
        *((e + 4) as *mut f32) = if jogos > 0 { (cpr11_w(l + 0x19) as f64 / jogos as f64) as f32 } else { 0.0 };
        i += 1;
    }
    // os três locais da original (dwords): o byte baixo de cada um vira a posição; os bytes altos são o que sobrou
    // neles (o float A e a conta das linhas na escala pela reputação, o que resta de jogos do clube no terceiro)
    let (mut normal, mut melhor, mut pior) = (0u32, 0u32, 0u32);
    if (*((linha + 0x11) as *const i8) as i32) * 3 < cpr11_w(this + 0xa7) as i32 {
        crate::comparison::branch("cpr_11 projecao pela injury_word_0c");
        let n = cpr11_w(this + 0x3e);
        qsort(buf, n as i32 as u32, 0xe, 0x68afb0);
        let c = *((this + 0xc2) as *const i8) as f64;
        let a = (c * read_f64_at(0x96d0e8)) as f32; // fstp no local: o primeiro arredonda para float
        normal = a.to_bits();
        let b = c * read_f64_at(0x96d0e0);
        if n > 0 {
            let d = a as f64 - b;
            let step_towards = d / n as f64;
            for k in 0..n as i32 {
                *((projection_entry(buf, k) + 4) as *mut f32) = (a as f64 - step_towards * k as f64) as f32; // do maior ao menor
            }
            melhor = n as i32 as u32;
        }
    }
    let mut i: i16 = 0;
    while i < cpr11_w(this + 0x3e) {
        let e = projection_entry(buf, i as i32);
        let resta = cpr11_w(this + 0xa7) as i32 - cpr11_w(e + 0xc) as i32;
        pior = resta as u32;
        *((e + 0xa) as *mut i16) = (match_pl_ftol(resta as f64 * *((e + 4) as *const f32) as f64) as i16).wrapping_add(cpr11_w(e + 8));
        i += 1;
    }
    qsort(buf, cpr11_w(this + 0x3e) as i32 as u32, 0xe, 0x68afd0);
    if let Some(k) = projection_position(buf, cpr11_w(this + 0x3e), clube) {
        normal = (normal & !0xff) | (k as u8).wrapping_add(1) as u32; // a posição na forma normal
        pior = apply_form_factor(this, projection_entry(buf, k), read_f64_at(0x968940)) as u32;
    }
    qsort(buf, cpr11_w(this + 0x3e) as i32 as u32, 0xe, 0x68afd0);
    if let Some(k) = projection_position(buf, cpr11_w(this + 0x3e), clube) {
        melhor = (melhor & !0xff) | (k as u8).wrapping_add(1) as u32;
        pior = apply_form_factor(this, projection_entry(buf, k), read_f64_at(0x96d0d8)) as u32;
    }
    qsort(buf, cpr11_w(this + 0x3e) as i32 as u32, 0xe, 0x68afd0);
    for k in 0..cpr11_w(this + 0x3e) as i32 {
        if cpr11_d(projection_entry(buf, k)) == clube {
            pior = (pior & !0xff) | (k as u8).wrapping_add(1) as u32; // a última (o clube está uma vez só)
        }
    }
    liberar(buf);
    crate::comparison::branch("cpr_11 projecao");
    // ponytail: os bytes altos que a original deixa nos três locais fora da escala pela reputação são sobra de pilha
    // (aqui, zero); as 69 versões da virtual +0x68 só leem o byte baixo de cada argumento (ou repassam os dwords a
    // outra que lê)
    let progresso: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) =
        core::mem::transmute(cpr11_d(cpr11_d(this) + 0x68));
    let competicao = (melhor & !0xff) | *((this + 0x44) as *const u8) as u32; // dl sobre o edx do segundo local
    progresso(this, clube, competicao, normal, melhor, pior);
    1
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn compare_compare_projection_by_reputation_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = compare_projection_by_reputation_desc(a, b);
    SCOREBOARD.record(r == n, || format!("compare_projection_by_reputation_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn compare_compare_projection_by_points_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = compare_projection_by_points_desc(a, b);
    SCOREBOARD.record(r == n, || format!("compare_projection_by_points_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: o bloco da projeção é alocado e liberado dentro (não fica nada dele); o efeito é a virtual +0x68, que
/// grava os bytes +0x73..+0x75 do registro do clube no Comp_Stats da liga (+8, achado pela 0x4a4850, que dentro da
/// 0x4ae300 devolve o registro provisório dela): lado a lado nesses 3 bytes e na devolução. As duas versões que repassam
/// a outra competição (0x48e380, 0x7ef100) gravam fora da região: aí só a devolução pesa.
unsafe extern "thiscall" fn compare_update_projected_positions(this: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    let stats = cpr11_d(this + 8);
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let reg = if stats != 0 { busca(stats, clube) } else { 0 };
    if reg == 0 {
        return o(this, clube); // ponytail: sem registro para olhar, só a original
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(reg + 0x73, 3)], || o(this, clube),
        || update_projected_positions(this, clube));
    SCOREBOARD.record(igual && r == n, || format!("update_projected_positions(liga {:x}, clube {:x}) original={} rust={} {:x?} x {:x?}",
        this, clube, r, n, mo, mr));
    r
}
