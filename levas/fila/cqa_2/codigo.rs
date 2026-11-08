// ---------- %%LEVA%%a leva: os amistosos dos clubes: a resposta ao pedido de amistoso, o pedido enviado e a lista das nações para os amistosos internacionais (friendly.cpp) ----------

unsafe fn cqa2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqa2_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqa2_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v)
}

/// O outro clube do pedido de amistoso (0x19 bytes: +0 dia, +2 ano, +0xc quem pede, +0x10 e +0x14 os dois clubes do
/// jogo, +0x18 a situação): o +0x10, ou o +0x14 quando o +0x10 é o que pede.
unsafe fn other_friendly_club(m: usize) -> usize {
    if cqa2_d(m + 0xc) == cqa2_d(m + 0x10) {
        cqa2_d(m + 0x14)
    } else {
        cqa2_d(m + 0x10)
    }
}

/// O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c]; sem add esp -> thiscall(notícias,
/// clube), ret 4; a cópia Rust é stdcall(clube): o this não é lido).
unsafe fn club_has_human_manager(clube: usize) -> bool {
    let f: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(clube) != 0
}

/// A notícia do amistoso (`tipo` 6 ou 7) numa local de 0xe0 bytes: 0x77bbe0 (push 0; push tipo; ecx = a local; sem add
/// esp -> thiscall, ret 8) com os dados 0x785790 (push valor; push k; ecx = a local -> thiscall, ret 8) 0 e 1 = os ids
/// dos clubes do jogo (+0x10, +0x14), 2 = o id de quem pede (+0xc), 3 = o dia (com sinal), 4 = o ano (sem sinal), 5 =
/// a situação; entregue ao técnico do `para` (0x7861e0: push para; push &local; ecx = [0xb7431c]; sem add esp ->
/// thiscall, ret 8).
unsafe fn send_friendly_news(m: usize, tipo: u32, situacao: u32, para: usize) {
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    construir(n, tipo, 0);
    gravar(n, 0, cqa2_d(cqa2_d(m + 0x10)) as u32);
    gravar(n, 1, cqa2_d(cqa2_d(m + 0x14)) as u32);
    gravar(n, 2, cqa2_d(cqa2_d(m + 0xc)) as u32);
    gravar(n, 3, cqa2_w(m) as i32 as u32);
    gravar(n, 4, core::ptr::read_unaligned((m + 2) as *const u16) as u32);
    gravar(n, 5, situacao);
    entregar(cqa2_d(0xb7431c), n, para);
}

/// A 0x5c5f40 decide: o outro clube existe, a situação é 1 e ele não é de técnico humano (0x600a50: push 0; push 0;
/// push 1; push 0; push o; add esp, 0x14 -> cdecl(o, 0, 1, 0, 0)), salvo com `forcar`.
unsafe fn friendly_needs_decision(m: usize, forcar: u32) -> bool {
    let o = other_friendly_club(m);
    if o == 0 || cqa2_b(m + 0x18) != 1 {
        return false;
    }
    let human_club: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    human_club(o, 0, 1, 0, 0) == 0 || forcar != 0
}

