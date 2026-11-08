use super::*;





/// Club record i in the club table (0x245 bytes each).
///
/// [pt] O registro do clube `i` (0x245 bytes em [0xae23b4]; a original calcula base + i + 580·i).
pub(crate) unsafe fn fix_man_club_by_index(i: usize) -> usize {
    read_u32_as_usize(G_CLUB_TABLE).wrapping_add(i).wrapping_add(i.wrapping_mul(0x244))
}

/// True if the club has a human manager (0x600a50 with 1,1,0,0).
///
/// [pt] O clube tem técnico humano (0x600a50: push 0; push 0; push 1; push 1; push clube; add esp, 0x14 -> cdecl).
pub(crate) unsafe fn fix_man_is_human_club(clube: usize) -> bool {
    let f: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    f(clube, 1, 1, 0, 0) != 0
}

/// on days 2 and 3 the reserve fixtures 3 days ahead (and tomorrow on day 3); the queued fixture news to both clubs'
/// managers; on days 5 and 6 yesterday's friendlies between clubs without a human manager leave the lists; the cup
/// round labels from the leg check list
/// Original: `FIXTURE_MANAGER::daily_update` (0x5aaf00, fix_man.cpp, thiscall, ret 0).
///
/// [pt] { 0x5aaf00 } (thiscall, ret 0, fix_man.cpp, quadro SEH sem destrutor no caminho normal: as datas locais):
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
pub(crate) unsafe extern "thiscall" fn fix_man_daily_update(this: usize) {
    let week: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_DATE_GET_WEEKDAY);
    let more: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let minus: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    let reservas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_RESERVECOMP_SCHEDULE_RESERVE_FIXTURES);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let previous: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_PREV);
    let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE_NODE);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let slot: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_SLOT);
    let warn: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    let rotular: unsafe extern "thiscall" fn(usize, u32, u32, u32) = core::mem::transmute(FN_CUP_LABEL_ROUND_EXTRA_FIXTURES);
    let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    let (pa, pb, pc) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, c.as_mut_ptr() as usize);
    let day = week(G_CURRENT_DATE) as i16;
    if day == 2 || day == 3 {
        more(G_CURRENT_DATE, pa, 3);
        reservas(read_u32_as_usize(G_RESERVE_COMP), pa);
        if day == 3 {
            more(G_CURRENT_DATE, pb, 1);
            crate::comparacao::ramo("cqd_6 reservas de amanha");
            reservas(read_u32_as_usize(G_RESERVE_COMP), pb); // cqd6: as reservas de amanhã
        }
    }
    let l = read_u32_as_usize(this + 0x1128);
    if l != 0 {
        write_u32_from_usize(l + 0xa, read_u32_as_usize(l + 2));
        let mut n = next(l);
        while n != 0 {
            let ca = fix_man_club_by_index(slot(n, 0) as usize);
            let cb = fix_man_club_by_index(slot(n, 1) as usize);
            if slot(n, 0xe) != 0 {
                warn(read_u32_as_usize(G_NEWS), n, ca);
            }
            if slot(n, 0xf) != 0 {
                warn(read_u32_as_usize(G_NEWS), n, cb);
            }
            n = next(read_u32_as_usize(this + 0x1128));
        }
        let l = read_u32_as_usize(this + 0x1128);
        if l != 0 {
            destroy(l);
            free(l);
        }
        write_u32_from_usize(this + 0x1128, 0);
    }
    if day == 5 || day == 6 {
        let t = minus(G_CURRENT_DATE, pa, 1);
        if read_i16(t + 2) == read_i16(G_CURRENT_DATE_YEAR) {
            let ontem = read_i16(minus(G_CURRENT_DATE, pc, 1)) as u16 as usize;
            for fatia in 0..3usize {
                let lists = read_u32_as_usize(this + (ontem * 3 + fatia) * 4);
                let mut k: i8 = 0;
                while k < read_u8(G_KEY_NATION_ACTIVE_COUNT) as i8 {
                    let list = read_u32_as_usize(lists + k as isize as usize * 4);
                    let mut no = read_u32_as_usize(list + 6);
                    write_u32_from_usize(list + 0xa, no);
                    while no != 0 {
                        let f = read_u32_as_usize(no);
                        if read_u32_as_usize(f) != read_u32_as_usize(KEY_CLUB_COMP_RESERVE) || read_u8(f + 0x3a) != 1 {
                            break;
                        }
                        if !fix_man_is_human_club(read_u32_as_usize(f + 0x1c)) && !fix_man_is_human_club(read_u32_as_usize(f + 0x20)) {
                            previous(list);
                            crate::comparacao::ramo("cqd_6 amistoso descartado");
                            remove(list, no); // cqd6: o amistoso de ontem sai da lista
                        } else {
                            previous(list);
                        }
                        no = read_u32_as_usize(list + 0xa);
                    }
                    k = k.wrapping_add(1);
                }
            }
        }
    }
    if read_i16(G_FIXTURE_LEGS_LIST) != 0 {
        write_u32_from_usize(G_FIXTURE_LEGS_LIST_CURSOR, read_u32_as_usize(G_FIXTURE_LEGS_LIST_HEAD));
        let mut e = next(G_FIXTURE_LEGS_LIST);
        while e != 0 {
            let i = read_u32_as_usize(e) as i32;
            let table = read_u32_as_usize(G_COMP_OBJECT_TABLE);
            if i >= 0 && table != 0 {
                let mut cp = read_u32_as_usize(table + i as usize * 4);
                if cp != 0 {
                    let stage = read_u8(e + 4) as i8;
                    if stage as i32 <= read_u32_as_usize(cp + 0x30) as i32 {
                        if stage != -1 {
                            cp = read_u32_as_usize(read_u32_as_usize(cp + 0xc) + stage as isize as usize * 4);
                        }
                        if read_u8(cp + 0x43) == 1 {
                            // simplification: a original empurra eax com a palavra da rodada e os bytes altos do ponteiro da
                            // entrada; a 0x522c70 só lê a palavra
                            rotular(cp, read_i16(e + 5) as u16 as u32, read_u32_as_usize(e + 7) as u32, read_u32_as_usize(e + 0xb) as u32);
                        }
                    }
                }
            }
            e = next(G_FIXTURE_LEGS_LIST);
        }
        esvaziar(G_FIXTURE_LEGS_LIST);
    }
}


/// the fixture list of a year, day, slice and key nation, switching the other year block to the requested year or
/// creating it
/// Original: `FIXTURE_MANAGER::get_fixture_list` (0x5abf10, fix_man.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5abf10 } (thiscall, ret 0x10, fix_man.cpp, quadro SEH sem destrutor no caminho normal):
/// `FIXTURE_MANAGER::get_fixture_list(ano, dia, fatia, tipo)`: a lista dos jogos do ano, dia (palavra), fatia (byte) e
/// nação-chave (byte com sinal) na corrente dos blocos dos anos. Pelo bloco do ano (+0x1130): se o outro (+0x112c) não
/// é do ano, ele é trocado para o ano (0x5aa2d0: push ano; ecx = o outro -> thiscall, ret 4); sem o outro, um bloco
/// novo (0x944cf1, 0x1184 bytes; 0x5a9cf0: push 0; push this; push ano; ecx = o novo -> thiscall, ret 0xc) entra em
/// +0x112c; sem memória, a caixa da linha 0x681 e 0. A lista: [[bloco + (dia·3 + fatia)·4] + tipo·4].
pub(crate) unsafe extern "thiscall" fn get_fixture_list(this: usize, year: u32, day: u32, slice: u32, kind: u32) -> usize {
    let swap: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_SET_YEAR);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construct: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_FIXTURE_MANAGER);
    let mut b = this;
    while read_i16(b + 0x1130) != year as i16 {
        let o = read_u32_as_usize(b + 0x112c);
        if o == 0 {
            let m = new(0x1184);
            let r = if m == 0 { 0 } else { construct(m, year, b, 0) };
            write_u32_from_usize(b + 0x112c, r);
            if r == 0 {
                show_error_at(0x9cc208, 0x681);
                return 0;
            }
        } else if read_i16(o + 0x1130) != year as i16 {
            swap(o, year);
        }
        b = read_u32_as_usize(b + 0x112c);
    }
    let i = (day & 0xffff) as usize * 3 + (slice & 0xff) as usize;
    read_u32_as_usize(read_u32_as_usize(b + i * 4) + (kind as u8 as i8) as isize as usize * 4)
}


/// reads fixtures_<year>.tmp: each 0x4f-byte fixture gets its club, competition and stadium pointers and goes into the
/// list of its day, slice and key nation
/// Original: `FIXTURE_MANAGER::load_fixtures` (0x5ac3b0, fix_man.cpp, thiscall, ret 0).
///
/// [pt] { 0x5ac3b0 } (thiscall, ret 0, fix_man.cpp): `FIXTURE_MANAGER::load_fixtures()`: o nome `fixtures_<ano>.tmp` em
/// +0x1133 (0x944c9f) e, com ano (+0x1130, sem sinal) desde 1980 e o arquivo em CM3_TEMP (0x90bde0: push nome; push
/// "CM3_TEMP"; add esp, 8 -> cdecl; 0x945b00 fopen com "rb"), a conta (dword) e os jogos de 0x4f bytes (0x945bfe
/// fread; falhando, a caixa da linha 0x730 ou 0x73e e o arquivo fechado): os ids dos clubes (+0xc, +0x10 → +0x1c,
/// +0x20), da competição (+0 → +0x14, 0x6b bytes em [0xae23d0]) e do estádio (+4 → +0x18, 0x4e bytes em [0xae23b0], 0
/// fora de [0xae2358]) viram ponteiros (id -1: o jogo para, linhas 0x74b, 0x751, 0x758), e o jogo entra (0x68d0d0: push
/// 0x4f; push &jogo; ecx = a lista -> thiscall, ret 8; falhando, o jogo para, 0x785) na lista do dia (+0x2a), fatia
/// (+0x3f) e nação-chave do jogo (0x48dd70: push &jogo; push competição; add esp, 8 -> cdecl). No fim, fclose
/// (0x945a09).
pub(crate) unsafe extern "thiscall" fn load_fixtures(this: usize) {
    let format: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let path: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let open: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let read: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
    let close: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let nacao: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_ACTIVE_KEY_NATION_OF_FIXTURE);
    let append: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let name = this + 0x1133;
    let year = read_i16(this + 0x1130) as u16;
    format(name, 0x9cc238, year as u32);
    if year < 0x7bc {
        return;
    }
    let f = open(path(0x987da0, name), 0x98f928);
    if f == 0 {
        return;
    }
    let mut count = 0i32;
    if read(&mut count as *mut i32 as usize, 4, 1, f) != 1 {
        show_error_at(0x9cc208, 0x730);
        close(f);
        return;
    }
    let mut jogo = [0u8; 0x4f];
    let j = jogo.as_mut_ptr() as usize;
    let mut i = 0i32;
    while i < count {
        if read(j, 0x4f, 1, f) != 1 {
            show_error_at(0x9cc208, 0x73e);
            close(f);
            return;
        }
        let a = read_u32_as_usize(j + 0xc);
        if a as u32 == u32::MAX {
            fatal_error_at(0x9cc208, 0x74b);
        }
        write_u32_from_usize(j + 0x1c, fix_man_club_by_index(a));
        let b = read_u32_as_usize(j + 0x10);
        if b as u32 == u32::MAX {
            fatal_error_at(0x9cc208, 0x751);
        }
        write_u32_from_usize(j + 0x20, fix_man_club_by_index(b));
        let c = read_u32_as_usize(j);
        if c as u32 == u32::MAX {
            fatal_error_at(0x9cc208, 0x758);
        }
        let comp = read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add(c.wrapping_mul(0x6b));
        write_u32_from_usize(j + 0x14, comp);
        let e = read_u32_as_usize(j + 4) as i32;
        let estadio = if e != -1 && e < read_u32_as_usize(G_STADIUM_COUNT) as i32 {
            read_u32_as_usize(G_STADIUM_TABLE).wrapping_add((e as usize).wrapping_mul(0x4e))
        } else {
            0
        };
        write_u32_from_usize(j + 0x18, estadio);
        let k = nacao(comp, j) as i8;
        let day = read_i16(j + 0x2a) as u16 as usize; // cqd6: o dia do jogo carregado
        let list = read_u32_as_usize(read_u32_as_usize(this + (day * 3 + read_u8(j + 0x3f) as usize) * 4) + k as isize as usize * 4);
        crate::comparacao::ramo("cqd_6 jogo carregado");
        if append(list, j, 0x4f) == 0 {
            fatal_error_at(0x9cc208, 0x785);
        }
        i += 1;
    }
    close(f);
}






/// O ensaio do comparador dos conflitos de um dia: a cópia não acrescenta (0x68d0d0 aloca o nó e a cópia do jogo) e
/// guarda os jogos que acrescentaria, na ordem.
pub(crate) static mut DAY_CLASHES_DRY_RUN: bool = false;
pub(crate) static mut DAY_CLASHES_APPENDED: Vec<usize> = Vec::new();

/// Appends a copy of the 0x4f-byte fixture to the list (0x68d0d0); in dry-run mode only records it.
///
/// [pt] Acrescenta a cópia dos 0x4f bytes do jogo no fim da lista (0x68d0d0: ecx = lista; push 0x4f; push jogo -> thiscall,
/// ret 8); no ensaio, só guarda o jogo.
pub(crate) unsafe fn append_fixture(list: usize, jogo: usize) {
    if DAY_CLASHES_DRY_RUN {
        DAY_CLASHES_APPENDED.push(jogo);
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    f(list, jogo, 0x4f);
}

/// Data pointers of a LLIST's nodes in order (head +2, node next +6, data +0).
///
/// [pt] Os dados dos nós da lista (cabeça em +2, o próximo em +6 do nó, o dado em +0), na ordem.
pub(crate) unsafe fn fix_man_list_items(list: usize) -> Vec<usize> {
    let mut v = Vec::new();
    if list == 0 {
        return v;
    }
    let mut no = read_usize(list + 2);
    while no != 0 {
        v.push(read_usize(no));
        no = read_usize(no + 6);
    }
    v
}

/// Concatenated 0x4f bytes of each fixture in a sequence.
///
/// [pt] O conteúdo (0x4f bytes por jogo) de uma sequência de jogos.
pub(crate) unsafe fn fixtures_bytes(games: &[usize]) -> Vec<u8> {
    let mut v = Vec::new();
    for &j in games {
        v.extend_from_slice(core::slice::from_raw_parts(j as *const u8, 0x4f));
    }
    v
}

/// Builds a date from (day, month, year) via 0x549f70 and returns (day of year, year).
///
/// [pt] A data (dia do mês, mês 0..11) do ano dado (0x549f70: lea ecx, local; push −1; push ano; push mês; push dia ->
/// thiscall, ret 0x10): (dia do ano, ano).
pub(crate) unsafe fn day_of_year(day: u32, month: u32, year: u32) -> (u16, u16) {
    let f: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32, u32, u32) -> *mut [u8; 8] = core::mem::transmute(FN_DATE_DATE_549F70);
    let mut t = [0u8; 8];
    let r = f(&mut t, day, month, year, 0xffff_ffff);
    (read_u16(r as usize), read_u16(r as usize + 2))
}

/// True if the club's division (+0x57) has the given competition id.
///
/// [pt] O jogo `x` está na liga de id [0x9cf590] por um dos clubes (+0x57 → +0).
pub(crate) unsafe fn is_in_league(clube: usize, id: i32) -> bool {
    let l = read_usize(clube + club::DIVISION);
    l != 0 && read_i32(l) == id
}

/// Calls the competition's clash-rule method (vtable +0x60) for fixtures a and b; 0 without the table or the
/// competition.
///
/// [pt] A regra de conflito da competição (método +0x60 da competição [0xadadfc][id]: push b; push a; ecx = comp ->
/// thiscall): 0 sem a tabela ou sem a competição.
pub(crate) unsafe fn comp_clash_rule(id_de: usize, a: usize, b: usize) -> u32 {
    let table = read_usize(G_COMP_OBJECT_TABLE);
    if table == 0 {
        return 0;
    }
    let c = read_usize(table.wrapping_add((read_i32(id_de) as usize).wrapping_mul(4)));
    if c == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + 0x60));
    f(c, a, b)
}

/// If checking played and the fixture was played (+0x43 != 0xff) sets severity 0x16; otherwise appends it and raises
/// severity to its type (+0x3a).
///
/// [pt] O jogo `x` entra (0x16 se já foi jogado, +0x43 diferente de 0xff) e a gravidade sobe ao tipo dele (+0x3a).
pub(crate) unsafe fn add_clash_by_type(grav: *mut i32, list: usize, x: usize, conferir_jogado: bool) {
    if conferir_jogado && read_u8(x + 0x43) != 0xff {
        *grav = 0x16;
        return;
    }
    append_fixture(list, x);
    let t = read_u8(x + 0x3a) as i8 as i32;
    if t > *grav {
        *grav = t;
    }
}

