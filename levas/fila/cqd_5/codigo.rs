// ---------- %%LEVA%%a leva: a liga: o dia da liga, a criação dos jogos da temporada e a promoção e o rebaixamento (league.cpp) ----------

unsafe fn cqd5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqd5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqd5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqd5_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqd5_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// O método `k` (deslocamento na tabela virtual) do objeto `o`.
unsafe fn vtable_method(o: usize, k: usize) -> usize {
    cqd5_d(cqd5_d(o) + k)
}

/// As duas datas (dia, ano) são a mesma (as palavras, como a original compara).
unsafe fn same_date(a: usize, b: usize) -> bool {
    cqd5_w(a + 2) == cqd5_w(b + 2) && cqd5_w(a) == cqd5_w(b)
}

/// { 0x6847c0 } (thiscall, ret 8, league.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `League::update_for_date(data, completo)`, o dia de cada liga (pelas competições, 46522 chamadas até 2002-06-01).
/// Liga parada (+0xb5) ou sem clubes (+0x45 = 0): 0. Senão (devolve 1, no byte):
/// - +0xa7 = -1 com times (+0x3e) > 0: +0xa7 = (times − 1) · turnos (+0x3c), na palavra;
/// - na primeira fase (+0x44 = 0xff) com tabela (+0xb1, +0x11 = 0), completo e a data nova (≠ +0xd1): no dia seguinte ao
///   do dia da Copa da nação-chave (0x669470; [0xb63d75 + 72·k] + 1, 0 depois de 364) e no ano da liga (+0x40), a
///   notícia da tabela (0x683ee0: push 0; mov ecx, this -> thiscall, ret 4);
/// - na primeira fase, o dia de cada fase (+0xc, até +0x30; método 0x14 com a data e o completo);
/// - na data +0xc9 (sem +0xdd): +0xad = 0 e o método 0x94;
/// - na primeira fase com tabela e a data nova: o método 0x10 (0, 0); fim da temporada (+0xdd = 1) com todas as fases
///   acabadas (método 0x10 com (0, 1) ≠ 0): a fase seguinte (método 0x28) se +0x30 < +0x2c − 1, senão (uma vez, +0x4c)
///   o histórico (0x442e80: push liga; mov ecx, [0xad9da8] -> thiscall, ret 4), os recordes com 1 (0x447430: push 1;
///   push liga; mov ecx, [0xad9dac] -> thiscall, ret 8) se a competição conta (0x4b0700: push id; add esp, 4 -> cdecl),
///   a caixa da linha 0x74c se +0xe6, o método 0x30 e o fim do anexo +8 (0x4a19f0: mov ecx, [+8] -> thiscall, ret 0);
///   depois os recordes com 0 e o anexo (0x4a2450: mov ecx, [+8] -> thiscall, ret 0);
/// - com tabela e completo, sem +0xdd: os jogos para a TV quatro semanas adiante (0x54c770 + 0x685640: push &data; mov
///   ecx, this -> thiscall, ret 4);
/// - completo: na primeira fase com inscrições (+0x52 bit 2), a data nova, sem +0xdd e tabela em andamento: se a primeira
///   data da tabela da TV (+0xba; dia, ano − o da liga) cai daqui a 5 dias (Argentina [0x9cf218]), 20 (Itália
///   [0x9cf360]) ou 14 dias, o prazo das inscrições (0x859e20: push 0; push 0; push liga; mov ecx, [0xdd7ef0] ->
///   thiscall, ret 0xc); e +0xd1 fica com a data (0x417c10).
unsafe extern "thiscall" fn league_update_for_date(this: usize, data: usize, completo: u32) -> u32 {
    let semanas: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c770usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let chave: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x669470usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x683ee0usize);
    let historico: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x442e80usize);
    let conta: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x4b0700usize);
    let recordes: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x447430usize);
    let fim_anexo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4a19f0usize);
    let anexo: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x4a2450usize);
    let tv: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x685640usize);
    let prazo: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x859e20usize);
    // a data pedida copiada no começo (a original guarda o dword do dia|ano e o do bissexto)
    let d0 = [cqd5_d(data) as u32, cqd5_d(data + 4) as u32];
    let pd0 = d0.as_ptr() as usize;
    let (mut t, mut x, mut w) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (pt, px, pw) = (t.as_mut_ptr() as usize, x.as_mut_ptr() as usize, w.as_mut_ptr() as usize);
    if cqd5_b(this + 0xb5) != 0 || cqd5_d(this + 0x45) == 0 {
        return 0;
    }
    if cqd5_w(this + 0xa7) == -1 {
        let n = cqd5_w(this + 0x3e);
        if n > 0 {
            cqd5_pw(this + 0xa7, n.wrapping_sub(1).wrapping_mul(cqd5_w(this + 0x3c)) as u16);
        }
    }
    let tab = cqd5_d(this + 0xb1);
    if tab != 0 && cqd5_b(tab + 0x11) == 0 && cqd5_b(this + 0x44) == 0xff && completo != 0 && !same_date(this + 0xd1, data) {
        let c = cqd5_d(this + 4);
        if cqd5_d(c + 0x5d) != 0 {
            let k = chave(c) as i8 as isize;
            let mut dia = (cqd5_w((0xb63d75 + k * 72) as usize) as u16).wrapping_add(1);
            if dia > 0x16c {
                dia = 0;
            }
            if dia as i32 == cqd5_w(0xae2c90) as i32 && cqd5_w(this + 0x40) as i32 == cqd5_w(0xae2c92) as u16 as i32 {
                noticia(this, 0);
            }
        }
    }
    if cqd5_b(this + 0x44) == 0xff {
        let mut i = 0i32;
        while i <= cqd5_d(this + 0x30) as i32 {
            let fase = cqd5_d(cqd5_d(this + 0xc) + i as usize * 4);
            let f: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(vtable_method(fase, 0x14));
            f(fase, data, completo);
            i += 1;
        }
    }
    if cqd5_w(this + 0xc9 + 2) == cqd5_w(0xae2c92) && cqd5_w(this + 0xc9) == cqd5_w(0xae2c90) && cqd5_d(this + 0xdd) == 0 {
        cqd5_pd(this + 0xad, 0);
        let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(vtable_method(this, 0x94));
        f(this);
    }
    let mut seguir_tv = true;
    if cqd5_b(this + 0x44) == 0xff {
        if cqd5_d(this + 0xb1) == 0 {
            seguir_tv = false;
        } else if !same_date(this + 0xd1, data) {
            let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(vtable_method(this, 0x10));
            f(this, 0, 0);
            if cqd5_d(this + 0xdd) == 1 {
                let mut acabadas = true;
                let mut i = 0i32;
                while i <= cqd5_d(this + 0x30) as i32 {
                    let fase = cqd5_d(cqd5_d(this + 0xc) + i as usize * 4);
                    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(vtable_method(fase, 0x10));
                    if g(fase, 0, 1) as u8 == 0 {
                        acabadas = false;
                        break;
                    }
                    i += 1;
                }
                if acabadas {
                    if (cqd5_d(this + 0x30) as i32) < (cqd5_d(this + 0x2c) as i32).wrapping_sub(1) {
                        let h: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(vtable_method(this, 0x28));
                        h(this);
                    } else if cqd5_d(this + 0x4c) == 0 {
                        historico(cqd5_d(0xad9da8), this);
                        if conta(cqd5_d(cqd5_d(this + 4)) as u32) != 0 {
                            recordes(cqd5_d(0xad9dac), this, 1);
                        }
                        if cqd5_d(this + 0xe6) != 0 {
                            show_error_at(0x9f15f4, 0x74c);
                        }
                        let h: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(vtable_method(this, 0x30));
                        h(this);
                        if cqd5_d(this + 8) != 0 {
                            fim_anexo(cqd5_d(this + 8));
                        }
                        cqd5_pd(this + 0x4c, 1);
                    }
                }
            }
            if conta(cqd5_d(cqd5_d(this + 4)) as u32) != 0 {
                recordes(cqd5_d(0xad9dac), this, 0);
            }
            if cqd5_d(this + 8) != 0 {
                anexo(cqd5_d(this + 8));
            }
        }
    }
    if seguir_tv && cqd5_d(this + 0xb1) != 0 {
        if completo == 0 {
            return 1;
        }
        if cqd5_d(this + 0xdd) == 0 {
            semanas(pd0, px, 4); // cqd5: a TV quatro semanas adiante
            crate::comparison::branch("cqd_5 matches for tv");
            tv(this, px);
        }
    }
    if completo == 0 {
        return 1;
    }
    if cqd5_b(this + 0x52) & 2 != 0 && cqd5_b(this + 0x44) == 0xff && !same_date(this + 0xd1, data)
        && cqd5_d(this + 0xdd) == 0 && cqd5_b(cqd5_d(this + 0xb1) + 0x11) == 0 {
        let e = cqd5_d(this + 0xba);
        // ponytail: a original empurra o dia e o ano em registradores com sobra nos bytes altos; a 0x549ef0 só lê as palavras
        nova(pt, cqd5_w(e) as u16 as u32, cqd5_w(e + 2).wrapping_add(cqd5_w(this + 0x40)) as u16 as u32);
        let c = cqd5_d(this + 4);
        let nacao = if c != 0 && cqd5_d(c + 0x5d) != 0 { cqd5_d(cqd5_d(c + 0x5d)) } else { 0 };
        let tem = c != 0 && cqd5_d(c + 0x5d) != 0;
        let dias = if tem && nacao == cqd5_d(0x9cf218) {
            5
        } else if tem && nacao == cqd5_d(0x9cf360) {
            0x14
        } else {
            0xe
        };
        let r = mais(pd0, pw, dias);
        if same_date(pt, r) {
            prazo(cqd5_d(0xdd7ef0), this, 0, 0);
        }
    }
    copia(this + 0xd1, pw, data);
    1
}

