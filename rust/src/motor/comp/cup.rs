use super::*;

// ---------- 266a leva: a copa: os jogos do confronto, a notícia do confronto pendente e os avisos das janelas de transferência (Cup.cpp, transfer_manager.cpp) ----------







/// True when the club is run by a human manager (News::club_has_human_manager on G_NEWS).
///
/// [pt] O clube é dirigido por um técnico humano (0x7875e0: push clube; ecx = [0xb7431c]; sem add esp -> thiscall(notícias,
/// clube), ret 4).
pub(crate) unsafe fn cup_has_human_manager(clube: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    f(read_u32_as_usize(G_NEWS), clube) != 0
}

/// Moves a fixture's date (+0x2a day, +0x28 year) to tomorrow if it is not after today, then adds the fixture to the
/// fixture manager.
///
/// [pt] A data da partida (+0x2a dia, +0x28 ano, pelos dwords que a original empurra: 0x549ef0 push ano; push dia; lea ecx,
/// local -> thiscall, ret 8) que não estiver depois de hoje ([0xae2c90]: ano sem sinal, dia com sinal) passa a amanhã
/// (0x54c5b0: push 1; push saída; mov ecx, 0xae2c90 -> thiscall(data, saída, n), ret 8); e a partida vai ao calendário
/// (0x5aa680: push 1; push &partida; ecx = [0xae2a58] -> thiscall, ret 8).
pub(crate) unsafe fn add_fixture_to_calendar(p: usize) {
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_ADD_FIXTURE);
    let (mut t, mut saida) = ([0u8; 8], [0u8; 8]);
    let f = nova(t.as_mut_ptr() as usize, read_u32_as_usize(p + 0x2a) as u32, read_u32_as_usize(p + 0x28) as u32);
    let (hd, ha) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let fa = read_i16(f + 2) as u16;
    let antes = ha < fa || (ha == fa && hd < read_i16(f));
    if !antes {
        let s = saida.as_mut_ptr() as usize;
        mais(0xae2c90, s, 1);
        write_u16(p + 0x2a, read_i16(s) as u16);
        write_u16(p + 0x28, read_i16(s + 2) as u16);
    }
    juntar(read_u32_as_usize(G_FIXTURE_MANAGER), p, 1);
}

/// creates the fixture of the tie (or only marks it as deferred): competition, round, rules, clubs, date (tomorrow if
/// already past), slot, stadium and calendar; the news to human managers; in a two-legged round, the second leg +0x22
/// days later, with home and away swapped
/// Original: `Cup::create_tie_fixtures` (0x51cb90, Cup.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x51cb90 } (thiscall, ret 0xc, Cup.cpp, quadro SEH sem destrutor no caminho normal: as datas e a notícia locais):
/// `Cup::create_tie_fixtures(rodada, confronto, adiar)`, da montagem da rodada (0x51c800) e da tela da competição.
/// Confronto −1: o primeiro da rodada com a marca +0x34 = 1 (adiar) ou 2 (criar); nenhum: 0. Adiar: marca 2. Criar:
/// marca 3; com time faltando (+0/+2 = −1), a notícia do confronto pendente (0x522950: push confronto; push rodada;
/// mov ecx, esi -> thiscall, ret 8). Senão a partida numa local de 0x4f bytes: a competição (+0x14) e o id (+0, −1
/// sem), −1 em +8, o ano da copa (+0x40) em +0x2c, +0x3a em +0x32, da rodada (0x68 bytes em +0xa3) o código +7 (+3
/// na de duas mãos, +0x21 = 2) em +0x30, +9 em +0x34, +0xb em +0x36, as regras +0xd em +0x38 (sem mão de volta e com
/// 0x800: 0x1000 se o nível (+4 do time) do mandante não é menor que o do visitante, senão 0x800), +0x17 em +0x3a, a
/// liga (+0x44) em +0x42, as marcas (0xff, 1, 0), o virtual +0x7c da copa (push &partida; mov ecx, esi), o bit 0x400
/// de +0x4d (sem mão de volta e +0x20 > 0), os clubes (+0x1c, +0x20) e ids (+0xc, +0x10; −1 sem clube), a data (dia
/// +3, ano +5 + o da copa), a fatia +6, a data da rodada seguinte (+0x6b/+0x6d; na última, 0), o estádio (0x85c260:
/// push +0x24 da copa; push &partida; push +0x13; mov ecx, [0xdd7ef4] -> thiscall, ret 0xc) e o calendário
/// (add_fixture_to_calendar). Com técnico humano num dos dois e sem o bit 0x100 em +0xd, a notícia 0x14 aos dois. Na de duas
/// mãos, com a copa ainda em [0xadadfc]: a volta +0x22 dias depois (0x549ef0, 0x54c5b0, 0x417c10), o código +4, a
/// fatia pelo dia da semana (0x54a3f0: sábado ou domingo 1, senão 2), mando invertido, o estádio e o calendário.
/// Devolve 1.
pub(crate) unsafe extern "thiscall" fn create_tie_fixtures(this: usize, rodada: u32, confronto: u32, adiar: u32) -> u32 {
    let id = read_u32_as_usize(read_u32_as_usize(this + 4));
    let ri = rodada as i16 as i32;
    let (mut k, mut adiar) = (confronto, adiar);
    if confronto as i16 == -1 {
        let n = read_i16(round_entry(this, ri) + 0x1a);
        let mut i: u32 = 0;
        loop {
            if (i as i16) >= n {
                return 0;
            }
            let m = read_u8(tie_of_round(this, ri, i as i16 as i32) + 0x34);
            if m == 1 {
                adiar = 1;
                break;
            }
            if m == 2 {
                adiar = 0;
                break;
            }
            i = i.wrapping_add(1);
        }
        k = i;
        if k as i16 == -1 {
            return 0;
        }
    }
    let t = tie_of_round(this, ri, k as i16 as i32);
    if adiar != 0 {
        *((t + 0x34) as *mut u8) = 2;
        return 1;
    }
    *((t + 0x34) as *mut u8) = 3;
    if read_i16(t) == -1 || read_i16(t + 2) == -1 {
        let pendente: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_CUP_POST_DRAW_NEWS);
        pendente(this, rodada, k);
        return 1;
    }
    let mut buf = [0u8; 0x50];
    let p = buf.as_mut_ptr() as usize;
    let comp = read_u32_as_usize(this + 4);
    if comp != 0 {
        write_u32_from_usize(p + 0x14, comp);
        write_u32_from_usize(p, read_u32_as_usize(comp));
    } else {
        write_u32_from_usize(p, usize::MAX);
        write_u32_from_usize(p + 0x14, 0);
    }
    write_u16(p + 0x2c, read_i16(this + 0x40) as u16);
    write_u16(p + 0x32, read_i16(this + 0x3a) as u16);
    let e = round_entry(this, ri);
    let mut codigo = read_i16(e + 7);
    if read_u8(e + 0x21) == 2 {
        codigo = codigo.wrapping_add(3);
    }
    write_u16(p + 0x30, codigo as u16);
    write_u16(p + 0x34, read_i16(e + 9) as u16);
    write_u8(p + 0x3a, read_u8(e + 0x17));
    write_u16(p + 0x36, read_i16(e + 0xb) as u16);
    // ponytail: a original faz and 0xf4ff na palavra +0x4d de um local sem valor (o +0x4d e os bits 2, 4..7 de
    // +0x4e ficam com a sobra de pilha) e não grava +4, +0x18, +0x2e, +0x2f, +0x3c antes do estádio: aqui, zero
    write_u16(p + 0x38, read_i16(e + 0xd) as u16);
    for x in [0x43usize, 0x44, 0x4b, 0x4c, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a] {
        write_u8(p + x, 0xff);
    }
    write_u32_from_usize(p + 8, usize::MAX);
    write_u8(p + 0x42, read_u8(this + 0x44));
    write_u8(p + 0x3d, 0);
    write_u8(p + 0x3e, 0);
    write_u8(p + 0x3b, 0xff);
    write_u8(p + 0x40, 1);
    write_u8(p + 0x41, 1);
    if (read_u8(e + 0x21) as i8) < 2 && read_i16(p + 0x38) as u16 & 0x800 != 0 {
        let nc = read_u8(cup_team_entry(this, read_i16(t)) + 4) as i8;
        let nf = read_u8(cup_team_entry(this, read_i16(t + 2)) + 4) as i8;
        write_u16(p + 0x38, if nc >= nf { 0x1000 } else { 0x800 });
    }
    let virtual_7c: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x7c));
    virtual_7c(this, p);
    let e = round_entry(this, ri);
    if (read_u8(e + 0x21) as i8) < 2 && (read_u8(e + 0x20) as i8) > 0 {
        write_u8(p + 0x4e, read_u8(p + 0x4e) | 4);
    } else {
        write_u8(p + 0x4e, read_u8(p + 0x4e) & !4);
    }
    let t = tie_of_round(this, ri, k as i16 as i32);
    let (lc, lf) = (read_i16(t), read_i16(t + 2)); // cqc5: os lados da ida
    let casa = read_u32_as_usize(cup_team_entry(this, lc));
    write_u32_from_usize(p + 0x1c, casa);
    write_u32_from_usize(p + 0xc, if casa != 0 { read_u32_as_usize(casa) } else { usize::MAX });
    let fora = read_u32_as_usize(cup_team_entry(this, lf));
    write_u32_from_usize(p + 0x20, fora);
    write_u32_from_usize(p + 0x10, if fora != 0 { read_u32_as_usize(fora) } else { usize::MAX });
    let ano_copa = read_i16(this + 0x40);
    write_u16(p + 0x28, (read_u8(e + 5) as i8 as i16).wrapping_add(ano_copa) as u16);
    write_u16(p + 0x2a, read_i16(e + 3) as u16); // cqc5: o dia da ida
    write_u8(p + 0x3f, read_u8(e + 6));
    if ri < read_i16(this + 0x3c) as i32 - 1 {
        write_u16(p + 0x26, read_i16(e + 0x6b) as u16);
        write_u16(p + 0x24, (read_u8(e + 0x6d) as i8 as i16).wrapping_add(read_i16(this + 0x40)) as u16);
    } else {
        write_u16(p + 0x26, 0);
        write_u16(p + 0x24, 0);
    }
    let estadio: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_STADIUM_SET_FIXTURE_VENUE);
    estadio(read_u32_as_usize(G_STADIUM), read_u32_as_usize(e + 0x13), p, read_u32_as_usize(this + 0x24));
    crate::comparacao::ramo("cqc_5 jogo do confronto");
    add_fixture_to_calendar(p);
    let (casa, fora) = (read_u32_as_usize(p + 0x1c), read_u32_as_usize(p + 0x20));
    if (cup_has_human_manager(casa) || cup_has_human_manager(fora)) && read_i16(round_entry(this, ri) + 0xd) as u16 & 0x100 == 0 {
        let mut noticia = [0u32; 0x38];
        let n = noticia.as_mut_ptr() as usize;
        let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
        let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
        let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
        construir(n, 0x14, 0);
        gravar(n, 0, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32);
        gravar(n, 1, read_i16(this + 0x3a) as i32 as u32);
        gravar(n, 2, read_i16(round_entry(this, ri) + 7) as i32 as u32);
        gravar(n, 5, (read_u8(round_entry(this, ri) + 0x21) == 2) as u32);
        gravar(n, 6, read_u32_as_usize(p + 4) as u32);
        gravar(n, 7, read_i16(p + 0x2a) as u16 as u32);
        gravar(n, 8, read_i16(p + 0x28) as u16 as u32);
        if cup_has_human_manager(casa) {
            gravar(n, 3, read_u32_as_usize(p + 0xc) as u32);
            gravar(n, 4, read_u32_as_usize(p + 0x10) as u32);
        } else {
            gravar(n, 3, read_u32_as_usize(p + 0x10) as u32);
            gravar(n, 4, read_u32_as_usize(p + 0xc) as u32);
        }
        entregar(read_u32_as_usize(G_NEWS), n, casa);
        entregar(read_u32_as_usize(G_NEWS), n, fora);
    }
    let e = round_entry(this, ri);
    if read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + id * 4) == 0 || read_u8(e + 0x21) != 2 {
        return 1;
    }
    crate::comparacao::ramo("cqc_5 jogo de volta");
    if (read_u8(e + 0x20) as i8) < 1 {
        write_u8(p + 0x4e, read_u8(p + 0x4e) & !4);
    } else {
        write_u8(p + 0x4e, read_u8(p + 0x4e) | 4);
    }
    write_u16(p + 0x30, read_i16(e + 7).wrapping_add(4) as u16);
    let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_DATE_GET_WEEKDAY);
    let (mut a, mut s, mut c, mut x) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let pa = a.as_mut_ptr() as usize;
    // ponytail: a original empurra o dia e o ano em registradores com sobra nos 16 bits altos; a 0x549ef0 só lê as
    // palavras
    nova(pa, read_i16(e + 3) as u16 as u32, (read_u8(e + 5) as i8 as i16).wrapping_add(read_i16(this + 0x40)) as u16 as u32);
    let r = mais(pa, s.as_mut_ptr() as usize, read_u8(e + 0x22) as u32);
    copia(pa, c.as_mut_ptr() as usize, r);
    write_u16(p + 0x28, read_i16(pa + 2) as u16);
    write_u16(p + 0x2a, read_i16(pa) as u16);
    let dia = nova(x.as_mut_ptr() as usize, read_u32_as_usize(pa) as u32, read_u32_as_usize(pa + 2) as u32);
    let sd = semana(dia) as i16;
    write_u8(p + 0x3f, if sd == 5 || sd == 6 { 1 } else { 2 });
    let (c1, c2, i1, i2) = (read_u32_as_usize(p + 0x1c), read_u32_as_usize(p + 0x20), read_u32_as_usize(p + 0xc), read_u32_as_usize(p + 0x10)); // cqc5: os lados da volta
    write_u8(p + 0x4e, read_u8(p + 0x4e) & 0xf4);
    write_u32_from_usize(p + 0x20, c1);
    write_u32_from_usize(p + 0xc, i2);
    write_u32_from_usize(p + 0x10, i1);
    write_u32_from_usize(p + 0x1c, c2);
    let e = round_entry(this, ri);
    estadio(read_u32_as_usize(G_STADIUM), read_u32_as_usize(e + 0x13), p, read_u32_as_usize(this + 0x24));
    add_fixture_to_calendar(p);
    1
}

/// (year, day of year) + n days, rolling over 365/366-day years.
///
/// [pt] (ano, dia) + n dias (o dia vai de 0 a 364, ou 365 no bissexto da 0x549ef0).
pub(crate) fn add_days_to_day_year((ano, dia): (i32, i32), n: i32) -> (i32, i32) {
    let (mut a, mut d) = (ano, dia + n);
    loop {
        let t = if crate::data::bissexto(a as u16) == 1 { 366 } else { 365 };
        if d < t {
            return (a, d);
        }
        d -= t;
        a += 1;
    }
}

/// Counts the club's fixtures in the competition from today to (year, day) via FixtureManager::find_fixtures, for the
/// comparer.
///
/// [pt] Os jogos do clube (id) na competição de hoje até (ano, dia) (0x5acbe0: lista nula, tipo 0xfe e lado 0xff
/// quaisquer), para o comparador. A janela não passa de onde a 0x5aa680 pode pôr os jogos novos + 14 dias: a busca cria
/// o bloco do ano que faltar (0x5abf10).
pub(crate) unsafe fn club_fixtures_until(clube: u32, comp: u32, (ano, dia): (i32, i32)) -> u32 {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    achar(read_u32_as_usize(G_FIXTURE_MANAGER), read_i16(G_CURRENT_DATE) as u16 as u32, read_i16(G_CURRENT_DATE_YEAR) as u16 as u32, dia as u32, ano as u32, clube, 0,
        comp, 0xfe, 0xff) & 0xffff
}

/// Comparador da 0x51cb90: só a original (a partida vai ao calendário pela 0x5aa680, que aloca, e as notícias aos
/// técnicos). O plano, feito antes: o confronto escolhido (o −1 procura a marca 1 ou 2), a devolução, a marca (2 ao
/// adiar, 3 ao criar) e, criando com os dois times, os jogos do mandante na competição sobem 1 (2 na de duas mãos com a
/// copa ainda ativa).
pub(crate) unsafe extern "thiscall" fn comparar_create_tie_fixtures(this: usize, rodada: u32, confronto: u32, adiar: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(IDX_CREATE_TIE_FIXTURES);
    let ri = rodada as i16 as i32;
    let id = read_u32_as_usize(read_u32_as_usize(this + 4)) as u32;
    let (mut k, mut ad) = (confronto as i16 as i32, adiar);
    if k == -1 {
        let n = read_i16(round_entry(this, ri) + 0x1a) as i32;
        k = -1;
        for i in 0..n.max(0) {
            let m = read_u8(tie_of_round(this, ri, i) + 0x34);
            if m == 1 || m == 2 {
                k = i;
                ad = (m == 1) as u32;
                break;
            }
        }
        if k == -1 {
            let r = o(this, rodada, confronto, adiar);
            PLACAR.registrar(r == 0, || format!("create_tie_fixtures(rodada {}) sem confronto marcado: {}", ri, r));
            return r;
        }
    }
    let t = tie_of_round(this, ri, k);
    let completo = read_i16(t) != -1 && read_i16(t + 2) != -1;
    let e = round_entry(this, ri);
    let duas = read_u8(e + 0x21) == 2;
    let casa = if completo { read_u32_as_usize(cup_team_entry(this, read_i16(t))) } else { 0 };
    let ic = if casa != 0 { read_u32_as_usize(casa) as u32 } else { u32::MAX };
    let conferir = ad == 0 && completo && ic != u32::MAX;
    // o fim da janela: a data da rodada (+ os dias até a volta), amanhã ou a data da rodada seguinte (+0x6b/+0x6d), o
    // que vier depois, + 14 dias. Com conflito, a 0x5aa680 remarca o jogo (0x5ab510): copa (tipo 2) até a data da
    // rodada seguinte (sem ela, 2 semanas), os outros tipos até 2 semanas depois; com mais ligas acontece (a 1a rodada
    // da copa 101, 2001-225, foi a mais de 7 dias)
    let ida = (read_u8(e + 5) as i8 as i32 + read_i16(this + 0x40) as i32, read_i16(e + 3) as i32);
    let ultimo = if duas { add_days_to_day_year(ida, read_u8(e + 0x22) as i32) } else { ida };
    let amanha = add_days_to_day_year((read_i16(G_CURRENT_DATE_YEAR) as u16 as i32, read_i16(G_CURRENT_DATE) as i32), 1);
    let seguinte = if ri < read_i16(this + 0x3c) as i32 - 1 {
        (read_u8(e + 0x6d) as i8 as i32 + read_i16(this + 0x40) as i32, read_i16(e + 0x6b) as i32)
    } else {
        ultimo
    };
    let fim = add_days_to_day_year(ultimo.max(amanha).max(seguinte), 14);
    let antes = if conferir { club_fixtures_until(ic, id, fim) } else { 0 };
    let r = o(this, rodada, confronto, adiar);
    let marca = read_u8(t + 0x34);
    let mut ok = r == 1 && marca == if ad != 0 { 2 } else { 3 };
    let mut esperado = 0;
    let mut depois = 0;
    if conferir && read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + id as usize * 4) != 0 {
        esperado = 1 + duas as u32;
        depois = club_fixtures_until(ic, id, fim);
        ok = ok && depois == antes + esperado;
    }
    PLACAR.registrar(ok, || format!("create_tie_fixtures(rodada {}, confronto {}, adiar {}) devolveu {} marca {} jogos {} -> {} (esperado +{})",
        ri, k, ad, r, marca, antes, depois, esperado));
    r
}

/// news 0x26 of a tie with an undecided team: the cup, the round and the possible teams, to the managers of each
/// Original: `Cup::post_draw_news` (0x522950, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x522950 } (thiscall, ret 8, Cup.cpp, quadro SEH sem destrutor no caminho normal: a notícia local):
/// `Cup::post_draw_news(rodada, confronto)`, só da 0x51cb90. Com os dois times definidos, ou rodada (0x68 bytes em
/// +0xa3) com menos de 2 confrontos (+0x1a): nada. Cada lado sem time (+0/+2 = −1) vem do confronto de origem (rodada
/// +0xe/+0x10, confronto +0xa/+0xc; sem origem ou com time faltando lá: nada): os dois clubes possíveis; com time, o
/// clube dele (+0xa7, 6 bytes). A notícia 0x26: 0 = o id da copa, 1 = +0x3a, 2 = o código da rodada (+7), 3/4 = o
/// mandante e −1 ou os dois possíveis, 5/6 = o visitante e −1 ou os dois possíveis; aos técnicos de cada clube dela
/// (0x7861e0: push clube; push &notícia; mov ecx, [0xb7431c] -> thiscall, ret 8; os possíveis só se não nulos).
pub(crate) unsafe extern "thiscall" fn post_draw_news(this: usize, rodada: u32, confronto: u32) {
    let ri = rodada as i16 as i32;
    let t = tie_of_round(this, ri, confronto as i16 as i32);
    let (mut a1, mut a2, mut b1, mut b2, mut casa) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let h = read_i16(t);
    if h != -1 && read_i16(t + 2) != -1 {
        return;
    }
    if read_i16(round_entry(this, ri) + 0x1a) < 2 {
        return;
    }
    if h == -1 {
        let r2 = read_i16(t + 0xe);
        if r2 == -1 {
            return;
        }
        let o = tie_of_round(this, r2 as i32, read_i16(t + 0xa) as i32);
        let (x, y) = (read_i16(o), read_i16(o + 2));
        if x == -1 || y == -1 {
            return;
        }
        a1 = read_u32_as_usize(cup_team_entry(this, x));
        a2 = read_u32_as_usize(cup_team_entry(this, y));
    } else {
        casa = read_u32_as_usize(cup_team_entry(this, h));
    }
    let fora;
    let v = read_i16(t + 2);
    if v == -1 {
        let r2 = read_i16(t + 0x10);
        if r2 == -1 {
            return;
        }
        let o = tie_of_round(this, r2 as i32, read_i16(t + 0xc) as i32);
        let (x, y) = (read_i16(o), read_i16(o + 2));
        if x == -1 || y == -1 {
            return;
        }
        b1 = read_u32_as_usize(cup_team_entry(this, x));
        b2 = read_u32_as_usize(cup_team_entry(this, y));
        fora = 0;
    } else {
        fora = read_u32_as_usize(cup_team_entry(this, v));
    }
    crate::comparacao::ramo("cqc_5 confronto pendente");
    let mut noticia = [0u32; 0x38];
    let n = noticia.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    construir(n, 0x26, 0);
    gravar(n, 0, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32);
    gravar(n, 1, read_i16(this + 0x3a) as i32 as u32);
    gravar(n, 2, read_i16(round_entry(this, ri) + 7) as i32 as u32);
    if casa != 0 {
        gravar(n, 3, read_u32_as_usize(casa) as u32);
        gravar(n, 4, u32::MAX);
    } else {
        gravar(n, 3, read_u32_as_usize(a1) as u32);
        gravar(n, 4, read_u32_as_usize(a2) as u32);
    }
    if fora != 0 {
        gravar(n, 5, read_u32_as_usize(fora) as u32);
        gravar(n, 6, u32::MAX);
    } else {
        gravar(n, 5, read_u32_as_usize(b1) as u32);
        gravar(n, 6, read_u32_as_usize(b2) as u32);
    }
    let news = read_u32_as_usize(G_NEWS);
    if casa != 0 {
        entregar(news, n, casa);
    } else {
        if a1 != 0 {
            entregar(news, n, a1);
        }
        if a2 != 0 {
            entregar(news, n, a2);
        }
    }
    if fora != 0 {
        entregar(news, n, fora);
    } else {
        if b1 != 0 {
            entregar(news, n, b1);
        }
        if b2 != 0 {
            entregar(news, n, b2);
        }
    }
}

/// Comparador da 0x522950: só a original (a notícia vai aos técnicos e não tem outro efeito).
pub(crate) unsafe extern "thiscall" fn comparar_post_draw_news(this: usize, rodada: u32, confronto: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_POST_DRAW_NEWS)(this, rodada, confronto) // ponytail: só a notícia
}



// ---------- 264a leva: a copa: o sorteio pela semente, o sorteio que separa as nações e o começo da temporada da copa (Cup.cpp) ----------


/// Takes the teams out of round r's ties into the 7-byte draw array (team, link, level), freeing the slots (-2);
/// returns the entry count.
///
/// [pt] Tira os times dos confrontos da rodada `r` para o vetor de 7 bytes do sorteio (o time +0, o vínculo +2/+4, o
/// nível +6), como nas 0x51d2d0 e 0x51d9f0: o mandante (+0, com +0xe/+0xa) e o visitante (+2, com +0x10/+0xc); com
/// time, a entrada leva o nível do time (+4 do time; a semente), `com_time(vaga)` roda com a vaga ainda intacta e a
/// vaga vira −2 (livre); sem time (−1), a entrada fica −1 com 0xff no nível (o resto da entrada fica como estava).
/// Devolve quantas entradas (a conta de 32 bits, usada em 16).
pub(crate) unsafe fn take_round_teams(copa: usize, r: i32, vetor: usize, mut com_time: impl FnMut(usize)) -> i32 {
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    while (k as i16) < read_i16(round_entry(copa, r) + 0x1a) {
        for (lado, ligado, jogo) in [(0usize, 0xeusize, 0xausize), (2, 0x10, 0xc)] {
            let t = tie_of_round(copa, r, k as i16 as i32);
            let e = index_addr(vetor, n as i16 as i32, 7);
            n = n.wrapping_add(1);
            let time = read_i16(t + lado);
            if time != -1 {
                write_u16(e, time as u16);
                write_u16(e + 2, read_i16(t + ligado) as u16);
                write_u16(e + 4, read_i16(t + jogo) as u16);
                *((e + 6) as *mut u8) = read_u8(cup_team_entry(copa, time) + 4); // a semente do time
                com_time(t + lado);
                write_u16(t + lado, 0xfffe);
            } else {
                write_u16(e, 0xffff);
                *((e + 6) as *mut u8) = 0xff;
            }
        }
        k += 1;
    }
    n
}

