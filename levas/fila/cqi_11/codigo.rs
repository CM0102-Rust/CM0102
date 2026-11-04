// ---------- %%LEVA%%a leva: a Copa dos Campeões da CONCACAF: os oito classificados, o dia da copa e as rodadas (con_champ.cpp) ----------

unsafe fn cqi11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqi11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqi11_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro da competição `id` no banco ([0xae23d0] + 0x6b × id).
unsafe fn cqi11_comp(id: usize) -> usize {
    cqi11_d(0xae23d0).wrapping_add(id.wrapping_mul(0x6b))
}

/// As quatro listas da 0x4c11a0, na ordem do banco de clubes ([0xae23b4], 0x245 bytes, [0xae235c] clubes; para no
/// primeiro que é seleção, 0x53b900): (mexicanos, centro-americanos, caribenhos, da MLS) com as contagens.
struct Cqi11Listas {
    mex: [usize; 200],
    centro: [usize; 200],
    caribe: [usize; 200],
    mls: [usize; 12],
    n: [u32; 4],
}

unsafe fn cqi11_listas() -> Cqi11Listas {
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    // ponytail: as quatro listas começam zeradas na cópia (na original, sobra de pilha que só é lida quando falta clube;
    // o comparador manda esse caso para a original sozinha)
    let mut l = Cqi11Listas { mex: [0; 200], centro: [0; 200], caribe: [0; 200], mls: [0; 12], n: [0; 4] };
    let (mut a, mut b, mut c, mut m) = (0u8, 0u8, 0u8, 0i8);
    let mut clube = cqi11_d(0xae23b4);
    let mut i = 0i32;
    while i < cqi11_d(0xae235c) as i32 {
        if selecao(clube) != 0 {
            break;
        }
        let nacao = cqi11_d(clube + 0x53);
        if nacao != 0 {
            let id = cqi11_d(nacao);
            if id == cqi11_d(0x9cf3d0) {
                if a < 0xc8 {
                    l.mex[a as usize] = clube;
                    a += 1;
                }
            } else {
                let div = cqi11_d(clube + 0x57);
                if div != 0 && cqi11_d(div) == cqi11_d(0x9cf590) {
                    if m < 0xc {
                        l.mls[m as usize] = clube;
                        m += 1;
                    }
                } else if id != cqi11_d(0x9cf4f8) && cqi11_b(nacao + 0x75) == 7 {
                    if b < 0xc8 {
                        l.centro[b as usize] = clube;
                        b += 1;
                    }
                } else if cqi11_b(nacao + 0x75) == 6 && c < 0xc8 {
                    l.caribe[c as usize] = clube;
                    c += 1;
                }
            }
        }
        i += 1;
        clube = clube.wrapping_add(0x245);
    }
    l.n = [a as u32, b as u32, c as u32, m as u8 as u32];
    l
}

