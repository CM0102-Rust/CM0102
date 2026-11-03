// ---------- 271a leva: o contrato assinado e posto no vetor dos contratos (contract_manager.cpp) ----------

unsafe fn cqg1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqg1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqg1_u(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqg1_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqg1_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqg1_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqg1_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

unsafe fn cqg1_clube(id: usize) -> usize {
    cqg1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O registro de 79 bytes da pessoa `id` em [0xae2d0c] (+0 o índice do contrato no vetor).
unsafe fn cqg1_registro(id: usize) -> usize {
    cqg1_d(0xae2d0c).wrapping_add(id.wrapping_mul(0x4f))
}

/// A cópia do contrato de 0x50 bytes de `o` para `d`, campo a campo como a original (as datas +0x25 e +0x2d pela
/// 0x417c10; os campos de bits de +0x35 e o nibble de +0x4f, mesclados em passos, acabam iguais à origem).
unsafe fn cqg1_copiar(d: usize, o: usize) {
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    for k in (0..0x1c).step_by(4) {
        cqg1_pd(d + k, cqg1_d(o + k));
    }
    for k in 0x1c..0x21 {
        cqg1_pb(d + k, cqg1_b(o + k));
    }
    cqg1_pd(d + 0x21, cqg1_d(o + 0x21));
    let mut tmp = [0u8; 8];
    copiar_data(d + 0x25, tmp.as_mut_ptr(), o + 0x25);
    copiar_data(d + 0x2d, tmp.as_mut_ptr(), o + 0x2d);
    let s = cqg1_b(o + 0x35);
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ cqg1_b(d + 0x35)) & 0x3f) ^ cqg1_b(d + 0x35);
    let al = ((((s << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl;
    cqg1_pb(d + 0x35, ((al ^ s) & 0x7f) ^ s);
    cqg1_pd(d + 0x36, cqg1_d(o + 0x36));
    cqg1_pb(d + 0x3a, cqg1_b(o + 0x3a));
    cqg1_pw(d + 0x3b, cqg1_u(o + 0x3b));
    for k in [0x3dusize, 0x3e, 0x3f] {
        cqg1_pb(d + k, cqg1_b(o + k));
    }
    cqg1_pd(d + 0x40, cqg1_d(o + 0x40));
    cqg1_pb(d + 0x44, cqg1_b(o + 0x44));
    cqg1_pd(d + 0x45, cqg1_d(o + 0x45));
    cqg1_pb(d + 0x49, cqg1_b(o + 0x49));
    cqg1_pd(d + 0x4a, cqg1_d(o + 0x4a));
    cqg1_pb(d + 0x4e, cqg1_b(o + 0x4e));
    let s = cqg1_b(o + 0x4f);
    let a = ((cqg1_b(d + 0x4f) ^ s) & 0xf) ^ cqg1_b(d + 0x4f);
    cqg1_pb(d + 0x4f, ((a ^ s) & 0xf) ^ s);
}

/// O tipo do contrato (+0x3d) pelo cargo (nibble alto de +0x4f): 1 → 75, 2 → 50, 3 → 40, 4, 5, 6 e 8 → 0; os outros
/// não mexem (a tabela de saltos da original).
unsafe fn cqg1_tipo_pelo_cargo(c: usize) {
    match cqg1_b(c + 0x4f) >> 4 {
        1 => cqg1_pb(c + 0x3d, 0x4b),
        2 => cqg1_pb(c + 0x3d, 0x32),
        3 => cqg1_pb(c + 0x3d, 0x28),
        4 | 5 | 6 | 8 => cqg1_pb(c + 0x3d, 0),
        _ => {}
    }
}

/// { 0x4cf720 } (thiscall, ret 8, contract_manager.cpp, sem SEH): `CONTRACT_MANAGER::sign_contract(contrato, cargo)`,
/// pela conclusão da transferência (0x8c9720), pela proposta aceita (0x8c38b0) e pela tela do pessoal. Pessoa sem clube
/// (+0x39) fora do empréstimo (+0x35 = 5): inclui no clube do contrato (0x53bd10 com hoje); falhou: 0. Cargo novo fora do
/// empréstimo sem vaga no clube atual (0x53b9a0): a notícia da proposta cancelada (0x4d64e0) e 0. Clube atual humano
/// (0x600a50 com 1, 1): as listas dos olheiros (0x7f7270) e 0x8b2880. No empréstimo: elenco acima de 50, a notícia do
/// teste cancelado (0x4dade0) e 0; senão 0x90d860, inclui com o cargo 11, o fim pela duração (0x54cdb0 e 0x54c5b0 a
/// partir de hoje), o cargo 8 no nibble alto de +0x4f, o início hoje e 0x540980. O fim no mínimo 4 semanas depois do
/// início (0x54c770); o fim também em +0x46 da pessoa. O lugar no vetor (+0 ponteiro, +4 capacidade, +8 conta): o
/// índice do registro de 79 bytes; com contrato no mesmo clube, os termos antigos de +0x3a..+0x4f ficam (+0x49 0, +0x4a
/// −1) e, na nação de [0x9cf434] para quem não é comunitário (0x53d740), daqui a 100 dias (0x54cab0) depois da data +0x3e
/// da pessoa, a segunda nacionalidade (+0x1e) vira a dessa nação (com a 1ª de [0x9cf260] ou num sorteio de 3); noutro
/// clube, o tipo pelo cargo, +0x49/+0x4a, 0x866ee0 e +0x3b zerado. Sem registro: a primeira vaga livre (+0 = −1) recebe
/// o contrato (tipo pelo cargo, 0x866ee0), o registro, a folha (0x598370) e 1; senão no fim (cresce 100, realloc
/// 0x945667 ou malloc 0x944e46; sem memória, a caixa 0x1637). Depois: o valor (+0xc) em +0x4e da pessoa, a moral (+0x14,
/// 0x544710) no tipo 2 com ficha, a folha (0x598370), e com cargo novo a saída do clube atual (0x53c190) e a entrada no
/// do contrato (0x53bd10); falhou: a caixa (0x1672, 0x167a) e 0.
unsafe extern "thiscall" fn assinar_contrato(this: usize, contrato: usize, cargo: u32) -> u32 {
    let incluir: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x53bd10usize);
    let tem_vaga: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x53b9a0usize);
    let noticia_sem_vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4d64e0usize);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let olheiros: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7f7270usize);
    let f8b2880: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8b2880usize);
    let elenco: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(0x53c590usize);
    let noticia_teste: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4dade0usize);
    let f90d860: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x90d860usize);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, i32) -> usize = core::mem::transmute(0x54c5b0usize);
    let mais_semanas: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c770usize);
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let marcar_elenco: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x540980usize);
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54cab0usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let zerar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x866ee0usize);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x598370usize);
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x544710usize);
    let tirar: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53c190usize);
    let c = contrato;
    let clube = cqg1_clube(cqg1_d(c + 4));
    let pessoa = cqg1_d(0xae23bc).wrapping_add(cqg1_d(c).wrapping_mul(0x6e));
    let emprestimo = || cqg1_b(c + 0x35) & 0x3f == 5;
    if cqg1_d(pessoa + 0x39) == 0 && !emprestimo() && incluir(clube, pessoa, cargo, 0xae2c90, 1, 0) == 0 {
        return 0;
    }
    if cargo as u8 != cqg1_b(pessoa + 0x3d) && !emprestimo() && tem_vaga(cqg1_d(pessoa + 0x39), cargo, pessoa) == 0 {
        noticia_sem_vaga(this, pessoa, cqg1_d(pessoa + 0x39));
        return 0;
    }
    if cqg1_d(pessoa + 0x39) != 0 && humano(cqg1_d(pessoa + 0x39), 1, 1, 0, 0) != 0 {
        olheiros(cqg1_d(0xb74448), cqg1_d(cqg1_d(pessoa + 0x39) + 0xcf));
        f8b2880(cqg1_d(0xde1f44), cqg1_d(pessoa + 0x39), 0);
    }
    let mut tmp = [0u8; 8];
    let mut tmp2 = [0u8; 8];
    if emprestimo() {
        crate::comparacao::ramo("cqg_1 empréstimo");
        if (elenco(clube, 0, 0) as i32) + 1 > 0x32 {
            noticia_teste(this, pessoa, clube);
            return 0;
        }
        f90d860(cqg1_d(0xde2f70), cqg1_clube(cqg1_d(c + 4)), 0);
        incluir(cqg1_clube(cqg1_d(c + 4)), pessoa, 0xb, 0xae2c90, 1, 0);
        let n = dias_ate(c + 0x2d, cqg1_d(c + 0x25) as u32, cqg1_d(c + 0x29) as u32);
        let fim = mais(0xae2c90, tmp2.as_mut_ptr(), n); // cqg1: o fim pela duração
        copiar_data(c + 0x2d, tmp.as_mut_ptr(), fim);
        cqg1_pb(c + 0x4f, (cqg1_b(c + 0x4f) & 0xf) | 0x80);
        copiar_data(c + 0x25, tmp.as_mut_ptr(), 0xae2c90);
        marcar_elenco(pessoa, 1);
    }
    // o fim no mínimo 4 semanas depois do início
    let (ini_dia, ini_ano) = (cqg1_w(c + 0x25), cqg1_u(c + 0x27));
    let fim_ano = cqg1_u(c + 0x2f);
    if ini_ano > fim_ano || (ini_ano == fim_ano && !(ini_dia < cqg1_w(c + 0x2d))) {
        crate::comparacao::ramo("cqg_1 fim em 4 semanas");
        let p = mais_semanas(c + 0x25, tmp2.as_mut_ptr(), 4); // cqg1: as 4 semanas
        copiar_data(c + 0x2d, tmp.as_mut_ptr(), p);
    }
    copiar_data(pessoa + 0x46, tmp.as_mut_ptr(), c + 0x2d);
    let idx = cqg1_d(cqg1_registro(cqg1_d(c))) as i32;
    if idx > -1 {
        let off = (idx as usize).wrapping_mul(0x50);
        let r = cqg1_d(this).wrapping_add(off);
        if cqg1_d(r + 4) == cqg1_d(c + 4) { // cqg1: o mesmo clube
            crate::comparacao::ramo("cqg_1 mesmo clube");
            let r = || cqg1_d(this).wrapping_add(off);
            cqg1_pb(c + 0x3a, cqg1_b(r() + 0x3a));
            for k in [0x3dusize, 0x3e, 0x3f, 0x44] {
                cqg1_pb(c + k, cqg1_b(r() + k));
            }
            cqg1_pd(c + 0x45, cqg1_d(r() + 0x45));
            cqg1_pb(c + 0x4e, cqg1_b(r() + 0x4e));
            cqg1_pb(c + 0x49, 0);
            cqg1_pd(c + 0x4a, u32::MAX as usize);
            let a = cqg1_b(c + 0x4f);
            cqg1_pb(c + 0x4f, ((cqg1_b(r() + 0x4f) ^ a) & 0xf) ^ a);
            let a = cqg1_b(c + 0x35);
            cqg1_pb(c + 0x35, ((((cqg1_b(r() + 0x35) << 1) as i8 >> 1) as u8 ^ a) & 0x40) ^ a);
            cqg1_pw(c + 0x3b, cqg1_u(r() + 0x3b));
            let nacao = cqg1_d(clube + 0x53);
            if nacao != 0 && cqg1_d(nacao) == cqg1_d(0x9cf434) && comunitaria(pessoa, 0) == 0 {
                let p = mais_dias(0xae2c90, tmp2.as_mut_ptr(), 0x64);
                let (dia, ano) = (cqg1_w(p), cqg1_u(p + 2));
                let depois = ano > cqg1_u(pessoa + 0x40) || (ano == cqg1_u(pessoa + 0x40) && dia > cqg1_w(pessoa + 0x3e));
                if depois {
                    let primeira = cqg1_d(pessoa + 0x1a);
                    if (primeira != 0 && cqg1_d(primeira) == cqg1_d(0x9cf260)) || sorteio(3) == 0 {
                        cqg1_pd(pessoa + 0x1e, cqg1_d(0xae23a8).wrapping_add(cqg1_d(0x9cf434).wrapping_mul(0x122)));
                    }
                }
            }
        } else {
            cqg1_tipo_pelo_cargo(c);
            cqg1_pd(c + 0x4a, u32::MAX as usize);
            cqg1_pb(c + 0x49, 0);
            zerar(c, pessoa, cqg1_clube(cqg1_d(c + 4)));
            cqg1_pw(c + 0x3b, 0);
        }
        cqg1_copiar(cqg1_d(this).wrapping_add(off), c);
    } else {
        let conta = cqg1_d(this + 8) as i32;
        let mut i = 0i32;
        while i < conta {
            let r = cqg1_d(this).wrapping_add((i as usize).wrapping_mul(0x50));
            if cqg1_d(r) as i32 == -1 { // cqg1: a vaga livre
                crate::comparacao::ramo("cqg_1 vaga livre");
                cqg1_tipo_pelo_cargo(c);
                cqg1_pb(c + 0x49, 0);
                cqg1_pd(c + 0x4a, u32::MAX as usize);
                zerar(c, pessoa, cqg1_clube(cqg1_d(c + 4)));
                cqg1_copiar(r, c);
                cqg1_pd(cqg1_registro(cqg1_d(r)), i as u32 as usize);
                let f = financas(cqg1_d(0xae2a38), cqg1_clube(cqg1_d(c + 4)));
                if f != 0 {
                    folha(f);
                }
                return 1;
            }
            i += 1;
        }
        if conta >= cqg1_d(this + 4) as i32 {
            let cap = (cqg1_d(this + 4) as u32).wrapping_add(0x64);
            cqg1_pd(this + 4, cap as usize);
            let p = if cqg1_d(this) == 0 { malloc(cap.wrapping_mul(0x50)) } else { realocar(cqg1_d(this), cap.wrapping_mul(0x50)) };
            cqg1_pd(this, p);
            if p == 0 {
                aviso_em(0x9a8190, 0x1637);
            }
        }
        cqg1_tipo_pelo_cargo(c);
        cqg1_pb(c + 0x49, 0);
        cqg1_pd(c + 0x4a, u32::MAX as usize);
        zerar(c, pessoa, cqg1_clube(cqg1_d(c + 4)));
        cqg1_pw(c + 0x3b, 0);
        cqg1_pd(cqg1_registro(cqg1_d(c)), cqg1_d(this + 8));
        let n = cqg1_d(this + 8);
        let r = cqg1_d(this).wrapping_add(n.wrapping_mul(0x50));
        cqg1_pd(this + 8, n.wrapping_add(1));
        cqg1_copiar(r, c);
    }
    crate::comparacao::ramo("cqg_1 valor na pessoa");
    cqg1_pd(pessoa + 0x4e, cqg1_d(c + 0xc)); // cqg1: o valor do contrato na pessoa
    if cqg1_b(c + 0x35) & 0x3f == 2 && cqg1_d(pessoa + 0x61) != 0 {
        moral(pessoa, 0x14, 0);
    }
    let f = financas(cqg1_d(0xae2a38), cqg1_clube(cqg1_d(c + 4)));
    if f != 0 {
        folha(f);
    }
    if cargo as u8 == cqg1_b(pessoa + 0x3d) {
        return 1;
    }
    if tirar(cqg1_d(pessoa + 0x39), pessoa, 0, 1, 0) == 0 {
        aviso_em(0x9a8190, 0x1672);
        return 0;
    }
    if incluir(cqg1_clube(cqg1_d(c + 4)), pessoa, cargo, pessoa + 0x3e, 1, 0) != 0 {
        return 1;
    }
    aviso_em(0x9a8190, 0x167a);
    0
}

