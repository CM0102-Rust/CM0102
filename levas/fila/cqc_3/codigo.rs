// ---------- %%LEVA%%a leva: os conflitos de calendário: os conflitos de um jogo nos dias em volta, os conflitos de um dia e a carga da conferência das pernas (fix_man.cpp) ----------

unsafe fn cqc3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqc3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqc3_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqc3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O ensaio do comparador dos conflitos de um dia: a cópia não acrescenta (0x68d0d0 aloca o nó e a cópia do jogo) e
/// guarda os jogos que acrescentaria, na ordem.
static mut CQC3_ENSAIO: bool = false;
static mut CQC3_ACRESCIMOS: Vec<usize> = Vec::new();

/// Acrescenta a cópia dos 0x4f bytes do jogo no fim da lista (0x68d0d0: ecx = lista; push 0x4f; push jogo -> thiscall,
/// ret 8); no ensaio, só guarda o jogo.
unsafe fn cqc3_acrescentar(lista: usize, jogo: usize) {
    if CQC3_ENSAIO {
        CQC3_ACRESCIMOS.push(jogo);
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    f(lista, jogo, 0x4f);
}

/// Os dados dos nós da lista (cabeça em +2, o próximo em +6 do nó, o dado em +0), na ordem.
unsafe fn cqc3_itens(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if lista == 0 {
        return v;
    }
    let mut no = cqc3_d(lista + 2);
    while no != 0 {
        v.push(cqc3_d(no));
        no = cqc3_d(no + 6);
    }
    v
}

/// O conteúdo (0x4f bytes por jogo) de uma sequência de jogos.
unsafe fn cqc3_conteudo(jogos: &[usize]) -> Vec<u8> {
    let mut v = Vec::new();
    for &j in jogos {
        v.extend_from_slice(core::slice::from_raw_parts(j as *const u8, 0x4f));
    }
    v
}

/// A data (dia do mês, mês 0..11) do ano dado (0x549f70: lea ecx, local; push −1; push ano; push mês; push dia ->
/// thiscall, ret 0x10): (dia do ano, ano).
unsafe fn cqc3_dma(dia: u32, mes: u32, ano: u32) -> (u16, u16) {
    let f: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32, u32, u32) -> *mut [u8; 8] = core::mem::transmute(0x549f70usize);
    let mut t = [0u8; 8];
    let r = f(&mut t, dia, mes, ano, 0xffff_ffff);
    (cqc3_w(r as usize), cqc3_w(r as usize + 2))
}

/// O jogo `x` está na liga de id [0x9cf590] por um dos clubes (+0x57 → +0).
unsafe fn cqc3_na_liga(clube: usize, id: i32) -> bool {
    let l = cqc3_d(clube + 0x57);
    l != 0 && cqc3_i(l) == id
}

/// A regra de conflito da competição (método +0x60 da competição [0xadadfc][id]: push b; push a; ecx = comp ->
/// thiscall): 0 sem a tabela ou sem a competição.
unsafe fn cqc3_regra(id_de: usize, a: usize, b: usize) -> u32 {
    let tab = cqc3_d(0xadadfc);
    if tab == 0 {
        return 0;
    }
    let c = cqc3_d(tab.wrapping_add((cqc3_i(id_de) as usize).wrapping_mul(4)));
    if c == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(cqc3_d(cqc3_d(c) + 0x60));
    f(c, a, b)
}

/// O jogo `x` entra (0x16 se já foi jogado, +0x43 diferente de 0xff) e a gravidade sobe ao tipo dele (+0x3a).
unsafe fn cqc3_entra_pelo_tipo(grav: *mut i32, lista: usize, x: usize, conferir_jogado: bool) {
    if conferir_jogado && cqc3_b(x + 0x43) != 0xff {
        *grav = 0x16;
        return;
    }
    cqc3_acrescentar(lista, x);
    let t = cqc3_b(x + 0x3a) as i8 as i32;
    if t > *grav {
        *grav = t;
    }
}

/// O fim comum dos ramos do mesmo clube e do mesmo estádio (0x5adf91/0x5ae060): sem gravidade, 0x14 (`vinte`) ou 1.
unsafe fn cqc3_sem_gravidade(grav: *mut i32, vinte: bool) {
    if *grav == 0 {
        *grav = if vinte { 0x14 } else { 1 };
    }
}

