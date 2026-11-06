// ---------- 278a leva: o mata-mata dos estaduais e regionais: Gaúcho, Centro-Oeste e Goiano (bra_reg_gaucho.cpp, bra_reg_central.cpp, bra_reg_goias.cpp) ----------

unsafe fn cqn9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqn9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube da linha `l` da tabela (+0xb1, linhas de 0x3b bytes, +0 o clube).
unsafe fn cqn9_clube(liga: usize, l: usize) -> usize {
    cqn9_d(cqn9_d(liga + 0xb1) + l * 0x3b)
}

/// O mata-mata montado da tabela (o mesmo molde nas semifinais e nos mata-matas dos estaduais; só mudam o fonte, as
/// linhas da caixa e o arranjo): a lista dos `linhas.len()` times (malloc 0x944e46 de 4 × n; sem memória: a caixa
/// `linha_sem_lista` e nada mais) com os clubes das `linhas` da tabela, nesta ordem (as chaves: 0 × 1, 2 × 3...); a
/// virtual +0x3c (create_round_table: thiscall(this, fase 0, &rodadas, &extra, nulo), ret 0x10) monta a tabela de
/// rodadas; a fase nova (operator new 0x944cf1 de 0xb2 bytes; Cup_Stage::Cup_Stage 0x522e00, thiscall com 15
/// argumentos, ret 0x3c: a liga, n, a lista, as rodadas, +4, a tabela de rodadas, o ano +0x40, 0, 1, o extra da
/// virtual, 0x14, 0, 1, 0 e as sementes, um byte por time, que o construtor grava no +4 da lista da fase: na original,
/// a linha de cada time) vai para a
/// primeira vaga de fases ([+0xc]). Sem fase: a caixa `linha_sem_fase` e volta (sem liberar). Com ela:
/// Cup::set_up_round (0x51c800, thiscall(fase, 0), ret 4), a tabela de rodadas e a lista liberadas (free 0x9452ca,
/// nessa ordem) e +0x30 = 0. O quadro SEH da original só cobre o construtor (sem destrutor).
unsafe fn cqn9_mata_mata(this: usize, linhas: &[usize], sementes: &[u8], arquivo: usize, linha_sem_lista: u32, linha_sem_fase: u32, ramo: &'static str) {
    let n = linhas.len();
    // as sementes: a original grava os bytes na pilha antes de alocar (iguais às linhas, mas constantes à parte)
    let mut s = [0u8; 8];
    s[..sementes.len()].copy_from_slice(sementes);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let lista = alocar((n * 4) as u32);
    if lista == 0 {
        return aviso_em(arquivo, linha_sem_lista);
    }
    for (k, &l) in linhas.iter().enumerate() {
        core::ptr::write_unaligned((lista + 4 * k) as *mut usize, cqn9_clube(this, l));
    }
    crate::comparacao::ramo(ramo);
    let tabela: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqn9_d(cqn9_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let t = tabela(this, 0, &mut rodadas, &mut extra, 0);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let p = novo(0xb2);
    // ponytail: a original empurra as rodadas, o extra e o ano em dwords com lixo nos 16 bits altos (as saídas da
    // virtual são words na pilha; o ano vai no cx); o construtor só lê as palavras.
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, n as u32, lista, rodadas as u16 as u32, cqn9_d(this + 4), t, cqn9_w(this + 0x40) as u16 as u32, 0, 1, extra as u16 as u32, 0x14, 0, 1, 0, s.as_ptr())
    } else {
        0
    };
    core::ptr::write_unaligned(cqn9_d(this + 0xc) as *mut usize, fase);
    let fase = cqn9_d(cqn9_d(this + 0xc));
    if fase == 0 {
        return aviso_em(arquivo, linha_sem_fase);
    }
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    montar(fase, 0);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(t);
    liberar(lista);
    core::ptr::write_unaligned((this + 0x30) as *mut u32, 0);
}

/// { 0x4325b0 } (thiscall, ret 0, bra_reg_gaucho.cpp): `BraRegGaucho::create_knockout_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As semifinais: 1º × 4º, 2º × 3º (linhas 0, 3, 1, 2). Fonte 0x98ae50, linhas 0x217 (sem lista) e 0x22b (sem fase).
unsafe extern "thiscall" fn cqn9_mata_mata_do_gaucho(this: usize) {
    cqn9_mata_mata(this, &[0, 3, 1, 2], &[0, 3, 1, 2], 0x98ae50, 0x217, 0x22b, "cqn_9 gaucho"); // cqn9: 1º × 4º, 2º × 3º Gaúcho
}

/// { 0x4312d0 } (thiscall, ret 0, bra_reg_central.cpp): `BraRegCentral::create_knockout_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As quartas de final: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º (linhas 0, 7, 2, 5, 6, 1, 4, 3). Fonte 0x98ae10, linhas 0x21c (sem lista) e 0x234 (sem fase).
unsafe extern "thiscall" fn cqn9_mata_mata_do_centro_oeste(this: usize) {
    cqn9_mata_mata(this, &[0, 7, 2, 5, 6, 1, 4, 3], &[0, 7, 2, 5, 6, 1, 4, 3], 0x98ae10, 0x21c, 0x234, "cqn_9 centro_oeste"); // cqn9: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º Centro-Oeste
}

/// { 0x433930 } (thiscall, ret 0, bra_reg_goias.cpp): `BraRegGoias::create_knockout_stage()`, da fase seguinte (advance_stage, quando a fase passa de −1 a
/// 0). As quartas de final: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º (linhas 0, 7, 2, 5, 6, 1, 4, 3). Fonte 0x98ae90, linhas 0x214 (sem lista) e 0x22c (sem fase).
unsafe extern "thiscall" fn cqn9_mata_mata_do_goiano(this: usize) {
    cqn9_mata_mata(this, &[0, 7, 2, 5, 6, 1, 4, 3], &[0, 7, 2, 5, 6, 1, 4, 3], 0x98ae90, 0x214, 0x22c, "cqn_9 goiano"); // cqn9: 1º × 8º, 3º × 6º, 7º × 2º, 5º × 4º Goiano
}

/// O comparador dos três: a original aloca a lista, a tabela de rodadas e a fase e monta a rodada (só ela roda). O
/// plano, feito antes: os clubes das linhas. Depois, a fase da primeira vaga tem de existir, com +0x3e = n, +4 = o +4 da
/// liga e os n primeiros clubes da lista dela (+0xa7, 6 bytes) iguais aos do plano (como conjunto: o sorteio da rodada
/// 0x51c800 pode reordenar as chaves), e +0x30 = 0.
unsafe fn cqn9_comparar(nome: &str, this: usize, linhas: &[usize], original: impl FnOnce()) {
    let mut plano: Vec<usize> = linhas.iter().map(|&l| cqn9_clube(this, l)).collect();
    original();
    let fase = cqn9_d(cqn9_d(this + 0xc));
    if fase == 0 {
        PLACAR.registrar(false, || format!("{}({:#x}) sem fase", nome, this));
        return;
    }
    let lista = cqn9_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..linhas.len()).map(|k| cqn9_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqn9_w(fase + 0x3e) as usize == linhas.len() && cqn9_d(fase + 4) == cqn9_d(this + 4) && cqn9_d(this + 0x30) == 0;
    PLACAR.registrar(igual, || format!("{}({:#x}) plano={:x?} fase={:#x} lista={:x?} +0x3e={}", nome, this, pl, fase, ac, cqn9_w(fase + 0x3e)));
}

unsafe extern "thiscall" fn comparar_cqn9_mata_mata_do_gaucho(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1849);
    cqn9_comparar("cqn9_mata_mata_do_gaucho", this, &[0, 3, 1, 2], || o(this))
}

unsafe extern "thiscall" fn comparar_cqn9_mata_mata_do_centro_oeste(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1850);
    cqn9_comparar("cqn9_mata_mata_do_centro_oeste", this, &[0, 7, 2, 5, 6, 1, 4, 3], || o(this))
}

unsafe extern "thiscall" fn comparar_cqn9_mata_mata_do_goiano(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1851);
    cqn9_comparar("cqn9_mata_mata_do_goiano", this, &[0, 7, 2, 5, 6, 1, 4, 3], || o(this))
}

// ---------- 278a leva: o desempate da Série A e da Série B italianas no fim da temporada (o "spareggio": empate em pontos na fronteira de duas faixas), a ordem dos empatados pelo confronto direto e o resultado do jogo de desempate (ita_ser_a.cpp, ita_ser_b.cpp) ----------

unsafe fn cql4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cql4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cql4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cql4_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A linha `i` da tabela [+0xb1] (0x3b bytes).
unsafe fn cql4_linha(liga: usize, i: usize) -> usize {
    cql4_d(liga + 0xb1) + i * 0x3b
}

/// As faixas da Série A, por posição (0 = o primeiro): 1 o campeão; 2 as outras vagas da Liga dos Campeões (as `cl`
/// vagas da nação, UefaSeeding::get_nation_places 0x905c60: push &uefa; push &cl; push [[liga+4]+0x5d] (a nação da
/// competição); mov ecx, [0xde1f58] -> thiscall, ret 0xc, que grava 1 nas duas antes de procurar); 3 as `uefa` vagas da
/// Copa da UEFA, logo depois; 4 as duas seguintes [inferência: a Intertoto]; 5 as `+0xc1` últimas de 18 (o
/// rebaixamento); 0 o resto. A original monta as 18 faixas num registro local (16 + 2 bytes) e lê duas além com uma
/// tabela de mais de 18 clubes; a cópia usa 64 bytes com zero depois das 18 (a Série A tem 18).
unsafe fn cql4_faixas_da_serie_a(liga: usize) -> [u8; 64] {
    let vagas: unsafe extern "thiscall" fn(usize, usize, *mut i8, *mut i8) = core::mem::transmute(0x905c60usize);
    let mut z = [0u8; 64];
    z[0] = 1;
    let (mut cl, mut uefa) = (0i8, 0i8);
    vagas(cql4_d(0xde1f58), cql4_d(cql4_d(liga + 4) + 0x5d), &mut cl, &mut uefa);
    // ponytail: as contas da nação ficam de 1 a 16 no jogo; a cópia ignora as faixas que cairiam fora do registro.
    let cl = cl as i32;
    let uefa = uefa as i32;
    if cl > 1 {
        for k in 1..cl {
            if (k as usize) < 64 {
                z[k as usize] = 2;
            }
        }
    }
    if uefa > 0 {
        for k in 0..uefa {
            if ((cl + k) as usize) < 64 {
                z[(cl + k) as usize] = 3;
            }
        }
    }
    let i = uefa + cl;
    if i >= 0 && (i as usize) + 1 < 64 {
        z[i as usize] = 4; // cql4: as duas posições depois das vagas europeias
        z[i as usize + 1] = 4;
    }
    let rebaixados = *((liga + 0xc1) as *const i8) as i32;
    if rebaixados > 0 {
        for k in 0..rebaixados {
            if 17 - k >= 0 {
                z[(17 - k) as usize] = 5;
            }
        }
    }
    z
}

/// As faixas da Série B: 1 o campeão; 2 as 3 vagas seguintes de acesso; 3 as 4 últimas de 20 (o rebaixamento); 0 o
/// resto (registro local de 20 bytes na original).
fn cql4_faixas_da_serie_b() -> [u8; 64] {
    let mut z = [0u8; 64];
    z[0] = 1;
    z[1] = 2;
    z[2] = 2;
    z[3] = 2; // cql4: as vagas de acesso da Série B
    for k in 16..20 {
        z[k] = 3;
    }
    z
}

/// Há duas linhas vizinhas com os mesmos pontos (+0x19) em faixas diferentes? (a conferência do comparador)
unsafe fn cql4_empate_na_fronteira(liga: usize, z: &[u8; 64]) -> bool {
    let n = cql4_w(liga + 0x3e) as i32 - 1;
    (0..n.max(0) as usize).any(|i| {
        let l = cql4_linha(liga, i);
        cql4_w(l + 0x19) == cql4_w(l + 0x3b + 0x19) && z[i.min(63)] != z[(i + 1).min(63)]
    })
}

/// { 0x642880 } (thiscall, ret 8, ita_ser_a.cpp, slot +0x14): `ItaSerA::update_for_date(hoje, completo)`. Primeiro o
/// dia de liga (League::update_for_date 0x6847c0: push completo; push hoje; mov ecx, this -> thiscall, ret 8), cuja
/// devolução ela repassa. Depois, no fim da temporada (+0xdd), na fase principal (+0x2c = 0, sem fases criadas), com
/// `completo` e com +0xe6 [inferência: a tabela fechada]: zera +0x4c e procura, de cima para baixo, o primeiro par de
/// clubes vizinhos com os mesmos pontos em faixas diferentes (cql4_faixas_da_serie_a). Achando: ordena os empatados
/// pelo confronto direto (0x6471a0: push pontos; mov ecx, this -> thiscall, ret 4), cria o vetor de 1 fase (+0x2c =
/// 1, +0xc = malloc 0x944e46 de 4: push 4; add esp, 4 -> cdecl) e chama a virtual +0x28 (a fase seguinte: o jogo de
/// desempate). Sem empate na fronteira: +0xdd = 0, +0xe6 = 0 e a virtual +0x94 (League::update_positions_and_status).
/// A regra: na Itália, empate em pontos entre clubes de faixas diferentes (título, Liga dos Campeões, Copa da UEFA,
/// Intertoto [inferência], rebaixamento) não se decide pelo saldo: vai a um jogo extra.
unsafe extern "thiscall" fn desempate_da_serie_a(this: usize, hoje: usize, completo: u32) -> u32 {
    let dia: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x6471a0usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let r = dia(this, hoje, completo);
    // ponytail: a original devolve o byte do dia de liga com os bits de cima do eax que sobram da última chamada; o
    // laço das competições só lê o byte.
    if cql4_d(this + 0xdd) == 0 || cql4_d(this + 0x2c) != 0 || completo == 0 || cql4_d(this + 0xe6) == 0 {
        return r;
    }
    let z = cql4_faixas_da_serie_a(this);
    let n = cql4_w(this + 0x3e) as i32 - 1;
    cql4_pd(this + 0x4c, 0);
    crate::comparacao::ramo("cql_4 fim da serie a");
    let mut i: i16 = 0;
    if n > 0 {
        loop {
            let l = cql4_linha(this, i as usize);
            let pontos = cql4_w(l + 0x19);
            if pontos == cql4_w(l + 0x54) && z[(i as usize).min(63)] != z[(i as usize + 1).min(63)] { // cql4: o par da Série A na fronteira
                ordenar(this, pontos as u16 as u32);
                cql4_pd(this + 0x2c, 1);
                let v = alocar(4);
                cql4_pd(this + 0xc, v);
                let proxima: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cql4_d(cql4_d(this) + 0x28));
                proxima(this);
                return r;
            }
            i = i.wrapping_add(1);
            if i as i32 >= n {
                break;
            }
        }
    }
    cql4_pd(this + 0xdd, 0);
    cql4_pd(this + 0xe6, 0);
    let situacao: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cql4_d(cql4_d(this) + 0x94));
    situacao(this);
    r
}

/// { 0x647050 } (thiscall, ret 8, ita_ser_b.cpp, slot +0x14): `ItaSerB::update_for_date(hoje, completo)`. Como a da
/// Série A, com as faixas fixas da Série B (o campeão, mais 3 de acesso, os 4 rebaixados de 20) e sem parar no
/// primeiro par: cada par vizinho empatado na fronteira é ordenado pelo confronto direto (0x6471a0: push edi, os
/// pontos na palavra de baixo e o resto do `completo` nos bits de cima, que a 0x6471a0 não lê; mov ecx -> thiscall,
/// ret 4); havendo algum, a fase do desempate (+0x2c = 1, +0xc = malloc de 4, virtual +0x28). Sem nenhum: +0xdd = 0,
/// +0xe6 = 0 e a virtual +0x94.
unsafe extern "thiscall" fn desempate_da_serie_b(this: usize, hoje: usize, completo: u32) -> u32 {
    let dia: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x6471a0usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let r = dia(this, hoje, completo);
    if cql4_d(this + 0xdd) == 0 || cql4_d(this + 0x2c) != 0 || completo == 0 || cql4_d(this + 0xe6) == 0 {
        return r;
    }
    let z = cql4_faixas_da_serie_b();
    cql4_pd(this + 0x4c, 0);
    crate::comparacao::ramo("cql_4 fim da serie b");
    let mut achou = false;
    let mut i: i16 = 0;
    while (i as i32) < cql4_w(this + 0x3e) as i32 - 1 {
        let l = cql4_linha(this, i as usize);
        let pontos = cql4_w(l + 0x19);
        if pontos == cql4_w(l + 0x54) && z[(i as usize).min(63)] != z[(i as usize + 1).min(63)] { // cql4: o par da Série B na fronteira
            achou = true;
            ordenar(this, pontos as u16 as u32);
        }
        i = i.wrapping_add(1);
    }
    if achou {
        cql4_pd(this + 0x2c, 1);
        let v = alocar(4);
        cql4_pd(this + 0xc, v);
        let proxima: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cql4_d(cql4_d(this) + 0x28));
        proxima(this);
        return r;
    }
    cql4_pd(this + 0xdd, 0);
    cql4_pd(this + 0xe6, 0);
    let situacao: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cql4_d(cql4_d(this) + 0x94));
    situacao(this);
    r
}

/// O comparador das duas: o dia de liga e a fase do desempate não rodam duas vezes: só a original, conferindo o que
/// ela garante. Na fase principal e com `completo`: se a fase do desempate foi criada (+0x2c mudou), há na tabela um
/// par vizinho empatado em faixas diferentes; e a original nunca sai com o fim da temporada pendente (+0xdd e +0xe6
/// ligados e +0x2c = 0), porque ou cria a fase ou os zera.
unsafe fn cql4_comparar(nome: &str, this: usize, completo: u32, faixas: impl Fn() -> [u8; 64], original: impl FnOnce() -> u32) -> u32 {
    let fases = cql4_d(this + 0x2c);
    let r = original();
    if completo != 0 && fases == 0 {
        if cql4_d(this + 0x2c) != 0 {
            let z = faixas();
            PLACAR.registrar(cql4_empate_na_fronteira(this, &z) && cql4_d(this + 0xc) != 0, || format!("{}({:#x}): fase do desempate sem empate na fronteira (+0xc = {:#x})", nome, this, cql4_d(this + 0xc)));
        } else {
            PLACAR.registrar(!(cql4_d(this + 0xdd) != 0 && cql4_d(this + 0xe6) != 0), || format!("{}({:#x}): o fim da temporada ficou pendente (+0xdd e +0xe6 ligados)", nome, this));
        }
    }
    r
}

unsafe extern "thiscall" fn comparar_desempate_da_serie_a(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1852);
    cql4_comparar("desempate_da_serie_a", this, completo, || cql4_faixas_da_serie_a(this), || o(this, hoje, completo))
}

unsafe extern "thiscall" fn comparar_desempate_da_serie_b(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1853);
    cql4_comparar("desempate_da_serie_b", this, completo, cql4_faixas_da_serie_b, || o(this, hoje, completo))
}

/// { 0x6471a0 } (thiscall, ret 4, ita_ser_b.cpp; chamada também pela da Série A): `ita_league_sort_clubs_level_on_points
/// (pontos)`. Monta o confronto direto em +4 de cada linha (League::build_h2h_table 0x682b30: push [+0xb1]; mov ecx,
/// this -> thiscall, ret 4), acha o bloco das linhas com esses pontos (a primeira e quantas; a tabela já está ordenada
/// pelos pontos), ordena o bloco pela comparação do confronto (qsort 0x945333 com 0x683410, linhas de 0x3b: push;
/// add esp, 0x10 -> cdecl; sem linha, o qsort recebe 0 e 0) e renumera a posição (+4) de todas as linhas.
unsafe extern "thiscall" fn ordem_dos_empatados_na_italia(this: usize, pontos: u32) {
    let confronto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x682b30usize);
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    confronto(this, cql4_d(this + 0xb1));
    let n = cql4_w(this + 0x3e);
    let mut primeira = 0usize;
    let mut quantas: u32 = 0;
    if n > 0 {
        let mut l = cql4_d(this + 0xb1);
        for _ in 0..n as i32 {
            if cql4_w(l + 0x19) == pontos as u16 as i16 {
                if primeira == 0 {
                    primeira = l;
                }
                quantas = quantas.wrapping_add(1);
            }
            l += 0x3b;
        }
    }
    qsort(primeira, quantas as u16 as i16 as i32, 0x3b, 0x683410); // cql4: o bloco dos empatados
    let mut i: i16 = 0;
    while i < cql4_w(this + 0x3e) {
        *((cql4_linha(this, i as usize) + 4) as *mut u8) = i as u8;
        i += 1;
    }
}

/// Lado a lado na tabela inteira (a 0x682b30 grava +4 de cada linha e solta a sua minitabela; o qsort só mexe no
/// bloco).
unsafe extern "thiscall" fn comparar_ordem_dos_empatados_na_italia(this: usize, pontos: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1854);
    let t = cql4_d(this + 0xb1);
    let n = cql4_w(this + 0x3e).max(0) as usize;
    if t == 0 || n == 0 {
        return o(this, pontos);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(t, n * 0x3b)], || o(this, pontos), || ordem_dos_empatados_na_italia(this, pontos));
    PLACAR.registrar(igual, || format!("ordem_dos_empatados_na_italia({:#x}, {}) original={:x?} rust={:x?}", this, pontos as u16 as i16, mo, mr));
}

/// { 0x641dc0 } (thiscall, ret 0x18, ita_ser_a.cpp, slot +0x44): `ItaSerA::update_staff_records_for_status(clube,
/// situação, fase, a4, jogo, a6)`. Na liga (fase 0xff): a da base (League::update_staff_records_for_status 0x686940:
/// push a6; push jogo; push a4; push -1; push situação; push clube; mov ecx, this -> thiscall, ret 0x18), cuja devolução
/// repassa. No jogo de desempate (outra fase), só com a situação 1 [inferência: o clube venceu]: +0xad = 0, ordena a
/// tabela (League::sort_table 0x6827d0: push 0; mov ecx -> thiscall, ret 4) e acha a linha do clube: se o de cima é
/// um dos dois do jogo (+0x1c, +0x20), as duas linhas trocam de lugar; senão, se o de baixo é um deles, ficam. Nos dois
/// casos as duas linhas ganham a posição (+4) e o destino em aberto (+0x37 = 0xff). Depois, se nenhuma linha tem o
/// destino 1 (o play-off de cima), fecha a temporada: +0xe6 = 0, +0xdd = 0, a virtual +0x94, a virtual +0x10 (0, 0) e
/// +0xdd = 1. Devolve −1. A regra: quem vence o jogo de desempate fica à frente do outro.
unsafe extern "thiscall" fn resultado_do_desempate_da_serie_a(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    if fase as u8 == 0xff {
        let base: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x686940usize);
        return base(this, clube, situacao, 0xffff_ffff, a4, jogo, a6);
    }
    if situacao as u8 != 1 {
        return 0xffff_ffff;
    }
    crate::comparacao::ramo("cql_4 resultado do desempate");
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    cql4_pd(this + 0xad, 0);
    ordenar(this, 0);
    let t = cql4_d(this + 0xb1);
    let (a, b) = (cql4_d(jogo + 0x1c), cql4_d(jogo + 0x20));
    let mut i: i16 = 0;
    while i < cql4_w(this + 0x3e) {
        let l = t + i as usize * 0x3b;
        if cql4_d(l) == clube {
            if i > 0 {
                let acima = cql4_d(l - 0x3b);
                if acima == a || acima == b {
                    // o clube estava abaixo do outro: trocam
                    let mut tmp = [0u8; 0x3b];
                    core::ptr::copy_nonoverlapping(l as *const u8, tmp.as_mut_ptr(), 0x3b);
                    core::ptr::copy_nonoverlapping((l - 0x3b) as *const u8, l as *mut u8, 0x3b);
                    let l2 = cql4_d(this + 0xb1) + i as usize * 0x3b;
                    core::ptr::copy_nonoverlapping(tmp.as_ptr(), (l2 - 0x3b) as *mut u8, 0x3b);
                    let l3 = cql4_d(this + 0xb1) + i as usize * 0x3b;
                    *((l3 + 4) as *mut u8) = i as u8;
                    *((l3 - 0x3b + 4) as *mut u8) = (i as u8).wrapping_sub(1);
                    *((l3 + 0x37) as *mut u8) = 0xff;
                    *((l3 - 0x3b + 0x37) as *mut u8) = 0xff;
                    break;
                }
            }
            if i < cql4_w(this + 0x3e) {
                // ponytail: a original lê o clube da linha seguinte mesmo na última (4 bytes além da tabela), como aqui.
                let abaixo = cql4_d(l + 0x3b);
                if abaixo == a || abaixo == b {
                    *((t + i as usize * 0x3b + 4) as *mut u8) = i as u8;
                    let t2 = cql4_d(this + 0xb1);
                    *((t2 + i as usize * 0x3b + 0x3f) as *mut u8) = (i as u8).wrapping_add(1);
                    *((cql4_d(this + 0xb1) + i as usize * 0x3b + 0x37) as *mut u8) = 0xff;
                    *((cql4_d(this + 0xb1) + i as usize * 0x3b + 0x72) as *mut u8) = 0xff;
                    break;
                }
            }
        }
        i += 1;
    }
    let n = cql4_w(this + 0x3e);
    let mut k: i16 = 0;
    while k < n {
        if cql4_b(cql4_linha(this, k as usize) + 0x37) == 1 {
            return 0xffff_ffff;
        }
        k += 1;
    }
    cql4_pd(this + 0xe6, 0);
    cql4_pd(this + 0xdd, 0);
    let situacao_f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cql4_d(cql4_d(this) + 0x94));
    situacao_f(this);
    let fim: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(cql4_d(cql4_d(this) + 0x10));
    fim(this, 0, 0);
    cql4_pd(this + 0xdd, 1);
    0xffff_ffff
}

/// Na liga e no desempate com a situação 1 há chamadas que não rodam duas vezes (os registros da base, a ordenação, as
/// virtuais): só a original. Nas outras situações a cópia só devolve −1: compara a devolução.
unsafe extern "thiscall" fn comparar_resultado_do_desempate_da_serie_a(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1855);
    let r = o(this, clube, situacao, fase, a4, jogo, a6);
    if fase as u8 != 0xff && situacao as u8 != 1 {
        PLACAR.registrar(r == 0xffff_ffff, || format!("resultado_do_desempate_da_serie_a: devolveu {:#x}", r));
    }
    r
}

// ---------- 278a leva: o play-off de acesso e o play-out da Série C1 italiana (os dois grupos) e a passagem de fase da Third Division inglesa (ita_ser_c1a.cpp, ita_ser_c1b.cpp, eng_third.cpp) ----------

unsafe fn cql5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cql5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O clube da linha `i` da tabela [+0xb1] (0x3b bytes).
unsafe fn cql5_clube(liga: usize, i: usize) -> usize {
    cql5_d(cql5_d(liga + 0xb1) + i * 0x3b)
}

/// A passagem de fase das três (League::advance_stage, o slot +0x28; o corpo de 0x64b9e0, 0x64fba0 e 0x578330): com a
/// fase atual (+0x30) já na última (+0x2c − 1, com sinal), a caixa do fonte `arquivo` na linha `linha`; senão +0x30 + 1
/// e, ao chegar à fase 0 (a primeira depois da liga), cria as fases (`criar`: mov ecx, this -> thiscall, ret 0).
unsafe fn cql5_passar_de_fase(this: usize, arquivo: usize, linha: u32, criar: usize) {
    let fase = cql5_d(this + 0x30) as i32;
    if fase >= (cql5_d(this + 0x2c) as i32).wrapping_sub(1) {
        aviso_em(arquivo, linha);
        return;
    }
    let nova = fase.wrapping_add(1);
    cql5_pd(this + 0x30, nova as usize);
    if nova == 0 {
        crate::comparacao::ramo("cql_5 fases do fim da liga");
        let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(criar);
        f(this); // cql5: as fases do fim da liga
    }
}

/// { 0x64b9e0 } (thiscall, ret 0, ita_ser_c1a.cpp): `ItaSerC1a::advance_stage()`; cria o play-off e o play-out pela
/// 0x64ba70. Fonte 0x9ea474, linha 0x250.
unsafe extern "thiscall" fn fase_seguinte_da_c1a(this: usize) {
    cql5_passar_de_fase(this, 0x9ea474, 0x250, 0x64ba70)
}

/// { 0x64fba0 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::advance_stage()`; pela 0x64fc30. Fonte 0x9ea558, linha
/// 0x255.
unsafe extern "thiscall" fn fase_seguinte_da_c1b(this: usize) {
    cql5_passar_de_fase(this, 0x9ea558, 0x255, 0x64fc30)
}

/// { 0x578330 } (thiscall, ret 0, eng_third.cpp): `EngThird::advance_stage()`; cria o play-off da Third Division pela
/// 0x5783c0 (já em Rust). Fonte 0x9c4424, linha 0x29c.
unsafe extern "thiscall" fn fase_seguinte_da_third(this: usize) {
    cql5_passar_de_fase(this, 0x9c4424, 0x29c, 0x5783c0)
}

