// ---------- %%LEVA%%a leva: a copa e as transferências: os jogos da rodada levados às datas marcadas, o jogador que sai da troca e as permissões de trabalho da oferta (Cup.cpp, transfer_manager.cpp) ----------

unsafe fn cqc7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqc7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc7_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc7_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqc7_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqc7_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c] -> thiscall(notícias, clube),
/// ret 4).
unsafe fn cqc7_humano(clube: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(cqc7_d(0xb7431c), clube) != 0
}

/// A pessoa (0x6e bytes em [0xae23bc]) e o clube (0x245 bytes em [0xae23b4]) pelo id.
unsafe fn transfer_manager_staff_by_id(id: usize) -> usize {
    cqc7_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn transfer_manager_club_by_id(id: usize) -> usize {
    cqc7_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4), gravado em [0xae1080].
unsafe fn transfer_manager_get_contract_stored(pessoa: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqc7_d(0xae1050), pessoa);
    cqc7_pd(0xae1080, c);
    c
}

/// Lista vazia (palavra +0 = 0) da oferta (+4): destruída (0x68d040), liberada (0x944c94) e +4 = 0, como a original
/// (que lê a palavra antes de testar o ponteiro).
unsafe fn free_empty_offer_list(oferta: usize) {
    let l = cqc7_d(oferta + 4);
    if cqc7_w(l) != 0 {
        return;
    }
    if l != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(l);
        liberar(l);
    }
    cqc7_pd(oferta + 4, 0);
}

/// A entrada da rodada (0x68 bytes em [copa + 0xa3], o índice como o dword que a original multiplica).
unsafe fn cup_round_entry(copa: usize, rodada: u32) -> usize {
    cqc7_d(copa + 0xa3).wrapping_add((rodada as usize).wrapping_mul(0x68))
}

/// Os jogos da copa na data e fatia da rodada, numa lista (0x5abd90: push lista; push liga +0x44; push rodada; push
/// id; push fatia +6; push dia; push ano; mov ecx, [0xae2a58] -> thiscall, ret 0x1c).
unsafe fn collect_round_fixtures(copa: usize, data: usize, rodada: u32, lista: usize) -> u32 {
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x5abd90usize);
    // ponytail: a original empurra o dia, a fatia e o tipo em registradores com sobra nos bytes altos; a 0x5abd90 só
    // lê as palavras e os bytes
    jogos(cqc7_d(0xae2a58), cqc7_w(data + 2) as u16 as u32, cqc7_w(data) as u16 as u32, cqc7_b(cup_round_entry(copa, rodada) + 6) as u32,
        cqc7_d(cqc7_d(copa + 4)) as u32, rodada, cqc7_b(copa + 0x44) as u32, lista) & 0xffff
}

/// O jogo `f` está na data marcada da rodada: dia (+0x2a, sem sinal) = +3 da rodada, ano (+0x28) = +5 + o da copa
/// (+0x40), fatia (+0x3f) = +6 e o tipo (+0x42) da liga.
unsafe fn fixture_on_round_date(copa: usize, e: usize, f: usize) -> bool {
    cqc7_w(f + 0x2a) as u16 as i32 == cqc7_w(e + 3) as i32
        && cqc7_w(f + 0x28) as u16 as i32 == cqc7_b(e + 5) as i8 as i32 + cqc7_w(copa + 0x40) as i32
        && cqc7_b(f + 0x3f) as i32 == cqc7_b(e + 6) as i8 as i32
        && cqc7_b(f + 0x42) == cqc7_b(copa + 0x44)
}