/// If severity is still 0 sets it to 0x14 or 1 (common tail of the same-club/same-stadium branches).
///
/// [pt] O fim comum dos ramos do mesmo clube e do mesmo estádio (0x5adf91/0x5ae060): sem gravidade, 0x14 (`vinte`) ou 1.
pub(crate) unsafe fn default_clash_severity(grav: *mut i32, vinte: bool) {
    if *grav == 0 {
        *grav = if vinte { 0x14 } else { 1 };
    }
}

/// on the fixture day plus the offset: forbidden dates (25 December, Sunday and 10 to 20 April in some nations, 17 May)
/// give 19; in each nation and slice day list, fixtures of the same club (same slice by type; other slice 0x14 or 1),
/// the same stadium, the national teams and big clubs by the competition rule go into the list; a played fixture gives
/// 0x16
/// Original: `FIXTURE_MANAGER::collect_day_clashes` (0x5adba0, fix_man.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x5adba0 } (thiscall, ret 0x1c, fix_man.cpp; quadro SEH só das datas): `FIXTURE_MANAGER::collect_day_clashes(jogo,
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
pub(crate) unsafe extern "thiscall" fn collect_day_clashes(this: usize, jogo: usize, grav: *mut i32, list: usize, nations: usize, n: u32, data: usize, desloc: u32) {
    // 0x53b900 (push; add esp, 4 -> cdecl), 0x53b3d0 (push; add esp, 4 -> cdecl)
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let more: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let week: unsafe extern "thiscall" fn(*const [u8; 8]) -> i32 = core::mem::transmute(FN_DATE_GET_WEEKDAY);
    let list_of_day: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_GET_FIXTURE_LIST);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    // 0x76d940: ecx = [0xb74318]; push comp -> thiscall, ret 4 (a cópia Rust ignora o ecx)
    let special: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
    let e_sel = national_team(read_usize(jogo + 0x1c));
    let n1 = read_usize(read_usize(jogo + 0x1c) + 0x53);
    // simplification: sem nação a original deixa a seleção sem valor; todo clube tem nação, então 0 não acontece no jogo
    let s1 = if n1 != 0 { principal(n1) } else { 0 };
    let n2 = read_usize(read_usize(jogo + 0x20) + 0x53);
    let s2 = if n2 != 0 { principal(n2) } else { 0 };
    let flag = desloc as u8 as i8;
    let mut day = [0u8; 8];
    more(data, &mut day, flag as i16 as u16 as u32);
    let d = day.as_ptr() as usize;
    let year = || read_usize(d + 2) as u32; // o dword do ano como a original o empurra (a 0x549f70 e a 0x5abf10 leem a palavra)
    let same = |t: (u16, u16)| t.1 == read_u16(d + 2) && t.0 == read_u16(d);
    let dezenove = || {
        if *grav < 0x13 {
            *grav = 0x13;
        }
    };
    let kind = read_u8(jogo + 0x3a);
    if kind == 1 && flag != 0 {
        return;
    }
    if flag == 0 {
        let comp = read_usize(jogo + 0x14);
        let nc = if comp != 0 { read_usize(comp + competition::CLUB_COMP_NATION) } else { 0 };
        let e_da = |g: usize| nc != 0 && read_i32(nc) == read_i32(g);
        if same(day_of_year(0x19, 0xb, year())) && !e_da(KEY_NATION_JAPAN) {
            crate::comparacao::ramo("cqc_3 natal");
            return dezenove();
        }
        if e_da(KEY_NATION_NORTHERN_IRELAND) && week(&day) as i16 == 6 {
            return dezenove();
        }
        if e_da(KEY_NATION_GREECE) {
            let t = day_of_year(10, 3, year());
            if t.1 < read_u16(d + 2) || (t.1 == read_u16(d + 2) && (t.0 as i16) < read_u16(d) as i16) {
                let t = day_of_year(0x14, 3, year());
                if t.1 > read_u16(d + 2) || (t.1 == read_u16(d + 2) && (t.0 as i16) > read_u16(d) as i16) {
                    return dezenove();
                }
            }
        }
    }
    let nac = read_usize(G_NATION_TABLE).wrapping_add((read_i32(KEY_NATION_NORWAY) as usize).wrapping_mul(nation::SIZE));
    if read_usize(read_usize(jogo + 0x1c) + 0x53) == nac || read_usize(read_usize(jogo + 0x20) + 0x53) == nac {
        if same(day_of_year(0x11, 4, year())) && flag == 0 {
            return dezenove();
        }
    }
    let mut k: i8 = 0;
    while k < n as u8 as i8 {
        let p = nations.wrapping_add(k as isize as usize);
        if read_u8(p) != 0xff {
            let mut slice: i8 = 0;
            while slice <= 2 {
                let l = list_of_day(this, year(), read_usize(d) as u32, slice as u8 as u32, read_u8(p) as u32);
                if l != 0 {
                    core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
                    let mut x = next(l);
                    while x != 0 {
                        let c1 = read_usize(x + 0x1c);
                        let h = read_usize(jogo + 0x1c);
                        let a = read_usize(jogo + 0x20);
                        if c1 == 0 || h == 0 || a == 0 {
                            // a caixa (linha 0xcc8) e o fim do jogo: a original
                            return original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32)>(IDX_COLLECT_DAY_CLASHES)(this, jogo, grav, list, nations, n, data, desloc);
                        }
                        let c2 = read_usize(x + 0x20);
                        let (sx0, sx1, s0, s1b) = (read_u8(x + 0x40), read_u8(x + 0x41), read_u8(jogo + 0x40), read_u8(jogo + 0x41));
                        if (c1 == h && sx0 == s0) || (c1 == a && sx0 == s1b) || (c2 == h && sx1 == s0) || (c2 == a && sx1 == s1b) {
                            crate::comparacao::ramo("cqc_3 mesmo clube na mesma fatia");
                            add_clash_by_type(grav, list, x, true); // cqc3: o mesmo clube na mesma fatia
                        } else if (flag == 0 && c1 == h && sx0 != s0) || (c1 == a && sx0 != s1b) || (c2 == h && sx1 != s0) || (c2 == a && sx1 != s1b) {
                            if read_u8(x + 0x43) != 0xff {
                                *grav = 0x16;
                            } else {
                                append_fixture(list, x);
                                if read_u8(jogo + 0x3f) == read_u8(x + 0x3f) || read_u8(x + 0x3a) != 1 {
                                    if *grav == 0 {
                                        crate::comparacao::ramo("cqc_3 mesmo clube noutra fatia");
                                    }
                                    default_clash_severity(grav, true); // cqc3: o mesmo clube noutra fatia
                                } else if read_u8(jogo + 0x3a) != 1 {
                                    default_clash_severity(grav, false);
                                }
                            }
                        } else if flag == 0 && read_usize(x + 0x18) != 0 && read_usize(jogo + 0x18) == read_usize(x + 0x18) && read_i32(x) != read_i32(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) && read_i32(jogo) != read_i32(KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION) {
                            if read_u8(x + 0x43) != 0xff {
                                *grav = 0x16;
                            } else {
                                append_fixture(list, x);
                                let t = read_u8(x + 0x3a);
                                if read_u8(jogo + 0x3f) == read_u8(x + 0x3f) {
                                    if t != 1 {
                                        default_clash_severity(grav, true);
                                    } else if *grav < 1 {
                                        *grav = 1;
                                    }
                                } else if t != 1 {
                                    default_clash_severity(grav, true);
                                } else if read_u8(jogo + 0x3a) != 1 {
                                    default_clash_severity(grav, false);
                                }
                            }
                        } else {
                            let abs_flag = (flag as i32).abs();
                            let selecoes = (e_sel == 0
                                && special(read_usize(G_NATIONAL_TEAMS), read_usize(x + 0x14)) == 0
                                && abs_flag < 2
                                && (read_u8(p) == 0 || read_u8(p) == 1)
                                && (read_u8(jogo + 0x3a) as i8) < 5
                                && c1 == s1)
                                || c1 == s2
                                || c2 == s1
                                || c2 == s2;
                            if selecoes {
                                if (read_u16(h + 0x80) as i16 > 0x1388 || read_u16(a + 0x80) as i16 > 0x1388) && comp_clash_rule(jogo, jogo, x) != 0 {
                                    add_clash_by_type(grav, list, x, true);
                                }
                            } else if read_i32(x) == read_i32(KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS) && (is_in_league(h, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE)) || is_in_league(a, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE))) {
                                *grav = 0x13;
                            } else if read_i32(jogo) == read_i32(KEY_CLUB_COMP_AMERICAN_MLS_ALL_STARS) && (is_in_league(c1, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE)) || is_in_league(c2, read_i32(KEY_CLUB_COMP_AMERICAN_MAJOR_LEAGUE))) {
                                add_clash_by_type(grav, list, x, false);
                            } else if e_sel != 0
                                && special(read_usize(G_NATIONAL_TEAMS), read_usize(jogo + 0x14)) == 0
                                && abs_flag < 2
                                && read_u8(p) != 0
                                && read_u8(p) != 1
                            {
                                let (nx1, nx2) = (read_usize(c1 + 0x53), read_usize(c2 + 0x53));
                                if (nx1 == n1 || nx2 == n1 || nx1 == n2 || nx2 == n2)
                                    && (read_u16(c1 + 0x80) as i16 > 0x1388 || read_u16(c2 + 0x80) as i16 > 0x1388)
                                    && (read_u8(x + 0x3a) as i8) < 5
                                    && comp_clash_rule(x, x, jogo) != 0
                                {
                                    add_clash_by_type(grav, list, x, true);
                                }
                            }
                        }
                        x = next(l);
                    }
                }
                slice += 1;
            }
        }
        k += 1;
    }
}


/// builds the nations to check (0 to 6, the fixture's, the leagues', the stadium's, the partners and the national
/// teams', without repeats), creates the list and collects the clashes of the two days before, the day before, the day
/// itself and the day after; with no clash, destroys the list, 0x15 if the national team forbids it and 0; otherwise
/// the severity and the list
/// Original: `FIXTURE_MANAGER::find_fixture_clashes` (0x5ad780, fix_man.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5ad780 } (thiscall, ret 0xc, fix_man.cpp; quadro SEH da lista nova): `FIXTURE_MANAGER::find_fixture_clashes(jogo,
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
pub(crate) unsafe extern "thiscall" fn find_fixture_clashes(this: usize, jogo: usize, out: *mut i32, two_days: u32) -> usize {
    if jogo == 0 || out.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize>(IDX_FIND_FIXTURE_CLASHES)(this, jogo, out, two_days); // a caixa
    }
    let key_of_game: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_ACTIVE_KEY_NATION_OF_FIXTURE);
    let of_stadium: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_COMP_ACTIVE_KEY_NATION_OF_STADIUM);
    let parceira: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_FIX_MAN_GET_PARTNER_KEY_NATION_INDEX);
    let of_national_team: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(FN_COMP_ACTIVE_KEY_NATION_OF_CLUB);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let new_data: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    let of_day: unsafe extern "thiscall" fn(usize, usize, *mut i32, usize, usize, u32, usize, u32) = core::mem::transmute(FN_FIXTURE_MANAGER_COLLECT_DAY_CLASHES);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let can: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONALTEAMS_CAN_SCHEDULE_CLUB_FIXTURE);
    let mut two = two_days;
    *out = 0;
    let mut a = [0xffu8; 16];
    for k in 0..7 {
        a[k] = k as u8;
    }
    let key = key_of_game(read_usize(jogo + 0x14), jogo) as i8;
    a[7] = key as u8;
    let mut n: usize = 8;
    let home = read_usize(jogo + 0x1c);
    if home != 0 && read_usize(home + 0x57) != 0 {
        a[8] = key_of_game(read_usize(home + 0x57), 0);
        n = 9;
    }
    let away = read_usize(jogo + 0x20);
    if away != 0 && read_usize(away + 0x57) != 0 {
        a[n] = key_of_game(read_usize(away + 0x57), 0);
        n += 1;
    }
    if read_usize(jogo + 0x18) != 0 {
        a[n] = of_stadium(read_usize(jogo + 0x18));
        n += 1;
    }
    a[n] = parceira(read_usize(jogo + 0x1c));
    n += 1;
    a[n] = parceira(read_usize(jogo + 0x20));
    n += 1;
    if read_i32(jogo) != read_i32(KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP) && (key == 3 || key == 5) {
        two = 0;
    }
    if national_team(read_usize(jogo + 0x1c)) != 0 {
        a[n] = of_national_team(read_usize(jogo + 0x1c));
        n += 1;
        a[n] = of_national_team(read_usize(jogo + 0x20));
        n += 1;
    }
    for i in 0..n.saturating_sub(1) {
        for j in i + 1..n {
            if (a[i] as i8) > -1 && a[i] == a[j] {
                a[j] = 0xff;
            }
        }
    }
    let block = new(0xe);
    let list = if block != 0 { construct(block) } else { 0 };
    if list == 0 {
        show_error_at(0x9cc208, 0xc1d); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
        return 0;
    }
    let mut data = [0u8; 8];
    new_data(&mut data, read_u16(jogo + 0x2a) as u32, read_u16(jogo + 0x28) as u32);
    let mut g: i32 = 0;
    let (pa, pd) = (a.as_ptr() as usize, data.as_ptr() as usize);
    if (read_u8(jogo + 0x3a) as i8) < 0x17 {
        if two != 0 {
            of_day(this, jogo, &mut g, list, pa, n as u32, pd, -2i32 as u32);
        }
        of_day(this, jogo, &mut g, list, pa, n as u32, pd, -1i32 as u32);
    }
    of_day(this, jogo, &mut g, list, pa, n as u32, pd, 0);
    crate::comparacao::ramo("cqc_3 dia seguinte");
    of_day(this, jogo, &mut g, list, pa, n as u32, pd, 1); // cqc3: o dia seguinte
    if two != 0 {
        of_day(this, jogo, &mut g, list, pa, n as u32, pd, -2i32 as u32);
    }
    if g == 0 {
        destroy(list);
        free(list);
        if can(read_usize(G_NATIONAL_TEAMS), jogo) == 0 {
            *out = 0x15;
        }
        return 0;
    }
    *out = g;
    list
}


