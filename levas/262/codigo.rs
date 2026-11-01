// ---------- 262a leva: os play-offs das divisões inglesas: o mata-mata da First, da Second e da Third (eng_first.cpp, eng_second.cpp, eng_third.cpp) ----------

unsafe fn cpz1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpz1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube da linha `k` da tabela da liga (+0xb1, linhas de 0x3b bytes; o clube no +0).
unsafe fn cpz1_clube(liga: usize, k: usize) -> usize {
    cpz1_d(cpz1_d(liga + 0xb1) + k * 0x3b)
}

/// O mata-mata das três divisões (o corpo de 0x573080, 0x576a70 e 0x5783c0, que só mudam nas linhas, no fonte e no
/// +0x3a da fase). A lista de `+0xbf` clubes (malloc 0x944e46 de 4 × o byte com sinal; sem memória: a caixa
/// `linha_sem_lista` e nada mais) leva os clubes das 4 `linhas` da tabela, nesta ordem (as chaves: 0 × 1 e 2 × 3).
/// A virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, a conta, a lista, as rodadas, +4, a tabela de rodadas, a palavra +0x40, 0, 1,
/// `a10` (ou o extra da virtual), 0x14, 0, 0, 0, 0) vai para a primeira vaga de fases ([+0xc]); a lista e a tabela de
/// rodadas são liberadas (free 0x9452ca). Sem fase: a caixa `linha_sem_fase`; senão Cup::set_up_round (0x51c800,
/// thiscall(fase, 0), ret 4). O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe fn cpz1_mata_mata(this: usize, linhas: [usize; 4], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, a10: Option<u32>, nome: &'static str) {
    let n = cpz1_b(this + 0xbf) as i8 as i32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar((n << 2) as u32);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cpz1_clube(this, l));
    }
    crate::comparacao::ramo(nome);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cpz1_d(cpz1_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra a conta, as rodadas e a palavra +0x40 em registradores inteiros com lixo nos 16 bits
    // altos; o construtor só lê as palavras ([esp+0x230], [esp+0x238], [esp+0x244] e [esp+0x250] com mov ax/dx).
    let fase = if p != 0 {
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(0x522e00usize);
        let a10 = a10.unwrap_or(extra as u16 as u32);
        construir(p, this, n as u16 as u32, lista, rodadas as u16 as u32, cpz1_d(this + 4), t, cpz1_w(this + 0x40) as u16 as u32, 0, 1, a10, 0x14, 0, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned(cpz1_d(this + 0xc) as *mut usize, fase);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(lista);
    liberar(t);
    let fase = cpz1_d(cpz1_d(this + 0xc));
    if fase == 0 {
        return aviso_em(arquivo, linha_sem_fase);
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
}

/// { 0x573080 } (thiscall, ret 0, eng_first.cpp): `EngFirst::create_playoff_stage()`, da fase seguinte da First
/// (0x572ff0, quando a fase passa de −1 a 0). As chaves: o 6º contra o 3º e o 5º contra o 4º (linhas 5, 2, 4 e 3);
/// +0x3a da fase = 0xa0. Fonte 0x9c4374, linhas 0x2f0 (sem lista) e 0x306 (sem fase).
unsafe extern "thiscall" fn mata_mata_da_first(this: usize) {
    cpz1_mata_mata(this, [5, 2, 4, 3], 0x9c4374, 0x2f0, 0x306, Some(0xa0), "cpz_1 mata-mata da first"); // cpz1: 6º × 3º e 5º × 4º da First
}

/// { 0x576a70 } (thiscall, ret 0, eng_second.cpp): `EngSecond::create_play_off_stage()` (de 0x5769e0). As chaves da
/// First (linhas 5, 2, 4 e 3); +0x3a da fase = o extra da virtual +0x3c. Fonte 0x9c43e8, linhas 0x2b6 e 0x2cc.
unsafe extern "thiscall" fn mata_mata_da_second(this: usize) {
    cpz1_mata_mata(this, [5, 2, 4, 3], 0x9c43e8, 0x2b6, 0x2cc, None, "cpz_1 mata-mata da second"); // cpz1: 6º × 3º e 5º × 4º da Second
}

/// { 0x5783c0 } (thiscall, ret 0, eng_third.cpp): `EngThird::create_play_off_stage()` (de 0x578330). As chaves: o 7º
/// contra o 4º e o 6º contra o 5º (linhas 6, 3, 5 e 4); +0x3a = o extra da virtual. Fonte 0x9c4424, linhas 0x2be e
/// 0x2d5.
unsafe extern "thiscall" fn mata_mata_da_third(this: usize) {
    cpz1_mata_mata(this, [6, 3, 5, 4], 0x9c4424, 0x2be, 0x2d5, None, "cpz_1 mata-mata da third"); // cpz1: 7º × 4º e 6º × 5º da Third
}

/// O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das 4 linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = a conta
/// (+0xbf), +4 = o +4 da liga e os 4 primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como
/// conjunto: o sorteio da rodada 0x51c800 pode reordenar as chaves).
unsafe fn cpz1_comparar(nome: &str, this: usize, linhas: [usize; 4], original: impl FnOnce()) {
    let mut plano: [usize; 4] = [0; 4];
    for (k, &l) in linhas.iter().enumerate() {
        plano[k] = cpz1_clube(this, l);
    }
    let n = cpz1_b(this + 0xbf) as i8 as i16;
    original();
    let fase = cpz1_d(cpz1_d(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = cpz1_d(fase + 0xa7);
    let mut achou: [usize; 4] = [0; 4];
    for (k, a) in achou.iter_mut().enumerate() {
        *a = cpz1_d(lista + 6 * k);
    }
    let (mut x, mut y) = (plano, achou);
    x.sort_unstable();
    y.sort_unstable();
    let igual = x == y && cpz1_w(fase + 0x3e) == n && cpz1_d(fase + 4) == cpz1_d(this + 4);
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={} n={}", nome, this, plano, fase, achou, cpz1_w(fase + 0x3e), n));
}

unsafe extern "thiscall" fn comparar_mata_mata_da_first(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1380);
    cpz1_comparar("mata_mata_da_first", this, [5, 2, 4, 3], || o(this))
}

unsafe extern "thiscall" fn comparar_mata_mata_da_second(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1381);
    cpz1_comparar("mata_mata_da_second", this, [5, 2, 4, 3], || o(this))
}

unsafe extern "thiscall" fn comparar_mata_mata_da_third(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1382);
    cpz1_comparar("mata_mata_da_third", this, [6, 3, 5, 4], || o(this))
}

// ---------- 262a leva: as divisões inglesas no fim da temporada: os registros pela situação na tabela e no mata-mata, e a situação do clube na First e na Third (eng_second.cpp, eng_first.cpp, eng_third.cpp) ----------

unsafe fn cpz2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz2_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de staff ([0xdd7f00]).
unsafe fn cpz2_registros() -> usize {
    cpz2_d(0xdd7f00)
}

/// { 0x576c50 } (thiscall, ret 0x18, eng_second.cpp): `EngSecond::update_staff_records_for_status(clube, situação,
/// fase, &saiu, _, _)`, a virtual +0x44 da First, da Second e da Third (anunciada pela 0x686670). Devolve 0.
/// Na liga (fase 0xff), pela situação: 5, o título (Staff_Records::add_league_champions_record 0x868c50,
/// thiscall(registros, clube, comp +4), ret 8); 0, o acesso (0x869480, thiscall(registros, clube, comp, 0x64), ret
/// 0xc); 1, a vaga no mata-mata (0x86bdd0, thiscall(registros, clube, comp, 0xa0, 0x82, 0x1e), ret 0x14); 3, o
/// rebaixamento (0x86a1c0, thiscall(registros, clube, comp), ret 8). No mata-mata (outra fase), na linha do clube da
/// tabela (+0xb1, 0x3b bytes, +0x3e linhas): 1 (venceu a final), o acesso com peso 0x32, +0x37 = 0 e saiu = 1; 0
/// (passou da rodada), a classificação (0xa0, 0x96, 0xf) e segue procurando; 0xff ou 2 (caiu), +0x37 = 0xfe.
unsafe extern "thiscall" fn registros_pela_situacao_inglesa(this: usize, clube: usize, situacao: u32, fase: u32, saiu: usize, _a: u32, _b: u32) -> u32 {
    let s = situacao as u8;
    let comp = cpz2_d(this + 4);
    if fase as u8 == 0xff {
        match s {
            5 => {
                let titulo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
                titulo(cpz2_registros(), clube, comp);
            }
            0 => {
                crate::comparacao::ramo("cpz_2 acesso na tabela");
                let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
                acesso(cpz2_registros(), clube, comp, 0x64); // cpz2: o acesso pela tabela pesa 0x64
            }
            1 => {
                crate::comparacao::ramo("cpz_2 vaga no mata-mata");
                let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
                vaga(cpz2_registros(), clube, comp, 0xa0, 0x82, 0x1e); // cpz2: a vaga no mata-mata pesa 0x1e
            }
            3 => {
                crate::comparacao::ramo("cpz_2 rebaixamento na tabela");
                let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
                cair(cpz2_registros(), clube, comp); // cpz2: o rebaixamento pela tabela
            }
            _ => {}
        }
        return 0;
    }
    let mut i: i32 = 0;
    while i < *((this + 0x3e) as *const i16) as i32 {
        let linha = cpz2_d(this + 0xb1) + (i as usize) * 0x3b;
        if cpz2_d(linha) == clube {
            match s {
                1 => {
                    crate::comparacao::ramo("cpz_2 venceu o mata-mata");
                    let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
                    acesso(cpz2_registros(), clube, comp, 0x32);
                    *((cpz2_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0;
                    *(saiu as *mut u8) = 1;
                    return 0;
                }
                0 => {
                    let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
                    vaga(cpz2_registros(), clube, comp, 0xa0, 0x96, 0xf);
                }
                0xff | 2 => {
                    *((cpz2_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                    return 0;
                }
                _ => {}
            }
        }
        i += 1;
    }
    0
}

/// Comparador da 0x576c50: os registros gravam o histórico no arquivo, sorteiam o moral e podem renovar contratos
/// (não rodam duas vezes): só a original. O plano, feito antes, no mata-mata: a primeira linha do clube decide o que a
/// rotina grava (+0x37 = 0 e saiu = 1 com 1; +0x37 = 0xfe com 0xff ou 2; nada com 0, que segue procurando outra linha
/// do mesmo clube); depois a linha e `saiu` têm de estar assim. Na liga a rotina não grava nada: a devolução.
unsafe extern "thiscall" fn comparar_registros_pela_situacao_inglesa(this: usize, clube: usize, situacao: u32, fase: u32, saiu: usize, a: u32, b: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32, u32) -> u32>(1383);
    let s = situacao as u8;
    let mut plano: Option<(usize, u8, bool)> = None;
    if fase as u8 != 0xff {
        let tab = cpz2_d(this + 0xb1);
        let n = *((this + 0x3e) as *const i16) as i32;
        let mut i: i32 = 0;
        while i < n {
            let linha = tab + (i as usize) * 0x3b;
            if cpz2_d(linha) == clube {
                match s {
                    1 => {
                        plano = Some((linha, 0, true));
                        break;
                    }
                    0xff | 2 => {
                        plano = Some((linha, 0xfe, false));
                        break;
                    }
                    _ => {}
                }
            }
            i += 1;
        }
    }
    let r = o(this, clube, situacao, fase, saiu, a, b);
    let igual = r == 0
        && match plano {
            Some((linha, v, marca)) => cpz2_b(linha + 0x37) == v && (!marca || cpz2_b(saiu) == 1),
            None => true,
        };
    PLACAR.registrar(igual, || format!("registros_pela_situacao_inglesa({:#x}, {:#x}, {}, {}) original={} plano={:x?}", this, clube, s, fase as u8, r, plano));
    r
}

/// A entrada do clube nas estatísticas da competição (Comp_Stats::find_club_entry 0x4a4850: thiscall(+8, clube), ret 4;
/// põe a tabela na ordem dos clubes se precisa e procura).
unsafe fn cpz2_entrada(this: usize, clube: usize) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    achar(cpz2_d(this + 8), clube)
}

/// A situação do clube nas estatísticas (+0x73 a fase, +0x74 a melhor, +0x75 a pior), comum à First e à Third: na liga
/// (fase 0xff) a melhor e a pior de `baixo` a `alto` (as vagas do mata-mata) viram `baixo`, e com a pior nessa faixa a
/// pior vira `alto`; no mata-mata (fase 0), 1, 2 e 3 viram `baixo`, `baixo` + 1 e `baixo` + 2 (a melhor, a pior e a
/// fase); noutra fase, como vieram.
unsafe fn cpz2_situacao(this: usize, clube: usize, fase: u8, atual: u8, melhor: u8, pior: u8, baixo: u8, alto: u8) {
    let e = cpz2_entrada(this, clube);
    let (mut m, mut p, mut a) = (melhor, pior, atual);
    if fase == 0xff {
        if m >= baixo && m <= alto {
            m = baixo;
        }
        if p >= baixo && p <= alto {
            p = alto;
        }
    } else if fase == 0 {
        let mapa = |v: u8| match v {
            1 => baixo,
            2 => baixo + 1,
            3 => baixo + 2,
            _ => v,
        };
        m = mapa(m);
        p = mapa(p);
        a = mapa(a);
    }
    *((e + 0x73) as *mut u8) = a;
    *((e + 0x74) as *mut u8) = m;
    *((e + 0x75) as *mut u8) = p;
}

/// { 0x573260 } (thiscall, ret 0x14, eng_first.cpp): `EngFirst::set_club_progress(clube, fase, atual, melhor, pior)`,
/// a virtual +0x68 da First e da Second (da 0x684cd0 a cada linha da tabela): as vagas do mata-mata são 3 a 6.
unsafe extern "thiscall" fn situacao_na_first(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    cpz2_situacao(this, clube, fase as u8, atual as u8, melhor as u8, pior as u8, 3, 6)
}

/// { 0x578660 } (thiscall, ret 0x14, eng_third.cpp): `EngThird::set_club_progress(clube, fase, atual, melhor, pior)`,
/// a virtual +0x68 da Third: as vagas do mata-mata são 4 a 7.
unsafe extern "thiscall" fn situacao_na_third(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    cpz2_situacao(this, clube, fase as u8, atual as u8, melhor as u8, pior as u8, 4, 7)
}

/// O comparador das duas: a entrada é achada antes (a original e a cópia acham a mesma; a ordem dos clubes já fica
/// pronta) e as duas rodam lado a lado nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
unsafe fn cpz2_comparar(nome: &str, this: usize, clube: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let e = cpz2_entrada(this, clube);
    if e == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x73, 3)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}, {:#x}) original={:x?} rust={:x?}", nome, this, clube, mo, mr));
}

unsafe extern "thiscall" fn comparar_situacao_na_first(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(1384);
    cpz2_comparar("situacao_na_first", this, clube, || o(this, clube, fase, atual, melhor, pior), || situacao_na_first(this, clube, fase, atual, melhor, pior))
}

unsafe extern "thiscall" fn comparar_situacao_na_third(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(1385);
    cpz2_comparar("situacao_na_third", this, clube, || o(this, clube, fase, atual, melhor, pior), || situacao_na_third(this, clube, fase, atual, melhor, pior))
}

// ---------- 262a leva: a tabela da liga: as posições possíveis e a situação de cada clube (o título, o acesso, o mata-mata, o rebaixamento, a vaga europeia), e a fase seguinte da First e da Second (league.cpp, eng_first.cpp, eng_second.cpp) ----------

unsafe fn cpz3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpz3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpz3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpz3_s(a: usize) -> i8 {
    *(a as *const i8)
}

/// No ensaio (o comparador da 0x684cd0) a cópia não chama as virtuais +0x40 (o anúncio da situação, que manda notícia
/// e grava os registros) e +0x68 (a situação nas estatísticas) nem as cláusulas de rebaixamento (0x4cd030).
static mut CPZ3_ENSAIO: bool = false;

/// A linha `i` da tabela (+0xb1, 0x3b bytes).
unsafe fn cpz3_linha(liga: usize, i: i32) -> usize {
    cpz3_d(liga + 0xb1).wrapping_add((i as usize).wrapping_mul(0x3b))
}

/// A virtual +0x40 da liga (League::announce_team_status: thiscall(liga, clube, situação, fase, 0, 0), ret 0x14), com
/// o clube da linha `i`. `alto` = os bits de cima do registrador que a original empurra com a fase (+0x44) no byte de
/// baixo (o ponteiro da linha ou a devolução das cláusulas: reproduzidos).
unsafe fn cpz3_anunciar(liga: usize, i: i32, situacao: u32, alto: u32) {
    if CPZ3_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(cpz3_d(cpz3_d(liga) + 0x40));
    f(liga, cpz3_d(cpz3_linha(liga, i)), situacao, alto | cpz3_b(liga + 0x44) as u32, 0, 0);
}

/// Grava a situação `v` no +0x37 da linha `i` e a anuncia.
unsafe fn cpz3_situacao(liga: usize, i: i32, v: u8, alto: u32) {
    *((cpz3_linha(liga, i) + 0x37) as *mut u8) = v;
    cpz3_anunciar(liga, i, v as i8 as i32 as u32, alto);
}

/// O máximo de pontos da linha: (jogos da temporada +0xa7 − jogos feitos +0x11) × pontos da vitória (+0xc2) + pontos
/// (+0x19).
unsafe fn cpz3_maximo(liga: usize, l: usize) -> i32 {
    ((cpz3_w(liga + 0xa7) as i32) - (cpz3_s(l + 0x11) as i32)).wrapping_mul(cpz3_s(liga + 0xc2) as i32).wrapping_add(cpz3_w(l + 0x19) as i32)
}

/// { 0x684cd0 } (thiscall, ret 0, league.cpp): `League::update_positions_and_status()`, a virtual +0x94 da liga (do
/// passe do dia 0x684946). +0xa7 (os jogos da temporada) = (+0x3e − 1) × +0x3c (16 bits) se ainda −1, com linhas e
/// +0x45. Com +0xdd nada mais. Senão ordena a tabela (League::sort_table 0x6827d0, thiscall(liga, 0), ret 4) e monta
/// o mapa das faixas (malloc 0x944e46 de +0x3e bytes em 0xff; sem memória: a caixa da linha 0x7c9): de cima, as
/// contas +0xbe (faixa 0, acesso) e +0xbf (faixa 1, mata-mata) nas linhas que não estão em 0xfe, 0xfc ou 3; de baixo,
/// +0xc1 (faixa 3, rebaixamento) e +0xc0 (faixa 2) nas que não estão em 0, 1 ou 5. Com +0xd9 bit 0, o primeiro da
/// tabela fora de 0xfe, 0xfc e 3. A melhor posição (+0x39) e a pior (+0x3a) de cada linha que ainda não estão
/// decididas, pelos máximos de pontos e pelos jogos feitos (com +0xe6 no empate dos que terminaram). Depois, linha a
/// linha: a pior = o primeiro + 1, o título (5); faixa 0 na pior, o acesso (0 com +0xd9 bit 1; 7 com bit 2; 6 com bit
/// 3) em quem está em 0xff ou 1; faixa 1, o mata-mata (1) em quem está em 0xff; faixa 3 na melhor, as cláusulas de
/// rebaixamento (CONTRACT_MANAGER::trigger_relegation_clauses 0x4cd030, thiscall([0xae1050], clube, +4), ret 8) e o
/// rebaixamento (3) em quem está em 0xff, 2 ou 0xfc; faixa 2 na melhor, 2 em quem está em 0xff ou 0xfe. Cada situação
/// nova é anunciada (+0x40). Com +0xd9 bit 0x20, quem fica entre as faixas sem faixa no meio e está em 0xff vira 0xfe
/// (anunciado com −2). Melhor = pior abaixo de 9, liga de cima (+0x44 = 0xff) que conta (0x4b0700, cdecl(id)), sem
/// +0x1c e clube sem vaga europeia ([0xae2d14] + 6 × id = −1) de nação europeia ([0x9cfa10]) fora de [0x9cf45c]: as
/// vagas da nação (UefaSeeding::get_nation_places 0x905c60, thiscall([0xde1f58], nação, &cl, &uefa), ret 0xc) dão a
/// Liga dos Campeões ([0x9cf6e8]) ou a Copa da UEFA ([0x9cf6f0]). Por fim a virtual +0x68 (a situação nas
/// estatísticas: clube, +0x44, posição, melhor, pior) e o free (0x9452ca) do mapa.
unsafe extern "thiscall" fn posicoes_e_situacao_na_liga(this: usize) {
    let n0 = cpz3_w(this + 0x3e);
    if cpz3_w(this + 0xa7) == -1 && n0 > 0 && cpz3_d(this + 0x45) != 0 {
        *((this + 0xa7) as *mut i16) = (n0 - 1).wrapping_mul(cpz3_w(this + 0x3c));
    }
    if cpz3_d(this + 0xdd) != 0 {
        return;
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    ordenar(this, 0);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let mapa = alocar(cpz3_w(this + 0x3e) as i32 as u32);
    if mapa == 0 {
        return aviso_em(0x9f15f4, 0x7c9);
    }
    let n = cpz3_w(this + 0x3e);
    core::ptr::write_bytes(mapa as *mut u8, 0xff, n as i32 as usize);
    let m = |i: i32| (mapa as isize + i as isize) as *mut u8;
    let mut contas = [cpz3_b(this + 0xbe), cpz3_b(this + 0xbf), cpz3_b(this + 0xc0), cpz3_b(this + 0xc1)];
    let mut primeiro: i8 = -1;
    if cpz3_b(this + 0xd9) & 1 != 0 {
        let mut i: i16 = 0;
        while i < n {
            let s = cpz3_b(cpz3_linha(this, i as i32) + 0x37);
            if s != 0xfe && s != 0xfc && s != 3 {
                primeiro = i as i8;
                break;
            }
            i += 1;
        }
    }
    // de cima: as faixas 0 e 1
    let mut i: i32 = 0;
    if n > 0 {
        let mut k: i8 = 0;
        while k <= 1 {
            if contas[k as usize] != 0 {
                while (i as i16) < n {
                    let s = cpz3_b(cpz3_linha(this, i as i16 as i32) + 0x37);
                    if s != 0xfe && s != 0xfc && s != 3 {
                        *m(i as i16 as i32) = k as u8;
                        contas[k as usize] = contas[k as usize].wrapping_sub(1);
                    }
                    i += 1;
                    if contas[k as usize] == 0 {
                        break;
                    }
                }
            }
            k += 1;
            if (i as i16) >= n {
                break;
            }
        }
    }
    // de baixo: as faixas 3 e 2
    let mut i: i32 = (n as i32) - 1;
    let mut k: i8 = 3;
    while (i as i16) >= 0 && k >= 2 {
        if contas[k as usize] != 0 {
            while (i as i16) >= 0 {
                let s = cpz3_b(cpz3_linha(this, i as i16 as i32) + 0x37);
                if s != 0 && s != 1 && s != 5 {
                    *m(i as i16 as i32) = k as u8;
                    contas[k as usize] = contas[k as usize].wrapping_sub(1);
                }
                i -= 1;
                if contas[k as usize] == 0 {
                    break;
                }
            }
        }
        k -= 1;
    }
    // a melhor (+0x39) e a pior (+0x3a) posição possível
    let mut i: i32 = 0;
    while (i as i16) < cpz3_w(this + 0x3e) {
        let li = cpz3_linha(this, i as i16 as i32);
        if cpz3_b(li + 0x39) != cpz3_b(li + 0x3a) {
            let pos = (i as u8).wrapping_add(1);
            *((li + 0x39) as *mut u8) = pos;
            // os de cima (j < i): a melhor cai um para cada um que o máximo de i alcança
            let max_i = cpz3_maximo(this, li);
            let mut j: i32 = 0;
            while (j as i16) < (i as i16) {
                let lj = cpz3_linha(this, j);
                let jogos = cpz3_w(this + 0xa7);
                let fim_i = (cpz3_s(li + 0x11) as i16) >= jogos;
                let fim_j = (cpz3_s(lj + 0x11) as i16) >= jogos;
                let pj = cpz3_w(lj + 0x19) as i32;
                let cai = if !fim_i && !fim_j {
                    max_i >= pj
                } else if !fim_j {
                    max_i == pj
                } else if !fim_i {
                    max_i >= pj
                } else {
                    max_i == pj && cpz3_d(this + 0xe6) != 0
                };
                if cai {
                    *((li + 0x39) as *mut u8) = cpz3_b(li + 0x39).wrapping_sub(1);
                }
                j += 1;
            }
            // os de baixo (j > i): a pior sobe um para cada um cujo máximo alcança i
            *((li + 0x3a) as *mut u8) = pos;
            let mut j: i32 = pos as i8 as i32;
            while (j as i16) < cpz3_w(this + 0x3e) {
                let lj = cpz3_linha(this, j as i16 as i32);
                let jogos = cpz3_w(this + 0xa7);
                let pj = cpz3_w(lj + 0x19) as i32;
                let max_j = cpz3_maximo(this, lj);
                let fim_i = (cpz3_s(li + 0x11) as i16) >= jogos;
                let fim_j = (cpz3_s(lj + 0x11) as i16) >= jogos;
                let pi = cpz3_w(li + 0x19) as i32;
                let sobe = if !fim_i && !fim_j {
                    max_j >= pi
                } else if !fim_j {
                    max_j >= pi
                } else if !fim_i {
                    max_j == pi
                } else {
                    max_j == pj && cpz3_d(this + 0xe6) != 0
                };
                if sobe {
                    *((li + 0x3a) as *mut u8) = cpz3_b(li + 0x3a).wrapping_add(1);
                }
                j += 1;
            }
        }
        i += 1;
    }
    // a situação de cada linha
    let um_depois = (primeiro as i32) + 1;
    let mut i: i32 = 0;
    while (i as i16) < cpz3_w(this + 0x3e) {
        let ii = i as i16 as i32;
        let li = cpz3_linha(this, ii);
        let alto_linha = (li as u32) & !0xff;
        let pior = cpz3_s(li + 0x3a) as i32;
        let s = cpz3_b(li + 0x37);
        if pior == um_depois {
            if s != 3 && s != 0xfe && s != 0xfc && s != 5 {
                crate::comparacao::ramo("cpz_3 titulo");
                cpz3_situacao(this, ii, 5, alto_linha); // cpz3: o título na pior posição = o primeiro + 1
            }
        } else {
            let f = *m(pior - 1);
            if f == 0 {
                if s == 0xff || s == 1 {
                    let d9 = cpz3_b(this + 0xd9);
                    if d9 & 2 != 0 {
                        if s != 0 {
                            cpz3_situacao(this, ii, 0, alto_linha);
                        }
                    } else if d9 & 4 != 0 {
                        if s != 7 {
                            cpz3_situacao(this, ii, 7, alto_linha);
                        }
                    } else if d9 & 8 != 0 {
                        if s != 6 {
                            cpz3_situacao(this, ii, 6, alto_linha);
                        }
                    }
                }
            } else if f == 1 {
                if s == 0xff {
                    crate::comparacao::ramo("cpz_3 vaga no mata-mata");
                    cpz3_situacao(this, ii, 1, alto_linha); // cpz3: a vaga no mata-mata (situação 1)
                }
            } else {
                let g = *m(cpz3_s(li + 0x39) as i32 - 1);
                if g == 3 {
                    let mut alto = alto_linha;
                    if !CPZ3_ENSAIO {
                        crate::comparacao::ramo("cpz_3 clausulas de rebaixamento");
                        let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x4cd030usize);
                        alto = clausulas(cpz3_d(0xae1050), cpz3_d(li), cpz3_d(this + 4)) & !0xff; // cpz3: as cláusulas de rebaixamento
                    }
                    let li = cpz3_linha(this, ii);
                    let s = cpz3_b(li + 0x37);
                    if s == 0xff || s == 2 || s == 0xfc {
                        cpz3_situacao(this, ii, 3, alto);
                    }
                } else if g == 2 {
                    if s == 0xff || s == 0xfe {
                        cpz3_situacao(this, ii, 2, alto_linha);
                    }
                }
            }
        }
        // entre as faixas, sem faixa no meio: 0xfe
        if cpz3_b(this + 0xd9) & 0x20 != 0 {
            let li = cpz3_linha(this, ii);
            let c0 = cpz3_s(this + 0xbe) as i32;
            let c2 = cpz3_s(this + 0xc0) as i32;
            let melhor = cpz3_s(li + 0x39);
            if (melhor as i32) > c0 + c2 {
                let c3 = cpz3_s(this + 0xc1) as i32;
                let pior = cpz3_s(li + 0x3a);
                if (pior as i32) <= (cpz3_w(this + 0x3e) as i32) - c3 - c2 {
                    let mut livre = true;
                    let mut a: i16 = (melhor as i16) - 1;
                    while a < pior as i16 {
                        if !livre {
                            break;
                        }
                        if *m(a as i32) != 0xff {
                            livre = false;
                        }
                        a += 1;
                    }
                    if livre && cpz3_b(li + 0x37) == 0xff {
                        *((li + 0x37) as *mut u8) = 0xfe;
                        cpz3_anunciar(this, ii, 0xffff_fffe, (li as u32) & 0xffff_0000);
                    }
                }
            }
        }
        // a vaga europeia de quem já tem a posição decidida
        let li = cpz3_linha(this, ii);
        let melhor = cpz3_s(li + 0x39);
        if melhor == cpz3_s(li + 0x3a) && melhor < 9 && cpz3_b(this + 0x44) == 0xff {
            let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
            if conta(cpz3_i(cpz3_d(this + 4))) != 0 && cpz3_i(this + 0x1c) == -1 {
                let vaga = cpz3_d(0xae2d14) + (cpz3_i(cpz3_d(li)) as usize).wrapping_mul(6);
                let reg = cpz3_d(this + 4);
                let nacao = cpz3_d(reg + 0x5d);
                if cpz3_i(vaga) == -1 && nacao != 0 && cpz3_i(nacao) != cpz3_i(0x9cf45c) {
                    let cont = cpz3_d(nacao + 0x71);
                    if cont != 0 && cpz3_i(cont) == cpz3_i(0x9cfa10) {
                        let (mut cl, mut uefa) = (0i8, 0i8);
                        let vagas: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8) = core::mem::transmute(0x905c60usize);
                        vagas(cpz3_d(0xde1f58), nacao, &mut cl, &mut uefa);
                        let li = cpz3_linha(this, ii);
                        let melhor = cpz3_s(li + 0x39) as i32;
                        let vaga = cpz3_d(0xae2d14) + (cpz3_i(cpz3_d(li)) as usize).wrapping_mul(6);
                        if melhor <= cl as i32 {
                            core::ptr::write_unaligned(vaga as *mut i32, cpz3_i(0x9cf6e8));
                        } else if melhor <= (cl as i32) + (uefa as i32) - 1 {
                            core::ptr::write_unaligned(vaga as *mut i32, cpz3_i(0x9cf6f0));
                        }
                    }
                }
            }
        }
        // a situação nas estatísticas (virtual +0x68)
        if !CPZ3_ENSAIO {
            let li = cpz3_linha(this, ii);
            // ponytail: a original empurra a pior, a melhor, a posição e a fase em registradores com lixo nos bits de
            // cima; as +0x68 (0x573260, 0x578660, Comp::set_club_progress 0x48e380) só leem o byte.
            let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(cpz3_d(cpz3_d(this) + 0x68));
            f(this, cpz3_d(li), cpz3_b(this + 0x44) as u32, (i as u8).wrapping_add(1) as u32, cpz3_b(li + 0x39) as u32, cpz3_b(li + 0x3a) as u32);
        }
        i += 1;
    }
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(mapa);
}

/// As regiões que a 0x684cd0 grava: a tabela inteira (+0xb1, +0x3e linhas de 0x3b bytes), +0xa7 e a vaga europeia
/// ([0xae2d14] + 6 × id, 4 bytes) de cada clube da tabela.
unsafe fn cpz3_regioes(this: usize) -> Vec<(usize, usize)> {
    let n = cpz3_w(this + 0x3e).max(0) as usize;
    let mut v = vec![(this + 0xa7, 2)];
    let tab = cpz3_d(this + 0xb1);
    if n > 0 && tab != 0 {
        v.push((tab, n * 0x3b));
        let vagas = cpz3_d(0xae2d14);
        if vagas != 0 {
            for i in 0..n {
                let c = cpz3_d(tab + i * 0x3b);
                if c != 0 {
                    let a = vagas + (cpz3_i(c) as usize).wrapping_mul(6);
                    if !v.iter().any(|r| r.0 == a) {
                        v.push((a, 4));
                    }
                }
            }
        }
    }
    v
}

/// Comparador da 0x684cd0, em ensaio (modelo CPV6_ENSAIO): a cópia roda primeiro sem as virtuais +0x40 e +0x68 nem
/// as cláusulas (que mandam notícia, gravam registros e sorteiam o moral); a memória volta e a original roda inteira.
/// As duas a partir do mesmo estado da tabela, de +0xa7 e das vagas europeias dos clubes da tabela, que têm de sair
/// iguais (as virtuais e as cláusulas não gravam nelas). Sem linhas ou com +0xdd: só a original.
unsafe extern "thiscall" fn comparar_posicoes_e_situacao_na_liga(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1386);
    if cpz3_d(this + 0xdd) != 0 || cpz3_w(this + 0x3e) <= 0 || cpz3_d(this + 0xb1) == 0 {
        return o(this);
    }
    let regioes = cpz3_regioes(this);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPZ3_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| posicoes_e_situacao_na_liga(this));
    CPZ3_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("posicoes_e_situacao_na_liga({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// A fase seguinte das ligas com mata-mata (o corpo de 0x572ff0 e 0x5769e0): na última fase (+0x30 ≥ +0x2c − 1, com
/// sinal) a caixa da linha `linha`; senão +0x30 + 1 e, se chegou a 0, o mata-mata (`mata_mata`, thiscall(this)).
unsafe fn cpz3_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata_mata: usize, nome: &'static str) {
    let f = cpz3_i(this + 0x30);
    if f >= cpz3_i(this + 0x2c).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let f = f.wrapping_add(1);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, f);
    if f == 0 {
        crate::comparacao::ramo(nome);
        let criar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(mata_mata);
        criar(this);
    }
}