/// Comparador da 0x4cf720: só a original (inclui e tira a pessoa dos clubes, realoca o vetor dos contratos, entrega
/// notícias, sorteia a segunda nacionalidade, recalcula a folha).
unsafe extern "thiscall" fn comparar_assinar_contrato(this: usize, contrato: usize, cargo: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1629)(this, contrato, cargo) // ponytail: só a original
}

// ---------- 271a leva: o regen jovem: a pessoa aposentada refeita como jogador novo do banco de fichas (nação, nomes, idiomas, idade, habilidade, reputações, personalidade) e posta num clube; a saída da disputa (player_regen.cpp, dispute.cpp) ----------

unsafe fn cqg7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqg7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqg7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqg7_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cqg7_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cqg7_pb(a: usize, v: i8) {
    *(a as *mut i8) = v;
}

unsafe fn cqg7_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// Uma chamada thiscall de um argumento pelo endereço (as rotinas de limpeza da pessoa).
unsafe fn cqg7_com(rotina: usize, this: usize, a: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(rotina);
    f(this, a);
}

/// O byte `k` do registro de 79 bytes da pessoa ([0xae2d0c] + 0x4f × id).
unsafe fn cqg7_r79(p: usize, k: usize) -> *mut u8 {
    (cqg7_d(0xae2d0c) as isize + cqg7_i(p) as isize * 0x4f + k as isize) as *mut u8
}

/// O inteiro × a constante double do .rdata, truncado (fild; fmul qword; _ftol), em 16 bits.
unsafe fn cqg7_mul(v: i16, k: usize) -> i16 {
    ftol(v as f64 * f64_em(k)) as i16
}

/// A habilidade (ficha +5) pelo sorteio: r = sorteio(trunc(PA × m1)); trunc(r + PA × m2) (fild r; fild PA; fmul m2;
/// faddp; _ftol), a PA na ficha +7.
unsafe fn cqg7_habilidade(f: usize, m1: usize, m2: usize) {
    let pa = cqg7_w(f + 7) as f64;
    let r = cqg7_sortear(ftol(pa * f64_em(m1)));
    crate::comparacao::ramo("cqg_7 habilidade");
    cqg7_pw(f + 5, ftol(r as f64 + pa * f64_em(m2)) as i16); // cqg7: a habilidade sorteada pela PA
}

/// O acerto de um atributo de personalidade de 1..20: acima de 15 cai sorteio(`cai`); abaixo de `a1` sobe sorteio(`s1`);
/// abaixo de `a2` sobe sorteio(`s2`) (0 = nada).
unsafe fn cqg7_acertar(a: usize, cai: i32, a1: i8, s1: i32, a2: i8, s2: i32) {
    let v = cqg7_b(a);
    if v > 0xf {
        let r = cqg7_sortear(cai);
        cqg7_pb(a, cqg7_b(a).wrapping_sub(r as i8));
    } else if v < a1 {
        let r = cqg7_sortear(s1);
        cqg7_pb(a, cqg7_b(a).wrapping_add(r as i8));
    } else if s2 != 0 && v < a2 {
        let r = cqg7_sortear(s2);
        cqg7_pb(a, cqg7_b(a).wrapping_add(r as i8));
    }
}

/// Abaixo de `lim`: soma sorteio(n).
unsafe fn cqg7_subir_abaixo(a: usize, lim: i8, n: i32) {
    if cqg7_b(a) < lim {
        let r = cqg7_sortear(n);
        cqg7_pb(a, cqg7_b(a).wrapping_add(r as i8));
    }
}

/// Tira sorteio(n).
unsafe fn cqg7_tirar(a: usize, n: i32) {
    let r = cqg7_sortear(n);
    cqg7_pb(a, cqg7_b(a).wrapping_sub(r as i8));
}

/// O teto da habilidade: acima de `teto` e sorteio(n) diferente de 0, o teto.
unsafe fn cqg7_teto(f: usize, teto: i16, n: i32) {
    if cqg7_w(f + 5) > teto && cqg7_sortear(n) != 0 {
        cqg7_pw(f + 5, teto);
    }
}