/// { 0x5adba0 } (thiscall, ret 0x1c, fix_man.cpp; quadro SEH só das datas): `FIXTURE_MANAGER::collect_day_clashes(jogo,
/// gravidade, lista, nações, n, data, deslocamento)`, pelos conflitos de um jogo (0x5ad780) no dia `data` + deslocamento
/// (0x54c5b0: push n de 16 bits; push &local; ecx = data -> thiscall, ret 8). Copa (+0x3a = 1) só no próprio dia. No
/// próprio dia, as datas proibidas dão 19 e saem: 25 de dezembro (fora das competições da nação de [0x9cf36c]), o
/// domingo nas da nação de [0x9cf3e4] (0x54a3f0: ecx = data -> thiscall), entre 10 e 20 de abril nas de [0x9cf31c] (as
/// datas pela 0x549f70: 4 pushes -> thiscall); 17 de maio com clube da nação [0x9cf408] (também com deslocamento,
/// sem sair, se for o próprio dia). Depois, para cada nação de `nações` (fora as 0xff) e cada uma das 3 fatias, a lista
/// do dia (0x5abf10: ecx = gerenciador; 4 pushes -> thiscall, ret 0x10) com o cursor do começo: cada jogo dela (0x68d1b0)
/// que divide clube e fatia (+0x40/+0x41) entra pelo tipo; clube em outra fatia, entra com 0x14 ou 1 pelo +0x3f e pelo
/// tipo 1; o mesmo estádio (+0x18) fora da competição de [0x9cf76c], igual; as seleções da nação dos clubes (0x53b3d0:
/// push; add esp, 4 -> cdecl) e os clubes de reputação acima de 5000 pela regra da competição (+0x60); os jogos da
/// competição de [0x9cf690] com clube da liga de [0x9cf590] dão 19; jogo já jogado (+0x43) dá 0x16. Clube ausente: a
/// caixa e o fim do jogo (a original).
unsafe extern "thiscall" fn conflitos_do_dia(this: usize, jogo: usize, grav: *mut i32, lista: usize, nacoes: usize, n: u32, data: usize, desloc: u32) {
    // 0x53b900 (push; add esp, 4 -> cdecl), 0x53b3d0 (push; add esp, 4 -> cdecl)
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let semana: unsafe extern "thiscall" fn(*const [u8; 8]) -> i32 = core::mem::transmute(0x54a3f0usize);
    let lista_do_dia: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5abf10usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    // 0x76d940: ecx = [0xb74318]; push comp -> thiscall, ret 4 (a cópia Rust ignora o ecx)
    let especial: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x76d940usize);
    let e_sel = selecao(cqc3_d(jogo + 0x1c));
    let n1 = cqc3_d(cqc3_d(jogo + 0x1c) + 0x53);
    // ponytail: sem nação a original deixa a seleção sem valor; todo clube tem nação, então 0 não acontece no jogo
    let s1 = if n1 != 0 { principal(n1) } else { 0 };
    let n2 = cqc3_d(cqc3_d(jogo + 0x20) + 0x53);
    let s2 = if n2 != 0 { principal(n2) } else { 0 };
    let flag = desloc as u8 as i8;
    let mut dia = [0u8; 8];
    mais(data, &mut dia, flag as i16 as u16 as u32);
    let d = dia.as_ptr() as usize;
    let ano = || cqc3_d(d + 2) as u32; // o dword do ano como a original o empurra (a 0x549f70 e a 0x5abf10 leem a palavra)
    let mesma = |t: (u16, u16)| t.1 == cqc3_w(d + 2) && t.0 == cqc3_w(d);
    let dezenove = || {
        if *grav < 0x13 {
            *grav = 0x13;
        }
    };
    let tipo = cqc3_b(jogo + 0x3a);
    if tipo == 1 && flag != 0 {
        return;
    }
    if flag == 0 {
        let comp = cqc3_d(jogo + 0x14);
        let nc = if comp != 0 { cqc3_d(comp + 0x5d) } else { 0 };
        let e_da = |g: usize| nc != 0 && cqc3_i(nc) == cqc3_i(g);
        if mesma(cqc3_dma(0x19, 0xb, ano())) && !e_da(0x9cf36c) {
            crate::comparacao::ramo("cqc_3 natal");
            return dezenove();
        }
        if e_da(0x9cf3e4) && semana(&dia) as i16 == 6 {
            return dezenove();
        }
        if e_da(0x9cf31c) {
            let t = cqc3_dma(10, 3, ano());
            if t.1 < cqc3_w(d + 2) || (t.1 == cqc3_w(d + 2) && (t.0 as i16) < cqc3_w(d) as i16) {
                let t = cqc3_dma(0x14, 3, ano());
                if t.1 > cqc3_w(d + 2) || (t.1 == cqc3_w(d + 2) && (t.0 as i16) > cqc3_w(d) as i16) {
                    return dezenove();
                }
            }
        }
    }
    let nac = cqc3_d(0xae23a8).wrapping_add((cqc3_i(0x9cf408) as usize).wrapping_mul(0x122));
    if cqc3_d(cqc3_d(jogo + 0x1c) + 0x53) == nac || cqc3_d(cqc3_d(jogo + 0x20) + 0x53) == nac {
        if mesma(cqc3_dma(0x11, 4, ano())) && flag == 0 {
            return dezenove();
        }
    }
    let mut k: i8 = 0;
    while k < n as u8 as i8 {
        let p = nacoes.wrapping_add(k as isize as usize);
        if cqc3_b(p) != 0xff {
            let mut fatia: i8 = 0;
            while fatia <= 2 {
                let l = lista_do_dia(this, ano(), cqc3_d(d) as u32, fatia as u8 as u32, cqc3_b(p) as u32);
                if l != 0 {
                    core::ptr::write_unaligned((l + 0xa) as *mut usize, cqc3_d(l + 2));
                    let mut x = proximo(l);
                    while x != 0 {
                        let c1 = cqc3_d(x + 0x1c);
                        let h = cqc3_d(jogo + 0x1c);
                        let a = cqc3_d(jogo + 0x20);
                        if c1 == 0 || h == 0 || a == 0 {
                            // a caixa (linha 0xcc8) e o fim do jogo: a original
                            return orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32)>(%%I0%%)(this, jogo, grav, lista, nacoes, n, data, desloc);
                        }
                        let c2 = cqc3_d(x + 0x20);
                        let (sx0, sx1, s0, s1b) = (cqc3_b(x + 0x40), cqc3_b(x + 0x41), cqc3_b(jogo + 0x40), cqc3_b(jogo + 0x41));
                        if (c1 == h && sx0 == s0) || (c1 == a && sx0 == s1b) || (c2 == h && sx1 == s0) || (c2 == a && sx1 == s1b) {
                            crate::comparacao::ramo("cqc_3 mesmo clube na mesma fatia");
                            cqc3_entra_pelo_tipo(grav, lista, x, true); // cqc3: o mesmo clube na mesma fatia
                        } else if (flag == 0 && c1 == h && sx0 != s0) || (c1 == a && sx0 != s1b) || (c2 == h && sx1 != s0) || (c2 == a && sx1 != s1b) {
                            if cqc3_b(x + 0x43) != 0xff {
                                *grav = 0x16;
                            } else {
                                cqc3_acrescentar(lista, x);
                                if cqc3_b(jogo + 0x3f) == cqc3_b(x + 0x3f) || cqc3_b(x + 0x3a) != 1 {
                                    if *grav == 0 {
                                        crate::comparacao::ramo("cqc_3 mesmo clube noutra fatia");
                                    }
                                    cqc3_sem_gravidade(grav, true); // cqc3: o mesmo clube noutra fatia
                                } else if cqc3_b(jogo + 0x3a) != 1 {
                                    cqc3_sem_gravidade(grav, false);
                                }
                            }
                        } else if flag == 0 && cqc3_d(x + 0x18) != 0 && cqc3_d(jogo + 0x18) == cqc3_d(x + 0x18) && cqc3_i(x) != cqc3_i(0x9cf76c) && cqc3_i(jogo) != cqc3_i(0x9cf76c) {
                            if cqc3_b(x + 0x43) != 0xff {
                                *grav = 0x16;
                            } else {
                                cqc3_acrescentar(lista, x);
                                let t = cqc3_b(x + 0x3a);
                                if cqc3_b(jogo + 0x3f) == cqc3_b(x + 0x3f) {
                                    if t != 1 {
                                        cqc3_sem_gravidade(grav, true);
                                    } else if *grav < 1 {
                                        *grav = 1;
                                    }
                                } else if t != 1 {
                                    cqc3_sem_gravidade(grav, true);
                                } else if cqc3_b(jogo + 0x3a) != 1 {
                                    cqc3_sem_gravidade(grav, false);
                                }
                            }
                        } else {
                            let abs_flag = (flag as i32).abs();
                            let selecoes = (e_sel == 0
                                && especial(cqc3_d(0xb74318), cqc3_d(x + 0x14)) == 0
                                && abs_flag < 2
                                && (cqc3_b(p) == 0 || cqc3_b(p) == 1)
                                && (cqc3_b(jogo + 0x3a) as i8) < 5
                                && c1 == s1)
                                || c1 == s2
                                || c2 == s1
                                || c2 == s2;
                            if selecoes {
                                if (cqc3_w(h + 0x80) as i16 > 0x1388 || cqc3_w(a + 0x80) as i16 > 0x1388) && cqc3_regra(jogo, jogo, x) != 0 {
                                    cqc3_entra_pelo_tipo(grav, lista, x, true);
                                }
                            } else if cqc3_i(x) == cqc3_i(0x9cf690) && (cqc3_na_liga(h, cqc3_i(0x9cf590)) || cqc3_na_liga(a, cqc3_i(0x9cf590))) {
                                *grav = 0x13;
                            } else if cqc3_i(jogo) == cqc3_i(0x9cf690) && (cqc3_na_liga(c1, cqc3_i(0x9cf590)) || cqc3_na_liga(c2, cqc3_i(0x9cf590))) {
                                cqc3_entra_pelo_tipo(grav, lista, x, false);
                            } else if e_sel != 0
                                && especial(cqc3_d(0xb74318), cqc3_d(jogo + 0x14)) == 0
                                && abs_flag < 2
                                && cqc3_b(p) != 0
                                && cqc3_b(p) != 1
                            {
                                let (nx1, nx2) = (cqc3_d(c1 + 0x53), cqc3_d(c2 + 0x53));
                                if (nx1 == n1 || nx2 == n1 || nx1 == n2 || nx2 == n2)
                                    && (cqc3_w(c1 + 0x80) as i16 > 0x1388 || cqc3_w(c2 + 0x80) as i16 > 0x1388)
                                    && (cqc3_b(x + 0x3a) as i8) < 5
                                    && cqc3_regra(x, x, jogo) != 0
                                {
                                    cqc3_entra_pelo_tipo(grav, lista, x, true);
                                }
                            }
                        }
                        x = proximo(l);
                    }
                }
                fatia += 1;
            }
        }
        k += 1;
    }
}

