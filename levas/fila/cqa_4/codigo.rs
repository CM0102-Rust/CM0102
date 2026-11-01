// ---------- %%LEVA%%a leva: a copa pelo desfecho e no fim da temporada: os registros pelo desfecho, as honras da temporada e os semifinalistas para a UEFA (Cup.cpp) ----------

unsafe fn cqa4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa4_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de 0x68 bytes da rodada `r` (+0xa3).
unsafe fn cqa4_rodada(copa: usize, r: i32) -> usize {
    cqa4_d(copa + 0xa3).wrapping_add((r as isize * 0x68) as usize)
}

/// O clube do time `t` (+0xa7, 6 bytes por time, +0 o clube).
unsafe fn cqa4_clube(copa: usize, t: i32) -> usize {
    cqa4_d(cqa4_d(copa + 0xa7).wrapping_add((t as isize * 6) as usize))
}

/// O registro de 6 bytes do estado do clube ([0xae2d14] + id × 6; +0 a competição europeia da temporada).
unsafe fn cqa4_estado(clube: usize) -> usize {
    cqa4_d(0xae2d14).wrapping_add((cqa4_i(clube) as isize * 6) as usize)
}

/// A competição é a amistosa (0x5c6710: push comp; ecx = [0xae2c74]; sem add esp -> thiscall, ret 4).
unsafe fn cqa4_amistosa(comp: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    f(cqa4_d(0xae2c74), comp) != 0
}

/// O índice da nação dona da competição na tabela das chaves (0x669470: push comp; add esp, 4 -> cdecl; devolve o byte).
unsafe fn cqa4_indice(comp: usize) -> u8 {
    let f: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669470usize);
    f(comp)
}

/// A competição é a copa principal guardada da nação dela ([0xb63d7e + 72 × índice]).
unsafe fn cqa4_copa_principal(comp: usize) -> bool {
    cqa4_d(0xb63d7e_usize.wrapping_add((cqa4_indice(comp) as i8 as isize * 72) as usize)) == comp
}

/// A vaga na Copa da UEFA que o campeão da copa (`copa_registros_pelo_desfecho`, desfecho 1) ganha: a Copa da Liga
/// inglesa ([0x9cf750]) sempre; senão, a copa principal de uma nação europeia ([0x9cfa10] o continente, pela nação
/// +0x5d e o continente +0x71), não amistosa, com o clube ainda sem competição europeia (−1) e a nação com índice.
unsafe fn cqa4_vaga_na_uefa(comp: usize, clube: usize) -> bool {
    if cqa4_i(comp) == cqa4_i(0x9cf750) {
        return true;
    }
    let nacao = cqa4_d(comp + 0x5d);
    if nacao == 0 {
        return false;
    }
    let continente = cqa4_d(nacao + 0x71);
    if continente == 0 || cqa4_i(continente) != cqa4_i(0x9cfa10) || cqa4_amistosa(comp) {
        return false;
    }
    if cqa4_i(cqa4_estado(clube)) != -1 || cqa4_indice(comp) == 0xff {
        return false;
    }
    cqa4_copa_principal(comp)
}

/// Os registros de final (0x86a800 campeão, 0x86b0b0 vice, 0x86b710 terceiro: push final; push jogo; push clube;
/// ecx = [0xdd7f00]; sem add esp -> thiscall(registros, clube, jogo, final), ret 0xc).
unsafe fn cqa4_registro_de_final(f: usize, clube: usize, jogo: usize, final_: u32) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(cqa4_d(0xdd7f00), clube, jogo, final_);
}

