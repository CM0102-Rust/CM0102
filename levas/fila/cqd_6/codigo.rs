// ---------- %%LEVA%%a leva: o calendário: o dia do calendário, a lista dos jogos do dia e a carga dos jogos do ano (fix_man.cpp) ----------

unsafe fn cqd6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqd6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O registro do clube `i` (0x245 bytes em [0xae23b4]; a original calcula base + i + 580·i).
unsafe fn cqd6_clube(i: usize) -> usize {
    cqd6_d(0xae23b4).wrapping_add(i).wrapping_add(i.wrapping_mul(0x244))
}

/// O clube tem técnico humano (0x600a50: push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl).
unsafe fn cqd6_humano(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, 1, 1, 0, 0) != 0
}

/// { 0x5aaf00 } (thiscall, ret 0, fix_man.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `FIXTURE_MANAGER::daily_update()`, do laço do dia (0x5cc850). Pelo dia da semana de hoje (0x54a3f0 em 0xae2c90):
/// 1. dias 2 e 3: os jogos de reservas para daqui a 3 dias (0x54c5b0 + 0x7e6380: push &data; mov ecx, [0xb74434] ->
///    thiscall, ret 4) e, no dia 3, também para amanhã;
/// 2. a fila das notícias de jogo ([+0x1128], uma lista): cada notícia vai ao técnico do clube da vaga 0 se a vaga 0xe
///    pede e ao do clube da vaga 1 se a vaga 0xf pede (0x785830: push k; mov ecx, notícia -> thiscall, ret 4;
///    0x7861e0: push clube; push notícia; mov ecx, [0xb7431c] -> thiscall, ret 8); a lista é destruída (0x68d040 +
///    0x944c94) e [+0x1128] fica 0;
/// 3. dias 5 e 6, se ontem é do mesmo ano: nas listas de ontem (as 3 fatias, as nações-chave até [0xb64738]), de trás
///    para a frente, os amistosos ([0x9cf684], tipo +0x3a = 1) de dois clubes sem técnico humano saem da lista (0x68d1d0 +
///    0x68d280); o primeiro jogo que não é amistoso desse tipo encerra a lista;
/// 4. a lista das conferências das pernas ([0xae2a48]): cada entrada cuja copa (ou fase +4 dela) tem +0x43 = 1 rotula
///    os jogos seguintes da rodada (0x522c70: push +0xb; push +7; push rodada; ecx = a copa -> thiscall, ret 0xc); a
///    lista é esvaziada (0x68d080).
unsafe extern "thiscall" fn calendario_dia(this: usize) {
    let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let reservas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7e6380usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let anterior: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1d0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d280usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let vaga: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x785830usize);
    let avisar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let rotular: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(0x522c70usize);
    let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (pa, pb, pc) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, c.as_mut_ptr() as usize);
    let dia = semana(0xae2c90) as i16;
    if dia == 2 || dia == 3 {
        mais(0xae2c90, pa, 3);
        reservas(cqd6_d(0xb74434), pa);
        if dia == 3 {
            mais(0xae2c90, pb, 1);
            crate::comparacao::ramo("cqd_6 reservas de amanha");
            reservas(cqd6_d(0xb74434), pb); // cqd6: as reservas de amanhã
        }
    }
    let l = cqd6_d(this + 0x1128);
    if l != 0 {
        cqd6_pd(l + 0xa, cqd6_d(l + 2));
        let mut n = proximo(l);
        while n != 0 {
            let ca = cqd6_clube(vaga(n, 0) as usize);
            let cb = cqd6_clube(vaga(n, 1) as usize);
            if vaga(n, 0xe) != 0 {
                avisar(cqd6_d(0xb7431c), n, ca);
            }
            if vaga(n, 0xf) != 0 {
                avisar(cqd6_d(0xb7431c), n, cb);
            }
            n = proximo(cqd6_d(this + 0x1128));
        }
        let l = cqd6_d(this + 0x1128);
        if l != 0 {
            destruir(l);
            liberar(l);
        }
        cqd6_pd(this + 0x1128, 0);
    }
    if dia == 5 || dia == 6 {
        let t = menos(0xae2c90, pa, 1);
        if cqd6_w(t + 2) == cqd6_w(0xae2c92) {
            let ontem = cqd6_w(menos(0xae2c90, pc, 1)) as u16 as usize;
            for fatia in 0..3usize {
                let listas = cqd6_d(this + (ontem * 3 + fatia) * 4);
                let mut k: i8 = 0;
                while k < cqd6_b(0xb64738) as i8 {
                    let lista = cqd6_d(listas + k as isize as usize * 4);
                    let mut no = cqd6_d(lista + 6);
                    cqd6_pd(lista + 0xa, no);
                    while no != 0 {
                        let f = cqd6_d(no);
                        if cqd6_d(f) != cqd6_d(0x9cf684) || cqd6_b(f + 0x3a) != 1 {
                            break;
                        }
                        if !cqd6_humano(cqd6_d(f + 0x1c)) && !cqd6_humano(cqd6_d(f + 0x20)) {
                            anterior(lista);
                            crate::comparacao::ramo("cqd_6 amistoso descartado");
                            tirar(lista, no); // cqd6: o amistoso de ontem sai da lista
                        } else {
                            anterior(lista);
                        }
                        no = cqd6_d(lista + 0xa);
                    }
                    k = k.wrapping_add(1);
                }
            }
        }
    }
    if cqd6_w(0xae2a48) != 0 {
        cqd6_pd(0xae2a52, cqd6_d(0xae2a4a));
        let mut e = proximo(0xae2a48);
        while e != 0 {
            let i = cqd6_d(e) as i32;
            let tab = cqd6_d(0xadadfc);
            if i >= 0 && tab != 0 {
                let mut cp = cqd6_d(tab + i as usize * 4);
                if cp != 0 {
                    let fase = cqd6_b(e + 4) as i8;
                    if fase as i32 <= cqd6_d(cp + 0x30) as i32 {
                        if fase != -1 {
                            cp = cqd6_d(cqd6_d(cp + 0xc) + fase as isize as usize * 4);
                        }
                        if cqd6_b(cp + 0x43) == 1 {
                            // ponytail: a original empurra eax com a palavra da rodada e os bytes altos do ponteiro da
                            // entrada; a 0x522c70 só lê a palavra
                            rotular(cp, cqd6_w(e + 5) as u16 as u32, cqd6_d(e + 7) as u32, cqd6_d(e + 0xb) as u32);
                        }
                    }
                }
            }
            e = proximo(0xae2a48);
        }
        esvaziar(0xae2a48);
    }
}