/// O comparador dos conflitos de um dia: a cópia em ensaio (guarda os jogos em vez de acrescentar, com uma gravidade
/// própria) e a original a partir do mesmo estado; confere a gravidade e os nós novos no fim da lista (os 0x4f bytes de
/// cada jogo, na ordem). As listas dos dias (0x5abf10) têm o cursor refeito nas duas.
unsafe extern "thiscall" fn comparar_conflitos_do_dia(this: usize, jogo: usize, grav: *mut i32, lista: usize, nacoes: usize, n: u32, data: usize, desloc: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32)>(%%I0%%);
    if jogo == 0 || lista == 0 || grav.is_null() || cqc3_d(jogo + 0x1c) == 0 || cqc3_d(jogo + 0x20) == 0 {
        return o(this, jogo, grav, lista, nacoes, n, data, desloc);
    }
    let mut g = *grav;
    CQC3_ACRESCIMOS.clear();
    CQC3_ENSAIO = true;
    conflitos_do_dia(this, jogo, &mut g, lista, nacoes, n, data, desloc);
    CQC3_ENSAIO = false;
    let esperado = cqc3_conteudo(&CQC3_ACRESCIMOS);
    let ne = CQC3_ACRESCIMOS.len();
    let antes = cqc3_itens(lista);
    o(this, jogo, grav, lista, nacoes, n, data, desloc);
    let depois = cqc3_itens(lista);
    let igual = *grav == g && depois.len() == antes.len() + ne && depois[..antes.len()] == antes[..] && cqc3_conteudo(&depois[antes.len()..]) == esperado;
    PLACAR.registrar(igual, || format!("conflitos_do_dia(jogo {:x}, desloc {}) gravidade original={} rust={}, jogos novos original={} rust={}", jogo, desloc as u8 as i8, *grav, g, depois.len().saturating_sub(antes.len()), ne));
}