/// Puts draw entry e as home or away of tie cur in round r; with a link, sets the linked tie in the next round.
///
/// [pt] A entrada `e` do vetor vai ao confronto `cur` da rodada `r`: no mandante (+0, +0xe, +0xa) ou no visitante (+2,
/// +0x10, +0xc); com vínculo (+2 da entrada ≠ −1), o confronto ligado na rodada seguinte ganha a rodada (+0x12), o
/// confronto sorteado (+6) e o lado (+8: 1 mandante, 0 visitante).
pub(crate) unsafe fn place_team_in_tie(copa: usize, r: i32, rodada: u32, e: usize, cur: i32, mandante: bool) {
    let t = tie_of_round(copa, r, cur as i16 as i32);
    let (lado, ligado, jogo) = if mandante { (0usize, 0xeusize, 0xausize) } else { (2, 0x10, 0xc) };
    write_u16(t + lado, read_i16(e) as u16);
    write_u16(t + ligado, read_i16(e + 2) as u16);
    write_u16(t + jogo, read_i16(e + 4) as u16);
    let seguinte = read_i16(e + 2);
    if seguinte != -1 {
        let v = tie_of_round(copa, seguinte as i32, read_i16(e + 4) as i32);
        write_u16(v + 0x12, rodada as u16);
        write_u16(v + 6, cur as u16);
        write_u16(v + 8, mandante as u16);
    }
}

/// True when cup teams a and b both have clubs of the same nation (+0x53).
///
/// [pt] Os times `a` e `b` (índices em [copa + 0xa7]) têm clube e são da mesma nação (+0x53).
pub(crate) unsafe fn teams_same_nation(copa: usize, a: i16, b: i16) -> bool {
    let c1 = read_ptr(cup_team_entry(copa, a));
    if c1 == 0 {
        return false;
    }
    let c2 = read_ptr(cup_team_entry(copa, b));
    c2 != 0 && read_u32(c2 + 0x53) == read_u32(c1 + 0x53)
}

/// Advances the circular walk over a round's ties, switching to the second pass and then the rule fallback; false when
/// exhausted (error box).
///
/// [pt] O passo da volta pelos confrontos: o próximo (em círculo, 16 bits); de volta ao sorteado, a primeira volta vira a
/// segunda (`segunda`), na segunda cai a regra (`regra`: a nação, ou o "sem a mesma nação" da 0x51d9f0) e, sem ela,
/// devolve false (a caixa).
pub(crate) unsafe fn next_tie_step(copa: usize, r: i32, cur: &mut i32, inicio: i16, segunda: &mut bool, regra: &mut bool) -> bool {
    *cur = cur.wrapping_add(1);
    if (*cur as i16) >= read_i16(round_entry(copa, r) + 0x1a) {
        *cur = 0;
    }
    if *cur as i16 == inicio {
        if !*segunda {
            *segunda = true;
        } else if *regra {
            *regra = false;
        } else {
            return false;
        }
    }
    true
}

/// the round draw by seeding: the teams leave the ties, sorted by seed (highest first), and each goes to a drawn tie;
/// on the first pass only empty ties (with both slots free, the side is drawn), then any free slot, with rule 4 keeping
/// clubs of one nation apart until there is no way
/// Original: `Cup::sort_round_teams` (0x51d2d0, Cup.cpp, thiscall).
///
/// [pt] { 0x51d2d0 } (thiscall, ret 4, Cup.cpp): `Cup::sort_round_teams(rodada)`: o sorteio pela semente (regra 0x400 da
/// rodada). 1) Tira os times dos confrontos para o vetor de 7 bytes (malloc 0x944e46 de 7 × vagas +0x18; sem memória,
/// a caixa da linha 0xbf8 e o jogo para), cada um com a semente (+4 do time). 2) Ordena pela semente, a maior primeiro
/// (qsort 0x945333 com 0x51ebc0). 3) Cada entrada com time (até a primeira sem, nível 0xff) vai a um confronto
/// sorteado (0x90cfc0 dos confrontos), avançando em círculo: na primeira volta, só confrontos com as duas vagas sem
/// time (< 0): com as duas livres (−2), o lado sorteado (0x90cfc0(2) ≠ 0: mandante); só o mandante livre: mandante;
/// só o visitante: visitante. Da segunda volta em diante (a marca não volta a zero), qualquer vaga livre, mas com a
/// regra 4 da rodada (+0xb) o mandante livre não serve quando o visitante do confronto é da mesma nação, e o
/// visitante livre não serve quando o mandante é; dando a volta inteira de novo, cai a regra das nações (também para
/// as entradas seguintes) e, sem ela, a caixa da linha 0xcff e sai. Libera o vetor (0x9452ca).
pub(crate) unsafe extern "thiscall" fn sort_round_teams(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    // os quatro: push n; add esp, 4 (malloc e sorteio) / push p; add esp, 4 (free) / 4 pushes; add esp, 0x10 (qsort) -> cdecl
    let vetor = alocar((read_i16(round_entry(copa, r) + 0x18) as i32).wrapping_mul(7) as u32);
    if vetor == 0 {
        fatal_memory_error_box(0xbf8, 0x9b56dc); // "CUP::sort_round_teams() : Cannot allocate memory"
    }
    let mut nacao = read_u8(round_entry(copa, r) + 0xb) & 4 != 0; // a regra 4 (+0xb) da rodada
    let n = take_round_teams(copa, r, vetor, |_| {}) as i16;
    crate::comparacao::ramo("cqb_11 ordena pela semente");
    ordenar(vetor, n as i32, 7, 0x51ebc0); // cqb11: pela semente, a maior primeiro
    let mut segunda = false;
    let mut j: i16 = 0;
    while j < n {
        let e = index_addr(vetor, j as i32, 7);
        if read_u8(e + 6) == 0xff {
            break;
        }
        let s = sorteio(read_i16(round_entry(copa, r) + 0x1a) as i32);
        let inicio = s as i16;
        let mut cur = s;
        let mandante = loop {
            let t = tie_of_round(copa, r, cur as i16 as i32);
            let (casa, fora) = (read_i16(t), read_i16(t + 2));
            let mut lado: Option<bool> = None;
            if segunda {
                let w0 = read_i16(e);
                let livre = casa == -2 && !(nacao && w0 >= 0 && fora >= 0 && teams_same_nation(copa, w0, fora));
                if fora == -2 {
                    if livre && sorteio(2) != 0 {
                        lado = Some(true);
                    } else if livre || !(nacao && w0 >= 0 && casa >= 0 && teams_same_nation(copa, w0, casa)) {
                        lado = Some(false);
                    }
                } else if livre {
                    lado = Some(true);
                }
            } else if casa < 0 && fora < 0 {
                if fora == -2 {
                    if casa == -2 && sorteio(2) != 0 {
                        crate::comparacao::ramo("cqb_11 semente lado sorteado");
                        lado = Some(true); // cqb11: o lado sorteado
                    } else {
                        lado = Some(false);
                    }
                } else if casa == -2 {
                    lado = Some(true);
                }
            }
            if let Some(m) = lado {
                break m;
            }
            if !next_tie_step(copa, r, &mut cur, inicio, &mut segunda, &mut nacao) {
                show_error_at(0x9b55ac, 0xcff);
                liberar(vetor);
                return;
            }
        };
        place_team_in_tie(copa, r, rodada, e, cur, mandante);
        j = j.wrapping_add(1);
    }
    liberar(vetor);
}

/// Comparador: sorteia (0x90cfc0) e só usa vetores temporários: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
pub(crate) unsafe extern "thiscall" fn comparar_sort_round_teams(copa: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SORT_ROUND_TEAMS);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(all_ties_regions(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| sort_round_teams(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("sort_round_teams(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// the round draw that keeps nations apart: each team's level becomes how many teams of its nation are in the round
/// (the bigger nations go first) and, from the second pass on, a team does not land in a tie with another of its nation
/// while possible
/// Original: `Cup::check_round_teams_for_nation` (0x51d9f0, Cup.cpp, thiscall).
///
/// [pt] { 0x51d9f0 } (thiscall, ret 4, Cup.cpp): `Cup::check_round_teams_for_nation(rodada)`: o sorteio que separa as nações
/// (regra 4 da rodada sem a 0x400). Conta os times de cada nação num vetor zerado de [0xae2350] bytes (0x945501;
/// sem memória, a caixa da linha 0xd27 e o jogo para) enquanto tira os times dos confrontos (malloc de 7 × vagas, a
/// caixa da linha 0xd34), pelo +0 da nação (+0x53) do clube; o nível de cada entrada com time vira a conta da nação
/// dele (sem nação, fica a semente); libera as contas e ordena, a maior primeiro (as nações com mais times saem
/// antes). Cada entrada (até a primeira com 0xff) vai a um confronto sorteado, em círculo: na primeira volta, só
/// confrontos sem time nas duas vagas (com as duas livres, o lado sorteado; só uma livre, ela); da segunda em diante
/// (a marca fica), qualquer vaga livre, menos (enquanto valer a regra, que volta a valer a cada entrada) um confronto
/// com um time da mesma nação do clube da entrada; dando a volta inteira de novo, cai a regra e, sem ela, a caixa da
/// linha 0xe5c e sai. Libera o vetor.
pub(crate) unsafe extern "thiscall" fn check_round_teams_for_nation(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let zerado: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    // calloc: push 1; push [0xae2350]; add esp, 8 -> cdecl (n, 1); free e qsort: add esp, 0x14 cobre os dois -> cdecl
    let contas = zerado(read_u32(G_NATION_COUNT), 1);
    if contas == 0 {
        fatal_memory_error_box(0xd27, 0x9b5710); // "CUP::check_round_teams_for_nation() : cannot malloc array"
    }
    let vetor = alocar((read_i16(round_entry(copa, r) + 0x18) as i32).wrapping_mul(7) as u32);
    if vetor == 0 {
        fatal_memory_error_box(0xd34, 0x9b56dc);
    }
    let n = take_round_teams(copa, r, vetor, |vaga| {
        let clube = read_ptr(cup_team_entry(copa, read_i16(vaga)));
        if clube != 0 {
            let nacao = read_ptr(clube + club::NATION);
            if nacao != 0 {
                let c = read_ptr(nacao).wrapping_add(contas);
                *(c as *mut u8) = read_u8(c).wrapping_add(1);
            }
        }
    }) as i16;
    if n > 0 {
        let mut e = vetor;
        for _ in 0..n as i32 {
            if read_i16(e) != -1 {
                // ponytail: sem conferir o clube nulo, como a original (que leria o +0x53 dele)
                let nacao = read_ptr(read_ptr(cup_team_entry(copa, read_i16(e))) + 0x53);
                if nacao != 0 {
                    crate::comparacao::ramo("cqb_11 conta da nacao");
                    *((e + 6) as *mut u8) = read_u8(read_ptr(nacao).wrapping_add(contas)); // cqb11: o nível vira a conta da nação
                }
            }
            e += 7;
        }
    }
    liberar(contas);
    ordenar(vetor, n as i32, 7, 0x51ebc0);
    let mut segunda = false;
    let mut j: i16 = 0;
    while j < n {
        let e = index_addr(vetor, j as i32, 7);
        let mut regra = true; // volta a valer a cada entrada
        let clube = if read_i16(e) != -1 { read_ptr(cup_team_entry(copa, read_i16(e))) } else { 0 };
        if read_u8(e + 6) == 0xff {
            break;
        }
        let s = sorteio(read_i16(round_entry(copa, r) + 0x1a) as i32);
        let inicio = s as i16;
        let mut cur = s;
        let mandante = loop {
            let t = tie_of_round(copa, r, cur as i16 as i32);
            let mut lado: Option<bool> = None;
            if segunda {
                let mut livre = false;
                let mut m = false;
                let casa = read_i16(t);
                let c0 = if casa == -2 {
                    m = true;
                    livre = true;
                    0
                } else if casa == -1 {
                    0
                } else {
                    read_ptr(cup_team_entry(copa, casa))
                };
                let fora = read_i16(t + 2);
                let c1 = if fora == -2 {
                    if !(livre && sorteio(2) != 0) {
                        m = false;
                        livre = true;
                    }
                    0
                } else if fora == -1 {
                    0
                } else {
                    read_ptr(cup_team_entry(copa, fora))
                };
                let mesma = clube != 0 && regra
                    && ((c0 != 0 && read_u32(clube + club::NATION) == read_u32(c0 + 0x53)) || (c1 != 0 && read_u32(clube + club::NATION) == read_u32(c1 + 0x53)));
                if !mesma && livre {
                    lado = Some(m);
                }
            } else {
                let (casa, fora) = (read_i16(t), read_i16(t + 2));
                if casa < 0 && fora < 0 {
                    if fora == -2 {
                        lado = Some(casa == -2 && sorteio(2) != 0);
                    } else if casa == -2 {
                        lado = Some(true);
                    }
                }
            }
            if let Some(m) = lado {
                break m;
            }
            if !next_tie_step(copa, r, &mut cur, inicio, &mut segunda, &mut regra) {
                show_error_at(0x9b55ac, 0xe5c);
                liberar(vetor);
                return;
            }
        };
        place_team_in_tie(copa, r, rodada, e, cur, mandante); // cqb11: a entrada vai ao confronto sorteado
        j = j.wrapping_add(1);
    }
    liberar(vetor);
}

/// Comparador: como o da 0x51d2d0 (sorteia; as contas e o vetor são temporários).
pub(crate) unsafe extern "thiscall" fn comparar_check_round_teams_for_nation(copa: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_CHECK_ROUND_TEAMS_FOR_NATION);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(all_ties_regions(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| check_round_teams_for_nation(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("check_round_teams_for_nation(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// the base cup season start: clears byte +0xb1
/// Original: `Cup::start_new_season` (0x518780, Cup.cpp, thiscall).
///
/// [pt] { 0x518780 } (thiscall, ret 0, Cup.cpp): `Cup::start_new_season()` da classe base: só zera o byte +0xb1 da copa
/// (as copas derivadas liberam as tabelas delas).
pub(crate) unsafe extern "thiscall" fn cup_start_new_season(copa: usize) {
    *((copa + 0xb1) as *mut u8) = 0;
}

/// Comparador: lado a lado no byte +0xb1.
pub(crate) unsafe extern "thiscall" fn comparar_cup_start_new_season(copa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_START_NEW_SEASON);
    let reg = [(copa + 0xb1, 1)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa), || cup_start_new_season(copa));
    PLACAR.registrar(igual, || format!("cup_start_new_season({:#x}) {:x?} x {:x?}", copa, mo, mr));
}

// ---------- 263a leva: a rodada da copa: o prêmio da rodada, os mandantes da rodada e o progresso do time (Cup.cpp) ----------





/// Address of the 0x68-byte record of round r (array at cup +0xa3).
///
/// [pt] O registro de 0x68 bytes da rodada `r` (+0xa3).
pub(crate) unsafe fn round_record(copa: usize, r: i32) -> usize {
    read_usize(copa + 0xa3).wrapping_add((r as isize * 0x68) as usize)
}

/// Address of the 6-byte entry of cup team t (+0xa7: +0 club, +4 level, +5 outcome).
///
/// [pt] A entrada de 6 bytes do time `t` (+0xa7: +0 o clube, +4 o nível, +5 o desfecho).
pub(crate) unsafe fn team_entry(copa: usize, t: i32) -> usize {
    read_usize(copa + 0xa7).wrapping_add((t as isize * 6) as usize)
}

/// Address of tie k (0x35 bytes) of round r (per-round arrays at cup +0xad).
///
/// [pt] O confronto `k` (0x35 bytes) da rodada `r` (+0xad: um vetor por rodada).
pub(crate) unsafe fn round_tie(copa: usize, r: i32, k: i32) -> usize {
    read_usize(read_usize(copa + 0xad).wrapping_add((r as isize * 4) as usize)).wrapping_add((k as isize * 0x35) as usize)
}



/// O ensaio do comparador da 0x522240: a cópia roda sem a notícia do prêmio (0x48e3e0, que aloca).
pub(crate) static mut CQA1_ENSAIO: bool = false;

/// Sends the prize money news (Comp 0x48e3e0) for a club, amount, next round, fixture and outcome; nothing in dry-run
/// mode.
///
/// [pt] A notícia do prêmio (Comp 0x48e3e0: 7 pushes; ecx = a copa; sem add esp -> thiscall(copa, clube, valor, alto do
/// valor, −1, rodada seguinte, partida, desfecho), ret 0x1c); no ensaio, nada.
pub(crate) unsafe fn send_prize_news(copa: usize, clube: usize, valor: i32, seguinte: u32, partida: usize, desfecho: u8) {
    if CQA1_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, i32, i32, u32, usize, u32) = core::mem::transmute(FN_COMP_SEND_PRIZE_MONEY_NEWS);
    // ponytail: a original empurra o dword de um local de que só grava o byte baixo (os bits altos são sobra de pilha);
    // a notícia só lê o byte (movsx ecx, byte [esp+0x114]): aqui, o byte limpo.
    f(copa, clube, valor, valor >> 31, -1, seguinte, partida, desfecho as u32);
}

/// Round r's prize (to winner, to loser), from the round record (+0x60/+0x64) or from the prize fund; None where the
/// original division would fault.
///
/// [pt] Os prêmios da rodada `r`: (ao vencedor, ao perdedor), do registro (+0x60/+0x64) ou do fundo (−1); None quando a
/// divisão da original cairia (divisor zero ou estouro). `contar`: o contador do ramo do fundo (não no comparador).
pub(crate) unsafe fn round_prizes(copa: usize, r: i32, contar: bool) -> Option<(i32, i32)> {
    let e = round_record(copa, r);
    let (mut ao_vencedor, mut ao_perdedor) = (read_i32(e + 0x60), read_i32(e + 0x64));
    if ao_vencedor == -1 {
        let (fundo, resta, confrontos) = (read_i32(copa + 0x28), read_i16(copa + 0x3c) as i32 - r, read_i16(e + 0x1a) as i32);
        if resta == 0 || confrontos == 0 || (fundo == i32::MIN && resta == -1) {
            return None;
        }
        let q0 = fundo / resta;
        if q0 == i32::MIN && confrontos == -1 {
            return None;
        }
        if contar {
            crate::comparacao::ramo("cqa_1 premio pelo fundo");
        }
        let q = q0 / confrontos; // cqa1: o fundo pelas rodadas que faltam e pelos confrontos
        if ao_perdedor == -1 {
            ao_vencedor = q.wrapping_mul(3) / 4; // idiv por 4 com o arredondamento para zero (cdq; and 3; sar 2)
            ao_perdedor = q / 4;
        } else {
            ao_vencedor = q;
        }
    }
    Some((ao_vencedor, ao_perdedor))
}

/// pays the round prize money (+0x60 to the winner, +0x64 to the loser; with -1, from the fund +0x28 over the remaining
/// rounds and the ties, 3/4 and 1/4 when both are -1): the winner with the news item (0x48e3e0) and the money
/// (0x5999a0), the loser only the money; the fund loses both
/// Original: `Cup::pay_round_prize_money` (0x522240, Cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x522240 } (thiscall, ret 0x10, Cup.cpp): `Cup::pay_round_prize_money(vencedor, perdedor, rodada, partida)`
/// (entradas de 6 bytes dos dois times). Os prêmios da rodada (+0x60 o do vencedor, +0x64 o do perdedor); com −1 no
/// do vencedor, o fundo (+0x28) / (rodadas +0x3c − rodada) / confrontos (+0x1a), e com −1 também no do perdedor, 3/4
/// para o vencedor e 1/4 para o perdedor. O vencedor (se o prêmio não é zero) recebe a notícia (0x48e3e0: o
/// desfecho +5 do time, 3 se era 1 na partida 0x136, e a rodada seguinte: a palavra +7 dela se não é a última nem
/// 0x8c, sobre os 16 bits altos do ponteiro da rodada, como a original deixa no ebp) e o dinheiro (0x5999a0 nas
/// finanças 0x5a0590); o perdedor só o dinheiro; o fundo perde os dois.
pub(crate) unsafe extern "thiscall" fn pay_round_prize_money(copa: usize, vencedor: usize, perdedor: usize, rodada: u32, partida: usize) {
    let r = rodada as i16 as i32;
    let (ao_vencedor, ao_perdedor) = match round_prizes(copa, r, true) {
        Some(p) => p,
        None => return original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(IDX_PAY_ROUND_PRIZE_MONEY)(copa, vencedor, perdedor, rodada, partida), // ponytail: a original cai na divisão; que caia nela
    };
    let mut desfecho = read_u8(vencedor + 5);
    if read_i16(partida + 0x30) == 0x136 && desfecho == 1 {
        desfecho = 3;
    }
    let e = round_record(copa, r);
    let mut seguinte = 0u32;
    if r + 1 < read_i16(copa + 0x3c) as i32 {
        let w = core::ptr::read_unaligned((e + 0x6f) as *const u16);
        if w != 0x8c {
            seguinte = (e as u32 & 0xffff_0000) | w as u32; // mov bp, [ebp+0x6f]: só a palavra baixa do ebp muda
        }
    }
    if ao_vencedor != 0 {
        send_prize_news(copa, read_usize(vencedor), ao_vencedor, seguinte, partida, desfecho);
        crate::comparacao::ramo("cqa_1 premio do vencedor");
        european_cup_add_prize_money(european_cup_club_finance(read_usize(vencedor)), ao_vencedor); // cqa1: o prêmio do vencedor
        core::ptr::write_unaligned((copa + 0x28) as *mut i32, read_i32(copa + 0x28).wrapping_sub(ao_vencedor));
    }
    if ao_perdedor != 0 {
        european_cup_add_prize_money(european_cup_club_finance(read_usize(perdedor)), ao_perdedor);
        core::ptr::write_unaligned((copa + 0x28) as *mut i32, read_i32(copa + 0x28).wrapping_sub(ao_perdedor));
    }
}

/// Comparador da 0x522240, em ensaio: a cópia sem a notícia e a original inteira, a partir do mesmo estado (o fundo e
/// os registros de finanças dos dois clubes, restaurados depois do ensaio); confere o fundo e as finanças. Sem clube,
/// sem finanças ou com a divisão que cai: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_pay_round_prize_money(copa: usize, vencedor: usize, perdedor: usize, rodada: u32, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, usize)>(IDX_PAY_ROUND_PRIZE_MONEY);
    let (cv, cp) = (read_usize(vencedor), read_usize(perdedor));
    if cv == 0 || cp == 0 || cv == cp || round_prizes(copa, rodada as i16 as i32, false).is_none() {
        return o(copa, vencedor, perdedor, rodada, partida); // ponytail: sem clube ou com a divisão que cai, só a original
    }
    let (fv, fp) = (european_cup_club_finance(cv), european_cup_club_finance(cp));
    if fv == 0 || fp == 0 {
        return o(copa, vencedor, perdedor, rodada, partida); // ponytail: sem finanças, só a original
    }
    let regioes = [(copa + 0x28, 4), (fv, 0x167), (fp, 0x167)];
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CQA1_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| pay_round_prize_money(copa, vencedor, perdedor, rodada, partida)));
    CQA1_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(copa, vencedor, perdedor, rodada, partida);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("pay_round_prize_money(rodada {}, vencedor {:x}, perdedor {:x}) o fundo e as finanças diferem", rodada as i16, cv, cp));
}

/// Range [start, end) of ties 0x51e870 walks: a single tie or, with -1, all ties of the round (+0x1a).
///
/// [pt] Os confrontos [início, fim) que a 0x51e870 percorre: um só (`laco`; o fim é a palavra baixa de laco + 1, como o
/// `lea edx, [eax+1]` da original) ou, com −1, todos os da rodada (+0x1a).
pub(crate) unsafe fn ties_range(copa: usize, rodada: u32, laco: u32) -> (i16, i16) {
    if laco as u16 != 0xffff {
        (laco as i16, laco.wrapping_add(1) as i16)
    } else {
        (0, read_i16(round_record(copa, rodada as i16 as i32) + 0x1a))
    }
}

/// Swaps home and away in tie t: the word pairs +0/+2, +0xa/+0xc and +0xe/+0x10.
///
/// [pt] Troca mandante e visitante no confronto `t`: os pares de palavras +0/+2, +0xa/+0xc e +0xe/+0x10.
pub(crate) unsafe fn swap_home_away(t: usize) {
    for (a, b) in [(0usize, 2usize), (0xa, 0xc), (0xe, 0x10)] {
        let (x, y) = (read_i16(t + a), read_i16(t + b));
        core::ptr::write_unaligned((t + a) as *mut i16, y);
        core::ptr::write_unaligned((t + b) as *mut i16, x);
    }
}

/// swaps home and away in one tie or in every tie of the round by the round rule: the club in the higher division
/// (virtual +0x90), the team level or the cup stadium
/// Original: `Cup::set_home_teams_for_round` (0x51e870, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x51e870 } (thiscall, ret 8, Cup.cpp): `Cup::set_home_teams_for_round(rodada, laco)`: num confronto (`laco`) ou
/// em todos os da rodada (−1), com os dois times definidos (≠ −1), inverte mandante e visitante pela regra da rodada
/// (byte baixo de +0xb): bit 1, se a virtual +0x90 (club_in_higher_division(clube a, clube b)) devolve o clube b;
/// senão bit 8, se o nível (+4 do time, com sinal) de a é menor que o de b; senão, com o estádio da copa (+0x24) ≠
/// −1, se o clube b existe, tem estádio (+0x69) e o id dele (+0) é o da copa.
pub(crate) unsafe extern "thiscall" fn set_home_teams_for_round(copa: usize, rodada: u32, laco: u32) {
    let r = rodada as i16 as i32;
    let (ini, fim) = ties_range(copa, rodada, laco);
    let regra = read_u8(round_record(copa, r) + 0xb);
    if regra & 1 == 0 && regra & 8 == 0 && read_i32(copa + 0x24) == -1 {
        return;
    }
    let mut k = ini as i32;
    while k < fim as i32 {
        let t = round_tie(copa, r, k);
        let (a, b) = (read_i16(t), read_i16(t + 2));
        k += 1;
        if a == -1 || b == -1 {
            continue;
        }
        let (ta, tb) = (team_entry(copa, a as i32), team_entry(copa, b as i32));
        let inverter = if regra & 1 != 0 {
            let (ca, cb) = (read_usize(ta), read_usize(tb));
            // a virtual +0x90: push cb; push ca; ecx = a copa; sem add esp -> thiscall(copa, ca, cb), ret 8
            let maior: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(read_usize(read_usize(copa) + 0x90));
            let m = maior(copa, ca, cb);
            m != 0 && m == cb
        } else if regra & 8 != 0 {
            (read_u8(ta + 4) as i8) < (read_u8(tb + 4) as i8)
        } else {
            let cb = read_usize(tb);
            cb != 0 && read_usize(cb + 0x69) != 0 && read_i32(read_usize(cb + 0x69)) == read_i32(copa + 0x24) // o estádio da copa
        };
        if inverter {
            swap_home_away(t);
        }
    }
}

/// Comparador da 0x51e870: lado a lado nos confrontos percorridos (a virtual +0x90 só consulta e roda nos dois lados).
pub(crate) unsafe extern "thiscall" fn comparar_set_home_teams_for_round(copa: usize, rodada: u32, laco: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_SET_HOME_TEAMS_FOR_ROUND);
    let (ini, fim) = ties_range(copa, rodada, laco);
    let mut reg = Vec::new();
    if ini < fim {
        reg.push((round_tie(copa, rodada as i16 as i32, ini as i32), (fim as i32 - ini as i32) as usize * 0x35));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada, laco), || set_home_teams_for_round(copa, rodada, laco));
    PLACAR.registrar(igual, || format!("set_home_teams_for_round(rodada {}, laco {}) original={:x?} rust={:x?}", rodada as i16, laco as i16, mo, mr));
}

