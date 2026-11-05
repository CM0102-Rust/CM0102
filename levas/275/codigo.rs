// ---------- 275a leva: a MLS: o destino de cada clube no fim das fases, o resultado nas tabelas das duas conferências e a conferência do Jogo das Estrelas (usa_mls.cpp) ----------

unsafe fn cqk9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqk9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// 0x86bdd0 Staff_Records::add_qualified_for_round_record (thiscall em [0xdd7f00], ret 0x14; já em Rust).
unsafe fn cqk9_classificou(clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
    f(cqk9_d(0xdd7f00), clube, comp, fase, rodada, peso);
}

/// 0x86c1d0 Staff_Records::add_failed_to_qualify_record (thiscall em [0xdd7f00], ret 0x10; já em Rust).
unsafe fn cqk9_nao_classificou(clube: usize, comp: usize, fase: u32, peso: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x86c1d0usize);
    f(cqk9_d(0xdd7f00), clube, comp, fase, peso);
}

/// 0x4bf850 comp_util_scale_by_round (cdecl): o peso de 50 a 70 pela rodada do jogo, em 2 rodadas.
unsafe fn cqk9_escala(jogo: usize) -> u32 {
    let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4bf850usize);
    escala(0x32, 0x46, cqk9_w(jogo + 0x34) as u16 as u32, 2)
}

/// A MLS no fim das fases (0x908200). `fazer` = falso: o ensaio do comparador (só a devolução).
unsafe fn cqk9_destino(this: usize, clube: usize, situacao: u32, fase: u32, jogo: usize, fazer: bool) -> u32 {
    let (st, fa) = (situacao as u8, fase as u8 as i8);
    let comp = cqk9_d(this + 4);
    if fa < 2 {
        // a temporada (-1) e as duas primeiras rodadas dos play-offs
        if st == 6 {
            if fazer {
                crate::comparacao::ramo("cqk_9 mls classificado");
                cqk9_classificou(clube, comp, 0xa0, 0x78, 0x28); // cqk9: o classificado da MLS
            }
            return 0;
        }
        if st != 0xfe {
            return 0;
        }
        if fazer {
            let f = if fa == -1 { 0x413 } else if fa != 0 { 0x414 } else { 0x47b };
            cqk9_nao_classificou(clube, comp, f, 0x28);
        }
        return u32::MAX;
    }
    if fa == 2 {
        // a final
        if st == 1 {
            if fazer {
                cqk9_classificou(clube, comp, 0x417, 0x96, 0x50);
            }
        } else if st == 0 {
            if fazer {
                // ponytail: a original empurra eax/ecx/edx inteiros (lixo acima das palavras: o do chamador, a metade
                // de cima do this); a 0x4bf850 e a 0x86bdd0 só leem as palavras: vão as palavras.
                let peso = cqk9_escala(jogo);
                cqk9_classificou(clube, comp, cqk9_w(jogo + 0x32) as u16 as u32, cqk9_w(jogo + 0x30) as u16 as u32, peso);
            }
        } else if st == 0xff || st == 2 {
            if fazer {
                let peso = cqk9_escala(jogo);
                let elim: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
                elim(cqk9_d(0xdd7f00), clube, comp, cqk9_w(jogo + 0x32) as u16 as u32, cqk9_w(jogo + 0x30) as u16 as u32, peso);
            }
        }
        return 0;
    }
    if st == 1 && fazer {
        let f: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
        f(cqk9_d(0xdd7f00), clube, comp);
    }
    0
}

/// { 0x908200 } (thiscall, ret 0x18, usa_mls.cpp): `UsaMls::update_staff_records_for_status` (slot 18): na temporada e
/// nas fases 0 e 1, quem avança (6) ganha o registro de classificação (fase 0xa0, rodada 0x78) e quem fica de fora (-2) o
/// de não classificado (fase 0x413 na temporada, 0x47b na fase 0, 0x414 na 1) e -1; na fase 2, o vencedor (1) ganha
/// o registro da final (0x417, 0x96, peso 0x50), quem passa (0) o da rodada do jogo e os eliminados (-1, 2) o de
/// eliminação, com peso de 50 a 70 pela rodada; depois, o campeão (1) ganha o registro de campeão. Devolve 0 ou -1.
unsafe extern "thiscall" fn cqk9_destino_na_mls(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let _ = (a4, a6);
    cqk9_destino(this, clube, situacao, fase, jogo, true)
}

/// Comparador: só a original (os registros alocam e vão para o histórico do técnico; não rodam duas vezes) com a
/// devolução prevista pelo ensaio.
unsafe extern "thiscall" fn comparar_cqk9_destino_na_mls(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let n = cqk9_destino(this, clube, situacao, fase, jogo, false);
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1737)(this, clube, situacao, fase, a4, jogo, a6);
    PLACAR.registrar(r == n, || format!("cqk9_destino_na_mls({:x}, {:x}, {}, {}) original={} rust={}", this, clube, situacao as u8 as i8, fase as u8 as i8, r as i32, n as i32));
    r
}

/// { 0x9099d0 } (thiscall, ret 8, usa_mls.cpp): `UsaMls::add_result(jogo, estatisticas)` (slot 2): o jogo vai para a
/// tabela geral (`League::add_result` 0x685d30) com a marca +0x42 do jogo em -1 quando abaixo de 2; no estágio 1 (a
/// temporada regular), vai também para as tabelas das duas conferências (os estágios 0 e 1, pelo slot 2 deles, com a
/// marca 0 e 1 e sem o bloco de estatísticas +8, guardado e devolvido). Devolve 1.
unsafe extern "thiscall" fn cqk9_resultado_na_mls(this: usize, jogo: usize, estat: u32) -> u32 {
    if (*((jogo + 0x42) as *const i8)) < 2 {
        *((jogo + 0x42) as *mut u8) = 0xff;
    }
    let geral: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x685d30usize);
    geral(this, jogo, estat);
    if cqk9_d(this + 0x30) != 1 {
        return 1;
    }
    let guardado = cqk9_d(this + 8);
    *((this + 8) as *mut u32) = 0;
    *((jogo + 0x42) as *mut u8) = 0;
    let estagio = |k: usize| -> (usize, unsafe extern "thiscall" fn(usize, usize, u32) -> u32) {
        let e = cqk9_d(cqk9_d(this + 0xc) + 4 * k);
        (e, core::mem::transmute(cqk9_d(cqk9_d(e) + 4)))
    };
    let (e0, f0) = estagio(0);
    f0(e0, jogo, estat);
    *((jogo + 0x42) as *mut u8) = 1;
    let (e1, f1) = estagio(1);
    crate::comparacao::ramo("cqk_9 mls segunda conferencia");
    f1(e1, jogo, estat); // cqk9: o jogo na tabela da segunda conferência
    *((this + 8) as *mut u32) = guardado as u32;
    1
}

/// Comparador: só a original (ponytail: o resultado entra nas tabelas; não roda duas vezes), com o plano: a devolução
/// 1, a marca +0x42 do jogo (1 no estágio 1; senão -1 se estava abaixo de 2) e o bloco +8 devolvido.
unsafe extern "thiscall" fn comparar_cqk9_resultado_na_mls(this: usize, jogo: usize, estat: u32) -> u32 {
    let marca = *((jogo + 0x42) as *const i8);
    let bloco = cqk9_d(this + 8);
    let esperada = if cqk9_d(this + 0x30) == 1 { 1 } else if marca < 2 { -1 } else { marca };
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1738)(this, jogo, estat);
    let (m, b) = (*((jogo + 0x42) as *const i8), cqk9_d(this + 8));
    PLACAR.registrar(r == 1 && m == esperada && b == bloco, || format!("cqk9_resultado_na_mls({:x}, {:x}) devolveu {} marca {} x {} bloco {:x} x {:x}", this, jogo, r, m, esperada, b, bloco));
    r
}

/// { 0x90a000 } (cdecl, usa_mls.cpp): `usa_mls_get_club_all_star_conference(clube)`: a conferência do clube no Jogo das
/// Estrelas: 0x413 (o Leste) para New England, MetroStars, Tampa Bay, D.C. United, Columbus e Miami (as chaves 0x9d0390,
/// 0x9d0394, 0x9d0388, 0x9d0384, 0x9d03a4, 0x9d0380), 0x414 (o Oeste) para Colorado, Dallas, Kansas City, Los Angeles,
/// San Jose e Chicago (0x9d03a8, 0x9d03a0, 0x9d039c, 0x9d0398, 0x9d038c, 0x9d037c); 0 para clube cuja liga (+0x57) não
/// é a MLS (chave 0x9cf590); clube da MLS fora dos doze: a caixa "Error" (a original).
unsafe extern "C" fn cqk9_conferencia_das_estrelas(clube: usize) -> u16 {
    let nacao = cqk9_d(clube + 0x57);
    if nacao == 0 || cqk9_d(nacao) != cqk9_d(0x9cf590) {
        return 0;
    }
    let id = cqk9_d(clube);
    const LESTE: [usize; 6] = [0x9d0390, 0x9d0394, 0x9d0388, 0x9d0384, 0x9d03a4, 0x9d0380];
    const OESTE: [usize; 6] = [0x9d03a8, 0x9d03a0, 0x9d039c, 0x9d0398, 0x9d038c, 0x9d037c];
    if LESTE.iter().any(|&k| cqk9_d(k) == id) {
        crate::comparacao::ramo("cqk_9 conferencia 0x413");
        return 0x413; // cqk9: a primeira conferência
    }
    if OESTE.iter().any(|&k| cqk9_d(k) == id) {
        return 0x414;
    }
    orig::<unsafe extern "C" fn(usize) -> u16>(1739)(clube)
}