/// { 0x519890 } (thiscall, ret 0x18, Cup.cpp; a virtual +0x44 das copas inglesas e da base):
/// `Cup::update_staff_records_for_status(clube, desfecho, _, _, jogo, final)`; devolve 0. Nada com o jogo +0x3a = 1,
/// com +0x2c ou com +0x44 ≠ 0xff. Desfecho 1: o registro de campeão e, com a vaga (cqa4_vaga_na_uefa), o estado do
/// clube recebe a Copa da UEFA ([0x9cf6f0]); 2: o de vice; 3: o de terceiro; 0: se a rodada seguinte (+0x34 + 1) existe
/// e não é a do terceiro lugar (+7 ≠ 0x8c), a classificação (0x86bdd0) com o peso rodada × 99 / (rodadas − 1) + 1;
/// 0xff: a eliminação (0x86c000) com o peso rodada × 99 / rodadas + 1. Os registros de fase levam os dwords que a
/// original empurra: a palavra do jogo (+0x32, +0x30) ou da rodada (+7) sobre os 16 bits altos do registrador.
unsafe extern "thiscall" fn copa_registros_pelo_desfecho(copa: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, final_: u32) -> u32 {
    if cqa4_b(jogo + 0x3a) == 1 || cqa4_d(copa + 0x2c) != 0 || cqa4_b(copa + 0x44) != 0xff {
        return 0;
    }
    let comp = cqa4_d(copa + 4);
    match desfecho as u8 {
        1 => {
            crate::comparacao::ramo("cqa_4 campeao");
            cqa4_registro_de_final(0x86a800, clube, jogo, final_); // cqa4: o registro de campeão
            if cqa4_vaga_na_uefa(comp, clube) {
                core::ptr::write_unaligned(cqa4_estado(clube) as *mut i32, cqa4_i(0x9cf6f0));
            }
        }
        2 => {
            crate::comparacao::ramo("cqa_4 vice");
            cqa4_registro_de_final(0x86b0b0, clube, jogo, final_); // cqa4: o registro de vice
        }
        3 => cqa4_registro_de_final(0x86b710, clube, jogo, final_),
        0 => {
            let r = cqa4_w(jogo + 0x34);
            let n = cqa4_w(copa + 0x3c);
            let seguinte = r.wrapping_add(1);
            if seguinte >= n {
                return 0;
            }
            let w7 = core::ptr::read_unaligned((cqa4_rodada(copa, seguinte as i32) + 7) as *const u16);
            if w7 == 0x8c {
                return 0;
            }
            let div = n as i32 - 1;
            if div == 0 {
                return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%)(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: a original cai na divisão
            }
            let q = (r as i32 * 99) / div;
            let peso = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32; // inc al
            let fase = (jogo as u32 & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32; // mov cx, [ecx+0x32]
            let rodada = (cqa4_d(copa + 0xa3) as u32 & 0xffff_0000) | w7 as u32; // mov di, [edi+...+7]
            // push peso; push rodada; push fase; push comp; push clube; ecx = [0xdd7f00] -> thiscall, ret 0x14
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
            f(cqa4_d(0xdd7f00), clube, comp, fase, rodada, peso);
        }
        0xff => {
            let r = cqa4_w(jogo + 0x34) as i32;
            let n = cqa4_w(copa + 0x3c) as i32;
            if n == 0 {
                return orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%)(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: a original cai na divisão
            }
            let (q, resto) = ((r * 99) / n, (r * 99) % n);
            let peso = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32;
            let fase = (peso & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32; // mov ax, [ecx+0x32]
            let rodada = (resto as u32 & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x30) as *const u16) as u32; // mov dx, [ecx+0x30]
            // push peso; push rodada; push fase; push comp; push clube; ecx = [0xdd7f00] -> thiscall, ret 0x14
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
            f(cqa4_d(0xdd7f00), clube, comp, fase, rodada, peso);
        }
        _ => {}
    }
    0
}

/// Comparador da 0x519890: só a original (os registros vão ao histórico das pessoas e mexem nas reputações e no moral,
/// não podem rodar duas vezes). O plano, feito antes: o estado do clube (+0 do registro de 6 bytes) passa à Copa da
/// UEFA quando o campeão ganha a vaga; senão não muda. Sem clube: só a original.
unsafe extern "thiscall" fn comparar_copa_registros_pelo_desfecho(copa: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, final_: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I0%%);
    if clube == 0 || jogo == 0 {
        return o(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: sem clube ou jogo, só a original
    }
    let estado = cqa4_estado(clube);
    let antes = cqa4_i(estado);
    let previsto = if cqa4_b(jogo + 0x3a) != 1 && cqa4_d(copa + 0x2c) == 0 && cqa4_b(copa + 0x44) == 0xff && desfecho as u8 == 1
        && cqa4_vaga_na_uefa(cqa4_d(copa + 4), clube) { cqa4_i(0x9cf6f0) } else { antes };
    let r = o(copa, clube, desfecho, a3, a4, jogo, final_);
    let depois = cqa4_i(estado);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("copa_registros_pelo_desfecho(desfecho {}) estado {:x} -> {:x}, previsto {:x}, devolveu {}", desfecho as u8, antes, depois, previsto, r));
    r
}

