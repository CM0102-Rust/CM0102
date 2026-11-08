// ---------- %%LEVA%%a leva: as copas entre continentes: os dois clubes da Copa Intercontinental e da Copa Interamericana; as posições previstas da Merconorte (world_club_cup.cpp, inter_amer_cup.cpp, con_merc_cup.cpp) ----------

unsafe fn cqu13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqu13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqu13_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O continente do clube ([[[clube+0x53]+0x71]], dword) é o da chave `continente` (global)?
unsafe fn cqu13_do_continente(c: usize, continente: usize) -> bool {
    let n = cqu13_d(c + 0x53);
    if n == 0 {
        return false;
    }
    let k = cqu13_d(n + 0x71);
    k != 0 && cqu13_d(k) == cqu13_d(continente)
}

/// A competição da chave `chave` (global): [0xae23d0] + 0x6b × [chave].
unsafe fn cqu13_comp(chave: usize) -> usize {
    cqu13_d(0xae23d0).wrapping_add(cqu13_d(chave).wrapping_mul(0x6b))
}

/// Uma vaga de copa entre continentes. Se o continente (global `continente`; o registro [0xae23a4] + 0xc6 × chave) tem
/// liga carregada (0x6699d0(continente): push; add esp, 4 -> cdecl) ou o ano da copa (+0x40) é 2001: o colocado `lugar`
/// mais recente da competição `comp` (0x4afd80(comp, lugar): push; push; add esp, 8 -> cdecl; o campeão é o lugar 0),
/// e devolve (clube, true, false). Senão, os clubes (até a primeira seleção, 0x53b900: push; add esp, 4 -> cdecl) do
/// continente com reputação (+0x80, word) acima de `reputacao`, numa lista local; com algum, a lista ordenada e
/// embaralhada com 1 escolha (0x4afbd0(lista, n, 1): 3 pushes; add esp, 0xc -> cdecl) e o primeiro (clube, false,
/// false); sem nenhum, a
/// caixa da linha `linha` e o campeão da competição, que a original grava SEMPRE na vaga 1 (+6), mesmo na busca da
/// vaga 0 (um defeito: a vaga 0 fica com o que a memória alocada tinha); devolve (clube, false, true).
unsafe fn cqu13_vaga(this: usize, continente: usize, injury_word_0c: i16, comp: usize, lugar: u32, arquivo: usize, linha: u32) -> (usize, bool, bool) {
    let carregado: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x6699d0usize);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    let registro = cqu13_d(0xae23a4).wrapping_add(cqu13_d(continente).wrapping_mul(0xc6));
    if carregado(registro) != 0 || cqu13_w(this + 0x40) == 0x7d1 {
        return (colocado(cqu13_comp(comp), lugar), true, false);
    }
    let mut lista: Vec<usize> = Vec::new();
    let mut c = cqu13_d(0xae23b4);
    for _ in 0..(cqu13_d(0xae235c) as i32).max(0) {
        if selecao(c) != 0 {
            break;
        }
        if cqu13_w(c + 0x80) > injury_word_0c && cqu13_do_continente(c, continente) {
            lista.push(c);
        }
        c += 0x245;
    }
    if !lista.is_empty() {
        embaralhar(lista.as_mut_ptr() as usize, lista.len() as u32, 1);
        return (lista[0], false, false);
    }
    show_error_at(arquivo, linha);
    (colocado(cqu13_comp(comp), 0), false, true)
}

/// O começo das duas copas: +0x3e = 2 e a tabela +0xa7 de 2 times de 6 bytes (0x944e46(0xc): push; add esp, 4 ->
/// cdecl; sem memória, a caixa da linha 0xc6 e −1).
unsafe fn cqu13_tabela(this: usize, arquivo: usize) -> Option<usize> {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 2);
    let t = alocar(0xc);
    cqu13_pd(this + 0xa7, t);
    if t == 0 {
        show_error_at(arquivo, 0xc6);
        return None;
    }
    Some(t)
}