/// Comparador: a devolução (só lê; o caso de erro fica com a original sozinha).
unsafe extern "C" fn comparar_cqk9_conferencia_das_estrelas(clube: usize) -> u16 {
    let r = orig::<unsafe extern "C" fn(usize) -> u16>(1739)(clube);
    let nacao = cqk9_d(clube + 0x57);
    if nacao != 0 && cqk9_d(nacao) == cqk9_d(0x9cf590) && r == 0 {
        return r; // o caso de erro (a caixa já saiu na original)
    }
    let n = cqk9_conferencia_das_estrelas(clube);
    PLACAR.registrar(r == n, || format!("cqk9_conferencia_das_estrelas({:x}) original={:x} rust={:x}", clube, r, n));
    r
}

// ---------- 275a leva: as regras das quatro divisões inglesas: turnos, pontos, vagas de acesso, mata-mata e rebaixamento, desempates e a tabela de rodadas (eng_prm.cpp, eng_first.cpp, eng_second.cpp, eng_third.cpp) ----------

unsafe fn cqi4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
static mut CQI4_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
unsafe fn cqi4_regras(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQI4_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqi4_d(cqi4_d(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// As regras comuns às quatro divisões (os turnos +0x3c e os pontos da vitória +0xc2 vão em cada uma): 1 ponto pelo empate (+0xc3),
/// categoria 2 nas partidas (+0xc4, liga), clubes nacionais (+0x42 = 1), os desempates 1, 2 e 3 (+0xc5..+0xc7: saldo,
/// gols pró e a posição [inferência pelos códigos da Inglaterra]) e +0x52 = 2 [sem nome].
fn cqi4_comuns() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0x52, 1, 2)]
}

/// { 0x574b70 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::set_up_rules()`. A Premier: as comuns, +0xab e +0xdb = 0
/// (as palavras copiadas em cada partida), ninguém sobe (+0xbe = 0, +0xbf = 0, +0xc0 = 0), 3 caem direto (+0xc1),
/// nenhuma liga acima (+0x1c = −1) e a First abaixo (+0x20 = [0x9cf5c0]).
unsafe extern "thiscall" fn regras_da_premier(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da premier");
    cqi4_regras(this, &cqi4_premier())
}

unsafe fn cqi4_premier() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Premier, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 3)); // cqi4: a Premier rebaixa 3
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x20, 4, cqi4_d(0x9cf5c0) as u32)]);
    v
}

/// { 0x572d90 } (thiscall, ret 0, eng_first.cpp): `EngFirst::set_up_rules()`. A First: as comuns, 2 sobem direto
/// (+0xbe), 4 vão ao mata-mata de acesso (+0xbf, do 3º ao 6º), nenhum play-off de permanência (+0xc0), 3 caem
/// (+0xc1), a Premier acima (+0x1c = [0x9cf5bc]) e a Second abaixo (+0x20 = [0x9cf5c4]), 5 reservas no banco
/// (+0x49) e 3 substituições (+0x4a).
unsafe extern "thiscall" fn regras_da_first(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da first");
    cqi4_regras(this, &cqi4_first())
}

unsafe fn cqi4_first() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a First joga 2 turnos
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, cqi4_d(0x9cf5bc) as u32), (0x4a, 1, 3), (0x20, 4, cqi4_d(0x9cf5c4) as u32), (0x49, 1, 5)]);
    v
}

/// { 0x576780 } (thiscall, ret 0, eng_second.cpp): `EngSecond::set_up_rules()`. A Second: como a First, mas 4
/// caem (+0xc1); a First acima ([0x9cf5c0]) e a Third abaixo ([0x9cf5c8]).
unsafe extern "thiscall" fn regras_da_second(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da second");
    cqi4_regras(this, &cqi4_second())
}

unsafe fn cqi4_second() -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Second dá 3 pontos pela vitória
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, cqi4_d(0x9cf5c0) as u32), (0x4a, 1, 3), (0x20, 4, cqi4_d(0x9cf5c8) as u32), (0x49, 1, 5)]);
    v
}

/// { 0x5780c0 } (thiscall, ret 0, eng_third.cpp): `EngThird::set_up_rules()`. A Third: as comuns, 3 sobem direto
/// (+0xbe), 4 ao mata-mata (+0xbf, do 4º ao 7º), 1 cai (+0xc1), a Second acima ([0x9cf5c4]) e abaixo a Conference
/// ([0x9cf69c]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c da nação, [[+4]+0x5d]
/// [inferência]), senão −1; 5 reservas e 3 substituições.
unsafe extern "thiscall" fn regras_da_third(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_4 regras da third");
    cqi4_regras(this, &cqi4_third(this))
}

unsafe fn cqi4_third(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Third, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xbe, 1, 3), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 1), (0x1c, 4, cqi4_d(0x9cf5c4) as u32)]);
    let nacao = cqi4_d(cqi4_d(this + 4) + 0x5d);
    let abaixo = if *((nacao + 0x11c) as *const u8) & 4 != 0 { cqi4_d(0x9cf69c) as u32 } else { 0xffff_ffff };
    v.extend_from_slice(&[(0x20, 4, abaixo), (0x4a, 1, 3), (0x49, 1, 5)]);
    v
}

/// O comparador das quatro: a original monta a tabela de rodadas (aloca; não roda duas vezes), então é ensaio. A
/// original roda inteira; a faixa +0x1c..+0xec volta como estava e a cópia grava as regras sem a virtual +0x3c; os
/// campos que a cópia grava têm de sair iguais aos da original, e a devolução 1; no fim fica a faixa da original
/// (com a tabela de rodadas dela em +0xba, +0xa9 e +0x3a).
unsafe fn cqi4_comparar(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let faixa = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&faixa);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&faixa);
    antes.escrever();
    CQI4_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQI4_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&faixa);
    mo.escrever();
    let campo = |m: &crate::comparacao::Memoria, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = escritas.iter().filter(|e| campo(&mo, e.0, e.1) != campo(&mr, e.0, e.1)).map(|e| (e.0, campo(&mo, e.0, e.1), campo(&mr, e.0, e.1))).collect();
    PLACAR.registrar(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (deslocamento, original, rust)={:x?}", nome, this, r, rr, difs));
    r
}

unsafe extern "thiscall" fn comparar_regras_da_premier(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1740);
    cqi4_comparar("regras_da_premier", this, cqi4_premier(), || o(this), || regras_da_premier(this))
}

unsafe extern "thiscall" fn comparar_regras_da_first(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1741);
    cqi4_comparar("regras_da_first", this, cqi4_first(), || o(this), || regras_da_first(this))
}

unsafe extern "thiscall" fn comparar_regras_da_second(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1742);
    cqi4_comparar("regras_da_second", this, cqi4_second(), || o(this), || regras_da_second(this))
}

unsafe extern "thiscall" fn comparar_regras_da_third(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1743);
    cqi4_comparar("regras_da_third", this, cqi4_third(this), || o(this), || regras_da_third(this))
}

// ---------- 275a leva: a tabela das quatro divisões inglesas: os clubes de cada divisão, o fundo de prêmios e o dinheiro da TV no início da temporada (eng_prm.cpp, eng_second.cpp, eng_third.cpp, eng_first.cpp) ----------

unsafe fn cqi5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqi5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
unsafe fn cqi5_registro(g: usize) -> usize {
    cqi5_d(0xae23d0).wrapping_add((cqi5_d(g) as i32 as usize).wrapping_mul(0x6b))
}

/// O ano do jogo ([0xae2c92]) passou de 1999 (o dinheiro da TV maior).
unsafe fn cqi5_depois_de_1999() -> bool {
    core::ptr::read_unaligned(0xae2c92 as *const u16) > 0x7cf
}

/// League::pay_tv_rights (0x683010: thiscall(liga, valor, filtro), ret 8): o valor a cada clube da tabela, ou só aos
/// que vieram da competição `filtro` (+0x5b do clube).
unsafe fn cqi5_pagar(this: usize, valor: i32, filtro: usize) {
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    pagar(this, valor, filtro);
}