/// O comparador do dia do calendário: só a original (ponytail: as notícias vão aos técnicos, os nós dos amistosos e
/// das filas são liberados e os jogos de reservas entram no calendário: não roda duas vezes). Confere o que a rotina
/// garante: a fila das notícias some ([+0x1128] = 0 se havia) e a lista das conferências fica vazia.
unsafe extern "thiscall" fn comparar_calendario_dia(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let havia = cqd6_d(this + 0x1128);
    let antes = cqd6_w(0xae2a48);
    o(this);
    let fila = cqd6_d(this + 0x1128);
    let depois = cqd6_w(0xae2a48);
    PLACAR.registrar((havia == 0 || fila == 0) && (antes == 0 || depois == 0),
        || format!("calendario_dia: fila das notícias {:x} -> {:x}, conferências {} -> {}", havia, fila, antes, depois));
}

/// { 0x5abf10 } (thiscall, ret 0x10, fix_man.cpp, quadro SEH sem destrutor no caminho normal):
/// `FIXTURE_MANAGER::get_fixture_list(ano, dia, fatia, tipo)`: a lista dos jogos do ano, dia (palavra), fatia (byte) e
/// nação-chave (byte com sinal) na corrente dos blocos dos anos. Pelo bloco do ano (+0x1130): se o outro (+0x112c) não
/// é do ano, ele é trocado para o ano (0x5aa2d0: push ano; ecx = o outro -> thiscall, ret 4); sem o outro, um bloco
/// novo (0x944cf1, 0x1184 bytes; 0x5a9cf0: push 0; push this; push ano; ecx = o novo -> thiscall, ret 0xc) entra em
/// +0x112c; sem memória, a caixa da linha 0x681 e 0. A lista: [[bloco + (dia·3 + fatia)·4] + tipo·4].
unsafe extern "thiscall" fn calendario_lista(this: usize, ano: u32, dia: u32, fatia: u32, tipo: u32) -> usize {
    let trocar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5aa2d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(0x5a9cf0usize);
    let mut b = this;
    while cqd6_w(b + 0x1130) != ano as i16 {
        let o = cqd6_d(b + 0x112c);
        if o == 0 {
            let m = novo(0x1184);
            let r = if m == 0 { 0 } else { construir(m, ano, b, 0) };
            cqd6_pd(b + 0x112c, r);
            if r == 0 {
                aviso_em(0x9cc208, 0x681);
                return 0;
            }
        } else if cqd6_w(o + 0x1130) != ano as i16 {
            trocar(o, ano);
        }
        b = cqd6_d(b + 0x112c);
    }
    let i = (dia & 0xffff) as usize * 3 + (fatia & 0xff) as usize;
    cqd6_d(cqd6_d(b + i * 4) + (tipo as u8 as i8) as isize as usize * 4)
}

/// O comparador da lista do dia: com o ano no bloco ou no outro bloco, as duas (só leem) e a mesma lista; senão só a
/// original (ponytail: troca o bloco do outro ano, que grava e lê o arquivo, ou aloca um bloco novo).
unsafe extern "thiscall" fn comparar_calendario_lista(this: usize, ano: u32, dia: u32, fatia: u32, tipo: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize>(%%I1%%);
    let outro = cqd6_d(this + 0x112c);
    let direto = cqd6_w(this + 0x1130) == ano as i16 || (outro != 0 && cqd6_w(outro + 0x1130) == ano as i16);
    if !direto {
        return o(this, ano, dia, fatia, tipo);
    }
    let r = o(this, ano, dia, fatia, tipo);
    let n = calendario_lista(this, ano, dia, fatia, tipo);
    PLACAR.registrar(r == n, || format!("calendario_lista(ano {} dia {} fatia {} tipo {}): original={:x} rust={:x}",
        ano as u16, dia as u16, fatia as u8, tipo as u8 as i8, r, n));
    r
}