/// { 0x4c11a0 } (thiscall, ret 0, con_champ.cpp): `ConChamp::select_qualified_clubs()`; devolve 1. Oito times (+0x3e).
/// Os clubes (até a primeira seleção) em quatro listas: os da nação [0x9cf3d0] (México, até 200); os da divisão
/// [0x9cf590] (a MLS, +0x57 do clube; até 12); os de nação de região 7 (+0x75 da nação, América Central) fora da
/// [0x9cf4f8] (até 200); os de região 6 (o Caribe, até 200). Cada lista ordenada pela reputação, com os 3 × k
/// primeiros embaralhados (0x4afbd0: push k; push n; push lista; add esp, 0xc -> cdecl; k = 2, 3, 1 e 1, a contagem
/// na palavra de baixo). Os times (+0xa7, 6 bytes): 2 e 3 os dois mexicanos; 4, 5 e 6 os três centro-americanos; 7 o
/// caribenho; 0 e 1: sem o objeto da competição [0x9cf728], os dois primeiros da MLS; com ele, o último campeão da MLS
/// (0x4afd80(competição [0x9cf590], 0): push 0; push comp; add esp, 8 -> cdecl) e o da [0x9cf728] (a mesma com +4 do
/// objeto) e, se for o mesmo clube, o 0 passa ao último vice da MLS (lugar 1). Depois +4 e +5 de cada time zerados.
unsafe extern "thiscall" fn cqi11_concacaf_classificados(this: usize) -> u32 {
    let ordenar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 8);
    let outro = cqi11_d(cqi11_d(0xadadfc).wrapping_add(cqi11_d(0x9cf728).wrapping_mul(4)));
    let mut l = cqi11_listas();
    ordenar(l.mex.as_mut_ptr() as usize, l.n[0], 2);
    ordenar(l.centro.as_mut_ptr() as usize, l.n[1], 3); // cqi11: três centro-americanos
    ordenar(l.caribe.as_mut_ptr() as usize, l.n[2], 1);
    ordenar(l.mls.as_mut_ptr() as usize, l.n[3], 1);
    let t = |k: usize| cqi11_d(this + 0xa7).wrapping_add(k * 6);
    let pd = |a: usize, v: usize| core::ptr::write_unaligned(a as *mut u32, v as u32);
    crate::comparacao::ramo("cqi_11 classificados");
    pd(t(2), l.mex[0]);
    pd(t(3), l.mex[1]); // cqi11: o segundo mexicano
    pd(t(4), l.centro[0]);
    pd(t(5), l.centro[1]);
    pd(t(6), l.centro[2]);
    pd(t(7), l.caribe[0]);
    if outro == 0 {
        pd(t(0), l.mls[0]);
        pd(t(1), l.mls[1]);
    } else {
        crate::comparacao::ramo("cqi_11 campeao da mls");
        let mls = cqi11_comp(cqi11_d(0x9cf590));
        pd(t(0), colocado(mls, 0));
        pd(t(1), colocado(cqi11_d(outro + 4), 0));
        if cqi11_d(t(0)) == cqi11_d(t(1)) { // cqi11: o campeão das duas dá a vaga ao vice da MLS
            pd(t(0), colocado(mls, 1));
        }
    }
    let mut i = 0i32;
    while i < cqi11_w(this + 0x3e) as i32 {
        *((cqi11_d(this + 0xa7) + i as usize * 6 + 4) as *mut u8) = 0;
        *((cqi11_d(this + 0xa7) + i as usize * 6 + 5) as *mut u8) = 0;
        i += 1;
    }
    1
}

/// Comparador da 0x4c11a0: lado a lado nos oito times (+0xa7, 48 bytes), em +0x3e e no estado do sorteador (a 0x4afbd0
/// embaralha; a cópia sem contar). Quando falta clube numa lista (menos de 2 mexicanos, 3 centro-americanos, 1
/// caribenho, ou 2 da MLS sem o objeto [0x9cf728]) a original lê sobra de pilha: só a original.
unsafe extern "thiscall" fn comparar_cqi11_concacaf_classificados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let l = cqi11_listas();
    let outro = cqi11_d(cqi11_d(0xadadfc).wrapping_add(cqi11_d(0x9cf728).wrapping_mul(4)));
    if l.n[0] < 2 || l.n[1] < 3 || l.n[2] < 1 || (outro == 0 && l.n[3] < 2) || cqi11_d(this + 0xa7) == 0 {
        return o(this); // ponytail: lista curta (a original lê sobra de pilha) ou sem tabela: só a original
    }
    let mut regioes = vec![(this + 0x3e, 2), (cqi11_d(this + 0xa7), 48)];
    regioes.extend_from_slice(&crate::aleatorio::REGIOES_DO_ESTADO);
    // a 0x4afd80 reordena o histórico de competições ([0xae23d4], [0xae237c] linhas de 0x1a bytes) a cada chamada; com
    // empate, a ordem que a original deixou muda o campeão que a cópia acharia: o histórico entra nas regiões
    let historico = cqi11_d(0xae23d4);
    if historico != 0 {
        regioes.push((historico, cqi11_d(0xae237c).wrapping_mul(0x1a)));
    }
    let (r, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this), || crate::aleatorio::sem_contar(|| cqi11_concacaf_classificados(this)));
    PLACAR.registrar(igual && r == rr, || format!("cqi11_concacaf_classificados({:#x}) listas {:?} original={:x?} rust={:x?}", this, l.n, mo, mr));
    r
}