/// { 0x51ebe0 } (thiscall, ret 8, Cup.cpp, quadro SEH sem destrutor no caminho normal: as notícias, a data e a lista
/// locais): `Cup::move_round_fixtures_to_set_dates(data, rodada)`, da montagem da rodada (0x51c800, em Rust). A notícia
/// 0x14 local é montada logo (0x77bbe0). Rodada (0x68 bytes em +0xa3) com o bit 0x100 em +0xd e a primeira vaga
/// marcada (+0x29 ≠ 0xff): os jogos da copa na data e fatia (collect_round_fixtures). Com jogos, a notícia leva o id da
/// copa, +0x3a, o código da rodada (+7) e a rodada de duas mãos (+0x21 = 2). Cada jogo do tipo da liga (+0x42) ocupa
/// uma vaga (7 bytes a partir de +0x23: +0 regras do estádio, +4 dia da semana, +5 fatia, +6 tipo; até 8, até a
/// primeira 0xff): na data da rodada, vai ao dia da semana da vaga (0x549ef0 num bloco de 8 bytes de 0x944cf1, 0x54a2d0,
/// liberado depois), com a notícia 3 aos clubes fora do tipo 3, sai do calendário (0x5abc30), ganha a data, a fatia e
/// o tipo da vaga, o estádio (0x85c260) e volta (0x5aa680); sem a copa ainda em [0xadadfc], para; senão a vaga fica
/// 0xff. Com ou sem a mudança, a notícia 0x14 vai aos clubes se um é de técnico humano. Sem vaga (fim das vagas ou
/// vaga vazia): só a notícia, com os dados do jogo.
unsafe extern "thiscall" fn move_round_fixtures_to_set_dates(this: usize, data: usize, rodada: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut noticia = [0u32; 0x38];
    let n1 = noticia.as_mut_ptr() as usize;
    construir(n1, 0x14, 0);
    let id = cqc7_d(cqc7_d(this + 4));
    let e = cup_round_entry(this, rodada);
    if cqc7_w(e + 0xd) as u16 & 0x100 == 0 || cqc7_b(e + 0x29) == 0xff {
        return;
    }
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    if collect_round_fixtures(this, data, rodada, l) != 0 {
        gravar(n1, 0, cqc7_d(cqc7_d(this + 4)) as u32);
        gravar(n1, 1, cqc7_w(this + 0x3a) as i32 as u32);
        gravar(n1, 2, cqc7_w(cup_round_entry(this, rodada) + 7) as i32 as u32);
        gravar(n1, 5, (cqc7_b(cup_round_entry(this, rodada) + 0x21) == 2) as u32);
        // a notícia 0x14 do jogo (dados 3, 4, 6, 7, 8) aos dois clubes, se um deles é de técnico humano
        let noticia_do_jogo = |c1: usize, c2: usize, i1: usize, i2: usize, est: usize, dia: u16, ano: u16| {
            if cqc7_humano(c1) || cqc7_humano(c2) {
                if cqc7_humano(c1) {
                    gravar(n1, 3, i1 as u32);
                    gravar(n1, 4, i2 as u32);
                } else {
                    gravar(n1, 3, i2 as u32);
                    gravar(n1, 4, i1 as u32);
                }
                gravar(n1, 6, est as u32);
                gravar(n1, 7, dia as u32);
                gravar(n1, 8, ano as u32);
                entregar(cqc7_d(0xb7431c), n1, c1);
                entregar(cqc7_d(0xb7431c), n1, c2);
            }
        };
        cqc7_pd(l + 0xa, cqc7_d(l + 2));
        let mut j = 0usize;
        let mut f = proximo(l);
        while f != 0 {
            if cqc7_b(this + 0x44) == cqc7_b(f + 0x42) {
                let e = cup_round_entry(this, rodada);
                if j < 0x38 && cqc7_b(e + j + 0x29) != 0xff {
                    let mut buf = [0u8; 0x50];
                    let r = buf.as_mut_ptr() as usize;
                    core::ptr::copy_nonoverlapping(f as *const u8, r as *mut u8, 0x4f);
                    if fixture_on_round_date(this, e, f) {
                        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
                        let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
                        let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
                        let ajustar: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x54a2d0usize);
                        let p = alocar(8);
                        let dp = if p != 0 { nova(p, cqc7_w(f + 0x2a) as u16 as u32, cqc7_w(f + 0x28) as u16 as u32) } else { 0 };
                        let mut t = [0u8; 8];
                        let pt = t.as_mut_ptr() as usize;
                        padrao(pt);
                        let e = cup_round_entry(this, rodada);
                        ajustar(dp, cqc7_b(e + j + 0x27) as i8 as i16 as u16 as u32, pt); // cqc7: o dia da semana marcado
                        if cqc7_b(cup_round_entry(this, rodada) + j + 0x29) != 3 {
                            let mut noticia2 = [0u32; 0x38];
                            let n2 = noticia2.as_mut_ptr() as usize;
                            construir(n2, 3, 0);
                            gravar(n2, 0, cqc7_d(r + 0xc) as u32);
                            gravar(n2, 1, cqc7_d(r + 0x10) as u32);
                            gravar(n2, 2, cqc7_d(r) as u32);
                            gravar(n2, 3, cqc7_w(r + 0x32) as i32 as u32);
                            gravar(n2, 4, cqc7_w(r + 0x30) as i32 as u32);
                            gravar(n2, 5, cqc7_w(r + 0x2a) as u16 as u32);
                            gravar(n2, 6, cqc7_w(r + 0x28) as u16 as u32);
                            gravar(n2, 7, cqc7_b(r + 0x3f) as u32);
                            gravar(n2, 8, cqc7_w(pt) as i32 as u32);
                            gravar(n2, 9, cqc7_w(pt + 2) as u16 as u32);
                            gravar(n2, 10, cqc7_b(cup_round_entry(this, rodada) + j + 0x28) as i8 as i32 as u32);
                            gravar(n2, 11, cqc7_b(cup_round_entry(this, rodada) + j + 0x29) as i8 as i32 as u32);
                            entregar(cqc7_d(0xb7431c), n2, cqc7_d(r + 0x1c));
                            entregar(cqc7_d(0xb7431c), n2, cqc7_d(r + 0x20));
                        }
                        let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5abc30usize);
                        tirar(cqc7_d(0xae2a58), r);
                        crate::comparison::branch("cqc_7 jogo levado a data");
                        cqc7_pw(r + 0x2a, cqc7_w(pt) as u16); // cqc7: o dia marcado
                        cqc7_pw(r + 0x28, cqc7_w(pt + 2) as u16);
                        let v = cup_round_entry(this, rodada) + j;
                        cqc7_pb(r + 0x3f, cqc7_b(v + 0x28)); // cqc7: a fatia marcada
                        cqc7_pb(r + 0x3b, cqc7_b(v + 0x29));
                        let estadio: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x85c260usize);
                        estadio(cqc7_d(0xdd7ef4), cqc7_d(cup_round_entry(this, rodada) + j + 0x23), r, cqc7_d(this + 0x24));
                        let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
                        juntar(cqc7_d(0xae2a58), r, 1);
                        if dp != 0 {
                            let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
                            liberar(dp);
                        }
                        if cqc7_d(cqc7_d(0xadadfc) + id * 4) == 0 {
                            destruir(l);
                            return;
                        }
                        cqc7_pb(cup_round_entry(this, rodada) + j + 0x29, 0xff);
                    }
                    noticia_do_jogo(cqc7_d(r + 0x1c), cqc7_d(r + 0x20), cqc7_d(r + 0xc), cqc7_d(r + 0x10), cqc7_d(r + 4),
                        cqc7_w(r + 0x2a) as u16, cqc7_w(r + 0x28) as u16);
                    j += 7;
                } else {
                    noticia_do_jogo(cqc7_d(f + 0x1c), cqc7_d(f + 0x20), cqc7_d(f + 0xc), cqc7_d(f + 0x10), cqc7_d(f + 4),
                        cqc7_w(f + 0x2a) as u16, cqc7_w(f + 0x28) as u16);
                }
            }
            f = proximo(l);
        }
    }
    destruir(l);
}