/// O comparador das três: sem criar fase (a fase atual só sobe), lado a lado em +0x30; na caixa ou ao criar as fases
/// (alocam), só a original (a criação tem o seu comparador).
unsafe fn cql5_comparar_passagem(nome: &str, this: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let fase = cql5_d(this + 0x30) as i32;
    if fase >= (cql5_d(this + 0x2c) as i32).wrapping_sub(1) || fase.wrapping_add(1) == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x30, 4)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}) original={:x?} rust={:x?}", nome, this, mo, mr));
}

unsafe extern "thiscall" fn comparar_fase_seguinte_da_c1a(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1856);
    cql5_comparar_passagem("fase_seguinte_da_c1a", this, || o(this), || fase_seguinte_da_c1a(this))
}

unsafe extern "thiscall" fn comparar_fase_seguinte_da_c1b(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1858);
    cql5_comparar_passagem("fase_seguinte_da_c1b", this, || o(this), || fase_seguinte_da_c1b(this))
}

unsafe extern "thiscall" fn comparar_fase_seguinte_da_third(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1860);
    cql5_comparar_passagem("fase_seguinte_da_third", this, || o(this), || fase_seguinte_da_third(this))
}

/// As linhas dos 4 clubes do play-off de acesso (2º contra 5º, 3º contra 4º: a lista é 5º, 2º, 4º, 3º) e do play-out
/// (14º contra 17º, 15º contra 16º: 17º, 14º, 16º, 15º), na ordem da lista que vai para a fase.
const CQL5_PLAYOFF: [usize; 4] = [4, 1, 3, 2]; // cql5: as linhas do play-off de acesso
const CQL5_PLAYOUT: [usize; 4] = [16, 13, 15, 14]; // cql5: as linhas do play-out

/// O corpo das duas (0x64ba70 e 0x64fc30, iguais fora o fonte, as linhas e a ordem de soltar as listas no fim):
/// 1) o campeão (a linha 0) vai para a Supercopa da Série C1 (a competição [[0xadadfc] + [0x9cf938] × 4], a lista de
///    inscritos [+0xa7] de 6 bytes): na vaga 0 se vazia, senão na vaga 1;
/// 2) a lista de 4 clubes (malloc 0x944e46 de 0x10: push; add esp, 4 -> cdecl; sem memória: a caixa da linha
///    `linhas[0]` e volta) com os da faixa do play-off (CQL5_PLAYOFF);
/// 3) o calendário da fase 0 (a virtual +0x3c: push 0; push &extra; push &rodadas; push 0; mov ecx, this -> thiscall,
///    ret 0x10; devolve o gabarito, `rodadas` e `extra` são palavras);
/// 4) a fase (operator new 0x944cf1 de 0xb2: push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage 0x522e00: 15 argumentos
///    empurrados, mov ecx, nova -> thiscall, ret 0x3c: a liga, os clubes (+0xbf, o byte com sinal), a lista, as
///    rodadas, o registro da competição (+4), o gabarito, o ano (+0x40), 0, 1, o `extra`, 0x14, 0, 0, 0 e as posições
///    dos 4 na tabela (registro local de 4 bytes: 4, 1, 3, 2)) vai para [+0xc][0]; sem fase: a caixa da linha
///    `linhas[1]` e volta; senão o sorteio da primeira rodada (Cup::set_up_round 0x51c800: push 0; mov ecx, fase ->
///    thiscall, ret 4) e solta o gabarito (0x9452ca: push; add esp, 4 -> cdecl);
/// 5) a mesma coisa com a fase 1 e os do play-out (CQL5_PLAYOUT; as posições 16, 13, 15, 14 no registro local e
///    1, 1 nos dois bytes) em [+0xc][1]; solta a lista e o gabarito (na ordem `lista_primeiro`); sem fase: a caixa da
///    linha `linhas[2]`; senão o sorteio da primeira rodada e +0x30 = 1.
/// Os bits de cima das palavras que a original empurra como dupla (os clubes, as rodadas, o ano e o `extra`) são lixo
/// da pilha ou de outros registradores; o construtor só lê a palavra (0x522e00: mov ax, word ptr [esp + ...]) e a
/// cópia passa zero neles.
unsafe fn cql5_criar_fases(this: usize, arquivo: usize, linhas: [u32; 3], lista_primeiro: bool) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let fase_de_copa: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
        core::mem::transmute(0x522e00usize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    let calendario: unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32, u32) -> usize = core::mem::transmute(cql5_d(cql5_d(this) + 0x3c));
    // a Supercopa da Série C1
    let copa = cql5_d(cql5_d(0xadadfc) + cql5_d(0x9cf938) * 4);
    let inscritos = cql5_d(copa + 0xa7);
    let campeao = cql5_clube(this, 0);
    if cql5_d(inscritos) != 0 {
        cql5_pd(inscritos + 6, campeao);
    } else {
        cql5_pd(inscritos, campeao);
    }
    let lista = alocar(0x10);
    if lista == 0 {
        aviso_em(arquivo, linhas[0]);
        return;
    }
    for (k, &i) in CQL5_PLAYOFF.iter().enumerate() {
        cql5_pd(lista + k * 4, cql5_clube(this, i));
    }
    let clubes = *((this + 0xbf) as *const i8) as i16 as u16 as u32;
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let registro = cql5_d(this + 4);
    let (mut rodadas, mut extra) = (0u32, 0u32);
    let gabarito = calendario(this, 0, &mut rodadas, &mut extra, 0);
    let posicoes: [u8; 4] = [4, 1, 3, 2]; // cql5: as posições do play-off
    let mut fase = novo(0xb2);
    if fase != 0 {
        fase = fase_de_copa(fase, this, clubes, lista, rodadas & 0xffff, registro, gabarito, ano, 0, 1, extra & 0xffff, 0x14, 0, 0, 0, posicoes.as_ptr());
    }
    cql5_pd(cql5_d(this + 0xc), fase);
    if cql5_d(cql5_d(this + 0xc)) == 0 {
        aviso_em(arquivo, linhas[1]);
        return;
    }
    sortear(cql5_d(cql5_d(this + 0xc)), 0);
    soltar(gabarito);
    let (mut rodadas, mut extra) = (0u32, 0u32);
    let calendario: unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32, u32) -> usize = core::mem::transmute(cql5_d(cql5_d(this) + 0x3c));
    let gabarito = calendario(this, 1, &mut rodadas, &mut extra, 0);
    for (k, &i) in CQL5_PLAYOUT.iter().enumerate() {
        cql5_pd(lista + k * 4, cql5_clube(this, i));
    }
    let posicoes: [u8; 4] = [0x10, 0xd, 0xf, 0xe];
    let mut fase = novo(0xb2);
    if fase != 0 {
        fase = fase_de_copa(fase, this, clubes, lista, rodadas & 0xffff, registro, gabarito, ano, 1, 1, extra & 0xffff, 0x14, 0, 0, 0, posicoes.as_ptr());
    }
    cql5_pd(cql5_d(this + 0xc) + 4, fase);
    if lista_primeiro {
        soltar(lista);
        soltar(gabarito);
    } else {
        soltar(gabarito);
        soltar(lista);
    }
    if cql5_d(cql5_d(this + 0xc) + 4) == 0 {
        aviso_em(arquivo, linhas[2]);
        return;
    }
    sortear(cql5_d(cql5_d(this + 0xc) + 4), 0);
    cql5_pd(this + 0x30, 1);
}

/// { 0x64ba70 } (thiscall, ret 0, ita_ser_c1a.cpp, quadro SEH só para o operator new): `ItaSerC1a::create_playoff_stages()`.
/// A regra da C1/A: o campeão sobe direto (e vai à Supercopa da C1); do 2º ao 5º, um play-off de acesso (2º × 5º,
/// 3º × 4º) dá a segunda vaga; do 14º ao 17º, um play-out (14º × 17º, 15º × 16º) decide quem cai junto com o 18º
/// [inferência pelas faixas da cqk_6 e pelos nomes do mapa]. Fonte 0x9ea474, linhas 0x27c, 0x290, 0x2b3.
unsafe extern "thiscall" fn fases_do_fim_da_c1a(this: usize) {
    cql5_criar_fases(this, 0x9ea474, [0x27c, 0x290, 0x2b3], true)
}

/// { 0x64fc30 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::create_play_off_stages()`, a mesma regra na C1/B.
/// Fonte 0x9ea558, linhas 0x281, 0x295, 0x2b8; no fim solta o gabarito antes da lista.
unsafe extern "thiscall" fn fases_do_fim_da_c1b(this: usize) {
    cql5_criar_fases(this, 0x9ea558, [0x281, 0x295, 0x2b8], false)
}

/// O comparador das duas: alocam e sorteiam: só a original. O plano, feito antes: o campeão e a vaga da Supercopa. Depois:
/// a vaga com o campeão, as duas fases em [+0xc] e +0x30 = 1.
unsafe fn cql5_comparar_fases(nome: &str, this: usize, original: impl FnOnce()) {
    let copa = cql5_d(cql5_d(0xadadfc) + cql5_d(0x9cf938) * 4);
    let inscritos = cql5_d(copa + 0xa7);
    let campeao = cql5_clube(this, 0);
    let vaga = if cql5_d(inscritos) != 0 { inscritos + 6 } else { inscritos };
    original();
    let fases = cql5_d(this + 0xc);
    let ok = cql5_d(vaga) == campeao && fases != 0 && cql5_d(fases) != 0 && cql5_d(fases + 4) != 0 && cql5_d(this + 0x30) == 1;
    PLACAR.registrar(ok, || format!("{}({:#x}): vaga da Supercopa {:#x} (campeão {:#x}), fases {:#x}, +0x30 = {}", nome, this, cql5_d(vaga), campeao, fases, cql5_d(this + 0x30) as i32));
}

unsafe extern "thiscall" fn comparar_fases_do_fim_da_c1a(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1857);
    cql5_comparar_fases("fases_do_fim_da_c1a", this, || o(this))
}

unsafe extern "thiscall" fn comparar_fases_do_fim_da_c1b(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1859);
    cql5_comparar_fases("fases_do_fim_da_c1b", this, || o(this))
}

// ---------- 278a leva: os construtores das quatro divisões inglesas (a Premier, a First, a Second e a Third Division: as regras, a tabela, os jogos, as estatísticas e as posições previstas na criação) (eng_prm.cpp, eng_first.cpp, eng_second.cpp, eng_third.cpp) ----------

unsafe fn cql6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cql6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O que muda de uma divisão inglesa para outra no construtor.
struct Cql6Divisao {
    /// a vtable da classe derivada
    vtabela: usize,
    /// a capacidade mínima do estádio (+0xe2) e os lugares sentados mínimos (+0xe4), quando a divisão os grava
    estadio: Option<(u16, u16)>,
    /// quantas fases além da liga (+0x2c): 1 = o play-off de acesso (o vetor [+0xc] de 1 fase, malloc de 4)
    fases: u32,
    /// set_up_rules e create_table da divisão (mov ecx, this -> thiscall, ret 0; devolvem 0 na falha)
    regras: usize,
    tabela: usize,
    /// o fonte e as linhas das cinco caixas (regras, tabela, jogos, ordem, estatísticas)
    arquivo: usize,
    linhas: [u32; 5],
}

/// O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para o operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o ano (+0x40, palavra), o registro da competição (+4), a vtable,
/// a regra de disciplina 9 (+0x50) e o estádio mínimo; tenta carregar o estado do arquivo temporário
/// (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4). Carregou: pronto. Senão, a liga nova:
/// fase 0xff (+0x44), fase atual −1 (+0x30), +0x2c fases (e o vetor [+0xc]), as regras, a tabela, os jogos
/// (League::create_fixtures 0x6835c0: mov ecx -> thiscall, ret 0), a ordem da tabela (League::sort_table 0x6827d0:
/// push 0; mov ecx -> thiscall, ret 4), as estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl;
/// Comp_Stats::Comp_Stats 0x49ee70: push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas
/// (League::set_predicted_positions 0x68a850: mov ecx -> thiscall, ret 0). Cada passo que falha mostra a caixa da
/// sua linha e encerra. Devolve this.
unsafe fn cql6_construir(this: usize, ano: u32, registro: usize, p: &Cql6Divisao) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68a850usize);
    base(this);
    cql6_pd(this + 4, registro);
    cql6_pd(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = 9; // cql6: a regra de disciplina das ligas inglesas
    if let Some((capacidade, sentados)) = p.estadio {
        core::ptr::write_unaligned((this + 0xe2) as *mut u16, capacidade);
        core::ptr::write_unaligned((this + 0xe4) as *mut u16, sentados);
    }
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cql_6 divisao inglesa nova");
    *((this + 0x44) as *mut u8) = 0xff;
    cql6_pd(this + 0x30, 0xffff_ffff);
    cql6_pd(this + 0x2c, p.fases as usize);
    if p.fases != 0 {
        let v = alocar(4);
        cql6_pd(this + 0xc, v);
    } else {
        cql6_pd(this + 0xc, 0);
    }
    if regras(this) == 0 {
        aviso_em(p.arquivo, p.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(p.arquivo, p.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(p.arquivo, p.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(p.arquivo, p.linhas[3]);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    cql6_pd(this + 8, s);
    if s == 0 {
        aviso_em(p.arquivo, p.linhas[4]);
        return this;
    }
    previstas(this);
    this
}

const CQL6_PREMIER: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969d1c,
    estadio: Some((0x2710, 0x2710)),
    fases: 0,
    regras: 0x574b70,
    tabela: 0x574a20,
    arquivo: 0x9c43b0,
    linhas: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};
const CQL6_FIRST: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969c68,
    estadio: None,
    fases: 1, // cql6: o play-off da First Division
    regras: 0x572d90,
    tabela: 0x572bf0,
    arquivo: 0x9c4374,
    linhas: [0x49, 0x50, 0x57, 0x60, 0x68],
};
const CQL6_SECOND: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969dd0,
    estadio: None,
    fases: 1, // cql6: o play-off da Second Division
    regras: 0x576780,
    tabela: 0x576630,
    arquivo: 0x9c43e8,
    linhas: [0x4a, 0x51, 0x58, 0x61, 0x68],
};
const CQL6_THIRD: Cql6Divisao = Cql6Divisao {
    vtabela: 0x969e84,
    estadio: Some((0x1770, 0x3e8)),
    fases: 1,
    regras: 0x5780c0,
    tabela: 0x577f70,
    arquivo: 0x9c4424,
    linhas: [0x4e, 0x55, 0x5c, 0x65, 0x6d],
};

/// { 0x573380 } (thiscall, ret 8, eng_prm.cpp): `EngPrm::EngPrm(ano, registro)`. A Premier: sem fases além da liga
/// (+0x2c = 0, +0xc = 0) e o estádio mínimo de 10.000 lugares, todos sentados.
unsafe extern "thiscall" fn construir_premier(this: usize, ano: u32, registro: usize) -> usize {
    cql6_construir(this, ano, registro, &CQL6_PREMIER)
}

/// { 0x571680 } (thiscall, ret 8, eng_first.cpp): `EngFirst::EngFirst(ano, registro)`. A First Division: 1 fase além da
/// liga (o play-off de acesso), sem estádio mínimo.
unsafe extern "thiscall" fn construir_first(this: usize, ano: u32, registro: usize) -> usize {
    cql6_construir(this, ano, registro, &CQL6_FIRST)
}

/// { 0x5754a0 } (thiscall, ret 8, eng_second.cpp): `EngSecond::EngSecond(ano, registro)`. A Second Division: 1 fase além
/// da liga (o play-off), sem estádio mínimo.
unsafe extern "thiscall" fn construir_second(this: usize, ano: u32, registro: usize) -> usize {
    cql6_construir(this, ano, registro, &CQL6_SECOND)
}

/// { 0x576dd0 } (thiscall, ret 8, eng_third.cpp): `EngThird::EngThird(ano, registro)`. A Third Division: 1 fase além da
/// liga (o play-off) e o estádio mínimo de 6.000 lugares, 1.000 sentados.
unsafe extern "thiscall" fn construir_third(this: usize, ano: u32, registro: usize) -> usize {
    cql6_construir(this, ano, registro, &CQL6_THIRD)
}

/// O comparador dos quatro: alocam (as estatísticas, a tabela, os jogos): só a original, conferindo o que ela grava:
/// a devolução, a vtable, o ano, o registro e a regra de disciplina; na liga nova (+0x44 = 0xff), +0x2c, o vetor das
/// fases, a fase atual −1 e as estatísticas em +8.
unsafe fn cql6_comparar(nome: &str, this: usize, ano: u32, registro: usize, p: &Cql6Divisao, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cql6_d(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cql6_d(this + 4) == registro && *((this + 0x50) as *const u8) == 9;
    if let Some((c, s)) = p.estadio {
        ok = ok && core::ptr::read_unaligned((this + 0xe2) as *const u16) == c && core::ptr::read_unaligned((this + 0xe4) as *const u16) == s;
    }
    if *((this + 0x44) as *const u8) == 0xff && cql6_d(this + 8) != 0 {
        ok = ok && cql6_d(this + 0x2c) == p.fases as usize && (cql6_d(this + 0xc) != 0) == (p.fases != 0);
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cql6_d(this), cql6_d(this + 0x2c), cql6_d(this + 0xc), cql6_d(this + 8)));
    r
}

unsafe extern "thiscall" fn comparar_construir_premier(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1861);
    cql6_comparar("construir_premier", this, ano, registro, &CQL6_PREMIER, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_first(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1862);
    cql6_comparar("construir_first", this, ano, registro, &CQL6_FIRST, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_second(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1863);
    cql6_comparar("construir_second", this, ano, registro, &CQL6_SECOND, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_third(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1864);
    cql6_comparar("construir_third", this, ano, registro, &CQL6_THIRD, || o(this, ano, registro))
}

// ---------- 278a leva: os construtores da Série A, da Série B e dos dois grupos da Série C1 italianas (as regras, a tabela, o calendário real de 2001/02, os jogos, as estatísticas e as posições previstas na criação) (ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------

unsafe fn cqm4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqm4_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O que muda de uma divisão italiana para outra no construtor.
struct Cqm4Divisao {
    vtabela: usize,
    /// a Série A e a Série B gravam +0xee = 0 (o campo próprio da classe [inferência: o desempate em curso]) e +0x4c = 1
    serie: bool,
    /// fases além da liga (+0x2c): 0 na A e na B; 2 na C1 (o play-off e o play-out; o vetor [+0xc] = malloc de 8)
    fases: u32,
    regras: usize,
    tabela: usize,
    /// o calendário real de 2001/02 pelo nome dos clubes (set_up_fixed_fixtures; mov ecx, this -> thiscall, ret 0)
    fixos: usize,
    arquivo: usize,
    /// as caixas: regras, tabela, jogos, ordem, estatísticas
    linhas: [u32; 5],
}

/// O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para o operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o registro (+4), a vtable, o ano (+0x40) e a regra de
/// disciplina 0x11 (+0x50, a das ligas italianas); tenta carregar o estado (League::load_from_temp_file 0x687b10: push
/// 1; mov ecx -> thiscall, ret 4). Carregou: pronto. Senão, a liga nova: na A e na B, +0xee = 0 e +0x4c = 1; fase 0xff
/// (+0x44), fase atual −1 (+0x30), +0x2c fases e o vetor [+0xc] (malloc 0x944e46 de 4 × fases: push; add esp, 4 ->
/// cdecl; 0 sem fases); as regras e a tabela (caixa e fim se falharem); o calendário real de 2001/02 (sem conferir);
/// os jogos (League::create_fixtures 0x6835c0), a ordem (League::sort_table 0x6827d0: push 0 -> thiscall, ret 4), as
/// estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas (0x68a850: mov ecx -> thiscall, ret 0).
/// Devolve this.
unsafe fn cqm4_construir(this: usize, ano: u32, registro: usize, p: &Cqm4Divisao) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.tabela);
    let fixos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(p.fixos);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68a850usize);
    base(this);
    cqm4_pd(this + 4, registro);
    cqm4_pd(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = 0x11; // cqm4: a regra de disciplina das ligas italianas
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqm_4 divisao italiana nova");
    if p.serie {
        *((this + 0xee) as *mut u8) = 0;
        cqm4_pd(this + 0x4c, 1); // cqm4: a marca do histórico da A e da B
    }
    *((this + 0x44) as *mut u8) = 0xff;
    cqm4_pd(this + 0x30, 0xffff_ffff);
    cqm4_pd(this + 0x2c, p.fases as usize);
    if p.fases != 0 {
        let v = alocar(p.fases * 4);
        cqm4_pd(this + 0xc, v);
    } else {
        cqm4_pd(this + 0xc, 0);
    }
    if regras(this) == 0 {
        aviso_em(p.arquivo, p.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(p.arquivo, p.linhas[1]);
        return this;
    }
    fixos(this); // cqm4: o calendário real de 2001/02
    if jogos(this) == 0 {
        aviso_em(p.arquivo, p.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(p.arquivo, p.linhas[3]);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    cqm4_pd(this + 8, s);
    if s == 0 {
        aviso_em(p.arquivo, p.linhas[4]);
        return this;
    }
    previstas(this); // cqm4: as posições previstas
    this
}

const CQM4_SERIE_A: Cqm4Divisao = Cqm4Divisao {
    vtabela: 0x96c3cc,
    serie: true,
    fases: 0,
    regras: 0x641640,
    tabela: 0x641510,
    fixos: 0x642a70,
    arquivo: 0x9ea2a8,
    linhas: [0x53, 0x5a, 0x64, 0x6d, 0x74],
};
const CQM4_SERIE_B: Cqm4Divisao = Cqm4Divisao {
    vtabela: 0x96c480,
    serie: true,
    fases: 0,
    regras: 0x6468a0,
    tabela: 0x646780,
    fixos: 0x647230,
    arquivo: 0x9ea384,
    linhas: [0x4b, 0x52, 0x5c, 0x65, 0x6c],
};
const CQM4_C1A: Cqm4Divisao = Cqm4Divisao {
    vtabela: 0x96c534,
    serie: false,
    fases: 2, // cqm4: o play-off e o play-out da C1/A
    regras: 0x64b770,
    tabela: 0x64b670,
    fixos: 0x64be90,
    arquivo: 0x9ea474,
    linhas: [0x49, 0x50, 0x5a, 0x63, 0x6a],
};
const CQM4_C1B: Cqm4Divisao = Cqm4Divisao {
    vtabela: 0x96c5e8,
    serie: false,
    fases: 2,
    regras: 0x64f920,
    tabela: 0x64f820,
    fixos: 0x650100,
    arquivo: 0x9ea558,
    linhas: [0x4a, 0x51, 0x5b, 0x64, 0x6b],
};

/// { 0x6406d0 } (thiscall, ret 8, ita_ser_a.cpp): `ItaSerA::ItaSerA(ano, registro)`: a Série A, só a liga (o jogo de
/// desempate nasce no fim, pela 0x642880).
unsafe extern "thiscall" fn construir_serie_a(this: usize, ano: u32, registro: usize) -> usize {
    cqm4_construir(this, ano, registro, &CQM4_SERIE_A)
}

/// { 0x645810 } (thiscall, ret 8, ita_ser_b.cpp): `ItaSerB::ItaSerB(ano, registro)`: a Série B, só a liga.
unsafe extern "thiscall" fn construir_serie_b(this: usize, ano: u32, registro: usize) -> usize {
    cqm4_construir(this, ano, registro, &CQM4_SERIE_B)
}

/// { 0x64aa70 } (thiscall, ret 8, ita_ser_c1a.cpp): `ItaSerC1a::ItaSerC1a(ano, registro)`: a C1/A, a liga e 2 fases
/// (o play-off de acesso e o play-out).
unsafe extern "thiscall" fn construir_c1a(this: usize, ano: u32, registro: usize) -> usize {
    cqm4_construir(this, ano, registro, &CQM4_C1A)
}

/// { 0x64ec20 } (thiscall, ret 8, ita_ser_c1b.cpp): `ItaSerC1b::ItaSerC1b(ano, registro)`: a C1/B, como a C1/A.
unsafe extern "thiscall" fn construir_c1b(this: usize, ano: u32, registro: usize) -> usize {
    cqm4_construir(this, ano, registro, &CQM4_C1B)
}

/// O comparador dos quatro: alocam: só a original, conferindo o que ela grava: a devolução, a vtable, o ano, o
/// registro e a disciplina; na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c, o vetor das fases e, na A e na
/// B, +0x4c = 1 e +0xee = 0.
unsafe fn cqm4_comparar(nome: &str, this: usize, ano: u32, registro: usize, p: &Cqm4Divisao, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqm4_d(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqm4_d(this + 4) == registro && *((this + 0x50) as *const u8) == 0x11;
    if *((this + 0x44) as *const u8) == 0xff && cqm4_d(this + 8) != 0 {
        ok = ok && cqm4_d(this + 0x2c) == p.fases as usize && (cqm4_d(this + 0xc) != 0) == (p.fases != 0);
        if p.serie {
            ok = ok && *((this + 0xee) as *const u8) == 0;
        }
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", nome, this, r, cqm4_d(this), cqm4_d(this + 0x2c), cqm4_d(this + 0xc), cqm4_d(this + 8)));
    r
}

unsafe extern "thiscall" fn comparar_construir_serie_a(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1865);
    cqm4_comparar("construir_serie_a", this, ano, registro, &CQM4_SERIE_A, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_serie_b(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1866);
    cqm4_comparar("construir_serie_b", this, ano, registro, &CQM4_SERIE_B, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_c1a(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1867);
    cqm4_comparar("construir_c1a", this, ano, registro, &CQM4_C1A, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_c1b(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1868);
    cqm4_comparar("construir_c1b", this, ano, registro, &CQM4_C1B, || o(this, ano, registro))
}

// ---------- 278a leva: os construtores da segunda e da terceira divisões francesas e da Primera e da Segunda espanholas (as regras, a tabela, os jogos, as estatísticas, as posições previstas e a divisão de baixo espanhola na criação) (fra_second.cpp, fra_third.cpp, spa_second.cpp, spa_first.cpp) ----------

unsafe fn cqm5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqm5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O que muda de uma divisão para outra no construtor.
struct Cqm5Divisao {
    vtabela: usize,
    /// a regra de disciplina (+0x50): 0xc a francesa, 0x1a a espanhola
    disciplina: u8,
    regras: usize,
    tabela: usize,
    arquivo: usize,
    /// as caixas: regras, tabela, jogos, ordem, estatísticas
    linhas: [u32; 5],
    /// a Primera cria no fim a divisão de baixo espanhola (+0xee)
    baixo: bool,
}

/// O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para os operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o registro (+4), a vtable, o ano (+0x40) e a disciplina
/// (+0x50); tenta carregar o estado (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4).
/// Carregou: pronto. Senão, a liga nova, só a liga: fase 0xff (+0x44), fase atual −1 (+0x30), +0x2c = 0, +0xc = 0; as
/// regras, a tabela, os jogos (League::create_fixtures 0x6835c0), a ordem (League::sort_table 0x6827d0: push 0 ->
/// thiscall, ret 4), as estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats 0x49ee70:
/// push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas (0x68a850: mov ecx -> thiscall, ret 0);
/// cada passo que falha mostra a caixa da sua linha e encerra. Na Primera, por fim: se a nação da competição
/// ([+4]+0x5d) não tem o bit 4 de +0x11c [inferência: a divisão de baixo jogável], a divisão de baixo espanhola
/// (operator new de 0xee; SpaLower::SpaLower 0x851020: push ano; mov ecx, novo -> thiscall, ret 4) vai para +0xee
/// (0 sem memória); com o bit, +0xee = 0. Devolve this.
unsafe fn cqm5_construir(this: usize, ano: u32, registro: usize, p: &Cqm5Divisao) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.regras);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68a850usize);
    let baixo: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x851020usize);
    base(this);
    cqm5_pd(this + 4, registro);
    cqm5_pd(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = p.disciplina;
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqm_5 divisao nova");
    *((this + 0x44) as *mut u8) = 0xff;
    cqm5_pd(this + 0x30, 0xffff_ffff);
    cqm5_pd(this + 0x2c, 0);
    cqm5_pd(this + 0xc, 0);
    if regras(this) == 0 {
        aviso_em(p.arquivo, p.linhas[0]);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(p.arquivo, p.linhas[1]);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(p.arquivo, p.linhas[2]);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(p.arquivo, p.linhas[3]);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    cqm5_pd(this + 8, s);
    if s == 0 {
        aviso_em(p.arquivo, p.linhas[4]);
        return this;
    }
    previstas(this);
    if p.baixo {
        let nacao = cqm5_d(cqm5_d(this + 4) + 0x5d);
        if *((nacao + 0x11c) as *const u8) & 4 == 0 { // cqm5: a divisão de baixo espanhola
            crate::comparacao::ramo("cqm_5 divisao de baixo espanhola");
            let e = novo(0xee);
            // ponytail: a original empurra o ano na palavra de baixo do ecx, com o resto do ponteiro do registro em
            // cima; a 0x851020 só lê a palavra (mov ax, word ptr [esp + 0x21c]).
            let b = if e != 0 { baixo(e, core::ptr::read_unaligned((this + 0x40) as *const u16) as u32) } else { 0 };
            cqm5_pd(this + 0xee, b);
        } else {
            cqm5_pd(this + 0xee, 0);
        }
    }
    this
}

const CQM5_FRANCA_2: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96ac2c,
    disciplina: 0xc, // cqm5: a regra de disciplina francesa
    regras: 0x5bf0a0,
    tabela: 0x5bef10,
    arquivo: 0x9ccc0c,
    linhas: [0x43, 0x4a, 0x51, 0x5a, 0x61],
    baixo: false,
};
const CQM5_FRANCA_3: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96ad80,
    disciplina: 0xc,
    regras: 0x5c0700,
    tabela: 0x5c05d0,
    arquivo: 0x9ccc80,
    linhas: [0x42, 0x49, 0x50, 0x59, 0x60],
    baixo: false,
};
const CQM5_SEGUNDA: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96fbd0,
    disciplina: 0x1a, // cqm5: a regra de disciplina espanhola
    regras: 0x855cc0,
    tabela: 0x855b40,
    arquivo: 0xa828bc,
    linhas: [0x46, 0x4d, 0x54, 0x5b, 0x62],
    baixo: false,
};
const CQM5_PRIMERA: Cqm5Divisao = Cqm5Divisao {
    vtabela: 0x96fa68,
    disciplina: 0x1a,
    regras: 0x84fdf0,
    tabela: 0x84fc70,
    arquivo: 0xa82844,
    linhas: [0x47, 0x4e, 0x55, 0x5c, 0x63],
    baixo: true,
};

/// { 0x5be330 } (thiscall, ret 8, fra_second.cpp): `FraSecond::FraSecond(ano, registro)`: a segunda divisão francesa
/// (a Division 2), disciplina 0xc, só a liga.
unsafe extern "thiscall" fn construir_franca_2(this: usize, ano: u32, registro: usize) -> usize {
    cqm5_construir(this, ano, registro, &CQM5_FRANCA_2)
}

/// { 0x5bfb40 } (thiscall, ret 8, fra_third.cpp): `FraThird::FraThird(ano, registro)`: a terceira divisão francesa (o
/// National), disciplina 0xc, só a liga.
unsafe extern "thiscall" fn construir_franca_3(this: usize, ano: u32, registro: usize) -> usize {
    cqm5_construir(this, ano, registro, &CQM5_FRANCA_3)
}

/// { 0x8539a0 } (thiscall, ret 8, spa_second.cpp): `SpaSecond::SpaSecond(ano, registro)`: a Segunda espanhola,
/// disciplina 0x1a, só a liga.
unsafe extern "thiscall" fn construir_segunda(this: usize, ano: u32, registro: usize) -> usize {
    cqm5_construir(this, ano, registro, &CQM5_SEGUNDA)
}

/// { 0x84e130 } (thiscall, ret 8, spa_first.cpp, 606 bytes): `SpaFirst::SpaFirst(ano, registro)`: a Primera,
/// disciplina 0x1a, só a liga, e a divisão de baixo espanhola em +0xee.
unsafe extern "thiscall" fn construir_primera(this: usize, ano: u32, registro: usize) -> usize {
    cqm5_construir(this, ano, registro, &CQM5_PRIMERA)
}

/// O comparador dos quatro: alocam: só a original, conferindo a devolução, a vtable, o ano, o registro e a disciplina;
/// na liga nova (+0x44 = 0xff, com as estatísticas), +0x2c = 0 e +0xc = 0; na Primera, a divisão de baixo em +0xee
/// conforme o bit 4 da nação.
unsafe fn cqm5_comparar(nome: &str, this: usize, ano: u32, registro: usize, p: &Cqm5Divisao, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this && cqm5_d(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqm5_d(this + 4) == registro && *((this + 0x50) as *const u8) == p.disciplina;
    if *((this + 0x44) as *const u8) == 0xff && cqm5_d(this + 8) != 0 {
        ok = ok && cqm5_d(this + 0x2c) == 0 && cqm5_d(this + 0xc) == 0;
        if p.baixo {
            let nacao = cqm5_d(cqm5_d(this + 4) + 0x5d);
            let tem = cqm5_d(this + 0xee) != 0;
            ok = ok && tem == (*((nacao + 0x11c) as *const u8) & 4 == 0);
        }
    }
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +0x2c = {}, +8 = {:#x}", nome, this, r, cqm5_d(this), cqm5_d(this + 0x2c), cqm5_d(this + 8)));
    r
}

unsafe extern "thiscall" fn comparar_construir_franca_2(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1869);
    cqm5_comparar("construir_franca_2", this, ano, registro, &CQM5_FRANCA_2, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_franca_3(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1870);
    cqm5_comparar("construir_franca_3", this, ano, registro, &CQM5_FRANCA_3, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_segunda(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1871);
    cqm5_comparar("construir_segunda", this, ano, registro, &CQM5_SEGUNDA, || o(this, ano, registro))
}

unsafe extern "thiscall" fn comparar_construir_primera(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1872);
    cqm5_comparar("construir_primera", this, ano, registro, &CQM5_PRIMERA, || o(this, ano, registro))
}

// ---------- 278a leva: a divisão de baixo espanhola (o construtor e a tabela com os times B e os melhores da Segunda B) e a tabela da Segunda espanhola (os times B que não sobem) (spa_lower.cpp, spa_second.cpp) ----------

unsafe fn cqm6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqm6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqm6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de competição de id [g] (vetor [0xae23d0], 0x6b bytes).
unsafe fn cqm6_registro(g: usize) -> usize {
    cqm6_d(0xae23d0).wrapping_add(cqm6_d(g).wrapping_mul(0x6b))
}

/// A competição do clube (+0x57, o id no +0) é a de id [g]?
unsafe fn cqm6_na(c: usize, g: usize) -> bool {
    let comp = cqm6_d(c + 0x57);
    comp != 0 && cqm6_d(comp) == cqm6_d(g)
}

/// { 0x851020 } (thiscall, ret 4, spa_lower.cpp, quadro SEH só no caminho de exceção): `SpaLower::SpaLower(ano)`, a
/// divisão de baixo espanhola (criada pela Primera quando a nação não tem o bit 4 de +0x11c): a base (League::League
/// 0x682200: mov ecx -> thiscall, ret 0), a vtable 0x96fb1c, o ano (+0x40), a regra de disciplina 0xff (nenhuma) e o
/// registro da competição de id [0x9cf6b0] (+4). Tenta carregar o estado (0x687b10: push 1 -> thiscall, ret 4);
/// senão, só a liga (+0x44 = 0xff, +0x30 = −1, +0x2c = 0), as regras (0x853790), a tabela (0x8533d0) e os jogos
/// (League::create_fixtures 0x6835c0), todos mov ecx -> thiscall, ret 0; cada falha, a caixa da sua linha (0x3f,
/// 0x46, 0x4d do fonte 0xa82880) e fim. Devolve this.
unsafe extern "thiscall" fn construir_divisao_de_baixo_espanhola(this: usize, ano: u32) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x853790usize);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8533d0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    base(this);
    cqm6_pd(this, 0x96fb1c);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = 0xff;
    cqm6_pd(this + 4, cqm6_registro(0x9cf6b0));
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqm_6 divisao de baixo nova");
    *((this + 0x44) as *mut u8) = 0xff;
    cqm6_pd(this + 0x30, 0xffff_ffff);
    cqm6_pd(this + 0x2c, 0);
    if regras(this) == 0 {
        aviso_em(0xa82880, 0x3f);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(0xa82880, 0x46);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(0xa82880, 0x4d);
    }
    this
}

/// Aloca e sorteia (pela tabela): só a original, conferindo a devolução, a vtable, o ano, a disciplina e o registro.
unsafe extern "thiscall" fn comparar_construir_divisao_de_baixo_espanhola(this: usize, ano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(1873);
    let r = o(this, ano);
    let ok = r == this && cqm6_d(this) == 0x96fb1c && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && *((this + 0x50) as *const u8) == 0xff && cqm6_d(this + 4) == cqm6_registro(0x9cf6b0);
    PLACAR.registrar(ok, || format!("construir_divisao_de_baixo_espanhola({:#x}): devolveu {:#x}, vtable {:#x}, +4 = {:#x}", this, r, cqm6_d(this), cqm6_d(this + 4)));
    r
}

/// O clube de cima do clube `c` (club_get_linked_b_team 0x540a50: push 1; push &eh_a; push c; add esp, 0xc -> cdecl),
/// se `c` é o time B dele (eh_a = 0) e ele está na competição de id [g].
unsafe fn cqm6_time_b_de(c: usize, g: usize) -> bool {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut eh_a: u32 = 0;
    let p = parceiro(c, &mut eh_a, 1);
    p != 0 && eh_a == 0 && cqm6_na(p, g)
}

/// { 0x855b40 } (thiscall, ret 0, spa_second.cpp): `SpaSecond::create_table()`: a Segunda tem 22 clubes (+0x3e),
/// a tabela (malloc 0x944e46 de 0x512: push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x298 e 0) recebe, na
/// ordem do banco ([0xae23b4], [0xae235c] clubes de 0x245 bytes), os clubes da competição de id [0x9cf628] até encher
/// (League::init_table_row 0x687430: push 0; push 0; push clube; push linha (a palavra empurrada tem o byte da linha e o
/// lixo da pilha em cima; a 0x687430 só lê o byte); mov ecx, this -> thiscall, ret 0x10); o time B de um clube da
/// Primera ([0x9cf624]) leva o destino −4 (+0x37 = 0xfc: não pode subir). Cheia: o fundo de prêmios (+0x28 =
/// League::calc_prize_fund 0x682f70: push 50000; mov ecx -> thiscall, ret 4), +0xe1 = 1 e 1; senão 0.
unsafe extern "thiscall" fn tabela_da_segunda(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let premios: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x682f70usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x16);
    let t = alocar(0x512);
    cqm6_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xa828bc, 0x298);
        return 0;
    }
    let mut k: u8 = 0;
    let n = cqm6_d(0xae235c) as i32;
    let base = cqm6_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245; // cqm6: o clube i do banco
        if cqm6_na(c, 0x9cf628) {
            iniciar(this, k as u32, c, 0, 0);
            if cqm6_time_b_de(c, 0x9cf624) {
                crate::comparacao::ramo("cqm_6 time b na segunda");
                *((cqm6_d(this + 0xb1) + (k as i8 as isize as usize).wrapping_mul(0x3b) + 0x37) as *mut u8) = 0xfc;
            }
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqm6_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    crate::comparacao::ramo("cqm_6 tabela da segunda");
    if k as i8 as i16 != cqm6_w(this + 0x3e) {
        return 0;
    }
    let f = premios(this, 0xc350);
    cqm6_pd(this + 0x28, f as usize);
    *((this + 0xe1) as *mut u8) = 1;
    1
}

/// Aloca: só a original. O plano, feito antes: os clubes da Segunda na ordem do banco (até 22) e a marca dos times B.
/// Depois: a devolução, +0x3e e as linhas (o clube e o destino −4 dos marcados).
unsafe extern "thiscall" fn comparar_tabela_da_segunda(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1874);
    let n = cqm6_d(0xae235c) as i32;
    let base = cqm6_d(0xae23b4);
    let mut plano: Vec<(usize, bool)> = Vec::new();
    for i in 0..n.max(0) as usize {
        let c = base + i * 0x245;
        if cqm6_na(c, 0x9cf628) {
            plano.push((c, cqm6_time_b_de(c, 0x9cf624)));
            if plano.len() == 0x16 {
                break;
            }
        }
    }
    let r = o(this);
    let t = cqm6_d(this + 0xb1);
    let mut ok = r == (plano.len() == 0x16) as u32 && cqm6_w(this + 0x3e) == 0x16 && t != 0;
    if ok {
        for (k, &(c, b)) in plano.iter().enumerate() {
            let l = t + k * 0x3b;
            ok = ok && cqm6_d(l) == c && (!b || *((l + 0x37) as *const u8) == 0xfc);
        }
    }
    PLACAR.registrar(ok, || format!("tabela_da_segunda({:#x}): devolveu {}, plano {:x?}", this, r, plano));
    r
}