/// { 0x7abac0 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::regenerate_as_youth_player(pessoa)`, pela fila dos
/// regens (0x7a65d0) em todo regen jovem. Pessoa não aposentada (bits 0x30 do +0xb do registro de 79 bytes) ou com
/// clube: só limpa os bits. Senão: sai da seleção (0x775ba0), pega a ficha da frente do banco de fichas (this+0x3c,
/// entradas de 16 bytes: ficha, nação, segunda nação; 0x7a8470), vira jogador (+0x5f = 2, +0x61 a ficha) e o registro
/// de 79 bytes ganha a marca do regen (0x40) e perde as outras; as listas e históricos da pessoa são limpos (névoa,
/// estatísticas, histórico, suspensões, lesões, disputa, disciplina, físico, contratos); a nação da ficha (ou, se ela
/// tem reputação abaixo de 100, com 1/4, uma qualquer de reputação abaixo de 100), a segunda nação (a Espanha com 1/100
/// mantém a anterior; outra que não o País Basco com 2/3 nenhuma), os nomes da nação, os idiomas (todos tirados; os da
/// nação pelo peso, nível (peso − (20 − +0x56) × 10) / 75 até 10), a data de entrada hoje, +0x3d/+0x28 = 11. A idade e
/// a habilidade: nação pequena (reputação abaixo de 350, +0x11c zero) pela reputação (23..27 ou 17..23 anos) e pela PA;
/// senão pela posição da ficha e a idade por sorteio(10000) (15..18 raros, 16..18, 17..20, 21..26). As reputações (+9,
/// +0xb, +0xd), os tetos da habilidade, o ano e o dia do nascimento, a idade (0x53ab90), os atributos de personalidade
/// (+0x1b, +0x36, +0x1d, +0x2e, +0x34, +0x44, +0x3c, +0x3d, +0x1e, +0x24, +0x2c, +0x2f, +0x21) pela idade; abaixo de 19
/// anos e nação de +0x11c, com 3/4, o melhor clube (0x7ad0e0) e o contrato (0x7a6f10, notícia 0x7a8ba0 no clube humano e
/// o auxiliar pode gostar dele); senão, com chance, um clube da nação (0x7ad590). Sem clube e abaixo de 20 anos, as
/// reputações até 1250 e +0xd = 1. No fim, os atributos normalizados (0x538ec0, 0x538bc0).
unsafe extern "thiscall" fn refazer_como_jovem(this: usize, p: usize) {
    let marca = cqg7_r79(p, 0xb);
    let b = *marca;
    if b & 0x30 == 0 || cqg7_d(p + 0x39) != 0 {
        *marca = b & 0xcf;
        return;
    }
    cqg7_com(0x775ba0, cqg7_d(0xb74318), p);
    let banco = cqg7_d(this + 0x3c);
    let (ficha_id, nacao_id, nacao2_id) = (cqg7_i(banco), cqg7_i(banco + 4), cqg7_i(banco + 8));
    let tirar_do_banco: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x7a8470usize);
    tirar_do_banco(this, ficha_id);
    *((p + 0x5f) as *mut u8) = 2;
    core::ptr::write_unaligned((p + 0x61) as *mut usize, cqg7_d(0xae23c4).wrapping_add((ficha_id as isize * 0x46) as usize));
    *cqg7_r79(p, 0xb) |= 0x40;
    *cqg7_r79(p, 0xd) = 0;
    *cqg7_r79(p, 0xc) &= 0xfd;
    *cqg7_r79(p, 0xb) &= 0x7f;
    *cqg7_r79(p, 0xc) &= 0xfe;
    *cqg7_r79(p, 9) = 0x9c;
    if cqg7_d(0xae2a60) != 0 && *(0x9cc270 as *const u8) != 0 {
        cqg7_com(0x5afd00, cqg7_d(0xae2a60), p);
    }
    cqg7_com(0x7c6650, cqg7_d(0xb74350), p);
    cqg7_com(0x86fb80, cqg7_d(0xdd7f00), p);
    cqg7_com(0x55e1d0, cqg7_d(0xae28f0), p);
    cqg7_com(0x631f90, cqg7_d(0xb63d28), p);
    cqg7_com(0x56a090, cqg7_d(0xae2a30), p);
    cqg7_com(0x556830, cqg7_d(0xae28f0), cqg7_d(p));
    cqg7_com(0x62bfa0, cqg7_d(0xb63d28), cqg7_d(p));
    cqg7_com(0x4d0b50, cqg7_d(0xae1050), p);
    cqg7_com(0x4d0bd0, cqg7_d(0xae1050), p);
    *((p + 0x22) as *mut u8) = 0;
    *((p + 0x23) as *mut u8) = 0;
    if cqg7_d(p + 0x69) != 0 {
        core::ptr::write_unaligned((p + 0x69) as *mut u32, 0);
    }
    *cqg7_r79(p, 0xb) &= 0xf0;
    *cqg7_r79(p, 8) = 0;
    *cqg7_r79(p, 9) = 0x9c;
    let nacoes = cqg7_d(0xae23a8);
    let nac = nacoes.wrapping_add((nacao_id as isize * 0x122) as usize);
    core::ptr::write_unaligned((p + 0x1a) as *mut usize, nac);
    if cqg7_i(nac + 0x88) < 100 && cqg7_sortear(4) == 0 {
        let k = cqg7_sortear(cqg7_i(0xae2350));
        let outra = cqg7_d(0xae23a8).wrapping_add((k as isize * 0x122) as usize);
        if cqg7_d(outra + 0x71) != 0 && cqg7_i(outra + 0x88) < 100 {
            core::ptr::write_unaligned((p + 0x1a) as *mut usize, outra);
        }
    }
    if nacao2_id > 0 && nacao2_id < cqg7_i(0xae2350) {
        if !(nacao_id == cqg7_i(0x9cf488) && cqg7_sortear(100) == 0) {
            if nacao2_id != cqg7_i(0x9cf424) && cqg7_sortear(3) != 0 {
                core::ptr::write_unaligned((p + 0x1e) as *mut u32, 0);
            } else {
                core::ptr::write_unaligned((p + 0x1e) as *mut usize, cqg7_d(0xae23a8).wrapping_add((nacao2_id as isize * 0x122) as usize));
            }
        }
    } else {
        core::ptr::write_unaligned((p + 0x1e) as *mut u32, 0);
    }
    let nome1: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x539c60usize);
    let nome2: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a1b0usize);
    let nome3: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53a5b0usize);
    let v = nome1(cqg7_d(p + 0x1a));
    core::ptr::write_unaligned((p + 4) as *mut usize, v);
    let v = nome2(cqg7_d(p + 0x1a));
    core::ptr::write_unaligned((p + 8) as *mut usize, v);
    let v = nome3(cqg7_d(p + 0x1a));
    core::ptr::write_unaligned((p + 0xc) as *mut usize, v);
    cqg7_com(0x5fff30, cqg7_d(0xb63c98), p);
    cqg7_pw(p + 0x40, cqg7_w(0xae2c92));
    cqg7_pw(p + 0x3e, cqg7_w(0xae2c90));
    core::ptr::write_unaligned((p + 0x39) as *mut u32, 0);
    core::ptr::write_unaligned((p + 0x24) as *mut u32, 0);
    core::ptr::write_unaligned((p + 0x4e) as *mut u32, 0);
    core::ptr::write_unaligned((p + 0x52) as *mut u32, 0);
    // os idiomas: a original empurra o dword do argumento com o byte de baixo trocado (as chamadas leem o byte)
    let tirar_idioma: unsafe extern "thiscall" fn(usize, usize, i8) = core::mem::transmute(0x7c6bf0usize);
    for k in 1..=0x40i8 {
        tirar_idioma(cqg7_d(0xb74350), p, k);
    }
    let peso: unsafe extern "stdcall" fn(usize, i8) -> i16 = core::mem::transmute(0x7c6930usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x7c6a30usize);
    let base = (0x14i16.wrapping_sub(cqg7_b(p + 0x56) as i16)).wrapping_mul(10);
    for k in 1..=0x40i8 {
        let v = peso(p, k);
        if v != 0 && v >= base {
            let q = (v as i32 - base as i32) / 0x4b;
            // ponytail: a fluência vai num dword da pilha com só o byte de baixo gravado (a 0x7c6a30 lê o byte)
            let nivel = if q > 0xa { 0xa } else { q as u8 };
            acrescentar(cqg7_d(0xb74350), p, k as u8 as u32, nivel as u32, 1);
        }
    }
    *((p + 0x3d) as *mut u8) = 0xb;
    *((p + 0x28) as *mut u8) = 0xb;
    *cqg7_r79(p, 0xb) &= 0xcf;
    let f = cqg7_d(p + 0x61);
    let nac = cqg7_d(p + 0x1a);
    let idade: i8;
    if nac != 0 && *((nac + 0x11c) as *const u8) == 0 && cqg7_i(nac + 0x88) < 0x15e {
        // a nação pequena: a idade pela reputação dela e a habilidade pela PA
        let r1 = cqg7_sortear(cqg7_i(nac + 0x88));
        let r2 = cqg7_sortear(1000);
        idade = if r1 < r2 {
            (cqg7_sortear(5) as i8).wrapping_add(0x17)
        } else {
            (cqg7_sortear(7) as i8).wrapping_add(0x11)
        };
        crate::comparacao::ramo("cqg_7 nacao pequena");
        if idade > 0x14 {
            cqg7_habilidade(f, 0x96a5d8, 0x9689e0);
        } else if idade > 0x1a {
            cqg7_habilidade(f, 0x968918, 0x968e48);
        } else if idade > 0x17 {
            cqg7_habilidade(f, 0x968fb0, 0x96a688);
        } else {
            cqg7_habilidade(f, 0x968e48, 0x9689e0);
        }
        let r = cqg7_sortear(0x2ee);
        cqg7_pw(f + 9, (r as i16).wrapping_add(cqg7_w(f + 5).wrapping_mul(0x1e)).wrapping_add(0xfa));
        let mut l121 = idade <= 0x12;
        if !l121 && cqg7_w(f + 7) > 0x8c && cqg7_w(f + 5) > 0x64 && cqg7_w(f + 9) < 0x1964 && idade < 0x16 && cqg7_sortear(4) != 0 {
            cqg7_pw(f + 9, cqg7_mul(cqg7_w(f + 9), 0x968978));
        }
        if idade <= 0x12 {
            l121 = true;
        }
        if l121 {
            let a = cqg7_w(f + 5);
            let r = cqg7_sortear(0xc8) as i16;
            let v = if a < 0x8c {
                r.wrapping_add(cqg7_w(f + 5).wrapping_mul(5)).wrapping_add(1)
            } else {
                r.wrapping_add(cqg7_w(f + 5).wrapping_add(0x19).wrapping_mul(10))
            };
            cqg7_pw(f + 0xd, v);
        } else {
            let a = cqg7_w(f + 5);
            if a < 0x78 {
                let r = cqg7_sortear(0xc8) as i16;
                cqg7_pw(f + 0xd, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(0xf)).wrapping_add(1));
            } else if idade > 0x15 && a > 0x78 {
                let r = cqg7_sortear(0xc8) as i16;
                cqg7_pw(f + 0xd, r.wrapping_add(cqg7_w(f + 5).wrapping_add(0xa).wrapping_mul(0x19)));
            } else {
                let r = cqg7_sortear(0xc8) as i16;
                let soma = if a < 0x8c { 1 } else { 0xfa };
                cqg7_pw(f + 0xd, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(0x14)).wrapping_add(soma));
            }
        }
        if cqg7_w(f + 9) < 0x8ca {
            let r = cqg7_sortear(5);
            cqg7_pw(f + 0xd, (r as i16).wrapping_add(1));
        }
        if cqg7_w(f + 9) < cqg7_w(f + 0xd) {
            cqg7_pw(f + 9, cqg7_w(f + 0xd).wrapping_add(0xa));
            cqg7_pw(f + 0xd, 1);
        }
        let r = cqg7_sortear(0x32);
        cqg7_pw(f + 0xb, (r as i16).wrapping_add(cqg7_w(f + 9)));
        if idade > 0x17 {
            if cqg7_w(f + 9) < 0x1770 {
                cqg7_pw(f + 9, cqg7_mul(cqg7_w(f + 9), 0x969588));
            }
            if cqg7_w(f + 0xb) < 0x1770 {
                cqg7_pw(f + 0xb, cqg7_mul(cqg7_w(f + 0xb), 0x969588));
            }
            let a = cqg7_w(f + 5);
            if a < cqg7_mul(a, 0x96a5d0) && cqg7_sortear(4) != 0 {
                cqg7_pw(f + 5, cqg7_mul(cqg7_w(f + 5), 0x96a5d0));
            }
        } else if idade < 0x14 {
            if cqg7_w(f + 5) < cqg7_mul(cqg7_w(f + 7), 0x968968) && cqg7_sortear(4) != 0 {
                cqg7_pw(f + 5, cqg7_mul(cqg7_w(f + 7), 0x968968));
            }
            if idade < 0x12 {
                cqg7_teto(f, 0x78, 0x96);
            } else {
                cqg7_teto(f, 0x78, 0x32);
            }
        } else {
            if cqg7_w(f + 9) < 0x1676 {
                cqg7_pw(f + 9, cqg7_mul(cqg7_w(f + 9), 0x968928));
            }
            if cqg7_w(f + 0xb) < 0x1676 {
                cqg7_pw(f + 0xb, cqg7_mul(cqg7_w(f + 0xb), 0x968928));
            }
            if cqg7_w(f + 5) < cqg7_mul(cqg7_w(f + 7), 0x968e48) && cqg7_sortear(4) != 0 {
                cqg7_pw(f + 5, cqg7_mul(cqg7_w(f + 7), 0x968e48));
            }
            cqg7_teto(f, 0x8c, 0x32);
        }
    } else {
        // a nação grande ou sem nação: a habilidade pela posição e a idade por sorteio(10000)
        if *((f + 0x15) as *const u8) == 0x14 {
            cqg7_habilidade(f, 0x968e48, 0x968e80);
        } else if *((f + 0x11) as *const u8) == 0x14 || *((f + 0x10) as *const u8) == 0x14 {
            cqg7_habilidade(f, 0x96a5d8, 0x9689e0);
        } else if *((f + 0xf) as *const u8) == 0x14 {
            cqg7_habilidade(f, 0x968968, 0x9689e0);
        } else {
            cqg7_habilidade(f, 0x968e48, 0x9689e0);
        }
        if cqg7_sortear(0xfa) != 0 {
            let r = cqg7_sortear(0x2ee) as i16;
            cqg7_pw(f + 9, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(0x14)).wrapping_add(0xfa)); // cqg7: a reputação +9 do regen
            crate::comparacao::ramo("cqg_7 reputacao do regen");
            let r = cqg7_sortear(0x14) as i16;
            cqg7_pw(f + 0xd, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(5)).wrapping_add(1));
        } else {
            let r = cqg7_sortear(0x2ee) as i16;
            cqg7_pw(f + 9, r.wrapping_add(cqg7_w(f + 5).wrapping_mul(0xf)).wrapping_add(0x7d));
            cqg7_pw(f + 0xd, 1);
        }
        if cqg7_w(f + 9) < cqg7_w(f + 0xd) {
            cqg7_pw(f + 9, cqg7_w(f + 0xd).wrapping_add(0xa));
            cqg7_pw(f + 0xd, 1);
        }
        let r = cqg7_sortear(0x32);
        cqg7_pw(f + 0xb, (r as i16).wrapping_add(cqg7_w(f + 9)));
        let s = cqg7_sortear(0x2710);
        crate::comparacao::ramo("cqg_7 nacao grande");
        if s == 0 {
            idade = (cqg7_sortear(4) as i8).wrapping_add(0xf);
            if idade == 0xf {
                cqg7_teto(f, 0x50, 0xfa);
            } else {
                cqg7_teto(f, 0x64, 0x96);
            }
        } else if s < 0x1388 {
            idade = (cqg7_sortear(3) as i8).wrapping_add(0x10); // cqg7: a idade de 16 a 18
            crate::comparacao::ramo("cqg_7 idade 16 a 18");
            if idade < 0x12 {
                cqg7_teto(f, 0x50, 0xfa);
            } else {
                cqg7_teto(f, 0x64, 0x96);
            }
        } else if s > 0x26f7 {
            idade = (cqg7_sortear(6) as i8).wrapping_add(0x15);
        } else {
            idade = (cqg7_sortear(4) as i8).wrapping_add(0x11);
            if idade == 0x11 {
                cqg7_teto(f, 0x5a, 0xfa);
            } else {
                cqg7_teto(f, 0x6e, 0x96);
            }
        }
    }
    let grande: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x549ea0usize);
    if grande(cqg7_d(p + 0x1a)) == 0 {
        if idade < 0x19 && cqg7_w(f + 5) < 0x78 {
            cqg7_pw(f + 0xd, 1);
        } else {
            cqg7_pw(f + 0xd, cqg7_mul(cqg7_w(f + 0xd), 0x96a5d0));
        }
    }
    cqg7_pw(p + 0x12, cqg7_w(0xae2c92).wrapping_sub(idade as i16)); // cqg7: o ano do nascimento
    let r = cqg7_sortear(0x16d);
    cqg7_pw(p + 0x10, r as i16);
    let anos: unsafe extern "C" fn(usize, *const u16) -> u16 = core::mem::transmute(0x53ab90usize);
    let a = anos(p, 0xae2c90 as *const u16);
    cqg7_pw(p + 0x18, a as u8 as i8 as i16);
    // +0x1b
    if cqg7_w(f + 7) > 0x64 && (cqg7_b(f + 0x15) >= 0xf || cqg7_b(f + 0x14) >= 0xf) {
        if cqg7_sortear(3) != 0 {
            let r = cqg7_sortear(0xb);
            cqg7_pb(f + 0x1b, (r as i8).wrapping_add(0xa));
        } else if cqg7_sortear(3) == 0 {
            let r = cqg7_sortear(7);
            cqg7_pb(f + 0x1b, (r as i8).wrapping_add(0xe));
        } else {
            let r = cqg7_sortear(0x10);
            cqg7_pb(f + 0x1b, (r as i8).wrapping_add(5));
        }
    } else if cqg7_sortear(4) != 0 {
        let r = cqg7_sortear(0xa);
        cqg7_pb(f + 0x1b, (r as i8).wrapping_add(8));
    } else {
        let r = cqg7_sortear(0x10);
        cqg7_pb(f + 0x1b, (r as i8).wrapping_add(5));
    }
    cqg7_acertar(f + 0x1b, 5, 5, 0xa, 0xa, 8);
    // +0x36 pelo +0x1b
    let a = cqg7_b(f + 0x1b);
    let v = if a > 0xf {
        (cqg7_sortear(0xb) as i8).wrapping_add(0xa)
    } else if a > 0xa {
        (cqg7_sortear(0xa) as i8).wrapping_add(8)
    } else if a < 5 {
        (cqg7_sortear(0xa) as i8).wrapping_add(1)
    } else if cqg7_sortear(4) != 0 {
        (cqg7_sortear(0x10) as i8).wrapping_add(5)
    } else {
        (cqg7_sortear(0xa) as i8).wrapping_add(5)
    };
    cqg7_pb(f + 0x36, v);
    // +0x1d (o goleiro de 20 com 4/5 pela PA)
    if cqg7_b(f + 0xf) == 0x14 && cqg7_sortear(5) != 0 {
        let r = cqg7_sortear(0x14);
        let v = if cqg7_w(f + 7) as i32 > r + 0x64 {
            (cqg7_sortear(0xb) as i8).wrapping_add(0xa)
        } else {
            (cqg7_sortear(0x10) as i8).wrapping_add(5)
        };
        cqg7_pb(f + 0x1d, v);
    } else {
        let r = cqg7_sortear(0x14);
        cqg7_pb(f + 0x1d, (r as i8).wrapping_add(1));
        cqg7_acertar(f + 0x1d, 6, 5, 6, 0, 0);
    }
    // +0x2e
    let mut k: i8 = -5;
    if cqg7_b(f + 0x11) >= 0xf {
        k = if cqg7_b(f + 0x19) >= 0xf { 5 } else { 0 };
    } else if ((cqg7_b(f + 0x13) >= 0xf || cqg7_b(f + 0x12) >= 0xf) && cqg7_b(f + 0x19) >= 0xf)
        || cqg7_b(f + 0xf) >= 0xf
        || cqg7_b(f + 0x15) >= 0xf
    {
        k = 0;
    }
    let r1 = cqg7_sortear(k as i32 * 3);
    let r2 = cqg7_sortear(0x14);
    cqg7_pb(f + 0x2e, (r1 as i8).wrapping_add(r2 as i8));
    let a = cqg7_b(f + 0x2e);
    cqg7_pb(f + 0x2e, if a < 1 { 1 } else if a > 0x14 { 0x14 } else { a });
    // +0x34
    let r = cqg7_sortear(0x14);
    cqg7_pb(f + 0x34, (r as i8).wrapping_add(1));
    cqg7_acertar(f + 0x34, 6, 8, 0xa, 0, 0);
    // +0x44
    let r = cqg7_sortear(0x14);
    cqg7_pb(f + 0x44, (r as i8).wrapping_add(1));
    if cqg7_b(f + 0x44) > 0xf && cqg7_sortear(3) != 0 {
        cqg7_tirar(f + 0x44, 5);
    } else {
        let a = cqg7_b(f + 0x44);
        if a < 5 {
            cqg7_subir_abaixo(f + 0x44, 5, 0xa);
        } else if a < 0xa {
            cqg7_subir_abaixo(f + 0x44, 0xa, 5);
        }
    }
    // +0x3c pela idade
    if idade < 0x12 {
        let r = cqg7_sortear(0x14);
        cqg7_pb(f + 0x3c, (r as i8).wrapping_add(1));
        cqg7_acertar(f + 0x3c, 8, 5, 8, 0xa, 6);
    } else if idade < 0x18 {
        let r = cqg7_sortear(0x12);
        cqg7_pb(f + 0x3c, (r as i8).wrapping_add(3));
        cqg7_acertar(f + 0x3c, 5, 5, 0xa, 0xa, 5);
    } else {
        let r = cqg7_sortear(0xf);
        cqg7_pb(f + 0x3c, (r as i8).wrapping_add(6));
        cqg7_acertar(f + 0x3c, 3, 0xa, 8, 0, 0);
    }
    if cqg7_b(f + 0x3c) < 0xa && cqg7_b(f + 0x44) > 0xf {
        let r = cqg7_sortear(5);
        cqg7_pb(f + 0x3c, cqg7_b(f + 0x3c).wrapping_add(r as i8));
    }
    // +0x3d pela idade
    let r = cqg7_sortear(0x14);
    cqg7_pb(f + 0x3d, (r as i8).wrapping_add(1));
    if idade < 0x12 {
        cqg7_acertar(f + 0x3d, 0xa, 5, 6, 0, 0);
    } else if idade < 0x18 {
        cqg7_acertar(f + 0x3d, 8, 5, 8, 0, 0);
    } else {
        cqg7_acertar(f + 0x3d, 5, 5, 0xa, 0, 0);
    }
    if (cqg7_b(f + 0x11) == 0x14 || cqg7_b(f + 0x12) == 0x14) && cqg7_b(f + 0x3d) < 0xa {
        let r = cqg7_sortear(5);
        cqg7_pb(f + 0x3d, cqg7_b(f + 0x3d).wrapping_add(r as i8));
    }
    if cqg7_b(f + 0x2e) > 0xf && cqg7_b(f + 0x3d) < 0xa {
        let r = cqg7_sortear(5);
        cqg7_pb(f + 0x3d, cqg7_b(f + 0x3d).wrapping_add(r as i8));
    }
    // +0x1e, +0x24, +0x2c, +0x2f (e +0x21) pela idade
    if idade < 0x18 {
        if idade < 0x12 {
            cqg7_tirar(f + 0x1e, 0x28);
            cqg7_tirar(f + 0x24, 0x28);
            let a = cqg7_b(f + 0x2c);
            if a > 0xf {
                cqg7_tirar(f + 0x2c, 8);
            } else if a > 0xa {
                cqg7_tirar(f + 0x2c, 5);
            }
            let a = cqg7_b(f + 0x2f);
            if a > 0xf {
                cqg7_tirar(f + 0x2f, 0xa);
            } else if a > 0xa {
                cqg7_tirar(f + 0x2f, 8);
            }
        } else {
            cqg7_tirar(f + 0x1e, 0x14);
            cqg7_tirar(f + 0x24, 0x14);
            if cqg7_b(f + 0x2c) > 0xf {
                cqg7_tirar(f + 0x2c, 5);
            }
            let a = cqg7_b(f + 0x2f);
            if a > 0xf {
                cqg7_tirar(f + 0x2f, 8);
            } else if a > 0xa {
                cqg7_tirar(f + 0x2f, 5);
            }
        }
        if cqg7_b(f + 0x2f) > 0xf {
            cqg7_tirar(f + 0x2f, 8);
        }
    } else if idade > 0x19 && idade < 0x1d {
        cqg7_subir_abaixo(f + 0x3d, 5, 8);
        cqg7_subir_abaixo(f + 0x21, 5, 8);
        cqg7_subir_abaixo(f + 0x3c, 5, 8);
        cqg7_tirar(f + 0x1e, 0xa);
        cqg7_tirar(f + 0x24, 0xa);
        cqg7_subir_abaixo(f + 0x2c, 5, 8);
        let a = cqg7_b(f + 0x2f);
        if a > 0xf {
            cqg7_tirar(f + 0x2f, 5);
        } else if a > 0xa {
            cqg7_tirar(f + 0x2f, 3);
        }
    }
    // o clube
    let nac = cqg7_d(p + 0x1a);
    if idade < 0x13 && nac != 0 && *((nac + 0x11c) as *const u8) != 0 && cqg7_sortear(4) != 0 {
        let melhor: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7ad0e0usize);
        let contratar: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(0x7a6f10usize);
        let c = melhor(this, p);
        if c != 0 && contratar(p, c) != 0 {
            crate::comparacao::ramo("cqg_7 melhor clube");
            let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
            if humano(c) != 0 {
                let noticia: unsafe extern "stdcall" fn(usize, usize) = core::mem::transmute(0x7a8ba0usize);
                noticia(p, c);
            }
            if cqg7_w(cqg7_d(p + 0x61) + 7) > 0x50 {
                let t = cqg7_d(c + 0xbf);
                if t != 0 && cqg7_d(t + 0x69) != 0 {
                    let r = cqg7_sortear(100);
                    if cqg7_b(cqg7_d(t + 0x69) + 0x16) as i32 > r {
                        let gostar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547a10usize);
                        gostar(t, p, 0);
                    }
                }
            }
        }
    } else if (nac != 0 && *((nac + 0x11c) as *const u8) == 0 && cqg7_sortear(0xf) != 0) || cqg7_sortear(0xa) == 0 {
        let da_nacao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7ad590usize);
        let c = da_nacao(this, p);
        if c != 0 {
            let contratar: unsafe extern "stdcall" fn(usize, usize) -> u8 = core::mem::transmute(0x7a6f10usize);
            contratar(p, c);
        }
    }
    if cqg7_d(p + 0x39) == 0 && idade < 0x14 {
        let f = cqg7_d(p + 0x61);
        if cqg7_w(f + 0xb) > 0x4e2 {
            cqg7_pw(f + 0xb, 0x4e2);
        }
        if cqg7_w(f + 9) > 0x4e2 {
            cqg7_pw(f + 9, 0x4e2);
        }
        cqg7_pw(f + 0xd, 1);
    }
    let fisicos: unsafe extern "C" fn(usize) = core::mem::transmute(0x538ec0usize);
    let atributos: unsafe extern "C" fn(usize) = core::mem::transmute(0x538bc0usize);
    fisicos(p);
    atributos(p);
}