/// { 0x572ff0 } (thiscall, ret 0, eng_first.cpp): `EngFirst::advance_stage()` (a virtual +0x28): fonte 0x9c4374,
/// linha 0x2ce; o mata-mata 0x573080.
unsafe extern "thiscall" fn fase_seguinte_da_first(this: usize) {
    cpz3_fase_seguinte(this, 0x9c4374, 0x2ce, 0x573080, "cpz_3 mata-mata da first")
}

/// { 0x5769e0 } (thiscall, ret 0, eng_second.cpp): `EngSecond::advance_stage()` (a virtual +0x28): fonte 0x9c43e8,
/// linha 0x294; o mata-mata 0x576a70.
unsafe extern "thiscall" fn fase_seguinte_da_second(this: usize) {
    cpz3_fase_seguinte(this, 0x9c43e8, 0x294, 0x576a70, "cpz_3 mata-mata da second")
}

/// O comparador das duas: na fase que não chega a 0 (só +0x30 muda) a original e a cópia lado a lado em +0x30; na
/// última fase (a caixa) e na que chega a 0 (o mata-mata aloca a fase): só a original.
unsafe fn cpz3_comparar_fase(nome: &str, this: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let f = cpz3_i(this + 0x30);
    if f >= cpz3_i(this + 0x2c).wrapping_sub(1) || f.wrapping_add(1) == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x30, 4)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}) original={:x?} rust={:x?}", nome, this, mo, mr));
}

unsafe extern "thiscall" fn comparar_fase_seguinte_da_first(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1387);
    cpz3_comparar_fase("fase_seguinte_da_first", this, || o(this), || fase_seguinte_da_first(this))
}

unsafe extern "thiscall" fn comparar_fase_seguinte_da_second(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1388);
    cpz3_comparar_fase("fase_seguinte_da_second", this, || o(this), || fase_seguinte_da_second(this))
}

// ---------- 262a leva: a Liga dos Campeões: os registros pela fase alcançada, o sorteio das quartas e a lista dos clubes inscritos (european_cup.cpp) ----------

unsafe fn cpz4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpz4_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de staff ([0xdd7f00]).
unsafe fn cpz4_registros() -> usize {
    cpz4_d(0xdd7f00)
}

/// comp_util_scale_by_round (0x4bf850, cdecl(baixo, alto, rodada, rodadas)): a faixa pela rodada, de 0 a 100.
unsafe fn cpz4_escala(baixo: u32, alto: u32, rodada: i16, rodadas: u32) -> u32 {
    let f: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4bf850usize);
    // ponytail: a original empurra a rodada num registrador com lixo nos 16 bits altos; a 0x4bf850 só lê a palavra.
    f(baixo, alto, rodada as u16 as u32, rodadas)
}