/// Os candidatos da divisão de baixo espanhola, do registro local de 100 da original: os clubes (todos, na ordem do
/// banco, até 100) das competições de id [0x9cf62c], [0x9cf630], [0x9cf634], [0x9cf638] [inferência: os 4 grupos da
/// Segunda B] ou, se espanhóis, da de id [0x9cf764] [inferência: a divisão genérica de baixo], que não são time B nem
/// time A de ninguém (club_get_linked_b_team devolve 0).
unsafe fn cqm6_candidatos() -> Vec<usize> {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut v = Vec::new();
    let n = cqm6_d(0xae235c) as i32;
    let base = cqm6_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n && v.len() < 100 {
        let c = base + (i as u16 as usize) * 0x245;
        let comp = cqm6_d(c + 0x57);
        let nacao = cqm6_d(c + 0x53);
        if comp != 0 && nacao != 0 {
            let id = cqm6_d(comp);
            let grupo = [0x9cf62c, 0x9cf630, 0x9cf634, 0x9cf638].iter().any(|&g| id == cqm6_d(g));
            if grupo || (cqm6_d(nacao) == cqm6_d(0x9cf488) && id == cqm6_d(0x9cf764)) {
                let mut eh_a: u32 = 0;
                if parceiro(c, &mut eh_a, 1) == 0 {
                    v.push(c);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    v
}

/// Os times B espanhóis (fora da Primera e da Segunda) cujo clube de cima está na competição de id [g], na ordem do
/// banco, até 22 linhas (`k` já usadas). Os últimos `2 × [0xae2350]` clubes do banco ficam de fora [inferência: os
/// clubes criados depois, sem divisão].
unsafe fn cqm6_times_b(g: usize, k: &mut u8, linhas: i16, mut por: impl FnMut(usize, bool)) {
    let n = (cqm6_d(0xae235c) as i32).wrapping_sub((cqm6_d(0xae2350) as i32).wrapping_mul(2));
    let base = cqm6_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245;
        let comp = cqm6_d(c + 0x57);
        let nacao = cqm6_d(c + 0x53);
        if comp != 0 && nacao != 0 && cqm6_d(nacao) == cqm6_d(0x9cf488) && cqm6_d(comp) != cqm6_d(0x9cf624) && cqm6_d(comp) != cqm6_d(0x9cf628) {
            let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
            let mut eh_a: u32 = 0;
            let p = parceiro(c, &mut eh_a, 1);
            let entra = p != 0 && eh_a == 0 && cqm6_d(p + 0x57) != 0 && (*k as i8 as i16) < linhas && cqm6_d(cqm6_d(p + 0x57)) == cqm6_d(g);
            if entra {
                *k = k.wrapping_add(1);
            }
            por(c, entra);
        }
        i = i.wrapping_add(1);
    }
}

/// { 0x8533d0 } (thiscall, ret 0, spa_lower.cpp, 947 bytes): `SpaLower::create_table()`: a divisão de baixo tem 22
/// clubes (+0x3e), a tabela (malloc de 0x512; sem memória, a caixa da linha 0x2ac e 0). Primeiro, os clubes que estão
/// nela (a competição de id [0x9cf6b0]) passam à de id [0x9cf764] [inferência: a divisão genérica]. Depois enche, na
/// ordem do banco: os times B espanhóis cujo clube de cima está na Primera, depois os da Segunda (cada um com +0x60 =
/// o registro desta competição; os outros times espanhóis de fora das duas, com +0x60 = 0); o resto com os
/// candidatos (cqm6_candidatos), ordenados pela reputação e com os 3 × faltam primeiros embaralhados
/// (comp_util_sort_and_shuffle_top_clubs 0x4afbd0: push faltam; push quantos; push lista; add esp, 0xc -> cdecl; sorteia),
/// cada um com +0x60 = o registro. As linhas: League::init_table_row 0x687430 (push 0; push 0; push clube; push eax
/// (al = a linha, os bits de cima são lixo de outra conta; a 0x687430 só lê o byte); mov ecx, this -> thiscall, ret
/// 0x10). Devolve se encheu.
unsafe extern "thiscall" fn tabela_da_divisao_de_baixo_espanhola(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x16);
    let t = alocar(0x512);
    cqm6_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xa82880, 0x2ac);
        return 0;
    }
    // os clubes desta divisão passam à genérica
    let n = (cqm6_d(0xae235c) as i32).wrapping_sub((cqm6_d(0xae2350) as i32).wrapping_mul(2));
    let base = cqm6_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245;
        if cqm6_na(c, 0x9cf6b0) {
            cqm6_pd(c + 0x57, cqm6_registro(0x9cf764));
        }
        i = i.wrapping_add(1);
    }
    let mut k: u8 = 0;
    for g in [0x9cf624usize, 0x9cf628] { // cqm6: os times B da Primera e da Segunda
        let linhas = cqm6_w(this + 0x3e);
        // a conta das linhas para a condição anda junto com `k`
        let mut kk = k;
        cqm6_times_b(g, &mut kk, linhas, |c, entra| {
            if entra {
                iniciar(this, k as u32, c, 0, 0);
                k = k.wrapping_add(1);
                cqm6_pd(c + 0x60, cqm6_d(this + 4));
            } else {
                cqm6_pd(c + 0x60, 0);
            }
        });
    }
    crate::comparacao::ramo("cqm_6 tabela da divisao de baixo");
    let mut lista = cqm6_candidatos();
    let quantos = lista.len() as u16 as u32;
    let faltam = (cqm6_w(this + 0x3e) as u16).wrapping_sub(k as i8 as i16 as u16) as u32; // cqm6: quantos faltam
    lista.resize(100, 0);
    embaralhar(lista.as_mut_ptr() as usize, quantos, faltam);
    let mut j: i16 = 0;
    while j < quantos as u16 as i16 && (k as i8 as i16) < cqm6_w(this + 0x3e) {
        let c = lista[j as usize];
        iniciar(this, k as u32, c, 0, 0);
        k = k.wrapping_add(1);
        cqm6_pd(c + 0x60, cqm6_d(this + 4));
        j += 1;
    }
    (k as i8 as i16 == cqm6_w(this + 0x3e)) as u32
}

/// Aloca e sorteia: só a original. O plano, feito antes: os times B (da Primera e da Segunda) das primeiras linhas.
/// Depois: +0x3e e o clube das primeiras linhas. O +0x60 não entra: a segunda passada (a dos times B da Segunda) zera o
/// +0x60 de todo time espanhol que não entra nela, inclusive os da Primera que a primeira acabou de pôr na tabela
/// (comportamento da original, que a cópia repete; achado pela comparação de 8 ligas, 05/11/2026).
unsafe extern "thiscall" fn comparar_tabela_da_divisao_de_baixo_espanhola(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1875);
    let mut plano: Vec<usize> = Vec::new();
    let mut k: u8 = 0;
    for g in [0x9cf624usize, 0x9cf628] {
        cqm6_times_b(g, &mut k, 0x16, |c, entra| {
            if entra {
                plano.push(c);
            }
        });
    }
    let r = o(this);
    let t = cqm6_d(this + 0xb1);
    let mut ok = cqm6_w(this + 0x3e) == 0x16 && t != 0;
    if ok {
        for (i, &c) in plano.iter().enumerate() {
            ok = ok && cqm6_d(t + i * 0x3b) == c;
        }
    }
    PLACAR.registrar(ok, || format!("tabela_da_divisao_de_baixo_espanhola({:#x}): devolveu {}, plano {:x?}, tabela {:x?}", this, r, plano,
        (0..plano.len()).map(|i| if t != 0 { cqm6_d(t + i * 0x3b) } else { 0 }).collect::<Vec<_>>()));
    r
}

// ---------- 278a leva: as ligas de fundo francesas: a CFA (4 grupos de 18) e a divisão de baixo (2 grupos de 20): os construtores, as regras e os destrutores (fra_cfa.cpp, fra_lower.cpp) ----------

unsafe fn cqn4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqn4_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O registro de competição de id [g] (vetor [0xae23d0], 0x6b bytes).
unsafe fn cqn4_registro(g: usize) -> usize {
    cqn4_d(0xae23d0).wrapping_add(cqn4_d(g).wrapping_mul(0x6b))
}

/// { 0x5ba0a0 } (thiscall, ret 0, fra_cfa.cpp; usada também pela divisão de baixo): `FraCfa::set_up_rules()`: turno e
/// returno (+0x3c = 2), 3 pontos por vitória (+0xc2) e 1 por empate (+0xc3), a categoria 2 das partidas (+0xc4), os
/// desempates 4, 1, 2 (+0xc5..+0xc7: [inferência] a posição, o saldo, os gols), o tipo 1 (+0x42), nenhuma vaga de
/// acesso, de play-off nem de rebaixamento (+0xbe..+0xc1 = 0), a liga de cima de id [0x9cf624] (+0x1c) e nenhuma de
/// baixo (+0x20 = −1), os bits de regra 0x43 (+0xd9); por fim o calendário (a virtual +0x3c: push 0; push &+0x3a;
/// push &+0xa9; push −1; mov ecx, esi -> thiscall, ret 0x10) em +0xba. Devolve 1.
unsafe extern "thiscall" fn regras_da_cfa(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3c) as *mut u16, 2);
    *((this + 0xc4) as *mut u8) = 2;
    *((this + 0xc7) as *mut u8) = 2;
    *((this + 0xc2) as *mut u8) = 3; // cqn4: os pontos por vitória nas ligas de fundo francesas
    *((this + 0xc3) as *mut u8) = 1;
    *((this + 0x42) as *mut u8) = 1;
    *((this + 0xc5) as *mut u8) = 4;
    *((this + 0xc6) as *mut u8) = 1;
    *((this + 0xbe) as *mut u8) = 0;
    *((this + 0xbf) as *mut u8) = 0;
    *((this + 0xc0) as *mut u8) = 0;
    *((this + 0xc1) as *mut u8) = 0;
    // ponytail: a original grava em +0x1c o id de [0x9cf624] (o mesmo global que a Segunda espanhola usa para a
    // Primera); fiel.
    cqn4_pd(this + 0x1c, cqn4_d(0x9cf624));
    cqn4_pd(this + 0x20, 0xffff_ffff);
    core::ptr::write_unaligned((this + 0xd9) as *mut u16, 0x43); // cqn4: os bits de regra das ligas de fundo
    let calendario: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqn4_d(cqn4_d(this) + 0x3c));
    let g = calendario(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    cqn4_pd(this + 0xba, g);
    1
}

/// O calendário (a virtual +0x3c) aloca: só a original, conferindo os campos e a devolução 1.
unsafe extern "thiscall" fn comparar_regras_da_cfa(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1876);
    let r = o(this);
    let b = |k: usize| *((this + k) as *const u8);
    let ok = r == 1 && core::ptr::read_unaligned((this + 0x3c) as *const u16) == 2 && b(0xc2) == 3 && b(0xc3) == 1 && b(0xc4) == 2
        && b(0xc5) == 4 && b(0xc6) == 1 && b(0xc7) == 2 && b(0x42) == 1 && b(0xbe) == 0 && b(0xbf) == 0 && b(0xc0) == 0 && b(0xc1) == 0
        && cqn4_d(this + 0x1c) == cqn4_d(0x9cf624) && cqn4_d(this + 0x20) == 0xffff_ffff
        && core::ptr::read_unaligned((this + 0xd9) as *const u16) == 0x43;
    PLACAR.registrar(ok, || format!("regras_da_cfa({:#x}): devolveu {}, campos fora do esperado", this, r));
    r
}

/// O que muda entre as duas ligas de fundo.
struct Cqn4Fundo {
    vtabela: usize,
    /// o id do registro da competição (+4)
    competicao: usize,
    /// os ids das competições cujos clubes entram todos; os outros vêm da divisão genérica ([0x9cf764])
    todas: &'static [usize],
    /// só os da nação de id [g] (a divisão de baixo: a França) entre os da genérica, e sem competição de reserva
    /// (+0x60 = 0); a CFA pede, ao contrário, a competição de reserva = a própria ([[+0x60]] = [competicao])
    nacao: Option<usize>,
    /// o máximo da lista (72 a CFA, 40 a de baixo) e o máximo dos da genérica (34 e 20)
    maximo: i16,
    genericos: i16,
    /// os grupos e a tabela de cada um (thiscall(liga, lista), ret 4: 0x5b9fa0 e 0x5be230)
    grupos: usize,
    por_grupo: usize,
    tabela: usize,
    arquivo: usize,
    /// as caixas: regras, tabela do grupo, jogos
    linhas: [u32; 3],
}

/// O corpo dos dois construtores (thiscall(this, ano), ret 4, quadro SEH só no caminho de exceção; sem estado salvo):
/// a base (League::League 0x682200: mov ecx -> thiscall, ret 0), a vtable, o ano (+0x40), +0x2c = 0, sem disciplina
/// (+0x50 = 0xff), o registro (+4), a fase 0xff (+0x44) e −1 (+0x30); as regras (0x5ba0a0: mov ecx -> thiscall, ret 0;
/// falhando, a caixa e fim). Depois a lista dos clubes, na ordem do banco (os [0xae235c] − 2 × [0xae2350] primeiros),
/// até o máximo: os das competições `todas`; e, até o máximo dos genéricos, os da divisão genérica com a condição da
/// liga. A lista é embaralhada (comp_util_shuffle_pointer_array 0x4afb40: push quantos; push lista; add esp, 8 ->
/// cdecl; sorteia) e cada grupo, com os seus clubes em sequência na lista, ganha a tabela (falhando, a caixa), os jogos
/// (League::create_fixtures 0x6835c0: mov ecx -> thiscall, ret 0; falhando, a caixa) e a tabela solta
/// (League::free_table 0x687970: push 0; mov ecx -> thiscall, ret 4): os jogos do grupo ficam no calendário do jogo e a
/// liga segue sem tabela. A lista é um registro local de 72 ou 40 ponteiros; a cópia usa um vetor zerado (a carreira
/// enche a lista; a original leria lixo da pilha sem clubes bastantes).
unsafe fn cqn4_construir(this: usize, ano: u32, p: &Cqn4Fundo) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5ba0a0usize);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(p.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let soltar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x687970usize);
    base(this);
    cqn4_pd(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    cqn4_pd(this + 0x2c, 0);
    *((this + 0x50) as *mut u8) = 0xff;
    cqn4_pd(this + 4, cqn4_registro(p.competicao));
    *((this + 0x44) as *mut u8) = 0xff;
    cqn4_pd(this + 0x30, 0xffff_ffff);
    if regras(this) == 0 {
        aviso_em(p.arquivo, p.linhas[0]);
        return this;
    }
    crate::comparacao::ramo("cqn_4 liga de fundo francesa");
    let mut lista = vec![0usize; p.maximo as usize];
    let mut k: i16 = 0;
    let mut genericos: i16 = 0;
    let n = (cqn4_d(0xae235c) as i32).wrapping_sub((cqn4_d(0xae2350) as i32).wrapping_mul(2));
    let base_clubes = cqn4_d(0xae23b4);
    let mut i: i32 = 0;
    while i < n {
        let c = base_clubes + (i as usize) * 0x245;
        let comp = cqn4_d(c + 0x57);
        let id = if comp != 0 { cqn4_d(comp) } else { 0 };
        if comp != 0 && p.todas.iter().any(|&g| id == cqn4_d(g)) {
            lista[k as usize] = c;
            k += 1;
            if k == p.maximo {
                break;
            }
        } else {
            let entra = match p.nacao {
                Some(g) => {
                    let nacao = cqn4_d(c + 0x53);
                    nacao != 0 && cqn4_d(nacao) == cqn4_d(g) && cqn4_d(c + 0x60) == 0 && comp != 0 && id == cqn4_d(0x9cf764)
                }
                None => {
                    let reserva = cqn4_d(c + 0x60);
                    comp != 0 && id == cqn4_d(0x9cf764) && reserva != 0 && cqn4_d(reserva) == cqn4_d(p.competicao)
                }
            };
            if entra && genericos < p.genericos { // cqn4: o máximo dos clubes da divisão genérica
                lista[k as usize] = c;
                k += 1;
                genericos += 1;
                if k == p.maximo {
                    break;
                }
            }
        }
        i += 1;
    }
    embaralhar(lista.as_mut_ptr() as usize, k as u16 as u32); // cqn4: o sorteio da lista
    for g in 0..p.grupos {
        if tabela(this, lista.as_ptr() as usize + g * p.por_grupo * 4) == 0 {
            aviso_em(p.arquivo, p.linhas[1]);
            return this;
        }
        if jogos(this) == 0 {
            aviso_em(p.arquivo, p.linhas[2]);
            return this;
        }
        soltar(this, 0);
    }
    this
}

const CQN4_CFA: Cqn4Fundo = Cqn4Fundo {
    vtabela: 0x96a7b0,
    competicao: 0x9cf5d8,
    todas: &[0x9cf5cc, 0x9cf5d0], // cqn4: os clubes da primeira e da segunda divisões na CFA
    nacao: None,
    maximo: 0x48,
    genericos: 0x22,
    grupos: 4,
    por_grupo: 0x12,
    tabela: 0x5b9fa0,
    arquivo: 0x9ccaf0,
    linhas: [0x40, 0x64, 0x6b],
};
const CQN4_BAIXO: Cqn4Fundo = Cqn4Fundo {
    vtabela: 0x96ab78,
    competicao: 0x9cf548,
    todas: &[0x9cf5d4],
    nacao: Some(0x9cf300),
    maximo: 0x28,
    genericos: 0x14,
    grupos: 2,
    por_grupo: 0x14,
    tabela: 0x5be230,
    arquivo: 0x9ccbd0,
    linhas: [0x40, 0x62, 0x69],
};