/// A decisão do convidado (depois da friendly_needs_decision): 3 aceita, 4 recusa. O fator: 1,65 (1,32 quando o
/// outro é o +0x10), × 0,925 com nações diferentes (+0x53), × 0,85 com regiões (+0x76) diferentes, × 0,85 com
/// continentes (+0x71) diferentes; quando o outro é o +0x10 e os dois têm cidade do estádio (+0x69 → +0x38), × (1 +
/// +0x77 de quem pede / ((distância das coordenadas +0x23/+0x2b × 0,55 + 1) × 40000)). Aceita se a reputação (+0x80)
/// de quem pede é ao menos a do outro / fator, ou se a diferença das reputações não passa de fator × 600.
unsafe fn friendly_reply(m: usize, contar: bool) -> u8 {
    let a = cqa2_d(m + 0xc);
    let o = other_friendly_club(m);
    let (na, no) = (cqa2_d(a + 0x53), cqa2_d(o + 0x53));
    let segundo = o == cqa2_d(m + 0x10);
    let mut f = if segundo { read_f64_at(0x96af38) } else { read_f64_at(0x96af20) };
    if no != na {
        f *= read_f64_at(0x96af30);
    }
    if cqa2_b(no + 0x76) != cqa2_b(na + 0x76) {
        f *= read_f64_at(0x969520);
    }
    if cqa2_i(no + 0x71) != cqa2_i(na + 0x71) {
        f *= read_f64_at(0x969520);
    }
    if segundo {
        let (fa, fo) = (cqa2_d(a + 0x69), cqa2_d(o + 0x69));
        let sa = if fa != 0 { cqa2_d(fa + 0x38) } else { 0 };
        let so = if sa != 0 && fo != 0 { cqa2_d(fo + 0x38) } else { 0 };
        if sa != 0 && so != 0 {
            if contar {
                crate::comparison::branch("cqa_2 distancia do amistoso");
            }
            let dx = read_f64_at(sa + 0x2b) - read_f64_at(so + 0x2b);
            let dy = read_f64_at(sa + 0x23) - read_f64_at(so + 0x23);
            let i = cqa2_i(a + 0x77) as f64;
            let v = i / (((dy * dy + dx * dx).sqrt() * read_f64_at(0x96a5d8) + read_f64_at(0x967890)) * read_f64_at(0x96af28)) + read_f64_at(0x967890);
            f *= v; // cqa2: o fator da distância
        }
    }
    let (ro, ra) = (cqa2_w(o + 0x80) as i32, cqa2_w(a + 0x80) as i32);
    if contar {
        crate::comparison::branch("cqa_2 decisao do amistoso");
    }
    // fcompp/test ah, 1: o C0 vale para "menor" e para "sem ordem" (NaN): os >= abaixo dão o mesmo caminho
    let aceita = ra as f64 >= ro as f64 / f || f * read_f64_at(0x96a690) >= ro.wrapping_sub(ra) as f64; // cqa2: aceita pela reputação
    if aceita { 3 } else { 4 }
}