/// { 0x5ad780 } (thiscall, ret 0xc, fix_man.cpp; quadro SEH da lista nova): `FIXTURE_MANAGER::find_fixture_clashes(jogo,
/// gravidade, dois_dias)`, pelo calendário (0x5aa680, 0x5ab510, 0x5ae380, 0x5ae9d0). Sem jogo ou sem saída: a caixa (a
/// original). Zera a saída; as nações a olhar: 0 a 6, a nação-chave do jogo (0x48dd70: 2 pushes; add esp, 8 -> cdecl),
/// as das ligas dos dois clubes, a do estádio (0x48df40: push; add esp, 4 -> cdecl), as parceiras (0x5af5b0: push;
/// add esp, 4 -> cdecl) e, com seleção mandante (0x53b900), as das duas seleções (0x48dee0: push; add esp, 4 -> cdecl),
/// sem repetir. A chave 3 ou 5 fora da competição de [0x9cf7bc] desliga os dois dias antes. A lista nova (new 0xe:
/// 0x944cf1, push; add esp, 4 -> cdecl; 0x68d020: ecx = bloco -> thiscall; sem memória, a caixa e 0) e os conflitos
/// de cada dia (0x5adba0: ecx = this; 7 pushes -> thiscall, ret 0x1c) pela data do jogo (0x549ef0: push +0x28; push
/// +0x2a; ecx = local -> thiscall): abaixo do tipo 0x17, os dois dias antes (com a marca) e o anterior; o próprio dia e
/// o seguinte; de novo os dois antes com a marca. Sem conflito: a lista destruída e liberada (0x68d040: ecx = lista ->
/// thiscall; 0x944c94: push; add esp, 4 -> cdecl), 0x15 na saída se a seleção não deixa (0x776fc0: ecx = [0xb74318];
/// push jogo -> thiscall, ret 4) e 0. Senão a gravidade na saída e a lista.
unsafe extern "thiscall" fn conflitos_do_jogo(this: usize, jogo: usize, saida: *mut i32, dois_dias: u32) -> usize {
    if jogo == 0 || saida.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(%%I1%%)(this, jogo, saida, dois_dias); // a caixa
    }
    let chave_do_jogo: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x48dd70usize);
    let do_estadio: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x48df40usize);
    let parceira: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x5af5b0usize);
    let da_selecao: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x48dee0usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let nova_data: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let do_dia: unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32) = core::mem::transmute(0x5adba0usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let pode: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x776fc0usize);
    let mut dois = dois_dias;
    *saida = 0;
    let mut a = [0xffu8; 16];
    for k in 0..7 {
        a[k] = k as u8;
    }
    let chave = chave_do_jogo(cqc3_d(jogo + 0x14), jogo) as i8;
    a[7] = chave as u8;
    let mut n: usize = 8;
    let casa = cqc3_d(jogo + 0x1c);
    if casa != 0 && cqc3_d(casa + 0x57) != 0 {
        a[8] = chave_do_jogo(cqc3_d(casa + 0x57), 0);
        n = 9;
    }
    let fora = cqc3_d(jogo + 0x20);
    if fora != 0 && cqc3_d(fora + 0x57) != 0 {
        a[n] = chave_do_jogo(cqc3_d(fora + 0x57), 0);
        n += 1;
    }
    if cqc3_d(jogo + 0x18) != 0 {
        a[n] = do_estadio(cqc3_d(jogo + 0x18));
        n += 1;
    }
    a[n] = parceira(cqc3_d(jogo + 0x1c));
    n += 1;
    a[n] = parceira(cqc3_d(jogo + 0x20));
    n += 1;
    if cqc3_i(jogo) != cqc3_i(0x9cf7bc) && (chave == 3 || chave == 5) {
        dois = 0;
    }
    if selecao(cqc3_d(jogo + 0x1c)) != 0 {
        a[n] = da_selecao(cqc3_d(jogo + 0x1c));
        n += 1;
        a[n] = da_selecao(cqc3_d(jogo + 0x20));
        n += 1;
    }
    for i in 0..n.saturating_sub(1) {
        for j in i + 1..n {
            if (a[i] as i8) > -1 && a[i] == a[j] {
                a[j] = 0xff;
            }
        }
    }
    let bloco = novo(0xe);
    let lista = if bloco != 0 { construir(bloco) } else { 0 };
    if lista == 0 {
        aviso_em(0x9cc208, 0xc1d); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
        return 0;
    }
    let mut data = [0u8; 8];
    nova_data(&mut data, cqc3_w(jogo + 0x2a) as u32, cqc3_w(jogo + 0x28) as u32);
    let mut g: i32 = 0;
    let (pa, pd) = (a.as_ptr() as usize, data.as_ptr() as usize);
    if (cqc3_b(jogo + 0x3a) as i8) < 0x17 {
        if dois != 0 {
            do_dia(this, jogo, &mut g, lista, pa, n as u32, pd, -2i32 as u32);
        }
        do_dia(this, jogo, &mut g, lista, pa, n as u32, pd, -1i32 as u32);
    }
    do_dia(this, jogo, &mut g, lista, pa, n as u32, pd, 0);
    crate::comparacao::ramo("cqc_3 dia seguinte");
    do_dia(this, jogo, &mut g, lista, pa, n as u32, pd, 1); // cqc3: o dia seguinte
    if dois != 0 {
        do_dia(this, jogo, &mut g, lista, pa, n as u32, pd, -2i32 as u32);
    }
    if g == 0 {
        destruir(lista);
        liberar(lista);
        if pode(cqc3_d(0xb74318), jogo) == 0 {
            *saida = 0x15;
        }
        return 0;
    }
    *saida = g;
    lista
}