/// O comparador do dia da liga: só a original (ponytail: chama os métodos de cada fase e da liga, as notícias, os
/// recordes e os jogos para a TV, que mexem no calendário: não roda duas vezes). Confere o que a rotina garante: 0 só
/// com a liga parada ou sem clubes e, completo, +0xd1 com a data.
unsafe extern "thiscall" fn compare_league_update_for_date(this: usize, data: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    let parada = cqd5_b(this + 0xb5) != 0 || cqd5_d(this + 0x45) == 0;
    let r = o(this, data, completo);
    let ok = if parada { r as u8 == 0 } else { r as u8 == 1 && (completo == 0 || same_date(this + 0xd1, data)) };
    SCOREBOARD.record(ok, || format!("league_update_for_date(completo {}): parada {} devolveu {} +0xd1 {:x} data {:x}",
        completo, parada, r as u8, cqd5_d(this + 0xd1), cqd5_d(data)));
    r
}

/// Um jogo da tabela da liga no registro `r` (0x4f bytes) e no calendário: os clubes das entradas `ea` (mandante) e
/// `eb` (0x3b bytes na tabela +0xb1: +0 clube, +0x38 marca), as marcas +0x40/+0x41 (marca 0), os ids +0xc/+0x10 (-1 sem
/// clube), o estádio (0x85c260: push +0x24; push &jogo; push [+0x3d da data]; mov ecx, [0xdd7ef4] -> thiscall, ret
/// 0xc), a data (+0x28 ano + o da liga, +0x2a dia) e a fatia (+0x3f) da entrada `off` da tabela das datas (+0xba), os
/// bits 0..1 de +0x4e zerados e o bit 3 na última rodada (+0x34 = +0xa9 − 1; ou com `tipo4`, data do tipo 4), e o jogo
/// vai ao calendário (0x5aa680: push 1; push &jogo; mov ecx, [0xae2a58] -> thiscall, ret 8).
unsafe fn add_league_fixture(this: usize, r: usize, ea: usize, eb: usize, off: usize, tipo4: bool) {
    let estadio: unsafe extern "thiscall" fn(usize, u32, usize, u32) = core::mem::transmute(0x85c260usize);
    let calendario: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
    let (ea, eb) = (ea, eb); // cqd5: o mandante e o visitante
    let (ca, cb) = (cqd5_d(ea), cqd5_d(eb));
    cqd5_pd(r + 0x1c, ca);
    cqd5_pd(r + 0x20, cb);
    cqd5_pb(r + 0x40, (cqd5_b(ea + 0x38) == 0) as u8);
    cqd5_pb(r + 0x41, (cqd5_b(eb + 0x38) == 0) as u8);
    cqd5_pd(r + 0xc, if ca != 0 { cqd5_d(ca) } else { u32::MAX as usize });
    cqd5_pd(r + 0x10, if cb != 0 { cqd5_d(cb) } else { u32::MAX as usize });
    estadio(cqd5_d(0xdd7ef4), cqd5_d(cqd5_d(this + 0xba) + off + 0x3d) as u32, r, cqd5_d(this + 0x24) as u32);
    let e = cqd5_d(this + 0xba) + off;
    cqd5_pw(r + 0x28, cqd5_w(e + 2).wrapping_add(cqd5_w(this + 0x40)) as u16);
    cqd5_pw(r + 0x2a, cqd5_w(e) as u16);
    cqd5_pb(r + 0x3f, cqd5_b(e + 4));
    cqd5_pw(r + 0x4d, cqd5_w(r + 0x4d) as u16 & 0xfcff);
    if cqd5_w(r + 0x34) as i32 == cqd5_w(this + 0xa9) as i32 - 1 || (tipo4 && cqd5_b(e + 0xb) == 4) {
        cqd5_pb(r + 0x4e, cqd5_b(r + 0x4e) | 8);
    } else {
        cqd5_pw(r + 0x4d, cqd5_w(r + 0x4d) as u16 & 0xf7ff);
    }
    crate::comparison::branch("cqd_5 fixture list match");
    calendario(cqd5_d(0xae2a58), r, 1);
}

