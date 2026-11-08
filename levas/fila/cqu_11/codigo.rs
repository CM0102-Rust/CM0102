// ---------- %%LEVA%%a leva: a Recopa asiática: os 32 classificados das duas zonas, a fase da zona oeste e as posições previstas (asia_cup_winner.cpp) ----------

unsafe fn cqu11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqu11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqu11_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A chave da nação do clube ([[clube+0x53]], dword); sem clube ou sem nação, −1.
unsafe fn cqu11_nacao(c: usize) -> i32 {
    if c == 0 || cqu11_d(c + 0x53) == 0 {
        return -1;
    }
    cqu11_d(cqu11_d(c + 0x53)) as i32
}

/// A zona leste da Recopa (23 nações, na ordem da original; o Paquistão vem depois do Laos): Bangladesh, Brunei, China,
/// Hong Kong, Índia, Indonésia, Japão, Camboja, Laos, Paquistão, Macau, Malásia, Maldivas, Mianmar, Nepal, Coreia do
/// Norte, Singapura, Coreia do Sul, Sri Lanka, Taiwan, Tailândia, Filipinas e Vietnã.
const CQU11_LESTE: [usize; 23] = [0x9cf238, 0x9cf264, 0x9cf298, 0x9cf340, 0x9cf34c, 0x9cf350, 0x9cf36c, 0x9cf374, 0x9cf388, 0x9cf410, 0x9cf3ac,
    0x9cf3b8, 0x9cf3bc, 0x9cf3e0, 0x9cf3ec, 0x9cf404, 0x9cf46c, 0x9cf484, 0x9cf48c, 0x9cf4b8, 0x9cf4c4, 0x9cf42c, 0x9cf508];

/// A zona oeste (19 nações, como na Liga dos Campeões asiática).
const CQU11_OESTE: [usize; 19] = [0x9cf200, 0x9cf234, 0x9cf354, 0x9cf358, 0x9cf370, 0x9cf378, 0x9cf380, 0x9cf384, 0x9cf390, 0x9cf40c, 0x9cf414,
    0x9cf43c, 0x9cf458, 0x9cf4b0, 0x9cf4bc, 0x9cf4e0, 0x9cf4e4, 0x9cf4fc, 0x9cf518];

/// O índice da nação do clube numa lista de chaves (0x40cba0(clube, lista, n): 3 pushes; add esp, 0xc -> cdecl;
/// devolve o byte do índice ou −1).
unsafe fn cqu11_na_zona(clube: usize, lista: &[u32]) -> i8 {
    let f: unsafe extern "C" fn(usize, *const u32, u32) -> u32 = core::mem::transmute(0x40cba0usize);
    f(clube, lista.as_ptr(), lista.len() as u32) as u8 as i8
}

/// O clube já joga a Liga dos Campeões asiática (0x40e7b0(clube): push; add esp, 4 -> cdecl): na tabela da copa ou na
/// da fase 0 dela.
unsafe fn cqu11_na_liga_dos_campeoes(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x40e7b0usize);
    f(clube) != 0
}

/// O campeão da copa `chave` da nação `nacao` (byte +0x11c no registro de 0x122 bytes de [0xae23a8]): o campeão, ou o
/// vice se o campeão já está na vaga 0 ou 16 ou joga a Liga dos Campeões; o vice nessas condições (ou nulo) não entra.
unsafe fn cqu11_campeao_da_copa(this: usize, nacao: usize, chave: usize) -> Option<usize> {
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let n = cqu11_d(nacao);
    if *((cqu11_d(0xae23a8)).wrapping_add(n.wrapping_mul(0x122)).wrapping_add(0x11c) as *const u8) == 0 {
        return None;
    }
    let copa = cqu11_d(0xae23d0).wrapping_add(cqu11_d(chave).wrapping_mul(0x6b));
    let lista = cqu11_d(this + 0xb2);
    let c = colocado(copa, 0); // cqu11: o campeão da copa
    if c != cqu11_d(lista) && c != cqu11_d(lista + 0x40) && !cqu11_na_liga_dos_campeoes(c) {
        return if c != 0 { Some(c) } else { None };
    }
    let c = colocado(copa, 1);
    if c == cqu11_d(lista) || c == cqu11_d(lista + 0x40) || cqu11_na_liga_dos_campeoes(c) || c == 0 {
        return None;
    }
    Some(c)
}

