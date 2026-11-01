// ---------- %%LEVA%%a leva: o calendário: a nova data de um jogo, a entrada do jogo no calendário e a troca do ano do calendário (fix_man.cpp) ----------

unsafe fn cqc4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc4_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqc4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc4_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

type Cqc4Data = [u8; 8];

/// `a` depois de `b` como a original compara as datas: o ano (+2) sem sinal, depois o dia (+0) com sinal.
unsafe fn cqc4_depois(a: usize, b: usize) -> bool {
    let (ya, yb) = (cqc4_w(a + 2), cqc4_w(b + 2));
    ya > yb || (ya == yb && (cqc4_w(a) as i16) > (cqc4_w(b) as i16))
}

/// As operações de data (thiscall; as de +n devolvem a data estática delas ou `ret`): 0x54a110 a data padrão,
/// 0x549ef0 a data (dia, ano) (push ano; push dia), 0x417c10 a atribuição (push fonte; push ret), 0x54c5b0 + dias,
/// 0x54c8e0 − dias, 0x54c770 + semanas (push n; push ret), 0x54a2d0 o dia da semana alvo (push dest; push alvo),
/// 0x54a3f0 o dia da semana (sem pushes).
unsafe fn cqc4_padrao(d: &mut Cqc4Data) {
    let f: unsafe extern "thiscall" fn(*mut Cqc4Data) -> usize = core::mem::transmute(0x54a110usize);
    f(d);
}
unsafe fn cqc4_nova(d: &mut Cqc4Data, dia: u16, ano: u16) -> usize {
    let f: unsafe extern "thiscall" fn(*mut Cqc4Data, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    f(d, dia as u32, ano as u32)
}
unsafe fn cqc4_atribuir(d: &mut Cqc4Data, fonte: usize) {
    let f: unsafe extern "thiscall" fn(*mut Cqc4Data, *mut Cqc4Data, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut ret = [0u8; 8];
    f(d, &mut ret, fonte);
}
unsafe fn cqc4_op(rotina: usize, d: usize, n: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, *mut Cqc4Data, u32) -> usize = core::mem::transmute(rotina);
    let mut ret = [0u8; 8];
    // ponytail: a original passa um ret na pilha dela; as três escrevem nele ou na estática e devolvem o ponteiro,
    // que é lido logo depois (antes da próxima chamada), como na original
    let r = f(d, &mut ret, n);
    if r == ret.as_ptr() as usize {
        CQC4_RET = ret;
        return core::ptr::addr_of!(CQC4_RET) as usize;
    }
    r
}
static mut CQC4_RET: Cqc4Data = [0; 8];
unsafe fn cqc4_semana(d: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
    f(d)
}
unsafe fn cqc4_alvo(d: usize, alvo: u32, dest: &mut Cqc4Data) {
    let f: unsafe extern "thiscall" fn(usize, u32, *mut Cqc4Data) -> u32 = core::mem::transmute(0x54a2d0usize);
    f(d, alvo, dest);
}

/// A faixa de horário (0x5af2f0: mov ecx, gerenciador; push dia; push comp -> thiscall, ret 8).
unsafe fn cqc4_faixa(this: usize, comp: usize, dia: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x5af2f0usize);
    f(this, comp, dia)
}

/// Os conflitos do jogo (0x5ad780: mov ecx, gerenciador; push marca; push &gravidade; push jogo -> thiscall, ret
/// 0xc): a lista (ou 0) e a gravidade.
unsafe fn cqc4_conflitos(this: usize, jogo: usize, dois: u32) -> (usize, i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize = core::mem::transmute(0x5ad780usize);
    let mut g: i32 = 0;
    let l = f(this, jogo, &mut g, dois);
    (l, g)
}

/// Destrói e libera a lista (0x68d040: mov ecx, lista -> thiscall; 0x944c94: push; add esp, 4 -> cdecl).
unsafe fn cqc4_liberar(l: usize) {
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    destruir(l);
    liberar(l);
}

