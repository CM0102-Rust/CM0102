// ---------- %%LEVA%%a leva: a Liga dos Campeões asiática: os 32 classificados das duas zonas, a fase da zona oeste e as posições previstas (asia_club_champ.cpp) ----------

unsafe fn cqu10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqu10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqu10_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A chave da nação do clube ([[clube+0x53]], dword); sem clube ou sem nação, −1.
unsafe fn cqu10_nacao(c: usize) -> i32 {
    if c == 0 || cqu10_d(c + 0x53) == 0 {
        return -1;
    }
    cqu10_d(cqu10_d(c + 0x53)) as i32
}

/// A zona leste (23 nações, na ordem da original): Bangladesh, Brunei, China, Hong Kong, Índia, Indonésia, Japão,
/// Camboja, Laos, Macau, Malásia, Maldivas, Mianmar, Nepal, Coreia do Norte, Singapura, Coreia do Sul, Sri Lanka,
/// Taiwan, Tailândia, Filipinas, Vietnã e Paquistão (as globais das chaves das nações).
const CQU10_LESTE: [usize; 23] = [0x9cf238, 0x9cf264, 0x9cf298, 0x9cf340, 0x9cf34c, 0x9cf350, 0x9cf36c, 0x9cf374, 0x9cf388, 0x9cf3ac, 0x9cf3b8,
    0x9cf3bc, 0x9cf3e0, 0x9cf3ec, 0x9cf404, 0x9cf46c, 0x9cf484, 0x9cf48c, 0x9cf4b8, 0x9cf4c4, 0x9cf42c, 0x9cf508, 0x9cf410];

/// A zona oeste (19 nações): Afeganistão, Bahrein, Irã, Iraque, Jordânia, Cazaquistão, Kuwait, Quirguistão, Líbano,
/// Omã, Palestina, Catar, Arábia Saudita, Síria, Tadjiquistão, Turcomenistão, Emirados, Uzbequistão e Iêmen.
const CQU10_OESTE: [usize; 19] = [0x9cf200, 0x9cf234, 0x9cf354, 0x9cf358, 0x9cf370, 0x9cf378, 0x9cf380, 0x9cf384, 0x9cf390, 0x9cf40c, 0x9cf414,
    0x9cf43c, 0x9cf458, 0x9cf4b0, 0x9cf4bc, 0x9cf4e0, 0x9cf4e4, 0x9cf4fc, 0x9cf518];

/// O índice da chave `n` numa das zonas (a primeira igual), como o laço de bytes com sinal da original.
unsafe fn cqu10_indice(zona: &[usize], n: i32) -> Option<usize> {
    zona.iter().position(|&g| cqu10_d(g) as i32 == n)
}

/// O campeão (ou, se ele já está na lista, o vice) da liga `chave` (global da chave da competição), se a nação
/// `nacao` (global da chave) tem o byte +0x11c ligado no registro de 0x122 bytes ([0xae23a8]).
unsafe fn cqu10_campeao_da_liga(this: usize, nacao: usize, chave: usize) -> Option<usize> {
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let n = cqu10_d(nacao);
    if *((cqu10_d(0xae23a8)).wrapping_add(n.wrapping_mul(0x122)).wrapping_add(0x11c) as *const u8) == 0 {
        return None;
    }
    let liga = cqu10_d(0xae23d0).wrapping_add(cqu10_d(chave).wrapping_mul(0x6b));
    let lista = cqu10_d(this + 0xb2);
    let mut c = colocado(liga, 0); // cqu10: o campeão da liga
    if c == cqu10_d(lista) || c == cqu10_d(lista + 0x40) {
        c = colocado(liga, 1);
    }
    Some(c)
}