/// Staff_Records::add_qualified_for_round_record (0x86bdd0: thiscall(registros, clube, comp, fase, rodada, peso),
/// ret 0x14).
unsafe fn cpz4_classificado(clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
    f(cpz4_registros(), clube, comp, fase, rodada, peso);
}

/// Staff_Records::add_failed_to_qualify_record (0x86c1d0: thiscall(registros, clube, comp, fase, _), ret 0x10).
unsafe fn cpz4_nao_classificado(clube: usize, comp: usize, fase: u32, x: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x86c1d0usize);
    f(cpz4_registros(), clube, comp, fase, x);
}

/// Staff_Records::add_knocked_out_record (0x86c000: thiscall(registros, clube, comp, fase, rodada, _), ret 0x14).
unsafe fn cpz4_eliminado(clube: usize, comp: usize, fase: u32, rodada: u32, x: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
    f(cpz4_registros(), clube, comp, fase, rodada, x);
}

/// O clube sai da competição europeia da temporada (+0x1db = −1).
unsafe fn cpz4_sair(clube: usize) {
    core::ptr::write_unaligned((clube + 0x1db) as *mut i32, -1);
}

/// { 0x584ae0 } (thiscall, ret 0x18, european_cup.cpp): `EuropeanCup::update_staff_records_for_status(clube,
/// desfecho, fase, _, jogo, final)`, a virtual +0x44 da Liga dos Campeões. Devolve 0. Comp = +4.
/// - Na qualificação (fase 0xff): desfecho 0 ou 1, a classificação pela rodada do jogo (+0x34): na rodada 0, com a
///   fase +0x3a e o +0x6f da primeira rodada (+0xa3) e o peso 0x4bf850(1, 0x14, 0, 2); noutra, 0x41c, 0 e
///   0x4bf850(1, 0x14, rodada, 2). Desfecho 0xff ou 2: a eliminação (0x86c000) com +0x32 e +0x30 do jogo e o peso
///   0x4bf850(1, 0x14, rodada, 2), e o clube sai (+0x1db = −1).
/// - Na fase 1 (fase < 8): 6, a classificação (0x44a, −1, 0x32); 0xfe, a não classificação (0x449, 0x32) e sai.
/// - Na fase 2 (8 a 11): 6, a classificação (0, 0x78, 0x32); 0xfe, a não classificação (0x44a, 0x32) e sai.
/// - No mata-mata (12): 1, o título (Staff_Records::add_cup_winners_record 0x86a800, thiscall(registros, clube, jogo,
///   final), ret 0xc) e sai; 2, o vice (0x86b0b0, idem) e sai; 0, a classificação (0, o +0x6f da rodada do jogo na
///   tabela de rodadas da fase do mata-mata ([+0xc] + 0x30, +0xa3, 0x68 bytes), 0x4bf850(0x4b, 0x64, rodada, 2));
///   0xff, a eliminação (0, +0x30 do jogo, 0x4bf850(0x4b, 0x64, rodada, 2)) e sai.
unsafe extern "thiscall" fn registros_da_liga_dos_campeoes(this: usize, clube: usize, desfecho: u32, fase: u32, _a: u32, jogo: usize, fim: u32) -> u32 {
    let d = desfecho as u8;
    let f = fase as u8;
    let comp = cpz4_d(this + 4);
    if f == 0xff {
        match d {
            0 | 1 => {
                let r = cpz4_w(jogo + 0x34);
                if r == 0 {
                    let peso = cpz4_escala(1, 0x14, 0, 2);
                    let rodada = cpz4_w(cpz4_d(this + 0xa3) + 0x6f) as u16 as u32;
                    // ponytail: a fase +0x3a vai num registrador com lixo nos 16 bits altos; a 0x86bdd0 só usa a palavra.
                    cpz4_classificado(clube, comp, cpz4_w(this + 0x3a) as u16 as u32, rodada, peso);
                } else {
                    let peso = cpz4_escala(1, 0x14, r, 2);
                    cpz4_classificado(clube, comp, 0x41c, 0, peso);
                }
            }
            0xff | 2 => {
                let peso = cpz4_escala(1, 0x14, cpz4_w(jogo + 0x34), 2);
                // ponytail: +0x32 e +0x30 vão em registradores com lixo nos 16 bits altos; a 0x86c000 só usa as palavras.
                cpz4_eliminado(clube, comp, cpz4_w(jogo + 0x32) as u16 as u32, cpz4_w(jogo + 0x30) as u16 as u32, peso);
                crate::comparacao::ramo("cpz_4 caiu na qualificacao");
                cpz4_sair(clube); // cpz4: quem cai na qualificação sai da competição europeia
            }
            _ => {}
        }
    } else if (f as i8) < 8 {
        if d == 6 {
            cpz4_classificado(clube, comp, 0x44a, 0xffff_ffff, 0x32);
        } else if d == 0xfe {
            cpz4_nao_classificado(clube, comp, 0x449, 0x32);
            crate::comparacao::ramo("cpz_4 caiu na fase 1");
            cpz4_sair(clube); // cpz4: quem cai na fase 1 sai da competição europeia
        }
    } else if (f as i8) < 0xc {
        if d == 6 {
            crate::comparacao::ramo("cpz_4 passou da fase 2");
            cpz4_classificado(clube, comp, 0, 0x78, 0x32); // cpz4: passar da fase 2 pesa 0x32
        } else if d == 0xfe {
            cpz4_nao_classificado(clube, comp, 0x44a, 0x32);
            cpz4_sair(clube);
        }
    } else if f == 0xc {
        match d {
            1 => {
                let titulo: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x86a800usize);
                titulo(cpz4_registros(), clube, jogo, fim);
                cpz4_sair(clube);
            }
            2 => {
                let vice: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x86b0b0usize);
                vice(cpz4_registros(), clube, jogo, fim);
                cpz4_sair(clube);
            }
            0 => {
                let peso = cpz4_escala(0x4b, 0x64, cpz4_w(jogo + 0x34), 2);
                let mata = cpz4_d(cpz4_d(this + 0xc) + 0x30);
                let rodadas = cpz4_d(mata + 0xa3);
                let r = cpz4_w(jogo + 0x34) as i32;
                // os bits de cima do registrador que a original empurra são os do ponteiro das fases (+0xc)
                let rodada = (cpz4_d(this + 0xc) as u32 & 0xffff_0000) | cpz4_w(rodadas.wrapping_add((r as usize).wrapping_mul(0x68)) + 0x6f) as u16 as u32;
                cpz4_classificado(clube, comp, 0, rodada, peso);
            }
            0xff => {
                let peso = cpz4_escala(0x4b, 0x64, cpz4_w(jogo + 0x34), 2);
                cpz4_eliminado(clube, comp, 0, cpz4_w(jogo + 0x30) as u16 as u32, peso);
                cpz4_sair(clube);
            }
            _ => {}
        }
    }
    0
}

// ponytail: os registros vão para o arquivo, mexem nas reputações e no moral (sorteiam) e o título e o vice gravam o
// elenco da final: não rodam duas vezes. Só a original roda; o plano confere o que a rotina mesma grava (+0x1db do
// clube: −1 nos casos em que ele sai, intacto nos outros).
unsafe extern "thiscall" fn comparar_registros_da_liga_dos_campeoes(this: usize, clube: usize, desfecho: u32, fase: u32, a: u32, jogo: usize, fim: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1389);
    let (d, f) = (desfecho as u8, fase as u8);
    // as faixas da original comparam o byte da fase com sinal (0x80 a 0xfe contam como fase 1)
    let sai = if f == 0xff {
        d == 0xff || d == 2
    } else if (f as i8) < 0xc {
        d == 0xfe
    } else if f == 0xc {
        d == 1 || d == 2 || d == 0xff
    } else {
        false
    };
    let antes = if clube != 0 { core::ptr::read_unaligned((clube + 0x1db) as *const i32) } else { 0 };
    let r = o(this, clube, desfecho, fase, a, jogo, fim);
    let depois = if clube != 0 { core::ptr::read_unaligned((clube + 0x1db) as *const i32) } else { 0 };
    let igual = r == 0 && (clube == 0 || depois == if sai { -1 } else { antes });
    PLACAR.registrar(igual, || format!("registros_da_liga_dos_campeoes({:#x}, {:#x}, {}, {}) original={} +0x1db {} -> {} sai={}", this, clube, d, f, r, antes, depois, sai));
    r
}

/// Os quatro sorteados da fase 2 (os índices dos grupos, todos diferentes): a, b ≠ a, c ≠ a, b e d ≠ a, b, c, cada um
/// com sorteio(4) (0x90cfc0) até sair um que serve (a d sorteia pelo menos uma vez: começa igual à a).
unsafe fn cpz4_sorteio_dos_grupos() -> [i8; 4] {
    let a = sortear(4) as i8;
    let mut b = sortear(4) as i8;
    while b == a {
        b = sortear(4) as i8;
    }
    let mut c = sortear(4) as i8;
    while c == a || c == b {
        c = sortear(4) as i8;
    }
    let mut d = a;
    while d == a || d == b || d == c {
        d = sortear(4) as i8;
    }
    [a, b, c, d]
}

/// O clube da linha `k` do grupo `g` da fase 2 (a fase [+0xc] + 0x20 + 4 × g; League::get_table_row 0x6826d0,
/// thiscall(grupo, k), ret 4).
unsafe fn cpz4_do_grupo(this: usize, g: i8, k: u32) -> usize {
    let linha: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x6826d0usize);
    cpz4_d(linha(cpz4_d(cpz4_d(this + 0xc) + 0x20 + 4 * (g as i32 as usize)), k))
}

/// { 0x586330 } (thiscall, ret 0, european_cup.cpp): `EuropeanCup::draw_quarter_finals()`, da fase seguinte 0x5851d0
/// na fase 12. A lista das 8 (calloc 0x945501 de 8 × 4; sem memória: a caixa da linha 0x762) com os grupos sorteados
/// a, b, c e d: o 1º de a × o 2º de b, o 2º de a × o 1º de b, o 1º de c × o 2º de d e o 2º de c × o 1º de d. Cada um
/// passa pela 0x7e9180 (thiscall(o objeto [[0xae28f0]] + 4 × +0x50, clube), ret 4). A virtual +0x3c (create_round_table:
/// fase 12) e a fase do mata-mata (operator new 0x944cf1 de 0xb2; Cup_Stage::Cup_Stage 0x522e00 com a copa, 8, a lista,
/// as rodadas, +4, a tabela, +0x40, 0xc, 2, o extra, 0x14, 1, 0, 0, 0) na vaga [+0xc] + 0x30; a lista e a tabela
/// liberadas (0x9452ca). Sem fase: a caixa da linha 0x78a. Senão o 3º e o 4º de cada grupo da fase 2 saem (+0x1db =
/// −1), a rodada montada (Cup::set_up_round 0x51c800, thiscall(fase, 0)) e +0x30 = 12.
unsafe extern "thiscall" fn sorteio_das_quartas(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let lista = calloc(8, 4);
    if lista == 0 {
        return aviso_em(0x9c4900, 0x762);
    }
    crate::comparacao::ramo("cpz_4 sorteio das quartas");
    let [a, b, c, d] = cpz4_sorteio_dos_grupos();
    let ordem: [(i8, u32); 8] = [(a, 0), (b, 1), (a, 1), (b, 0), (c, 0), (d, 1), (c, 1), (d, 0)];
    for (k, &(g, l)) in ordem.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cpz4_do_grupo(this, g, l));
    }
    let tv: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7e9180usize);
    for k in 0..8 {
        let obj = cpz4_d(cpz4_d(cpz4_d(0xae28f0)) + 4 * (cpz4_b(this + 0x50) as i8 as i32 as usize));
        tv(obj, cpz4_d(lista + 4 * k));
    }
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cpz4_d(cpz4_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0xc, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: as rodadas, o extra e a palavra +0x40 vão em registradores com lixo nos 16 bits altos; o construtor
    // só lê as palavras.
    let fase = if p != 0 {
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 8, lista, rodadas as u16 as u32, cpz4_d(this + 4), t, cpz4_w(this + 0x40) as u16 as u32, 0xc, 2, extra as u16 as u32, 0x14, 1, 0, 0, 0)
    } else {
        0
    };
    core::ptr::write_unaligned((cpz4_d(this + 0xc) + 0x30) as *mut usize, fase);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(lista);
    liberar(t);
    let fase = cpz4_d(cpz4_d(this + 0xc) + 0x30);
    if fase == 0 {
        return aviso_em(0x9c4900, 0x78a);
    }
    for g in 0..4i8 {
        cpz4_sair(cpz4_do_grupo(this, g, 2));
        cpz4_sair(cpz4_do_grupo(this, g, 3));
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    core::ptr::write_unaligned((this + 0x30) as *mut i32, 0xc);
}

// ponytail: o sorteio sorteia, a 0x7e9180 e o construtor alocam e a rodada cria os jogos: só a original. O plano,
// feito antes: os 8 primeiros e segundos dos grupos e o 3º e 4º de cada um; depois, +0x30 = 12, a fase do mata-mata
// com 8 clubes que são os primeiros e segundos (como conjunto) e o 3º e o 4º de cada grupo com +0x1db = −1.
unsafe extern "thiscall" fn comparar_sorteio_das_quartas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1390);
    let mut vivos: Vec<usize> = Vec::new();
    let mut fora: Vec<usize> = Vec::new();
    for g in 0..4i8 {
        vivos.push(cpz4_do_grupo(this, g, 0));
        vivos.push(cpz4_do_grupo(this, g, 1));
        fora.push(cpz4_do_grupo(this, g, 2));
        fora.push(cpz4_do_grupo(this, g, 3));
    }
    o(this);
    let fase = cpz4_d(cpz4_d(this + 0xc) + 0x30);
    if fase == 0 {
        PLACAR.registrar(false, || format!("sorteio_das_quartas({:#x}) sem fase", this));
        return;
    }
    let lista = cpz4_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..8).map(|k| cpz4_d(lista + 6 * k)).collect();
    let mut esperado = vivos.clone();
    achou.sort_unstable();
    esperado.sort_unstable();
    let saiu = fora.iter().all(|&c| core::ptr::read_unaligned((c + 0x1db) as *const i32) == -1);
    let igual = achou == esperado && saiu && cpz4_w(fase + 0x3e) == 8 && core::ptr::read_unaligned((this + 0x30) as *const i32) == 0xc;
    PLACAR.registrar(igual, || format!("sorteio_das_quartas({:#x}) esperado={:x?} lista={:x?} saiu={} +0x30={}", this, esperado, achou, saiu, core::ptr::read_unaligned((this + 0x30) as *const i32)));
}

/// A lista dos inscritos (+0x14, 6 bytes: +0 o clube, +4 a rodada de entrada; +0x38 deles) que seguem na competição
/// (+0x1db = o id da copa, [+4]); com `sem_7`, fora os de rodada de entrada 7. Grava em `v` (até 71) e devolve a conta.
unsafe fn cpz4_inscritos(this: usize, sem_7: bool, v: *mut usize) -> i16 {
    let mut n: i16 = 0;
    let mut i: i16 = 0;
    while i < cpz4_w(this + 0x38) {
        let e = cpz4_d(this + 0x14).wrapping_add((i as i32 as usize).wrapping_mul(6));
        let c = cpz4_d(e);
        if core::ptr::read_unaligned((c + 0x1db) as *const i32) == core::ptr::read_unaligned(cpz4_d(this + 4) as *const i32) && !(sem_7 && cpz4_b(e + 4) == 7) {
            core::ptr::write_unaligned(v.add(n as i32 as usize), c);
            n = n.wrapping_add(1);
        }
        i += 1;
    }
    n
}

/// { 0x586660 } (thiscall, ret 8, european_cup.cpp): `EuropeanCup::collect_club_list(&lista, tipo)`, a virtual +0x58
/// da Liga dos Campeões. Tipo 1: os inscritos que seguem, fora os da rodada 7; tipo 2: todos os que seguem; tipo 3 na
/// fase 11: os que seguem; noutro caso a da competição (Comp::collect_club_list 0x48e1c0, thiscall(this, &lista, 0),
/// ret 8). A lista (malloc 0x944e46 de 0x11c bytes; sem memória: a caixa das linhas 0x7b5, 0x7cf ou 0x7e9, lista
/// nula e 0) vai em `lista`; devolve a conta na palavra de baixo (a de cima é a do ponteiro da lista).
unsafe extern "thiscall" fn clubes_da_liga_dos_campeoes(this: usize, saida: usize, tipo: u32) -> u32 {
    let t = tipo as u8;
    let (linha, sem_7) = match t {
        1 => (0x7b5, true),
        2 => (0x7cf, false),
        3 if core::ptr::read_unaligned((this + 0x30) as *const i32) == 0xb => (0x7e9, false),
        _ => {
            let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x48e1c0usize);
            return base(this, saida, 0);
        }
    };
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar(0x11c);
    if v == 0 {
        aviso_em(0x9c4900, linha);
        core::ptr::write_unaligned(saida as *mut usize, 0);
        return 0;
    }
    crate::comparacao::ramo("cpz_4 lista dos inscritos");
    let n = cpz4_inscritos(this, sem_7, v as *mut usize);
    core::ptr::write_unaligned(saida as *mut usize, v);
    (v as u32 & 0xffff_0000) | n as u16 as u32
}

/// Comparador da 0x586660: a original aloca a lista (só ela roda; quem chama libera). O plano, feito antes numa lista
/// local pelos mesmos inscritos: a conta (palavra de baixo da devolução) e os clubes da lista dela iguais. Nos tipos
/// que vão para a 0x48e1c0 e sem lista: a devolução e a saída como a original deixou.
unsafe extern "thiscall" fn comparar_clubes_da_liga_dos_campeoes(this: usize, saida: usize, tipo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1391);
    let t = tipo as u8;
    let sem_7 = match t {
        1 => Some(true),
        2 => Some(false),
        3 if core::ptr::read_unaligned((this + 0x30) as *const i32) == 0xb => Some(false),
        _ => None,
    };
    let mut plano = [0usize; 71];
    let n = match sem_7 {
        Some(s) if cpz4_w(this + 0x38) <= 71 => cpz4_inscritos(this, s, plano.as_mut_ptr()),
        _ => return o(this, saida, tipo),
    };
    let r = o(this, saida, tipo);
    let v = cpz4_d(saida);
    if v == 0 {
        PLACAR.registrar(r as u16 == 0, || format!("clubes_da_liga_dos_campeoes({:#x}, {}) sem lista original={:#x}", this, t, r));
        return r;
    }
    let lista = core::slice::from_raw_parts(v as *const usize, (n.max(0) as usize).min(71));
    let igual = r as u16 as i16 == n && lista == &plano[..lista.len()];
    PLACAR.registrar(igual, || format!("clubes_da_liga_dos_campeoes({:#x}, {}) original={:#x} rust={} lista={:x?} plano={:x?}", this, t, r, n, lista, &plano[..lista.len()]));
    r
}