/// { 0x6835c0 } (thiscall, ret 0, league.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
/// `League::create_fixtures()`, da montagem das competições (44 chamadas até 2002-06-01). Com n = times (+0x3e) +
/// times % 2: +0xdd = 0; a grade de n linhas de n dwords (0x944cf1; sem memória, a caixa da linha 0x3fc ou 0x405 e 0),
/// montada pela 0x6844b0 (push n; push grade -> stdcall, ret 8: o mov ecx, this antes ela não lê) e, fora das
/// competições [0x9cf79c] e [0x9cf7a4], a ordem dos clubes sorteada (0x686a70: push n; mov ecx, this -> thiscall, ret
/// 4). O registro do jogo (0x4f bytes na pilha): +0 id da competição, +8 = -1, +0x14 a competição, +0x24/+0x26 a última
/// data da tabela (+0xba, entrada +0xa9 − 1), +0x2c ano (+0x40), +0x30 = 0, +0x32 (+0x3a), +0x36 (+0xdb), +0x38
/// (+0xab), +0x3a (+0xc4), +0x3b = 0xff, +0x3d/+0x3e = 0, +0x42 fase (+0x44), +0x43..+0x4c = 0xff, e o método 0x7c da
/// liga com ele. Com a lista própria (+0xea, entradas de 0x10 bytes: +8/+0xc os ids dos clubes) do ano da liga: em cada
/// rodada k (+0xa9) os times/2 jogos da lista (os índices pela tabela +0xb1), e a lista é liberada (0x9462bb +
/// 0x944c94). Senão o rodízio: para j = 1..turnos (+0x3c)·(n − 1), a rodada pela 0x68a030 (push +0xd9; push +0xa9;
/// push +0x3c; push +0x3e; push id; push &passo; push rodada; add esp, 0x1c -> cdecl) e a coluna j % (n − 1) (n − 1 no
/// zero): cada time i com adversário v ≠ 0 na grade apaga a vaga do adversário; o mando pela paridade de v > 0, do turno
/// ((j − 1)/(n − 1)) e do ano + fase; com a nação +0x24, o clube dessa nação (+0x69) fica como mandante. Cada jogo vai
/// ao calendário (add_league_fixture). No fim, a grade é liberada e as datas do tipo 3 da tabela levam os jogos para a TV
/// (0x685640). Devolve 1.
unsafe extern "thiscall" fn create_fixtures(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let montar: unsafe extern "stdcall" fn(usize, i32) -> u32 = core::mem::transmute(0x6844b0usize);
    let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x686a70usize);
    let girar: unsafe extern "C" fn(i32, *mut i8, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x68a030usize);
    let destruir_lista: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    let tv: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x685640usize);
    let mut rodada: i32 = -1;
    let times = cqd5_w(this + 0x3e) as i32;
    let n = times + times % 2;
    let mut step_towards: i8 = 0;
    let (mut x, mut y, mut z) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (px, py, pz) = (x.as_mut_ptr() as usize, y.as_mut_ptr() as usize, z.as_mut_ptr() as usize);
    padrao(px);
    cqd5_pd(this + 0xdd, 0);
    let grade = novo((n * 4 + 4) as u32);
    if grade == 0 {
        show_error_at(0x9f15f4, 0x3fc);
        return 0;
    }
    let mut i = 1i32;
    while i <= n {
        let l = novo((n * 4) as u32);
        if l == 0 {
            show_error_at(0x9f15f4, 0x405);
            liberar(grade);
            return 0;
        }
        cqd5_pd(grade + i as usize * 4, l);
        i += 1;
    }
    montar(grade, n);
    let c = cqd5_d(this + 4);
    if c == 0 || (cqd5_d(c) != cqd5_d(0x9cf79c) && cqd5_d(c) != cqd5_d(0x9cf7a4)) {
        crate::comparison::branch("cqd_5 order drawn");
        sortear(this, n as u32); // cqd5: a ordem dos clubes
    }
    // o registro do jogo; os bytes que a original não grava (+4..+7, +0x18, +0x2e, +0x39, +0x4d e os bits altos de
    // +0x4e: sobra da pilha dela, que o método 0x7c e a 0x85c260 podem preencher) ficam 0, um valor que o jogo produz
    let mut reg = [0u8; 0x60];
    let r = reg.as_mut_ptr() as usize;
    let c = cqd5_d(this + 4);
    cqd5_pd(r + 0x14, c);
    cqd5_pw(r + 0x32, cqd5_w(this + 0x3a) as u16);
    cqd5_pd(r, cqd5_d(c));
    cqd5_pw(r + 0x2c, cqd5_w(this + 0x40) as u16);
    cqd5_pw(r + 0x38, cqd5_w(this + 0xab) as u16);
    cqd5_pw(r + 0x4d, cqd5_w(r + 0x4d) as u16 & 0xfbff);
    cqd5_pb(r + 0x3a, cqd5_b(this + 0xc4));
    cqd5_pw(r + 0x36, cqd5_w(this + 0xdb) as u16);
    cqd5_pw(r + 0x30, 0);
    for k in 0x43..=0x4cusize {
        cqd5_pb(r + k, 0xff);
    }
    cqd5_pd(r + 8, u32::MAX as usize);
    cqd5_pb(r + 0x42, cqd5_b(this + 0x44));
    cqd5_pb(r + 0x3d, 0);
    cqd5_pb(r + 0x3e, 0);
    cqd5_pb(r + 0x3b, 0xff);
    let preparar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(vtable_method(this, 0x7c));
    preparar(this, r);
    let ult = cqd5_d(this + 0xba).wrapping_add((cqd5_w(this + 0xa9) as i32 * 0x41) as usize);
    // ponytail: a original empurra o dia e o ano com sobra nos bytes altos; a 0x549ef0 só lê as palavras
    let yy = nova(py, cqd5_w(ult - 0x41) as u16 as u32, cqd5_w(ult - 0x3f).wrapping_add(cqd5_w(this + 0x40)) as u16 as u32);
    copia(px, pz, yy);
    cqd5_pw(r + 0x26, cqd5_w(px) as u16);
    cqd5_pw(r + 0x24, cqd5_w(px + 2) as u16);
    let lista = cqd5_d(this + 0xea);
    if lista != 0 && cqd5_w(this + 0x40) as i32 == cqd5_w(lista + 2) as u16 as i32 {
        let mut k = 0i32;
        let mut off = 0usize;
        while k < cqd5_w(this + 0xa9) as i32 {
            let mut i = 0i32;
            loop {
                let times = cqd5_w(this + 0x3e) as i32;
                let metade = times / 2;
                if i >= metade {
                    break;
                }
                cqd5_pw(r + 0x34, k as u16);
                let e = cqd5_d(this + 0xea) + ((metade * k + i) as usize) * 0x10;
                let (mut ia, mut ib) = (-1i32, -1i32);
                if times > 0 {
                    let (a, b) = (cqd5_d(e + 8), cqd5_d(e + 0xc));
                    let tb = cqd5_d(this + 0xb1);
                    for p in 0..times {
                        let id = cqd5_d(cqd5_d(tb + p as usize * 0x3b));
                        if a == id {
                            ia = p;
                        }
                        if b == id {
                            ib = p;
                        }
                    }
                }
                let tb = cqd5_d(this + 0xb1);
                add_league_fixture(this, r, tb.wrapping_add((ia as isize * 0x3b) as usize), tb.wrapping_add((ib as isize * 0x3b) as usize), off, true);
                i += 1;
            }
            k += 1;
            off += 0x41;
        }
        let lista = cqd5_d(this + 0xea);
        if lista != 0 {
            destruir_lista(lista, 0x10, cqd5_d(lista - 4) as u32, 0x5a53c0);
            liberar(lista - 4);
        }
        cqd5_pd(this + 0xea, 0);
    } else {
        let mut j = 1i32;
        while j <= cqd5_w(this + 0x3c) as i32 * (n - 1) {
            // ponytail: a original empurra as palavras +0x3e, +0x3c, +0xa9 e +0xd9 com sobra nos bytes altos; a 0x68a030
            // só lê as palavras e o byte
            rodada = girar(rodada, &mut step_towards, cqd5_d(cqd5_d(this + 4)) as u32, cqd5_w(this + 0x3e) as u16 as u32,
                cqd5_w(this + 0x3c) as u16 as u32, cqd5_w(this + 0xa9) as u16 as u32, cqd5_w(this + 0xd9) as u16 as u32);
            let mut col = j % (n - 1);
            if col == 0 {
                col = n - 1;
            }
            cqd5_pw(r + 0x34, rodada as u16);
            if cqd5_w(this + 0x3e) >= 1 {
                let mut i = 0i32;
                loop {
                    let v = cqd5_d(cqd5_d(grade + 4 + i as usize * 4) + col as usize * 4) as i32;
                    if v != 0 {
                        let turno = ((j - 1) / (n - 1)) % 2 != 0;
                        let paridade = (cqd5_w(this + 0x40) as i32 + cqd5_b(this + 0x44) as i8 as i32) % 2 != 0;
                        let idx = if v > 0 { v - 1 } else { -1 - v };
                        let linha = cqd5_d(grade + 4 + (idx as isize * 4) as usize);
                        let (a, b) = if (v > 0) ^ turno ^ paridade { (idx, i) } else { (i, idx) };
                        cqd5_pd(linha + col as usize * 4, 0);
                        let ult = cqd5_w(this + 0x3e) as i32 - 1;
                        if a >= 0 && a <= ult && b >= 0 && b <= ult {
                            let tb = cqd5_d(this + 0xb1);
                            let (ea, eb) = (tb + a as usize * 0x3b, tb + b as usize * 0x3b);
                            let sel = cqd5_d(this + 0x24) as u32;
                            let (m, f) = if sel != u32::MAX {
                                let nac = cqd5_d(cqd5_d(eb) + 0x69);
                                if nac != 0 && cqd5_d(nac) as u32 == sel { (eb, ea) } else { (ea, eb) }
                            } else {
                                (ea, eb)
                            };
                            add_league_fixture(this, r, m, f, (rodada as isize * 0x41) as usize, false);
                        }
                    }
                    i += 1;
                    if i + 1 > cqd5_w(this + 0x3e) as i32 {
                        break;
                    }
                }
            }
            j += 1;
        }
    }
    let mut i = 0i32;
    while i < n {
        liberar(cqd5_d(grade + 4 + i as usize * 4));
        i += 1;
    }
    liberar(grade);
    let mut k = 0i32;
    while k < cqd5_w(this + 0xa9) as i32 {
        let e = cqd5_d(this + 0xba) + k as usize * 0x41;
        if cqd5_b(e + 0xb) == 3 {
            nova(py, cqd5_w(e) as u16 as u32, cqd5_w(e + 2).wrapping_add(cqd5_w(this + 0x40)) as u16 as u32);
            tv(this, py);
        }
        k += 1;
    }
    1
}