/// { 0x40b950 } (thiscall, ret 0, asia_club_champ.cpp, ~1,4 KB de pilha 0x1594): `AsiaClubChamp::select_qualified_
/// clubs()`: +0xb6 = 32 e a lista +0xb2 de 32 vagas (calloc 0x945501(0x20, 4): push 4; push 0x20; add esp, 8 -> cdecl;
/// sem memória, a caixa da linha 0x289 e 0). O campeão mais recente da própria copa (0x4afd80([+4], 0): push 0; push
/// comp; add esp, 8 -> cdecl) vai para a vaga 0 se a nação dele é da zona leste, senão para a 16. Depois, numa lista
/// local de até 1.200 clubes: o campeão da J1 (ou o vice, se o campeão já está na vaga 0 ou 16), se o Japão tem o
/// byte +0x11c, e o da liga coreana do mesmo jeito; e todos os clubes da Ásia (nação com continente [0x9cfa0c]) fora
/// das vagas 0 e 16. A lista é ordenada e embaralhada (0x4afbd0(lista, n − 1, 40): 3 pushes; add esp, 0xc -> cdecl).
/// Na ordem dela, cada clube cuja nação ainda não teve vaga entra na primeira vaga livre da sua zona (0..15 leste,
/// 16..31 oeste), enquanto a última vaga da zona (15 ou 31) estiver livre; a nação fica marcada mesmo com a zona
/// cheia. Devolve 1.
unsafe extern "thiscall" fn cqu10_asia_classificados(this: usize) -> u32 {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    cqu10_pd(this + 0xb6, 0x20);
    let lista = calloc(0x20, 4);
    cqu10_pd(this + 0xb2, lista);
    if lista == 0 {
        show_error_at(0x987a40, 0x289);
        return 0;
    }
    let atual = colocado(cqu10_d(this + 4), 0);
    if cqu10_indice(&CQU10_LESTE, cqu10_nacao(atual)).is_some() {
        cqu10_pd(lista, atual);
    } else {
        cqu10_pd(lista + 0x40, atual);
    }
    let mut clubes = [0usize; 0x4b0];
    let mut n = 0usize;
    if let Some(c) = cqu10_campeao_da_liga(this, 0x9cf36c, 0x9cf65c) {
        crate::comparacao::ramo("cqu_10 clube da j1");
        clubes[0] = c;
        n = 1;
    }
    if let Some(c) = cqu10_campeao_da_liga(this, 0x9cf484, 0x9cfa00) {
        clubes[n] = c;
        n += 1;
    }
    let total = (cqu10_d(0xae235c) as i32).wrapping_sub((cqu10_d(0xae2350) as i32).wrapping_mul(2));
    let asia = cqu10_d(0x9cfa0c);
    let mut c = cqu10_d(0xae23b4);
    for _ in 0..total.max(0) {
        let nacao = cqu10_d(c + 0x53);
        if nacao != 0 {
            let continente = cqu10_d(nacao + 0x71);
            if continente != 0 && cqu10_d(continente) == asia && (n as i16) < 0x4b0 && c != cqu10_d(lista) && c != cqu10_d(lista + 0x40) {
                clubes[n] = c;
                n += 1;
            }
        }
        c += 0x245;
    }
    crate::comparacao::ramo("cqu_10 sorteio dos clubes");
    embaralhar(clubes.as_mut_ptr() as usize, (n as u32).wrapping_sub(1), 0x28); // cqu10: 40 escolhas no embaralhamento
    let (mut leste, mut oeste) = ([0u8; 23], [0u8; 19]);
    for &clube in clubes.iter().take((n as i16).max(0) as usize) {
        let nacao = cqu10_nacao(clube);
        let (marca, inicio) = match cqu10_indice(&CQU10_LESTE, nacao) {
            Some(i) => (&mut leste[i], 0usize),
            None => match cqu10_indice(&CQU10_OESTE, nacao) {
                Some(i) => (&mut oeste[i], 16usize),
                None => continue,
            },
        };
        if *marca != 0 {
            continue;
        }
        *marca = 1;
        if cqu10_d(lista + (inicio + 15) * 4) != 0 {
            continue;
        }
        if let Some(k) = (inicio..inicio + 16).find(|&k| cqu10_d(lista + k * 4) == 0) {
            cqu10_pd(lista + k * 4, clube);
        }
    }
    1
}

// ponytail: a lista é alocada e o embaralhamento sorteia: não roda duas vezes. Só a original; o plano: devolve 1, +0xb6
// = 32, a lista em +0xb2 e, fora das vagas 0 e 16 (a do campeão da copa), cada clube na zona da sua nação (leste nas
// vagas 0..15, oeste nas 16..31) e nenhuma nação repetida.
unsafe extern "thiscall" fn comparar_cqu10_asia_classificados(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let r = o(this);
    if r == 1 {
        let lista = cqu10_d(this + 0xb2);
        let mut nacoes: Vec<i32> = Vec::new();
        let mut erro = 0usize;
        for k in 0..32usize {
            let c = if lista != 0 { cqu10_d(lista + k * 4) } else { 0 };
            if c == 0 || k == 0 || k == 16 {
                continue;
            }
            let nacao = cqu10_nacao(c);
            let zona = if k < 16 { &CQU10_LESTE[..] } else { &CQU10_OESTE[..] };
            if cqu10_indice(zona, nacao).is_none() || nacoes.contains(&nacao) {
                erro = k;
            }
            nacoes.push(nacao);
        }
        let ok = lista != 0 && cqu10_d(this + 0xb6) == 0x20 && erro == 0;
        PLACAR.registrar(ok, || format!("cqu10_asia_classificados({:#x}): lista {:#x} +0xb6 {} vaga errada {}", this, lista, cqu10_d(this + 0xb6), erro));
    }
    r
}