/// clears the list at 0xae2a48 and refills it with the 15-byte records of check_fixture_legs.tmp
/// Original: `fix_man_load_fixture_leg_checks` (0x5af390, fix_man.cpp, cdecl).
///
/// [pt] { 0x5af390 } (cdecl, fix_man.cpp): `fix_man_load_fixture_leg_checks()`, pela construção do calendário principal
/// (0x5a9cf0, na carga e na criação do jogo): esvazia a lista das conferências das pernas [0xae2a48] (0x68d080: ecx =
/// lista -> thiscall) e a refaz com os registros de 15 bytes do check_fixture_legs.tmp da pasta temporária (0x90bde0:
/// 2 pushes; add esp, 8 -> cdecl; fopen 0x945b00 "rb", fread 0x945bfe, fclose 0x945a09: cdecl), pela conta de 16 bits
/// do começo; registro curto: a caixa (linha 0x10b9) e entra o que ficou no registro (0x68d0d0).
pub(crate) unsafe extern "C" fn fix_man_load_fixture_leg_checks() {
    let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    let path: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let open: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let read: unsafe extern "C" fn(*mut u8, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
    let close: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let append: unsafe extern "thiscall" fn(usize, *const u8, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    esvaziar(G_FIXTURE_LEGS_LIST);
    let f = open(path(0x987da0, 0x9cc258), 0x98f928); // "CM3_TEMP", "check_fixture_legs.tmp", "rb"
    if f == 0 {
        return;
    }
    // simplification: com a leitura da conta falhando a original usa o lixo da pilha; aqui 0 (arquivo do próprio jogo)
    let mut count = [0u8; 2];
    read(count.as_mut_ptr(), 2, 1, f);
    let count = i16::from_le_bytes(count);
    let mut reg = [0u8; 15];
    let mut k: i16 = 0;
    while k < count {
        if read(reg.as_mut_ptr(), 0xf, 1, f) != 1 {
            show_error_at(0x9cc208, 0x10b9);
        }
        append(G_FIXTURE_LEGS_LIST, reg.as_ptr(), 0xf);
        k = k.wrapping_add(1);
    }
    close(f);
}






pub(crate) type RawDate = [u8; 8];

/// True if date a is after date b (year +2 unsigned, then day signed).
///
/// [pt] `a` depois de `b` como a original compara as datas: o ano (+2) sem sinal, depois o dia (+0) com sinal.
pub(crate) unsafe fn fix_man_date_is_after(a: usize, b: usize) -> bool {
    let (ya, yb) = (read_u16(a + 2), read_u16(b + 2));
    ya > yb || (ya == yb && (read_u16(a) as i16) > (read_u16(b) as i16))
}

/// Initialises a date to the default (0x54a110).
///
/// [pt] As operações de data (thiscall; as de +n devolvem a data estática delas ou `ret`): 0x54a110 a data padrão,
/// 0x549ef0 a data (dia, ano) (push ano; push dia), 0x417c10 a atribuição (push fonte; push ret), 0x54c5b0 + dias,
/// 0x54c8e0 − dias, 0x54c770 + semanas (push n; push ret), 0x54a2d0 o dia da semana alvo (push dest; push alvo),
/// 0x54a3f0 o dia da semana (sem pushes).
pub(crate) unsafe fn date_default(d: &mut RawDate) {
    let f: unsafe extern "thiscall" fn(*mut RawDate) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    f(d);
}
/// Builds a date from (day, year) (Date::Date).
pub(crate) unsafe fn date_from_day_year(d: &mut RawDate, day: u16, year: u16) -> usize {
    let f: unsafe extern "thiscall" fn(*mut RawDate, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    f(d, day as u32, year as u32)
}
/// Assigns date `fonte` to d (Date operator=).
pub(crate) unsafe fn date_assign(d: &mut RawDate, font: usize) {
    let f: unsafe extern "thiscall" fn(*mut RawDate, *mut RawDate, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut ret = [0u8; 8];
    f(d, &mut ret, font);
}
/// Calls a date arithmetic routine (add/sub days or weeks) with a local return buffer and returns a pointer to the
/// result.
pub(crate) unsafe fn date_op(routine: usize, d: usize, n: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, *mut RawDate, u32) -> usize = core::mem::transmute(routine);
    let mut ret = [0u8; 8];
    // simplification: a original passa um ret na pilha dela; as três escrevem nele ou na estática e devolvem o ponteiro,
    // que é lido logo depois (antes da próxima chamada), como na original
    let r = f(d, &mut ret, n);
    if r == ret.as_ptr() as usize {
        DATE_OP_RESULT = ret;
        return core::ptr::addr_of!(DATE_OP_RESULT) as usize;
    }
    r
}
pub(crate) static mut DATE_OP_RESULT: RawDate = [0; 8];
/// Weekday of a date (0x54a3f0).
pub(crate) unsafe fn date_weekday(d: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(FN_DATE_GET_WEEKDAY);
    f(d)
}
/// Writes into dest the nearest date with the target weekday (0x54a2d0).
pub(crate) unsafe fn nearest_weekday(d: usize, target: u32, dest: &mut RawDate) {
    let f: unsafe extern "thiscall" fn(usize, u32, *mut RawDate) -> u32 = core::mem::transmute(FN_DATE_GET_NEAREST_WEEKDAY);
    f(d, target, dest);
}

/// Time slice for a competition on a day (FIXTURE_MANAGER get_time_slice 0x5af2f0).
///
/// [pt] A faixa de horário (0x5af2f0: mov ecx, gerenciador; push dia; push comp -> thiscall, ret 8).
pub(crate) unsafe fn fixture_time_slice(this: usize, comp: usize, day: u32) -> u8 {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(FN_FIXTURE_MANAGER_GET_TIME_SLICE);
    f(this, comp, day)
}

/// Clashes of a fixture (0x5ad780): returns (clash list or 0, severity).
///
/// [pt] Os conflitos do jogo (0x5ad780: mov ecx, gerenciador; push marca; push &gravidade; push jogo -> thiscall, ret
/// 0xc): a lista (ou 0) e a gravidade.
pub(crate) unsafe fn fix_man_find_fixture_clashes(this: usize, jogo: usize, two: u32) -> (usize, i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURE_CLASHES);
    let mut g: i32 = 0;
    let l = f(this, jogo, &mut g, two);
    (l, g)
}

/// Destroys and deletes a LLIST.
///
/// [pt] Destrói e libera a lista (0x68d040: mov ecx, lista -> thiscall; 0x944c94: push; add esp, 4 -> cdecl).
pub(crate) unsafe fn free_list(l: usize) {
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    destroy(l);
    free(l);
}

/// the window by fixture type; in two passes, weekdays in the order 2, 1, 5, 6, 0, 3, 4, every 4 days, from 2 days
/// after today: the first date with no clash (or severity 1 or below the type) stays, with the time slot; otherwise 2
/// days later; for rounds ending in 3 and 4, the leg check record
/// Original: `FIXTURE_MANAGER::find_new_fixture_date` (0x5ab510, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5ab510 } (thiscall, ret 4, fix_man.cpp; quadro SEH só das datas): `FIXTURE_MANAGER::find_new_fixture_date(jogo)`,
/// pela entrada no calendário (0x5aa680), pelos conflitos do mesmo dia (0x5ae380) e pela remarcação (0x5ae9d0). Sem
/// jogo: a caixa (a original). Pela casa das unidades da rodada (+0x30, 0x4b0850: push; add esp, 4 -> cdecl) 3 ou 4,
/// o registro de 15 bytes do jogo na lista de conferência das pernas [0xae2a48] (0x68d0d0). A janela pelo tipo (+0x3a):
/// 2, até a data de +0x24/+0x26 (sem ela, a caixa e 2 semanas), começando 4 semanas depois (se o fim passa de 20
/// semanas) ou 3 dias depois; 4 e 6, de 3 dias a 2 semanas; 5, de 0 a 1 semana; 7, 8 e 10, de 3 dias antes a 1 semana;
/// os outros, nenhuma. Em duas passadas (a segunda olhando os dois dias antes), pelos dias da semana na ordem 2, 1, 5,
/// 6, 0, 3, 4, de 4 em 4 dias da janela: o dia alvo (0x54a2d0) a partir de 2 dias depois de hoje e dentro da janela
/// vira a data (+0x28, +0x2a) e a faixa (+0x3f, 0x5af2f0); sem conflito (0x5ad780), com gravidade 1 ou abaixo do tipo,
/// fica. Sem achar: 2 dias depois da data do jogo (ou de hoje, se já passou).
pub(crate) unsafe extern "thiscall" fn find_new_fixture_date(this: usize, jogo: usize) {
    if jogo == 0 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_FIND_NEW_FIXTURE_DATE)(this, jogo); // a caixa
    }
    let (mut s, mut e, mut j, mut h, mut k) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    date_default(&mut s);
    date_default(&mut e);
    date_default(&mut j);
    date_default(&mut h);
    date_default(&mut k);
    let days: [i16; 9] = [2, 1, -1, 5, 6, -1, 0, 3, 4]; // cqc4: a ordem dos dias da semana
    let mut tmp = [0u8; 8];
    let r = date_from_day_year(&mut tmp, read_u16(jogo + 0x2a), read_u16(jogo + 0x28));
    date_assign(&mut j, r);
    let units_digit: unsafe extern "C" fn(i32) -> u8 = core::mem::transmute(FN_COMP_UTIL_GET_ROUND_VARIANT);
    let round = read_u16(jogo + 0x30) as i16 as i32;
    if units_digit(round) == 3 || units_digit(round) == 4 {
        let mut reg = [0u8; 15];
        reg[0..4].copy_from_slice(&core::slice::from_raw_parts(jogo as *const u8, 4));
        reg[4] = read_u8(jogo + 0x42);
        reg[5..7].copy_from_slice(&read_u16(jogo + 0x34).to_le_bytes());
        reg[7..11].copy_from_slice(&core::slice::from_raw_parts((jogo + 0xc) as *const u8, 4));
        reg[11..15].copy_from_slice(&core::slice::from_raw_parts((jogo + 0x10) as *const u8, 4));
        let append: unsafe extern "thiscall" fn(usize, *const u8, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
        append(G_FIXTURE_LEGS_LIST, reg.as_ptr(), 0xf);
    }
    let pj = j.as_ptr() as usize;
    match (read_u8(jogo + 0x3a) as i8 as i32).wrapping_sub(2) {
        0 => {
            if read_u16(jogo + 0x24) != 0 {
                let mut t = [0u8; 8];
                let r = date_from_day_year(&mut t, read_u16(jogo + 0x26), read_u16(jogo + 0x24));
                date_assign(&mut e, r);
            } else {
                show_error_at(0x9cc208, 0x4f9); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
                let r = date_op(0x54c770, pj, 2);
                date_assign(&mut e, r);
            }
            let end = e;
            let r = date_op(0x54c770, pj, 0x14);
            let r = if fix_man_date_is_after(end.as_ptr() as usize, r) { date_op(0x54c770, pj, 4) } else { date_op(0x54c5b0, pj, 3) };
            date_assign(&mut s, r);
        }
        2 | 4 => {
            let r = date_op(0x54c770, pj, 2);
            date_assign(&mut e, r);
            let r = date_op(0x54c5b0, pj, 3);
            date_assign(&mut s, r);
        }
        3 => {
            let r = date_op(0x54c770, pj, 1);
            date_assign(&mut e, r);
            date_assign(&mut s, pj);
        }
        5 | 6 | 8 => {
            let r = date_op(0x54c770, pj, 1);
            date_assign(&mut e, r);
            let r = date_op(0x54c8e0, pj, 3);
            date_assign(&mut s, r);
        }
        _ => {}
    }
    let (ps, pe, ph, pk) = (s.as_ptr() as usize, e.as_ptr() as usize, h.as_ptr() as usize, k.as_ptr() as usize);
    for passada in 0..2u8 {
        for i in 0..3usize {
            for jj in 0..3usize {
                let target = days[i * 3 + jj];
                if target == -1 {
                    continue;
                }
                date_assign(&mut k, ps);
                while fix_man_date_is_after(pe, pk) {
                    nearest_weekday(pk, target as u16 as u32, &mut h);
                    let t = date_op(0x54c5b0, G_CURRENT_DATE, 2);
                    let t = [read_usize(t), read_usize(t + 4)];
                    let pt = t.as_ptr() as usize;
                    if !fix_man_date_is_after(pt, ph) && !fix_man_date_is_after(ph, pe) {
                        crate::comparacao::ramo("cqc_4 nova data");
                        write_u16(jogo + 0x28, read_u16(ph + 2));
                        write_u16(jogo + 0x2a, read_u16(ph));
                        *((jogo + 0x3f) as *mut u8) = fixture_time_slice(this, read_usize(jogo + 0x14), target as u16 as u32);
                        let (l, g) = fix_man_find_fixture_clashes(this, jogo, (passada == 1) as u32);
                        if l == 0 {
                            if g == 0 {
                                return;
                            }
                        } else {
                            free_list(l);
                        }
                        if g == 1 || (read_u8(jogo + 0x3a) as i8 as i32) > g {
                            return;
                        }
                    }
                    let r = date_op(0x54c5b0, pk, 4); // cqc4: o passo da busca
                    date_assign(&mut k, r);
                }
            }
        }
    }
    if fix_man_date_is_after(G_CURRENT_DATE, pj) {
        date_assign(&mut j, G_CURRENT_DATE);
    }
    let r = date_op(0x54c5b0, pj, 2);
    date_assign(&mut j, r);
    write_u16(jogo + 0x2a, read_u16(pj));
    write_u16(jogo + 0x28, read_u16(pj + 2));
}


/// fixes the stage, gives a late fixture a new date, moves a weekend league fixture to the home club's preferred day,
/// resolves the clashes (same day, competition refusal, new date or removing the clashing fixtures with the news),
/// handles national team fixtures and neighbours and puts the fixture in the day list, or in the other year's block
/// Original: `FIXTURE_MANAGER::add_fixture` (0x5aa680, fix_man.cpp, thiscall, ret 8).
///
/// [pt] { 0x5aa680 } (thiscall, ret 8, fix_man.cpp; quadro SEH das datas e do gerenciador novo): `FIXTURE_MANAGER::add_fixture(jogo,
/// zerar)`, pelas copas, ligas, amistosos e remarcações (20 chamadores). `zerar` zera a conta das remarcações em
/// cadeia (palavra [0xae2a40]). Acerta a fase (0x5af180: push; add esp, 4 -> cdecl), +0x2e = −1; jogo de data já
/// passada e não jogado: nova data (0x5ab510). Jogo da liga do mandante (+0x57 ou +0x60) sem rodada (+0x30, +0x32) nem
/// fase (+0x3b), uma vez (bit 0x800 de +0x4d): no sábado ou domingo, vai ao dia preferido do mandante (+0x72: 4, 5 ou
/// 6) se lá não houver conflito. Com menos de 10 remarcações em cadeia, os conflitos (0x5ad780): gravidade 0x14, o
/// conflito do mesmo dia (0x5ae380) e 1; copa ou tipo 9 recusado pela competição (+0x64): 0; gravidade de pelo menos o
/// tipo: 0 se a seleção do jogo tem convocação ([0xb74434], 0x7e6f20), senão nova data e entra de novo; senão entra
/// (0x5ab230) e os jogos do conflito saem: os amistosos (0x5c6710/0x5c6570 em [0xae2c74]), o tipo 9 (0x5abc30), os
/// outros remarcados (0x5abc30, 0x5ab510, 0x5aa680) com a notícia (0x5aac70 em [0xae2a58]). No ano do bloco (+0x1130):
/// a nação-chave (0x48dd70), a ordem (0x5ab3c0), os jogos de seleções (0x7e6720 em [0xb74434], 0x76bab0 em [0xb74318]),
/// os vizinhos (0x5ae9d0, chave 3 ou 5); se não entrou, na lista do dia, fatia e chave (0x68d0d0). Fora do ano, no
/// bloco do outro ano (+0x112c: 0x5aa2d0 para o ano, ou novo de 0x1184 bytes, 0x5a9cf0; sem memória, a caixa e 0).
pub(crate) unsafe extern "thiscall" fn add_fixture(this: usize, jogo: usize, zerar: u32) -> u32 {
    let stage: unsafe extern "C" fn(usize) = core::mem::transmute(FN_FIX_MAN_RECORD_CLUB_FIXTURE_YEARS);
    let new_data: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_NEW_FIXTURE_DATE);
    let entrar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_ADD_FIXTURE);
    let contagem = G_ADD_FIXTURE_RETRY_COUNT as *mut i16;
    let mut entrou = false;
    if zerar != 0 {
        *contagem = 0;
    }
    stage(jogo);
    write_u16(jogo + 0x2e, 0xffff);
    let today = [read_usize(G_CURRENT_DATE), read_usize(G_CURRENT_DATE_LEAP)];
    let mut dj = [0u8; 8];
    let r = date_from_day_year(&mut dj, read_u16(jogo + 0x2a), read_u16(jogo + 0x28));
    if !fix_man_date_is_after(r, today.as_ptr() as usize) && read_u8(jogo + 0x43) == 0xff {
        new_data(this, jogo);
    }
    let marks = read_u16(jogo + 0x4d);
    let home = read_usize(jogo + 0x1c);
    let comp = read_usize(jogo + 0x14);
    if marks & 0x800 == 0
        && (comp == read_usize(home + 0x57) || comp == read_usize(home + 0x60))
        && read_u16(jogo + 0x32) == 0
        && read_u16(jogo + 0x30) == 0
        && read_u8(jogo + 0x3b) == 0xff
    {
        write_u16(jogo + 0x4d, marks | 0x800);
        let preferido = read_u8(home + 0x72);
        if preferido != 0xff && (preferido == 5 || preferido == 6 || preferido == 4) {
            let mut d = [0u8; 8];
            date_from_day_year(&mut d, read_u16(jogo + 0x2a), read_u16(jogo + 0x28));
            let pd = d.as_ptr() as usize;
            let day = date_weekday(pd) as u8;
            if preferido != day && (day == 5 || day == 6) {
                let mut t = [0u8; 8];
                date_default(&mut t);
                crate::comparacao::ramo("cqc_4 dia preferido do mandante");
                nearest_weekday(pd, preferido as i8 as i16 as u16 as u32, &mut t); // cqc4: o dia preferido do mandante
                let pt = t.as_ptr() as usize;
                write_u16(jogo + 0x28, read_u16(pt + 2));
                write_u16(jogo + 0x2a, read_u16(pt));
                *((jogo + 0x3f) as *mut u8) = fixture_time_slice(this, read_usize(jogo + 0x14), date_weekday(pt) as u32);
                let (l, g) = fix_man_find_fixture_clashes(this, jogo, 0);
                if l != 0 || g != 0 {
                    write_u16(jogo + 0x2a, read_u16(pd));
                    write_u16(jogo + 0x28, read_u16(pd + 2));
                    *((jogo + 0x3f) as *mut u8) = fixture_time_slice(this, read_usize(jogo + 0x14), date_weekday(pd) as u32);
                    if l != 0 {
                        free_list(l);
                    }
                }
            }
        }
    }
    if *contagem < 10 {
        let (l, g) = fix_man_find_fixture_clashes(this, jogo, 0);
        if l != 0 || g != 0 {
            *contagem = (*contagem).wrapping_add(1);
            if g == 0x14 {
                crate::comparacao::ramo("cqc_4 mesmo dia");
                let same_day: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_RESOLVE_SAME_DAY_CLASH);
                same_day(this, jogo, l); // cqc4: o conflito do mesmo dia
                if l != 0 {
                    free_list(l);
                }
                return 1;
            }
            let kind = read_u8(jogo + 0x3a);
            if (kind == 1 || kind == 9) && read_usize(jogo + 0x14) != 0 && read_usize(G_COMP_OBJECT_TABLE) != 0 {
                let c = read_usize(read_usize(G_COMP_OBJECT_TABLE).wrapping_add((read_usize(read_usize(jogo + 0x14)) as usize).wrapping_mul(4)));
                if c != 0 {
                    let accepted: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(read_usize(read_usize(c) + 0x64));
                    if accepted(c, jogo) == 0 {
                        if l != 0 {
                            free_list(l);
                        }
                        return 0;
                    }
                }
            }
            // 0x7e6f20: mov ecx, [0xb74434]; push comp -> thiscall, ret 4
            let convocada: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_IS_RESERVE_OR_MINOR_COMP);
            if g >= read_u8(jogo + 0x3a) as i8 as i32 {
                if l != 0 {
                    free_list(l);
                }
                if convocada(read_usize(G_RESERVE_COMP), read_usize(jogo + 0x14)) != 0 {
                    return 0;
                }
                new_data(this, jogo);
                return (entrar(this, jogo, 0) != 0) as u32;
            }
            let store: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_INSERT_FIXTURE);
            store(this, jogo);
            entrou = true;
            if l != 0 {
                let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
                let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
                let remove_friendly: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_FRIENDLY_ON_FRIENDLY_FIXTURE_CANCELLED);
                let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_REMOVE_FIXTURE);
                let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_FIXTURE_MANAGER_QUEUE_FIXTURE_MOVED_NEWS);
                core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2));
                let mut x = next(l);
                while x != 0 {
                    if friendly(read_usize(G_FRIENDLY), read_usize(x + 0x14)) != 0 {
                        remove_friendly(read_usize(G_FRIENDLY), x, 0);
                    } else if read_u8(x + 0x3a) == 9 {
                        remove(this, x);
                    } else {
                        if convocada(read_usize(G_RESERVE_COMP), read_usize(jogo + 0x14)) != 0 {
                            return 0; // como na original, sem liberar a lista
                        }
                        let mut copy = [0u8; 0x4f];
                        core::ptr::copy_nonoverlapping(x as *const u8, copy.as_mut_ptr(), 0x4f);
                        let pc = copy.as_mut_ptr() as usize;
                        if remove(this, x) != 0 {
                            new_data(this, pc);
                            entrar(this, pc, 0);
                        }
                        noticia(read_usize(G_FIXTURE_MANAGER), pc, jogo, 0);
                    }
                    x = next(l);
                }
                free_list(l);
            }
        }
    }
    let year = read_u16(jogo + 0x28);
    let mut key: i8 = 0;
    if year == read_u16(this + 0x1130) {
        let key_of_game: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(FN_COMP_ACTIVE_KEY_NATION_OF_FIXTURE);
        key = key_of_game(read_usize(jogo + 0x14), jogo) as i8;
        let order: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_ADD_AWAY_TRIP_ILLNESS_RISK);
        order(this, jogo);
        let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
        if national_team(read_usize(jogo + 0x1c)) != 0 && national_team(read_usize(jogo + 0x20)) != 0 && read_u8(jogo + 0x43) == 0xff {
            let c = read_usize(jogo + 0x14);
            let id = read_usize(c) as i32;
            if id != read_usize(KEY_CLUB_COMP_B_INTERNATIONAL) as i32 && id != read_usize(KEY_CLUB_COMP_UNDER_21_INTERNATIONAL) as i32 && read_u8(jogo + 0x4e) & 3 == 0 {
                let special: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
                let friendly: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
                if special(read_usize(G_NATIONAL_TEAMS), c) == 0 || friendly(read_usize(G_FRIENDLY), read_usize(jogo + 0x14)) != 0 {
                    let mark: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_RESERVECOMP_ADD_B_OR_UNDER21_FIXTURE);
                    mark(read_usize(G_RESERVE_COMP), jogo);
                }
            }
            let jogo_de_selecoes: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_SELECT_SQUADS_FOR_FIXTURE);
            jogo_de_selecoes(read_usize(G_NATIONAL_TEAMS), jogo);
        }
        if key == 3 || key == 5 {
            let vizinhos: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_MOVE_NEARBY_CLUB_FIXTURES);
            vizinhos(this, jogo);
        }
    }
    if entrou {
        return 1;
    }
    let year = read_u16(jogo + 0x28);
    if year == read_u16(this + 0x1130) {
        let day = read_u16(jogo + 0x2a) as usize;
        let slice = read_u8(jogo + 0x3f) as usize;
        let row = read_usize(this + (day * 3 + slice) * 4);
        let list = read_usize(row.wrapping_add((key as isize as usize).wrapping_mul(4)));
        let append: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LLIST_ADD_TAIL);
        return (append(list, jogo, 0x4f) != 0) as u32;
    }
    let mut other = read_usize(this + 0x112c);
    if other != 0 {
        if read_u16(other + 0x1130) != year {
            let year_of_block: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_SET_YEAR);
            year_of_block(other, year as u32);
        }
    } else {
        let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
        let construct: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_FIXTURE_MANAGER);
        let b = new(0x1184);
        other = if b != 0 { construct(b, year as u32, this, 0) } else { 0 };
        core::ptr::write_unaligned((this + 0x112c) as *mut usize, other);
        if other == 0 {
            show_error_at(0x9cc208, 0x2e6);
            return 0;
        }
    }
    entrar(read_usize(this + 0x112c), jogo, 0);
    1
}