/// Comparador da 0x51ebe0: só a original (os jogos saem do calendário e voltam pela 0x5aa680, que aloca, e as notícias
/// vão aos técnicos). O plano, feito antes: os jogos da copa na data e fatia (0x5abd90 numa lista provisória), na
/// ordem da lista; cada um do tipo da liga ocupa a vaga seguinte (de 7 em 7 bytes, até 0x38 e até a primeira 0xff), e
/// a vaga do que está na data da rodada fica 0xff; as outras vagas não mudam. Sem a copa em [0xadadfc] depois: nada a
/// conferir.
unsafe extern "thiscall" fn comparar_move_round_fixtures_to_set_dates(this: usize, data: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let e = cup_round_entry(this, rodada);
    if cqc7_w(e + 0xd) as u16 & 0x100 == 0 || cqc7_b(e + 0x29) == 0xff {
        return o(this, data, rodada);
    }
    let id = cqc7_d(cqc7_d(this + 4));
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let mut esperado = [0u8; 8];
    for k in 0..8 {
        esperado[k] = cqc7_b(e + k * 7 + 0x29);
    }
    if collect_round_fixtures(this, data, rodada, l) != 0 {
        cqc7_pd(l + 0xa, cqc7_d(l + 2));
        let mut j = 0usize;
        let mut f = proximo(l);
        while f != 0 {
            if cqc7_b(this + 0x44) == cqc7_b(f + 0x42) && j < 0x38 && esperado[j / 7] != 0xff {
                if fixture_on_round_date(this, e, f) {
                    esperado[j / 7] = 0xff;
                }
                j += 7;
            }
            f = proximo(l);
        }
    }
    destruir(l);
    o(this, data, rodada);
    if cqc7_d(cqc7_d(0xadadfc) + id * 4) == 0 {
        return; // ponytail: a copa saiu do jogo; nada a conferir
    }
    let e = cup_round_entry(this, rodada);
    let depois: [u8; 8] = core::array::from_fn(|k| cqc7_b(e + k * 7 + 0x29));
    SCOREBOARD.record(depois == esperado, || format!("move_round_fixtures_to_set_dates(rodada {}) vagas {:02x?} (esperado {:02x?})",
        rodada, depois, esperado));
}

