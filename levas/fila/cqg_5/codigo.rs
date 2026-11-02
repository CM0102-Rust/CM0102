// ---------- %%LEVA%%a leva: a Liga dos Campeões: as posições previstas dos inscritos; o grupo de uma fase (european_cup.cpp, comp_util.cpp) ----------

unsafe fn cqg5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqg5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// As posições que a 0x586900 grava, na ordem das chamadas à 0x4a2540: (índice do clube no vetor ordenado, posição).
/// `mata` é a tabela das rodadas do mata-mata (0x68 bytes por rodada, +0x1a a conta de confrontos) com `nm` rodadas;
/// as preliminares vêm de [+0xa3] com +0x3c rodadas.
/// 1) O mata-mata, da final para trás: na final, os da primeira volta ganham a posição 1 (o contador sobe a cada um);
///    em toda rodada, cada um dos seguintes ganha o contador do começo da rodada (que sobe a cada um).
/// 2) A segunda fase: 2 × (4 clubes com o contador; o contador sobe 4). 3) A primeira fase: 2 × (8 clubes; sobe 8).
/// 4) As preliminares, da última para a primeira: cada clube da rodada com o contador do começo dela (sobe a cada um).
unsafe fn cqg5_plano(this: usize, mata: usize, nm: i16) -> (Vec<(i16, u8)>, usize) {
    let mut v = Vec::new();
    let mut i = 0i16;
    let mut cont = 1u8;
    let mut r = nm.wrapping_sub(1);
    while r >= 0 {
        let n = cqg5_w(mata + r as usize * 0x68 + 0x1a);
        if r == nm.wrapping_sub(1) {
            let mut k = 0i16;
            while k < n {
                crate::comparacao::ramo("cqg_5 campeão");
                v.push((i, 1u8)); // cqg5: o campeão
                i += 1;
                cont = cont.wrapping_add(1);
                k += 1;
            }
        }
        let p = cont;
        let mut k = 0i16;
        while k < cqg5_w(mata + r as usize * 0x68 + 0x1a) {
            v.push((i, p));
            i += 1;
            cont = cont.wrapping_add(1);
            k += 1;
        }
        r -= 1;
    }
    let n_mata = v.len();
    for &(vezes, quantos) in &[(2, 4u8), (2, 8u8)] {
        for _ in 0..vezes {
            let p = cont;
            cont = cont.wrapping_add(quantos);
            for _ in 0..quantos {
                v.push((i, p));
                i += 1;
            }
        }
    }
    let mut r = cqg5_w(this + 0x3c).wrapping_sub(1);
    while r >= 0 {
        let p = cont;
        let mut k = 0i16;
        while k < cqg5_w(cqg5_d(this + 0xa3) + r as usize * 0x68 + 0x1a) {
            crate::comparacao::ramo("cqg_5 posição na preliminar");
            v.push((i, p)); // cqg5: a posição na preliminar
            i += 1;
            cont = cont.wrapping_add(1);
            k += 1;
        }
        r -= 1;
    }
    (v, n_mata)
}

/// { 0x586900 } (thiscall, ret 0, european_cup.cpp, método +0x5c da Liga dos Campeões): `EuropeanCup::rank_teams()`, as
/// posições previstas. Sem estatística (+8): a caixa da linha 0x81b. Os inscritos da tabela [+0x14] (6 bytes, +0x36
/// deles) num vetor (0x944e46; sem memória, a caixa da linha 0x823), ordenado pela força (0x4c08f0: push n; push vetor;
/// add esp, 8 -> cdecl); a tabela do mata-mata pelo método +0x3c (push 0; push &id; push &rodadas; push 0xc; mov ecx ->
/// thiscall, ret 0x10), liberada depois; a posição de cada um, na ordem do vetor, pela 0x4a2540 (push posição; push
/// clube; mov ecx, [+8] -> thiscall, ret 8; a original empurra o dword da pilha com a posição no byte de baixo e lixo
/// nos outros, e a 0x4a2540 só grava o byte: a cópia passa o byte limpo) no plano de `cqg5_plano`; o vetor liberado.
unsafe extern "thiscall" fn campeoes_posicoes_previstas(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let ordenar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(0x4c08f0usize);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    if cqg5_d(this + 8) == 0 {
        aviso_em(0x9c4900, 0x81b);
        return;
    }
    let v = alocar(((cqg5_w(this + 0x36) as i32) << 2) as u32);
    if v == 0 {
        aviso_em(0x9c4900, 0x823);
        return;
    }
    let mut i = 0i16;
    while i < cqg5_w(this + 0x36) {
        core::ptr::write_unaligned((v + i as usize * 4) as *mut u32, cqg5_d(cqg5_d(this + 0x14) + i as usize * 6) as u32);
        i += 1;
    }
    ordenar(v, cqg5_w(this + 0x36) as i32);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqg5_d(cqg5_d(this) + 0x3c));
    let (mut nm, mut id) = (0i16, 0i16);
    let mata = rodadas_da_copa(this, 0xc, &mut nm, &mut id, 0);
    // as posições do mata-mata antes de liberar a tabela dele, as outras depois (como a original)
    let (plano, n_mata) = cqg5_plano(this, mata, nm);
    for (j, &(k, p)) in plano.iter().enumerate() {
        if j == n_mata {
            soltar(mata);
        }
        posicao(cqg5_d(this + 8), cqg5_d(v + k as usize * 4), p as u32);
    }
    if n_mata == plano.len() {
        soltar(mata);
    }
    soltar(v);
}