/// O fim das duas copas: os bytes +4, +5, +0xa e +0xb da tabela zerados; devolve 1.
unsafe fn cqu13_fim(this: usize) -> i32 {
    for k in [4usize, 5, 0xa, 0xb] {
        *((cqu13_d(this + 0xa7) + k) as *mut u8) = 0;
    }
    1
}

/// { 0x92b8b0 } (thiscall, ret 0, world_club_cup.cpp, pilha 0x11a4): `WorldClubCup::pick_qualifiers()`, os dois clubes
/// da Copa Intercontinental. A vaga 0 é da Europa: o campeão da Liga dos Campeões [0x9cf6e8], ou um clube europeu de
/// reputação acima de 5.000 sorteado quando a Europa não tem liga carregada fora de 2001 (caixa na linha 0xea sem
/// nenhum; campeão nulo: a caixa da linha 0xd1 e 0). A vaga 1 é da América do Sul: o campeão da Libertadores
/// [0x9cf63c] ou o sorteio do mesmo jeito (caixas nas linhas 0x12a e 0xff); quando é o campeão e ele é sul-americano,
/// o vice da Libertadores entra no lugar dele se também for sul-americano. Devolve 1 (−1 sem memória).
unsafe extern "thiscall" fn cqu13_intercontinental_clubes(this: usize) -> i32 {
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let Some(t) = cqu13_tabela(this, 0xad5848) else {
        return -1;
    };
    let (europa, pelo_campeao, reserva) = cqu13_vaga(this, 0x9cfa10, 0x1388, 0x9cf6e8, 0, 0xad5848, 0xea); // cqu13: o campeão europeu
    cqu13_pd(t + if reserva { 6 } else { 0 }, europa);
    if pelo_campeao {
        crate::comparison::branch("cqu_13 clube europeu");
        if europa == 0 {
            show_error_at(0xad5848, 0xd1);
            return 0;
        }
    }
    let (sul, pelo_campeao, _) = cqu13_vaga(this, 0x9cfa1c, 0x1388, 0x9cf63c, 0, 0xad5848, 0x12a);
    cqu13_pd(cqu13_d(this + 0xa7) + 6, sul);
    if pelo_campeao {
        if sul == 0 {
            show_error_at(0xad5848, 0xff);
            return 0;
        }
        if cqu13_do_continente(sul, 0x9cfa1c) {
            let vice = colocado(cqu13_comp(0x9cf63c), 1);
            if cqu13_do_continente(vice, 0x9cfa1c) {
                cqu13_pd(cqu13_d(this + 0xa7) + 6, vice);
            }
        }
    }
    cqu13_fim(this)
}

// ponytail: o sorteio (na Europa ou na América do Sul sem liga) e a tabela alocada não rodam duas vezes. Só a original;
// o plano: devolve 1, +0x3e = 2, os dois clubes nulos não, o primeiro de um continente e o segundo do outro, os bytes
// de estado zerados.
unsafe fn cqu13_comparar_clubes(nome: &str, this: usize, continentes: (usize, usize), o: impl FnOnce() -> i32) -> i32 {
    let r = o();
    if r == 1 {
        let t = cqu13_d(this + 0xa7);
        let (a, b) = (cqu13_d(t), cqu13_d(t + 6));
        let bytes = [4usize, 5, 0xa, 0xb].iter().all(|&k| *((t + k) as *const u8) == 0);
        let ok = cqu13_w(this + 0x3e) == 2 && a != 0 && b != 0 && cqu13_do_continente(a, continentes.0) && cqu13_do_continente(b, continentes.1) && bytes;
        PLACAR.record(ok, || format!("{}({:#x}): clubes {:#x} {:#x} estado zerado {}", nome, this, a, b, bytes));
    }
    r
}

unsafe extern "thiscall" fn comparar_cqu13_intercontinental_clubes(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I0%%);
    cqu13_comparar_clubes("cqu13_intercontinental_clubes", this, (0x9cfa10, 0x9cfa1c), || o(this))
}