/// Arguments (p1, p2, p3) of virtual +0x68 that 0x521b30 calls for team t in round r, by its outcome, or None.
///
/// [pt] Os argumentos da virtual +0x68 que a 0x521b30 chama para o time `t` na rodada `r`: (p1, p2, p3) ou None.
pub(crate) unsafe fn team_progress_args(copa: usize, t: i32, r: i32) -> Option<(u8, u8, u8)> {
    let n = read_i16(copa + 0x3c) as i32;
    let d = read_u8(team_entry(copa, t) + 5); // o desfecho
    let byte_da_rodada = |k: i32, desl: usize| read_u8(round_record(copa, k) + desl);
    if n > 0 && read_i16(round_record(copa, n - 1) + 7) == 0x8c {
        // a última rodada é a do terceiro lugar
        if r == n - 2 {
            return match d {
                1 => Some((1, 1, 1)),
                2 => Some((2, 2, 2)),
                _ => None,
            };
        }
        if r == n - 1 {
            return match d {
                3 => Some((3, 3, 3)),
                4 => Some((4, 4, 4)),
                _ => None,
            };
        }
        if d != 0 {
            let v = byte_da_rodada(r, 0x5b);
            return Some((v, v, v));
        }
        if r + 1 == n - 2 {
            return Some((4, 1, 4));
        }
        let v = byte_da_rodada(r + 1, 0x5b);
        return Some((v, 1, v));
    }
    if r == n - 1 {
        return match d {
            1 => Some((1, 1, 1)),
            2 => {
                let v = byte_da_rodada(r, 0x1a).wrapping_add(1);
                Some((v, v, v))
            }
            _ => None,
        };
    }
    if d != 0 {
        let v = byte_da_rodada(r, 0x5b);
        return Some((v, v, v));
    }
    if r + 1 == n - 1 {
        let v = byte_da_rodada(r + 1, 0x1a).wrapping_add(1);
        return Some((v, 1, v));
    }
    let v = byte_da_rodada(r + 1, 0x5b);
    Some((v, 1, v))
}

/// how far the team went in the cup, by outcome and round (with or without the third place round), handed to virtual
/// +0x68 (set_club_progress) with competition +0x44
/// Original: `Cup::record_team_progress` (0x521b30, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x521b30 } (thiscall, ret 8, Cup.cpp): `Cup::record_team_progress(time, rodada)`: quanto o time foi na copa,
/// pela virtual +0x68 (`set_club_progress(clube, competição +0x44, p1, p2, p3)`). Com a última rodada do tipo 0x8c
/// (terceiro lugar): na semifinal (rodadas − 2), desfecho 1 ou 2 → (d, d, d); na última, 3 ou 4 → (d, d, d); noutra
/// rodada, desfecho ≠ 0 → o +0x5b da rodada três vezes; 0 → (4, 1, 4) antes da semifinal, senão (x, 1, x) com o +0x5b
/// da rodada seguinte. Sem ela: na final, 1 → (1, 1, 1), 2 → os confrontos (+0x1a) da final + 1 três vezes; noutra,
/// desfecho ≠ 0 → o +0x5b três vezes; 0 → (x, 1, x) com os confrontos + 1 da final antes dela, senão o +0x5b da
/// seguinte. Os desfechos fora desses não chamam nada.
pub(crate) unsafe extern "thiscall" fn record_team_progress(copa: usize, time: u32, rodada: u32) {
    let (t, r) = (time as i16 as i32, rodada as i16 as i32);
    if let Some((p1, p2, p3)) = team_progress_args(copa, t, r) {
        // a virtual +0x68: 5 pushes; ecx = a copa; sem add esp -> thiscall(copa, clube, comp, p1, p2, p3), ret 0x14
        let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(read_usize(read_usize(copa) + 0x68));
        // ponytail: a original empurra bytes sobre registradores inteiros (o ebx de quem chamou, o dword do argumento
        // `time` com o byte baixo trocado, 13 × rodada): os bits altos são sobra; as 70 versões da +0x68 só leem o byte
        // baixo de cada um (ou repassam os dwords a outra que lê): aqui, os bytes limpos.
        f(copa, read_usize(team_entry(copa, t)), read_u8(copa + 0x44) as u32, p1 as u32, p2 as u32, p3 as u32);
    }
}

/// Comparador da 0x521b30: lado a lado nos 3 bytes de progresso (+0x73..+0x75) do registro do clube no Comp_Stats da
/// copa (+8, achado pela 0x4a4850: push clube; ecx = as estatísticas; thiscall, ret 4), que a +0x68 grava; as versões
/// que repassam a outra competição gravam fora da região (aí a comparação só confere que nada mudou nela). Sem
/// estatísticas, sem clube ou sem registro: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_record_team_progress(copa: usize, time: u32, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32)>(IDX_RECORD_TEAM_PROGRESS);
    let stats = read_usize(copa + 8);
    let clube = read_usize(team_entry(copa, time as i16 as i32));
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let reg = if stats != 0 && clube != 0 { busca(stats, clube) } else { 0 };
    if reg == 0 {
        return o(copa, time, rodada); // ponytail: sem registro para olhar, só a original
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(reg + 0x73, 3)], || o(copa, time, rodada), || record_team_progress(copa, time, rodada));
    PLACAR.registrar(igual, || format!("record_team_progress(time {}, rodada {}) original={:x?} rust={:x?}", time as i16, rodada as i16, mo, mr));
}

// ---------- 263a leva: o sorteio da rodada da copa, o nome da fase e da rodada e o clube da divisão mais alta (Cup.cpp, comp_util.cpp) ----------




/// Address of the 0x68-byte record of round r (+0xa3), r used as the full dword like the original.
///
/// [pt] O registro de 0x68 bytes da rodada `r` (+0xa3); `r` como a original o usa (o dword inteiro).
pub(crate) unsafe fn round_record_by_dword(copa: usize, r: u32) -> usize {
    read_usize(copa + 0xa3).wrapping_add((r as usize).wrapping_mul(0x68))
}

/// Start of round r's tie array (0x35-byte ties, per-round pointers at cup +0xad).
///
/// [pt] Os confrontos da rodada `r` (+0xad: um vetor de confrontos de 0x35 bytes por rodada).
pub(crate) unsafe fn round_ties_array(copa: usize, r: u32) -> usize {
    read_usize(read_usize(copa + 0xad).wrapping_add((r as usize).wrapping_mul(4)))
}

/// True when some defined team of a round's ties still has no club (an earlier tie undecided): the round cannot be
/// drawn.
///
/// [pt] Algum time definido (≠ −1) de algum confronto da rodada ainda sem clube (o vencedor de um confronto anterior em
/// aberto): a rodada não pode ser sorteada.
pub(crate) unsafe fn round_has_open_team(copa: usize, r: u32) -> bool {
    let reg = round_record_by_dword(copa, r);
    let mut t = round_ties_array(copa, r);
    let mut k = 0i32;
    while k < read_i16(reg + 0x1a) as i32 {
        for lado in [0usize, 2] {
            let x = read_i16(t + lado);
            if x != -1 && read_usize(read_usize(copa + 0xa7).wrapping_add((x as isize * 6) as usize)) == 0 {
                return true;
            }
        }
        t += 0x35;
        k += 1;
    }
    false
}

/// Builds the round's draw date (day +0, year byte +2 relative to cup +0x40) into a local Date.
///
/// [pt] A data do sorteio da rodada (+0 o dia, +2 o ano em byte a partir de +0x40 da copa), numa Data local (0x549ef0:
/// push ano; push dia; ecx = a local; sem add esp -> thiscall, ret 8; os dois em registradores com lixo nos bits altos,
/// a Rust só lê as palavras).
pub(crate) unsafe fn round_draw_date(copa: usize, reg: usize, d: *mut [u8; 8]) {
    let nova: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let ano = (read_u8(reg + 2) as i8 as i16).wrapping_add(read_i16(copa + 0x40));
    nova(d, read_i16(reg) as u16 as u32, ano as u16 as u32);
}

/// Stores date d as the round's draw date: the day word and the year byte minus the low byte of cup +0x40.
///
/// [pt] Grava a data `d` como a do sorteio da rodada: a palavra do dia e o byte do ano menos o byte baixo de +0x40.
pub(crate) unsafe fn set_round_draw_date(copa: usize, reg: usize, d: &[u8; 8]) {
    core::ptr::write_unaligned(reg as *mut [u8; 2], [d[0], d[1]]);
    *((reg + 2) as *mut u8) = d[2].wrapping_sub(read_u8(copa + 0x40));
}

/// True when date a is after date b (unsigned year, then signed day), as 0x51c410 compares.
///
/// [pt] A data `a` vem depois da `b` (o ano sem sinal, depois o dia com sinal), como a comparação da 0x51c410.
pub(crate) fn cup_date_is_after(a: &[u8; 8], b: &[u8; 8]) -> bool {
    let (aa, ab) = (u16::from_le_bytes([a[2], a[3]]), u16::from_le_bytes([b[2], b[3]]));
    aa > ab || (aa == ab && i16::from_le_bytes([a[0], a[1]]) > i16::from_le_bytes([b[0], b[1]]))
}

/// True when the competition gets no draw news: fewer than 2 ties, rule 0x200, national-team finals competition or
/// friendly.
///
/// [pt] A competição não leva a notícia do sorteio: menos de 2 confrontos, regra 0x200 (+0xd), competição especial das
/// seleções (0x76d940 em [0xb74318]: push comp; ecx = as seleções; sem add esp -> ret 4; a cópia Rust é stdcall(comp))
/// ou amistosa (0x5c6710 em [0xae2c74]: push comp; ecx = o Friendly -> thiscall, ret 4).
pub(crate) unsafe fn skips_draw_news(copa: usize, r: u32) -> bool {
    let reg = round_record_by_dword(copa, r);
    if read_i16(reg + 0x1a) < 2 || read_i16(reg + 0xd) & 0x200 != 0 {
        return true;
    }
    if read_usize(G_NATIONAL_TEAMS) != 0 {
        let especial: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
        if especial(read_usize(copa + 4)) != 0 {
            return true;
        }
    }
    if read_usize(G_FRIENDLY) != 0 {
        let amistosa: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
        if amistosa(read_usize(G_FRIENDLY), read_usize(copa + 4)) != 0 {
            return true;
        }
    }
    false
}

/// with some team still open it puts the draw off by 1 day (and the following rounds to at least that date); otherwise
/// draws by the rule (seeded 0x51d2d0, nations apart 0x51d9f0 or open 0x51e220), sets the home teams (0x51e870) and the
/// shared stadium (0x522610), marks the drawn ties and sends the draw news to the human managed teams; returns 1
/// Original: `Cup::draw_round` (0x51c410, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x51c410 } (thiscall, ret 4, Cup.cpp, quadro SEH sem destrutor no caminho normal): `Cup::draw_round(rodada)`;
/// devolve 1. Sem +0x45 ou com a rodada já sorteada (+0x34 do primeiro confronto): nada. Com um time de algum confronto
/// ainda sem clube: a data do sorteio anda 1 dia (0x54c5b0, 0x417c10) e as das rodadas seguintes passam a ser, no
/// mínimo, essa. Senão sorteia: regra 0x400 (+0xd) pela semente (0x51d2d0), regra 4 (+0xb) separando as nações
/// (0x51d9f0), sem 0x200 o sorteio livre (0x51e220); os mandantes (0x51e870 com −1); estádio compartilhado (+0x21 = 2,
/// 0x522610); marca os confrontos sorteados (+0x34 = 1). Com notícia (skips_draw_news): rodada que não é a do
/// terceiro lugar (+7 ≠ 0x8c) soma 1 em +0xac; a notícia 0x27 (a competição, +0x3a, o tipo, +0x44, a rodada, 0, e 1
/// se +0xab + 1 < +0xac) vai a cada um dos times ainda vivos (desfecho 0) entre os primeiros (soma das entradas +0x1c
/// das rodadas até esta) que é de técnico humano, com o id dele no dado 3.
pub(crate) unsafe extern "thiscall" fn draw_round(copa: usize, rodada: u32) -> u32 {
    if read_usize(copa + 0x45) == 0 || read_u8(round_ties_array(copa, rodada) + 0x34) != 0 {
        return 1;
    }
    let reg = round_record_by_dword(copa, rodada);
    if round_has_open_team(copa, rodada) {
        let mais: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> *mut [u8; 8] = core::mem::transmute(FN_DATE_ADD_DAYS);
        let copia: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], *mut [u8; 8]) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
        let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        round_draw_date(copa, reg, &mut a);
        crate::comparacao::ramo("cqa_3 sorteio adiado");
        let r = mais(&mut a, &mut b, 1); // cqa3: o sorteio adiado
        copia(&mut a, &mut c, r); // push &b; push &c; ecx = &a -> thiscall, ret 8
        set_round_draw_date(copa, reg, &a);
        let mut k = rodada.wrapping_add(1);
        while (k as i16) < read_i16(copa + 0x3c) {
            let rk = round_record_by_dword(copa, k as i16 as i32 as u32);
            round_draw_date(copa, rk, &mut b);
            if cup_date_is_after(&a, &b) {
                set_round_draw_date(copa, rk, &a);
            }
            k = k.wrapping_add(1);
        }
        return 1;
    }
    // os sorteios: push rodada; ecx = a copa; sem add esp -> thiscall, ret 4 (as três)
    let sorteio = |f: usize| {
        let g: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
        g(copa, rodada)
    };
    let regra = read_i16(reg + 0xd);
    if regra & 0x400 != 0 {
        crate::comparacao::ramo("cqa_3 sorteio pela semente");
        sorteio(0x51d2d0); // cqa3: pela semente
    } else if read_u8(reg + 0xb) & 4 != 0 {
        sorteio(0x51d9f0);
    } else if regra & 0x200 == 0 {
        crate::comparacao::ramo("cqa_3 sorteio livre");
        sorteio(0x51e220); // cqa3: o sorteio livre
    }
    // push −1; push rodada; ecx = a copa -> thiscall, ret 8
    let mandantes: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_CUP_SET_HOME_TEAMS_FOR_ROUND);
    mandantes(copa, rodada, u32::MAX);
    if read_u8(round_record_by_dword(copa, rodada) + 0x21) == 2 {
        sorteio(0x522610);
    }
    let mut k = 0i32;
    while k < read_i16(round_record_by_dword(copa, rodada) + 0x1a) as i32 {
        *((round_ties_array(copa, rodada) + k as usize * 0x35 + 0x34) as *mut u8) = 1;
        k += 1;
    }
    if skips_draw_news(copa, rodada) {
        return 1;
    }
    let reg = round_record_by_dword(copa, rodada);
    if read_i16(reg + 7) != 0x8c {
        *((copa + 0xac) as *mut u8) = read_u8(copa + 0xac).wrapping_add(1);
    }
    let mut soma: i16 = 0;
    if (rodada as i32) >= 0 {
        for i in 0..=rodada {
            soma = soma.wrapping_add(read_i16(round_record_by_dword(copa, i) + 0x1c));
        }
    }
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
    construir(n, 0x27, 0); // push 0; push 0x27; ecx = a local -> thiscall, ret 8
    gravar(n, 0, read_usize(read_usize(copa + 4)) as u32); // push valor; push k; ecx = a local -> thiscall, ret 8
    gravar(n, 1, read_i16(copa + 0x3a) as i32 as u32);
    gravar(n, 2, read_i16(round_record_by_dword(copa, rodada) + 7) as i32 as u32);
    gravar(n, 4, read_u8(copa + 0x44) as i8 as i32 as u32);
    gravar(n, 5, rodada);
    gravar(n, 6, 0);
    gravar(n, 7, ((read_u8(copa + 0xab) as i8 as i32 + 1) < read_u8(copa + 0xac) as i8 as i32) as u32);
    for i in 0..soma.max(0) as usize {
        let e = read_usize(copa + 0xa7) + 6 * i;
        if read_u8(e + 5) != 0 || read_usize(e) == 0 {
            continue;
        }
        // 0x7875e0: push clube; ecx = [0xb7431c] -> ret 4 (a cópia Rust é stdcall(clube))
        if humano(read_usize(e)) == 0 {
            continue;
        }
        gravar(n, 3, read_usize(read_usize(read_usize(copa + 0xa7) + 6 * i)) as u32);
        // push clube; push &local; ecx = [0xb7431c]; sem add esp -> thiscall, ret 8
        entregar(read_usize(G_NEWS), n, read_usize(read_usize(copa + 0xa7) + 6 * i));
    }
    1
}

/// Comparador da 0x51c410: só a original (os sorteios sorteiam, a notícia aloca). O plano, feito antes: sem sorteio
/// (sem +0x45 ou já sorteada), nada muda nos confrontos nem nas datas; com time em aberto, as datas previstas das
/// rodadas (a desta mais 1 dia, as seguintes no mínimo essa); sorteada, todos os confrontos com +0x34 = 1 e +0xac com
/// a soma prevista (as consultas da notícia não mudam com o sorteio).
pub(crate) unsafe extern "thiscall" fn comparar_draw_round(copa: usize, rodada: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_DRAW_ROUND);
    if read_usize(copa + 0x45) == 0 || read_u8(round_ties_array(copa, rodada) + 0x34) != 0 {
        return o(copa, rodada); // ponytail: nada a sortear, a original não faz nada
    }
    let n = read_i16(copa + 0x3c);
    if round_has_open_team(copa, rodada) {
        // o plano das datas: (registro, palavra do dia, byte do ano) de cada rodada daqui em diante
        let mut a = [0u8; 8];
        let (mut dia, mut ano) = (read_i16(round_record_by_dword(copa, rodada)), (read_u8(round_record_by_dword(copa, rodada) + 2) as i8 as i16).wrapping_add(read_i16(copa + 0x40)) as u16);
        // a 0x54c5b0 (mais 1): normalizar para a frente, 365 dias no bissexto (crate::data::bissexto)
        dia = dia.wrapping_add(1);
        let tam = |y: u16| if crate::data::bissexto(y) == 1 { 365i16 } else { 364 };
        while dia > tam(ano) {
            dia = dia.wrapping_sub(tam(ano) + 1);
            ano = ano.wrapping_add(1);
        }
        a[0..2].copy_from_slice(&dia.to_le_bytes());
        a[2..4].copy_from_slice(&ano.to_le_bytes());
        let mut plano = vec![(round_record_by_dword(copa, rodada), dia, a[2].wrapping_sub(read_u8(copa + 0x40)))];
        let mut k = rodada.wrapping_add(1);
        while (k as i16) < n {
            let rk = round_record_by_dword(copa, k as i16 as i32 as u32);
            let mut b = [0u8; 8];
            let anob = (read_u8(rk + 2) as i8 as i16).wrapping_add(read_i16(copa + 0x40)) as u16;
            b[0..2].copy_from_slice(&read_i16(rk).to_le_bytes());
            b[2..4].copy_from_slice(&anob.to_le_bytes());
            plano.push(if cup_date_is_after(&a, &b) { (rk, dia, a[2].wrapping_sub(read_u8(copa + 0x40))) } else { (rk, read_i16(rk), read_u8(rk + 2)) });
            k = k.wrapping_add(1);
        }
        let r = o(copa, rodada);
        let ok = plano.iter().all(|&(rk, d, y)| read_i16(rk) == d && read_u8(rk + 2) == y);
        PLACAR.registrar(ok, || format!("draw_round(rodada {}) as datas adiadas diferem do plano", rodada as i32));
        return r;
    }
    let antes_ac = read_u8(copa + 0xac);
    let r = o(copa, rodada);
    let mut ok = r == 1;
    let reg = round_record_by_dword(copa, rodada);
    for k in 0..read_i16(reg + 0x1a).max(0) as usize {
        ok &= read_u8(round_ties_array(copa, rodada) + k * 0x35 + 0x34) == 1;
    }
    let soma = if !skips_draw_news(copa, rodada) && read_i16(reg + 7) != 0x8c { 1 } else { 0 };
    ok &= read_u8(copa + 0xac) == antes_ac.wrapping_add(soma);
    PLACAR.registrar(ok, || format!("draw_round(rodada {}) marcas dos confrontos ou +0xac ({} -> {}) diferem do plano", rodada as i32, antes_ac, read_u8(copa + 0xac)));
    r
}



/// Club's division level (word +0x69 of its division +0x57), or 0 without division.
///
/// [pt] O nível da divisão do clube: a palavra +0x69 da divisão (+0x57), ou 0 sem divisão.
pub(crate) unsafe fn division_level(clube: usize) -> i16 {
    let d = read_usize(clube + club::DIVISION);
    if d != 0 {
        read_i16(d + 0x69)
    } else {
        0
    }
}

/// returns the club whose division level (+0x57 -> +0x69) is lower, or 0 when level
/// Original: `Cup::club_in_higher_division` (0x522360, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x522360 } (thiscall, ret 8, Cup.cpp; o this não é lido; a virtual +0x90 da copa base):
/// `Cup::club_in_higher_division(a, b)`: o clube de nível menor (+0x57 → +0x69, com sinal; 0 sem divisão); empate, 0.
pub(crate) unsafe extern "thiscall" fn club_in_higher_division(_this: usize, a: usize, b: usize) -> usize {
    let (na, nb) = (division_level(a), division_level(b));
    if na < nb {
        a
    } else if na == nb {
        0
    } else {
        b
    }
}

/// Comparador da 0x522360: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_club_in_higher_division(this: usize, a: usize, b: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(IDX_CLUB_IN_HIGHER_DIVISION)(this, a, b);
    let n = club_in_higher_division(this, a, b);
    PLACAR.registrar(r == n, || format!("club_in_higher_division({:x}, {:x}) original={:x} rust={:x}", a, b, r, n));
    r
}

// ---------- 263a leva: a copa pelo desfecho e no fim da temporada: os registros pelo desfecho, as honras da temporada e os semifinalistas para a UEFA (Cup.cpp) ----------






/// Club of cup team t (+0xa7, 6 bytes per team, +0 club).
///
/// [pt] O clube do time `t` (+0xa7, 6 bytes por time, +0 o clube).
pub(crate) unsafe fn club_of_team(copa: usize, t: i32) -> usize {
    read_usize(read_usize(copa + 0xa7).wrapping_add((t as isize * 6) as usize))
}

/// Address of the club's 6-byte state record in G_CLUB_STATE_TABLE (+0 = season's European competition).
///
/// [pt] O registro de 6 bytes do estado do clube ([0xae2d14] + id × 6; +0 a competição europeia da temporada).
pub(crate) unsafe fn club_state_record(clube: usize) -> usize {
    read_usize(G_CLUB_STATE_TABLE).wrapping_add((read_i32(clube) as isize * 6) as usize)
}

/// True when the competition is the friendly one (Friendly::is_friendly_competition on G_FRIENDLY).
///
/// [pt] A competição é a amistosa (0x5c6710: push comp; ecx = [0xae2c74]; sem add esp -> thiscall, ret 4).
pub(crate) unsafe fn cup_is_friendly_comp(comp: usize) -> bool {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    f(read_usize(G_FRIENDLY), comp) != 0
}