/// { 0x5c5f40 } (thiscall, ret 8, friendly.cpp, quadro SEH sem destrutor no caminho normal):
/// `Friendly::process_friendly_reply(pedido, forcar)`. Sem quem pede (+0xc): nada. Com o outro clube (+0x10, ou +0x14
/// se o +0x10 é quem pede) na situação 1 e sem técnico humano (0x600a50) ou com `forcar`: sem nação num dos dois, a
/// situação 4 (sobrescrita logo depois); a decisão (friendly_reply) grava 3 ou 4 em +0x18 e a notícia 7 vai a quem pede.
/// Aceito (3): a partida do amistoso numa local de 0x4f bytes (a competição [+4] e o id dela, o estádio do clube +0x10 (+0x69) e o
/// id dele, os clubes +0x10 e +0x14 e os ids, a data do pedido, o ano de hoje, 9 para seleção (0x53b900) ou 1, o horário 1
/// no sábado e no domingo (0x54a3f0 = 5 ou 6) ou 2, as marcas, −1 nos placares) vai ao calendário (0x5aa680: push 1;
/// push &local; ecx = [0xae2a58] -> thiscall, ret 8). Com 3 ou 4: a vaga do amistoso marcado (0x5c6740) fica com +0x18
/// = 3 ou com +0x10 = 0 e +0x18 = 0, e o pedido sai (0x5c87b0 com 0). Noutra situação, o pedido sai da lista das
/// respostas (+0x34d4; 0x68d1f0, o cursor +0xa volta ao começo antes, depois de cada um e no fim).
unsafe extern "thiscall" fn process_friendly_reply(this: usize, m: usize, forcar: u32) {
    let a = cqa2_d(m + 0xc);
    if a == 0 {
        return;
    }
    let o = other_friendly_club(m);
    if friendly_needs_decision(m, forcar) {
        if cqa2_d(a + 0x53) == 0 || cqa2_d(o + 0x53) == 0 {
            *((m + 0x18) as *mut u8) = 4; // como a original (que cai logo depois, lendo a nação nula)
        }
        let s = friendly_reply(m, true);
        *((m + 0x18) as *mut u8) = s;
        send_friendly_news(m, 7, s as u32, a);
    }
    let s = cqa2_b(m + 0x18);
    if s == 3 {
        crate::comparison::branch("cqa_2 amistoso aceito");
        let mut p = [0u8; 0x50];
        let pp = p.as_mut_ptr() as usize;
        let comp = cqa2_d(this + 4);
        let (c2, c3) = (cqa2_d(m + 0x10), cqa2_d(m + 0x14));
        let estadio = cqa2_d(c2 + 0x69);
        cqa2_pd(pp, cqa2_d(comp));
        cqa2_pd(pp + 0x14, comp);
        cqa2_pd(pp + 0x18, estadio);
        cqa2_pd(pp + 4, if estadio != 0 { cqa2_d(estadio) } else { usize::MAX });
        let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
        let sel = is_national_team(c2); // push c2; add esp, 4 -> cdecl
        p[0x2c..0x2e].copy_from_slice(&core::ptr::read_unaligned(0xae2c92 as *const u16).to_le_bytes());
        p[0x3a] = if sel != 0 { 9 } else { 1 };
        p[0x40] = 1;
        p[0x41] = 1;
        for k in [0x43usize, 0x44, 0x4b, 0x4c, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x3b, 0x42] {
            p[k] = 0xff;
        }
        cqa2_pd(pp + 8, usize::MAX);
        cqa2_pd(pp + 0x1c, c2); // cqa2: o mandante do amistoso
        cqa2_pd(pp + 0xc, cqa2_d(c2));
        cqa2_pd(pp + 0x20, c3);
        cqa2_pd(pp + 0x10, cqa2_d(c3));
        p[0x2a..0x2c].copy_from_slice(&core::ptr::read_unaligned(m as *const u16).to_le_bytes());
        p[0x28..0x2a].copy_from_slice(&core::ptr::read_unaligned((m + 2) as *const u16).to_le_bytes());
        let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
        p[0x3f] = if semana(m) as i16 == 5 || semana(m) as i16 == 6 { 1 } else { 2 };
        // ponytail: a original faz and 0xf8ff e depois and 0xff99 / or 0x99 na palavra +0x4d de um local sem valor (os
        // bits 11..15 ficam com a sobra de pilha) e não grava +0x24..+0x27, +0x2e, +0x2f, +0x3c: aqui, zero.
        p[0x4d] = 0x99;
        p[0x3d] = 0;
        p[0x3e] = 0;
        let juntar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5aa680usize);
        juntar(cqa2_d(0xae2a58), pp, 1);
    }
    let s = cqa2_b(m + 0x18);
    if s == 3 || s == 4 {
        let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(0x5c6740usize);
        // push +0x10; push +0x14; push ano; push dia; ecx = this; sem add esp -> thiscall, ret 0x10
        // ponytail: o dia e o ano vão em registradores com lixo nos 16 bits altos; a 0x5c6740 só lê as palavras
        let v = vaga(this, cqa2_w(m) as u16 as u32, cqa2_w(m + 2) as u16 as u32, cqa2_d(m + 0x14), cqa2_d(m + 0x10));
        if v != 0 {
            if s == 3 {
                *((v + 0x18) as *mut u8) = 3;
            } else {
                cqa2_pd(v + 0x10, 0);
                *((v + 0x18) as *mut u8) = 0;
            }
        }
        let tirar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x5c87b0usize);
        tirar(this, m, 0); // push 0; push m; ecx = this -> thiscall, ret 8
        return;
    }
    let l = cqa2_d(this + 0x34d4);
    if l == 0 {
        return;
    }
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    cqa2_pd(l + 0xa, cqa2_d(l + 2));
    let mut item = proximo(cqa2_d(this + 0x34d4));
    while item != 0 {
        if cqa2_d(item) == m {
            tirar(cqa2_d(this + 0x34d4), item); // push item; ecx = a lista -> thiscall, ret 4
            let l = cqa2_d(this + 0x34d4);
            cqa2_pd(l + 0xa, cqa2_d(l + 2));
        }
        item = proximo(cqa2_d(this + 0x34d4));
    }
    let l = cqa2_d(this + 0x34d4);
    cqa2_pd(l + 0xa, cqa2_d(l + 2));
}

/// Algum nó da lista (+2 a cabeça, +6 o próximo de cada nó, +0 o dado) guarda `m`, sem mexer no cursor.
unsafe fn list_holds(l: usize, m: usize) -> bool {
    if l == 0 {
        return false;
    }
    let mut no = cqa2_d(l + 2);
    let mut guarda = 0;
    while no != 0 && guarda < 100_000 {
        if cqa2_d(no) != 0 && cqa2_d(cqa2_d(no)) == m {
            return true;
        }
        no = cqa2_d(no + 6);
        guarda += 1;
    }
    false
}

