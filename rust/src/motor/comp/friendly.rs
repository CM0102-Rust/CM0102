use super::*;




/// Returns dword d with its low 16 bits replaced by a (mimics the original's `mov ax, word` on a full register).
///
/// [pt] O dword `d` com a palavra baixa trocada pela de `a` (o `mov ax, word` da original sobre um registrador já cheio).
pub(crate) fn replace_low_word(d: usize, a: i16) -> u32 {
    ((d as u32) & 0xffff_0000) | a as u16 as u32
}

/// a friendly of this competition: news 8 to the managers of both clubs that did not cancel and the friendly slot
/// freed; of another competition, its tournament is cancelled; a fixture without a round leaves the calendar (the
/// national team one through 0x5cb9e0)
/// Original: `Friendly::on_friendly_fixture_cancelled` (0x5c6570, friendly.cpp, thiscall, ret 8).
///
/// [pt] { 0x5c6570 } (thiscall, ret 8, friendly.cpp, quadro SEH da notícia local): `Friendly::on_friendly_fixture_cancelled(
/// jogo, clube_que_cancela)`, pelo calendário (0x5aa680) e pela tela do clube. Amistoso desta competição (+0x14 = +4):
/// a notícia 8 (0x77bbe0) com +0xc, +0x10, o dia e o ano, aos técnicos do mandante e do visitante que não cancelaram
/// (0x7861e0); de outra competição: o torneio dela entre os de +0xb1 (0x69 bytes; índice = byte da competição − +0xaf),
/// se marcado, é cancelado (0x5c8260 com 0). Amistoso desta competição com a lista de vagas (+0x34cc): a vaga do jogo
/// (0x5c6740) fica livre (+0x10 = 0, +0x18 = 0). Jogo sem rodada (+0x43 = 0xff): o de seleção vai pela 0x5cb9e0, o de
/// clube só sai do calendário (0x5abc30).
pub(crate) unsafe extern "thiscall" fn on_friendly_fixture_cancelled(this: usize, jogo: usize, quem: usize) {
    let comp = read_usize(jogo + 0x14);
    if comp == read_usize(this + 4) {
        crate::comparacao::ramo("cqg_10 noticia do amistoso cancelado");
        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
        let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
        // 0x77bbe0: push 0; push 8; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall, ret 8; 0x7861e0:
        // push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8
        let mut noticia = [0u32; 0x38];
        let nt = noticia.as_mut_ptr() as usize;
        construir(nt, 8, 0);
        gravar(nt, 0, read_usize(jogo + 0xc) as u32);
        gravar(nt, 1, read_usize(jogo + 0x10) as u32);
        gravar(nt, 2, *((jogo + 0x2a) as *const u16) as u32);
        gravar(nt, 3, *((jogo + 0x28) as *const u16) as u32);
        let m = read_usize(jogo + 0x1c);
        if quem != m {
            entregar(read_usize(G_NEWS), nt, m);
        }
        let v = read_usize(jogo + 0x20);
        if quem != v {
            entregar(read_usize(G_NEWS), nt, v);
        }
    } else if this.wrapping_add(0xb1) != 0 {
        let k = read_u8(comp).wrapping_sub(read_u8(this + 0xaf)) as i8 as isize;
        let torneio = (this + 0xb1).wrapping_add((k * 0x69) as usize);
        if read_usize(torneio) != 0 && torneio != 0 {
            crate::comparacao::ramo("cqg_10 torneio cancelado");
            let cancelar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_FRIENDLY_CANCEL_TOURNAMENT);
            // 0x5c8260: push 0; push torneio; mov ecx, this -> thiscall, ret 8
            cancelar(this, torneio, 0);
        }
    }
    if read_usize(jogo + 0x14) == read_usize(this + 4) && read_usize(this + 0x34cc) != 0 {
        let vaga_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
        // 0x5c6740: push +0x1c; push +0x20; push (dx = +0x28); push (ax = +0x2a); mov ecx, this -> thiscall, ret 0x10;
        // os registradores levam nas palavras altas o que já tinham (o ponteiro +0x14 e o +0x1c)
        let v = vaga_de(this, replace_low_word(read_usize(jogo + 0x1c), read_i16(jogo + 0x2a)), replace_low_word(read_usize(jogo + 0x14), read_i16(jogo + 0x28)),
            read_usize(jogo + 0x20) as u32, read_usize(jogo + 0x1c) as u32);
        if v != 0 {
            crate::comparacao::ramo("cqg_10 vaga do amistoso livre");
            core::ptr::write_unaligned((v + 0x10) as *mut u32, 0); // cqg10: a vaga fica livre
            *((v + 0x18) as *mut u8) = 0;
        }
    }
    if read_u8(jogo + 0x43) == 0xff { // cqg10: jogo sem rodada
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
        // 0x53b900: push; add esp, 4 -> cdecl; 0x5cb9e0: push jogo; mov ecx, this -> thiscall, ret 4; 0x5abc30: push
        // jogo; mov ecx, [0xae2a58] -> thiscall, ret 4
        if selecao(read_usize(jogo + 0x1c)) != 0 {
            let de_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FRIENDLY_REMOVE_NATIONAL_FIXTURE);
            de_selecao(this, jogo);
        } else {
            let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_REMOVE_FIXTURE);
            crate::comparacao::ramo("cqg_10 amistoso fora do calendario");
            tirar(read_usize(G_FIXTURE_MANAGER), jogo); // cqg10: o amistoso sai do calendário
        }
    }
}

/// Comparer for `on_friendly_fixture_cancelled` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c6570: a original entrega notícias, cancela torneios e tira jogos do calendário (libera): só ela
/// roda. Antes, a cópia acha a vaga do amistoso (0x5c6740, só leitura); depois a vaga tem de estar livre (+0x10 = 0 e
/// +0x18 = 0).
pub(crate) unsafe extern "thiscall" fn comparar_on_friendly_fixture_cancelled(this: usize, jogo: usize, quem: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_ON_FRIENDLY_FIXTURE_CANCELLED);
    let mut vaga = 0usize;
    if read_usize(jogo + 0x14) == read_usize(this + 4) && read_usize(this + 0x34cc) != 0 {
        let vaga_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
        vaga = vaga_de(this, replace_low_word(read_usize(jogo + 0x1c), read_i16(jogo + 0x2a)), replace_low_word(read_usize(jogo + 0x14), read_i16(jogo + 0x28)),
            read_usize(jogo + 0x20) as u32, read_usize(jogo + 0x1c) as u32);
    }
    o(this, jogo, quem);
    let livre = vaga == 0 || (read_usize(vaga + 0x10) == 0 && read_u8(vaga + 0x18) == 0);
    PLACAR.registrar(livre, || format!("on_friendly_fixture_cancelled(jogo {:#x}) a vaga {:#x} não ficou livre", jogo, vaga));
}

/// by the fixture flag, the home national team (or its nation's first team): its fixture on the previous day leaves the
/// calendar; then the fixture itself leaves
/// Original: `friendly_cancel_national_fixture` (0x5cb9e0, friendly.cpp, thiscall, ret 4).
///
/// [pt] { 0x5cb9e0 } (thiscall, ret 4, friendly.cpp, quadro SEH da lista local e das datas): o amistoso de seleção cancelado
/// (0x5c6570). Pela marca do jogo (+0x4d & 0x300): 0x100 a seleção é o mandante (+0x1c), 0x200 a seleção principal da
/// nação dele (0x53b4f0); com id válido, o jogo dela na véspera (0x549ef0, 0x54c8e0 com 1, 0x417c10; 0x5acbe0 com a
/// véspera nas duas pontas) sai do calendário (0x5abc30). Depois o próprio jogo sai.
pub(crate) unsafe extern "thiscall" fn friendly_cancel_national_fixture(this: usize, jogo: usize) {
    let _ = this;
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_REMOVE_FIXTURE);
    // 0x68d020 / 0x68d040 / 0x68d1b0: lea ecx, local -> thiscall; 0x5abc30: push jogo; mov ecx, [0xae2a58] -> thiscall,
    // ret 4; 0x53b4f0: push nação; add esp, 4 -> cdecl; 0x549ef0: push +0x28; push +0x2a; lea ecx -> thiscall, ret 8;
    // 0x54c8e0: push 1; push &saída; lea ecx -> thiscall, ret 8; 0x417c10: push origem; push &saída; lea ecx ->
    // thiscall, ret 8; 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let marca = read_i16(jogo + 0x4d) as u16 & 0x300;
    let quem = if marca == 0x100 {
        read_usize(jogo + 0x1c)
    } else if marca == 0x200 {
        let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_U21_TEAM);
        principal(read_usize(read_usize(jogo + 0x1c) + 0x53))
    } else {
        0
    };
    if quem != 0 && read_usize(quem) as i32 != -1 {
        crate::comparacao::ramo("cqg_10 jogo da selecao na vespera");
        let id = read_usize(quem) as u32;
        let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
        let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
        let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
        let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, i32, i32, i32) -> i16 = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        let (mut d0, mut d1, mut d2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        let (p0, p1, p2) = (d0.as_mut_ptr() as usize, d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize);
        data_nova(p0, *((jogo + 0x2a) as *const u16) as u32, *((jogo + 0x28) as *const u16) as u32);
        let v = menos(p0, p1, 1); // cqg10: a véspera
        copiar(p0, p2, v);
        let a = read_usize(p0) as u32;
        let b = core::ptr::read_unaligned((p0 + 2) as *const u32);
        achar(read_usize(G_FIXTURE_MANAGER), a, b, a, b, id, l, -1, -2, -1);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
        let item = proximo(l);
        if item != 0 {
            tirar(read_usize(G_FIXTURE_MANAGER), item);
        }
    }
    tirar(read_usize(G_FIXTURE_MANAGER), jogo);
    soltar(l);
}

/// Comparer for `friendly_cancel_national_fixture` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5cb9e0: a original tira jogos do calendário (libera): só ela roda (ponytail: a cópia não tem o que
/// conferir sem refazer a busca no calendário já mudado; a contagem do ramo prova no modo Rust).
pub(crate) unsafe extern "thiscall" fn comparar_friendly_cancel_national_fixture(this: usize, jogo: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FRIENDLY_CANCEL_NATIONAL_FIXTURE)(this, jogo);
}

/// news 0xb to the invited clubs (a tournament starting after a week ago); the tournament leaves the pending list; the
/// created competition fixtures become friendlies again or leave; the competition is destroyed and its slot and owner
/// are cleared
/// Original: `Friendly::cancel_tournament` (0x5c8260, friendly.cpp, thiscall, ret 8).
///
/// [pt] { 0x5c8260 } (thiscall, ret 8, friendly.cpp, quadro SEH da notícia, das datas e da lista locais): `Friendly::
/// cancel_tournament(torneio, sem_o_dono)`, pelo amistoso cancelado (0x5c6570) e pelo dia dos amistosos. Sem torneio, a
/// caixa da linha 0x10ad. Se o torneio começa depois de uma semana atrás (0x54c8e0 com 7), a notícia 0xb (o dono, o dia
/// e o ano do começo +4/+6) vai aos convidados (+0x15, 4) que não recusaram (+0x25 = 1, salvo o dono) e ao dono se
/// `sem_o_dono` é 0. O torneio sai da lista dos pendentes (+0x34d0, 0x68d1f0). Com a competição criada (+0x65): os
/// jogos dela de uma semana antes a 10 dias depois do começo (0x5acbe0): o de hoje volta a ser amistoso (0x5ac7a0:
/// +0x42 = 0xff, +0x14 = a competição de amistosos, +0 = [0x9cf668]); os outros saem do calendário (0x5abc30) e, com
/// rodada, voltam como amistosos (0x5ab230). A competição é destruída (virtual +0 com 1) e +0x65 = 0. A vaga da tabela
/// de competições (+0xaf + o byte +0x64) e o dono (+0) zeram.
pub(crate) unsafe extern "thiscall" fn cancel_tournament(this: usize, torneio: usize, sem_o_dono: u32) {
    if torneio == 0 {
        show_error_at(0x9cd0a0, 0x10ad);
        return;
    }
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let manager_manager_remove: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_LLIST_REMOVE);
    // 0x54c8e0 / 0x54c5b0: push n; push &saída; mov ecx, data -> thiscall, ret 8; 0x68d020 / 0x68d040 / 0x68d1b0: lea ecx
    // (ou mov ecx, lista) -> thiscall; 0x68d1f0: push item; mov ecx, [this+0x34d0] -> thiscall, ret 4
    let mut d7 = [0u32; 2];
    let semana = menos(G_CURRENT_DATE, d7.as_mut_ptr() as usize, 7);
    let (dia7, ano7) = (read_i16(semana), read_i16(semana + 2) as u16);
    let inicio = torneio + 4;
    let (dia_i, ano_i) = (read_i16(inicio), read_i16(torneio + 6) as u16);
    if ano7 < ano_i || (ano7 == ano_i && dia7 < dia_i) {
        crate::comparacao::ramo("cqg_10 noticia do torneio cancelado");
        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
        let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
        // 0x77bbe0: push 0; push 0xb; lea ecx -> thiscall; 0x785790: push v; push k; lea ecx -> thiscall, ret 8;
        // 0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8
        let mut noticia = [0u32; 0x38];
        let nt = noticia.as_mut_ptr() as usize;
        construir(nt, 0xb, 0);
        let dono = read_usize(torneio);
        gravar(nt, 0, read_usize(dono) as u32);
        gravar(nt, 1, read_i16(inicio) as i32 as u32);
        gravar(nt, 2, *((torneio + 6) as *const u16) as u32);
        for k in 0..4usize {
            let c = read_usize(torneio + 0x15 + k * 4);
            if c == 0 {
                continue;
            }
            if c == dono && sem_o_dono != 0 {
                continue;
            }
            if read_u8(torneio + 0x25 + k) == 1 && c != dono {
                continue;
            }
            entregar(read_usize(G_NEWS), nt, c);
        }
    }
    let pendentes = read_usize(this + 0x34d0);
    if pendentes != 0 {
        core::ptr::write_unaligned((pendentes + 0xa) as *mut usize, read_usize(pendentes + 2));
        let mut item = proximo(read_usize(this + 0x34d0));
        while item != 0 {
            if torneio == read_usize(item) {
                manager_manager_remove(read_usize(this + 0x34d0), item);
                let p = read_usize(this + 0x34d0);
                core::ptr::write_unaligned((p + 0xa) as *mut usize, read_usize(p + 2));
            }
            item = proximo(read_usize(this + 0x34d0));
        }
    }
    let (mut da, mut db) = ([0u8; 8], [0u8; 8]);
    let (pa, pb) = (da.as_mut_ptr() as usize, db.as_mut_ptr() as usize);
    menos(inicio, pa, 7);
    mais(inicio, pb, 0xa);
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let competicao = read_usize(torneio + 0x65);
    if competicao != 0 {
        crate::comparacao::ramo("cqg_10 competicao do torneio desfeita");
        let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, i32, usize, u32, i32, i32) -> i16 = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
        let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
        let de_volta: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_STORED_FIXTURE);
        let tirar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_REMOVE_FIXTURE);
        let guardar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_INSERT_FIXTURE);
        // 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24; 0x549ef0: push +0x28; push +0x2a; lea ecx ->
        // thiscall, ret 8; 0x5ac7a0 / 0x5abc30 / 0x5ab230: push jogo; mov ecx, [0xae2a58] -> thiscall, ret 4
        let id_comp = read_usize(read_usize(competicao + 4)) as u32;
        achar(read_usize(G_FIXTURE_MANAGER), read_usize(pa) as u32, core::ptr::read_unaligned((pa + 2) as *const u32), read_usize(pb) as u32,
            core::ptr::read_unaligned((pb + 2) as *const u32), -1, l, id_comp, -2, -1);
        core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
        let mut j = proximo(l);
        while j != 0 {
            let mut d = [0u8; 8];
            let pd = d.as_mut_ptr() as usize;
            data_nova(pd, *((j + 0x2a) as *const u16) as u32, *((j + 0x28) as *const u16) as u32);
            if read_i16(G_CURRENT_DATE_YEAR) == read_i16(pd + 2) && read_i16(G_CURRENT_DATE) == read_i16(pd) {
                let r = de_volta(read_usize(G_FIXTURE_MANAGER), j);
                if r != 0 {
                    *((r + 0x42) as *mut u8) = 0xff;
                    core::ptr::write_unaligned((r + 0x14) as *mut usize, read_usize(this + 4));
                    core::ptr::write_unaligned(r as *mut usize, read_usize(KEY_CLUB_COMP_FRIENDLY));
                }
            } else {
                tirar(read_usize(G_FIXTURE_MANAGER), j);
                if read_u8(j + 0x43) != 0xff {
                    *((j + 0x42) as *mut u8) = 0xff;
                    core::ptr::write_unaligned((j + 0x14) as *mut usize, read_usize(this + 4));
                    core::ptr::write_unaligned(j as *mut usize, read_usize(KEY_CLUB_COMP_FRIENDLY));
                    guardar(read_usize(G_FIXTURE_MANAGER), j);
                }
            }
            j = proximo(l);
        }
        let c = read_usize(torneio + 0x65);
        if c != 0 {
            let destruir: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(read_usize(read_usize(c)));
            // o destrutor virtual (vtable +0): push 1; mov ecx, competição -> thiscall, ret 4
            destruir(c, 1);
        }
        core::ptr::write_unaligned((torneio + 0x65) as *mut usize, 0);
    }
    let k = (read_i16(this + 0xaf) as i32 + read_u8(torneio + 0x64) as i8 as i32) as usize;
    core::ptr::write_unaligned(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(k.wrapping_mul(4)) as *mut usize, 0);
    core::ptr::write_unaligned(torneio as *mut usize, 0);
    soltar(l);
}

/// Comparer for `cancel_tournament` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c8260: a original entrega notícias, tira jogos do calendário e destrói a competição: só ela roda,
/// com o plano conferido depois: o dono (+0) e a competição (+0x65) zerados, a vaga da tabela de competições zerada e
/// o torneio fora da lista dos pendentes.
pub(crate) unsafe extern "thiscall" fn comparar_cancel_tournament(this: usize, torneio: usize, sem_o_dono: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_CANCEL_TOURNAMENT);
    if torneio == 0 {
        o(this, torneio, sem_o_dono);
        return;
    }
    let k = (read_i16(this + 0xaf) as i32 + read_u8(torneio + 0x64) as i8 as i32) as usize;
    let vaga = read_usize(G_COMP_OBJECT_TABLE).wrapping_add(k.wrapping_mul(4));
    o(this, torneio, sem_o_dono);
    let mut in_list = false;
    let p = read_usize(this + 0x34d0);
    if p != 0 {
        let mut no = read_usize(p + 2);
        let mut n = 0;
        while no != 0 && n < 0x10000 {
            if read_usize(read_usize(no)) == torneio {
                in_list = true;
            }
            no = read_usize(no + 6);
            n += 1;
        }
    }
    let ok = read_usize(torneio) == 0 && read_usize(torneio + 0x65) == 0 && read_usize(vaga) == 0 && !in_list;
    PLACAR.registrar(ok, || format!("cancel_tournament(torneio {:#x}) dono {:#x} competição {:#x} vaga {:#x} na lista {}",
        torneio, read_usize(torneio), read_usize(torneio + 0x65), read_usize(vaga), in_list));
}




/// The Friendly object's nation list q (array of lists at +0x34f6, indexed by nation id or sorted position).
///
/// [pt] A lista de nações `q` do Friendly (vetor de listas em +0x34f6; a primeira parte indexa pela id da nação, a segunda
/// pela posição na lista ordenada +0x34fa, como na original).
pub(crate) unsafe fn nation_friendly_list(this: usize, q: usize) -> usize {
    read_usize(read_usize(this + 0x34f6).wrapping_add(q.wrapping_mul(4)))
}

/// The 4-byte values pointed to by each node of a LLIST, walking nodes without moving the cursor.
///
/// [pt] Os itens (os 4 bytes de cada) da lista, percorrendo os nós sem mexer no cursor (+2 o primeiro; nó: +0 o item, +6 o
/// seguinte).
pub(crate) unsafe fn list_item_ids(lista: usize) -> Vec<u32> {
    let mut v = Vec::new();
    if lista == 0 {
        return v;
    }
    let mut no = read_usize(lista + 2);
    while no != 0 {
        v.push(read_usize(read_usize(no)) as u32);
        no = read_usize(no + 6);
    }
    v
}

/// O ensaio do comparador da 0x5cbdf0: as nações usadas (o vetor local da original), as listas antes e o que a cópia
/// acrescentaria a cada uma (índice da lista, item).
#[derive(Clone)]
pub(crate) struct Cqc11Ensaio {
    pub(crate) usadas: Vec<u8>,
    pub(crate) antes: std::rc::Rc<Vec<Vec<u32>>>,
    pub(crate) novos: Vec<Vec<u32>>,
}

/// A fita do ensaio: os sorteios que a original fez (n, resposta), o próximo e se um n não bateu. Também estão nela os
/// sorteios de dentro das rotinas que a original chama no amistoso (0x53b3d0, 0x5c6f00, 0x5c5f40), que o ensaio pula.
pub(crate) static mut CQC11_FITA: (Vec<(i32, i32)>, usize, bool) = (Vec::new(), 0, false);

/// Dry-run RNG replacement: returns the next taped result if its n matches, else marks the tape broken and returns 0.
///
/// [pt] O sorteio do ensaio: o próximo da fita, com o mesmo n.
pub(crate) unsafe extern "C" fn tape_random(n: i32) -> i32 {
    let f = &mut *core::ptr::addr_of_mut!(CQC11_FITA);
    match f.0.get(f.1) {
        Some(&(m, r)) if m == n && !f.2 => {
            f.1 += 1;
            r
        }
        _ => {
            f.2 = true;
            0
        }
    }
}

/// True if the dry-run RNG tape was consumed exactly to its end without a mismatch.
pub(crate) unsafe fn tape_fully_consumed() -> bool {
    let f = &*core::ptr::addr_of!(CQC11_FITA);
    !f.2 && f.1 == f.0.len()
}

impl Cqc11Ensaio {
    fn usada(&self, id: usize) -> bool {
        self.usadas.get(id).is_some_and(|&b| b != 0)
    }
    fn usar(&mut self, id: usize) {
        if id >= self.usadas.len() {
            self.usadas.resize(id + 1, 0);
        }
        self.usadas[id] = 1;
    }
    fn por(&mut self, q: usize, item: u32) {
        if q >= self.novos.len() {
            self.novos.resize(q + 1, Vec::new());
        }
        self.novos[q].push(item);
    }
    fn tem(&self, q: usize, item: u32) -> bool {
        self.antes.get(q).is_some_and(|v| v.contains(&item)) || self.novos.get(q).is_some_and(|v| v.contains(&item))
    }
}

/// For a fixture whose two clubs (+0x1c, +0x20) both have nations: marks both nations as used and adds each to the
/// other's friendly list (dry run: records only).
///
/// [pt] Um jogo da primeira parte: com nação nos dois clubes (+0x1c, +0x20; +0x53, a id em +0), as duas ficam usadas e cada
/// uma vai à lista (pela id) da outra: a de casa recebe a de fora, depois o contrário (0x68d0d0 com 4: thiscall na
/// lista). No ensaio, só o registro.
pub(crate) unsafe fn link_fixture_nations(this: usize, p: usize, usadas: usize, ens: Option<&mut Cqc11Ensaio>) {
    let na = read_usize(read_usize(p + 0x1c) + 0x53);
    if na == 0 {
        return;
    }
    let a = read_usize(na) as u32;
    let nb = read_usize(read_usize(p + 0x20) + 0x53);
    if nb == 0 {
        return;
    }
    let b = read_usize(nb) as u32;
    match ens {
        Some(e) => {
            e.usar(a as usize);
            e.usar(b as usize);
            e.por(a as usize, b);
            e.por(b as usize, a);
        }
        None => {
            let por: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
            *((usadas.wrapping_add(a as usize)) as *mut u8) = 1;
            crate::comparacao::ramo("cqc_11 nacoes com jogo");
            *((usadas.wrapping_add(b as usize)) as *mut u8) = 1; // cqc11: as nações com jogo perto da data
            let (va, vb) = (a, b);
            por(nation_friendly_list(this, a as usize), &vb as *const u32 as usize, 4);
            por(nation_friendly_list(this, b as usize), &va as *const u32 as usize, 4);
        }
    }
}

/// First part of 0x5cbdf0: for every fixture from 4 days before to 4 days after the date (3 slices x 2 groups per day),
/// links the two clubs' nations (link_fixture_nations).
///
/// [pt] A primeira parte da 0x5cbdf0: de 4 dias antes a 4 dias depois da data (+3 do registro de datas; 0x54c8e0, 0x54c5b0
/// e 0x417c10), nas 3 fatias e nos 2 grupos de cada dia (0x5ae6a0), cada jogo com nação nos dois clubes (+0x1c, +0x20;
/// +0x53, a id em +0) marca as duas nações como usadas e cada uma vai à lista da outra (pela id: a de casa recebe a de
/// fora, depois o contrário; 0x68d0d0 com 4). No ensaio, nada se grava fora das datas estáticas (o cursor das listas
/// do calendário não se mexe).
pub(crate) unsafe fn link_nations_near_date(this: usize, datas: usize, usadas: usize, mut ens: Option<&mut Cqc11Ensaio>) {
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let lista_de: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_FIX_MAN_GET_FIXTURE_LIST_FOR_DATE);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let (mut x, mut y, mut t, mut u) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let (px, py, pt, pu) = (x.as_mut_ptr() as usize, y.as_mut_ptr() as usize, t.as_mut_ptr() as usize, u.as_mut_ptr() as usize);
    let data = datas + 3;
    copiar(px, pu, menos(data, pt, 4)); // a primeira data: 4 dias antes
    copiar(py, pu, mais(data, pt, 4)); // a última: 4 dias depois
    while read_u16(py + 2) > read_u16(px + 2) || (read_u16(py + 2) == read_u16(px + 2) && read_i16(py) >= read_i16(px)) {
        for fatia in 0u32..3 {
            for grupo in 0u32..2 {
                // ponytail: a original empurra os dois bytes com sobra de pilha em cima; a 0x5abf10 só lê o byte
                let l = lista_de(px, fatia, grupo);
                if let Some(e) = ens.as_deref_mut() {
                    let mut no = read_usize(l + 2);
                    while no != 0 {
                        link_fixture_nations(this, read_usize(no), usadas, Some(&mut *e));
                        no = read_usize(no + 6);
                    }
                } else {
                    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
                    let mut p = proximo(l);
                    while p != 0 {
                        link_fixture_nations(this, p, usadas, None);
                        p = proximo(l);
                    }
                }
            }
        }
        copiar(px, pu, mais(px, pt, 1)); // o dia seguinte
    }
}

