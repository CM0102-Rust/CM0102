// ---------- %%LEVA%%a leva: a liga: os jogos levados para a TV, a notícia dos grupos e a notícia da tabela (league.cpp) ----------

unsafe fn cqc6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqc6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqc6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqc6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqc6_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqc6_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c] -> thiscall(notícias, clube),
/// ret 4).
unsafe fn cqc6_humano(clube: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(cqc6_d(0xb7431c), clube) != 0
}

/// A entrada da tabela das datas da TV da liga (0x41 bytes em [+0xba], conta +0xa9) para a data, como a 0x685640: sem
/// entradas, data depois da última (ano +2 + o da liga +0x40, sem sinal; dia +0 com sinal) ou sem a entrada pela busca
/// (0x946464: push 0x685cf0; push 0x41; push conta; push tabela; push &chave; add esp, 0x14 -> cdecl bsearch; a chave
/// é o dia e o ano menos o da liga) ou com a primeira vaga vazia (+0xb = 0xff): nenhuma.
unsafe fn tv_date_entry(this: usize, data: usize) -> Option<usize> {
    let n = cqc6_w(this + 0xa9);
    if n == 0 {
        return None;
    }
    let tab = cqc6_d(this + 0xba);
    let base = cqc6_w(this + 0x40);
    let ult = (tab as isize + n as isize * 0x41 - 0x41) as usize;
    let ult_ano = cqc6_w(ult + 2) as u16 as i32 + base as i32;
    let ano = cqc6_w(data + 2) as u16 as i32;
    if ano > ult_ano || (ano == ult_ano && cqc6_w(data) > cqc6_w(ult)) {
        return None;
    }
    let chave = [cqc6_w(data), cqc6_w(data + 2).wrapping_sub(base)];
    let busca: unsafe extern "C" fn(*const i16, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let e = busca(chave.as_ptr(), tab, n as i32, 0x41, 0x685cf0);
    if e == 0 || cqc6_b(e + 0xb) == 0xff {
        return None;
    }
    Some(e)
}


/// { 0x685640 } (thiscall, ret 4, league.cpp, quadro SEH sem destrutor no caminho normal: as datas e a lista locais):
/// `League::move_fixtures_for_tv(data)`, do dia da liga (0x6847c0) e da montagem do calendário (0x683e30). Pela
/// entrada da data (tv_date_entry), os jogos da liga no dia e fatia da entrada (+4) vão a uma lista local (0x68d020;
/// 0x5abd90: push lista; push liga +0x44; push -1; push id; push fatia; push dia; push ano; mov ecx, [0xae2a58] ->
/// thiscall, ret 0x1c). Com jogos: as vagas da entrada (7 bytes a partir de +5, até a primeira com +0xb = 0xff) são
/// ordenadas (0x945333 com 0x8b4e00) se alguma tem bits 0x7fc0 no dword +5. Para cada vaga com fatia (+0xa) 0..2: a data
/// levada ao dia da semana da vaga (+9; 0x54a2d0); a janela de conflito: 2..5 dias depois da data pedida se o dia da TV
/// cai depois dela, senão 5..2 dias antes (0x54c5b0/0x54c8e0 + 0x417c10). Entre os jogos da lista (do tipo da liga,
/// +0x42), o de maior nota (palavra com sinal; o primeiro no empate): vaga do tipo 3, 100 para jogo de seleção (0x53b900
/// no mandante) no estádio (+0x18 → +0x38 → +0x1f) da nação de um dos dois, senão 50; senão o sorteio (0x90cfc0) da
/// reputação (+0x80) do visitante + 1500 mais o da do mandante, zerada se um dos dois tem jogo na janela (0x5acbe0). O
/// escolhido: fora do tipo 3, a notícia 3 aos dois clubes; sai do calendário (0x5abc30), ganha a data da TV, a fatia e
/// o tipo da vaga, o estádio (0x85c260) e volta ao calendário (0x5aa680); a vaga fica 0xff e o jogo sai da lista
/// (0x68d1f0).
unsafe extern "thiscall" fn move_fixtures_for_tv(this: usize, data: usize) {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    // as datas pelos dwords que a original lê da pilha (dia|ano, ano|bissexto)
    let (pa, pb, pc) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, c.as_mut_ptr() as usize);
    padrao(pa);
    padrao(pb);
    padrao(pc);
    let e = match tv_date_entry(this, data) {
        Some(e) => e,
        None => return,
    };
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x5abd90usize);
    // ponytail: a original empurra o ano, o dia, a fatia e o tipo em registradores com sobra nos bytes altos; a
    // 0x5abd90 só lê as palavras e os bytes
    let n = jogos(cqc6_d(0xae2a58), cqc6_w(data + 2) as u16 as u32, cqc6_w(data) as u16 as u32, cqc6_b(e + 4) as u32,
        cqc6_d(cqc6_d(this + 4)) as u32, u32::MAX, cqc6_b(this + 0x44) as u32, l) & 0xffff;
    if n != 0 {
        let mut k = 0usize;
        let mut ordenar = false;
        while k < 8 {
            if cqc6_b(e + k * 7 + 0xb) == 0xff {
                break;
            }
            if cqc6_d(e + k * 7 + 5) & 0x7fc0 != 0 {
                ordenar = true;
            }
            k += 1;
        }
        if ordenar {
            let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
            qsort(e + 5, k as u32, 7, 0x8b4e00);
        }
        let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
        let ajustar: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x54a2d0usize);
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
        let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
        let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
        let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
            core::mem::transmute(0x5acbe0usize);
        for k in 0..8usize {
            let v = e + k * 7;
            if cqc6_b(v + 0xb) == 0xff {
                break;
            }
            let fatia = cqc6_b(v + 0xa) as i8;
            if !(0..=2).contains(&fatia) {
                continue;
            }
            let mut x = [0u8; 8];
            let px = x.as_mut_ptr() as usize;
            nova(px, cqc6_w(data) as u16 as u32, cqc6_w(data + 2) as u16 as u32);
            ajustar(px, cqc6_b(v + 9) as i8 as i16 as u16 as u32, pa);
            let (xa, aa) = (cqc6_w(px + 2) as u16, cqc6_w(pa + 2) as u16);
            let (mut t1, mut t2, mut t3, mut t4) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
            if xa < aa || (xa == aa && cqc6_w(px) < cqc6_w(pa)) {
                let r = mais(px, t1.as_mut_ptr() as usize, 2);
                copia(pb, t2.as_mut_ptr() as usize, r);
                let r = mais(px, t3.as_mut_ptr() as usize, 5);
                copia(pc, t4.as_mut_ptr() as usize, r);
            } else {
                let r = menos(px, t1.as_mut_ptr() as usize, 5);
                copia(pb, t2.as_mut_ptr() as usize, r);
                let r = menos(px, t3.as_mut_ptr() as usize, 2);
                copia(pc, t4.as_mut_ptr() as usize, r);
            }
            cqc6_pd(l + 0xa, cqc6_d(l + 2));
            let mut melhor = 0usize;
            let mut nota: i16 = -1;
            let mut f = proximo(l);
            while f != 0 {
                if cqc6_b(this + 0x44) == cqc6_b(f + 0x42) {
                    let mut s: i32;
                    if cqc6_b(v + 0xb) == 3 {
                        let est = cqc6_d(f + 0x18);
                        s = 50;
                        if selecao(cqc6_d(f + 0x1c)) != 0 && est != 0 && cqc6_d(est + 0x38) != 0 {
                            let g = cqc6_d(cqc6_d(est + 0x38) + 0x1f);
                            if cqc6_d(cqc6_d(f + 0x1c) + 0x53) == g || cqc6_d(cqc6_d(f + 0x20) + 0x53) == g {
                                s = 100;
                            }
                        }
                    } else {
                        s = sorteio(cqc6_w(cqc6_d(f + 0x20) + 0x80) as i32 + 0x5dc); // cqc6: o visitante com o bônus
                        s = s.wrapping_add(sorteio(cqc6_w(cqc6_d(f + 0x1c) + 0x80) as i32));
                    }
                    if cqc6_b(v + 0xb) != 3 {
                        let janela = |clube: usize| {
                            achar(cqc6_d(0xae2a58), cqc6_d(pb) as u32, cqc6_d(pb + 2) as u32, cqc6_d(pc) as u32, cqc6_d(pc + 2) as u32,
                                cqc6_d(clube) as u32, 0, u32::MAX, 0xffff_fffe, u32::MAX) as u16 != 0
                        };
                        if janela(cqc6_d(f + 0x1c)) || janela(cqc6_d(f + 0x20)) {
                            crate::comparacao::ramo("cqc_6 conflito na janela");
                            s = 0; // cqc6: conflito na janela
                        }
                    }
                    if s as i16 > nota {
                        melhor = f;
                        nota = s as i16;
                    }
                }
                f = proximo(l);
            }
            if melhor == 0 {
                continue;
            }
            let mut buf = [0u8; 0x50];
            let r = buf.as_mut_ptr() as usize;
            core::ptr::copy_nonoverlapping(melhor as *const u8, r as *mut u8, 0x4f);
            if cqc6_b(v + 0xb) != 3 {
                let mut noticia = [0u32; 0x38];
                let n = noticia.as_mut_ptr() as usize;
                let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
                let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
                let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
                construir(n, 3, 0);
                gravar(n, 0, cqc6_d(r + 0xc) as u32);
                gravar(n, 1, cqc6_d(r + 0x10) as u32);
                gravar(n, 2, cqc6_d(r) as u32);
                gravar(n, 3, cqc6_w(r + 0x32) as i32 as u32);
                gravar(n, 4, cqc6_w(r + 0x30) as i32 as u32);
                gravar(n, 5, cqc6_w(r + 0x2a) as u16 as u32);
                gravar(n, 6, cqc6_w(r + 0x28) as u16 as u32);
                gravar(n, 7, cqc6_b(r + 0x3f) as u32);
                gravar(n, 8, cqc6_w(pa) as i32 as u32);
                gravar(n, 9, cqc6_w(pa + 2) as u16 as u32);
                gravar(n, 10, cqc6_b(v + 0xa) as i8 as i32 as u32);
                gravar(n, 11, cqc6_b(v + 0xb) as i8 as i32 as u32);
                entregar(cqc6_d(0xb7431c), n, cqc6_d(r + 0x1c));
                entregar(cqc6_d(0xb7431c), n, cqc6_d(r + 0x20));
            }
            let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5abc30usize);
            tirar(cqc6_d(0xae2a58), r);
            crate::comparacao::ramo("cqc_6 jogo para a TV");
            cqc6_pw(r + 0x28, cqc6_w(pa + 2) as u16);
            cqc6_pw(r + 0x2a, cqc6_w(pa) as u16); // cqc6: o dia da TV
            cqc6_pb(r + 0x3f, cqc6_b(v + 0xa));
            cqc6_pb(r + 0x3b, cqc6_b(v + 0xb));
            let estadio: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x85c260usize);
            estadio(cqc6_d(0xdd7ef4), cqc6_d(v + 5), r, cqc6_d(this + 0x24));
            let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5aa680usize);
            juntar(cqc6_d(0xae2a58), r, 1);
            cqc6_pb(v + 0xb, 0xff);
            let sair: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
            sair(l, melhor);
        }
    }
    destruir(l);
}