/// saves the year's fixtures, moves the other block to the neighbouring year if needed, destroys the 366 days' lists,
/// switches the year and leap flag, creates the new lists and loads the new year's fixtures
/// Original: `FIXTURE_MANAGER::set_year` (0x5aa2d0, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5aa2d0 } (thiscall, ret 4, fix_man.cpp; quadro SEH dos blocos novos): `FIXTURE_MANAGER::set_year(ano)`, pela entrada
/// no calendário (0x5aa680, o bloco do outro ano), pela guarda e pela retirada de jogos e pelos amistosos. O mesmo ano
/// (palavra, +0x1130): 1. Grava os jogos do ano (0x5ac060: ecx = bloco; push 0 -> thiscall; falhando, a caixa e 0); se
/// o outro bloco (+0x112c) está no ano pedido, ele vai para o ano vizinho do deste (0x5aa2d0 nele); destrói e libera as
/// listas dos 366 dias × 3 fatias × nações-chave ([0xb64738], byte com sinal; 0x68d040, 0x944c94); o outro bloco no
/// ano pedido, se este é o principal (+0x1183), é destruído e liberado (0x5aa0d0: ecx = bloco -> thiscall; 0x944c94); o
/// ano novo, bissexto em +0x1132 (0x54cfe0: push ano; add esp, 4 -> cdecl), as listas novas (new 0xe: 0x944cf1, push;
/// add esp, 4 -> cdecl; 0x68d020: ecx = bloco -> thiscall; sem memória, a caixa e 0) e a carga dos jogos do ano
/// (0x5ac3b0: ecx = bloco -> thiscall). Devolve 1.
pub(crate) unsafe extern "thiscall" fn set_year(this: usize, year: u32) -> u32 {
    let a = year as u16;
    if a == read_u16(this + 0x1130) {
        return 1;
    }
    let write: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_SAVE_FIXTURES);
    if write(this, 0) == 0 {
        show_error_at(0x9cc208, 0x126);
        return 0;
    }
    let other = read_usize(this + 0x112c);
    if other != 0 {
        let oa = read_u16(other + 0x1130);
        if oa == a {
            let vizinho = if read_u16(this + 0x1130) as u32 == oa as u32 + 1 { oa.wrapping_sub(1) } else { oa.wrapping_add(1) };
            let mudar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_SET_YEAR);
            mudar(other, vizinho as u32);
        }
    }
    let n = || read_u8(G_KEY_NATION_ACTIVE_COUNT) as i8;
    for dia in 0..0x16eusize {
        if n() > 0 {
            let base = this + 8 + dia * 0xc;
            let mut k: i16 = 0;
            loop {
                for p in [base - 8, base - 4, base] {
                    let slot = read_usize(p).wrapping_add((k as i32 as usize).wrapping_mul(4));
                    let l = read_usize(slot);
                    if l != 0 {
                        free_list(l);
                        core::ptr::write_unaligned(read_usize(p).wrapping_add((k as i32 as usize).wrapping_mul(4)) as *mut usize, 0);
                    }
                }
                k = k.wrapping_add(1);
                if k >= n() as i16 {
                    break;
                }
            }
        }
    }
    let other = read_usize(this + 0x112c);
    if other != 0 && read_u16(other + 0x1130) == a && read_u8(this + 0x1183) != 0 {
        let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FIXTURE_MANAGER_DESTRUCTOR_FIXTURE_MANAGER);
        let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
        destroy(other);
        free(other);
        core::ptr::write_unaligned((this + 0x112c) as *mut usize, 0);
    }
    write_u16(this + 0x1130, a);
    let bissexto: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATE_IS_LEAP_YEAR);
    *((this + 0x1132) as *mut u8) = (bissexto(year) != 0) as u8;
    let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construct: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    crate::comparacao::ramo("cqc_4 ano novo do calendario");
    for dia in 0..0x16eusize {
        if n() > 0 {
            let e = this + dia * 0xc;
            let mut k: i16 = 0;
            loop {
                let desloc = (k as i32 as usize).wrapping_mul(4);
                for j in 0..3usize {
                    let b = new(0xe);
                    let l = if b != 0 { construct(b) } else { 0 };
                    core::ptr::write_unaligned(read_usize(e + j * 4).wrapping_add(desloc) as *mut usize, l);
                }
                if (0..3usize).any(|j| read_usize(read_usize(e + j * 4).wrapping_add(desloc)) == 0) {
                    show_error_at(0x9cc208, 0x170);
                    return 0;
                }
                k = k.wrapping_add(1);
                if k >= n() as i16 {
                    break;
                }
            }
        }
    }
    let load: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_FIXTURE_MANAGER_LOAD_FIXTURES);
    load(this);
    1
}



/// Opens a file by name in the CM3_TEMP folder with fopen and the given mode.
///
/// [pt] Abre `nome` em CM3_TEMP (0x90bde0: cdecl(pasta, nome); fopen 0x945b00: cdecl(caminho, modo)).
pub(crate) unsafe fn open_temp_file(name: usize, mode: usize) -> usize {
    let path: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let open: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    open(path(0x987da0, name), mode)
}


/// writes the [0xae2a5c] table, one word per club ([0xae235c]), into CM3_TEMP\club_dates.tmp; without the table or on a
/// short write, the "Error" box (fix_man.cpp 0x1011, 0x101b)
/// Original: `fix_man_save_club_dates` (0x5af040, fix_man.cpp, cdecl).
///
/// [pt] { 0x5af040 } (cdecl, fix_man.cpp): `fix_man_save_club_dates()`, do save do calendário (FIXTURE_MANAGER::save
/// 0x5ac060, 0x5ac394): sem a tabela das datas dos clubes ([0xae2a5c]), a caixa da linha 0x1011. Grava em
/// CM3_TEMP\club_dates.tmp ("wb") uma palavra por clube ([0xae235c] delas, fwrite 0x945ce6); gravação curta: a caixa
/// da linha 0x101b; fecha. Sem abrir: nada.
pub(crate) unsafe extern "C" fn fix_man_save_club_dates() {
    let write: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
    if read_usize(G_CLUB_FIXTURE_YEAR_TABLE) == 0 {
        return show_error_at(0x9cc208, 0x1011);
    }
    let f = open_temp_file(0x9cc248, 0x9a7f28);
    if f == 0 {
        return;
    }
    crate::comparacao::ramo("cpy_7 gravar datas dos clubes");
    if write(read_usize(G_CLUB_FIXTURE_YEAR_TABLE), 2, club_count(), f) != club_count() { // cpy7: as datas dos clubes
        show_error_at(0x9cc208, 0x101b);
    }
    uefa_seeding_crt_fclose(f);
}


/// allocates the [0xae2a5c] table (one word per club) and reads it from CM3_TEMP\club_dates.tmp, or zeroes it without
/// the file; already allocated, out of memory or on a short read, the "Error" box (0xfe4, 0xfec, 0xff6)
/// Original: `fix_man_load_club_dates` (0x5aee90, fix_man.cpp, cdecl).
///
/// [pt] { 0x5aee90 } (cdecl, fix_man.cpp): `fix_man_load_club_dates()`, do construtor do calendário (0x5a9cf0, 0x5a9e38:
/// na carreira nova e na carga do save). Com a tabela já alocada ([0xae2a5c]), a caixa da linha 0xfe4. Aloca uma
/// palavra por clube (malloc 0x944e46 de 2 × [0xae235c]; sem memória, a caixa da linha 0xfec); lê de
/// CM3_TEMP\club_dates.tmp ("rb", fread 0x945bfe; leitura curta: a caixa da linha 0xff6) e fecha; sem o arquivo (a
/// carreira nova), zera a tabela.
pub(crate) unsafe extern "C" fn fix_man_load_club_dates() {
    let alloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let read: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
    if read_usize(G_CLUB_FIXTURE_YEAR_TABLE) != 0 {
        return show_error_at(0x9cc208, 0xfe4);
    }
    let t = alloc(club_count().wrapping_mul(2));
    *(G_CLUB_FIXTURE_YEAR_TABLE as *mut usize) = t;
    if t == 0 {
        return show_error_at(0x9cc208, 0xfec);
    }
    let f = open_temp_file(0x9cc248, 0x98f928);
    if f == 0 {
        crate::comparacao::ramo("cpy_7 datas dos clubes zeradas");
        core::ptr::write_bytes(read_usize(G_CLUB_FIXTURE_YEAR_TABLE) as *mut u8, 0, club_count().wrapping_mul(2) as usize);
        return;
    }
    crate::comparacao::ramo("cpy_7 datas dos clubes lidas");
    if read(read_usize(G_CLUB_FIXTURE_YEAR_TABLE), 2, club_count(), f) != club_count() { // cpy7: a leitura das datas
        show_error_at(0x9cc208, 0xff6);
    }
    uefa_seeding_crt_fclose(f);
}


/// with the [0xae2a48] list not empty, writes the count (2 bytes) and the 15-byte items into
/// CM3_TEMP\check_fixture_legs.tmp; on a short item write, the "Error" box (0x10e0)
/// Original: `fix_man_save_fixture_leg_checks` (0x5af4a0, fix_man.cpp, cdecl).
///
/// [pt] { 0x5af4a0 } (cdecl, fix_man.cpp): `fix_man_save_fixture_leg_checks()`, do save do calendário (0x5ac399): com a
/// lista de conferência das pernas ([0xae2a48], LLIST) vazia (a palavra da conta zero), nada. Grava em
/// CM3_TEMP\check_fixture_legs.tmp ("wb") a conta (2 bytes) e, do começo da lista (o cursor +0xa ← a cabeça +2), cada
/// item de 15 bytes (0x68d1b0; gravação curta: a caixa da linha 0x10e0, e segue); fecha. Sem abrir: nada.
pub(crate) unsafe extern "C" fn fix_man_save_fixture_leg_checks() {
    let write: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let count = read_usize(G_FIXTURE_LEGS_LIST);
    if count as u16 == 0 {
        return;
    }
    let f = open_temp_file(0x9cc258, 0x9a7f28);
    if f == 0 {
        return;
    }
    crate::comparacao::ramo("cpy_7 gravar conferencia das pernas");
    write(&count as *const usize as usize, 2, 1, f);
    *(G_FIXTURE_LEGS_LIST_CURSOR as *mut usize) = read_usize(G_FIXTURE_LEGS_LIST_HEAD);
    loop {
        let item = next(G_FIXTURE_LEGS_LIST);
        if item == 0 {
            break;
        }
        if write(item, 0xf, 1, f) != 1 {
            show_error_at(0x9cc208, 0x10e0);
        }
    }
    uefa_seeding_crt_fclose(f);
}