// ---------- 262a leva: as estatísticas das táticas: o resultado de cada partida na linha da tática do clube do computador, o capitão guardado e o capitão escolhido (tactics.cpp) ----------

unsafe fn cpz5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A linha da tabela de estatísticas das táticas ([this], 0x28 bytes por tática) usada pelo clube: a palavra +0 do
/// registro de 0x91 bytes do clube (id em +0) na tabela tática (+0x601).
unsafe fn cpz5_linha(this: usize, clube: usize) -> usize {
    let id = cpz5_d(clube);
    let k = cpz5_w(cpz5_d(this + 0x601).wrapping_add(id.wrapping_mul(0x91))) as i32;
    cpz5_d(this).wrapping_add((k as isize * 0x28) as usize)
}

/// x < 1 vale 1 (palavra com sinal), como as três comparações antes de cada `fild`.
fn cpz5_pelo_menos_1(x: i16) -> f64 {
    if x < 1 { 1.0 } else { x as f64 }
}

/// { 0x896290 } (thiscall, ret 4, tactics.cpp): `Tactics::record_tactic_result(jogo)`, do fim de cada partida no laço
/// do motor (0x6ba9a0, com o placar agregado; fixture +0 ≠ −2). Jogo nulo: a caixa da linha 0x1f9 e 0. Mandante (+0x1c)
/// ou visitante (+0x20) dirigido por humano (0x600a50 com 0, 1, 0, 0; cdecl 5): 0. Senão, para cada lado (clube nulo:
/// a caixa da linha 0x20a e segue), o registro da partida pelo clube (0x7e4750, num registro local que começa com a
/// data padrão 0x54a110) soma na linha da tática do clube (cpz5_linha): +2 jogos, +4 vitórias (+3 pontos em +0xe), +6
/// empates (+1 ponto), +8 derrotas, +0xa gols pró (+0x19 do registro, com sinal), +0xc contra (+0x1a); com p = jogos,
/// a = pró, b = contra e c = pontos (cada um pelo menos 1), q = c/p, g = a/p, h = g·q², r = p/b: +0x10 = r·h·g, +0x18
/// = r·g·q²·q, +0x20 = r·r·h (a ordem das contas do x87). A escolha da formação do computador (0x6e0230) lê +2 e as
/// três razões. Devolve 1.
unsafe extern "thiscall" fn registrar_resultado_da_tatica(this: usize, jogo: usize) -> u32 {
    if jogo == 0 {
        aviso_em(0xa8b908, 0x1f9);
        return 0;
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    for k in 0..2 {
        let c = cpz5_d(jogo + 0x1c + 4 * k);
        if c != 0 && humano(c, 0, 1, 0, 0) != 0 {
            return 0;
        }
    }
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let registro: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7e4750usize);
    let mut reg = [0u8; 0x20];
    let pr = reg.as_mut_ptr() as usize;
    data_padrao(pr);
    for k in 0..2 {
        let c = cpz5_d(jogo + 0x1c + 4 * k);
        if c == 0 {
            aviso_em(0xa8b908, 0x20a);
            continue;
        }
        let l = cpz5_linha(this, c);
        registro(jogo, c, pr);
        let m = |o: usize| (l + o) as *mut i16;
        crate::comparacao::ramo("cpz_5 linha da tatica");
        *m(2) = (*m(2)).wrapping_add(1);
        let (pro, contra) = (reg[0x19] as i8, reg[0x1a] as i8);
        if pro > contra {
            *m(4) = (*m(4)).wrapping_add(1);
            *m(0xe) = (*m(0xe)).wrapping_add(3); // cpz5: a vitória vale 3 pontos
        } else if pro == contra {
            *m(6) = (*m(6)).wrapping_add(1);
            *m(0xe) = (*m(0xe)).wrapping_add(1);
        } else {
            *m(8) = (*m(8)).wrapping_add(1);
        }
        *m(0xa) = (*m(0xa)).wrapping_add(pro as i16); // cpz5: os gols pró
        *m(0xc) = (*m(0xc)).wrapping_add(contra as i16);
        let a = cpz5_pelo_menos_1(*m(0xa));
        let b = cpz5_pelo_menos_1(*m(0xc));
        let c = cpz5_pelo_menos_1(*m(0xe));
        let p = cpz5_pelo_menos_1(*m(2));
        let q = c / p;
        let q2 = q * q;
        let g = a / p;
        let h = g * q2;
        let r = p / b;
        let f = |o: usize| (l + o) as *mut f64;
        core::ptr::write_unaligned(f(0x10), r * h * g);
        core::ptr::write_unaligned(f(0x18), r * g * q2 * q); // cpz5: a razão da postura normal
        core::ptr::write_unaligned(f(0x20), r * r * h);
    }
    1
}

/// Comparador da 0x896290: lado a lado nas linhas das táticas dos dois clubes (0x28 bytes cada) e na devolução; o
/// registro da partida é local. Jogo ou clube nulos: a caixa, só a original.
unsafe extern "thiscall" fn comparar_registrar_resultado_da_tatica(this: usize, jogo: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1392);
    if jogo == 0 || cpz5_d(jogo + 0x1c) == 0 || cpz5_d(jogo + 0x20) == 0 {
        return o(this, jogo);
    }
    let reg = [(cpz5_linha(this, cpz5_d(jogo + 0x1c)), 0x28), (cpz5_linha(this, cpz5_d(jogo + 0x20)), 0x28)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo), || registrar_resultado_da_tatica(this, jogo));
    PLACAR.registrar(igual && r == n, || format!("registrar_resultado_da_tatica(jogo {:x}) original={} rust={} {:x?} x {:x?}", jogo, r, n, mo, mr));
    r
}

/// { 0x8976a0 } (thiscall, ret 8, tactics.cpp): `Tactics::get_captain(clube, time)`, das telas de tática (0x8a7440). Clube
/// nulo: a caixa da linha 0x527 e 0. A linha do clube (0x8982b0, thiscall(this, clube, time), ret 8; −1: 0); devolve o
/// dword +0x52 do registro de 0x91 bytes da linha na tabela tática (+0x601).
unsafe extern "thiscall" fn capitao_guardado(this: usize, clube: usize, time: u32) -> u32 {
    if clube == 0 {
        aviso_em(0xa8b908, 0x527);
        return 0;
    }
    let linha: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(this, clube, time);
    if k == -1 {
        return 0;
    }
    cpz5_d(cpz5_d(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52) as u32
}

/// Comparador da 0x8976a0: pela devolução (só lê; a linha nula ou fora da faixa mostra a caixa nas duas).
unsafe extern "thiscall" fn comparar_capitao_guardado(this: usize, clube: usize, time: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1393);
    if clube == 0 {
        return o(this, clube, time);
    }
    let r = o(this, clube, time);
    let n = capitao_guardado(this, clube, time);
    PLACAR.registrar(r == n, || format!("capitao_guardado(clube {:x}, time {}) original={:x} rust={:x}", clube, time, r, n));
    r
}

/// { 0x897880 } (thiscall, ret 0xc, tactics.cpp): `Tactics::set_captain(clube, &pessoa, time)`, ao aplicar as mudanças da
/// tela de tática (0x8a80d0). Clube nulo: a caixa da linha 0x579; ponteiro nulo: a da 0x581; 0. A linha do clube
/// (0x8982b0; −1: 0); grava o dword apontado em +0x52 do registro da linha (+0x601) e devolve 1.
unsafe extern "thiscall" fn escolher_capitao(this: usize, clube: usize, pessoa: usize, time: u32) -> u32 {
    if clube == 0 {
        aviso_em(0xa8b908, 0x579);
        return 0;
    }
    if pessoa == 0 {
        aviso_em(0xa8b908, 0x581);
        return 0;
    }
    let linha: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(this, clube, time);
    if k == -1 {
        return 0;
    }
    let p = cpz5_d(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52;
    core::ptr::write_unaligned(p as *mut u32, core::ptr::read_unaligned(pessoa as *const u32));
    1
}

/// Comparador da 0x897880: lado a lado no dword do capitão da linha (achada antes pela 0x8982b0, que só lê) e na
/// devolução. Clube ou ponteiro nulos, ou sem linha: só a original.
unsafe extern "thiscall" fn comparar_escolher_capitao(this: usize, clube: usize, pessoa: usize, time: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(1394);
    if clube == 0 || pessoa == 0 {
        return o(this, clube, pessoa, time);
    }
    let linha: unsafe extern "thiscall" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(this, clube, time);
    if k == -1 {
        return o(this, clube, pessoa, time);
    }
    let p = cpz5_d(this + 0x601).wrapping_add((k as isize * 0x91) as usize) + 0x52;
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(p, 4)], || o(this, clube, pessoa, time), || escolher_capitao(this, clube, pessoa, time));
    PLACAR.registrar(igual && r == n, || format!("escolher_capitao(clube {:x}, time {}) original={} rust={} {:x?} x {:x?}", clube, time, r, n, mo, mr));
    r
}

// ---------- 262a leva: o jogador a serviço da seleção: a seleção ou o time das estrelas que o tem na data, o jogador que serve à seleção do técnico e as competições das convocações (national_teams.cpp) ----------

unsafe fn cpz6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz6_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpz6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube do time das estrelas `g` ([0x9d00a4] ou [0x9d00a8]): o registro de 0x245 bytes em [0xae23b4].
unsafe fn cpz6_estrelas(g: usize) -> usize {
    cpz6_d(0xae23b4).wrapping_add(cpz6_d(g).wrapping_mul(0x245))
}

/// { 0x7757d0 } (thiscall, ret 0x14, national_teams.cpp): `NationalTeams::get_international_duty_club(jogador, data,
/// clube, comentario, adversario)`, da escalação (0x6dbb80, 0x6dbc50), dos olheiros, das transferências e das telas.
/// Com uma lista provisória (0x68d020, destruída no fim por 0x68d040). Clube seleção (0x53b900) ou um dos times das
/// estrelas (id em [0x9d00a4]/[0x9d00a8]): 0. A janela vai da véspera (0x54c8e0) ao dia seguinte (0x54c5b0). Jogador
/// convocado (+0x24, a seleção): os jogos da seleção na janela (0x5acbe0, lado pelo bit 4 de +0x5e: ligado 1, senão 0;
/// os dias e anos como os dwords da original); com jogo e sem adversário, ou com o adversário (+0x14 do jogo) entre
/// eles, ele está a serviço (o texto 0xa62ff4 em `comentario`, 0x66f4e0) e devolve a seleção. Com o bit 4 ligado, as
/// convocações da lista [this] (+0 a seleção, +4 o adversário, +8 a data) com a seleção dele (e o adversário, se dado)
/// e data antes da pedida: está no time (o texto 0xa62fc8) e devolve a seleção. Senão o time das estrelas (a
/// competição [[0xadadfc] + 4·[0x9cf690]]: a data do jogo pelo registro +0xa3, dia +3 e ano +5 + o ano +0x40, 0x549ef0)
/// para jogador de clube (+0x39) da liga [0x9cf590] (+0x57), com a data na janela: os 50 nomes (+0xd7) dos dois times
/// (primeiro [0x9d00a8]; o texto 0xa62fa4 em 0xde1f64 se há `comentario`). Senão 0.
unsafe extern "thiscall" fn selecao_que_tem_o_jogador(this: usize, jogador: usize, data: usize, clube: usize, comentario: usize,
                                                      adversario: usize) -> usize {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let mut lista = [0u8; 0x10];
    let pl = lista.as_mut_ptr() as usize;
    criar(pl);
    let sai = |r: usize| -> usize {
        destruir(pl);
        r
    };
    if clube != 0 {
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        if selecao(clube) != 0 {
            return sai(0);
        }
        let id = cpz6_d(clube);
        if id == cpz6_d(0x9d00a4) || id == cpz6_d(0x9d00a8) {
            return sai(0);
        }
    }
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let (mut lo, mut hi) = ([0u8; 8], [0u8; 8]);
    let (plo, phi) = (lo.as_mut_ptr() as usize, hi.as_mut_ptr() as usize);
    menos(data, plo, 1); // cpz6: a véspera
    mais(data, phi, 1); // cpz6: o dia seguinte
    let nacional = cpz6_d(jogador + 0x24);
    if nacional != 0 {
        let sem_bit = (cpz6_b(jogador + 0x5e) & 4 == 0) as u32;
        // ponytail: a original empurra o lado com os 24 bits altos de ecx sobrando; a 0x5acbe0 só lê o byte
        let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
            core::mem::transmute(0x5acbe0usize);
        let n = jogos(cpz6_d(0xae2a58), cpz6_d(plo) as u32, cpz6_d(plo + 2) as u32, cpz6_d(phi) as u32, cpz6_d(phi + 2) as u32,
            cpz6_d(nacional) as u32, pl, u32::MAX, 0xffff_fffe, (sem_bit == 0) as u32);
        let mut servico = false;
        if n as u16 != 0 {
            if adversario == 0 {
                servico = true;
            } else {
                core::ptr::write_unaligned((pl + 0xa) as *mut usize, cpz6_d(pl + 2));
                loop {
                    let j = proximo(pl);
                    if j == 0 {
                        break;
                    }
                    if cpz6_d(j + 0x14) == adversario {
                        servico = true;
                        break;
                    }
                }
            }
        }
        if servico {
            if comentario != 0 {
                texto(comentario, 0xa62ff4, 0xad9c64usize);
            }
            crate::comparacao::ramo("cpz_6 a servico da selecao");
            return sai(cpz6_d(jogador + 0x24)); // cpz6: a serviço da seleção
        }
        let conv = cpz6_d(this);
        if conv != 0 && sem_bit == 0 {
            core::ptr::write_unaligned((conv + 0xa) as *mut usize, cpz6_d(conv + 2));
            loop {
                let e = proximo(cpz6_d(this));
                if e == 0 {
                    break;
                }
                if cpz6_d(e) != cpz6_d(cpz6_d(jogador + 0x24)) {
                    continue;
                }
                if adversario != 0 && cpz6_d(e + 4) != cpz6_d(adversario) {
                    continue;
                }
                let (dia, ano) = (cpz6_w(e + 8) as i16, cpz6_w(e + 0xa));
                let hoje = cpz6_w(data + 2);
                if ano < hoje || (ano == hoje && dia < cpz6_w(data) as i16) {
                    if comentario != 0 {
                        texto(comentario, 0xa62fc8, 0xad9c64usize);
                    }
                    crate::comparacao::ramo("cpz_6 no time da selecao");
                    return sai(cpz6_d(jogador + 0x24)); // cpz6: já no time da seleção
                }
            }
        }
    }
    let comp = cpz6_d(cpz6_d(0xadadfc).wrapping_add(cpz6_d(0x9cf690).wrapping_mul(4)));
    if comp == 0 {
        return sai(0);
    }
    let reg = cpz6_d(comp + 0xa3);
    let ano = (cpz6_b(reg + 5) as i8 as i16).wrapping_add(cpz6_w(comp + 0x40) as i16) as u16;
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mut jogo = [0u8; 8];
    let pj = jogo.as_mut_ptr() as usize;
    data_nova(pj, cpz6_w(reg + 3) as u32, ano as u32);
    let cj = cpz6_d(jogador + 0x39);
    if cj == 0 {
        return sai(0);
    }
    let liga = cpz6_d(cj + 0x57);
    if liga == 0 || cpz6_d(liga) != cpz6_d(0x9cf590) {
        return sai(0);
    }
    let (jd, ja) = (cpz6_w(pj) as i16, cpz6_w(pj + 2));
    let (ld, la) = (cpz6_w(plo) as i16, cpz6_w(plo + 2));
    if la > ja || (la == ja && ld > jd) {
        return sai(0);
    }
    let (hd, ha) = (cpz6_w(phi) as i16, cpz6_w(phi + 2));
    if ha < ja || (ha == ja && hd < jd) {
        return sai(0);
    }
    for i in 0..0x32usize {
        for g in [0x9d00a8usize, 0x9d00a4] {
            if cpz6_d(cpz6_estrelas(g) + 0xd7 + 4 * i) == jogador {
                if comentario != 0 {
                    texto(0xde1f64, 0xa62fa4, 0xad9c64usize);
                }
                crate::comparacao::ramo("cpz_6 time das estrelas");
                return sai(cpz6_estrelas(g));
            }
        }
    }
    sai(0)
}