/// { 0x5b91c0 } (thiscall, ret 4, fra_cfa.cpp, 596 bytes): `FraCfa::FraCfa(ano)`, a CFA (a quarta divisão francesa, de
/// fundo; competição de id [0x9cf5d8]): até 72 clubes, os da primeira e da segunda divisões ([0x9cf5cc], [0x9cf5d0];
/// [inferência: os times reserva]) e até 34 da divisão genérica cuja competição de reserva é a CFA, embaralhados em 4
/// grupos de 18. Fonte 0x9ccaf0, linhas 0x40, 0x64, 0x6b.
unsafe extern "thiscall" fn construir_cfa(this: usize, ano: u32) -> usize {
    cqn4_construir(this, ano, &CQN4_CFA)
}

/// { 0x5bd600 } (thiscall, ret 4, fra_lower.cpp, 578 bytes): `FraLower::FraLower(ano)`, a divisão de baixo francesa
/// (competição de id [0x9cf548]): até 40 clubes, os do National ([0x9cf5d4]) e até 20 franceses ([0x9cf300]) da divisão
/// genérica sem competição de reserva, embaralhados em 2 grupos de 20. Fonte 0x9ccbd0, linhas 0x40, 0x62, 0x69.
unsafe extern "thiscall" fn construir_divisao_de_baixo_francesa(this: usize, ano: u32) -> usize {
    cqn4_construir(this, ano, &CQN4_BAIXO)
}

/// Sorteiam e alocam: só a original, conferindo a devolução, a vtable, o ano, a disciplina, o registro e a liga sem
/// tabela no fim (+0x3e = 0, +0xb1 = 0: a última tabela foi solta).
unsafe fn cqn4_comparar(nome: &str, this: usize, ano: u32, p: &Cqn4Fundo, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let ok = r == this && cqn4_d(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && *((this + 0x50) as *const u8) == 0xff && cqn4_d(this + 4) == cqn4_registro(p.competicao)
        && cqn4_d(this + 0xb1) == 0;
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +4 = {:#x}, +0xb1 = {:#x}", nome, this, r, cqn4_d(this), cqn4_d(this + 4), cqn4_d(this + 0xb1)));
    r
}

unsafe extern "thiscall" fn comparar_construir_cfa(this: usize, ano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(1877);
    cqn4_comparar("construir_cfa", this, ano, &CQN4_CFA, || o(this, ano))
}

unsafe extern "thiscall" fn comparar_construir_divisao_de_baixo_francesa(this: usize, ano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(1878);
    cqn4_comparar("construir_divisao_de_baixo_francesa", this, ano, &CQN4_BAIXO, || o(this, ano))
}

/// O corpo dos dois destrutores (thiscall, ret 0, quadro SEH só no caminho de exceção; iguais fora a vtable): a vtable
/// da classe, a tabela solta (League::free_table 0x687970: push 0 -> thiscall, ret 4), o calendário [+0xba] solto
/// (0x9452ca: push; add esp, 4 -> cdecl) e zerado, cada fase de 0 a +0x30 (com +0x30 ≥ 0) destruída pela sua virtual
/// 0 com 1 (o destrutor que solta: push 1; mov ecx, fase -> thiscall, ret 4), o vetor [+0xc] solto e zerado e, por fim,
/// League::~League (0x682300: mov ecx -> thiscall, ret 0).
unsafe fn cqn4_destruir(this: usize, vtabela: usize) {
    let soltar_tabela: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x687970usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x682300usize);
    cqn4_pd(this, vtabela);
    soltar_tabela(this, 0);
    let g = cqn4_d(this + 0xba);
    if g != 0 {
        soltar(g);
        cqn4_pd(this + 0xba, 0);
    }
    if (cqn4_d(this + 0x30) as i32) >= 0 {
        let mut i: i32 = 0;
        loop {
            let f = cqn4_d(cqn4_d(this + 0xc) + i as usize * 4);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(cqn4_d(cqn4_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqn4_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    let v = cqn4_d(this + 0xc);
    if v != 0 {
        soltar(v);
        cqn4_pd(this + 0xc, 0);
    }
    base(this);
}

/// { 0x5b9460 } (thiscall, ret 0, fra_cfa.cpp): `FraCfa::~FraCfa()`.
unsafe extern "thiscall" fn destruir_cfa(this: usize) {
    crate::comparacao::ramo("cqn_4 destrutor da cfa");
    cqn4_destruir(this, 0x96a7b0)
}

/// { 0x5bd870 } (thiscall, ret 0, fra_lower.cpp): `FraLower::~FraLower()`.
unsafe extern "thiscall" fn destruir_divisao_de_baixo_francesa(this: usize) {
    crate::comparacao::ramo("cqn_4 destrutor da divisao de baixo");
    cqn4_destruir(this, 0x96ab78)
}

/// Soltam memória: só a original.
unsafe extern "thiscall" fn comparar_destruir_cfa(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1879)(this)
}

unsafe extern "thiscall" fn comparar_destruir_divisao_de_baixo_francesa(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1880)(this)
}

// ---------- 278a leva: o calendário real de 2001/02 da Série A e da Série B italianas (os jogos fixos pelo nome dos clubes) (ita_ser_a.cpp, ita_ser_b.cpp) ----------

unsafe fn cqn5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqn5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Quantos jogos a liga tem: (clubes +0x3e / 2, com sinal, para zero) × rodadas +0xa9.
unsafe fn cqn5_jogos(liga: usize) -> i32 {
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32;
    let rodadas = core::ptr::read_unaligned((liga + 0xa9) as *const i16) as i32;
    (clubes / 2).wrapping_mul(rodadas)
}

/// O corpo das duas (set_up_fixed_fixtures; thiscall, ret 0, quadro SEH só para o construtor de vetor): o vetor dos
/// jogos fixos (operator new 0x944cf1 de n × 16 + 4: push; add esp, 4 -> cdecl; a conta n no primeiro dword e cada jogo
/// pelo construtor 0x645800, com o destrutor 0x5a53c0 para a exceção: `eh vector constructor iterator` 0x9463af, push
/// 0x5a53c0; push 0x645800; push n; push 0x10; push vetor -> stdcall, ret 0x14) em +0xea; sem memória, a caixa da
/// linha `linha` do fonte `arquivo` e fim. Depois cada jogo da `tabela`, na ordem, no seu lugar do vetor
/// (fixed_fixture_set_by_club_names 0x68aff0: push o 6º (fora, pelo mapa); push o 5º (casa); push ano; push mês (0 =
/// janeiro); push dia; push liga; mov ecx, [+0xea] + 16 × k -> thiscall, ret 0x18: a data e os clubes achados pelo
/// nome, −1 em +8/+0xc se não acha). Por fim, se algum dos n jogos ficou com um clube sem achar (+8 ou +0xc = −1), o
/// vetor é destruído (`eh vector destructor iterator` 0x9462bb: push 0x5a53c0; push conta; push 0x10; push vetor ->
/// stdcall, ret 0x10; operator delete 0x944c94 do bloco: push; add esp, 4 -> cdecl) e +0xea = 0: a liga volta ao
/// calendário sorteado.
unsafe fn cqn5_fixos(this: usize, tabela: &[(u8, u8, u16, u32, u32)], arquivo: usize, linha: u32) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let destruir: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let jogo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) = core::mem::transmute(0x68aff0usize);
    let n = cqn5_jogos(this);
    let bloco = novo((n as u32).wrapping_mul(16).wrapping_add(4));
    let vetor = if bloco != 0 {
        cqn5_pd(bloco, n as usize);
        construir(bloco + 4, 0x10, n as u32, 0x645800, 0x5a53c0);
        bloco + 4
    } else {
        0
    };
    cqn5_pd(this + 0xea, vetor);
    if vetor == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, &(dia, mes, ano, casa, fora)) in tabela.iter().enumerate() {
        jogo(cqn5_d(this + 0xea) + k * 0x10, this, dia as u32, mes as u32, ano as u32, casa, fora); // cqn5: cada jogo fixo
    }
    crate::comparacao::ramo("cqn_5 calendario fixo");
    let n = cqn5_jogos(this);
    if n <= 0 {
        return;
    }
    let v = cqn5_d(this + 0xea);
    for i in 0..n as usize {
        let f = v + i * 0x10;
        if cqn5_d(f + 8) == 0xffff_ffff || cqn5_d(f + 0xc) == 0xffff_ffff { // cqn5: um clube sem achar derruba a lista
            if v != 0 {
                destruir(v, 0x10, cqn5_d(v - 4) as u32, 0x5a53c0);
                apagar(v - 4);
            }
            cqn5_pd(this + 0xea, 0);
            return;
        }
    }
}

/// { 0x642a70 } (thiscall, ret 0, ita_ser_a.cpp, 11.650 bytes): `ItaSerA::create_fixed_fixtures()`: os 306 jogos da
/// Série A 2001/02 (34 rodadas de 9, com o dia, o mês (0 = janeiro) e o ano que a original empurra). Fonte 0x9ea2a8, linha 0x60f.
unsafe extern "thiscall" fn calendario_fixo_da_serie_a(this: usize) {
    cqn5_fixos(this, &CQN5_SERIE_A, 0x9ea2a8, 0x60f)
}

/// { 0x647230 } (thiscall, ret 0, ita_ser_b.cpp, 14.388 bytes): `ItaSerB::set_up_fixed_fixtures()`: os 380 jogos da
/// Série B 2001/02 (38 rodadas de 10). Fonte 0x9ea384, linha 0x39a.
unsafe extern "thiscall" fn calendario_fixo_da_serie_b(this: usize) {
    cqn5_fixos(this, &CQN5_SERIE_B, 0x9ea384, 0x39a)
}

/// O comparador das duas: alocam: só a original, conferindo o que ela garante: ou a lista ficou (+0xea com a conta n e
/// cada jogo com os dois clubes achados), ou foi derrubada (+0xea = 0).
unsafe fn cqn5_comparar(nome: &str, this: usize, original: impl FnOnce()) {
    original();
    let v = cqn5_d(this + 0xea);
    let n = cqn5_jogos(this);
    let ok = v == 0 || (cqn5_d(v - 4) as i32 == n && (0..n.max(0) as usize).all(|i| cqn5_d(v + i * 0x10 + 8) != 0xffff_ffff && cqn5_d(v + i * 0x10 + 0xc) != 0xffff_ffff));
    PLACAR.registrar(ok, || format!("{}({:#x}): a lista dos jogos fixos (+0xea = {:#x}) fora do esperado", nome, this, v));
}

unsafe extern "thiscall" fn comparar_calendario_fixo_da_serie_a(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1881);
    cqn5_comparar("calendario_fixo_da_serie_a", this, || o(this))
}

unsafe extern "thiscall" fn comparar_calendario_fixo_da_serie_b(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1882);
    cqn5_comparar("calendario_fixo_da_serie_b", this, || o(this))
}

/// Os jogos fixos (dia, mês com 0 = janeiro, ano, o 5º argumento (casa, pelo mapa), o 6º (fora)), os nomes no .rdata do
/// exe; na ordem da original (extraídos da desmontagem e conferidos: os lugares do vetor vão de 16 em 16).
const CQN5_SERIE_A: [(u8, u8, u16, u32, u32); 306] = [
    (28, 7, 2001, 0x9ea370, 0x9ea378), // Bologna × Atalanta
    (28, 7, 2001, 0x9ea360, 0x9ea368), // Brescia × Milan
    (28, 7, 2001, 0x9ea34c, 0x9ea358), // Fiorentina × Chievo
    (28, 7, 2001, 0x9ea33c, 0x9ea344), // Inter × Perugia
    (28, 7, 2001, 0x9ea328, 0x9ea334), // Juventus × Venezia
    (28, 7, 2001, 0x9ea314, 0x9ea31c), // Lazio × Piacenza
    (28, 7, 2001, 0x9ea304, 0x9ea30c), // Lecce × Parma
    (28, 7, 2001, 0x9ea2f4, 0x9ea2fc), // Udinese × Torino
    (28, 7, 2001, 0x9ea2e4, 0x9ea2ec), // Verona × Roma
    (9, 8, 2001, 0x9ea378, 0x9ea328), // Atalanta × Juventus
    (9, 8, 2001, 0x9ea358, 0x9ea370), // Chievo × Bologna
    (9, 8, 2001, 0x9ea368, 0x9ea34c), // Milan × Fiorentina
    (9, 8, 2001, 0x9ea30c, 0x9ea33c), // Parma × Inter
    (9, 8, 2001, 0x9ea344, 0x9ea314), // Perugia × Lazio
    (9, 8, 2001, 0x9ea31c, 0x9ea304), // Piacenza × Lecce
    (9, 8, 2001, 0x9ea2ec, 0x9ea2f4), // Roma × Udinese
    (9, 8, 2001, 0x9ea2fc, 0x9ea360), // Torino × Brescia
    (9, 8, 2001, 0x9ea334, 0x9ea2e4), // Venezia × Verona
    (16, 8, 2001, 0x9ea370, 0x9ea30c), // Bologna × Parma
    (16, 8, 2001, 0x9ea360, 0x9ea304), // Brescia × Lecce
    (16, 8, 2001, 0x9ea34c, 0x9ea378), // Fiorentina × Atalanta
    (16, 8, 2001, 0x9ea33c, 0x9ea334), // Inter × Venezia
    (16, 8, 2001, 0x9ea328, 0x9ea358), // Juventus × Chievo
    (16, 8, 2001, 0x9ea314, 0x9ea2fc), // Lazio × Torino
    (16, 8, 2001, 0x9ea31c, 0x9ea2ec), // Piacenza × Roma
    (16, 8, 2001, 0x9ea2f4, 0x9ea368), // Udinese × Milan
    (16, 8, 2001, 0x9ea2e4, 0x9ea344), // Verona × Perugia
    (23, 8, 2001, 0x9ea378, 0x9ea2e4), // Atalanta × Verona
    (23, 8, 2001, 0x9ea358, 0x9ea31c), // Chievo × Piacenza
    (23, 8, 2001, 0x9ea304, 0x9ea328), // Lecce × Juventus
    (23, 8, 2001, 0x9ea368, 0x9ea314), // Milan × Lazio
    (23, 8, 2001, 0x9ea30c, 0x9ea360), // Parma × Brescia
    (23, 8, 2001, 0x9ea344, 0x9ea2f4), // Perugia × Udinese
    (23, 8, 2001, 0x9ea2ec, 0x9ea34c), // Roma × Fiorentina
    (23, 8, 2001, 0x9ea2fc, 0x9ea33c), // Torino × Inter
    (23, 8, 2001, 0x9ea334, 0x9ea370), // Venezia × Bologna
    (30, 8, 2001, 0x9ea360, 0x9ea378), // Brescia × Atalanta
    (30, 8, 2001, 0x9ea34c, 0x9ea334), // Fiorentina × Venezia
    (30, 8, 2001, 0x9ea33c, 0x9ea370), // Inter × Bologna
    (30, 8, 2001, 0x9ea328, 0x9ea2ec), // Juventus × Roma
    (30, 8, 2001, 0x9ea314, 0x9ea30c), // Lazio × Parma
    (30, 8, 2001, 0x9ea344, 0x9ea368), // Perugia × Milan
    (30, 8, 2001, 0x9ea31c, 0x9ea2fc), // Piacenza × Torino
    (30, 8, 2001, 0x9ea2f4, 0x9ea358), // Udinese × Chievo
    (30, 8, 2001, 0x9ea2e4, 0x9ea304), // Verona × Lecce
    (10, 9, 2001, 0x9ea378, 0x9ea368), // Atalanta × Milan
    (10, 9, 2001, 0x9ea370, 0x9ea2f4), // Bologna × Udinese
    (10, 9, 2001, 0x9ea358, 0x9ea314), // Chievo × Lazio
    (10, 9, 2001, 0x9ea34c, 0x9ea328), // Fiorentina × Juventus
    (10, 9, 2001, 0x9ea33c, 0x9ea2e4), // Inter × Verona
    (10, 9, 2001, 0x9ea304, 0x9ea344), // Lecce × Perugia
    (10, 9, 2001, 0x9ea2ec, 0x9ea360), // Roma × Brescia
    (10, 9, 2001, 0x9ea2fc, 0x9ea30c), // Torino × Parma
    (10, 9, 2001, 0x9ea334, 0x9ea31c), // Venezia × Piacenza
    (14, 9, 2001, 0x9ea360, 0x9ea358), // Brescia × Chievo
    (14, 9, 2001, 0x9ea328, 0x9ea2fc), // Juventus × Torino
    (14, 9, 2001, 0x9ea314, 0x9ea378), // Lazio × Atalanta
    (14, 9, 2001, 0x9ea304, 0x9ea34c), // Lecce × Fiorentina
    (14, 9, 2001, 0x9ea368, 0x9ea334), // Milan × Venezia
    (14, 9, 2001, 0x9ea30c, 0x9ea31c), // Parma × Piacenza
    (14, 9, 2001, 0x9ea344, 0x9ea2ec), // Perugia × Roma
    (14, 9, 2001, 0x9ea2f4, 0x9ea33c), // Udinese × Inter
    (14, 9, 2001, 0x9ea2e4, 0x9ea370), // Verona × Bologna
    (21, 9, 2001, 0x9ea378, 0x9ea2f4), // Atalanta × Udinese
    (21, 9, 2001, 0x9ea370, 0x9ea328), // Bologna × Juventus
    (21, 9, 2001, 0x9ea358, 0x9ea30c), // Chievo × Parma
    (21, 9, 2001, 0x9ea34c, 0x9ea2e4), // Fiorentina × Verona
    (21, 9, 2001, 0x9ea33c, 0x9ea368), // Inter × Milan
    (21, 9, 2001, 0x9ea31c, 0x9ea360), // Piacenza × Brescia
    (21, 9, 2001, 0x9ea2ec, 0x9ea304), // Roma × Lecce
    (21, 9, 2001, 0x9ea2fc, 0x9ea344), // Torino × Perugia
    (21, 9, 2001, 0x9ea334, 0x9ea314), // Venezia × Lazio
    (28, 9, 2001, 0x9ea360, 0x9ea334), // Brescia × Venezia
    (28, 9, 2001, 0x9ea358, 0x9ea2fc), // Chievo × Torino
    (28, 9, 2001, 0x9ea328, 0x9ea33c), // Juventus × Inter
    (28, 9, 2001, 0x9ea304, 0x9ea378), // Lecce × Atalanta
    (28, 9, 2001, 0x9ea368, 0x9ea370), // Milan × Bologna
    (28, 9, 2001, 0x9ea30c, 0x9ea2e4), // Parma × Verona
    (28, 9, 2001, 0x9ea344, 0x9ea31c), // Perugia × Piacenza
    (28, 9, 2001, 0x9ea2ec, 0x9ea314), // Roma × Lazio
    (28, 9, 2001, 0x9ea2f4, 0x9ea34c), // Udinese × Fiorentina
    (4, 10, 2001, 0x9ea378, 0x9ea2ec), // Atalanta × Roma
    (4, 10, 2001, 0x9ea370, 0x9ea34c), // Bologna × Fiorentina
    (4, 10, 2001, 0x9ea33c, 0x9ea304), // Inter × Lecce
    (4, 10, 2001, 0x9ea314, 0x9ea360), // Lazio × Brescia
    (4, 10, 2001, 0x9ea30c, 0x9ea344), // Parma × Perugia
    (4, 10, 2001, 0x9ea31c, 0x9ea2f4), // Piacenza × Udinese
    (4, 10, 2001, 0x9ea2fc, 0x9ea368), // Torino × Milan
    (4, 10, 2001, 0x9ea334, 0x9ea358), // Venezia × Chievo
    (4, 10, 2001, 0x9ea2e4, 0x9ea328), // Verona × Juventus
    (18, 10, 2001, 0x9ea378, 0x9ea334), // Atalanta × Venezia
    (18, 10, 2001, 0x9ea34c, 0x9ea2fc), // Fiorentina × Torino
    (18, 10, 2001, 0x9ea328, 0x9ea30c), // Juventus × Parma
    (18, 10, 2001, 0x9ea304, 0x9ea370), // Lecce × Bologna
    (18, 10, 2001, 0x9ea368, 0x9ea31c), // Milan × Piacenza
    (18, 10, 2001, 0x9ea344, 0x9ea360), // Perugia × Brescia
    (18, 10, 2001, 0x9ea2ec, 0x9ea33c), // Roma × Inter
    (18, 10, 2001, 0x9ea2f4, 0x9ea314), // Udinese × Lazio
    (18, 10, 2001, 0x9ea2e4, 0x9ea358), // Verona × Chievo
    (25, 10, 2001, 0x9ea370, 0x9ea2ec), // Bologna × Roma
    (25, 10, 2001, 0x9ea360, 0x9ea2f4), // Brescia × Udinese
    (25, 10, 2001, 0x9ea358, 0x9ea344), // Chievo × Perugia
    (25, 10, 2001, 0x9ea33c, 0x9ea34c), // Inter × Fiorentina
    (25, 10, 2001, 0x9ea314, 0x9ea328), // Lazio × Juventus
    (25, 10, 2001, 0x9ea30c, 0x9ea368), // Parma × Milan
    (25, 10, 2001, 0x9ea31c, 0x9ea378), // Piacenza × Atalanta
    (25, 10, 2001, 0x9ea2fc, 0x9ea2e4), // Torino × Verona
    (25, 10, 2001, 0x9ea334, 0x9ea304), // Venezia × Lecce
    (2, 11, 2001, 0x9ea378, 0x9ea33c), // Atalanta × Inter
    (2, 11, 2001, 0x9ea370, 0x9ea2fc), // Bologna × Torino
    (2, 11, 2001, 0x9ea34c, 0x9ea31c), // Fiorentina × Piacenza
    (2, 11, 2001, 0x9ea328, 0x9ea344), // Juventus × Perugia
    (2, 11, 2001, 0x9ea304, 0x9ea314), // Lecce × Lazio
    (2, 11, 2001, 0x9ea368, 0x9ea358), // Milan × Chievo
    (2, 11, 2001, 0x9ea2ec, 0x9ea334), // Roma × Venezia
    (2, 11, 2001, 0x9ea2f4, 0x9ea30c), // Udinese × Parma
    (2, 11, 2001, 0x9ea2e4, 0x9ea360), // Verona × Brescia
    (9, 11, 2001, 0x9ea360, 0x9ea33c), // Brescia × Inter
    (9, 11, 2001, 0x9ea358, 0x9ea304), // Chievo × Lecce
    (9, 11, 2001, 0x9ea314, 0x9ea34c), // Lazio × Fiorentina
    (9, 11, 2001, 0x9ea368, 0x9ea328), // Milan × Juventus
    (9, 11, 2001, 0x9ea30c, 0x9ea2ec), // Parma × Roma
    (9, 11, 2001, 0x9ea344, 0x9ea334), // Perugia × Venezia
    (9, 11, 2001, 0x9ea31c, 0x9ea370), // Piacenza × Bologna
    (9, 11, 2001, 0x9ea2fc, 0x9ea378), // Torino × Atalanta
    (9, 11, 2001, 0x9ea2f4, 0x9ea2e4), // Udinese × Verona
    (16, 11, 2001, 0x9ea378, 0x9ea30c), // Atalanta × Parma
    (16, 11, 2001, 0x9ea370, 0x9ea344), // Bologna × Perugia
    (16, 11, 2001, 0x9ea34c, 0x9ea360), // Fiorentina × Brescia
    (16, 11, 2001, 0x9ea33c, 0x9ea358), // Inter × Chievo
    (16, 11, 2001, 0x9ea328, 0x9ea31c), // Juventus × Piacenza
    (16, 11, 2001, 0x9ea304, 0x9ea2fc), // Lecce × Torino
    (16, 11, 2001, 0x9ea2ec, 0x9ea368), // Roma × Milan
    (16, 11, 2001, 0x9ea334, 0x9ea2f4), // Venezia × Udinese
    (16, 11, 2001, 0x9ea2e4, 0x9ea314), // Verona × Lazio
    (6, 0, 2002, 0x9ea360, 0x9ea328), // Brescia × Juventus
    (6, 0, 2002, 0x9ea358, 0x9ea2ec), // Chievo × Roma
    (6, 0, 2002, 0x9ea314, 0x9ea370), // Lazio × Bologna
    (6, 0, 2002, 0x9ea368, 0x9ea2e4), // Milan × Verona
    (6, 0, 2002, 0x9ea30c, 0x9ea34c), // Parma × Fiorentina
    (6, 0, 2002, 0x9ea344, 0x9ea378), // Perugia × Atalanta
    (6, 0, 2002, 0x9ea31c, 0x9ea33c), // Piacenza × Inter
    (6, 0, 2002, 0x9ea2fc, 0x9ea334), // Torino × Venezia
    (6, 0, 2002, 0x9ea2f4, 0x9ea304), // Udinese × Lecce
    (13, 0, 2002, 0x9ea378, 0x9ea358), // Atalanta × Chievo
    (13, 0, 2002, 0x9ea370, 0x9ea360), // Bologna × Brescia
    (13, 0, 2002, 0x9ea34c, 0x9ea344), // Fiorentina × Perugia
    (13, 0, 2002, 0x9ea33c, 0x9ea314), // Inter × Lazio
    (13, 0, 2002, 0x9ea328, 0x9ea2f4), // Juventus × Udinese
    (13, 0, 2002, 0x9ea304, 0x9ea368), // Lecce × Milan
    (13, 0, 2002, 0x9ea2ec, 0x9ea2fc), // Roma × Torino
    (13, 0, 2002, 0x9ea334, 0x9ea30c), // Venezia × Parma
    (13, 0, 2002, 0x9ea2e4, 0x9ea31c), // Verona × Piacenza
    (20, 0, 2002, 0x9ea378, 0x9ea370), // Atalanta × Bologna
    (20, 0, 2002, 0x9ea368, 0x9ea360), // Milan × Brescia
    (20, 0, 2002, 0x9ea358, 0x9ea34c), // Chievo × Fiorentina
    (20, 0, 2002, 0x9ea344, 0x9ea33c), // Perugia × Inter
    (20, 0, 2002, 0x9ea334, 0x9ea328), // Venezia × Juventus
    (20, 0, 2002, 0x9ea31c, 0x9ea314), // Piacenza × Lazio
    (20, 0, 2002, 0x9ea30c, 0x9ea304), // Parma × Lecce
    (20, 0, 2002, 0x9ea2fc, 0x9ea2f4), // Torino × Udinese
    (20, 0, 2002, 0x9ea2ec, 0x9ea2e4), // Roma × Verona
    (27, 0, 2002, 0x9ea328, 0x9ea378), // Juventus × Atalanta
    (27, 0, 2002, 0x9ea370, 0x9ea358), // Bologna × Chievo
    (27, 0, 2002, 0x9ea34c, 0x9ea368), // Fiorentina × Milan
    (27, 0, 2002, 0x9ea33c, 0x9ea30c), // Inter × Parma
    (27, 0, 2002, 0x9ea314, 0x9ea344), // Lazio × Perugia
    (27, 0, 2002, 0x9ea304, 0x9ea31c), // Lecce × Piacenza
    (27, 0, 2002, 0x9ea2f4, 0x9ea2ec), // Udinese × Roma
    (27, 0, 2002, 0x9ea360, 0x9ea2fc), // Brescia × Torino
    (27, 0, 2002, 0x9ea2e4, 0x9ea334), // Verona × Venezia
    (3, 1, 2002, 0x9ea30c, 0x9ea370), // Parma × Bologna
    (3, 1, 2002, 0x9ea304, 0x9ea360), // Lecce × Brescia
    (3, 1, 2002, 0x9ea378, 0x9ea34c), // Atalanta × Fiorentina
    (3, 1, 2002, 0x9ea334, 0x9ea33c), // Venezia × Inter
    (3, 1, 2002, 0x9ea358, 0x9ea328), // Chievo × Juventus
    (3, 1, 2002, 0x9ea2fc, 0x9ea314), // Torino × Lazio
    (3, 1, 2002, 0x9ea2ec, 0x9ea31c), // Roma × Piacenza
    (3, 1, 2002, 0x9ea368, 0x9ea2f4), // Milan × Udinese
    (3, 1, 2002, 0x9ea344, 0x9ea2e4), // Perugia × Verona
    (10, 1, 2002, 0x9ea2e4, 0x9ea378), // Verona × Atalanta
    (10, 1, 2002, 0x9ea31c, 0x9ea358), // Piacenza × Chievo
    (10, 1, 2002, 0x9ea328, 0x9ea304), // Juventus × Lecce
    (10, 1, 2002, 0x9ea314, 0x9ea368), // Lazio × Milan
    (10, 1, 2002, 0x9ea360, 0x9ea30c), // Brescia × Parma
    (10, 1, 2002, 0x9ea2f4, 0x9ea344), // Udinese × Perugia
    (10, 1, 2002, 0x9ea34c, 0x9ea2ec), // Fiorentina × Roma
    (10, 1, 2002, 0x9ea33c, 0x9ea2fc), // Inter × Torino
    (10, 1, 2002, 0x9ea370, 0x9ea334), // Bologna × Venezia
    (17, 1, 2002, 0x9ea378, 0x9ea360), // Atalanta × Brescia
    (17, 1, 2002, 0x9ea334, 0x9ea34c), // Venezia × Fiorentina
    (17, 1, 2002, 0x9ea370, 0x9ea33c), // Bologna × Inter
    (17, 1, 2002, 0x9ea2ec, 0x9ea328), // Roma × Juventus
    (17, 1, 2002, 0x9ea30c, 0x9ea314), // Parma × Lazio
    (17, 1, 2002, 0x9ea368, 0x9ea344), // Milan × Perugia
    (17, 1, 2002, 0x9ea2fc, 0x9ea31c), // Torino × Piacenza
    (17, 1, 2002, 0x9ea358, 0x9ea2f4), // Chievo × Udinese
    (17, 1, 2002, 0x9ea304, 0x9ea2e4), // Lecce × Verona
    (24, 1, 2002, 0x9ea368, 0x9ea378), // Milan × Atalanta
    (24, 1, 2002, 0x9ea2f4, 0x9ea370), // Udinese × Bologna
    (24, 1, 2002, 0x9ea314, 0x9ea358), // Lazio × Chievo
    (24, 1, 2002, 0x9ea328, 0x9ea34c), // Juventus × Fiorentina
    (24, 1, 2002, 0x9ea2e4, 0x9ea33c), // Verona × Inter
    (24, 1, 2002, 0x9ea344, 0x9ea304), // Perugia × Lecce
    (24, 1, 2002, 0x9ea360, 0x9ea2ec), // Brescia × Roma
    (24, 1, 2002, 0x9ea30c, 0x9ea2fc), // Parma × Torino
    (24, 1, 2002, 0x9ea31c, 0x9ea334), // Piacenza × Venezia
    (3, 2, 2002, 0x9ea358, 0x9ea360), // Chievo × Brescia
    (3, 2, 2002, 0x9ea2fc, 0x9ea328), // Torino × Juventus
    (3, 2, 2002, 0x9ea378, 0x9ea314), // Atalanta × Lazio
    (3, 2, 2002, 0x9ea34c, 0x9ea304), // Fiorentina × Lecce
    (3, 2, 2002, 0x9ea334, 0x9ea368), // Venezia × Milan
    (3, 2, 2002, 0x9ea31c, 0x9ea30c), // Piacenza × Parma
    (3, 2, 2002, 0x9ea2ec, 0x9ea344), // Roma × Perugia
    (3, 2, 2002, 0x9ea33c, 0x9ea2f4), // Inter × Udinese
    (3, 2, 2002, 0x9ea370, 0x9ea2e4), // Bologna × Verona
    (10, 2, 2002, 0x9ea2f4, 0x9ea378), // Udinese × Atalanta
    (10, 2, 2002, 0x9ea328, 0x9ea370), // Juventus × Bologna
    (10, 2, 2002, 0x9ea30c, 0x9ea358), // Parma × Chievo
    (10, 2, 2002, 0x9ea2e4, 0x9ea34c), // Verona × Fiorentina
    (10, 2, 2002, 0x9ea368, 0x9ea33c), // Milan × Inter
    (10, 2, 2002, 0x9ea360, 0x9ea31c), // Brescia × Piacenza
    (10, 2, 2002, 0x9ea304, 0x9ea2ec), // Lecce × Roma
    (10, 2, 2002, 0x9ea344, 0x9ea2fc), // Perugia × Torino
    (10, 2, 2002, 0x9ea314, 0x9ea334), // Lazio × Venezia
    (17, 2, 2002, 0x9ea334, 0x9ea360), // Venezia × Brescia
    (17, 2, 2002, 0x9ea2fc, 0x9ea358), // Torino × Chievo
    (17, 2, 2002, 0x9ea33c, 0x9ea328), // Inter × Juventus
    (17, 2, 2002, 0x9ea378, 0x9ea304), // Atalanta × Lecce
    (17, 2, 2002, 0x9ea370, 0x9ea368), // Bologna × Milan
    (17, 2, 2002, 0x9ea2e4, 0x9ea30c), // Verona × Parma
    (17, 2, 2002, 0x9ea31c, 0x9ea344), // Piacenza × Perugia
    (17, 2, 2002, 0x9ea314, 0x9ea2ec), // Lazio × Roma
    (17, 2, 2002, 0x9ea34c, 0x9ea2f4), // Fiorentina × Udinese
    (24, 2, 2002, 0x9ea2ec, 0x9ea378), // Roma × Atalanta
    (24, 2, 2002, 0x9ea34c, 0x9ea370), // Fiorentina × Bologna
    (24, 2, 2002, 0x9ea304, 0x9ea33c), // Lecce × Inter
    (24, 2, 2002, 0x9ea360, 0x9ea314), // Brescia × Lazio
    (24, 2, 2002, 0x9ea344, 0x9ea30c), // Perugia × Parma
    (24, 2, 2002, 0x9ea2f4, 0x9ea31c), // Udinese × Piacenza
    (24, 2, 2002, 0x9ea368, 0x9ea2fc), // Milan × Torino
    (24, 2, 2002, 0x9ea358, 0x9ea334), // Chievo × Venezia
    (24, 2, 2002, 0x9ea328, 0x9ea2e4), // Juventus × Verona
    (30, 2, 2002, 0x9ea334, 0x9ea378), // Venezia × Atalanta
    (30, 2, 2002, 0x9ea2fc, 0x9ea34c), // Torino × Fiorentina
    (30, 2, 2002, 0x9ea30c, 0x9ea328), // Parma × Juventus
    (30, 2, 2002, 0x9ea370, 0x9ea304), // Bologna × Lecce
    (30, 2, 2002, 0x9ea31c, 0x9ea368), // Piacenza × Milan
    (30, 2, 2002, 0x9ea360, 0x9ea344), // Brescia × Perugia
    (30, 2, 2002, 0x9ea33c, 0x9ea2ec), // Inter × Roma
    (30, 2, 2002, 0x9ea314, 0x9ea2f4), // Lazio × Udinese
    (30, 2, 2002, 0x9ea358, 0x9ea2e4), // Chievo × Verona
    (7, 3, 2002, 0x9ea2ec, 0x9ea370), // Roma × Bologna
    (7, 3, 2002, 0x9ea2f4, 0x9ea360), // Udinese × Brescia
    (7, 3, 2002, 0x9ea344, 0x9ea358), // Perugia × Chievo
    (7, 3, 2002, 0x9ea34c, 0x9ea33c), // Fiorentina × Inter
    (7, 3, 2002, 0x9ea328, 0x9ea314), // Juventus × Lazio
    (7, 3, 2002, 0x9ea368, 0x9ea30c), // Milan × Parma
    (7, 3, 2002, 0x9ea378, 0x9ea31c), // Atalanta × Piacenza
    (7, 3, 2002, 0x9ea2e4, 0x9ea2fc), // Verona × Torino
    (7, 3, 2002, 0x9ea304, 0x9ea334), // Lecce × Venezia
    (14, 3, 2002, 0x9ea33c, 0x9ea378), // Inter × Atalanta
    (14, 3, 2002, 0x9ea2fc, 0x9ea370), // Torino × Bologna
    (14, 3, 2002, 0x9ea31c, 0x9ea34c), // Piacenza × Fiorentina
    (14, 3, 2002, 0x9ea344, 0x9ea328), // Perugia × Juventus
    (14, 3, 2002, 0x9ea314, 0x9ea304), // Lazio × Lecce
    (14, 3, 2002, 0x9ea358, 0x9ea368), // Chievo × Milan
    (14, 3, 2002, 0x9ea334, 0x9ea2ec), // Venezia × Roma
    (14, 3, 2002, 0x9ea30c, 0x9ea2f4), // Parma × Udinese
    (14, 3, 2002, 0x9ea360, 0x9ea2e4), // Brescia × Verona
    (21, 3, 2002, 0x9ea33c, 0x9ea360), // Inter × Brescia
    (21, 3, 2002, 0x9ea304, 0x9ea358), // Lecce × Chievo
    (21, 3, 2002, 0x9ea34c, 0x9ea314), // Fiorentina × Lazio
    (21, 3, 2002, 0x9ea328, 0x9ea368), // Juventus × Milan
    (21, 3, 2002, 0x9ea2ec, 0x9ea30c), // Roma × Parma
    (21, 3, 2002, 0x9ea334, 0x9ea344), // Venezia × Perugia
    (21, 3, 2002, 0x9ea370, 0x9ea31c), // Bologna × Piacenza
    (21, 3, 2002, 0x9ea378, 0x9ea2fc), // Atalanta × Torino
    (21, 3, 2002, 0x9ea2e4, 0x9ea2f4), // Verona × Udinese
    (28, 3, 2002, 0x9ea30c, 0x9ea378), // Parma × Atalanta
    (28, 3, 2002, 0x9ea344, 0x9ea370), // Perugia × Bologna
    (28, 3, 2002, 0x9ea360, 0x9ea34c), // Brescia × Fiorentina
    (28, 3, 2002, 0x9ea358, 0x9ea33c), // Chievo × Inter
    (28, 3, 2002, 0x9ea31c, 0x9ea328), // Piacenza × Juventus
    (28, 3, 2002, 0x9ea2fc, 0x9ea304), // Torino × Lecce
    (28, 3, 2002, 0x9ea368, 0x9ea2ec), // Milan × Roma
    (28, 3, 2002, 0x9ea2f4, 0x9ea334), // Udinese × Venezia
    (28, 3, 2002, 0x9ea314, 0x9ea2e4), // Lazio × Verona
    (5, 4, 2002, 0x9ea328, 0x9ea360), // Juventus × Brescia
    (5, 4, 2002, 0x9ea2ec, 0x9ea358), // Roma × Chievo
    (5, 4, 2002, 0x9ea370, 0x9ea314), // Bologna × Lazio
    (5, 4, 2002, 0x9ea2e4, 0x9ea368), // Verona × Milan
    (5, 4, 2002, 0x9ea34c, 0x9ea30c), // Fiorentina × Parma
    (5, 4, 2002, 0x9ea378, 0x9ea344), // Atalanta × Perugia
    (5, 4, 2002, 0x9ea33c, 0x9ea31c), // Inter × Piacenza
    (5, 4, 2002, 0x9ea334, 0x9ea2fc), // Venezia × Torino
    (5, 4, 2002, 0x9ea304, 0x9ea2f4), // Lecce × Udinese
    (12, 4, 2002, 0x9ea358, 0x9ea378), // Chievo × Atalanta
    (12, 4, 2002, 0x9ea360, 0x9ea370), // Brescia × Bologna
    (12, 4, 2002, 0x9ea344, 0x9ea34c), // Perugia × Fiorentina
    (12, 4, 2002, 0x9ea314, 0x9ea33c), // Lazio × Inter
    (12, 4, 2002, 0x9ea2f4, 0x9ea328), // Udinese × Juventus
    (12, 4, 2002, 0x9ea368, 0x9ea304), // Milan × Lecce
    (12, 4, 2002, 0x9ea2fc, 0x9ea2ec), // Torino × Roma
    (12, 4, 2002, 0x9ea30c, 0x9ea334), // Parma × Venezia
    (12, 4, 2002, 0x9ea31c, 0x9ea2e4), // Piacenza × Verona
];

