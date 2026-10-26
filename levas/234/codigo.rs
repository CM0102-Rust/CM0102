// ---------- 234a leva: o olheiro pergunta se o clube tem defensores, meias e atacantes que bastem (Database.cpp) ----------

/// Quantos dos 50 do elenco (+0xd7) contam para as três consultas da força do elenco pelo limite /65: pessoa não
/// nula com jogador (+0x61) da `posicao`, sem lesão grave (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364]
/// (com sinal) e habilidade atual (+5 do jogador, curto com sinal) acima do `limite`. A ordem dos testes é a da
/// original (a 0x62c400 só é chamada para os da posição). Contador de 8 bits (no máximo 50).
unsafe fn dbo1_contar(clube: usize, limite: i32, posicao: impl Fn(usize) -> bool) -> u8 {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let mut n = 0u8;
    for k in 0..50 {
        let p = rd(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || !posicao(j) || lesao(rd(0xb63d28), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) >= *(0xae2364 as *const i32) {
            continue;
        }
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limite {
            n += 1;
        }
    }
    n
}

/// O limite de habilidade das três: reputação do clube (+0x80, curto com sinal) / 65 + 40, entre 40 e `teto`.
unsafe fn dbo1_limite(clube: usize, teto: i32) -> i32 {
    (core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32 / 65 + 40).clamp(40, teto)
}

/// A média de gols por jogo do clube na competição (+0x57): a estatística do clube (COMP_STATS 0x4a4850, thiscall
/// no +8 da competição [[0xadadfc] + 4·id]); `None` sem competição, sem estatística ou sem jogos (+4); senão o byte
/// `campo` / jogos (inteiros sem sinal). A 0x4a4850 ordena a tabela da competição na primeira chamada (marca +0x18)
/// e guarda a competição em [0xadae10]: na comparação a original vem antes e a cópia acha a tabela já ordenada.
unsafe fn dbo1_gols_por_jogo(clube: usize, campo: usize) -> Option<i32> {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let comp = rd(clube + 0x57);
    if comp == 0 {
        return None;
    }
    let c = rd(rd(0xadadfc).wrapping_add(rd(comp).wrapping_mul(4)));
    if c == 0 || rd(c + 8) == 0 {
        return None;
    }
    let estatistica: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = estatistica(rd(c + 8), clube);
    if e == 0 {
        return None;
    }
    let jogos = *((e + 4) as *const u8) as i32;
    if jogos == 0 {
        return None;
    }
    Some(*((e + campo) as *const u8) as i32 / jogos)
}

/// { 0x549100 } (cdecl, Database.cpp): `club_has_enough_defenders(clube)`: clube nulo: 0. Se o clube toma mais de
/// 1,35 gol por jogo ([0x969588]; gols sofridos +0x6a da estatística / jogos): 0. Senão, 1 se mais de 2 zagueiros
/// (posição +0x11 = 20) passam do limite /65 (teto 145). Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day
/// 0x7b3e20, relatório de um clube).
unsafe extern "C" fn clube_tem_defensores(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    if let Some(v) = dbo1_gols_por_jogo(clube, 0x6a) {
        if v as f64 > *(0x969588 as *const f64) {
            return 0; // a defesa toma gols demais
        }
    }
    let limite = dbo1_limite(clube, 0x91);
    (dbo1_contar(clube, limite, |j| *((j + 0x11) as *const u8) == 20) > 2) as u8 // zagueiros (dbo)
}

unsafe extern "C" fn comparar_clube_tem_defensores(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(701)(clube);
    let n = clube_tem_defensores(clube);
    PLACAR.registrar(r == n, || format!("clube_tem_defensores({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549200 } (cdecl, Database.cpp): `club_has_enough_midfielders(clube)`: clube nulo: 0; senão 1 se mais de 2
/// meias (posição +0x14, +0x12 ou +0x13 = 20, nessa ordem) passam do limite /65 (teto 145). Só `al`. Só pelo olheiro.
unsafe extern "C" fn clube_tem_meias(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let limite = dbo1_limite(clube, 0x91);
    let meia = |j: usize| *((j + 0x14) as *const u8) == 20 || *((j + 0x12) as *const u8) == 20 || *((j + 0x13) as *const u8) == 20;
    (dbo1_contar(clube, limite, meia) > 2) as u8 // meias (dbo)
}

unsafe extern "C" fn comparar_clube_tem_meias(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(702)(clube);
    let n = clube_tem_meias(clube);
    PLACAR.registrar(r == n, || format!("clube_tem_meias({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x5492c0 } (cdecl, Database.cpp): `club_has_enough_strikers(clube)`: clube nulo: 0. Se o clube faz menos de
/// 1,5 gol por jogo ([0x968940]; gols marcados +0x69 da estatística / jogos): 0. Senão, 1 se mais de 1 atacante
/// (posição +0x15 = 20) passa do limite /65 (teto 150). Só `al`. Só pelo olheiro.
unsafe extern "C" fn clube_tem_atacantes(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    if let Some(v) = dbo1_gols_por_jogo(clube, 0x69) {
        if (v as f64) < *(0x968940 as *const f64) {
            return 0; // o ataque faz poucos gols
        }
    }
    let limite = dbo1_limite(clube, 0x96);
    (dbo1_contar(clube, limite, |j| *((j + 0x15) as *const u8) == 20) > 1) as u8 // atacantes (dbo)
}

unsafe extern "C" fn comparar_clube_tem_atacantes(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(703)(clube);
    let n = clube_tem_atacantes(clube);
    PLACAR.registrar(r == n, || format!("clube_tem_atacantes({:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 234a leva: o olheiro pergunta se o clube tem esforçados, goleiro em condições e meias que roubam a bola (Database.cpp) ----------

/// Quantos dos 50 do elenco (+0xd7) contam: pessoa não nula com jogador (+0x61) da `posicao`, sem lesão grave
/// (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5 do jogador, curto com
/// sinal) acima do `limite` e o `extra` (os atributos do jogador). Na ordem da original (a 0x62c400 só para os da
/// posição). Contador de 8 bits (no máximo 50).
unsafe fn dbo2_contar(clube: usize, limite: i32, posicao: impl Fn(usize) -> bool, extra: impl Fn(usize) -> bool) -> u8 {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let mut n = 0u8;
    for k in 0..50 {
        let p = rd(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || !posicao(j) || lesao(rd(0xb63d28), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) >= *(0xae2364 as *const i32) {
            continue;
        }
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limite && extra(j) {
            n += 1;
        }
    }
    n
}

/// A reputação do clube (+0x80, curto com sinal).
unsafe fn dbo2_reputacao(clube: usize) -> i32 {
    core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32
}

/// { 0x5493c0 } (cdecl, Database.cpp): `club_has_enough_hard_workers(clube)`: clube nulo: 0; senão 1 se mais de 4
/// jogadores (qualquer posição) passam do limite reputação/65 + 40 (de 40 a 140) com o atributo +0x44 (byte com
/// sinal; o mesmo que a 0x549850 usa) acima de 14. Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day 0x7b3e20,
/// relatório de um clube).
unsafe extern "C" fn clube_tem_esforcados(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let limite = (dbo2_reputacao(clube) / 65 + 40).clamp(40, 0x8c);
    let n = dbo2_contar(clube, limite, |_| true, |j| *((j + 0x44) as *const i8) > 14);
    (n > 4) as u8 // esforcados (dbo)
}

unsafe extern "C" fn comparar_clube_tem_esforcados(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(704)(clube);
    let n = clube_tem_esforcados(clube);
    PLACAR.registrar(r == n, || format!("clube_tem_esforcados({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549470 } (cdecl, Database.cpp): `club_has_fit_goalkeeper(clube)`: 1 assim que um dos 50 do elenco (+0xd7) é
/// pessoa com jogador (+0x61) goleiro (+0xf = 20), sem lesão grave (0x62c400 ≠ 2) e id < [0xae2364]; nenhum ou clube
/// nulo: 0. Sem limite de habilidade. Só `al`. Só pelo olheiro.
unsafe extern "C" fn clube_tem_goleiro(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    for k in 0..50 {
        let p = rd(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || *((j + 0xf) as *const u8) != 20 || lesao(rd(0xb63d28), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) < *(0xae2364 as *const i32) {
            return 1; // goleiro em condicoes (dbo)
        }
    }
    0
}

unsafe extern "C" fn comparar_clube_tem_goleiro(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(705)(clube);
    let n = clube_tem_goleiro(clube);
    PLACAR.registrar(r == n, || format!("clube_tem_goleiro({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549850 } (cdecl, Database.cpp): `club_has_ball_winning_midfielders(clube)`: clube nulo: 0; senão 1 se mais
/// de 1 meia (posição +0x14, +0x12 ou +0x13 = 20) sem lesão grave passa do limite reputação/75 + 25 (de 40 a 135)
/// com os atributos +0x44 e +0x3e (bytes com sinal) acima de reputação/500 + 5 (no máximo 14). Só `al`. Só pelo
/// olheiro.
unsafe extern "C" fn clube_tem_meias_que_roubam(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let rep = dbo2_reputacao(clube);
    let limite = (rep / 75 + 25).clamp(40, 0x87);
    let a = (rep / 500 + 5).min(14);
    let meia = |j: usize| *((j + 0x14) as *const u8) == 20 || *((j + 0x12) as *const u8) == 20 || *((j + 0x13) as *const u8) == 20;
    let n = dbo2_contar(clube, limite, meia, |j| *((j + 0x44) as *const i8) as i32 > a && *((j + 0x3e) as *const i8) as i32 > a);
    (n > 1) as u8 // meias que roubam (dbo)
}

unsafe extern "C" fn comparar_clube_tem_meias_que_roubam(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(706)(clube);
    let n = clube_tem_meias_que_roubam(clube);
    PLACAR.registrar(r == n, || format!("clube_tem_meias_que_roubam({:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 234a leva: o olheiro pergunta se faltam ao clube atacantes e zagueiros bons no alto e zagueiros rápidos (Database.cpp) ----------

/// Quantos dos 50 do elenco (+0xd7) são pessoa com jogador (+0x61) na `posicao` (byte = 20), sem lesão grave
/// (INJURY 0x62c400 em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5, curto com sinal)
/// acima de reputação/75 + 25 (de 40 a 135) e o `atributo` (byte com sinal) acima de reputação/500 + 5 (no máximo
/// `teto`). Reputação: +0x80 do clube, curto com sinal. Na ordem da original. Contador de 8 bits (no máximo 50).
unsafe fn dbo3_contar(clube: usize, posicao: usize, atributo: usize, teto: i32) -> u8 {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let rep = core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32;
    let limite = (rep / 75 + 25).clamp(40, 0x87);
    let a = (rep / 500 + 5).min(teto);
    let mut n = 0u8;
    for k in 0..50 {
        let p = rd(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || *((j + posicao) as *const u8) != 20 || lesao(rd(0xb63d28), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) >= *(0xae2364 as *const i32) {
            continue;
        }
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limite && *((j + atributo) as *const i8) as i32 > a {
            n += 1;
        }
    }
    n
}

/// { 0x5494d0 } (cdecl, Database.cpp): `club_lacks_aerial_strikers(clube)`: clube nulo: 0; senão 1 se no máximo 1
/// atacante (posição +0x15) passa dos limites com o atributo +0x2e (o jogo pelo alto) acima de reputação/500 + 5 (no
/// máximo 13). Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day 0x7b3e20, relatório de um clube).
unsafe extern "C" fn faltam_atacantes_pelo_alto(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    (dbo3_contar(clube, 0x15, 0x2e, 13) <= 1) as u8 // atacantes pelo alto (dbo)
}

unsafe extern "C" fn comparar_faltam_atacantes_pelo_alto(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(707)(clube);
    let n = faltam_atacantes_pelo_alto(clube);
    PLACAR.registrar(r == n, || format!("faltam_atacantes_pelo_alto({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x5495b0 } (cdecl, Database.cpp): `club_lacks_aerial_defenders(clube)`: clube nulo: 0; senão 1 se no máximo 1
/// zagueiro (posição +0x11) passa dos limites com o atributo +0x2e acima de reputação/500 + 5 (no máximo 13). Só
/// `al`. Só pelo olheiro.
unsafe extern "C" fn faltam_zagueiros_pelo_alto(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    (dbo3_contar(clube, 0x11, 0x2e, 13) <= 1) as u8 // zagueiros pelo alto (dbo)
}

unsafe extern "C" fn comparar_faltam_zagueiros_pelo_alto(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(708)(clube);
    let n = faltam_zagueiros_pelo_alto(clube);
    PLACAR.registrar(r == n, || format!("faltam_zagueiros_pelo_alto({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549690 } (cdecl, Database.cpp): `club_lacks_fast_defenders(clube)`: clube nulo: 0; senão 1 se no máximo 1
/// zagueiro (posição +0x11) passa dos limites com o atributo +0x36 (a velocidade) acima de reputação/500 + 5 (no
/// máximo 10). Só `al`. Só pelo olheiro.
unsafe extern "C" fn faltam_zagueiros_rapidos(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    (dbo3_contar(clube, 0x11, 0x36, 10) <= 1) as u8 // zagueiros rapidos (dbo)
}

unsafe extern "C" fn comparar_faltam_zagueiros_rapidos(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(709)(clube);
    let n = faltam_zagueiros_rapidos(clube);
    PLACAR.registrar(r == n, || format!("faltam_zagueiros_rapidos({:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 234a leva: o olheiro pergunta se o clube tem atacante rápido e dois meias criativos, e a faixa de força do elenco (Database.cpp) ----------

/// Quantos dos 50 do elenco (+0xd7) são pessoa com jogador (+0x61) da `posicao`, sem lesão grave (INJURY 0x62c400
/// em [0xb63d28] ≠ 2), id (+0) < [0xae2364] (com sinal), habilidade atual (+5, curto com sinal) acima de
/// reputação/75 + 25 (de 40 a 135) e o `extra` com o limite de atributo reputação/500 + 5 (no máximo `teto`).
/// Reputação: +0x80 do clube, curto com sinal. Na ordem da original. Contador de 8 bits (no máximo 50).
unsafe fn dbo4_contar(clube: usize, teto: i32, posicao: impl Fn(usize) -> bool, extra: impl Fn(usize, i32) -> bool) -> u8 {
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let rep = core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32;
    let limite = (rep / 75 + 25).clamp(40, 0x87);
    let a = (rep / 500 + 5).min(teto);
    let mut n = 0u8;
    for k in 0..50 {
        let p = rd(clube + 0xd7 + 4 * k);
        if p == 0 {
            continue;
        }
        let j = rd(p + 0x61);
        if j == 0 || !posicao(j) || lesao(rd(0xb63d28), p) == 2 {
            continue;
        }
        if core::ptr::read_unaligned(p as *const i32) >= *(0xae2364 as *const i32) {
            continue;
        }
        if core::ptr::read_unaligned((j + 5) as *const i16) as i32 > limite && extra(j, a) {
            n += 1;
        }
    }
    n
}

/// { 0x549770 } (cdecl, Database.cpp): `club_has_fast_striker(clube)`: clube nulo: 0; senão 1 se algum atacante
/// (posição +0x15 = 20) passa dos limites com o atributo +0x36 (a velocidade, byte com sinal) acima de
/// reputação/500 + 5 (no máximo 17). Só `al`. Só pelo olheiro (PLAYER_SEARCH::run_scout_day 0x7b3e20, relatório de
/// um clube).
unsafe extern "C" fn clube_tem_atacante_rapido(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let n = dbo4_contar(clube, 17, |j| *((j + 0x15) as *const u8) == 20, |j, a| *((j + 0x36) as *const i8) as i32 > a);
    (n > 0) as u8 // atacante rapido (dbo)
}

unsafe extern "C" fn comparar_clube_tem_atacante_rapido(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(710)(clube);
    let n = clube_tem_atacante_rapido(clube);
    PLACAR.registrar(r == n, || format!("clube_tem_atacante_rapido({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549940 } (cdecl, Database.cpp): `club_has_two_creative_midfielders(clube)`: clube nulo: 0; senão 1 se mais de
/// 1 meia (posição +0x14, +0x12 ou +0x13 = 20) passa dos limites com os atributos +0x43 e +0x37 (bytes com sinal)
/// acima de reputação/500 + 5 (no máximo 14). Só `al`. Só pelo olheiro.
unsafe extern "C" fn clube_tem_meias_criativos(clube: usize) -> u8 {
    if clube == 0 {
        return 0;
    }
    let meia = |j: usize| *((j + 0x14) as *const u8) == 20 || *((j + 0x12) as *const u8) == 20 || *((j + 0x13) as *const u8) == 20;
    let n = dbo4_contar(clube, 14, meia, |j, a| *((j + 0x43) as *const i8) as i32 > a && *((j + 0x37) as *const i8) as i32 > a);
    (n > 1) as u8 // meias criativos (dbo)
}

unsafe extern "C" fn comparar_clube_tem_meias_criativos(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(711)(clube);
    let n = clube_tem_meias_criativos(clube);
    PLACAR.registrar(r == n, || format!("clube_tem_meias_criativos({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x549a30 } (cdecl, Database.cpp): `club_squad_strength_band(clube, outro)`: a faixa de força do elenco, 0..6.
/// Clube nulo: 3. Junta os do elenco (+0xd7) com jogador (+0x61) e id < [0xae2364] (lido uma vez) num vetor local de
/// 50, ordena pelo qsort da CRT (0x945333) com a comparação 0x549c30 (pelo endereço do jogo) e soma a habilidade atual
/// (+5 do jogador) dos primeiros min(n − 1, 16). Sem nenhum somado: pela reputação (+0x80; > 7250: 6, > 5750: 5,
/// > 4750: 4, > 3250: 3, > 2250: 2, senão 1); senão pela média (> 155: 6, > 135: 5, > 110: 4, > 85: 3, > 60: 2,
/// > 40: 1, senão 0). Com `outro` (o clube do olheiro): reputação dele > 7250 tira 2 se a do clube < 4250 e a faixa
/// > 2, e mais 1 se a do clube < 7250 e a faixa > 0; senão, reputação dele < 4250 soma 1 se a faixa < 5. Na mesma
/// competição (+0x57 igual), a diferença de reputação (16 bits) > 1250 tira 1 (faixa > 0, byte +4 da tabela
/// [0xae2d14] de 6 bytes por id: do outro ≥ 0 e do clube ≤ 0); < 1250 soma 1 (faixa < 5, do clube ≥ 0 e do outro
/// ≤ 0). No fim, entre 0 e 6. Só `al`. Só pelo olheiro.
unsafe extern "C" fn faixa_de_forca_do_elenco(clube: usize, outro: usize) -> u8 {
    if clube == 0 {
        return 3;
    }
    let rd = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let rd16 = |a: usize| core::ptr::read_unaligned(a as *const i16);
    let id = |a: usize| core::ptr::read_unaligned(a as *const i32);
    let total = *(0xae2364 as *const i32);
    let mut v = [0usize; 50];
    let mut n = 0usize;
    for k in 0..50 {
        let p = rd(clube + 0xd7 + 4 * k);
        if p != 0 && rd(p + 0x61) != 0 && id(p) < total {
            v[n] = p;
            n += 1;
        }
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(v.as_mut_ptr() as usize, n as u32, 4, 0x549c30); // os melhores primeiro (dbo)
    let m = (n as i32 - 1).clamp(0, 16) as usize;
    let mut soma = 0i32;
    for &p in &v[..m] {
        soma = soma.wrapping_add(rd16(rd(p + 0x61) + 5) as i32);
    }
    let rep = rd16(clube + 0x80);
    let mut f: i8 = if m == 0 {
        match rep {
            r if r > 0x1c52 => 6,
            r if r > 0x1676 => 5,
            r if r > 0x128e => 4,
            r if r > 0xcb2 => 3,
            r => (r > 0x8ca) as i8 + 1,
        }
    } else {
        match soma / m as i32 {
            x if x > 0x9b => 6,
            x if x > 0x87 => 5,
            x if x > 0x6e => 4,
            x if x > 0x55 => 3,
            x if x > 0x3c => 2,
            x => (x > 0x28) as i8,
        }
    };
    if outro != 0 {
        let ro = rd16(outro + 0x80);
        if ro > 0x1c52 {
            if rep < 0x109a && f > 2 {
                f -= 2;
            }
            if rep < 0x1c52 && f > 0 {
                f -= 1;
            }
        } else if ro < 0x109a && f < 5 {
            f += 1; // o olheiro de clube pequeno ve o clube mais forte (dbo)
        }
        if rd(outro + 0x57) == rd(clube + 0x57) {
            let d = ro.wrapping_sub(rep);
            let tab = rd(0xae2d14);
            let marca = |c: usize| *(tab.wrapping_add((id(c).wrapping_mul(6) as usize).wrapping_add(4)) as *const i8);
            if d > 0x4e2 && f > 0 && marca(outro) >= 0 && marca(clube) <= 0 {
                f -= 1;
            } else if d < 0x4e2 && f < 5 && marca(clube) >= 0 && marca(outro) <= 0 {
                f += 1;
            }
        }
    }
    f.clamp(0, 6) as u8
}

unsafe extern "C" fn comparar_faixa_de_forca_do_elenco(clube: usize, outro: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(712)(clube, outro);
    let n = faixa_de_forca_do_elenco(clube, outro);
    PLACAR.registrar(r == n, || format!("faixa_de_forca_do_elenco({:x}, {:x}) original={} rust={}", clube, outro, r, n));
    r
}

// ---------- 234a leva: a reputação do clube para a pessoa e quantos favoritos e rejeitados dela estão no clube (Database.cpp) ----------

unsafe fn dbp_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn dbp_i16(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O clube do contrato da `pessoa` como a 0x545e90 o acha: o contrato pela 0x4cf5a0 (this = [0xae1050]), gravado
/// em [0xae1080] como na original; com contrato, o clube de id +4 na tabela [0xae23b4] (581 bytes); sem: 0.
unsafe fn dbp_clube_do_contrato(pessoa: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato(dbp_u32(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    if c == 0 {
        return 0;
    }
    dbp_u32(0xae23b4).wrapping_add(dbp_u32(c + 4).wrapping_mul(581))
}

/// +500 se o `clube` está entre os três clubes favoritos da pessoa (preferências +4/+8/+0xc), −500 se entre os três
/// rejeitados (+0x10/+0x14/+0x18), senão 0; sem preferências: 0.
unsafe fn dbp_preferencia_do_clube(clube: usize, pessoa: usize) -> i32 {
    let pref = dbp_u32(pessoa + 0x65);
    if pref == 0 {
        return 0;
    }
    if (0..3).any(|k| dbp_u32(pref + 4 + 4 * k) == clube) {
        500
    } else if (0..3).any(|k| dbp_u32(pref + 0x10 + 4 * k) == clube) {
        -500
    } else {
        0
    }
}

/// { 0x545e90 } (cdecl, Database.cpp): `club_reputation_for_staff(clube, pessoa)`: a reputação do clube (+0x80) vista
/// pela pessoa. Sem divisão (+0x57): a reputação. Com a liga da divisão ([0xadadfc][id]) e a reputação da divisão
/// (+0x69) × 500 acima da do clube: a posição (0x4a4850 na tabela +8 da liga; sem linha: 0) escalada a 20 clubes
/// (+0x3e) dá o peso da tabela 0x9b59c0; com menos de 20 jogos (+4 da linha) mistura com o peso da posição anterior
/// (+0x5f, se +0x5b = +0x57; senão 1,3) × 0,05; se a pessoa joga (+0x61) e o contrato é com o clube, soma a média
/// 0x7c1e30(dados, 6, 20, peso) e divide por 2, e ganha ±500 pelas preferências; o peso vira x = 14p − 18 entre 0 e
/// 10 e o resultado é ((10 − x) × reputação + x × divisão × 500 (16 bits)) / 10 truncado, mais o ±500. Senão: a
/// reputação ±500 (pessoa que joga, com contrato no clube). Os chamadores leem só `ax`; o resto do eax segue a
/// original (o registro que sobrou em cada caminho). Grava [0xae1080] (o contrato achado). No diário das pessoas
/// (game_daily_staff_update 0x5ce2d0, duas chamadas).
unsafe extern "C" fn reputacao_do_clube_para_pessoa(clube: usize, pessoa: usize) -> u32 {
    let rep = dbp_i16(clube + 0x80);
    let comp = dbp_u32(clube + 0x57);
    if comp == 0 {
        return rep as u16 as u32;
    }
    let liga = dbp_u32(dbp_u32(0xadadfc).wrapping_add(dbp_u32(comp).wrapping_mul(4)));
    let divisao = dbp_i16(comp + 0x69);
    let alto = (divisao as i32).wrapping_mul(500);
    if liga != 0 && alto > rep as i32 {
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
        let e = busca(dbp_u32(liga + 8), clube);
        if e == 0 {
            return 0;
        }
        let n = dbp_i16(liga + 0x3e) as i32;
        let peso = |k: i32| f64_em(0x9b59c0 + 8 * k.clamp(0, 19) as usize);
        let mut posicao = *((e + 0x73) as *const u8) as i32 - 1;
        if n != 0 && n != 20 {
            posicao = posicao * 20 / n;
        }
        let mut v = peso(posicao);
        let jogos = *((e + 4) as *const u8) as i32;
        if jogos < 20 {
            let antes = *((clube + 0x5f) as *const i8) as i32;
            let w = if dbp_u32(clube + 0x5b) != comp {
                f64_em(0x968de8)
            } else if n != 0 && n != 20 {
                peso((antes * 20 - 20) / n)
            } else if (1..21).contains(&antes) {
                peso(antes - 1)
            } else {
                f64_em(0x968de8)
            };
            v = ((20 - jogos) as f64 * w + jogos as f64 * v) * f64_em(0x967888);
        }
        let mut bonus = 0;
        if pessoa != 0 && dbp_u32(pessoa + 0x61) != 0 && dbp_clube_do_contrato(pessoa) == clube {
            let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
            let m = media(dbp_u32(0xb74350), dbp_u32(pessoa + 0x61), 6, 0x14, v as f32);
            v = (m + v) * f64_em(0x968968);
            bonus = dbp_preferencia_do_clube(clube, pessoa);
        }
        let mut x = v * f64_em(0x969558) - f64_em(0x969550); // o peso entre 0 e 10
        let (baixo, teto) = (f64_em(0x9689a0), f64_em(0x9688b0));
        if baixo > x {
            x = baixo;
        } else if !(teto >= x) {
            x = teto; // fcomp: acima do teto ou NaN
        }
        let cx = (divisao as i32).wrapping_mul(500) as i16 as f64; // imul cx, cx, 0x1f4: 16 bits
        let r = ((teto - x) * rep as f64 + cx * x) * f64_em(0x967880);
        return (ftol(r) as u32).wrapping_add(bonus as u32);
    }
    // sem liga, ou o clube já à altura da divisão: a reputação ±500; a parte alta do eax é o que sobrou
    let mut eax = if liga == 0 { comp } else { alto as usize };
    let mut bonus = 0;
    if pessoa != 0 {
        eax = dbp_u32(pessoa + 0x61);
        if eax != 0 {
            eax = dbp_clube_do_contrato(pessoa);
            if eax == clube {
                eax = dbp_u32(pessoa + 0x65);
                if eax != 0 {
                    bonus = dbp_preferencia_do_clube(clube, pessoa); // os clubes da pessoa
                }
            }
        }
    }
    ((eax as u32 & 0xffff_0000) | rep as u16 as u32).wrapping_add(bonus as u32)
}

unsafe extern "C" fn comparar_reputacao_do_clube_para_pessoa(clube: usize, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(713);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(clube, pessoa),
        || reputacao_do_clube_para_pessoa(clube, pessoa));
    // os chamadores leem só o ax (movsx)
    PLACAR.registrar(igual && r as u16 == n as u16, || format!("reputacao_do_clube_para_pessoa({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}", clube, pessoa, r, n, mo, mr));
    r
}

/// Quantas das três pessoas da lista de preferências em `base` (+0x65 da pessoa) têm o clube (+0x39) igual a `clube`;
/// pessoa, preferências ou clube nulos: 0. Só `al`.
unsafe fn dbp_contar_no_clube(pessoa: usize, clube: usize, base: usize) -> u8 {
    if pessoa == 0 {
        return 0;
    }
    let pref = dbp_u32(pessoa + 0x65);
    if pref == 0 || clube == 0 {
        return 0;
    }
    let mut n = 0u8;
    for k in 0..3 {
        let p = dbp_u32(pref + base + 4 * k);
        if p != 0 && dbp_u32(p + 0x39) == clube {
            n = n.wrapping_add(1);
        }
    }
    n
}

/// { 0x547c90 } (cdecl, Database.cpp): `staff_count_disliked_staff_at_club(pessoa, clube)`: quantas das três pessoas
/// que a pessoa rejeita (preferências +0x28/+0x2c/+0x30) estão no clube. No diário das pessoas (0x5ce2d0).
unsafe extern "C" fn contar_rejeitados_no_clube(pessoa: usize, clube: usize) -> u8 {
    dbp_contar_no_clube(pessoa, clube, 0x28) // os rejeitados
}

unsafe extern "C" fn comparar_contar_rejeitados_no_clube(pessoa: usize, clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(714)(pessoa, clube);
    let n = contar_rejeitados_no_clube(pessoa, clube);
    PLACAR.registrar(r == n, || format!("contar_rejeitados_no_clube({:x}, {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}

/// { 0x547ce0 } (cdecl, Database.cpp): `staff_count_favoured_staff_at_club(pessoa, clube)`: quantas das três pessoas
/// favoritas da pessoa (preferências +0x1c/+0x20/+0x24) estão no clube. No diário das pessoas (0x5ce2d0).
unsafe extern "C" fn contar_favoritos_no_clube(pessoa: usize, clube: usize) -> u8 {
    dbp_contar_no_clube(pessoa, clube, 0x1c) // os favoritos
}

unsafe extern "C" fn comparar_contar_favoritos_no_clube(pessoa: usize, clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> u8>(715)(pessoa, clube);
    let n = contar_favoritos_no_clube(pessoa, clube);
    PLACAR.registrar(r == n, || format!("contar_favoritos_no_clube({:x}, {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}

// ---------- 234a leva: tirar das preferências: o clube favorito da pessoa, a pessoa favorita do clube e a pessoa favorita da pessoa (Database.cpp) ----------

/// Zera a primeira das três posições (`reg`, +4, +8) que tem `alvo` (a ordem da original: achou, zera e sai).
unsafe fn dbp_tirar_de_tres(reg: usize, alvo: usize) {
    for k in 0..3 {
        let a = (reg + 4 * k) as *mut usize;
        if core::ptr::read_unaligned(a) == alvo {
            core::ptr::write_unaligned(a, 0);
            return;
        }
    }
}

/// As preferências (+0x65) da pessoa não nula; pessoa nula: 0 (o eax que as 0x547c10/0x547c50/0x547f50 devolvem).
unsafe fn dbp_preferencias(pessoa: usize) -> usize {
    if pessoa == 0 {
        0
    } else {
        core::ptr::read_unaligned((pessoa + 0x65) as *const usize)
    }
}

/// { 0x547f50 } (cdecl, Database.cpp): `staff_remove_favoured_club(pessoa, clube)`: tira o clube dos três clubes
/// favoritos da pessoa (preferências +4/+8/+0xc, o primeiro que bate). Pessoa, preferências ou clube nulos: nada.
/// Devolve o eax da original (as preferências; pessoa nula: 0). No diário das pessoas (0x5ce2d0), quando o
/// apego ao clube acaba.
unsafe extern "C" fn tirar_clube_favorito(pessoa: usize, clube: usize) -> usize {
    let pref = dbp_preferencias(pessoa);
    if pref != 0 && clube != 0 {
        dbp_tirar_de_tres(pref + 4, clube); // os clubes favoritos
    }
    pref
}

unsafe extern "C" fn comparar_tirar_clube_favorito(pessoa: usize, clube: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> usize>(716);
    let pref = dbp_preferencias(pessoa);
    if pref == 0 {
        return o(pessoa, clube); // não escreve nada
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(pref + 4, 12)], || o(pessoa, clube), || tirar_clube_favorito(pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("tirar_clube_favorito({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// { 0x547bd0 } (cdecl, Database.cpp): `club_remove_favoured_staff(clube, pessoa)`: tira a pessoa das três pessoas
/// favoritas do clube (+0x9b/+0x9f/+0xa3, o primeiro que bate). Clube ou pessoa nulos: nada. Devolve o clube (o eax
/// da original). Nas audiências das disputas (DISPUTE::process_hearings 0x569ed0).
unsafe extern "C" fn clube_tirar_pessoa_favorita(clube: usize, pessoa: usize) -> usize {
    if clube != 0 && pessoa != 0 {
        dbp_tirar_de_tres(clube + 0x9b, pessoa); // os favoritos do clube
    }
    clube
}

unsafe extern "C" fn comparar_clube_tirar_pessoa_favorita(clube: usize, pessoa: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> usize>(717);
    if clube == 0 {
        return o(clube, pessoa);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(clube + 0x9b, 12)], || o(clube, pessoa), || clube_tirar_pessoa_favorita(clube, pessoa));
    PLACAR.registrar(igual && r == n, || format!("clube_tirar_pessoa_favorita({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", clube, pessoa, r, n, mo, mr));
    r
}

/// { 0x547c10 } (cdecl, Database.cpp): `staff_remove_favoured_staff(pessoa, alvo)`: tira o alvo das três pessoas
/// favoritas da pessoa (preferências +0x1c/+0x20/+0x24, o primeiro que bate). Pessoa, preferências ou alvo nulos:
/// nada. Devolve o eax da original (as preferências; pessoa nula: 0). Nas disputas (0x569ed0), nas relações do
/// elenco (game_update_squad_relationships 0x5d6d10, diária), na volta da lesão (0x630ee0), no MANAGER_MANAGER
/// (0x697a7a) e na mídia (0x75a340).
unsafe extern "C" fn tirar_pessoa_favorita(pessoa: usize, alvo: usize) -> usize {
    let pref = dbp_preferencias(pessoa);
    if pref != 0 && alvo != 0 {
        dbp_tirar_de_tres(pref + 0x1c, alvo); // as pessoas favoritas
    }
    pref
}

unsafe extern "C" fn comparar_tirar_pessoa_favorita(pessoa: usize, alvo: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> usize>(718);
    let pref = dbp_preferencias(pessoa);
    if pref == 0 {
        return o(pessoa, alvo);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(pref + 0x1c, 12)], || o(pessoa, alvo), || tirar_pessoa_favorita(pessoa, alvo));
    PLACAR.registrar(igual && r == n, || format!("tirar_pessoa_favorita({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", pessoa, alvo, r, n, mo, mr));
    r
}

// ---------- 234a leva: a pessoa rejeitada que sai das preferências e as pessoas favoritas num vetor (Database.cpp) ----------

/// { 0x547c50 } (cdecl, Database.cpp): `staff_remove_disliked_staff(pessoa, alvo)`: zera a primeira das três pessoas
/// rejeitadas da pessoa (preferências +0x28/+0x2c/+0x30) que é o alvo. Pessoa, preferências ou alvo nulos: nada.
/// Devolve o eax da original (as preferências; pessoa nula: 0). Nas relações do elenco (game_update_squad_relationships
/// 0x5d6d10, diária), na volta da lesão (0x630ee0) e na mídia (0x7414ac, 0x75a340).
unsafe extern "C" fn tirar_pessoa_rejeitada(pessoa: usize, alvo: usize) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let pref = core::ptr::read_unaligned((pessoa + 0x65) as *const usize);
    if pref != 0 && alvo != 0 {
        let reg = pref + 0x28; // as pessoas rejeitadas
        for k in 0..3 {
            let a = (reg + 4 * k) as *mut usize;
            if core::ptr::read_unaligned(a) == alvo {
                core::ptr::write_unaligned(a, 0);
                break;
            }
        }
    }
    pref
}

unsafe extern "C" fn comparar_tirar_pessoa_rejeitada(pessoa: usize, alvo: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> usize>(719);
    let pref = if pessoa == 0 { 0 } else { core::ptr::read_unaligned((pessoa + 0x65) as *const usize) };
    if pref == 0 {
        return o(pessoa, alvo); // não escreve nada
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(pref + 0x28, 12)], || o(pessoa, alvo), || tirar_pessoa_rejeitada(pessoa, alvo));
    PLACAR.registrar(igual && r == n, || format!("tirar_pessoa_rejeitada({:x}, {:x}) {:x}/{:x} {:x?} x {:x?}", pessoa, alvo, r, n, mo, mr));
    r
}

/// { 0x548090 } (cdecl, Database.cpp): `staff_get_favoured_staff(pessoa, vetor, n)`: zera o byte `n` e, com pessoa,
/// preferências e vetor não nulos, copia as pessoas favoritas não nulas (preferências +0x1c, +0x20, +0x24, nessa
/// ordem) para `vetor[n]`, somando 1 a `n` a cada uma (a primeira vai direto em `vetor[0]`). Devolve `n` (o eax da
/// original). No MANAGER_MANAGER: a opinião da diretoria sobre o elenco (0x693850) e a reação dela à partida
/// (0x6aaee0).
unsafe extern "C" fn pessoas_favoritas(pessoa: usize, vetor: usize, n: *mut i8) -> *mut i8 {
    *n = 0;
    if pessoa == 0 {
        return n;
    }
    let pref = |p: usize| core::ptr::read_unaligned((p + 0x65) as *const usize);
    if pref(pessoa) == 0 || vetor == 0 {
        return n;
    }
    for (k, d) in [0x1cusize, 0x20, 0x24].into_iter().enumerate() { // as três favoritas
        let p = core::ptr::read_unaligned((pref(pessoa) + d) as *const usize);
        if p != 0 {
            let i = if k == 0 { 0 } else { *n as isize };
            core::ptr::write_unaligned((vetor as isize + 4 * i) as *mut usize, p);
            *n = (*n).wrapping_add(1);
        }
    }
    n
}

unsafe extern "C" fn comparar_pessoas_favoritas(pessoa: usize, vetor: usize, n: *mut i8) -> *mut i8 {
    let o = orig::<unsafe extern "C" fn(usize, usize, *mut i8) -> *mut i8>(720);
    let com_vetor = [(vetor, 12), (n as usize, 1)];
    let regioes: &[(usize, usize)] = if vetor == 0 { &com_vetor[1..] } else { &com_vetor };
    let (r, x, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(regioes, || o(pessoa, vetor, n), || pessoas_favoritas(pessoa, vetor, n));
    PLACAR.registrar(igual && r == x, || format!("pessoas_favoritas({:x}, {:x}) {:x?} x {:x?}", pessoa, vetor, mo, mr));
    r
}