/// Comparador da 0x7757d0 (2,1 milhões de chamadas até 2002-06-01: o lado a lado leve, sem alocar até 256 bytes): cada
/// uma com a sua lista provisória (como a original); no texto de 0xde1f64 (0x80) e no do comentário (0x40) e na devolução.
unsafe extern "thiscall" fn comparar_selecao_que_tem_o_jogador(this: usize, jogador: usize, data: usize, clube: usize, comentario: usize,
                                                               adversario: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize>(1395);
    let regioes = [(0xde1f64usize, 0x80usize), (comentario, 0x40)];
    let reg = if comentario != 0 { &regioes[..] } else { &regioes[..1] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg,|| o(this, jogador, data, clube, comentario, adversario),
        || selecao_que_tem_o_jogador(this, jogador, data, clube, comentario, adversario));
    PLACAR.registrar(igual && r == n, || format!("selecao_que_tem_o_jogador(jogador {:x}, data {:x}, clube {:x}, adversario {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        jogador, data, clube, adversario, r, n, mo, mr));
    r
}

/// { 0x76d9b0 } (stdcall, ret 8, national_teams.cpp): `national_teams_player_is_eligible(jogador, tecnico)`, da busca de
/// jogadores (0x7afa70, 0x807450) e da tela de comparação. Sem ficha de jogador (+0x61), sem a seleção do técnico (+0x24)
/// ou ela não sendo seleção (0x53b900): 0. Já convocado para ela: 1. Seleção do segundo conjunto (0x5409b0, a sub-21)
/// e idade (+0x18) acima de 21: 0. Com +0x22 ou o bit 2 de +0xc do registro de 79 bytes da pessoa ([0xae2d0c]), a
/// nacionalidade (+0x1a) tem de ser a da seleção (+0x53); convocado para outra seleção de outra nação: 0. A nação da
/// seleção tem de ser a primeira ou a segunda (+0x1e) nacionalidade; sendo a nação [0x9cf434] e a segunda também: 0.
/// Senão 1.
unsafe extern "stdcall" fn jogador_serve_a_selecao(jogador: usize, tecnico: usize) -> u32 {
    let sel = cpz6_d(tecnico + 0x24);
    if cpz6_d(jogador + 0x61) == 0 || sel == 0 {
        return 0;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(sel) == 0 {
        return 0;
    }
    if cpz6_d(jogador + 0x24) == sel {
        return 1;
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    if segunda(sel) != 0 && cpz6_b(jogador + 0x18) as i8 > 0x15 {
        return 0;
    }
    let nacao = cpz6_d(sel + 0x53);
    let primeira = cpz6_d(jogador + 0x1a);
    if cpz6_b(jogador + 0x22) != 0 && primeira != nacao {
        return 0;
    }
    let p = cpz6_d(0xae2d0c).wrapping_add(cpz6_d(jogador).wrapping_mul(79));
    if cpz6_b(p + 0xc) & 2 != 0 && cpz6_d(jogador + 0x1a) != nacao {
        return 0;
    }
    let outra = cpz6_d(jogador + 0x24);
    if outra != 0 && cpz6_d(outra + 0x53) != nacao {
        return 0;
    }
    let primeira = cpz6_d(jogador + 0x1a);
    if primeira != nacao && cpz6_d(jogador + 0x1e) != nacao {
        return 0;
    }
    let x = cpz6_d(0x9cf434);
    if cpz6_d(nacao) == x {
        let s = cpz6_d(jogador + 0x1e);
        if s != 0 && cpz6_d(s) == x {
            return 0;
        }
    }
    1
}

/// Comparador da 0x76d9b0: pela devolução (só lê).
unsafe extern "stdcall" fn comparar_jogador_serve_a_selecao(jogador: usize, tecnico: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, usize) -> u32>(1396)(jogador, tecnico);
    let n = jogador_serve_a_selecao(jogador, tecnico);
    PLACAR.registrar(r == n, || format!("jogador_serve_a_selecao(jogador {:x}, tecnico {:x}) original={} rust={}", jogador, tecnico, r, n));
    r
}

/// { 0x775580 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::collect_duty_competitions(saida)`, dos olheiros
/// (0x7f7010). Sem a lista de convocações [this] ou sem `saida`: 0. Para cada convocação, a competição dela (+4) entra
/// em `saida` (0x68d0d0, 4 bytes) se ainda não estiver lá; devolve quantas entraram (byte).
unsafe extern "thiscall" fn juntar_competicoes_das_convocacoes(this: usize, saida: usize) -> u8 {
    let conv = cpz6_d(this);
    if conv == 0 || saida == 0 {
        return 0;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    let mut n: u8 = 0;
    core::ptr::write_unaligned((conv + 0xa) as *mut usize, cpz6_d(conv + 2));
    let mut e = proximo(cpz6_d(this));
    while e != 0 {
        core::ptr::write_unaligned((saida + 0xa) as *mut usize, cpz6_d(saida + 2));
        let mut ja = false;
        loop {
            let x = proximo(saida);
            if x == 0 {
                break;
            }
            if cpz6_d(x) == cpz6_d(e + 4) {
                ja = true;
                break;
            }
        }
        if !ja {
            acrescentar(saida, e + 4, 4);
            n = n.wrapping_add(1);
        }
        e = proximo(cpz6_d(this));
    }
    n
}

/// Os valores dos nós de uma lista (cabeça em +2, nó: +0 o item, +6 o próximo), sem mexer no cursor (+0xa).
unsafe fn cpz6_itens(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = cpz6_d(lista + 2);
    while no != 0 {
        v.push(cpz6_d(no));
        no = cpz6_d(no + 6);
    }
    v
}

// ponytail: a 0x775580 acrescenta nós (aloca): só a original roda. O plano, feito antes pelos nós (sem mexer nos
// cursores): as competições das convocações que ainda não estão em `saida`, na ordem, sem repetir; depois, a devolução
// é quantas e os últimos itens de `saida` (4 bytes cada) são elas.
unsafe extern "thiscall" fn comparar_juntar_competicoes_das_convocacoes(this: usize, saida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(1397);
    let conv = cpz6_d(this);
    if conv == 0 || saida == 0 {
        let r = o(this, saida);
        PLACAR.registrar(r == 0, || format!("juntar_competicoes_das_convocacoes(saida {:x}) sem lista original={}", saida, r));
        return r;
    }
    let mut tem: Vec<usize> = cpz6_itens(saida).iter().map(|&x| cpz6_d(x)).collect();
    let antes = tem.len();
    for e in cpz6_itens(conv) {
        let c = cpz6_d(e + 4);
        if !tem.contains(&c) {
            tem.push(c);
        }
    }
    let r = o(this, saida);
    let depois: Vec<usize> = cpz6_itens(saida).iter().map(|&x| cpz6_d(x)).collect();
    let igual = r == (tem.len() - antes) as u8 && depois == tem;
    PLACAR.registrar(igual, || format!("juntar_competicoes_das_convocacoes(saida {:x}) original={} plano={:x?} depois={:x?}", saida, r, tem, depois));
    r
}

// ---------- 262a leva: os registros de partida e de nome: a partida vista por um clube, a referência ao nome da pessoa e o código do resultado (record_utils.cpp) ----------

unsafe fn cpz7_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpz7_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpz7_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x7e4750 } (cdecl, record_utils.cpp): `fill_match_record_from_fixture(jogo, clube, saida)`, da estatística da tática
/// (0x896290, no fim de cada partida sem humano) e dos recordes. Jogo, clube ou saída nulos: a caixa (linhas 0x29, 0x31,
/// 0x39) e 0. Senão a data (+0, 0x417c10 da Date(dia +0x2a, ano +0x28) da 0x549ef0), +8 o id do jogo (+0), +0xc a
/// palavra +0x32, +0xe a +0x30, +0x10 o lado (0x4afa80, cdecl(clube, jogo)), +0x11 o id do clube, +0x1b o dword +8 do
/// jogo; mandante (+0x1c): +0x15 o +0x10 do jogo e o placar (+0x19 pró, +0x1a contra) pelo da prorrogação (+0x47/+0x48)
/// se ele não for negativo, senão o normal (+0x43/+0x44); visitante: +0x15 o +0xc e o placar ao contrário. Devolve 1.
unsafe extern "C" fn registro_da_partida_pelo_clube(jogo: usize, clube: usize, saida: usize) -> u32 {
    if jogo == 0 {
        aviso_em(0xa7a088, 0x29);
        return 0;
    }
    if clube == 0 {
        aviso_em(0xa7a088, 0x31);
        return 0;
    }
    if saida == 0 {
        aviso_em(0xa7a088, 0x39);
        return 0;
    }
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let lado: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4afa80usize);
    let (mut t0, mut t1) = ([0u8; 8], [0u8; 8]);
    let (p0, p1) = (t0.as_mut_ptr() as usize, t1.as_mut_ptr() as usize);
    let d = data_nova(p0, cpz7_w(jogo + 0x2a) as u32, cpz7_w(jogo + 0x28) as u32);
    copiar(saida, p1, d);
    let s = |o: usize| (saida + o) as *mut u8;
    core::ptr::write_unaligned(s(8) as *mut u32, cpz7_d(jogo));
    core::ptr::write_unaligned(s(0xc) as *mut u16, cpz7_w(jogo + 0x32));
    core::ptr::write_unaligned(s(0xe) as *mut u16, cpz7_w(jogo + 0x30));
    *s(0x10) = lado(clube, jogo);
    core::ptr::write_unaligned(s(0x11) as *mut u32, cpz7_d(clube));
    core::ptr::write_unaligned(s(0x1b) as *mut u32, cpz7_d(jogo + 8));
    if clube as u32 == cpz7_d(jogo + 0x1c) { // cpz7: o clube é o mandante
        core::ptr::write_unaligned(s(0x15) as *mut u32, cpz7_d(jogo + 0x10));
        if cpz7_b(jogo + 0x47) as i8 > -1 {
            *s(0x19) = cpz7_b(jogo + 0x47);
            *s(0x1a) = cpz7_b(jogo + 0x48);
        } else {
            crate::comparacao::ramo("cpz_7 placar do mandante");
            *s(0x19) = cpz7_b(jogo + 0x43); // cpz7: os gols do mandante
            *s(0x1a) = cpz7_b(jogo + 0x44);
        }
    } else {
        core::ptr::write_unaligned(s(0x15) as *mut u32, cpz7_d(jogo + 0xc));
        if cpz7_b(jogo + 0x48) as i8 > -1 {
            *s(0x19) = cpz7_b(jogo + 0x48);
            *s(0x1a) = cpz7_b(jogo + 0x47);
        } else {
            crate::comparacao::ramo("cpz_7 placar do visitante");
            *s(0x19) = cpz7_b(jogo + 0x44);
            *s(0x1a) = cpz7_b(jogo + 0x43); // cpz7: os gols sofridos pelo visitante
        }
    }
    1
}

/// Comparador da 0x7e4750: lado a lado (leve) no registro de saída (0x1f bytes) e na devolução; ponteiro nulo: a caixa,
/// só a original.
unsafe extern "C" fn comparar_registro_da_partida_pelo_clube(jogo: usize, clube: usize, saida: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, usize) -> u32>(1398);
    if jogo == 0 || clube == 0 || saida == 0 {
        return o(jogo, clube, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(saida, 0x1f)], || o(jogo, clube, saida), || registro_da_partida_pelo_clube(jogo, clube, saida));
    PLACAR.registrar(igual && r == n, || format!("registro_da_partida_pelo_clube(jogo {:x}, clube {:x}) original={} rust={} {:x?} x {:x?}", jogo, clube, r, n, mo, mr));
    r
}

/// O id do nome (+0x33 do registro de nome) ou −1 sem nome.
unsafe fn cpz7_nome(p: u32) -> u32 {
    if p != 0 { cpz7_d(p as usize + 0x33) } else { u32::MAX }
}

/// { 0x7e4920 } (cdecl, record_utils.cpp): `fill_staff_name_ref(pessoa, saida)`, dos recordes do clube e da competição,
/// da notícia da suspensão, do prêmio e da busca de jogadores. Pessoa ou saída nulas: a caixa (linhas 0x80, 0x88) e 0.
/// Senão +0, +4, +8 os ids dos nomes (+4, +8, +0xc da pessoa; −1 sem nome), +0xc o id da pessoa e +0x10 a data de
/// nascimento (+0x10, 0x417c10); devolve 1.
unsafe extern "C" fn referencia_ao_nome(pessoa: usize, saida: usize) -> u32 {
    if pessoa == 0 {
        aviso_em(0xa7a088, 0x80);
        return 0;
    }
    if saida == 0 {
        aviso_em(0xa7a088, 0x88);
        return 0;
    }
    for k in 0..3 {
        core::ptr::write_unaligned((saida + 4 * k) as *mut u32, cpz7_nome(cpz7_d(pessoa + 4 + 4 * k)));
    }
    core::ptr::write_unaligned((saida + 0xc) as *mut u32, cpz7_d(pessoa));
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copiar(saida + 0x10, t.as_mut_ptr() as usize, pessoa + 0x10);
    1
}

/// Comparador da 0x7e4920: lado a lado (leve) na referência (0x18 bytes) e na devolução; ponteiro nulo: só a original.
unsafe extern "C" fn comparar_referencia_ao_nome(pessoa: usize, saida: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(1399);
    if pessoa == 0 || saida == 0 {
        return o(pessoa, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(saida, 0x18)], || o(pessoa, saida), || referencia_ao_nome(pessoa, saida));
    PLACAR.registrar(igual && r == n, || format!("referencia_ao_nome(pessoa {:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

/// { 0x7e56d0 } (cdecl, record_utils.cpp): `match_record_result_code(registro)`, da estatística da competição
/// (0x49f570, a cada partida). Registro nulo: a caixa (linha 0x25a) e 0. Pró (+0x19) > contra (+0x1a), com sinal: 3;
/// empate com gols: 2; empate sem gols: 1; derrota: −3 (só o byte conta: quem chama lê al).
unsafe extern "C" fn codigo_do_resultado(registro: usize) -> u8 {
    if registro == 0 {
        aviso_em(0xa7a088, 0x25a);
        return 0;
    }
    let (p, c) = (cpz7_b(registro + 0x19) as i8, cpz7_b(registro + 0x1a) as i8);
    if p > c {
        3
    } else if p == c && p > 0 {
        2
    } else if p == c {
        1
    } else {
        (-3i8) as u8
    }
}

/// Comparador da 0x7e56d0: pelo byte devolvido (só lê); registro nulo: só a original.
unsafe extern "C" fn comparar_codigo_do_resultado(registro: usize) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize) -> u8>(1400);
    if registro == 0 {
        return o(registro);
    }
    let r = o(registro);
    let n = codigo_do_resultado(registro);
    PLACAR.registrar(r == n, || format!("codigo_do_resultado(registro {:x}) original={} rust={}", registro, r as i8, n as i8));
    r
}

// ---------- 262a leva: os jogos remarcados: os jogos vizinhos do clube movidos um dia, os jogos perto da data da seleção e a notícia do jogo remarcado (fix_man.cpp) ----------

unsafe fn cpz8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz8_u(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpz8_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpz8_sw(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i16) as i32
}

unsafe fn cpz8_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}

/// LLIST (0xe bytes: +0 conta i16, +2 cabeça, +6 cauda, +0xa cursor; nó: +0 dados, +6 próximo): o construtor
/// (0x68d020), o destrutor (0x68d040), o esvaziar (0x68d080), o cursor na cabeça e o próximo (0x68d1b0).
unsafe fn cpz8_lista_nova(l: usize) {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    f(l);
}

unsafe fn cpz8_lista_fim(l: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    f(l);
}

unsafe fn cpz8_lista_limpar(l: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    f(l);
}

unsafe fn cpz8_inicio(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut u32, cpz8_u(l + 2));
}

unsafe fn cpz8_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    f(l)
}

/// A competição do jogo é amistosa? (0x5c6710: thiscall em [0xae2c74], 1 argumento.)
unsafe fn cpz8_amistosa(comp: u32) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5c6710usize);
    f(cpz8_d(0xae2c74), comp) != 0
}

/// As datas (Date, 8 bytes): construir por (dia, ano) (0x549ef0), menos e mais n dias (0x54c8e0, 0x54c5b0: thiscall
/// (data, saída, n), devolvem a saída) e a atribuição (0x417c10: thiscall(destino, temporária, origem)).
unsafe fn cpz8_data(d: *mut u8, dia: u32, ano: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    f(d as usize, dia, ano)
}

unsafe fn cpz8_menos(d: usize, saida: *mut u8, n: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    f(d, saida as usize, n)
}

unsafe fn cpz8_mais(d: usize, saida: *mut u8, n: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    f(d, saida as usize, n)
}

unsafe fn cpz8_atribuir(destino: *mut u8, origem: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut temporaria = [0u8; 8];
    f(destino as usize, temporaria.as_mut_ptr() as usize, origem);
}

/// Os jogos do clube entre duas datas (0x5acbe0, thiscall, 9 argumentos): a original empurra os dwords em +0 e +2 de
/// cada data (só a palavra baixa conta), o clube, a lista, comp −1, tipo −2 e lado 1.
unsafe fn cpz8_achar(this: usize, a: *const u8, b: *const u8, clube: u32, lista: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x5acbe0usize);
    let (a, b) = (a as usize, b as usize);
    f(this, cpz8_u(a), cpz8_u(a + 2), cpz8_u(b), cpz8_u(b + 2), clube, lista, u32::MAX, 0xffff_fffe, 1)
}

/// Tirar o jogo do calendário (0x5abc30), gravar a data (0x5ab510), juntar (0x5aa680) e a notícia (0x5aac70).
unsafe fn cpz8_tirar(this: usize, jogo: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5abc30usize);
    f(this, jogo)
}

unsafe fn cpz8_juntar(this: usize, jogo: usize, zerar: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    f(this, jogo, zerar);
}

unsafe fn cpz8_noticia(this: usize, jogo: usize, causa: usize, velha: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x5aac70usize);
    f(this, jogo, causa, velha);
}