const CQN5_SERIE_B: [(u8, u8, u16, u32, u32); 380] = [
    (26, 7, 2001, 0x9ea464, 0x9ea46c), // Ancona × Reggina
    (26, 7, 2001, 0x9ea450, 0x9ea45c), // Cagliari × Messina
    (26, 7, 2001, 0x9ea440, 0x9ea448), // Como × Crotone
    (26, 7, 2001, 0x9ea42c, 0x9ea434), // Cosenza × Cittadella
    (26, 7, 2001, 0x9ea41c, 0x9ea424), // Empoli × Palermo
    (26, 7, 2001, 0x9ea40c, 0x9ea414), // Genoa × Napoli
    (26, 7, 2001, 0x9ea3fc, 0x9ea404), // Modena × Bari
    (26, 7, 2001, 0x9ea3e4, 0x9ea3f0), // Salernitana × Sampdoria
    (26, 7, 2001, 0x9ea3d4, 0x9ea3dc), // Ternana × Siena
    (26, 7, 2001, 0x9ea3c0, 0x9ea3c8), // Vicenza × Pistoiese
    (2, 8, 2001, 0x9ea404, 0x9ea41c), // Bari × Empoli
    (2, 8, 2001, 0x9ea434, 0x9ea3e4), // Cittadella × Salernitana
    (2, 8, 2001, 0x9ea448, 0x9ea3fc), // Crotone × Modena
    (2, 8, 2001, 0x9ea45c, 0x9ea3d4), // Messina × Ternana
    (2, 8, 2001, 0x9ea414, 0x9ea464), // Napoli × Ancona
    (2, 8, 2001, 0x9ea424, 0x9ea450), // Palermo × Cagliari
    (2, 8, 2001, 0x9ea3c8, 0x9ea40c), // Pistoiese × Genoa
    (2, 8, 2001, 0x9ea46c, 0x9ea440), // Reggina × Como
    (2, 8, 2001, 0x9ea3f0, 0x9ea42c), // Sampdoria × Cosenza
    (2, 8, 2001, 0x9ea3dc, 0x9ea3c0), // Siena × Vicenza
    (9, 8, 2001, 0x9ea464, 0x9ea3dc), // Ancona × Siena
    (9, 8, 2001, 0x9ea450, 0x9ea404), // Cagliari × Bari
    (9, 8, 2001, 0x9ea42c, 0x9ea3d4), // Cosenza × Ternana
    (9, 8, 2001, 0x9ea41c, 0x9ea45c), // Empoli × Messina
    (9, 8, 2001, 0x9ea40c, 0x9ea440), // Genoa × Como
    (9, 8, 2001, 0x9ea3fc, 0x9ea414), // Modena × Napoli
    (9, 8, 2001, 0x9ea424, 0x9ea3c8), // Palermo × Pistoiese
    (9, 8, 2001, 0x9ea46c, 0x9ea3f0), // Reggina × Sampdoria
    (9, 8, 2001, 0x9ea3e4, 0x9ea448), // Salernitana × Crotone
    (9, 8, 2001, 0x9ea3c0, 0x9ea434), // Vicenza × Cittadella
    (16, 8, 2001, 0x9ea404, 0x9ea424), // Bari × Palermo
    (16, 8, 2001, 0x9ea434, 0x9ea46c), // Cittadella × Reggina
    (16, 8, 2001, 0x9ea440, 0x9ea3e4), // Como × Salernitana
    (16, 8, 2001, 0x9ea448, 0x9ea450), // Crotone × Cagliari
    (16, 8, 2001, 0x9ea45c, 0x9ea464), // Messina × Ancona
    (16, 8, 2001, 0x9ea414, 0x9ea3c0), // Napoli × Vicenza
    (16, 8, 2001, 0x9ea3c8, 0x9ea42c), // Pistoiese × Cosenza
    (16, 8, 2001, 0x9ea3f0, 0x9ea41c), // Sampdoria × Empoli
    (16, 8, 2001, 0x9ea3dc, 0x9ea40c), // Siena × Genoa
    (16, 8, 2001, 0x9ea3d4, 0x9ea3fc), // Ternana × Modena
    (23, 8, 2001, 0x9ea464, 0x9ea448), // Ancona × Crotone
    (23, 8, 2001, 0x9ea450, 0x9ea3f0), // Cagliari × Sampdoria
    (23, 8, 2001, 0x9ea42c, 0x9ea440), // Cosenza × Como
    (23, 8, 2001, 0x9ea41c, 0x9ea414), // Empoli × Napoli
    (23, 8, 2001, 0x9ea40c, 0x9ea45c), // Genoa × Messina
    (23, 8, 2001, 0x9ea3fc, 0x9ea3dc), // Modena × Siena
    (23, 8, 2001, 0x9ea424, 0x9ea434), // Palermo × Cittadella
    (23, 8, 2001, 0x9ea46c, 0x9ea404), // Reggina × Bari
    (23, 8, 2001, 0x9ea3e4, 0x9ea3c8), // Salernitana × Pistoiese
    (23, 8, 2001, 0x9ea3c0, 0x9ea3d4), // Vicenza × Ternana
    (30, 8, 2001, 0x9ea404, 0x9ea448), // Bari × Crotone
    (30, 8, 2001, 0x9ea434, 0x9ea41c), // Cittadella × Empoli
    (30, 8, 2001, 0x9ea440, 0x9ea464), // Como × Ancona
    (30, 8, 2001, 0x9ea42c, 0x9ea46c), // Cosenza × Reggina
    (30, 8, 2001, 0x9ea45c, 0x9ea3c0), // Messina × Vicenza
    (30, 8, 2001, 0x9ea414, 0x9ea450), // Napoli × Cagliari
    (30, 8, 2001, 0x9ea3c8, 0x9ea3fc), // Pistoiese × Modena
    (30, 8, 2001, 0x9ea3f0, 0x9ea424), // Sampdoria × Palermo
    (30, 8, 2001, 0x9ea3dc, 0x9ea3e4), // Siena × Salernitana
    (30, 8, 2001, 0x9ea3d4, 0x9ea40c), // Ternana × Genoa
    (7, 9, 2001, 0x9ea464, 0x9ea40c), // Ancona × Genoa
    (7, 9, 2001, 0x9ea450, 0x9ea3dc), // Cagliari × Siena
    (7, 9, 2001, 0x9ea448, 0x9ea434), // Crotone × Cittadella
    (7, 9, 2001, 0x9ea41c, 0x9ea3c8), // Empoli × Pistoiese
    (7, 9, 2001, 0x9ea3fc, 0x9ea440), // Modena × Como
    (7, 9, 2001, 0x9ea46c, 0x9ea424), // Reggina × Palermo
    (7, 9, 2001, 0x9ea3e4, 0x9ea45c), // Salernitana × Messina
    (7, 9, 2001, 0x9ea3f0, 0x9ea404), // Sampdoria × Bari
    (7, 9, 2001, 0x9ea3d4, 0x9ea414), // Ternana × Napoli
    (7, 9, 2001, 0x9ea3c0, 0x9ea42c), // Vicenza × Cosenza
    (10, 9, 2001, 0x9ea464, 0x9ea3f0), // Ancona × Sampdoria
    (10, 9, 2001, 0x9ea440, 0x9ea3d4), // Como × Ternana
    (10, 9, 2001, 0x9ea42c, 0x9ea3fc), // Cosenza × Modena
    (10, 9, 2001, 0x9ea41c, 0x9ea450), // Empoli × Cagliari
    (10, 9, 2001, 0x9ea40c, 0x9ea434), // Genoa × Cittadella
    (10, 9, 2001, 0x9ea45c, 0x9ea3dc), // Messina × Siena
    (10, 9, 2001, 0x9ea414, 0x9ea3e4), // Napoli × Salernitana
    (10, 9, 2001, 0x9ea424, 0x9ea448), // Palermo × Crotone
    (10, 9, 2001, 0x9ea3c8, 0x9ea46c), // Pistoiese × Reggina
    (10, 9, 2001, 0x9ea3c0, 0x9ea404), // Vicenza × Bari
    (14, 9, 2001, 0x9ea404, 0x9ea40c), // Bari × Genoa
    (14, 9, 2001, 0x9ea434, 0x9ea414), // Cittadella × Napoli
    (14, 9, 2001, 0x9ea448, 0x9ea3c0), // Crotone × Vicenza
    (14, 9, 2001, 0x9ea45c, 0x9ea424), // Messina × Palermo
    (14, 9, 2001, 0x9ea3fc, 0x9ea450), // Modena × Cagliari
    (14, 9, 2001, 0x9ea46c, 0x9ea41c), // Reggina × Empoli
    (14, 9, 2001, 0x9ea3e4, 0x9ea42c), // Salernitana × Cosenza
    (14, 9, 2001, 0x9ea3f0, 0x9ea3c8), // Sampdoria × Pistoiese
    (14, 9, 2001, 0x9ea3dc, 0x9ea440), // Siena × Como
    (14, 9, 2001, 0x9ea3d4, 0x9ea464), // Ternana × Ancona
    (28, 9, 2001, 0x9ea464, 0x9ea434), // Ancona × Cittadella
    (28, 9, 2001, 0x9ea450, 0x9ea46c), // Cagliari × Reggina
    (28, 9, 2001, 0x9ea440, 0x9ea45c), // Como × Messina
    (28, 9, 2001, 0x9ea42c, 0x9ea3dc), // Cosenza × Siena
    (28, 9, 2001, 0x9ea41c, 0x9ea448), // Empoli × Crotone
    (28, 9, 2001, 0x9ea40c, 0x9ea3e4), // Genoa × Salernitana
    (28, 9, 2001, 0x9ea414, 0x9ea3f0), // Napoli × Sampdoria
    (28, 9, 2001, 0x9ea424, 0x9ea3d4), // Palermo × Ternana
    (28, 9, 2001, 0x9ea3c8, 0x9ea404), // Pistoiese × Bari
    (28, 9, 2001, 0x9ea3c0, 0x9ea3fc), // Vicenza × Modena
    (4, 10, 2001, 0x9ea404, 0x9ea414), // Bari × Napoli
    (4, 10, 2001, 0x9ea434, 0x9ea450), // Cittadella × Cagliari
    (4, 10, 2001, 0x9ea440, 0x9ea3c0), // Como × Vicenza
    (4, 10, 2001, 0x9ea448, 0x9ea46c), // Crotone × Reggina
    (4, 10, 2001, 0x9ea40c, 0x9ea3f0), // Genoa × Sampdoria
    (4, 10, 2001, 0x9ea45c, 0x9ea42c), // Messina × Cosenza
    (4, 10, 2001, 0x9ea3fc, 0x9ea41c), // Modena × Empoli
    (4, 10, 2001, 0x9ea3e4, 0x9ea464), // Salernitana × Ancona
    (4, 10, 2001, 0x9ea3dc, 0x9ea424), // Siena × Palermo
    (4, 10, 2001, 0x9ea3d4, 0x9ea3c8), // Ternana × Pistoiese
    (11, 10, 2001, 0x9ea404, 0x9ea3dc), // Bari × Siena
    (11, 10, 2001, 0x9ea450, 0x9ea3d4), // Cagliari × Ternana
    (11, 10, 2001, 0x9ea42c, 0x9ea40c), // Cosenza × Genoa
    (11, 10, 2001, 0x9ea41c, 0x9ea3e4), // Empoli × Salernitana
    (11, 10, 2001, 0x9ea414, 0x9ea448), // Napoli × Crotone
    (11, 10, 2001, 0x9ea424, 0x9ea3fc), // Palermo × Modena
    (11, 10, 2001, 0x9ea3c8, 0x9ea440), // Pistoiese × Como
    (11, 10, 2001, 0x9ea46c, 0x9ea45c), // Reggina × Messina
    (11, 10, 2001, 0x9ea3f0, 0x9ea434), // Sampdoria × Cittadella
    (11, 10, 2001, 0x9ea3c0, 0x9ea464), // Vicenza × Ancona
    (18, 10, 2001, 0x9ea464, 0x9ea42c), // Ancona × Cosenza
    (18, 10, 2001, 0x9ea434, 0x9ea404), // Cittadella × Bari
    (18, 10, 2001, 0x9ea440, 0x9ea424), // Como × Palermo
    (18, 10, 2001, 0x9ea448, 0x9ea3f0), // Crotone × Sampdoria
    (18, 10, 2001, 0x9ea40c, 0x9ea3c0), // Genoa × Vicenza
    (18, 10, 2001, 0x9ea45c, 0x9ea414), // Messina × Napoli
    (18, 10, 2001, 0x9ea3fc, 0x9ea46c), // Modena × Reggina
    (18, 10, 2001, 0x9ea3e4, 0x9ea450), // Salernitana × Cagliari
    (18, 10, 2001, 0x9ea3dc, 0x9ea3c8), // Siena × Pistoiese
    (18, 10, 2001, 0x9ea3d4, 0x9ea41c), // Ternana × Empoli
    (25, 10, 2001, 0x9ea404, 0x9ea440), // Bari × Como
    (25, 10, 2001, 0x9ea450, 0x9ea464), // Cagliari × Ancona
    (25, 10, 2001, 0x9ea434, 0x9ea3d4), // Cittadella × Ternana
    (25, 10, 2001, 0x9ea448, 0x9ea42c), // Crotone × Cosenza
    (25, 10, 2001, 0x9ea41c, 0x9ea3dc), // Empoli × Siena
    (25, 10, 2001, 0x9ea424, 0x9ea40c), // Palermo × Genoa
    (25, 10, 2001, 0x9ea3c8, 0x9ea45c), // Pistoiese × Messina
    (25, 10, 2001, 0x9ea46c, 0x9ea414), // Reggina × Napoli
    (25, 10, 2001, 0x9ea3e4, 0x9ea3c0), // Salernitana × Vicenza
    (25, 10, 2001, 0x9ea3f0, 0x9ea3fc), // Sampdoria × Modena
    (2, 11, 2001, 0x9ea464, 0x9ea3c8), // Ancona × Pistoiese
    (2, 11, 2001, 0x9ea440, 0x9ea450), // Como × Cagliari
    (2, 11, 2001, 0x9ea42c, 0x9ea404), // Cosenza × Bari
    (2, 11, 2001, 0x9ea40c, 0x9ea448), // Genoa × Crotone
    (2, 11, 2001, 0x9ea45c, 0x9ea3f0), // Messina × Sampdoria
    (2, 11, 2001, 0x9ea3fc, 0x9ea434), // Modena × Cittadella
    (2, 11, 2001, 0x9ea414, 0x9ea424), // Napoli × Palermo
    (2, 11, 2001, 0x9ea3dc, 0x9ea46c), // Siena × Reggina
    (2, 11, 2001, 0x9ea3d4, 0x9ea3e4), // Ternana × Salernitana
    (2, 11, 2001, 0x9ea3c0, 0x9ea41c), // Vicenza × Empoli
    (9, 11, 2001, 0x9ea404, 0x9ea464), // Bari × Ancona
    (9, 11, 2001, 0x9ea450, 0x9ea3c8), // Cagliari × Pistoiese
    (9, 11, 2001, 0x9ea434, 0x9ea45c), // Cittadella × Messina
    (9, 11, 2001, 0x9ea448, 0x9ea3dc), // Crotone × Siena
    (9, 11, 2001, 0x9ea41c, 0x9ea440), // Empoli × Como
    (9, 11, 2001, 0x9ea3fc, 0x9ea40c), // Modena × Genoa
    (9, 11, 2001, 0x9ea414, 0x9ea42c), // Napoli × Cosenza
    (9, 11, 2001, 0x9ea424, 0x9ea3c0), // Palermo × Vicenza
    (9, 11, 2001, 0x9ea46c, 0x9ea3e4), // Reggina × Salernitana
    (9, 11, 2001, 0x9ea3f0, 0x9ea3d4), // Sampdoria × Ternana
    (16, 11, 2001, 0x9ea464, 0x9ea3fc), // Ancona × Modena
    (16, 11, 2001, 0x9ea440, 0x9ea3f0), // Como × Sampdoria
    (16, 11, 2001, 0x9ea42c, 0x9ea450), // Cosenza × Cagliari
    (16, 11, 2001, 0x9ea40c, 0x9ea41c), // Genoa × Empoli
    (16, 11, 2001, 0x9ea45c, 0x9ea404), // Messina × Bari
    (16, 11, 2001, 0x9ea3c8, 0x9ea414), // Pistoiese × Napoli
    (16, 11, 2001, 0x9ea3e4, 0x9ea424), // Salernitana × Palermo
    (16, 11, 2001, 0x9ea3dc, 0x9ea434), // Siena × Cittadella
    (16, 11, 2001, 0x9ea3d4, 0x9ea448), // Ternana × Crotone
    (16, 11, 2001, 0x9ea3c0, 0x9ea46c), // Vicenza × Reggina
    (23, 11, 2001, 0x9ea404, 0x9ea3e4), // Bari × Salernitana
    (23, 11, 2001, 0x9ea450, 0x9ea40c), // Cagliari × Genoa
    (23, 11, 2001, 0x9ea434, 0x9ea440), // Cittadella × Como
    (23, 11, 2001, 0x9ea448, 0x9ea3c8), // Crotone × Pistoiese
    (23, 11, 2001, 0x9ea41c, 0x9ea464), // Empoli × Ancona
    (23, 11, 2001, 0x9ea3fc, 0x9ea45c), // Modena × Messina
    (23, 11, 2001, 0x9ea414, 0x9ea3dc), // Napoli × Siena
    (23, 11, 2001, 0x9ea424, 0x9ea42c), // Palermo × Cosenza
    (23, 11, 2001, 0x9ea46c, 0x9ea3d4), // Reggina × Ternana
    (23, 11, 2001, 0x9ea3f0, 0x9ea3c0), // Sampdoria × Vicenza
    (6, 0, 2002, 0x9ea464, 0x9ea424), // Ancona × Palermo
    (6, 0, 2002, 0x9ea440, 0x9ea414), // Como × Napoli
    (6, 0, 2002, 0x9ea42c, 0x9ea41c), // Cosenza × Empoli
    (6, 0, 2002, 0x9ea40c, 0x9ea46c), // Genoa × Reggina
    (6, 0, 2002, 0x9ea45c, 0x9ea448), // Messina × Crotone
    (6, 0, 2002, 0x9ea3c8, 0x9ea434), // Pistoiese × Cittadella
    (6, 0, 2002, 0x9ea3e4, 0x9ea3fc), // Salernitana × Modena
    (6, 0, 2002, 0x9ea3dc, 0x9ea3f0), // Siena × Sampdoria
    (6, 0, 2002, 0x9ea3d4, 0x9ea404), // Ternana × Bari
    (6, 0, 2002, 0x9ea3c0, 0x9ea450), // Vicenza × Cagliari
    (13, 0, 2002, 0x9ea46c, 0x9ea464), // Reggina × Ancona
    (13, 0, 2002, 0x9ea45c, 0x9ea450), // Messina × Cagliari
    (13, 0, 2002, 0x9ea448, 0x9ea440), // Crotone × Como
    (13, 0, 2002, 0x9ea434, 0x9ea42c), // Cittadella × Cosenza
    (13, 0, 2002, 0x9ea424, 0x9ea41c), // Palermo × Empoli
    (13, 0, 2002, 0x9ea414, 0x9ea40c), // Napoli × Genoa
    (13, 0, 2002, 0x9ea404, 0x9ea3fc), // Bari × Modena
    (13, 0, 2002, 0x9ea3f0, 0x9ea3e4), // Sampdoria × Salernitana
    (13, 0, 2002, 0x9ea3dc, 0x9ea3d4), // Siena × Ternana
    (13, 0, 2002, 0x9ea3c8, 0x9ea3c0), // Pistoiese × Vicenza
    (20, 0, 2002, 0x9ea41c, 0x9ea404), // Empoli × Bari
    (20, 0, 2002, 0x9ea3e4, 0x9ea434), // Salernitana × Cittadella
    (20, 0, 2002, 0x9ea3fc, 0x9ea448), // Modena × Crotone
    (20, 0, 2002, 0x9ea3d4, 0x9ea45c), // Ternana × Messina
    (20, 0, 2002, 0x9ea464, 0x9ea414), // Ancona × Napoli
    (20, 0, 2002, 0x9ea450, 0x9ea424), // Cagliari × Palermo
    (20, 0, 2002, 0x9ea40c, 0x9ea3c8), // Genoa × Pistoiese
    (20, 0, 2002, 0x9ea440, 0x9ea46c), // Como × Reggina
    (20, 0, 2002, 0x9ea42c, 0x9ea3f0), // Cosenza × Sampdoria
    (20, 0, 2002, 0x9ea3c0, 0x9ea3dc), // Vicenza × Siena
    (3, 1, 2002, 0x9ea3dc, 0x9ea464), // Siena × Ancona
    (3, 1, 2002, 0x9ea404, 0x9ea450), // Bari × Cagliari
    (3, 1, 2002, 0x9ea3d4, 0x9ea42c), // Ternana × Cosenza
    (3, 1, 2002, 0x9ea45c, 0x9ea41c), // Messina × Empoli
    (3, 1, 2002, 0x9ea440, 0x9ea40c), // Como × Genoa
    (3, 1, 2002, 0x9ea414, 0x9ea3fc), // Napoli × Modena
    (3, 1, 2002, 0x9ea3c8, 0x9ea424), // Pistoiese × Palermo
    (3, 1, 2002, 0x9ea3f0, 0x9ea46c), // Sampdoria × Reggina
    (3, 1, 2002, 0x9ea448, 0x9ea3e4), // Crotone × Salernitana
    (3, 1, 2002, 0x9ea434, 0x9ea3c0), // Cittadella × Vicenza
    (10, 1, 2002, 0x9ea424, 0x9ea404), // Palermo × Bari
    (10, 1, 2002, 0x9ea46c, 0x9ea434), // Reggina × Cittadella
    (10, 1, 2002, 0x9ea3e4, 0x9ea440), // Salernitana × Como
    (10, 1, 2002, 0x9ea450, 0x9ea448), // Cagliari × Crotone
    (10, 1, 2002, 0x9ea464, 0x9ea45c), // Ancona × Messina
    (10, 1, 2002, 0x9ea3c0, 0x9ea414), // Vicenza × Napoli
    (10, 1, 2002, 0x9ea42c, 0x9ea3c8), // Cosenza × Pistoiese
    (10, 1, 2002, 0x9ea41c, 0x9ea3f0), // Empoli × Sampdoria
    (10, 1, 2002, 0x9ea40c, 0x9ea3dc), // Genoa × Siena
    (10, 1, 2002, 0x9ea3fc, 0x9ea3d4), // Modena × Ternana
    (17, 1, 2002, 0x9ea448, 0x9ea464), // Crotone × Ancona
    (17, 1, 2002, 0x9ea3f0, 0x9ea450), // Sampdoria × Cagliari
    (17, 1, 2002, 0x9ea440, 0x9ea42c), // Como × Cosenza
    (17, 1, 2002, 0x9ea414, 0x9ea41c), // Napoli × Empoli
    (17, 1, 2002, 0x9ea45c, 0x9ea40c), // Messina × Genoa
    (17, 1, 2002, 0x9ea3dc, 0x9ea3fc), // Siena × Modena
    (17, 1, 2002, 0x9ea434, 0x9ea424), // Cittadella × Palermo
    (17, 1, 2002, 0x9ea404, 0x9ea46c), // Bari × Reggina
    (17, 1, 2002, 0x9ea3c8, 0x9ea3e4), // Pistoiese × Salernitana
    (17, 1, 2002, 0x9ea3d4, 0x9ea3c0), // Ternana × Vicenza
    (24, 1, 2002, 0x9ea448, 0x9ea404), // Crotone × Bari
    (24, 1, 2002, 0x9ea41c, 0x9ea434), // Empoli × Cittadella
    (24, 1, 2002, 0x9ea464, 0x9ea440), // Ancona × Como
    (24, 1, 2002, 0x9ea46c, 0x9ea42c), // Reggina × Cosenza
    (24, 1, 2002, 0x9ea3c0, 0x9ea45c), // Vicenza × Messina
    (24, 1, 2002, 0x9ea450, 0x9ea414), // Cagliari × Napoli
    (24, 1, 2002, 0x9ea3fc, 0x9ea3c8), // Modena × Pistoiese
    (24, 1, 2002, 0x9ea424, 0x9ea3f0), // Palermo × Sampdoria
    (24, 1, 2002, 0x9ea3e4, 0x9ea3dc), // Salernitana × Siena
    (24, 1, 2002, 0x9ea40c, 0x9ea3d4), // Genoa × Ternana
    (3, 2, 2002, 0x9ea40c, 0x9ea464), // Genoa × Ancona
    (3, 2, 2002, 0x9ea3dc, 0x9ea450), // Siena × Cagliari
    (3, 2, 2002, 0x9ea434, 0x9ea448), // Cittadella × Crotone
    (3, 2, 2002, 0x9ea3c8, 0x9ea41c), // Pistoiese × Empoli
    (3, 2, 2002, 0x9ea440, 0x9ea3fc), // Como × Modena
    (3, 2, 2002, 0x9ea424, 0x9ea46c), // Palermo × Reggina
    (3, 2, 2002, 0x9ea45c, 0x9ea3e4), // Messina × Salernitana
    (3, 2, 2002, 0x9ea404, 0x9ea3f0), // Bari × Sampdoria
    (3, 2, 2002, 0x9ea414, 0x9ea3d4), // Napoli × Ternana
    (3, 2, 2002, 0x9ea42c, 0x9ea3c0), // Cosenza × Vicenza
    (10, 2, 2002, 0x9ea3f0, 0x9ea464), // Sampdoria × Ancona
    (10, 2, 2002, 0x9ea3d4, 0x9ea440), // Ternana × Como
    (10, 2, 2002, 0x9ea3fc, 0x9ea42c), // Modena × Cosenza
    (10, 2, 2002, 0x9ea450, 0x9ea41c), // Cagliari × Empoli
    (10, 2, 2002, 0x9ea434, 0x9ea40c), // Cittadella × Genoa
    (10, 2, 2002, 0x9ea3dc, 0x9ea45c), // Siena × Messina
    (10, 2, 2002, 0x9ea3e4, 0x9ea414), // Salernitana × Napoli
    (10, 2, 2002, 0x9ea448, 0x9ea424), // Crotone × Palermo
    (10, 2, 2002, 0x9ea46c, 0x9ea3c8), // Reggina × Pistoiese
    (10, 2, 2002, 0x9ea404, 0x9ea3c0), // Bari × Vicenza
    (17, 2, 2002, 0x9ea40c, 0x9ea404), // Genoa × Bari
    (17, 2, 2002, 0x9ea414, 0x9ea434), // Napoli × Cittadella
    (17, 2, 2002, 0x9ea3c0, 0x9ea448), // Vicenza × Crotone
    (17, 2, 2002, 0x9ea424, 0x9ea45c), // Palermo × Messina
    (17, 2, 2002, 0x9ea450, 0x9ea3fc), // Cagliari × Modena
    (17, 2, 2002, 0x9ea41c, 0x9ea46c), // Empoli × Reggina
    (17, 2, 2002, 0x9ea42c, 0x9ea3e4), // Cosenza × Salernitana
    (17, 2, 2002, 0x9ea3c8, 0x9ea3f0), // Pistoiese × Sampdoria
    (17, 2, 2002, 0x9ea440, 0x9ea3dc), // Como × Siena
    (17, 2, 2002, 0x9ea464, 0x9ea3d4), // Ancona × Ternana
    (30, 2, 2002, 0x9ea434, 0x9ea464), // Cittadella × Ancona
    (30, 2, 2002, 0x9ea46c, 0x9ea450), // Reggina × Cagliari
    (30, 2, 2002, 0x9ea45c, 0x9ea440), // Messina × Como
    (30, 2, 2002, 0x9ea3dc, 0x9ea42c), // Siena × Cosenza
    (30, 2, 2002, 0x9ea448, 0x9ea41c), // Crotone × Empoli
    (30, 2, 2002, 0x9ea3e4, 0x9ea40c), // Salernitana × Genoa
    (30, 2, 2002, 0x9ea3f0, 0x9ea414), // Sampdoria × Napoli
    (30, 2, 2002, 0x9ea3d4, 0x9ea424), // Ternana × Palermo
    (30, 2, 2002, 0x9ea404, 0x9ea3c8), // Bari × Pistoiese
    (30, 2, 2002, 0x9ea3fc, 0x9ea3c0), // Modena × Vicenza
    (7, 3, 2002, 0x9ea414, 0x9ea404), // Napoli × Bari
    (7, 3, 2002, 0x9ea450, 0x9ea434), // Cagliari × Cittadella
    (7, 3, 2002, 0x9ea3c0, 0x9ea440), // Vicenza × Como
    (7, 3, 2002, 0x9ea46c, 0x9ea448), // Reggina × Crotone
    (7, 3, 2002, 0x9ea3f0, 0x9ea40c), // Sampdoria × Genoa
    (7, 3, 2002, 0x9ea42c, 0x9ea45c), // Cosenza × Messina
    (7, 3, 2002, 0x9ea41c, 0x9ea3fc), // Empoli × Modena
    (7, 3, 2002, 0x9ea464, 0x9ea3e4), // Ancona × Salernitana
    (7, 3, 2002, 0x9ea424, 0x9ea3dc), // Palermo × Siena
    (7, 3, 2002, 0x9ea3c8, 0x9ea3d4), // Pistoiese × Ternana
    (14, 3, 2002, 0x9ea3dc, 0x9ea404), // Siena × Bari
    (14, 3, 2002, 0x9ea3d4, 0x9ea450), // Ternana × Cagliari
    (14, 3, 2002, 0x9ea40c, 0x9ea42c), // Genoa × Cosenza
    (14, 3, 2002, 0x9ea3e4, 0x9ea41c), // Salernitana × Empoli
    (14, 3, 2002, 0x9ea448, 0x9ea414), // Crotone × Napoli
    (14, 3, 2002, 0x9ea3fc, 0x9ea424), // Modena × Palermo
    (14, 3, 2002, 0x9ea440, 0x9ea3c8), // Como × Pistoiese
    (14, 3, 2002, 0x9ea45c, 0x9ea46c), // Messina × Reggina
    (14, 3, 2002, 0x9ea434, 0x9ea3f0), // Cittadella × Sampdoria
    (14, 3, 2002, 0x9ea464, 0x9ea3c0), // Ancona × Vicenza
    (21, 3, 2002, 0x9ea42c, 0x9ea464), // Cosenza × Ancona
    (21, 3, 2002, 0x9ea404, 0x9ea434), // Bari × Cittadella
    (21, 3, 2002, 0x9ea424, 0x9ea440), // Palermo × Como
    (21, 3, 2002, 0x9ea3f0, 0x9ea448), // Sampdoria × Crotone
    (21, 3, 2002, 0x9ea3c0, 0x9ea40c), // Vicenza × Genoa
    (21, 3, 2002, 0x9ea414, 0x9ea45c), // Napoli × Messina
    (21, 3, 2002, 0x9ea46c, 0x9ea3fc), // Reggina × Modena
    (21, 3, 2002, 0x9ea450, 0x9ea3e4), // Cagliari × Salernitana
    (21, 3, 2002, 0x9ea3c8, 0x9ea3dc), // Pistoiese × Siena
    (21, 3, 2002, 0x9ea41c, 0x9ea3d4), // Empoli × Ternana
    (28, 3, 2002, 0x9ea440, 0x9ea404), // Como × Bari
    (28, 3, 2002, 0x9ea464, 0x9ea450), // Ancona × Cagliari
    (28, 3, 2002, 0x9ea3d4, 0x9ea434), // Ternana × Cittadella
    (28, 3, 2002, 0x9ea42c, 0x9ea448), // Cosenza × Crotone
    (28, 3, 2002, 0x9ea3dc, 0x9ea41c), // Siena × Empoli
    (28, 3, 2002, 0x9ea40c, 0x9ea424), // Genoa × Palermo
    (28, 3, 2002, 0x9ea45c, 0x9ea3c8), // Messina × Pistoiese
    (28, 3, 2002, 0x9ea414, 0x9ea46c), // Napoli × Reggina
    (28, 3, 2002, 0x9ea3c0, 0x9ea3e4), // Vicenza × Salernitana
    (28, 3, 2002, 0x9ea3fc, 0x9ea3f0), // Modena × Sampdoria
    (5, 4, 2002, 0x9ea3c8, 0x9ea464), // Pistoiese × Ancona
    (5, 4, 2002, 0x9ea450, 0x9ea440), // Cagliari × Como
    (5, 4, 2002, 0x9ea404, 0x9ea42c), // Bari × Cosenza
    (5, 4, 2002, 0x9ea448, 0x9ea40c), // Crotone × Genoa
    (5, 4, 2002, 0x9ea3f0, 0x9ea45c), // Sampdoria × Messina
    (5, 4, 2002, 0x9ea434, 0x9ea3fc), // Cittadella × Modena
    (5, 4, 2002, 0x9ea424, 0x9ea414), // Palermo × Napoli
    (5, 4, 2002, 0x9ea46c, 0x9ea3dc), // Reggina × Siena
    (5, 4, 2002, 0x9ea3e4, 0x9ea3d4), // Salernitana × Ternana
    (5, 4, 2002, 0x9ea41c, 0x9ea3c0), // Empoli × Vicenza
    (12, 4, 2002, 0x9ea464, 0x9ea404), // Ancona × Bari
    (12, 4, 2002, 0x9ea3c8, 0x9ea450), // Pistoiese × Cagliari
    (12, 4, 2002, 0x9ea45c, 0x9ea434), // Messina × Cittadella
    (12, 4, 2002, 0x9ea3dc, 0x9ea448), // Siena × Crotone
    (12, 4, 2002, 0x9ea440, 0x9ea41c), // Como × Empoli
    (12, 4, 2002, 0x9ea40c, 0x9ea3fc), // Genoa × Modena
    (12, 4, 2002, 0x9ea42c, 0x9ea414), // Cosenza × Napoli
    (12, 4, 2002, 0x9ea3c0, 0x9ea424), // Vicenza × Palermo
    (12, 4, 2002, 0x9ea3e4, 0x9ea46c), // Salernitana × Reggina
    (12, 4, 2002, 0x9ea3d4, 0x9ea3f0), // Ternana × Sampdoria
    (19, 4, 2002, 0x9ea3fc, 0x9ea464), // Modena × Ancona
    (19, 4, 2002, 0x9ea3f0, 0x9ea440), // Sampdoria × Como
    (19, 4, 2002, 0x9ea450, 0x9ea42c), // Cagliari × Cosenza
    (19, 4, 2002, 0x9ea41c, 0x9ea40c), // Empoli × Genoa
    (19, 4, 2002, 0x9ea404, 0x9ea45c), // Bari × Messina
    (19, 4, 2002, 0x9ea414, 0x9ea3c8), // Napoli × Pistoiese
    (19, 4, 2002, 0x9ea424, 0x9ea3e4), // Palermo × Salernitana
    (19, 4, 2002, 0x9ea434, 0x9ea3dc), // Cittadella × Siena
    (19, 4, 2002, 0x9ea448, 0x9ea3d4), // Crotone × Ternana
    (19, 4, 2002, 0x9ea46c, 0x9ea3c0), // Reggina × Vicenza
    (26, 4, 2002, 0x9ea3e4, 0x9ea404), // Salernitana × Bari
    (26, 4, 2002, 0x9ea40c, 0x9ea450), // Genoa × Cagliari
    (26, 4, 2002, 0x9ea440, 0x9ea434), // Como × Cittadella
    (26, 4, 2002, 0x9ea3c8, 0x9ea448), // Pistoiese × Crotone
    (26, 4, 2002, 0x9ea464, 0x9ea41c), // Ancona × Empoli
    (26, 4, 2002, 0x9ea45c, 0x9ea3fc), // Messina × Modena
    (26, 4, 2002, 0x9ea3dc, 0x9ea414), // Siena × Napoli
    (26, 4, 2002, 0x9ea42c, 0x9ea424), // Cosenza × Palermo
    (26, 4, 2002, 0x9ea3d4, 0x9ea46c), // Ternana × Reggina
    (26, 4, 2002, 0x9ea3c0, 0x9ea3f0), // Vicenza × Sampdoria
    (2, 5, 2002, 0x9ea424, 0x9ea464), // Palermo × Ancona
    (2, 5, 2002, 0x9ea414, 0x9ea440), // Napoli × Como
    (2, 5, 2002, 0x9ea41c, 0x9ea42c), // Empoli × Cosenza
    (2, 5, 2002, 0x9ea46c, 0x9ea40c), // Reggina × Genoa
    (2, 5, 2002, 0x9ea448, 0x9ea45c), // Crotone × Messina
    (2, 5, 2002, 0x9ea434, 0x9ea3c8), // Cittadella × Pistoiese
    (2, 5, 2002, 0x9ea3fc, 0x9ea3e4), // Modena × Salernitana
    (2, 5, 2002, 0x9ea3f0, 0x9ea3dc), // Sampdoria × Siena
    (2, 5, 2002, 0x9ea404, 0x9ea3d4), // Bari × Ternana
    (2, 5, 2002, 0x9ea450, 0x9ea3c0), // Cagliari × Vicenza
];