/// A fase de mata-mata de uma zona das copas asiáticas (0x40bec0 e 0x40ddf0, o mesmo corpo; só o arquivo e as linhas
/// das caixas mudam): os +0x3e times (0x944e46(4 × n): push; add esp, 4 -> cdecl; sem memória, a caixa `linhas.0`)
/// são os classificados 16.. da lista +0xb2; a tabela das rodadas pela virtual +0x3c (push 0; push &extra; push
/// &rodadas; push 0; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2) e o construtor Cup_Stage 0x522e00,
/// thiscall, ret 0x3c, 15 argumentos: a copa, n, os times, as rodadas, +4, a tabela, o ano +0x40, 0, 2, o extra,
/// +0x51, 0, 0, 0, 0; a original empurra os words e o byte com sobra nos bytes altos, que o construtor não lê: a cópia
/// passa os valores limpos) em [+0xc][0]; os times e a tabela liberados (0x9452ca: push; add esp, 4 -> cdecl). Sem
/// fase, a caixa `linhas.1`; com ela, +0x30 = 0.
unsafe fn cqu10_fase_da_zona(this: usize, arquivo: usize, linhas: (u32, u32), ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(0x522e00usize);
    let n = cqu10_w(this + 0x3e);
    let times = alocar((n as i32 as u32).wrapping_mul(4));
    if times == 0 {
        return show_error_at(arquivo, linhas.0);
    }
    for i in 0..n.max(0) as usize {
        crate::comparacao::ramo(ramo);
        cqu10_pd(times + i * 4, cqu10_d(cqu10_d(this + 0xb2) + 0x40 + i * 4)); // cqu10: o time i da zona oeste
    }
    let tabela_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqu10_d(cqu10_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = tabela_da_copa(this, 0, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, n as u16 as u32, times, rodadas as u16 as u32, cqu10_d(this + 4), tabela, cqu10_w(this + 0x40) as u16 as u32, 0, 2,
                  extra as u16 as u32, *((this + 0x51) as *const u8) as u32, 0, 0, 0, 0)
    } else {
        0
    };
    cqu10_pd(cqu10_d(this + 0xc), fase);
    soltar(times);
    soltar(tabela);
    if cqu10_d(cqu10_d(this + 0xc)) == 0 {
        return show_error_at(arquivo, linhas.1);
    }
    cqu10_pd(this + 0x30, 0);
}

/// { 0x40bec0 } (thiscall, ret 0, asia_club_champ.cpp; quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `AsiaClubChamp::create_second_zone_cup_stage()`: a fase 0, mata-mata dos 16 classificados da zona oeste
/// (caixas nas linhas 0x32a e 0x341).
unsafe extern "thiscall" fn cqu10_asia_fase_oeste(this: usize) {
    cqu10_fase_da_zona(this, 0x987a40, (0x32a, 0x341), "cqu_10 time da zona oeste")
}

// ponytail: a fase é alocada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][0], +0x30 = 0 e, na tabela
// de times dela (+0xa7, linhas de 6 bytes), os classificados 16.. da lista (em qualquer ordem).
unsafe fn cqu10_comparar_fase(nome: &str, this: usize, o: impl FnOnce()) {
    let lista = cqu10_d(this + 0xb2);
    let n = cqu10_w(this + 0x3e).max(0) as usize;
    if lista == 0 || n > 16 {
        return o();
    }
    let mut plano: Vec<usize> = (0..n).map(|i| cqu10_d(lista + 0x40 + i * 4)).collect();
    o();
    let f = cqu10_d(cqu10_d(this + 0xc));
    if f == 0 {
        return;
    }
    let t = cqu10_d(f + 0xa7);
    let mut lidos: Vec<usize> = if t != 0 { (0..n).map(|i| cqu10_d(t + i * 6)).collect() } else { Vec::new() };
    lidos.sort_unstable();
    plano.sort_unstable();
    PLACAR.registrar(cqu10_d(this + 0x30) == 0 && lidos == plano, || format!("{}({:#x}): times {:x?} plano {:x?} +0x30 {}", nome, this, lidos, plano, cqu10_d(this + 0x30) as i32));
}

unsafe extern "thiscall" fn comparar_cqu10_asia_fase_oeste(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqu10_comparar_fase("cqu10_asia_fase_oeste", this, || o(this))
}