/// Os quatro da linha do histórico: (campeão, vice, terceiro, seleção anfitriã). O campeão, o vice e o terceiro pelo
/// desfecho dos times (1, 2, 3; até achar os três); na copa galesa ([0x9cf958]) o terceiro é o primeiro time com o
/// progresso +0x73 = 3 no Comp_Stats (0x4a4850 de +8 do objeto da competição em [0xadadfc]: push clube; ecx = as
/// estatísticas -> thiscall, ret 4). A anfitriã: com a virtual +0x88 do objeto da competição (ecx = o objeto; sem push
/// -> thiscall, ret 0) e as anfitriãs do ano (+0x40) da HostCountry ([0xb5e134], 0x5fa730: push 0; push &b; push &a;
/// push ano; push id; ecx = [0xb5e134] -> thiscall, ret 0x14): duas, a seleção principal (0x53b3d0: push nação; add
/// esp, 4 -> cdecl) da de +0x8e maior (empate, a primeira); uma, a da primeira; nenhuma, 0.
unsafe fn cqa4_honras(copa: usize) -> (usize, usize, usize, usize) {
    let n = cqa4_w(copa + 0x3e);
    let (mut campeao, mut vice, mut terceiro) = (0usize, 0usize, 0usize);
    let mut i = 0i16;
    while i < n {
        let e = cqa4_d(copa + 0xa7).wrapping_add((i as isize * 6) as usize);
        match cqa4_b(e + 5) {
            1 => campeao = cqa4_d(e),
            2 => vice = cqa4_d(e),
            3 => terceiro = cqa4_d(e),
            _ => {}
        }
        if campeao != 0 && vice != 0 && terceiro != 0 {
            break;
        }
        i += 1;
    }
    let comp = cqa4_d(copa + 4);
    let objeto = || cqa4_d(cqa4_d(0xadadfc).wrapping_add((cqa4_i(cqa4_d(copa + 4)) as isize * 4) as usize));
    if comp != 0 && cqa4_i(comp) == cqa4_i(0x9cf958) {
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
        let mut j = 0i16;
        while j < cqa4_w(copa + 0x3e) {
            let c = cqa4_clube(copa, j as i32);
            if cqa4_b(busca(cqa4_d(objeto() + 8), c) + 0x73) == 3 {
                terceiro = c;
                break;
            }
            j += 1;
        }
    }
    let o = objeto();
    let tem: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqa4_d(cqa4_d(o) + 0x88));
    let mut anfitria = 0usize;
    if tem(o) != 0 {
        let (mut a, mut b) = (0i32, 0i32);
        let achar: unsafe extern "thiscall" fn(usize, i32, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(0x5fa730usize);
        // ponytail: o ano vai num registrador com lixo nos 16 bits altos; a 0x5fa730 recebe um short
        let k = achar(cqa4_d(0xb5e134), cqa4_i(cqa4_d(copa + 4)), cqa4_w(copa + 0x40) as u16 as u32, &mut a, &mut b, 0);
        let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
        let nacao = |x: i32| cqa4_d(0xae23a8).wrapping_add((x as isize * 0x122) as usize);
        if k == 2 {
            let (na, nb) = (nacao(a), nacao(b));
            anfitria = if cqa4_w(na + 0x8e) < cqa4_w(nb + 0x8e) { selecao(nb) } else { selecao(na) };
        } else if k != 0 {
            anfitria = selecao(nacao(a));
        }
    }
    (campeao, vice, terceiro, anfitria)
}

/// { 0x519a90 } (thiscall, ret 0, Cup.cpp; a virtual +0x30 das copas inglesas e da base): `Cup::record_season_honours()`:
/// a linha do histórico (0x4afce0: 5 pushes; add esp, 0x14 -> cdecl(copa, campeão, vice, terceiro, anfitriã)) com os
/// quatro de cqa4_honras; fora da competição amistosa, na copa principal da nação (cqa4_copa_principal), os
/// semifinalistas para a UEFA (0x519fc0: ecx = a copa -> thiscall, ret 0).
unsafe extern "thiscall" fn copa_honras_da_temporada(copa: usize) {
    let (campeao, vice, terceiro, anfitria) = cqa4_honras(copa);
    let historico: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x4afce0usize);
    historico(copa, campeao, vice, terceiro, anfitria);
    let comp = cqa4_d(copa + 4);
    if !cqa4_amistosa(comp) && cqa4_copa_principal(comp) {
        let semi: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x519fc0usize);
        semi(copa);
    }
}