// ---------- 278a leva: o calendário real de 2001/02 dos dois grupos da Série C1 italiana (os jogos fixos pelo nome dos clubes) e os dois tocos que devolvem 1 (ita_ser_c1a.cpp, ita_ser_c1b.cpp, fra_first.cpp) ----------

unsafe fn cqn6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqn6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Quantos jogos a liga tem: (clubes +0x3e / 2, com sinal, para zero) × rodadas +0xa9.
unsafe fn cqn6_jogos(liga: usize) -> i32 {
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32;
    let rodadas = core::ptr::read_unaligned((liga + 0xa9) as *const i16) as i32;
    (clubes / 2).wrapping_mul(rodadas)
}

/// O corpo das duas (set_up_fixed_fixtures; thiscall, ret 0, quadro SEH só para o construtor de vetor): o vetor dos
/// jogos fixos (operator new 0x944cf1 de n × 16 + 4: push; add esp, 4 -> cdecl; a conta n no primeiro dword e cada jogo
/// pelo construtor 0x645800, com o destrutor 0x5a53c0 para a exceção: `eh vector constructor iterator` 0x9463af, push
/// 0x5a53c0; push 0x645800; push n; push 0x10; push vetor -> stdcall, ret 0x14) em +0xea; sem memória, a caixa da
/// linha `linha` do fonte `arquivo` e fim. Depois cada jogo da `tabela`, na ordem, no seu lugar do vetor
/// (fixed_fixture_set_by_club_names 0x68aff0: push o 6º (fora, pelo mapa); push o 5º (casa); push ano; push mês (0 =
/// janeiro); push dia; push liga; mov ecx, [+0xea] + 16 × k -> thiscall, ret 0x18: a data e os clubes achados pelo
/// nome, −1 em +8/+0xc se não acha). Por fim, se algum dos n jogos ficou com um clube sem achar (+8 ou +0xc = −1), o
/// vetor é destruído (`eh vector destructor iterator` 0x9462bb: push 0x5a53c0; push conta; push 0x10; push vetor ->
/// stdcall, ret 0x10; operator delete 0x944c94 do bloco: push; add esp, 4 -> cdecl) e +0xea = 0: a liga volta ao
/// calendário sorteado.
unsafe fn cqn6_fixos(this: usize, tabela: &[(u8, u8, u16, u32, u32)], arquivo: usize, linha: u32) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let destruir: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let jogo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) = core::mem::transmute(0x68aff0usize);
    let n = cqn6_jogos(this);
    let bloco = novo((n as u32).wrapping_mul(16).wrapping_add(4));
    let vetor = if bloco != 0 {
        cqn6_pd(bloco, n as usize);
        construir(bloco + 4, 0x10, n as u32, 0x645800, 0x5a53c0);
        bloco + 4
    } else {
        0
    };
    cqn6_pd(this + 0xea, vetor);
    if vetor == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, &(dia, mes, ano, casa, fora)) in tabela.iter().enumerate() {
        jogo(cqn6_d(this + 0xea) + k * 0x10, this, dia as u32, mes as u32, ano as u32, casa, fora); // cqn6: cada jogo fixo
    }
    crate::comparacao::ramo("cqn_6 calendario fixo");
    let n = cqn6_jogos(this);
    if n <= 0 {
        return;
    }
    let v = cqn6_d(this + 0xea);
    for i in 0..n as usize {
        let f = v + i * 0x10;
        if cqn6_d(f + 8) == 0xffff_ffff || cqn6_d(f + 0xc) == 0xffff_ffff { // cqn6: um clube sem achar derruba a lista
            if v != 0 {
                destruir(v, 0x10, cqn6_d(v - 4) as u32, 0x5a53c0);
                apagar(v - 4);
            }
            cqn6_pd(this + 0xea, 0);
            return;
        }
    }
}

/// { 0x64be90 } (thiscall, ret 0, ita_ser_c1a.cpp, 11.650 bytes): `ItaSerC1a::set_up_fixed_fixtures()`: os 306 jogos
/// da C1/A 2001/02 (34 rodadas de 9). Fonte 0x9ea474, linha 0x3fd.
unsafe extern "thiscall" fn calendario_fixo_da_c1a(this: usize) {
    cqn6_fixos(this, &CQN6_C1A, 0x9ea474, 0x3fd)
}

/// { 0x650100 } (thiscall, ret 0, ita_ser_c1b.cpp, 11.650 bytes): `ItaSerC1b::set_up_fixed_fixtures()`: os 306 jogos
/// da C1/B 2001/02. Fonte 0x9ea558, linha 0x404.
unsafe extern "thiscall" fn calendario_fixo_da_c1b(this: usize) {
    cqn6_fixos(this, &CQN6_C1B, 0x9ea558, 0x404)
}
/// O comparador das duas: alocam: só a original, conferindo o que ela garante: ou a lista ficou (+0xea com a conta n e
/// cada jogo com os dois clubes achados), ou foi derrubada (+0xea = 0).
unsafe fn cqn6_comparar(nome: &str, this: usize, original: impl FnOnce()) {
    original();
    let v = cqn6_d(this + 0xea);
    let n = cqn6_jogos(this);
    let ok = v == 0 || (cqn6_d(v - 4) as i32 == n && (0..n.max(0) as usize).all(|i| cqn6_d(v + i * 0x10 + 8) != 0xffff_ffff && cqn6_d(v + i * 0x10 + 0xc) != 0xffff_ffff));
    PLACAR.registrar(ok, || format!("{}({:#x}): a lista dos jogos fixos (+0xea = {:#x}) fora do esperado", nome, this, v));
}

unsafe extern "thiscall" fn comparar_calendario_fixo_da_c1a(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1883);
    cqn6_comparar("calendario_fixo_da_c1a", this, || o(this))
}

unsafe extern "thiscall" fn comparar_calendario_fixo_da_c1b(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1884);
    cqn6_comparar("calendario_fixo_da_c1b", this, || o(this))
}

/// { 0x5bb540 } (thiscall, ret 8, fra_first.cpp, 8 bytes): `stub_return_one_ret8(a, b)`, o toco das competições que não
/// jogam partidas no lugar do add_result (o slot +4): ignora os dois e devolve 1.
unsafe extern "thiscall" fn toco_um_com_dois(_this: usize, _a: u32, _b: u32) -> u32 {
    1 // cqn6: o toco devolve 1
}

/// { 0x5bb550 } (thiscall, ret 4, fra_first.cpp, 8 bytes): `stub_return_one_ret4(a)`, o padrão do slot 25 das
/// competições e do slot 6 do órgão disciplinar: ignora o argumento e devolve 1.
unsafe extern "thiscall" fn toco_um_com_um(_this: usize, _a: u32) -> u32 {
    1
}