/// True if the nation's continent (+0x71) matches (or -1 wildcard) one of the `quantas` 20-byte confederation entries
/// of the friendly date.
///
/// [pt] A nação `n` (ponteiro) está numa das confederações da data: alguma das `quantas` entradas de 20 bytes a partir de
/// `datas` tem em +0xc −1 ou a id do continente da nação (+0x71, lido sem conferir o nulo, como na original). Sem
/// entradas: não.
pub(crate) unsafe fn nation_in_date_confederations(datas: usize, quantas: i32, n: usize) -> bool {
    let mut k = 0;
    while k < quantas {
        let c = read_usize(datas + 0xc + k as usize * 0x14) as i32;
        if c == -1 || read_usize(read_usize(n + 0x71)) as i32 == c {
            return true;
        }
        k += 1;
    }
    false
}

/// True on an Africa-only friendly date (+2 = 1) for a nation whose continent is KEY_CONTINENT_AFRICA.
///
/// [pt] Na data só da África (+2 = 1), a nação africana (+0x71 = [0x9cfa08]) fica de fora.
pub(crate) unsafe fn is_african_excluded(datas: usize, n: usize) -> bool {
    *((datas + 2) as *const u8) == 1 && read_usize(n + 0x71) != 0 && read_usize(read_usize(n + 0x71)) == read_usize(KEY_CONTINENT_AFRICA)
}

/// Second part of 0x5cbdf0: for each free nation in the sorted list, finds the first suitable later free nation (random
/// draws, confederation, rivalry ban, reputation) and creates the international friendly; dry run replays taped draws.
///
/// [pt] A segunda parte da 0x5cbdf0: cada nação `a` da lista ordenada (+0x34fa, conta +0x34fe relida a cada volta) ainda
/// livre, numa das confederações da data e não africana na data da África procura, entre as seguintes `b` livres, a
/// primeira que serve: sorteio de 500 (0x90cfc0, antes dos filtros de `b`), `b` na data e não africana, uma das duas
/// com +0x88 ≥ 5, o par não proibido (0x5cc3e0), mais um sorteio de 1000 se o continente (+0x71) é o mesmo, a soma
/// acima de 400 e `b` ainda não na lista da posição de `a` (+0x34f6, pela posição; o cursor volta ao começo). Achada:
/// o mando sorteado (0x90cfc0 de 2), as seleções principais (0x53b3d0, `b` antes) e o amistoso (0x5c6f00 com a data
/// +3); criado, a resposta forçada (0x5c5f40 com 1), as duas marcadas e cada uma na lista da posição da outra (`b` na
/// de `a`, depois `a` na de `b`; os 4 bytes do registro, a id). Criado ou não, a próxima `a`. No ensaio, o amistoso
/// não é criado: vale o que a original fez (a id de `b` apareceu na lista da posição de `a`), os sorteios vêm da fita
/// e, depois do mando, o resto continua (desde `a` seguinte) no primeiro sorteio de 500 da fita (ou no fim) com que
/// fecha: os de dentro das rotinas do amistoso ficam para trás. Começa na nação `i0`.
pub(crate) unsafe fn pair_nations_for_friendlies(this: usize, datas: usize, n: u32, usadas: usize, mut ens: Option<&mut Cqc11Ensaio>, i0: i32) {
    let sortear: unsafe extern "C" fn(i32) -> i32 =
        if ens.is_some() { tape_random } else { core::mem::transmute(FN_UTILS_RANDOM) };
    let proibido: unsafe extern "thiscall" fn(usize, i32, i32) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_BANNED_NATION_PAIR);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> usize = core::mem::transmute(FN_FRIENDLY_ADD_FRIENDLY_MATCH);
    let respond: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_FRIENDLY_PROCESS_FRIENDLY_REPLY);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let por: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    // os sorteios: push n; add esp, 4 -> cdecl; 0x53b3d0: push; add esp, 4 -> cdecl; 0x5cc3e0, 0x5c6f00, 0x5c5f40 e
    // 0x68d0d0: mov ecx, esi (ou a lista) -> thiscall
    let quantas = n as i16 as i32;
    let total = |this: usize| read_usize(this + 0x34fe) as i32;
    let nacao = |this: usize, k: i32| read_usize(read_usize(this + 0x34fa).wrapping_add(k as usize * 4));
    let mut i: i32 = i0;
    while i < total(this) {
        let a = nacao(this, i);
        let ida = read_usize(a);
        let livre_a = match ens.as_deref() {
            Some(e) => !e.usada(ida),
            None => *((usadas.wrapping_add(ida)) as *const u8) == 0,
        };
        if livre_a && quantas > 0 && nation_in_date_confederations(datas, quantas, a) && !is_african_excluded(datas, a) {
            let mut j = i + 1;
            while j < total(this) {
                if ens.is_some() && (*core::ptr::addr_of!(CQC11_FITA)).2 {
                    return; // a fita já não bate
                }
                let b = nacao(this, j);
                let idb = read_usize(b);
                let livre_b = match ens.as_deref() {
                    Some(e) => !e.usada(idb),
                    None => *((usadas.wrapping_add(idb)) as *const u8) == 0,
                };
                if livre_b {
                    let mut v = sortear(500);
                    if nation_in_date_confederations(datas, quantas, b) && !is_african_excluded(datas, b)
                        && (read_usize(a + 0x88) as i32 >= 5 || read_usize(b + 0x88) as i32 >= 5)
                        && proibido(this, ida as i32, idb as i32) == 0
                    {
                        if read_usize(a + 0x71) == read_usize(b + 0x71) {
                            v = v.wrapping_add(sortear(1000)); // cqc11: o mesmo continente
                        }
                        if v > 400 { // cqc11: a soma acima de 400
                            let ja = match ens.as_deref() {
                                Some(e) => e.tem(i as usize, idb as u32),
                                None => {
                                    let l = nation_friendly_list(this, i as usize);
                                    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
                                    let mut achou = false;
                                    let mut p = proximo(l);
                                    while p != 0 {
                                        if read_usize(p) == idb {
                                            achou = true;
                                            break;
                                        }
                                        p = proximo(nation_friendly_list(this, i as usize));
                                    }
                                    achou
                                }
                            };
                            if !ja {
                                crate::comparacao::ramo("cqc_11 amistoso entre selecoes");
                                let casa = sortear(2) as u32; // cqc11: o mando sorteado
                                match ens.as_deref_mut() {
                                    Some(e) => {
                                        if list_item_ids(nation_friendly_list(this, i as usize)).contains(&(idb as u32)) {
                                            e.usar(idb);
                                            e.usar(ida);
                                            e.por(i as usize, idb as u32);
                                            e.por(j as usize, ida as u32);
                                        }
                                        let f = core::ptr::addr_of_mut!(CQC11_FITA);
                                        if (*f).2 {
                                            return;
                                        }
                                        let base = e.clone();
                                        let pontos: Vec<usize> = {
                                            let v = &(*f).0;
                                            ((*f).1..=v.len()).filter(|&p| p == v.len() || v[p].0 == 500).collect()
                                        };
                                        for p in pontos {
                                            (*f).1 = p;
                                            *e = base.clone();
                                            pair_nations_for_friendlies(this, datas, n, 0, Some(&mut *e), i + 1);
                                            if tape_fully_consumed() {
                                                return;
                                            }
                                            (*f).2 = false;
                                        }
                                        (*f).2 = true; // nenhum ponto fecha
                                        return;
                                    }
                                    None => {
                                        let signed_byte_at = selecao(b);
                                        let sa = selecao(a);
                                        let m = acrescentar(this, sa, datas + 3, signed_byte_at, casa);
                                        if m != 0 {
                                            respond(this, m, 1);
                                            *((usadas.wrapping_add(idb)) as *mut u8) = 1;
                                            *((usadas.wrapping_add(ida)) as *mut u8) = 1; // cqc11: as duas usadas
                                            por(nation_friendly_list(this, i as usize), b, 4);
                                            por(nation_friendly_list(this, j as usize), a, 4);
                                        }
                                    }
                                }
                                break;
                            }
                        }
                    }
                }
                j += 1;
            }
        }
        i += 1;
    }
}

/// the national team friendlies on an international date: the nations with a match from 4 days before to 4 after become
/// used (and each goes to the other one's list); then each free nation of the sorted list, in the date's confederation,
/// looks for the first following free one that fits (a draw of 500, plus 1000 on the same continent, above 400, one of
/// the two with +0x88 >= 5, the pair not banned and no match between them in the year) and the friendly is added with a
/// drawn home side
/// Original: `Friendly::arrange_international_friendlies` (0x5cbdf0, friendly.cpp, thiscall).
///
/// [pt] { 0x5cbdf0 } (thiscall, ret 8, friendly.cpp, quadro SEH só das datas locais, sem destrutor): `Friendly::
/// arrange_international_friendlies(datas, quantas)`, pelo dia dos amistosos internacionais (0x5cbb00, as entradas de
/// hoje + 12 semanas). O vetor das nações usadas ([0xae2350] bytes zerados, 0x945501; sem memória: a caixa, linha
/// 0x1738, e nada), a primeira parte (as nações com jogo de 4 dias antes a 4 depois) e a segunda (os pares), e o vetor
/// sai (0x9452ca).
pub(crate) unsafe extern "thiscall" fn arrange_international_friendlies(this: usize, datas: usize, n: u32) {
    let zerado: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    // calloc: push 1; push [0xae2350]; add esp, 8 -> cdecl (n, 1); free: push; add esp, 4 -> cdecl
    let usadas = zerado(read_usize(G_NATION_COUNT) as u32, 1);
    if usadas == 0 {
        show_error_at(0x9cd0a0, 0x1738);
        return;
    }
    link_nations_near_date(this, datas, usadas, None);
    pair_nations_for_friendlies(this, datas, n, usadas, None, 0);
    liberar(usadas);
}

/// Comparer for `arrange_international_friendlies` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original cria amistosos (0x5c6f00, 0x5c5f40), acrescenta às listas (0x68d0d0) e sorteia: só ela roda,
/// com o plano em ensaio. A primeira parte do ensaio roda antes dela (depois, o calendário já teria os amistosos
/// novos); a segunda, depois dela, pela fita dos sorteios que ela fez (aleatorio::rastrear: n e resposta, os de dentro
/// das rotinas do amistoso também), decide cada amistoso pelo que a original fez. Confere a sequência dos sorteios (os
/// do ensaio são os da original, salvo os de dentro das rotinas do amistoso, e a fita acaba junto) e, lista a lista de
/// nações, que os itens novos da original são os que o ensaio acrescentaria. (Lado a lado pelo estado do sorteador não
/// servia: a 0x5c6f00 e a 0x5c5f40 sorteiam por dentro na original e o ensaio não as chama.)
pub(crate) unsafe extern "thiscall" fn comparar_arrange_international_friendlies(this: usize, datas: usize, n: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ARRANGE_INTERNATIONAL_FRIENDLIES);
    let nacoes = read_usize(G_NATION_COUNT) as i32;
    let k = nacoes.max(read_usize(this + 0x34fe) as i32).max(0) as usize;
    let antes: Vec<Vec<u32>> = (0..k).map(|q| list_item_ids(nation_friendly_list(this, q))).collect();
    let mut e = Cqc11Ensaio { usadas: vec![0u8; nacoes.max(0) as usize], antes: std::rc::Rc::new(antes), novos: vec![Vec::new(); k] };
    link_nations_near_date(this, datas, 0, Some(&mut e));
    let ((), tape_lock) = crate::aleatorio::rastrear(|| o(this, datas, n));
    let quantos = tape_lock.len();
    *core::ptr::addr_of_mut!(CQC11_FITA) = (tape_lock, 0, false);
    pair_nations_for_friendlies(this, datas, n, 0, Some(&mut e), 0);
    let igual = tape_fully_consumed();
    let (pos, falhou) = { let f = &*core::ptr::addr_of!(CQC11_FITA); (f.1, f.2) };
    *core::ptr::addr_of_mut!(CQC11_FITA) = (Vec::new(), 0, false);
    let mut dif = Vec::new();
    for q in 0..k.max(e.novos.len()) {
        let mut depois = if q < k { list_item_ids(nation_friendly_list(this, q)) } else { Vec::new() };
        let mut falta = false;
        for x in e.antes.get(q).map(|v| v.as_slice()).unwrap_or(&[]) {
            match depois.iter().position(|y| y == x) {
                Some(p) => {
                    depois.remove(p);
                }
                None => falta = true,
            }
        }
        let mut esperado = e.novos.get(q).cloned().unwrap_or_default();
        depois.sort_unstable();
        esperado.sort_unstable();
        if falta || depois != esperado {
            dif.push((q, depois, esperado));
        }
    }
    PLACAR.registrar(igual && dif.is_empty(), || {
        format!("arrange_international_friendlies(datas {:x}, {}) tape_lock de {} sorteios, o ensaio parou em {} (n diferente: {}); listas (indice, original, ensaio) {:x?}",
            datas, n as i16, quantos, pos, falhou, dif.iter().take(4).collect::<Vec<_>>())
    });
}

/// the friendly invitation still holds: without a date, the text says whether the match was arranged or cancelled (the
/// club's matches on the day); with a date, before the day after tomorrow it is cancelled, the club free on the date
/// holds (1), else the text says the club is unavailable
/// Original: `friendly_check_match_invitation` (0x5ca090, friendly.cpp, thiscall).
///
/// [pt] { 0x5ca090 } (thiscall, ret 0x20, friendly.cpp, quadro SEH da LLIST local 0x68d020/0x68d040):
/// `friendly_check_match_invitation(a, b, clube, dia, ano, data, texto, tamanho)`, pela notícia do convite (0x77bc80,
/// 0x782b70). Com texto e tamanho (palavra) abaixo de 100: a caixa da linha 0x1527 e 1 (a original). Sem a data: com
/// texto, os jogos do clube `a` (+0 id) no dia (dia, ano; 0x5acbe0 numa lista local) dizem se o amistoso foi marcado
/// (o primeiro jogo é de `a` contra `b`) ou cancelado (0x66f4e0 no texto); 0. Com a data: antes de hoje + 2 dias
/// (0x54c5b0), cancelado (com texto) e 0; o clube livre na data (0x5c6b70 com tipo 0): 1; senão, com texto, "{} ... are
/// unavailable" (o byte +0x52 duas vezes e o nome +0x38) e 0.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn friendly_check_match_invitation(this: usize, a: usize, b: usize, clube: usize, dia: u32, ano: u32,
    data: usize, texto: usize, tamanho: u32) -> u32 {
    if texto != 0 && (tamanho as i16) < 100 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_FRIENDLY_CHECK_MATCH_INVITATION)(
            this, a, b, clube, dia, ano, data, texto, tamanho); // a caixa
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    // 0x66f4e0: add esp, 8 / 0x14 depois -> cdecl (variádica)
    if data == 0 {
        if texto != 0 {
            let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
            let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
            let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
                core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
            // 0x5acbe0: 9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24
            let mut l = [0u8; 0x10];
            let lista = l.as_mut_ptr() as usize;
            nova(lista);
            jogos(read_usize(G_FIXTURE_MANAGER), dia, ano, dia, ano, read_usize(a) as u32, lista, u32::MAX, 0xffff_fffe, u32::MAX);
            core::ptr::write_unaligned((lista + 0xa) as *mut usize, read_usize(lista + 2));
            let p = proximo(lista);
            let msg = if p != 0 && read_usize(p + 0x1c) == a && read_usize(p + 0x20) == b {
                0x9cd168 // "This match has been arranged."
            } else {
                0x9cd148 // "This match has been cancelled."
            };
            formatar(texto, msg);
            soltar(lista);
        }
        return 0;
    }
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let mut t = [0u8; 8];
    let r = mais(G_CURRENT_DATE, t.as_mut_ptr() as usize, 2);
    let (tano, tdia) = (read_u16(r + 2), read_i16(r));
    if tano > read_u16(data + 2) || (tano == read_u16(data + 2) && tdia > read_i16(data)) {
        if texto != 0 {
            formatar(texto, 0x9cd148usize);
        }
        return 0;
    }
    let livre: unsafe extern "stdcall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_CLUB_FREE_ON_DATE);
    // 0x5c6b70: 3 pushes sem add esp -> stdcall (o mov ecx da original sobra)
    if livre(data, clube, 0) != 0 {
        return 1;
    }
    if texto != 0 {
        let k = *((clube + club::SHORT_GENDER_NAME) as *const i8) as i32;
        formatar(texto, 0x9cd104usize, k, k, clube + club::SHORT_NAME); // "{}<%s - Club Name...>{} are unavailable for this match."
    }
    0
}

/// Region of the news text (up to the given length, max 0x200 bytes), or none for a null text.
///
/// [pt] O texto da notícia (até o tamanho dado, no máximo 0x200 bytes), quando há.
pub(crate) fn news_text_regions(texto: usize, tamanho: u32) -> Vec<(usize, usize)> {
    if texto == 0 {
        Vec::new()
    } else {
        vec![(texto, ((tamanho as i16).max(0) as usize).min(0x200))]
    }
}

/// Comparer for `friendly_check_match_invitation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo texto e a devolução (as listas locais nascem e morrem nas duas); com a caixa, só a
/// original.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn comparar_friendly_check_match_invitation(this: usize, a: usize, b: usize, clube: usize, dia: u32,
    ano: u32, data: usize, texto: usize, tamanho: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, usize, usize, u32) -> u32>(IDX_FRIENDLY_CHECK_MATCH_INVITATION);
    if texto != 0 && (tamanho as i16) < 100 {
        return o(this, a, b, clube, dia, ano, data, texto, tamanho);
    }
    let reg = news_text_regions(texto, tamanho);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, a, b, clube, dia, ano, data, texto, tamanho),
        || friendly_check_match_invitation(this, a, b, clube, dia, ano, data, texto, tamanho));
    PLACAR.registrar(igual && r == n, || format!("friendly_check_match_invitation(a {:x}, b {:x}, clube {:x}, data {:x}) original={} rust={} {:x?} x {:x?}",
        a, b, clube, data, r, n, mo, mr));
    r
}

/// the tournament invitation still holds: without a tournament or with its date before the day after tomorrow,
/// cancelled; the club free on the date (by the tournament type) holds (1), else the text says the club is unavailable
/// Original: `friendly_check_tournament_invitation` (0x5ca290, friendly.cpp, thiscall).
///
/// [pt] { 0x5ca290 } (thiscall, ret 0x10, friendly.cpp): `friendly_check_tournament_invitation(clube, torneio, texto,
/// tamanho)`, pela notícia do convite para torneio. Com texto e tamanho abaixo de 100: a caixa da linha 0x1575 e 1 (a
/// original). Sem torneio ou com a data dele (+4) antes de hoje + 2 dias: "This competition has been cancelled" (com
/// texto) e 0. O clube livre na data (0x5c6b70 com o tipo +0xc): 1; senão, com texto, "{} ... are unavailable for this
/// competition." e 0.
pub(crate) unsafe extern "thiscall" fn friendly_check_tournament_invitation(this: usize, clube: usize, torneio: usize, texto: usize, tamanho: u32) -> u32 {
    if texto != 0 && (tamanho as i16) < 100 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(IDX_FRIENDLY_CHECK_TOURNAMENT_INVITATION)(this, clube, torneio, texto, tamanho);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let mut cancelado = torneio == 0;
    if !cancelado {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
        let mut t = [0u8; 8];
        let r = mais(G_CURRENT_DATE, t.as_mut_ptr() as usize, 2);
        let (tano, tdia) = (read_u16(r + 2), read_i16(r));
        let d = torneio + 4;
        cancelado = tano > read_u16(d + 2) || (tano == read_u16(d + 2) && tdia > read_i16(d));
    }
    if cancelado {
        if texto != 0 {
            formatar(texto, 0x9cd1d4usize); // "This competition has been cancelled"
        }
        return 0;
    }
    let livre: unsafe extern "stdcall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_CLUB_FREE_ON_DATE);
    // ponytail: a original empurra o tipo (byte +0xc) com o resto do ecx em cima; a 0x5c6b70 só lê o byte
    if livre(torneio + 4, clube, *((torneio + 0xc) as *const u8) as u32) != 0 {
        return 1;
    }
    if texto != 0 {
        let k = *((clube + club::SHORT_GENDER_NAME) as *const i8) as i32;
        formatar(texto, 0x9cd188usize, k, k, clube + club::SHORT_NAME); // "... are unavailable for this competition."
    }
    0
}

/// Comparer for `friendly_check_tournament_invitation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo texto e a devolução; com a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_friendly_check_tournament_invitation(this: usize, clube: usize, torneio: usize, texto: usize, tamanho: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> u32>(IDX_FRIENDLY_CHECK_TOURNAMENT_INVITATION);
    if texto != 0 && (tamanho as i16) < 100 {
        return o(this, clube, torneio, texto, tamanho);
    }
    let reg = news_text_regions(texto, tamanho);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, torneio, texto, tamanho),
        || friendly_check_tournament_invitation(this, clube, torneio, texto, tamanho));
    PLACAR.registrar(igual && r == n, || format!("friendly_check_tournament_invitation(clube {:x}, torneio {:x}) original={} rust={} {:x?} x {:x?}",
        clube, torneio, r, n, mo, mr));
    r
}