/// Comparador: só a original (ponytail: a rotina tira a ficha do banco, limpa listas e históricos, aloca idiomas e o
/// contrato e manda notícia; não pode rodar duas vezes). A cópia roda no modo Rust (contadores "cqg_7 ...") e é provada
/// pelas mutações e pelas duas carreiras Rust.
unsafe extern "thiscall" fn comparar_refazer_como_jovem(this: usize, p: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1630)(this, p)
}

/// { 0x56a090 } (thiscall, ret 4, dispute.cpp): `DISPUTE::remove_dispute(pessoa)`: sem pessoa ou sem a tabela de marcas
/// (+4), ou com a marca da pessoa zerada: nada. Senão zera a marca e, com a lista (+0), volta o cursor ao começo (+0xa =
/// +2) e procura, nas `conta` (+0, palavra) primeiras, a entrada do id da pessoa (0x68d1b0): achando, tira-a (0x68d1f0).
unsafe extern "thiscall" fn tirar_da_disputa(this: usize, p: usize) {
    if p == 0 || cqg7_d(this + 4) == 0 {
        return;
    }
    let m = (cqg7_d(this + 4) as isize + cqg7_i(p) as isize) as *mut u8;
    if *m == 0 {
        return;
    }
    *m = 0;
    let lista = cqg7_d(this);
    if lista == 0 {
        return;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cqg7_d(lista + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut k = 0i32;
    while (k as i16 as i32) < cqg7_w(cqg7_d(this)) as i32 {
        let no = proximo(cqg7_d(this));
        if no == 0 {
            return;
        }
        if cqg7_i(no) == cqg7_i(p) {
            crate::comparacao::ramo("cqg_7 saiu da disputa");
            let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
            tirar(cqg7_d(this), no);
            return;
        }
        k += 1;
    }
}

/// Comparador: sem a marca (nada a fazer), lado a lado na marca e no cursor da lista. Com a marca, só a original com o
/// plano: a marca zerada e a conta da lista uma a menos quando a pessoa está nela (a procura feita antes, com o cursor
/// devolvido).
unsafe extern "thiscall" fn comparar_tirar_da_disputa(this: usize, p: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1631);
    if p == 0 || cqg7_d(this + 4) == 0 {
        return o(this, p);
    }
    let m = cqg7_d(this + 4).wrapping_add(cqg7_i(p) as isize as usize);
    let lista = cqg7_d(this);
    if *(m as *const u8) == 0 {
        let mut reg = vec![(m, 1)];
        if lista != 0 {
            reg.push((lista, 0xe));
        }
        let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, p), || tirar_da_disputa(this, p));
        PLACAR.registrar(igual, || format!("tirar_da_disputa(pessoa {:x}) {:x?} x {:x?}", p, mo.0, mr.0));
        return;
    }
    let mut achou = 0i16;
    let conta_antes = if lista != 0 { cqg7_w(lista) } else { 0 };
    if lista != 0 {
        let cursor = cqg7_d(lista + 0xa);
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, cqg7_d(lista + 2) as u32);
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        for _ in 0..conta_antes.max(0) {
            let no = proximo(lista);
            if no == 0 {
                break;
            }
            if cqg7_i(no) == cqg7_i(p) {
                achou = 1;
                break;
            }
        }
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, cursor as u32);
    }
    o(this, p);
    let conta_depois = if lista != 0 { cqg7_w(lista) } else { 0 };
    PLACAR.registrar(*(m as *const u8) == 0 && conta_depois == conta_antes - achou,
        || format!("tirar_da_disputa(pessoa {:x}) conta {} -> {} (achou {})", p, conta_antes, conta_depois, achou));
}

// ---------- 271a leva: a formação lida do arquivo .pct (as táticas da IA e as do humano) e a troca da ordem dos bytes de palavras e dwords (formation.cpp, cash.cpp) ----------

unsafe fn cqg8_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cqg8_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// fread (0x945bfe, cdecl): `n` registros de `tam` bytes; devolve quantos leu.
unsafe fn cqg8_ler(buf: usize, tam: u32, f: usize) -> u32 {
    let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    fread(buf, tam, 1, f)
}

/// fclose (0x945a09, cdecl).
unsafe fn cqg8_fechar(f: usize) {
    let fclose: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    fclose(f);
}

/// A caixa de erro de formation.cpp (0x9cc3dc) na linha `linha`. A original também formata a mensagem num texto local
/// (sprintf 0x944c9f) que não sai dali.
unsafe fn cqg8_erro(linha: u32) {
    aviso_em(0x9cc3dc, linha);
}

/// A troca dos bytes de um dword (a original faz em linha).
fn cqg8_inverter(v: u32) -> u32 {
    v.swap_bytes()
}

/// A leitura que falha e encerra: sem `quieto`, a caixa; o arquivo fecha e devolve 0.
unsafe fn cqg8_falhou(quieto: u32, linha: u32, f: usize) -> u32 {
    if quieto == 0 {
        cqg8_erro(linha);
    }
    cqg8_fechar(f);
    0
}

/// { 0x5b5d10 } (thiscall, ret 0xc, formation.cpp): `Formation::load_from_disk(caminho, quieto, ofuscado)`, pela
/// criação das táticas (0x893dc0: as formações da IA e as pré-definidas, 45 na carreira) e pelas telas de tática.
/// Caminho nulo (linha 0x8d8) ou o arquivo que não abre (0x8e0): 0 (sem `quieto`, a caixa). Lê o identificador (4 bytes;
/// invertido se o marcador do arquivo 0x90c140 não é o nativo; menos 123456789 se ofuscado): fora de 0x98ec54..0x98ec5e
/// "não é formação" (0x8f7), abaixo de 0x98ec59 "versão antiga" (0x900). Depois, na ordem: a senha (+0x33, 50 bytes,
/// cada byte invertido, 0xff fica), o criador (+0x65, 200), a máscara (+0x187b, a partir de 0x98ec5a; antes 0), as 11
/// posições gerais (+0x12d, palavras) e a máscara refeita (0x5b59a0), a ordem de exibição (+0x143, 11 bytes, a partir de
/// 0x98ec5c; antes zeros), as 11 posições extras (+0x14e), as casas (+0x164, 0x420 bytes = 0x210 palavras), nas versões
/// 0x98ec59/0x98ec5a 0x108 bytes descartados, as instruções do time (+0x584, dword), as instruções dos jogadores (abaixo
/// de 0x98ec5e 11 dwords, cada um com 10 ao lado, em +0x188b de 8 em 8; depois 0x58 bytes em +0x188b) e os tipos de
/// marcação (+0x5b4, 11 bytes); as três últimas, se falham, só mostram a caixa. As instruções do time ganham 0x2000
/// (sem 0x4000) e, abaixo de 0x98ec5d, 0x20000. As 11 vagas refazem as posições pela casa guardada (0x5b3c70 e
/// 0x5b3560 com a fase 0 e a faixa 0); o nome (+0) é o do arquivo (_splitpath 0x944cff); o mapa das casas (0x5b7160);
/// +0x187f = 1, +0x1883 = senha não vazia, +0x1887 = 1. Fecha e devolve 1. Os arquivos não nativos trocam a ordem dos
/// bytes (0x43f2d0 palavras, 0x43f300 dwords, 0x43f350 qwords).
unsafe extern "thiscall" fn ler_formacao_do_disco(this: usize, caminho: usize, quieto: u32, ofuscado: u32) -> u32 {
    if caminho == 0 {
        if quieto == 0 {
            cqg8_erro(0x8d8);
        }
        return 0;
    }
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let f = fopen(caminho, 0x98f928);
    if f == 0 {
        if quieto == 0 {
            cqg8_erro(0x8e0);
        }
        return 0;
    }
    let marcador: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x90c140usize);
    let nativo = marcador(f) != 0xf1ff_fff1;
    let mut id_ = [0u8; 4];
    if cqg8_ler(id_.as_mut_ptr() as usize, 4, f) != 1 {
        return cqg8_falhou(quieto, 0x8ea, f);
    }
    let mut id = u32::from_le_bytes(id_);
    if !nativo {
        id = cqg8_inverter(id);
    }
    if ofuscado != 0 {
        id = id.wrapping_sub(0x75bcd15);
    }
    let id = id as i32;
    if !(0x98ec54..=0x98ec5e).contains(&id) {
        return cqg8_falhou(quieto, 0x8f7, f);
    }
    if id < 0x98ec59 {
        return cqg8_falhou(quieto, 0x900, f);
    }
    let senha = this + 0x33;
    if cqg8_ler(senha, 0x32, f) != 1 {
        return cqg8_falhou(quieto, 0x909, f);
    }
    let strlen = |a: usize| (0..).take_while(|&k| *((a + k) as *const u8) != 0).count();
    if strlen(senha) != 0 {
        let mut k = 0usize;
        loop {
            let b = *((senha + k) as *const u8);
            *((senha + k) as *mut u8) = if b == 0xff { 0xff } else { !b }; // cqg8: a senha invertida
            k += 1;
            if k >= strlen(senha) {
                break;
            }
        }
    }
    if cqg8_ler(this + 0x65, 200, f) != 1 {
        return cqg8_falhou(quieto, 0x915, f);
    }
    let troca16: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x43f2d0usize);
    if id >= 0x98ec5a {
        if cqg8_ler(this + 0x187b, 4, f) != 1 {
            return cqg8_falhou(quieto, 0x920, f);
        }
        if !nativo {
            cqg8_pd(this + 0x187b, cqg8_inverter(cqg8_d(this + 0x187b)));
        }
    } else {
        cqg8_pd(this + 0x187b, 0);
    }
    if cqg8_ler(this + 0x12d, 0x16, f) != 1 {
        return cqg8_falhou(quieto, 0x930, f);
    }
    if !nativo {
        troca16(this + 0x12d, 0xb);
    }
    let mascara: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5b59a0usize);
    mascara(this);
    if id >= 0x98ec5c {
        if cqg8_ler(this + 0x143, 0xb, f) != 1 {
            return cqg8_falhou(quieto, 0x940, f);
        }
    } else {
        core::ptr::write_bytes((this + 0x143) as *mut u8, 0, 0xb);
    }
    if cqg8_ler(this + 0x14e, 0x16, f) != 1 {
        return cqg8_falhou(quieto, 0x94f, f);
    }
    if !nativo {
        troca16(this + 0x14e, 0xb);
    }
    if cqg8_ler(this + 0x164, 0x420, f) != 1 {
        return cqg8_falhou(quieto, 0x959, f);
    }
    if !nativo {
        troca16(this + 0x164, 0x210);
    }
    if id == 0x98ec59 || id == 0x98ec5a {
        let mut descarte = [0u8; 0x108];
        if cqg8_ler(descarte.as_mut_ptr() as usize, 0x108, f) != 1 {
            return cqg8_falhou(quieto, 0x968, f);
        }
        if !nativo {
            troca16(descarte.as_mut_ptr() as usize, 0x84);
        }
    }
    let time = this + 0x584;
    if cqg8_ler(time, 4, f) == 1 {
        if !nativo {
            cqg8_pd(time, cqg8_inverter(cqg8_d(time)));
        }
    } else if quieto == 0 {
        cqg8_erro(0x971);
    }
    if id < 0x98ec5e {
        // ponytail: a original copia o vetor local mesmo quando a leitura falha (sobra de pilha num arquivo truncado);
        // a cópia começa com zeros
        let mut v = [0u32; 11];
        if cqg8_ler(v.as_mut_ptr() as usize, 0x2c, f) == 1 {
            if !nativo {
                let troca32: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x43f300usize);
                troca32(v.as_mut_ptr() as usize, 0xb);
            }
        } else if quieto == 0 {
            cqg8_erro(0x97b);
        }
        for (k, x) in v.iter().enumerate() {
            cqg8_pd(this + 0x188b + 8 * k, *x);
            cqg8_pd(this + 0x188b + 8 * k + 4, 0xa); // cqg8: a instrução padrão das versões antigas
        }
        crate::comparacao::ramo("cqg_8 instrucoes antigas");
    } else if cqg8_ler(this + 0x188b, 0x58, f) == 1 {
        if !nativo {
            let troca64: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x43f350usize);
            troca64(this + 0x188b, 0xb);
        }
    } else if quieto == 0 {
        cqg8_erro(0x988);
    }
    if cqg8_ler(this + 0x5b4, 0xb, f) != 1 && quieto == 0 {
        cqg8_erro(0x98f);
    }
    let mut v = cqg8_d(time);
    if v & 0x4000 == 0 {
        v |= 0x2000; // cqg8: a instrução 0x2000 sem a 0x4000
        cqg8_pd(time, v);
        crate::comparacao::ramo("cqg_8 instrucao 0x2000");
    }
    if id < 0x98ec5d {
        cqg8_pd(time, cqg8_d(time) | 0x20000);
    }
    let guardada: unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32 = core::mem::transmute(0x5b3c70usize);
    let refazer: unsafe extern "thiscall" fn(usize, i32, i32, u32, u32) -> u32 = core::mem::transmute(0x5b3560usize);
    let mut c = 0u32;
    for vaga in 0..0xbi32 {
        guardada(this, vaga as i8, 0, 0, &mut c); // cqg8: a casa guardada da fase 0
        crate::comparacao::ramo("cqg_8 posicoes refeitas");
        refazer(this, vaga, 0, 0, c); // cqg8: as posições refeitas pela casa
    }
    let separar: unsafe extern "C" fn(usize, usize, usize, usize, usize) = core::mem::transmute(0x944cffusize);
    separar(caminho, 0, 0, this, 0);
    let mapa: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5b7160usize);
    mapa(this);
    cqg8_pd(this + 0x187f, 1);
    cqg8_pd(this + 0x1883, (*((this + 0x33) as *const u8) != 0) as u32);
    cqg8_pd(this + 0x1887, 1);
    cqg8_fechar(f);
    1
}