/// { 0x5ab510 } (thiscall, ret 4, fix_man.cpp; quadro SEH só das datas): `FIXTURE_MANAGER::find_new_fixture_date(jogo)`,
/// pela entrada no calendário (0x5aa680), pelos conflitos do mesmo dia (0x5ae380) e pela remarcação (0x5ae9d0). Sem
/// jogo: a caixa (a original). Pela casa das unidades da rodada (+0x30, 0x4b0850: push; add esp, 4 -> cdecl) 3 ou 4,
/// o registro de 15 bytes do jogo na lista de conferência das pernas [0xae2a48] (0x68d0d0). A janela pelo tipo (+0x3a):
/// 2, até a data de +0x24/+0x26 (sem ela, a caixa e 2 semanas), começando 4 semanas depois (se o fim passa de 20
/// semanas) ou 3 dias depois; 4 e 6, de 3 dias a 2 semanas; 5, de 0 a 1 semana; 7, 8 e 10, de 3 dias antes a 1 semana;
/// os outros, nenhuma. Em duas passadas (a segunda olhando os dois dias antes), pelos dias da semana na ordem 2, 1, 5,
/// 6, 0, 3, 4, de 4 em 4 dias da janela: o dia alvo (0x54a2d0) a partir de 2 dias depois de hoje e dentro da janela
/// vira a data (+0x28, +0x2a) e a faixa (+0x3f, 0x5af2f0); sem conflito (0x5ad780), com gravidade 1 ou abaixo do tipo,
/// fica. Sem achar: 2 dias depois da data do jogo (ou de hoje, se já passou).
unsafe extern "thiscall" fn nova_data_do_jogo(this: usize, jogo: usize) {
    if jogo == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, jogo); // a caixa
    }
    let (mut s, mut e, mut j, mut h, mut k) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    cqc4_padrao(&mut s);
    cqc4_padrao(&mut e);
    cqc4_padrao(&mut j);
    cqc4_padrao(&mut h);
    cqc4_padrao(&mut k);
    let dias: [i16; 9] = [2, 1, -1, 5, 6, -1, 0, 3, 4]; // cqc4: a ordem dos dias da semana
    let mut tmp = [0u8; 8];
    let r = cqc4_nova(&mut tmp, cqc4_w(jogo + 0x2a), cqc4_w(jogo + 0x28));
    cqc4_atribuir(&mut j, r);
    let unidades: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(0x4b0850usize);
    let rodada = cqc4_w(jogo + 0x30) as i16 as i32;
    if unidades(rodada) == 3 || unidades(rodada) == 4 {
        let mut reg = [0u8; 15];
        reg[0..4].copy_from_slice(&core::slice::from_raw_parts(jogo as *const u8, 4));
        reg[4] = cqc4_b(jogo + 0x42);
        reg[5..7].copy_from_slice(&cqc4_w(jogo + 0x34).to_le_bytes());
        reg[7..11].copy_from_slice(&core::slice::from_raw_parts((jogo + 0xc) as *const u8, 4));
        reg[11..15].copy_from_slice(&core::slice::from_raw_parts((jogo + 0x10) as *const u8, 4));
        let acrescentar: unsafe extern "thiscall" fn(usize, *const u8, u32) = core::mem::transmute(0x68d0d0usize);
        acrescentar(0xae2a48, reg.as_ptr(), 0xf);
    }
    let pj = j.as_ptr() as usize;
    match (cqc4_b(jogo + 0x3a) as i8 as i32).wrapping_sub(2) {
        0 => {
            if cqc4_w(jogo + 0x24) != 0 {
                let mut t = [0u8; 8];
                let r = cqc4_nova(&mut t, cqc4_w(jogo + 0x26), cqc4_w(jogo + 0x24));
                cqc4_atribuir(&mut e, r);
            } else {
                aviso_em(0x9cc208, 0x4f9); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
                let r = cqc4_op(0x54c770, pj, 2);
                cqc4_atribuir(&mut e, r);
            }
            let fim = e;
            let r = cqc4_op(0x54c770, pj, 0x14);
            let r = if cqc4_depois(fim.as_ptr() as usize, r) { cqc4_op(0x54c770, pj, 4) } else { cqc4_op(0x54c5b0, pj, 3) };
            cqc4_atribuir(&mut s, r);
        }
        2 | 4 => {
            let r = cqc4_op(0x54c770, pj, 2);
            cqc4_atribuir(&mut e, r);
            let r = cqc4_op(0x54c5b0, pj, 3);
            cqc4_atribuir(&mut s, r);
        }
        3 => {
            let r = cqc4_op(0x54c770, pj, 1);
            cqc4_atribuir(&mut e, r);
            cqc4_atribuir(&mut s, pj);
        }
        5 | 6 | 8 => {
            let r = cqc4_op(0x54c770, pj, 1);
            cqc4_atribuir(&mut e, r);
            let r = cqc4_op(0x54c8e0, pj, 3);
            cqc4_atribuir(&mut s, r);
        }
        _ => {}
    }
    let (ps, pe, ph, pk) = (s.as_ptr() as usize, e.as_ptr() as usize, h.as_ptr() as usize, k.as_ptr() as usize);
    for passada in 0..2u8 {
        for i in 0..3usize {
            for jj in 0..3usize {
                let alvo = dias[i * 3 + jj];
                if alvo == -1 {
                    continue;
                }
                cqc4_atribuir(&mut k, ps);
                while cqc4_depois(pe, pk) {
                    cqc4_alvo(pk, alvo as u16 as u32, &mut h);
                    let t = cqc4_op(0x54c5b0, 0xae2c90, 2);
                    let t = [cqc4_d(t), cqc4_d(t + 4)];
                    let pt = t.as_ptr() as usize;
                    if !cqc4_depois(pt, ph) && !cqc4_depois(ph, pe) {
                        crate::comparacao::ramo("cqc_4 nova data");
                        cqc4_pw(jogo + 0x28, cqc4_w(ph + 2));
                        cqc4_pw(jogo + 0x2a, cqc4_w(ph));
                        *((jogo + 0x3f) as *mut u8) = cqc4_faixa(this, cqc4_d(jogo + 0x14), alvo as u16 as u32);
                        let (l, g) = cqc4_conflitos(this, jogo, (passada == 1) as u32);
                        if l == 0 {
                            if g == 0 {
                                return;
                            }
                        } else {
                            cqc4_liberar(l);
                        }
                        if g == 1 || (cqc4_b(jogo + 0x3a) as i8 as i32) > g {
                            return;
                        }
                    }
                    let r = cqc4_op(0x54c5b0, pk, 4); // cqc4: o passo da busca
                    cqc4_atribuir(&mut k, r);
                }
            }
        }
    }
    if cqc4_depois(0xae2c90, pj) {
        cqc4_atribuir(&mut j, 0xae2c90);
    }
    let r = cqc4_op(0x54c5b0, pj, 2);
    cqc4_atribuir(&mut j, r);
    cqc4_pw(jogo + 0x2a, cqc4_w(pj));
    cqc4_pw(jogo + 0x28, cqc4_w(pj + 2));
}