/// As 110 datas internacionais da 0x5ca4f0, na ordem em que a original as grava: (+0 o dia da semana, +1 o dia, +2 o
/// mês a partir de 0, +0xb o ano a somar ao da Copa, o global do continente em +0xc, o global da competição em +0x10
/// ou 0 para −1). Extraídas da desmontagem (0x5ca588..0x5cb8e6); as 5 últimas vão pela 0x5cb9b0.
pub(crate) const CQB3_DATAS: [(u8, u8, u8, u8, usize, usize); 110] = [
    (5, 10, 9, 0, KEY_CONTINENT_AFRICA, 0),
    (2, 18, 10, 0, KEY_CONTINENT_AFRICA, 0),
    (6, 15, 11, 0, KEY_CONTINENT_AFRICA, 0),
    (6, 12, 0, 1, KEY_CONTINENT_AFRICA, 0),
    (6, 24, 1, 1, KEY_CONTINENT_AFRICA, 0),
    (2, 24, 2, 1, KEY_CONTINENT_AFRICA, 0),
    (2, 28, 3, 1, KEY_CONTINENT_AFRICA, 0),
    (2, 9, 5, 1, KEY_CONTINENT_AFRICA, 0),
    (6, 24, 6, 1, KEY_CONTINENT_AFRICA, 0),
    (2, 18, 7, 1, KEY_CONTINENT_AFRICA, 0),
    (5, 9, 9, 1, KEY_CONTINENT_AFRICA, 0),
    (5, 1, 5, 2, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (5, 15, 5, 2, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (6, 1, 9, 2, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (6, 10, 10, 2, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (6, 12, 0, 3, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (6, 24, 1, 3, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (5, 5, 3, 3, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (6, 27, 3, 3, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (5, 7, 5, 3, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (6, 24, 6, 3, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (6, 17, 7, 3, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (6, 28, 8, 3, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (5, 20, 9, 3, KEY_CONTINENT_AFRICA, KEY_CLUB_COMP_WORLD_CUP_AFRICAN_QUALIFYING_SECTION),
    (2, 16, 7, 0, KEY_CONTINENT_SOUTH_AMERICA, 0),
    (2, 6, 8, 0, KEY_CONTINENT_SOUTH_AMERICA, 0),
    (2, 7, 9, 0, KEY_CONTINENT_SOUTH_AMERICA, 0),
    (2, 15, 10, 0, KEY_CONTINENT_SOUTH_AMERICA, 0),
    (2, 29, 2, 1, KEY_CONTINENT_SOUTH_AMERICA, 0),
    (2, 26, 3, 1, KEY_CONTINENT_SOUTH_AMERICA, 0),
    (2, 16, 7, 1, KEY_CONTINENT_SOUTH_AMERICA, 0),
    (2, 6, 8, 1, KEY_CONTINENT_SOUTH_AMERICA, 0),
    (2, 7, 9, 1, KEY_CONTINENT_SOUTH_AMERICA, 0),
    (2, 26, 3, 2, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (6, 26, 6, 2, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (2, 6, 8, 2, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (5, 7, 9, 2, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (2, 15, 10, 2, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (2, 26, 3, 3, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (6, 4, 5, 3, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (6, 26, 6, 3, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (2, 16, 7, 3, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (2, 6, 8, 3, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (2, 7, 9, 3, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (2, 15, 10, 3, KEY_CONTINENT_SOUTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_SOUTH_AMERICAN_QUALIFYING_SECT),
    (5, 10, 9, 0, KEY_CONTINENT_OCEANIA, 0),
    (2, 18, 10, 0, KEY_CONTINENT_OCEANIA, 0),
    (6, 15, 11, 0, KEY_CONTINENT_OCEANIA, 0),
    (6, 10, 0, 1, KEY_CONTINENT_OCEANIA, 0),
    (2, 24, 2, 1, KEY_CONTINENT_OCEANIA, 0),
    (6, 25, 6, 1, KEY_CONTINENT_OCEANIA, 0),
    (1, 7, 8, 1, KEY_CONTINENT_OCEANIA, 0),
    (5, 9, 9, 1, KEY_CONTINENT_OCEANIA, 0),
    (2, 17, 10, 1, KEY_CONTINENT_OCEANIA, 0),
    (6, 10, 11, 2, KEY_CONTINENT_OCEANIA, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION),
    (6, 1, 1, 3, KEY_CONTINENT_OCEANIA, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION),
    (5, 10, 3, 3, KEY_CONTINENT_OCEANIA, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION),
    (5, 15, 3, 3, KEY_CONTINENT_OCEANIA, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION),
    (5, 10, 6, 3, KEY_CONTINENT_OCEANIA, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION),
    (5, 17, 8, 3, KEY_CONTINENT_OCEANIA, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION),
    (5, 29, 9, 3, KEY_CONTINENT_OCEANIA, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION),
    (5, 29, 10, 3, KEY_CONTINENT_OCEANIA, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION),
    (5, 24, 2, 0, KEY_CONTINENT_EUROPE, 0),
    (2, 25, 3, 0, KEY_CONTINENT_EUROPE, 0),
    (5, 2, 8, 0, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING),
    (2, 15, 10, 0, KEY_CONTINENT_EUROPE, 0),
    (5, 7, 9, 0, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING),
    (2, 15, 10, 1, KEY_CONTINENT_EUROPE, 0),
    (5, 24, 2, 1, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING),
    (2, 25, 3, 1, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING),
    (2, 6, 5, 1, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING),
    (5, 2, 8, 1, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING),
    (2, 6, 8, 1, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING),
    (5, 24, 2, 2, KEY_CONTINENT_EUROPE, 0),
    (2, 25, 3, 2, KEY_CONTINENT_EUROPE, 0),
    (5, 7, 9, 2, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_EUROPEAN_CHAMPIONSHIP_QUALIFYING),
    (2, 15, 10, 2, KEY_CONTINENT_EUROPE, 0),
    (2, 15, 10, 3, KEY_CONTINENT_EUROPE, 0),
    (5, 7, 9, 2, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION),
    (5, 24, 2, 3, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION),
    (2, 25, 3, 3, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION),
    (2, 6, 5, 3, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION),
    (5, 2, 8, 3, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION),
    (2, 6, 8, 3, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION),
    (5, 7, 9, 3, KEY_CONTINENT_EUROPE, KEY_CLUB_COMP_WORLD_CUP_EUROPEAN_QUALIFYING_SECTION),
    (5, 25, 7, 0, KEY_CONTINENT_NORTH_AMERICA, 0),
    (2, 18, 10, 0, KEY_CONTINENT_NORTH_AMERICA, 0),
    (6, 15, 11, 0, KEY_CONTINENT_NORTH_AMERICA, 0),
    (6, 10, 0, 1, KEY_CONTINENT_NORTH_AMERICA, 0),
    (6, 23, 1, 1, KEY_CONTINENT_NORTH_AMERICA, 0),
    (2, 24, 2, 1, KEY_CONTINENT_NORTH_AMERICA, 0),
    (2, 28, 3, 1, KEY_CONTINENT_NORTH_AMERICA, 0),
    (1, 7, 8, 1, KEY_CONTINENT_NORTH_AMERICA, 0),
    (5, 9, 9, 1, KEY_CONTINENT_NORTH_AMERICA, 0),
    (5, 25, 6, 2, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (5, 25, 7, 2, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (6, 17, 10, 2, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (6, 26, 0, 3, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (6, 23, 1, 3, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (6, 2, 2, 3, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (5, 15, 2, 3, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (6, 20, 3, 3, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (6, 29, 5, 3, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (2, 9, 8, 3, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (2, 1, 9, 3, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (6, 16, 10, 3, KEY_CONTINENT_NORTH_AMERICA, KEY_CLUB_COMP_WORLD_CUP_CONCACAF_QUALIFYING_SECTION),
    (2, 9, 5, 1, KEY_CONTINENT_ASIA, 0),
    (1, 7, 8, 1, KEY_CONTINENT_ASIA, 0),
    (2, 4, 5, 3, KEY_CONTINENT_ASIA, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION),
    (5, 13, 8, 3, KEY_CONTINENT_ASIA, KEY_CLUB_COMP_WORLD_CUP_ASIAN_QUALIFYING_SECTION),
];

/// allocates the table (+0x34ec) and writes the 110 international dates of 20 bytes (weekday, day, month, year offset
/// from the World Cup year, confederation and qualifying competition), shrinks the table to 110, resolves the dates
/// (0x5cc510) and marks the last day done (+0x34f4 = 400)
/// Original: `Friendly::build_international_date_table` (0x5ca4f0, friendly.cpp, thiscall).
///
/// [pt] { 0x5ca4f0 } (thiscall, ret 0, friendly.cpp): `Friendly::build_international_date_table()`, pelo construtor da
/// competição de amistosos (0x5c1f80, na criação do jogo). Conta 0xc8 em +0x34f0 e a tabela de 0xfa0 bytes (malloc
/// 0x944e46: push n; add esp, 4 -> cdecl) em +0x34ec (sem memória: a caixa da linha 0x15d5, conta 0: a original);
/// grava as 110 datas de 20 bytes (as 105 primeiras direto, as 5 últimas pela 0x5cb9b0: mov ecx, entrada; 6 pushes;
/// sem add esp -> thiscall, ret 0x18); com a conta abaixo de 110 (nunca: é a 0xc8 de cima) a caixa da linha 0x15d8;
/// conta 110, a tabela reduzida a 0x898 bytes (realloc 0x945667: 2 pushes; add esp, 8 -> cdecl), as datas no ciclo
/// da Copa (0x5cc510: mov ecx, esi; sem push -> thiscall) e o último dia feito (+0x34f4) = 400.
pub(crate) unsafe extern "thiscall" fn build_international_date_table(this: usize) {
    *((this + 0x34f0) as *mut i32) = 0xc8;
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = malloc(0xfa0);
    *((this + 0x34ec) as *mut usize) = t;
    if t == 0 {
        // ponytail: sem memória, a caixa; a original refaz a conta e cai nela (raro demais para copiar a caixa)
        *((this + 0x34f0) as *mut i32) = 0xc8;
        return original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_INTERNATIONAL_DATE_TABLE)(this);
    }
    let preencher: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32) = core::mem::transmute(FN_FRIENDLY_INTERNATIONAL_DATE_SET);
    for (k, &(semana, dia, mes, ano, continente, competicao)) in CQB3_DATAS.iter().enumerate() {
        let e = read_usize(this + 0x34ec) + k * 0x14;
        let comp = if competicao == 0 { -1 } else { read_i32(competicao) };
        if k >= 105 {
            // ponytail: a original empurra registradores inteiros (o ecx, o edx, o ebx) com o byte certo embaixo; a 0x5cb9b0
            // só lê os bytes: aqui, os bytes limpos
            preencher(e, semana as u32, dia as u32, mes as u32, ano as u32, read_i32(continente) as u32, comp as u32);
            continue;
        }
        *(e as *mut u8) = semana;
        *((e + 1) as *mut u8) = dia;
        *((e + 2) as *mut u8) = mes;
        *((e + 0xb) as *mut u8) = ano; // cqb3: o ano a somar ao da Copa
        core::ptr::write_unaligned((e + 0xc) as *mut i32, read_i32(continente));
        core::ptr::write_unaligned((e + 0x10) as *mut i32, comp);
    }
    if read_i32(this + 0x34f0) < 0x6e {
        return; // ponytail: inalcançável (a conta é a 0xc8 gravada acima); a original dá a caixa da linha 0x15d8
    }
    crate::comparacao::ramo("cqb_3 tabela montada");
    *((this + 0x34f0) as *mut i32) = 0x6e;
    let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    *((this + 0x34ec) as *mut usize) = realloc(read_usize(this + 0x34ec), 0x898);
    let resolver: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FRIENDLY_RESOLVE_INTERNATIONAL_DATES);
    resolver(this);
    *((this + 0x34f4) as *mut u16) = 0x190;
}

/// Comparer for `build_international_date_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5ca4f0: a cópia primeiro (aloca a tabela dela, que sai copiada e liberada: free 0x9452ca, cdecl),
/// depois a original a partir dos mesmos +0x34ec..+0x34f5; confere a conta, o último dia feito e as 110 datas. Roda
/// uma vez, no construtor dos amistosos.
pub(crate) unsafe extern "thiscall" fn comparar_build_international_date_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_INTERNATIONAL_DATE_TABLE);
    let campos = crate::comparacao::Memoria::ler(&[(this + 0x34ec, 10)]);
    build_international_date_table(this);
    let (t, n) = (read_usize(this + 0x34ec), read_i32(this + 0x34f0));
    if t == 0 || n != 0x6e {
        campos.escrever();
        return o(this); // ponytail: sem memória na cópia, só a original
    }
    let mr = crate::comparacao::Memoria::ler(&[(t, 0x898), (this + 0x34f0, 6)]);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    free(t);
    campos.escrever();
    o(this);
    let t = read_usize(this + 0x34ec);
    if t == 0 {
        return PLACAR.registrar(false, || "build_international_date_table: a original ficou sem tabela".to_string());
    }
    let mo = crate::comparacao::Memoria::ler(&[(t, 0x898), (this + 0x34f0, 6)]);
    PLACAR.registrar(mo.0[0].1 == mr.0[0].1 && mo.0[1].1 == mr.0[1].1, || format!("build_international_date_table original={:x?} rust={:x?}", mo, mr));
}

/// gives each entry its date in the World Cup cycle (the first year with (year - 2) a multiple of 4, plus the offset,
/// less 4 when more than 3 years ahead) through 0x54a130 and sorts the table by date
/// Original: `Friendly::resolve_international_dates` (0x5cc510, friendly.cpp, thiscall).
///
/// [pt] { 0x5cc510 } (thiscall, ret 0, friendly.cpp): `Friendly::resolve_international_dates()`, pela tabela (0x5ca4f0). O
/// ano da Copa: o primeiro a partir do de hoje ([0xae2c92]) com (ano − 2) múltiplo de 4. Cada data (+0x34ec, conta
/// +0x34f0) ganha a data em +3 pela 0x54a130 (lea ecx, entrada + 3; 4 pushes; sem add esp -> thiscall(data, dia, mês,
/// ano, dia da semana), ret 0x10) com o dia (+1) e o dia da semana (+0) em byte com sinal, o mês (+2) e o ano da Copa
/// mais o +0xb (com sinal), menos 4 se passar de 3 anos à frente de hoje (palavras). Depois o qsort pela data (0x945333:
/// 4 pushes; add esp, 0x10 -> cdecl(tabela, conta, 0x14, 0x5cc610)).
pub(crate) unsafe extern "thiscall" fn resolve_international_dates(this: usize) {
    let hoje = read_u16(G_CURRENT_DATE_YEAR);
    let mut ciclo = hoje as u32;
    while ((ciclo & 0xffff) as i32 - 2) % 4 != 0 {
        crate::comparacao::ramo("cqb_3 ano da copa adiante");
        ciclo = ciclo.wrapping_add(1); // cqb3: o ano da Copa
    }
    let acertar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_DATE_SET);
    let mut k = 0i32;
    while k < read_i32(this + 0x34f0) {
        let e = read_usize(this + 0x34ec).wrapping_add((k as usize).wrapping_mul(0x14));
        let mut ano = (read_u8(e + 0xb) as i8 as i16 as u16).wrapping_add(ciclo as u16);
        if ano as i32 - hoje as i32 > 3 {
            crate::comparacao::ramo("cqb_3 ano para tras");
            ano = ano.wrapping_add(0xfffc); // cqb3: menos 4 se passar de 3 anos à frente
        }
        let com_sinal = |o: usize| read_u8(e + o) as i8 as i16 as u16 as u32;
        // ponytail: o dia e o dia da semana vão sobre o edx de metade alta zerada (como a original); o mês e o ano vão
        // sobre o eax, de metade alta lixo, e a 0x54a130 só lê o byte do mês e a palavra do ano: aqui, limpos
        acertar(e + 3, com_sinal(1), read_u8(e + 2) as u32, ano as u32, com_sinal(0));
        k += 1;
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_usize(this + 0x34ec), read_i32(this + 0x34f0), 0x14, 0x5cc610);
}

/// Comparer for `resolve_international_dates` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5cc510: lado a lado nas datas da tabela (só grava nelas; o qsort e a 0x54a130 não sorteiam).
pub(crate) unsafe extern "thiscall" fn comparar_resolve_international_dates(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_RESOLVE_INTERNATIONAL_DATES);
    let (t, n) = (read_usize(this + 0x34ec), read_i32(this + 0x34f0));
    let reg = if t != 0 && n > 0 { vec![(t, n as usize * 0x14)] } else { Vec::new() };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || resolve_international_dates(this));
    PLACAR.registrar(igual, || format!("resolve_international_dates(conta {}) original={:x?} rust={:x?}", n, mo, mr));
}

/// the tournament in use (of 127, 0x69 bytes from +0xb1) that the current human manager club hosts, or 0
/// Original: `Friendly::find_human_club_tournament` (0x5c6830, friendly.cpp, thiscall).
///
/// [pt] { 0x5c6830 } (thiscall, ret 0, friendly.cpp): `Friendly::find_human_club_tournament()`, pelas telas dos amistosos. O
/// clube (+0x39) do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]); entre os 127 torneios (0x69 bytes a partir
/// de +0xb1), o primeiro com esse anfitrião (+0) e em uso (+0x29 ≠ 0): o ponteiro dele; senão 0.
pub(crate) unsafe extern "thiscall" fn find_human_club_tournament(this: usize) -> usize {
    let tecnico = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if tecnico == 0 {
        return 0;
    }
    let clube = read_usize(tecnico + 0x39);
    if clube == 0 {
        return 0;
    }
    for i in 0..0x7fusize {
        let t = this + 0x69 * i + 0xb1;
        let anfitriao = read_usize(t);
        if anfitriao != 0 && anfitriao == clube && read_usize(t + 0x29) != 0 {
            return t;
        }
    }
    0
}

comparar_valor!(comparar_find_human_club_tournament, find_human_club_tournament, IDX_FIND_HUMAN_CLUB_TOURNAMENT, () -> usize);







/// True if the 8-byte date at `a` is before the current game date (G_CURRENT_DATE / G_CURRENT_DATE_YEAR).
///
/// [pt] A data de 8 bytes em `a` está antes de hoje ([0xae2c92] o ano, sem sinal; [0xae2c90] o dia, com sinal).
pub(crate) unsafe fn is_before_today(a: usize) -> bool {
    let (ano, hoje) = (read_i16(G_CURRENT_DATE_YEAR) as u16, read_i16(G_CURRENT_DATE));
    let y = read_i16(a + 2) as u16;
    y < ano || (y == ano && read_i16(a) < hoje)
}

/// True if the 8-byte date at `a` is after the current game date.
///
/// [pt] A data de 8 bytes em `a` está depois de hoje.
pub(crate) unsafe fn is_after_today(a: usize) -> bool {
    let (ano, hoje) = (read_i16(G_CURRENT_DATE_YEAR) as u16, read_i16(G_CURRENT_DATE));
    let y = read_i16(a + 2) as u16;
    y > ano || (y == ano && read_i16(a) > hoje)
}

/// rebuilds the friendly plan (+0x34d8); for each league in pre-season, the AI clubs of the nation (by searching the
/// sorted candidates) get a band by reputation and a draw, and up to draw(3) + 4 friendlies with opponents found
/// (0x5c4cb0) on free dates (0x5c4380), the band falling 20% each time; at the end, the plan sorted by date
/// Original: `Friendly::plan_club_friendlies` (0x5c4770, friendly.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5c4770 } (thiscall, ret 0xc, friendly.cpp): `Friendly::plan_club_friendlies(entradas, ordenados, conta)`, pelo
/// plano da pré-temporada da IA (0x5c27a0). Libera o plano antigo (+0x34d8; free 0x9452ca, cdecl), aloca o novo (malloc
/// 0x944e46 de 2000 bytes, cdecl; sem memória a caixa da linha 0x6f1), capacidade +0x34e4 = 100, conta +0x34e0 e
/// +0x34dc zerados. Para cada uma das 35 ligas ativas (0xb63d60, 0x48 bytes) de nação com +0x11c, cuja temporada (+0x2a)
/// não começou e cuja data +0x39 menos 0x34 (0x54cab0: lea ecx, [liga + 0x39]; 2 pushes; sem add esp -> thiscall) já
/// chegou: os candidatos da nação (bsearch 0x946464: 5 pushes; add esp, 0x14 -> cdecl, pela 0x5c5290; recua até o
/// primeiro da nação; o último da lista nunca entra) até o primeiro de outra nação. Cada clube que não é humano
/// (0x600a50: 5 pushes; add esp, 0x14 -> cdecl(clube, 1, 1, 0, 0)), não é o de +0x1db = [0x9cf6f8] e tem liga (+0x57)
/// registrada em [0xadadfc]: faixa = reputação / 500 + (sorteio(3) + 1, 3 vira 2 com sorteio(4) zero); até sorteio(3) +
/// 4 amistosos (pela conta +4 do candidato): o adversário (0x5c4cb0: 5 pushes; ecx = this -> thiscall) e, achado, o
/// amistoso em data livre (0x5c4380: 3 pushes; ecx = this -> thiscall) com o de faixa (+5) maior como anfitrião e
/// mandante; sem adversário ou com o amistoso marcado, a faixa × 0,8 (no mínimo 1). No fim, o qsort do plano pela data
/// (0x945333: 4 pushes; add esp, 0x10 -> cdecl, pela 0x778e90).
pub(crate) unsafe extern "thiscall" fn plan_club_friendlies(this: usize, entradas: usize, ordenados: usize, conta: u32) {
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    if read_usize(this + 0x34d8) != 0 {
        free(read_usize(this + 0x34d8));
    }
    *((this + 0x34e4) as *mut i32) = 100;
    *((this + 0x34e0) as *mut i32) = 0;
    *((this + 0x34dc) as *mut i32) = 0;
    let p = malloc(0x7d0);
    *((this + 0x34d8) as *mut usize) = p;
    if p == 0 {
        show_error_at(0x9cd0a0, 0x6f1);
        return;
    }
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let busca: unsafe extern "C" fn(*const usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let adversario: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_FRIENDLY_OPPONENT);
    let planejar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_PLAN_FRIENDLY_ON_FREE_DATE);
    for k in 0..0x23usize {
        let liga = 0xb63d60 + k * 0x48;
        let nacao = read_usize(liga);
        if nacao == 0 || *((nacao + nation::NATION_LEAGUE_SELECTED) as *const u8) == 0 {
            continue;
        }
        if is_before_today(liga + 0x2a) {
            continue;
        }
        let mut d = [0u8; 8];
        let r = menos(liga + 0x39, &mut d, 0x34);
        if is_after_today(r) {
            continue;
        }
        // a chave da busca: um candidato de 0x16 bytes com a nação (+6) e a faixa 0xff (+5)
        let mut chave = [0u8; 0x16];
        chave[5] = 0xff;
        chave[6..10].copy_from_slice(&read_i32(nacao).to_le_bytes());
        let pchave = chave.as_ptr() as usize;
        let mut q = busca(&pchave, ordenados, conta, 4, 0x5c5290);
        if q == 0 {
            continue;
        }
        let id = read_i32(read_usize(liga));
        while q > ordenados {
            if read_i32(read_usize(q - 4) + 6) != id {
                break;
            }
            q -= 4;
        }
        let ultimo = ordenados.wrapping_add((conta as usize).wrapping_mul(4)).wrapping_sub(4);
        while q < ultimo {
            let cand = read_usize(q);
            let clube = month_award_club_by_id(read_i32(cand));
            if read_usize(clube + club::NATION) != read_usize(liga) {
                break;
            }
            q += 4;
            if humano(clube, 1, 1, 0, 0) != 0 {
                continue;
            }
            if read_i32(clube + club::EURO_FLAG) == read_i32(KEY_CLUB_COMP_INTER_TOTO_CUP) {
                continue;
            }
            let lg = read_usize(clube + club::DIVISION);
            if lg == 0 || read_usize(read_usize(G_COMP_OBJECT_TABLE) + (read_i32(lg) as usize).wrapping_mul(4)) == 0 {
                continue;
            }
            crate::comparacao::ramo("cqb_4 clube planejado");
            let mut b = (friendly_random(3) as u8).wrapping_add(1); // cqb4: o acréscimo da faixa
            if b == 3 && friendly_random(4) == 0 {
                b = 2;
            }
            let mut band = ((read_i16(clube + club::REPUTATION) as i32 / 500) as u8).wrapping_add(b);
            let quantos = (friendly_random(3) as u8).wrapping_add(4) as i8; // cqb4: quantos amistosos
            let mut j = 0i8;
            while read_i8(cand + 4) < quantos && j < quantos {
                // ponytail: a original empurra o dword do local da faixa (os bytes altos são sobra); a 0x5c4cb0 só lê o byte
                let opp = adversario(this, cand, entradas, conta, band as u32, 0);
                let marcado = if opp != 0 {
                    crate::comparacao::ramo("cqb_4 adversario achado");
                    if read_i8(cand + 5) > read_i8(opp + 5) { // cqb4: o de faixa maior em casa
                        planejar(this, cand, opp, cand)
                    } else {
                        planejar(this, cand, cand, opp)
                    }
                } else {
                    1
                };
                if marcado != 0 {
                    let f = match_pl_ftol(band as i8 as i32 as f64 * read_f64_at(0x9689b0)) as u8;
                    band = if (f as i8) < 1 { 1 } else { f };
                }
                j = j.wrapping_add(1);
            }
        }
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(read_usize(this + 0x34d8), read_i32(this + 0x34e0), 0x14, 0x778e90);
}

/// Comparer for `plan_club_friendlies` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c4770: só a original (ponytail: ela libera e aloca o plano, e a 0x5c4380 aloca as listas de datas e
/// de adversários dos candidatos; não roda duas vezes), conferindo o plano que ela deixa: o plano alocado, a capacidade
/// (+0x34e4) não menor que a conta (+0x34e0), o +0x34dc zerado e o plano em ordem de data (a 0x778e90 entre vizinhos).
pub(crate) unsafe extern "thiscall" fn comparar_plan_club_friendlies(this: usize, entradas: usize, ordenados: usize, conta: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(IDX_PLAN_CLUB_FRIENDLIES)(this, entradas, ordenados, conta);
    let (p, n, cap, z) = (read_usize(this + 0x34d8), read_i32(this + 0x34e0), read_i32(this + 0x34e4), read_i32(this + 0x34dc));
    if p == 0 {
        return; // sem memória: a caixa
    }
    let ordem: unsafe extern "C" fn(usize, usize) -> i32 = core::mem::transmute(FN_COMPARE_CALL_UP_RECORDS_BY_DATE);
    let ordenado = (1..n.max(0) as usize).all(|i| ordem(p + (i - 1) * 0x14, p + i * 0x14) <= 0);
    PLACAR.registrar(n >= 0 && cap >= n && z == 0 && ordenado, || format!("plan_club_friendlies: plano {:#x} conta {} capacidade {} +0x34dc {} em ordem {}", p, n, cap, z, ordenado));
}

/// First of the 127 tournament slots (0x69 bytes from +0xb1) with no host (+0), or 0.
///
/// [pt] O torneio livre: o primeiro dos 127 (0x69 bytes a partir de +0xb1) sem anfitrião (+0), ou 0.
pub(crate) unsafe fn free_tournament_slot(this: usize) -> usize {
    (0..0x7fusize).map(|i| this + 0x69 * i + 0xb1).find(|&t| read_usize(t) == 0).unwrap_or(0)
}

/// fills the first of the 127 free tournaments with the host, the type, the start date, the rule bytes and words, the
/// name and the 4 invited clubs (flagged), or 0 with no free slot
/// Original: `Friendly::add_tournament` (0x5c6e00, friendly.cpp, thiscall, ret 0x28).
///
/// [pt] { 0x5c6e00 } (thiscall, ret 0x28, friendly.cpp): `Friendly::add_tournament(anfitrião, tipo, início, a, b, c, d, e,
/// convidados, nome)`, pelo torneio da IA (0x5c3be0) e pelas telas do clube (0x4701f0). O primeiro torneio livre (sem
/// livre: 0) ganha o anfitrião (+0), o tipo (+0xc), os bytes a e b (+0xd, +0xe), a data de início (+4, pela 0x417c10:
/// lea ecx, [t + 4]; 2 pushes; sem add esp -> thiscall), as palavras c, d, e (+0xf, +0x11, +0x13), +0x29, +0x2d e
/// +0x65 zerados, o nome (+0x31, com o zero) e os 4 convidados (+0x15, dwords) com as marcas +0x25..+0x28 em 1.
pub(crate) unsafe extern "thiscall" fn add_tournament(this: usize, anfitriao: usize, tipo: u32, inicio: usize, a: u32, b: u32, c: u32, d: u32, e: u32, convidados: usize, nome: usize) -> usize {
    let t = free_tournament_slot(this);
    if t == 0 {
        return 0;
    }
    *((t + 0xc) as *mut u8) = tipo as u8;
    *((t + 0xd) as *mut u8) = a as u8;
    core::ptr::write_unaligned(t as *mut usize, anfitriao);
    *((t + 0xe) as *mut u8) = b as u8;
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut tmp = [0u8; 8];
    copiar(t + 4, &mut tmp, inicio);
    core::ptr::write_unaligned((t + 0x11) as *mut u16, d as u16);
    core::ptr::write_unaligned((t + 0x13) as *mut u16, e as u16);
    core::ptr::write_unaligned((t + 0x65) as *mut u32, 0);
    core::ptr::write_unaligned((t + 0x29) as *mut u32, 0);
    core::ptr::write_unaligned((t + 0x2d) as *mut u32, 0);
    core::ptr::write_unaligned((t + 0xf) as *mut u16, c as u16);
    let mut k = 0usize;
    loop {
        let x = *((nome + k) as *const u8);
        *((t + 0x31 + k) as *mut u8) = x;
        if x == 0 {
            break;
        }
        k += 1;
    }
    crate::comparacao::ramo("cqb_4 torneio acrescentado");
    for k in 0..4usize {
        core::ptr::write_unaligned((t + 0x15 + 4 * k) as *mut u32, read_usize(convidados + 4 * k) as u32);
        *((t + 0x25 + k) as *mut u8) = 1;
    }
    t
}

/// Comparer for `add_tournament` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c6e00: lado a lado no torneio livre (0x69 bytes, mais o que um nome comprido passar) e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_add_tournament(this: usize, anfitriao: usize, tipo: u32, inicio: usize, a: u32, b: u32, c: u32, d: u32, e: u32, convidados: usize, nome: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32, u32, u32, usize, usize) -> usize>(IDX_ADD_TOURNAMENT);
    let t = free_tournament_slot(this);
    let mut tam = 0usize;
    while nome != 0 && *((nome + tam) as *const u8) != 0 {
        tam += 1;
    }
    let reg = if t != 0 { vec![(t, 0x69usize.max(0x31 + tam + 1))] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, anfitriao, tipo, inicio, a, b, c, d, e, convidados, nome), || add_tournament(this, anfitriao, tipo, inicio, a, b, c, d, e, convidados, nome));
    PLACAR.registrar(igual && r == n, || format!("add_tournament(anfitriao {:x}) original={:#x} {:x?} rust={:#x} {:x?}", anfitriao, r, mo, n, mr));
    r
}

/// the friendly in the list (+0x34c8) that the current human manager club requested and that still waits for an
/// opponent, or 0
/// Original: `Friendly::find_human_club_open_invitation` (0x5c68b0, friendly.cpp, thiscall).
///
/// [pt] { 0x5c68b0 } (thiscall, ret 0, friendly.cpp): `Friendly::find_human_club_open_invitation()`, pelas telas dos
/// amistosos. Com a lista de amistosos (+0x34c8) e o clube (+0x39) do técnico humano da vez ([0xb74c26 + 0x300 ×
/// [0xb77c7a]]): o cursor da lista (+0xa) volta ao começo (+2) e o primeiro amistoso desse clube (+0xc) ainda sem
/// data (+8 ≠ 0) pelo próximo da lista (0x68d1b0: ecx = a lista; sem push -> thiscall); senão 0.
pub(crate) unsafe extern "thiscall" fn find_human_club_open_invitation(this: usize) -> usize {
    let lista = read_usize(this + 0x34c8);
    if lista == 0 {
        return 0;
    }
    let tecnico = read_usize(G_SCREEN_RECORDS_MANAGER.wrapping_add((*(G_SCREEN_MANAGER_CURRENT_SLOT as *const i16) as i32 as usize).wrapping_mul(0x300)));
    if tecnico == 0 {
        return 0;
    }
    let clube = read_usize(tecnico + 0x39);
    if clube == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, read_usize(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    loop {
        let m = proximo(read_usize(this + 0x34c8));
        if m == 0 {
            return 0;
        }
        if read_usize(m + 0xc) == clube && read_usize(m + 8) != 0 {
            return m;
        }
    }
}

/// Comparer for `find_human_club_open_invitation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c68b0: lado a lado no cursor da lista (+0xa) e a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_find_human_club_open_invitation(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(IDX_FIND_HUMAN_CLUB_OPEN_INVITATION);
    let lista = read_usize(this + 0x34c8);
    let reg = if lista != 0 { vec![(lista + 0xa, 4)] } else { Vec::new() };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || find_human_club_open_invitation(this));
    PLACAR.registrar(igual && r == n, || format!("find_human_club_open_invitation original={:#x} {:x?} rust={:#x} {:x?}", r, mo, n, mr));
    r
}






/// The other club of a friendly request (0x19-byte record): +0x10, or +0x14 when +0x10 is the requesting club (+0xc).
///
/// [pt] O outro clube do pedido de amistoso (0x19 bytes: +0 dia, +2 ano, +0xc quem pede, +0x10 e +0x14 os dois clubes do
/// jogo, +0x18 a situação): o +0x10, ou o +0x14 quando o +0x10 é o que pede.
pub(crate) unsafe fn other_friendly_club(m: usize) -> usize {
    if read_usize(m + 0xc) == read_usize(m + 0x10) {
        read_usize(m + 0x14)
    } else {
        read_usize(m + 0x10)
    }
}

/// True if the club is run by a human manager (NEWS club_has_human_manager 0x7875e0).
///
/// [pt] O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c]; sem add esp -> thiscall(notícias,
/// clube), ret 4; a cópia Rust é stdcall(clube): o this não é lido).
pub(crate) unsafe fn club_has_human_manager(clube: usize) -> bool {
    let f: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    f(clube) != 0
}

/// Builds the friendly news item (type 6 or 7) with clubs, requester, date and status and sends it to the manager of
/// `para`.
///
/// [pt] A notícia do amistoso (`tipo` 6 ou 7) numa local de 0xe0 bytes: 0x77bbe0 (push 0; push tipo; ecx = a local; sem add
/// esp -> thiscall, ret 8) com os dados 0x785790 (push valor; push k; ecx = a local -> thiscall, ret 8) 0 e 1 = os ids
/// dos clubes do jogo (+0x10, +0x14), 2 = o id de quem pede (+0xc), 3 = o dia (com sinal), 4 = o ano (sem sinal), 5 =
/// a situação; entregue ao técnico do `para` (0x7861e0: push para; push &local; ecx = [0xb7431c]; sem add esp ->
/// thiscall, ret 8).
pub(crate) unsafe fn send_friendly_news(m: usize, tipo: u32, situacao: u32, para: usize) {
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    construir(n, tipo, 0);
    gravar(n, 0, read_usize(read_usize(m + 0x10)) as u32);
    gravar(n, 1, read_usize(read_usize(m + 0x14)) as u32);
    gravar(n, 2, read_usize(read_usize(m + 0xc)) as u32);
    gravar(n, 3, read_i16(m) as i32 as u32);
    gravar(n, 4, core::ptr::read_unaligned((m + 2) as *const u16) as u32);
    gravar(n, 5, situacao);
    entregar(read_usize(G_NEWS), n, para);
}

/// True if 0x5c5f40 decides the request: the other club exists, status is 1 and it is not human-managed (or `forcar`).
///
/// [pt] A 0x5c5f40 decide: o outro clube existe, a situação é 1 e ele não é de técnico humano (0x600a50: push 0; push 0;
/// push 1; push 0; push o; add esp, 0x14 -> cdecl(o, 0, 1, 0, 0)), salvo com `forcar`.
pub(crate) unsafe fn friendly_needs_decision(m: usize, forcar: u32) -> bool {
    let o = other_friendly_club(m);
    if o == 0 || read_u8(m + 0x18) != 1 {
        return false;
    }
    let human_club: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    human_club(o, 0, 1, 0, 0) == 0 || forcar != 0
}

/// Invited club's decision (3 accept, 4 refuse): a factor reduced by different nation/region/continent and adjusted by
/// distance, then compares the clubs' reputations (+0x80).
///
/// [pt] A decisão do convidado (depois da friendly_needs_decision): 3 aceita, 4 recusa. O fator: 1,65 (1,32 quando o
/// outro é o +0x10), × 0,925 com nações diferentes (+0x53), × 0,85 com regiões (+0x76) diferentes, × 0,85 com
/// continentes (+0x71) diferentes; quando o outro é o +0x10 e os dois têm cidade do estádio (+0x69 → +0x38), × (1 +
/// +0x77 de quem pede / ((distância das coordenadas +0x23/+0x2b × 0,55 + 1) × 40000)). Aceita se a reputação (+0x80)
/// de quem pede é ao menos a do outro / fator, ou se a diferença das reputações não passa de fator × 600.
pub(crate) unsafe fn friendly_reply(m: usize, contar: bool) -> u8 {
    let a = read_usize(m + 0xc);
    let o = other_friendly_club(m);
    let (na, no) = (read_usize(a + 0x53), read_usize(o + 0x53));
    let segundo = o == read_usize(m + 0x10);
    let mut f = if segundo { read_f64_at(0x96af38) } else { read_f64_at(0x96af20) };
    if no != na {
        f *= read_f64_at(0x96af30);
    }
    if read_u8(no + 0x76) != read_u8(na + 0x76) {
        f *= read_f64_at(0x969520);
    }
    if read_i32(no + 0x71) != read_i32(na + 0x71) {
        f *= read_f64_at(0x969520);
    }
    if segundo {
        let (fa, fo) = (read_usize(a + 0x69), read_usize(o + 0x69));
        let sa = if fa != 0 { read_usize(fa + 0x38) } else { 0 };
        let so = if sa != 0 && fo != 0 { read_usize(fo + 0x38) } else { 0 };
        if sa != 0 && so != 0 {
            if contar {
                crate::comparacao::ramo("cqa_2 distancia do amistoso");
            }
            let dx = read_f64_at(sa + 0x2b) - read_f64_at(so + 0x2b);
            let dy = read_f64_at(sa + 0x23) - read_f64_at(so + 0x23);
            let i = read_i32(a + 0x77) as f64;
            let v = i / (((dy * dy + dx * dx).sqrt() * read_f64_at(0x96a5d8) + read_f64_at(0x967890)) * read_f64_at(0x96af28)) + read_f64_at(0x967890);
            f *= v; // cqa2: o fator da distância
        }
    }
    let (ro, ra) = (read_i16(o + 0x80) as i32, read_i16(a + 0x80) as i32);
    if contar {
        crate::comparacao::ramo("cqa_2 decisao do amistoso");
    }
    // fcompp/test ah, 1: o C0 vale para "menor" e para "sem ordem" (NaN): os >= abaixo dão o mesmo caminho
    let aceita = ra as f64 >= ro as f64 / f || f * read_f64_at(0x96a690) >= ro.wrapping_sub(ra) as f64; // cqa2: aceita pela reputação
    if aceita { 3 } else { 4 }
}

/// the invited club (without a human manager, or when forced) accepts or refuses by reputation, with the nation,
/// region, continent and stadium city distance factor; tells the requester (news 7); an accepted one goes to the
/// fixture list (0x5aa680) and marks the friendly slot; with 3 or 4 the request is removed (0x5c87b0); otherwise it
/// only leaves the reply list
/// Original: `Friendly::process_friendly_reply` (0x5c5f40, friendly.cpp, thiscall, ret 8).
///
/// [pt] { 0x5c5f40 } (thiscall, ret 8, friendly.cpp, quadro SEH sem destrutor no caminho normal):
/// `Friendly::process_friendly_reply(pedido, forcar)`. Sem quem pede (+0xc): nada. Com o outro clube (+0x10, ou +0x14
/// se o +0x10 é quem pede) na situação 1 e sem técnico humano (0x600a50) ou com `forcar`: sem nação num dos dois, a
/// situação 4 (sobrescrita logo depois); a decisão (friendly_reply) grava 3 ou 4 em +0x18 e a notícia 7 vai a quem pede.
/// Aceito (3): a partida do amistoso numa local de 0x4f bytes (a competição [+4] e o id dela, o estádio do clube +0x10 (+0x69) e o
/// id dele, os clubes +0x10 e +0x14 e os ids, a data do pedido, o ano de hoje, 9 para seleção (0x53b900) ou 1, o horário 1
/// no sábado e no domingo (0x54a3f0 = 5 ou 6) ou 2, as marcas, −1 nos placares) vai ao calendário (0x5aa680: push 1;
/// push &local; ecx = [0xae2a58] -> thiscall, ret 8). Com 3 ou 4: a vaga do amistoso marcado (0x5c6740) fica com +0x18
/// = 3 ou com +0x10 = 0 e +0x18 = 0, e o pedido sai (0x5c87b0 com 0). Noutra situação, o pedido sai da lista das
/// respostas (+0x34d4; 0x68d1f0, o cursor +0xa volta ao começo antes, depois de cada um e no fim).
pub(crate) unsafe extern "thiscall" fn process_friendly_reply(this: usize, m: usize, forcar: u32) {
    let a = read_usize(m + 0xc);
    if a == 0 {
        return;
    }
    let o = other_friendly_club(m);
    if friendly_needs_decision(m, forcar) {
        if read_usize(a + 0x53) == 0 || read_usize(o + 0x53) == 0 {
            *((m + 0x18) as *mut u8) = 4; // como a original (que cai logo depois, lendo a nação nula)
        }
        let s = friendly_reply(m, true);
        *((m + 0x18) as *mut u8) = s;
        send_friendly_news(m, 7, s as u32, a);
    }
    let s = read_u8(m + 0x18);
    if s == 3 {
        crate::comparacao::ramo("cqa_2 amistoso aceito");
        let mut p = [0u8; 0x50];
        let pp = p.as_mut_ptr() as usize;
        let comp = read_usize(this + 4);
        let (c2, c3) = (read_usize(m + 0x10), read_usize(m + 0x14));
        let estadio = read_usize(c2 + 0x69);
        write_usize(pp, read_usize(comp));
        write_usize(pp + 0x14, comp);
        write_usize(pp + 0x18, estadio);
        write_usize(pp + 4, if estadio != 0 { read_usize(estadio) } else { usize::MAX });
        let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
        let sel = is_national_team(c2); // push c2; add esp, 4 -> cdecl
        p[0x2c..0x2e].copy_from_slice(&core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16).to_le_bytes());
        p[0x3a] = if sel != 0 { 9 } else { 1 };
        p[0x40] = 1;
        p[0x41] = 1;
        for k in [0x43usize, 0x44, 0x4b, 0x4c, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x3b, 0x42] {
            p[k] = 0xff;
        }
        write_usize(pp + 8, usize::MAX);
        write_usize(pp + 0x1c, c2); // cqa2: o mandante do amistoso
        write_usize(pp + 0xc, read_usize(c2));
        write_usize(pp + 0x20, c3);
        write_usize(pp + 0x10, read_usize(c3));
        p[0x2a..0x2c].copy_from_slice(&core::ptr::read_unaligned(m as *const u16).to_le_bytes());
        p[0x28..0x2a].copy_from_slice(&core::ptr::read_unaligned((m + 2) as *const u16).to_le_bytes());
        let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_DATE_GET_WEEKDAY);
        p[0x3f] = if semana(m) as i16 == 5 || semana(m) as i16 == 6 { 1 } else { 2 };
        // ponytail: a original faz and 0xf8ff e depois and 0xff99 / or 0x99 na palavra +0x4d de um local sem valor (os
        // bits 11..15 ficam com a sobra de pilha) e não grava +0x24..+0x27, +0x2e, +0x2f, +0x3c: aqui, zero.
        p[0x4d] = 0x99;
        p[0x3d] = 0;
        p[0x3e] = 0;
        let juntar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_FIXTURE_MANAGER_ADD_FIXTURE);
        juntar(read_usize(G_FIXTURE_MANAGER), pp, 1);
    }
    let s = read_u8(m + 0x18);
    if s == 3 || s == 4 {
        let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
        // push +0x10; push +0x14; push ano; push dia; ecx = this; sem add esp -> thiscall, ret 0x10
        // ponytail: o dia e o ano vão em registradores com lixo nos 16 bits altos; a 0x5c6740 só lê as palavras
        let v = vaga(this, read_i16(m) as u16 as u32, read_i16(m + 2) as u16 as u32, read_usize(m + 0x14), read_usize(m + 0x10));
        if v != 0 {
            if s == 3 {
                *((v + 0x18) as *mut u8) = 3;
            } else {
                write_usize(v + 0x10, 0);
                *((v + 0x18) as *mut u8) = 0;
            }
        }
        let tirar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FRIENDLY_REMOVE_FRIENDLY_MATCH);
        tirar(this, m, 0); // push 0; push m; ecx = this -> thiscall, ret 8
        return;
    }
    let l = read_usize(this + 0x34d4);
    if l == 0 {
        return;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    write_usize(l + 0xa, read_usize(l + 2));
    let mut item = proximo(read_usize(this + 0x34d4));
    while item != 0 {
        if read_usize(item) == m {
            tirar(read_usize(this + 0x34d4), item); // push item; ecx = a lista -> thiscall, ret 4
            let l = read_usize(this + 0x34d4);
            write_usize(l + 0xa, read_usize(l + 2));
        }
        item = proximo(read_usize(this + 0x34d4));
    }
    let l = read_usize(this + 0x34d4);
    write_usize(l + 0xa, read_usize(l + 2));
}

/// True if some node of the LLIST points to a record whose first dword is m (cursor untouched).
///
/// [pt] Algum nó da lista (+2 a cabeça, +6 o próximo de cada nó, +0 o dado) guarda `m`, sem mexer no cursor.
pub(crate) unsafe fn list_holds(l: usize, m: usize) -> bool {
    if l == 0 {
        return false;
    }
    let mut no = read_usize(l + 2);
    let mut guarda = 0;
    while no != 0 && guarda < 100_000 {
        if read_usize(no) != 0 && read_usize(read_usize(no)) == m {
            return true;
        }
        no = read_usize(no + 6);
        guarda += 1;
    }
    false
}

/// Comparer for `process_friendly_reply` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c5f40: só a original (a partida vai ao calendário e a notícia ao técnico, que alocam, e o pedido é
/// liberado pela 0x5c87b0). O plano, feito antes: a decisão (friendly_reply) e a vaga do amistoso (0x5c6740). Com 3 ou 4,
/// a vaga fica com +0x18 = 3, ou com +0x10 = 0 e +0x18 = 0; sem decisão, o pedido continua com a mesma situação e sai
/// da lista das respostas (+0x34d4). Sem quem pede ou sem nação num dos dois: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_process_friendly_reply(this: usize, m: usize, forcar: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_PROCESS_FRIENDLY_REPLY);
    CQA2_RESPONDIDO = m;
    let a = if m != 0 { read_usize(m + 0xc) } else { 0 };
    let outro = if m != 0 { other_friendly_club(m) } else { 0 };
    if a == 0 || (outro != 0 && (read_usize(a + 0x53) == 0 || read_usize(outro + 0x53) == 0)) {
        return o(this, m, forcar); // ponytail: sem quem pede ou sem nação, só a original
    }
    let s = if friendly_needs_decision(m, forcar) { friendly_reply(m, false) } else { read_u8(m + 0x18) };
    let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
    let v = if s == 3 || s == 4 { vaga(this, read_i16(m) as u16 as u32, read_i16(m + 2) as u16 as u32, read_usize(m + 0x14), read_usize(m + 0x10)) } else { 0 };
    o(this, m, forcar);
    let ok = if s == 3 || s == 4 {
        // o pedido foi liberado: só a vaga
        v == 0 || if s == 3 { read_u8(v + 0x18) == 3 } else { read_usize(v + 0x10) == 0 && read_u8(v + 0x18) == 0 }
    } else {
        read_u8(m + 0x18) == s && !list_holds(read_usize(this + 0x34d4), m)
    };
    PLACAR.registrar(ok, || format!("process_friendly_reply(pedido {:x}) situação prevista {} vaga {:x}", m, s, v));
}

/// stores the request on the reply list (+0x34d4, created when missing); an invited club with a human manager: status 2
/// and news 6
/// Original: `Friendly::send_friendly_request` (0x5c5570, friendly.cpp, thiscall, ret 4).
///
/// [pt] { 0x5c5570 } (thiscall, ret 4, friendly.cpp, quadro SEH sem destrutor no caminho normal):
/// `Friendly::send_friendly_request(pedido)`: sem a lista das respostas (+0x34d4), cria-a (0x944cf1(0xe): push 0xe;
/// add esp, 4 -> cdecl; 0x68d020: ecx = o bloco -> thiscall, ret 0); sem memória, a caixa (linha 0x982) e nada mais.
/// Guarda o ponteiro do pedido na lista (0x68d0d0: push 4; push &pedido; ecx = a lista -> thiscall, ret 8). Com o outro
/// clube na situação 1 e de técnico humano: situação 2 e, se o +0x10 ou o +0x14 é de técnico humano, a notícia 6 ao
/// outro.
pub(crate) unsafe extern "thiscall" fn send_friendly_request(this: usize, m: usize) {
    let o = other_friendly_club(m);
    if read_usize(this + 0x34d4) == 0 {
        let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
        let bloco = novo(0xe);
        let l = if bloco != 0 {
            let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
            construir(bloco)
        } else {
            0
        };
        write_usize(this + 0x34d4, l);
        if l == 0 {
            return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEND_FRIENDLY_REQUEST)(this, m); // ponytail: sem memória, a caixa da original
        }
    }
    let guardar: unsafe extern "thiscall" fn(usize, *const usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let copia = m;
    guardar(read_usize(this + 0x34d4), &copia, 4);
    if o == 0 || read_u8(m + 0x18) != 1 || !club_has_human_manager(o) {
        return;
    }
    crate::comparacao::ramo("cqa_2 pedido ao humano");
    *((m + 0x18) as *mut u8) = 2;
    if !club_has_human_manager(read_usize(m + 0x10)) && !club_has_human_manager(read_usize(m + 0x14)) {
        return;
    }
    send_friendly_news(m, 6, 2, o);
}

/// Number of nodes in a LLIST (cursor untouched).
///
/// [pt] Quantos nós a lista tem, sem mexer no cursor.
pub(crate) unsafe fn list_len(l: usize) -> usize {
    let mut n = 0;
    let mut no = if l != 0 { read_usize(l + 2) } else { 0 };
    while no != 0 && n < 100_000 {
        n += 1;
        no = read_usize(no + 6);
    }
    n
}

/// O último pedido que passou pelo comparador da 0x5c5f40 (a resposta).
pub(crate) static mut CQA2_RESPONDIDO: usize = 0;

/// Comparer for `send_friendly_request` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5c5570: só a original (o nó da lista e a notícia alocam). O plano, feito antes: a lista das
/// respostas fica com um nó a mais e guarda o pedido; a situação vai a 2 quando o outro clube, na situação 1, é de
/// técnico humano; senão não muda. A notícia 6 ao humano de férias é respondida na hora pelo assistente (0x784100:
/// situação 1 e a 0x5c5f40 com `forcar`, conferida pelo comparador dela): então o pedido já saiu da lista (a lista fica
/// com os nós de antes) e pode ter sido liberado (a situação não se lê).
pub(crate) unsafe extern "thiscall" fn comparar_send_friendly_request(this: usize, m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SEND_FRIENDLY_REQUEST);
    let outro = other_friendly_club(m);
    let antes = list_len(read_usize(this + 0x34d4));
    let s = if outro != 0 && read_u8(m + 0x18) == 1 && club_has_human_manager(outro) { 2 } else { read_u8(m + 0x18) };
    CQA2_RESPONDIDO = 0;
    o(this, m);
    let l = read_usize(this + 0x34d4);
    let respondido = s == 2 && CQA2_RESPONDIDO == m;
    if respondido {
        crate::comparacao::ramo("cqa_2 pedido respondido pelo assistente");
    }
    let ok = l == 0 || if respondido {
        list_len(l) == antes && !list_holds(l, m)
    } else {
        list_len(l) == antes + 1 && list_holds(l, m) && read_u8(m + 0x18) == s
    };
    PLACAR.registrar(ok, || format!("send_friendly_request(pedido {:x}) nós {} -> {} situação {} prevista {}", m, antes, list_len(l), read_u8(m + 0x18), s));
}

/// Nations in the nation table that have a continent (+0x71), in table order.
///
/// [pt] As nações com continente (+0x71) da tabela [0xae23a8] (0x122 bytes cada, [0xae2350] delas), na ordem da tabela.
pub(crate) unsafe fn nations_with_continent() -> Vec<usize> {
    let mut v = Vec::new();
    let t = read_usize(G_NATION_TABLE);
    let mut i = 0;
    while i < read_i32(0xae2350) {
        let n = t.wrapping_add(i as usize * 0x122);
        if read_usize(n + 0x71) != 0 {
            v.push(n);
        }
        i += 1;
    }
    v
}

/// CRT qsort of n pointers with comparator 0x5cc5f0 (highest reputation first).
///
/// [pt] O qsort da CRT (0x945333: 4 pushes; add esp, 0x10 -> cdecl(base, n, 4, ordem)) com a ordem 0x5cc5f0 (reputação,
/// a maior primeiro).
pub(crate) unsafe fn sort_by_reputation(base: usize, n: u32) {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    qsort(base, n, 4, 0x5cc5f0);
}

/// allocates the list (+0x34fa) of nations with a continent, counts it in +0x34fe and sorts it by reputation (0x5cc5f0)
/// Original: `Friendly::build_nation_list` (0x5ca3e0, friendly.cpp, thiscall, ret 0).
///
/// [pt] { 0x5ca3e0 } (thiscall, ret 0, friendly.cpp): `Friendly::build_nation_list()`: +0x34fe = 0; aloca [0xae2350] × 4
/// bytes (0x944e46: push n; add esp, 4 -> cdecl) em +0x34fa (sem memória, a caixa, linha 0x15af); põe nele as nações
/// com continente (+0x71), contando em +0x34fe, e ordena pela reputação (0x945333 com 0x5cc5f0).
pub(crate) unsafe extern "thiscall" fn build_nation_list(this: usize) {
    core::ptr::write_unaligned((this + 0x34fe) as *mut i32, 0);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let p = malloc((read_i32(0xae2350) as u32).wrapping_mul(4));
    write_usize(this + 0x34fa, p);
    if p == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_NATION_LIST)(this); // ponytail: sem memória, a caixa da original
    }
    for n in nations_with_continent() {
        let k = read_i32(this + 0x34fe);
        write_usize(read_usize(this + 0x34fa).wrapping_add((k as isize * 4) as usize), n);
        core::ptr::write_unaligned((this + 0x34fe) as *mut i32, k + 1);
    }
    sort_by_reputation(read_usize(this + 0x34fa), read_i32(this + 0x34fe) as u32);
}

/// Comparer for `build_nation_list` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5ca3e0: só a original (aloca a lista). O plano, feito antes numa lista local com o mesmo qsort e a
/// mesma ordem: a conta (+0x34fe) e as nações, na ordem.
pub(crate) unsafe extern "thiscall" fn comparar_build_nation_list(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_BUILD_NATION_LIST);
    let mut plano = nations_with_continent();
    sort_by_reputation(plano.as_mut_ptr() as usize, plano.len() as u32);
    o(this);
    let p = read_usize(this + 0x34fa);
    let n = read_i32(this + 0x34fe);
    let ok = p == 0 || (n as usize == plano.len() && (0..plano.len()).all(|k| read_usize(p + 4 * k) == plano[k]));
    PLACAR.registrar(ok, || format!("build_nation_list: conta {} prevista {}", n, plano.len()));
}



/// Club of the player's contract (stored in G_CONTRACT_LOOKUP_RESULT), or 0 without contract or for contract type 5
/// (+0x35 & 0x3f).
///
/// [pt] O clube do contrato do jogador (0x4cf5a0, guardado em [0xae1080]; registros de 0x245 bytes), ou 0 sem contrato ou
/// com o contrato do tipo 5 (+0x35 & 0x3f).
pub(crate) unsafe fn contract_club_except_type5(p: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c == 0 || read_u8(c + 0x35) & 0x3f == 5 {
        0
    } else {
        read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE))
    }
}

/// Body of Staff_Records::add_month_award_record (0x86c8c0): history entry 0x33, award count for a manager, contract
/// renewal and thank-you news for a player; `gravar` false skips the history file entry (comparison copy).
///
/// [pt] O corpo da 0x86c8c0; sem `gravar` (a cópia pelo comparador), sem a entrada no arquivo do histórico.
pub(crate) unsafe fn add_month_award_record_body(this: usize, p: usize, clube: usize, premio: usize, a4: u32, a5: u32, gravar: bool) {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x33;
    put_i32(&mut e, 8, if clube != 0 { read_usize(clube) as i32 } else { -1 });
    put_i32(&mut e, 0xd, read_usize(premio) as i32);
    put_i32(&mut e, 0x11, (a4 & 0xff) as i32);
    put_i32(&mut e, 0x15, (a5 & 0xffff) as i32);
    append_person_history_by_id(gravar, this, read_usize(p), &e);
    let (t3d, t28) = (read_u8(p + 0x3d), read_u8(p + 0x28));
    if t3d == 5 || t3d == 0xc || t28 == 5 || t28 == 0xc {
        let r = staff_records_manager_record(this, p);
        if r != 0 {
            crate::comparacao::ramo("cpv_13 prêmio do técnico");
            let a = (r + 0x9e) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1)); // cpv13: conta o prêmio
        }
        return;
    }
    if read_usize(p + 0x61) == 0 {
        return;
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_usize(G_CONTRACT_MANAGER), p);
    *(G_CONTRACT_LOOKUP_RESULT as *mut usize) = c;
    if c == 0 || read_u8(c + 0x35) & 0x3f == 5 {
        return;
    }
    if read_usize(G_CLUB_TABLE).wrapping_add(read_usize(c + 4).wrapping_mul(club::SIZE)) == 0 {
        return;
    }
    let do_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let k = do_clube(read_usize(G_CONTRACT_MANAGER), p, contract_club_except_type5(p));
    if k == 0 {
        return;
    }
    let gt = read_usize(G_TRANSFER_MANAGER);
    if gt != 0 {
        let dias: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
        if dias(G_CURRENT_DATE, read_usize(k + 0x25), read_usize(k + 0x29)) > 0x12c {
            let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
            let x = entrada(read_usize(read_usize(G_TRANSFER_MANAGER) + 0x856), contract_club_except_type5(p), p, 0, 0);
            if x != 0 && read_usize(x + 0xf) == read_usize(p) {
                crate::comparacao::ramo("cpv_13 renovação");
                offer_renewal(p);
            }
        }
    }
    if read_usize(k + 0x45) != 0 {
        return;
    }
    let v59 = read_u8(p + 0x59) as i8;
    if v59 <= 0xf { // cpv13: o +0x59
        return;
    }
    if read_u8(p + 0x5b) as i8 <= 0xa { // cpv13: o +0x5b
        return;
    }
    crate::comparacao::ramo("cpv_13 sorteio do agradecimento");
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    if sorteio(v59 as i32) <= 5 {
        return;
    }
    let agradece: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_POST_PLAYER_CREDITS_MANAGER_FOR_AWARD);
    agradece(p, clube, premio);
}


/// the club's reputation tier 1..5 (2000, 4000, 6000 and 8000)
/// Original: `friendly_club_reputation_tier` (0x5c52d0, friendly.cpp, stdcall, ret 4).
///
/// [pt] { 0x5c52d0 } (stdcall, ret 4, friendly.cpp): `friendly_club_reputation_tier(clube)`: a faixa da reputação (+0x80):
/// 5 a partir de 8000, 4 de 6000, 3 de 4000, 2 de 2000, senão 1 (só o al vale).
pub(crate) unsafe extern "stdcall" fn friendly_club_reputation_tier(clube: usize) -> u8 {
    let r = core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16);
    if r >= 0x1f40 {
        5
    } else if r >= 0x1770 {
        4
    } else if r >= 0xfa0 {
        3
    } else {
        1 + (r >= 0x7d0) as u8
    }
}

/// the rest days around a friendly by kind: 2 for kinds 1 and 10, 3 for kind 2, else 1
/// Original: `friendly_get_rest_days_for_kind` (0x5c74d0, friendly.cpp, stdcall, ret 4).
///
/// [pt] { 0x5c74d0 } (stdcall, ret 4, friendly.cpp): `friendly_get_rest_days_for_kind(tipo)`: a folga em volta do amistoso:
/// 2 nos tipos 1 e 10, 3 no 2, 1 nos 0, 3, 4 e 11. Os tipos 5..9 e fora de 0..11: a original (caixa "Error" e 1).
pub(crate) unsafe extern "stdcall" fn friendly_get_rest_days_for_kind(tipo: u32) -> u8 {
    match tipo as u8 as i8 {
        1 | 10 => 2,
        2 => 3,
        0 | 3 | 4 | 11 => 1,
        _ => original_fn::<unsafe extern "stdcall" fn(u32) -> u8>(IDX_FRIENDLY_GET_REST_DAYS_FOR_KIND)(tipo),
    }
}


/// Comparador: pura.
pub(crate) unsafe extern "stdcall" fn comparar_friendly_club_reputation_tier(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u8>(IDX_FRIENDLY_CLUB_REPUTATION_TIER)(clube);
    let n = friendly_club_reputation_tier(clube);
    PLACAR.registrar(r == n, || format!("friendly_club_reputation_tier({:x}) original={} rust={}", clube, r, n));
    r
}

/// Comparer for `friendly_get_rest_days_for_kind` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura; os tipos da caixa, só a original.
pub(crate) unsafe extern "stdcall" fn comparar_friendly_get_rest_days_for_kind(tipo: u32) -> u8 {
    let r = original_fn::<unsafe extern "stdcall" fn(u32) -> u8>(IDX_FRIENDLY_GET_REST_DAYS_FOR_KIND)(tipo);
    if !matches!(tipo as u8 as i8, 0..=4 | 10 | 11) {
        return r;
    }
    let n = friendly_get_rest_days_for_kind(tipo);
    PLACAR.registrar(r == n, || format!("friendly_get_rest_days_for_kind({}) original={} rust={}", tipo as u8 as i8, r, n));
    r
}




/// True if date a is after date b (year unsigned, then day signed).
///
/// [pt] A data `a` é depois da `b` (+2 o ano, sem sinal; +0 o dia, com sinal), como as comparações da original.
pub(crate) unsafe fn friendly_date_after(a: usize, b: usize) -> bool {
    let (aa, ab) = (read_u16(a + 2), read_u16(b + 2));
    aa > ab || (aa == ab && read_u16(a) as i16 > read_u16(b) as i16)
}

/// on the pre-season days plans the AI friendlies and cancels old tournaments; every day creates the planned
/// friendlies, answers requests and expires tournaments, requests and invitations
/// Original: `Friendly::update_for_date` (0x5c2460, friendly.cpp, thiscall, ret 8).
///
/// [pt] { 0x5c2460 } (thiscall, ret 8, friendly.cpp): `Friendly::update_for_date(hoje, ativo)`: com `ativo`, nos dois dias
/// da pré-temporada (+0x34ea, +0x34e8) e com o início do jogo [0xdd6ec8] até hoje + 8 semanas (0x54c770; depois disso,
/// nada mais no dia), guarda hoje em +0xa7, planeja a pré-temporada da IA (0x5c27a0) e cancela (0x5c8260) os torneios
/// em uso (+0x65) com data anterior a um ano atrás (0x54cab0, 0x456770). Todo dia: cria os amistosos planejados
/// (0x5c2ae0), atualiza os amistosos das seleções (0x5cbb00), tira e responde as respostas dos torneios (+0x34d0,
/// 0x5c5820) e responde as dos amistosos (+0x34d4, 0x5c5f40), tira os pedidos sem clubes ou vencidos (+0x34c8), cancela
/// os torneios fora de uso cuja data − 2 dias (0x54c8e0) já passou e zera os convites vencidos (+0x34cc, 10 de 0x19
/// bytes por item, a data em +9, a marca em +0x15). Devolve 1.
pub(crate) unsafe extern "thiscall" fn friendly_update_for_date(this: usize, hoje: usize, ativo: u32) -> u32 {
    if ativo == 0 {
        return 1;
    }
    let semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let menos_semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let anterior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let cancelar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FRIENDLY_CANCEL_TOURNAMENT);
    let planejar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FRIENDLY_PLAN_AI_PRESEASON_FRIENDLIES);
    let criar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FRIENDLY_CREATE_DUE_PLANNED_FRIENDLIES);
    let selecoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FRIENDLY_UPDATE_INTERNATIONAL_FRIENDLIES);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let resposta_do_torneio: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(FN_FRIENDLY_PROCESS_TOURNAMENT_REPLIES);
    let resposta_do_amistoso: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_FRIENDLY_PROCESS_FRIENDLY_REPLY);
    let dia = read_u16(hoje) as i16 as i32;
    if dia == read_u16(this + 0x34ea) as i32 || dia == read_u16(this + 0x34e8) as i32 {
        let mut t = [0u8; 8];
        let limit = semanas(hoje, &mut t, 8);
        if friendly_date_after(G_GAME_START_DATE, limit) {
            return 1;
        }
        copiar(this + 0xa7, &mut t, hoje);
        crate::comparacao::ramo("cpt_11 planejou a pré-temporada");
        planejar(this); // cpt11: planejar
        let mut ha_um_ano = [0u8; 8];
        menos_semanas(G_CURRENT_DATE, &mut ha_um_ano, 0x34);
        let lo = u32::from_le_bytes([ha_um_ano[0], ha_um_ano[1], ha_um_ano[2], ha_um_ano[3]]);
        let hi = u32::from_le_bytes([ha_um_ano[4], ha_um_ano[5], ha_um_ano[6], ha_um_ano[7]]);
        for k in 0..0x7f {
            let e = this + 0xb1 + k * 0x69;
            if read_u32_as_usize(e) != 0 && anterior(e + 4, lo, hi) != 0 && read_u32_as_usize(e + 0x65) != 0 {
                cancelar(this, e, 0);
            }
        }
    }
    criar(this);
    selecoes(this);
    let l = read_u32_as_usize(this + 0x34d0);
    if l != 0 {
        let conta = read_u16(l) as i16;
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        if conta > 0 {
            let mut n = conta as i32;
            loop {
                let item = proximo(read_u32_as_usize(this + 0x34d0));
                if item != 0 {
                    let r = read_u32_as_usize(item);
                    tirar(read_u32_as_usize(this + 0x34d0), item);
                    resposta_do_torneio(this, r, 1, 0);
                }
                let l = read_u32_as_usize(this + 0x34d0);
                n -= 1;
                write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
                if n == 0 {
                    break;
                }
            }
        }
    }
    let l = read_u32_as_usize(this + 0x34d4);
    if l != 0 {
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        let mut item = proximo(read_u32_as_usize(this + 0x34d4));
        while item != 0 {
            resposta_do_amistoso(this, read_u32_as_usize(item), 0);
            item = proximo(read_u32_as_usize(this + 0x34d4));
        }
    }
    let l = read_u32_as_usize(this + 0x34c8);
    if l != 0 {
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        let mut p = proximo(read_u32_as_usize(this + 0x34c8));
        while p != 0 {
            if read_u32_as_usize(p + 0x10) == 0 || read_u32_as_usize(p + 0x14) == 0 {
                tirar(read_u32_as_usize(this + 0x34c8), p);
            }
            // ponytail: como a original, a data do pedido é olhada mesmo depois de tirá-lo (o bloco já tirado)
            if friendly_date_after(hoje, p) {
                tirar(read_u32_as_usize(this + 0x34c8), p);
            }
            p = proximo(read_u32_as_usize(this + 0x34c8));
        }
    }
    for k in 0..0x7f {
        let e = this + 0xb1 + k * 0x69;
        if read_u32_as_usize(e) != 0 && read_u32_as_usize(e + 0x65) == 0 {
            let mut t = [0u8; 8];
            let fim = menos_dias(e + 4, &mut t, 2);
            if friendly_date_after(hoje, fim) {
                cancelar(this, e, 0);
            }
        }
    }
    let l = read_u32_as_usize(this + 0x34cc);
    if l != 0 {
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        let mut item = proximo(read_u32_as_usize(this + 0x34cc));
        while item != 0 {
            for k in 0..10 {
                let s = item + 0x15 + k * 0x19;
                let mut t = [0u8; 8];
                let fim = menos_dias(s - 0xc, &mut t, 2);
                if friendly_date_after(hoje, fim) {
                    write_u32_from_usize(s, 0);
                }
            }
            item = proximo(read_u32_as_usize(this + 0x34cc));
        }
    }
    // ponytail: a original faz `mov al, 1` e deixa sobra em cima; quem chama testa o byte
    1
}

/// builds one candidate per club with a nation (band = reputation / 500), sorts by band and by nation, sets the windows
/// and plans tournaments, tours and friendlies
/// Original: `Friendly::plan_ai_preseason_friendlies` (0x5c27a0, friendly.cpp, thiscall, ret 0).
///
/// [pt] { 0x5c27a0 } (thiscall, ret 0, friendly.cpp): `Friendly::plan_ai_preseason_friendlies()`: monta um candidato de
/// 0x16 bytes (+0 o id do clube, +4 zero, +5 a faixa = reputação +0x80 / 500, +6 o id da nação, +0xa e +0x12 listas
/// nulas, +0xe/+0x10 a janela −1) para cada clube ([0xae23b4], 0x245 bytes, [0xae235c] deles) que não é seleção
/// (0x53b900) e tem nação; ordena os candidatos pela faixa (qsort 0x945333 com 0x5c5270) e um vetor de ponteiros pela
/// nação e faixa (0x5c5290); põe as janelas (0x5c2d40), planeja torneios e excursões (0x5c3040) e os amistosos
/// (0x5c4770); libera as listas dos candidatos (0x68d040 + 0x944c94) e os dois vetores (0x9452ca). Sem memória para os
/// candidatos: a caixa (linha 0x18a).
pub(crate) unsafe extern "thiscall" fn plan_ai_preseason_friendlies(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let janelas: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(FN_FRIENDLY_SET_PRESEASON_WINDOWS);
    let torneios: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(FN_FRIENDLY_PLAN_PRESEASON_TOURNAMENTS_AND_TOURS);
    let amistosos: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(FN_FRIENDLY_PLAN_CLUB_FRIENDLIES);
    let fim_da_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let v = alocar((read_u32_as_usize(G_CLUB_COUNT) as u32).wrapping_mul(0x16));
    if v == 0 {
        return show_error_at(0x9cd0a0, 0x18a);
    }
    let mut n: i16 = 0;
    let mut clube = read_u32_as_usize(G_CLUB_TABLE);
    let mut i: i32 = 0;
    if read_u32_as_usize(G_CLUB_COUNT) as i32 > 0 {
        loop {
            if is_national_team(clube) == 0 && read_u32_as_usize(clube + club::NATION) != 0 {
                let r = v + n as usize * 0x16;
                let rep = read_u16(clube + club::REPUTATION) as i16 as i32;
                crate::comparacao::ramo("cpt_11 candidato da pré-temporada");
                let band = (rep / 500) as u8; // cpt11: a faixa
                write_u32_from_usize(r, read_u32_as_usize(clube));
                *((r + 4) as *mut u8) = 0;
                *((r + 5) as *mut u8) = band;
                write_u32_from_usize(r + 6, read_u32_as_usize(read_u32_as_usize(clube + club::NATION)));
                write_u32_from_usize(r + 0xa, 0);
                write_u32_from_usize(r + 0x12, 0);
                core::ptr::write_unaligned((r + 0xe) as *mut u16, 0xffff);
                core::ptr::write_unaligned((r + 0x10) as *mut u16, 0xffff);
                n += 1;
            }
            clube += 0x245;
            i += 1;
            if i as i16 as i32 >= read_u32_as_usize(G_CLUB_COUNT) as i32 {
                break;
            }
        }
    }
    let ns = n as i32;
    qsort(v, ns as u32, 0x16, 0x5c5270);
    let p = alocar((ns as u32).wrapping_mul(4)); // a original não confere
    for k in 0..ns.max(0) as usize {
        write_u32_from_usize(p + k * 4, v + k * 0x16);
    }
    qsort(p, ns as u32, 4, 0x5c5290);
    janelas(this, p, ns);
    torneios(this, v, p, ns);
    amistosos(this, v, p, ns);
    for k in 0..ns.max(0) as usize {
        for l in [read_u32_as_usize(v + k * 0x16 + 0xa), read_u32_as_usize(v + k * 0x16 + 0x12)] {
            if l != 0 {
                fim_da_lista(l);
                apagar(l);
            }
        }
    }
    liberar(v);
    liberar(p);
}

/// Work of 0x5c2d40 over candidate pointers: for each in-use league nation, computes the pre-season friendly window (18
/// days ending a week before the league start) and writes it on that nation's candidates; with gravar false only
/// returns the plan.
///
/// [pt] O trabalho da 0x5c2d40 sobre os `n` ponteiros para candidatos `ptrs` (ordenados pela nação e faixa): para cada uma
/// das 35 entradas de [0xb63d60] (0x48 bytes: +0 a nação, +0x26 a liga, +0x2a o início) com nação em uso (+0x11c):
/// o fim da janela é o início − 1 semana (0x54cab0) em dias desde hoje e o começo, 18 dias antes (no mínimo 3); acha
/// (bsearch 0x946464 com 0x5c5290) o primeiro candidato da nação e grava a janela (+0xe, +0x10) nos seguintes enquanto
/// o clube for da nação (sem o último do vetor, como a original); o primeiro e o último clube da tabela da liga (+0xa7
/// da competição de tipo 1) ganham uma lista nova (0x944cf1 + 0x68d020) em +0x12. Com `gravar` falso só monta a lista
/// (candidato, começo, fim, lista nova). A original calcula a data de início da liga (e + dias, tipo 2) e não usa.
pub(crate) unsafe fn set_friendly_windows(ptrs: usize, n: i32, gravar: bool) -> Vec<(usize, i16, i16, bool)> {
    let menos_semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let data_nova: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copiar: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let bsearch: unsafe extern "C" fn(*const usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let mut plano = Vec::new();
    for k in 0..0x23 {
        let ent = 0xb63d60 + k * 0x48;
        let nac = read_u32_as_usize(ent);
        if nac == 0 || *((nac + 0x11c) as *const u8) == 0 {
            continue;
        }
        let (mut prim, mut ult) = (u32::MAX as usize, u32::MAX as usize);
        let lg = read_u32_as_usize(ent + 0x26);
        let comp = if lg != 0 { read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + read_u32_as_usize(lg) * 4) } else { 0 };
        if comp != 0 && *((comp + 0x43) as *const u8) == 1 {
            if gravar {
                // as datas de início da liga que a original monta e não usa (só locais)
                let (mut t1, mut t2, mut t3, mut t4, mut t5, mut t6) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
                let x = read_u32_as_usize(comp + 0xa3);
                let ano = *((x + 5) as *const i8) as i16 as u16 as u32;
                let r = data_nova(&mut t1, read_u16(x + 3) as u32, ano);
                copiar(&mut t2, &mut t3, r);
                if *((x + 0x21) as *const u8) == 2 {
                    let r = data_nova(&mut t4, read_u16(x + 3) as u32, ano);
                    let r = mais_dias(r, &mut t5, *((x + 0x22) as *const i8) as i16 as u16 as u32);
                    copiar(&mut t6, &mut t3, r);
                }
            }
            let t = read_u32_as_usize(comp + 0xa7);
            let a = read_u32_as_usize(t);
            prim = if a != 0 { read_u32_as_usize(a) } else { u32::MAX as usize };
            let b = read_u32_as_usize(t + 6);
            ult = if b != 0 { read_u32_as_usize(b) } else { u32::MAX as usize };
        }
        let mut l = [0u8; 8];
        menos_semanas(ent + 0x2a, &mut l, 1);
        let dano = u16::from_le_bytes([l[2], l[3]]).wrapping_sub(read_u16(G_CURRENT_DATE_YEAR));
        let fim = (dano as u32).wrapping_mul(365).wrapping_sub(read_u32_as_usize(G_CURRENT_DATE) as u32)
            .wrapping_add(u32::from_le_bytes([l[0], l[1], l[2], l[3]])) as u16 as i16;
        let mut ini = fim.wrapping_sub(18); // cpt11: o início da janela
        if ini < 3 {
            ini = 3;
        }
        let mut chave = [0u8; 0x16];
        chave[5] = 0xff;
        chave[6..10].copy_from_slice(&(read_u32_as_usize(nac) as u32).to_le_bytes());
        let pchave = chave.as_ptr() as usize;
        let mut q = bsearch(&pchave, ptrs, n as u32, 4, 0x5c5290);
        if q == 0 {
            continue;
        }
        while q > ptrs {
            q -= 4;
            if read_u32_as_usize(read_u32_as_usize(q) + 6) != read_u32_as_usize(nac) {
                q += 4;
                break;
            }
        }
        let ate = ptrs.wrapping_add((n as usize).wrapping_mul(4)).wrapping_sub(4);
        while q < ate {
            let c = read_u32_as_usize(q);
            let id = read_u32_as_usize(c);
            if read_u32_as_usize(read_u32_as_usize(G_CLUB_TABLE) + id * club::SIZE + 0x53) != nac {
                break;
            }
            let nova = id == prim || id == ult;
            if gravar {
                if nova {
                    let m = novo(0xe);
                    write_u32_from_usize(c + 0x12, if m != 0 { lista_nova(m) } else { 0 });
                }
                crate::comparacao::ramo("cpt_11 janela gravada");
                core::ptr::write_unaligned((c + 0xe) as *mut i16, ini);
                core::ptr::write_unaligned((c + 0x10) as *mut i16, fim);
            }
            plano.push((c, ini, fim, nova));
            q += 4;
        }
    }
    plano
}

/// for each nation in use, writes to its candidates the window up to the season start − 1 week (18 days before, at
/// least 3) and gives a date list to the league's first and last club
/// Original: `Friendly::set_preseason_windows` (0x5c2d40, friendly.cpp, thiscall, ret 8).
///
/// [pt] { 0x5c2d40 } (thiscall, ret 8, friendly.cpp): `Friendly::set_preseason_windows(candidatos, n)`: a janela da
/// pré-temporada de cada nação nos candidatos dela (ver `set_friendly_windows`); o `this` não é lido. A original constrói e
/// destrói duas datas locais que não usa (0x9463af / 0x9462bb com 0x54a110 / 0x5a53c0).
pub(crate) unsafe extern "thiscall" fn set_preseason_windows(_this: usize, ptrs: usize, n: i32) {
    set_friendly_windows(ptrs, n, true);
}

/// Comparer for `friendly_update_for_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: anda o calendário e planeja (aloca, sorteia): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_friendly_update_for_date(this: usize, hoje: usize, ativo: u32) -> u32 {
    // ponytail: cancela torneios, tira pedidos e cria amistosos (não roda duas vezes); a cópia se prova pela carreira
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_FRIENDLY_UPDATE_FOR_DATE)(this, hoje, ativo)
}

/// Comparer for `plan_ai_preseason_friendlies` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: monta e planeja os amistosos da IA (aloca, sorteia): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_plan_ai_preseason_friendlies(this: usize) {
    // ponytail: os torneios, as excursões e os amistosos sorteiam e alocam; a cópia se prova pela carreira
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_PLAN_AI_PRESEASON_FRIENDLIES)(this)
}