/// Modo de ensaio da 0x5ae380 (o comparador): a cópia para na decisão, antes de mexer no calendário.
pub(crate) static mut SAME_DAY_CLASH_DRY_RUN: bool = false;



/// the new fixture and the first one on the same day: the higher priority stays (the competition one, +21 with a
/// national team at home; ties by the home club reputation), as long as the other was not played yet; the one that
/// leaves goes to the target weekday from the other date (weekdays 0 and 5 go to 6, 1 and 3 to 2, 2 to 1, 4 and 6 to 5)
/// or, with a clash there, to another date, with the rescheduling news; a fixed fixture with a clash does not move
/// (returns 0)
/// Original: `FIXTURE_MANAGER::resolve_same_day_clash` (0x5ae380, fix_man.cpp, thiscall).
///
/// [pt] { 0x5ae380 } (thiscall, ret 8, fix_man.cpp, quadro SEH sem destrutor no caminho normal):
/// `FIXTURE_MANAGER::resolve_same_day_clash(jogo, conflitos)`: o jogo novo `jogo` cai no mesmo dia de outro, o primeiro
/// da lista `conflitos` (o cursor +0xa volta ao começo +2 e 0x68d1b0 o lê); sem um dos dois, a caixa da linha 0xde6
/// e devolve 1. Copia os dois (0x4f bytes) e acha o dia da semana alvo pela data do da lista (+0x2a, +0x28; 0x549ef0,
/// 0x417c10, 0x54a3f0): os dias 0 e 5 da 0x54a3f0 vão a 6, 1 e 3 a 2, o 2 a 1, 4 e 6 a 5 (0x54a2d0 dá a
/// data). A prioridade de cada um é a da competição (+0x69 da +0x14), +0x15 com seleção
/// como mandante (+0x1c, 0x53b900); o da lista sai do dia quando a do novo é maior (ou igual, com a reputação +0x80
/// do mandante do novo maior) e ele ainda não foi jogado (+0x43 = 0xff): tirado do calendário (0x5abc30), o novo entra
/// (0x5aa680); senão sai o novo. O que sai ganha a data nova; com conflito nela (0x5ad780, a lista destruída e
/// liberada: 0x68d040, 0x944c94), jogo fixo (+0x3a = 1) devolve 0, senão 0x5ab510 acha outra data; entra no calendário
/// (0x5aa680) com a notícia da remarcação (0x5aac70, causada pelo outro). Devolve 1.
pub(crate) unsafe extern "thiscall" fn resolve_same_day_clash(this: usize, jogo: usize, conflitos: usize) -> u32 {
    // as datas: ecx = a data, sem pushes -> thiscall (0x54a110, 0x54a3f0); 0x549ef0: push [+0x28]; push [+0x2a];
    // 0x417c10: push ret; push &tmp; 0x54a2d0: push &b; push alvo -> thiscall
    let default: unsafe extern "thiscall" fn(*mut [u8; 8]) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let new: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> *mut [u8; 8] = core::mem::transmute(FN_DATE_DATE);
    let copy: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], *mut [u8; 8]) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let week: unsafe extern "thiscall" fn(*mut [u8; 8]) -> i32 = core::mem::transmute(FN_DATE_GET_WEEKDAY);
    let adjust: unsafe extern "thiscall" fn(*mut [u8; 8], u32, *mut [u8; 8]) -> u32 = core::mem::transmute(FN_DATE_GET_NEAREST_WEEKDAY);
    // 0x68d1b0: ecx = a lista, sem pushes -> thiscall; 0x53b900: push clube; add esp, 4 -> cdecl
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let (mut a, mut b) = ([0u8; 8], [0u8; 8]);
    default(&mut a);
    default(&mut b);
    let mut severity: u32 = 0; // [esp+0x10]: o alvo sem caso da tabela e a gravidade da 0x5ad780
    core::ptr::write_unaligned((conflitos + 0xa) as *mut usize, read_usize(conflitos + 2));
    let other = next(conflitos);
    if jogo == 0 || other == 0 {
        if !SAME_DAY_CLASH_DRY_RUN {
            show_error_at(0x9cc208, 0xde6); // "E:\dev\CM3\cm3 00-01\cm3\code\comp\fix_man.cpp"
        }
        return 1;
    }
    let (mut novo, mut velho) = ([0u8; 0x50], [0u8; 0x50]);
    core::ptr::copy_nonoverlapping(jogo as *const u8, novo.as_mut_ptr(), 0x4f);
    core::ptr::copy_nonoverlapping(other as *const u8, velho.as_mut_ptr(), 0x4f);
    let (mut tmp, mut tmp2) = ([0u8; 8], [0u8; 8]);
    // simplification: os bytes de cima dos dois pushes de 16 bits não contam (0x549ef0 lê a palavra)
    let r = new(&mut tmp, read_i16(other + 0x2a) as u16 as u32, read_i16(other + 0x28) as u16 as u32);
    copy(&mut a, &mut tmp2, r);
    let target = match week(&mut a) as i16 {
        0 | 5 => 6,
        1 | 3 => 2,
        2 => 1,
        4 | 6 => 5,
        _ => severity,
    };
    crate::comparacao::ramo("cqb_13 dia alvo");
    adjust(&mut a, target, &mut b); // cqb13: o dia da semana alvo
    let n = novo.as_mut_ptr() as usize;
    let v = velho.as_mut_ptr() as usize;
    let mut pn = read_i16(read_usize(n + 0x14) + 0x69);
    let mut pv = read_i16(read_usize(v + 0x14) + 0x69);
    if national_team(read_usize(n + 0x1c)) != 0 {
        pn = pn.wrapping_add(0x15);
    }
    if national_team(read_usize(v + 0x1c)) != 0 {
        pv = pv.wrapping_add(0x15);
    }
    let novo_vence = pn > pv // cqb13: a prioridade maior fica
        || (pn == pv && read_i16(read_usize(n + 0x1c) + 0x80) > read_i16(read_usize(v + 0x1c) + 0x80));
    if novo_vence {
        crate::comparacao::ramo("cqb_13 novo vence");
    }
    let sai_o_velho = novo_vence && *((v + 0x43) as *const u8) == 0xff; // cqb13: o da lista ainda não jogado
    if SAME_DAY_CLASH_DRY_RUN {
        return 1;
    }
    let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_REMOVE_FIXTURE);
    let join: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_ADD_FIXTURE);
    let conflito: unsafe extern "thiscall" fn(usize, usize, *mut u32, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURE_CLASHES);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let other_data: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_NEW_FIXTURE_DATE);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_FIXTURE_MANAGER_QUEUE_FIXTURE_MOVED_NEWS);
    // as do calendário: ecx = this; push o jogo (0x5abc30, 0x5ab510; ret 4), push 0; push o jogo (0x5aa680; ret 8),
    // push 0; push &gravidade; push o jogo (0x5ad780; ret 0xc), push 0; push causa; push o jogo (0x5aac70; ret 0xc);
    // 0x68d040: ecx = a lista; 0x944c94: push lista; add esp, 4 -> cdecl
    let (sai, causa) = if sai_o_velho {
        crate::comparacao::ramo("cqb_13 sai o jogo da lista");
        remove(this, other);
        join(this, jogo, 0);
        (v, jogo)
    } else {
        (n, other)
    };
    core::ptr::write_unaligned((sai + 0x2a) as *mut u16, u16::from_le_bytes([b[0], b[1]]));
    core::ptr::write_unaligned((sai + 0x28) as *mut u16, u16::from_le_bytes([b[2], b[3]]));
    let list = conflito(this, sai, &mut severity, 0);
    if list != 0 {
        destroy(list);
        free(list);
        if *((sai + 0x3a) as *const u8) == 1 {
            return 0;
        }
        other_data(this, sai);
    }
    join(this, sai, 0);
    noticia(this, sai, causa, 0);
    1
}


/// the club's first year with fixtures: 2001, or 2002 by bit 0 of its word in [0xae2a5c]
/// Original: `FIXTURE_MANAGER::get_club_first_fixture_year` (0x5aee40, fix_man.cpp, thiscall).
///
/// [pt] { 0x5aee40 } (thiscall, ret 4, fix_man.cpp; o this não é lido): `FIXTURE_MANAGER::get_club_first_fixture_year(&id)`:
/// o primeiro ano com jogos do clube `[id]`: 2001, ou 2002 com o bit 0 da palavra dele em [0xae2a5c]. Devolve `ax`
/// (o chamador da tela compara 16 bits).
pub(crate) unsafe extern "thiscall" fn get_club_first_fixture_year(_this: usize, id: usize) -> u16 {
    let b = *((read_usize(G_CLUB_FIXTURE_YEAR_TABLE) as isize).wrapping_add((*(id as *const i32) as isize).wrapping_mul(2)) as *const u8);
    0x7d1 + (b & 1) as u16
}

/// the club's last year with fixtures: its word in [0xae2a5c] without bit 0, or the current year
/// Original: `FIXTURE_MANAGER::get_club_last_fixture_year` (0x5aee60, fix_man.cpp, thiscall).
///
/// [pt] { 0x5aee60 } (thiscall, ret 4, fix_man.cpp; o this não é lido): `FIXTURE_MANAGER::get_club_last_fixture_year(&id)`:
/// o último ano com jogos do clube: a palavra dele em [0xae2a5c] sem o bit 0 (metade); sem ano, o ano corrente
/// [0xae2c92]. Devolve `ax` (o chamador da tela guarda 16 bits).
pub(crate) unsafe extern "thiscall" fn get_club_last_fixture_year(_this: usize, id: usize) -> u16 {
    let w = core::ptr::read_unaligned((read_usize(G_CLUB_FIXTURE_YEAR_TABLE) as isize).wrapping_add((*(id as *const i32) as isize).wrapping_mul(2)) as *const u16);
    if w & 0xfffe != 0 { w >> 1 } else { *(G_CURRENT_DATE_YEAR as *const u16) }
}

comparar_valor!(comparar_get_club_first_fixture_year, get_club_first_fixture_year, IDX_GET_CLUB_FIRST_FIXTURE_YEAR, (id: usize) -> u16);
comparar_valor!(comparar_get_club_last_fixture_year, get_club_last_fixture_year, IDX_GET_CLUB_LAST_FIXTURE_YEAR, (id: usize) -> u16);






/// LLIST constructor (0x68d020).
///
/// [pt] LLIST (0xe bytes: +0 conta i16, +2 cabeça, +6 cauda, +0xa cursor; nó: +0 dados, +6 próximo): o construtor
/// (0x68d020), o destrutor (0x68d040), o esvaziar (0x68d080), o cursor na cabeça e o próximo (0x68d1b0).
pub(crate) unsafe fn list_init(l: usize) {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    f(l);
}

/// LLIST destructor (0x68d040).
pub(crate) unsafe fn list_destroy(l: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    f(l);
}

/// Empties a LLIST (0x68d080).
pub(crate) unsafe fn list_clear(l: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_CLEAR);
    f(l);
}

/// Sets the LLIST cursor (+0xa) to the head (+2).
pub(crate) unsafe fn list_rewind(l: usize) {
    core::ptr::write_unaligned((l + 0xa) as *mut u32, read_u32(l + 2));
}


/// True if the competition is a friendly (0x5c6710 on G_FRIENDLY).
///
/// [pt] A competição do jogo é amistosa? (0x5c6710: thiscall em [0xae2c74], 1 argumento.)
pub(crate) unsafe fn fix_man_is_friendly_comp(comp: u32) -> bool {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_FRIENDLY_IS_FRIENDLY_COMPETITION);
    f(read_usize(G_FRIENDLY), comp) != 0
}

/// Builds a date from (day, year) at d (0x549ef0) and returns it.
///
/// [pt] As datas (Date, 8 bytes): construir por (dia, ano) (0x549ef0), menos e mais n dias (0x54c8e0, 0x54c5b0: thiscall
/// (data, saída, n), devolvem a saída) e a atribuição (0x417c10: thiscall(destino, temporária, origem)).
pub(crate) unsafe fn date_from_day_year_ptr(d: *mut u8, day: u32, year: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE);
    f(d as usize, day, year)
}

/// Date minus n days into `saida` (0x54c8e0).
pub(crate) unsafe fn date_sub_days(d: usize, out: *mut u8, n: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_SUB_DAYS);
    f(d, out as usize, n)
}

/// Date plus n days into `saida` (0x54c5b0).
pub(crate) unsafe fn fix_man_date_add_days(d: usize, out: *mut u8, n: u32) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    f(d, out as usize, n)
}

/// Assigns date `origem` to `destino` (0x417c10).
pub(crate) unsafe fn date_assign_ptr(target: *mut u8, source: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut temporaria = [0u8; 8];
    f(target as usize, temporaria.as_mut_ptr() as usize, source);
}

/// Finds the club's fixtures between two dates into a list (FIXTURE_MANAGER find_fixtures 0x5acbe0, any comp/type, side
/// 1).
///
/// [pt] Os jogos do clube entre duas datas (0x5acbe0, thiscall, 9 argumentos): a original empurra os dwords em +0 e +2 de
/// cada data (só a palavra baixa conta), o clube, a lista, comp −1, tipo −2 e lado 1.
pub(crate) unsafe fn find_club_fixtures_between(this: usize, a: *const u8, b: *const u8, clube: u32, list: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURES);
    let (a, b) = (a as usize, b as usize);
    f(this, read_u32(a), read_u32(a + 2), read_u32(b), read_u32(b + 2), clube, list, u32::MAX, 0xffff_fffe, 1)
}

/// Removes a fixture from the calendar (0x5abc30).
///
/// [pt] Tirar o jogo do calendário (0x5abc30), gravar a data (0x5ab510), juntar (0x5aa680) e a notícia (0x5aac70).
pub(crate) unsafe fn fix_man_remove_fixture(this: usize, jogo: usize) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_REMOVE_FIXTURE);
    f(this, jogo)
}

/// Adds a fixture to the calendar (0x5aa680).
pub(crate) unsafe fn fix_man_add_fixture(this: usize, jogo: usize, zerar: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_ADD_FIXTURE);
    f(this, jogo, zerar);
}

/// Queues the "fixture moved" news item (0x5aac70) for the fixture, cause and old date.
pub(crate) unsafe fn fix_man_queue_fixture_moved_news(this: usize, jogo: usize, causa: usize, velha: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_QUEUE_FIXTURE_MOVED_NEWS);
    f(this, jogo, causa, velha);
}

/// True if the fixture's new date clashes with a fixture other than the original f1 (via 0x5ad780); frees the clash
/// list.
///
/// [pt] A data nova do jogo `f2` tem outro jogo? (0x5ad780: thiscall(jogo, &gravidade, 0) devolve uma LLIST alocada;
/// percorre e acha um item com bytes diferentes do jogo original `f1`; depois destrói e libera, 0x944c94.)
pub(crate) unsafe fn new_date_has_clash(this: usize, f2: *mut u8, f1: *const u8) -> bool {
    let find: unsafe extern "thiscall" fn(usize, usize, *mut i32, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_FIXTURE_CLASHES);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let mut severity = 0i32;
    let l = find(this, f2 as usize, &mut severity, 0);
    if l == 0 {
        return false;
    }
    list_rewind(l);
    let mut other = false;
    loop {
        let q = contract_manager_list_next(l);
        if q == 0 {
            break;
        }
        if core::slice::from_raw_parts(f1, 0x4f) != core::slice::from_raw_parts(q as *const u8, 0x4f) {
            other = true;
            break;
        }
    }
    list_destroy(l);
    free(l);
    other
}

/// Copies the first fixture of the list into f1 and f2 (0x4f bytes).
///
/// [pt] O primeiro jogo da lista copiado para `f1` e `f2` (0x4f bytes).
pub(crate) unsafe fn copy_first_fixture(l: usize, f1: &mut [u8; 0x4f], f2: &mut [u8; 0x4f]) {
    list_rewind(l);
    let p = contract_manager_list_next(l);
    core::ptr::copy_nonoverlapping(p as *const u8, f1.as_mut_ptr(), 0x4f);
    *f2 = *f1;
}