/// O comparador da nova data: lado a lado no jogo (0x4f bytes); os conflitos de dentro (0x5ad780) criam e destroem a
/// lista deles. Só a original quando o jogo entra na lista de conferência das pernas (aloca o nó).
unsafe extern "thiscall" fn comparar_nova_data_do_jogo(this: usize, jogo: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let unidades: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(0x4b0850usize);
    // a caixa da janela sem data (+0x24, tipo 2) apareceria duas vezes: só a original
    if jogo == 0 || matches!(unidades(cqc4_w(jogo + 0x30) as i16 as i32), 3 | 4) || (cqc4_b(jogo + 0x3a) == 2 && cqc4_w(jogo + 0x24) == 0) {
        return o(this, jogo);
    }
    let reg = [(jogo, 0x4f)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogo), || nova_data_do_jogo(this, jogo));
    PLACAR.registrar(igual, || format!("nova_data_do_jogo(jogo {:x}) original={:x?} rust={:x?}", jogo, mo, mr));
}

/// { 0x5aa680 } (thiscall, ret 8, fix_man.cpp; quadro SEH das datas e do gerenciador novo): `FIXTURE_MANAGER::add_fixture(jogo,
/// zerar)`, pelas copas, ligas, amistosos e remarcações (20 chamadores). `zerar` zera a conta das remarcações em
/// cadeia (palavra [0xae2a40]). Acerta a fase (0x5af180: push; add esp, 4 -> cdecl), +0x2e = −1; jogo de data já
/// passada e não jogado: nova data (0x5ab510). Jogo da liga do mandante (+0x57 ou +0x60) sem rodada (+0x30, +0x32) nem
/// fase (+0x3b), uma vez (bit 0x800 de +0x4d): no sábado ou domingo, vai ao dia preferido do mandante (+0x72: 4, 5 ou
/// 6) se lá não houver conflito. Com menos de 10 remarcações em cadeia, os conflitos (0x5ad780): gravidade 0x14, o
/// conflito do mesmo dia (0x5ae380) e 1; copa ou tipo 9 recusado pela competição (+0x64): 0; gravidade de pelo menos o
/// tipo: 0 se a seleção do jogo tem convocação ([0xb74434], 0x7e6f20), senão nova data e entra de novo; senão entra
/// (0x5ab230) e os jogos do conflito saem: os amistosos (0x5c6710/0x5c6570 em [0xae2c74]), o tipo 9 (0x5abc30), os
/// outros remarcados (0x5abc30, 0x5ab510, 0x5aa680) com a notícia (0x5aac70 em [0xae2a58]). No ano do bloco (+0x1130):
/// a nação-chave (0x48dd70), a ordem (0x5ab3c0), os jogos de seleções (0x7e6720 em [0xb74434], 0x76bab0 em [0xb74318]),
/// os vizinhos (0x5ae9d0, chave 3 ou 5); se não entrou, na lista do dia, fatia e chave (0x68d0d0). Fora do ano, no
/// bloco do outro ano (+0x112c: 0x5aa2d0 para o ano, ou novo de 0x1184 bytes, 0x5a9cf0; sem memória, a caixa e 0).
unsafe extern "thiscall" fn jogo_no_calendario(this: usize, jogo: usize, zerar: u32) -> u32 {
    let fase: unsafe extern "C" fn(usize) = core::mem::transmute(0x5af180usize);
    let nova_data: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5ab510usize);
    let entrar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    let contagem = 0xae2a40 as *mut i16;
    let mut entrou = false;
    if zerar != 0 {
        *contagem = 0;
    }
    fase(jogo);
    cqc4_pw(jogo + 0x2e, 0xffff);
    let hoje = [cqc4_d(0xae2c90), cqc4_d(0xae2c94)];
    let mut dj = [0u8; 8];
    let r = cqc4_nova(&mut dj, cqc4_w(jogo + 0x2a), cqc4_w(jogo + 0x28));
    if !cqc4_depois(r, hoje.as_ptr() as usize) && cqc4_b(jogo + 0x43) == 0xff {
        nova_data(this, jogo);
    }
    let marcas = cqc4_w(jogo + 0x4d);
    let casa = cqc4_d(jogo + 0x1c);
    let comp = cqc4_d(jogo + 0x14);
    if marcas & 0x800 == 0
        && (comp == cqc4_d(casa + 0x57) || comp == cqc4_d(casa + 0x60))
        && cqc4_w(jogo + 0x32) == 0
        && cqc4_w(jogo + 0x30) == 0
        && cqc4_b(jogo + 0x3b) == 0xff
    {
        cqc4_pw(jogo + 0x4d, marcas | 0x800);
        let preferido = cqc4_b(casa + 0x72);
        if preferido != 0xff && (preferido == 5 || preferido == 6 || preferido == 4) {
            let mut d = [0u8; 8];
            cqc4_nova(&mut d, cqc4_w(jogo + 0x2a), cqc4_w(jogo + 0x28));
            let pd = d.as_ptr() as usize;
            let dia = cqc4_semana(pd) as u8;
            if preferido != dia && (dia == 5 || dia == 6) {
                let mut t = [0u8; 8];
                cqc4_padrao(&mut t);
                crate::comparacao::ramo("cqc_4 dia preferido do mandante");
                cqc4_alvo(pd, preferido as i8 as i16 as u16 as u32, &mut t); // cqc4: o dia preferido do mandante
                let pt = t.as_ptr() as usize;
                cqc4_pw(jogo + 0x28, cqc4_w(pt + 2));
                cqc4_pw(jogo + 0x2a, cqc4_w(pt));
                *((jogo + 0x3f) as *mut u8) = cqc4_faixa(this, cqc4_d(jogo + 0x14), cqc4_semana(pt) as u32);
                let (l, g) = cqc4_conflitos(this, jogo, 0);
                if l != 0 || g != 0 {
                    cqc4_pw(jogo + 0x2a, cqc4_w(pd));
                    cqc4_pw(jogo + 0x28, cqc4_w(pd + 2));
                    *((jogo + 0x3f) as *mut u8) = cqc4_faixa(this, cqc4_d(jogo + 0x14), cqc4_semana(pd) as u32);
                    if l != 0 {
                        cqc4_liberar(l);
                    }
                }
            }
        }
    }
    if *contagem < 10 {
        let (l, g) = cqc4_conflitos(this, jogo, 0);
        if l != 0 || g != 0 {
            *contagem = (*contagem).wrapping_add(1);
            if g == 0x14 {
                crate::comparacao::ramo("cqc_4 mesmo dia");
                let mesmo_dia: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x5ae380usize);
                mesmo_dia(this, jogo, l); // cqc4: o conflito do mesmo dia
                if l != 0 {
                    cqc4_liberar(l);
                }
                return 1;
            }
            let tipo = cqc4_b(jogo + 0x3a);
            if (tipo == 1 || tipo == 9) && cqc4_d(jogo + 0x14) != 0 && cqc4_d(0xadadfc) != 0 {
                let c = cqc4_d(cqc4_d(0xadadfc).wrapping_add((cqc4_d(cqc4_d(jogo + 0x14)) as usize).wrapping_mul(4)));
                if c != 0 {
                    let aceita: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(cqc4_d(cqc4_d(c) + 0x64));
                    if aceita(c, jogo) == 0 {
                        if l != 0 {
                            cqc4_liberar(l);
                        }
                        return 0;
                    }
                }
            }
            // 0x7e6f20: mov ecx, [0xb74434]; push comp -> thiscall, ret 4
            let convocada: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7e6f20usize);
            if g >= cqc4_b(jogo + 0x3a) as i8 as i32 {
                if l != 0 {
                    cqc4_liberar(l);
                }
                if convocada(cqc4_d(0xb74434), cqc4_d(jogo + 0x14)) != 0 {
                    return 0;
                }
                nova_data(this, jogo);
                return (entrar(this, jogo, 0) != 0) as u32;
            }
            let guardar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5ab230usize);
            guardar(this, jogo);
            entrou = true;
            if l != 0 {
                let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
                let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
                let tirar_amistoso: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5c6570usize);
                let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5abc30usize);
                let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x5aac70usize);
                core::ptr::write_unaligned((l + 0xa) as *mut usize, cqc4_d(l + 2));
                let mut x = proximo(l);
                while x != 0 {
                    if amistoso(cqc4_d(0xae2c74), cqc4_d(x + 0x14)) != 0 {
                        tirar_amistoso(cqc4_d(0xae2c74), x, 0);
                    } else if cqc4_b(x + 0x3a) == 9 {
                        tirar(this, x);
                    } else {
                        if convocada(cqc4_d(0xb74434), cqc4_d(jogo + 0x14)) != 0 {
                            return 0; // como na original, sem liberar a lista
                        }
                        let mut copia = [0u8; 0x4f];
                        core::ptr::copy_nonoverlapping(x as *const u8, copia.as_mut_ptr(), 0x4f);
                        let pc = copia.as_mut_ptr() as usize;
                        if tirar(this, x) != 0 {
                            nova_data(this, pc);
                            entrar(this, pc, 0);
                        }
                        noticia(cqc4_d(0xae2a58), pc, jogo, 0);
                    }
                    x = proximo(l);
                }
                cqc4_liberar(l);
            }
        }
    }
    let ano = cqc4_w(jogo + 0x28);
    let mut chave: i8 = 0;
    if ano == cqc4_w(this + 0x1130) {
        let chave_do_jogo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x48dd70usize);
        chave = chave_do_jogo(cqc4_d(jogo + 0x14), jogo) as i8;
        let ordem: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5ab3c0usize);
        ordem(this, jogo);
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        if selecao(cqc4_d(jogo + 0x1c)) != 0 && selecao(cqc4_d(jogo + 0x20)) != 0 && cqc4_b(jogo + 0x43) == 0xff {
            let c = cqc4_d(jogo + 0x14);
            let id = cqc4_d(c) as i32;
            if id != cqc4_d(0x9cf794) as i32 && id != cqc4_d(0x9cf798) as i32 && cqc4_b(jogo + 0x4e) & 3 == 0 {
                let especial: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x76d940usize);
                let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
                if especial(cqc4_d(0xb74318), c) == 0 || amistoso(cqc4_d(0xae2c74), cqc4_d(jogo + 0x14)) != 0 {
                    let marcar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7e6720usize);
                    marcar(cqc4_d(0xb74434), jogo);
                }
            }
            let jogo_de_selecoes: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x76bab0usize);
            jogo_de_selecoes(cqc4_d(0xb74318), jogo);
        }
        if chave == 3 || chave == 5 {
            let vizinhos: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5ae9d0usize);
            vizinhos(this, jogo);
        }
    }
    if entrou {
        return 1;
    }
    let ano = cqc4_w(jogo + 0x28);
    if ano == cqc4_w(this + 0x1130) {
        let dia = cqc4_w(jogo + 0x2a) as usize;
        let fatia = cqc4_b(jogo + 0x3f) as usize;
        let linha = cqc4_d(this + (dia * 3 + fatia) * 4);
        let lista = cqc4_d(linha.wrapping_add((chave as isize as usize).wrapping_mul(4)));
        let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
        return (acrescentar(lista, jogo, 0x4f) != 0) as u32;
    }
    let mut outro = cqc4_d(this + 0x112c);
    if outro != 0 {
        if cqc4_w(outro + 0x1130) != ano {
            let ano_do_bloco: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5aa2d0usize);
            ano_do_bloco(outro, ano as u32);
        }
    } else {
        let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
        let construir: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(0x5a9cf0usize);
        let b = novo(0x1184);
        outro = if b != 0 { construir(b, ano as u32, this, 0) } else { 0 };
        core::ptr::write_unaligned((this + 0x112c) as *mut usize, outro);
        if outro == 0 {
            aviso_em(0x9cc208, 0x2e6);
            return 0;
        }
    }
    entrar(cqc4_d(this + 0x112c), jogo, 0);
    1
}