/// { 0x632480 } (thiscall, ret 0, inter_amer_cup.cpp, pilha 0x11a4): `InterAmerCup::select_teams()`, os dois clubes da
/// Copa Interamericana. A vaga 0 é da América do Norte: o campeão da Copa dos Campeões da CONCACAF [0x9cf72c], ou um
/// clube norte-americano de reputação acima de 1.000 sorteado quando ela não tem liga carregada fora de 2001 (caixas
/// nas linhas 0xea e 0xd1); a vaga 1 é da América do Sul: o campeão da Libertadores [0x9cf63c] ou o sorteio acima de
/// 5.000 (caixas nas linhas 0x118 e 0xff). Devolve 1 (0 com campeão nulo; −1 sem memória).
unsafe extern "thiscall" fn cqu13_interamericana_clubes(this: usize) -> i32 {
    let Some(t) = cqu13_tabela(this, 0x9e9d94) else {
        return -1;
    };
    let (norte, pelo_campeao, reserva) = cqu13_vaga(this, 0x9cfa14, 0x3e8, 0x9cf72c, 0, 0x9e9d94, 0xea); // cqu13: o campeão da CONCACAF
    cqu13_pd(t + if reserva { 6 } else { 0 }, norte);
    if pelo_campeao {
        crate::comparison::branch("cqu_13 clube norte-americano");
        if norte == 0 {
            show_error_at(0x9e9d94, 0xd1);
            return 0;
        }
    }
    let (sul, pelo_campeao, _) = cqu13_vaga(this, 0x9cfa1c, 0x1388, 0x9cf63c, 0, 0x9e9d94, 0x118); // cqu13: o campeão da Libertadores na Interamericana
    cqu13_pd(cqu13_d(this + 0xa7) + 6, sul);
    if pelo_campeao {
        crate::comparison::branch("cqu_13 clube sul-americano da interamericana");
        if sul == 0 {
            show_error_at(0x9e9d94, 0xff);
            return 0;
        }
    }
    cqu13_fim(this)
}

unsafe extern "thiscall" fn comparar_cqu13_interamericana_clubes(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    cqu13_comparar_clubes("cqu13_interamericana_clubes", this, (0x9cfa14, 0x9cfa1c), || o(this))
}

/// As chamadas das posições previstas das copas da CONMEBOL (0x4c2a90, 0x4c4210 e 0x4c5a90, o mesmo corpo), na ordem:
/// a lista dos convidados (+0xee, +0xf2 clubes) já ordenada; as rodadas da tabela `tabela` (linhas de 0x68 bytes; o
/// word +0x1a de cada uma, os que saem nela), da última para a primeira: a última dá a posição 1 aos que saem nela e,
/// como todas, a posição corrente (que sobe a cada clube) a mais tantos; os que restam ficam em blocos de `grupo` com a
/// posição do começo do bloco. Devolve (clube, posição).
unsafe fn cqu13_plano_previstas(this: usize, tabela: usize, rodadas: i16, grupo: u32) -> Vec<(usize, u8)> {
    let mut v = Vec::new();
    let lista = cqu13_d(this + 0xee);
    let mut i: i16 = 0;
    let mut bl: u8 = 1;
    let mut atual: u8 = 1;
    let mut r = rodadas as i32 - 1;
    while r >= 0 {
        let p = tabela + r as usize * 0x68 + 0x1a;
        if r == rodadas as i32 - 1 {
            let mut j: i16 = 0;
            while j < cqu13_w(p) {
                v.push((cqu13_d(lista + i as isize as usize * 4), atual));
                i = i.wrapping_add(1);
                bl = bl.wrapping_add(1);
                j += 1;
            }
        }
        atual = bl;
        let mut j: i16 = 0;
        while j < cqu13_w(p) {
            v.push((cqu13_d(lista + i as isize as usize * 4), atual));
            i = i.wrapping_add(1);
            bl = bl.wrapping_add(1);
            j += 1;
        }
        r -= 1;
    }
    let mut pos = bl;
    while (i as i32) < cqu13_d(this + 0xf2) as i32 {
        v.push((cqu13_d(lista + i as isize as usize * 4), pos));
        bl = bl.wrapping_add(1);
        if bl as u32 == pos as u32 + grupo {
            pos = bl;
        }
        i = i.wrapping_add(1);
    }
    v
}