/// O corpo comum: +0x3e = `n` clubes; a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e devolve 0); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Sem encher: 0. Cheia: o fundo de prêmios
/// +0x28 (League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4), +0xe1 = 1, o dinheiro da TV (`pagar`) e 1.
unsafe fn cqi5_tabela(this: usize, n: i16, arquivo: usize, linha: u32, g: usize, cota: i32, pagar: impl FnOnce()) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return 0;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqi5_d(0xae23b4);
    while (i as i32) < cqi5_d(0xae235c) as i32 {
        let comp = cqi5_d(clube + 0x57);
        if comp != 0 && cqi5_d(comp) == cqi5_d(g) {
            let linha = k; // cqi5: a linha k recebe o k-ésimo clube da divisão
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqi5_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cqi5_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo("cqi_5 tabela cheia");
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
    let f = premio(this, cota);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
    pagar();
    1
}

/// { 0x574a20 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::create_table()`: 20 clubes ([0x9cf5bc]), cota 250.000 do
/// fundo de prêmios e a TV: 12.000.000 a cada clube depois de 1999 (5.000.000 antes). Fonte 0x9c43b0, linha 0x1ee.
unsafe extern "thiscall" fn tabela_da_premier(this: usize) -> u32 {
    cqi5_tabela(this, 0x14, 0x9c43b0, 0x1ee, 0x9cf5bc, 0x3d090, || {
        let valor = if cqi5_depois_de_1999() { 0xb71b00 } else { 0x4c4b40 }; // cqi5: a TV da Premier
        cqi5_pagar(this, valor, 0)
    })
}

/// { 0x576630 } (thiscall, ret 0, eng_second.cpp): `EngSecond::create_table()`: 24 clubes ([0x9cf5c4]), cota 20.000
/// e a TV: 2.500.000 depois de 1999 (300.000 antes). Fonte 0x9c43e8, linha 0x1f5.
unsafe extern "thiscall" fn tabela_da_second(this: usize) -> u32 {
    cqi5_tabela(this, 0x18, 0x9c43e8, 0x1f5, 0x9cf5c4, 0x4e20, || cqi5_pagar(this, if cqi5_depois_de_1999() { 0x2625a0 } else { 0x493e0 }, 0))
}

/// { 0x577f70 } (thiscall, ret 0, eng_third.cpp): `EngThird::create_table()`: 24 clubes ([0x9cf5c8]), cota 10.000
/// e a TV: 1.000.000 depois de 1999 (200.000 antes). Fonte 0x9c4424, linha 0x1fa.
unsafe extern "thiscall" fn tabela_da_third(this: usize) -> u32 {
    cqi5_tabela(this, 0x18, 0x9c4424, 0x1fa, 0x9cf5c8, 0x2710, || cqi5_pagar(this, if cqi5_depois_de_1999() { 0xf4240 } else { 0x30d40 }, 0))
}

/// { 0x572bf0 } (thiscall, ret 0, eng_first.cpp): `EngFirst::create_table()`: 24 clubes ([0x9cf5c0]), cota 70.000
/// e a TV pela divisão de onde o clube veio (+0x5b): depois de 1999, 5.877.777 aos que caíram da Premier (o registro
/// de [0x9cf5bc]), 4.099.999 aos que já estavam na First (o próprio registro, +4) e 4.099.999 aos que subiram da
/// Second ([0x9cf5c4]); antes, 1.000.000 a todos. Fonte 0x9c4374, linha 0x22d.
unsafe extern "thiscall" fn tabela_da_first(this: usize) -> u32 {
    cqi5_tabela(this, 0x18, 0x9c4374, 0x22d, 0x9cf5c0, 0x11170, || {
        if cqi5_depois_de_1999() {
            cqi5_pagar(this, 0x59b011, cqi5_registro(0x9cf5bc)); // cqi5: a TV de quem caiu da Premier
            cqi5_pagar(this, 0x3e8fa0, cqi5_d(this + 4));
            cqi5_pagar(this, 0x3e8fa0, cqi5_registro(0x9cf5c4));
        } else {
            cqi5_pagar(this, 0xf4240, 0);
        }
    })
}

/// O comparador das quatro: a original aloca a tabela e paga os clubes (não roda duas vezes): só ela. O plano, feito
/// antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu
/// (senão 0) e as linhas da tabela com os clubes do plano, na ordem; cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
unsafe fn cqi5_comparar(nome: &str, this: usize, n: i16, g: usize, cota: i32, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqi5_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cqi5_d(0xae23b4) + (i as usize) * 0x245;
        let comp = cqi5_d(clube + 0x57);
        if comp != 0 && cqi5_d(comp) == cqi5_d(g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqi5_d(this + 0xb1);
    let esperado = (plano.len() as i16 == n) as u32;
    let mut igual = r == esperado && cqi5_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqi5_d(t + k * 0x3b)).collect();
        igual = achou == plano;
        if r == 1 {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, cota);
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_tabela_da_premier(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1744);
    cqi5_comparar("tabela_da_premier", this, 0x14, 0x9cf5bc, 0x3d090, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_second(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1745);
    cqi5_comparar("tabela_da_second", this, 0x18, 0x9cf5c4, 0x4e20, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_third(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1746);
    cqi5_comparar("tabela_da_third", this, 0x18, 0x9cf5c8, 0x2710, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_first(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1747);
    cqi5_comparar("tabela_da_first", this, 0x18, 0x9cf5c0, 0x11170, || o(this))
}

// ---------- 275a leva: as regras das divisões italianas (Série A, B, C1/A e C1/B): turnos, pontos, vagas, desempates, o empate que vai ao jogo-desempate e a tabela de rodadas (ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------

unsafe fn cqi6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
static mut CQI6_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
unsafe fn cqi6_regras(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQI6_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqi6_d(cqi6_d(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// As regras comuns às quatro: 1 ponto pelo empate (+0xc3), categoria 2 nas partidas (+0xc4), +0xab e +0xdb = 0 (as
/// palavras copiadas nas partidas), clubes nacionais (+0x42 = 1), o 1º desempate 1 (+0xc5) e o 2º 2 (+0xc6), a ordem
/// fixa das rodadas na grade (bit 0x40 de +0xd9, com o byte de baixo como está), 7 reservas (+0x49) e 3 substituições
/// (+0x4a).
unsafe fn cqi6_comuns(this: usize) -> Vec<(usize, u8, u32)> {
    let d9 = *((this + 0xd9) as *const u8) | 0x40;
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0xab, 2, 0), (0xdb, 2, 0), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xd9, 1, d9 as u32), (0x49, 1, 7), (0x4a, 1, 3)]
}

/// { 0x641640 } (thiscall, ret 0, ita_ser_a.cpp): `ItaSerA::set_up_rules()`. A Série A: as comuns, 2 turnos, 3
/// pontos, os desempates 3º = 4 e 4º = 0 (+0xc7, +0xc8), ninguém sobe, nenhum play-off, 4 caem direto (+0xc1),
/// nenhuma liga acima (+0x1c = −1), a Série B abaixo (+0x20 = [0x9cf574]), +0x52 = 2 [sem nome] e +0xe6 = 1 (o
/// empate em pontos na fronteira de uma vaga não se decide pelos critérios: vai ao jogo-desempate, ver 0x642880).
unsafe extern "thiscall" fn regras_da_serie_a(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da serie a");
    cqi6_regras(this, &cqi6_serie_a(this))
}

unsafe fn cqi6_serie_a(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc7, 1, 4), (0xc8, 1, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 4)); // cqi6: a Série A rebaixa 4
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x20, 4, cqi6_d(0x9cf574) as u32), (0xe6, 4, 1)]);
    v
}

/// { 0x6468a0 } (thiscall, ret 0, ita_ser_b.cpp): `ItaSerB::set_up_rules()`. A Série B: como a A, mas 4 sobem
/// direto (+0xbe) e 4 caem (+0xc1); a A acima ([0x9cf570]) e a C1/A abaixo ([0x9cf578]).
unsafe extern "thiscall" fn regras_da_serie_b(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da serie b");
    cqi6_regras(this, &cqi6_serie_b(this))
}

unsafe fn cqi6_serie_b(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: a Série B joga 2 turnos
    v.extend_from_slice(&[(0xc7, 1, 4), (0xc8, 1, 0), (0xbe, 1, 4), (0xbf, 1, 0), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, cqi6_d(0x9cf570) as u32), (0x20, 4, cqi6_d(0x9cf578) as u32), (0x52, 1, 2), (0xe6, 4, 1)]);
    v
}

/// { 0x64b770 } (thiscall, ret 0, ita_ser_c1a.cpp): `ItaSerC1a::set_up_rules()`. A C1/A: as comuns, 2 turnos, 3
/// pontos, os desempates 3º = 0 (+0xc7), o campeão sobe direto (+0xbe = 1), do 2º ao 5º vão ao play-off de acesso
/// (+0xbf = 4), do 14º ao 17º ao play-out de permanência (+0xc0 = 4), o último cai (+0xc1 = 1), a Série B acima
/// ([0x9cf574]) e abaixo o id de [0x9cf57c] (o da C1/B, o outro grupo: assim no original).
unsafe extern "thiscall" fn regras_da_c1a(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da c1a");
    cqi6_regras(this, &cqi6_c1a(this))
}