/// Comparador da 0x519a90: só a original (a 0x4afce0 pode crescer o histórico e a 0x519fc0 tem o seu comparador). O
/// plano, feito antes: a linha livre do histórico (cpa2_linha_livre) recebe a competição (+4), o ano [0xae2c92] e os
/// quatro de cqa4_honras. Sem linha livre (o histórico cresce) ou sem a competição: só a original.
unsafe extern "thiscall" fn comparar_copa_honras_da_temporada(copa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let linha = cpa2_linha_livre();
    if linha == 0 || cqa4_d(copa + 4) == 0 {
        return o(copa); // ponytail: o histórico cresce (realloc) ou não há competição: só a original
    }
    let (campeao, vice, terceiro, anfitria) = cqa4_honras(copa);
    let ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
    o(copa);
    let lido = (cqa4_d(linha + 4), core::ptr::read_unaligned((linha + 8) as *const u16), cqa4_d(linha + 0xa), cqa4_d(linha + 0xe), cqa4_d(linha + 0x12), cqa4_d(linha + 0x16));
    let previsto = (cqa4_d(copa + 4), ano, campeao, vice, terceiro, anfitria);
    PLACAR.registrar(lido == previsto, || format!("copa_honras_da_temporada: linha {:x?} prevista {:x?}", lido, previsto));
}

/// Os dois clubes (ids) que a 0x519fc0 manda à semeadura da UEFA, ou None quando ela sai sem mandar: a rodada 0x82
/// (as semifinais) e a 0x96 (a final), as duas achadas; percorre os confrontos da 0x82 e os times que não estão no
/// primeiro confronto da 0x96: o primeiro vira a, o segundo b (aí para); um time −1 (ainda sem confronto) cancela;
/// faltando, −1.
unsafe fn cqa4_semifinalistas(copa: usize) -> Option<(i32, i32)> {
    let (mut semi, mut final_) = (-1i16, -1i16);
    let mut i = 0i16;
    while i < cqa4_w(copa + 0x3c) {
        let t = cqa4_w(cqa4_rodada(copa, i as i32) + 7);
        if t == 0x82 {
            semi = i;
        } else if t == 0x96 {
            final_ = i;
        }
        if semi != -1 && final_ != -1 {
            break;
        }
        i += 1;
    }
    if semi == -1 || final_ == -1 {
        return None;
    }
    let confrontos = |r: i16| cqa4_d(cqa4_d(copa + 0xad).wrapping_add((r as isize * 4) as usize));
    let f0 = confrontos(final_);
    let (fa, fb) = (cqa4_w(f0), cqa4_w(f0 + 2));
    let n = cqa4_w(cqa4_rodada(copa, semi as i32) + 0x1a);
    let base = confrontos(semi);
    let (mut a, mut b) = (-1i32, -1i32);
    let mut k = 0i16;
    'lacos: while k < n {
        let t = base.wrapping_add((k as isize * 0x35) as usize);
        for lado in [0usize, 2] {
            let x = cqa4_w(t + lado);
            if x == fa || x == fb {
                continue;
            }
            if x == -1 {
                return None;
            }
            let id = cqa4_i(cqa4_clube(copa, x as i32));
            if a == -1 {
                a = id;
            } else {
                b = id;
                break 'lacos;
            }
        }
        k += 1;
    }
    Some((a, b))
}

/// { 0x519fc0 } (thiscall, ret 0, Cup.cpp): `Cup::register_semi_final_losers_for_uefa()`: os semifinalistas fora da
/// final (cqa4_semifinalistas) vão à semeadura da UEFA (0x905790: push b; push a; push id da competição; ecx =
/// [0xde1f58]; sem add esp -> thiscall, ret 0xc).
unsafe extern "thiscall" fn copa_semifinalistas_para_a_uefa(copa: usize) {
    if let Some((a, b)) = cqa4_semifinalistas(copa) {
        let f: unsafe extern "thiscall" fn(usize, i32, i32, i32) = core::mem::transmute(0x905790usize);
        f(cqa4_d(0xde1f58), cqa4_i(cqa4_d(copa + 4)), a, b);
    }
}

/// Comparador da 0x519fc0: lado a lado na tabela da semeadura (+0x440, 50 vagas de 12 bytes, onde a 0x905790 grava; ela
/// não aloca: acha a vaga da competição ou a primeira livre). Sem a semeadura: só a original.
unsafe extern "thiscall" fn comparar_copa_semifinalistas_para_a_uefa(copa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let s = cqa4_d(0xde1f58);
    if s == 0 {
        return o(copa); // ponytail: sem a semeadura, só a original
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(s + 0x440, 50 * 12)], || o(copa), || copa_semifinalistas_para_a_uefa(copa));
    PLACAR.registrar(igual, || format!("copa_semifinalistas_para_a_uefa(copa {:x}) a semeadura difere ({:?})", copa, cqa4_semifinalistas(copa)));
}