/// O corpo das posições previstas: sem estatísticas (+8), a caixa `linha`; a lista +0xee de +0xf2 convidados ordenada
/// pela reputação (qsort 0x945333(lista, n, 4, 0x4b07d0): 4 pushes; add esp -> cdecl); a tabela das rodadas da fase
/// `fase` pela virtual +0x3c (push 0; push &extra; push &rodadas; push fase; mov ecx, this -> thiscall, ret 0x10); cada
/// clube do plano com a sua posição (Comp_Stats::set_club_final_position 0x4a2540: push posição; push clube; mov ecx,
/// [+8] -> thiscall, ret 8; a original empurra o byte com sobra nos bytes altos, a 0x4a2540 lê o byte); a tabela
/// liberada (0x9452ca).
unsafe fn cqu13_previstas(this: usize, fase: u32, grupo: u32, arquivo: usize, linha: u32) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cqu13_d(this + 8) == 0 {
        return show_error_at(arquivo, linha);
    }
    ordenar(cqu13_d(this + 0xee), cqu13_d(this + 0xf2) as u32, 4, 0x4b07d0);
    let tabela_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, u32) -> usize = core::mem::transmute(cqu13_d(cqu13_d(this) + 0x3c));
    let (mut rodadas, mut extra) = (0i16, 0i16);
    let tabela = tabela_da_copa(this, fase, &mut rodadas, &mut extra, 0);
    for (c, p) in cqu13_plano_previstas(this, tabela, rodadas, grupo) {
        gravar(cqu13_d(this + 8), c, p as u32);
    }
    soltar(tabela);
}

/// { 0x4c2a90 } (thiscall, ret 0, con_merc_cup.cpp): `ConMercCup::set_predicted_positions()`: as posições previstas da
/// Merconorte, pelas rodadas da fase 3 (o mata-mata) e em blocos de 4 para os que ficam nos grupos (caixa na linha
/// 0x32e).
unsafe extern "thiscall" fn cqu13_merconorte_previstas(this: usize) {
    cqu13_previstas(this, 3, 4, 0x9a7d6c, 0x32e)
}

// ponytail: Comp_Stats::set_club_final_position acrescenta o registro do clube (realoca): não roda duas vezes. Só a
// original; o plano, com a lista já ordenada por ela: cada convidado com registro e as posições (+0x72, 0x4a4850)
// crescentes na ordem da lista, a primeira 1. (Pedir de novo a tabela das rodadas da fase mexe no calendário da copa:
// a comparação de 07/11 deu a caixa da linha 1278 do Cup.cpp; o plano não chama mais nada além da busca.)
unsafe fn cqu13_comparar_previstas(nome: &str, this: usize, _fase: u32, _grupo: u32, o: impl FnOnce()) {
    o();
    let est = cqu13_d(this + 8);
    let lista = cqu13_d(this + 0xee);
    let n = cqu13_d(this + 0xf2) as i32;
    if est == 0 || lista == 0 || !(1..=256).contains(&n) {
        return;
    }
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut lidas: Vec<i32> = Vec::new();
    for k in 0..n as usize {
        let c = cqu13_d(lista + k * 4);
        let e = if c != 0 { buscar(est, c) } else { 0 };
        lidas.push(if e != 0 { *((e + 0x72) as *const u8) as i32 } else { -1 });
    }
    let ok = lidas[0] == 1 && lidas.iter().all(|&x| x >= 1) && lidas.windows(2).all(|w| w[0] <= w[1]);
    PLACAR.record(ok, || format!("{}({:#x}): posições {:?}", nome, this, lidas));
}

unsafe extern "thiscall" fn comparar_cqu13_merconorte_previstas(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    cqu13_comparar_previstas("cqu13_merconorte_previstas", this, 3, 4, || o(this))
}