/// O comparador da entrada no calendário: só a original (ponytail: grava nas listas do calendário, aloca nós e o
/// bloco do outro ano, remarca e entrega notícias; não pode rodar duas vezes). A cópia segue no modo Rust, com os
/// contadores.
unsafe extern "thiscall" fn comparar_jogo_no_calendario(this: usize, jogo: usize, zerar: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%)(this, jogo, zerar)
}

/// { 0x5aa2d0 } (thiscall, ret 4, fix_man.cpp; quadro SEH dos blocos novos): `FIXTURE_MANAGER::set_year(ano)`, pela entrada
/// no calendário (0x5aa680, o bloco do outro ano), pela guarda e pela retirada de jogos e pelos amistosos. O mesmo ano
/// (palavra, +0x1130): 1. Grava os jogos do ano (0x5ac060: ecx = bloco; push 0 -> thiscall; falhando, a caixa e 0); se
/// o outro bloco (+0x112c) está no ano pedido, ele vai para o ano vizinho do deste (0x5aa2d0 nele); destrói e libera as
/// listas dos 366 dias × 3 fatias × nações-chave ([0xb64738], byte com sinal; 0x68d040, 0x944c94); o outro bloco no
/// ano pedido, se este é o principal (+0x1183), é destruído e liberado (0x5aa0d0: ecx = bloco -> thiscall; 0x944c94); o
/// ano novo, bissexto em +0x1132 (0x54cfe0: push ano; add esp, 4 -> cdecl), as listas novas (new 0xe: 0x944cf1, push;
/// add esp, 4 -> cdecl; 0x68d020: ecx = bloco -> thiscall; sem memória, a caixa e 0) e a carga dos jogos do ano
/// (0x5ac3b0: ecx = bloco -> thiscall). Devolve 1.
unsafe extern "thiscall" fn ano_do_calendario(this: usize, ano: u32) -> u32 {
    let a = ano as u16;
    if a == cqc4_w(this + 0x1130) {
        return 1;
    }
    let gravar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5ac060usize);
    if gravar(this, 0) == 0 {
        aviso_em(0x9cc208, 0x126);
        return 0;
    }
    let outro = cqc4_d(this + 0x112c);
    if outro != 0 {
        let oa = cqc4_w(outro + 0x1130);
        if oa == a {
            let vizinho = if cqc4_w(this + 0x1130) as u32 == oa as u32 + 1 { oa.wrapping_sub(1) } else { oa.wrapping_add(1) };
            let mudar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5aa2d0usize);
            mudar(outro, vizinho as u32);
        }
    }
    let n = || cqc4_b(0xb64738) as i8;
    for dia in 0..0x16eusize {
        if n() > 0 {
            let base = this + 8 + dia * 0xc;
            let mut k: i16 = 0;
            loop {
                for p in [base - 8, base - 4, base] {
                    let slot = cqc4_d(p).wrapping_add((k as i32 as usize).wrapping_mul(4));
                    let l = cqc4_d(slot);
                    if l != 0 {
                        cqc4_liberar(l);
                        core::ptr::write_unaligned(cqc4_d(p).wrapping_add((k as i32 as usize).wrapping_mul(4)) as *mut usize, 0);
                    }
                }
                k = k.wrapping_add(1);
                if k >= n() as i16 {
                    break;
                }
            }
        }
    }
    let outro = cqc4_d(this + 0x112c);
    if outro != 0 && cqc4_w(outro + 0x1130) == a && cqc4_b(this + 0x1183) != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5aa0d0usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(outro);
        liberar(outro);
        core::ptr::write_unaligned((this + 0x112c) as *mut usize, 0);
    }
    cqc4_pw(this + 0x1130, a);
    let bissexto: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x54cfe0usize);
    *((this + 0x1132) as *mut u8) = (bissexto(ano) != 0) as u8;
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    crate::comparacao::ramo("cqc_4 ano novo do calendario");
    for dia in 0..0x16eusize {
        if n() > 0 {
            let e = this + dia * 0xc;
            let mut k: i16 = 0;
            loop {
                let desloc = (k as i32 as usize).wrapping_mul(4);
                for j in 0..3usize {
                    let b = novo(0xe);
                    let l = if b != 0 { construir(b) } else { 0 };
                    core::ptr::write_unaligned(cqc4_d(e + j * 4).wrapping_add(desloc) as *mut usize, l);
                }
                if (0..3usize).any(|j| cqc4_d(cqc4_d(e + j * 4).wrapping_add(desloc)) == 0) {
                    aviso_em(0x9cc208, 0x170);
                    return 0;
                }
                k = k.wrapping_add(1);
                if k >= n() as i16 {
                    break;
                }
            }
        }
    }
    let carregar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5ac3b0usize);
    carregar(this);
    1
}

/// O comparador do ano do calendário: só a original (ponytail: grava e lê os arquivos dos jogos do ano e refaz todas
/// as listas; não pode rodar duas vezes).
unsafe extern "thiscall" fn comparar_ano_do_calendario(this: usize, ano: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I2%%)(this, ano)
}