/// Comparador da 0x685640: só a original (os jogos saem do calendário e voltam pela 0x5aa680, que aloca, as notas
/// sorteiam e as notícias vão aos técnicos). O plano, feito antes: pela entrada da data, os jogos da liga no dia e fatia
/// dela (0x5abd90 numa lista provisória) e as vagas válidas (fatia 0..2) antes da primeira vazia; cada vaga válida leva
/// um jogo enquanto a lista tiver (toda nota passa do −1 inicial), e a vaga fica 0xff: as vagas 0xff do bloco depois da
/// original são min(válidas, jogos).
unsafe extern "thiscall" fn comparar_move_fixtures_for_tv(this: usize, data: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let e = match tv_date_entry(this, data) {
        Some(e) => e,
        None => return o(this, data),
    };
    let mut bloco = 0usize;
    let mut validas = 0usize;
    while bloco < 8 && cqc6_b(e + bloco * 7 + 0xb) != 0xff {
        if (0..=2).contains(&(cqc6_b(e + bloco * 7 + 0xa) as i8)) {
            validas += 1;
        }
        bloco += 1;
    }
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x5abd90usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let n = jogos(cqc6_d(0xae2a58), cqc6_w(data + 2) as u16 as u32, cqc6_w(data) as u16 as u32, cqc6_b(e + 4) as u32,
        cqc6_d(cqc6_d(this + 4)) as u32, u32::MAX, cqc6_b(this + 0x44) as u32, l) as usize & 0xffff;
    destruir(l);
    o(this, data);
    let esperado = if n == 0 { 0 } else { validas.min(n) };
    let movidas = (0..bloco).filter(|k| cqc6_b(e + k * 7 + 0xb) == 0xff).count();
    PLACAR.registrar(movidas == esperado, || format!("move_fixtures_for_tv(data {:x}) vagas {} válidas {} jogos {}: movidas {} (esperado {})",
        data, bloco, validas, n, movidas, esperado));
}

