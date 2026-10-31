// ---------- %%LEVA%%a leva: a temporada do clube e a nação principal: se o clube está na temporada e as datas dela, o continente principal e a nação principal (key_nation.cpp) ----------

unsafe fn cpu2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpu2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// Grava em `dst` (se não nulo) a data (dia, ano) montada pela 0x549ef0 (com o bissexto), como a cópia 0x417c10.
unsafe fn cpu2_data(dst: usize, dia: i16, ano: u16) {
    if dst == 0 {
        return;
    }
    let data: unsafe extern "thiscall" fn(*mut u8, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mut b = [0u8; 8];
    data(b.as_mut_ptr(), dia as u16 as u32, ano as u32);
    core::ptr::copy_nonoverlapping(b.as_ptr(), dst as *mut u8, 8);
}

/// { 0x669bd0 } (cdecl, key_nation.cpp, quadro SEH só das datas locais): `key_nation_club_in_season(clube, &início,
/// &fim)`. A linha da nação do clube (+0x53) na tabela das 35 nações-chave (0xb63d60, 72 bytes; sem nação ou fora da
/// tabela, a linha 3 0xb63e38 e o dia de fim da data padrão 0x54a110, 40). `a` = o dia +0x2a da linha (início), `c` = o
/// dia da data +0x39 (fim; a original também copia os 7 bytes seguintes e, com o clube +0x1db ≠ −1 e o byte +0x43 3, 4
/// ou 5, troca dois deles, mas não os usa), `t` o dia de hoje ([0xae2c90]) e `y` o ano ([0xae2c92]); comparações de 16
/// bits com sinal. Com a > c (temporada que vira o ano): t > a, (a, y) e (c, y+1), 1; t < c, (a, y−1) e (c, y), 1; senão
/// (a, y) e (c, y), 0. Com a ≤ c: t < a, (a, y) e (c, y), 0; t > c, (a, y+1) e (c, y+1), 1; senão (a, y) e (c, y), 1. As
/// datas vão para `início` e `fim` quando não nulos. Pelo clube na temporada 0x547770, pela forma física 0x62a920,
/// 0x62c680 e 0x62ce30, pelo mês do fim da temporada 0x4d45f0 e pela data da transferência 0x8cee70. Chamada a cada
/// jogador: sem contador.
unsafe extern "C" fn clube_na_temporada(clube: usize, inicio: usize, fim: usize) -> u32 {
    let mut linha = 0xb63e38usize;
    let mut c: i16 = 0x28;
    let nacao = cpu2_d(clube + 0x53);
    if nacao != 0 {
        if let Some(k) = (0..0x23usize).find(|&k| cpu2_d(0xb63d60 + 72 * k) == nacao) {
            linha = 0xb63d60 + 72 * k;
            c = cpu2_w(linha + 0x39);
        }
    }
    let a = cpu2_w(linha + 0x2a);
    let t = *(0xae2c90 as *const i16);
    let y = *(0xae2c92 as *const u16);
    if a > c {
        if t > a {
            cpu2_data(inicio, a, y);
            if fim != 0 {
                cpu2_data(fim, c, y.wrapping_add(1)); // cpu2: o fim no ano seguinte
            }
            return 1;
        }
        if t < c {
            cpu2_data(inicio, a, y.wrapping_sub(1));
            cpu2_data(fim, c, y);
            return 1; // cpu2: antes do fim, na temporada
        }
        cpu2_data(inicio, a, y);
        cpu2_data(fim, c, y);
        return 0; // cpu2: entre o fim e o inicio, fora
    }
    if t < a {
        cpu2_data(inicio, a, y);
        cpu2_data(fim, c, y);
        return 0;
    }
    if t > c {
        cpu2_data(inicio, a, y.wrapping_add(1));
        cpu2_data(fim, c, y.wrapping_add(1));
        return 1;
    }
    cpu2_data(inicio, a, y);
    cpu2_data(fim, c, y);
    1 // cpu2: entre o inicio e o fim do ano
}

unsafe extern "C" fn comparar_clube_na_temporada(clube: usize, inicio: usize, fim: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize, usize, usize) -> u32>(%%I0%%)(clube, inicio, fim);
    let (mut i, mut f) = ([0u8; 8], [0u8; 8]);
    let n = clube_na_temporada(
        clube,
        if inicio != 0 { i.as_mut_ptr() as usize } else { 0 },
        if fim != 0 { f.as_mut_ptr() as usize } else { 0 },
    );
    let ler = |p: usize| if p != 0 { *(p as *const [u8; 8]) } else { [0u8; 8] };
    let (oi, of) = (ler(inicio), ler(fim));
    let ok = r == n && (inicio == 0 || oi == i) && (fim == 0 || of == f);
    PLACAR.registrar(ok, || {
        format!("clube_na_temporada(clube {:x}) original={} {:02x?} {:02x?} rust={} {:02x?} {:02x?}", clube, r, oi, of, n, i, f)
    });
    r
}

/// { 0x6695c0 } (cdecl, key_nation.cpp): `key_nation_get_main_continent()`: o continente (+0x71) da nação do humano
/// ([0xae2d08]); sem ela ou sem continente, o da primeira nação da tabela com o bit 2 de +0x11c, ou 0. Pelo menu do jogo.
unsafe extern "C" fn continente_principal() -> usize {
    let h = cpu2_d(0xae2d08);
    if h != 0 && cpu2_d(h + 0x71) != 0 {
        return cpu2_d(h + 0x71);
    }
    for k in 0..0x23usize {
        let n = cpu2_d(0xb63d60 + 72 * k);
        if n != 0 && *((n + 0x11c) as *const u8) & 2 != 0 {
            return cpu2_d(n + 0x71);
        }
    }
    0
}

unsafe extern "C" fn comparar_continente_principal() -> usize {
    let r = orig::<unsafe extern "C" fn() -> usize>(%%I1%%)();
    let n = continente_principal();
    PLACAR.registrar(r == n, || format!("continente_principal() original={:x} rust={:x}", r, n));
    r
}

/// { 0x669600 } (cdecl, key_nation.cpp): `key_nation_get_main_nation()`: a nação do humano ([0xae2d08]); sem ela, a
/// primeira nação da tabela com o bit 2 de +0x11c, ou 0. Pelo menu do jogo.
unsafe extern "C" fn nacao_principal() -> usize {
    let h = cpu2_d(0xae2d08);
    if h != 0 {
        return h;
    }
    for k in 0..0x23usize {
        let n = cpu2_d(0xb63d60 + 72 * k);
        if n != 0 && *((n + 0x11c) as *const u8) & 2 != 0 {
            return n;
        }
    }
    0
}

unsafe extern "C" fn comparar_nacao_principal() -> usize {
    let r = orig::<unsafe extern "C" fn() -> usize>(%%I2%%)();
    let n = nacao_principal();
    PLACAR.registrar(r == n, || format!("nacao_principal() original={:x} rust={:x}", r, n));
    r
}