/// O comparador dos conflitos de um jogo: a original e a cópia, cada uma com a sua lista nova, a partir do mesmo estado
/// (a original não muda o jogo: só cria a lista e mexe nos cursores das listas dos dias, que as duas refazem); confere a
/// devolução (lista ou 0), a saída e os jogos das duas listas (0x4f bytes cada, na ordem), e destrói e libera a lista
/// da cópia como o calendário faz (0x68d040, 0x944c94).
unsafe extern "thiscall" fn comparar_conflitos_do_jogo(this: usize, jogo: usize, saida: *mut i32, dois_dias: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(%%I1%%);
    if jogo == 0 || saida.is_null() {
        return o(this, jogo, saida, dois_dias);
    }
    let r = o(this, jogo, saida, dois_dias);
    let go = *saida;
    let mut gr: i32 = 0;
    let nr = conflitos_do_jogo(this, jogo, &mut gr, dois_dias);
    let (lo, lr) = (cqc3_conteudo(&cqc3_itens(r)), cqc3_conteudo(&cqc3_itens(nr)));
    let igual = (r == 0) == (nr == 0) && go == gr && lo == lr;
    PLACAR.registrar(igual, || format!("conflitos_do_jogo(jogo {:x}) original={:x} saida={} jogos={} rust={:x} saida={} jogos={}", jogo, r, go, lo.len() / 0x4f, nr, gr, lr.len() / 0x4f));
    if nr != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(nr);
        liberar(nr);
    }
    r
}