/// Comparador: lado a lado na formação (0x18e3 bytes: até as instruções dos jogadores em +0x188b + 0x58) e na
/// devolução; o arquivo é aberto e lido pelas duas (só leitura).
unsafe extern "thiscall" fn comparar_ler_formacao_do_disco(this: usize, caminho: usize, quieto: u32, ofuscado: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(1632);
    if this == 0 {
        return o(this, caminho, quieto, ofuscado);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x18e3)], || o(this, caminho, quieto, ofuscado),
        || ler_formacao_do_disco(this, caminho, quieto, ofuscado));
    PLACAR.registrar(igual && r == n, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&k| a[k] != b[k]).take(8).collect();
        format!("ler_formacao_do_disco(this {:x}) original={} rust={}, bytes diferentes {:x?}", this, r, n, dif)
    });
    r
}

/// { 0x43f2d0 } (cdecl, cash.cpp): `swap_bytes_16_array(palavras, conta)`: troca os dois bytes de cada palavra.
unsafe extern "C" fn trocar_bytes_das_palavras(p: usize, conta: u32) {
    for k in 0..conta as usize {
        let a = (p + 2 * k) as *mut u16;
        core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).swap_bytes());
    }
}

/// Comparador: lado a lado no vetor (até 0x420 palavras conferidas).
unsafe extern "C" fn comparar_trocar_bytes_das_palavras(p: usize, conta: u32) {
    let o = orig::<unsafe extern "C" fn(usize, u32)>(1633);
    if p == 0 || conta == 0 || conta > 0x1000 {
        return o(p, conta);
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(p, 2 * conta as usize)], || o(p, conta), || trocar_bytes_das_palavras(p, conta));
    PLACAR.registrar(igual, || format!("trocar_bytes_das_palavras({:x}, {})", p, conta));
}

/// { 0x43f300 } (cdecl, cash.cpp): `swap_bytes_32_array(dwords, conta)`: inverte os quatro bytes de cada dword.
unsafe extern "C" fn trocar_bytes_dos_dwords(p: usize, conta: u32) {
    for k in 0..conta as usize {
        cqg8_pd(p + 4 * k, cqg8_inverter(cqg8_d(p + 4 * k)));
    }
}

/// Comparador: lado a lado no vetor.
unsafe extern "C" fn comparar_trocar_bytes_dos_dwords(p: usize, conta: u32) {
    let o = orig::<unsafe extern "C" fn(usize, u32)>(1634);
    if p == 0 || conta == 0 || conta > 0x1000 {
        return o(p, conta);
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(p, 4 * conta as usize)], || o(p, conta), || trocar_bytes_dos_dwords(p, conta));
    PLACAR.registrar(igual, || format!("trocar_bytes_dos_dwords({:x}, {})", p, conta));
}

// ---------- 271a leva: os árbitros do jogo novo: os que ficam por nação, as vagas das competições e a parte de cada uma; a ordem das vagas pelo nível da competição (officials_manager.cpp) ----------

unsafe fn cqg9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqg9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqg9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqg9_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cqg9_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cqg9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O registro do árbitro `i` (0x2b bytes em [0xae23e8], [0xae2390] deles).
unsafe fn cqg9_arbitro(i: i32) -> usize {
    cqg9_d(0xae23e8).wrapping_add((i as isize * 0x2b) as usize)
}

/// O fim fatal da original sem memória: a caixa (linha `linha` de officials_manager.cpp, 0xa6a22c), 0x5cc7d0, 0x5a8f60
/// e exit(−1) (0x945934).
unsafe fn cqg9_fatal(linha: u32) -> ! {
    aviso_em(0xa6a22c, linha);
    let f1: unsafe extern "C" fn() = core::mem::transmute(0x5cc7d0usize);
    let f2: unsafe extern "C" fn() = core::mem::transmute(0x5a8f60usize);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(0x945934usize);
    f1();
    f2();
    sair(-1)
}

/// O qsort do jogo (0x945333) pela rotina de comparação `cmp`.
unsafe fn cqg9_ordenar(base: usize, n: u32, tam: u32, cmp: usize) {
    let q: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    q(base, n, tam, cmp);
}

/// calloc (0x945501, cdecl).
unsafe fn cqg9_calloc(n: u32, tam: u32) -> usize {
    let f: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    f(n, tam)
}

/// free (0x9452ca, cdecl).
unsafe fn cqg9_liberar(p: usize) {
    let f: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    f(p);
}

/// { 0x79bf60 } (thiscall, ret 0, officials_manager.cpp): `OfficialsManager::set_up_officials()`, pelo construtor
/// (0x79b580) no jogo novo. Revê os registros (0x79caa0), ordena por nação e pela qualidade (qsort 0x79d4e0) e, por
/// nação, fica com os 6 primeiros (nação com +0x11c zero) ou 61 (o primeiro de cada nação depois da primeira não fica:
/// a conta recomeça nele), copiados (0x79c530) numa tabela nova; os blocos por nação (this+0xc, 0x102 bytes: +0 a nação,
/// +4 os árbitros, +8 o primeiro, +0xc o pedido, +0x10 as competições e 15 vagas de 16 bytes em +0x12): cada liga
/// (0x4b0700, não amistosa 0x5c6710, com nação +0x5d) ganha uma vaga com o pedido (+0x3e da competição) / 2 + 10; a nação
/// com competições pede o maior × a quantidade e, faltando, sorteia os que faltam (0x79c5c0); sem competições e menos de
/// 5, sorteia até o +0x8e da nação (1..5). Os índices (+0), o primeiro de cada nação e, por nação, as vagas ordenadas
/// (qsort 0x79d410) com a parte de cada uma; this+4 a conta e this+8 as marcas (calloc).
unsafe extern "thiscall" fn montar_os_arbitros(this: usize) {
    let rever: unsafe extern "C" fn() = core::mem::transmute(0x79caa0usize);
    rever();
    cqg9_ordenar(cqg9_d(0xae23e8), cqg9_i(0xae2390) as u32, 0x2b, 0x79d4e0);
    let n = cqg9_i(0xae2390);
    let marcas = cqg9_calloc(n as u32, 1);
    let mut ficam = 0i32;
    if n > 0 {
        let mut atual = cqg9_d(cqg9_arbitro(0) + 0x16);
        let mut grupo = 0i32;
        for i in 0..n {
            let nac = cqg9_d(cqg9_arbitro(i) + 0x16);
            if nac != atual {
                grupo = 0;
                atual = nac;
                continue;
            }
            let m = (marcas + i as usize) as *mut u8;
            if nac == 0 {
                *m = 0;
                continue;
            }
            let limite = if *((nac + 0x11c) as *const u8) == 0 { 5 } else { 0x3c }; // cqg9: os árbitros que ficam por nação
            if grupo <= limite {
                ficam += 1;
                *m = 1;
                crate::comparacao::ramo("cqg_9 arbitro mantido");
            } else {
                *m = 0;
            }
            grupo += 1;
        }
    }
    let nova = cqg9_calloc(ficam as u32, 0x2b);
    let mut dst = nova;
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x79c530usize);
    let mut i = 0i32;
    while i < cqg9_i(0xae2390) {
        if *((marcas + i as usize) as *const u8) != 0 {
            copiar(dst, cqg9_arbitro(i));
            dst += 0x2b;
        }
        i += 1;
    }
    cqg9_liberar(cqg9_d(0xae23e8));
    cqg9_pd(0xae23e8, nova as u32);
    cqg9_pd(0xae2390, ficam as u32);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let nacoes = cqg9_i(0xae2350);
    let blocos = malloc((nacoes as u32).wrapping_mul(0x102));
    cqg9_pd(this + 0xc, blocos as u32);
    if blocos == 0 {
        cqg9_fatal(0x1ff);
    }
    for k in 0..nacoes.max(0) {
        let b = blocos + k as usize * 0x102;
        cqg9_pw(b + 0x10, 0);
        cqg9_pd(b + 4, 0);
        cqg9_pd(b + 8, 1_000_000);
        cqg9_pd(b + 0xc, 0);
        cqg9_pd(b, k as u32);
        for j in 0..0xf {
            let v = b + 0x12 + 16 * j;
            cqg9_pd(v, 1_000_000);
            cqg9_pd(v + 4, 0);
            cqg9_pd(v + 8, 0);
            cqg9_pd(v + 0xc, 0xffff_ffff);
        }
    }
    let ativa: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    for k in 0..cqg9_i(0xadadf8).max(0) {
        if cqg9_d(cqg9_d(0xadadfc) + 4 * k as usize) == 0 || ativa(k) == 0 {
            continue;
        }
        let comp = cqg9_d(0xae23d0).wrapping_add(k as usize * 0x6b);
        if amistoso(cqg9_d(0xae2c74), comp) != 0 {
            continue;
        }
        let nac = cqg9_d(comp + 0x5d);
        if nac == 0 {
            continue;
        }
        let b = blocos.wrapping_add((cqg9_i(nac) as isize * 0x102) as usize);
        let v = b.wrapping_add((cqg9_w(b + 0x10) as isize * 0x10) as usize);
        cqg9_pd(v + 0x1e, k as u32);
        let pedido = cqg9_w(cqg9_d(cqg9_d(0xadadfc) + 4 * k as usize) + 0x3e) as i32 / 2 + 0xa; // cqg9: o pedido da vaga
        crate::comparacao::ramo("cqg_9 vaga de competicao");
        cqg9_pd(v + 0x1a, pedido as u32);
        if cqg9_i(v + 0x1a) > cqg9_i(b + 0xc) {
            cqg9_pd(b + 0xc, cqg9_d(v + 0x1a) as u32);
        }
        cqg9_pw(b + 0x10, cqg9_w(b + 0x10).wrapping_add(1));
    }
    for i in 0..cqg9_i(0xae2390).max(0) {
        let b = blocos.wrapping_add((cqg9_i(cqg9_d(cqg9_arbitro(i) + 0x16)) as isize * 0x102) as usize);
        cqg9_pd(b + 4, cqg9_d(b + 4).wrapping_add(1) as u32);
    }
    let novos: unsafe extern "stdcall" fn(i32, usize) = core::mem::transmute(0x79c5c0usize);
    for k in 0..cqg9_i(0xae2350).max(0) {
        let b = blocos + k as usize * 0x102;
        let nac = cqg9_d(0xae23a8).wrapping_add(k as usize * 0x122);
        let c = cqg9_w(b + 0x10);
        if c > 0 {
            cqg9_pd(b + 0xc, (c as i32).wrapping_mul(cqg9_i(b + 0xc)) as u32);
            if cqg9_i(b + 0xc) > cqg9_i(b + 4) {
                novos(cqg9_i(b + 0xc) - cqg9_i(b + 4), nac);
                cqg9_pd(b + 4, cqg9_d(b + 0xc) as u32);
            }
        } else if cqg9_i(b + 4) < 5 {
            if cqg9_w(nac + 0x8e) == 0 {
                cqg9_pw(nac + 0x8e, 1);
            }
            novos((cqg9_w(nac + 0x8e) as i32).min(5), nac);
            cqg9_pd(b + 4, (cqg9_w(cqg9_d(0xae23a8).wrapping_add(k as usize * 0x122) + 0x8e) as i32).min(5) as u32);
        }
    }
    for i in 0..cqg9_i(0xae2390).max(0) {
        cqg9_pd(cqg9_arbitro(i), i as u32);
    }
    for i in 0..cqg9_i(0xae2390).max(0) {
        let r = cqg9_arbitro(i);
        let b = blocos.wrapping_add((cqg9_i(cqg9_d(r + 0x16)) as isize * 0x102) as usize);
        if cqg9_i(r) < cqg9_i(b + 8) {
            cqg9_pd(b + 8, cqg9_d(r) as u32);
        }
    }
    for k in 0..cqg9_i(0xae2350).max(0) {
        let b = cqg9_d(this + 0xc) + k as usize * 0x102;
        let c = cqg9_w(b + 0x10);
        if c == 0 {
            continue;
        }
        cqg9_ordenar(b + 0x12, c as i32 as u32, 0x10, 0x79d410);
        let mut j = 0i32;
        while j < cqg9_w(b + 0x10) as i32 {
            let c = cqg9_w(b + 0x10) as i32;
            crate::comparacao::ramo("cqg_9 parte da vaga");
            cqg9_pd(b + 0x16 + 16 * j as usize, (cqg9_i(b + 4) / c) as u32); // cqg9: a parte de cada vaga
            let c = cqg9_w(b + 0x10) as i32;
            cqg9_pd(b + 0x12 + 16 * j as usize, (cqg9_i(b + 4).wrapping_mul(j) / c).wrapping_add(cqg9_i(b + 8)) as u32);
            j += 1;
        }
    }
    let total = cqg9_i(0xae2390);
    cqg9_pd(this + 4, total as u32);
    let m = cqg9_calloc(total as u32, 1);
    cqg9_pd(this + 8, m as u32);
    if m == 0 {
        cqg9_fatal(0x2dd);
    }
    cqg9_liberar(marcas);
}