/// A data nova do jogo `f2` tem outro jogo? (0x5ad780: thiscall(jogo, &gravidade, 0) devolve uma LLIST alocada;
/// percorre e acha um item com bytes diferentes do jogo original `f1`; depois destrói e libera, 0x944c94.)
unsafe fn cpz8_conflito(this: usize, f2: *mut u8, f1: *const u8) -> bool {
    let achar: unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize = core::mem::transmute(0x5ad780usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let mut gravidade = 0i32;
    let l = achar(this, f2 as usize, &mut gravidade, 0);
    if l == 0 {
        return false;
    }
    cpz8_inicio(l);
    let mut outro = false;
    loop {
        let q = cpz8_proximo(l);
        if q == 0 {
            break;
        }
        if core::slice::from_raw_parts(f1, 0x4f) != core::slice::from_raw_parts(q as *const u8, 0x4f) {
            outro = true;
            break;
        }
    }
    cpz8_lista_fim(l);
    liberar(l);
    outro
}

/// O primeiro jogo da lista copiado para `f1` e `f2` (0x4f bytes).
unsafe fn cpz8_primeiro(l: usize, f1: &mut [u8; 0x4f], f2: &mut [u8; 0x4f]) {
    cpz8_inicio(l);
    let p = cpz8_proximo(l);
    core::ptr::copy_nonoverlapping(p as *const u8, f1.as_mut_ptr(), 0x4f);
    *f2 = *f1;
}

/// { 0x5ae9d0 } (thiscall, ret 4, fix_man.cpp, SEH da lista local e das datas): `FIXTURE_MANAGER::
/// move_nearby_club_fixtures(jogo)`, pela junção do jogo (0x5aa680). Jogo de competição amistosa (+0x14, 0x5c6710):
/// nada. Para os dois clubes (+0x1c, +0x20) de nação (+0x53) com a marca +0x11c: o primeiro jogo do clube dois dias
/// antes (0x5acbe0, lado 1), não amistoso e com +0x43 = 0xff, vai para um dia antes dele; o primeiro jogo dois dias
/// depois, não amistoso, vai para um dia depois. Só se a data nova não tiver outro jogo (0x5ad780): tira (0x5abc30),
/// junta (0x5aa680, 0) e dá a notícia do jogo remarcado (0x5aac70, com o jogo como causa).
unsafe extern "thiscall" fn mover_jogos_vizinhos_do_clube(this: usize, jogo: usize) {
    let iniciar_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mut d = [0u8; 8];
    iniciar_data(d.as_mut_ptr() as usize);
    let mut l = [0u8; 0xe];
    let lp = l.as_mut_ptr() as usize;
    cpz8_lista_nova(lp);
    if cpz8_amistosa(cpz8_u(jogo + 0x14)) {
        cpz8_lista_fim(lp);
        return;
    }
    let mut f1 = [0u8; 0x4f];
    let mut f2 = [0u8; 0x4f];
    for k in 0..2 {
        let clube = cpz8_d(jogo + if k == 0 { 0x1c } else { 0x20 });
        let nacao = cpz8_d(clube + 0x53);
        if nacao == 0 || *((nacao + 0x11c) as *const u8) == 0 {
            continue;
        }
        // dois dias antes
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = cpz8_data(t.as_mut_ptr(), cpz8_w(jogo + 0x2a) as u32, cpz8_w(jogo + 0x28) as u32);
        let r = cpz8_menos(r, u.as_mut_ptr(), 2);
        cpz8_atribuir(d.as_mut_ptr(), r);
        cpz8_lista_limpar(lp);
        if cpz8_achar(this, d.as_ptr(), d.as_ptr(), cpz8_u(clube), lp) as u16 != 0 {
            cpz8_primeiro(lp, &mut f1, &mut f2);
            if !cpz8_amistosa(cpz8_u(f1.as_ptr() as usize + 0x14)) {
                crate::comparacao::ramo("cpz_8 jogo dois dias antes");
                if f2[0x43] == 0xff { // cpz8: o jogo de dois dias antes sem a marca
                    let mut v = [0u8; 8];
                    let r = cpz8_menos(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia antes
                    cpz8_atribuir(d.as_mut_ptr(), r);
                    f2[0x2a..0x2c].copy_from_slice(&d[0..2]);
                    f2[0x28..0x2a].copy_from_slice(&d[2..4]);
                    if !cpz8_conflito(this, f2.as_mut_ptr(), f1.as_ptr()) {
                        crate::comparacao::ramo("cpz_8 jogo antecipado");
                        cpz8_tirar(this, f1.as_ptr() as usize);
                        cpz8_juntar(this, f2.as_mut_ptr() as usize, 0);
                        cpz8_noticia(this, f2.as_mut_ptr() as usize, jogo, 0);
                    }
                }
            }
        }
        // dois dias depois
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = cpz8_data(t.as_mut_ptr(), cpz8_w(jogo + 0x2a) as u32, cpz8_w(jogo + 0x28) as u32);
        let r = cpz8_mais(r, u.as_mut_ptr(), 2);
        cpz8_atribuir(d.as_mut_ptr(), r);
        cpz8_lista_limpar(lp);
        if cpz8_achar(this, d.as_ptr(), d.as_ptr(), cpz8_u(clube), lp) as u16 == 0 {
            continue;
        }
        cpz8_primeiro(lp, &mut f1, &mut f2);
        if cpz8_amistosa(cpz8_u(f1.as_ptr() as usize + 0x14)) {
            continue;
        }
        let mut v = [0u8; 8];
        let r = cpz8_mais(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia depois
        cpz8_atribuir(d.as_mut_ptr(), r);
        f2[0x2a..0x2c].copy_from_slice(&d[0..2]);
        f2[0x28..0x2a].copy_from_slice(&d[2..4]);
        if !cpz8_conflito(this, f2.as_mut_ptr(), f1.as_ptr()) {
            crate::comparacao::ramo("cpz_8 jogo adiado");
            cpz8_tirar(this, f1.as_ptr() as usize);
            cpz8_juntar(this, f2.as_mut_ptr() as usize, 0);
            cpz8_noticia(this, f2.as_mut_ptr() as usize, jogo, 0);
        }
    }
    cpz8_lista_fim(lp);
}

// ponytail: move jogos no calendário (tira, junta, aloca e libera listas, notícia): só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_mover_jogos_vizinhos_do_clube(this: usize, jogo: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1401)(this, jogo)
}

/// { 0x5abb10 } (thiscall, ret 8, fix_man.cpp, SEH da lista local): `FIXTURE_MANAGER::
/// reschedule_club_fixtures_near_date(clube, data)`, pelas convocações das seleções (0x777940: clube com mais de dois
/// convocados). Os jogos do clube (o id em `*clube`) de um dia antes a um dia depois da data (0x5acbe0, lado 1); cada
/// um que sai do calendário (0x5abc30) ganha data nova (0x5ab510), volta (0x5aa680, 1) e dá a notícia com a data velha
/// (0x5aac70, sem causa).
unsafe extern "thiscall" fn remarcar_jogos_perto_da_data(this: usize, clube: usize, data: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5ab510usize);
    let mut l = [0u8; 0xe];
    let lp = l.as_mut_ptr() as usize;
    cpz8_lista_nova(lp);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    cpz8_menos(data, a.as_mut_ptr(), 1);
    cpz8_mais(data, b.as_mut_ptr(), 1);
    cpz8_achar(this, a.as_ptr(), b.as_ptr(), cpz8_u(clube), lp);
    cpz8_inicio(lp);
    loop {
        let p = cpz8_proximo(lp);
        if p == 0 {
            break;
        }
        let mut velha = [0u8; 8];
        cpz8_data(velha.as_mut_ptr(), cpz8_w(p + 0x2a) as u32, cpz8_w(p + 0x28) as u32);
        if cpz8_tirar(this, p) != 0 {
            crate::comparacao::ramo("cpz_8 jogo remarcado pela selecao");
            gravar(this, p);
            cpz8_juntar(this, p, 1);
            cpz8_noticia(this, p, 0, velha.as_ptr() as usize);
        }
    }
    cpz8_lista_fim(lp);
}

// ponytail: move jogos no calendário e dá a notícia (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_remarcar_jogos_perto_da_data(this: usize, clube: usize, data: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1402)(this, clube, data)
}

/// O ensaio da notícia: a cópia monta a notícia nova aqui em vez de pô-la na fila (e não cria a fila).
static mut CPZ8_ENSAIO: bool = false;
static mut CPZ8_NOTICIA: [u8; 0xe0] = [0; 0xe0];
static mut CPZ8_MONTADA: bool = false;

unsafe fn cpz8_dado(n: usize, k: u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x785830usize);
    f(n, k)
}

/// A notícia já na fila para o mesmo jogo (dados 0, 1, 2, 0xe e 0xf: +0xc, +0x10, +0, +0x40 e +0x41 do jogo).
unsafe fn cpz8_na_fila(l: usize, f: usize) -> usize {
    cpz8_inicio(l);
    loop {
        let it = cpz8_proximo(l);
        if it == 0 {
            return 0;
        }
        if cpz8_dado(it, 0) == cpz8_u(f + 0xc)
            && cpz8_dado(it, 1) == cpz8_u(f + 0x10)
            && cpz8_dado(it, 2) == cpz8_u(f)
            && cpz8_dado(it, 0xe) as i32 == cpz8_sb(f + 0x40)
            && cpz8_dado(it, 0xf) as i32 == cpz8_sb(f + 0x41)
        {
            return it;
        }
    }
}

/// { 0x5aac70 } (thiscall, ret 0xc, fix_man.cpp, SEH da notícia local): `FIXTURE_MANAGER::queue_fixture_moved_news(jogo,
/// causa, data velha)`, pelas remarcações (0x5aa680, 0x5abb10, 0x5ae380, 0x5ae9d0). A fila em +0x1128 (criada na
/// primeira vez: 0x944cf1 e 0x68d020): se já há a notícia do mesmo jogo, só troca a data nova (dados 5 e 6); senão a
/// notícia 1: o jogo (+0xc, +0x10, +0, +0x32, +0x30, a data nova +0x2a/+0x28, +0x40, +0x41) e a causa (os mesmos
/// campos dela nos dados 7 a 0xd) ou, sem causa, −1 e a data velha (dados 0xc e 0xd); entra na fila (0x68d0d0, 0xdf).
unsafe extern "thiscall" fn noticia_do_jogo_remarcado(this: usize, f: usize, causa: usize, velha: usize) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    if cpz8_d(this + 0x1128) == 0 && !CPZ8_ENSAIO {
        let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
        let m = novo(0xe);
        if m != 0 {
            cpz8_lista_nova(m);
        }
        *((this + 0x1128) as *mut usize) = m;
    }
    let l = cpz8_d(this + 0x1128);
    if l != 0 {
        let it = cpz8_na_fila(l, f);
        if it != 0 {
            gravar(it, 5, cpz8_w(f + 0x2a) as u32);
            gravar(it, 6, cpz8_w(f + 0x28) as u32);
            return;
        }
    }
    crate::comparacao::ramo("cpz_8 noticia do jogo remarcado");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 1, 0);
    let campos = |nt: usize, k: i8, j: usize| {
        gravar(nt, k, cpz8_u(j + 0xc));
        gravar(nt, k + 1, cpz8_u(j + 0x10));
        gravar(nt, k + 2, cpz8_u(j));
        gravar(nt, k + 3, cpz8_sw(j + 0x32) as u32);
        gravar(nt, k + 4, cpz8_sw(j + 0x30) as u32);
        gravar(nt, k + 5, cpz8_w(j + 0x2a) as u32);
    };
    campos(nt, 0, f);
    gravar(nt, 6, cpz8_w(f + 0x28) as u32);
    gravar(nt, 0xe, cpz8_sb(f + 0x40) as u32);
    gravar(nt, 0xf, cpz8_sb(f + 0x41) as u32);
    if causa != 0 {
        campos(nt, 7, causa);
        gravar(nt, 0xd, cpz8_w(causa + 0x28) as u32);
    } else {
        gravar(nt, 7, u32::MAX);
        gravar(nt, 0xc, cpz8_sw(velha) as u32);
        gravar(nt, 0xd, cpz8_w(velha + 2) as u32);
    }
    if CPZ8_ENSAIO {
        core::ptr::copy_nonoverlapping(nt as *const u8, core::ptr::addr_of_mut!(CPZ8_NOTICIA) as *mut u8, 0xe0);
        CPZ8_MONTADA = true;
        return;
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    copiar(cpz8_d(this + 0x1128), nt, 0xdf);
}

/// O comparador da notícia, em ensaio. Já na fila: lado a lado no cabeçalho da fila e na notícia achada (a data nova).
/// Nova: a cópia monta a notícia sem pô-la na fila e a original roda; a fila tem de crescer um e o último item (a cauda,
/// +6, nó +0) ser a notícia da cópia nos 0xde bytes que o construtor (0x77bbe0) grava; o último dos 0xdf copiados é
/// sobra de pilha na original (a cópia leva zero).
unsafe extern "thiscall" fn comparar_noticia_do_jogo_remarcado(this: usize, f: usize, causa: usize, velha: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1403);
    let l = cpz8_d(this + 0x1128);
    let alvo = if l != 0 { cpz8_na_fila(l, f) } else { 0 };
    if alvo != 0 {
        let regioes = [(this + 0x1128, 4), (l, 0xe), (alvo, 0xdf)];
        let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, f, causa, velha), || noticia_do_jogo_remarcado(this, f, causa, velha));
        PLACAR.registrar(igual, || format!("noticia_do_jogo_remarcado(jogo {:#x}, na fila) original={:x?} rust={:x?}", f, mo, mr));
        return;
    }
    let mut regioes = vec![(this + 0x1128, 4)];
    if l != 0 {
        regioes.push((l, 0xe));
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    let conta = if l != 0 { cpz8_sw(l) } else { 0 };
    CPZ8_ENSAIO = true;
    CPZ8_MONTADA = false;
    crate::aleatorio::pausar_rastro(|| noticia_do_jogo_remarcado(this, f, causa, velha));
    CPZ8_ENSAIO = false;
    antes.escrever();
    o(this, f, causa, velha);
    let l2 = cpz8_d(this + 0x1128);
    let montada = CPZ8_MONTADA;
    let igual = montada
        && l2 != 0
        && cpz8_sw(l2) == conta + 1
        && cpz8_d(cpz8_d(l2 + 6)) != 0
        && core::slice::from_raw_parts(cpz8_d(cpz8_d(l2 + 6)) as *const u8, 0xde)
            == core::slice::from_raw_parts(core::ptr::addr_of!(CPZ8_NOTICIA) as *const u8, 0xde);
    PLACAR.registrar(igual, || format!("noticia_do_jogo_remarcado(jogo {:#x}, nova) montada={} conta {} -> {}", f, montada, conta, if l2 != 0 { cpz8_sw(l2) } else { -1 }));
}

// ---------- 262a leva: a forma ruim do jogador e as notícias das multas: o incidente de forma com a multa da IA, a reação à multa e o retorno do treinador (awol.cpp, Fine.cpp) ----------

unsafe fn cpz9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz9_u(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpz9_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cpz9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

unsafe fn cpz9_gravar(n: usize, k: i8, v: u32) {
    let f: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    f(n, k, v);
}

/// O id e as três chaves (+0x33 de +4, +8, +0xc) da pessoa `p` nos dados `k`..`k + 3`.
unsafe fn cpz9_pessoa_nos_dados(n: usize, k: i8, p: usize) {
    cpz9_gravar(n, k, cpz9_u(p));
    cpz9_gravar(n, k + 1, cpz9_u(cpz9_d(p + 4) + 0x33));
    cpz9_gravar(n, k + 2, cpz9_u(cpz9_d(p + 8) + 0x33));
    cpz9_gravar(n, k + 3, cpz9_u(cpz9_d(p + 0xc) + 0x33));
}

/// Um humano dirige o clube? (news_club_has_human_manager 0x7875e0: stdcall, 1 arg; a original deixa o News em ecx.)
unsafe fn cpz9_clube_humano(clube: usize) -> bool {
    let f: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(clube) != 0
}

/// O ensaio das notícias: a cópia para no ponto em que montaria a notícia (que sorteia e vai às caixas dos técnicos).
static mut CPZ9_ENSAIO: bool = false;
static mut CPZ9_MONTARIA: bool = false;

/// { 0x41add0 } (thiscall, ret 8, awol.cpp, SEH da data e da notícia local, sem destrutor): `AWOL::
/// report_poor_fitness(jogador, clube)`, pelo diário das pessoas (0x5ce2d0: a queda de condição grande do dia). O
/// registro (jogador, clube, hoje e um byte que na original é sobra de pilha) entra na lista dos incidentes ([this+8],
/// 0x68d0d0, 0x11 bytes). Clube de humano (0x7875e0): um treinador das 5 vagas (+0x19f, sorteio(5), até achar um ou
/// sorteio(20) dar zero) e a notícia 0x138f ao clube (o jogador, sorteio(2), as chaves dele e do treinador ou −1).
/// Clube da IA (0x600a50) com técnico (+0xcf, +0x69): sorteio(+0x14 dele) acima de sorteio(10) leva à multa (0x5a5980,
/// prazo 1 se sorteio(nota 0x544410 com o +0x19) passar de sorteio(5), senão 2; motivo 1).
unsafe extern "thiscall" fn incidente_de_forma(this: usize, jogador: usize, clube: usize) {
    let iniciar_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mut reg = [0u8; 0x11];
    iniciar_data(reg.as_mut_ptr() as usize + 8);
    if jogador == 0 || clube == 0 {
        return;
    }
    reg[..4].copy_from_slice(&cpz9_u(jogador).to_le_bytes());
    reg[4..8].copy_from_slice(&cpz9_u(clube).to_le_bytes());
    let atribuir: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut temporaria = [0u8; 8];
    atribuir(reg.as_mut_ptr() as usize + 8, temporaria.as_mut_ptr() as usize, 0xae2c90);
    // ponytail: o byte +0x10 do registro é sobra de pilha na original; a cópia grava 0 (ninguém o lê)
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    acrescentar(cpz9_d(this + 8), reg.as_ptr() as usize, 0x11);
    if cpz9_clube_humano(clube) {
        let mut treinador;
        loop {
            let r = cpz9_sortear(5);
            treinador = cpz9_d(clube.wrapping_add((r as usize).wrapping_mul(4)) + 0x19f);
            if cpz9_sortear(0x14) == 0 || treinador != 0 {
                break;
            }
        }
        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
        let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
        let nt = local.as_mut_ptr() as usize;
        construir(nt, 0x138f, 0);
        cpz9_gravar(nt, 0, cpz9_u(jogador));
        cpz9_gravar(nt, 1, cpz9_u(clube));
        let r = cpz9_sortear(2);
        cpz9_gravar(nt, 2, r as u32);
        cpz9_gravar(nt, 3, cpz9_u(cpz9_d(jogador + 4) + 0x33));
        cpz9_gravar(nt, 4, cpz9_u(cpz9_d(jogador + 8) + 0x33));
        cpz9_gravar(nt, 5, cpz9_u(cpz9_d(jogador + 0xc) + 0x33));
        if treinador != 0 {
            cpz9_pessoa_nos_dados(nt, 6, treinador);
        } else {
            for k in 6..10 {
                cpz9_gravar(nt, k, u32::MAX);
            }
        }
        entregar(cpz9_d(0xb7431c), nt, clube);
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 0, 1, 0, 0) != 0 {
        return;
    }
    let tecnico = cpz9_d(clube + 0xcf);
    if tecnico == 0 {
        return;
    }
    let np = cpz9_d(tecnico + 0x69);
    if np == 0 {
        return;
    }
    crate::comparacao::ramo("cpz_9 multa avaliada");
    let a = cpz9_sortear(cpz9_sb(np + 0x14));
    let b = cpz9_sortear(10); // cpz9: o sorteio da multa
    if a <= b {
        return;
    }
    let np = cpz9_d(cpz9_d(clube + 0xcf) + 0x69);
    crate::comparacao::ramo("cpz_9 multa pela forma");
    let nota: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(0x544410usize);
    // ponytail: a original empurra ecx com o byte +0x19 só no cl (os altos são sobra); a 0x544410 só lê o byte
    let c =cpz9_sortear(nota(np, *((np + 0x19) as *const u8) as u32) as i32);
    let e = cpz9_sortear(5); // cpz9: o sorteio do prazo
    let prazo = if c > e { 1 } else { 2 }; // cpz9: o prazo da multa
    let multar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x5a5980usize);
    multar(cpz9_d(0xae2a3c), jogador, prazo, 1);
}