/// Só a original (acrescenta registros à estatística), conferindo o plano feito antes (o vetor ordenado numa cópia e a
/// tabela do mata-mata pedida ao método +0x3c e liberada): a posição (+0x72 do registro achado por 0x4a4850) de cada
/// clube do plano.
unsafe extern "thiscall" fn comparar_campeoes_posicoes_previstas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    if cqg5_d(this + 8) == 0 || cqg5_d(this + 0x14) == 0 || cqg5_w(this + 0x36) <= 0 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut u32, i32) = core::mem::transmute(0x4c08f0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let n = cqg5_w(this + 0x36) as usize;
    let mut v: Vec<u32> = (0..n).map(|i| cqg5_d(cqg5_d(this + 0x14) + i * 6) as u32).collect();
    ordenar(v.as_mut_ptr(), n as i32);
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqg5_d(cqg5_d(this) + 0x3c));
    let (mut nm, mut id) = (0i16, 0i16);
    let mata = rodadas_da_copa(this, 0xc, &mut nm, &mut id, 0);
    let plano = if mata != 0 { cqg5_plano(this, mata, nm).0 } else { Vec::new() };
    soltar(mata);
    o(this);
    if plano.iter().any(|&(k, _)| k as usize >= n) {
        return; // a original lê além do vetor: sem conferência
    }
    let errado = plano.iter().find(|&&(k, p)| {
        let c = v[k as usize] as usize;
        c != 0 && {
            let e = achar(cqg5_d(this + 8), c);
            e == 0 || *((e + 0x72) as *const u8) != p
        }
    });
    PLACAR.registrar(errado.is_none(), || format!("campeoes_posicoes_previstas: a posição {:?} não bate", errado));
}

/// { 0x4b01d0 } (cdecl, comp_util.cpp): `comp_util_get_parent_stage_id(id)`, o grupo de uma fase pelo id: 0x3e8..0x3fb,
/// 0x41f..0x42e, 0x413, 0x414, 0x47b e 0x3fc -> 0x41c; 0x473..0x474 -> 0x410; 0x475..0x479 -> 0x47a; 0x3fd..0x406 ->
/// 0xa; 0x407..0x40b e 0x472 -> 0x14; 0x47e..0x47f -> 0x1e; 0x40c..0x40e -> 0x82; 0x43d..0x444 -> 0x449; 0x445..0x448
/// -> 0x44a; 0x44d..0x450 -> 0x451; o resto −1 (na ordem dos testes da original).
unsafe extern "C" fn grupo_da_fase(id: i32) -> i32 {
    let em = |a: i32, b: i32| id >= a && id <= b;
    if em(0x3e8, 0x3fb) {
        return 0x41c;
    }
    if em(0x473, 0x474) {
        return 0x410;
    }
    if em(0x475, 0x479) {
        return 0x47a;
    }
    if em(0x3fd, 0x406) {
        crate::comparacao::ramo("cqg_5 grupo 0xa");
        return 0xa; // cqg5: o grupo das fases 0x3fd..0x406
    }
    if em(0x407, 0x40b) || id == 0x472 {
        return 0x14;
    }
    if em(0x47e, 0x47f) {
        return 0x1e;
    }
    if em(0x41f, 0x42e) {
        return 0x41c;
    }
    if em(0x40c, 0x40e) {
        return 0x82;
    }
    if id == 0x413 || id == 0x414 || id == 0x47b || id == 0x3fc {
        return 0x41c;
    }
    if em(0x43d, 0x444) {
        return 0x449;
    }
    if em(0x445, 0x448) {
        return 0x44a;
    }
    if em(0x44d, 0x450) {
        return 0x451;
    }
    -1
}

/// Pura: a devolução.
unsafe extern "C" fn comparar_grupo_da_fase(id: i32) -> i32 {
    let r = orig::<unsafe extern "C" fn(i32) -> i32>(%%I1%%)(id);
    let n = grupo_da_fase(id);
    PLACAR.registrar(r == n, || format!("grupo_da_fase({:#x}): original={:#x} rust={:#x}", id, r, n));
    r
}
