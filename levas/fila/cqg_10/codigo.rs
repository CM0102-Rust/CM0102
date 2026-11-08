// ---------- %%LEVA%%a leva: o amistoso cancelado, o jogo da seleção que sai junto e o torneio cancelado (friendly.cpp) ----------

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
fn replace_low_word(d: usize, a: i16) -> u32 {
    ((d as u32) & 0xffff_0000) | a as u16 as u32
}

/// { 0x5c6570 } (thiscall, ret 8, friendly.cpp, quadro SEH da notícia local): `Friendly::on_friendly_fixture_cancelled(
/// jogo, clube_que_cancela)`, pelo calendário (0x5aa680) e pela tela do clube. Amistoso desta competição (+0x14 = +4):
/// a notícia 8 (0x77bbe0) com +0xc, +0x10, o dia e o ano, aos técnicos do mandante e do visitante que não cancelaram
/// (0x7861e0); de outra competição: o torneio dela entre os de +0xb1 (0x69 bytes; índice = byte da competição − +0xaf),
/// se marcado, é cancelado (0x5c8260 com 0). Amistoso desta competição com a lista de vagas (+0x34cc): a vaga do jogo
/// (0x5c6740) fica livre (+0x10 = 0, +0x18 = 0). Jogo sem rodada (+0x43 = 0xff): o de seleção vai pela 0x5cb9e0, o de
/// clube só sai do calendário (0x5abc30).
unsafe extern "thiscall" fn on_friendly_fixture_cancelled(this: usize, jogo: usize, quem: usize) {
    let comp = cqg10_d(jogo + 0x14);
    if comp == cqg10_d(this + 4) {
        crate::comparison::branch("cqg_10 noticia do amistoso cancelado");
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
            crate::comparison::branch("cqg_10 torneio cancelado");
            let cancelar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5c8260usize);
            // 0x5c8260: push 0; push torneio; mov ecx, this -> thiscall, ret 8
            cancelar(this, torneio, 0);
        }
    }
    if cqg10_d(jogo + 0x14) == cqg10_d(this + 4) && cqg10_d(this + 0x34cc) != 0 {
        let vaga_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5c6740usize);
        // 0x5c6740: push +0x1c; push +0x20; push (dx = +0x28); push (ax = +0x2a); mov ecx, this -> thiscall, ret 0x10;
        // os registradores levam nas palavras altas o que já tinham (o ponteiro +0x14 e o +0x1c)
        let v = vaga_de(this, replace_low_word(cqg10_d(jogo + 0x1c), cqg10_w(jogo + 0x2a)), replace_low_word(cqg10_d(jogo + 0x14), cqg10_w(jogo + 0x28)),
            cqg10_d(jogo + 0x20) as u32, cqg10_d(jogo + 0x1c) as u32);
        if v != 0 {
            crate::comparison::branch("cqg_10 vaga do amistoso livre");
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
            crate::comparison::branch("cqg_10 amistoso fora do calendario");
            tirar(cqg10_d(0xae2a58), jogo); // cqg10: o amistoso sai do calendário
        }
    }
}

/// Comparador da 0x5c6570: a original entrega notícias, cancela torneios e tira jogos do calendário (libera): só ela
/// roda. Antes, a cópia acha a vaga do amistoso (0x5c6740, só leitura); depois a vaga tem de estar livre (+0x10 = 0 e
/// +0x18 = 0).
unsafe extern "thiscall" fn comparar_on_friendly_fixture_cancelled(this: usize, jogo: usize, quem: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    let mut vaga = 0usize;
    if cqg10_d(jogo + 0x14) == cqg10_d(this + 4) && cqg10_d(this + 0x34cc) != 0 {
        let vaga_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5c6740usize);
        vaga = vaga_de(this, replace_low_word(cqg10_d(jogo + 0x1c), cqg10_w(jogo + 0x2a)), replace_low_word(cqg10_d(jogo + 0x14), cqg10_w(jogo + 0x28)),
            cqg10_d(jogo + 0x20) as u32, cqg10_d(jogo + 0x1c) as u32);
    }
    o(this, jogo, quem);
    let livre = vaga == 0 || (cqg10_d(vaga + 0x10) == 0 && cqg10_b(vaga + 0x18) == 0);
    SCOREBOARD.record(livre, || format!("on_friendly_fixture_cancelled(jogo {:#x}) a vaga {:#x} não ficou livre", jogo, vaga));
}