/// non-friendly fixture: for each club of a flagged nation (+0x11c), its fixture two days before (with +0x43 = 0xff)
/// moves one day earlier, and the one two days after moves one day later, if the new date is free (0x5ad780); removes,
/// adds and posts the news
/// Original: `FIXTURE_MANAGER::move_nearby_club_fixtures` (0x5ae9d0, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5ae9d0 } (thiscall, ret 4, fix_man.cpp, SEH da lista local e das datas): `FIXTURE_MANAGER::
/// move_nearby_club_fixtures(jogo)`, pela junção do jogo (0x5aa680). Jogo de competição amistosa (+0x14, 0x5c6710):
/// nada. Para os dois clubes (+0x1c, +0x20) de nação (+0x53) com a marca +0x11c: o primeiro jogo do clube dois dias
/// antes (0x5acbe0, lado 1), não amistoso e com +0x43 = 0xff, vai para um dia antes dele; o primeiro jogo dois dias
/// depois, não amistoso, vai para um dia depois. Só se a data nova não tiver outro jogo (0x5ad780): tira (0x5abc30),
/// junta (0x5aa680, 0) e dá a notícia do jogo remarcado (0x5aac70, com o jogo como causa).
pub(crate) unsafe extern "thiscall" fn move_nearby_club_fixtures(this: usize, jogo: usize) {
    let init_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mut d = [0u8; 8];
    init_data(d.as_mut_ptr() as usize);
    let mut l = [0u8; 0xe];
    let lp = l.as_mut_ptr() as usize;
    list_init(lp);
    if fix_man_is_friendly_comp(read_u32(jogo + 0x14)) {
        list_destroy(lp);
        return;
    }
    let mut f1 = [0u8; 0x4f];
    let mut f2 = [0u8; 0x4f];
    for k in 0..2 {
        let clube = read_usize(jogo + if k == 0 { 0x1c } else { 0x20 });
        let nacao = read_usize(clube + club::NATION);
        if nacao == 0 || *((nacao + nation::NATION_LEAGUE_SELECTED) as *const u8) == 0 {
            continue;
        }
        // dois dias antes
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = date_from_day_year_ptr(t.as_mut_ptr(), read_u16(jogo + 0x2a) as u32, read_u16(jogo + 0x28) as u32);
        let r = date_sub_days(r, u.as_mut_ptr(), 2);
        date_assign_ptr(d.as_mut_ptr(), r);
        list_clear(lp);
        if find_club_fixtures_between(this, d.as_ptr(), d.as_ptr(), read_u32(clube), lp) as u16 != 0 {
            copy_first_fixture(lp, &mut f1, &mut f2);
            if !fix_man_is_friendly_comp(read_u32(f1.as_ptr() as usize + 0x14)) {
                crate::comparacao::ramo("cpz_8 jogo dois dias antes");
                if f2[0x43] == 0xff { // cpz8: o jogo de dois dias antes sem a marca
                    let mut v = [0u8; 8];
                    let r = date_sub_days(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia antes
                    date_assign_ptr(d.as_mut_ptr(), r);
                    f2[0x2a..0x2c].copy_from_slice(&d[0..2]);
                    f2[0x28..0x2a].copy_from_slice(&d[2..4]);
                    if !new_date_has_clash(this, f2.as_mut_ptr(), f1.as_ptr()) {
                        crate::comparacao::ramo("cpz_8 jogo antecipado");
                        fix_man_remove_fixture(this, f1.as_ptr() as usize);
                        fix_man_add_fixture(this, f2.as_mut_ptr() as usize, 0);
                        fix_man_queue_fixture_moved_news(this, f2.as_mut_ptr() as usize, jogo, 0);
                    }
                }
            }
        }
        // dois dias depois
        let mut t = [0u8; 8];
        let mut u = [0u8; 8];
        let r = date_from_day_year_ptr(t.as_mut_ptr(), read_u16(jogo + 0x2a) as u32, read_u16(jogo + 0x28) as u32);
        let r = fix_man_date_add_days(r, u.as_mut_ptr(), 2);
        date_assign_ptr(d.as_mut_ptr(), r);
        list_clear(lp);
        if find_club_fixtures_between(this, d.as_ptr(), d.as_ptr(), read_u32(clube), lp) as u16 == 0 {
            continue;
        }
        copy_first_fixture(lp, &mut f1, &mut f2);
        if fix_man_is_friendly_comp(read_u32(f1.as_ptr() as usize + 0x14)) {
            continue;
        }
        let mut v = [0u8; 8];
        let r = fix_man_date_add_days(d.as_ptr() as usize, v.as_mut_ptr(), 1); // cpz8: um dia depois
        date_assign_ptr(d.as_mut_ptr(), r);
        f2[0x2a..0x2c].copy_from_slice(&d[0..2]);
        f2[0x28..0x2a].copy_from_slice(&d[2..4]);
        if !new_date_has_clash(this, f2.as_mut_ptr(), f1.as_ptr()) {
            crate::comparacao::ramo("cpz_8 jogo adiado");
            fix_man_remove_fixture(this, f1.as_ptr() as usize);
            fix_man_add_fixture(this, f2.as_mut_ptr() as usize, 0);
            fix_man_queue_fixture_moved_news(this, f2.as_mut_ptr() as usize, jogo, 0);
        }
    }
    list_destroy(lp);
}

// simplification: move jogos no calendário (tira, junta, aloca e libera listas, notícia): só a original; a cópia leva os ramos.

/// the club's fixtures from one day before to one day after the national team date: those removed from the calendar get
/// a new date, are added back and post the news with the old date
/// Original: `FIXTURE_MANAGER::reschedule_club_fixtures_near_date` (0x5abb10, fix_man.cpp, thiscall, ret 8).
///
/// [pt] { 0x5abb10 } (thiscall, ret 8, fix_man.cpp, SEH da lista local): `FIXTURE_MANAGER::
/// reschedule_club_fixtures_near_date(clube, data)`, pelas convocações das seleções (0x777940: clube com mais de dois
/// convocados). Os jogos do clube (o id em `*clube`) de um dia antes a um dia depois da data (0x5acbe0, lado 1); cada
/// um que sai do calendário (0x5abc30) ganha data nova (0x5ab510), volta (0x5aa680, 1) e dá a notícia com a data velha
/// (0x5aac70, sem causa).
pub(crate) unsafe extern "thiscall" fn reschedule_club_fixtures_near_date(this: usize, clube: usize, data: usize) {
    let write: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_FIXTURE_MANAGER_FIND_NEW_FIXTURE_DATE);
    let mut l = [0u8; 0xe];
    let lp = l.as_mut_ptr() as usize;
    list_init(lp);
    let mut a = [0u8; 8];
    let mut b = [0u8; 8];
    date_sub_days(data, a.as_mut_ptr(), 1);
    fix_man_date_add_days(data, b.as_mut_ptr(), 1);
    find_club_fixtures_between(this, a.as_ptr(), b.as_ptr(), read_u32(clube), lp);
    list_rewind(lp);
    loop {
        let p = contract_manager_list_next(lp);
        if p == 0 {
            break;
        }
        let mut velha = [0u8; 8];
        date_from_day_year_ptr(velha.as_mut_ptr(), read_u16(p + 0x2a) as u32, read_u16(p + 0x28) as u32);
        if fix_man_remove_fixture(this, p) != 0 {
            crate::comparacao::ramo("cpz_8 jogo remarcado pela selecao");
            write(this, p);
            fix_man_add_fixture(this, p, 1);
            fix_man_queue_fixture_moved_news(this, p, 0, velha.as_ptr() as usize);
        }
    }
    list_destroy(lp);
}

// simplification: move jogos no calendário e dá a notícia (aloca): só a original; a cópia leva o ramo.

/// O ensaio da notícia: a cópia monta a notícia nova aqui em vez de pô-la na fila (e não cria a fila).
pub(crate) static mut FIXTURE_MOVED_NEWS_DRY_RUN: bool = false;
pub(crate) static mut FIXTURE_MOVED_NEWS_BUILT: [u8; 0xe0] = [0; 0xe0];
pub(crate) static mut FIXTURE_MOVED_NEWS_WAS_BUILT: bool = false;

/// Data slot k of a news item (NEWSITEM get_slot).
pub(crate) unsafe fn news_slot(n: usize, k: u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_GET_SLOT);
    f(n, k)
}

/// News item already in the list for the same fixture (slots 0,1,2,0xe,0xf vs fixture +0xc,+0x10,+0,+0x40,+0x41), or 0.
///
/// [pt] A notícia já na fila para o mesmo jogo (dados 0, 1, 2, 0xe e 0xf: +0xc, +0x10, +0, +0x40 e +0x41 do jogo).
pub(crate) unsafe fn find_queued_fixture_news(l: usize, f: usize) -> usize {
    list_rewind(l);
    loop {
        let it = contract_manager_list_next(l);
        if it == 0 {
            return 0;
        }
        if news_slot(it, 0) == read_u32(f + 0xc)
            && news_slot(it, 1) == read_u32(f + 0x10)
            && news_slot(it, 2) == read_u32(f)
            && news_slot(it, 0xe) as i32 == read_i8_as_i32(f + 0x40)
            && news_slot(it, 0xf) as i32 == read_i8_as_i32(f + 0x41)
        {
            return it;
        }
    }
}

/// the news queue at +0x1128 (created the first time): updates the new date of the same fixture's news, or queues news
/// 1 with the fixture, the cause or the old date
/// Original: `FIXTURE_MANAGER::queue_fixture_moved_news` (0x5aac70, fix_man.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5aac70 } (thiscall, ret 0xc, fix_man.cpp, SEH da notícia local): `FIXTURE_MANAGER::queue_fixture_moved_news(jogo,
/// causa, data velha)`, pelas remarcações (0x5aa680, 0x5abb10, 0x5ae380, 0x5ae9d0). A fila em +0x1128 (criada na
/// primeira vez: 0x944cf1 e 0x68d020): se já há a notícia do mesmo jogo, só troca a data nova (dados 5 e 6); senão a
/// notícia 1: o jogo (+0xc, +0x10, +0, +0x32, +0x30, a data nova +0x2a/+0x28, +0x40, +0x41) e a causa (os mesmos
/// campos dela nos dados 7 a 0xd) ou, sem causa, −1 e a data velha (dados 0xc e 0xd); entra na fila (0x68d0d0, 0xdf).
pub(crate) unsafe extern "thiscall" fn queue_fixture_moved_news(this: usize, f: usize, causa: usize, velha: usize) {
    let construct: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let write: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    if read_usize(this + 0x1128) == 0 && !FIXTURE_MOVED_NEWS_DRY_RUN {
        let new: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
        let m = new(0xe);
        if m != 0 {
            list_init(m);
        }
        *((this + 0x1128) as *mut usize) = m;
    }
    let l = read_usize(this + 0x1128);
    if l != 0 {
        let it = find_queued_fixture_news(l, f);
        if it != 0 {
            write(it, 5, read_u16(f + 0x2a) as u32);
            write(it, 6, read_u16(f + 0x28) as u32);
            return;
        }
    }
    crate::comparacao::ramo("cpz_8 noticia do jogo remarcado");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construct(nt, 1, 0);
    let campos = |nt: usize, k: i8, j: usize| {
        write(nt, k, read_u32(j + 0xc));
        write(nt, k + 1, read_u32(j + 0x10));
        write(nt, k + 2, read_u32(j));
        write(nt, k + 3, read_i16_as_i32(j + 0x32) as u32);
        write(nt, k + 4, read_i16_as_i32(j + 0x30) as u32);
        write(nt, k + 5, read_u16(j + 0x2a) as u32);
    };
    campos(nt, 0, f);
    write(nt, 6, read_u16(f + 0x28) as u32);
    write(nt, 0xe, read_i8_as_i32(f + 0x40) as u32);
    write(nt, 0xf, read_i8_as_i32(f + 0x41) as u32);
    if causa != 0 {
        campos(nt, 7, causa);
        write(nt, 0xd, read_u16(causa + 0x28) as u32);
    } else {
        write(nt, 7, u32::MAX);
        write(nt, 0xc, read_i16_as_i32(velha) as u32);
        write(nt, 0xd, read_u16(velha + 2) as u32);
    }
    if FIXTURE_MOVED_NEWS_DRY_RUN {
        core::ptr::copy_nonoverlapping(nt as *const u8, core::ptr::addr_of_mut!(FIXTURE_MOVED_NEWS_BUILT) as *mut u8, 0xe0);
        FIXTURE_MOVED_NEWS_WAS_BUILT = true;
        return;
    }
    let copy: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    copy(read_usize(this + 0x1128), nt, 0xdf);
}




/// Calendar block for the fixture's year (+0x28), walking the chain from `fm`, retargeting the next block's year or
/// allocating a new block; 0 on out of memory.
///
/// [pt] O bloco do ano do jogo `f` (+0x28, palavra) na corrente do calendário a partir de `fm` (o laço comum das 0x5ac7a0,
/// 0x5ab230 e 0x5abc30): o bloco seguinte (+0x112c) de outro ano é levado ao ano pedido (0x5aa2d0 set_year); sem
/// seguinte, um bloco novo (0x944cf1, 0x1184 bytes; 0x5a9cf0 com o ano, o bloco de antes e 0) entra na corrente. 0 se
/// faltou memória (a corrente fica com 0 em +0x112c, como na original).
pub(crate) unsafe fn calendar_year_block(fm: usize, f: usize) -> usize {
    let mudar_ano: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_FIXTURE_MANAGER_SET_YEAR);
    let new: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construct: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_FIXTURE_MANAGER);
    let mut b = fm;
    loop {
        let year = read_u16(f + 0x28);
        if year == read_u16(b + 0x1130) {
            return b;
        }
        let next = read_u32_as_usize(b + 0x112c);
        if next != 0 {
            if read_u16(next + 0x1130) != year {
                // simplification: a original empurra o eax com o ano na palavra baixa e sobra de registrador em cima; a
                // 0x5aa2d0 compara a palavra (o caminho só roda na virada do ano do calendário)
                mudar_ano(next, year as u32);
            }
        } else {
            let p = new(0x1184);
            // o ecx da original era o +0x112c nulo: o ano vai limpo
            let r = if p != 0 { construct(p, year as u32, b, 0) } else { 0 };
            core::ptr::write_unaligned((b + 0x112c) as *mut u32, r as u32);
            if r == 0 {
                return 0;
            }
        }
        b = read_u32_as_usize(b + 0x112c);
    }
}

/// Fixture list in block b for the fixture's day (+0x2a), time slice (+0x3f) and key nation.
///
/// [pt] A lista do dia, fatia (+0x3f) e nação-chave do jogo `f` no bloco `b`: [[b + (dia +0x2a × 3 + fatia) × 4] + nação × 4],
/// a nação pela 0x48dd70 (competição +0x14, o jogo), com sinal.
pub(crate) unsafe fn fixture_list_in_block(b: usize, f: usize) -> usize {
    let nacao: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_ACTIVE_KEY_NATION_OF_FIXTURE);
    let n = nacao(read_u32_as_usize(f + 0x14), f) as u8 as i8 as isize;
    let k = read_u16(f + 0x2a) as usize * 3 + *((f + 0x3f) as *const u8) as usize;
    read_u32_as_usize((read_u32_as_usize(b + k * 4) as isize + n * 4) as usize)
}


/// returns the stored copy whose 0x4f bytes match the fixture's, in its day, slice and key-nation list of the year
/// block; 0 if none
/// Original: `FIXTURE_MANAGER::find_stored_fixture` (0x5ac7a0, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5ac7a0 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::find_stored_fixture(jogo)`: a cópia guardada no
/// calendário cujos 0x4f bytes são iguais aos do jogo, na lista do dia, fatia e nação-chave dele (o bloco do ano pela
/// corrente; o cursor da lista volta ao começo e anda). Jogo nulo ou nenhum igual: 0; sem memória para o bloco do ano:
/// a caixa (linha 0x7cf) e 0.
pub(crate) unsafe extern "thiscall" fn find_stored_fixture(this: usize, f: usize) -> usize {
    if f == 0 {
        return 0;
    }
    let b = calendar_year_block(this, f);
    if b == 0 {
        show_error_at(0x9cc208, 0x7cf);
        return 0;
    }
    let l = fixture_list_in_block(b, f);
    core::ptr::write_unaligned((l + 0xa) as *mut u32, read_u32_as_usize(l + 2) as u32);
    let target = core::slice::from_raw_parts(f as *const u8, 0x4f);
    let mut item = fix_man_list_next(l);
    while item != 0 {
        if core::slice::from_raw_parts(item as *const u8, 0x4f) == target {
            crate::comparacao::ramo("cps_11 achou o jogo guardado");
            return item; // cps11: a cópia guardada
        }
        item = fix_man_list_next(l);
    }
    0
}