/// { 0x40d830 } (thiscall, ret 0, asia_cup_winner.cpp, pilha 0x1598): `AsiaCupWinner::select_qualified_clubs()`: +0xb6
/// = 32 e a lista +0xb2 de 32 vagas (calloc 0x945501(0x20, 4): push 4; push 0x20; add esp, 8 -> cdecl; sem memória, a
/// caixa da linha 0x241 e 0). O campeão mais recente da própria Recopa (0x4afd80([+4], 0): push 0; push comp; add esp,
/// 8 -> cdecl), se não joga a Liga dos Campeões, vai para a vaga 0 se a nação é da zona leste, senão para a 16. Numa
/// lista local de até 1.200 clubes: o campeão da Copa do Imperador (a copa [0x9cf660]; o vice se o campeão já está nas
/// vagas 0/16 ou joga a Liga dos Campeões), se o Japão tem o byte +0x11c, e o da copa coreana [0x9cfa04] do mesmo
/// jeito; e todos os clubes da Ásia fora das vagas 0 e 16. A lista é ordenada e embaralhada (0x4afbd0(lista, n − 1,
/// 40): 3 pushes; add esp, 0xc -> cdecl). Na ordem dela, cada clube cuja nação ainda não teve vaga e que não joga a
/// Liga dos Campeões entra na primeira vaga livre da sua zona (0..15 leste, 16..31 oeste), enquanto a última vaga dela
/// estiver livre; a nação fica marcada mesmo com a zona cheia. Devolve 1.
unsafe extern "thiscall" fn cqu11_recopa_classificados(this: usize) -> u32 {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    cqu11_pd(this + 0xb6, 0x20);
    let leste: [u32; 23] = core::array::from_fn(|i| cqu11_d(CQU11_LESTE[i]) as u32);
    let oeste: [u32; 19] = core::array::from_fn(|i| cqu11_d(CQU11_OESTE[i]) as u32);
    let lista = calloc(0x20, 4);
    cqu11_pd(this + 0xb2, lista);
    if lista == 0 {
        show_error_at(0x987b60, 0x241);
        return 0;
    }
    let atual = colocado(cqu11_d(this + 4), 0);
    if !cqu11_na_liga_dos_campeoes(atual) {
        if cqu11_na_zona(atual, &leste) > -1 {
            cqu11_pd(lista, atual);
        } else {
            cqu11_pd(lista + 0x40, atual);
        }
    }
    let mut clubes = [0usize; 0x4b0];
    let mut n = 0usize;
    if let Some(c) = cqu11_campeao_da_copa(this, 0x9cf36c, 0x9cf660) {
        crate::comparison::branch("cqu_11 clube da copa do imperador");
        clubes[0] = c;
        n = 1;
    }
    if let Some(c) = cqu11_campeao_da_copa(this, 0x9cf484, 0x9cfa04) {
        clubes[n] = c;
        n += 1;
    }
    let total = (cqu11_d(0xae235c) as i32).wrapping_sub((cqu11_d(0xae2350) as i32).wrapping_mul(2));
    let asia = cqu11_d(0x9cfa0c);
    let mut c = cqu11_d(0xae23b4);
    for _ in 0..total.max(0) {
        let nacao = cqu11_d(c + 0x53);
        if nacao != 0 {
            let continente = cqu11_d(nacao + 0x71);
            if continente != 0 && cqu11_d(continente) == asia && (n as i16) < 0x4b0 && c != cqu11_d(lista) && c != cqu11_d(lista + 0x40) {
                clubes[n] = c;
                n += 1;
            }
        }
        c += 0x245;
    }
    crate::comparison::branch("cqu_11 sorteio dos clubes");
    embaralhar(clubes.as_mut_ptr() as usize, (n as u32).wrapping_sub(1), 0x28); // cqu11: 40 escolhas no embaralhamento
    let (mut marca_leste, mut marca_oeste) = ([0u8; 23], [0u8; 19]);
    for &clube in clubes.iter().take((n as i16).max(0) as usize) {
        let i = cqu11_na_zona(clube, &leste);
        let (marca, inicio) = if i > -1 {
            (&mut marca_leste[i as usize], 0usize)
        } else {
            let j = cqu11_na_zona(clube, &oeste);
            if j <= -1 {
                continue;
            }
            (&mut marca_oeste[j as usize], 16usize)
        };
        if *marca != 0 || cqu11_na_liga_dos_campeoes(clube) {
            continue;
        }
        *marca = 1;
        if cqu11_d(lista + (inicio + 15) * 4) != 0 {
            continue;
        }
        if let Some(k) = (inicio..inicio + 16).find(|&k| cqu11_d(lista + k * 4) == 0) {
            cqu11_pd(lista + k * 4, clube);
        }
    }
    1
}