/// { 0x8c41c0 } (thiscall, ret 0x10, transfer_manager.cpp, quadro SEH sem destrutor no caminho normal: a lista local):
/// `TRANSFER_MANAGER::remove_exchange_player(pessoa, oferta, só_esta, motivo)`, da oferta de contrato ao jogador
/// (0x8c1e40), da confirmação pelo clube (0x8c3310), da soltura dos jogadores da oferta (0x8c6ae0) e da tela. Só esta:
/// sem a lista da troca (+4), nada; com a pessoa nela (+0 de cada item = id), sai (0x68d1f0), o registro de 8 bytes dela
/// ([this+0x86e] + id × 8, +2) perde uma oferta, a notícia (0x8de150 com o valor = +0x52 da pessoa + +0x1b da oferta e o
/// motivo) e, com o dono do contrato do alvo (0x4cf5a0, fora do tipo 5) do computador (0x600a50), a resposta dele
/// (0x8c8090 com &oferta e 1): oferta resolvida (status 2, 3, 0x14, 0x10, 4, ou 0xf com +0x2e ≥ 3) só solta a lista
/// vazia; senão o preço (+0x1b) vira o valor e a oferta segue (0x8c2f50). No fim, a lista vazia sai. Senão, com a pessoa
/// em alguma oferta (+2 do registro): as ofertas dela pelo clube (0x8c5e40 numa lista local), menos esta; em cada uma, sai
/// da lista da troca (a lista vazia sai) e, desde a primeira que a tinha, o preço vira +0x52 + o preço, a notícia (motivo
/// 0) e a oferta segue.
unsafe extern "thiscall" fn remove_exchange_player(this: usize, pessoa: usize, oferta: usize, so_esta: u32, motivo: u32) {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) = core::mem::transmute(0x8de150usize);
    let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let menos_uma = |p: usize| {
        let a = cqc7_d(this + 0x86e) + cqc7_d(p) * 8 + 2;
        cqc7_pb(a, cqc7_b(a).wrapping_sub(1));
    };
    let mut oferta = oferta;
    if so_esta != 0 {
        let c = transfer_manager_get_contract_stored(transfer_manager_staff_by_id(cqc7_d(oferta + 0xc)));
        let dono = if c != 0 && cqc7_b(c + 0x35) & 0x3f != 5 { transfer_manager_club_by_id(cqc7_d(c + 4)) } else { 0 };
        let troca = cqc7_d(oferta + 4);
        if troca == 0 {
            destruir(l);
            return;
        }
        cqc7_pd(troca + 0xa, cqc7_d(troca + 2));
        let mut it = proximo(cqc7_d(oferta + 4));
        while it != 0 && cqc7_d(pessoa) != cqc7_d(it) {
            it = proximo(cqc7_d(oferta + 4));
        }
        if it != 0 {
            tirar(cqc7_d(oferta + 4), it);
            menos_uma(pessoa);
            let v = cqc7_d(pessoa + 0x52).wrapping_add(cqc7_d(oferta + 0x1b));
            noticia(this, oferta, pessoa, v, motivo);
            let human_club: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            if dono != 0 && human_club(dono, 0, 1, 0, 0) == 0 {
                let resposta: unsafe extern "thiscall" fn(usize, *mut usize, u32) = core::mem::transmute(0x8c8090usize);
                resposta(this, &mut oferta, 1);
                let s = cqc7_b(oferta + 0x2c);
                if matches!(s, 2 | 3 | 0x14 | 0x10 | 4) || (s == 0xf && cqc7_b(oferta + 0x2e) >= 3) {
                    free_empty_offer_list(oferta);
                    destruir(l);
                    return;
                }
            }
            crate::comparison::branch("cqc_7 sai da troca da oferta");
            cqc7_pd(oferta + 0x1b, v);
            seguir(this, oferta);
        }
        free_empty_offer_list(oferta);
    } else {
        if cqc7_b(cqc7_d(this + 0x86e) + cqc7_d(pessoa) * 8 + 2) != 0 {
            let trocas: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8c5e40usize);
            trocas(this, transfer_manager_staff_by_id(cqc7_d(oferta + 0xc)), l);
            cqc7_pd(l + 0xa, cqc7_d(l + 2));
            let mut achou = false;
            let mut it = proximo(l);
            while it != 0 {
                let o = cqc7_d(cqc7_d(this) + cqc7_d(it) * 4);
                if o != oferta {
                    if cqc7_d(o + 4) != 0 {
                        let t = cqc7_d(o + 4);
                        cqc7_pd(t + 0xa, cqc7_d(t + 2));
                        let mut x = proximo(cqc7_d(o + 4));
                        while x != 0 {
                            if cqc7_d(pessoa) == cqc7_d(x) {
                                achou = true;
                                tirar(cqc7_d(o + 4), x);
                                menos_uma(pessoa);
                                break;
                            }
                            x = proximo(cqc7_d(o + 4));
                        }
                        free_empty_offer_list(o);
                    }
                    if achou {
                        crate::comparison::branch("cqc_7 sai da troca das outras");
                        let v = cqc7_d(pessoa + 0x52).wrapping_add(cqc7_d(o + 0x1b));
                        noticia(this, o, pessoa, v, 0);
                        cqc7_pd(o + 0x1b, v);
                        seguir(this, o);
                    }
                }
                it = proximo(l);
            }
        }
    }
    destruir(l);
}