/// { 0x4c1620 } (thiscall, ret 8, con_champ.cpp; a virtual +0x14 da copa): `ConChamp::update_for_date(hoje, fase)`.
/// Com a tabela de times vazia (o clube do time 0 = 0) e fase ≠ 0: os classificados (0x4c11a0: mov ecx, this ->
/// thiscall, ret 0; negativo, a caixa da linha 0x1ba e devolve 0) e as posições previstas (virtual +0x5c). Depois a
/// atualização da copa (0x51f890: push fase; push hoje; mov ecx, this -> thiscall, ret 8), cuja devolução sai.
unsafe extern "thiscall" fn cqi11_concacaf_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    if cqi11_d(cqi11_d(this + 0xa7)) == 0 && fase != 0 {
        let classificar: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x4c11a0usize);
        if classificar(this) < 0 {
            // ponytail: a 0x4c11a0 sempre devolve 1; o caso de erro fica com a caixa e o 0 da original
            aviso_em(0x9a7d30, 0x1ba);
            return 0;
        }
        crate::comparacao::ramo("cqi_11 dia da selecao");
        let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqi11_d(cqi11_d(this) + 0x5c));
        prever(this);
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51f890usize);
    base(this, hoje, fase)
}

// ponytail: a atualização da copa (0x51f890) joga e sorteia: não roda duas vezes. Só a original roda; o plano: com a
// tabela vazia e fase ≠ 0, a copa sai com 8 times e o time 0 preenchido (a 0x4c11a0 tem o seu comparador); senão +0x3e
// não muda.
unsafe extern "thiscall" fn comparar_cqi11_concacaf_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    if cqi11_d(this + 0xa7) == 0 {
        return o(this, hoje, fase);
    }
    let seleciona = cqi11_d(cqi11_d(this + 0xa7)) == 0 && fase != 0;
    let n = cqi11_w(this + 0x3e);
    let r = o(this, hoje, fase);
    let igual = if seleciona { cqi11_w(this + 0x3e) == 8 } else { cqi11_w(this + 0x3e) == n };
    PLACAR.registrar(igual, || format!("cqi11_concacaf_dia({:#x}, fase {}) seleciona={} times {} -> {}", this, fase, seleciona, n, cqi11_w(this + 0x3e)));
    r
}

/// { 0x4c15f0 } (thiscall, ret 0, con_champ.cpp; a virtual +0x8c): `ConChamp::set_up_rounds()`; devolve 1. O fundo de
/// prêmios (+0x28) de 2.000.000 e a tabela das rodadas (+0xa3) da virtual +0x3c (push 0; push &+0x3a; push &+0x3c;
/// push −1; mov ecx, this -> thiscall, ret 0x10).
unsafe extern "thiscall" fn cqi11_concacaf_rodadas(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x28) as *mut u32, 0x1e8480);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqi11_d(cqi11_d(this) + 0x3c));
    let t = tabela(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xa3) as *mut u32, t as u32);
    1
}

// ponytail: a virtual +0x3c aloca a tabela das rodadas: não roda duas vezes. Só a original; o plano confere o fundo de
// prêmios (+0x28 = 2.000.000) e a devolução 1.
unsafe extern "thiscall" fn comparar_cqi11_concacaf_rodadas(this: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this);
    PLACAR.registrar(r == 1 && cqi11_d(this + 0x28) == 0x1e8480, || format!("cqi11_concacaf_rodadas({:#x}) devolveu {} prêmios {}", this, r, cqi11_d(this + 0x28)));
    r
}