/// Comparador: só a original (ponytail: aloca, libera e troca a tabela dos árbitros, e as chamadas sorteiam: não pode
/// rodar duas vezes), com o plano: os blocos por nação (this+0xc), a conta (this+4) igual à da tabela e as marcas (this+8).
unsafe extern "thiscall" fn comparar_montar_os_arbitros(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1635)(this);
    PLACAR.registrar(cqg9_d(this + 0xc) != 0 && cqg9_i(this + 4) == cqg9_i(0xae2390) && cqg9_d(this + 8) != 0,
        || format!("montar_os_arbitros: blocos {:x}, conta {} x {}", cqg9_d(this + 0xc), cqg9_i(this + 4), cqg9_i(0xae2390)));
}

unsafe fn cqg9v_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqg9v_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x79d410 } (cdecl, officials_manager.cpp): `officials_manager_compare_comp_level(a, b)`, a ordem das vagas de
/// competição de uma nação (qsort da 0x79bf60, entradas de 16 bytes com a competição em +0xc): competição de `a` fora de
/// 0..[0xae2378]: 1; a de `b` fora: −1; pelo nível (+0x69, palavra, da competição de 0x6b bytes em [0xae23d0]); mesmo
/// nível: a diferença dos ids.
unsafe extern "C" fn ordem_das_vagas_de_competicao(a: usize, b: usize) -> i32 {
    let (ca, cb) = (cqg9v_i(a + 0xc), cqg9v_i(b + 0xc));
    let max = cqg9v_i(0xae2378);
    if ca < 0 || ca > max {
        return 1;
    }
    if cb < 0 || cb > max {
        return -1;
    }
    let base = cqg9v_i(0xae23d0) as u32 as usize;
    let sa = cqg9v_w(base.wrapping_add((ca as isize * 0x6b + 0x69) as usize));
    let sb = cqg9v_w(base.wrapping_add((cb as isize * 0x6b + 0x69) as usize));
    if sa == sb {
        return ca.wrapping_sub(cb);
    }
    sa as i32 - sb as i32
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_ordem_das_vagas_de_competicao(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1636)(a, b);
    let n = ordem_das_vagas_de_competicao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_vagas_de_competicao({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 271a leva: o amistoso cancelado, o jogo da seleção que sai junto e o torneio cancelado (friendly.cpp) ----------

unsafe fn cqg10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqg10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqg10_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O dword `d` com a palavra baixa trocada pela de `a` (o `mov ax, word` da original sobre um registrador já cheio).
fn cqg10_baixa(d: usize, a: i16) -> u32 {
    ((d as u32) & 0xffff_0000) | a as u16 as u32
}

/// { 0x5c6570 } (thiscall, ret 8, friendly.cpp, quadro SEH da notícia local): `Friendly::on_friendly_fixture_cancelled(
/// jogo, clube_que_cancela)`, pelo calendário (0x5aa680) e pela tela do clube. Amistoso desta competição (+0x14 = +4):
/// a notícia 8 (0x77bbe0) com +0xc, +0x10, o dia e o ano, aos técnicos do mandante e do visitante que não cancelaram
/// (0x7861e0); de outra competição: o torneio dela entre os de +0xb1 (0x69 bytes; índice = byte da competição − +0xaf),
/// se marcado, é cancelado (0x5c8260 com 0). Amistoso desta competição com a lista de vagas (+0x34cc): a vaga do jogo
/// (0x5c6740) fica livre (+0x10 = 0, +0x18 = 0). Jogo sem rodada (+0x43 = 0xff): o de seleção vai pela 0x5cb9e0, o de
/// clube só sai do calendário (0x5abc30).
unsafe extern "thiscall" fn cqg10_amistoso_cancelado(this: usize, jogo: usize, quem: usize) {
    let comp = cqg10_d(jogo + 0x14);
    if comp == cqg10_d(this + 4) {
        crate::comparacao::ramo("cqg_10 noticia do amistoso cancelado");
        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
        let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
        // 0x77bbe0: push 0; push 8; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall, ret 8; 0x7861e0:
        // push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8
        let mut noticia = [0u32; 0x38];
        let nt = noticia.as_mut_ptr() as usize;
        construir(nt, 8, 0);
        gravar(nt, 0, cqg10_d(jogo + 0xc) as u32);
        gravar(nt, 1, cqg10_d(jogo + 0x10) as u32);
        gravar(nt, 2, *((jogo + 0x2a) as *const u16) as u32);
        gravar(nt, 3, *((jogo + 0x28) as *const u16) as u32);
        let m = cqg10_d(jogo + 0x1c);
        if quem != m {
            entregar(cqg10_d(0xb7431c), nt, m);
        }
        let v = cqg10_d(jogo + 0x20);
        if quem != v {
            entregar(cqg10_d(0xb7431c), nt, v);
        }
    } else if this.wrapping_add(0xb1) != 0 {
        let k = cqg10_b(comp).wrapping_sub(cqg10_b(this + 0xaf)) as i8 as isize;
        let torneio = (this + 0xb1).wrapping_add((k * 0x69) as usize);
        if cqg10_d(torneio) != 0 && torneio != 0 {
            crate::comparacao::ramo("cqg_10 torneio cancelado");
            let cancelar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5c8260usize);
            // 0x5c8260: push 0; push torneio; mov ecx, this -> thiscall, ret 8
            cancelar(this, torneio, 0);
        }
    }
    if cqg10_d(jogo + 0x14) == cqg10_d(this + 4) && cqg10_d(this + 0x34cc) != 0 {
        let vaga_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5c6740usize);
        // 0x5c6740: push +0x1c; push +0x20; push (dx = +0x28); push (ax = +0x2a); mov ecx, this -> thiscall, ret 0x10;
        // os registradores levam nas palavras altas o que já tinham (o ponteiro +0x14 e o +0x1c)
        let v = vaga_de(this, cqg10_baixa(cqg10_d(jogo + 0x1c), cqg10_w(jogo + 0x2a)), cqg10_baixa(cqg10_d(jogo + 0x14), cqg10_w(jogo + 0x28)),
            cqg10_d(jogo + 0x20) as u32, cqg10_d(jogo + 0x1c) as u32);
        if v != 0 {
            crate::comparacao::ramo("cqg_10 vaga do amistoso livre");
            core::ptr::write_unaligned((v + 0x10) as *mut u32, 0); // cqg10: a vaga fica livre
            *((v + 0x18) as *mut u8) = 0;
        }
    }
    if cqg10_b(jogo + 0x43) == 0xff { // cqg10: jogo sem rodada
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        // 0x53b900: push; add esp, 4 -> cdecl; 0x5cb9e0: push jogo; mov ecx, this -> thiscall, ret 4; 0x5abc30: push
        // jogo; mov ecx, [0xae2a58] -> thiscall, ret 4
        if selecao(cqg10_d(jogo + 0x1c)) != 0 {
            let de_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5cb9e0usize);
            de_selecao(this, jogo);
        } else {
            let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5abc30usize);
            crate::comparacao::ramo("cqg_10 amistoso fora do calendario");
            tirar(cqg10_d(0xae2a58), jogo); // cqg10: o amistoso sai do calendário
        }
    }
}

/// Comparador da 0x5c6570: a original entrega notícias, cancela torneios e tira jogos do calendário (libera): só ela
/// roda. Antes, a cópia acha a vaga do amistoso (0x5c6740, só leitura); depois a vaga tem de estar livre (+0x10 = 0 e
/// +0x18 = 0).
unsafe extern "thiscall" fn comparar_cqg10_amistoso_cancelado(this: usize, jogo: usize, quem: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1637);
    let mut vaga = 0usize;
    if cqg10_d(jogo + 0x14) == cqg10_d(this + 4) && cqg10_d(this + 0x34cc) != 0 {
        let vaga_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5c6740usize);
        vaga = vaga_de(this, cqg10_baixa(cqg10_d(jogo + 0x1c), cqg10_w(jogo + 0x2a)), cqg10_baixa(cqg10_d(jogo + 0x14), cqg10_w(jogo + 0x28)),
            cqg10_d(jogo + 0x20) as u32, cqg10_d(jogo + 0x1c) as u32);
    }
    o(this, jogo, quem);
    let livre = vaga == 0 || (cqg10_d(vaga + 0x10) == 0 && cqg10_b(vaga + 0x18) == 0);
    PLACAR.registrar(livre, || format!("cqg10_amistoso_cancelado(jogo {:#x}) a vaga {:#x} não ficou livre", jogo, vaga));
}

/// { 0x5cb9e0 } (thiscall, ret 4, friendly.cpp, quadro SEH da lista local e das datas): o amistoso de seleção cancelado
/// (0x5c6570). Pela marca do jogo (+0x4d & 0x300): 0x100 a seleção é o mandante (+0x1c), 0x200 a seleção principal da
/// nação dele (0x53b4f0); com id válido, o jogo dela na véspera (0x549ef0, 0x54c8e0 com 1, 0x417c10; 0x5acbe0 com a
/// véspera nas duas pontas) sai do calendário (0x5abc30). Depois o próprio jogo sai.
unsafe extern "thiscall" fn cqg10_jogo_da_selecao_cancelado(this: usize, jogo: usize) {
    let _ = this;
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5abc30usize);
    // 0x68d020 / 0x68d040 / 0x68d1b0: lea ecx, local -> thiscall; 0x5abc30: push jogo; mov ecx, [0xae2a58] -> thiscall,
    // ret 4; 0x53b4f0: push nação; add esp, 4 -> cdecl; 0x549ef0: push +0x28; push +0x2a; lea ecx -> thiscall, ret 8;
    // 0x54c8e0: push 1; push &saída; lea ecx -> thiscall, ret 8; 0x417c10: push origem; push &saída; lea ecx ->
    // thiscall, ret 8; 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let marca = cqg10_w(jogo + 0x4d) as u16 & 0x300;
    let quem = if marca == 0x100 {
        cqg10_d(jogo + 0x1c)
    } else if marca == 0x200 {
        let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
        principal(cqg10_d(cqg10_d(jogo + 0x1c) + 0x53))
    } else {
        0
    };
    if quem != 0 && cqg10_d(quem) as i32 != -1 {
        crate::comparacao::ramo("cqg_10 jogo da selecao na vespera");
        let id = cqg10_d(quem) as u32;
        let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
        let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
        let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
        let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, i32) -> i16 = core::mem::transmute(0x5acbe0usize);
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        let (mut d0, mut d1, mut d2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let (p0, p1, p2) = (d0.as_mut_ptr() as usize, d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize);
        data_nova(p0, *((jogo + 0x2a) as *const u16) as u32, *((jogo + 0x28) as *const u16) as u32);
        let v = menos(p0, p1, 1); // cqg10: a véspera
        copiar(p0, p2, v);
        let a = cqg10_d(p0) as u32;
        let b = core::ptr::read_unaligned((p0 + 2) as *const u32);
        achar(cqg10_d(0xae2a58), a, b, a, b, id, l, -1, -2, -1);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cqg10_d(l + 2));
        let item = proximo(l);
        if item != 0 {
            tirar(cqg10_d(0xae2a58), item);
        }
    }
    tirar(cqg10_d(0xae2a58), jogo);
    soltar(l);
}

/// Comparador da 0x5cb9e0: a original tira jogos do calendário (libera): só ela roda (ponytail: a cópia não tem o que
/// conferir sem refazer a busca no calendário já mudado; a contagem do ramo prova no modo Rust).
unsafe extern "thiscall" fn comparar_cqg10_jogo_da_selecao_cancelado(this: usize, jogo: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1638)(this, jogo);
}