unsafe fn cqi6_c1a(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi6: a C1/A dá 3 pontos pela vitória
    v.extend_from_slice(&[(0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 4), (0xc0, 1, 4), (0xc1, 1, 1), (0x1c, 4, cqi6_d(0x9cf574) as u32), (0x20, 4, cqi6_d(0x9cf57c) as u32)]);
    v
}

/// { 0x64f920 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::set_up_rules()`. A C1/B: como a C1/A; acima o id de
/// [0x9cf578] e abaixo a C2/A ([0x9cf580]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c da
/// nação [inferência]), senão −1.
unsafe extern "thiscall" fn regras_da_c1b(this: usize) -> u32 {
    crate::comparacao::ramo("cqi_6 regras da c1b");
    cqi6_regras(this, &cqi6_c1b(this))
}

unsafe fn cqi6_c1b(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqi6_comuns(this);
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc7, 1, 0), (0xbe, 1, 1), (0xbf, 1, 4), (0xc0, 1, 4), (0xc1, 1, 1), (0x1c, 4, cqi6_d(0x9cf578) as u32)]);
    let nacao = cqi6_d(cqi6_d(this + 4) + 0x5d);
    let abaixo = if *((nacao + 0x11c) as *const u8) & 4 != 0 { cqi6_d(0x9cf580) as u32 } else { 0xffff_ffff };
    v.push((0x20, 4, abaixo));
    v
}

/// O comparador das quatro (ensaio, como o da cqi_4): a original roda inteira; a faixa +0x1c..+0xec volta e a cópia
/// grava as regras sem a virtual +0x3c; os campos que a cópia grava saem iguais aos da original, e a devolução 1; no
/// fim fica a faixa da original.
unsafe fn cqi6_comparar(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let faixa = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&faixa);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&faixa);
    antes.escrever();
    CQI6_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQI6_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&faixa);
    mo.escrever();
    let campo = |m: &crate::comparacao::Memoria, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = escritas.iter().filter(|e| campo(&mo, e.0, e.1) != campo(&mr, e.0, e.1)).map(|e| (e.0, campo(&mo, e.0, e.1), campo(&mr, e.0, e.1))).collect();
    PLACAR.registrar(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (deslocamento, original, rust)={:x?}", nome, this, r, rr, difs));
    r
}

unsafe extern "thiscall" fn comparar_regras_da_serie_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1748);
    cqi6_comparar("regras_da_serie_a", this, cqi6_serie_a(this), || o(this), || regras_da_serie_a(this))
}

unsafe extern "thiscall" fn comparar_regras_da_serie_b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1749);
    cqi6_comparar("regras_da_serie_b", this, cqi6_serie_b(this), || o(this), || regras_da_serie_b(this))
}

unsafe extern "thiscall" fn comparar_regras_da_c1a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1750);
    cqi6_comparar("regras_da_c1a", this, cqi6_c1a(this), || o(this), || regras_da_c1a(this))
}

unsafe extern "thiscall" fn comparar_regras_da_c1b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1751);
    cqi6_comparar("regras_da_c1b", this, cqi6_c1b(this), || o(this), || regras_da_c1b(this))
}

// ---------- 275a leva: a tabela das divisões italianas (Série A, B, C1/A e C1/B): os clubes de cada divisão, o fundo de prêmios e o dinheiro da TV (ita_ser_a.cpp, ita_ser_b.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------

unsafe fn cqj4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqj4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O corpo comum: +0x3e = `n` clubes; a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e None); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve se encheu.
unsafe fn cqj4_encher(this: usize, n: i16, arquivo: usize, linha: u32, g: usize) -> Option<bool> {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return None;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqj4_d(0xae23b4);
    while (i as i32) < cqj4_d(0xae235c) as i32 {
        let comp = cqj4_d(clube + 0x57);
        if comp != 0 && cqj4_d(comp) == cqj4_d(g) {
            let linha = k; // cqj4: a linha k recebe o k-ésimo clube da divisão
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0); // cqj4: as linhas sem marca
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqj4_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    Some(k as i8 as i16 == cqj4_w(this + 0x3e))
}

/// O fundo de prêmios (+0x28 = League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4) e +0xe1 = 1 (o fundo
/// é pago pela tabela no fim da liga, 0x682e10).
unsafe fn cqj4_fundo(this: usize, cota: i32) {
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
    let f = premio(this, cota);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
}

/// { 0x641510 } (thiscall, ret 0, ita_ser_a.cpp): `ItaSerA::create_table()`: 18 clubes ([0x9cf570]); cheia, o fundo
/// pela cota de 250.000 e a TV de 5.000.000 a cada clube (League::pay_tv_rights 0x683010: thiscall(liga, valor, 0),
/// ret 8); devolve 1 (0 sem encher). Fonte 0x9ea2a8, linha 0x253.
unsafe extern "thiscall" fn tabela_da_serie_a(this: usize) -> u32 {
    match cqj4_encher(this, 0x12, 0x9ea2a8, 0x253, 0x9cf570) {
        Some(true) => {}
        _ => return 0,
    }
    crate::comparacao::ramo("cqj_4 tabela da serie a");
    cqj4_fundo(this, 0x3d090); // cqj4: a cota da Série A
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    pagar(this, 0x4c4b40, 0); // cqj4: a TV da Série A
    1
}

/// { 0x646780 } (thiscall, ret 0, ita_ser_b.cpp): `ItaSerB::create_table()`: 20 clubes ([0x9cf574]); cheia, o fundo
/// pela cota de 70.000 (sem TV) e 1. Fonte 0x9ea384, linha 0x19d.
unsafe extern "thiscall" fn tabela_da_serie_b(this: usize) -> u32 {
    match cqj4_encher(this, 0x14, 0x9ea384, 0x19d, 0x9cf574) {
        Some(true) => {}
        _ => return 0,
    }
    cqj4_fundo(this, 0x11170);
    1
}

/// { 0x64b670 } (thiscall, ret 0, ita_ser_c1a.cpp): `ItaSerC1a::create_table()`: 18 clubes ([0x9cf578]), sem fundo
/// nem TV; devolve se encheu. Fonte 0x9ea474, linha 0x1bd.
unsafe extern "thiscall" fn tabela_da_c1a(this: usize) -> u32 {
    cqj4_encher(this, 0x12, 0x9ea474, 0x1bd, 0x9cf578).unwrap_or(false) as u32
}

/// { 0x64f820 } (thiscall, ret 0, ita_ser_c1b.cpp): `ItaSerC1b::create_table()`: 18 clubes ([0x9cf57c]); como a
/// C1/A. Fonte 0x9ea558, linha 0x1be.
unsafe extern "thiscall" fn tabela_da_c1b(this: usize) -> u32 {
    cqj4_encher(this, 0x12, 0x9ea558, 0x1be, 0x9cf57c).unwrap_or(false) as u32
}

/// O comparador das quatro: a original aloca a tabela (e a da A paga a TV, com notícia): só ela. O plano, feito
/// antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu,
/// as linhas com os clubes do plano na ordem e, com `cota`, cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
unsafe fn cqj4_comparar(nome: &str, this: usize, n: i16, g: usize, cota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqj4_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cqj4_d(0xae23b4) + (i as usize) * 0x245;
        let comp = cqj4_d(clube + 0x57);
        if comp != 0 && cqj4_d(comp) == cqj4_d(g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqj4_d(this + 0xb1);
    let mut igual = r == (plano.len() as i16 == n) as u32 && cqj4_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqj4_d(t + k * 0x3b)).collect();
        igual = achou == plano;
        if let (1, Some(c)) = (r, cota) {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, c);
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_tabela_da_serie_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1752);
    cqj4_comparar("tabela_da_serie_a", this, 0x12, 0x9cf570, Some(0x3d090), || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_serie_b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1753);
    cqj4_comparar("tabela_da_serie_b", this, 0x14, 0x9cf574, Some(0x11170), || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_c1a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1754);
    cqj4_comparar("tabela_da_c1a", this, 0x12, 0x9cf578, None, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_c1b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1755);
    cqj4_comparar("tabela_da_c1b", this, 0x12, 0x9cf57c, None, || o(this))
}

// ---------- 275a leva: as regras das três divisões francesas: turnos, pontos, acesso e rebaixamento pelo ano (a virada para 20 clubes em 2002), desempates e a tabela de rodadas (fra_first.cpp, fra_second.cpp, fra_third.cpp) ----------