/// Comparador da 0x8c41c0: só a original (os nós da troca são liberados, a notícia vai aos técnicos e a oferta segue
/// pela 0x8c2f50, que pode fechar a transferência e tirar outros jogadores das trocas, ou ser respondida pelo vendedor
/// do computador, 0x8c8090, que pode mudar a oferta da vaga).
unsafe extern "thiscall" fn comparar_remove_exchange_player(this: usize, pessoa: usize, oferta: usize, so_esta: u32, motivo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(%%I1%%)(this, pessoa, oferta, so_esta, motivo) // ponytail: o efeito segue por outras rotinas
}

/// A permissão de trabalho da pessoa no clube: precisa (0x8c7560: push 0; push clube; push pessoa; mov ecx, this ->
/// thiscall, ret 0xc), a proposta de contrato dela ao clube (0x4cc9e0: push id do clube; push id da pessoa; mov ecx,
/// [0xae1050] -> thiscall, ret 8) e a concessão (0x8c7950: push 0; push 0; push proposta; push nação do clube +0x53; push
/// pessoa; mov ecx, this -> thiscall, ret 0x14, sem sorteio). Devolve (precisa, proposta, concedida).
unsafe fn check_work_permit(this: usize, pessoa: usize, clube: usize) -> (bool, usize, bool) {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    if precisa(this, pessoa, clube, 0) == 0 {
        return (false, 0, false);
    }
    let proposta: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x4cc9e0usize);
    let r = proposta(cqc7_d(0xae1050), cqc7_d(pessoa) as u32, cqc7_d(clube) as u32);
    if r == 0 {
        return (true, 0, false);
    }
    let concedida: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7950usize);
    (true, r, concedida(this, pessoa, cqc7_d(clube + 0x53), r, 0, 0) != 0)
}