/// Comparer for `set_preseason_windows` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: grava as janelas e cria listas (não roda duas vezes): o plano antes (sem gravar), a original grava;
/// cada candidato planejado tem de ter a janela e, quando é a ponta da tabela, uma lista em +0x12.
pub(crate) unsafe extern "thiscall" fn comparar_set_preseason_windows(this: usize, ptrs: usize, n: i32) {
    let plano = set_friendly_windows(ptrs, n, false);
    original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_SET_PRESEASON_WINDOWS)(this, ptrs, n);
    let erros: Vec<String> = plano.iter()
        .filter(|&&(c, ini, fim, nova)| {
            core::ptr::read_unaligned((c + 0xe) as *const i16) != ini || core::ptr::read_unaligned((c + 0x10) as *const i16) != fim
                || (nova && read_u32_as_usize(c + 0x12) == 0)
        })
        .map(|&(c, ini, fim, nova)| format!("candidato {:x}: esperado {}..{} lista {} obtido {}..{} lista {:x}", c, ini, fim, nova,
            core::ptr::read_unaligned((c + 0xe) as *const i16), core::ptr::read_unaligned((c + 0x10) as *const i16), read_u32_as_usize(c + 0x12)))
        .collect();
    PLACAR.registrar(erros.is_empty(), || format!("set_preseason_windows(n {}) {} candidatos: {}", n, plano.len(), erros.join("; ")));
}