unsafe fn cqj5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
static mut CQJ5_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
unsafe fn cqj5_regras(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQJ5_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqj5_d(cqj5_d(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// A temporada 2001 (+0x40, o ano da liga, = 0x7d1): a última com 18 clubes na primeira divisão.
unsafe fn cqj5_em_2001(this: usize) -> bool {
    core::ptr::read_unaligned((this + 0x40) as *const u16) == 0x7d1
}

/// As regras comuns às três: 1 ponto pelo empate (+0xc3), categoria 2 nas partidas (+0xc4), clubes nacionais (+0x42
/// = 1), os desempates 1, 2 e 3 (+0xc5..+0xc7), nenhum play-off (+0xbf = +0xc0 = 0), os bits de regra +0xd9 = 0x83
/// (a liga tem campeão, as vagas de cima são de acesso e a ordem das rodadas dividida ao meio) e 3 substituições
/// (+0x4a).
fn cqj5_comuns() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0xbf, 1, 0), (0xc0, 1, 0), (0xd9, 2, 0x83), (0x4a, 1, 3)]
}

/// { 0x5bc410 } (thiscall, ret 0, fra_first.cpp): `FraFirst::set_up_rules()`. A primeira divisão: as comuns, 2
/// turnos, 3 pontos, +0xab e +0xdb = 0, ninguém sobe (+0xbe = 0), caem 2 em 2001 e 3 depois (+0xc1), 5 reservas
/// (+0x49), nenhuma liga acima (+0x1c = −1), +0x52 = 2 [sem nome] e a segunda abaixo (+0x20 = [0x9cf5d0]).
unsafe extern "thiscall" fn regras_da_franca_1(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_5 regras da primeira");
    cqj5_regras(this, &cqj5_primeira(this))
}

unsafe fn cqj5_primeira(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqj5_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0)]);
    let caem = if cqj5_em_2001(this) { 2 } else { 3 }; // cqj5: a primeira rebaixa 2 em 2001
    v.extend_from_slice(&[(0xc1, 1, caem), (0x49, 1, 5), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x20, 4, cqj5_d(0x9cf5d0) as u32)]);
    v
}

/// { 0x5bf0a0 } (thiscall, ret 0, fra_second.cpp): `FraSecond::set_up_rules()`. A segunda: as comuns, 2 turnos, 3
/// pontos; em 2001 sobem 4 (+0xbe) e caem 2 (+0xc1), depois sobem 3 e caem 3; a primeira acima ([0x9cf5cc]), a
/// terceira abaixo ([0x9cf5d4]), +0x52 = 2 e 5 reservas.
unsafe extern "thiscall" fn regras_da_franca_2(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_5 regras da segunda");
    cqj5_regras(this, &cqj5_segunda(this))
}

unsafe fn cqj5_segunda(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqj5_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqj5: a segunda dá 3 pontos pela vitória
    let (sobem, caem) = if cqj5_em_2001(this) { (4, 2) } else { (3, 3) };
    v.extend_from_slice(&[(0xbe, 1, sobem), (0xc1, 1, caem), (0x1c, 4, cqj5_d(0x9cf5cc) as u32), (0x52, 1, 2), (0x20, 4, cqj5_d(0x9cf5d4) as u32), (0x49, 1, 5)]);
    v
}

/// { 0x5c0700 } (thiscall, ret 0, fra_third.cpp): `FraThird::set_up_rules()`. O National: as comuns, 2 turnos, 3
/// pontos; sobem 4 em 2001 e 3 depois (+0xbe), caem 4 (+0xc1); a segunda acima ([0x9cf5d0]), nenhuma abaixo (+0x20
/// = −1), +0xa7 (os jogos da temporada) = −1 (recalculado pela 0x687430) e 3 reservas (+0x49).
unsafe extern "thiscall" fn regras_da_franca_3(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_5 regras do national");
    cqj5_regras(this, &cqj5_terceira(this))
}

unsafe fn cqj5_terceira(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqj5_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqj5: o National joga 2 turnos
    let sobem = if cqj5_em_2001(this) { 4 } else { 3 };
    v.extend_from_slice(&[(0xbe, 1, sobem), (0xc1, 1, 4), (0x1c, 4, cqj5_d(0x9cf5d0) as u32), (0x49, 1, 3), (0x20, 4, 0xffff_ffff), (0xa7, 2, 0xffff)]);
    v
}

/// O comparador das três (ensaio, como o da cqi_4): a original roda inteira; a faixa +0x1c..+0xec volta e a cópia
/// grava as regras sem a virtual +0x3c; os campos que a cópia grava saem iguais aos da original, e a devolução 1; no
/// fim fica a faixa da original.
unsafe fn cqj5_comparar(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let faixa = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&faixa);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&faixa);
    antes.escrever();
    CQJ5_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQJ5_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&faixa);
    mo.escrever();
    let campo = |m: &crate::comparacao::Memoria, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = escritas.iter().filter(|e| campo(&mo, e.0, e.1) != campo(&mr, e.0, e.1)).map(|e| (e.0, campo(&mo, e.0, e.1), campo(&mr, e.0, e.1))).collect();
    PLACAR.registrar(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (deslocamento, original, rust)={:x?}", nome, this, r, rr, difs));
    r
}

unsafe extern "thiscall" fn comparar_regras_da_franca_1(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1756);
    cqj5_comparar("regras_da_franca_1", this, cqj5_primeira(this), || o(this), || regras_da_franca_1(this))
}

unsafe extern "thiscall" fn comparar_regras_da_franca_2(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1757);
    cqj5_comparar("regras_da_franca_2", this, cqj5_segunda(this), || o(this), || regras_da_franca_2(this))
}

unsafe extern "thiscall" fn comparar_regras_da_franca_3(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1758);
    cqj5_comparar("regras_da_franca_3", this, cqj5_terceira(this), || o(this), || regras_da_franca_3(this))
}

// ---------- 275a leva: a tabela das três divisões francesas: os clubes (18 ou 20 na primeira, pelo ano), o fundo de prêmios e o dinheiro da TV, na segunda pela divisão de onde o clube veio (fra_first.cpp, fra_second.cpp, fra_third.cpp) ----------

unsafe fn cqj6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqj6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
unsafe fn cqj6_registro(g: usize) -> usize {
    cqj6_d(0xae23d0).wrapping_add((cqj6_d(g) as i32 as usize).wrapping_mul(0x6b))
}

/// League::pay_tv_rights (0x683010: thiscall(liga, valor, filtro), ret 8).
unsafe fn cqj6_pagar(this: usize, valor: i32, filtro: usize) {
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    pagar(this, valor, filtro);
}

/// O fundo de prêmios (+0x28 = League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4) e +0xe1 = 1 (pago pela
/// tabela no fim da liga, 0x682e10).
unsafe fn cqj6_fundo(this: usize, cota: i32) {
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
    let f = premio(this, cota);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
}