/// Comparador da 0x5c5f40: só a original (a partida vai ao calendário e a notícia ao técnico, que alocam, e o pedido é
/// liberado pela 0x5c87b0). O plano, feito antes: a decisão (friendly_reply) e a vaga do amistoso (0x5c6740). Com 3 ou 4,
/// a vaga fica com +0x18 = 3, ou com +0x10 = 0 e +0x18 = 0; sem decisão, o pedido continua com a mesma situação e sai
/// da lista das respostas (+0x34d4). Sem quem pede ou sem nação num dos dois: só a original.
unsafe extern "thiscall" fn comparar_process_friendly_reply(this: usize, m: usize, forcar: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let a = if m != 0 { cqa2_d(m + 0xc) } else { 0 };
    let outro = if m != 0 { other_friendly_club(m) } else { 0 };
    if a == 0 || (outro != 0 && (cqa2_d(a + 0x53) == 0 || cqa2_d(outro + 0x53) == 0)) {
        return o(this, m, forcar); // ponytail: sem quem pede ou sem nação, só a original
    }
    let s = if friendly_needs_decision(m, forcar) { friendly_reply(m, false) } else { cqa2_b(m + 0x18) };
    let vaga: unsafe extern "thiscall" fn(usize, u32, u32, usize, usize) -> usize = core::mem::transmute(0x5c6740usize);
    let v = if s == 3 || s == 4 { vaga(this, cqa2_w(m) as u16 as u32, cqa2_w(m + 2) as u16 as u32, cqa2_d(m + 0x14), cqa2_d(m + 0x10)) } else { 0 };
    o(this, m, forcar);
    let ok = if s == 3 || s == 4 {
        // o pedido foi liberado: só a vaga
        v == 0 || if s == 3 { cqa2_b(v + 0x18) == 3 } else { cqa2_d(v + 0x10) == 0 && cqa2_b(v + 0x18) == 0 }
    } else {
        cqa2_b(m + 0x18) == s && !list_holds(cqa2_d(this + 0x34d4), m)
    };
    SCOREBOARD.record(ok, || format!("process_friendly_reply(pedido {:x}) situação prevista {} vaga {:x}", m, s, v));
}

/// { 0x5c5570 } (thiscall, ret 4, friendly.cpp, quadro SEH sem destrutor no caminho normal):
/// `Friendly::send_friendly_request(pedido)`: sem a lista das respostas (+0x34d4), cria-a (0x944cf1(0xe): push 0xe;
/// add esp, 4 -> cdecl; 0x68d020: ecx = o bloco -> thiscall, ret 0); sem memória, a caixa (linha 0x982) e nada mais.
/// Guarda o ponteiro do pedido na lista (0x68d0d0: push 4; push &pedido; ecx = a lista -> thiscall, ret 8). Com o outro
/// clube na situação 1 e de técnico humano: situação 2 e, se o +0x10 ou o +0x14 é de técnico humano, a notícia 6 ao
/// outro.
unsafe extern "thiscall" fn send_friendly_request(this: usize, m: usize) {
    let o = other_friendly_club(m);
    if cqa2_d(this + 0x34d4) == 0 {
        let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
        let bloco = novo(0xe);
        let l = if bloco != 0 {
            let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
            construir(bloco)
        } else {
            0
        };
        cqa2_pd(this + 0x34d4, l);
        if l == 0 {
            return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, m); // ponytail: sem memória, a caixa da original
        }
    }
    let guardar: unsafe extern "thiscall" fn(usize, *const usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let copia = m;
    guardar(cqa2_d(this + 0x34d4), &copia, 4);
    if o == 0 || cqa2_b(m + 0x18) != 1 || !club_has_human_manager(o) {
        return;
    }
    crate::comparison::branch("cqa_2 pedido ao humano");
    *((m + 0x18) as *mut u8) = 2;
    if !club_has_human_manager(cqa2_d(m + 0x10)) && !club_has_human_manager(cqa2_d(m + 0x14)) {
        return;
    }
    send_friendly_news(m, 6, 2, o);
}