/// Key-table index of the nation that owns the competition (0x669470, cdecl).
///
/// [pt] O índice da nação dona da competição na tabela das chaves (0x669470: push comp; add esp, 4 -> cdecl; devolve o byte).
pub(crate) unsafe fn cup_key_nation_index_of_comp(comp: usize) -> u8 {
    let f: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_COMP);
    f(comp)
}

/// True when the competition is its nation's stored main cup ([0xb63d7e + 72 × index]).
///
/// [pt] A competição é a copa principal guardada da nação dela ([0xb63d7e + 72 × índice]).
pub(crate) unsafe fn is_main_national_cup(comp: usize) -> bool {
    read_usize(0xb63d7e_usize.wrapping_add((cup_key_nation_index_of_comp(comp) as i8 as isize * 72) as usize)) == comp
}

/// Whether the cup winner earns a UEFA Cup place: always the English League Cup, else a European nation's main
/// non-friendly cup with the club not yet in Europe.
///
/// [pt] A vaga na Copa da UEFA que o campeão da copa (`cup_update_staff_records_for_status`, desfecho 1) ganha: a Copa da Liga
/// inglesa ([0x9cf750]) sempre; senão, a copa principal de uma nação europeia ([0x9cfa10] o continente, pela nação
/// +0x5d e o continente +0x71), não amistosa, com o clube ainda sem competição europeia (−1) e a nação com índice.
pub(crate) unsafe fn wins_uefa_cup_place(comp: usize, clube: usize) -> bool {
    if read_i32(comp) == read_i32(KEY_CLUB_COMP_ENGLISH_LEAGUE_CUP) {
        return true;
    }
    let nacao = read_usize(comp + competition::CLUB_COMP_NATION);
    if nacao == 0 {
        return false;
    }
    let continente = read_usize(nacao + nation::CONTINENT);
    if continente == 0 || read_i32(continente) != read_i32(KEY_CONTINENT_EUROPE) || cup_is_friendly_comp(comp) {
        return false;
    }
    if read_i32(club_state_record(clube)) != -1 || cup_key_nation_index_of_comp(comp) == 0xff {
        return false;
    }
    is_main_national_cup(comp)
}

/// Calls a staff-records final routine f (winner/runner-up/third) on G_STAFF_RECORDS with club, match and final.
///
/// [pt] Os registros de final (0x86a800 campeão, 0x86b0b0 vice, 0x86b710 terceiro: push final; push jogo; push clube;
/// ecx = [0xdd7f00]; sem add esp -> thiscall(registros, clube, jogo, final), ret 0xc).
pub(crate) unsafe fn cup_record_final_result(f: usize, clube: usize, jogo: usize, final_: u32) {
    let g: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(f);
    g(read_usize(G_STAFF_RECORDS), clube, jogo, final_);
}

/// by the club outcome: the winners record (and the UEFA Cup place through the English League Cup or a European nation
/// main cup), runners-up, third place, qualified for the next round or knocked out, weighted by round; returns 0
/// Original: `Cup::update_staff_records_for_status` (0x519890, Cup.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x519890 } (thiscall, ret 0x18, Cup.cpp; a virtual +0x44 das copas inglesas e da base):
/// `Cup::update_staff_records_for_status(clube, desfecho, _, _, jogo, final)`; devolve 0. Nada com o jogo +0x3a = 1,
/// com +0x2c ou com +0x44 ≠ 0xff. Desfecho 1: o registro de campeão e, com a vaga (wins_uefa_cup_place), o estado do
/// clube recebe a Copa da UEFA ([0x9cf6f0]); 2: o de vice; 3: o de terceiro; 0: se a rodada seguinte (+0x34 + 1) existe
/// e não é a do terceiro lugar (+7 ≠ 0x8c), a classificação (0x86bdd0) com o peso rodada × 99 / (rodadas − 1) + 1;
/// 0xff: a eliminação (0x86c000) com o peso rodada × 99 / rodadas + 1. Os registros de fase levam os dwords que a
/// original empurra: a palavra do jogo (+0x32, +0x30) ou da rodada (+7) sobre os 16 bits altos do registrador.
pub(crate) unsafe extern "thiscall" fn cup_update_staff_records_for_status(copa: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, final_: u32) -> u32 {
    if read_u8(jogo + 0x3a) == 1 || read_usize(copa + 0x2c) != 0 || read_u8(copa + 0x44) != 0xff {
        return 0;
    }
    let comp = read_usize(copa + 4);
    match desfecho as u8 {
        1 => {
            crate::comparacao::ramo("cqa_4 campeao");
            cup_record_final_result(0x86a800, clube, jogo, final_); // cqa4: o registro de campeão
            if wins_uefa_cup_place(comp, clube) {
                core::ptr::write_unaligned(club_state_record(clube) as *mut i32, read_i32(KEY_CLUB_COMP_UEFA_CUP));
            }
        }
        2 => {
            crate::comparacao::ramo("cqa_4 vice");
            cup_record_final_result(0x86b0b0, clube, jogo, final_); // cqa4: o registro de vice
        }
        3 => cup_record_final_result(0x86b710, clube, jogo, final_),
        0 => {
            let r = read_i16(jogo + 0x34);
            let n = read_i16(copa + 0x3c);
            let seguinte = r.wrapping_add(1);
            if seguinte >= n {
                return 0;
            }
            let w7 = core::ptr::read_unaligned((round_record(copa, seguinte as i32) + 7) as *const u16);
            if w7 == 0x8c {
                return 0;
            }
            let div = n as i32 - 1;
            if div == 0 {
                return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: a original cai na divisão
            }
            let q = (r as i32 * 99) / div;
            let peso = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32; // inc al
            let fase = (jogo as u32 & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32; // mov cx, [ecx+0x32]
            let rodada = (read_usize(copa + 0xa3) as u32 & 0xffff_0000) | w7 as u32; // mov di, [edi+...+7]
            // push peso; push rodada; push fase; push comp; push clube; ecx = [0xdd7f00] -> thiscall, ret 0x14
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_QUALIFIED_FOR_ROUND_RECORD);
            f(read_usize(G_STAFF_RECORDS), clube, comp, fase, rodada, peso);
        }
        0xff => {
            let r = read_i16(jogo + 0x34) as i32;
            let n = read_i16(copa + 0x3c) as i32;
            if n == 0 {
                return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS)(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: a original cai na divisão
            }
            let (q, resto) = ((r * 99) / n, (r * 99) % n);
            let peso = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32;
            let fase = (peso & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32; // mov ax, [ecx+0x32]
            let rodada = (resto as u32 & 0xffff_0000) | core::ptr::read_unaligned((jogo + 0x30) as *const u16) as u32; // mov dx, [ecx+0x30]
            // push peso; push rodada; push fase; push comp; push clube; ecx = [0xdd7f00] -> thiscall, ret 0x14
            let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(FN_STAFF_RECORDS_ADD_KNOCKED_OUT_RECORD);
            f(read_usize(G_STAFF_RECORDS), clube, comp, fase, rodada, peso);
        }
        _ => {}
    }
    0
}

/// Comparador da 0x519890: só a original (os registros vão ao histórico das pessoas e mexem nas reputações e no moral,
/// não podem rodar duas vezes). O plano, feito antes: o estado do clube (+0 do registro de 6 bytes) passa à Copa da
/// UEFA quando o campeão ganha a vaga; senão não muda. Sem clube: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_cup_update_staff_records_for_status(copa: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, final_: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CUP_UPDATE_STAFF_RECORDS_FOR_STATUS);
    if clube == 0 || jogo == 0 {
        return o(copa, clube, desfecho, a3, a4, jogo, final_); // ponytail: sem clube ou jogo, só a original
    }
    let estado = club_state_record(clube);
    let antes = read_i32(estado);
    let previsto = if read_u8(jogo + 0x3a) != 1 && read_usize(copa + 0x2c) == 0 && read_u8(copa + 0x44) == 0xff && desfecho as u8 == 1
        && wins_uefa_cup_place(read_usize(copa + 4), clube) { read_i32(KEY_CLUB_COMP_UEFA_CUP) } else { antes };
    let r = o(copa, clube, desfecho, a3, a4, jogo, final_);
    let depois = read_i32(estado);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("cup_update_staff_records_for_status(desfecho {}) estado {:x} -> {:x}, previsto {:x}, devolveu {}", desfecho as u8, antes, depois, previsto, r));
    r
}

/// History line of the cup: (winner, runner-up, third, host national team), with the Welsh cup and host-country special
/// cases.
///
/// [pt] Os quatro da linha do histórico: (campeão, vice, terceiro, seleção anfitriã). O campeão, o vice e o terceiro pelo
/// desfecho dos times (1, 2, 3; até achar os três); na copa galesa ([0x9cf958]) o terceiro é o primeiro time com o
/// progresso +0x73 = 3 no Comp_Stats (0x4a4850 de +8 do objeto da competição em [0xadadfc]: push clube; ecx = as
/// estatísticas -> thiscall, ret 4). A anfitriã: com a virtual +0x88 do objeto da competição (ecx = o objeto; sem push
/// -> thiscall, ret 0) e as anfitriãs do ano (+0x40) da HostCountry ([0xb5e134], 0x5fa730: push 0; push &b; push &a;
/// push ano; push id; ecx = [0xb5e134] -> thiscall, ret 0x14): duas, a seleção principal (0x53b3d0: push nação; add
/// esp, 4 -> cdecl) da de +0x8e maior (empate, a primeira); uma, a da primeira; nenhuma, 0.
pub(crate) unsafe fn season_honours(copa: usize) -> (usize, usize, usize, usize) {
    let n = read_i16(copa + 0x3e);
    let (mut campeao, mut vice, mut terceiro) = (0usize, 0usize, 0usize);
    let mut i = 0i16;
    while i < n {
        let e = read_usize(copa + 0xa7).wrapping_add((i as isize * 6) as usize);
        match read_u8(e + 5) {
            1 => campeao = read_usize(e),
            2 => vice = read_usize(e),
            3 => terceiro = read_usize(e),
            _ => {}
        }
        if campeao != 0 && vice != 0 && terceiro != 0 {
            break;
        }
        i += 1;
    }
    let comp = read_usize(copa + 4);
    let objeto = || read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_i32(read_usize(copa + 4)) as isize * 4) as usize));
    if comp != 0 && read_i32(comp) == read_i32(KEY_CLUB_COMP_WELSH_PREMIER_CUP) {
        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
        let mut j = 0i16;
        while j < read_i16(copa + 0x3e) {
            let c = club_of_team(copa, j as i32);
            if read_u8(busca(read_usize(objeto() + 8), c) + 0x73) == 3 {
                terceiro = c;
                break;
            }
            j += 1;
        }
    }
    let o = objeto();
    let tem: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(read_usize(read_usize(o) + 0x88));
    let mut anfitria = 0usize;
    if tem(o) != 0 {
        let (mut a, mut b) = (0i32, 0i32);
        let achar: unsafe extern "thiscall" fn(usize, i32, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(FN_HOSTCOUNTRY_FIND_HOSTS);
        // ponytail: o ano vai num registrador com lixo nos 16 bits altos; a 0x5fa730 recebe um short
        let k = achar(read_usize(G_HOST_COUNTRY), read_i32(read_usize(copa + 4)), read_i16(copa + 0x40) as u16 as u32, &mut a, &mut b, 0);
        let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
        let nacao = |x: i32| read_usize(G_NATION_TABLE).wrapping_add((x as isize * 0x122) as usize);
        if k == 2 {
            let (na, nb) = (nacao(a), nacao(b));
            anfitria = if read_i16(na + 0x8e) < read_i16(nb + 0x8e) { selecao(nb) } else { selecao(na) };
        } else if k != 0 {
            anfitria = selecao(nacao(a));
        }
    }
    (campeao, vice, terceiro, anfitria)
}

/// finds the winner, runner-up, third (in the Welsh cup, through the statistics) and the host national team, writes the
/// history row and, in the nation main cup, passes the semi-final losers to UEFA
/// Original: `Cup::record_season_honours` (0x519a90, Cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x519a90 } (thiscall, ret 0, Cup.cpp; a virtual +0x30 das copas inglesas e da base): `Cup::record_season_honours()`:
/// a linha do histórico (0x4afce0: 5 pushes; add esp, 0x14 -> cdecl(copa, campeão, vice, terceiro, anfitriã)) com os
/// quatro de season_honours; fora da competição amistosa, na copa principal da nação (is_main_national_cup), os
/// semifinalistas para a UEFA (0x519fc0: ecx = a copa -> thiscall, ret 0).
pub(crate) unsafe extern "thiscall" fn cup_record_season_honours(copa: usize) {
    let (campeao, vice, terceiro, anfitria) = season_honours(copa);
    let historico: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_COMP_UTIL_ADD_HISTORY_RECORD);
    historico(copa, campeao, vice, terceiro, anfitria);
    let comp = read_usize(copa + 4);
    if !cup_is_friendly_comp(comp) && is_main_national_cup(comp) {
        let semi: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_CUP_REGISTER_SEMI_FINAL_LOSERS_FOR_UEFA);
        semi(copa);
    }
}

/// Comparador da 0x519a90: só a original (a 0x4afce0 pode crescer o histórico e a 0x519fc0 tem o seu comparador). O
/// plano, feito antes: a linha livre do histórico (free_comp_history_row) recebe a competição (+4), o ano [0xae2c92] e os
/// quatro de season_honours. Sem linha livre (o histórico cresce) ou sem a competição: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_cup_record_season_honours(copa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_RECORD_SEASON_HONOURS);
    let linha = free_comp_history_row();
    if linha == 0 || read_usize(copa + 4) == 0 {
        return o(copa); // ponytail: o histórico cresce (realloc) ou não há competição: só a original
    }
    let (campeao, vice, terceiro, anfitria) = season_honours(copa);
    let ano = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16);
    o(copa);
    let lido = (read_usize(linha + 4), core::ptr::read_unaligned((linha + 8) as *const u16), read_usize(linha + 0xa), read_usize(linha + 0xe), read_usize(linha + 0x12), read_usize(linha + 0x16));
    let previsto = (read_usize(copa + 4), ano, campeao, vice, terceiro, anfitria);
    PLACAR.registrar(lido == previsto, || format!("cup_record_season_honours: linha {:x?} prevista {:x?}", lido, previsto));
}

/// The two club ids 0x519fc0 sends to UEFA seeding (semi-final losers not in the final), or None when it sends nothing.
///
/// [pt] Os dois clubes (ids) que a 0x519fc0 manda à semeadura da UEFA, ou None quando ela sai sem mandar: a rodada 0x82
/// (as semifinais) e a 0x96 (a final), as duas achadas; percorre os confrontos da 0x82 e os times que não estão no
/// primeiro confronto da 0x96: o primeiro vira a, o segundo b (aí para); um time −1 (ainda sem confronto) cancela;
/// faltando, −1.
pub(crate) unsafe fn semi_final_losers(copa: usize) -> Option<(i32, i32)> {
    let (mut semi, mut final_) = (-1i16, -1i16);
    let mut i = 0i16;
    while i < read_i16(copa + 0x3c) {
        let t = read_i16(round_record(copa, i as i32) + 7);
        if t == 0x82 {
            semi = i;
        } else if t == 0x96 {
            final_ = i;
        }
        if semi != -1 && final_ != -1 {
            break;
        }
        i += 1;
    }
    if semi == -1 || final_ == -1 {
        return None;
    }
    let confrontos = |r: i16| read_usize(read_usize(copa + 0xad).wrapping_add((r as isize * 4) as usize));
    let f0 = confrontos(final_);
    let (fa, fb) = (read_i16(f0), read_i16(f0 + 2));
    let n = read_i16(round_record(copa, semi as i32) + 0x1a);
    let base = confrontos(semi);
    let (mut a, mut b) = (-1i32, -1i32);
    let mut k = 0i16;
    'lacos: while k < n {
        let t = base.wrapping_add((k as isize * 0x35) as usize);
        for lado in [0usize, 2] {
            let x = read_i16(t + lado);
            if x == fa || x == fb {
                continue;
            }
            if x == -1 {
                return None;
            }
            let id = read_i32(club_of_team(copa, x as i32));
            if a == -1 {
                a = id;
            } else {
                b = id;
                break 'lacos;
            }
        }
        k += 1;
    }
    Some((a, b))
}

/// finds the semi-finals (0x82) and the final (0x96) and sends the two semi-finalists not in the final to the UEFA
/// seeding
/// Original: `Cup::register_semi_final_losers_for_uefa` (0x519fc0, Cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x519fc0 } (thiscall, ret 0, Cup.cpp): `Cup::register_semi_final_losers_for_uefa()`: os semifinalistas fora da
/// final (semi_final_losers) vão à semeadura da UEFA (0x905790: push b; push a; push id da competição; ecx =
/// [0xde1f58]; sem add esp -> thiscall, ret 0xc).
pub(crate) unsafe extern "thiscall" fn register_semi_final_losers_for_uefa(copa: usize) {
    if let Some((a, b)) = semi_final_losers(copa) {
        let f: unsafe extern "thiscall" fn(usize, i32, i32, i32) = core::mem::transmute(FN_UEFASEEDING_RECORD_CUP_SEMI_FINAL_LOSERS);
        f(read_usize(G_UEFA_SEEDING), read_i32(read_usize(copa + 4)), a, b);
    }
}

/// Comparador da 0x519fc0: lado a lado na tabela da semeadura (+0x440, 50 vagas de 12 bytes, onde a 0x905790 grava; ela
/// não aloca: acha a vaga da competição ou a primeira livre). Sem a semeadura: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_register_semi_final_losers_for_uefa(copa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_REGISTER_SEMI_FINAL_LOSERS_FOR_UEFA);
    let s = read_usize(G_UEFA_SEEDING);
    if s == 0 {
        return o(copa); // ponytail: sem a semeadura, só a original
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(s + 0x440, 50 * 12)], || o(copa), || register_semi_final_losers_for_uefa(copa));
    PLACAR.registrar(igual, || format!("register_semi_final_losers_for_uefa(copa {:x}) a semeadura difere ({:?})", copa, semi_final_losers(copa)));
}

// ---------- 257a leva: a ordem dos clubes da liga para a tabela de jogos e os clubes possíveis de cada lado de um jogo da copa (league.cpp, Cup.cpp) ----------



/// O registro de 59 bytes da tabela da liga.
pub(crate) const CPU12_LINHA: usize = 0x3b;

/// Copies one 59-byte row record (CPU12_LINHA) from origem to destino.
///
/// [pt] Copia um registro de 59 bytes (o `rep movsd` de 14 dwords, a palavra e o byte da original).
pub(crate) unsafe fn copy_row_59(destino: usize, origem: usize) {
    core::ptr::copy(origem as *const u8, destino as *mut u8, CPU12_LINHA);
}

/// Puts table pair (i, j) in grid slots k and half + k, marks both slots and both clubs used, increments k.
///
/// [pt] Põe o par (i, j) da tabela nas vagas k e metade + k da grade, marca as vagas e os dois clubes.
pub(crate) unsafe fn place_pair(grade: usize, posto: *mut u32, usado: *mut u32, tab: usize, i: usize, j: usize, k: &mut i32, metade: i32) {
    copy_row_59(grade + *k as usize * CPU12_LINHA, tab + i * CPU12_LINHA);
    let m = metade + *k;
    copy_row_59(grade + m as usize * CPU12_LINHA, tab + j * CPU12_LINHA);
    *posto.add(m as usize) = 1;
    *posto.add(*k as usize) = 1;
    *usado.add(j) = 1;
    *usado.add(i) = 1;
    *k += 1;
}