/// O corpo comum, com +0x3e já gravado: a tabela (malloc 0x944e46 de +0x3e × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e None); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve se encheu.
unsafe fn cqj6_encher(this: usize, arquivo: usize, linha: u32, g: usize) -> Option<bool> {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((cqj6_w(this + 0x3e) as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return None;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqj6_d(0xae23b4);
    while (i as i32) < cqj6_d(0xae235c) as i32 {
        let comp = cqj6_d(clube + 0x57);
        if comp != 0 && cqj6_d(comp) == cqj6_d(g) {
            let linha = k; // cqj6: a linha k recebe o k-ésimo clube da divisão
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqj6_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    Some(k as i8 as i16 == cqj6_w(this + 0x3e))
}

/// { 0x5bc2b0 } (thiscall, ret 0, fra_first.cpp): `FraFirst::create_table()`: 18 clubes na temporada 2001 (+0x40 =
/// 0x7d1) e 20 depois ([0x9cf5cc]), +0xa7 = −1; cheia, o fundo pela cota de 200.000 e a TV de 6.000.000 a cada clube;
/// 1 (0 sem encher). Fonte 0x9ccb5c, linha 0x1d3.
unsafe extern "thiscall" fn tabela_da_franca_1(this: usize) -> u32 {
    let n: i16 = if core::ptr::read_unaligned((this + 0x40) as *const u16) == 0x7d1 { 0x12 } else { 0x14 };
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    core::ptr::write_unaligned((this + 0xa7) as *mut u16, 0xffff);
    if cqj6_encher(this, 0x9ccb5c, 0x1d3, 0x9cf5cc) != Some(true) {
        return 0;
    }
    crate::comparacao::ramo("cqj_6 tabela da primeira");
    cqj6_fundo(this, 0x30d40);
    cqj6_pagar(this, 0x5b8d80, 0); // cqj6: a TV da primeira
    1
}

/// { 0x5bef10 } (thiscall, ret 0, fra_second.cpp): `FraSecond::create_table()`: 20 clubes ([0x9cf5d0]); cheia, sem
/// fundo (+0x28 = 0, +0xe1 = 2: nada a pagar no fim) e a TV pela divisão de onde o clube veio (+0x5b): 2.000.000 aos
/// que caíram da primeira ([0x9cf5cc]), 1.500.000 aos que ficaram ([0x9cf5d0]) e 1.500.000 aos que subiram do
/// National ([0x9cf5d4]); 1. Fonte 0x9ccc0c, linha 0x171.
unsafe extern "thiscall" fn tabela_da_franca_2(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x14);
    if cqj6_encher(this, 0x9ccc0c, 0x171, 0x9cf5d0) != Some(true) {
        return 0;
    }
    crate::comparacao::ramo("cqj_6 tabela da segunda");
    core::ptr::write_unaligned((this + 0x28) as *mut i32, 0);
    *((this + 0xe1) as *mut u8) = 2;
    cqj6_pagar(this, 0x1e8480, cqj6_registro(0x9cf5cc)); // cqj6: a TV de quem caiu da primeira
    cqj6_pagar(this, 0x16e360, cqj6_registro(0x9cf5d0));
    cqj6_pagar(this, 0x16e360, cqj6_registro(0x9cf5d4));
    1
}

/// { 0x5c05d0 } (thiscall, ret 0, fra_third.cpp): `FraThird::create_table()`: 20 clubes ([0x9cf5d4]); cheia, o fundo
/// pela cota de 20.000 e a TV de 200.000 a cada clube; 1. Fonte 0x9ccc80, linha 0x15d.
unsafe extern "thiscall" fn tabela_da_franca_3(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x14);
    if cqj6_encher(this, 0x9ccc80, 0x15d, 0x9cf5d4) != Some(true) {
        return 0;
    }
    cqj6_fundo(this, 0x4e20);
    cqj6_pagar(this, 0x30d40, 0);
    1
}

/// O comparador das três: a original aloca a tabela e paga a TV (com notícia): só ela. O plano, feito antes: os
/// clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu, as linhas
/// com os clubes do plano na ordem e, cheia, +0xe1 e +0x28 como a rotina grava (`cota`: o fundo da cota e 1; None: 0
/// e 2).
unsafe fn cqj6_comparar(nome: &str, this: usize, n: i16, g: usize, cota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < cqj6_d(0xae235c) as i32 && (plano.len() as i16) < n {
        let clube = cqj6_d(0xae23b4) + (i as usize) * 0x245;
        let comp = cqj6_d(clube + 0x57);
        if comp != 0 && cqj6_d(comp) == cqj6_d(g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = cqj6_d(this + 0xb1);
    let mut igual = r == (plano.len() as i16 == n) as u32 && cqj6_w(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqj6_d(t + k * 0x3b)).collect();
        igual = achou == plano;
        if r == 1 {
            let e1 = *((this + 0xe1) as *const u8);
            let f = core::ptr::read_unaligned((this + 0x28) as *const i32);
            igual = igual
                && match cota {
                    Some(c) => {
                        let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
                        e1 == 1 && f == premio(this, c)
                    }
                    None => e1 == 2 && f == 0,
                };
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

unsafe extern "thiscall" fn comparar_tabela_da_franca_1(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1759);
    let n: i16 = if core::ptr::read_unaligned((this + 0x40) as *const u16) == 0x7d1 { 0x12 } else { 0x14 };
    cqj6_comparar("tabela_da_franca_1", this, n, 0x9cf5cc, Some(0x30d40), || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_franca_2(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1760);
    cqj6_comparar("tabela_da_franca_2", this, 0x14, 0x9cf5d0, None, || o(this))
}

unsafe extern "thiscall" fn comparar_tabela_da_franca_3(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1761);
    cqj6_comparar("tabela_da_franca_3", this, 0x14, 0x9cf5d4, Some(0x4e20), || o(this))
}

// ---------- 275a leva: as regras das divisões espanholas (Primera, Segunda e a divisão de baixo): turnos, pontos, vagas, o confronto direto primeiro nos desempates e a tabela de rodadas (spa_first.cpp, spa_second.cpp, spa_lower.cpp) ----------

unsafe fn cqk4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
static mut CQK4_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
unsafe fn cqk4_regras(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQK4_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqk4_d(cqk4_d(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// As regras comuns às três: 1 ponto pelo empate (+0xc3), categoria 2 nas partidas (+0xc4), clubes nacionais (+0x42
/// = 1), os desempates 4, 1 e 2 (+0xc5..+0xc7: o confronto direto primeiro [inferência pelo código 4], depois o
/// saldo e os gols pró), nenhum play-off de permanência (+0xc0 = 0), os bits de regra +0xd9 = 0x43 (campeão, vagas de
/// cima de acesso e a ordem fixa das rodadas), 7 reservas (+0x49) e 3 substituições (+0x4a).
fn cqk4_comuns() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 4), (0xc6, 1, 1), (0xc7, 1, 2), (0xc0, 1, 0), (0xd9, 2, 0x43), (0x49, 1, 7), (0x4a, 1, 3)]
}

/// { 0x84fdf0 } (thiscall, ret 0, spa_first.cpp): `SpaFirst::set_up_rules()`. A Primera: +0xa9 = 38 (as rodadas;
/// a virtual reescreve), as comuns, 2 turnos, 3 pontos, o 4º desempate 3 (+0xc8), +0xab e +0xdb = 0, ninguém sobe
/// (+0xbe = +0xbf = 0), 3 caem (+0xc1), nenhuma liga acima (+0x1c = −1), a Segunda abaixo (+0x20 = [0x9cf628]) e
/// +0x52 = 2 [sem nome].
unsafe extern "thiscall" fn regras_da_primera(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_4 regras da primera");
    core::ptr::write_unaligned((this + 0xa9) as *mut u16, 0x26);
    cqk4_regras(this, &cqk4_primera())
}

unsafe fn cqk4_primera() -> Vec<(usize, u8, u32)> {
    let mut v = cqk4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xc8, 1, 3), (0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0), (0xbf, 1, 0)]);
    v.push((0xc1, 1, 3)); // cqk4: a Primera rebaixa 3
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x20, 4, cqk4_d(0x9cf628) as u32), (0x52, 1, 2)]);
    v
}

/// { 0x855cc0 } (thiscall, ret 0, spa_second.cpp): `SpaSecond::set_up_rules()`. A Segunda: as comuns, 2 turnos, 3
/// pontos, o 4º desempate 3, 3 sobem direto (+0xbe), nenhum play-off de acesso, 4 caem (+0xc1), a Primera acima
/// ([0x9cf624]) e abaixo a Segunda B ([0x9cf7b8]) quando a nação tem as divisões de baixo carregadas (bit 4 de +0x11c
/// da nação [inferência]), senão −1; +0x52 = 2.
unsafe extern "thiscall" fn regras_da_segunda(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_4 regras da segunda");
    cqk4_regras(this, &cqk4_segunda(this))
}

unsafe fn cqk4_segunda(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqk4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqk4: a Segunda joga 2 turnos
    v.extend_from_slice(&[(0xc8, 1, 3), (0xbe, 1, 3), (0xbf, 1, 0), (0xc1, 1, 4), (0x1c, 4, cqk4_d(0x9cf624) as u32)]);
    let nacao = cqk4_d(cqk4_d(this + 4) + 0x5d);
    let abaixo = if *((nacao + 0x11c) as *const u8) & 4 != 0 { cqk4_d(0x9cf7b8) as u32 } else { 0xffff_ffff };
    v.extend_from_slice(&[(0x20, 4, abaixo), (0x52, 1, 2)]);
    v
}

/// { 0x853790 } (thiscall, ret 0, spa_lower.cpp): `SpaLower::set_up_rules()`. A divisão de baixo (os times B e os
/// melhores das divisões menores): as comuns, 2 turnos, 3 pontos, 2 sobem direto (+0xbe) e 2 vão ao play-off de
/// acesso (+0xbf), 4 caem (+0xc1), a Primera como liga de cima ([0x9cf624], assim no original) e nenhuma abaixo.
unsafe extern "thiscall" fn regras_da_divisao_de_baixo(this: usize) -> u32 {
    crate::comparacao::ramo("cqk_4 regras da divisao de baixo");
    cqk4_regras(this, &cqk4_baixo())
}

unsafe fn cqk4_baixo() -> Vec<(usize, u8, u32)> {
    let mut v = cqk4_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqk4: a divisão de baixo dá 3 pontos pela vitória
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 2), (0xc1, 1, 4), (0x1c, 4, cqk4_d(0x9cf624) as u32), (0x20, 4, 0xffff_ffff)]);
    v
}

/// O comparador das três (ensaio, como o da cqi_4): a original roda inteira; a faixa +0x1c..+0xec volta e a cópia
/// grava as regras sem a virtual +0x3c; os campos da lista saem iguais aos da original (o +0xa9 da Primera, que a
/// virtual reescreve, fica de fora), e a devolução 1; no fim fica a faixa da original.
unsafe fn cqk4_comparar(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let faixa = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&faixa);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&faixa);
    antes.escrever();
    CQK4_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQK4_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&faixa);
    mo.escrever();
    let campo = |m: &crate::comparacao::Memoria, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = escritas.iter().filter(|e| campo(&mo, e.0, e.1) != campo(&mr, e.0, e.1)).map(|e| (e.0, campo(&mo, e.0, e.1), campo(&mr, e.0, e.1))).collect();
    PLACAR.registrar(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (deslocamento, original, rust)={:x?}", nome, this, r, rr, difs));
    r
}