/// { 0x5af390 } (cdecl, fix_man.cpp): `fix_man_load_fixture_leg_checks()`, pela construção do calendário principal
/// (0x5a9cf0, na carga e na criação do jogo): esvazia a lista das conferências das pernas [0xae2a48] (0x68d080: ecx =
/// lista -> thiscall) e a refaz com os registros de 15 bytes do check_fixture_legs.tmp da pasta temporária (0x90bde0:
/// 2 pushes; add esp, 8 -> cdecl; fopen 0x945b00 "rb", fread 0x945bfe, fclose 0x945a09: cdecl), pela conta de 16 bits
/// do começo; registro curto: a caixa (linha 0x10b9) e entra o que ficou no registro (0x68d0d0).
unsafe extern "C" fn carregar_conferencia_das_pernas() {
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let ler: unsafe extern "C" fn(*mut u8, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8, u32) = core::mem::transmute(0x68d0d0usize);
    esvaziar(0xae2a48);
    let f = abrir(caminho(0x987da0, 0x9cc258), 0x98f928); // "CM3_TEMP", "check_fixture_legs.tmp", "rb"
    if f == 0 {
        return;
    }
    // ponytail: com a leitura da conta falhando a original usa o lixo da pilha; aqui 0 (arquivo do próprio jogo)
    let mut conta = [0u8; 2];
    ler(conta.as_mut_ptr(), 2, 1, f);
    let conta = i16::from_le_bytes(conta);
    let mut reg = [0u8; 15];
    let mut k: i16 = 0;
    while k < conta {
        if ler(reg.as_mut_ptr(), 0xf, 1, f) != 1 {
            aviso_em(0x9cc208, 0x10b9);
        }
        acrescentar(0xae2a48, reg.as_ptr(), 0xf);
        k = k.wrapping_add(1);
    }
    fechar(f);
}

/// O comparador da carga das conferências: só a original (ponytail: lê o arquivo e aloca os nós da lista; não pode rodar
/// duas vezes).
unsafe extern "C" fn comparar_carregar_conferencia_das_pernas() {
    orig::<unsafe extern "C" fn()>(%%I2%%)()
}