/// Compares dates a and b: -1, 0 or 1 (year unsigned, then day signed).
///
/// [pt] A data `a` (+2 o ano sem sinal, +0 o dia com sinal) comparada à `b`: -1, 0 ou 1.
pub(crate) unsafe fn compare_dates(a: usize, b: usize) -> i32 {
    let (aa, ab) = (read_u16(a + 2), read_u16(b + 2));
    if aa != ab {
        return if aa < ab { -1 } else { 1 };
    }
    let (da, db) = (read_u16(a) as i16, read_u16(b) as i16);
    if da < db { -1 } else if da > db { 1 } else { 0 }
}

/// Allocates and constructs a new 0xe-byte LLIST, or 0 on out of memory.
///
/// [pt] Uma lista nova (0x944cf1 de 0xe bytes + 0x68d020) ou 0 sem memória.
pub(crate) unsafe fn alloc_list() -> usize {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let m = novo(0xe);
    if m != 0 { lista_nova(m) } else { 0 }
}

/// trims the window by the first opponent, draws the start (up to 5 tries) and books one friendly per opponent, 2 days
/// apart, storing the dates in both lists
/// Original: `Friendly::schedule_tour_matches` (0x5c4040, friendly.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5c4040 } (thiscall, ret 0xc, friendly.cpp): `Friendly::schedule_tour_matches(candidato, adversários, n)`: a
/// janela é a do candidato (+0xe..+0x10) cortada pela do primeiro adversário (se ele tem uma) e encurtada em 2n + 1
/// dias; sem espaço, 0. Até 5 tentativas: o começo anda um sorteio (0x90cfc0) da largura (o começo fica andado entre
/// as tentativas, como na original), o primeiro jogo é o início da temporada (+0xa7) + começo e o último, esse + 2n + 1
/// (0x54c5b0); uma data já guardada na lista do candidato (+0x12) depois do primeiro − 2 dias ou antes do último + 2
/// estraga a tentativa (com qualquer data guardada, todas falham; na quinta, segue assim mesmo). Depois, para cada
/// adversário, com a data livre para ele (0x5c6b70) e o amistoso montado (0x5c6f00, o clube em casa), manda o pedido
/// (0x5c5570) e guarda a data na lista dos dois (criadas se faltam; 0x68d0d0); a data anda 2 dias a cada adversário.
/// Devolve 1.
pub(crate) unsafe extern "thiscall" fn schedule_tour_matches(this: usize, cand: usize, adv: usize, n: u32) -> u32 {
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let copiar: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let data_padrao: unsafe extern "thiscall" fn(*mut [u8; 8]) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let livre: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize, u32) -> usize = core::mem::transmute(FN_FRIENDLY_IS_CLUB_FREE_ON_DATE);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, *mut [u8; 8], usize, u32) -> usize = core::mem::transmute(FN_FRIENDLY_ADD_FRIENDLY_MATCH);
    let pedir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FRIENDLY_SEND_FRIENDLY_REQUEST);
    let guardar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let n = n as u8 as i8;
    let mut ini = read_u16(cand + 0xe) as i16;
    let mut fim = read_u16(cand + 0x10) as i16;
    let a0 = read_u32_as_usize(adv);
    let x = read_u16(a0 + 0xe) as i16;
    if x != -1 {
        if x > ini {
            ini = x;
        }
        let y = read_u16(a0 + 0x10) as i16;
        if y < fim {
            fim = y;
        }
    }
    let largura = (n as u8).wrapping_mul(2).wrapping_add(1) as i8 as i16; // cpt12: a largura da excursão
    fim = fim.wrapping_sub(largura);
    if ini >= fim {
        return 0;
    }
    crate::comparacao::ramo("cpt_12 janela da excursão");
    let (mut d1, mut d2, mut t, mut u) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    data_padrao(&mut d1);
    data_padrao(&mut d2);
    let mut tentativas: i8 = 0;
    while tentativas < 5 {
        let mut ok = true;
        ini = ini.wrapping_add(sortear(fim as i32 - ini as i32) as i16);
        let r = mais_dias(this + 0xa7, &mut t, ini as u16 as u32);
        copiar(&mut d1, &mut u, r);
        let r = mais_dias(d1.as_ptr() as usize, &mut t, largura as u16 as u32);
        copiar(&mut d2, &mut u, r);
        let l = read_u32_as_usize(cand + 0x12);
        if l == 0 {
            break;
        }
        core::ptr::write_unaligned((l + 0xa) as *mut u32, read_u32_as_usize(l + 2) as u32);
        let mut item = proximo(read_u32_as_usize(cand + 0x12));
        while item != 0 {
            let antes = menos_dias(d1.as_ptr() as usize, &mut t, 2);
            let estraga = if compare_dates(antes, item) < 0 {
                true
            } else {
                let depois = mais_dias(d2.as_ptr() as usize, &mut u, 2);
                compare_dates(depois, item) > 0
            };
            if estraga {
                ok = false;
                break;
            }
            item = proximo(read_u32_as_usize(cand + 0x12));
        }
        tentativas += 1;
        if ok {
            break;
        }
    }
    let clube = read_u32_as_usize(G_CLUB_TABLE) + read_u32_as_usize(cand) * club::SIZE;
    let mut p = adv;
    for k in 0..(n as i32).max(0) {
        let outro = read_u32_as_usize(G_CLUB_TABLE) + read_u32_as_usize(read_u32_as_usize(p)) * club::SIZE;
        if livre(this, &mut d1, outro, 0) != 0 {
            let m = acrescentar(this, clube, &mut d1, outro, 0); // cpt12: o clube em casa
            if m != 0 {
                crate::comparacao::ramo("cpt_12 jogo da excursão marcado");
                pedir(this, m);
                if read_u32_as_usize(cand + 0x12) == 0 {
                    let l = alloc_list();
                    core::ptr::write_unaligned((cand + 0x12) as *mut u32, l as u32);
                }
                if read_u32_as_usize(read_u32_as_usize(p) + 0x12) == 0 {
                    let l = alloc_list();
                    core::ptr::write_unaligned((read_u32_as_usize(p) + 0x12) as *mut u32, l as u32);
                }
                guardar(read_u32_as_usize(cand + 0x12), &mut d1, 8);
                guardar(read_u32_as_usize(read_u32_as_usize(p) + 0x12), &mut d1, 8);
            }
        }
        p += 4;
        if k + 1 < n as i32 {
            crate::comparacao::ramo("cpt_12 data do jogo seguinte");
        }
        let r = mais_dias(d1.as_ptr() as usize, &mut t, 2); // cpt12: o próximo jogo
        copiar(&mut d1, &mut u, r);
    }
    1
}