/// O comparador da criação dos jogos: só a original (ponytail: aloca a grade, sorteia a ordem dos clubes e põe os
/// jogos no calendário, que aloca: não roda duas vezes). Confere o que a rotina garante: com a lista própria do ano, ela
/// é liberada (+0xea = 0) quando devolve 1.
unsafe extern "thiscall" fn compare_create_fixtures(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let lista = cqd5_d(this + 0xea);
    let propria = lista != 0 && cqd5_w(this + 0x40) as i32 == cqd5_w(lista + 2) as u16 as i32;
    let r = o(this);
    let ok = !(propria && r == 1) || cqd5_d(this + 0xea) == 0;
    SCOREBOARD.record(ok, || format!("create_fixtures: lista própria {} devolveu {} +0xea {:x}", propria, r, cqd5_d(this + 0xea)));
    r
}

/// { 0x689c80 } (ret 0x18, league.cpp): `league_promote_and_relegate(acima, abaixo, contratos, aos_pares, sobem,
/// descem)`, do fim das ligas (depois de 2002-06-01). Recebe o ecx de quem chama e o repassa à 0x6830b0, que não o lê:
/// thiscall aqui para o guardar. Com `contratos`, os clubes da liga de baixo que não sobem (+0x37 ≠ 0 e ≠ 5) acionam
/// as cláusulas de não promoção (0x4cced0: push liga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8). Aos pares:
/// cada rebaixado de cima (+0x37 = 3) troca com o primeiro de baixo que sobe (+0x37 = 0 ou 5): esse sobe (0x6830b0:
/// push 0; push competição de cima; push clube; ecx de quem chama -> thiscall, ret 0xc), o rebaixado guarda a
/// competição antiga (+0x5b) e fica com a de baixo (+0x57), aciona as cláusulas de rebaixamento (0x4cd030: push
/// antiga; push clube; mov ecx, [0xae1050] -> thiscall, ret 8) e, numa competição sem registro ativo e com técnico
/// humano (0x600a50), perde o técnico (0x69b790: push 0; push 7; push 1; push clube; mov ecx, [0xb67a44] -> thiscall,
/// ret 0x10); os dois ficam com +0x37 = 0xff. Senão, todos os rebaixados descem assim e depois todos os de baixo que
/// sobem sobem. Os contados que diferem dos esperados (0xff = qualquer) dão a caixa da linha 0x15dc (sobem) ou 0x15e2
/// (descem).
unsafe extern "thiscall" fn league_promote_and_relegate(ecx: usize, acima: usize, abaixo: usize, contratos: u32, aos_pares: u32, sobem: u32, descem: u32) {
    let clausulas: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cced0usize);
    let subir: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6830b0usize);
    let rebaixamento: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cd030usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let sem_tecnico: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x69b790usize);
    let (mut desceram, mut subiram) = (0u8, 0u8);
    if contratos != 0 {
        let mut i = 0i32;
        while i < cqd5_w(abaixo + 0x3e) as i32 {
            let e = cqd5_d(abaixo + 0xb1) + i as usize * 0x3b;
            let s = cqd5_b(e + 0x37);
            if s != 5 && s != 0 {
                clausulas(cqd5_d(0xae1050), cqd5_d(e), cqd5_d(abaixo + 4));
            }
            i += 1;
        }
    }
    // o rebaixado desce para a competição de baixo e perde o técnico humano numa competição sem registro ativo
    let descer = |clube: usize| {
        let comp = cqd5_d(abaixo + 4);
        let antiga = cqd5_d(clube + 0x57);
        cqd5_pd(clube + 0x5b, antiga);
        cqd5_pd(clube + 0x57, comp);
        rebaixamento(cqd5_d(0xae1050), clube, antiga);
        let tab = cqd5_d(0xadadfc);
        if tab == 0 || comp == 0 || cqd5_d(tab + cqd5_d(comp) * 4) == 0 {
            if humano(clube, 1, 1, 0, 0) != 0 {
                sem_tecnico(cqd5_d(0xb67a44), clube, 1, 7, 0);
            }
        }
    };
    if aos_pares != 0 {
        let mut i = 0i32;
        while i < cqd5_w(acima + 0x3e) as i32 {
            let ue = cqd5_d(acima + 0xb1) + i as usize * 0x3b;
            if cqd5_b(ue + 0x37) == 3 {
                let n = cqd5_w(abaixo + 0x3e) as i32;
                let mut j = 0i32;
                while j < n {
                    let s = cqd5_b(cqd5_d(abaixo + 0xb1) + j as usize * 0x3b + 0x37);
                    if s == 0 || s == 5 {
                        break;
                    }
                    j += 1;
                }
                if j < n {
                    let le = cqd5_d(abaixo + 0xb1) + j as usize * 0x3b;
                    subir(ecx, cqd5_d(le), cqd5_d(acima + 4), 0);
                    descer(cqd5_d(ue));
                    desceram = desceram.wrapping_add(1);
                    subiram = subiram.wrapping_add(1);
                    cqd5_pb(le + 0x37, 0xff);
                    cqd5_pb(ue + 0x37, 0xff);
                }
            }
            i += 1;
        }
    } else {
        let mut i = 0i32;
        while i < cqd5_w(acima + 0x3e) as i32 {
            let ue = cqd5_d(acima + 0xb1) + i as usize * 0x3b;
            if cqd5_b(ue + 0x37) == 3 {
                descer(cqd5_d(ue));
                desceram = desceram.wrapping_add(1);
                cqd5_pb(ue + 0x37, 0xff);
            }
            i += 1;
        }
        let mut i = 0i32;
        while i < cqd5_w(abaixo + 0x3e) as i32 {
            let le = cqd5_d(abaixo + 0xb1) + i as usize * 0x3b;
            let s = cqd5_b(le + 0x37);
            if s == 0 || s == 5 {
                subir(ecx, cqd5_d(le), cqd5_d(acima + 4), 0);
                subiram = subiram.wrapping_add(1);
                cqd5_pb(le + 0x37, 0xff);
            }
            i += 1;
        }
    }
    if sobem as u8 != 0xff && subiram != sobem as u8 {
        show_error_at(0x9f15f4, 0x15dc);
    }
    if descem as u8 != 0xff && desceram != descem as u8 {
        show_error_at(0x9f15f4, 0x15e2);
    }
}

/// O comparador da promoção e do rebaixamento: só a original (ponytail: os contratos, o técnico demitido e as notícias
/// não rodam duas vezes); a cópia leva o ramo.
unsafe extern "thiscall" fn compare_league_promote_and_relegate(ecx: usize, acima: usize, abaixo: usize, contratos: u32, aos_pares: u32, sobem: u32, descem: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(%%I2%%)(ecx, acima, abaixo, contratos, aos_pares, sobem, descem)
}