/// { 0x8c7690 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::check_offer_work_permits(oferta, quieto)`,
/// do processamento da oferta (status 0xa, 0x8c6880) e da conclusão (0x8cbc40). O dono do contrato do alvo (0x4cf5a0,
/// [0xae1080]; qualquer tipo). Alvo (+0xc) com permissão a pedir no comprador (+8) e proposta, sem concessão
/// (check_work_permit): a oferta desfeita (0x8de790 com 0x13), status 0x10, as propostas fechadas (0x8c6db0), as marcas de
/// lance saem (0x84aeb0 no gerente de listas [this+0x856]: o comprador e o alvo, depois o dono do contrato do alvo e cada
/// jogador da troca), reagendada em 10 dias (0x8c6220) e devolve 0. Concedida (ou sem proposta) e sem `quieto`: a
/// notícia da concessão (0x8e3130). Depois, cada jogador da troca (+4) no dono do contrato do alvo: recusado, a notícia
/// (0x8e35a0 com o valor +0x52 + o preço), sai da troca (0x68d1f0), o registro de 8 bytes dele perde uma oferta e o
/// preço (+0x1b) vira o valor; concedido sem `quieto`, a notícia. A lista vazia sai; sem `quieto`, a oferta segue
/// (0x8c2f50). Devolve 1.
unsafe extern "thiscall" fn check_offer_work_permits(this: usize, oferta: usize, quieto: u32) -> u32 {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let concedida_noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x8e3130usize);
    let alvo = transfer_manager_staff_by_id(cqc7_d(oferta + 0xc));
    let comprador = transfer_manager_club_by_id(cqc7_d(oferta + 8));
    let c = transfer_manager_get_contract_stored(alvo);
    let dono = if c != 0 { transfer_manager_club_by_id(cqc7_d(c + 4)) } else { 0 };
    let (precisa, proposta, concedida) = check_work_permit(this, alvo, comprador);
    if precisa {
        if proposta != 0 && !concedida {
            crate::comparison::branch("cqc_7 permissao recusada");
            let desfeita: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8de790usize);
            let fechar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c6db0usize);
            let desmarcar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x84aeb0usize);
            let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
            desfeita(this, oferta, comprador, 0x13);
            cqc7_pb(oferta + 0x2c, 0x10); // cqc7: a oferta cai pela permissão
            fechar(this, oferta);
            let alvo = transfer_manager_staff_by_id(cqc7_d(oferta + 0xc));
            let c = transfer_manager_get_contract_stored(alvo);
            let dono = if c != 0 { transfer_manager_club_by_id(cqc7_d(c + 4)) } else { 0 };
            desmarcar(cqc7_d(this + 0x856), transfer_manager_club_by_id(cqc7_d(oferta + 8)), alvo);
            if cqc7_d(oferta + 4) != 0 {
                let t = cqc7_d(oferta + 4);
                cqc7_pd(t + 0xa, cqc7_d(t + 2));
                let mut it = proximo(cqc7_d(oferta + 4));
                while it != 0 {
                    desmarcar(cqc7_d(this + 0x856), dono, transfer_manager_staff_by_id(cqc7_d(it)));
                    it = proximo(cqc7_d(oferta + 4));
                }
            }
            agendar(this, oferta, 0xa, 0);
            return 0;
        }
        if quieto == 0 {
            concedida_noticia(this, alvo, comprador, oferta);
        }
    }
    if cqc7_d(oferta + 4) != 0 {
        let t = cqc7_d(oferta + 4);
        cqc7_pd(t + 0xa, cqc7_d(t + 2));
        let mut it = proximo(cqc7_d(oferta + 4));
        while it != 0 {
            let x = transfer_manager_staff_by_id(cqc7_d(it));
            let (precisa, proposta, concedida) = check_work_permit(this, x, dono);
            if precisa {
                if proposta != 0 && !concedida {
                    crate::comparison::branch("cqc_7 troca recusada pela permissao");
                    let v = cqc7_d(x + 0x52).wrapping_add(cqc7_d(oferta + 0x1b));
                    let recusada: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x8e35a0usize);
                    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
                    recusada(this, oferta, x, v);
                    tirar(cqc7_d(oferta + 4), it);
                    let a = cqc7_d(this + 0x86e) + cqc7_d(x) * 8 + 2;
                    cqc7_pb(a, cqc7_b(a).wrapping_sub(1));
                    cqc7_pd(oferta + 0x1b, v);
                } else if quieto == 0 {
                    concedida_noticia(this, x, dono, oferta);
                }
            }
            it = proximo(cqc7_d(oferta + 4));
        }
        free_empty_offer_list(oferta);
    }
    if quieto == 0 {
        let seguir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c2f50usize);
        seguir(this, oferta);
    }
    1
}

/// Comparador da 0x8c7690: só a original (as notícias alocam, a oferta é reagendada e segue pela 0x8c2f50). O plano,
/// feito antes pelas consultas sem efeito (0x8c7560, 0x4cc9e0, 0x8c7950 sem sorteio): o alvo recusado devolve 0 com a
/// oferta no status 0x10; senão 1.
unsafe extern "thiscall" fn comparar_check_offer_work_permits(this: usize, oferta: usize, quieto: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    let (precisa, proposta, concedida) = check_work_permit(this, transfer_manager_staff_by_id(cqc7_d(oferta + 0xc)), transfer_manager_club_by_id(cqc7_d(oferta + 8)));
    let recusada = precisa && proposta != 0 && !concedida;
    let r = o(this, oferta, quieto);
    let ok = if recusada { r == 0 && cqc7_b(oferta + 0x2c) == 0x10 } else { r == 1 };
    SCOREBOARD.record(ok, || format!("check_offer_work_permits(oferta {:x}) recusada prevista {} devolveu {}", oferta, recusada, r));
    r
}