/// Core of 0x5219b0: collects the clubs that can play a fixture side, descending to feeder ties for empty sides;
/// returns the count.
///
/// [pt] O núcleo da 0x5219b0: o jogo `jogo` da rodada `rodada` ([[+0xad] + rodada × 4] + jogo × 0x35): cada lado (+0 e
/// +2) com clube vai a `saida[*conta]` (e conta sobe); lado vazio (−1) desce ao jogo de onde ele vem (+0xe/+0xa e
/// +0x10/+0xc). Rodada negativa ou jogo −1 (também nas descidas): a caixa (linha 0x1592, se `caixa`) e a conta.
/// Devolve a conta.
pub(crate) unsafe fn collect_fixture_teams(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32, caixa: bool) -> i32 {
    if rodada < 0 || jogo == -1 {
        if caixa {
            show_error_at(0x9b55ac, 0x1592);
        }
        return *conta;
    }
    let reg = read_u32_as_usize(read_u32_as_usize(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let a = read_i16(reg);
    if a == -1 {
        collect_fixture_teams(this, read_i16(reg + 0xe) as i32, read_i16(reg + 0xa) as i32, saida, conta, caixa);
    } else {
        *saida.offset(*conta as isize) = a;
        *conta += 1;
    }
    let reg = read_u32_as_usize(read_u32_as_usize(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let b = read_i16(reg + 2);
    if b == -1 {
        collect_fixture_teams(this, read_i16(reg + 0x10) as i32, read_i16(reg + 0xc) as i32, saida, conta, caixa);
        return *conta;
    }
    *saida.offset(*conta as isize) = b;
    *conta += 1;
    *conta
}

/// appends to the output the teams that can play the fixture, walking down the fixtures the empty sides come from
/// Original: `Cup::collect_possible_teams` (0x5219b0, Cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5219b0 } (thiscall, ret 0x10, Cup.cpp): `Cup::collect_possible_teams(rodada, jogo, saida, *conta)`: acrescenta
/// a `saida` os clubes que ainda podem jogar esse jogo, descendo pelos jogos de onde vêm os lados vazios. Saída ou
/// conta nula: a original (a caixa).
pub(crate) unsafe extern "thiscall" fn collect_possible_teams(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32) -> i32 {
    if saida.is_null() || conta.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(IDX_COLLECT_POSSIBLE_TEAMS)(this, rodada, jogo, saida, conta);
    }
    collect_fixture_teams(this, rodada, jogo, saida, conta, true)
}

/// the team on one side of the fixture or, if empty, the teams that can still reach it (count in global 0xae1880)
/// Original: `Cup::get_fixture_side_teams` (0x521790, Cup.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x521790 } (thiscall, ret 0x10, Cup.cpp): `Cup::get_fixture_side_teams(rodada, jogo, lado, saida)`: o clube de um
/// lado do jogo (lado ≠ 0: +0; 0: +2) ou, vazio, os clubes que podem chegar a ele (0x5219b0 com a conta no global
/// 0xae1880). Devolve a conta (a caixa se nenhum). Rodada fora de 0..+0x3c, copa sem clubes (+0xa7) ou jogo −1: a
/// original (a caixa e 0).
pub(crate) unsafe extern "thiscall" fn get_fixture_side_teams(this: usize, rodada: i32, jogo: i32, lado: u32, saida: *mut i16) -> i32 {
    if rodada < 0 || rodada > read_i16(this + 0x3c) as i32 || read_u32_as_usize(this + 0xa7) == 0 || jogo == -1 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(IDX_GET_FIXTURE_SIDE_TEAMS)(this, rodada, jogo, lado, saida);
    }
    let coletar: unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32 = core::mem::transmute(FN_CUP_COLLECT_POSSIBLE_TEAMS);
    let conta = G_CUP_POSSIBLE_TEAM_COUNT as *mut i32;
    *conta = 0;
    let reg = read_u32_as_usize(read_u32_as_usize(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let (c, r, j) = if lado as u8 != 0 { (read_i16(reg), reg + 0xe, reg + 0xa) } else { (read_i16(reg + 2), reg + 0x10, reg + 0xc) };
    if c != -1 {
        *saida = c;
        *conta = 1;
        return 1;
    }
    coletar(this, read_i16(r) as i32, read_i16(j) as i32, saida, conta);
    if *conta == 0 {
        show_error_at(0x9b55ac, 0x157f);
    }
    *conta
}


/// Comparador: o esperado sai do núcleo sobre uma conta própria e uma saída própria antes da original; depois, a
/// devolução, a conta global e a saída têm de bater.
pub(crate) unsafe extern "thiscall" fn comparar_get_fixture_side_teams(this: usize, rodada: i32, jogo: i32, lado: u32, saida: *mut i16) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(IDX_GET_FIXTURE_SIDE_TEAMS);
    if saida.is_null() || rodada < 0 || rodada > read_i16(this + 0x3c) as i32 || read_u32_as_usize(this + 0xa7) == 0 || jogo == -1 {
        return o(this, rodada, jogo, lado, saida); // ponytail: a caixa, só a original
    }
    let reg = read_u32_as_usize(read_u32_as_usize(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let (c, r, j) = if lado as u8 != 0 { (read_i16(reg), reg + 0xe, reg + 0xa) } else { (read_i16(reg + 2), reg + 0x10, reg + 0xc) };
    let mut esperado = vec![0i16; 0x100];
    let mut n: i32 = 0;
    if c != -1 {
        esperado[0] = c;
        n = 1;
    } else {
        collect_fixture_teams(this, read_i16(r) as i32, read_i16(j) as i32, esperado.as_mut_ptr(), &mut n, false);
    }
    let res = o(this, rodada, jogo, lado, saida);
    let obtido = core::slice::from_raw_parts(saida, res.clamp(0, 0x100) as usize);
    let global = *(G_CUP_POSSIBLE_TEAM_COUNT as *const i32);
    PLACAR.registrar(res == n && global == n && obtido == &esperado[..n.clamp(0, 0x100) as usize], || format!("get_fixture_side_teams({:x}, {}, {}, {}) {}/{} global {} {:?} x {:?}", this, rodada, jogo, lado as u8, res, n, global, obtido, &esperado[..n.clamp(0, 0x100) as usize]));
    res
}

/// Comparador: o esperado sai do núcleo sobre uma cópia da conta e uma saída própria antes da original; depois, a
/// devolução, a conta e os clubes acrescentados têm de bater.
pub(crate) unsafe extern "thiscall" fn comparar_collect_possible_teams(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(IDX_COLLECT_POSSIBLE_TEAMS);
    if saida.is_null() || conta.is_null() || rodada < 0 || jogo == -1 || !(0..0x100).contains(&*conta) {
        return o(this, rodada, jogo, saida, conta); // ponytail: a caixa ou conta fora do esperado, só a original
    }
    let c0 = *conta;
    let mut esperado = vec![0i16; 0x200];
    let mut n = c0;
    collect_fixture_teams(this, rodada, jogo, esperado.as_mut_ptr(), &mut n, false);
    let res = o(this, rodada, jogo, saida, conta);
    let fim = (*conta).clamp(c0, 0x200);
    let obtido = core::slice::from_raw_parts(saida.offset(c0 as isize), (fim - c0) as usize);
    let ok = res == n && *conta == n && obtido == &esperado[c0 as usize..n.clamp(c0, 0x200) as usize];
    PLACAR.registrar(ok, || format!("collect_possible_teams({:x}, {}, {}) {}/{} conta {} {:?} x {:?}", this, rodada, jogo, res, n, *conta, obtido, &esperado[c0 as usize..n.clamp(c0, 0x200) as usize]));
    res
}

// ---------- 255a leva: o dia a dia das copas e o risco de doença na viagem: a atualização da copa no dia, montar a rodada da copa e o risco de doença dos visitantes (Cup.cpp, fix_man.cpp) ----------




/// Builds a date from day and year with Date::Date (0x549ef0).
///
/// [pt] Uma data montada pela 0x549ef0 (dia, ano: ela lê as palavras; os dwords vão como os da original).
pub(crate) unsafe fn make_date(dia: u32, ano: u32) -> [u8; 8] {
    let montar: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let mut d = [0u8; 8];
    montar(&mut d, dia, ano);
    d
}

/// (day, year) words of an 8-byte date.
///
/// [pt] (dia, ano) de uma data.
pub(crate) fn cup_date_day_year(d: &[u8; 8]) -> (u16, u16) {
    (u16::from_le_bytes([d[0], d[1]]), u16::from_le_bytes([d[2], d[3]]))
}

/// Address of cup round k (0x68 bytes each at +0xa3), signed index.
///
/// [pt] A rodada `k` da copa (0x68 bytes cada, em +0xa3).
pub(crate) unsafe fn round_record_at(this: usize, k: isize) -> usize {
    (read_u32_as_usize(this + 0xa3) as isize).wrapping_add(k.wrapping_mul(0x68)) as usize
}

/// Date of cup round k (day +0, year byte +2 relative to cup +0x40), built like the original's blocks in 0x51f890.
///
/// [pt] A data da rodada `k` (+0 o dia, +2 o ano a partir do ano-base +0x40 da copa, byte com sinal), como nos blocos da
/// 0x51f890 em que a metade alta do dword do dia é a do ponteiro da rodada (a do ano é zero).
pub(crate) unsafe fn round_date(this: usize, k: isize) -> [u8; 8] {
    let r = round_record_at(this, k);
    let ano = (read_u8(r + 2) as i8 as i16).wrapping_add(read_u16(this + 0x40) as i16) as u16 as u32;
    make_date((r as u32 & 0xffff_0000) | read_u16(r) as u32, ano)
}

/// Calls the thiscall virtual at offset off of object p with no arguments.
///
/// [pt] A virtual `off` do objeto `p` (thiscall, sem argumentos).
pub(crate) unsafe fn cup_call_virtual_no_args(p: usize, off: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(p) + off));
    f(p)
}

/// Stage i of the cup ([+0xc][i]).
///
/// [pt] A fase `i` da copa ([+0xc][i]).
pub(crate) unsafe fn cup_stage(this: usize, i: usize) -> usize {
    read_u32_as_usize(read_u32_as_usize(this + 0xc) + i * 4)
}

/// moves on completed stages and ends the cup, gives each stage the day and, on the date, draws (phase 0) or sets up
/// (phase ≠ 0) the rounds until none advances
/// Original: `Cup::update_for_date` (0x51f890, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x51f890 } (thiscall, ret 8, Cup.cpp; a atualização diária de cada copa): `Cup::update_for_date(hoje, fase)`. Copa
/// sem pai (+0x44 = 0xff): com +0xb1, se as fases 0..=+0x30 estão completas (virtual +0x10 com 0, 1), passa à fase
/// seguinte (virtual +0x28, se +0x30 < +0x2c − 1) ou encerra uma vez (+0x4c: o histórico 0x442e80 de [0xad9da8], a
/// virtual +0x30, os recordes 0x4a19f0 do Comp_Stats +8); depois as notícias dos recordes (0x4a2450). Ainda sem pai,
/// cada fase recebe o dia (virtual +0x14 com 0xae2c90 e a fase). Fase 0 (com [0xae2c88] = 1): a última rodada final
/// (+7 = 0x8c) sorteia na data dela (0x51c410); sem +0xb1, sorteia as rodadas a partir de +0xac cuja data chegou, até
/// não avançar ou acabar. Fase ≠ 0 (com +0x45): o mesmo com a montagem das rodadas (0x51c800) a partir de +0xab, que
/// também monta na data pendente da rodada (+0x34 da fase da rodada). Devolve 0.
pub(crate) unsafe extern "thiscall" fn cup_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    let sortear: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_DRAW_ROUND);
    let montar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    if read_u8(this + 0x44) == 0xff {
        let mut completas = read_u8(this + 0xb1) != 0;
        if completas {
            let mut i = 0usize;
            while (i as i32) <= read_u32_as_usize(this + 0x30) as i32 {
                let p = cup_stage(this, i);
                let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(p) + 0x10));
                if completa(p, 0, 1) == 0 {
                    completas = false;
                    break;
                }
                i += 1;
            }
        }
        if completas {
            if (read_u32_as_usize(this + 0x30) as i32) < (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
                cup_call_virtual_no_args(this, 0x28); // a fase seguinte
            } else if read_u32_as_usize(this + 0x4c) == 0 {
                let historico: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_CLUB_HISTORY_RECORD_COMP_SEASON);
                historico(read_u32_as_usize(G_CLUB_HISTORY), this);
                cup_call_virtual_no_args(this, 0x30);
                if read_u32_as_usize(this + 8) != 0 {
                    let recordes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_COMP_STATS_UPDATE_SEASON_END_RECORDS);
                    recordes(read_u32_as_usize(this + 8));
                }
                core::ptr::write_unaligned((this + 0x4c) as *mut u32, 1);
            }
        }
        if read_u32_as_usize(this + 8) != 0 {
            let noticias: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_COMP_STATS_SEND_RECORD_NEWS);
            noticias(read_u32_as_usize(this + 8));
        }
    }
    if read_u8(this + 0x44) == 0xff {
        let mut i = 0usize;
        while (i as i32) <= read_u32_as_usize(this + 0x30) as i32 {
            let p = cup_stage(this, i);
            let dia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(p) + 0x14));
            dia(p, 0xae2c90, fase);
            i += 1;
        }
    }
    let (hdia, hano) = (read_u16(hoje), read_u16(hoje + 2));
    let n = read_u16(this + 0x3c) as i16;
    if fase == 0 {
        if read_u8(G_CURRENT_DAY_SLICE) != 1 {
            return 0;
        }
        if n > 0 && read_u16(round_record_at(this, n as isize - 1) + 7) == 0x8c {
            let r = round_record_at(this, n as isize - 1);
            let ano = (read_u8(r + 2) as i8 as i16).wrapping_add(read_u16(this + 0x40) as i16) as u16 as u32;
            let d = make_date(read_u16(r) as u32, ano);
            if cup_date_day_year(&d) == (hdia, hano) {
                sortear(this, (n as i32 - 1) as u32); // a final na data dela
            }
        }
        if read_u8(this + 0xb1) != 0 {
            return 0;
        }
        let mut k = read_u8(this + 0xac) as i8;
        if k as i16 == read_u16(this + 0x3c) as i16 {
            return 0;
        }
        loop {
            let (dia, ano) = cup_date_day_year(&round_date(this, k as isize));
            // o ano sem sinal, o dia com sinal
            if ano > hano || (ano == hano && dia as i16 > hdia as i16) { // cps13: a data do sorteio
                return 0;
            }
            crate::comparacao::ramo("cps_13 sorteio da rodada");
            sortear(this, read_u8(this + 0xac) as i8 as i32 as u32);
            let novo = read_u8(this + 0xac) as i8;
            if novo == k || novo as i16 == read_u16(this + 0x3c) as i16 {
                return 0;
            }
            k = novo;
        }
    }
    if read_u32_as_usize(this + 0x45) == 0 {
        return 0;
    }
    if n > 0 && read_u16(round_record_at(this, n as isize - 1) + 7) == 0x8c {
        let r = round_record_at(this, n as isize - 1);
        let ano = (read_u8(r + 2) as i8 as i16).wrapping_add(read_u16(this + 0x40) as i16) as u16 as u32;
        let d = make_date(read_u16(r) as u32, ano);
        if cup_date_day_year(&d) == (hdia, hano) {
            montar(this, (n as i32 - 1) as u32); // a final na data dela
        }
    }
    if read_u8(this + 0xb1) != 0 || read_u32_as_usize(this + 0x45) == 0 {
        return 0;
    }
    let mut k = read_u8(this + 0xab) as i8;
    if k as i16 == read_u16(this + 0x3c) as i16 {
        return 0;
    }
    loop {
        let (dia, ano) = cup_date_day_year(&round_date(this, k as isize));
        if (ano, dia) != (hano, hdia) { // cps13: a data da montagem
            let atual = read_u8(this + 0xab) as i8 as isize;
            let p = read_u32_as_usize((read_u32_as_usize(this + 0xad) as isize + atual * 4) as usize);
            if read_u8(p + 0x34) == 0 {
                return 0; // nem a data nem pendente
            }
        }
        crate::comparacao::ramo("cps_13 montagem da rodada");
        montar(this, read_u8(this + 0xab) as i8 as i32 as u32);
        let novo = read_u8(this + 0xab) as i8;
        if novo == k || novo as i16 == read_u16(this + 0x3c) as i16 {
            return 0;
        }
        k = novo;
    }
}

/// draws the round if needed, creates the fixtures of the marked ties, moves them to the round dates and moves on; a
/// team without a club postpones the rounds by a day
/// Original: `Cup::set_up_round` (0x51c800, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x51c800 } (thiscall, ret 4, Cup.cpp): `Cup::set_up_round(rodada)`: sem +0x45: 1. Rodada fora de 0..+0x3c − 1: a
/// caixa (linha 0xa64) e −1. Sorteia a rodada se a fase dela não tem +0x34 (0x51c410). Na rodada atual (+0xab): se um
/// confronto (0x35 bytes na fase, +0/+2 os times ou 0xffff) tem time sem clube (+0xa7, 6 bytes), adia a rodada pedida e
/// as seguintes um dia (0x54c5b0, 0x417c10; as seguintes só se ficaram antes) e devolve 1. Senão cria os jogos de cada
/// confronto marcado (+0x34 = 1 adiado, 2 já: 0x51cb90; depois do 2, sem a copa em [0xadadfc][id]: 1), leva os jogos às
/// datas da rodada (data +3/+5 num bloco de 8 bytes, 0x51ebe0, liberado depois) e, com a copa em [0xadadfc][id], passa à
/// rodada seguinte (fora da final 0x8c) e leva +0xac junto. Devolve 1.
pub(crate) unsafe extern "thiscall" fn set_up_round(this: usize, rodada: u32) -> u32 {
    let id = read_u32_as_usize(read_u32_as_usize(this + 4));
    let ativa = || read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + id * 4) != 0;
    if read_u32_as_usize(this + 0x45) == 0 {
        return 1;
    }
    let ri = rodada as i32;
    if ri > read_u16(this + 0x3c) as i16 as i32 - 1 || ri < 0 {
        show_error_at(0x9b55ac, 0xa64);
        return u32::MAX;
    }
    let ad = read_u32_as_usize(this + 0xad);
    if read_u8(read_u32_as_usize(ad + rodada as usize * 4) + 0x34) == 0 {
        let sortear: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_CUP_DRAW_ROUND);
        sortear(this, rodada);
    }
    let atual = read_u8(this + 0xab) as i8 as isize;
    let base = read_u32_as_usize(this + 0xa3);
    let off = atual * 0x68;
    let n = *((base as isize + off + 0x1a) as *const i16) as i32;
    let mut sem_clube = false;
    for i in 0..n.max(0) as usize {
        let t = read_u32_as_usize((read_u32_as_usize(this + 0xad) as isize + atual * 4) as usize) + i * 0x35;
        for lado in [0usize, 2] {
            let time = read_u16(t + lado);
            if time != 0xffff && read_u32_as_usize(read_u32_as_usize(this + 0xa7) + (time as i16 as isize * 6) as usize) == 0 {
                sem_clube = true;
                break;
            }
        }
        if sem_clube {
            break;
        }
    }
    if sem_clube {
        // a rodada pedida (não a atual) e as seguintes ficam um dia depois
        let mais_dias: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
        let atribuir: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
        let r = round_record_at(this, ri as isize);
        let ano = (read_u8(r + 2) as i8 as i16).wrapping_add(read_u16(this + 0x40) as i16) as u16 as u32;
        let mut d = make_date((read_u32_as_usize(this + 0xa7) as u32 & 0xffff_0000) | read_u16(r) as u32, ano);
        let (mut amanha, mut copia) = ([0u8; 8], [0u8; 8]);
        let p = mais_dias(&mut d, &mut amanha, 1);
        atribuir(&mut d, &mut copia, p);
        let (dia, ano) = cup_date_day_year(&d);
        crate::comparacao::ramo("cps_13 rodada adiada");
        let r = round_record_at(this, ri as isize);
        core::ptr::write_unaligned(r as *mut u16, dia);
        *((r + 2) as *mut u8) = (ano as u8).wrapping_sub(read_u8(this + 0x40));
        let mut k = ri + 1;
        while (k as i16) < read_u16(this + 0x3c) as i16 {
            let r = round_record_at(this, k as i16 as isize);
            let rano = (read_u8(r + 2) as i8 as i16).wrapping_add(read_u16(this + 0x40) as i16) as u16 as u32;
            let (odia, oano) = cup_date_day_year(&make_date((read_u32_as_usize(this + 0xa3) as u32 & 0xffff_0000) | read_u16(r) as u32, rano));
            if ano > oano || (ano == oano && dia as i16 > odia as i16) {
                let r = round_record_at(this, k as i16 as isize);
                core::ptr::write_unaligned(r as *mut u16, dia);
                *((r + 2) as *mut u8) = (ano as u8).wrapping_sub(read_u8(this + 0x40));
            }
            k += 1;
        }
        return 1;
    }
    let criar: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_CUP_CREATE_TIE_FIXTURES);
    let mut i = 0i32;
    while n > 0 && i < *((read_u32_as_usize(this + 0xa3) as isize + off + 0x1a) as *const i16) as i32 {
        let marca = |i: i32| read_u8(read_u32_as_usize((read_u32_as_usize(this + 0xad) as isize + atual * 4) as usize) + i as usize * 0x35 + 0x34);
        if marca(i) == 1 {
            criar(this, atual as i32 as u32, i as u32, 1);
        }
        if marca(i) == 2 {
            criar(this, atual as i32 as u32, i as u32, 0);
            if !ativa() {
                return 1;
            }
        }
        i += 1;
    }
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let montar_data: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let p = novo(8);
    let data = if p != 0 {
        let base = read_u32_as_usize(this + 0xa3);
        let r = (base as isize + off) as usize;
        let ano = (read_u8(r + 5) as i8 as i16).wrapping_add(read_u16(this + 0x40) as i16) as u16 as u32;
        let dia = read_u16(r + 3) as u32;
        montar_data(p, (r as u32 & 0xffff_0000) | dia, (base as u32 & 0xffff_0000) | ano)
    } else {
        0
    };
    let levar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_CUP_MOVE_ROUND_FIXTURES_TO_SET_DATES);
    crate::comparacao::ramo("cps_13 jogos da rodada");
    levar(this, data, atual as i32 as u32);
    if data != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
        liberar(data);
    }
    if !ativa() {
        return 1;
    }
    if read_u16((read_u32_as_usize(this + 0xa3) as isize + off + 7) as usize) != 0x8c {
        *((this + 0xab) as *mut u8) = read_u8(this + 0xab).wrapping_add(1);
    }
    if (read_u8(this + 0xac) as i8) < read_u8(this + 0xab) as i8 {
        *((this + 0xac) as *mut u8) = read_u8(this + 0xab);
    }
    1
}

/// Adds an illness outbreak risk (type 3, Injury 0x62e820) on the eve of fixture f's date for club id in the stadium's
/// nation.
///
/// [pt] Põe o risco de doença (0x62e820 de [0xb63d28], tipo 3) na véspera da data do jogo `f` (+0x2a o dia, +0x28 o ano;
/// 0x549ef0, 0x54c8e0, 0x417c10) para o clube `id` com a nação do estádio. `alto_dia`/`alto_ano`: as metades altas dos
/// dwords que a original empurra à 0x549ef0.
pub(crate) unsafe fn add_illness_risk_before_match(f: usize, id: usize, nacao: usize, alto_dia: u32, alto_ano: u32) {
    let menos_dias: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let atribuir: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let doenca: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32, usize, usize) -> u32 = core::mem::transmute(FN_INJURY_ADD_ILLNESS_OUTBREAK);
    let mut d = make_date((alto_dia & 0xffff_0000) | read_u16(f + 0x2a) as u32, (alto_ano & 0xffff_0000) | read_u16(f + 0x28) as u32);
    let (mut vespera, mut copia) = ([0u8; 8], [0u8; 8]);
    let p = menos_dias(&mut d, &mut vespera, 1);
    atribuir(&mut d, &mut copia, p);
    doenca(read_u32_as_usize(G_INJURY), &mut d, 3, id, nacao);
}


// ponytail: sorteia e monta rodadas (aloca jogos): não roda duas vezes. Só a original; a cópia leva os ramos (sorteio e
// montagem; a atualização é diária em cada copa, 29.934 chamadas até 2002-06-01: sem ramo no caminho comum).
pub(crate) unsafe extern "thiscall" fn comparar_cup_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CUP_UPDATE_FOR_DATE)(this, hoje, fase)
}

// ponytail: sorteia, cria os jogos e passa a rodada: não roda duas vezes. Só a original; a cópia leva os ramos.
pub(crate) unsafe extern "thiscall" fn comparar_set_up_round(this: usize, rodada: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_SET_UP_ROUND)(this, rodada)
}

// ponytail: acrescenta o risco de doença (não roda duas vezes; o caminho comum não faz nada). Só a original; a cópia leva
// os ramos.

// ---------- 253a leva: a copa: o sorteio dos times da rodada, os confrontos no estádio compartilhado e a escala pela rodada (Cup.cpp, comp_util.cpp) ----------


/// Reads a u32 at a as usize.
pub(crate) unsafe fn read_ptr(a: usize) -> usize {
    read_u32(a) as usize
}



/// base + i × size with signed index (like the original's movsx).
///
/// [pt] `base + i × tam` (o índice com sinal, como o movsx do original).
pub(crate) fn index_addr(base: usize, i: i32, tam: usize) -> usize {
    base.wrapping_add((i as isize).wrapping_mul(tam as isize) as usize)
}

/// Address of tie k (0x35 bytes) of round r: [[cup + 0xad] + 4r] + 0x35k.
///
/// [pt] O confronto `k` (0x35 bytes) da rodada `r` da copa: [[copa + 0xad] + 4r] + 0x35k.
pub(crate) unsafe fn tie_of_round(copa: usize, r: i32, k: i32) -> usize {
    index_addr(read_ptr(index_addr(read_ptr(copa + 0xad), r, 4)), k, 0x35)
}

/// Address of round r's 0x68-byte entry in [cup + 0xa3] (+0x18 team slots, +0x1a ties).
///
/// [pt] A entrada da rodada `r` (0x68 bytes) em [copa + 0xa3]: +0x18 vagas de times, +0x1a confrontos.
pub(crate) unsafe fn round_entry(copa: usize, r: i32) -> usize {
    index_addr(read_ptr(copa + 0xa3), r, 0x68)
}

/// Address of cup team t's 6-byte entry in [cup + 0xa7] (+0 club, +4 level).
///
/// [pt] O clube do time `t` da copa (6 bytes em [copa + 0xa7]: +0 clube, +4 nível).
pub(crate) unsafe fn cup_team_entry(copa: usize, t: i16) -> usize {
    index_addr(read_ptr(copa + 0xa7), t as i32, 6)
}

/// Memory regions of all rounds' tie arrays (plus 0xb67a34), for the comparers.
///
/// [pt] Os confrontos de todas as rodadas da copa (rodadas em +0x3c), para os comparadores.
pub(crate) unsafe fn all_ties_regions(copa: usize) -> Vec<(usize, usize)> {
    let mut v = vec![(0xb67a34, 4)];
    for r in 0..(read_i16(copa + 0x3c) as i32).max(0) {
        let f = read_ptr(index_addr(read_ptr(copa + 0xad), r, 4));
        let n = read_i16(round_entry(copa, r) + 0x1a) as i32;
        if f != 0 && n > 0 {
            v.push((f, n as usize * 0x35));
        }
    }
    v
}

/// Shows Cup.cpp's "Memory Allocation Error" box for a line and text, then exits the game like the original.
///
/// [pt] A caixa "Memory Allocation Error" ("v%s %s.%s %d (%s)" de Cup.cpp na linha `linha`) e o jogo para (como a original).
pub(crate) unsafe fn fatal_memory_error_box(linha: u32, texto: usize) -> ! {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let fechar1: unsafe extern "C" fn() = core::mem::transmute(FN_GAME_FATAL_CLEANUP);
    let fechar2: unsafe extern "C" fn() = core::mem::transmute(FN_SHARED_NOOP_RET0_5A8F60);
    let sair: unsafe extern "C" fn(i32) -> ! = core::mem::transmute(FN_CRT_EXIT);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9b55ac, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
    *(G_MOUSE_LEFT_BUTTON_DOWN as *mut u32) = 0;
    fechar1();
    fechar2();
    sair(-1)
}

/// takes the teams out of the round ties, shuffles them and puts each one in a drawn tie (empty ones first), drawing
/// the side when both slots are free, and sets the link to the next round
/// Original: `Cup::randomize_round_teams` (0x51e220, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x51e220 } (thiscall, ret 4, Cup.cpp): `Cup::randomize_round_teams(rodada)`: o sorteio dos times da rodada.
/// 1) Tira os times dos confrontos (+0x1a da rodada) para um vetor de 7 bytes por vaga (malloc de 7 × vagas +0x18;
///    sem memória, caixa da linha 0xe7f e o jogo para): o mandante (+0, com o vínculo +0xe/+0xa) e o visitante (+2,
///    com +0x10/+0xc), cada um com o nível do time (+4 do time); a vaga do confronto vira −2 (livre); sem time (−1),
///    a entrada fica −1 (com 0xff no nível).
/// 2) Embaralha: para cada posição, troca com a entrada sorteada entre todas (0x90cfc0(n)).
/// 3) Cada entrada com time vai a um confronto sorteado (0x90cfc0 dos confrontos), avançando em círculo: na primeira
///    volta só confrontos com as duas vagas vazias (< 0); depois qualquer um com vaga livre (−2); dando a volta
///    inteira na segunda, caixa da linha 0xf4c e sai (a marca da segunda volta não volta a zero). Com as duas vagas
///    livres, sorteia o lado (0x90cfc0(2): ≠ 0 mandante). O time leva o vínculo (+0xe/+0xa ou +0x10/+0xc) e, com
///    vínculo, o confronto da rodada seguinte ganha a rodada (+0x12), o confronto sorteado (+6) e o lado (+8: 1
///    mandante, 0 visitante).
/// Libera o vetor (0x9452ca).
pub(crate) unsafe extern "thiscall" fn randomize_round_teams(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let vagas = read_i16(round_entry(copa, r) + 0x18) as i32;
    let vetor = alocar(vagas.wrapping_mul(7) as u32);
    if vetor == 0 {
        fatal_memory_error_box(0xe7f, 0x9b574c); // "CUP::randomize_round_teams() : Cannot allocate memory"
    }
    let entrada = |i: i16| index_addr(vetor, i as i32, 7);
    // 1) os times saem dos confrontos
    let mut n: i16 = 0;
    let mut k: i32 = 0;
    while (k as i16) < read_i16(round_entry(copa, r) + 0x1a) {
        for (lado, ligado, jogo) in [(0usize, 0xeusize, 0xausize), (2, 0x10, 0xc)] {
            let t = tie_of_round(copa, r, k as i16 as i32);
            let e = entrada(n);
            n = n.wrapping_add(1);
            let time = read_i16(t + lado);
            if time != -1 {
                write_u16(e, time as u16);
                write_u16(e + 2, read_i16(t + ligado) as u16);
                write_u16(e + 4, read_i16(t + jogo) as u16);
                *((e + 6) as *mut u8) = *((cup_team_entry(copa, time) + 4) as *const u8); // o nível do time
                write_u16(t + lado, 0xfffe); // a vaga fica livre
            } else {
                write_u16(e, 0xffff);
                *((e + 6) as *mut u8) = 0xff;
            }
        }
        k += 1;
    }
    // 2) o embaralhamento
    if n > 0 {
        let mut p = vetor;
        for _ in 0..n as i32 {
            let q = entrada(sorteio(n as i32) as i16);
            let (a, b, c) = (read_u32(q), read_i16(q + 4), *((q + 6) as *const u8));
            core::ptr::write_unaligned(q as *mut u32, read_u32(p));
            write_u16(q + 4, read_i16(p + 4) as u16);
            *((q + 6) as *mut u8) = *((p + 6) as *const u8);
            core::ptr::write_unaligned(p as *mut u32, a);
            write_u16(p + 4, b as u16);
            *((p + 6) as *mut u8) = c;
            p += 7;
        }
    }
    // 3) cada time vai a um confronto sorteado
    let mut segunda_volta = false;
    let mut i: i16 = 0;
    while i < n {
        let e = entrada(i);
        i = i.wrapping_add(1);
        if read_i16(e) == -1 {
            continue;
        }
        let mut s = sorteio(read_i16(round_entry(copa, r) + 0x1a) as i32) as i16;
        let inicio = s;
        let mandante = loop {
            let t = tie_of_round(copa, r, s as i32);
            let (casa, fora) = (read_i16(t), read_i16(t + 2));
            if segunda_volta || (casa < 0 && fora < 0) {
                let casa_livre = casa == -2;
                if fora == -2 {
                    if casa_livre && sorteio(2) != 0 {
                        crate::comparacao::ramo("cpq_11 sorteio mandante");
                        break true;
                    }
                    break false;
                }
                if casa_livre {
                    break true;
                }
            }
            s = s.wrapping_add(1);
            if s >= read_i16(round_entry(copa, r) + 0x1a) {
                s = 0;
            }
            if s == inicio {
                if segunda_volta {
                    show_error_at(0x9b55ac, 0xf4c);
                    liberar(vetor);
                    return;
                }
                segunda_volta = true;
            }
        };
        let t = tie_of_round(copa, r, s as i32);
        let (lado, ligado, jogo) = if mandante { (0usize, 0xeusize, 0xausize) } else { (2, 0x10, 0xc) };
        write_u16(t + lado, read_i16(e) as u16);
        write_u16(t + ligado, read_i16(e + 2) as u16);
        write_u16(t + jogo, read_i16(e + 4) as u16);
        let seguinte = read_i16(e + 2);
        if seguinte != -1 {
            let v = tie_of_round(copa, seguinte as i32, read_i16(e + 4) as i32);
            write_u16(v + 0x12, rodada as u16);
            write_u16(v + 6, s as u16);
            write_u16(v + 8, mandante as u16);
        }
    }
    liberar(vetor);
}