/// candidate comparator by band (+5), highest first
/// Original: `compare_friendly_candidates_by_band` (0x5c5270, friendly.cpp, cdecl).
///
/// [pt] { 0x5c5270 } (cdecl, friendly.cpp): o comparador do qsort dos candidatos do amistoso pela faixa (+5, com sinal),
/// a maior primeiro: faixa de b − faixa de a.
pub(crate) unsafe extern "C" fn compare_friendly_candidates_by_band(a: usize, b: usize) -> i32 {
    *((b + 5) as *const i8) as i32 - *((a + 5) as *const i8) as i32
}

/// comparator of candidate pointers by nation (+6) and, within a nation, by band (except 0xff)
/// Original: `compare_friendly_candidate_ptrs_by_nation` (0x5c5290, friendly.cpp, cdecl).
///
/// [pt] { 0x5c5290 } (cdecl, friendly.cpp): o comparador (qsort e bsearch) dos ponteiros para candidatos: pela nação (+6,
/// a diferença em 32 bits); na mesma nação, pela faixa (+5, a maior primeiro), salvo faixa 0xff em um dos dois (0).
pub(crate) unsafe extern "C" fn compare_friendly_candidate_ptrs_by_nation(a: usize, b: usize) -> i32 {
    let (ca, cb) = (read_u32_as_usize(a), read_u32_as_usize(b));
    let dif = (read_u32_as_usize(ca + 6) as u32).wrapping_sub(read_u32_as_usize(cb + 6) as u32) as i32;
    if dif != 0 {
        return dif;
    }
    let (fa, fb) = (*((ca + 5) as *const u8), *((cb + 5) as *const u8));
    if fa == 0xff || fb == 0xff {
        return 0;
    }
    fb as i8 as i32 - fa as i8 as i32
}

/// Comparer for `schedule_tour_matches` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: sorteia, monta amistosos e cria listas (não roda duas vezes): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_schedule_tour_matches(this: usize, cand: usize, adv: usize, n: u32) -> u32 {
    // ponytail: o sorteio e os amistosos novos não rodam duas vezes; a cópia se prova pela carreira
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(IDX_SCHEDULE_TOUR_MATCHES)(this, cand, adv, n)
}

/// Comparer for `compare_friendly_candidates_by_band` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: rotina pura, pela devolução.
pub(crate) unsafe extern "C" fn comparar_compare_friendly_candidates_by_band(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_FRIENDLY_CANDIDATES_BY_BAND)(a, b);
    let n = compare_friendly_candidates_by_band(a, b);
    PLACAR.registrar(r == n, || format!("compare_friendly_candidates_by_band({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_friendly_candidate_ptrs_by_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: rotina pura, pela devolução.
pub(crate) unsafe extern "C" fn comparar_compare_friendly_candidate_ptrs_by_nation(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_FRIENDLY_CANDIDATE_PTRS_BY_NATION)(a, b);
    let n = compare_friendly_candidate_ptrs_by_nation(a, b);
    PLACAR.registrar(r == n, || format!("compare_friendly_candidate_ptrs_by_nation({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}





/// Allocates and constructs a new 0xe-byte LLIST; 0 on out of memory.
///
/// [pt] Uma lista nova (0x944cf1 de 0xe bytes + 0x68d020); 0 sem memória.
pub(crate) unsafe fn alloc_empty_list() -> usize {
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let p = novo(0xe);
    if p != 0 { criar(p) } else { 0 }
}

/// Appends a copy of n bytes at p to list l (0x68d0d0); returns the copy or 0.
///
/// [pt] Acrescenta à lista `l` uma cópia de `n` bytes de `p` (0x68d0d0); devolve a cópia (0 se falhou).
pub(crate) unsafe fn list_append_copy(l: usize, p: usize, n: u32) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    copiar(l, p, n)
}

/// Friendly::get_club_date_status (0x5c75a0) of a date for the club, checking the stadium; 1 = free.
///
/// [pt] A situação da data `data` para o clube (0x5c75a0 Friendly::get_club_date_status, com as três saídas em locais, sem
/// datas escolhidas, tipo 0 e conferindo o estádio): 1 = livre.
pub(crate) unsafe fn club_date_status(this: usize, clube: usize, data: usize) -> u8 {
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, *mut u32, *mut u32, u32, u32, u32, u32) -> u8 =
        core::mem::transmute(FN_FRIENDLY_GET_CLUB_DATE_STATUS);
    let (mut t, mut m, mut q) = (0u32, 0u32, 0u32);
    situacao(this, clube, data, &mut t, &mut m, &mut q, 0, 0, 0, 1)
}

/// walks the AI friendly plan up to today + 1 week: on the date, with the clubs free, creates the friendly and sends
/// the request; when the plan is used up, frees the array
/// Original: `Friendly::create_due_planned_friendlies` (0x5c2ae0, friendly.cpp, thiscall, ret 0).
///
/// [pt] { 0x5c2ae0 } (thiscall, ret 0, friendly.cpp): `Friendly::create_due_planned_friendlies()`: com o plano dos amistosos
/// da IA (+0x34d8, entradas de 20 bytes: +0 a data, +8 o clube A, +0xc o B, +0x10 o C; o índice em +0x34dc e a conta em
/// +0x34e0), anda o índice: entrada de data anterior a hoje + 1 semana (0x54c770) passa; de data posterior para; na data,
/// com A, B e C válidos e B e C livres na data (0x5c75a0 = 1), cria o amistoso (0x5c6f00: A contra C em casa se A = B;
/// senão A contra B fora) e manda o pedido (0x5c5570). Plano acabado: libera o vetor (0x9452ca) e zera +0x34d8, +0x34e0 e
/// +0x34e4.
pub(crate) unsafe extern "thiscall" fn create_due_planned_friendlies(this: usize) {
    let semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_ADD_WEEKS);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> usize = core::mem::transmute(FN_FRIENDLY_ADD_FRIENDLY_MATCH);
    let pedir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FRIENDLY_SEND_FRIENDLY_REQUEST);
    let mut alvo = [0u8; 8];
    semanas(G_CURRENT_DATE, &mut alvo, 1);
    if read_u32_as_usize(this + 0x34d8) == 0 {
        return;
    }
    let (dia, ano) = (u16::from_le_bytes([alvo[0], alvo[1]]), u16::from_le_bytes([alvo[2], alvo[3]]));
    while (read_u32_as_usize(this + 0x34dc) as i32) < read_u32_as_usize(this + 0x34e0) as i32 {
        let i = read_u32_as_usize(this + 0x34dc);
        let e = read_u32_as_usize(this + 0x34d8).wrapping_add(i.wrapping_mul(20));
        let (edia, eano) = (read_u16(e), read_u16(e + 2));
        if eano == ano && edia == dia {
            let (a, b, c) = (read_u32_as_usize(e + 8), read_u32_as_usize(e + 0xc), read_u32_as_usize(e + 0x10));
            if a == u32::MAX as usize || c == u32::MAX as usize || b == u32::MAX as usize {
                write_u32_from_usize(this + 0x34dc, i.wrapping_add(1));
                continue;
            }
            let mut data = [0u8; 8]; // cópia da data da entrada
            core::ptr::copy_nonoverlapping(e as *const u8, data.as_mut_ptr(), 8);
            let pdata = data.as_mut_ptr() as usize;
            if club_date_status(this, usa_mls_all_stars_club_by_index(b), pdata) == 1 && club_date_status(this, usa_mls_all_stars_club_by_index(c), pdata) == 1 {
                let e = read_u32_as_usize(this + 0x34d8).wrapping_add(read_u32_as_usize(this + 0x34dc).wrapping_mul(20));
                crate::comparacao::ramo("cps_12 amistoso planejado");
                let m = if a == b { acrescentar(this, usa_mls_all_stars_club_by_index(a), e, usa_mls_all_stars_club_by_index(c), 1) } else { acrescentar(this, usa_mls_all_stars_club_by_index(a), e, usa_mls_all_stars_club_by_index(b), 0) }; // cps12: criar
                if m != 0 {
                    pedir(this, m);
                }
            }
            write_u32_from_usize(this + 0x34dc, read_u32_as_usize(this + 0x34dc).wrapping_add(1));
            continue;
        }
        // datas: o ano sem sinal, o dia com sinal
        if ano < eano || (ano == eano && (dia as i16) < edia as i16) {
            return; // a entrada é de depois
        }
        if ano > eano || (dia as i16) > edia as i16 {
            write_u32_from_usize(this + 0x34dc, i.wrapping_add(1)); // ficou para trás
        }
    }
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    liberar(read_u32_as_usize(this + 0x34d8));
    write_u32_from_usize(this + 0x34d8, 0);
    write_u32_from_usize(this + 0x34e0, 0);
    write_u32_from_usize(this + 0x34e4, 0);
}

/// True if the club's stadium (+0x69) already has a match on the date in any slice (0x5ae870); false without stadium.
///
/// [pt] O estádio do clube (+0x69) já tem jogo na data em alguma fatia (0x5ae870 do calendário [0xae2a58] com −1)? Sem
/// estádio: não.
pub(crate) unsafe fn stadium_busy_on(clube: usize, data: usize) -> bool {
    let estadio = read_u32_as_usize(clube + club::STADIUM);
    if estadio == 0 {
        return false;
    }
    let ocupado: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_IS_STADIUM_IN_USE);
    // ponytail: a original empurra o dia e o ano (palavras) com sobra de registrador em cima; a 0x5ae870 lê as palavras
    ocupado(read_u32_as_usize(G_FIXTURE_MANAGER), estadio, read_u16(data) as u32, read_u16(data + 2) as u32, u32::MAX) != 0
}

/// True if the stadium of the home side (the club when em_casa, else the opponent) is busy on the date.
///
/// [pt] O estádio de quem joga em casa (o clube com `em_casa`, senão o adversário, se houver) está ocupado na data?
pub(crate) unsafe fn home_stadium_busy_on(clube: usize, data: usize, adversario: usize, em_casa: u32) -> bool {
    if em_casa != 0 {
        stadium_busy_on(clube, data)
    } else {
        adversario != 0 && stadium_busy_on(adversario, data)
    }
}

/// reuses the club's open invitation or builds a new friendly; with the home side's stadium free, stores date, clubs
/// and venue and puts it in the list
/// Original: `Friendly::add_friendly_match` (0x5c6f00, friendly.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5c6f00 } (thiscall, ret 0x10, friendly.cpp): `Friendly::add_friendly_match(clube, data, adversario, em_casa)`:
/// reaproveita o convite aberto do clube (0x5c88f0 sem data) ou monta um amistoso novo de 0x19 bytes (0x944cf1;
/// 0x54a110; sem memória: a caixa, linha 0xe6b, e 0). Se o estádio de quem joga em casa está ocupado na data (0x5ae870):
/// 0 (o novo fica sem liberar, como na original). Grava a data (0x417c10), +0xc o clube, +0x18 = 1, +8 = 0 e o mandante
/// (+0x10) e o visitante (+0x14). Sem a lista dos amistosos (+0x34c8), cria (0x68d020; sem memória: a caixa, linha
/// 0xe9f, libera o amistoso e 0). O reaproveitado volta; o novo vai em cópia para a lista (0x68d0d0; falhou: a caixa,
/// linha 0xeac, e 0), é liberado (0x944c94) e a cópia volta.
pub(crate) unsafe extern "thiscall" fn add_friendly_match(this: usize, clube: usize, data: usize, adversario: usize, em_casa: u32) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_FRIENDLY_MATCH);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let mut reaproveitado = false;
    let mut m = achar(this, 0, clube);
    if m != 0 {
        reaproveitado = true;
    } else {
        let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
        let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
        let p = novo(0x19);
        m = if p != 0 { data_padrao(p); p } else { 0 };
        if m == 0 {
            show_error_at(0x9cd0a0, 0xe6b);
            return 0;
        }
    }
    if home_stadium_busy_on(clube, data, adversario, em_casa) {
        return 0;
    }
    let atribuir: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut copia = [0u8; 8];
    write_u32_from_usize(m + 0xc, clube);
    atribuir(m, &mut copia, data);
    *((m + 0x18) as *mut u8) = 1;
    write_u32_from_usize(m + 8, 0);
    let (mandante, visitante) = if em_casa != 0 {
        crate::comparacao::ramo("cps_12 amistoso em casa");
        (clube, adversario) // cps12: o clube em casa
    } else {
        (adversario, clube)
    };
    write_u32_from_usize(m + 0x10, mandante);
    write_u32_from_usize(m + 0x14, visitante);
    if read_u32_as_usize(this + 0x34c8) == 0 {
        let l = alloc_empty_list();
        write_u32_from_usize(this + 0x34c8, l);
        if l == 0 {
            show_error_at(0x9cd0a0, 0xe9f);
            liberar(m);
            return 0;
        }
    }
    if reaproveitado {
        return m;
    }
    let c = list_append_copy(read_u32_as_usize(this + 0x34c8), m, 0x19);
    if c == 0 {
        show_error_at(0x9cd0a0, 0xeac);
        liberar(m);
        return 0;
    }
    liberar(m);
    c
}

/// draws a date in both clubs' window and, with both free, books the friendly and links both clubs' dates and opponents
/// Original: `Friendly::plan_friendly_on_free_date` (0x5c4380, friendly.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5c4380 } (thiscall, ret 0xc, friendly.cpp): `Friendly::plan_friendly_on_free_date(anfitriao, casa, fora)`: a janela
/// é a do clube da casa (+0xe..+0x10, palavras com sinal) cortada pela do de fora (se +0xe ≠ −1); vazia: 0. Sorteia um
/// deslocamento (0x90cfc0) e a data = início da temporada (+0xa7) + início da janela + sorteio (0x54c5b0). Até 5 vezes,
/// os dois livres na data (0x5c45a0 = 0 para os dois) marcam achado; a marca começa no lugar do argumento de fora, que a
/// original zera antes. Sem achado: 0. Achado: o amistoso entra (0x5c4b60), a data vai para as listas de datas dos dois
/// (+0x12, criadas se faltam; 8 bytes), cada um entra na lista de adversários do outro (+0xa; os 4 primeiros bytes do
/// candidato) e as contas (+4) sobem. Devolve 1.
pub(crate) unsafe extern "thiscall" fn plan_friendly_on_free_date(this: usize, anfitriao: usize, casa: usize, fora: usize) -> u32 {
    let mut ini = *((casa + 0xe) as *const i16);
    let mut fim = *((casa + 0x10) as *const i16);
    let fini = *((fora + 0xe) as *const i16);
    if fini != -1 {
        if fini > ini {
            ini = fini;
        }
        let ffim = *((fora + 0x10) as *const i16);
        if ffim < fim {
            fim = ffim;
        }
    }
    if ini >= fim {
        return 0;
    }
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let n = (ini as i32).wrapping_add(sortear(fim as i32 - ini as i32)); // cps12: o dia sorteado
    let mut data = [0u8; 8];
    // ponytail: a original empurra o esi inteiro (a palavra baixa é a soma, em cima sobra de registrador); a 0x54c5b0 lê
    // a palavra
    mais_dias(this + 0xa7, &mut data, n as u16 as u32);
    let pdata = data.as_mut_ptr() as usize;
    let ocupado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_CANDIDATE_HAS_NEARBY_DATE);
    let mut achado = false;
    let mut k = 0u8;
    loop {
        if k == 5 {
            if !achado {
                return 0;
            }
            break;
        }
        if ocupado(this, casa, pdata) == 0 && ocupado(this, fora, pdata) == 0 {
            achado = true;
        }
        k += 1;
        if achado {
            break;
        }
    }
    let marcar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_ADD_PLANNED_FRIENDLY);
    crate::comparacao::ramo("cps_12 data livre");
    marcar(this, anfitriao, casa, fora, pdata);
    if read_u32_as_usize(casa + 0x12) == 0 {
        write_u32_from_usize(casa + 0x12, alloc_empty_list());
    }
    if read_u32_as_usize(fora + 0x12) == 0 {
        write_u32_from_usize(fora + 0x12, alloc_empty_list());
    }
    list_append_copy(read_u32_as_usize(casa + 0x12), pdata, 8);
    list_append_copy(read_u32_as_usize(fora + 0x12), pdata, 8);
    if read_u32_as_usize(casa + 0xa) == 0 {
        write_u32_from_usize(casa + 0xa, alloc_empty_list());
    }
    list_append_copy(read_u32_as_usize(casa + 0xa), fora, 4);
    if read_u32_as_usize(fora + 0xa) == 0 {
        write_u32_from_usize(fora + 0xa, alloc_empty_list());
    }
    list_append_copy(read_u32_as_usize(fora + 0xa), casa, 4);
    *((casa + 4) as *mut u8) = (*((casa + 4) as *const u8)).wrapping_add(1);
    *((fora + 4) as *mut u8) = (*((fora + 4) as *const u8)).wrapping_add(1);
    1
}

// ponytail: cria amistosos (aloca), manda os pedidos e libera o plano: não roda duas vezes. Só a original; a cópia leva o
// ramo (pré-temporada da IA, 515 chamadas até 2002-06-01).
/// Comparer for `create_due_planned_friendlies` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_create_due_planned_friendlies(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CREATE_DUE_PLANNED_FRIENDLIES)(this)
}

/// Comparer for `add_friendly_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: aloca e acrescenta à lista (não roda duas vezes): a original faz e a cópia confere o resultado com o
/// estádio de quem joga em casa visto antes (0x5ae870, só lê): ocupado → 0; senão o amistoso devolvido tem a data,
/// +0xc o clube, +8 = 0, +0x18 = 1 e o mandante e o visitante pelo `em_casa`.
pub(crate) unsafe extern "thiscall" fn comparar_add_friendly_match(this: usize, clube: usize, data: usize, adversario: usize, em_casa: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> usize>(IDX_ADD_FRIENDLY_MATCH);
    let ocupada = home_stadium_busy_on(clube, data, adversario, em_casa);
    let (dia, ano) = (read_u16(data), read_u16(data + 2));
    let r = o(this, clube, data, adversario, em_casa);
    let (casa, fora) = if em_casa != 0 { (clube, adversario) } else { (adversario, clube) };
    let igual = if ocupada {
        r == 0
    } else {
        // ponytail: r = 0 sem estar ocupada só sem memória (a caixa já saiu): não confere
        r == 0 || (read_u16(r) == dia && read_u16(r + 2) == ano && read_u32_as_usize(r + 0xc) == clube && read_u32_as_usize(r + 8) == 0
            && *((r + 0x18) as *const u8) == 1 && read_u32_as_usize(r + 0x10) == casa && read_u32_as_usize(r + 0x14) == fora)
    };
    PLACAR.registrar(igual, || format!("add_friendly_match(clube {:x}, {}/{}, adversario {:x}, em casa {}) ocupada {} original={:x}",
        clube, dia, ano, adversario, em_casa, ocupada, r));
    r
}

// ponytail: sorteia a data e acrescenta às listas: não roda duas vezes. Só a original; a cópia leva o ramo.
/// Comparer for `plan_friendly_on_free_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_plan_friendly_on_free_date(this: usize, anfitriao: usize, casa: usize, fora: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_PLAN_FRIENDLY_ON_FREE_DATE)(this, anfitriao, casa, fora)
}


/// tells whether the club is free for a friendly on the date: no fixture from the day before to the kind rest days and,
/// outside national sides, outside the key-nation season
/// Original: `friendly_is_club_free_on_date` (0x5c6b70, friendly.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x5c6b70 } (stdcall, ret 0xc, friendly.cpp): `friendly_is_club_free_on_date(data, clube, tipo)` (da marcação dos
/// amistosos: excursões, torneios, respostas aos convites e a situação das datas do clube): a janela vai da véspera
/// (0x54c8e0) até `folga` dias depois (0x54c5b0; folga 2 nos tipos 1 e 10, 3 no tipo 2, 1 nos tipos 0, 3, 4 e 11;
/// outro tipo: a caixa, linha 0xf18, e folga 1). Com tipo ≠ 0, clube que não é seleção e nação-chave ativa, a janela
/// que cai na temporada da nação (início em 0xb63d8a, fim em 0xb63d99, registro de 72 bytes da 0x669340) não serve.
/// Senão, livre se a 0x5acbe0 não acha jogo do clube (+0 id) na janela (com uma lista provisória, destruída no fim).
/// Clube ou data nulos: 0.
pub(crate) unsafe extern "stdcall" fn friendly_is_club_free_on_date(data: usize, clube: usize, tipo: u32) -> u32 {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let criar_lista: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let menos_dias: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mais_dias: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let (mut a, mut b, mut t0, mut t1) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let mut lista = [0u8; 0x10];
    let (pa, pb, p0, p1, pl) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, t0.as_mut_ptr() as usize,
        t1.as_mut_ptr() as usize, lista.as_mut_ptr() as usize);
    data_padrao(pa);
    data_padrao(pb);
    criar_lista(pl);
    let sai = |r: u32| -> u32 {
        destruir_lista(pl);
        r
    };
    if clube == 0 || data == 0 {
        return sai(0);
    }
    let k = tipo as u8 as i8;
    let folga: u8 = match k {
        1 | 10 => 2,
        2 => 3,
        0 | 3 | 4 | 11 => 1,
        _ => {
            show_error_at(0x9cd0a0, 0xf18);
            1
        }
    };
    crate::comparacao::ramo("cpr_13 clube livre");
    let r = menos_dias(data, p0, 1); // a véspera
    copiar(pa, p1, r);
    // ponytail: a original empurra a folga com os 16 bits altos do ponteiro que a 0x417c10 devolveu; a 0x54c5b0 só lê
    // a palavra
    let r = mais_dias(data, p1, folga as u32);
    copiar(pb, p0, r);
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const u16);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let nacao = read_u32(clube + club::NATION) as usize;
    if k != 0 && selecao(clube) == 0 && nacao != 0 && *((nacao + nation::NATION_LEAGUE_SELECTED) as *const u8) != 0 {
        let indice: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_NATION);
        let reg = (indice(nacao) as u8 as i8 as isize * 72) as usize;
        let inicio = G_LEAGUE_WORLD_CUP_START_DATE.wrapping_add(reg); // a temporada da nação-chave
        let (iano, idia) = (w(inicio, 2), w(inicio, 0) as i16);
        if iano < w(pb, 2) || (iano == w(pb, 2) && idia < w(pb, 0) as i16) {
            let fim = G_LEAGUE_WORLD_CUP_END_DATE.wrapping_add(reg);
            let (fano, fdia) = (w(fim, 2), w(fim, 0) as i16);
            if fano > w(pa, 2) || (fano == w(pa, 2) && fdia > w(pa, 0) as i16) {
                return sai(0); // a janela cai na temporada
            }
        }
    }
    // os dias e anos vão como os dwords da original (a palavra e os 2 bytes seguintes da data)
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    let n = jogos(read_u32(G_FIXTURE_MANAGER) as usize, read_u32(pa), read_u32(pa + 2), read_u32(pb), read_u32(pb + 2), read_u32(clube),
        pl, u32::MAX, 0xffff_fffe, u32::MAX);
    if n as u16 != 0 {
        crate::comparacao::ramo("cpr_13 ocupado");
        return sai(0); // tem jogo na janela
    }
    sai(1)
}



/// Comparer for `friendly_is_club_free_on_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a cópia roda com a lista provisória dela, como a original: pela devolução.
pub(crate) unsafe extern "stdcall" fn comparar_friendly_is_club_free_on_date(data: usize, clube: usize, tipo: u32) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32) -> u32>(IDX_FRIENDLY_IS_CLUB_FREE_ON_DATE)(data, clube, tipo);
    let n = friendly_is_club_free_on_date(data, clube, tipo);
    PLACAR.registrar(r == n, || format!("friendly_is_club_free_on_date(data {:x}, clube {:x}, tipo {}) original={} rust={}", data, clube, tipo as u8 as i8, r, n));
    r
}