/// Os dois tocos: compara a devolução.
unsafe extern "thiscall" fn comparar_toco_um_com_dois(this: usize, a: u32, b: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(1885)(this, a, b);
    let n = toco_um_com_dois(this, a, b);
    PLACAR.registrar(r == n, || format!("toco_um_com_dois: original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_toco_um_com_um(this: usize, a: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(1886)(this, a);
    let n = toco_um_com_um(this, a);
    PLACAR.registrar(r == n, || format!("toco_um_com_um: original={} rust={}", r, n));
    r
}

/// Os jogos fixos (dia, mês com 0 = janeiro, ano, o 5º argumento (casa, pelo mapa), o 6º (fora)), os nomes no .rdata do
/// exe; na ordem da original (extraídos da desmontagem e conferidos: os lugares do vetor vão de 16 em 16).
const CQN6_C1A: [(u8, u8, u16, u32, u32); 306] = [
    (2, 8, 2001, 0x9ea540, 0x9ea54c), // Albinoleffe × Carrarese
    (2, 8, 2001, 0x9ea530, 0x9ea538), // Alzano × Arezzo
    (2, 8, 2001, 0x9ea520, 0x9ea528), // Cesena × Livorno
    (2, 8, 2001, 0x9ea50c, 0x9ea518), // Lumezzane × Lecco
    (2, 8, 2001, 0x9ea4fc, 0x9ea504), // Pisa × Treviso
    (2, 8, 2001, 0x9ea4e4, 0x9ea4f0), // Reggiana × Triestina
    (2, 8, 2001, 0x9ea4d4, 0x9ea4dc), // Spal × Padova
    (2, 8, 2001, 0x9ea4c4, 0x9ea4cc), // Spezia × Monza
    (2, 8, 2001, 0x9ea4b0, 0x9ea4b8), // Varese × Lucchese
    (9, 8, 2001, 0x9ea538, 0x9ea4fc), // Arezzo × Pisa
    (9, 8, 2001, 0x9ea54c, 0x9ea4d4), // Carrarese × Spal
    (9, 8, 2001, 0x9ea518, 0x9ea540), // Lecco × Albinoleffe
    (9, 8, 2001, 0x9ea528, 0x9ea4b0), // Livorno × Varese
    (9, 8, 2001, 0x9ea4b8, 0x9ea50c), // Lucchese × Lumezzane
    (9, 8, 2001, 0x9ea4cc, 0x9ea520), // Monza × Cesena
    (9, 8, 2001, 0x9ea4dc, 0x9ea4e4), // Padova × Reggiana
    (9, 8, 2001, 0x9ea504, 0x9ea4c4), // Treviso × Spezia
    (9, 8, 2001, 0x9ea4f0, 0x9ea530), // Triestina × Alzano
    (16, 8, 2001, 0x9ea540, 0x9ea4f0), // Albinoleffe × Triestina
    (16, 8, 2001, 0x9ea530, 0x9ea4c4), // Alzano × Spezia
    (16, 8, 2001, 0x9ea538, 0x9ea528), // Arezzo × Livorno
    (16, 8, 2001, 0x9ea54c, 0x9ea4dc), // Carrarese × Padova
    (16, 8, 2001, 0x9ea520, 0x9ea4e4), // Cesena × Reggiana
    (16, 8, 2001, 0x9ea50c, 0x9ea4cc), // Lumezzane × Monza
    (16, 8, 2001, 0x9ea4fc, 0x9ea4b8), // Pisa × Lucchese
    (16, 8, 2001, 0x9ea4d4, 0x9ea518), // Spal × Lecco
    (16, 8, 2001, 0x9ea4b0, 0x9ea504), // Varese × Treviso
    (23, 8, 2001, 0x9ea518, 0x9ea520), // Lecco × Cesena
    (23, 8, 2001, 0x9ea528, 0x9ea4d4), // Livorno × Spal
    (23, 8, 2001, 0x9ea4b8, 0x9ea54c), // Lucchese × Carrarese
    (23, 8, 2001, 0x9ea4cc, 0x9ea4fc), // Monza × Pisa
    (23, 8, 2001, 0x9ea4dc, 0x9ea540), // Padova × Albinoleffe
    (23, 8, 2001, 0x9ea4e4, 0x9ea530), // Reggiana × Alzano
    (23, 8, 2001, 0x9ea4c4, 0x9ea4b0), // Spezia × Varese
    (23, 8, 2001, 0x9ea504, 0x9ea50c), // Treviso × Lumezzane
    (23, 8, 2001, 0x9ea4f0, 0x9ea538), // Triestina × Arezzo
    (30, 8, 2001, 0x9ea540, 0x9ea4e4), // Albinoleffe × Reggiana
    (30, 8, 2001, 0x9ea530, 0x9ea4cc), // Alzano × Monza
    (30, 8, 2001, 0x9ea538, 0x9ea4dc), // Arezzo × Padova
    (30, 8, 2001, 0x9ea54c, 0x9ea528), // Carrarese × Livorno
    (30, 8, 2001, 0x9ea520, 0x9ea504), // Cesena × Treviso
    (30, 8, 2001, 0x9ea518, 0x9ea4f0), // Lecco × Triestina
    (30, 8, 2001, 0x9ea4fc, 0x9ea4c4), // Pisa × Spezia
    (30, 8, 2001, 0x9ea4d4, 0x9ea4b8), // Spal × Lucchese
    (30, 8, 2001, 0x9ea4b0, 0x9ea50c), // Varese × Lumezzane
    (7, 9, 2001, 0x9ea530, 0x9ea4b0), // Alzano × Varese
    (7, 9, 2001, 0x9ea4b8, 0x9ea518), // Lucchese × Lecco
    (7, 9, 2001, 0x9ea50c, 0x9ea520), // Lumezzane × Cesena
    (7, 9, 2001, 0x9ea4cc, 0x9ea538), // Monza × Arezzo
    (7, 9, 2001, 0x9ea4dc, 0x9ea4f0), // Padova × Triestina
    (7, 9, 2001, 0x9ea4fc, 0x9ea540), // Pisa × Albinoleffe
    (7, 9, 2001, 0x9ea4e4, 0x9ea528), // Reggiana × Livorno
    (7, 9, 2001, 0x9ea4c4, 0x9ea54c), // Spezia × Carrarese
    (7, 9, 2001, 0x9ea504, 0x9ea4d4), // Treviso × Spal
    (14, 9, 2001, 0x9ea540, 0x9ea4b8), // Albinoleffe × Lucchese
    (14, 9, 2001, 0x9ea538, 0x9ea4b0), // Arezzo × Varese
    (14, 9, 2001, 0x9ea54c, 0x9ea4e4), // Carrarese × Reggiana
    (14, 9, 2001, 0x9ea520, 0x9ea4c4), // Cesena × Spezia
    (14, 9, 2001, 0x9ea518, 0x9ea530), // Lecco × Alzano
    (14, 9, 2001, 0x9ea528, 0x9ea4fc), // Livorno × Pisa
    (14, 9, 2001, 0x9ea4dc, 0x9ea504), // Padova × Treviso
    (14, 9, 2001, 0x9ea4d4, 0x9ea4cc), // Spal × Monza
    (14, 9, 2001, 0x9ea4f0, 0x9ea50c), // Triestina × Lumezzane
    (21, 9, 2001, 0x9ea520, 0x9ea538), // Cesena × Arezzo
    (21, 9, 2001, 0x9ea50c, 0x9ea540), // Lumezzane × Albinoleffe
    (21, 9, 2001, 0x9ea4cc, 0x9ea4dc), // Monza × Padova
    (21, 9, 2001, 0x9ea4fc, 0x9ea530), // Pisa × Alzano
    (21, 9, 2001, 0x9ea4e4, 0x9ea4b8), // Reggiana × Lucchese
    (21, 9, 2001, 0x9ea4c4, 0x9ea518), // Spezia × Lecco
    (21, 9, 2001, 0x9ea504, 0x9ea54c), // Treviso × Carrarese
    (21, 9, 2001, 0x9ea4f0, 0x9ea528), // Triestina × Livorno
    (21, 9, 2001, 0x9ea4b0, 0x9ea4d4), // Varese × Spal
    (28, 9, 2001, 0x9ea540, 0x9ea504), // Albinoleffe × Treviso
    (28, 9, 2001, 0x9ea530, 0x9ea520), // Alzano × Cesena
    (28, 9, 2001, 0x9ea538, 0x9ea50c), // Arezzo × Lumezzane
    (28, 9, 2001, 0x9ea54c, 0x9ea4cc), // Carrarese × Monza
    (28, 9, 2001, 0x9ea518, 0x9ea4e4), // Lecco × Reggiana
    (28, 9, 2001, 0x9ea528, 0x9ea4dc), // Livorno × Padova
    (28, 9, 2001, 0x9ea4b8, 0x9ea4f0), // Lucchese × Triestina
    (28, 9, 2001, 0x9ea4d4, 0x9ea4c4), // Spal × Spezia
    (28, 9, 2001, 0x9ea4b0, 0x9ea4fc), // Varese × Pisa
    (4, 10, 2001, 0x9ea520, 0x9ea4b0), // Cesena × Varese
    (4, 10, 2001, 0x9ea528, 0x9ea4b8), // Livorno × Lucchese
    (4, 10, 2001, 0x9ea50c, 0x9ea4fc), // Lumezzane × Pisa
    (4, 10, 2001, 0x9ea4cc, 0x9ea540), // Monza × Albinoleffe
    (4, 10, 2001, 0x9ea4dc, 0x9ea518), // Padova × Lecco
    (4, 10, 2001, 0x9ea4e4, 0x9ea4d4), // Reggiana × Spal
    (4, 10, 2001, 0x9ea4c4, 0x9ea538), // Spezia × Arezzo
    (4, 10, 2001, 0x9ea504, 0x9ea530), // Treviso × Alzano
    (4, 10, 2001, 0x9ea4f0, 0x9ea54c), // Triestina × Carrarese
    (11, 10, 2001, 0x9ea540, 0x9ea528), // Albinoleffe × Livorno
    (11, 10, 2001, 0x9ea530, 0x9ea50c), // Alzano × Lumezzane
    (11, 10, 2001, 0x9ea538, 0x9ea504), // Arezzo × Treviso
    (11, 10, 2001, 0x9ea518, 0x9ea54c), // Lecco × Carrarese
    (11, 10, 2001, 0x9ea4b8, 0x9ea4dc), // Lucchese × Padova
    (11, 10, 2001, 0x9ea4fc, 0x9ea520), // Pisa × Cesena
    (11, 10, 2001, 0x9ea4d4, 0x9ea4f0), // Spal × Triestina
    (11, 10, 2001, 0x9ea4c4, 0x9ea4e4), // Spezia × Reggiana
    (11, 10, 2001, 0x9ea4b0, 0x9ea4cc), // Varese × Monza
    (18, 10, 2001, 0x9ea540, 0x9ea4d4), // Albinoleffe × Spal
    (18, 10, 2001, 0x9ea54c, 0x9ea4fc), // Carrarese × Pisa
    (18, 10, 2001, 0x9ea528, 0x9ea518), // Livorno × Lecco
    (18, 10, 2001, 0x9ea4b8, 0x9ea538), // Lucchese × Arezzo
    (18, 10, 2001, 0x9ea50c, 0x9ea4c4), // Lumezzane × Spezia
    (18, 10, 2001, 0x9ea4dc, 0x9ea530), // Padova × Alzano
    (18, 10, 2001, 0x9ea4e4, 0x9ea4b0), // Reggiana × Varese
    (18, 10, 2001, 0x9ea504, 0x9ea4cc), // Treviso × Monza
    (18, 10, 2001, 0x9ea4f0, 0x9ea520), // Triestina × Cesena
    (25, 10, 2001, 0x9ea530, 0x9ea540), // Alzano × Albinoleffe
    (25, 10, 2001, 0x9ea538, 0x9ea4e4), // Arezzo × Reggiana
    (25, 10, 2001, 0x9ea520, 0x9ea4dc), // Cesena × Padova
    (25, 10, 2001, 0x9ea50c, 0x9ea54c), // Lumezzane × Carrarese
    (25, 10, 2001, 0x9ea4cc, 0x9ea4b8), // Monza × Lucchese
    (25, 10, 2001, 0x9ea4fc, 0x9ea4d4), // Pisa × Spal
    (25, 10, 2001, 0x9ea4c4, 0x9ea528), // Spezia × Livorno
    (25, 10, 2001, 0x9ea504, 0x9ea4f0), // Treviso × Triestina
    (25, 10, 2001, 0x9ea4b0, 0x9ea518), // Varese × Lecco
    (2, 11, 2001, 0x9ea540, 0x9ea4b0), // Albinoleffe × Varese
    (2, 11, 2001, 0x9ea54c, 0x9ea530), // Carrarese × Alzano
    (2, 11, 2001, 0x9ea518, 0x9ea538), // Lecco × Arezzo
    (2, 11, 2001, 0x9ea528, 0x9ea4cc), // Livorno × Monza
    (2, 11, 2001, 0x9ea4b8, 0x9ea504), // Lucchese × Treviso
    (2, 11, 2001, 0x9ea4dc, 0x9ea4fc), // Padova × Pisa
    (2, 11, 2001, 0x9ea4e4, 0x9ea50c), // Reggiana × Lumezzane
    (2, 11, 2001, 0x9ea4d4, 0x9ea520), // Spal × Cesena
    (2, 11, 2001, 0x9ea4f0, 0x9ea4c4), // Triestina × Spezia
    (9, 11, 2001, 0x9ea530, 0x9ea4d4), // Alzano × Spal
    (9, 11, 2001, 0x9ea538, 0x9ea540), // Arezzo × Albinoleffe
    (9, 11, 2001, 0x9ea520, 0x9ea54c), // Cesena × Carrarese
    (9, 11, 2001, 0x9ea50c, 0x9ea528), // Lumezzane × Livorno
    (9, 11, 2001, 0x9ea4cc, 0x9ea518), // Monza × Lecco
    (9, 11, 2001, 0x9ea4fc, 0x9ea4f0), // Pisa × Triestina
    (9, 11, 2001, 0x9ea4c4, 0x9ea4b8), // Spezia × Lucchese
    (9, 11, 2001, 0x9ea504, 0x9ea4e4), // Treviso × Reggiana
    (9, 11, 2001, 0x9ea4b0, 0x9ea4dc), // Varese × Padova
    (16, 11, 2001, 0x9ea540, 0x9ea520), // Albinoleffe × Cesena
    (16, 11, 2001, 0x9ea54c, 0x9ea538), // Carrarese × Arezzo
    (16, 11, 2001, 0x9ea518, 0x9ea4fc), // Lecco × Pisa
    (16, 11, 2001, 0x9ea528, 0x9ea504), // Livorno × Treviso
    (16, 11, 2001, 0x9ea4b8, 0x9ea530), // Lucchese × Alzano
    (16, 11, 2001, 0x9ea4dc, 0x9ea4c4), // Padova × Spezia
    (16, 11, 2001, 0x9ea4e4, 0x9ea4cc), // Reggiana × Monza
    (16, 11, 2001, 0x9ea4d4, 0x9ea50c), // Spal × Lumezzane
    (16, 11, 2001, 0x9ea4f0, 0x9ea4b0), // Triestina × Varese
    (23, 11, 2001, 0x9ea530, 0x9ea528), // Alzano × Livorno
    (23, 11, 2001, 0x9ea538, 0x9ea4d4), // Arezzo × Spal
    (23, 11, 2001, 0x9ea520, 0x9ea4b8), // Cesena × Lucchese
    (23, 11, 2001, 0x9ea50c, 0x9ea4dc), // Lumezzane × Padova
    (23, 11, 2001, 0x9ea4cc, 0x9ea4f0), // Monza × Triestina
    (23, 11, 2001, 0x9ea4fc, 0x9ea4e4), // Pisa × Reggiana
    (23, 11, 2001, 0x9ea4c4, 0x9ea540), // Spezia × Albinoleffe
    (23, 11, 2001, 0x9ea504, 0x9ea518), // Treviso × Lecco
    (23, 11, 2001, 0x9ea4b0, 0x9ea54c), // Varese × Carrarese
    (6, 0, 2002, 0x9ea54c, 0x9ea540), // Carrarese × Albinoleffe
    (6, 0, 2002, 0x9ea538, 0x9ea530), // Arezzo × Alzano
    (6, 0, 2002, 0x9ea528, 0x9ea520), // Livorno × Cesena
    (6, 0, 2002, 0x9ea518, 0x9ea50c), // Lecco × Lumezzane
    (6, 0, 2002, 0x9ea504, 0x9ea4fc), // Treviso × Pisa
    (6, 0, 2002, 0x9ea4f0, 0x9ea4e4), // Triestina × Reggiana
    (6, 0, 2002, 0x9ea4dc, 0x9ea4d4), // Padova × Spal
    (6, 0, 2002, 0x9ea4cc, 0x9ea4c4), // Monza × Spezia
    (6, 0, 2002, 0x9ea4b8, 0x9ea4b0), // Lucchese × Varese
    (13, 0, 2002, 0x9ea4fc, 0x9ea538), // Pisa × Arezzo
    (13, 0, 2002, 0x9ea4d4, 0x9ea54c), // Spal × Carrarese
    (13, 0, 2002, 0x9ea540, 0x9ea518), // Albinoleffe × Lecco
    (13, 0, 2002, 0x9ea4b0, 0x9ea528), // Varese × Livorno
    (13, 0, 2002, 0x9ea50c, 0x9ea4b8), // Lumezzane × Lucchese
    (13, 0, 2002, 0x9ea520, 0x9ea4cc), // Cesena × Monza
    (13, 0, 2002, 0x9ea4e4, 0x9ea4dc), // Reggiana × Padova
    (13, 0, 2002, 0x9ea4c4, 0x9ea504), // Spezia × Treviso
    (13, 0, 2002, 0x9ea530, 0x9ea4f0), // Alzano × Triestina
    (20, 0, 2002, 0x9ea4f0, 0x9ea540), // Triestina × Albinoleffe
    (20, 0, 2002, 0x9ea4c4, 0x9ea530), // Spezia × Alzano
    (20, 0, 2002, 0x9ea528, 0x9ea538), // Livorno × Arezzo
    (20, 0, 2002, 0x9ea4dc, 0x9ea54c), // Padova × Carrarese
    (20, 0, 2002, 0x9ea4e4, 0x9ea520), // Reggiana × Cesena
    (20, 0, 2002, 0x9ea4cc, 0x9ea50c), // Monza × Lumezzane
    (20, 0, 2002, 0x9ea4b8, 0x9ea4fc), // Lucchese × Pisa
    (20, 0, 2002, 0x9ea518, 0x9ea4d4), // Lecco × Spal
    (20, 0, 2002, 0x9ea504, 0x9ea4b0), // Treviso × Varese
    (27, 0, 2002, 0x9ea520, 0x9ea518), // Cesena × Lecco
    (27, 0, 2002, 0x9ea4d4, 0x9ea528), // Spal × Livorno
    (27, 0, 2002, 0x9ea54c, 0x9ea4b8), // Carrarese × Lucchese
    (27, 0, 2002, 0x9ea4fc, 0x9ea4cc), // Pisa × Monza
    (27, 0, 2002, 0x9ea540, 0x9ea4dc), // Albinoleffe × Padova
    (27, 0, 2002, 0x9ea530, 0x9ea4e4), // Alzano × Reggiana
    (27, 0, 2002, 0x9ea4b0, 0x9ea4c4), // Varese × Spezia
    (27, 0, 2002, 0x9ea50c, 0x9ea504), // Lumezzane × Treviso
    (27, 0, 2002, 0x9ea538, 0x9ea4f0), // Arezzo × Triestina
    (3, 1, 2002, 0x9ea4e4, 0x9ea540), // Reggiana × Albinoleffe
    (3, 1, 2002, 0x9ea4cc, 0x9ea530), // Monza × Alzano
    (3, 1, 2002, 0x9ea4dc, 0x9ea538), // Padova × Arezzo
    (3, 1, 2002, 0x9ea528, 0x9ea54c), // Livorno × Carrarese
    (3, 1, 2002, 0x9ea504, 0x9ea520), // Treviso × Cesena
    (3, 1, 2002, 0x9ea4f0, 0x9ea518), // Triestina × Lecco
    (3, 1, 2002, 0x9ea4c4, 0x9ea4fc), // Spezia × Pisa
    (3, 1, 2002, 0x9ea4b8, 0x9ea4d4), // Lucchese × Spal
    (3, 1, 2002, 0x9ea50c, 0x9ea4b0), // Lumezzane × Varese
    (10, 1, 2002, 0x9ea4b0, 0x9ea530), // Varese × Alzano
    (10, 1, 2002, 0x9ea518, 0x9ea4b8), // Lecco × Lucchese
    (10, 1, 2002, 0x9ea520, 0x9ea50c), // Cesena × Lumezzane
    (10, 1, 2002, 0x9ea538, 0x9ea4cc), // Arezzo × Monza
    (10, 1, 2002, 0x9ea4f0, 0x9ea4dc), // Triestina × Padova
    (10, 1, 2002, 0x9ea540, 0x9ea4fc), // Albinoleffe × Pisa
    (10, 1, 2002, 0x9ea528, 0x9ea4e4), // Livorno × Reggiana
    (10, 1, 2002, 0x9ea54c, 0x9ea4c4), // Carrarese × Spezia
    (10, 1, 2002, 0x9ea4d4, 0x9ea504), // Spal × Treviso
    (17, 1, 2002, 0x9ea4b8, 0x9ea540), // Lucchese × Albinoleffe
    (17, 1, 2002, 0x9ea4b0, 0x9ea538), // Varese × Arezzo
    (17, 1, 2002, 0x9ea4e4, 0x9ea54c), // Reggiana × Carrarese
    (17, 1, 2002, 0x9ea4c4, 0x9ea520), // Spezia × Cesena
    (17, 1, 2002, 0x9ea530, 0x9ea518), // Alzano × Lecco
    (17, 1, 2002, 0x9ea4fc, 0x9ea528), // Pisa × Livorno
    (17, 1, 2002, 0x9ea504, 0x9ea4dc), // Treviso × Padova
    (17, 1, 2002, 0x9ea4cc, 0x9ea4d4), // Monza × Spal
    (17, 1, 2002, 0x9ea50c, 0x9ea4f0), // Lumezzane × Triestina
    (3, 2, 2002, 0x9ea538, 0x9ea520), // Arezzo × Cesena
    (3, 2, 2002, 0x9ea540, 0x9ea50c), // Albinoleffe × Lumezzane
    (3, 2, 2002, 0x9ea4dc, 0x9ea4cc), // Padova × Monza
    (3, 2, 2002, 0x9ea530, 0x9ea4fc), // Alzano × Pisa
    (3, 2, 2002, 0x9ea4b8, 0x9ea4e4), // Lucchese × Reggiana
    (3, 2, 2002, 0x9ea518, 0x9ea4c4), // Lecco × Spezia
    (3, 2, 2002, 0x9ea54c, 0x9ea504), // Carrarese × Treviso
    (3, 2, 2002, 0x9ea528, 0x9ea4f0), // Livorno × Triestina
    (3, 2, 2002, 0x9ea4d4, 0x9ea4b0), // Spal × Varese
    (10, 2, 2002, 0x9ea504, 0x9ea540), // Treviso × Albinoleffe
    (10, 2, 2002, 0x9ea520, 0x9ea530), // Cesena × Alzano
    (10, 2, 2002, 0x9ea50c, 0x9ea538), // Lumezzane × Arezzo
    (10, 2, 2002, 0x9ea4cc, 0x9ea54c), // Monza × Carrarese
    (10, 2, 2002, 0x9ea4e4, 0x9ea518), // Reggiana × Lecco
    (10, 2, 2002, 0x9ea4dc, 0x9ea528), // Padova × Livorno
    (10, 2, 2002, 0x9ea4f0, 0x9ea4b8), // Triestina × Lucchese
    (10, 2, 2002, 0x9ea4c4, 0x9ea4d4), // Spezia × Spal
    (10, 2, 2002, 0x9ea4fc, 0x9ea4b0), // Pisa × Varese
    (17, 2, 2002, 0x9ea4b0, 0x9ea520), // Varese × Cesena
    (17, 2, 2002, 0x9ea4b8, 0x9ea528), // Lucchese × Livorno
    (17, 2, 2002, 0x9ea4fc, 0x9ea50c), // Pisa × Lumezzane
    (17, 2, 2002, 0x9ea540, 0x9ea4cc), // Albinoleffe × Monza
    (17, 2, 2002, 0x9ea518, 0x9ea4dc), // Lecco × Padova
    (17, 2, 2002, 0x9ea4d4, 0x9ea4e4), // Spal × Reggiana
    (17, 2, 2002, 0x9ea538, 0x9ea4c4), // Arezzo × Spezia
    (17, 2, 2002, 0x9ea530, 0x9ea504), // Alzano × Treviso
    (17, 2, 2002, 0x9ea54c, 0x9ea4f0), // Carrarese × Triestina
    (24, 2, 2002, 0x9ea528, 0x9ea540), // Livorno × Albinoleffe
    (24, 2, 2002, 0x9ea50c, 0x9ea530), // Lumezzane × Alzano
    (24, 2, 2002, 0x9ea504, 0x9ea538), // Treviso × Arezzo
    (24, 2, 2002, 0x9ea54c, 0x9ea518), // Carrarese × Lecco
    (24, 2, 2002, 0x9ea4dc, 0x9ea4b8), // Padova × Lucchese
    (24, 2, 2002, 0x9ea520, 0x9ea4fc), // Cesena × Pisa
    (24, 2, 2002, 0x9ea4f0, 0x9ea4d4), // Triestina × Spal
    (24, 2, 2002, 0x9ea4e4, 0x9ea4c4), // Reggiana × Spezia
    (24, 2, 2002, 0x9ea4cc, 0x9ea4b0), // Monza × Varese
    (30, 2, 2002, 0x9ea4d4, 0x9ea540), // Spal × Albinoleffe
    (30, 2, 2002, 0x9ea4fc, 0x9ea54c), // Pisa × Carrarese
    (30, 2, 2002, 0x9ea518, 0x9ea528), // Lecco × Livorno
    (30, 2, 2002, 0x9ea538, 0x9ea4b8), // Arezzo × Lucchese
    (30, 2, 2002, 0x9ea4c4, 0x9ea50c), // Spezia × Lumezzane
    (30, 2, 2002, 0x9ea530, 0x9ea4dc), // Alzano × Padova
    (30, 2, 2002, 0x9ea4b0, 0x9ea4e4), // Varese × Reggiana
    (30, 2, 2002, 0x9ea4cc, 0x9ea504), // Monza × Treviso
    (30, 2, 2002, 0x9ea520, 0x9ea4f0), // Cesena × Triestina
    (7, 3, 2002, 0x9ea540, 0x9ea530), // Albinoleffe × Alzano
    (7, 3, 2002, 0x9ea4e4, 0x9ea538), // Reggiana × Arezzo
    (7, 3, 2002, 0x9ea4dc, 0x9ea520), // Padova × Cesena
    (7, 3, 2002, 0x9ea54c, 0x9ea50c), // Carrarese × Lumezzane
    (7, 3, 2002, 0x9ea4b8, 0x9ea4cc), // Lucchese × Monza
    (7, 3, 2002, 0x9ea4d4, 0x9ea4fc), // Spal × Pisa
    (7, 3, 2002, 0x9ea528, 0x9ea4c4), // Livorno × Spezia
    (7, 3, 2002, 0x9ea4f0, 0x9ea504), // Triestina × Treviso
    (7, 3, 2002, 0x9ea518, 0x9ea4b0), // Lecco × Varese
    (14, 3, 2002, 0x9ea4b0, 0x9ea540), // Varese × Albinoleffe
    (14, 3, 2002, 0x9ea530, 0x9ea54c), // Alzano × Carrarese
    (14, 3, 2002, 0x9ea538, 0x9ea518), // Arezzo × Lecco
    (14, 3, 2002, 0x9ea4cc, 0x9ea528), // Monza × Livorno
    (14, 3, 2002, 0x9ea504, 0x9ea4b8), // Treviso × Lucchese
    (14, 3, 2002, 0x9ea4fc, 0x9ea4dc), // Pisa × Padova
    (14, 3, 2002, 0x9ea50c, 0x9ea4e4), // Lumezzane × Reggiana
    (14, 3, 2002, 0x9ea520, 0x9ea4d4), // Cesena × Spal
    (14, 3, 2002, 0x9ea4c4, 0x9ea4f0), // Spezia × Triestina
    (21, 3, 2002, 0x9ea4d4, 0x9ea530), // Spal × Alzano
    (21, 3, 2002, 0x9ea540, 0x9ea538), // Albinoleffe × Arezzo
    (21, 3, 2002, 0x9ea54c, 0x9ea520), // Carrarese × Cesena
    (21, 3, 2002, 0x9ea528, 0x9ea50c), // Livorno × Lumezzane
    (21, 3, 2002, 0x9ea518, 0x9ea4cc), // Lecco × Monza
    (21, 3, 2002, 0x9ea4f0, 0x9ea4fc), // Triestina × Pisa
    (21, 3, 2002, 0x9ea4b8, 0x9ea4c4), // Lucchese × Spezia
    (21, 3, 2002, 0x9ea4e4, 0x9ea504), // Reggiana × Treviso
    (21, 3, 2002, 0x9ea4dc, 0x9ea4b0), // Padova × Varese
    (28, 3, 2002, 0x9ea520, 0x9ea540), // Cesena × Albinoleffe
    (28, 3, 2002, 0x9ea538, 0x9ea54c), // Arezzo × Carrarese
    (28, 3, 2002, 0x9ea4fc, 0x9ea518), // Pisa × Lecco
    (28, 3, 2002, 0x9ea504, 0x9ea528), // Treviso × Livorno
    (28, 3, 2002, 0x9ea530, 0x9ea4b8), // Alzano × Lucchese
    (28, 3, 2002, 0x9ea4c4, 0x9ea4dc), // Spezia × Padova
    (28, 3, 2002, 0x9ea4cc, 0x9ea4e4), // Monza × Reggiana
    (28, 3, 2002, 0x9ea50c, 0x9ea4d4), // Lumezzane × Spal
    (28, 3, 2002, 0x9ea4b0, 0x9ea4f0), // Varese × Triestina
    (5, 4, 2002, 0x9ea528, 0x9ea530), // Livorno × Alzano
    (5, 4, 2002, 0x9ea4d4, 0x9ea538), // Spal × Arezzo
    (5, 4, 2002, 0x9ea4b8, 0x9ea520), // Lucchese × Cesena
    (5, 4, 2002, 0x9ea4dc, 0x9ea50c), // Padova × Lumezzane
    (5, 4, 2002, 0x9ea4f0, 0x9ea4cc), // Triestina × Monza
    (5, 4, 2002, 0x9ea4e4, 0x9ea4fc), // Reggiana × Pisa
    (5, 4, 2002, 0x9ea540, 0x9ea4c4), // Albinoleffe × Spezia
    (5, 4, 2002, 0x9ea518, 0x9ea504), // Lecco × Treviso
    (5, 4, 2002, 0x9ea54c, 0x9ea4b0), // Carrarese × Varese
];

const CQN6_C1B: [(u8, u8, u16, u32, u32); 306] = [
    (2, 8, 2001, 0x9ea638, 0x9ea640), // Ascoli × Castel di Sangro
    (2, 8, 2001, 0x9ea620, 0x9ea62c), // Avellino × L'Aquila
    (2, 8, 2001, 0x9ea60c, 0x9ea614), // Catania × Benevento
    (2, 8, 2001, 0x9ea5f8, 0x9ea604), // Giulianova × Chieti
    (2, 8, 2001, 0x9ea5e0, 0x9ea5ec), // Lodigiani × Lanciano
    (2, 8, 2001, 0x9ea5d0, 0x9ea5d8), // Pescara × Fermana
    (2, 8, 2001, 0x9ea5bc, 0x9ea5c4), // Sora × Nocerina
    (2, 8, 2001, 0x9ea5a8, 0x9ea5b0), // Taranto × Vis Pesaro
    (2, 8, 2001, 0x9ea594, 0x9ea5a0), // Viterbese × Torres
    (9, 8, 2001, 0x9ea614, 0x9ea594), // Benevento × Viterbese
    (9, 8, 2001, 0x9ea640, 0x9ea5f8), // Castel di Sangro × Giulianova
    (9, 8, 2001, 0x9ea604, 0x9ea5d0), // Chieti × Pescara
    (9, 8, 2001, 0x9ea5d8, 0x9ea5e0), // Fermana × Lodigiani
    (9, 8, 2001, 0x9ea62c, 0x9ea5bc), // L'Aquila × Sora
    (9, 8, 2001, 0x9ea5ec, 0x9ea5a8), // Lanciano × Taranto
    (9, 8, 2001, 0x9ea5c4, 0x9ea620), // Nocerina × Avellino
    (9, 8, 2001, 0x9ea5a0, 0x9ea60c), // Torres × Catania
    (9, 8, 2001, 0x9ea5b0, 0x9ea638), // Vis Pesaro × Ascoli
    (16, 8, 2001, 0x9ea638, 0x9ea604), // Ascoli × Chieti
    (16, 8, 2001, 0x9ea620, 0x9ea5a0), // Avellino × Torres
    (16, 8, 2001, 0x9ea60c, 0x9ea5ec), // Catania × Lanciano
    (16, 8, 2001, 0x9ea5f8, 0x9ea594), // Giulianova × Viterbese
    (16, 8, 2001, 0x9ea5e0, 0x9ea614), // Lodigiani × Benevento
    (16, 8, 2001, 0x9ea5d0, 0x9ea62c), // Pescara × L'Aquila
    (16, 8, 2001, 0x9ea5bc, 0x9ea5d8), // Sora × Fermana
    (16, 8, 2001, 0x9ea5a8, 0x9ea640), // Taranto × Castel di Sangro
    (16, 8, 2001, 0x9ea5b0, 0x9ea5c4), // Vis Pesaro × Nocerina
    (23, 8, 2001, 0x9ea614, 0x9ea5d0), // Benevento × Pescara
    (23, 8, 2001, 0x9ea640, 0x9ea620), // Castel di Sangro × Avellino
    (23, 8, 2001, 0x9ea604, 0x9ea5a8), // Chieti × Taranto
    (23, 8, 2001, 0x9ea5d8, 0x9ea60c), // Fermana × Catania
    (23, 8, 2001, 0x9ea62c, 0x9ea5f8), // L'Aquila × Giulianova
    (23, 8, 2001, 0x9ea5ec, 0x9ea638), // Lanciano × Ascoli
    (23, 8, 2001, 0x9ea5c4, 0x9ea5e0), // Nocerina × Lodigiani
    (23, 8, 2001, 0x9ea5a0, 0x9ea5b0), // Torres × Vis Pesaro
    (23, 8, 2001, 0x9ea594, 0x9ea5bc), // Viterbese × Sora
    (30, 8, 2001, 0x9ea638, 0x9ea5a0), // Ascoli × Torres
    (30, 8, 2001, 0x9ea620, 0x9ea5ec), // Avellino × Lanciano
    (30, 8, 2001, 0x9ea640, 0x9ea604), // Castel di Sangro × Chieti
    (30, 8, 2001, 0x9ea5f8, 0x9ea5c4), // Giulianova × Nocerina
    (30, 8, 2001, 0x9ea5e0, 0x9ea594), // Lodigiani × Viterbese
    (30, 8, 2001, 0x9ea5d0, 0x9ea60c), // Pescara × Catania
    (30, 8, 2001, 0x9ea5bc, 0x9ea614), // Sora × Benevento
    (30, 8, 2001, 0x9ea5a8, 0x9ea5d8), // Taranto × Fermana
    (30, 8, 2001, 0x9ea5b0, 0x9ea62c), // Vis Pesaro × L'Aquila
    (7, 9, 2001, 0x9ea620, 0x9ea5bc), // Avellino × Sora
    (7, 9, 2001, 0x9ea614, 0x9ea5f8), // Benevento × Giulianova
    (7, 9, 2001, 0x9ea60c, 0x9ea5e0), // Catania × Lodigiani
    (7, 9, 2001, 0x9ea604, 0x9ea5c4), // Chieti × Nocerina
    (7, 9, 2001, 0x9ea5d8, 0x9ea5b0), // Fermana × Vis Pesaro
    (7, 9, 2001, 0x9ea62c, 0x9ea638), // L'Aquila × Ascoli
    (7, 9, 2001, 0x9ea5ec, 0x9ea640), // Lanciano × Castel di Sangro
    (7, 9, 2001, 0x9ea5a0, 0x9ea5a8), // Torres × Taranto
    (7, 9, 2001, 0x9ea594, 0x9ea5d0), // Viterbese × Pescara
    (14, 9, 2001, 0x9ea638, 0x9ea614), // Ascoli × Benevento
    (14, 9, 2001, 0x9ea640, 0x9ea5a0), // Castel di Sangro × Torres
    (14, 9, 2001, 0x9ea5f8, 0x9ea5d8), // Giulianova × Fermana
    (14, 9, 2001, 0x9ea5ec, 0x9ea604), // Lanciano × Chieti
    (14, 9, 2001, 0x9ea5e0, 0x9ea5d0), // Lodigiani × Pescara
    (14, 9, 2001, 0x9ea5c4, 0x9ea594), // Nocerina × Viterbese
    (14, 9, 2001, 0x9ea5bc, 0x9ea60c), // Sora × Catania
    (14, 9, 2001, 0x9ea5a8, 0x9ea62c), // Taranto × L'Aquila
    (14, 9, 2001, 0x9ea5b0, 0x9ea620), // Vis Pesaro × Avellino
    (21, 9, 2001, 0x9ea620, 0x9ea604), // Avellino × Chieti
    (21, 9, 2001, 0x9ea614, 0x9ea5c4), // Benevento × Nocerina
    (21, 9, 2001, 0x9ea60c, 0x9ea5b0), // Catania × Vis Pesaro
    (21, 9, 2001, 0x9ea5d8, 0x9ea640), // Fermana × Castel di Sangro
    (21, 9, 2001, 0x9ea62c, 0x9ea5e0), // L'Aquila × Lodigiani
    (21, 9, 2001, 0x9ea5d0, 0x9ea5f8), // Pescara × Giulianova
    (21, 9, 2001, 0x9ea5a0, 0x9ea5ec), // Torres × Lanciano
    (21, 9, 2001, 0x9ea5bc, 0x9ea638), // Sora × Ascoli
    (21, 9, 2001, 0x9ea594, 0x9ea5a8), // Viterbese × Taranto
    (28, 9, 2001, 0x9ea638, 0x9ea620), // Ascoli × Avellino
    (28, 9, 2001, 0x9ea640, 0x9ea5e0), // Castel di Sangro × Lodigiani
    (28, 9, 2001, 0x9ea604, 0x9ea5b0), // Chieti × Vis Pesaro
    (28, 9, 2001, 0x9ea5f8, 0x9ea5bc), // Giulianova × Sora
    (28, 9, 2001, 0x9ea5ec, 0x9ea62c), // Lanciano × L'Aquila
    (28, 9, 2001, 0x9ea5c4, 0x9ea5d0), // Nocerina × Pescara
    (28, 9, 2001, 0x9ea5a0, 0x9ea614), // Torres × Benevento
    (28, 9, 2001, 0x9ea5a8, 0x9ea60c), // Taranto × Catania
    (28, 9, 2001, 0x9ea594, 0x9ea5d8), // Viterbese × Fermana
    (4, 10, 2001, 0x9ea620, 0x9ea5a8), // Avellino × Taranto
    (4, 10, 2001, 0x9ea614, 0x9ea640), // Benevento × Castel di Sangro
    (4, 10, 2001, 0x9ea60c, 0x9ea594), // Catania × Viterbese
    (4, 10, 2001, 0x9ea5d8, 0x9ea5c4), // Fermana × Nocerina
    (4, 10, 2001, 0x9ea62c, 0x9ea604), // L'Aquila × Chieti
    (4, 10, 2001, 0x9ea5e0, 0x9ea5f8), // Lodigiani × Giulianova
    (4, 10, 2001, 0x9ea5d0, 0x9ea638), // Pescara × Ascoli
    (4, 10, 2001, 0x9ea5bc, 0x9ea5a0), // Sora × Torres
    (4, 10, 2001, 0x9ea5b0, 0x9ea5ec), // Vis Pesaro × Lanciano
    (11, 10, 2001, 0x9ea638, 0x9ea5a8), // Ascoli × Taranto
    (11, 10, 2001, 0x9ea614, 0x9ea5d8), // Benevento × Fermana
    (11, 10, 2001, 0x9ea640, 0x9ea5b0), // Castel di Sangro × Vis Pesaro
    (11, 10, 2001, 0x9ea604, 0x9ea5e0), // Chieti × Lodigiani
    (11, 10, 2001, 0x9ea5f8, 0x9ea60c), // Giulianova × Catania
    (11, 10, 2001, 0x9ea5c4, 0x9ea5ec), // Nocerina × Lanciano
    (11, 10, 2001, 0x9ea5d0, 0x9ea5bc), // Pescara × Sora
    (11, 10, 2001, 0x9ea5a0, 0x9ea62c), // Torres × L'Aquila
    (11, 10, 2001, 0x9ea594, 0x9ea620), // Viterbese × Avellino
    (18, 10, 2001, 0x9ea620, 0x9ea614), // Avellino × Benevento
    (18, 10, 2001, 0x9ea60c, 0x9ea5c4), // Catania × Nocerina
    (18, 10, 2001, 0x9ea604, 0x9ea5a0), // Chieti × Torres
    (18, 10, 2001, 0x9ea5d8, 0x9ea638), // Fermana × Ascoli
    (18, 10, 2001, 0x9ea62c, 0x9ea640), // L'Aquila × Castel di Sangro
    (18, 10, 2001, 0x9ea5ec, 0x9ea5d0), // Lanciano × Pescara
    (18, 10, 2001, 0x9ea5e0, 0x9ea5bc), // Lodigiani × Sora
    (18, 10, 2001, 0x9ea5a8, 0x9ea5f8), // Taranto × Giulianova
    (18, 10, 2001, 0x9ea5b0, 0x9ea594), // Vis Pesaro × Viterbese
    (25, 10, 2001, 0x9ea614, 0x9ea5a8), // Benevento × Taranto
    (25, 10, 2001, 0x9ea60c, 0x9ea638), // Catania × Ascoli
    (25, 10, 2001, 0x9ea5d8, 0x9ea604), // Fermana × Chieti
    (25, 10, 2001, 0x9ea5f8, 0x9ea5a0), // Giulianova × Torres
    (25, 10, 2001, 0x9ea5e0, 0x9ea5b0), // Lodigiani × Vis Pesaro
    (25, 10, 2001, 0x9ea5c4, 0x9ea62c), // Nocerina × L'Aquila
    (25, 10, 2001, 0x9ea5d0, 0x9ea620), // Pescara × Avellino
    (25, 10, 2001, 0x9ea5bc, 0x9ea5ec), // Sora × Lanciano
    (25, 10, 2001, 0x9ea594, 0x9ea640), // Viterbese × Castel di Sangro
    (2, 11, 2001, 0x9ea638, 0x9ea5c4), // Ascoli × Nocerina
    (2, 11, 2001, 0x9ea620, 0x9ea5d8), // Avellino × Fermana
    (2, 11, 2001, 0x9ea640, 0x9ea5bc), // Castel di Sangro × Sora
    (2, 11, 2001, 0x9ea604, 0x9ea60c), // Chieti × Catania
    (2, 11, 2001, 0x9ea62c, 0x9ea594), // L'Aquila × Viterbese
    (2, 11, 2001, 0x9ea5ec, 0x9ea614), // Lanciano × Benevento
    (2, 11, 2001, 0x9ea5a0, 0x9ea5e0), // Torres × Lodigiani
    (2, 11, 2001, 0x9ea5a8, 0x9ea5d0), // Taranto × Pescara
    (2, 11, 2001, 0x9ea5b0, 0x9ea5f8), // Vis Pesaro × Giulianova
    (9, 11, 2001, 0x9ea614, 0x9ea5b0), // Benevento × Vis Pesaro
    (9, 11, 2001, 0x9ea60c, 0x9ea62c), // Catania × L'Aquila
    (9, 11, 2001, 0x9ea5d8, 0x9ea5a0), // Fermana × Torres
    (9, 11, 2001, 0x9ea5f8, 0x9ea620), // Giulianova × Avellino
    (9, 11, 2001, 0x9ea5e0, 0x9ea638), // Lodigiani × Ascoli
    (9, 11, 2001, 0x9ea5c4, 0x9ea5a8), // Nocerina × Taranto
    (9, 11, 2001, 0x9ea5d0, 0x9ea640), // Pescara × Castel di Sangro
    (9, 11, 2001, 0x9ea5bc, 0x9ea604), // Sora × Chieti
    (9, 11, 2001, 0x9ea594, 0x9ea5ec), // Viterbese × Lanciano
    (16, 11, 2001, 0x9ea638, 0x9ea5f8), // Ascoli × Giulianova
    (16, 11, 2001, 0x9ea620, 0x9ea5e0), // Avellino × Lodigiani
    (16, 11, 2001, 0x9ea640, 0x9ea60c), // Castel di Sangro × Catania
    (16, 11, 2001, 0x9ea604, 0x9ea594), // Chieti × Viterbese
    (16, 11, 2001, 0x9ea62c, 0x9ea614), // L'Aquila × Benevento
    (16, 11, 2001, 0x9ea5ec, 0x9ea5d8), // Lanciano × Fermana
    (16, 11, 2001, 0x9ea5a0, 0x9ea5c4), // Torres × Nocerina
    (16, 11, 2001, 0x9ea5a8, 0x9ea5bc), // Taranto × Sora
    (16, 11, 2001, 0x9ea5b0, 0x9ea5d0), // Vis Pesaro × Pescara
    (23, 11, 2001, 0x9ea614, 0x9ea604), // Benevento × Chieti
    (23, 11, 2001, 0x9ea60c, 0x9ea620), // Catania × Avellino
    (23, 11, 2001, 0x9ea5d8, 0x9ea62c), // Fermana × L'Aquila
    (23, 11, 2001, 0x9ea5f8, 0x9ea5ec), // Giulianova × Lanciano
    (23, 11, 2001, 0x9ea5e0, 0x9ea5a8), // Lodigiani × Taranto
    (23, 11, 2001, 0x9ea5c4, 0x9ea640), // Nocerina × Castel di Sangro
    (23, 11, 2001, 0x9ea5d0, 0x9ea5a0), // Pescara × Torres
    (23, 11, 2001, 0x9ea5bc, 0x9ea5b0), // Sora × Vis Pesaro
    (23, 11, 2001, 0x9ea594, 0x9ea638), // Viterbese × Ascoli
    (6, 0, 2002, 0x9ea640, 0x9ea638), // Castel di Sangro × Ascoli
    (6, 0, 2002, 0x9ea62c, 0x9ea620), // L'Aquila × Avellino
    (6, 0, 2002, 0x9ea614, 0x9ea60c), // Benevento × Catania
    (6, 0, 2002, 0x9ea604, 0x9ea5f8), // Chieti × Giulianova
    (6, 0, 2002, 0x9ea5ec, 0x9ea5e0), // Lanciano × Lodigiani
    (6, 0, 2002, 0x9ea5d8, 0x9ea5d0), // Fermana × Pescara
    (6, 0, 2002, 0x9ea5c4, 0x9ea5bc), // Nocerina × Sora
    (6, 0, 2002, 0x9ea5b0, 0x9ea5a8), // Vis Pesaro × Taranto
    (6, 0, 2002, 0x9ea5a0, 0x9ea594), // Torres × Viterbese
    (13, 0, 2002, 0x9ea594, 0x9ea614), // Viterbese × Benevento
    (13, 0, 2002, 0x9ea5f8, 0x9ea640), // Giulianova × Castel di Sangro
    (13, 0, 2002, 0x9ea5d0, 0x9ea604), // Pescara × Chieti
    (13, 0, 2002, 0x9ea5e0, 0x9ea5d8), // Lodigiani × Fermana
    (13, 0, 2002, 0x9ea5bc, 0x9ea62c), // Sora × L'Aquila
    (13, 0, 2002, 0x9ea5a8, 0x9ea5ec), // Taranto × Lanciano
    (13, 0, 2002, 0x9ea620, 0x9ea5c4), // Avellino × Nocerina
    (13, 0, 2002, 0x9ea60c, 0x9ea5a0), // Catania × Torres
    (13, 0, 2002, 0x9ea638, 0x9ea5b0), // Ascoli × Vis Pesaro
    (20, 0, 2002, 0x9ea604, 0x9ea638), // Chieti × Ascoli
    (20, 0, 2002, 0x9ea5a0, 0x9ea620), // Torres × Avellino
    (20, 0, 2002, 0x9ea5ec, 0x9ea60c), // Lanciano × Catania
    (20, 0, 2002, 0x9ea594, 0x9ea5f8), // Viterbese × Giulianova
    (20, 0, 2002, 0x9ea614, 0x9ea5e0), // Benevento × Lodigiani
    (20, 0, 2002, 0x9ea62c, 0x9ea5d0), // L'Aquila × Pescara
    (20, 0, 2002, 0x9ea5d8, 0x9ea5bc), // Fermana × Sora
    (20, 0, 2002, 0x9ea640, 0x9ea5a8), // Castel di Sangro × Taranto
    (20, 0, 2002, 0x9ea5c4, 0x9ea5b0), // Nocerina × Vis Pesaro
    (27, 0, 2002, 0x9ea5d0, 0x9ea614), // Pescara × Benevento
    (27, 0, 2002, 0x9ea620, 0x9ea640), // Avellino × Castel di Sangro
    (27, 0, 2002, 0x9ea5a8, 0x9ea604), // Taranto × Chieti
    (27, 0, 2002, 0x9ea60c, 0x9ea5d8), // Catania × Fermana
    (27, 0, 2002, 0x9ea5f8, 0x9ea62c), // Giulianova × L'Aquila
    (27, 0, 2002, 0x9ea638, 0x9ea5ec), // Ascoli × Lanciano
    (27, 0, 2002, 0x9ea5e0, 0x9ea5c4), // Lodigiani × Nocerina
    (27, 0, 2002, 0x9ea5b0, 0x9ea5a0), // Vis Pesaro × Torres
    (27, 0, 2002, 0x9ea5bc, 0x9ea594), // Sora × Viterbese
    (3, 1, 2002, 0x9ea5a0, 0x9ea638), // Torres × Ascoli
    (3, 1, 2002, 0x9ea5ec, 0x9ea620), // Lanciano × Avellino
    (3, 1, 2002, 0x9ea604, 0x9ea640), // Chieti × Castel di Sangro
    (3, 1, 2002, 0x9ea5c4, 0x9ea5f8), // Nocerina × Giulianova
    (3, 1, 2002, 0x9ea594, 0x9ea5e0), // Viterbese × Lodigiani
    (3, 1, 2002, 0x9ea60c, 0x9ea5d0), // Catania × Pescara
    (3, 1, 2002, 0x9ea614, 0x9ea5bc), // Benevento × Sora
    (3, 1, 2002, 0x9ea5d8, 0x9ea5a8), // Fermana × Taranto
    (3, 1, 2002, 0x9ea62c, 0x9ea5b0), // L'Aquila × Vis Pesaro
    (10, 1, 2002, 0x9ea5bc, 0x9ea620), // Sora × Avellino
    (10, 1, 2002, 0x9ea5f8, 0x9ea614), // Giulianova × Benevento
    (10, 1, 2002, 0x9ea5e0, 0x9ea60c), // Lodigiani × Catania
    (10, 1, 2002, 0x9ea5c4, 0x9ea604), // Nocerina × Chieti
    (10, 1, 2002, 0x9ea5b0, 0x9ea5d8), // Vis Pesaro × Fermana
    (10, 1, 2002, 0x9ea638, 0x9ea62c), // Ascoli × L'Aquila
    (10, 1, 2002, 0x9ea640, 0x9ea5ec), // Castel di Sangro × Lanciano
    (10, 1, 2002, 0x9ea5a8, 0x9ea5a0), // Taranto × Torres
    (10, 1, 2002, 0x9ea5d0, 0x9ea594), // Pescara × Viterbese
    (17, 1, 2002, 0x9ea614, 0x9ea638), // Benevento × Ascoli
    (17, 1, 2002, 0x9ea5a0, 0x9ea640), // Torres × Castel di Sangro
    (17, 1, 2002, 0x9ea5d8, 0x9ea5f8), // Fermana × Giulianova
    (17, 1, 2002, 0x9ea604, 0x9ea5ec), // Chieti × Lanciano
    (17, 1, 2002, 0x9ea5d0, 0x9ea5e0), // Pescara × Lodigiani
    (17, 1, 2002, 0x9ea594, 0x9ea5c4), // Viterbese × Nocerina
    (17, 1, 2002, 0x9ea60c, 0x9ea5bc), // Catania × Sora
    (17, 1, 2002, 0x9ea62c, 0x9ea5a8), // L'Aquila × Taranto
    (17, 1, 2002, 0x9ea620, 0x9ea5b0), // Avellino × Vis Pesaro
    (3, 2, 2002, 0x9ea604, 0x9ea620), // Chieti × Avellino
    (3, 2, 2002, 0x9ea5c4, 0x9ea614), // Nocerina × Benevento
    (3, 2, 2002, 0x9ea5b0, 0x9ea60c), // Vis Pesaro × Catania
    (3, 2, 2002, 0x9ea640, 0x9ea5d8), // Castel di Sangro × Fermana
    (3, 2, 2002, 0x9ea5e0, 0x9ea62c), // Lodigiani × L'Aquila
    (3, 2, 2002, 0x9ea5f8, 0x9ea5d0), // Giulianova × Pescara
    (3, 2, 2002, 0x9ea5ec, 0x9ea5a0), // Lanciano × Torres
    (3, 2, 2002, 0x9ea638, 0x9ea5bc), // Ascoli × Sora
    (3, 2, 2002, 0x9ea5a8, 0x9ea594), // Taranto × Viterbese
    (10, 2, 2002, 0x9ea620, 0x9ea638), // Avellino × Ascoli
    (10, 2, 2002, 0x9ea5e0, 0x9ea640), // Lodigiani × Castel di Sangro
    (10, 2, 2002, 0x9ea5b0, 0x9ea604), // Vis Pesaro × Chieti
    (10, 2, 2002, 0x9ea5bc, 0x9ea5f8), // Sora × Giulianova
    (10, 2, 2002, 0x9ea62c, 0x9ea5ec), // L'Aquila × Lanciano
    (10, 2, 2002, 0x9ea5d0, 0x9ea5c4), // Pescara × Nocerina
    (10, 2, 2002, 0x9ea614, 0x9ea5a0), // Benevento × Torres
    (10, 2, 2002, 0x9ea60c, 0x9ea5a8), // Catania × Taranto
    (10, 2, 2002, 0x9ea5d8, 0x9ea594), // Fermana × Viterbese
    (17, 2, 2002, 0x9ea5a8, 0x9ea620), // Taranto × Avellino
    (17, 2, 2002, 0x9ea640, 0x9ea614), // Castel di Sangro × Benevento
    (17, 2, 2002, 0x9ea594, 0x9ea60c), // Viterbese × Catania
    (17, 2, 2002, 0x9ea5c4, 0x9ea5d8), // Nocerina × Fermana
    (17, 2, 2002, 0x9ea604, 0x9ea62c), // Chieti × L'Aquila
    (17, 2, 2002, 0x9ea5f8, 0x9ea5e0), // Giulianova × Lodigiani
    (17, 2, 2002, 0x9ea638, 0x9ea5d0), // Ascoli × Pescara
    (17, 2, 2002, 0x9ea5a0, 0x9ea5bc), // Torres × Sora
    (17, 2, 2002, 0x9ea5ec, 0x9ea5b0), // Lanciano × Vis Pesaro
    (24, 2, 2002, 0x9ea5a8, 0x9ea638), // Taranto × Ascoli
    (24, 2, 2002, 0x9ea5d8, 0x9ea614), // Fermana × Benevento
    (24, 2, 2002, 0x9ea5b0, 0x9ea640), // Vis Pesaro × Castel di Sangro
    (24, 2, 2002, 0x9ea5e0, 0x9ea604), // Lodigiani × Chieti
    (24, 2, 2002, 0x9ea60c, 0x9ea5f8), // Catania × Giulianova
    (24, 2, 2002, 0x9ea5ec, 0x9ea5c4), // Lanciano × Nocerina
    (24, 2, 2002, 0x9ea5bc, 0x9ea5d0), // Sora × Pescara
    (24, 2, 2002, 0x9ea62c, 0x9ea5a0), // L'Aquila × Torres
    (24, 2, 2002, 0x9ea620, 0x9ea594), // Avellino × Viterbese
    (30, 2, 2002, 0x9ea614, 0x9ea620), // Benevento × Avellino
    (30, 2, 2002, 0x9ea5c4, 0x9ea60c), // Nocerina × Catania
    (30, 2, 2002, 0x9ea5a0, 0x9ea604), // Torres × Chieti
    (30, 2, 2002, 0x9ea638, 0x9ea5d8), // Ascoli × Fermana
    (30, 2, 2002, 0x9ea640, 0x9ea62c), // Castel di Sangro × L'Aquila
    (30, 2, 2002, 0x9ea5d0, 0x9ea5ec), // Pescara × Lanciano
    (30, 2, 2002, 0x9ea5bc, 0x9ea5e0), // Sora × Lodigiani
    (30, 2, 2002, 0x9ea5f8, 0x9ea5a8), // Giulianova × Taranto
    (30, 2, 2002, 0x9ea594, 0x9ea5b0), // Viterbese × Vis Pesaro
    (7, 3, 2002, 0x9ea5a8, 0x9ea614), // Taranto × Benevento
    (7, 3, 2002, 0x9ea638, 0x9ea60c), // Ascoli × Catania
    (7, 3, 2002, 0x9ea604, 0x9ea5d8), // Chieti × Fermana
    (7, 3, 2002, 0x9ea5a0, 0x9ea5f8), // Torres × Giulianova
    (7, 3, 2002, 0x9ea5b0, 0x9ea5e0), // Vis Pesaro × Lodigiani
    (7, 3, 2002, 0x9ea62c, 0x9ea5c4), // L'Aquila × Nocerina
    (7, 3, 2002, 0x9ea620, 0x9ea5d0), // Avellino × Pescara
    (7, 3, 2002, 0x9ea5ec, 0x9ea5bc), // Lanciano × Sora
    (7, 3, 2002, 0x9ea640, 0x9ea594), // Castel di Sangro × Viterbese
    (14, 3, 2002, 0x9ea5c4, 0x9ea638), // Nocerina × Ascoli
    (14, 3, 2002, 0x9ea5d8, 0x9ea620), // Fermana × Avellino
    (14, 3, 2002, 0x9ea5bc, 0x9ea640), // Sora × Castel di Sangro
    (14, 3, 2002, 0x9ea60c, 0x9ea604), // Catania × Chieti
    (14, 3, 2002, 0x9ea594, 0x9ea62c), // Viterbese × L'Aquila
    (14, 3, 2002, 0x9ea614, 0x9ea5ec), // Benevento × Lanciano
    (14, 3, 2002, 0x9ea5e0, 0x9ea5a0), // Lodigiani × Torres
    (14, 3, 2002, 0x9ea5d0, 0x9ea5a8), // Pescara × Taranto
    (14, 3, 2002, 0x9ea5f8, 0x9ea5b0), // Giulianova × Vis Pesaro
    (21, 3, 2002, 0x9ea5b0, 0x9ea614), // Vis Pesaro × Benevento
    (21, 3, 2002, 0x9ea62c, 0x9ea60c), // L'Aquila × Catania
    (21, 3, 2002, 0x9ea5a0, 0x9ea5d8), // Torres × Fermana
    (21, 3, 2002, 0x9ea620, 0x9ea5f8), // Avellino × Giulianova
    (21, 3, 2002, 0x9ea638, 0x9ea5e0), // Ascoli × Lodigiani
    (21, 3, 2002, 0x9ea5a8, 0x9ea5c4), // Taranto × Nocerina
    (21, 3, 2002, 0x9ea640, 0x9ea5d0), // Castel di Sangro × Pescara
    (21, 3, 2002, 0x9ea604, 0x9ea5bc), // Chieti × Sora
    (21, 3, 2002, 0x9ea5ec, 0x9ea594), // Lanciano × Viterbese
    (28, 3, 2002, 0x9ea5f8, 0x9ea638), // Giulianova × Ascoli
    (28, 3, 2002, 0x9ea5e0, 0x9ea620), // Lodigiani × Avellino
    (28, 3, 2002, 0x9ea60c, 0x9ea640), // Catania × Castel di Sangro
    (28, 3, 2002, 0x9ea594, 0x9ea604), // Viterbese × Chieti
    (28, 3, 2002, 0x9ea614, 0x9ea62c), // Benevento × L'Aquila
    (28, 3, 2002, 0x9ea5d8, 0x9ea5ec), // Fermana × Lanciano
    (28, 3, 2002, 0x9ea5c4, 0x9ea5a0), // Nocerina × Torres
    (28, 3, 2002, 0x9ea5bc, 0x9ea5a8), // Sora × Taranto
    (28, 3, 2002, 0x9ea5d0, 0x9ea5b0), // Pescara × Vis Pesaro
    (5, 4, 2002, 0x9ea604, 0x9ea614), // Chieti × Benevento
    (5, 4, 2002, 0x9ea620, 0x9ea60c), // Avellino × Catania
    (5, 4, 2002, 0x9ea62c, 0x9ea5d8), // L'Aquila × Fermana
    (5, 4, 2002, 0x9ea5ec, 0x9ea5f8), // Lanciano × Giulianova
    (5, 4, 2002, 0x9ea5a8, 0x9ea5e0), // Taranto × Lodigiani
    (5, 4, 2002, 0x9ea640, 0x9ea5c4), // Castel di Sangro × Nocerina
    (5, 4, 2002, 0x9ea5a0, 0x9ea5d0), // Torres × Pescara
    (5, 4, 2002, 0x9ea5b0, 0x9ea5bc), // Vis Pesaro × Sora
    (5, 4, 2002, 0x9ea638, 0x9ea594), // Ascoli × Viterbese
];