/// O comparador do incidente: a original roda sozinha (põe o registro na lista, dá a notícia e multa) e confere o
/// plano: jogador e clube não nulos põem logo depois da cauda de antes (ou na cabeça, lista vazia) o registro com o
/// jogador, o clube e a data de hoje (16 bytes; o byte +0x10 é sobra de pilha).
unsafe extern "thiscall" fn comparar_incidente_de_forma(this: usize, jogador: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1404);
    let l = cpz9_d(this + 8);
    if jogador == 0 || clube == 0 || l == 0 {
        return o(this, jogador, clube);
    }
    let cauda = cpz9_d(l + 6);
    let mut plano = [0u8; 0x10];
    plano[..4].copy_from_slice(&cpz9_u(jogador).to_le_bytes());
    plano[4..8].copy_from_slice(&cpz9_u(clube).to_le_bytes());
    plano[8..].copy_from_slice(core::slice::from_raw_parts(0xae2c90 as *const u8, 8));
    o(this, jogador, clube);
    let no = if cauda != 0 { cpz9_d(cauda + 6) } else { cpz9_d(l + 2) };
    let dados = if no != 0 { cpz9_d(no) } else { 0 };
    let igual = dados != 0 && core::slice::from_raw_parts(dados as *const u8, 0x10) == plano;
    PLACAR.registrar(igual, || format!("incidente_de_forma(jogador {:#x}, clube {:#x}) plano={:x?} lista: no {:#x} dados {:#x}", jogador, clube, plano, no, dados));
}

/// { 0x5a74b0 } (stdcall, ret 0x14, Fine.cpp, SEH da notícia local, sem destrutor): `post_fine_reaction_news(jogador,
/// motivo, reação, prazo, efeitos)`, pela multa (0x5a5980, cinco chamadas). Sem jogador, sem contrato (0x4cf5a0, em
/// [0xae1080]), sem o clube dele ou sem o contrato do clube (0x4cf5f0): nada; jogador de clube (+0x39) sem humano
/// (0x7875e0): nada. A notícia 0xbc6 ao clube do contrato: o jogador (id e chaves), motivo, reação, sorteio(2), a
/// personalidade (+0x5d + +0x5b abaixo de 12, +0x59 abaixo de 6 e reputação +0xb acima de 5250), prazo 1, o contrato
/// de tipo 7 (+0x4f), a transferência futura (0x8cca10), os efeitos e o jogo pelo tipo (+0x4f 1 com mais de 23 anos,
/// ou 2).
unsafe extern "stdcall" fn noticia_reacao_a_multa(jogador: usize, motivo: u32, reacao: u32, prazo: u32, efeitos: u32) {
    if jogador == 0 {
        return;
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let k = contrato_de(cpz9_d(0xae1050), jogador);
    *(0xae1080 as *mut usize) = k;
    if k == 0 {
        return;
    }
    let clube = cpz9_d(0xae23b4).wrapping_add(cpz9_d(k + 4).wrapping_mul(0x245));
    if clube == 0 {
        return;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let c = contrato(cpz9_d(0xae1050), jogador, clube);
    if c == 0 {
        return;
    }
    let atual = cpz9_d(jogador + 0x39);
    if atual != 0 && !cpz9_clube_humano(atual) {
        return;
    }
    crate::comparacao::ramo("cpz_9 noticia da reacao a multa");
    if CPZ9_ENSAIO {
        CPZ9_MONTARIA = true;
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let futura: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8cca10usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xbc6, 0);
    cpz9_pessoa_nos_dados(nt, 0, jogador);
    cpz9_gravar(nt, 4, motivo as i8 as u32);
    cpz9_gravar(nt, 5, reacao as i8 as u32);
    let r = cpz9_sortear(2);
    cpz9_gravar(nt, 6, r as u32);
    let p = cpz9_sb(jogador + 0x5d) + cpz9_sb(jogador + 0x5b) < 0xc
        && cpz9_sb(jogador + 0x59) < 6
        && core::ptr::read_unaligned((cpz9_d(jogador + 0x61) + 0xb) as *const i16) > 0x1482;
    cpz9_gravar(nt, 7, p as u32);
    cpz9_gravar(nt, 8, (prazo as u8 == 1) as u32);
    let tipo = *((c + 0x4f) as *const u8) & 0xf0;
    cpz9_gravar(nt, 9, (tipo == 0x70) as u32);
    cpz9_gravar(nt, 0xa, (futura(cpz9_d(0xde1f54), jogador, 0) != 0) as u32);
    cpz9_gravar(nt, 0xb, efeitos as i8 as u32);
    let jogo = (cpz9_sb(jogador + 0x18) > 0x17 && tipo == 0x10) || tipo == 0x20;
    cpz9_gravar(nt, 0xc, jogo as u32);
    entregar(cpz9_d(0xb7431c), nt, clube);
}

/// O comparador em ensaio: a cópia para antes da notícia; a original roda a partir do mesmo estado ([0xae1080] e
/// sorteio). Sem notícia, tudo igual; com notícia (que sorteia e vai às caixas), só [0xae1080].
unsafe extern "stdcall" fn comparar_noticia_reacao_a_multa(jogador: usize, motivo: u32, reacao: u32, prazo: u32, efeitos: u32) {
    let o = orig::<unsafe extern "stdcall" fn(usize, u32, u32, u32, u32)>(1405);
    let mut regioes = vec![(0xae1080, 4)];
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPZ9_ENSAIO = true;
    CPZ9_MONTARIA = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| noticia_reacao_a_multa(jogador, motivo, reacao, prazo, efeitos)));
    CPZ9_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(jogador, motivo, reacao, prazo, efeitos);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let montaria = CPZ9_MONTARIA;
    let igual = if montaria { mo.0[0].1 == mr.0[0].1 } else { mo == mr };
    PLACAR.registrar(igual, || format!("noticia_reacao_a_multa(jogador {:#x}) montaria={} original={:x?} rust={:x?}", jogador, montaria, mo, mr));
}

/// { 0x5a7f20 } (stdcall, ret 0xc, Fine.cpp, SEH da notícia local, sem destrutor): `post_coach_fine_feedback_news
/// (jogador, treinador, desfecho)`, pelo diário das multas (0x5a5820: o retorno marcado vencido, mais de 30 dias de
/// casa e sem lesão grave; a original deixa o Fine em ecx). Os dois com ficha (+0x61) e nação (+0x1a), o jogador com
/// clube (+0x39): a notícia 0x1394 (o jogador e o treinador, id e chaves; o desfecho; sorteio(5)) ao clube do jogador.
unsafe extern "stdcall" fn noticia_retorno_do_treinador(jogador: usize, treinador: usize, desfecho: u32) {
    if jogador == 0 || cpz9_d(jogador + 0x61) == 0 || cpz9_d(jogador + 0x1a) == 0 || cpz9_d(jogador + 0x39) == 0 {
        return;
    }
    if treinador == 0 || cpz9_d(treinador + 0x61) == 0 || cpz9_d(treinador + 0x1a) == 0 {
        return;
    }
    crate::comparacao::ramo("cpz_9 noticia do retorno do treinador");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0x1394, 0);
    cpz9_pessoa_nos_dados(nt, 0, jogador);
    cpz9_pessoa_nos_dados(nt, 4, treinador);
    cpz9_gravar(nt, 8, desfecho as i8 as u32);
    let r = cpz9_sortear(5);
    cpz9_gravar(nt, 9, r as u32);
    entregar(cpz9_d(0xb7431c), nt, cpz9_d(jogador + 0x39));
}

// ponytail: a notícia sorteia e vai às caixas dos técnicos (aloca): só a original, com a cópia levando o ramo; sem
// notícia as duas não fazem nada.
unsafe extern "stdcall" fn comparar_noticia_retorno_do_treinador(jogador: usize, treinador: usize, desfecho: u32) {
    orig::<unsafe extern "stdcall" fn(usize, usize, u32)>(1406)(jogador, treinador, desfecho)
}

// ---------- 262a leva: os veículos da imprensa: o comentarista sorteado, o nome do veículo e a notícia da queixa do técnico à federação (printouts.cpp, media.cpp) ----------

unsafe fn cpz10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz10_u(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpz10_sw(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i16) as i32
}

unsafe fn cpz10_inicio(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut u32, cpz10_u(l + 2));
}

unsafe fn cpz10_proximo(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    f(l)
}

/// { 0x7dab90 } (thiscall, ret 0xc, printouts.cpp, quadro de 0x1094 bytes pela 0x945610): `PunditList::
/// pick_random_pundit(clube, tipo, nação)`, pelas histórias da imprensa (0x72c1f0, 0x733a60) e pelos textos das
/// notícias. Tipo 1 no domingo (0x54a3f0 = 6) vira 7. Tipos 2 e 4 (locais): os da lista [this+4] cujo +0xcb é o id de
/// [[clube+0x69]+0x38] (sem eles: −1); os outros: os da lista [this] do tipo (+0xca) e da nação (+0xcf; −1 aceita
/// qualquer). Até 20; nenhum: −1; senão o id (palavra +0, com sinal) de um deles por sorteio(n).
unsafe extern "thiscall" fn sortear_comentarista(this: usize, clube: usize, tipo: u32, nacao: i32) -> i32 {
    let mut t = tipo as i8;
    if t == 1 {
        let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
        if semana(0xae2c90) as i16 == 6 {
            t = 7;
        }
    }
    let mut ids = [0i16; 20];
    let mut n = 0usize;
    if t == 2 || t == 4 {
        let est = cpz10_d(clube + 0x69);
        if est == 0 || cpz10_d(est + 0x38) == 0 {
            return -1;
        }
        let l = cpz10_d(this + 4);
        cpz10_inicio(l);
        if cpz10_sw(l) <= 0 {
            return -1;
        }
        let mut i = 0;
        loop {
            let p = cpz10_proximo(l);
            if p == 0 {
                break;
            }
            if cpz10_u(p + 0xcb) == cpz10_u(cpz10_d(cpz10_d(clube + 0x69) + 0x38)) { // cpz10: o comentarista local
                crate::comparacao::ramo("cpz_10 comentarista local");
                ids[n] = cpz10_sw(p) as i16;
                n += 1;
                if n >= 20 {
                    break;
                }
            }
            i += 1;
            if i >= cpz10_sw(cpz10_d(this + 4)) {
                break;
            }
        }
    } else {
        let l = cpz10_d(this);
        cpz10_inicio(l);
        if cpz10_sw(l) <= 0 {
            return -1;
        }
        let mut i = 0;
        loop {
            let p = cpz10_proximo(l);
            if p == 0 {
                break;
            }
            if *((p + 0xca) as *const i8) == t {
                let aceita = if nacao == -1 {
                    crate::comparacao::ramo("cpz_10 comentarista de qualquer nacao");
                    true // cpz10: qualquer nação
                } else {
                    cpz10_u(p + 0xcf) as i32 == nacao
                };
                if aceita {
                    ids[n] = cpz10_sw(p) as i16;
                    n += 1;
                    if n >= 20 {
                        break;
                    }
                }
            }
            i += 1;
            if i >= cpz10_sw(cpz10_d(this)) {
                break;
            }
        }
    }
    if n == 0 {
        return -1;
    }
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    crate::comparacao::ramo("cpz_10 sorteio do comentarista");
    let r = sortear(n as i32); // cpz10: o sorteio do comentarista
    ids[r as usize] as i32
}

/// O comparador: lado a lado nos cabeçalhos das duas listas (o cursor) e no sorteio, e a devolução.
unsafe extern "thiscall" fn comparar_sortear_comentarista(this: usize, clube: usize, tipo: u32, nacao: i32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32) -> i32>(1407);
    let mut regioes = Vec::new();
    for l in [cpz10_d(this), cpz10_d(this + 4)] {
        if l != 0 {
            regioes.push((l, 0xe));
        }
    }
    regioes.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, clube, tipo, nacao),
        || crate::aleatorio::sem_contar(|| sortear_comentarista(this, clube, tipo, nacao)),
    );
    PLACAR.registrar(igual && r == n, || format!("sortear_comentarista(clube {:#x}, tipo {}, nacao {}) original={} {:x?} rust={} {:x?}", clube, tipo as i8, nacao, r, mo, n, mr));
    r
}

/// O nome padrão do veículo pelo tipo (as três tabelas da original são iguais): 2 jornal local, 3 rádio, 4 rádio
/// local, 5 revista; os outros, jornal nacional.
fn cpz10_veiculo_padrao(tipo: i8) -> usize {
    match tipo {
        2 => 0xa24284,
        3 => 0xa794cc,
        4 => 0xa241f4,
        5 => 0xa794b8,
        _ => 0xa242cc,
    }
}

/// { 0x7dad20 } (thiscall, ret 0x10, printouts.cpp): `PunditList::get_pundit_name(tipo, id, nome, marca)`, pelos textos
/// das notícias. Sem `nome`: marca 0. Id −1: o nome padrão do tipo e marca 0. Senão o comentarista do id (palavra +0)
/// na lista [this+4] (tipos 2 e 4) ou [this]: o nome (+2, sprintf "%s" 0x944c9f) e a marca (+0xd3); sem ele, o nome
/// padrão e marca 0.
unsafe extern "thiscall" fn nome_do_veiculo(this: usize, tipo: u32, id: u32, nome: usize, marca: *mut u8) {
    if nome == 0 {
        *marca = 0;
        return;
    }
    let sprintf: unsafe extern "C" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x944c9fusize);
    let t = tipo as i8;
    let id = id as i16;
    if id != -1 {
        let l = cpz10_d(this + if t == 2 || t == 4 { 4 } else { 0 });
        cpz10_inicio(l);
        if cpz10_sw(l) > 0 {
            let mut i = 0;
            loop {
                let p = cpz10_proximo(l);
                if p == 0 {
                    break;
                }
                if cpz10_sw(p) as i16 == id {
                    sprintf(nome, 0x991944, p + 2);
                    *marca = *((p + 0xd3) as *const u8);
                    return;
                }
                i += 1;
                if i >= cpz10_sw(l) {
                    break;
                }
            }
        }
    }
    sprintf(nome, 0x991944, cpz10_veiculo_padrao(t));
    *marca = 0;
}

/// O comparador: lado a lado no nome (até 0xd4 bytes: o nome do comentarista tem 0xd1 e o padrão menos), na marca e
/// nos cabeçalhos das listas (o cursor).
unsafe extern "thiscall" fn comparar_nome_do_veiculo(this: usize, tipo: u32, id: u32, nome: usize, marca: *mut u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, *mut u8)>(1408);
    let mut regioes = vec![(marca as usize, 1)];
    if nome != 0 {
        regioes.push((nome, 0xd4));
    }
    for l in [cpz10_d(this), cpz10_d(this + 4)] {
        if l != 0 {
            regioes.push((l, 0xe));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, tipo, id, nome, marca), || nome_do_veiculo(this, tipo, id, nome, marca));
    PLACAR.registrar(igual, || format!("nome_do_veiculo(tipo {}, id {}) original={:x?} rust={:x?}", tipo as i8, id as i16, mo, mr));
}