/// { 0x684230 } (thiscall, ret 0, league.cpp, quadro SEH sem destrutor no caminho normal): `League::send_group_draw_news()`,
/// do sorteio dos grupos das competições. Grupo de até 8 times (+0x3e; os times de 0x3b bytes em [+0xb1], +0 o clube):
/// para cada clube de técnico humano, a notícia 0x15 (alocada uma vez, 0x944cf1(0xdf), montada por 0x77bbe0, liberada
/// no fim por 0x944c94 sem destrutor) com o id da competição, +0x3a, a primeira data (a da primeira entrada da tabela
/// [+0xba] levada ao dia da semana de cada vaga, 0x54a2d0, a menor), o número de adversários; por clube, o id dele (2)
/// e os dos outros (6 em diante), entregue a ele (0x7861e0).
unsafe extern "thiscall" fn send_group_draw_news(this: usize) {
    let n = cqc6_w(this + 0x3e);
    if n > 8 || n <= 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let time = |i: u8| cqc6_d(cqc6_d(this + 0xb1) + (i as i8 as isize * 0x3b) as usize);
    let mut noticia = 0usize;
    let mut i: u8 = 0;
    loop {
        if cqc6_humano(time(i)) {
            if noticia == 0 {
                let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
                let p = alocar(0xdf);
                noticia = if p != 0 { construir(p, 0x15, 0) } else { 0 };
                let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
                let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
                let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
                let ajustar: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x54a2d0usize);
                let (mut d1, mut d2, mut d3, mut t) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
                let (p1, p2, p3) = (d1.as_mut_ptr() as usize, d2.as_mut_ptr() as usize, d3.as_mut_ptr() as usize);
                let tab = cqc6_d(this + 0xba);
                // ponytail: o dia e o ano vão em registradores com sobra nos 16 bits altos; a 0x549ef0 só lê as palavras
                nova(p1, cqc6_w(tab) as u16 as u32, cqc6_w(tab + 2).wrapping_add(cqc6_w(this + 0x40)) as u16 as u32);
                padrao(p2);
                padrao(p3);
                copia(p2, t.as_mut_ptr() as usize, p1);
                for j in 0..8usize {
                    let v = cqc6_d(this + 0xba) + j * 7;
                    if cqc6_b(v + 0xb) == 0xff {
                        break;
                    }
                    ajustar(p1, cqc6_b(v + 9) as i8 as i16 as u16 as u32, p3);
                    let (y2, y3) = (cqc6_w(p2 + 2) as u16, cqc6_w(p3 + 2) as u16);
                    if y2 > y3 || (y2 == y3 && cqc6_w(p2) > cqc6_w(p3)) {
                        let mut t2 = [0u8; 8];
                        copia(p2, t2.as_mut_ptr() as usize, p3);
                    }
                }
                gravar(noticia, 0, cqc6_d(cqc6_d(this + 4)) as u32);
                gravar(noticia, 1, cqc6_w(this + 0x3a) as i32 as u32);
                gravar(noticia, 3, cqc6_w(p2) as i32 as u32);
                gravar(noticia, 4, cqc6_w(p2 + 2) as u16 as u32);
                gravar(noticia, 5, (cqc6_w(this + 0x3e) as i32 - 1) as u32);
            }
            gravar(noticia, 2, cqc6_d(time(i)) as u32);
            let mut k: u8 = 6;
            let mut j: u8 = 0;
            while (j as i8 as i16) < cqc6_w(this + 0x3e) {
                if j != i {
                    gravar(noticia, k as i8, cqc6_d(time(j)) as u32);
                    k = k.wrapping_add(1);
                }
                j = j.wrapping_add(1);
            }
            crate::comparacao::ramo("cqc_6 noticia do grupo");
            entregar(cqc6_d(0xb7431c), noticia, time(i));
        }
        i = i.wrapping_add(1);
        if (i as i8 as i16) >= cqc6_w(this + 0x3e) {
            break;
        }
    }
    if noticia != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        liberar(noticia);
    }
}

