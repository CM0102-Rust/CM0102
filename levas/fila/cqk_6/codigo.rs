// ---------- %%LEVA%%a leva: as tabelas dos grupos da CFA e da divisão de baixo francesa (a marca dos clubes de divisão maior) e a situação nas estatísticas da Série C1 italiana (o play-off e o play-out em faixas) (fra_cfa.cpp, fra_lower.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------

unsafe fn cqk6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqk6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O corpo das duas tabelas francesas: +0x3e = `n`, a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do
/// fonte `arquivo` na linha `linha` e 0) e os `n` clubes da lista `clubes`, na ordem, cada um na linha seguinte
/// (League::init_table_row 0x687430: thiscall(liga, linha, clube, marca, 0), ret 0x10), com a marca 1 nos clubes
/// cuja competição (+0x57, o id no +0) é uma de `marcadas`. Devolve se encheu (a conta das linhas = +0x3e).
unsafe fn create_french_table(this: usize, clubes: usize, n: i16, arquivo: usize, linha: u32, marcadas: &[usize]) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        show_error_at(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    while i < cqk6_w(this + 0x3e) {
        let clube = cqk6_d(clubes + (i as usize) * 4);
        let id = cqk6_d(cqk6_d(clube + 0x57));
        let marca = marcadas.iter().any(|&g| cqk6_d(g) == id) as u32; // cqk6: a marca dos clubes de divisão maior
        // ponytail: a original empurra a linha no ecx com os bits de cima do id da competição; a 0x687430 só lê o
        // byte.
        let linha = k; // cqk6: a linha k recebe o k-ésimo clube da lista
        iniciar(this, linha as u32, clube, marca, 0);
        k = k.wrapping_add(1);
        i = i.wrapping_add(1);
    }
    crate::comparacao::ramo("cqk_6 tabela francesa de baixo");
    (k as i8 as i16 == cqk6_w(this + 0x3e)) as u32
}

/// { 0x5b9fa0 } (thiscall, ret 4, fra_cfa.cpp): `FraCfa::create_group_table(clubes)`: um grupo da CFA com 18 clubes
/// da lista (o construtor 0x5b91c0 sorteia os clubes dos 4 grupos), marcando os que estão na primeira ([0x9cf5cc]) ou
/// na segunda ([0x9cf5d0]) divisão [inferência: os times reserva dos profissionais, que não sobem]. Fonte 0x9ccaf0,
/// linha 0x185.
unsafe extern "thiscall" fn fra_cfa_create_group_table(this: usize, clubes: usize) -> u32 {
    create_french_table(this, clubes, 0x12, 0x9ccaf0, 0x185, &[0x9cf5cc, 0x9cf5d0])
}

/// { 0x5be230 } (thiscall, ret 4, fra_lower.cpp): `FraLower::create_table(clubes)`: a divisão de baixo francesa com
/// 20 clubes da lista (o construtor 0x5bd600 escolhe-os), marcando os que estão no National ([0x9cf5d4]). Fonte
/// 0x9ccbd0, linha 0x175.
unsafe extern "thiscall" fn fra_cfa_create_table(this: usize, clubes: usize) -> u32 {
    create_french_table(this, clubes, 0x14, 0x9ccbd0, 0x175, &[0x9cf5d4])
}

/// O comparador das duas: a original aloca a tabela: só ela. O plano, feito antes: os `n` clubes da lista e a marca de
/// cada um. Depois: +0x3e = `n`, a devolução 1 e as linhas com os clubes na ordem e a marca (+0x38).
unsafe fn compare_french_table(nome: &str, this: usize, clubes: usize, n: i16, marcadas: &[usize], original: impl FnOnce() -> u32) -> u32 {
    let plano: Vec<(usize, u8)> = (0..n as usize)
        .map(|i| {
            let c = cqk6_d(clubes + i * 4);
            let id = cqk6_d(cqk6_d(c + 0x57));
            (c, marcadas.iter().any(|&g| cqk6_d(g) == id) as u8)
        })
        .collect();
    let r = original();
    let t = cqk6_d(this + 0xb1);
    let mut achou: Vec<(usize, u8)> = Vec::new();
    let mut igual = r == 1 && cqk6_w(this + 0x3e) == n && t != 0;
    if igual {
        achou = (0..n as usize).map(|k| (cqk6_d(t + k * 0x3b), *((t + k * 0x3b + 0x38) as *const u8))).collect();
        igual = achou == plano;
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_fra_cfa_create_group_table(this: usize, clubes: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I0%%);
    compare_french_table("fra_cfa_create_group_table", this, clubes, 0x12, &[0x9cf5cc, 0x9cf5d0], || o(this, clubes))
}

unsafe extern "thiscall" fn comparar_fra_cfa_create_table(this: usize, clubes: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I1%%);
    compare_french_table("fra_cfa_create_table", this, clubes, 0x14, &[0x9cf5d4], || o(this, clubes))
}