/// Quantos nós a lista tem, sem mexer no cursor.
unsafe fn list_len(l: usize) -> usize {
    let mut n = 0;
    let mut no = if l != 0 { cqa2_d(l + 2) } else { 0 };
    while no != 0 && n < 100_000 {
        n += 1;
        no = cqa2_d(no + 6);
    }
    n
}

/// Comparador da 0x5c5570: só a original (o nó da lista e a notícia alocam). O plano, feito antes: a lista das
/// respostas fica com um nó a mais e guarda o pedido; a situação vai a 2 quando o outro clube, na situação 1, é de
/// técnico humano; senão não muda.
unsafe extern "thiscall" fn comparar_send_friendly_request(this: usize, m: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let outro = other_friendly_club(m);
    let antes = list_len(cqa2_d(this + 0x34d4));
    let s = if outro != 0 && cqa2_b(m + 0x18) == 1 && club_has_human_manager(outro) { 2 } else { cqa2_b(m + 0x18) };
    o(this, m);
    let l = cqa2_d(this + 0x34d4);
    let ok = l == 0 || (list_len(l) == antes + 1 && list_holds(l, m) && cqa2_b(m + 0x18) == s);
    SCOREBOARD.record(ok, || format!("send_friendly_request(pedido {:x}) nós {} -> {} situação {} prevista {}", m, antes, list_len(l), cqa2_b(m + 0x18), s));
}

/// As nações com continente (+0x71) da tabela [0xae23a8] (0x122 bytes cada, [0xae2350] delas), na ordem da tabela.
unsafe fn nations_with_continent() -> Vec<usize> {
    let mut v = Vec::new();
    let t = cqa2_d(0xae23a8);
    let mut i = 0;
    while i < cqa2_i(0xae2350) {
        let n = t.wrapping_add(i as usize * 0x122);
        if cqa2_d(n + 0x71) != 0 {
            v.push(n);
        }
        i += 1;
    }
    v
}

/// O qsort da CRT (0x945333: 4 pushes; add esp, 0x10 -> cdecl(base, n, 4, ordem)) com a ordem 0x5cc5f0 (reputação,
/// a maior primeiro).
unsafe fn sort_by_reputation(base: usize, n: u32) {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(base, n, 4, 0x5cc5f0);
}

/// { 0x5ca3e0 } (thiscall, ret 0, friendly.cpp): `Friendly::build_nation_list()`: +0x34fe = 0; aloca [0xae2350] × 4
/// bytes (0x944e46: push n; add esp, 4 -> cdecl) em +0x34fa (sem memória, a caixa, linha 0x15af); põe nele as nações
/// com continente (+0x71), contando em +0x34fe, e ordena pela reputação (0x945333 com 0x5cc5f0).
unsafe extern "thiscall" fn build_nation_list(this: usize) {
    core::ptr::write_unaligned((this + 0x34fe) as *mut i32, 0);
    let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let p = malloc((cqa2_i(0xae2350) as u32).wrapping_mul(4));
    cqa2_pd(this + 0x34fa, p);
    if p == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this); // ponytail: sem memória, a caixa da original
    }
    for n in nations_with_continent() {
        let k = cqa2_i(this + 0x34fe);
        cqa2_pd(cqa2_d(this + 0x34fa).wrapping_add((k as isize * 4) as usize), n);
        core::ptr::write_unaligned((this + 0x34fe) as *mut i32, k + 1);
    }
    sort_by_reputation(cqa2_d(this + 0x34fa), cqa2_i(this + 0x34fe) as u32);
}

/// Comparador da 0x5ca3e0: só a original (aloca a lista). O plano, feito antes numa lista local com o mesmo qsort e a
/// mesma ordem: a conta (+0x34fe) e as nações, na ordem.
unsafe extern "thiscall" fn comparar_build_nation_list(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let mut plano = nations_with_continent();
    sort_by_reputation(plano.as_mut_ptr() as usize, plano.len() as u32);
    o(this);
    let p = cqa2_d(this + 0x34fa);
    let n = cqa2_i(this + 0x34fe);
    let ok = p == 0 || (n as usize == plano.len() && (0..plano.len()).all(|k| cqa2_d(p + 4 * k) == plano[k]));
    SCOREBOARD.record(ok, || format!("build_nation_list: conta {} prevista {}", n, plano.len()));
}