/// Comparador da 0x684230: só a original (aloca a notícia e a entrega aos técnicos).
unsafe extern "thiscall" fn comparar_send_group_draw_news(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this) // ponytail: só a notícia
}

/// { 0x683ee0 } (thiscall, ret 4, league.cpp, quadro SEH sem destrutor no caminho normal: as listas locais):
/// `League::send_fixture_list_news(lista)`, da criação do calendário (0x5cd1e0) e do dia da liga no ano dela. Usa a
/// lista local (0x68d020) se a liga é de primeira fase (+0x44 = 0xff) ou sem lista. Com a competição contando
/// (0x4b0700: push id; add esp, 4 -> cdecl) e de primeira fase: a data da primeira entrada da tabela [+0xba] menos 5 dias
/// (0x54c8e0, 0x417c10); para cada clube de técnico humano (times de 0x3b bytes em [+0xb1]) que não esteja na lista (ids
/// de 4 bytes): os primeiros 3 jogos dele a partir da data (0x5acee0 numa lista local, 18 argumentos: push 1; push -1;
/// push +0x44; push 0; push lista; push 3; push -1 ×4; push id do clube; push -1; push -1; push id da competição; push 1;
/// push &fatia; push &dia; push &ano; mov ecx, [0xae2a58] -> thiscall, ret 0x48); achando 3: a notícia 0xd (id da
/// competição, do clube, o ano da liga e, por jogo, o dia, o ano, o adversário e 1 em casa ou 0 fora) ao clube, que
/// entra na lista (0x68d0d0: push 4; push clube; mov ecx, lista -> thiscall, ret 8). Depois, nas fases ([+0xc], até
/// +0x30) com +0x43 = 2, a mesma rotina com a lista.
unsafe extern "thiscall" fn send_fixture_list_news(this: usize, lista: usize) {
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut l0 = [0u8; 0x10];
    let p0 = l0.as_mut_ptr() as usize;
    criar(p0);
    let lista = if cqc6_b(this + 0x44) == 0xff || lista == 0 { p0 } else { lista };
    let conta: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x4b0700usize);
    if conta(cqc6_d(cqc6_d(this + 4)) as u32) == 0 || cqc6_b(this + 0x44) != 0xff {
        destruir(p0);
        return;
    }
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let menos: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let (mut d1, mut t, mut t2) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let p1 = d1.as_mut_ptr() as usize;
    let tab = cqc6_d(this + 0xba);
    // ponytail: o dia e o ano vão em registradores com sobra nos 16 bits altos; a 0x549ef0 só lê as palavras
    nova(p1, cqc6_w(tab) as u16 as u32, cqc6_w(tab + 2).wrapping_add(cqc6_w(this + 0x40)) as u16 as u32);
    let r = menos(p1, t.as_mut_ptr() as usize, 5);
    copia(p1, t2.as_mut_ptr() as usize, r);
    let time = |i: u32| cqc6_d(cqc6_d(this + 0xb1) + (i as i16 as isize * 0x3b) as usize);
    let mut i: u32 = 0;
    if cqc6_w(this + 0x3e) > 0 {
        loop {
            if cqc6_humano(time(i)) {
                cqc6_pd(lista + 0xa, cqc6_d(lista + 2));
                let mut ja = false;
                let mut it = proximo(lista);
                while it != 0 {
                    if cqc6_d(it) == cqc6_d(time(i)) {
                        ja = true;
                        break;
                    }
                    it = proximo(lista);
                }
                if !ja {
                    let mut v_ano = cqc6_d(p1 + 2) as u32;
                    let mut v_dia = cqc6_d(p1) as u32;
                    let mut fatia: u8 = 0;
                    let mut l2 = [0u8; 0x10];
                    let p2 = l2.as_mut_ptr() as usize;
                    criar(p2);
                    let buscar: unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, u32, u32, i32, i32, u32, i32, i32, i32,
                                                            i32, u32, usize, u32, u32, i32, u32) -> u32 =
                        core::mem::transmute(0x5acee0usize);
                    let achou = buscar(cqc6_d(0xae2a58), &mut v_ano, &mut v_dia, &mut fatia, 1, cqc6_d(cqc6_d(this + 4)) as u32, -1, -1,
                        cqc6_d(time(i)) as u32, -1, -1, -1, -1, 3, p2, 0, cqc6_b(this + 0x44) as u32, -1, 1) as u16;
                    if achou == 3 {
                        crate::comparacao::ramo("cqc_6 noticia da tabela");
                        let mut noticia = [0u32; 0x38];
                        let n = noticia.as_mut_ptr() as usize;
                        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
                        let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
                        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
                        let guardar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
                        construir(n, 0xd, 0);
                        let mut k: u8 = 3;
                        gravar(n, 0, cqc6_d(cqc6_d(this + 4)) as u32);
                        gravar(n, 1, cqc6_d(time(i)) as u32);
                        gravar(n, 2, cqc6_w(this + 0x40) as i32 as u32);
                        cqc6_pd(p2 + 0xa, cqc6_d(p2 + 2));
                        let mut f = proximo(p2);
                        while f != 0 {
                            gravar(n, k as i8, cqc6_w(f + 0x2a) as u16 as u32);
                            k = k.wrapping_add(1);
                            gravar(n, k as i8, cqc6_w(f + 0x28) as u16 as u32);
                            k = k.wrapping_add(1);
                            if time(i) == cqc6_d(f + 0x1c) {
                                gravar(n, k as i8, cqc6_d(cqc6_d(f + 0x20)) as u32);
                                k = k.wrapping_add(1);
                                gravar(n, k as i8, 1);
                            } else {
                                gravar(n, k as i8, cqc6_d(cqc6_d(f + 0x1c)) as u32);
                                k = k.wrapping_add(1);
                                gravar(n, k as i8, 0);
                            }
                            k = k.wrapping_add(1);
                            f = proximo(p2);
                        }
                        entregar(cqc6_d(0xb7431c), n, time(i));
                        guardar(lista, time(i), 4);
                    }
                    destruir(p2);
                }
            }
            i = i.wrapping_add(1);
            if (i as i16) >= cqc6_w(this + 0x3e) {
                break;
            }
        }
    }
    if cqc6_b(this + 0x44) == 0xff && (cqc6_d(this + 0x30) as i32) >= 0 {
        let mut j: u8 = 0;
        loop {
            let fase = cqc6_d(cqc6_d(this + 0xc) + (j as i8 as isize * 4) as usize);
            if cqc6_b(fase + 0x43) == 2 {
                let mesma: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x683ee0usize);
                mesma(fase, lista);
            }
            j = j.wrapping_add(1);
            if j as i8 as i32 > cqc6_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    destruir(p0);
}

/// Comparador da 0x683ee0: só a original (as notícias vão aos técnicos e a lista dos avisados cresce, alocando).
unsafe extern "thiscall" fn comparar_send_fixture_list_news(this: usize, lista: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, lista) // ponytail: só as notícias
}