/// finds (or creates) the fixture's year block and appends a copy of it to the day, slice and key-nation list
/// Original: `FIXTURE_MANAGER::insert_fixture` (0x5ab230, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5ab230 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::insert_fixture(jogo)`: acrescenta uma cópia dos 0x4f
/// bytes do jogo (0x68d0d0) à lista do dia, fatia e nação-chave dele no bloco do ano (pela corrente). Sem memória para o
/// bloco: a caixa (linha 0x3ff); a cópia que falha: a caixa (linha 0x3eb).
pub(crate) unsafe extern "thiscall" fn insert_fixture(this: usize, f: usize) {
    let b = calendar_year_block(this, f);
    if b == 0 {
        return show_error_at(0x9cc208, 0x3ff);
    }
    let copy: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let l = fixture_list_in_block(b, f);
    if copy(l, f, 0x4f) == 0 { // cps11: guardar
        return show_error_at(0x9cc208, 0x3eb);
    }
    crate::comparacao::ramo("cps_11 jogo guardado");
}

/// removes the first item with the same bytes from the fixture's day, slice and key-nation list; returns 1 if removed
/// Original: `FIXTURE_MANAGER::remove_fixture` (0x5abc30, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5abc30 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::remove_fixture(jogo)`: tira da lista do dia, fatia e
/// nação-chave do jogo (no bloco do ano, pela corrente) o primeiro item com os mesmos bytes (0x68d1f0). Devolve 1 se
/// tirou, 0 se não; sem memória para o bloco: a caixa (linha 0x5ef) e 0.
pub(crate) unsafe extern "thiscall" fn remove_fixture(this: usize, f: usize) -> u32 {
    let b = calendar_year_block(this, f);
    if b == 0 {
        show_error_at(0x9cc208, 0x5ef);
        return 0;
    }
    let remove: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LLIST_REMOVE);
    let r = remove(fixture_list_in_block(b, f), f); // cps11: tirar
    if r != 0 {
        crate::comparacao::ramo("cps_11 jogo tirado");
    }
    (r != 0) as u32
}

/// Fixture list of the fixture in the calendar (year block, then list); 0 on out of memory.
///
/// [pt] A lista do jogo no calendário, achada antes da original (o laço da corrente faz o mesmo que ela faria: depois dele
/// a original acha o bloco do ano de primeira); 0 sem memória.
pub(crate) unsafe fn find_fixture_list(fm: usize, f: usize) -> usize {
    let b = calendar_year_block(fm, f);
    if b == 0 { 0 } else { fixture_list_in_block(b, f) }
}





/// Fixture list for (year, day, slice, key nation) in calendar fm (0x5abf10, creates the year block if missing).
///
/// [pt] A lista dos jogos de um ano, dia, fatia e nação-chave no calendário `fm` (0x5abf10; cria o bloco do ano se a
/// corrente não o tem). A original compara o ano como palavra e mascara o dia, a fatia e a nação.
pub(crate) unsafe fn fix_man_get_fixture_list(fm: usize, year: u32, day: u32, slice: u32, nacao: u32) -> usize {
    let list: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_GET_FIXTURE_LIST);
    list(fm, year, day, slice, nacao)
}

/// Next item of a LLIST (0x68d1b0, advances the cursor).
///
/// [pt] O próximo item da lista (0x68d1b0: o cursor +0xa anda).
pub(crate) unsafe fn fix_man_list_next(l: usize) -> usize {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    next(l)
}

/// Rewinds the LLIST cursor and returns the first item.
///
/// [pt] Põe o cursor da lista no começo (+0xa = +2) e devolve o primeiro item.
pub(crate) unsafe fn list_first(l: usize) -> usize {
    core::ptr::write_unaligned((l + 0xa) as *mut u32, read_u32(l + 2));
    fix_man_list_next(l)
}

/// Appends a copy of the 0x4f-byte fixture to the list (0x68d0d0); 0 on failure.
///
/// [pt] Acrescenta à lista `l` uma cópia dos 0x4f bytes do jogo `f` (0x68d0d0); 0 se falhou.
pub(crate) unsafe fn list_append_fixture(l: usize, f: usize) -> usize {
    let copy: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    copy(l, f, 0x4f)
}

/// Active key nation of competition `comp` (0x48dd70 on its club-comp record, no fixture).
///
/// [pt] A nação-chave ativa da competição `comp` (0x48dd70 sobre o registro [0xae23d0] + 0x6b × comp, sem jogo).
pub(crate) unsafe fn comp_key_nation(comp: u32) -> u8 {
    let nacao: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_ACTIVE_KEY_NATION_OF_FIXTURE);
    nacao((read_u32(G_CLUB_COMP_TABLE).wrapping_add(comp.wrapping_mul(0x6b))) as usize, 0) as u8
}

/// Fixture passes the 0x5acbe0 filters: type (+0x42), club home/away with its side (+0x40/+0x41) and competition (+0);
/// wildcards allowed.
///
/// [pt] O jogo `f` passa nos filtros da 0x5acbe0: tipo (+0x42; 0xfe qualquer), clube em casa (+0xc) ou fora (+0x10; −1
/// qualquer) com o lado dele (+0x40 em casa, +0x41 fora; 0xff qualquer) e a competição (+0; −1 qualquer).
pub(crate) unsafe fn fixture_matches_filter(f: usize, clube: u32, comp: u32, kind: u8, side: u8) -> bool {
    if kind != 0xfe && kind != *((f + 0x42) as *const u8) {
        return false;
    }
    if clube != u32::MAX {
        let home = read_u32(f + 0xc);
        if clube != home && clube != read_u32(f + 0x10) {
            return false;
        }
        if side != 0xff {
            let o_dele = if clube == home { *((f + 0x40) as *const u8) } else { *((f + 0x41) as *const u8) };
            if o_dele != side {
                return false;
            }
        }
    }
    comp == u32::MAX || comp == read_u32(f)
}

/// counts (and copies into the list, if given) the fixtures between two dates in the key-nation lists of the club or
/// competition, with type, club side and competition filters
/// Original: `FIXTURE_MANAGER::find_fixtures` (0x5acbe0, fix_man.cpp, thiscall, ret 0x24).
///
/// [pt] { 0x5acbe0 } (thiscall, ret 0x24, fix_man.cpp): `FIXTURE_MANAGER::find_fixtures(dia0, ano0, dia1, ano1, clube, lista,
/// comp, tipo, lado)`: conta (e, com `lista`, copia para ela) os jogos do calendário entre (dia0, ano0) e (dia1, ano1)
/// inclusive, dia a dia (366 por ano), nas três fatias de cada dia e nas listas das nações-chave: do clube (índice em
/// [0xae23b4], 0x245 bytes; seleção 0x53b900: as nações 0 e 1; senão 2..7, a da 0x48dee0 e a parceira 0x5af5b0, sem
/// repetir) ou só a da competição (0x48dd70). Filtros: tipo (0xfe qualquer), lado do clube (0xff qualquer) e
/// competição (−1 qualquer). Devolve a conta (palavra); a cópia que falha mostra a caixa (linha 0x93b) e para ali.
/// Quente (288 mil chamadas até 2002-06-01): sem contador.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn find_fixtures(this: usize, dia0: u32, ano0: u32, dia1: u32, ano1: u32, clube: u32,
                                                      list: usize, comp: u32, kind: u32, side: u32) -> u32 {
    let (kind, side) = (kind as u8, side as u8);
    let mut nations = [0xffu8; 8];
    let mut n = 0usize;
    if clube != u32::MAX {
        let reg = read_u32(G_CLUB_TABLE).wrapping_add(clube.wrapping_mul(0x245)) as usize;
        let national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
        if national_team(reg) != 0 {
            nations[0] = 0;
            nations[1] = 1;
            n = 2;
        } else {
            let of_club: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_COMP_ACTIVE_KEY_NATION_OF_CLUB);
            let parceira: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_FIX_MAN_GET_PARTNER_KEY_NATION_INDEX);
            nations[..6].copy_from_slice(&[2, 3, 4, 5, 6, 7]);
            nations[6] = of_club(reg) as u8;
            nations[7] = parceira(reg) as u8;
            n = 8;
            for i in 0..7 {
                if (i + 1..8).any(|j| nations[j] == nations[i]) {
                    nations[i] = 0xff; // repetida adiante
                }
            }
        }
    } else if (comp as i32) > -1 {
        nations[0] = comp_key_nation(comp);
        n = 1;
    }
    let (mut dia, mut ano, mut total) = (dia0, ano0, 0u32);
    loop {
        if dia as u16 >= 0x16e {
            ano = ano.wrapping_add(1); // o ano seguinte, do dia 0
            dia = 0;
            continue;
        }
        if ano as u16 > ano1 as u16 || (ano as u16 == ano1 as u16 && dia as u16 > dia1 as u16) {
            return total & 0xffff; // passou do último dia
        }
        for fatia in 0..3u32 {
            for &nacao in &nations[..n] {
                if nacao == 0xff {
                    continue;
                }
                let l = fix_man_get_fixture_list(this, ano, dia, fatia, nacao as u32);
                if l == 0 {
                    continue;
                }
                let mut f = list_first(l);
                while f != 0 {
                    if fixture_matches_filter(f, clube, comp, kind, side) {
                        if list != 0 && list_append_fixture(list, f) == 0 {
                            show_error_at(0x9cc208, 0x93b);
                            return total & 0xffff;
                        }
                        total = total.wrapping_add(1);
                    }
                    f = fix_man_list_next(l);
                }
            }
        }
        dia = dia.wrapping_add(1);
    }
}

/// copies into the list the fixtures of one day and slice for the requested competition, round and type and returns how
/// many
/// Original: `FIXTURE_MANAGER::collect_comp_fixtures_on_day` (0x5abd90, fix_man.cpp, thiscall, ret 0x1c).
///
/// [pt] { 0x5abd90 } (thiscall, ret 0x1c, fix_man.cpp): `FIXTURE_MANAGER::collect_comp_fixtures_on_day(ano, dia, fatia,
/// comp, rodada, tipo, lista)` (do adiamento pela TV das ligas e da mudança das rodadas das copas para as datas fixas):
/// copia para `lista` os jogos do dia e fatia, na lista da nação-chave da competição (0x48dd70; −1: a nação 0), da
/// competição (+0; −1 qualquer), da rodada (+0x34, palavra; −1 qualquer) e do tipo (+0x42; 0xfe qualquer). Devolve
/// quantos (palavra); a cópia que falha mostra a caixa (linha 0x634) e para ali.
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn collect_comp_fixtures_on_day(this: usize, year: u32, day: u32, slice: u32, comp: u32, round: u32,
                                                       kind: u32, list: usize) -> u32 {
    let kind = kind as u8;
    // simplification: a original empurra o local da nação com bytes altos de sobra de pilha; a 0x5abf10 só lê o byte
    let nacao = if comp != u32::MAX { comp_key_nation(comp) } else { 0 };
    let mut total = 0u32;
    let l = fix_man_get_fixture_list(this, year, day, slice, nacao as u32);
    if l == 0 {
        return 0;
    }
    let mut f = list_first(l);
    while f != 0 {
        let mut ok = true;
        if comp != u32::MAX && comp != read_u32(f) {
            ok = false;
        }
        if round != u32::MAX && round as i32 != *((f + 0x34) as *const i16) as i32 {
            ok = false; // outra rodada
        }
        if (kind == 0xfe || *((f + 0x42) as *const u8) == kind) && ok {
            if list_append_fixture(list, f) == 0 {
                show_error_at(0x9cc208, 0x634);
                return total & 0xffff;
            }
            crate::comparacao::ramo("cpr_12 jogo da rodada");
            total = total.wrapping_add(1);
        }
        f = fix_man_list_next(l);
    }
    total & 0xffff
}

/// tells whether the competition has a fixture on the date in any of the three calendar slices
/// Original: `fix_man_comp_has_fixture_on_date` (0x5ae6d0, fix_man.cpp, stdcall, ret 8).
///
/// [pt] { 0x5ae6d0 } (stdcall, ret 8, fix_man.cpp; o `ecx` = [0xae2a58] que o chamador põe não é usado):
/// `fix_man_comp_has_fixture_on_date(data, comp)` (das eliminatórias da Eurocopa 0x57fd60): 1 se a competição tem
/// jogo (+0 = comp) na data em alguma das três fatias do calendário [0xae2a58], na lista da nação-chave dela
/// (0x48dd70). Competição −1 ou abaixo: 0; data nula: a caixa (linha 0xe89) e 0.
pub(crate) unsafe extern "stdcall" fn fix_man_comp_has_fixture_on_date(data: usize, comp: u32) -> u8 {
    if (comp as i32) <= -1 {
        return 0;
    }
    let nacao = comp_key_nation(comp);
    if data == 0 {
        show_error_at(0x9cc208, 0xe89);
        return 0;
    }
    let fm = read_u32(G_FIXTURE_MANAGER) as usize;
    // simplification: a original empurra o ano e o dia (palavras) com bytes altos de sobra nos registradores; a 0x5abf10
    // compara o ano como palavra e mascara o dia (o dword do ano só segue se o bloco do ano faltar, e os dois anos da
    // carreira estão na corrente desde a criação do jogo)
    let (ano, dia) = (*((data + 2) as *const u16) as u32, *(data as *const u16) as u32);
    for fatia in 0..3u32 {
        let l = fix_man_get_fixture_list(fm, ano, dia, fatia, nacao as u32);
        if l == 0 {
            continue;
        }
        let mut f = list_first(l);
        while f != 0 {
            if read_u32(f) == comp {
                return 1;
            }
            f = fix_man_list_next(l);
        }
    }
    0
}

/// True if the items the original appended to `lista` (after the old tail) equal those of the temporary list `nova`, in
/// order, 0x4f bytes each.
///
/// [pt] Os itens que a original acrescentou à `lista` (depois da cauda de antes) são os da lista provisória `nova`, na
/// mesma ordem, 0x4f bytes cada?
pub(crate) unsafe fn appended_items_match(list: usize, count_before: u16, cauda_antes: u32, new: usize) -> bool {
    let acrescentados = (*(list as *const u16)).wrapping_sub(count_before);
    if acrescentados != *(new as *const u16) {
        return false;
    }
    let mut a = if count_before == 0 { read_u32(list + 2) } else { read_u32(cauda_antes as usize + 6) } as usize;
    let mut b = read_u32(new + 2) as usize;
    for _ in 0..acrescentados {
        if a == 0 || b == 0 {
            return false;
        }
        let (da, db) = (read_u32(a) as usize, read_u32(b) as usize);
        if core::slice::from_raw_parts(da as *const u8, 0x4f) != core::slice::from_raw_parts(db as *const u8, 0x4f) {
            return false;
        }
        a = read_u32(a + 6) as usize;
        b = read_u32(b + 6) as usize;
    }
    true
}

/// Runs the port with a temporary LLIST and checks its items against those the original appended to the real list;
/// returns (result, equal).
///
/// [pt] Roda a cópia com uma lista provisória (0x68d020; destruída pela 0x68d040) e confere os itens contra os que a
/// original acrescentou à lista real.
pub(crate) unsafe fn run_with_temp_list(list: usize, count_before: u16, cauda_antes: u32, copy: impl FnOnce(usize) -> u32) -> (u32, bool) {
    let create: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destroy: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let mut new = [0u8; 0x10];
    let p = new.as_mut_ptr() as usize;
    create(p);
    let n = copy(p);
    let iguais = appended_items_match(list, count_before, cauda_antes, p);
    destroy(p);
    (n, iguais)
}




/// Fixture list for (year, day, slice, group) via 0x5abf10 with the cursor rewound; 0 if null.
///
/// [pt] A lista das partidas de (ano, dia, fatia, grupo) no gerenciador `this` (0x5abf10, thiscall ret 0x10, original:
/// carrega o ano se preciso), com o cursor voltado ao começo (+0xa = +2), como as duas originais fazem; nula: 0.
pub(crate) unsafe fn get_fixture_list_rewound(this: usize, year: u32, day: u32, slice: u32, group: u32) -> usize {
    let list: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_GET_FIXTURE_LIST);
    let l = list(this, year, day, slice, group);
    if l != 0 {
        core::ptr::write_unaligned((l + 0xa) as *mut u32, core::ptr::read_unaligned((l + 2) as *const u32));
    }
    l
}

/// Next item of a LLIST (0x68d1b0) or 0.
///
/// [pt] O próximo item da lista `l` (0x68d1b0, next_list_item), ou 0.
pub(crate) unsafe fn list_next_item(l: usize) -> usize {
    let next: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    next(l)
}