/// As posições previstas das copas asiáticas (0x40c720 e 0x40e3b0, o mesmo corpo): sem estatísticas (+8), a caixa
/// `linhas.0`; um vetor de 2n clubes (0x944e46(8 × +0x3e); sem memória, a caixa `linhas.1`): os n da tabela da copa
/// (+0xa7, linhas de 6 bytes) e os n da tabela da fase 0 ([[+0xc]]+0xa7); cada metade ordenada pela reputação (qsort
/// 0x945333(v, n, 4, 0x4b07d0): 4 pushes; add esp -> cdecl); as 32 posições pela ordem (Comp_Stats::set_club_final_
/// position 0x4a2540: push posição; push clube; mov ecx, [+8] -> thiscall, ret 8): em cada metade 1, 3, 5, 7, 9, 9, 9, 9
/// e oito 17; o vetor liberado.
unsafe fn cqu10_posicoes(this: usize, arquivo: usize, linhas: (u32, u32)) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    if cqu10_d(this + 8) == 0 {
        return show_error_at(arquivo, linhas.0);
    }
    let v = alocar((cqu10_w(this + 0x3e) as i32 as u32).wrapping_mul(8));
    if v == 0 {
        return show_error_at(arquivo, linhas.1);
    }
    let mut i = 0i16;
    while i < cqu10_w(this + 0x3e) {
        let k = i as usize;
        cqu10_pd(v + k * 4, cqu10_d(cqu10_d(this + 0xa7) + k * 6));
        let n = cqu10_w(this + 0x3e) as isize as usize;
        cqu10_pd(v + (n + k) * 4, cqu10_d(cqu10_d(cqu10_d(cqu10_d(this + 0xc)) + 0xa7) + k * 6));
        i += 1;
    }
    let n = cqu10_w(this + 0x3e) as i32;
    ordenar(v, n as u32, 4, 0x4b07d0);
    let n = cqu10_w(this + 0x3e) as i32;
    ordenar(v + (n as usize).wrapping_mul(4), n as u32, 4, 0x4b07d0);
    for (k, &p) in CQU10_POSICOES.iter().enumerate() {
        gravar(cqu10_d(this + 8), cqu10_d(v + k * 4), p);
    }
    soltar(v);
}

/// As 32 posições previstas, na ordem do vetor (as duas metades).
const CQU10_POSICOES: [u32; 32] = [1, 3, 5, 7, 9, 9, 9, 9, 17, 17, 17, 17, 17, 17, 17, 17, 1, 3, 5, 7, 9, 9, 9, 9, 17, 17, 17, 17, 17, 17, 17, 17];

/// { 0x40c720 } (thiscall, ret 0, asia_club_champ.cpp): `AsiaClubChamp::rank_teams()`: as posições previstas das duas
/// zonas da Liga dos Campeões asiática (caixas nas linhas 0x45a e 0x462).
unsafe extern "thiscall" fn cqu10_asia_previstas(this: usize) {
    cqu10_posicoes(this, 0x987a40, (0x45a, 0x462))
}

// ponytail: Comp_Stats::set_club_final_position acrescenta o registro do clube (realoca): não roda duas vezes. Só a
// original; o plano: de cada metade (a tabela da copa e a da fase 0, 16 clubes cada), as posições gravadas (+0x72 do
// registro, 0x4a4850) pela reputação (+0x80) em ordem decrescente formam 1, 3, 5, 7, 9, 9, 9, 9 e oito 17; empates de
// reputação podem trocar entre si (o qsort não é estável).
unsafe fn cqu10_comparar_posicoes(nome: &str, this: usize, o: impl FnOnce()) {
    let n = cqu10_w(this + 0x3e);
    let est = cqu10_d(this + 8);
    let fases = cqu10_d(this + 0xc);
    if n != 16 || est == 0 || fases == 0 || cqu10_d(fases) == 0 || cqu10_d(this + 0xa7) == 0 || cqu10_d(cqu10_d(fases) + 0xa7) == 0 {
        return o();
    }
    let metades: [Vec<usize>; 2] = [
        (0..16).map(|k| cqu10_d(cqu10_d(this + 0xa7) + k * 6)).collect(),
        (0..16).map(|k| cqu10_d(cqu10_d(cqu10_d(fases) + 0xa7) + k * 6)).collect(),
    ];
    if metades.iter().flatten().any(|&c| c == 0) {
        return o();
    }
    o();
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut erro = None;
    for (m, metade) in metades.iter().enumerate() {
        let mut v: Vec<(i16, u8)> = metade
            .iter()
            .map(|&c| {
                let e = buscar(est, c);
                (cqu10_w(c + 0x80), if e != 0 { *((e + 0x72) as *const u8) } else { 0 })
            })
            .collect();
        v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        let posicoes: Vec<u8> = v.iter().map(|x| x.1).collect();
        if posicoes.iter().zip(CQU10_POSICOES.iter()).any(|(&a, &b)| a as u32 != b) {
            erro = Some((m, posicoes));
        }
    }
    PLACAR.registrar(erro.is_none(), || format!("{}({:#x}): posições pela reputação {:?}", nome, this, erro));
}

unsafe extern "thiscall" fn comparar_cqu10_asia_previstas(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    cqu10_comparar_posicoes("cqu10_asia_previstas", this, || o(this))
}