/// { 0x5cb9e0 } (thiscall, ret 4, friendly.cpp, quadro SEH da lista local e das datas): o amistoso de seleção cancelado
/// (0x5c6570). Pela marca do jogo (+0x4d & 0x300): 0x100 a seleção é o mandante (+0x1c), 0x200 a seleção principal da
/// nação dele (0x53b4f0); com id válido, o jogo dela na véspera (0x549ef0, 0x54c8e0 com 1, 0x417c10; 0x5acbe0 com a
/// véspera nas duas pontas) sai do calendário (0x5abc30). Depois o próprio jogo sai.
unsafe extern "thiscall" fn friendly_cancel_national_fixture(this: usize, jogo: usize) {
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
        crate::comparison::branch("cqg_10 jogo da selecao na vespera");
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
unsafe extern "thiscall" fn comparar_friendly_cancel_national_fixture(this: usize, jogo: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, jogo);
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
unsafe extern "thiscall" fn cancel_tournament(this: usize, torneio: usize, sem_o_dono: u32) {
    if torneio == 0 {
        show_error_at(0x9cd0a0, 0x10ad);
        return;
    }
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let manager_manager_remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    // 0x54c8e0 / 0x54c5b0: push n; push &saída; mov ecx, data -> thiscall, ret 8; 0x68d020 / 0x68d040 / 0x68d1b0: lea ecx
    // (ou mov ecx, lista) -> thiscall; 0x68d1f0: push item; mov ecx, [this+0x34d0] -> thiscall, ret 4
    let mut d7 = [0u32; 2];
    let semana = menos(0xae2c90, d7.as_mut_ptr() as usize, 7);
    let (dia7, ano7) = (cqg10_w(semana), cqg10_w(semana + 2) as u16);
    let inicio = torneio + 4;
    let (dia_i, ano_i) = (cqg10_w(inicio), cqg10_w(torneio + 6) as u16);
    if ano7 < ano_i || (ano7 == ano_i && dia7 < dia_i) {
        crate::comparison::branch("cqg_10 noticia do torneio cancelado");
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
                manager_manager_remove(cqg10_d(this + 0x34d0), item);
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
        crate::comparison::branch("cqg_10 competicao do torneio desfeita");
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
unsafe extern "thiscall" fn comparar_cancel_tournament(this: usize, torneio: usize, sem_o_dono: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%);
    if torneio == 0 {
        o(this, torneio, sem_o_dono);
        return;
    }
    let k = (cqg10_w(this + 0xaf) as i32 + cqg10_b(torneio + 0x64) as i8 as i32) as usize;
    let vaga = cqg10_d(0xadadfc).wrapping_add(k.wrapping_mul(4));
    o(this, torneio, sem_o_dono);
    let mut in_list = false;
    let p = cqg10_d(this + 0x34d0);
    if p != 0 {
        let mut no = cqg10_d(p + 2);
        let mut n = 0;
        while no != 0 && n < 0x10000 {
            if cqg10_d(cqg10_d(no)) == torneio {
                in_list = true;
            }
            no = cqg10_d(no + 6);
            n += 1;
        }
    }
    let ok = cqg10_d(torneio) == 0 && cqg10_d(torneio + 0x65) == 0 && cqg10_d(vaga) == 0 && !in_list;
    SCOREBOARD.record(ok, || format!("cancel_tournament(torneio {:#x}) dono {:#x} competição {:#x} vaga {:#x} na lista {}",
        torneio, cqg10_d(torneio), cqg10_d(torneio + 0x65), cqg10_d(vaga), in_list));
}