/// Fixture passes find_next_fixture's filter: competition (+0), stage (+0x34) and both clubs (home +0xc or away +0x10),
/// each -1 = any.
///
/// [pt] A partida `f` passa pelo filtro da find_next_fixture? Cada filtro em -1 deixa passar: a competição (+0), a fase
/// (+0x34, curto com sinal) e os dois clubes (cada um o mandante +0xc ou o visitante +0x10).
pub(crate) unsafe fn fixture_matches_next_filter(f: usize, comp: i32, stage: i32, clube_a: i32, club_b: i32) -> bool {
    let d = |o: usize| core::ptr::read_unaligned((f + o) as *const i32);
    let clube = |c: i32| c == -1 || c == d(0xc) || c == d(0x10);
    (comp == -1 || comp == d(0)) // a competição
        && (stage == -1 || stage == core::ptr::read_unaligned((f + 0x34) as *const i16) as i32)
        && clube(clube_a)
        && clube(club_b)
}

/// the next (or previous) fixture from the year/day/slice cursor, inclusive, by the competition (+0), stage (+0x34) and
/// two club (+0xc/+0x10) filters, each -1 = any; up to 5 years; writes the cursor back and returns the fixture, or 0
/// Original: `FIXTURE_MANAGER::find_next_fixture` (0x5ac920, fix_man.cpp, thiscall, ret 0x2c).
///
/// [pt] { 0x5ac920 } (thiscall, ret 0x2c, fix_man.cpp): `FIXTURE_MANAGER::find_next_fixture(ano*, dia*, fatia*, adiante,
/// comp, fase, clube_a, clube_b, ...)`: a partida seguinte (ou anterior, `adiante` 0) ao cursor (ano, dia, fatia),
/// o próprio cursor incluído, que passa pelo filtro (fixture_matches_next_filter). O grupo das listas vem da 0x48dd70 (a competição
/// da tabela [0xae23d0] + 107 × comp, 0). Adiante: fatias até 2, dias até 365 (0x16e exclusivo), anos seguintes no
/// dia 0 (a fatia só volta a 0 na virada do dia); atrás: fatias até 0, depois 2, dias até 0, anos anteriores no dia
/// 0x16d. A cinco anos do ano do cursor (adiante: com sinal menos sem sinal; atrás: com sinal), 0. Achou: grava o
/// cursor (dia, ano e a fatia em byte) e devolve a partida. Os 3 últimos argumentos não são lidos. A original passa
/// à 0x5abf10 ano, dia e fatia com a metade alta de locais nunca escritas (lixo), mas ela e o que chama (0x5aa2d0,
/// 0x5a9cf0, 0x54cfe0) só usam a palavra/o byte: a cópia passa zero. Nos sorteios de copa (Cup.cpp 0x519d90,
/// 0x51a150, 0x522c70). Sem SEH, sem caixa (a da 0x5abf10 só sem memória).
pub(crate) unsafe extern "thiscall" fn find_next_fixture(this: usize, pano: *mut u16, pdia: *mut u16, pfatia: *mut u8, adiante: u32,
    comp: i32, stage: i32, clube_a: i32, club_b: i32, _a9: u32, _a10: u32, _a11: u32) -> usize {
    let group_of: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(FN_COMP_ACTIVE_KEY_NATION_OF_FIXTURE);
    let table = *(G_CLUB_COMP_TABLE as *const usize);
    let group = group_of(table.wrapping_add((comp as usize).wrapping_mul(107)), 0) as u8 as u32;
    let (mut year, mut day, mut slice) = (*pano, *pdia, *pfatia as u16);
    let find = |year: u16, day: u16, slice: u16| -> usize {
        let l = get_fixture_list_rewound(this, year as u32, day as u32, slice as u32, group);
        if l == 0 {
            return 0;
        }
        loop {
            let f = list_next_item(l);
            if f == 0 || fixture_matches_next_filter(f, comp, stage, clube_a, club_b) {
                return f;
            }
        }
    };
    let f = 'busca: {
        if adiante != 0 {
            loop {
                while (day as i16) < 0x16e {
                    while (slice as i16) < 3 {
                        let f = find(year, day, slice);
                        if f != 0 {
                            break 'busca f;
                        }
                        slice = slice.wrapping_add(1);
                    }
                    day = day.wrapping_add(1);
                    slice = 0; // o dia seguinte
                }
                year = year.wrapping_add(1);
                day = 0;
                if (year as i16 as i32) - (*pano as i32) >= 5 {
                    return 0;
                }
            }
        } else {
            loop {
                while (day as i16) >= 0 {
                    while (slice as i16) >= 0 {
                        let f = find(year, day, slice);
                        if f != 0 {
                            break 'busca f;
                        }
                        slice = slice.wrapping_sub(1);
                    }
                    slice = 2; // o dia anterior
                    day = day.wrapping_sub(1);
                }
                year = year.wrapping_sub(1);
                day = 0x16d;
                if (*pano as i16 as i32) - (year as i16 as i32) >= 5 {
                    return 0;
                }
            }
        }
    };
    *pdia = day;
    *pano = year;
    *pfatia = slice as u8;
    f
}


/// 1 when a fixture (+0x18) in the day lists (groups 0 to 7 and the stadium one, 0x48df40) uses the stadium, in the
/// given slice or in slices 0 to 2 (slice -1)
/// Original: `FIXTURE_MANAGER::is_stadium_in_use` (0x5ae870, fix_man.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5ae870 } (thiscall, ret 0x10, fix_man.cpp): `FIXTURE_MANAGER::is_stadium_in_use(estadio, dia, ano, fatia)`:
/// 1 se alguma partida (+0x18 = o estádio, não nulo) das listas do dia usa o estádio. Os grupos: 0 a 7 e o do estádio
/// (0x48df40, byte), este fora se repete um dos oito. Fatia -1 (byte): as fatias 0 a 2, senão só ela. A fatia vai à
/// 0x5abf10 no byte baixo do argumento do ano (a original guarda o laço lá: os 3 bytes altos são os do ano). Estádio
/// nulo: 0. Na marcação dos amistosos (friendly.cpp 0x5c6f00, 0x5c75a0) e em 0x7e6380, 0x7e6720. Sem SEH, sem locais
/// sem valor.
pub(crate) unsafe extern "thiscall" fn is_stadium_in_use(this: usize, estadio: usize, day: u32, year: u32, slice: u32) -> u32 {
    if estadio == 0 {
        return 0;
    }
    let group_of_stadium: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_COMP_ACTIVE_KEY_NATION_OF_STADIUM);
    let mut groups = [0u8, 1, 2, 3, 4, 5, 6, 7, group_of_stadium(estadio) as u8];
    for i in 0..8 {
        for j in i + 1..9 {
            if groups[i] as i8 >= 0 && groups[i] == groups[j] {
                groups[j] = 0xff;
            }
        }
    }
    let (ini, fim) = if slice as u8 == 0xff { (0i8, 2i8) } else { (slice as i8, slice as i8) };
    for &g in &groups {
        if g == 0xff {
            continue;
        }
        let mut s = ini;
        while s <= fim {
            let l = get_fixture_list_rewound(this, year, day, (year & 0xffff_ff00) | s as u8 as u32, g as u32);
            if l != 0 {
                loop {
                    let f = list_next_item(l);
                    if f == 0 {
                        break;
                    }
                    let e = core::ptr::read_unaligned((f + 0x18) as *const usize);
                    if e != 0 && e == estadio { // o estádio da partida
                        return 1;
                    }
                }
            }
            s = s.wrapping_add(1);
        }
    }
    0
}

comparar_valor!(comparar_is_stadium_in_use, is_stadium_in_use, IDX_IS_STADIUM_IN_USE, (estadio: usize, dia: u32, ano: u32, fatia: u32) -> u32);


/// Calls the original routine (ret 4) with eax = year and ecx = this, as its only caller does.
///
/// [pt] A original (ret 4) com eax = ano, como no único chamador.
pub(crate) unsafe fn call_original_with_year(o: usize, this: usize, year: u32) -> u32 {
    let r: u32;
    core::arch::asm!(
        "push {a}",
        "call {f}",
        a = in(reg) year,
        f = in(reg) o,
        inout("eax") year => r,
        inout("ecx") this => _,
        clobber_abi("C"),
    );
    r
}

/// the fixture time slice (1 weekend, 2 otherwise) by competition nation and weekday
/// Original: `FIXTURE_MANAGER::get_time_slice` (0x5af2f0, fix_man.cpp, thiscall).
///
/// [pt] { 0x5af2f0 } (thiscall, ret 8, fix_man.cpp; `this` não é lido): a faixa de horário do jogo da
/// competição `comp` no dia `dia` (palavra): Holanda, Irlanda, Bélgica e França (id da nação +0x5d
/// em [0x9cf338], [0x9cf2dc], [0x9cf244], [0x9cf300]) só têm 1 no dia 6; as outras, 1 nos dias 5 e 6;
/// senão 2. Devolve só `al`.
pub(crate) unsafe extern "thiscall" fn get_time_slice(_this: usize, comp: usize, day: u32) -> u8 {
    let day = day as i16;
    if comp != 0 {
        let nacao = core::ptr::read_unaligned((comp + competition::CLUB_COMP_NATION) as *const usize);
        if nacao != 0 {
            let id = *(nacao as *const u32);
            if [KEY_NATION_HOLLAND, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_BELGIUM, KEY_NATION_FRANCE].iter().any(|&g| id == *(g as *const u32)) {
                return if day != 6 { 2 } else { 1 };
            }
        }
    }
    if day == 6 || day == 5 { 1 } else { 2 }
}


/// with the stadium in a third-world country, gives the visiting clubs the illness risk on the day before the fixture
/// Original: `FIXTURE_MANAGER::add_away_trip_illness_risk` (0x5ab3c0, fix_man.cpp, thiscall, ret 4).
///
/// [pt] { 0x5ab3c0 } (thiscall, ret 4, fix_man.cpp; o `this` não é usado): `FIXTURE_MANAGER::add_away_trip_illness_risk(jogo)`:
/// com o estádio do jogo (+0x18), a cidade (+0x38) e a nação dela (+0x1f) num país do terceiro mundo (+0x7e = 3), cada
/// clube visitante de outra nação (+0x53) — o mandante (+0x1c, id +0xc) e o visitante (+0x20, id +0x10) — ganha o risco
/// de doença na véspera do jogo.
pub(crate) unsafe extern "thiscall" fn add_away_trip_illness_risk(_this: usize, f: usize) {
    let estadio = read_u32_as_usize(f + 0x18);
    if estadio == 0 {
        return;
    }
    let cidade = read_u32_as_usize(estadio + stadium::STADIUM_CITY);
    if cidade == 0 {
        return;
    }
    let nacao = read_u32_as_usize(cidade + city::CITY_NATION);
    if nacao == 0 {
        return;
    }
    if read_u8(nacao + nation::STATE_OF_DEVELOPMENT) != 3 { // cps13: só o terceiro mundo
        return;
    }
    let home = read_u32_as_usize(f + 0x1c);
    if home != 0 && read_u32_as_usize(home + 0x53) != nacao {
        crate::comparacao::ramo("cps_13 doenca do mandante");
        // a original empurra o ano no eax do clube e o dia no ecx da nação (metades altas dos ponteiros)
        add_illness_risk_before_match(f, read_u32_as_usize(f + 0xc), read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(f + 0x18) + 0x38) + 0x1f), nacao as u32, home as u32);
    }
    let away = read_u32_as_usize(f + 0x20);
    if away != 0 && read_u32_as_usize(away + 0x53) != read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(f + 0x18) + 0x38) + 0x1f) {
        crate::comparacao::ramo("cps_13 doenca do visitante");
        // o ano no ecx do estádio e o dia no edx da cidade
        let (e, c) = (read_u32_as_usize(f + 0x18), read_u32_as_usize(read_u32_as_usize(f + 0x18) + 0x38));
        add_illness_risk_before_match(f, read_u32_as_usize(f + 0x10), read_u32_as_usize(c + 0x1f), c as u32, e as u32);
    }
}


/// for an English or Welsh club, the active key-nation index of the other nation; otherwise 0xff
/// Original: `fix_man_get_partner_key_nation_index` (0x5af5b0, fix_man.cpp, cdecl).
///
/// [pt] { 0x5af5b0 } (cdecl, fix_man.cpp; das buscas de jogos 0x5acbe0, 0x5acee0 e 0x5ad780): `fix_man_get_partner_key_nation_index(clube)`:
/// para clube da Inglaterra ([0x9cf2e4]) ou do País de Gales ([0x9cf510]) (nação +0x53, id +0), o índice da nação-chave
/// ativa da outra (os 35 registros de 72 bytes em 0xb63d60: +0 a nação, ativa em +0x11c, +0x14 o índice); senão 0xff.
pub(crate) unsafe extern "C" fn fix_man_get_partner_key_nation_index(clube: usize) -> u8 {
    let nacao = read_u32(clube + club::NATION) as usize;
    if nacao == 0 {
        return 0xff;
    }
    let (id, inglaterra, gales) = (read_u32(nacao), read_u32(KEY_NATION_ENGLAND), read_u32(KEY_NATION_WALES));
    let other = if id == inglaterra {
        gales
    } else if id == gales {
        inglaterra
    } else {
        return 0xff;
    };
    let target = read_u32(G_NATION_TABLE).wrapping_add(other.wrapping_mul(0x122)) as usize;
    for i in 0..0x23usize {
        let p = read_u32(G_LEAGUE_TABLE + i * 72) as usize;
        if p != 0 && *((p + 0x11c) as *const u8) != 0 && p == target {
            return *((0xb63d74 + i * 72) as *const u8);
        }
    }
    0xff
}


/// the fixture list (0x5abf10 of the manager [0xae2a58]) for the year (+2) and day (+0) of the date, in the given slice
/// and group
/// Original: `FIXTURE_MANAGER::get_fixture_list_for_date` (0x5ae6a0, fix_man.cpp, stdcall, ret 0xc).
///
/// [pt] { 0x5ae6a0 } (stdcall, ret 0xc, fix_man.cpp): `get_fixture_list_for_date(data, fatia, grupo)`: a lista de partidas
/// da 0x5abf10 do gerenciador [0xae2a58] para o ano (+2) e o dia (+0) da data. O argumento do ano leva na metade alta a
/// do ponteiro da data (`mov ax, [eax+2]`), reproduzida; o do dia leva a do ecx do chamador, que a 0x5abf10 descarta
/// (`and 0xffff`): a cópia passa zero. No dia de jogos (set_up_the_day 0x6b3c10, Rust, cada lista do dia), nos
/// amistosos (0x5cbf3e), no técnico humano (0x5fe83a), na mídia (0x7776a8) e em 0x7e6498, 0x7e6ac9.
pub(crate) unsafe extern "stdcall" fn get_fixture_list_for_date(data: usize, slice: u32, group: u32) -> usize {
    let list: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_FIXTURE_MANAGER_GET_FIXTURE_LIST);
    let year = (data as u32 & 0xffff_0000) | core::ptr::read_unaligned((data + 2) as *const u16) as u32;
    let day = core::ptr::read_unaligned(data as *const u16) as u32;
    list(*(G_FIXTURE_MANAGER as *const usize), year, day, slice, group) // o gerenciador de partidas
}


/// marks the match year in both clubs' entries (year × 2 + bit, 2001 rule)
/// Original: `mark_calendar_year` (0x5af180, fix_man.cpp, cdecl).
///
/// [pt] { 0x5af180 } (cdecl, fix_man.cpp): marca o ano (+0x28, palavra) do jogo `jogo` nas entradas dos
/// dois clubes (+0xc e +0x10) da tabela [0xae2a5c] (palavras: ano × 2 + bit 0). Entrada vazia: o
/// bit 0 liga, ou a entrada zera em 2001, e o ano entra; senão em 2001 o bit 0 apaga e o ano sobe se
/// for maior. Tabela nula: a original (caixa de erro).
pub(crate) unsafe extern "C" fn mark_calendar_year(jogo: usize) {
    let table = *(G_CLUB_FIXTURE_YEAR_TABLE as *const usize);
    if table == 0 {
        return original_fn::<unsafe extern "C" fn(usize)>(IDX_MARK_CALENDAR_YEAR)(jogo);
    }
    let year = core::ptr::read_unaligned((jogo + 0x28) as *const u16);
    for o in [0xc, 0x10] {
        let e = table.wrapping_add((*((jogo + o) as *const i32) as usize).wrapping_mul(2)) as *mut u16;
        let v = core::ptr::read_unaligned(e);
        if v & 0xfffe == 0 {
            let w = if year == 0x7d1 { 0 } else { v | 1 };
            core::ptr::write_unaligned(e, w & 1 | year << 1);
        } else {
            if v & 1 != 0 && year == 0x7d1 {
                core::ptr::write_unaligned(e, v & 0xfffe);
            }
            let v = core::ptr::read_unaligned(e);
            if year > v >> 1 {
                core::ptr::write_unaligned(e, year.wrapping_mul(2) | v & 1);
            }
        }
    }
}