// ponytail: a lista é alocada e o embaralhamento sorteia: não roda duas vezes. Só a original; o plano: devolve 1, +0xb6
// = 32, a lista em +0xb2, nenhum clube dela na Liga dos Campeões asiática e, fora das vagas 0 e 16 (a do campeão da
// Recopa), cada clube na zona da sua nação e nenhuma nação repetida.
unsafe extern "thiscall" fn comparar_cqu11_recopa_classificados(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let r = o(this);
    if r == 1 {
        let lista = cqu11_d(this + 0xb2);
        let mut nacoes: Vec<i32> = Vec::new();
        let mut erro = 0usize;
        for k in 0..32usize {
            let c = if lista != 0 { cqu11_d(lista + k * 4) } else { 0 };
            if c == 0 {
                continue;
            }
            if cqu11_na_liga_dos_campeoes(c) {
                erro = 100 + k;
            }
            if k == 0 || k == 16 {
                continue;
            }
            let nacao = cqu11_nacao(c);
            let zona = if k < 16 { &CQU11_LESTE[..] } else { &CQU11_OESTE[..] };
            if !zona.iter().any(|&g| cqu11_d(g) as i32 == nacao) || nacoes.contains(&nacao) {
                erro = k;
            }
            nacoes.push(nacao);
        }
        let ok = lista != 0 && cqu11_d(this + 0xb6) == 0x20 && erro == 0;
        PLACAR.record(ok, || format!("cqu11_recopa_classificados({:#x}): lista {:#x} +0xb6 {} vaga errada {}", this, lista, cqu11_d(this + 0xb6), erro));
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
unsafe fn cqu11_fase_da_zona(this: usize, arquivo: usize, linhas: (u32, u32), ramo: &'static str) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, u32) -> usize =
        core::mem::transmute(0x522e00usize);
    let n = cqu11_w(this + 0x3e);
    let times = alocar((n as i32 as u32).wrapping_mul(4));
    if times == 0 {
        return show_error_at(arquivo, linhas.0);
    }
    for i in 0..n.max(0) as usize {
        crate::comparison::branch(ramo);
        cqu11_pd(times + i * 4, cqu11_d(cqu11_d(this + 0xb2) + 0x40 + i * 4)); // cqu11: o time i da zona oeste
    }
    let tabela_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqu11_d(cqu11_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = tabela_da_copa(this, 0, &mut rodadas, &mut extra, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        construir(p, this, n as u16 as u32, times, rodadas as u16 as u32, cqu11_d(this + 4), tabela, cqu11_w(this + 0x40) as u16 as u32, 0, 2,
                  extra as u16 as u32, *((this + 0x51) as *const u8) as u32, 0, 0, 0, 0)
    } else {
        0
    };
    cqu11_pd(cqu11_d(this + 0xc), fase);
    soltar(times);
    soltar(tabela);
    if cqu11_d(cqu11_d(this + 0xc)) == 0 {
        return show_error_at(arquivo, linhas.1);
    }
    cqu11_pd(this + 0x30, 0);
}

/// { 0x40ddf0 } (thiscall, ret 0, asia_cup_winner.cpp; quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `AsiaCupWinner::create_zone_stage()`: a fase 0, mata-mata dos 16 classificados da zona oeste da Recopa
/// asiática (caixas nas linhas 0x2fd e 0x314).
unsafe extern "thiscall" fn cqu11_recopa_fase_oeste(this: usize) {
    cqu11_fase_da_zona(this, 0x987b60, (0x2fd, 0x314), "cqu_11 time da zona oeste")
}

// ponytail: a fase é alocada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][0], +0x30 = 0 e, na tabela
// de times dela (+0xa7, linhas de 6 bytes), os classificados 16.. da lista (em qualquer ordem).
unsafe fn cqu11_comparar_fase(nome: &str, this: usize, o: impl FnOnce()) {
    let lista = cqu11_d(this + 0xb2);
    let n = cqu11_w(this + 0x3e).max(0) as usize;
    if lista == 0 || n > 16 {
        return o();
    }
    let mut plano: Vec<usize> = (0..n).map(|i| cqu11_d(lista + 0x40 + i * 4)).collect();
    o();
    let f = cqu11_d(cqu11_d(this + 0xc));
    if f == 0 {
        return;
    }
    let t = cqu11_d(f + 0xa7);
    let mut lidos: Vec<usize> = if t != 0 { (0..n).map(|i| cqu11_d(t + i * 6)).collect() } else { Vec::new() };
    lidos.sort_unstable();
    plano.sort_unstable();
    PLACAR.record(cqu11_d(this + 0x30) == 0 && lidos == plano, || format!("{}({:#x}): times {:x?} plano {:x?} +0x30 {}", nome, this, lidos, plano, cqu11_d(this + 0x30) as i32));
}

unsafe extern "thiscall" fn comparar_cqu11_recopa_fase_oeste(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqu11_comparar_fase("cqu11_recopa_fase_oeste", this, || o(this))
}

/// As posições previstas das copas asiáticas (0x40c720 e 0x40e3b0, o mesmo corpo): sem estatísticas (+8), a caixa
/// `linhas.0`; um vetor de 2n clubes (0x944e46(8 × +0x3e); sem memória, a caixa `linhas.1`): os n da tabela da copa
/// (+0xa7, linhas de 6 bytes) e os n da tabela da fase 0 ([[+0xc]]+0xa7); cada metade ordenada pela reputação (qsort
/// 0x945333(v, n, 4, 0x4b07d0): 4 pushes; add esp -> cdecl); as 32 posições pela ordem (Comp_Stats::set_club_final_
/// position 0x4a2540: push posição; push clube; mov ecx, [+8] -> thiscall, ret 8): em cada metade 1, 3, 5, 7, 9, 9, 9, 9
/// e oito 17; o vetor liberado.
unsafe fn cqu11_posicoes(this: usize, arquivo: usize, linhas: (u32, u32)) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    if cqu11_d(this + 8) == 0 {
        return show_error_at(arquivo, linhas.0);
    }
    let v = alocar((cqu11_w(this + 0x3e) as i32 as u32).wrapping_mul(8));
    if v == 0 {
        return show_error_at(arquivo, linhas.1);
    }
    let mut i = 0i16;
    while i < cqu11_w(this + 0x3e) {
        let k = i as usize;
        cqu11_pd(v + k * 4, cqu11_d(cqu11_d(this + 0xa7) + k * 6));
        let n = cqu11_w(this + 0x3e) as isize as usize;
        cqu11_pd(v + (n + k) * 4, cqu11_d(cqu11_d(cqu11_d(cqu11_d(this + 0xc)) + 0xa7) + k * 6));
        i += 1;
    }
    let n = cqu11_w(this + 0x3e) as i32;
    ordenar(v, n as u32, 4, 0x4b07d0);
    let n = cqu11_w(this + 0x3e) as i32;
    ordenar(v + (n as usize).wrapping_mul(4), n as u32, 4, 0x4b07d0);
    for (k, &p) in CQU11_POSICOES.iter().enumerate() {
        gravar(cqu11_d(this + 8), cqu11_d(v + k * 4), p);
    }
    soltar(v);
}

/// As 32 posições previstas, na ordem do vetor (as duas metades).
const CQU11_POSICOES: [u32; 32] = [1, 3, 5, 7, 9, 9, 9, 9, 17, 17, 17, 17, 17, 17, 17, 17, 1, 3, 5, 7, 9, 9, 9, 9, 17, 17, 17, 17, 17, 17, 17, 17];

/// { 0x40e3b0 } (thiscall, ret 0, asia_cup_winner.cpp): `AsiaCupWinner::rank_teams()`: as posições previstas das duas
/// zonas da Recopa asiática (caixas nas linhas 0x3db e 0x3e3).
unsafe extern "thiscall" fn cqu11_recopa_previstas(this: usize) {
    cqu11_posicoes(this, 0x987b60, (0x3db, 0x3e3))
}

// ponytail: Comp_Stats::set_club_final_position acrescenta o registro do clube (realoca): não roda duas vezes. Só a
// original; o plano: de cada metade (a tabela da copa e a da fase 0, 16 clubes cada), as posições gravadas (+0x72 do
// registro, 0x4a4850) pela reputação (+0x80) em ordem decrescente formam 1, 3, 5, 7, 9, 9, 9, 9 e oito 17; empates de
// reputação podem trocar entre si (o qsort não é estável).
unsafe fn cqu11_comparar_posicoes(nome: &str, this: usize, o: impl FnOnce()) {
    let n = cqu11_w(this + 0x3e);
    let est = cqu11_d(this + 8);
    let fases = cqu11_d(this + 0xc);
    if n != 16 || est == 0 || fases == 0 || cqu11_d(fases) == 0 || cqu11_d(this + 0xa7) == 0 || cqu11_d(cqu11_d(fases) + 0xa7) == 0 {
        return o();
    }
    let metades: [Vec<usize>; 2] = [
        (0..16).map(|k| cqu11_d(cqu11_d(this + 0xa7) + k * 6)).collect(),
        (0..16).map(|k| cqu11_d(cqu11_d(cqu11_d(fases) + 0xa7) + k * 6)).collect(),
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
                (cqu11_w(c + 0x80), if e != 0 { *((e + 0x72) as *const u8) } else { 0 })
            })
            .collect();
        v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
        let posicoes: Vec<u8> = v.iter().map(|x| x.1).collect();
        if posicoes.iter().zip(CQU11_POSICOES.iter()).any(|(&a, &b)| a as u32 != b) {
            erro = Some((m, posicoes));
        }
    }
    PLACAR.record(erro.is_none(), || format!("{}({:#x}): posições pela reputação {:?}", nome, this, erro));
}

unsafe extern "thiscall" fn comparar_cqu11_recopa_previstas(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    cqu11_comparar_posicoes("cqu11_recopa_previstas", this, || o(this))
}