/// Comparador: sorteia (0x90cfc0) e só usa um vetor temporário: lado a lado pelo estado do sorteador e pelos
/// confrontos de todas as rodadas (os vínculos vão às rodadas seguintes), a cópia em sem_contar.
pub(crate) unsafe extern "thiscall" fn comparar_randomize_round_teams(copa: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_RANDOMIZE_ROUND_TEAMS);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend(all_ties_regions(copa));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada),
        || crate::aleatorio::sem_contar(|| randomize_round_teams(copa, rodada)));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("randomize_round_teams(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// Swaps the word pairs 0x522610 swaps in tie t: home/away (+0/+2), +0xe/+0x10 and +0xa/+0xc.
///
/// [pt] Os pares de palavras do confronto `t` que a 0x522610 troca: mandante/visitante (+0/+2), +0xa/+0xc e +0xe/+0x10.
pub(crate) unsafe fn swap_tie_sides(t: usize) {
    for (a, b) in [(0usize, 2usize), (0xe, 0x10), (0xa, 0xc)] {
        let (x, y) = (read_i16(t + a), read_i16(t + b));
        write_u16(t + a, y as u16);
        write_u16(t + b, x as u16);
    }
}

/// swaps home and away in the later tie whose team shares a stadium with a team of an earlier tie of the round, and
/// sets the side in the linked ties
/// Original: `Cup::separate_shared_stadium_ties` (0x522610, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x522610 } (thiscall, ret 4, Cup.cpp): `Cup::separate_shared_stadium_ties(rodada)`: para cada confronto `i` da
/// rodada, procura o primeiro `j > i` cujo mandante (ou, senão, o visitante) divide o estádio (+0x69 do clube, não
/// nulo) com o mandante (ou o visitante) de `i`; achando, inverte o confronto `j` (mandante e visitante, +0xa/+0xc e
/// +0xe/+0x10) e acerta o lado (+8) dos confrontos ligados na rodada seguinte: o de +0xe/+0xa fica 1 (mandante), o de
/// +0x10/+0xc fica 0. Os dois jogos no mesmo estádio deixam de cair no mesmo lado.
pub(crate) unsafe extern "thiscall" fn separate_shared_stadium_ties(copa: usize, rodada: u32) {
    let r = rodada as i16 as i32;
    let estadio = |t: i16| read_u32(read_ptr(cup_team_entry(copa, t)) as usize + 0x69);
    let mut n = read_i16(round_entry(copa, r) + 0x1a);
    let mut i: i32 = 0;
    while (i as i16) < n {
        let fi = tie_of_round(copa, r, i as i16 as i32);
        let casa_i = read_i16(fi);
        let mut j = i + 1;
        while (j as i16) < n {
            let fj = tie_of_round(copa, r, j as i16 as i32);
            let mut dividem = false;
            for lado in [0usize, 2] {
                let a = if lado == 0 { casa_i } else { read_i16(fi + 2) };
                let b = read_i16(fj + lado);
                if a != -1 && b != -1 {
                    crate::comparacao::ramo("cpq_11 estadio comparado");
                    let (sa, signed_byte_at) = (estadio(a), estadio(b));
                    if sa == signed_byte_at && sa != 0 {
                        dividem = true;
                        break;
                    }
                }
            }
            if dividem {
                swap_tie_sides(fj);
                let seguinte = read_i16(fj + 0xe);
                if seguinte != -1 {
                    write_u16(tie_of_round(copa, seguinte as i32, read_i16(fj + 0xa) as i32) + 8, 1);
                }
                let seguinte = read_i16(fj + 0x10);
                if seguinte != -1 {
                    write_u16(tie_of_round(copa, seguinte as i32, read_i16(fj + 0xc) as i32) + 8, 0);
                }
                break;
            }
            j += 1;
        }
        n = read_i16(round_entry(copa, r) + 0x1a);
        i += 1;
    }
}

/// Comparador: só grava confrontos (da rodada e das seguintes): lado a lado pelos confrontos de todas as rodadas.
pub(crate) unsafe extern "thiscall" fn comparar_separate_shared_stadium_ties(copa: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_SEPARATE_SHARED_STADIUM_TIES);
    let reg = all_ties_regions(copa);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(copa, rodada), || separate_shared_stadium_ties(copa, rodada));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0, &mr.0);
        let dif: Vec<usize> = (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect();
        format!("separate_shared_stadium_ties(rodada {}) diferencas em {:x?}", rodada as i16, dif)
    });
}

/// Side-effect-free 0x4bf850: value between low and high byte by round (midpoint with a single round); returns the
/// original eax and the byte.
///
/// [pt] A conta da 0x4bf850 sem efeitos: o eax da original e o byte devolvido. Com uma rodada só, (baixo + alto) / 2
/// (truncada para o zero) sobre a soma; senão o quociente (alto − baixo) × rodada / (rodadas − 1), sem somar o baixo.
pub(crate) fn interpolate_by_round(baixo: u32, alto: u32, rodada: u32, rodadas: u32) -> (u32, i8) {
    let (b, a) = (baixo as u8 as i8 as i32, alto as u8 as i8 as i32);
    let n = rodadas as u16 as i16 as i32;
    if n == 1 {
        let s = b.wrapping_add(a);
        let t = s.wrapping_sub(s >> 31); // cdq; sub eax, edx
        let v = (t >> 1) as i8;
        ((t as u32 & 0xffff_ff00) | v as u8 as u32, v)
    } else {
        let q = a.wrapping_sub(b).wrapping_mul(rodada as u16 as i16 as i32).wrapping_div(n - 1); // a média por rodada
        (q as u32, q as i8)
    }
}


// ---------- 252a leva: as copas e a divisão dos clubes: a ordem dos cabeças de chave do sorteio (decrescente e crescente) e o teto da divisão dos clubes no fim da temporada (Cup.cpp, league.cpp) ----------


/// Level byte (+4, signed) of a cup entry.
///
/// [pt] O nível (+4, byte com sinal) e a reputação do clube (+0 → +0x80, palavra com sinal) de uma entrada de copa.
pub(crate) unsafe fn entry_level(e: usize) -> i32 {
    *((e + 4) as *const i8) as i32
}

/// Reputation (+0x80) of the club (+0) of a cup entry.
pub(crate) unsafe fn entry_club_reputation(e: usize) -> i32 {
    *((read_usize(e) + 0x80) as *const i16) as i32
}

/// qsort comparator of cup entries by level (+4), highest first; on a tie by club reputation (+0x80), lowest first
/// Original: `cup_compare_entries_by_level_desc` (0x522190, Cup.cpp, cdecl).
///
/// [pt] { 0x522190 } (cdecl, Cup.cpp): `cup_compare_entries_by_level_desc(a, b)`: o qsort das entradas das copas (sorteios da
/// Copa da UEFA, Intertoto, copas do Japão e da Escócia) pelo nível, o maior primeiro; no empate pela reputação do
/// clube, a menor primeiro. Quente (qsort): sem contador.
pub(crate) unsafe extern "C" fn cup_compare_entries_by_level_desc(a: usize, b: usize) -> i32 {
    let r = entry_level(b) - entry_level(a); // o maior nível primeiro
    if r != 0 {
        return r;
    }
    entry_club_reputation(a) - entry_club_reputation(b)
}

/// qsort comparator of cup entries by level (+4), lowest first; on a tie by club reputation (+0x80), lowest first
/// Original: `cup_compare_entries_by_level_asc` (0x5221c0, Cup.cpp, cdecl).
///
/// [pt] { 0x5221c0 } (cdecl, Cup.cpp): `cup_compare_entries_by_level_asc(a, b)`: o qsort das entradas da copa da liga
/// inglesa (0x56ccc0) pelo nível, o menor primeiro; no empate pela reputação do clube, a menor primeiro. Quente
/// (qsort): sem contador.
pub(crate) unsafe extern "C" fn cup_compare_entries_by_level_asc(a: usize, b: usize) -> i32 {
    let r = entry_level(a) - entry_level(b); // o menor nível primeiro
    if r != 0 {
        return r;
    }
    entry_club_reputation(a) - entry_club_reputation(b)
}

/// Moves the club's professional status byte (+0x64) to valor: down if above (baixar ≠ 0), up if below otherwise.
///
/// [pt] Leva o byte +0x64 do clube (com sinal) até `valor`: para baixo (`baixar` ≠ 0) se está acima, para cima se está
/// abaixo.
pub(crate) unsafe fn cap_professional_status(clube: usize, valor: i8, baixar: u32) {
    let p = (clube + club::PROFESSIONAL_STATUS) as *mut i8;
    let muda = if baixar != 0 { *p > valor } else { *p < valor };
    if muda {
        crate::comparacao::ramo("cpp_13 teto da divisao");
        *p = valor; // o teto novo
    }
}


/// Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_cup_compare_entries_by_level_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_CUP_COMPARE_ENTRIES_BY_LEVEL_DESC)(a, b);
    let n = cup_compare_entries_by_level_desc(a, b);
    PLACAR.registrar(r == n, || format!("cup_compare_entries_by_level_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
pub(crate) unsafe extern "C" fn comparar_cup_compare_entries_by_level_asc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_CUP_COMPARE_ENTRIES_BY_LEVEL_ASC)(a, b);
    let n = cup_compare_entries_by_level_asc(a, b);
    PLACAR.registrar(r == n, || format!("cup_compare_entries_by_level_asc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}


// ---------- 249a leva: a copa: quem passa no jogo e no confronto e a ordem dos cabeças de chave no sorteio (Cup.cpp) ----------





/// Body of 0x51c020: the winning side of a fixture (goals, extra time, penalties), or None on an unresolved penalty
/// draw.
///
/// [pt] O corpo da 0x51c020: `Some(lado)`, ou `None` no empate também nos pênaltis com a regra deles (a original mostra a
/// caixa "Error" com a linha 0x97e e segue para W.O. e pontos): quem chama roda a original inteira, que refaz o único
/// que já foi gravado (+0x3d/+0x3e = 1 com o bit 0x10, idempotente).
pub(crate) unsafe fn fixture_winner_side(this: usize, f: usize) -> Option<u32> {
    let fl = read_i16(f + 0x38) as u16;
    if fl & 0x10 != 0 {
        write_i8(f + 0x3d, 1);
        write_i8(f + 0x3e, 1);
    }
    let (g1, g2) = (read_i8(f + 0x43), read_i8(f + 0x44));
    if g1 > g2 {
        if read_i8(f + 0x3d) == 1 {
            write_i8(f + 0x3d, 2);
        }
        if fl & 0x2000 != 0 {
            write_i8(f + 0x3d, read_i8(f + 0x3d).wrapping_sub(3));
        }
        return Some(1);
    }
    if g1 < g2 {
        if read_i8(f + 0x3e) == 1 {
            write_i8(f + 0x3e, 2);
        }
        if fl & 0x2000 != 0 {
            write_i8(f + 0x3e, read_i8(f + 0x3e).wrapping_sub(3));
        }
        return Some(2);
    }
    if fl & 2 != 0 {
        let (a1, a2) = (read_i8(f + 0x47), read_i8(f + 0x48));
        if a1 > a2 {
            crate::comparacao::ramo("cpm_1 jogo na prorrogacao");
            let c = read_i8(f + 0x3d);
            if c == 1 {
                write_i8(f + 0x3d, 2);
            } else if fl & 0x2000 != 0 {
                write_i8(f + 0x3d, c.wrapping_sub(3));
            } else {
                write_i8(f + 0x3d, 3);
            }
            return Some(1); // o mandante na prorrogação
        }
        if a1 < a2 {
            let c = read_i8(f + 0x3e);
            if c == 1 {
                write_i8(f + 0x3e, 2);
            } else if fl & 0x2000 != 0 {
                write_i8(f + 0x3e, c.wrapping_sub(3));
            } else {
                write_i8(f + 0x3e, 3);
            }
            return Some(2); // o visitante na prorrogação
        }
    }
    if fl & 0x21 != 0 {
        let (p1, p2) = (read_i8(f + 0x49), read_i8(f + 0x4a));
        // o código dos pênaltis: 4, ou 5 sem o bit 1 da regra (not al; and al, 1; or al, 4)
        let penalties = ((!(fl as u8)) & 1 | 4) as i8;
        if p1 > p2 {
            let c = read_i8(f + 0x3d);
            if c == 1 {
                write_i8(f + 0x3d, 2);
            } else if fl & 0x2000 != 0 {
                write_i8(f + 0x3d, c.wrapping_sub(3));
            } else {
                write_i8(f + 0x3d, penalties);
            }
            return Some(1);
        }
        if p1 < p2 {
            let c = read_i8(f + 0x3e);
            if c == 1 {
                write_i8(f + 0x3e, 2);
            } else if fl & 0x2000 != 0 {
                write_i8(f + 0x3e, c.wrapping_sub(3));
            } else {
                write_i8(f + 0x3e, penalties);
            }
            return Some(2);
        }
        return None; // empate nos pênaltis: a caixa da original
    }
    if fl & 0x800 != 0 {
        write_i8(f + 0x3d, 8); // W.O. a favor do mandante
        return Some(1);
    }
    if fl & 0x1000 != 0 {
        write_i8(f + 0x3e, 8);
        return Some(2);
    }
    if fl & 0x2000 == 0 {
        return Some(0);
    }
    let a = read_i8(f + 0x3d).wrapping_sub(1);
    let c = read_i8(f + 0x3e).wrapping_sub(1);
    write_i8(f + 0x3d, a);
    write_i8(f + 0x3e, c);
    if a < c {
        return Some(1);
    }
    if a > c {
        return Some(2);
    }
    // empate nos pontos: o cabeça de chave menor (byte +4 da entrada de 6 bytes [this+0xa7], +0x3e entradas)
    let n = read_i16(this + 0x3e);
    if n <= 0 {
        return Some(2);
    }
    let (c1, c2) = (read_usize(f + 0x1c), read_usize(f + 0x20));
    let (mut s1, mut s2) = (0i8, 0i8);
    let t = read_usize(this + 0xa7);
    for k in 0..n as usize {
        let e = t + 6 * k;
        if read_usize(e) == c1 {
            s1 = read_i8(e + 4);
        }
        if read_usize(e) == c2 {
            s2 = read_i8(e + 4);
        }
    }
    Some(if s1 < s2 { 1 } else { 2 })
}

/// who goes through in a cup match (1 home, 2 away, 0 nobody) by the score, extra time, penalties, walkover and points
/// of the rule (+0x38), storing each side's code (+0x3d/+0x3e)
/// Original: `Cup::decide_fixture_winner` (0x51c020, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x51c020 } (thiscall, ret 4, Cup.cpp): `Cup::decide_fixture_winner(jogo)`: quem passa num jogo de copa (1
/// mandante, 2 visitante, 0 ninguém) pela regra (+0x38): o placar (+0x43/+0x44), a prorrogação (bit 2, +0x47/+0x48), os
/// pênaltis (bits 0x21, +0x49/+0x4a), o W.O. (0x800/0x1000) e os pontos (0x2000: os códigos −1 e o cabeça de chave),
/// gravando o código de cada lado (+0x3d/+0x3e: 2 venceu, 3 na prorrogação, 4/5 nos pênaltis, 8 W.O.; −3 com 0x2000).
/// Jogo nulo ou empate nos pênaltis: a original (caixa "Error").
pub(crate) unsafe extern "thiscall" fn decide_fixture_winner(this: usize, jogo: usize) -> u32 {
    if jogo != 0 {
        if let Some(r) = fixture_winner_side(this, jogo) {
            return r;
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_DECIDE_FIXTURE_WINNER)(this, jogo)
}

/// Body of 0x51f2f0: the winning side of a tie over its legs, or None on an unresolved penalty draw.
///
/// [pt] O corpo da 0x51f2f0: `None` no empate nos pênaltis com a regra deles (a original mostra a caixa "Error" com a linha
/// 0x11d1 e devolve 4; nada foi gravado antes).
pub(crate) unsafe fn tie_winner_side(conf: usize, j: usize) -> Option<u32> {
    let t = |k: usize| read_i8(conf + k);
    let fl = read_i16(j + 0x38) as u16;
    if t(0x33) as u8 != 1 {
        // primeiro jogo (ou jogo único) do confronto: 3
        if fl & 0x10 != 0 {
            let lado1 = if t(0x14) != t(0x15) {
                t(0x14) > t(0x15)
            } else if t(0x18) != t(0x19) {
                t(0x18) > t(0x19)
            } else {
                t(0x1a) > t(0x1b)
            };
            write_i8(j + if lado1 { 0x3d } else { 0x3e }, 1);
        } else if fl & 0x2000 != 0 {
            let pares = [(0x14, 0x15), (0x18, 0x19), (0x1a, 0x1b)];
            match pares.iter().find(|p| t(p.0) != t(p.1)) {
                Some(&(a, b)) => {
                    let lado = if t(a) > t(b) { 0x3d } else { 0x3e };
                    write_i8(j + lado, read_i8(j + lado).wrapping_sub(3));
                }
                None => {
                    write_i8(j + 0x3d, read_i8(j + 0x3d).wrapping_sub(1));
                    write_i8(j + 0x3e, read_i8(j + 0x3e).wrapping_sub(1));
                }
            }
        }
        return Some(3);
    }
    // o jogo de volta: o agregado (os gols +0x14 e +0x1f contra +0x1e e +0x15, em 16 bits)
    let tot1 = (t(0x14) as i16).wrapping_add(t(0x1f) as i16);
    let tot2 = (t(0x1e) as i16).wrapping_add(t(0x15) as i16);
    let volta2 = t(0x15) as i16;
    if fl & 0x10 != 0 {
        // pontos: quem venceu cada jogo (placar, prorrogação, pênaltis); um de cada: 4
        let lado_volta = if t(0x14) != t(0x15) {
            t(0x14) > t(0x15)
        } else if t(0x18) != t(0x19) {
            t(0x18) > t(0x19)
        } else {
            t(0x1a) > t(0x1b)
        };
        let lado_ida = if t(0x1e) != t(0x1f) {
            t(0x1e) > t(0x1f)
        } else if t(0x22) != t(0x23) {
            t(0x22) > t(0x23)
        } else {
            t(0x24) > t(0x25)
        };
        if lado_volta == lado_ida {
            write_i8(j + 0x3d, 1);
            write_i8(j + 0x3e, 1);
            return Some(4);
        }
        if lado_volta {
            write_i8(j + 0x3e, 2);
            return Some(2);
        }
        write_i8(j + 0x3d, 2);
        return Some(1);
    }
    if fl & 0x2000 != 0 {
        let pares = [(0x43, 0x44), (0x47, 0x48), (0x49, 0x4a)];
        match pares.iter().find(|p| read_i8(j + p.0) != read_i8(j + p.1)) {
            Some(&(a, b)) => {
                let lado = if read_i8(j + a) > read_i8(j + b) { 0x3d } else { 0x3e };
                write_i8(j + lado, read_i8(j + lado).wrapping_sub(3));
            }
            None => {
                write_i8(j + 0x3d, read_i8(j + 0x3d).wrapping_sub(1));
                write_i8(j + 0x3e, read_i8(j + 0x3e).wrapping_sub(1));
            }
        }
        if read_i8(j + 0x3d) < -5 {
            return Some(1);
        }
        return Some(if read_i8(j + 0x3e) >= -5 { 4 } else { 2 });
    }
    if tot1 > tot2 {
        return Some(2); // agregado
    }
    if tot1 < tot2 {
        return Some(1);
    }
    crate::comparacao::ramo("cpm_1 confronto empatado no agregado");
    if fl & 0x800 != 0 {
        write_i8(j + 0x3d, 8);
        return Some(1);
    }
    if fl & 0x1000 != 0 {
        write_i8(j + 0x3e, 8);
        return Some(2);
    }
    if fl & 2 != 0 && t(0x22) as u8 != 0xff {
        if fl & 0x4000 != 0 && (t(0x22) as i32 + t(0x15) as i32) == (t(0x23) as i32 + t(0x14) as i32) {
            if t(0x23) > t(0x15) {
                write_i8(j + 0x3e, 6);
                return Some(2);
            }
            if t(0x23) < t(0x15) {
                write_i8(j + 0x3d, 6);
                return Some(1);
            }
        }
        let x = t(0x23) as i32 - t(0x1f) as i32;
        let y = t(0x22) as i32 - t(0x1e) as i32;
        if y > x {
            write_i8(j + 0x3d, 3);
            return Some(1);
        }
        if y < x {
            write_i8(j + 0x3e, 3);
            return Some(2);
        }
    }
    if fl & 4 == 0 && fl & 0x4000 == 0 {
        // os gols fora (o visitante da volta, +0x15, contra o da ida, +0x1f)
        let fora = t(0x1f) as i16;
        if volta2 < fora {
            write_i8(j + 0x3e, 6);
            return Some(2);
        }
        if volta2 > fora {
            write_i8(j + 0x3d, 6);
            return Some(1);
        }
    }
    if fl & 0x21 == 0 {
        return Some(4);
    }
    let (p1, p2) = (t(0x24), t(0x25));
    let codigo = if fl & 1 != 0 { 4 } else { 5 };
    if p2 < p1 {
        write_i8(j + 0x3d, codigo);
        return Some(1);
    }
    if p2 > p1 {
        write_i8(j + 0x3e, codigo);
        return Some(2);
    }
    None
}

/// in the first leg marks the winner and returns 3; in the second leg decides by aggregate, walkover, extra time, away
/// goals and penalties (1 or 2 the side through, 4 undecided)
/// Original: `Cup::decide_tie_winner` (0x51f2f0, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x51f2f0 } (thiscall, ret 8, Cup.cpp; `this` não é lido): `Cup::decide_tie_winner(confronto, jogo)`: no primeiro
/// jogo (+0x33 ≠ 1), só marca o vencedor (pontos 0x10: código 1; 0x2000: −3, ou −1 nos dois) e devolve 3. Na volta,
/// pelo agregado (+0x14 + +0x1f contra +0x1e + +0x15): 2 o primeiro lado à frente, 1 o segundo; empatado: W.O.
/// (0x800/0x1000, código 8), a prorrogação (bit 2 com +0x22 ≠ 0xff: com 0x4000 os gols fora dela; código 6, senão a
/// diferença da prorrogação, código 3), os gols fora (sem 4 nem 0x4000, código 6) e os pênaltis (0x21, +0x24/+0x25:
/// código 4 com o bit 1, senão 5); nada decidido: 4. Pontos (0x10) e regra 0x2000 têm os ramos próprios. Empate nos
/// pênaltis: a original (caixa "Error", 4).
pub(crate) unsafe extern "thiscall" fn decide_tie_winner(this: usize, confronto: usize, jogo: usize) -> u32 {
    match tie_winner_side(confronto, jogo) {
        Some(r) => r,
        None => original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_DECIDE_TIE_WINNER)(this, confronto, jogo),
    }
}

/// the qsort comparison of the 7-byte draw entries: the higher seed (+6) first
/// Original: `cup_compare_draw_entries_by_seed` (0x51ebc0, Cup.cpp, cdecl).
///
/// [pt] { 0x51ebc0 } (cdecl, Cup.cpp): `cup_compare_draw_entries_by_seed(a, b)`: a comparação do qsort das entradas de 7
/// bytes do sorteio (0x51d2d0 e 0x51d9f0): o cabeça de chave (byte com sinal +6) de `b` menos o de `a` (maior primeiro).
pub(crate) unsafe extern "C" fn cup_compare_draw_entries_by_seed(a: usize, b: usize) -> i32 {
    crate::comparacao::ramo("cpm_1 ordem dos cabecas de chave");
    read_i8(b + 6) as i32 - read_i8(a + 6) as i32 // o maior primeiro
}

/// Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; jogo nulo ou empate nos pênaltis (a
/// caixa): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_decide_fixture_winner(this: usize, jogo: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_DECIDE_FIXTURE_WINNER);
    if jogo == 0 {
        return o(this, jogo);
    }
    let reg = [(jogo + 0x3d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || Some(o(this, jogo)), || fixture_winner_side(this, jogo));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        PLACAR.registrar(igual && r == n, || format!("decide_fixture_winner(regra {:x}) original={} rust={} {:x?} x {:x?}", read_i16(jogo + 0x38) as u16, r, n, mo, mr));
    }
    r
}

/// Comparador: lado a lado pelos códigos (+0x3d/+0x3e) e pelo lado devolvido; empate nos pênaltis (a caixa): só a
/// original.
pub(crate) unsafe extern "thiscall" fn comparar_decide_tie_winner(this: usize, confronto: usize, jogo: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(IDX_DECIDE_TIE_WINNER);
    let reg = [(jogo + 0x3d, 2)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || Some(o(this, confronto, jogo)), || tie_winner_side(confronto, jogo));
    let r = r.unwrap_or(0);
    if let Some(n) = n {
        PLACAR.registrar(igual && r == n, || format!("decide_tie_winner(regra {:x}, jogo {}) original={} rust={} {:x?} x {:x?}",
            read_i16(jogo + 0x38) as u16, read_i8(confronto + 0x33), r, n, mo, mr));
    }
    r
}

/// Comparador: pelo valor.
pub(crate) unsafe extern "C" fn comparar_cup_compare_draw_entries_by_seed(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(IDX_CUP_COMPARE_DRAW_ENTRIES_BY_SEED)(a, b);
    let n = cup_compare_draw_entries_by_seed(a, b);
    PLACAR.registrar(r == n, || format!("cup_compare_draw_entries_by_seed({}, {}) original={} rust={}", read_i8(a + 6), read_i8(b + 6), r, n));
    r
}

// ---------- 213a leva: as datas do sorteio e do jogo da rodada da copa e o clube que sai dos amistosos planejados (Cup.cpp, friendly.cpp) ----------

/// Date from day/month/year moved to weekday semana (-1 any) via Date::Date 0x549f70 → (day of year, year low byte).
///
/// [pt] A data (Date::Date, 0x549f70, num Date local de 8 bytes) de `dia`/`mes`/`ano`, levada ao dia da
/// semana `semana` (−1 = qualquer): (palavra +0 = dia do ano, byte baixo do ano +2).
pub(crate) unsafe fn date_on_weekday(dia: u32, mes: u32, ano: u32, semana: u32) -> (u16, u8) {
    let construir: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut d = [0u32; 2];
    construir(&mut d, dia, mes, ano, semana);
    (d[0] as u16, (d[0] >> 16) as u8)
}

/// Address of the 104-byte record of round rodada (signed word) from rodadas.
///
/// [pt] O registro de 104 bytes da rodada `rodada` (palavra com sinal) a partir de `rodadas`.
pub(crate) fn round_record_from_base(rodadas: usize, rodada: u32) -> usize {
    rodadas.wrapping_add((rodada as i16 as isize * 104) as usize)
}

/// stores the cup round draw date: the day number and the year offset from the base year
/// Original: `cup_set_round_draw_date` (0x521e60, Cup.cpp, cdecl).
///
/// [pt] { 0x521e60 } (cdecl, Cup.cpp): a data do sorteio da rodada: dia do ano na palavra +0 e ano − ano-base
/// (byte) em +2, da data de `dia`/`mes`/ano-base + `anos` levada ao dia da semana. Devolve o registro.
pub(crate) unsafe extern "C" fn cup_set_round_draw_date(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, base: u32) -> usize {
    let (d, a) = date_on_weekday(dia, mes, base.wrapping_add(anos), semana);
    let r = round_record_from_base(rodadas, rodada);
    core::ptr::write_unaligned(r as *mut u16, d);
    *((r + 2) as *mut u8) = a.wrapping_sub(base as u8); // ano do sorteio
    r
}

/// stores the cup round match date, its match time and the venue flags
/// Original: `cup_set_round_match_date` (0x521eb0, Cup.cpp, cdecl).
///
/// [pt] { 0x521eb0 } (cdecl, Cup.cpp): a data do jogo da rodada, como a do sorteio, em +3 (dia) e +5 (ano −
/// ano-base); o horário (byte) em +6 e as marcas do mando em +0x13. Devolve o registro.
pub(crate) unsafe extern "C" fn cup_set_round_match_date(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, hora: u32, base: u32, mando: u32) -> usize {
    let (d, a) = date_on_weekday(dia, mes, base.wrapping_add(anos), semana);
    let r = round_record_from_base(rodadas, rodada);
    core::ptr::write_unaligned((r + 3) as *mut u16, d); // dia do jogo
    *((r + 5) as *mut u8) = a.wrapping_sub(base as u8);
    *((r + 6) as *mut u8) = hora as u8;
    core::ptr::write_unaligned((r + 0x13) as *mut u32, mando);
    r
}


pub(crate) unsafe extern "C" fn comparar_cup_set_round_draw_date(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, base: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize>(IDX_CUP_SET_ROUND_DRAW_DATE);
    let r = round_record_from_base(rodadas, rodada);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 3)], || o(rodadas, rodada, dia, mes, anos, semana, base),
        || cup_set_round_draw_date(rodadas, rodada, dia, mes, anos, semana, base));
    PLACAR.registrar(igual && a == b, || format!("cup_set_round_draw_date({}, {}/{} +{}) {:x?} x {:x?}", rodada as i16, dia, mes, anos, mo, mr));
    a
}

