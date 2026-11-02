// ---------- %%LEVA%%a leva: a fuga do jogador (as reputações, as antipatias do técnico e do elenco, a volta marcada) e a notícia dela (awol.cpp) ----------

unsafe fn cqh9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqh9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqh9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A antipatia (0x547af0, cdecl): `dono` passa a não gostar de `outro` (forçando).
unsafe fn cqh9_antipatia(dono: usize, outro: usize) {
    let f: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
    f(dono, outro, 1);
}

/// O ensaio do comparador: a cópia roda sem pôr a fuga na lista (0x68d0d0, que aloca) e sem a notícia (0x419ad0).
static mut CQH9_ENSAIO: bool = false;

/// A reputação (palavra da ficha) × (1 − sorteio(41) × 0,01), truncada (fild; fmul 0x967898; fsubr 0x967890; fild;
/// fmulp; _ftol).
unsafe fn cqh9_reputacao(a: usize) {
    let r = cqh9_sortear(0x29);
    let v = core::ptr::read_unaligned(a as *const i16);
    let k = f64_em(0x967890) - r as f64 * f64_em(0x967898); // cqh9: o corte da reputação
    core::ptr::write_unaligned(a as *mut i16, ftol(v as f64 * k) as i16);
}

/// { 0x419840 } (thiscall, ret 0xc, quadro SEH só da data local): `AWOL::start_absence(jogador, clube, tipo)`, pelas
/// faltas ao treino (0x41a360), pelo lance de outro clube (0x8c1290), pela volta das férias (0x630ee0), pelo diário
/// (0x5ce2d0) e pela mídia. Jogador nulo, sem ficha, fora dos jogadores ou já marcado (bit 1 da tabela this+0xc): nada.
/// Senão marca; as três reputações da ficha (+0xb, +9, +0xd) caem até 40%; com clube: ele não gosta do técnico (+0xcf),
/// o técnico do computador não gosta dele, cada um do elenco (50 em +0xd7) com sorteio(+0x59) e sem sorteio(+0x5b) não
/// gosta dele, nem o +0xbf do clube; a volta (id e a data de hoje + 2..32 dias no tipo 4, senão + 21..170) vai para a
/// lista das fugas (this+0, 12 bytes); a notícia (0x419ad0) com o clube (o dele, sem `clube`): tipo 1 → 2, tipo 4 → 4
/// com os dias, outros → 1.
unsafe extern "thiscall" fn comecar_a_fuga(this: usize, p: usize, clube: usize, tipo: u32) {
    let mut d = [0u8; 8];
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    data_padrao(d.as_mut_ptr() as usize);
    if p == 0 || cqh9_d(p + 0x61) == 0 || cqh9_i(p) >= cqh9_i(0xae2364) {
        return;
    }
    let m = (cqh9_d(this + 0xc) as isize + cqh9_i(p) as isize) as *mut u8;
    if *m & 1 != 0 {
        return;
    }
    *m |= 1;
    crate::comparacao::ramo("cqh_9 fuga");
    cqh9_reputacao(cqh9_d(p + 0x61) + 0xb);
    cqh9_reputacao(cqh9_d(p + 0x61) + 9);
    cqh9_reputacao(cqh9_d(p + 0x61) + 0xd);
    let c = cqh9_d(p + 0x39);
    if c != 0 {
        cqh9_antipatia(p, cqh9_d(c + 0xcf));
        let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
        if humano(cqh9_d(p + 0x39), 1, 1, 0, 0) == 0 {
            cqh9_antipatia(cqh9_d(cqh9_d(p + 0x39) + 0xcf), p);
        }
        for k in 0..50usize {
            let q = cqh9_d(cqh9_d(p + 0x39) + 0xd7 + 4 * k);
            if q == 0 || q == p {
                continue;
            }
            if cqh9_sortear(*((q + 0x59) as *const i8) as i32) != 0 && cqh9_sortear(*((q + 0x5b) as *const i8) as i32) == 0 {
                cqh9_antipatia(q, p); // cqh9: o companheiro não gosta dele
            }
        }
        cqh9_antipatia(cqh9_d(cqh9_d(p + 0x39) + 0xbf), p);
    }
    let mut reg = [0u8; 12];
    reg[0..4].copy_from_slice(&(cqh9_d(p) as u32).to_le_bytes());
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut tmp = [0u8; 8];
    copia(reg.as_mut_ptr() as usize + 4, tmp.as_mut_ptr() as usize, 0xae2c90);
    let t = tipo as u8;
    let dias = if t == 4 { cqh9_sortear(0x1f) + 2 } else { cqh9_sortear(0x96) + 0x15 }; // cqh9: os dias até a volta
    let mais: unsafe extern "thiscall" fn(usize, usize, i32) -> usize = core::mem::transmute(0x54c5b0usize);
    let mut tmp2 = [0u8; 8];
    let r = mais(reg.as_mut_ptr() as usize + 4, tmp2.as_mut_ptr() as usize, dias);
    let mut tmp3 = [0u8; 8];
    copia(reg.as_mut_ptr() as usize + 4, tmp3.as_mut_ptr() as usize, r);
    if CQH9_ENSAIO {
        return;
    }
    let por: unsafe extern "thiscall" fn(usize, *const u8, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    por(cqh9_d(this), reg.as_ptr(), 0xc);
    let clube = if clube == 0 { cqh9_d(p + 0x39) } else { clube };
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x419ad0usize);
    match t {
        1 => noticia(this, p, clube, 2, 0xffff_ffff),
        4 => noticia(this, p, clube, 4, dias as u32),
        _ => noticia(this, p, clube, 1, 0xffff_ffff),
    }
}