unsafe extern "thiscall" fn comparar_regras_da_primera(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1762);
    cqk4_comparar("regras_da_primera", this, cqk4_primera(), || o(this), || regras_da_primera(this))
}

unsafe extern "thiscall" fn comparar_regras_da_segunda(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1763);
    cqk4_comparar("regras_da_segunda", this, cqk4_segunda(this), || o(this), || regras_da_segunda(this))
}

unsafe extern "thiscall" fn comparar_regras_da_divisao_de_baixo(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1764);
    cqk4_comparar("regras_da_divisao_de_baixo", this, cqk4_baixo(), || o(this), || regras_da_divisao_de_baixo(this))
}

// ---------- 275a leva: a Primera espanhola (a tabela, a TV, a divisão de baixo genérica e a situação nas estatísticas) e os registros da Série C2/A italiana pela situação na tabela, no play-off e no play-out (spa_first.cpp, ita_ser_c2a.cpp) ----------

unsafe fn cqk5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqk5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
unsafe fn cqk5_registro(g: usize) -> usize {
    cqk5_d(0xae23d0).wrapping_add((cqk5_d(g) as i32 as usize).wrapping_mul(0x6b))
}

/// { 0x84fc70 } (thiscall, ret 0, spa_first.cpp): `SpaFirst::create_table()`. +0x3e = 20 e a tabela (malloc 0x944e46
/// de 0x49c; sem memória: a caixa do fonte 0xa82844, linha 0x250, e 0). Antes de encher, os clubes da divisão de
/// baixo espanhola ([0x9cf6b0]) passam à divisão de baixo genérica (+0x57 = o registro de [0x9cf764]). Depois os
/// clubes da Primera ([0x9cf624]) na ordem do banco, cada um na linha seguinte (League::init_table_row 0x687430:
/// thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher (sem encher: 0). Cheia: o fundo de prêmios (+0x28 =
/// League::calc_prize_fund 0x682f70 pela cota de 250.000), +0xe1 = 1, a TV de 5.000.000 a cada clube (0x683010:
/// thiscall(liga, valor, 0), ret 8) e 1.
unsafe extern "thiscall" fn tabela_da_primera(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x14);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(0x49c);
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(0xa82844, 0x250);
        return 0;
    }
    // a divisão de baixo espanhola vira a genérica
    let mut i: i16 = 0;
    let mut campo = cqk5_d(0xae23b4) + 0x57;
    while (i as i32) < cqk5_d(0xae235c) as i32 {
        let comp = cqk5_d(campo);
        if comp != 0 && cqk5_d(comp) == cqk5_d(0x9cf6b0) {
            core::ptr::write_unaligned(campo as *mut usize, cqk5_registro(0x9cf764));
        }
        i = i.wrapping_add(1);
        campo = campo.wrapping_add(0x245);
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqk5_d(0xae23b4);
    while (i as i32) < cqk5_d(0xae235c) as i32 {
        let comp = cqk5_d(clube + 0x57);
        if comp != 0 && cqk5_d(comp) == cqk5_d(0x9cf624) {
            let linha = k; // cqk5: a linha k recebe o k-ésimo clube da Primera
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqk5_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cqk5_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo("cqk_5 tabela da primera");
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
    let f = premio(this, 0x3d090); // cqk5: a cota da Primera
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1; // cqk5: o prêmio pela tabela no fim da liga
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    pagar(this, 0x4c4b40, 0); // cqk5: a TV da Primera
    1
}

/// Comparador da 0x84fc70: a original aloca a tabela, muda a divisão dos clubes e paga a TV (com notícia): só ela. O
/// plano, feito antes: os clubes da divisão de baixo espanhola (que passam à genérica) e os da Primera na ordem do
/// banco (até 20). Depois: os primeiros com +0x57 = o registro de [0x9cf764]; +0x3e = 20, a devolução, as linhas com
/// os clubes da Primera na ordem e, cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
unsafe extern "thiscall" fn comparar_tabela_da_primera(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1765);
    let (mut baixo, mut plano): (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    for i in 0..(cqk5_d(0xae235c) as i32).max(0) as usize {
        let clube = cqk5_d(0xae23b4) + i * 0x245;
        let comp = cqk5_d(clube + 0x57);
        if comp != 0 && cqk5_d(comp) == cqk5_d(0x9cf6b0) {
            baixo.push(clube);
        } else if comp != 0 && cqk5_d(comp) == cqk5_d(0x9cf624) && plano.len() < 0x14 {
            plano.push(clube);
        }
    }
    let r = o(this);
    let t = cqk5_d(this + 0xb1);
    let generica = cqk5_registro(0x9cf764);
    let mut igual = r == (plano.len() == 0x14) as u32 && cqk5_w(this + 0x3e) == 0x14 && t != 0 && baixo.iter().all(|&c| cqk5_d(c + 0x57) == generica);
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqk5_d(t + k * 0x3b)).collect();
        igual = achou == plano;
        if r == 1 {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, 0x3d090);
        }
    }
    PLACAR.registrar(igual, || format!("tabela_da_primera({:#x}) original={} baixo={} plano={:x?} tabela={:x?}", this, r, baixo.len(), plano, achou));
    r
}

/// O registro de staff ([0xdd7f00]).
unsafe fn cqk5_registros() -> usize {
    cqk5_d(0xdd7f00)
}