pub(crate) unsafe extern "C" fn comparar_cup_set_round_match_date(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, hora: u32, base: u32, mando: u32) -> usize {
    let o = original_fn::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(IDX_CUP_SET_ROUND_MATCH_DATE);
    let r = round_record_from_base(rodadas, rodada);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r + 3, 0x14)], || o(rodadas, rodada, dia, mes, anos, semana, hora, base, mando),
        || cup_set_round_match_date(rodadas, rodada, dia, mes, anos, semana, hora, base, mando));
    PLACAR.registrar(igual && a == b, || format!("cup_set_round_match_date({}, {}/{} +{}) {:x?} x {:x?}", rodada as i16, dia, mes, anos, mo, mr));
    a
}


// ---------- 210a leva: as regras da rodada no jogo da copa, a chave do jogo e o país da excursão (Cup.cpp, friendly.cpp) ----------

/// Address of cup round r: 104-byte record in [cup + 0xa3].
///
/// [pt] A rodada `r` da copa: registro de 104 bytes em [copa+0xa3].
pub(crate) unsafe fn cup_cup_round(copa: usize, r: i32) -> usize {
    (*((copa + 0xa3) as *const usize) as isize + r as isize * 104) as usize
}

/// Address of bracket row k of round r: [[cup + 0xad] + 4r] + 53k.
///
/// [pt] A linha da chave `k` da rodada `r` da copa: [[copa+0xad] + 4·r] + 53·k.
pub(crate) unsafe fn bracket_row(copa: usize, r: i32, k: i32) -> usize {
    (*((*((copa + 0xad) as *const usize) as isize + r as isize * 4) as *const usize) as isize + k as isize * 53) as usize
}

/// Address of the cup's 6-byte team slot v: [cup + 0xa7] + 6v.
///
/// [pt] O registro de 6 bytes da vaga `v` da copa: [copa+0xa7] + 6·v.
pub(crate) unsafe fn team_slot(copa: usize, v: i16) -> usize {
    (*((copa + 0xa7) as *const usize) as isize + v as isize * 6) as usize
}

/// copies the round rules onto the match and, with a decision on a draw, marks who goes through by both sides' levels
/// Original: `Cup::apply_round_rules_to_match` (0x521d60, Cup.cpp, thiscall).
///
/// [pt] { 0x521d60 } (thiscall, ret 8, Cup.cpp): passa as regras da rodada do jogo (palavra +0xf da rodada
/// [jogo+0x34]) para as marcas do jogo (+0x38, OU). Com a marca 0x800 (decide quem passa no empate), o
/// nível (byte +4 da vaga, com sinal) dos dois lados da chave `chave` decide: mandante menor tira 0x800 e
/// põe 0x1000; maior mantém 0x800; igual tira 0x800 e põe 1.
pub(crate) unsafe extern "thiscall" fn apply_round_rules_to_match(copa: usize, jogo: usize, chave: i32) {
    let r = *((jogo + 0x34) as *const i16) as i32;
    let marcas = (jogo + 0x38) as *mut u16;
    let m = core::ptr::read_unaligned(marcas) | core::ptr::read_unaligned((cup_cup_round(copa, r) + 0xf) as *const u16);
    core::ptr::write_unaligned(marcas, m);
    if m & 0x800 == 0 {
        return;
    }
    let linha = bracket_row(copa, r, chave);
    let nivel = |o: usize| *((team_slot(copa, core::ptr::read_unaligned((linha + o) as *const i16)) + 4) as *const i8);
    let (a, b) = (nivel(0), nivel(2));
    let m = if a < b { (m ^ 0x800) | 0x1000 } else if a > b { m | 0x800 } else { (m ^ 0x800) | 1 };
    core::ptr::write_unaligned(marcas, m);
}

/// the round tie that holds the match's home club (-1 if none)
/// Original: `Cup::find_tree_slot_for_fixture` (0x519c90, Cup.cpp, thiscall).
///
/// [pt] { 0x519c90 } (thiscall, ret 4, Cup.cpp): a chave da rodada do jogo (+0x34) em que está o clube
/// mandante do jogo (+0xc), olhando os dois lados de cada chave (53 bytes; vaga negativa é vazia); -1 se
/// nenhuma. O número de chaves é a palavra +0x1a da rodada. Jogo nulo: a original (caixa de erro).
pub(crate) unsafe extern "thiscall" fn find_tree_slot_for_fixture(copa: usize, jogo: usize) -> i32 {
    if jogo == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_FIND_TREE_SLOT_FOR_FIXTURE)(copa, jogo);
    }
    let r = *((jogo + 0x34) as *const i16) as i32;
    let n = core::ptr::read_unaligned((cup_cup_round(copa, r) + 0x1a) as *const i16) as i32;
    let mandante = *((jogo + 0xc) as *const usize);
    for i in 0..n {
        let linha = bracket_row(copa, r, i);
        for o in [0, 2] {
            let v = core::ptr::read_unaligned((linha + o) as *const i16);
            if v >= 0 && *(core::ptr::read_unaligned(team_slot(copa, v) as *const usize) as *const usize) == mandante {
                return i;
            }
        }
    }
    -1
}


pub(crate) unsafe extern "thiscall" fn comparar_apply_round_rules_to_match(copa: usize, jogo: usize, chave: i32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, i32)>(IDX_APPLY_ROUND_RULES_TO_MATCH);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(jogo + 0x38, 2)], || o(copa, jogo, chave),
        || apply_round_rules_to_match(copa, jogo, chave));
    PLACAR.registrar(igual, || format!("apply_round_rules_to_match(chave {}) {:x?} x {:x?}", chave, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_find_tree_slot_for_fixture(copa: usize, jogo: usize) -> i32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i32>(IDX_FIND_TREE_SLOT_FOR_FIXTURE)(copa, jogo);
    if jogo == 0 {
        return r;
    }
    let n = find_tree_slot_for_fixture(copa, jogo);
    PLACAR.registrar(r == n, || format!("find_tree_slot_for_fixture original={} rust={}", r, n));
    r
}

/// the cup match importance by round and the first-leg score difference
/// Original: `cup_match_importance` (0x5221f0, Cup.cpp, thiscall).
///
/// [pt] { 0x5221f0 } (thiscall, ret 0xc, Cup.cpp): a importância da partida da copa (a fase +0x34 da
/// partida menos +0x3c mais +0x51 da copa, pelo menos 1) e em `margem` a diferença do placar da ida
/// (+0x4b/+0x4c, 0xff sem ida) para o lado `segundo`, mais 1 (0xff abaixo de 1). Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn cup_match_importance(copa: usize, partida: usize, segundo: u8, margem: *mut u8) -> u8 {
    let b = |p: usize, o: usize| *((p + o) as *const u8);
    let mut al = b(partida, 0x34).wrapping_sub(b(copa, 0x3c)).wrapping_add(b(copa, 0x51));
    if (al as i8) < 1 {
        al = 1;
    }
    let ida = b(partida, 0x4b);
    if ida != 0xff {
        let volta = b(partida, 0x4c);
        *margem = if segundo == 0 { volta.wrapping_sub(ida) } else { ida.wrapping_sub(volta) }.wrapping_add(1);
        if (*margem as i8) < 1 {
            *margem = 0xff;
        }
    }
    al
}

pub(crate) unsafe extern "thiscall" fn comparar_cup_match_importance(copa: usize, partida: usize, segundo: u8, margem: *mut u8) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u8, *mut u8) -> u8>(IDX_CUP_MATCH_IMPORTANCE);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(margem as usize, 1)], || o(copa, partida, segundo, margem), || cup_match_importance(copa, partida, segundo, margem));
    PLACAR.registrar(igual && r == n, || format!("cup_match_importance(lado {}) original={} rust={} {:x?} x {:x?}", segundo, r, n, mo, mr));
    r
}

/// outside friendlies, the competition virtual +0x44 and news item 0x10 (club, competition, outcome, result category,
/// the match, the next round), delivered by scope, to the competition region (final and semi-final) or to the club
/// manager
/// Original: `Cup::add_team_result_news` (0x519690, Cup.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x519690 } (thiscall, ret 0x14, Cup.cpp, quadro SEH sem destrutor): `Cup::add_team_result_news(clube, desfecho,
/// a3, partida, a5)`: amistoso (0x5c6710), 0. A virtual +0x44 da competição ([0xadadfc] pelo id de +4) com o clube, o
/// desfecho, a3, a saída (0xff), a partida e a5; −1 dela ou sem partida: 0. A notícia 0x10 (local): 0 o clube, 1 a
/// competição, 2 o desfecho, 3 a categoria do resultado (0x4b0890), 4 o que a virtual devolveu, o jogo a partir do
/// dado 5 (0x4afeb0, que avança o cursor) e no cursor a rodada seguinte (+7 da rodada +0x34 + 1, até +0x3c − 1; 0x8c
/// ou fora: −1). Saída ≠ 0xff: a entrega pelo alcance (0x786090). Desfecho 1 na rodada 0x96 ou 3 na 0x8c (+0x30): à
/// região da competição (0x786860); senão ao técnico do clube (0x7861e0). Devolve 1.
pub(crate) unsafe extern "thiscall" fn add_team_result_news(copa: usize, clube: usize, desfecho: u32, a3: u32, p: usize, a5: u32) -> u32 {
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let categoria: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_UTIL_CLASSIFY_TIE_RESULT_FOR_NEWS);
    let save_game_data: unsafe extern "C" fn(usize, usize, *mut u8) -> u32 = core::mem::transmute(FN_COMP_UTIL_WRITE_FIXTURE_TO_NEWS_SLOTS);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let a_regiao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_COMP_REGION);
    let pelo_alcance: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_NEWS_SEND_BY_SCOPE);
    // ponytail: a original grava só o byte da saída num dword da pilha e empurra o dword; a 0x786090 lê o byte
    let mut saida: u32 = 0xff;
    if amistoso(read_usize(G_FRIENDLY), read_usize(copa + 4)) != 0 {
        return 0;
    }
    let comp = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_usize(read_usize(copa + 4)).wrapping_mul(4)));
    let virtual44: unsafe extern "thiscall" fn(usize, usize, u32, u32, *mut u32, usize, u32) -> i32 = core::mem::transmute(read_usize(read_usize(comp) + 0x44));
    let r = virtual44(comp, clube, desfecho, a3, &mut saida, p, a5);
    if r == -1 || p == 0 {
        return 0;
    }
    crate::comparacao::ramo("cpx_13 noticia do desfecho");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x10, 0);
    gravar(n, 0, read_usize(clube) as u32);
    gravar(n, 1, read_usize(read_usize(copa + 4)) as u32);
    gravar(n, 2, desfecho as u8 as i8 as i32 as u32);
    gravar(n, 3, categoria(clube, p) as i8 as i32 as u32);
    gravar(n, 4, r as u32);
    // ponytail: o cursor é um dword da pilha com 5 no byte baixo (o resto é sobra); só o byte baixo conta
    let mut cursor = [5u8, 0, 0, 0];
    save_game_data(p, n, cursor.as_mut_ptr());
    let k = read_i16(p + 0x34) as i32;
    let proxima = if k < read_i16(copa + 0x3c) as i32 - 1 {
        let v = read_i16(read_usize(copa + 0xa3).wrapping_add((k as usize).wrapping_mul(0x68)) + 0x6f);
        if v != 0x8c { v as i32 } else { -1 }
    } else {
        -1
    };
    gravar(n, cursor[0] as i8, proxima as u32);
    let noticias = read_usize(G_NEWS);
    if saida as u8 != 0xff {
        let nacao = read_usize(clube + club::NATION);
        let continente = if nacao != 0 { read_usize(nacao + nation::CONTINENT) } else { 0 };
        pelo_alcance(noticias, n, clube, read_usize(copa + 4), nacao, continente, saida);
        return 1;
    }
    let rodada = read_i16(p + 0x30);
    if (desfecho as u8 == 1 && rodada == 0x96) || (desfecho as u8 == 3 && rodada == 0x8c) {
        a_regiao(noticias, n, read_usize(copa + 4));
    } else {
        ao_clube(noticias, n, clube);
    }
    1
}

pub(crate) unsafe extern "thiscall" fn comparar_add_team_result_news(copa: usize, clube: usize, desfecho: u32, a3: u32, p: usize, a5: u32) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32) -> u32>(IDX_ADD_TEAM_RESULT_NEWS)(copa, clube, desfecho, a3, p, a5)
}

/// the cup started (+0xb1) and, asking for the stages with no stage set (+0x44), all up to the current complete and the
/// current is the last
/// Original: `Cup::check_complete` (0x521e00, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x521e00 } (thiscall, ret 8, Cup.cpp): `Cup::check_complete(fases, _)`: 0 se a copa não começou (+0xb1). Com
/// `fases` = 1 e sem fase marcada (+0x44 = 0xff): cada fase de 0 até a atual (+0x30, vetor +0xc) tem de estar completa
/// (método +0x10 com 0, 1) e a atual tem de ser a última (+0x2c − 1); senão 0. Fora isso, 1.
pub(crate) unsafe extern "thiscall" fn cup_check_complete(this: usize, fases: u32, _x: u32) -> u8 {
    if *((this + 0xb1) as *const u8) == 0 {
        return 0;
    }
    if fases != 1 || *((this + 0x44) as *const u8) != 0xff {
        return 1;
    }
    let mut i = 0i32;
    while i <= read_usize(this + 0x30) as i32 {
        let fase = read_usize(read_usize(this + 0xc).wrapping_add(i as usize * 4));
        let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(read_usize(read_usize(fase) + 0x10));
        if completa(fase, 0, 1) == 0 {
            return 0; // uma fase incompleta
        }
        i += 1;
    }
    (read_usize(this + 0x30) as i32 >= (read_usize(this + 0x2c) as i32).wrapping_sub(1)) as u8
}

/// Comparador: pelo valor (o método das fases só lê).
pub(crate) unsafe extern "thiscall" fn comparar_cup_check_complete(this: usize, fases: u32, x: u32) -> u8 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(IDX_CUP_CHECK_COMPLETE)(this, fases, x);
    let n = cup_check_complete(this, fases, x);
    PLACAR.registrar(r == n, || format!("cup_check_complete({:x}, {}) original={} rust={}", this, fases, r, n));
    r
}