/// Comparador em ensaio (modelo CPV6_ENSAIO): a cópia roda primeiro sem a lista e sem a notícia, depois a original a
/// partir do mesmo estado; compara a marca, o jogador e a ficha, o técnico, o +0xbf e o elenco do clube (as
/// antipatias) e o estado do sorteio. Já marcado ou fora das condições: nada muda, lado a lado na marca.
unsafe extern "thiscall" fn comparar_comecar_a_fuga(this: usize, p: usize, clube: usize, tipo: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I0%%);
    if p == 0 || cqh9_d(p + 0x61) == 0 || cqh9_i(p) >= cqh9_i(0xae2364) || this == 0 {
        return o(this, p, clube, tipo);
    }
    let mut reg: Vec<(usize, usize)> = Vec::from(crate::aleatorio::REGIOES_DO_ESTADO);
    reg.push(((cqh9_d(this + 0xc) as isize + cqh9_i(p) as isize) as usize, 1));
    reg.push((p, 0x6e));
    reg.push((cqh9_d(p + 0x61), 0x46));
    let c = cqh9_d(p + 0x39);
    if c != 0 {
        for k in [0xcfusize, 0xbf] {
            if cqh9_d(c + k) != 0 {
                reg.push((cqh9_d(c + k), 0x6e));
            }
        }
        for k in 0..50usize {
            let q = cqh9_d(c + 0xd7 + 4 * k);
            if q != 0 && q != p {
                reg.push((q, 0x6e));
            }
        }
    }
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQH9_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| comecar_a_fuga(this, p, clube, tipo)));
    CQH9_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    o(this, p, clube, tipo);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("comecar_a_fuga(pessoa {:x}, clube {:x}, tipo {}) {:x?} x {:x?}", p, clube, tipo as u8, mo.0, mr.0));
}

/// { 0x419ad0 } (thiscall, ret 0x10, awol.cpp; o this não é lido): `awol_post_news(jogador, clube, tipo, valor)`: sem
/// jogador, clube atual (+0x39), ficha ou `clube`: nada. A notícia 3013 (0x77bbe0) com o id, os três nomes, o `clube`, o
/// clube atual (−1 sem), o tipo, o valor e a nação do clube atual (−1 sem); reputação mundial (ficha +0xd) acima de
/// 7500: para todos (0x786780); senão para o país do clube (0x7863f0) e, se a nacionalidade (+0x1a) é outra, para ela.
unsafe extern "thiscall" fn noticia_da_fuga(_this: usize, p: usize, clube: usize, tipo: u32, valor: u32) {
    if p == 0 || cqh9_d(p + 0x39) == 0 || cqh9_d(p + 0x61) == 0 || clube == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let dado: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let mut noticia = [0u8; 0xe8];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0xbc5, 0);
    dado(n, 0, cqh9_d(p) as u32);
    dado(n, 1, cqh9_d(cqh9_d(p + 4) + 0x33) as u32);
    dado(n, 2, cqh9_d(cqh9_d(p + 8) + 0x33) as u32);
    dado(n, 3, cqh9_d(cqh9_d(p + 0xc) + 0x33) as u32);
    dado(n, 4, cqh9_d(clube) as u32);
    let atual = cqh9_d(p + 0x39);
    dado(n, 5, if atual != 0 { cqh9_d(atual) as u32 } else { 0xffff_ffff });
    dado(n, 6, tipo as u8 as i8 as i32 as u32);
    dado(n, 7, valor as u16 as i16 as i32 as u32);
    let nac = if atual != 0 { cqh9_d(atual + 0x53) } else { 0 };
    dado(n, 8, if nac != 0 { cqh9_d(nac) as u32 } else { 0xffff_ffff });
    let gerente = cqh9_d(0xb7431c);
    crate::comparacao::ramo("cqh_9 noticia da fuga");
    if core::ptr::read_unaligned((cqh9_d(p + 0x61) + 0xd) as *const i16) > 0x1d4c {
        let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
        todos(gerente, n);
        return;
    }
    let pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let nac = cqh9_d(cqh9_d(p + 0x39) + 0x53);
    if nac != 0 {
        pais(gerente, n, nac);
    }
    let natal = cqh9_d(p + 0x1a);
    if cqh9_d(cqh9_d(p + 0x39) + 0x53) != natal {
        pais(gerente, n, natal);
    }
}

/// Comparador: só a original (ponytail: a entrega aloca a notícia nas caixas de entrada).
unsafe extern "thiscall" fn comparar_noticia_da_fuga(this: usize, p: usize, clube: usize, tipo: u32, valor: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I1%%)(this, p, clube, tipo, valor)
}