/// { 0x5c8260 } (thiscall, ret 8, friendly.cpp, quadro SEH da notícia, das datas e da lista locais): `Friendly::
/// cancel_tournament(torneio, sem_o_dono)`, pelo amistoso cancelado (0x5c6570) e pelo dia dos amistosos. Sem torneio, a
/// caixa da linha 0x10ad. Se o torneio começa depois de uma semana atrás (0x54c8e0 com 7), a notícia 0xb (o dono, o dia
/// e o ano do começo +4/+6) vai aos convidados (+0x15, 4) que não recusaram (+0x25 = 1, salvo o dono) e ao dono se
/// `sem_o_dono` é 0. O torneio sai da lista dos pendentes (+0x34d0, 0x68d1f0). Com a competição criada (+0x65): os
/// jogos dela de uma semana antes a 10 dias depois do começo (0x5acbe0): o de hoje volta a ser amistoso (0x5ac7a0:
/// +0x42 = 0xff, +0x14 = a competição de amistosos, +0 = [0x9cf668]); os outros saem do calendário (0x5abc30) e, com
/// rodada, voltam como amistosos (0x5ab230). A competição é destruída (virtual +0 com 1) e +0x65 = 0. A vaga da tabela
/// de competições (+0xaf + o byte +0x64) e o dono (+0) zeram.
unsafe extern "thiscall" fn cqg10_torneio_cancelado(this: usize, torneio: usize, sem_o_dono: u32) {
    if torneio == 0 {
        aviso_em(0x9cd0a0, 0x10ad);
        return;
    }
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar_da_lista: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    // 0x54c8e0 / 0x54c5b0: push n; push &saída; mov ecx, data -> thiscall, ret 8; 0x68d020 / 0x68d040 / 0x68d1b0: lea ecx
    // (ou mov ecx, lista) -> thiscall; 0x68d1f0: push item; mov ecx, [this+0x34d0] -> thiscall, ret 4
    let mut d7 = [0u32; 2];
    let semana = menos(0xae2c90, d7.as_mut_ptr() as usize, 7);
    let (dia7, ano7) = (cqg10_w(semana), cqg10_w(semana + 2) as u16);
    let inicio = torneio + 4;
    let (dia_i, ano_i) = (cqg10_w(inicio), cqg10_w(torneio + 6) as u16);
    if ano7 < ano_i || (ano7 == ano_i && dia7 < dia_i) {
        crate::comparacao::ramo("cqg_10 noticia do torneio cancelado");
        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
        let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
        // 0x77bbe0: push 0; push 0xb; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall, ret 8;
        // 0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8
        let mut noticia = [0u32; 0x38];
        let nt = noticia.as_mut_ptr() as usize;
        construir(nt, 0xb, 0);
        let dono = cqg10_d(torneio);
        gravar(nt, 0, cqg10_d(dono) as u32);
        gravar(nt, 1, cqg10_w(inicio) as i32 as u32);
        gravar(nt, 2, *((torneio + 6) as *const u16) as u32);
        for k in 0..4usize {
            let c = cqg10_d(torneio + 0x15 + k * 4);
            if c == 0 {
                continue;
            }
            if c == dono && sem_o_dono != 0 {
                continue;
            }
            if cqg10_b(torneio + 0x25 + k) == 1 && c != dono {
                continue;
            }
            entregar(cqg10_d(0xb7431c), nt, c);
        }
    }
    let pendentes = cqg10_d(this + 0x34d0);
    if pendentes != 0 {
        core::ptr::write_unaligned((pendentes + 0xa) as *mut usize, cqg10_d(pendentes + 2));
        let mut item = proximo(cqg10_d(this + 0x34d0));
        while item != 0 {
            if torneio == cqg10_d(item) {
                tirar_da_lista(cqg10_d(this + 0x34d0), item);
                let p = cqg10_d(this + 0x34d0);
                core::ptr::write_unaligned((p + 0xa) as *mut usize, cqg10_d(p + 2));
            }
            item = proximo(cqg10_d(this + 0x34d0));
        }
    }
    let (mut da, mut db) = ([0u8; 8], [0u8; 8]);
    let (pa, pb) = (da.as_mut_ptr() as usize, db.as_mut_ptr() as usize);
    menos(inicio, pa, 7);
    mais(inicio, pb, 0xa);
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let competicao = cqg10_d(torneio + 0x65);
    if competicao != 0 {
        crate::comparacao::ramo("cqg_10 competicao do torneio desfeita");
        let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, i32, usize, u32, i32, i32) -> i16 = core::mem::transmute(0x5acbe0usize);
        let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
        let de_volta: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5ac7a0usize);
        let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5abc30usize);
        let guardar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5ab230usize);
        // 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24; 0x549ef0: push +0x28; push +0x2a; lea ecx ->
        // thiscall, ret 8; 0x5ac7a0 / 0x5abc30 / 0x5ab230: push jogo; mov ecx, [0xae2a58] -> thiscall, ret 4
        let id_comp = cqg10_d(cqg10_d(competicao + 4)) as u32;
        achar(cqg10_d(0xae2a58), cqg10_d(pa) as u32, core::ptr::read_unaligned((pa + 2) as *const u32), cqg10_d(pb) as u32,
            core::ptr::read_unaligned((pb + 2) as *const u32), -1, l, id_comp, -2, -1);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cqg10_d(l + 2));
        let mut j = proximo(l);
        while j != 0 {
            let mut d = [0u8; 8];
            let pd = d.as_mut_ptr() as usize;
            data_nova(pd, *((j + 0x2a) as *const u16) as u32, *((j + 0x28) as *const u16) as u32);
            if cqg10_w(0xae2c92) == cqg10_w(pd + 2) && cqg10_w(0xae2c90) == cqg10_w(pd) {
                let r = de_volta(cqg10_d(0xae2a58), j);
                if r != 0 {
                    *((r + 0x42) as *mut u8) = 0xff;
                    core::ptr::write_unaligned((r + 0x14) as *mut usize, cqg10_d(this + 4));
                    core::ptr::write_unaligned(r as *mut usize, cqg10_d(0x9cf668));
                }
            } else {
                tirar(cqg10_d(0xae2a58), j);
                if cqg10_b(j + 0x43) != 0xff {
                    *((j + 0x42) as *mut u8) = 0xff;
                    core::ptr::write_unaligned((j + 0x14) as *mut usize, cqg10_d(this + 4));
                    core::ptr::write_unaligned(j as *mut usize, cqg10_d(0x9cf668));
                    guardar(cqg10_d(0xae2a58), j);
                }
            }
            j = proximo(l);
        }
        let c = cqg10_d(torneio + 0x65);
        if c != 0 {
            let destruir: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(cqg10_d(cqg10_d(c)));
            // o destrutor virtual (vtable +0): push 1; mov ecx, competição -> thiscall, ret 4
            destruir(c, 1);
        }
        core::ptr::write_unaligned((torneio + 0x65) as *mut usize, 0);
    }
    let k = (cqg10_w(this + 0xaf) as i32 + cqg10_b(torneio + 0x64) as i8 as i32) as usize;
    core::ptr::write_unaligned(cqg10_d(0xadadfc).wrapping_add(k.wrapping_mul(4)) as *mut usize, 0);
    core::ptr::write_unaligned(torneio as *mut usize, 0);
    soltar(l);
}

/// Comparador da 0x5c8260: a original entrega notícias, tira jogos do calendário e destrói a competição: só ela roda,
/// com o plano conferido depois: o dono (+0) e a competição (+0x65) zerados, a vaga da tabela de competições zerada e
/// o torneio fora da lista dos pendentes.
unsafe extern "thiscall" fn comparar_cqg10_torneio_cancelado(this: usize, torneio: usize, sem_o_dono: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1639);
    if torneio == 0 {
        o(this, torneio, sem_o_dono);
        return;
    }
    let k = (cqg10_w(this + 0xaf) as i32 + cqg10_b(torneio + 0x64) as i8 as i32) as usize;
    let vaga = cqg10_d(0xadadfc).wrapping_add(k.wrapping_mul(4));
    o(this, torneio, sem_o_dono);
    let mut na_lista = false;
    let p = cqg10_d(this + 0x34d0);
    if p != 0 {
        let mut no = cqg10_d(p + 2);
        let mut n = 0;
        while no != 0 && n < 0x10000 {
            if cqg10_d(cqg10_d(no)) == torneio {
                na_lista = true;
            }
            no = cqg10_d(no + 6);
            n += 1;
        }
    }
    let ok = cqg10_d(torneio) == 0 && cqg10_d(torneio + 0x65) == 0 && cqg10_d(vaga) == 0 && !na_lista;
    PLACAR.registrar(ok, || format!("cqg10_torneio_cancelado(torneio {:#x}) dono {:#x} competição {:#x} vaga {:#x} na lista {}",
        torneio, cqg10_d(torneio), cqg10_d(torneio + 0x65), cqg10_d(vaga), na_lista));
}

// ---------- 271a leva: a ordem dos árbitros por nação e qualidade, nos dois sentidos (officials_manager.cpp) ----------

unsafe fn cqh7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqh7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A qualidade do árbitro na ordem: o byte +0x28 × 50 + a palavra +0x22, em 16 bits.
unsafe fn cqh7_qualidade(r: usize) -> i16 {
    (*((r + 0x28) as *const i8) as i16).wrapping_mul(0x32).wrapping_add(cqh7_w(r + 0x22)) // cqh7: o peso da qualidade
}

/// O id da nação do árbitro ([+0x16] + 0).
unsafe fn cqh7_nacao(r: usize) -> i32 {
    cqh7_i(cqh7_i(r + 0x16) as u32 as usize)
}

/// { 0x79d4e0 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_desc(a, b)`, a ordem do
/// qsort da 0x79bf60 (os que ficam por nação): pela nação (id) e, na mesma, pela qualidade da maior para a menor; empate:
/// o índice +0 de `b` menos o de `a`.
unsafe extern "C" fn ordem_dos_arbitros_decrescente(a: usize, b: usize) -> i32 {
    let (na, nb) = (cqh7_nacao(a), cqh7_nacao(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (cqh7_qualidade(a), cqh7_qualidade(b));
    crate::comparacao::ramo("cqh_7 ordem decrescente");
    if ka == kb {
        return cqh7_i(b).wrapping_sub(cqh7_i(a)); // cqh7: o empate pelo índice
    }
    kb as i32 - ka as i32
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_ordem_dos_arbitros_decrescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1640)(a, b);
    let n = ordem_dos_arbitros_decrescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_arbitros_decrescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x79d480 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_asc(a, b)`, a ordem do
/// qsort da 0x79c5c0 (depois dos árbitros novos): pela nação e pela qualidade da menor para a maior; empate: o índice +0
/// de `a` menos o de `b`.
unsafe extern "C" fn ordem_dos_arbitros_crescente(a: usize, b: usize) -> i32 {
    let (na, nb) = (cqh7_nacao(a), cqh7_nacao(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (cqh7_qualidade(a), cqh7_qualidade(b));
    crate::comparacao::ramo("cqh_7 ordem crescente");
    if ka == kb {
        return cqh7_i(a).wrapping_sub(cqh7_i(b));
    }
    ka as i32 - kb as i32 // cqh7: a ordem crescente
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_ordem_dos_arbitros_crescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1641)(a, b);
    let n = ordem_dos_arbitros_crescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_arbitros_crescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 271a leva: as estatísticas do jogador arquivadas no histórico (a troca de clube, o empréstimo, a aposentadoria) e os jogos e gols pelo clube recontados (player_stats.cpp) ----------

unsafe fn cqh8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqh8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// O ensaio do comparador: a cópia roda sem gravar no arquivo de histórico (FILE_LLIST::add_node 0x58e340, que escreve
/// no arquivo e move as posições da ficha); os nós que ela gravaria entram direto na soma dos jogos e gols pelo clube.
static mut CQH8_ENSAIO: bool = false;
static mut CQH8_PENDENTES: (u16, u16) = (0, 0);
static mut CQH8_CLUBE: i32 = -1;

/// O nó do histórico (0x2b bytes montados na pilha): +0 o ano da temporada do clube (0x669950), +2 o clube (−1 sem),
/// +6 os 32 bytes da categoria, +0x26 o valor, +0x2a o tipo (3 bits) e a categoria × 8.
unsafe fn cqh8_no(clube: usize, dados: Option<usize>, valor: u32, marca: u8) -> [u8; 0x2b] {
    let ano: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x669950usize);
    let mut r = [0u8; 0x2b];
    r[0..2].copy_from_slice(&ano(clube).to_le_bytes());
    let id = if clube == 0 { 0xffff_ffffu32 } else { cqh8_d(clube) as u32 };
    r[2..6].copy_from_slice(&id.to_le_bytes());
    if let Some(d) = dados {
        core::ptr::copy_nonoverlapping(d as *const u8, r.as_mut_ptr().add(6), 32);
    }
    r[0x26..0x2a].copy_from_slice(&valor.to_le_bytes());
    r[0x2a] = marca;
    r
}

/// Grava o nó no fim da lista do histórico da ficha (0x58e340 no arquivo [this], posições +0x18/+0x1c do registro);
/// no ensaio, só soma.
unsafe fn cqh8_gravar(this: usize, reg: usize, no: &[u8; 0x2b]) {
    if CQH8_ENSAIO {
        if i32::from_le_bytes([no[2], no[3], no[4], no[5]]) == CQH8_CLUBE && no[0x2a] & 0xf8 == 0x10 {
            CQH8_PENDENTES.0 = CQH8_PENDENTES.0.wrapping_add(no[6] as u16);
            CQH8_PENDENTES.1 = CQH8_PENDENTES.1.wrapping_add(no[8] as u16);
        }
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) = core::mem::transmute(0x58e340usize);
    f(cqh8_d(this), reg + 0x18, reg + 0x1c, no.as_ptr());
}

/// { 0x7c5fe0 } (thiscall, ret 0x18, player_stats.cpp): `PlayerStats::archive_player_stats(pessoa, valor, empréstimo,
/// fim, clube, outro)`, pelas saídas de clube (contratos 0x4c7ac0/0x4cbd20/0x4d01e0/0x4d07e0, 0x690730 o elenco, 0x7c1350
/// as seleções). O tipo do nó: 2 com `empréstimo`, 1 com `fim`, 3 com `outro`, senão 0. Pessoa nula: a caixa (linha
/// 0xf28) e 0; sem ficha: 0. Cada uma das seis categorias com registro (o registro de 39 bytes da ficha em [this+0x30])
/// vai para o histórico (o ano e o clube atuais, os 32 bytes do registro em [this+4], `valor`, a categoria e o tipo) e
/// o registro é zerado. Sem a sexta categoria, um nó vazio da categoria 6 com `valor` quando: `valor` ≥ 0 ou `fim`; ou
/// `clube` ≥ 0 e: fora da temporada da nação do clube (a tabela 0xb63d8a); ou, sem a nação ou na temporada dela, menos de
/// 365 dias desde a entrada no clube (+0x3e/+0x42, 0x54cdb0). Com `clube` ≥ 0 ou `valor` ≥ 0: recontam-se os jogos (+0x20) e os gols (+0x22) pelo
/// `clube` nos nós da categoria 2 do histórico (0x58f120). Devolve 1.
unsafe extern "thiscall" fn arquivar_estatisticas(this: usize, p: usize, valor: u32, emprestimo: u32, fim: u32, clube_id: u32, outro: u32) -> u32 {
    let tipo: u8 = if emprestimo as u8 != 0 {
        2
    } else if fim as u8 != 0 {
        1
    } else if outro != 0 {
        3
    } else {
        0
    };
    if p == 0 {
        aviso_em(0xa78310, 0xf28);
        return 0;
    }
    let clube = cqh8_d(p + 0x39);
    let f = cqh8_d(p + 0x61);
    if f == 0 {
        return 0;
    }
    let reg = cqh8_d(this + 0x30).wrapping_add((cqh8_i(f) as isize * 0x27) as usize);
    let mut sexta = false;
    for cat in 0..6usize {
        let id = cqh8_i(reg + 4 * cat);
        if id == -1 {
            continue;
        }
        let dados = cqh8_d(this + 4).wrapping_add((id as isize * 0x20) as usize);
        let no = cqh8_no(clube, Some(dados), valor, (((cat + 1) as u8) << 3) | tipo); // cqh8: a categoria e o tipo do nó
        crate::comparacao::ramo("cqh_8 categoria arquivada");
        cqh8_gravar(this, reg, &no);
        if cat == 5 {
            sexta = true;
        }
        core::ptr::write_bytes(dados as *mut u8, 0, 32); // cqh8: o registro da categoria zerado
    }
    // sem nação do clube (ou fora da tabela) ou na temporada dela: a marca que pede os 365 dias
    let marca = if clube == 0 || cqh8_d(clube + 0x53) == 0 {
        true
    } else {
        let linha: unsafe extern "C" fn(usize) -> i8 = core::mem::transmute(0x669340usize);
        let k = linha(cqh8_d(clube + 0x53));
        if !(0..0x23).contains(&k) {
            true
        } else {
            let t = 0xb63d8a + k as usize * 0x48;
            let (dia, ano) = (core::ptr::read_unaligned(0xae2c90 as *const i16), core::ptr::read_unaligned(0xae2c92 as *const u16));
            let (d0, a0) = (core::ptr::read_unaligned(t as *const i16), core::ptr::read_unaligned((t + 2) as *const u16));
            let (d1, a1) = (core::ptr::read_unaligned((t + 0xf) as *const i16), core::ptr::read_unaligned((t + 0x11) as *const u16));
            let depois_do_inicio = (dia > d0 && ano == a0) || ano > a0;
            let antes_do_fim = (dia < d1 && ano == a1) || ano < a1;
            depois_do_inicio && antes_do_fim
        }
    };
    if !sexta {
        let gravar = if (valor as i32) > -1 || fim as u8 != 0 {
            true
        } else if (clube_id as i32) <= -1 {
            false
        } else if !marca {
            true
        } else {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
            dias(0xae2c90, cqh8_d(p + 0x3e) as u32, cqh8_d(p + 0x42) as u32) < 0x16d
        };
        if gravar {
            let no = cqh8_no(clube, None, valor, 0x30 | tipo); // cqh8: o nó vazio da sexta categoria
            crate::comparacao::ramo("cqh_8 no vazio");
            cqh8_gravar(this, reg, &no);
        }
    }
    if (clube_id as i32) > -1 || (valor as i32) > -1 {
        core::ptr::write_unaligned((reg + 0x20) as *mut u16, 0);
        core::ptr::write_unaligned((reg + 0x22) as *mut u16, 0);
        let nos: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x58f120usize);
        let lista = nos(cqh8_d(this), cqh8_d(reg + 0x18) as u32, cqh8_d(reg + 0x1c) as u32);
        if lista != 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, cqh8_d(lista + 2) as u32);
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
            let mut no = proximo(lista);
            while no != 0 {
                if cqh8_d(no + 2) as u32 == clube_id && *((no + 0x2a) as *const u8) & 0xf8 == 0x10 {
                    crate::comparacao::ramo("cqh_8 jogos pelo clube");
                    let a = (reg + 0x20) as *mut u16;
                    core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(*((no + 6) as *const u8) as u16)); // cqh8: os jogos pelo clube
                    let g = (reg + 0x22) as *mut u16;
                    core::ptr::write_unaligned(g, core::ptr::read_unaligned(g).wrapping_add(*((no + 8) as *const u8) as u16));
                }
                no = proximo(lista);
            }
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
            let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
            destruir(lista);
            apagar(lista);
        }
        if CQH8_ENSAIO {
            let a = (reg + 0x20) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(CQH8_PENDENTES.0));
            let g = (reg + 0x22) as *mut u16;
            core::ptr::write_unaligned(g, core::ptr::read_unaligned(g).wrapping_add(CQH8_PENDENTES.1));
        }
    }
    1
}