/// in rounds with set dates, each round fixture moves to the weekday and time slot of its slot, with the news, the
/// stadium and the calendar; the used slot becomes empty
/// Original: `Cup::move_round_fixtures_to_set_dates` (0x51ebe0, Cup.cpp, thiscall, ret 8).
///
/// [pt] { 0x51ebe0 } (thiscall, ret 8, Cup.cpp, quadro SEH sem destrutor no caminho normal: as notícias, a data e a lista
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
pub(crate) unsafe extern "thiscall" fn move_round_fixtures_to_set_dates(this: usize, data: usize, rodada: u32) {
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let mut noticia = [0u32; 0x38];
    let n1 = noticia.as_mut_ptr() as usize;
    construir(n1, 0x14, 0);
    let id = read_u32_as_usize(read_u32_as_usize(this + 4));
    let e = cup_round_entry(this, rodada);
    if read_i16(e + 0xd) as u16 & 0x100 == 0 || read_u8(e + 0x29) == 0xff {
        return;
    }
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    if collect_round_fixtures(this, data, rodada, l) != 0 {
        gravar(n1, 0, read_u32_as_usize(read_u32_as_usize(this + 4)) as u32);
        gravar(n1, 1, read_i16(this + 0x3a) as i32 as u32);
        gravar(n1, 2, read_i16(cup_round_entry(this, rodada) + 7) as i32 as u32);
        gravar(n1, 5, (read_u8(cup_round_entry(this, rodada) + 0x21) == 2) as u32);
        // a notícia 0x14 do jogo (dados 3, 4, 6, 7, 8) aos dois clubes, se um deles é de técnico humano
        let noticia_do_jogo = |c1: usize, c2: usize, i1: usize, i2: usize, est: usize, dia: u16, ano: u16| {
            if cup_has_human_manager(c1) || cup_has_human_manager(c2) {
                if cup_has_human_manager(c1) {
                    gravar(n1, 3, i1 as u32);
                    gravar(n1, 4, i2 as u32);
                } else {
                    gravar(n1, 3, i2 as u32);
                    gravar(n1, 4, i1 as u32);
                }
                gravar(n1, 6, est as u32);
                gravar(n1, 7, dia as u32);
                gravar(n1, 8, ano as u32);
                entregar(read_u32_as_usize(G_NEWS), n1, c1);
                entregar(read_u32_as_usize(G_NEWS), n1, c2);
            }
        };
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        let mut j = 0usize;
        let mut f = proximo(l);
        while f != 0 {
            if read_u8(this + 0x44) == read_u8(f + 0x42) {
                let e = cup_round_entry(this, rodada);
                if j < 0x38 && read_u8(e + j + 0x29) != 0xff {
                    let mut buf = [0u8; 0x50];
                    let r = buf.as_mut_ptr() as usize;
                    core::ptr::copy_nonoverlapping(f as *const u8, r as *mut u8, 0x4f);
                    if fixture_on_round_date(this, e, f) {
                        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
                        let nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
                        let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
                        let ajustar: unsafe extern "thiscall" fn(usize, u32, usize) -> u32 = core::mem::transmute(FN_DATE_GET_NEAREST_WEEKDAY);
                        let p = alocar(8);
                        let dp = if p != 0 { nova(p, read_i16(f + 0x2a) as u16 as u32, read_i16(f + 0x28) as u16 as u32) } else { 0 };
                        let mut t = [0u8; 8];
                        let pt = t.as_mut_ptr() as usize;
                        padrao(pt);
                        let e = cup_round_entry(this, rodada);
                        ajustar(dp, read_u8(e + j + 0x27) as i8 as i16 as u16 as u32, pt); // cqc7: o dia da semana marcado
                        if read_u8(cup_round_entry(this, rodada) + j + 0x29) != 3 {
                            let mut noticia2 = [0u32; 0x38];
                            let n2 = noticia2.as_mut_ptr() as usize;
                            construir(n2, 3, 0);
                            gravar(n2, 0, read_u32_as_usize(r + 0xc) as u32);
                            gravar(n2, 1, read_u32_as_usize(r + 0x10) as u32);
                            gravar(n2, 2, read_u32_as_usize(r) as u32);
                            gravar(n2, 3, read_i16(r + 0x32) as i32 as u32);
                            gravar(n2, 4, read_i16(r + 0x30) as i32 as u32);
                            gravar(n2, 5, read_i16(r + 0x2a) as u16 as u32);
                            gravar(n2, 6, read_i16(r + 0x28) as u16 as u32);
                            gravar(n2, 7, read_u8(r + 0x3f) as u32);
                            gravar(n2, 8, read_i16(pt) as i32 as u32);
                            gravar(n2, 9, read_i16(pt + 2) as u16 as u32);
                            gravar(n2, 10, read_u8(cup_round_entry(this, rodada) + j + 0x28) as i8 as i32 as u32);
                            gravar(n2, 11, read_u8(cup_round_entry(this, rodada) + j + 0x29) as i8 as i32 as u32);
                            entregar(read_u32_as_usize(G_NEWS), n2, read_u32_as_usize(r + 0x1c));
                            entregar(read_u32_as_usize(G_NEWS), n2, read_u32_as_usize(r + 0x20));
                        }
                        let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_REMOVE_FIXTURE);
                        tirar(read_u32_as_usize(G_FIXTURE_MANAGER), r);
                        crate::comparacao::ramo("cqc_7 jogo levado a data");
                        write_u16(r + 0x2a, read_i16(pt) as u16); // cqc7: o dia marcado
                        write_u16(r + 0x28, read_i16(pt + 2) as u16);
                        let v = cup_round_entry(this, rodada) + j;
                        write_u8(r + 0x3f, read_u8(v + 0x28)); // cqc7: a fatia marcada
                        write_u8(r + 0x3b, read_u8(v + 0x29));
                        let estadio: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_STADIUM_SET_FIXTURE_VENUE);
                        estadio(read_u32_as_usize(G_STADIUM), read_u32_as_usize(cup_round_entry(this, rodada) + j + 0x23), r, read_u32_as_usize(this + 0x24));
                        let juntar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_ADD_FIXTURE);
                        juntar(read_u32_as_usize(G_FIXTURE_MANAGER), r, 1);
                        if dp != 0 {
                            let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
                            liberar(dp);
                        }
                        if read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + id * 4) == 0 {
                            destruir(l);
                            return;
                        }
                        write_u8(cup_round_entry(this, rodada) + j + 0x29, 0xff);
                    }
                    noticia_do_jogo(read_u32_as_usize(r + 0x1c), read_u32_as_usize(r + 0x20), read_u32_as_usize(r + 0xc), read_u32_as_usize(r + 0x10), read_u32_as_usize(r + 4),
                        read_i16(r + 0x2a) as u16, read_i16(r + 0x28) as u16);
                    j += 7;
                } else {
                    noticia_do_jogo(read_u32_as_usize(f + 0x1c), read_u32_as_usize(f + 0x20), read_u32_as_usize(f + 0xc), read_u32_as_usize(f + 0x10), read_u32_as_usize(f + 4),
                        read_i16(f + 0x2a) as u16, read_i16(f + 0x28) as u16);
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
pub(crate) unsafe extern "thiscall" fn comparar_move_round_fixtures_to_set_dates(this: usize, data: usize, rodada: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_MOVE_ROUND_FIXTURES_TO_SET_DATES);
    let e = cup_round_entry(this, rodada);
    if read_i16(e + 0xd) as u16 & 0x100 == 0 || read_u8(e + 0x29) == 0xff {
        return o(this, data, rodada);
    }
    let id = read_u32_as_usize(read_u32_as_usize(this + 4));
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    criar(l);
    let mut esperado = [0u8; 8];
    for k in 0..8 {
        esperado[k] = read_u8(e + k * 7 + 0x29);
    }
    if collect_round_fixtures(this, data, rodada, l) != 0 {
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        let mut j = 0usize;
        let mut f = proximo(l);
        while f != 0 {
            if read_u8(this + 0x44) == read_u8(f + 0x42) && j < 0x38 && esperado[j / 7] != 0xff {
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
    if read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE) + id * 4) == 0 {
        return; // ponytail: a copa saiu do jogo; nada a conferir
    }
    let e = cup_round_entry(this, rodada);
    let depois: [u8; 8] = core::array::from_fn(|k| read_u8(e + k * 7 + 0x29));
    PLACAR.registrar(depois == esperado, || format!("move_round_fixtures_to_set_dates(rodada {}) vagas {:02x?} (esperado {:02x?})",
        rodada, depois, esperado));
}

/// allocates the ties of each round, clears them, links each tie to a slot in the next round (by pair or drawn, the
/// side by a coin; with no slot, two rounds ahead), puts each round's teams into its ties (drawn, the cup nation's club
/// at home) and numbers the first tie of each round
/// Original: `Cup::map_fixture_tree` (0x518790, Cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x518790 } (thiscall, ret 0, Cup.cpp): `Cup::map_fixture_tree()`, da criação das copas (22 chamadas até
/// 2002-06-01). Os vetores dos confrontos de cada rodada (0x944e46: 4 por rodada e 0x35 por confronto; sem memória, a
/// caixa "Memory Allocation Error" da linha 0x8b ou 0x9e e o jogo para; rodada sem confrontos, a caixa da linha 0x95 e
/// -1); a marca +5 de cada time (+0xa7, 6 bytes) e +0xab/+0xac zeradas. Com clubes (+0x45): cada confronto vazio (os
/// times e as vagas −1, +8 = 0, +0x14..+0x31 = 0xff, +0x32..+0x34 = 0). Nas rodadas até a penúltima (a antepenúltima com
/// a última do tipo 0x8c): a ordem dos confrontos (embaralhada aos pares sorteados se a rodada seguinte tem menos da
/// metade); cada um ganha a vaga na rodada seguinte: pelo par (rodada +0xd bit 0x200: p/2) ou sorteada (com o bit 0x400,
/// a primeira vaga toda livre a partir dela), o lado pela moeda (0x90cfc0(2); ocupada, a vaga seguinte e outra moeda);
/// sem vaga livre, uma vaga sorteada duas rodadas adiante (sem ela, o jogo para: linhas 0x175 e 0x1a0). O confronto
/// guarda a vaga (+6), o lado (+8: 1 mandante) e a rodada de destino (+0x12); a vaga guarda o confronto (+0xa/+0xc) e a
/// rodada de origem (+0xe/+0x10). Depois os times de cada rodada (+0x1c, a partir de +0x1e) entram nos confrontos dela:
/// o primeiro pelo par ou sorteado (0x400: o primeiro todo livre), o lado pela moeda, e numa vaga ocupada o time da
/// nação +0x24 vira mandante (sem vaga, a caixa da linha 0x1c2 ou 0x222 e -1). Sem confrontos na primeira rodada, +0xb1
/// = 1. Por fim o número do primeiro confronto de cada rodada (+0x5b), de trás para a frente. Devolve 1.
pub(crate) unsafe extern "thiscall" fn map_fixture_tree(this: usize) -> i32 {
    let reservar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let t = reservar((read_i16(this + 0x3c) as i32 as u32) << 2);
    write_u32_from_usize(this + 0xad, t);
    if t == 0 {
        fatal_memory_error_box(0x8b, 0x9b5600);
    }
    let mut i = 0i32;
    while i < read_i16(this + 0x3c) as i32 {
        let n = read_i16(comp_cup_round(this, i) + 0x1a);
        if n < 1 {
            show_error_at(0x9b55ac, 0x95);
            return -1;
        }
        let v = reservar((n as i32 * 0x35) as u32);
        write_u32_from_usize(read_u32_as_usize(this + 0xad) + i as usize * 4, v);
        if read_u32_as_usize(read_u32_as_usize(this + 0xad) + i as usize * 4) == 0 {
            fatal_memory_error_box(0x9e, 0x9b55d8);
        }
        i += 1;
    }
    if read_u32_as_usize(this + 0xa7) != 0 {
        let mut k = 0i32;
        while k < read_i16(this + 0x3e) as i32 {
            write_u8(read_u32_as_usize(this + 0xa7) + k as usize * 6 + 5, 0);
            k += 1;
        }
    }
    write_u8(this + 0xab, 0);
    write_u8(this + 0xac, 0);
    if read_u32_as_usize(this + 0x45) == 0 {
        return 1;
    }
    let mut i = 0i32;
    while i < read_i16(this + 0x3c) as i32 {
        let mut j = 0i32;
        while j < read_i16(comp_cup_round(this, i) + 0x1a) as i32 {
            let c = cup_tie(this, i, j);
            for k in [0usize, 2, 6, 0xa, 0xc, 0xe, 0x10, 0x12] {
                write_u16(c + k, 0xffff);
            }
            write_u16(c + 8, 0);
            for k in 0x14..=0x31usize {
                write_u8(c + k, 0xff);
            }
            write_u8(c + 0x32, 0);
            write_u8(c + 0x33, 0);
            write_u8(c + 0x34, 0);
            j += 1;
        }
        i += 1;
    }
    let nr = read_i16(this + 0x3c);
    let m = (if read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x61)) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    let mut r = 0i32;
    while r < m - 1 {
        let n = read_i16(comp_cup_round(this, r) + 0x1a);
        let ordem = if n != 0 { reservar(n as i32 as u32) } else { 0 };
        let mut k = 0i32;
        while k < read_i16(comp_cup_round(this, r) + 0x1a) as i32 {
            write_u8(ordem + k as usize, k as u8);
            k += 1;
        }
        let nn = read_i16(comp_cup_round(this, r) + 0x1a) as i32;
        let prox = read_i16(comp_cup_round(this, r) + 0x82) as i32;
        if prox < nn / 2 && nn > 0 {
            let (mut q, mut conta) = (0i32, nn);
            loop {
                let a = sorteio(conta) as usize;
                let b = sorteio(read_i16(comp_cup_round(this, r) + 0x1a) as i32) as usize;
                let x = read_u8(ordem + a);
                write_u8(ordem + a, read_u8(ordem + b));
                write_u8(ordem + b, x);
                q += 1;
                conta = read_i16(comp_cup_round(this, r) + 0x1a) as i32;
                if q >= conta {
                    break;
                }
            }
        }
        let mut k = 0i32;
        while k < read_i16(comp_cup_round(this, r) + 0x1a) as i32 {
            let rr = comp_cup_round(this, r);
            let p = read_u8(ordem + k as usize) as i8 as i32;
            let mut s: i32;
            let mut inicio: i32;
            if read_i16(rr + 0x75) as u16 & 0x200 != 0 {
                s = p / 2; // cqd7: a vaga pelo par
                inicio = s;
            } else {
                s = sorteio(read_i16(rr + 0x82) as i32);
                inicio = s;
                if read_i16(rr + 0x75) as u16 & 0x400 != 0 {
                    loop {
                        let v = cup_tie(this, r + 1, s as i16 as i32);
                        if read_i16(v + 0xa) == -1 && read_i16(v + 0xc) == -1 {
                            inicio = s;
                            break;
                        }
                        s += 1;
                        if s as i16 >= read_i16(rr + 0x82) {
                            s = 0;
                        }
                        if s as i16 == inicio as i16 {
                            break;
                        }
                    }
                }
            }
            let (mut s, inicio) = (s, inicio); // cqd7: a vaga inicial na rodada seguinte
            let c = cup_tie(this, r, p);
            loop {
                let moeda = sorteio(2);
                let v = cup_tie(this, r + 1, s as i16 as i32);
                let primeiro_a = moeda != 0; // cqd7: a moeda do lado na rodada seguinte
                let lado = if primeiro_a {
                    if read_i16(v + 0xa) == -1 { Some(true) } else if read_i16(v + 0xc) == -1 { Some(false) } else { None }
                } else if read_i16(v + 0xc) == -1 {
                    Some(false)
                } else if read_i16(v + 0xa) == -1 {
                    Some(true)
                } else {
                    None
                };
                if let Some(a) = lado {
                    crate::comparacao::ramo("cqd_7 vaga na rodada seguinte");
                    write_u16(v + if a { 0xa } else { 0xc }, p as u16);
                    write_u16(v + if a { 0xe } else { 0x10 }, r as u16);
                    write_u16(c + 6, s as u16);
                    write_u16(c + 8, a as u16);
                    write_u16(c + 0x12, (r + 1) as u16);
                    break;
                }
                if (s as i16 as i32) < read_i16(comp_cup_round(this, r) + 0x82) as i32 - 1 {
                    s += 1;
                } else {
                    s = 0;
                }
                if inicio as i16 == s as i16 {
                    // sem vaga livre: uma vaga sorteada duas rodadas adiante
                    if r >= m - 2 {
                        fatal_error_at(0x9b55ac, 0x175);
                    }
                    let mut t = sorteio(read_i16(comp_cup_round(this, r) + 0xea) as i32);
                    let ini = t;
                    loop {
                        let v = cup_tie(this, r + 2, t as i16 as i32);
                        let a = if read_i16(v + 0xa) == -1 {
                            true
                        } else if read_i16(v + 0xc) == -1 {
                            false
                        } else {
                            if (t as i16 as i32) < read_i16(comp_cup_round(this, r) + 0xea) as i32 - 1 {
                                t += 1;
                            } else {
                                t = 0;
                            }
                            if t as i16 == ini as i16 {
                                fatal_error_at(0x9b55ac, 0x1a0);
                            }
                            continue;
                        };
                        write_u16(v + if a { 0xa } else { 0xc }, p as u16);
                        write_u16(v + if a { 0xe } else { 0x10 }, r as u16);
                        write_u16(c + 6, t as u16);
                        write_u16(c + 8, a as u16);
                        break;
                    }
                    write_u16(c + 0x12, (r + 2) as u16);
                    break;
                }
            }
            k += 1;
        }
        if ordem != 0 {
            soltar(ordem);
        }
        r += 1;
    }
    if read_i16(comp_cup_round(this, 0) + 0x1a) == 0 {
        write_u8(this + 0xb1, 1);
        return 1;
    }
    // os times de cada rodada nos confrontos dela; `inicio` segue de um time para o outro, como na original
    let mut inicio: i32 = -1;
    let mut r = 0i32;
    while r < m {
        if read_i16(comp_cup_round(this, r) + 0x1c) > 0 {
            let mut q = 0i32;
            loop {
                let rr = comp_cup_round(this, r);
                let mut di: i32;
                if read_i16(rr + 0xd) as u16 & 0x200 != 0 {
                    di = 0;
                    if read_i16(rr + 0x1a) <= 0 {
                        show_error_at(0x9b55ac, 0x1c2);
                        return -1;
                    }
                } else {
                    di = sorteio(read_i16(rr + 0x1a) as i32);
                    inicio = di;
                    let rr = comp_cup_round(this, r);
                    if read_i16(rr + 0xd) as u16 & 0x400 != 0 {
                        loop {
                            let c = cup_tie(this, r, di as i16 as i32);
                            if read_i16(c) == -1 && read_i16(c + 2) == -1 {
                                inicio = di;
                                break;
                            }
                            di += 1;
                            if di as i16 >= read_i16(rr + 0x1a) {
                                di = 0;
                            }
                            if di as i16 == inicio as i16 {
                                break;
                            }
                        }
                    }
                }
                loop {
                    let moeda = sorteio(2);
                    let x = di as i16 as i32;
                    let c = cup_tie(this, r, x);
                    let livre2 = || read_i16(c + 2) == -1 && read_i16(c + 0xc) == -1;
                    let primeiro_0 = moeda != 0; // cqd7: a moeda do lado na primeira rodada
                    let v0 = read_i16(c) as u16;
                    let livre0 = v0 == 0xffff && read_i16(c + 0xa) == -1;
                    let lado = if primeiro_0 {
                        if livre0 { Some(0usize) } else if livre2() { Some(2) } else { None }
                    } else if livre2() {
                        Some(2)
                    } else if livre0 {
                        Some(0)
                    } else {
                        None
                    };
                    if let Some(o) = lado {
                        crate::comparacao::ramo("cqd_7 time na primeira rodada");
                        write_u16(c + o, (read_i16(comp_cup_round(this, r) + 0x1e) as u16).wrapping_add(q as u16));
                        break;
                    }
                    // a vaga ocupada: o time da nação +0x24 no lado de fora vira mandante
                    if read_u32_as_usize(this + 0x24) as u32 != u32::MAX && v0 != 0xffff {
                        let fora = read_i16(c + 2);
                        if fora != -1 {
                            let clube = read_u32_as_usize(read_u32_as_usize(this + 0xa7) + (fora as isize * 6) as usize);
                            if clube != 0 {
                                let nacao = read_u32_as_usize(clube + club::STADIUM);
                                if nacao != 0 && read_u32_as_usize(nacao) == read_u32_as_usize(this + 0x24) {
                                    write_u16(c, fora as u16);
                                    write_u16(c + 2, v0);
                                }
                            }
                        }
                    }
                    if x < read_i16(comp_cup_round(this, r) + 0x1a) as i32 - 1 {
                        di += 1;
                    } else {
                        di = 0;
                    }
                    if inicio as i16 == di as i16 {
                        show_error_at(0x9b55ac, 0x222);
                        return -1;
                    }
                }
                q += 1;
                if q >= read_i16(comp_cup_round(this, r) + 0x1c) as i32 {
                    break;
                }
            }
        }
        r += 1;
    }
    let nr = read_i16(this + 0x3c);
    if nr > 0 && read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x61)) == 0x8c {
        let mut k = nr.wrapping_sub(3) as i32;
        let mut n: u8 = 5;
        while k >= 0 {
            write_u8(comp_cup_round(this, k) + 0x5b, n);
            n = n.wrapping_add(read_u8(comp_cup_round(this, k) + 0x1a));
            k -= 1;
        }
    } else {
        let mut n = read_u8(comp_cup_round(this, nr as i32).wrapping_sub(0x50)).wrapping_add(1);
        let mut k = nr.wrapping_sub(2) as i32;
        while k >= 0 {
            write_u8(comp_cup_round(this, k) + 0x5b, n);
            n = n.wrapping_add(read_u8(comp_cup_round(this, k) + 0x1a));
            k -= 1;
        }
    }
    1
}

/// O comparador da árvore dos confrontos: só a original (ponytail: aloca os vetores dos confrontos e sorteia; não
/// roda duas vezes). Confere o que ela garante quando devolve 1 com clubes: cada confronto das rodadas mapeadas aponta
/// (+6 vaga, +8 lado, +0x12 rodada uma ou duas adiante) uma vaga que aponta de volta para ele (+0xa/+0xe mandante,
/// +0xc/+0x10 visitante).
pub(crate) unsafe extern "thiscall" fn comparar_map_fixture_tree(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(IDX_MAP_FIXTURE_TREE);
    let r = o(this);
    if r != 1 || read_u32_as_usize(this + 0x45) == 0 {
        return r;
    }
    let nr = read_i16(this + 0x3c);
    let m = (if read_i16(comp_cup_round(this, nr as i32).wrapping_sub(0x61)) == 0x8c { nr.wrapping_sub(1) } else { nr }) as i32;
    let mut erro: Option<(i32, i32)> = None;
    'fora: for rd in 0..(m - 1).max(0) {
        for p in 0..read_i16(comp_cup_round(this, rd) + 0x1a) as i32 {
            let c = cup_tie(this, rd, p);
            let destino = read_i16(c + 0x12) as i32;
            let a = read_i16(c + 8) == 1;
            let ok = (destino == rd + 1 || destino == rd + 2) && {
                let v = cup_tie(this, destino, read_i16(c + 6) as i32);
                read_i16(v + if a { 0xa } else { 0xc }) as i32 == p && read_i16(v + if a { 0xe } else { 0x10 }) as i32 == rd
            };
            if !ok {
                erro = Some((rd, p));
                break 'fora;
            }
        }
    }
    PLACAR.registrar(erro.is_none(), || format!("map_fixture_tree: o confronto (rodada, índice) {:?} sem a vaga de volta", erro));
    r
}

/// sorts the cup clubs by strength and gives each one the predicted position of the round it goes out in, in the cup
/// statistics
/// Original: `Cup::rank_teams` (0x5223a0, Cup.cpp, thiscall, ret 0).
///
/// [pt] { 0x5223a0 } (thiscall, ret 0, Cup.cpp): `Cup::rank_teams()`: copia os clubes da copa ([+0xa7], registros de 6
/// bytes, +0x3e deles) para um vetor (0x944e46), ordena pela força (0x4c08f0) e dá a cada um a posição prevista pela
/// rodada em que sai (0x4a2540 na estatística [+8]); libera o vetor (0x9452ca). Sem estatística: a caixa (linha
/// 0x1827); sem memória: a original (a caixa da falta de memória e o fim do jogo).
pub(crate) unsafe extern "thiscall" fn cup_rank_teams(this: usize) {
    if read_u32_as_usize(this + 8) == 0 {
        return show_error_at(0x9b55ac, 0x1827);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let ordenar: unsafe extern "C" fn(usize, i32) = core::mem::transmute(FN_COMP_UTIL_SORT_CLUBS_BY_STRENGTH);
    let n = read_i16(this + 0x3e) as i32;
    let v = alocar((n << 2) as u32);
    if v == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_RANK_TEAMS)(this);
    }
    for i in 0..n.max(0) as usize {
        *((v + i * 4) as *mut u32) = read_u32_as_usize(read_u32_as_usize(this + 0xa7) + i * 6) as u32;
    }
    ordenar(v, n);
    set_final_positions_by_round(this, v as *const u32, 1);
    liberar(v);
}

/// Comparador: grava e pode realocar a estatística (não roda duas vezes): antes da original, as posições são
/// planejadas sobre uma cópia ordenada do vetor (as chaves num Vec); depois, o registro de cada clube planejado tem
/// de ter em +0x72 a última posição dele.
pub(crate) unsafe extern "thiscall" fn comparar_cup_rank_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CUP_RANK_TEAMS);
    let n = read_i16(this + 0x3e) as i32;
    if read_u32_as_usize(this + 8) == 0 || n <= 0 {
        return o(this); // ponytail: a caixa ou nada a ordenar, só a original
    }
    let mut v: Vec<u32> = (0..n as usize).map(|i| read_u32_as_usize(read_u32_as_usize(this + 0xa7) + i * 6) as u32).collect();
    let mut k = vec![0i32; n as usize];
    sort_clubs_by_strength(v.as_mut_ptr(), k.as_mut_ptr(), n);
    // ponytail: com mais vagas nas rodadas que clubes, a original lê além do vetor (sobra do bloco); o plano lê 0
    v.extend(core::iter::repeat(0).take(0x100));
    let plano = set_final_positions_by_round(this, v.as_ptr(), 0);
    o(this);
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let mut erros = Vec::new();
    for (i, &(c, p)) in plano.iter().enumerate() {
        if c == 0 || plano[i + 1..].iter().any(|x| x.0 == c) {
            continue;
        }
        let e = buscar(read_u32_as_usize(this + 8), c);
        if e == 0 || *((e + 0x72) as *const u8) != p {
            erros.push(format!("clube {:x} esperado {} obtido {}", c, p, if e == 0 { -1 } else { *((e + 0x72) as *const u8) as i32 }));
        }
    }
    PLACAR.registrar(erros.is_empty(), || format!("cup_rank_teams({:x}) {} posições: {}", this, plano.len(), erros.join("; ")));
}

/// stores one of the eight cup round match days: the venue flags, the weekday, the time slot and the code
/// Original: `cup_set_round_match_day` (0x521f10, Cup.cpp, cdecl).
///
/// [pt] { 0x521f10 } (cdecl, Cup.cpp): o dia `vaga` (0 a 7) da rodada `rodada` (registros de 104 bytes):
/// em 7 bytes a partir de +0x23, o mando (4 bytes), o dia da semana (−1 a 6), o horário (−1 a 2) e o
/// código (−1 a 3). Fora da faixa: a original (caixa de erro e fim do jogo). Devolve o mando.
pub(crate) unsafe extern "C" fn cup_set_round_match_day(rodadas: usize, rodada: i16, vaga: i16, semana: i16, hora: i8, codigo: i8, mando: u32) -> u32 {
    if !(0..8).contains(&vaga) || !(-1..=6).contains(&semana) || !(-1..3).contains(&hora) || !(-1..=3).contains(&codigo) {
        return original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(IDX_CUP_SET_ROUND_MATCH_DAY)(rodadas, rodada, vaga, semana, hora, codigo, mando);
    }
    let e = rodadas.wrapping_add((rodada as isize * 104 + vaga as isize * 7) as usize);
    *((e + 0x27) as *mut u8) = semana as u8; // dia da semana
    *((e + 0x28) as *mut u8) = hora as u8;
    *((e + 0x29) as *mut u8) = codigo as u8;
    core::ptr::write_unaligned((e + 0x23) as *mut u32, mando);
    mando
}

pub(crate) unsafe extern "C" fn comparar_cup_set_round_match_day(rodadas: usize, rodada: i16, vaga: i16, semana: i16, hora: i8, codigo: i8, mando: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, i16, i16, i16, i8, i8, u32) -> u32>(IDX_CUP_SET_ROUND_MATCH_DAY);
    if !(0..8).contains(&vaga) || !(-1..=6).contains(&semana) || !(-1..3).contains(&hora) || !(-1..=3).contains(&codigo) {
        return o(rodadas, rodada, vaga, semana, hora, codigo, mando);
    }
    let e = rodadas.wrapping_add((rodada as isize * 104 + vaga as isize * 7) as usize);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x23, 7)], || o(rodadas, rodada, vaga, semana, hora, codigo, mando),
        || cup_set_round_match_day(rodadas, rodada, vaga, semana, hora, codigo, mando));
    PLACAR.registrar(igual && r == n, || format!("cup_set_round_match_day({}, {}) {:x?} x {:x?}", rodada, vaga, mo, mr));
    r
}

/// tells whether the club is in the cup's team table
/// Original: `Cup::has_club` (0x522910, Cup.cpp, thiscall, ret 4).
///
/// [pt] { 0x522910 } (thiscall, ret 4, Cup.cpp; virtual +0x80 das copas, chamada pela 0x4ae300 da projeção da diretoria):
/// `Cup::has_club(clube)`: 1 se o clube está na tabela de times da copa (+0xa7, 6 bytes por time, +0 o clube; conta
/// +0x3e); senão 0.
pub(crate) unsafe extern "thiscall" fn has_club(this: usize, clube: u32) -> u32 {
    let n = *((this + 0x3e) as *const i16);
    let times = read_u32(this + 0xa7) as usize;
    for i in 0..n.max(0) as usize {
        if read_u32(times + i * 6) == clube {
            crate::comparacao::ramo("cpr_13 copa tem o clube");
            return 1; // está na copa
        }
    }
    0
}

/// Comparador: pelo valor (só lê).
pub(crate) unsafe extern "thiscall" fn comparar_has_club(this: usize, clube: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(IDX_HAS_CLUB)(this, clube);
    let n = has_club(this, clube);
    PLACAR.registrar(r == n, || format!("has_club(copa {:x}, clube {:x}) original={} rust={}", this, clube, r, n));
    r
}