/// { 0x5ac3b0 } (thiscall, ret 0, fix_man.cpp): `FIXTURE_MANAGER::load_fixtures()`: o nome `fixtures_<ano>.tmp` em
/// +0x1133 (0x944c9f) e, com ano (+0x1130, sem sinal) desde 1980 e o arquivo em CM3_TEMP (0x90bde0: push nome; push
/// "CM3_TEMP"; add esp, 8 -> cdecl; 0x945b00 fopen com "rb"), a conta (dword) e os jogos de 0x4f bytes (0x945bfe
/// fread; falhando, a caixa da linha 0x730 ou 0x73e e o arquivo fechado): os ids dos clubes (+0xc, +0x10 → +0x1c,
/// +0x20), da competição (+0 → +0x14, 0x6b bytes em [0xae23d0]) e do estádio (+4 → +0x18, 0x4e bytes em [0xae23b0], 0
/// fora de [0xae2358]) viram ponteiros (id -1: o jogo para, linhas 0x74b, 0x751, 0x758), e o jogo entra (0x68d0d0: push
/// 0x4f; push &jogo; ecx = a lista -> thiscall, ret 8; falhando, o jogo para, 0x785) na lista do dia (+0x2a), fatia
/// (+0x3f) e nação-chave do jogo (0x48dd70: push &jogo; push competição; add esp, 8 -> cdecl). No fim, fclose
/// (0x945a09).
unsafe extern "thiscall" fn calendario_carregar_ano(this: usize) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let nacao: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x48dd70usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    let nome = this + 0x1133;
    let ano = cqd6_w(this + 0x1130) as u16;
    formatar(nome, 0x9cc238, ano as u32);
    if ano < 0x7bc {
        return;
    }
    let f = abrir(caminho(0x987da0, nome), 0x98f928);
    if f == 0 {
        return;
    }
    let mut conta = 0i32;
    if ler(&mut conta as *mut i32 as usize, 4, 1, f) != 1 {
        aviso_em(0x9cc208, 0x730);
        fechar(f);
        return;
    }
    let mut jogo = [0u8; 0x4f];
    let j = jogo.as_mut_ptr() as usize;
    let mut i = 0i32;
    while i < conta {
        if ler(j, 0x4f, 1, f) != 1 {
            aviso_em(0x9cc208, 0x73e);
            fechar(f);
            return;
        }
        let a = cqd6_d(j + 0xc);
        if a as u32 == u32::MAX {
            erro_em(0x9cc208, 0x74b);
        }
        cqd6_pd(j + 0x1c, cqd6_clube(a));
        let b = cqd6_d(j + 0x10);
        if b as u32 == u32::MAX {
            erro_em(0x9cc208, 0x751);
        }
        cqd6_pd(j + 0x20, cqd6_clube(b));
        let c = cqd6_d(j);
        if c as u32 == u32::MAX {
            erro_em(0x9cc208, 0x758);
        }
        let comp = cqd6_d(0xae23d0).wrapping_add(c.wrapping_mul(0x6b));
        cqd6_pd(j + 0x14, comp);
        let e = cqd6_d(j + 4) as i32;
        let estadio = if e != -1 && e < cqd6_d(0xae2358) as i32 {
            cqd6_d(0xae23b0).wrapping_add((e as usize).wrapping_mul(0x4e))
        } else {
            0
        };
        cqd6_pd(j + 0x18, estadio);
        let k = nacao(comp, j) as i8;
        let dia = cqd6_w(j + 0x2a) as u16 as usize; // cqd6: o dia do jogo carregado
        let lista = cqd6_d(cqd6_d(this + (dia * 3 + cqd6_b(j + 0x3f) as usize) * 4) + k as isize as usize * 4);
        crate::comparacao::ramo("cqd_6 jogo carregado");
        if acrescentar(lista, j, 0x4f) == 0 {
            erro_em(0x9cc208, 0x785);
        }
        i += 1;
    }
    fechar(f);
}

/// O comparador da carga dos jogos do ano: só a original (ponytail: lê o arquivo e aloca os nós das listas; não roda
/// duas vezes). Confere o nome gravado em +0x1133.
unsafe extern "thiscall" fn comparar_calendario_carregar_ano(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this);
    let esperado = format!("fixtures_{}.tmp", cqd6_w(this + 0x1130) as u16);
    let s = core::slice::from_raw_parts((this + 0x1133) as *const u8, esperado.len() + 1);
    PLACAR.registrar(&s[..esperado.len()] == esperado.as_bytes() && s[esperado.len()] == 0,
        || format!("calendario_carregar_ano: nome {:?} (esperado {})", s, esperado));
}