/// { 0x758de0 } (thiscall, ret 0x18, media.cpp, SEH da notícia local, sem destrutor): `Media::
/// queue_manager_fa_complaint_story(clube, pessoa, tipo, a, b, c)`, pela disciplina (0x55e300) e pelo MANAGER_MANAGER
/// (0x6a9e60, 0x6ab800). Clube com nação (+0x53) e técnico (+0xcf), e pessoa: a notícia 0x178e (o clube; o técnico, id
/// e chaves, ou −1; o tipo; a pessoa, id e chaves; a; b; a nação ou −1; c) aos técnicos da nação (0x7863f0) se a
/// reputação (+0x80) passar de 4750, senão ao clube (0x7861e0).
unsafe extern "thiscall" fn noticia_queixa_a_federacao(this: usize, clube: usize, pessoa: usize, tipo: u32, a: u32, b: u32, c: u32) {
    let _ = this;
    if clube == 0 || cpz10_d(clube + 0x53) == 0 || cpz10_d(clube + 0xcf) == 0 || pessoa == 0 {
        return;
    }
    crate::comparacao::ramo("cpz_10 noticia da queixa a federacao");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0x178e, 0);
    gravar(nt, 0, cpz10_u(clube));
    let tecnico = cpz10_d(clube + 0xcf);
    if tecnico != 0 {
        gravar(nt, 1, cpz10_u(tecnico));
        gravar(nt, 2, cpz10_u(cpz10_d(tecnico + 4) + 0x33));
        gravar(nt, 3, cpz10_u(cpz10_d(tecnico + 8) + 0x33));
        gravar(nt, 4, cpz10_u(cpz10_d(tecnico + 0xc) + 0x33));
    } else {
        for k in 1..5 {
            gravar(nt, k, u32::MAX);
        }
    }
    gravar(nt, 5, tipo as i8 as u32);
    gravar(nt, 6, cpz10_u(pessoa));
    gravar(nt, 7, cpz10_u(cpz10_d(pessoa + 4) + 0x33));
    gravar(nt, 8, cpz10_u(cpz10_d(pessoa + 8) + 0x33));
    gravar(nt, 9, cpz10_u(cpz10_d(pessoa + 0xc) + 0x33));
    gravar(nt, 0xa, a as i8 as u32);
    gravar(nt, 0xb, b as i16 as u32);
    let nacao = cpz10_d(clube + 0x53);
    gravar(nt, 0xc, if nacao != 0 { cpz10_u(nacao) } else { u32::MAX });
    gravar(nt, 0xd, c as i8 as u32);
    let nacao = cpz10_d(clube + 0x53);
    if nacao != 0 && cpz10_sw(clube + 0x80) > 0x128e {
        let aos_da_nacao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
        aos_da_nacao(cpz10_d(0xb7431c), nt, nacao);
    } else {
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
        entregar(cpz10_d(0xb7431c), nt, clube);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo. Sem notícia as duas não
// fazem nada.
unsafe extern "thiscall" fn comparar_noticia_queixa_a_federacao(this: usize, clube: usize, pessoa: usize, tipo: u32, a: u32, b: u32, c: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(1409)(this, clube, pessoa, tipo, a, b, c)
}

// ---------- 262a leva: a folha salarial e a janela dos ingressos: a folha do clube, as folhas refeitas na criação e os dias até a temporada (finance.cpp) ----------

unsafe fn cpz11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpz11_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpz11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpz11_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpz11_por(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

/// { 0x598370 } (thiscall, finance.cpp): `Finance::update_wage_bill()`, pelo passe diário das finanças (0x59b9c0, no
/// sábado, clube com nação), pela assinatura de contrato (0x4cf720), pelo passe diário dos contratos (0x4c7ac0), pelo
/// empréstimo (0x4d01e0, 0x4d07e0) e pelas folhas da criação (0x59cf50). Zera o maior salário (+0x10) e a folha (+0xc)
/// e soma os salários (+0xc do contrato) das 50 vagas do elenco (+0xd7; contrato com o clube 0x4cf5f0; salário −1: o
/// salário padrão 0x864550 truncado; emprestado (0x4cf6d0) fora do tipo 6: menos o salário do outro contrato) e do
/// pessoal (+0x19f × 5, +0x1b3 × 7, +0x1cf × 3, contrato 0x4cf5a0). Elenco com menos de 15 vagas ocupadas: as que
/// faltam a um salário pela reputação (+0x80: acima de 4750, rep − 1950; acima de 2250, (rep − 2250) × 0,5 + 550; senão
/// rep × 0,25; fora do tipo 1 (+0x64), × 0,5; dividido pelo tipo da nação (+0x7e, entre 1 e 3)). Mais reputação² × k
/// (tipo 1: Inglaterra e Itália 0,001 / 0,0008 / 0,0006 acima de 5250 / 3250; outras 0,0004; fora do tipo 1: 0,0002
/// acima de 3250, senão 0,0001), truncado. O teto salarial (+0x14) refeito por Finance::get_max_wage (0x5968d0, 0, −1).
unsafe extern "thiscall" fn folha_salarial_do_clube(this: usize) {
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let padrao: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32, u32) -> f64 = core::mem::transmute(0x864550usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let teto: unsafe extern "thiscall" fn(usize, u32, i32) -> i32 = core::mem::transmute(0x5968d0usize);
    cpz11_por(this + 0x10, 0);
    cpz11_por(this + 0xc, 0);
    let clube = cpz11_d(0xae23b4).wrapping_add(cpz11_d(this + 8).wrapping_mul(0x245));
    let mut ocupadas: i8 = 0;
    for k in 0..50usize {
        let p = cpz11_d(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        ocupadas = ocupadas.wrapping_add(1);
        let c = com_clube(cpz11_d(0xae1050), p, clube);
        if c == 0 {
            continue;
        }
        let mut w = cpz11_i(c + 0xc);
        if w == -1 {
            w = ftol(padrao(c, p, clube, 0, -1, 0xb, 0));
        }
        if ficha(cpz11_d(0xae1050), p) != 0 && cpz11_b(c + 0x35) & 0x3f != 6 {
            w = w.wrapping_sub(cpz11_i(ficha(cpz11_d(0xae1050), p) + 0xc)); // a parte do outro contrato
        }
        if w > cpz11_i(this + 0x10) {
            cpz11_por(this + 0x10, w);
        }
        cpz11_por(this + 0xc, cpz11_i(this + 0xc).wrapping_add(w));
    }
    // o pessoal: a soma vai à folha depois das três listas (a mesma soma, em 32 bits)
    let mut comissao = 0i32;
    for (base, n) in [(0x19fusize, 5usize), (0x1b3, 7), (0x1cf, 3)] {
        for k in 0..n {
            let p = cpz11_d(clube + base + 4 * k);
            if p == 0 {
                continue;
            }
            let c = contrato_de(cpz11_d(0xae1050), p);
            if c == 0 {
                continue;
            }
            let w = cpz11_i(c + 0xc);
            if w > cpz11_i(this + 0x10) {
                cpz11_por(this + 0x10, w);
            }
            comissao = comissao.wrapping_add(w);
        }
    }
    if comissao != 0 {
        crate::comparacao::ramo("cpz_11 folha do pessoal");
        cpz11_por(this + 0xc, cpz11_i(this + 0xc).wrapping_add(comissao)); // cpz11: o pessoal na folha
    }
    let tipo1 = cpz11_b(clube + 0x64) == 1;
    let rep = cpz11_w(clube + 0x80) as i32;
    if ocupadas < 15 {
        let mut v = if rep > 0x128e {
            rep - 0x79e
        } else if rep > 0x8ca {
            ftol((rep - 0x8ca) as f64 * f64_em(0x968968) + f64_em(0x96a620))
        } else {
            ftol(rep as f64 * f64_em(0x9689e0))
        };
        if !tipo1 {
            v = ftol(v as f64 * f64_em(0x968968));
        }
        let nacao = cpz11_d(clube + 0x53);
        if nacao != 0 {
            let t = (cpz11_b(nacao + 0x7e) as i8).clamp(1, 3) as i32;
            v = v.wrapping_div(t);
        }
        let falta = 15 - ocupadas as i32;
        cpz11_por(this + 0xc, cpz11_i(this + 0xc).wrapping_add(falta.wrapping_mul(v)));
    }
    let k = if tipo1 {
        let nacao = cpz11_d(clube + 0x53);
        let grande = nacao != 0 && (cpz11_i(nacao) == cpz11_i(0x9cf2e4) || cpz11_i(nacao) == cpz11_i(0x9cf360));
        if !grande {
            0x96a608
        } else if rep > 0x1482 {
            0x96a350
        } else if rep > 0xcb2 {
            0x96a618
        } else {
            0x96a610
        }
    } else if rep > 0xcb2 {
        0x96a600
    } else {
        0x96a5c0
    };
    crate::comparacao::ramo("cpz_11 folha pela reputacao");
    let folha = ftol(rep.wrapping_mul(rep) as f64 * f64_em(k) + cpz11_i(this + 0xc) as f64); // cpz11: a reputação na folha
    cpz11_por(this + 0xc, folha);
    let t = teto(this, 0, -1);
    cpz11_por(this + 0x14, t);
}

/// Comparador: lado a lado na folha (+0xc), no maior salário (+0x10), no teto (+0x14) e no último contrato consultado
/// ([0xae1080], que o salário padrão 0x864550 grava); as outras chamadas (contratos, teto 0x5968d0) só leem.
unsafe extern "thiscall" fn comparar_folha_salarial_do_clube(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1410);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0xc, 12), (0xae1080, 4)], || o(this), || folha_salarial_do_clube(this));
    PLACAR.registrar(igual, || format!("folha_salarial_do_clube(fin {:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// { 0x59cf50 } (thiscall, finance.cpp): `FinanceTable::refresh_all_wage_bills()`, só pela criação do jogo (0x828110):
/// para cada clube de [0xae23b4] até a primeira seleção (0x53b900), a folha (0x598370) do registro de finanças dele
/// ([this] + i × 0x167).
unsafe extern "thiscall" fn refazer_todas_as_folhas(this: usize) {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x598370usize);
    let mut fin = cpz11_d(this);
    let mut i = 0i32;
    while i < cpz11_i(0xae235c) {
        let clube = cpz11_d(0xae23b4).wrapping_add((i as usize).wrapping_mul(0x245));
        if selecao(clube) != 0 {
            break;
        }
        folha(fin);
        fin = fin.wrapping_add(0x167);
        i += 1;
    }
}

/// Comparador: lado a lado na tabela de finanças inteira ([this], [0xae235c] × 0x167 bytes) e em [0xae1080].
unsafe extern "thiscall" fn comparar_refazer_todas_as_folhas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1411);
    let n = (cpz11_i(0xae235c).max(0) as usize).wrapping_mul(0x167);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(cpz11_d(this), n), (0xae1080, 4)], || o(this), || refazer_todas_as_folhas(this));
    PLACAR.registrar(igual, || format!("refazer_todas_as_folhas({:#x}) original={:x?} rust={:x?}", this, mo, mr));
}

/// { 0x59b4c0 } (ret 8, finance.cpp): `finance_get_days_to_season_start(clube, &passou)`, pelo passe diário das
/// finanças (0x59b9c0; ecx com a tabela, que ela não lê): dias até o começo da temporada da nação do clube (linha de
/// 72 bytes em 0xb63d60 pela 0x669360; sem linha, a caixa "Error" da linha 0x1185 e 0). `passou` = 1 se o ano de hoje
/// chega ao ano da temporada (2002 com +0x17, senão o ano de [0xdd6ec8] + 1). Mês do começo (+0x34) abaixo de 2: no ano
/// do começo (+0x2c), só depois do mês 12 − mês do começo; noutro ano, só antes do mês do começo; senão, só a 0..2
/// meses do começo. Na janela, os dias até a data do começo (+0x2a, 0x54cdb0) se positivos; fora, 0.
unsafe extern "thiscall" fn dias_ate_a_temporada(_this: usize, clube: usize, passou: *mut u32) -> i32 {
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669360usize);
    let mes_de: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x54cfb0usize);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    *passou = 0;
    let k = linha(clube) as i8;
    if k < 0 {
        aviso_em(0x9c8784, 0x1185);
        return 0;
    }
    let e = 0xb63d60usize.wrapping_add((k as isize * 72) as usize);
    let ano: u16 = if cpz11_d(e + 0x17) != 0 { 0x7d2 } else { (cpz11_w(0xdd6eca) as u16).wrapping_add(1) };
    if cpz11_w(0xae2c92) as u16 >= ano {
        *passou = 1;
    }
    let inicio = cpz11_b(e + 0x34) as i8;
    let ano_do_inicio = cpz11_w(e + 0x2c) as u16;
    let mes = mes_de(0xae2c90) as i8;
    let janela = if inicio < 2 {
        if ano_do_inicio == cpz11_w(0xae2c92) as u16 {
            mes as i32 > 12 - inicio as i32
        } else {
            mes < inicio
        }
    } else {
        let d = inicio.wrapping_sub(mes);
        (0..=2).contains(&d)
    };
    if !janela {
        return 0;
    }
    let r = dias_ate(e + 0x2a, cpz11_d(0xae2c90) as u32, cpz11_d(0xae2c94) as u32);
    if r > 0 {
        crate::comparacao::ramo("cpz_11 dias ate a temporada");
        return r; // cpz11: os dias até a temporada
    }
    0
}

/// Comparador: sem linha da nação, só a original (a caixa); senão lado a lado na saída `passou` e na data estática da
/// 0x54cdb0 (0xae2470), com a devolução.
unsafe extern "thiscall" fn comparar_dias_ate_a_temporada(this: usize, clube: usize, passou: *mut u32) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u32) -> i32>(1412);
    let linha: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669360usize);
    if (linha(clube) as i8) < 0 {
        return o(this, clube, passou);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(passou as usize, 4), (0xae2470, 8)], || o(this, clube, passou), || dias_ate_a_temporada(this, clube, passou));
    PLACAR.registrar(igual && r == n, || format!("dias_ate_a_temporada(clube {:#x}) original={} {:x?} rust={} {:x?}", clube, r, mo, n, mr));
    r
}

// ---------- 262a leva: o dinheiro das transferências: a venda de jogador, a compra de jogador e as luvas (finance.cpp) ----------

/// Soma `v` (64 bits) ao i64 em `a` (o add/adc da original).
unsafe fn cpz12_somar64(a: usize, v: i64) {
    core::ptr::write_unaligned(a as *mut i64, core::ptr::read_unaligned(a as *const i64).wrapping_add(v));
}

/// Soma `v` ao i32 em `a`.
unsafe fn cpz12_somar32(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, core::ptr::read_unaligned(a as *const i32).wrapping_add(v));
}

/// O valor de 64 bits que a original recebe em dois dwords (baixo primeiro).
fn cpz12_valor(baixo: u32, alto: u32) -> i64 {
    ((alto as u64) << 32 | baixo as u64) as i64
}

/// { 0x599800 } (thiscall, ret 8, finance.cpp): `Finance::add_players_sold(valor)` (valor de 64 bits), pelo pagamento
/// da transferência (TRANSFER_MANAGER::pay_fee_and_record_clauses 0x8ca8a0 e as parcelas 0x8caf.., 0x8cb3..) e pela
/// parte dos clubes anteriores (0x8e4f20): soma o valor às vendas do mês (+0x78) e do ano (+0x118), em 64 bits, e ao
/// saldo (i64 em +0).
unsafe extern "thiscall" fn receber_venda_de_jogador(this: usize, baixo: u32, alto: u32) {
    let v = cpz12_valor(baixo, alto);
    cpz12_somar64(this + 0x78, v);
    cpz12_somar64(this + 0x118, v);
    if v != 0 {
        crate::comparacao::ramo("cpz_12 venda recebida"); // só valor diferente de zero muda o saldo
    }
    cpz12_somar64(this, v); // cpz12: a venda entra no saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_receber_venda_de_jogador(this: usize, baixo: u32, alto: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(1413);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, baixo, alto), || receber_venda_de_jogador(this, baixo, alto));
    PLACAR.registrar(igual, || format!("receber_venda_de_jogador(fin {:#x}, {}) original={:x?} rust={:x?}", this, cpz12_valor(baixo, alto), mo, mr));
}

/// { 0x599890 } (thiscall, ret 8, finance.cpp): `Finance::pay_players_bought(valor)` (valor de 64 bits), pelo pagamento
/// da transferência (0x8ca8a0) e pelas parcelas (0x8caf.., 0x8cb3..): soma o valor às compras do mês (+0x70) e do ano
/// (+0x110), em 64 bits, e o tira do saldo.
unsafe extern "thiscall" fn pagar_compra_de_jogador(this: usize, baixo: u32, alto: u32) {
    let v = cpz12_valor(baixo, alto);
    cpz12_somar64(this + 0x70, v);
    cpz12_somar64(this + 0x110, v);
    if v != 0 {
        crate::comparacao::ramo("cpz_12 compra paga"); // só valor diferente de zero muda o saldo
    }
    cpz12_somar64(this, v.wrapping_neg()); // cpz12: a compra sai do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_pagar_compra_de_jogador(this: usize, baixo: u32, alto: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32)>(1414);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, baixo, alto), || pagar_compra_de_jogador(this, baixo, alto));
    PLACAR.registrar(igual, || format!("pagar_compra_de_jogador(fin {:#x}, {}) original={:x?} rust={:x?}", this, cpz12_valor(baixo, alto), mo, mr));
}

/// { 0x5998e0 } (thiscall, ret 4, finance.cpp): `Finance::pay_sign_on_fee(valor)`, pela oferta de contrato aceita
/// (0x8c38b0) e pela mudança do jogador de clube (0x8c9720): soma o valor às luvas do ano (+0x13c) e do mês (+0x9c) e o
/// tira do saldo (estendido com sinal).
unsafe extern "thiscall" fn pagar_luvas(this: usize, valor: i32) {
    cpz12_somar32(this + 0x13c, valor);
    cpz12_somar32(this + 0x9c, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_12 luvas pagas"); // só valor diferente de zero muda o saldo
    }
    cpz12_somar64(this, (valor as i64).wrapping_neg()); // cpz12: as luvas saem do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_pagar_luvas(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(1415);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_luvas(this, valor));
    PLACAR.registrar(igual, || format!("pagar_luvas(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

// ---------- 262a leva: as receitas e a compensação: o prêmio em dinheiro, os direitos de TV e o salário pago (finance.cpp) ----------

/// Soma `v` ao i32 em `a`.
unsafe fn cpz13_somar32(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, core::ptr::read_unaligned(a as *const i32).wrapping_add(v));
}

/// Soma `v` (estendido com sinal: o cdq/adc da original) ao saldo, i64 em +0.
unsafe fn cpz13_somar_saldo(fin: usize, v: i64) {
    core::ptr::write_unaligned(fin as *mut i64, core::ptr::read_unaligned(fin as *const i64).wrapping_add(v));
}

/// { 0x5999a0 } (thiscall, ret 4, finance.cpp): `Finance::add_prize_money(valor)`, pelas copas (Cup::process_fixture
/// 0x51a150, EuropeanCup::process_fixture 0x583b10, os sorteios das fases), pelo fim das ligas (League::pay_prize_money
/// 0x682e10) e pelas honras da temporada: soma o valor aos prêmios do ano (+0x148) e do mês (+0xa8) e ao saldo.
unsafe extern "thiscall" fn receber_premio_em_dinheiro(this: usize, valor: i32) {
    cpz13_somar32(this + 0x148, valor);
    cpz13_somar32(this + 0xa8, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 premio recebido"); // só valor diferente de zero muda o saldo
    }
    cpz13_somar_saldo(this, valor as i64); // cpz13: o prêmio entra no saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_receber_premio_em_dinheiro(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(1416);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || receber_premio_em_dinheiro(this, valor));
    PLACAR.registrar(igual, || format!("receber_premio_em_dinheiro(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// { 0x5987c0 } (thiscall, ret 4, finance.cpp): `Finance::add_tv_revenue(valor)`, só pelos direitos de TV da liga
/// (League::pay_tv_rights 0x683010, pela montagem da tabela de cada liga na temporada nova): soma o valor à TV do ano
/// (+0x158) e do mês (+0xb8) e ao saldo.
unsafe extern "thiscall" fn receber_direitos_de_tv(this: usize, valor: i32) {
    cpz13_somar32(this + 0x158, valor);
    cpz13_somar32(this + 0xb8, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 tv recebida"); // só valor diferente de zero muda o saldo
    }
    cpz13_somar_saldo(this, valor as i64); // cpz13: a TV entra no saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_receber_direitos_de_tv(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(1417);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || receber_direitos_de_tv(this, valor));
    PLACAR.registrar(igual, || format!("receber_direitos_de_tv(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}

/// { 0x599920 } (thiscall, ret 4, finance.cpp): `Finance::pay_wages(valor)`, só pela liberação sem custo
/// (TRANSFER_MANAGER::free_transfer_staff 0x8c1af0: a compensação do contrato ao clube): soma o valor aos salários do ano
/// (+0x130) e do mês (+0x90) e o tira do saldo.
unsafe extern "thiscall" fn pagar_salarios(this: usize, valor: i32) {
    cpz13_somar32(this + 0x130, valor);
    cpz13_somar32(this + 0x90, valor);
    if valor != 0 {
        crate::comparacao::ramo("cpz_13 salario pago"); // só valor diferente de zero muda o saldo
    }
    cpz13_somar_saldo(this, (valor as i64).wrapping_neg()); // cpz13: o salário sai do saldo
}

/// Comparador: lado a lado no registro de finanças (0x167 bytes).
unsafe extern "thiscall" fn comparar_pagar_salarios(this: usize, valor: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32)>(1418);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x167)], || o(this, valor), || pagar_salarios(this, valor));
    PLACAR.registrar(igual, || format!("pagar_salarios(fin {:#x}, {}) original={:x?} rust={:x?}", this, valor, mo, mr));
}