/// Comparador em ensaio (modelo CPV6_ENSAIO): a cópia roda primeiro sem gravar no arquivo (os nós que gravaria entram na
/// soma dos jogos e gols), depois a original a partir do mesmo estado; compara a devolução, os seis registros de 32 bytes
/// das categorias e os jogos e gols pelo clube (+0x20/+0x22 do registro da ficha). As posições do histórico (+0x18,
/// +0x1c), que só a gravação da original move, ficam fora. Pessoa nula ou sem ficha: só a original.
unsafe extern "thiscall" fn comparar_arquivar_estatisticas(this: usize, p: usize, valor: u32, emprestimo: u32, fim: u32, clube_id: u32, outro: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(1642);
    if p == 0 || cqh8_d(p + 0x61) == 0 {
        return o(this, p, valor, emprestimo, fim, clube_id, outro);
    }
    let reg = cqh8_d(this + 0x30).wrapping_add((cqh8_i(cqh8_d(p + 0x61)) as isize * 0x27) as usize);
    let mut regioes = vec![(reg + 0x20, 4)];
    for cat in 0..6usize {
        let id = cqh8_i(reg + 4 * cat);
        if id != -1 {
            regioes.push((cqh8_d(this + 4).wrapping_add((id as isize * 0x20) as usize), 32));
        }
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CQH8_ENSAIO = true;
    CQH8_PENDENTES = (0, 0);
    CQH8_CLUBE = clube_id as i32;
    let n = crate::aleatorio::pausar_rastro(|| arquivar_estatisticas(this, p, valor, emprestimo, fim, clube_id, outro));
    CQH8_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = o(this, p, valor, emprestimo, fim, clube_id, outro);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(r == n && mo == mr, || format!("arquivar_estatisticas(pessoa {:x}, valor {}, clube {}) original={} rust={} {:x?} x {:x?}",
        p, valor as i32, clube_id as i32, r, n, mo.0, mr.0));
    r
}

// ---------- 271a leva: o player_setup.cfg: a pessoa pelo nome (no clube ou em todos), o texto entre aspas da linha e o tipo da linha pelo nome (player_regen.cpp) ----------

unsafe fn cqh9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqh9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// strncmp (0x946fa0, cdecl).
unsafe fn cqh9_strncmp(a: usize, b: usize, n: u32) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, u32) -> i32 = core::mem::transmute(0x946fa0usize);
    f(a, b, n)
}

/// O nome `k` (+4 primeiro, +8 segundo, +0xc o comum; o registro do nome começa pelo texto) da pessoa bate com `texto`
/// (até 26 caracteres)? `texto` vazio sempre bate.
unsafe fn cqh9_bate(p: usize, k: usize, texto: usize) -> bool {
    *(texto as *const u8) == 0 || cqh9_strncmp(cqh9_d(p + k), texto, 0x1a) == 0
}

/// { 0x7a50f0 } (cdecl, player_regen.cpp): `player_setup_find_staff(primeiro, comum, segundo, clube, no_elenco)`. Sem
/// `clube` (nulo ou vazio): a primeira das [0xae2364] pessoas ([0xae23bc], 0x6e bytes) cujos nomes não vazios batem
/// (cada nome conferido só se o texto e o nome da pessoa existem). Com `clube`: o clube com esse nome (0x545880, 51
/// caracteres); sem achar, 0. Com `no_elenco` 1: o primeiro das 50 vagas do elenco (+0xd7) com os nomes que batem;
/// senão (ou nenhum do elenco), o técnico do clube (+0xcf) se os nomes batem; senão 0.
unsafe extern "C" fn pessoa_pelo_nome(primeiro: usize, comum: usize, segundo: usize, clube: usize, no_elenco: u32) -> usize {
    if clube == 0 || *(clube as *const u8) == 0 {
        let mut p = cqh9_d(0xae23bc);
        for _ in 0..cqh9_i(0xae2364).max(0) {
            let falha = (*(primeiro as *const u8) != 0 && cqh9_d(p + 4) != 0 && cqh9_strncmp(cqh9_d(p + 4), primeiro, 0x1a) != 0)
                || (*(segundo as *const u8) != 0 && cqh9_d(p + 8) != 0 && cqh9_strncmp(cqh9_d(p + 8), segundo, 0x1a) != 0) // cqh9: o segundo nome
                || (*(comum as *const u8) != 0 && cqh9_d(p + 0xc) != 0 && cqh9_strncmp(cqh9_d(p + 0xc), comum, 0x1a) != 0);
            if !falha {
                crate::comparacao::ramo("cqh_9 pessoa pelo nome");
                return p;
            }
            p += 0x6e;
        }
        return 0;
    }
    let nome_do_clube: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x545880usize);
    let mut achado = 0usize;
    for k in 0..cqh9_i(0xae235c).max(0) {
        let c = cqh9_d(0xae23b4).wrapping_add(k as usize * 0x245);
        if cqh9_strncmp(nome_do_clube(c), clube, 0x33) == 0 {
            achado = c;
            break;
        }
    }
    if achado == 0 {
        return 0;
    }
    let c = achado;
    if no_elenco as u8 == 1 {
        for k in 0..0x32usize {
            let q = cqh9_d(c + 0xd7 + 4 * k);
            if q != 0 && cqh9_bate(q, 4, primeiro) && cqh9_bate(q, 8, segundo) && cqh9_bate(q, 0xc, comum) {
                crate::comparacao::ramo("cqh_9 jogador do clube pelo nome");
                return q; // cqh9: o do elenco pelo nome
            }
        }
    }
    let t = cqh9_d(c + 0xcf);
    if t == 0 {
        return 0;
    }
    if cqh9_bate(t, 4, primeiro) && cqh9_bate(t, 8, segundo) && cqh9_bate(t, 0xc, comum) {
        return t;
    }
    0
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_pessoa_pelo_nome(primeiro: usize, comum: usize, segundo: usize, clube: usize, no_elenco: u32) -> usize {
    let r = orig::<unsafe extern "C" fn(usize, usize, usize, usize, u32) -> usize>(1643)(primeiro, comum, segundo, clube, no_elenco);
    let n = pessoa_pelo_nome(primeiro, comum, segundo, clube, no_elenco);
    PLACAR.registrar(r == n, || format!("pessoa_pelo_nome(clube {:x}, elenco {}) original={:x} rust={:x}", clube, no_elenco as u8, r, n));
    r
}

/// { 0x7a5090 } (cdecl, player_regen.cpp): `player_setup_read_quoted_token(origem, destino)`: copia para `destino` o
/// texto entre as primeiras aspas de `origem`: espaços e '\r' no começo do texto e '\n' antes dele são pulados; termina
/// nas aspas de fecho ou no zero (o destino ganha o zero) e devolve o ponteiro dois depois do fim.
unsafe extern "C" fn texto_entre_aspas(origem: usize, destino: usize) -> usize {
    let (mut i, mut n, mut aberto) = (0usize, 0usize, false);
    loop {
        let c = *((origem + i) as *const u8);
        i += 1;
        if c == b'"' {
            if aberto {
                break;
            }
            aberto = true;
            continue;
        }
        if (c == b' ' || c == b'\r') && n == 0 {
            continue;
        }
        if c == b'\n' {
            if n == 0 {
                continue;
            }
        } else if c == 0 {
            break;
        }
        if aberto {
            crate::comparacao::ramo("cqh_9 texto entre aspas");
            *((destino + n) as *mut u8) = c; // cqh9: o caractere do texto
            n += 1;
        }
    }
    *((destino + n) as *mut u8) = 0;
    origem + i + 1
}

/// Comparador: lado a lado no destino (até 0x100 bytes) e na devolução.
unsafe extern "C" fn comparar_texto_entre_aspas(origem: usize, destino: usize) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> usize>(1644);
    if origem == 0 || destino == 0 {
        return o(origem, destino);
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(destino, 0x100)], || o(origem, destino), || texto_entre_aspas(origem, destino));
    PLACAR.registrar(igual && r == n, || format!("texto_entre_aspas({:x}) original={:x} rust={:x}", origem, r, n));
    r
}

/// Os tipos de linha do player_setup.cfg (o texto no .rdata e o bit), na ordem da original (strncmp de 100); o último,
/// AWOL (0xa6b2f8), dá 0x40, e nenhum, 0.
const CQH9_TIPOS: [(usize, u32); 14] = [
    (0xa6b3a0, 1),      // INJURY
    (0xa6b390, 2),      // MANAGER_CONF
    (0xa6b384, 0x200),  // RETIREMENT
    (0xa6b374, 4),      // INT_RETIREMENT
    (0xa6b364, 8),      // FUTURE_TRANSFER
    (0xa6b35c, 0x10),   // LOAN
    (0xa6b350, 0x100),  // NANDROLONE
    (0xa6b348, 0x100),  // DRUGS
    (0xa6b33c, 0x1000), // MISCONDUCT
    (0xa6b32c, 0x800),  // FALSE_PASSPORT
    (0xa6b324, 0x20),   // ARSE
    (0xa6b314, 0x2000), // PRESET_TACTICS
    (0xa6b308, 0x400),  // COLOURBLIND
    (0xa6b300, 0x80),   // GERMAN
];

/// { 0x7a3140 } (cdecl, player_regen.cpp): `player_setup_type_from_name(nome)`: o bit do tipo da linha (INJURY 1,
/// MANAGER_CONF 2, RETIREMENT 0x200, INT_RETIREMENT 4, FUTURE_TRANSFER 8, LOAN 0x10, NANDROLONE e DRUGS 0x100,
/// MISCONDUCT 0x1000, FALSE_PASSPORT 0x800, ARSE 0x20, PRESET_TACTICS 0x2000, COLOURBLIND 0x400, GERMAN 0x80, AWOL 0x40;
/// nenhum 0), pela aplicação das linhas (0x7a3a10) na criação do jogo.
unsafe extern "C" fn tipo_da_linha(nome: usize) -> u32 {
    for &(texto, bit) in CQH9_TIPOS.iter() {
        if cqh9_strncmp(nome, texto, 100) == 0 {
            crate::comparacao::ramo("cqh_9 tipo da linha");
            return bit;
        }
    }
    if cqh9_strncmp(nome, 0xa6b2f8, 100) == 0 { 0x40 } else { 0 }
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_tipo_da_linha(nome: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(1645)(nome);
    let n = tipo_da_linha(nome);
    PLACAR.registrar(r == n, || format!("tipo_da_linha({:x}) original={:x} rust={:x}", nome, r, n));
    r
}