/// A situação do clube nas estatísticas da Série C1 (o corpo de 0x64bd80 e 0x64ff40): na entrada do clube
/// (Comp_Stats::find_club_entry 0x4a4850: thiscall(+8, clube), ret 4) grava a posição atual (+0x73), a melhor (+0x74)
/// e a pior (+0x75). Na liga (fase 0xff): a melhor de 2 a 5 (o play-off de acesso) vira 2 e de 14 a 17 (o play-out)
/// vira 14; a pior de 2 a 5 vira 5 e de 14 a 17 vira 17. No play-off (fase 0): as três + 1. No play-out (fase 1): a
/// melhor 1 vira 14 e 2 vira 16. Noutra fase, como vieram.
unsafe fn set_c1_club_progress(this: usize, clube: usize, fase: u8, atual: u8, melhor: u8, pior: u8) {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = achar(cqk6_d(this + 8), clube);
    let (mut a, mut m, mut p) = (atual, melhor, pior);
    if fase == 0xff {
        m = match m {
            2..=5 => 2,
            14..=17 => {
                crate::comparacao::ramo("cqk_6 band do play-out");
                0xe // cqk6: a melhor na faixa do play-out
            }
            v => v,
        };
        p = match p {
            2..=5 => 5,
            14..=17 => 0x11,
            v => v,
        };
    } else if fase == 0 {
        m = m.wrapping_add(1);
        p = p.wrapping_add(1);
        a = a.wrapping_add(1);
    } else if fase == 1 {
        m = match m {
            1 => 0xe,
            2 => 0x10,
            v => v,
        };
    }
    *((e + 0x73) as *mut u8) = a;
    *((e + 0x74) as *mut u8) = m;
    *((e + 0x75) as *mut u8) = p;
}

/// { 0x64bd80 } (thiscall, ret 0x14, ita_ser_c1a.cpp): `ItaSerC1a::set_club_progress(clube, fase, atual, melhor,
/// pior)`, a virtual +0x68 da 0x684cd0 a cada linha da tabela.
unsafe extern "thiscall" fn ita_ser_c1a_set_club_progress(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    set_c1_club_progress(this, clube, fase as u8, atual as u8, melhor as u8, pior as u8)
}

/// { 0x64ff40 } (thiscall, ret 0x14, ita_ser_c1b.cpp): `ItaSerC1b::set_club_progress(...)`, igual à da C1/A.
unsafe extern "thiscall" fn fra_cfa_set_club_progress(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    set_c1_club_progress(this, clube, fase as u8, atual as u8, melhor as u8, pior as u8)
}

/// O comparador das duas: a entrada é achada antes (a original e a cópia acham a mesma) e as duas rodam lado a lado
/// nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
unsafe fn compare_c1_club_progress(nome: &str, this: usize, clube: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = achar(cqk6_d(this + 8), clube);
    if e == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x73, 3)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}, {:#x}) original={:x?} rust={:x?}", nome, this, clube, mo, mr));
}

unsafe extern "thiscall" fn comparar_ita_ser_c1a_set_club_progress(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    compare_c1_club_progress("ita_ser_c1a_set_club_progress", this, clube, || o(this, clube, fase, atual, melhor, pior), || ita_ser_c1a_set_club_progress(this, clube, fase, atual, melhor, pior))
}

unsafe extern "thiscall" fn comparar_fra_cfa_set_club_progress(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I3%%);
    compare_c1_club_progress("fra_cfa_set_club_progress", this, clube, || o(this, clube, fase, atual, melhor, pior), || fra_cfa_set_club_progress(this, clube, fase, atual, melhor, pior))
}