/// { 0x654180 } (thiscall, ret 0x18, ita_ser_c2a.cpp): `ItaSerC2a::update_staff_records_for_status(clube, situação,
/// fase, _, jogo, _)`, a virtual +0x44 (anunciada pela 0x686670). Devolve 0. Na liga (fase 0xff), pela situação: 5,
/// o título (Staff_Records::add_league_champions_record 0x868c50: thiscall(registros, clube, comp +4), ret 8); 0, o
/// acesso (0x869480: thiscall(registros, clube, comp, 0x64), ret 0xc); 1, a vaga no play-off (0x86bdd0:
/// thiscall(registros, clube, comp, 0xa0, 0x82, 0x28), ret 0x14); 3, o rebaixamento (0x86a1c0: thiscall(registros,
/// clube, comp), ret 8). No play-off de acesso (fase 0), na primeira linha do clube com situação 1, 2 ou 0xff: 1
/// (venceu), o acesso com peso 0x32 e +0x37 = 0; 2 ou 0xff (caiu), a eliminação (0x86c000: thiscall(registros, clube,
/// comp, fase do jogo +0x32, rodada +0x30, peso), ret 0x14; o peso = comp_util_scale_by_round 0x4bf850: cdecl(0x1e,
/// 0x32, rodada do jogo +0x34, 2)) e +0x37 = 0xfe. No play-out (fase 1): 1 (salvou-se), +0x37 = 0xfe; 2 ou 0xff,
/// o rebaixamento e +0x37 = 3.
unsafe extern "thiscall" fn registros_pela_situacao_na_c2a(this: usize, clube: usize, situacao: u32, fase: u32, _a4: u32, jogo: usize, _a6: u32) -> u32 {
    let s = situacao as u8;
    let comp = cqk5_d(this + 4);
    let f = fase as u8;
    if f == 0xff {
        match s {
            5 => {
                let titulo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
                titulo(cqk5_registros(), clube, comp);
            }
            0 => {
                let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
                acesso(cqk5_registros(), clube, comp, 0x64);
            }
            1 => {
                crate::comparacao::ramo("cqk_5 vaga no play-off da c2a");
                let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
                vaga(cqk5_registros(), clube, comp, 0xa0, 0x82, 0x28);
            }
            3 => {
                crate::comparacao::ramo("cqk_5 rebaixamento na c2a");
                let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
                cair(cqk5_registros(), clube, comp); // cqk5: o rebaixamento pela tabela da C2/A
            }
            _ => {}
        }
        return 0;
    }
    if f > 1 {
        return 0;
    }
    let n = cqk5_w(this + 0x3e) as i32;
    let mut i: i32 = 0;
    while i < n {
        let linha = cqk5_d(this + 0xb1) + (i as usize) * 0x3b;
        if cqk5_d(linha) == clube {
            if f == 0 {
                match s {
                    1 => {
                        let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
                        acesso(cqk5_registros(), clube, comp, 0x32);
                        *((cqk5_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0;
                        return 0;
                    }
                    2 | 0xff => {
                        let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4bf850usize);
                        let peso = escala(0x1e, 0x32, cqk5_w(jogo + 0x34) as u16 as u32, 2);
                        // ponytail: a original empurra a fase (+0x32) e a rodada (+0x30) do jogo como palavras em
                        // registradores com lixo nos 16 bits de cima; a 0x86c000 só lê as palavras.
                        let fora: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
                        fora(cqk5_registros(), clube, comp, cqk5_w(jogo + 0x32) as u16 as u32, cqk5_w(jogo + 0x30) as u16 as u32, peso);
                        *((cqk5_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                        return 0;
                    }
                    _ => {}
                }
            } else {
                match s {
                    1 => {
                        *((cqk5_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                        return 0;
                    }
                    2 | 0xff => {
                        let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
                        cair(cqk5_registros(), clube, comp);
                        *((cqk5_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 3;
                        return 0;
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
    0
}

/// Comparador da 0x654180: os registros vão ao histórico das pessoas e mexem nas reputações e no moral (não rodam duas
/// vezes): só a original. O plano, feito antes, nas fases 0 e 1: a primeira linha do clube com situação 1, 2 ou 0xff
/// decide o +0x37 que a rotina grava (fase 0: 0 com 1, 0xfe com 2/0xff; fase 1: 0xfe com 1, 3 com 2/0xff); depois a
/// linha tem de estar assim. Na liga ela só registra: a devolução 0.
unsafe extern "thiscall" fn comparar_registros_pela_situacao_na_c2a(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(1766);
    let (s, f) = (situacao as u8, fase as u8);
    let mut plano: Option<(usize, u8)> = None;
    if f <= 1 {
        let n = cqk5_w(this + 0x3e) as i32;
        for i in 0..n.max(0) as usize {
            let l = cqk5_d(this + 0xb1) + i * 0x3b;
            if cqk5_d(l) == clube && (s == 1 || s == 2 || s == 0xff) {
                plano = Some((l, match (f, s) {
                    (0, 1) => 0,
                    (0, _) => 0xfe,
                    (_, 1) => 0xfe,
                    _ => 3,
                }));
                break;
            }
        }
    }
    let r = o(this, clube, situacao, fase, a4, jogo, a6);
    let igual = r == 0 && plano.map_or(true, |(l, v)| *((l + 0x37) as *const u8) == v);
    PLACAR.registrar(igual, || format!("registros_pela_situacao_na_c2a({:#x}, {:#x}, {}, {}) original={} plano={:x?}", this, clube, s, f, r, plano));
    r
}

/// { 0x850880 } (thiscall, ret 0x14, spa_first.cpp): `SpaFirst::set_club_progress(clube, fase, atual, melhor, pior)`,
/// a virtual +0x68 (da 0x684cd0 a cada linha da tabela): na entrada do clube nas estatísticas (Comp_Stats::
/// find_club_entry 0x4a4850: thiscall(+8, clube), ret 4) grava a posição atual (+0x73), a melhor possível (+0x74;
/// 17 e 18 viram 16) e a pior (+0x75; 17 e 18 viram 19): a faixa de 17 a 18 da Primera não conta como faixa própria
/// [inferência: as posições de 17 a 18 juntam-se à vizinha de cima na melhor e à de baixo na pior].
unsafe extern "thiscall" fn situacao_na_primera(this: usize, clube: usize, _fase: u32, atual: u32, melhor: u32, pior: u32) {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = achar(cqk5_d(this + 8), clube);
    let m = match melhor as u8 {
        0x11 | 0x12 => 0x10,
        v => v,
    };
    let p = match pior as u8 {
        0x11 | 0x12 => 0x13,
        v => v,
    };
    *((e + 0x73) as *mut u8) = atual as u8;
    *((e + 0x74) as *mut u8) = m;
    *((e + 0x75) as *mut u8) = p;
}

/// Comparador da 0x850880: a entrada é achada antes (a original e a cópia acham a mesma) e as duas rodam lado a lado
/// nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
unsafe extern "thiscall" fn comparar_situacao_na_primera(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(1767);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = achar(cqk5_d(this + 8), clube);
    if e == 0 {
        return o(this, clube, fase, atual, melhor, pior);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x73, 3)], || o(this, clube, fase, atual, melhor, pior), || situacao_na_primera(this, clube, fase, atual, melhor, pior));
    PLACAR.registrar(igual, || format!("situacao_na_primera({:#x}, {:#x}) original={:x?} rust={:x?}", this, clube, mo, mr));
}

// ---------- 275a leva: as tabelas dos grupos da CFA e da divisão de baixo francesa (a marca dos clubes de divisão maior) e a situação nas estatísticas da Série C1 italiana (o play-off e o play-out em faixas) (fra_cfa.cpp, fra_lower.cpp, ita_ser_c1a.cpp, ita_ser_c1b.cpp) ----------

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
unsafe fn cqk6_tabela(this: usize, clubes: usize, n: i16, arquivo: usize, linha: u32, marcadas: &[usize]) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
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
unsafe extern "thiscall" fn tabela_do_grupo_da_cfa(this: usize, clubes: usize) -> u32 {
    cqk6_tabela(this, clubes, 0x12, 0x9ccaf0, 0x185, &[0x9cf5cc, 0x9cf5d0])
}

/// { 0x5be230 } (thiscall, ret 4, fra_lower.cpp): `FraLower::create_table(clubes)`: a divisão de baixo francesa com
/// 20 clubes da lista (o construtor 0x5bd600 escolhe-os), marcando os que estão no National ([0x9cf5d4]). Fonte
/// 0x9ccbd0, linha 0x175.
unsafe extern "thiscall" fn tabela_da_divisao_de_baixo_francesa(this: usize, clubes: usize) -> u32 {
    cqk6_tabela(this, clubes, 0x14, 0x9ccbd0, 0x175, &[0x9cf5d4])
}

/// O comparador das duas: a original aloca a tabela: só ela. O plano, feito antes: os `n` clubes da lista e a marca de
/// cada um. Depois: +0x3e = `n`, a devolução 1 e as linhas com os clubes na ordem e a marca (+0x38).
unsafe fn cqk6_comparar(nome: &str, this: usize, clubes: usize, n: i16, marcadas: &[usize], original: impl FnOnce() -> u32) -> u32 {
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

unsafe extern "thiscall" fn comparar_tabela_do_grupo_da_cfa(this: usize, clubes: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1768);
    cqk6_comparar("tabela_do_grupo_da_cfa", this, clubes, 0x12, &[0x9cf5cc, 0x9cf5d0], || o(this, clubes))
}

unsafe extern "thiscall" fn comparar_tabela_da_divisao_de_baixo_francesa(this: usize, clubes: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1769);
    cqk6_comparar("tabela_da_divisao_de_baixo_francesa", this, clubes, 0x14, &[0x9cf5d4], || o(this, clubes))
}

/// A situação do clube nas estatísticas da Série C1 (o corpo de 0x64bd80 e 0x64ff40): na entrada do clube
/// (Comp_Stats::find_club_entry 0x4a4850: thiscall(+8, clube), ret 4) grava a posição atual (+0x73), a melhor (+0x74)
/// e a pior (+0x75). Na liga (fase 0xff): a melhor de 2 a 5 (o play-off de acesso) vira 2 e de 14 a 17 (o play-out)
/// vira 14; a pior de 2 a 5 vira 5 e de 14 a 17 vira 17. No play-off (fase 0): as três + 1. No play-out (fase 1): a
/// melhor 1 vira 14 e 2 vira 16. Noutra fase, como vieram.
unsafe fn cqk6_situacao_na_c1(this: usize, clube: usize, fase: u8, atual: u8, melhor: u8, pior: u8) {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = achar(cqk6_d(this + 8), clube);
    let (mut a, mut m, mut p) = (atual, melhor, pior);
    if fase == 0xff {
        m = match m {
            2..=5 => 2,
            14..=17 => {
                crate::comparacao::ramo("cqk_6 faixa do play-out");
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
unsafe extern "thiscall" fn situacao_na_c1a(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    cqk6_situacao_na_c1(this, clube, fase as u8, atual as u8, melhor as u8, pior as u8)
}

/// { 0x64ff40 } (thiscall, ret 0x14, ita_ser_c1b.cpp): `ItaSerC1b::set_club_progress(...)`, igual à da C1/A.
unsafe extern "thiscall" fn situacao_na_c1b(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    cqk6_situacao_na_c1(this, clube, fase as u8, atual as u8, melhor as u8, pior as u8)
}

/// O comparador das duas: a entrada é achada antes (a original e a cópia acham a mesma) e as duas rodam lado a lado
/// nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
unsafe fn cqk6_comparar_situacao(nome: &str, this: usize, clube: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = achar(cqk6_d(this + 8), clube);
    if e == 0 {
        return original();
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x73, 3)], original, copia);
    PLACAR.registrar(igual, || format!("{}({:#x}, {:#x}) original={:x?} rust={:x?}", nome, this, clube, mo, mr));
}

unsafe extern "thiscall" fn comparar_situacao_na_c1a(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(1770);
    cqk6_comparar_situacao("situacao_na_c1a", this, clube, || o(this, clube, fase, atual, melhor, pior), || situacao_na_c1a(this, clube, fase, atual, melhor, pior))
}

unsafe extern "thiscall" fn comparar_situacao_na_c1b(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(1771);
    cqk6_comparar_situacao("situacao_na_c1b", this, clube, || o(this, clube, fase, atual, melhor, pior), || situacao_na_c1b(this, clube, fase, atual, melhor, pior))
}