/// returns 1 when (a, b), (a, −1) or (−1, b) is among the banned pairs (+0x3502, count +0x3506), else 0
/// Original: `Friendly::is_banned_nation_pair` (0x5cc3e0, friendly.cpp, thiscall, ret 8).
///
/// [pt] { 0x5cc3e0 } (thiscall, ret 8, friendly.cpp): `Friendly::is_banned_nation_pair(a, b)`: percorre os pares proibidos
/// (vetor de 8 bytes em +0x3502, conta em byte com sinal em +0x3506) e devolve 1 quando (a, b), (a, −1) ou (−1, b) está
/// lá. Senão 0: o resto da original (Austrália, Nova Zelândia e Oceania, 0x9cf224/0x9cf3f4/0x9cfa18, pela tabela das
/// nações) só lê e cai sempre no `xor eax, eax`.
pub(crate) unsafe extern "thiscall" fn is_banned_nation_pair(this: usize, a: i32, b: i32) -> u32 {
    let n = *((this + 0x3506) as *const i8);
    let mut k: i8 = 0;
    while k < n {
        let p = read_usize(this + 0x3502).wrapping_add(k as isize as usize * 8);
        let (x, y) = (read_i32(p), read_i32(p + 4));
        if (x == a && (y == b || y == -1)) || (x == -1 && y == b) {
            crate::comparacao::ramo("cpp_11 par proibido");
            return 1; // o par proibido
        }
        k = k.wrapping_add(1);
    }
    // ponytail: as leituras da Oceania não mudam a devolução (sempre 0); a cópia não as repete.
    0
}

/// qsort comparator of nation pointers by reputation (+0x8e), highest first
/// Original: `compare_nations_by_reputation_desc` (0x5cc5f0, friendly.cpp, cdecl).
///
/// [pt] { 0x5cc5f0 } (cdecl, friendly.cpp): `compare_nations_by_reputation_desc(a, b)`: o qsort das listas de nações
/// (ponteiros) pela reputação (+0x8e, palavra com sinal), a maior primeiro. Quente (qsort): sem contador.
pub(crate) unsafe extern "C" fn compare_nations_by_reputation_desc(a: usize, b: usize) -> i32 {
    let ra = *((read_usize(a) + 0x8e) as *const i16) as i32;
    let rb = *((read_usize(b) + 0x8e) as *const i16) as i32;
    rb - ra // a maior reputação primeiro
}

/// qsort comparator of friendly records by the date at +3 (0x456770/0x456740), earliest first
/// Original: `compare_fixture_records_by_date` (0x5cc610, friendly.cpp, cdecl).
///
/// [pt] { 0x5cc610 } (cdecl, friendly.cpp): `compare_fixture_records_by_date(a, b)`: o qsort dos registros de 20 bytes dos
/// amistosos pela data em +3: −1 se a de `a` é antes (0x456770), 1 se é depois (0x456740), senão 0. Quente (qsort):
/// sem contador.
pub(crate) unsafe extern "C" fn compare_fixture_records_by_date(a: usize, b: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    let (d0, d1) = (read_usize(b + 3) as u32, read_usize(b + 7) as u32);
    let r = if antes(a + 3, d0, d1) != 0 { -1 } else { (depois(a + 3, d0, d1) != 0) as i32 };
    r // a data mais cedo primeiro
}

/// Comparer for `is_banned_nation_pair` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_is_banned_nation_pair(this: usize, a: i32, b: i32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(IDX_IS_BANNED_NATION_PAIR)(this, a, b);
    let n = is_banned_nation_pair(this, a, b);
    PLACAR.registrar(r == n, || format!("is_banned_nation_pair({}, {}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_nations_by_reputation_desc` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_compare_nations_by_reputation_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_NATIONS_BY_REPUTATION_DESC)(a, b);
    let n = compare_nations_by_reputation_desc(a, b);
    PLACAR.registrar(r == n, || format!("compare_nations_by_reputation_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparer for `compare_fixture_records_by_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_compare_fixture_records_by_date(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_COMPARE_FIXTURE_RECORDS_BY_DATE)(a, b);
    let n = compare_fixture_records_by_date(a, b);
    PLACAR.registrar(r == n, || format!("compare_fixture_records_by_date({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}





/// removes the match's items from the +0x34d4 list, clears the scheduled slot (with clear_linked) and removes the match
/// from the +0x34c8 list (freeing it)
/// Original: `Friendly::remove_friendly_match` (0x5c87b0, friendly.cpp, thiscall, ret 8).
///
/// [pt] { 0x5c87b0 } (thiscall, ret 8, friendly.cpp): `Friendly::remove_friendly_match(jogo, limpar_vaga)`: tira da lista
/// [+0x34d4] cada item que aponta para o `jogo` (0x68d1f0; o cursor +0xa volta ao começo antes, depois de cada um e no
/// fim), com `limpar_vaga` zera a vaga do amistoso marcado (0x5c6740 pelo dia +0, ano +2, clube +0x14 e adversário
/// +0x10 do jogo: +0x10 = 0 e +0x18 = 0) e tira o jogo da lista [+0x34c8] (0x68d1f0, que o libera). Jogo nulo ou sem
/// a lista +0x34c8: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn remove_friendly_match(this: usize, jogo: usize, limpar_vaga: u32) -> u32 {
    if jogo == 0 || read_usize(this + 0x34c8) == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_REMOVE_FRIENDLY_MATCH)(this, jogo, limpar_vaga);
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let recomecar = || {
        let l = read_usize(this + 0x34d4);
        write_usize(l + 0xa, read_usize(l + 2));
    };
    if read_usize(this + 0x34d4) != 0 {
        recomecar();
        let mut e = proximo(read_usize(this + 0x34d4));
        while e != 0 {
            if read_usize(e) == jogo {
                tirar(read_usize(this + 0x34d4), e);
                recomecar();
            }
            e = proximo(read_usize(this + 0x34d4));
        }
        recomecar();
    }
    if limpar_vaga != 0 {
        // ponytail: a original empurra dia e ano em registradores com os 16 bits altos de outros valores; a 0x5c6740 só
        // lê os 16 bits baixos
        let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
        let v = vaga(this, *(jogo as *const u16) as u32, *((jogo + 2) as *const u16) as u32, read_usize(jogo + 0x14), read_usize(jogo + 0x10));
        if v != 0 {
            write_usize(v + 0x10, 0);
            *((v + 0x18) as *mut u8) = 0;
        }
    }
    crate::comparacao::ramo("cpn_7 tirar o amistoso");
    tirar(read_usize(this + 0x34c8), jogo) // tira o jogo da lista dos amistosos
}

/// allocates the 4 nation pairs that play no friendlies with each other (Iran and USA, Iraq and anyone, Vietnam and
/// USA, the Koreas) at +0x3502, count at +0x3506
/// Original: `Friendly::init_banned_nation_pairs` (0x5cc2d0, friendly.cpp, thiscall).
///
/// [pt] { 0x5cc2d0 } (thiscall, friendly.cpp): `Friendly::init_banned_nation_pairs()`: +0x3506 = 4 pares e +0x3502 um vetor
/// novo de 0x20 bytes (malloc 0x944e46) com os pares de nações que não jogam amistoso entre si: Irã e EUA, Iraque e
/// qualquer uma (−1), Vietnã e EUA, as duas Coreias (as chaves [0x9cf354], [0x9cf4f8], [0x9cf358], [0x9cf508],
/// [0x9cf404], [0x9cf484]). Sem memória: a caixa "Error" (linha 0x1800) e +0x3506 = 0.
pub(crate) unsafe extern "thiscall" fn init_banned_nation_pairs(this: usize) {
    *((this + 0x3506) as *mut u8) = 4;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let v = alocar(0x20);
    write_usize(this + 0x3502, v);
    if v == 0 {
        show_error_at(0x9cd0a0, 0x1800);
        *((this + 0x3506) as *mut u8) = 0;
        return;
    }
    for (k, chave) in [KEY_NATION_IRAN, KEY_NATION_UNITED_STATES, KEY_NATION_IRAQ, 0, KEY_NATION_VIETNAM, KEY_NATION_UNITED_STATES, KEY_NATION_NORTH_KOREA, KEY_NATION_SOUTH_KOREA].into_iter().enumerate() {
        write_usize(read_usize(this + 0x3502) + 4 * k, if chave == 0 { usize::MAX } else { read_usize(chave) });
    }
}


/// Comparer for `remove_friendly_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a 0x68d1f0 libera o que tira: o plano antes (os itens que ficam nas duas listas e a vaga do amistoso);
/// a original roda; confere as listas, o cursor da +0x34d4 no começo e a vaga zerada. Jogo nulo ou sem a lista
/// +0x34c8: só a original (a caixa).
pub(crate) unsafe extern "thiscall" fn comparar_remove_friendly_match(this: usize, jogo: usize, limpar_vaga: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_REMOVE_FRIENDLY_MATCH);
    if jogo == 0 || read_usize(this + 0x34c8) == 0 {
        return o(this, jogo, limpar_vaga);
    }
    // ponytail: a 0x68d1f0 acha o nó pelo conteúdo do item; o plano supõe itens de conteúdo distinto (o normal)
    let l4 = read_usize(this + 0x34d4);
    let esperado4: Vec<usize> = if l4 != 0 { list_items_capped(l4).into_iter().filter(|&e| read_usize(e) != jogo).collect() } else { Vec::new() };
    let mut esperado8 = list_items_capped(read_usize(this + 0x34c8));
    if let Some(k) = esperado8.iter().position(|&e| e == jogo) {
        esperado8.remove(k);
    }
    let v = if limpar_vaga != 0 {
        let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(FN_FRIENDLY_FIND_SCHEDULED_FRIENDLY_SLOT);
        vaga(this, *(jogo as *const u16) as u32, *((jogo + 2) as *const u16) as u32, read_usize(jogo + 0x14), read_usize(jogo + 0x10))
    } else {
        0
    };
    let r = o(this, jogo, limpar_vaga);
    let l4 = read_usize(this + 0x34d4);
    let itens4 = if l4 != 0 { list_items_capped(l4) } else { Vec::new() };
    let itens8 = list_items_capped(read_usize(this + 0x34c8));
    let ok = itens4 == esperado4
        && (l4 == 0 || read_usize(l4 + 0xa) == read_usize(l4 + 2))
        && itens8 == esperado8
        && (v == 0 || (read_usize(v + 0x10) == 0 && read_u8(v + 0x18) == 0));
    PLACAR.registrar(ok, || format!("remove_friendly_match({:#x}, {}) itens {} e {} (esperava {} e {}), vaga {:#x}", jogo, limpar_vaga, itens4.len(), itens8.len(), esperado4.len(), esperado8.len(), v));
    r
}

/// Comparer for `init_banned_nation_pairs` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: o malloc não pode rodar duas vezes: a original roda e confere o vetor e a conta com o plano.
pub(crate) unsafe extern "thiscall" fn comparar_init_banned_nation_pairs(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_BANNED_NATION_PAIRS)(this);
    let v = read_usize(this + 0x3502);
    let ok = if v == 0 {
        read_u8(this + 0x3506) == 0
    } else {
        let esperado = [read_usize(KEY_NATION_IRAN), read_usize(KEY_NATION_UNITED_STATES), read_usize(KEY_NATION_IRAQ), usize::MAX, read_usize(KEY_NATION_VIETNAM), read_usize(KEY_NATION_UNITED_STATES), read_usize(KEY_NATION_NORTH_KOREA), read_usize(KEY_NATION_SOUTH_KOREA)];
        read_u8(this + 0x3506) == 4 && (0..8).all(|k| read_usize(v + 4 * k) == esperado[k])
    };
    PLACAR.registrar(ok, || format!("init_banned_nation_pairs: conta {} vetor {:#x}", read_u8(this + 0x3506), v));
}



/// (day signed, year unsigned) of a date.
///
/// [pt] (dia, ano) de uma data (+0 dia, palavra com sinal; +2 ano, palavra sem sinal).
pub(crate) unsafe fn friendly_date_day_year(a: usize) -> (i16, u16) {
    (core::ptr::read_unaligned(a as *const i16), core::ptr::read_unaligned((a + 2) as *const u16))
}

/// returns 1 for an ordinary competition; for a friendly tournament, only when the date is within 7 days of the slot
/// date
/// Original: `Friendly::is_comp_current_on_date` (0x5c29b0, friendly.cpp, thiscall, ret 8).
///
/// [pt] { 0x5c29b0 } (thiscall, ret 8, friendly.cpp): `Friendly::is_comp_current_on_date(comp, data)`: competição que não
/// é a do amistoso ([this+4]) e de id abaixo de [0xae2378] − 0x7f: 1. Senão, a vaga do torneio (this + 0x69 × (byte do
/// id − byte +0xaf), +0xb1 ocupada, +0xb5 a data): 1 se a `data` está depois da data da vaga − 7 dias (0x54c8e0) e
/// antes dela + 7 (0x54c5b0), pelo ano (sem sinal) e depois o dia (com sinal); vaga vazia: 0. O quadro SEH só guarda
/// as datas locais (os destrutores são vazios).
pub(crate) unsafe extern "thiscall" fn is_comp_current_on_date(this: usize, comp: usize, data: usize) -> u32 {
    if comp != read_usize(this + 4) && (read_usize(comp) as i32) < (read_usize(G_CLUB_COMP_COUNT) as i32).wrapping_sub(0x7f) {
        return 1;
    }
    let k = (*(comp as *const u8)).wrapping_sub(*((this + 0xaf) as *const u8)) as i8 as isize;
    let e = this.wrapping_add((k * 0x69) as usize);
    if read_usize(e + 0xb1) == 0 {
        return 0;
    }
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mais: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let (dia, ano) = friendly_date_day_year(data);
    let mut a = [0u8; 8];
    let (adia, aano) = friendly_date_day_year(menos(e + 0xb5, &mut a, 7));
    if ano < aano || (ano == aano && dia <= adia) {
        return 0;
    }
    let (dia, ano) = friendly_date_day_year(data);
    let mut b = [0u8; 8];
    let (bdia, bano) = friendly_date_day_year(mais(e + 0xb5, &mut b, 7));
    (ano < bano || (ano == bano && dia < bdia)) as u32
}

/// returns 1 when the club already has a planned friendly on the date, the day before or the day after
/// Original: `Friendly::candidate_has_nearby_date` (0x5c45a0, friendly.cpp, thiscall (this sem uso), ret 8).
///
/// [pt] { 0x5c45a0 } (thiscall sem usar o this, ret 8, friendly.cpp): `Friendly::candidate_has_nearby_date(clube, data)`:
/// 1 se a lista de amistosos planejados do candidato (+0x12; itens com dia +0 e ano +2) tem a `data`, o dia antes
/// (0x54c8e0) ou o dia depois (0x54c5b0). Percorre do começo (o cursor +0xa = a cabeça +2; 0x68d1b0). Sem lista: 0. As
/// datas locais vêm antes (como a original); o quadro SEH é delas (destrutores vazios).
pub(crate) unsafe extern "thiscall" fn candidate_has_nearby_date(_this: usize, clube: usize, data: usize) -> u32 {
    let menos: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let mais: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let (mut antes, mut depois) = ([0u8; 8], [0u8; 8]);
    menos(data, &mut antes, 1);
    mais(data, &mut depois, 1);
    let l = read_usize(clube + 0x12);
    if l == 0 {
        return 0;
    }
    write_usize(l + 0xa, read_usize(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let antes = friendly_date_day_year(antes.as_ptr() as usize);
    let depois = friendly_date_day_year(depois.as_ptr() as usize);
    let mut e = proximo(l);
    while e != 0 {
        let item = friendly_date_day_year(e);
        if item == friendly_date_day_year(data) {
            return 1;
        }
        if item == antes {
            crate::comparacao::ramo("cpm_7 amistoso no dia antes");
            return 1;
        }
        if item == depois {
            crate::comparacao::ramo("cpm_7 amistoso no dia depois");
            return 1;
        }
        e = proximo(l);
    }
    0
}

/// appends the friendly (date and three ids) to the 0x14-byte array, which grows by 100
/// Original: `Friendly::add_planned_friendly` (0x5c4b60, friendly.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5c4b60 } (thiscall, ret 0x10, friendly.cpp): `Friendly::add_planned_friendly(anfitriao, mandante, visitante,
/// data)`: acrescenta ao vetor de 0x14 bytes [this+0x34d8] (conta +0x34e0, capacidade +0x34e4; cheio: cresce de 100
/// em 100, realloc 0x945667; sem memória, a caixa "Error" 0x797 e volta) o amistoso: a data em +0 (0x417c10, a cópia
/// devolvida fica na pilha) e os ids (o dword apontado por cada argumento) em +8, +0xc e +0x10; a conta sobe.
pub(crate) unsafe extern "thiscall" fn add_planned_friendly(this: usize, anfitriao: usize, mandante: usize, visitante: usize, data: usize) {
    if read_usize(this + 0x34e0) == read_usize(this + 0x34e4) {
        crate::comparacao::ramo("cpm_7 amistosos crescem");
        let cap = read_usize(this + 0x34e4).wrapping_add(100);
        write_usize(this + 0x34e4, cap);
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realocar(read_usize(this + 0x34d8), cap.wrapping_mul(0x14) as u32);
        write_usize(this + 0x34d8, p);
        if p == 0 {
            show_error_at(0x9cd0a0, 0x797);
            return;
        }
    }
    crate::comparacao::ramo("cpm_7 amistoso planejado");
    let n = read_usize(this + 0x34e0);
    let item = read_usize(this + 0x34d8).wrapping_add(n.wrapping_mul(0x14));
    write_usize(item + 8, read_usize(anfitriao));
    write_usize(item + 0xc, read_usize(mandante)); // o mandante
    write_usize(item + 0x10, read_usize(visitante)); // o visitante
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut copia = [0u8; 8];
    copiar(item, &mut copia, data);
    write_usize(this + 0x34e0, n.wrapping_add(1)); // mais um amistoso
}

/// Comparer for `is_comp_current_on_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (as datas e a vaga só são lidas). Chamada pelas copas e ligas a cada data: sem contador.
pub(crate) unsafe extern "thiscall" fn comparar_is_comp_current_on_date(this: usize, comp: usize, data: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_IS_COMP_CURRENT_ON_DATE)(this, comp, data);
    let n = is_comp_current_on_date(this, comp, data);
    PLACAR.registrar(r == n, || format!("is_comp_current_on_date({}, {:?}) original={} rust={}", read_usize(comp), friendly_date_day_year(data), r, n));
    r
}

/// Comparer for `candidate_has_nearby_date` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo cursor da lista (+0xa) e o valor.
pub(crate) unsafe extern "thiscall" fn comparar_candidate_has_nearby_date(this: usize, clube: usize, data: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_CANDIDATE_HAS_NEARBY_DATE);
    let l = read_usize(clube + 0x12);
    let reg: &[(usize, usize)] = if l == 0 { &[] } else { &[(l + 0xa, 4)] };
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(reg, || o(this, clube, data), || candidate_has_nearby_date(this, clube, data));
    PLACAR.registrar(igual && r == n, || format!("candidate_has_nearby_date({:?}) original={} rust={} {:x?} x {:x?}", friendly_date_day_year(data), r, n, mo, mr));
    r
}

/// Comparer for `add_planned_friendly` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: a original acrescenta (pode realocar, não roda duas vezes): o plano antes (a conta, os ids e a data),
/// a original roda, confere o item novo e a conta.
pub(crate) unsafe extern "thiscall" fn comparar_add_planned_friendly(this: usize, anfitriao: usize, mandante: usize, visitante: usize, data: usize) {
    let n = read_usize(this + 0x34e0);
    let esperado = [read_usize(data), read_usize(data + 4), 0, read_usize(anfitriao), read_usize(mandante), read_usize(visitante)];
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, usize)>(IDX_ADD_PLANNED_FRIENDLY)(this, anfitriao, mandante, visitante, data);
    let v = read_usize(this + 0x34d8);
    if v == 0 {
        return; // sem memória: a caixa já saiu
    }
    let item = v.wrapping_add(n.wrapping_mul(0x14));
    let feito = [read_usize(item), read_usize(item + 4), 0, read_usize(item + 8), read_usize(item + 0xc), read_usize(item + 0x10)];
    PLACAR.registrar(feito == esperado && read_usize(this + 0x34e0) == n.wrapping_add(1), || {
        format!("add_planned_friendly conta {} -> {}, item {:x?} (esperava {:x?})", n, read_usize(this + 0x34e0), feito, esperado)
    });
}




/// Club record with the given id in the club table (0x245 bytes each).
///
/// [pt] O clube de id `id` na tabela [0xae23b4] (registros de 0x245 bytes).
pub(crate) unsafe fn friendly_club_by_id(id: usize) -> usize {
    read_usize(G_CLUB_TABLE).wrapping_add(id.wrapping_mul(club::SIZE))
}

/// True if candidate c already has the club (by id) in its opponents list (+0xa); rewinds and walks the list.
///
/// [pt] O candidato `c` já tem o candidato `clube` (pelo id, +0) na lista de adversários (+0xa)? Volta o cursor da lista
/// ao começo (+0xa = +2) e percorre com a 0x68d1b0, como as duas originais.
pub(crate) unsafe fn already_opponent(c: usize, clube: usize) -> bool {
    let l = read_usize(c + 0xa);
    if l == 0 {
        return false;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    loop {
        let x = proximo(read_usize(c + 0xa));
        if x == 0 {
            return false;
        }
        if read_usize(x) == read_usize(clube) {
            return true;
        }
    }
}

/// True in the friendly's year (+0x34e8) for a club whose euro competition (+0x1db) is the Intertoto Cup
/// (KEY_CLUB_COMP_INTER_TOTO_CUP).
///
/// [pt] Fora pela data: no ano do amistoso ([0xae2c90], com sinal, contra +0x34e8 do Friendly, sem sinal) o clube `op`
/// com +0x1db igual ao float de [0x9cf6f8] (comparado como inteiro).
pub(crate) unsafe fn excluded_by_intertoto(this: usize, op: usize) -> bool {
    *(G_CURRENT_DATE as *const i16) as i32 == core::ptr::read_unaligned((this + 0x34e8) as *const u16) as i32
        && read_usize(op + 0x1db) as u32 == *(KEY_CLUB_COMP_INTER_TOTO_CUP as *const u32)
}

/// True if the clubs are rivals (+0xb3/+0xb7/+0xbb, either direction).
///
/// [pt] Os clubes são rivais (+0xb3, +0xb7, +0xbb, nos dois sentidos)?
pub(crate) unsafe fn are_rival_clubs(a: usize, b: usize) -> bool {
    [0xb3usize, 0xb7, 0xbb].iter().any(|&o| a == read_usize(b + o) || b == read_usize(a + o))
}

/// True if the clubs' nations (+0x53) are rivals (+0x110/+0x114/+0x118, either direction).
///
/// [pt] As nações (+0x53) dos clubes são rivais (+0x110, +0x114, +0x118, nos dois sentidos)?
pub(crate) unsafe fn are_rival_nations(a: usize, b: usize) -> bool {
    let (na, nb) = (read_usize(a + 0x53), read_usize(b + 0x53));
    [0x110usize, 0x114, 0x118].iter().any(|&o| na == read_usize(nb + o) || nb == read_usize(na + o))
}

/// Utils random(n).
pub(crate) unsafe fn friendly_random(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// searches (bsearch) the candidates of the band (1 to 20, going up until found) and, from there, the first one that is
/// not the club, excluded, already an opponent, out by date, of crossed bands, a rival, of a rival nation (half the
/// time), from the same city or above the drawn band, that passes the nation check by the club reputation and is drawn
/// (1 in 5); else 0
/// Original: `Friendly::find_friendly_opponent` (0x5c4cb0, friendly.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x5c4cb0 } (thiscall, ret 0x14, friendly.cpp): `Friendly::find_friendly_opponent(clube, lista, n, faixa,
/// excluir)`: a faixa (byte com sinal) presa em 1..=20; busca (bsearch 0x946464 com 0x5c5270, pelo byte +5) na lista
/// de candidatos (0x16 bytes, `n`) um da faixa, subindo a faixa até achar (a partir de 20: o começo da lista). Dali
/// até o penúltimo candidato (o último fica de fora, como na original), pula: o próprio clube, os do vetor `excluir`
/// (4 ponteiros, pelo id), os que já têm o clube na lista (+0xa, o cursor volta ao começo), os fora pela data, os de
/// faixas cruzadas (+0xe/+0x10, −1 = qualquer), os rivais, os de nações rivais (meio a meio: sorteio(2) = 0), os da
/// mesma cidade (+0x57) e os acima da faixa do sorteio (+4 > sorteio(3) + 3). Pela reputação (+0x80) do clube: abaixo
/// de 2000 a mesma nação (+0x53); de 2000 a 3999 o mesmo +0x75 da nação; de 4000 a 7999 o mesmo +0x71; com o +4 do
/// clube acima de sorteio(2), a mesma nação. Fica com o candidato quando sorteio(5) = 1; nenhum: 0. Sem SEH, sem caixa.
pub(crate) unsafe extern "thiscall" fn find_friendly_opponent(this: usize, clube: usize, lista: usize, n: u32, band: u32, excluir: usize) -> usize {
    let mut band = band as i8;
    if band < 1 {
        band = 1;
    } else if band > 0x14 {
        band = 0x14;
    }
    let cp = friendly_club_by_id(read_usize(clube));
    let mut chave = [0u8; 0x16];
    chave[5] = band as u8;
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let mut c;
    loop {
        c = bsearch(chave.as_ptr() as usize, lista, n, 0x16, 0x5c5270);
        if chave[5] as i8 >= 0x14 {
            c = lista;
        }
        chave[5] = chave[5].wrapping_add(1);
        if c != 0 {
            break;
        }
    }
    let ultimo = lista.wrapping_add((n as usize).wrapping_mul(0x16)).wrapping_sub(0x16);
    while c < ultimo {
        'proximo: {
            let id = read_usize(c);
            let op = friendly_club_by_id(id);
            if cp == op {
                break 'proximo;
            }
            if excluir != 0 {
                for k in 0..4usize {
                    let e = read_usize(excluir + 4 * k);
                    if e != 0 && id == read_usize(e) {
                        break 'proximo;
                    }
                }
            }
            if already_opponent(c, clube) || excluded_by_intertoto(this, op) {
                break 'proximo;
            }
            let (fc, fcl) = (read_i16(c + 0xe), read_i16(clube + 0xe));
            if fc != -1 && fcl != -1 && (fc > read_i16(clube + 0x10) || fcl > read_i16(c + 0x10)) {
                break 'proximo;
            }
            if are_rival_clubs(cp, op) || (are_rival_nations(cp, op) && friendly_random(2) == 0) {
                break 'proximo;
            }
            if read_usize(cp + 0x57) == read_usize(op + 0x57) {
                break 'proximo;
            }
            if read_i8(c + 4) > (friendly_random(3) as i8).wrapping_add(3) {
                break 'proximo;
            }
            let rep = read_i16(cp + 0x80);
            let (nc, no) = (read_usize(cp + 0x53), read_usize(op + 0x53));
            let longe = if rep < 2000 {
                no != nc
            } else if rep < 4000 {
                read_i8(no + 0x75) != read_i8(nc + 0x75)
            } else if rep < 8000 {
                read_usize(no + 0x71) != read_usize(nc + 0x71)
            } else {
                false
            };
            if longe {
                break 'proximo;
            }
            if read_i8(clube + 4) as i32 > friendly_random(2) && no != nc {
                break 'proximo;
            }
            if friendly_random(5) == 1 { // o adversário do amistoso
                return c;
            }
        }
        c = c.wrapping_add(0x16);
    }
    0
}

/// Regions compared for both routines: RNG state plus the cursor (+0xa) of each candidate's list.
///
/// [pt] As regiões dos dois comparadores: o estado do sorteio e o cursor (+0xa) da lista de cada candidato.
pub(crate) unsafe fn candidate_regions(candidatos: impl Iterator<Item = usize>) -> Vec<(usize, usize)> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    for c in candidatos.filter(|&c| c != 0) {
        let l = read_usize(c + 0xa);
        if l != 0 {
            reg.push((l + 0xa, 4));
        }
    }
    reg
}

/// Comparer for `find_friendly_opponent` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_friendly_opponent(this: usize, clube: usize, lista: usize, n: u32, band: u32, excluir: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize>(IDX_FIND_FRIENDLY_OPPONENT);
    let reg = candidate_regions((0..(n as i32).max(0) as usize).map(|i| lista + 0x16 * i));
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, lista, n, band, excluir),
        || crate::aleatorio::sem_contar(|| find_friendly_opponent(this, clube, lista, n, band, excluir)));
    PLACAR.registrar(igual && r == m, || format!("find_friendly_opponent({:x}, {:x}, n {}, band {}) original={:x} rust={:x} {:x?} x {:x?}",
        clube, lista, n, band as i8, r, m, mo, mr));
    r
}

/// searches (bsearch) the candidates of the tour nation and, from its first one, the one not above the maximum band nor
/// out by date, the club itself, already an opponent, a rival, of a rival nation (half the time), from the same city or
/// above the drawn band, drawn half the time; stops at the first one of another nation; else 0
/// Original: `Friendly::find_tour_opponent` (0x5c5040, friendly.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x5c5040 } (thiscall, ret 0x14, friendly.cpp): `Friendly::find_tour_opponent(clube, ordenados, n, faixa_max,
/// nacao)`: busca (bsearch 0x946464 com 0x5c5290, pelo +6) no vetor de `n` ponteiros para candidatos, ordenado pela
/// nação, um da nação (o id, [nacao]) e volta ao primeiro dela. Dali até o penúltimo: o primeiro cujo clube (+0x53)
/// não é da nação encerra (0); pula os acima da faixa (+5 > faixa_max), os fora pela data, o próprio clube, os que já
/// têm o clube na lista (+0xa), os rivais, os de nações rivais (sorteio(2) = 0), os da mesma cidade (+0x57) e os acima
/// da faixa do sorteio (+4 > sorteio(3) + 3); fica com o candidato quando sorteio(2) = 1. Nenhum: 0. Sem SEH, sem caixa.
pub(crate) unsafe extern "thiscall" fn find_tour_opponent(this: usize, clube: usize, ordenados: usize, n: u32, faixa_max: u32, nacao: usize) -> usize {
    let cp = friendly_club_by_id(read_usize(clube));
    let mut chave = [0u8; 0x16];
    chave[5] = 0xff;
    core::ptr::write_unaligned(chave.as_mut_ptr().add(6) as *mut usize, read_usize(nacao));
    let pchave = chave.as_ptr() as usize;
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let mut s = bsearch(&pchave as *const usize as usize, ordenados, n, 4, 0x5c5290);
    if s == 0 {
        return 0;
    }
    if s > ordenados {
        loop {
            let anterior = read_usize(s - 4);
            s -= 4;
            if read_usize(anterior + 6) != read_usize(nacao) {
                s += 4;
                break;
            }
            if s <= ordenados {
                break;
            }
        }
    }
    let ultimo = ordenados.wrapping_add((n as usize).wrapping_mul(4)).wrapping_sub(4);
    while s < ultimo {
        let c = read_usize(s);
        let op = friendly_club_by_id(read_usize(c));
        if read_usize(op + 0x53) != nacao {
            return 0;
        }
        'proximo: {
            if read_i8(c + 5) > faixa_max as i8 || excluded_by_intertoto(this, op) || cp == op || already_opponent(c, clube) {
                break 'proximo;
            }
            if are_rival_clubs(cp, op) || (are_rival_nations(cp, op) && friendly_random(2) == 0) {
                break 'proximo;
            }
            if read_usize(cp + 0x57) == read_usize(op + 0x57) {
                break 'proximo;
            }
            if read_i8(c + 4) > (friendly_random(3) as i8).wrapping_add(3) {
                break 'proximo;
            }
            if friendly_random(2) == 1 { // o adversário da excursão
                return c;
            }
        }
        s += 4;
    }
    0
}

/// Comparer for `find_tour_opponent` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_find_tour_opponent(this: usize, clube: usize, ordenados: usize, n: u32, faixa_max: u32, nacao: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize>(IDX_FIND_TOUR_OPPONENT);
    let reg = candidate_regions((0..(n as i32).max(0) as usize).map(|i| read_usize(ordenados + 4 * i)));
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, ordenados, n, faixa_max, nacao),
        || crate::aleatorio::sem_contar(|| find_tour_opponent(this, clube, ordenados, n, faixa_max, nacao)));
    PLACAR.registrar(igual && r == m, || format!("find_tour_opponent({:x}, {:x}, n {}, band {}, nacao {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        clube, ordenados, n, faixa_max as i8, nacao, r, m, mo, mr));
    r
}

/// the friendly of the club (+0xc) in the friendly list (+0x34c8): with a date, those on that date (0x4a7f20), the
/// complete one (+0x10 and +0x14) or the second found at once, otherwise the last kept; without a date, the first
/// incomplete one with +8
/// Original: `find_friendly_match` (0x5c88f0, friendly.cpp, thiscall, ret 8).
///
/// [pt] { 0x5c88f0 } (thiscall, ret 8, friendly.cpp): `find_friendly_match(data, clube)`: o amistoso do clube na lista
/// de amistosos (+0x34c8; nula: 0), do começo (cursor +0xa = +2). Com `data`: os da mesma data (0x4a7f20 com os dois
/// dwords dela) e do clube (+0xc); o primeiro com +0x10 e +0x14 os dois não nulos sai na hora, os outros ficam
/// guardados (o último) e um segundo achado sai na hora; no fim, o guardado (ou 0). Sem `data`: o primeiro do clube
/// com +0x10 ou +0x14 nulo e +8 não nulo. A lista é relida do gerenciador a cada passo, como na original. Na marcação
/// e nas telas dos amistosos (0x5c6f32, 0x5c86de, 0x5c9139), na mídia (0x77ce23, 0x782d9e, 0x784314) e em 0x4741f9.
/// Sem SEH, sem caixa, sem locais.
pub(crate) unsafe extern "thiscall" fn find_friendly_match(this: usize, data: usize, clube: usize) -> usize {
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let mesma_data: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3D_3D);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let l = u(this + 0x34c8);
    if l == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, u(l + 2));
    let mut guardado = 0;
    let mut a = proximo(l);
    while a != 0 {
        let do_clube = u(a + 0xc) == clube; // o clube do amistoso
        if data != 0 {
            if mesma_data(a, u(data) as u32, u(data + 4) as u32) != 0 && do_clube {
                if guardado != 0 || (u(a + 0x10) != 0 && u(a + 0x14) != 0) {
                    return a;
                }
                guardado = a;
            }
        } else if do_clube && (u(a + 0x10) == 0 || u(a + 0x14) == 0) && u(a + 8) != 0 {
            return a;
        }
        a = proximo(u(this + 0x34c8));
    }
    guardado
}

comparar_valor!(comparar_find_friendly_match, find_friendly_match, IDX_FIND_FRIENDLY_MATCH, (data: usize, clube: usize) -> usize);



/// tells whether the competition is the friendlies one
/// Original: `Friendly::is_own_comp_record` (0x5c2990, friendly.cpp, thiscall).
///
/// [pt] { 0x5c2990 } (thiscall, ret 4, friendly.cpp): `competicao` é a competição dos amistosos (+4)?
pub(crate) unsafe extern "thiscall" fn is_own_comp_record(amistoso: usize, competicao: usize) -> u32 {
    (competicao == *((amistoso + 4) as *const usize)) as u32
}

/// fills an international date: day, month, span, year, continent and competition
/// Original: `friendly_international_date_set` (0x5cb9b0, friendly.cpp, thiscall).
///
/// [pt] { 0x5cb9b0 } (thiscall, ret 0x18, friendly.cpp): preenche a data internacional de 20 bytes: dia,
/// mês e duração (+0..+2), o ano a somar (+0xb), o continente (+0xc) e a competição (+0x10).
pub(crate) unsafe extern "thiscall" fn friendly_international_date_set(data: usize, dia: u8, mes: u8, duracao: u8, ano: u8, continente: u32, competicao: u32) {
    let b = data as *mut u8;
    *b = dia;
    *b.add(1) = mes;
    *b.add(2) = duracao;
    *b.add(0xb) = ano;
    core::ptr::write_unaligned(b.add(0xc) as *mut u32, continente);
    core::ptr::write_unaligned(b.add(0x10) as *mut u32, competicao);
}

/// The 210-byte manager record of entry *id ([registros+8] + id x 210).
///
/// [pt] O registro de técnico de 210 bytes da entrada `*id` ([registros+8]).
pub(crate) unsafe fn friendly_manager_record(registros: usize, id: *const i32) -> usize {
    (*((registros + 8) as *const usize) as isize + *id as isize * 210) as usize
}


/// Comparer for `is_own_comp_record` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_is_own_comp_record(amistoso: usize, competicao: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IS_OWN_COMP_RECORD)(amistoso, competicao);
    let n = is_own_comp_record(amistoso, competicao);
    PLACAR.registrar(r == n, || format!("is_own_comp_record({:#x}) original={} rust={}", competicao, r, n));
    r
}

/// Comparer for `friendly_international_date_set` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_friendly_international_date_set(data: usize, dia: u8, mes: u8, duracao: u8, ano: u8, continente: u32, competicao: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u8, u8, u8, u8, u32, u32)>(IDX_FRIENDLY_INTERNATIONAL_DATE_SET);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(data, 0x14)], || o(data, dia, mes, duracao, ano, continente, competicao),
        || friendly_international_date_set(data, dia, mes, duracao, ano, continente, competicao));
    PLACAR.registrar(igual, || format!("friendly_international_date_set {:x?} x {:x?}", mo, mr));
}

/// stores the opponent in the first empty slot of the club's friendly schedule and marks the request
/// Original: `Friendly::schedule_assign_opponent` (0x5c57d0, friendly.cpp, thiscall, ret 8).
///
/// [pt] { 0x5c57d0 } (thiscall, ret 8, friendly.cpp; `this` não é lido): `Friendly::schedule_assign_opponent(agenda,
/// adversario)`: na primeira das `[agenda+8]` (byte com sinal) vagas de 25 bytes (a partir de +0x19) sem adversário
/// (dword +0), grava o adversário e marca o pedido (+0x21 da vaga anterior, ou seja agenda + 25·i + 0x21).
pub(crate) unsafe extern "thiscall" fn schedule_assign_opponent(_this: usize, agenda: usize, adversario: usize) {
    let n = read_u8(agenda + 8) as i8;
    let mut i: i8 = 0;
    while i < n {
        let vaga = agenda + 25 * (i as usize + 1);
        if read_usize(vaga) == 0 {
            core::ptr::write_unaligned(vaga as *mut usize, adversario);
            *((agenda + 25 * i as usize + 0x21) as *mut u8) = 1;
            return;
        }
        i += 1;
    }
}

/// Comparer for `schedule_assign_opponent` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas vagas da agenda (até a última possível, 25 · (n + 1) + 4 bytes).
pub(crate) unsafe extern "thiscall" fn comparar_schedule_assign_opponent(this: usize, agenda: usize, adversario: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_SCHEDULE_ASSIGN_OPPONENT);
    let n = read_u8(agenda + 8) as i8;
    if n <= 0 {
        return o(this, agenda, adversario);
    }
    let reg = [(agenda, 25 * (n as usize + 1) + 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, agenda, adversario), || schedule_assign_opponent(this, agenda, adversario));
    PLACAR.registrar(igual, || format!("schedule_assign_opponent({:x}, {:x}) {:x?} x {:x?}", agenda, adversario, mo, mr));
}

/// stores the friendly and tournament objects in the competition table
/// Original: `Friendly::register_comp_objects` (0x5ca030, friendly.cpp, thiscall).
///
/// [pt] { 0x5ca030 } (thiscall, friendly.cpp): grava na tabela das competições ([0xadadfc]) o objeto dos
/// amistosos ([0xae2c74]) na chave [0x9cf668] e os torneios do amistoso (127 vagas de 0x69 bytes a partir
/// de +0xb1; com +0xb1 e +0x116 não nulos, +0x116 vai para a chave [+0xaf] + vaga).
pub(crate) unsafe extern "thiscall" fn register_comp_objects(amistoso: usize) {
    let tabela = || *(G_COMP_OBJECT_TABLE as *const usize) as isize;
    *((tabela() + *(KEY_CLUB_COMP_FRIENDLY as *const i32) as isize * 4) as *mut usize) = *(G_FRIENDLY as *const usize);
    for i in 0..0x7fusize {
        let p = amistoso + 0x116 + i * 0x69;
        let objeto = core::ptr::read_unaligned(p as *const usize);
        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && objeto != 0 {
            let chave = core::ptr::read_unaligned((amistoso + 0xaf) as *const i16) as isize + i as isize;
            *((tabela() + chave * 4) as *mut usize) = objeto;
        }
    }
}

/// Comparer for `register_comp_objects` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_register_comp_objects(amistoso: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REGISTER_COMP_OBJECTS);
    let tabela = *(G_COMP_OBJECT_TABLE as *const usize) as isize;
    let base = core::ptr::read_unaligned((amistoso + 0xaf) as *const i16) as isize;
    let reg = [((tabela + *(KEY_CLUB_COMP_FRIENDLY as *const i32) as isize * 4) as usize, 4), ((tabela + base * 4) as usize, 0x7f * 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(amistoso), || register_comp_objects(amistoso));
    PLACAR.registrar(igual, || format!("register_comp_objects {:x?} x {:x?}", mo, mr));
}

/// on the first day of the year empties the nation lists, sorts the nations by reputation and gives the international
/// dates their year by the World Cup cycle; then arranges the friendlies of the dates equal to today + 12 weeks
/// (0x5cbdf0), once a day
/// Original: `Friendly::update_international_friendlies` (0x5cbb00, friendly.cpp, thiscall).
///
/// [pt] { 0x5cbb00 } (thiscall, ret 0, friendly.cpp, quadro SEH da data local de destrutor vazio): `Friendly::
/// update_international_friendlies()`, pelo dia dos amistosos (0x5c2460, já em Rust). No primeiro dia do ano: as listas
/// de nações ([+0x34f6], [0xae2350] delas) se esvaziam (0x68d080), a lista em +0x34fa (conta +0x34fe) se ordena pela
/// reputação (0x5cc5f0) e cada data internacional (+0x34ec, 20 bytes, conta +0x34f0) ganha o ano pelo ciclo da Copa
/// (ano − 2 múltiplo de 4, mais o deslocamento +0xb, menos 4 se passar de 3 anos à frente; 0x5cbdd0), reordenadas pela
/// data (0x5cc610). Depois, se a data de 0xdd6ec8 não passa de hoje + 12 semanas (0x54c770) e o dia não foi feito
/// (+0x34f4), as entradas com a data de hoje + 12 semanas (bsearch e as vizinhas iguais) vão à 0x5cbdf0 e o dia fica
/// marcado.
pub(crate) unsafe extern "thiscall" fn update_international_friendlies(this: usize) {
    update_friendlies_body(this, false);
}

/// Comparer for `update_international_friendlies` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: no primeiro dia do ano (as listas se esvaziam, liberando) e quando há amistosos a marcar (a 0x5cbdf0
/// cria jogos e sorteia), só a original, conferindo o plano: o dia marcado em +0x34f4. Nos outros dias, lado a lado
/// pelo dia marcado e pelas datas estáticas.
pub(crate) unsafe extern "thiscall" fn comparar_update_international_friendlies(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_UPDATE_INTERNATIONAL_FRIENDLIES);
    if read_u16(G_CURRENT_DATE) == 0 {
        return o(this);
    }
    if let Some((primeira, quantas)) = update_friendlies_body(this, true) {
        o(this);
        let dia = read_u16(this + 0x34f4);
        PLACAR.registrar(dia == read_u16(G_CURRENT_DATE), || format!("update_international_friendlies(primeira {:x}, quantas {}) dia marcado={} hoje={}", primeira, quantas, dia, read_u16(G_CURRENT_DATE)));
        return;
    }
    let mut reg = CPY4_ESTATICAS.to_vec();
    reg.push((this + 0x34f4, 2));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || update_international_friendlies(this));
    PLACAR.registrar(igual, || format!("update_international_friendlies(sem amistosos) {:x?} x {:x?}", mo, mr));
}

/// in the friendlies list (+0x34cc), the 25-byte slot of the club with the requested day, year and opponent, or 0
/// Original: `Friendly::find_scheduled_friendly_slot` (0x5c6740, friendly.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5c6740 } (thiscall, ret 0x10, friendly.cpp): `Friendly::find_scheduled_friendly_slot(dia, ano, clube,
/// adversário)`: na lista dos amistosos (+0x34cc; o cursor volta ao começo), o primeiro registro do clube (+0) com uma
/// vaga (25 bytes a partir de +9, [+8] delas, com sinal) do dia (+0), ano (+2) e adversário (+0x10): a vaga. Senão 0.
pub(crate) unsafe extern "thiscall" fn find_scheduled_friendly_slot(this: usize, dia: u32, ano: u32, clube: usize, adversario: usize) -> usize {
    // ponytail: a Data local da original (0x549ef0) só guarda o dia e o ano em 16 bits (o bissexto não é lido)
    let (dia, ano) = (dia as u16, ano as u16);
    let l = read_usize(this + 0x34cc);
    if l == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut e = proximo(l);
    while e != 0 {
        if read_usize(e) == clube {
            for i in 0..(*((e + 8) as *const i8)).max(0) as usize {
                let v = e + 25 * i + 9;
                if *((v + 2) as *const u16) == ano && *(v as *const u16) == dia && read_usize(v + 0x10) == adversario {
                    crate::comparacao::ramo("cpk_4 vaga do amistoso");
                    return e + 25 * i + 9; // a vaga i
                }
            }
        }
        e = proximo(l);
    }
    0
}

/// Comparer for `find_scheduled_friendly_slot` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pelo valor (as duas voltam o cursor da lista ao começo e param no mesmo registro).
pub(crate) unsafe extern "thiscall" fn comparar_find_scheduled_friendly_slot(this: usize, dia: u32, ano: u32, clube: usize, adversario: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize>(IDX_FIND_SCHEDULED_FRIENDLY_SLOT)(this, dia, ano, clube, adversario);
    let n = find_scheduled_friendly_slot(this, dia, ano, clube, adversario);
    PLACAR.registrar(r == n, || format!("find_scheduled_friendly_slot({}/{}, {:x}, {:x}) original={:#x} rust={:#x}", dia as u16, ano as u16, clube, adversario, r, n));
    r
}

/// removes the club from the planned AI friendlies and from the friendly lists
/// Original: `Friendly::remove_club_from_ai_plans` (0x5c64e0, friendly.cpp, thiscall).
///
/// [pt] { 0x5c64e0 } (thiscall, ret 4, friendly.cpp): tira o `clube` dos amistosos planejados: no vetor
/// [+0x34d8] ([+0x34e0] registros de 0x14 bytes) o id do clube em +8 vira −1; na lista [+0x34c8]
/// (0x68d1b0), o ponteiro do clube em +0xc vira 0.
pub(crate) unsafe extern "thiscall" fn remove_club_from_ai_plans(amistosos: usize, clube: usize) {
    let vetor = (amistosos + 0x34d8) as *const usize;
    let n = (amistosos + 0x34e0) as *const i32;
    if *vetor != 0 {
        let mut i = 0;
        while i < *n {
            let id = (*vetor + 8 + 0x14 * i as usize) as *mut u32;
            if *id == *(clube as *const u32) {
                *id = 0xffff_ffff;
            }
            i += 1;
        }
    }
    let lista = (amistosos + 0x34c8) as *const usize;
    if *lista == 0 {
        return;
    }
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    core::ptr::write_unaligned((*lista + 0xa) as *mut u32, core::ptr::read_unaligned((*lista + 2) as *const u32));
    loop {
        let e = prox(*lista);
        if e == 0 {
            return;
        }
        if *((e + 0xc) as *const usize) == clube {
            *((e + 0xc) as *mut usize) = 0; // sai da lista
        }
    }
}

/// Comparer for `remove_club_from_ai_plans` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_remove_club_from_ai_plans(amistosos: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_REMOVE_CLUB_FROM_AI_PLANS);
    let mut regioes = vec![];
    let vetor = *((amistosos + 0x34d8) as *const usize);
    let n = *((amistosos + 0x34e0) as *const i32);
    if vetor != 0 && n > 0 {
        regioes.push((vetor, n as usize * 0x14));
    }
    let lista = *((amistosos + 0x34c8) as *const usize);
    if lista != 0 {
        // o cursor e o ponteiro +0xc de cada amistoso da lista (percorrida aqui antes; as duas
        // rotinas reiniciam o cursor)
        regioes.push((lista + 0xa, 4));
        let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
        loop {
            let e = prox(lista);
            if e == 0 {
                break;
            }
            regioes.push((e + 0xc, 4));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(amistosos, clube),
        || remove_club_from_ai_plans(amistosos, clube));
    PLACAR.registrar(igual, || format!("remove_club_from_ai_plans({:#x}) {:x?} x {:x?}", clube, mo, mr));
}

/// draws the nation for a club tour, nearer the lower the reputation
/// Original: `Friendly::pick_tour_nation` (0x5c4680, friendly.cpp, thiscall).
///
/// [pt] { 0x5c4680 } (thiscall, ret 4, friendly.cpp): sorteia o país da excursão do clube. Pela reputação
/// (+0x80) o alcance: ≥ 8000 qualquer país, ≥ 6000 e ≥ 4000 o mesmo continente (+0x71), ≥ 2000 o mesmo
/// continente e a mesma região (+0x75); abaixo, nenhum (0). Candidatos: os outros países (lista
/// [0xae23a8], 0x122 bytes, [0xae2350] países) com +0x86 ≥ 20, +0x7e = 1, +0x84 ≤ 2 e +0x8e ≥ 5; cada um
/// tira 0x90cfc0(100) e fica o maior (estrito, comparado como byte com sinal).
pub(crate) unsafe extern "thiscall" fn pick_tour_nation(_this: usize, clube: usize) -> usize {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let proprio = *((clube + club::NATION) as *const usize);
    let alcance: i8 = match *((clube + club::REPUTATION) as *const i16) {
        8000.. => 5,
        6000.. => 4,
        4000.. => 3,
        2000.. => 2,
        _ => return 0,
    };
    let mut escolhido = 0usize;
    let mut melhor: i8 = -1;
    if *(G_NATION_COUNT as *const i32) <= 0 {
        return 0;
    }
    let mut p = *(G_NATION_TABLE as *const usize);
    let mut i: i16 = 0;
    loop {
        let candidato = p != proprio
            && *((p + 0x86) as *const i16) >= 0x14
            && *((p + 0x7e) as *const u8) == 1
            && *((p + 0x84) as *const i8) <= 2
            && *((p + 0x8e) as *const i16) >= 5;
        if candidato {
            let continente = || *((p + 0x71) as *const u32) == *((proprio + 0x71) as *const u32);
            let perto = match alcance {
                2 => *((p + 0x75) as *const u8) == *((proprio + 0x75) as *const u8) && continente(),
                5.. => true,
                _ => continente(),
            };
            if perto {
                let r = sorteio(100) as i8;
                if r > melhor {
                    melhor = r;
                    escolhido = p;
                }
            }
        }
        i = i.wrapping_add(1);
        p += 0x122;
        if i as i32 >= *(G_NATION_COUNT as *const i32) {
            return escolhido;
        }
    }
}

/// Comparer for `pick_tour_nation` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_pick_tour_nation(this: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> usize>(IDX_PICK_TOUR_NATION);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube),
        || crate::aleatorio::sem_contar(|| pick_tour_nation(this, clube)));
    PLACAR.registrar(igual && r == n, || format!("pick_tour_nation({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

/// the date (+3) through 0x54a130 from the day (+1), month (+2), given year and weekday (+0); the high half of the
/// arguments comes from the year (the caller eax)
/// Original: `friendly_international_date_resolve` (0x5cbdd0, friendly.cpp, thiscall, ret 4).
///
/// [pt] { 0x5cbdd0 } (thiscall, ret 4, friendly.cpp): `friendly_international_date_resolve(ano)`: a data (+3) da entrada
/// de data internacional `this` pela 0x54a130 (Date.cpp: dia, mês, ano, dia da semana) com o dia (+1) e o dia da
/// semana (+0) em byte com sinal e o mês (+2). A original monta os argumentos sobre o eax e o edx de entrada: o edx é o
/// ano (metade alta do dia), o eax é o ano também no único chamador (0x5cbc18, `push eax; call`; metade alta do mês e
/// do dia da semana). O eax devolvido é lixo: o endereço de pilha do temporário que a 0x417c10 copia no fim da 0x54a2d0
/// (muda com a profundidade da pilha), e o chamador o descarta (0x5cbc1d `mov eax, [esp+14h]`); só a data (+3..+0xb) conta.
pub(crate) unsafe extern "thiscall" fn friendly_international_date_resolve(this: usize, ano: u32) -> u32 {
    let acertar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_DATE_SET);
    let b = |o: usize| *((this + o) as *const i8) as i16 as u16 as u32;
    let alto = ano & 0xffff_0000;
    let semana = alto | b(0);
    let mes = alto | (b(0) & 0xff00) | *((this + 2) as *const u8) as u32;
    acertar(this + 3, alto | b(1), mes, ano, semana) // a data com o ano dado
}

/// Comparer for `friendly_international_date_resolve` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_friendly_international_date_resolve(this: usize, ano: u32) -> u32 {
    let o = original_fn::<usize>(IDX_FRIENDLY_INTERNATIONAL_DATE_RESOLVE);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 11)], || call_original_with_year(o, this, ano),
        || friendly_international_date_resolve(this, ano));
    PLACAR.registrar(igual /* o eax é lixo de pilha: só a memória */, || format!("friendly_international_date_resolve({:x}, {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        this, ano, r, n, mo, mr));
    r
}
