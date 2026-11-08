use super::*;



/// Address of entry i (0x12 bytes) in the UEFA seeding nation table.
///
/// [pt] A entrada `i` da tabela das nações da classificação (registros de 0x12 bytes a partir do objeto: +0 id da nação,
/// +4 a nação, +8 o coeficiente das 5 temporadas (float), +0xc vagas na Liga dos Campeões, +0xd na Copa da UEFA,
/// +0xe na Intertoto, +0xf os pontos da temporada (short), +0x11 os clubes da nação nas copas (char)).
pub(crate) fn seeding_nation_entry(this: usize, i: i32) -> usize {
    this.wrapping_add((i * 0x12) as usize)
}

/// the UEFA coefficient rule: a nation adds up its last 5 seasons (each season = its clubs' points in the European cups
/// ÷ the number of clubs); at the turn of the year the seasons shift by one (in 2001 the game uses the database ones,
/// without shifting); the table is sorted from the highest coefficient down and the position is stored in the nation's
/// national team
/// Original: `UefaSeeding::roll_coefficients` (0x903b50, uefa_seeding.cpp, thiscall, ret 0).
///
/// [pt] { 0x903b50 } (thiscall, ret 0, uefa_seeding.cpp, `UefaSeeding::roll_coefficients`): na virada do ano do jogo
/// (0x9035a0), para cada nação da tabela (+0x438, char): fora de 2001, as 5 temporadas da nação (+0xe0..+0x108, double)
/// andam uma casa e a última vira os pontos da temporada / os clubes nas copas (0 sem clubes); em todo ano, o
/// coeficiente (+8, float) = a soma das 5 (na ordem +0x100, +0xf8, +0xf0, +0xe8, +0xe0, em double, arredondada para
/// float) e os pontos e os clubes zeram. Depois ordena a tabela pelo coeficiente (qsort 0x945333 com 0x905cb0, do
/// maior para o menor) e grava a posição de cada nação no +0x1df da seleção dela (0x53b3d0).
pub(crate) unsafe extern "thiscall" fn roll_coefficients(this: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let conta = || *((this + 0x438) as *const i8);
    let ano = *(G_CURRENT_DATE_YEAR as *const i16);
    let mut i = 0i8;
    while i < conta() {
        let e = seeding_nation_entry(this, i as i32);
        let p = read_u32(e + 4) as usize;
        if ano != 0x7d1 {
            for k in 0..10usize {
                core::ptr::write_unaligned((p + 0xe0 + k * 4) as *mut u32, read_u32(p + 0xe8 + k * 4));
            }
            let clubes = *((e + 0x11) as *const i8);
            if clubes != 0 {
                let pontos = core::ptr::read_unaligned((e + 0xf) as *const i16) as i32;
                core::ptr::write_unaligned((p + 0x108) as *mut f64, pontos as f64 / clubes as i32 as f64);
            } else {
                core::ptr::write_unaligned((p + 0x108) as *mut u64, 0);
            }
        }
        crate::comparacao::ramo("cqi_1 soma dos coeficientes");
        let soma = read_f64(p + 0x100) + read_f64(p + 0xf8) + read_f64(p + 0xf0) + read_f64(p + 0xe8) + read_f64(p + 0xe0); // cqi1: as 5 temporadas
        core::ptr::write_unaligned((e + 8) as *mut f32, soma as f32);
        *((e + 0x11) as *mut u8) = 0;
        core::ptr::write_unaligned((e + 0xf) as *mut u16, 0);
        i += 1;
    }
    ordenar(this, conta() as i32 as u32, 0x12, 0x905cb0);
    let mut i = 0i8;
    while i < conta() {
        let s = selecao(read_u32(seeding_nation_entry(this, i as i32) + 4) as usize);
        *((s + 0x1df) as *mut i8) = i;
        i += 1;
    }
}

/// Up to 16 differing addresses between two memory snapshots.
///
/// [pt] As diferenças entre duas memórias (os 16 primeiros endereços).
pub(crate) fn memory_diff_addresses(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Comparer regions: the nation table and each nation's 5 seasons (+0xe0..+0x110) and national team +0x1df.
///
/// [pt] A tabela das nações e, de cada nação, as 5 temporadas (+0xe0..+0x110) e o +0x1df da seleção.
pub(crate) unsafe fn seeding_regions(this: usize) -> Vec<(usize, usize)> {
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let n = (*((this + 0x438) as *const i8)).max(0) as i32;
    let mut reg = vec![(this, 0x43b)];
    for i in 0..n {
        let p = read_u32(seeding_nation_entry(this, i) + 4) as usize;
        reg.push((p + 0xe0, 0x30));
        reg.push((selecao(p) + 0x1df, 1));
    }
    reg
}


/// places by coefficient position. Champions League: 1st to 3rd 4 places, 4th to 6th 3, 7th to 15th 2, 16th to 48th 1.
/// UEFA Cup: 1st to 6th 3, 7th and 8th 4, 9th to 15th 2, 16th to 21st 3, 22nd to 48th 2, 49th and 50th 1 (Liechtenstein
/// and San Marino none). Intertoto: Moldova, San Marino, Andorra, Bosnia and Liechtenstein 0, Scotland 1; in 2001
/// France 4, Germany 3, Italy 1, Portugal, Norway, Spain and Russia 0; outside 2001 England 1; the others, in
/// coefficient order, 2 places up to the 16th of them, 1 up to the 43rd, then 0. Fair play: three nations get one extra
/// UEFA Cup place (2001: Finland, Slovakia and Belarus; later, three drawn at random, never Italy). Only the Intertoto
/// places are read by the distribution (0x904e30); the Champions League and UEFA Cup ones (and fair play) are stored,
/// but the distribution uses fixed bands by position [inference: they feed the screens]
/// Original: `UefaSeeding::set_place_counts` (0x903cc0, uefa_seeding.cpp, thiscall, ret 0).
///
/// [pt] { 0x903cc0 } (thiscall, ret 0, uefa_seeding.cpp, `UefaSeeding::set_place_counts`): as vagas de cada nação pela
/// posição `i` na tabela já ordenada (0x903b50). Liga dos Campeões (+0xc): 1ª a 3ª 4 vagas, 4ª a 6ª 3, 7ª a 15ª 2, 16ª
/// a 48ª 1, o resto 0 (Liechtenstein antes zerado, mas a faixa grava por cima). Copa da UEFA (+0xd): Liechtenstein e
/// San Marino 0, e por cima 1ª a 6ª 3, 7ª e 8ª 4, 9ª a 15ª 2, 16ª a 21ª 3, 22ª a 48ª 2, 49ª e 50ª 1 (da 51ª em diante
/// fica o que estava). Intertoto (+0xe): Moldávia, San Marino, Andorra, Bósnia e Liechtenstein 0; Escócia 1; em 2001
/// França 4, Alemanha 3, Portugal, Noruega, Espanha e Rússia 0, Itália e Portugal 1; fora de 2001 Inglaterra 1; as
/// outras, na ordem da tabela, 2 vagas até a 16ª delas (e a Bélgica em 2000), 1 até a 43ª, depois 0. O prêmio do fair
/// play: três nações ganham uma vaga a mais na Copa da UEFA; em 2001 a Finlândia, a Eslováquia e a Bielorrússia (sem
/// uma delas na tabela, a caixa da linha 0x285 e o sorteio); fora de 2001, três sorteadas (0x90cfc0 com o número de
/// nações, de três em três) distintas e fora a Itália.
pub(crate) unsafe extern "thiscall" fn set_place_counts(this: usize) {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let g = |a: usize| read_u32(a);
    let conta = || *((this + 0x438) as *const i8);
    let ano = *(G_CURRENT_DATE_YEAR as *const i16);
    let (mut fa, mut fb, mut fc, mut cont) = (-1i8, -1i8, -1i8, 0i8);
    let mut i = 0i8;
    while i < conta() {
        let e = seeding_nation_entry(this, i as i32);
        let id = read_u32(e);
        if id == g(KEY_NATION_FINLAND) {
            fa = i;
        } else if id == g(KEY_NATION_SLOVAKIA) {
            fb = i;
        } else if id == g(KEY_NATION_BELARUS) {
            fc = i;
        }
        if id == g(KEY_NATION_LIECHTENSTEIN) {
            *((e + 0xc) as *mut u8) = 0;
        }
        *((e + 0xc) as *mut u8) = if i < 3 {
            4
        } else if i < 6 {
            3
        } else if i < 0xf {
            2
        } else if i < 0x30 {
            1
        } else {
            0
        };
        let id = read_u32(e);
        if id == g(KEY_NATION_LIECHTENSTEIN) || id == g(KEY_NATION_SAN_MARINO) {
            *((e + 0xd) as *mut u8) = 0;
        }
        let uefa = if i < 6 {
            Some(3)
        } else if i < 8 {
            Some(4)
        } else if i < 0xf {
            Some(2)
        } else if i < 0x15 {
            Some(3)
        } else if i < 0x30 {
            Some(2)
        } else if i < 0x32 {
            Some(1)
        } else {
            None
        };
        if let Some(v) = uefa {
            *((e + 0xd) as *mut u8) = v;
        }
        let id = read_u32(e);
        let intertoto = if id == g(KEY_NATION_MOLDOVA) || id == g(KEY_NATION_SAN_MARINO) || id == g(KEY_NATION_ANDORRA) || id == g(KEY_NATION_BOSNIA_HERZEGOVINA) || id == g(KEY_NATION_LIECHTENSTEIN) {
            0
        } else if id == g(KEY_NATION_SCOTLAND) {
            1
        } else if id == g(KEY_NATION_FRANCE) && ano == 0x7d1 {
            crate::comparacao::ramo("cqi_1 intertoto da franca");
            4 // cqi1: a França na Intertoto em 2001
        } else if id == g(KEY_NATION_GERMANY) && ano == 0x7d1 {
            3
        } else if (id == g(KEY_NATION_PORTUGAL) || id == g(KEY_NATION_NORWAY) || id == g(KEY_NATION_SPAIN) || id == g(KEY_NATION_RUSSIA)) && ano == 0x7d1 {
            0
        } else if id == g(KEY_NATION_ENGLAND) && ano != 0x7d1 {
            1
        } else if (id == g(KEY_NATION_ITALY) || id == g(KEY_NATION_PORTUGAL)) && ano == 0x7d1 {
            1
        } else if cont < 0x10 || (id == g(KEY_NATION_BELGIUM) && ano == 0x7d0) {
            cont += 1;
            2
        } else if cont < 0x2b {
            cont += 1;
            1
        } else {
            cont += 1;
            0
        };
        *((e + 0xe) as *mut u8) = intertoto;
        i += 1;
    }
    let italia = |k: i8| read_u32(seeding_nation_entry(this, k as i32)) == g(KEY_NATION_ITALY);
    if ano == 0x7d1 {
        if fa == -1 || fb == -1 || fc == -1 {
            show_error_at(0xaa1a1c, 0x285);
        }
    } else {
        fa = -1;
    }
    while fa == -1 || fb == -1 || fc == -1 || fa == fb || fa == fc || fb == fc || italia(fa) || italia(fb) || italia(fc) {
        fa = sorteio(conta() as i32) as u8 as i8;
        fb = sorteio(conta() as i32) as u8 as i8;
        fc = sorteio(conta() as i32) as u8 as i8;
    }
    for k in [fa, fb, fc] {
        let p = seeding_nation_entry(this, k as i32) + 0xd;
        *(p as *mut u8) = (*(p as *const u8)).wrapping_add(1);
    }
}


/// in the European club table sorted by nation, the nth club of the nation, skipping on request those already holding a
/// European place and those with a human manager; this is how each place goes to the nation's next club (the order
/// within the nation comes from 0x904040)
/// Original: `uefa_find_nth_unplaced_club_of_nation` (0x9050d0, uefa_seeding.cpp, thiscall, ret 0x18 (this não lido)).
///
/// [pt] { 0x9050d0 } (thiscall, ret 0x18, uefa_seeding.cpp, `uefa_find_nth_unplaced_club_of_nation`; o this não é lido):
/// na tabela dos clubes europeus (registros de 12 bytes ordenados pela nação, 0x904040: +0 o clube, +4 o id da nação,
/// +8, +9, +0xb), a busca binária (bsearch 0x946464 com 0x905ce0; a chave só com a nação e +8 = 0xff, então só a nação
/// conta) acha um clube da nação; recua até o primeiro da nação; anda até o `n`-ésimo (char, 1 = o primeiro) que conta,
/// pulando, se pedido, os que já têm vaga nas copas europeias (+0x1db ≠ −1) e os de técnico humano (0x600a50(clube, 0,
/// 1, 0, 0)). O último registro da tabela nunca é examinado pela volta: se ela chega nele com a conta zerada, ele é o
/// escolhido sem os testes. Devolve o clube se for da nação, senão 0.
pub(crate) unsafe extern "thiscall" fn uefa_find_nth_unplaced_club_of_nation(_this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
    let busca: unsafe extern "C" fn(*const u8, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let k = conta as u16 as i16 as i32;
    let ultimo = clubes.wrapping_add((k * 12 - 12) as usize);
    let mut chave = [0u8; 12];
    chave[4..8].copy_from_slice(&nacao.to_le_bytes());
    chave[8] = 0xff;
    let mut p = busca(chave.as_ptr(), clubes, k, 0xc, 0x905ce0);
    if p == 0 {
        return 0;
    }
    while p > clubes && read_u32(p - 8) == nacao {
        p -= 12;
    }
    let mut falta = (n as u8).wrapping_sub(1);
    let mut achou = false;
    if p < ultimo {
        loop {
            let c = read_u32(p) as usize;
            if sem_vaga != 0 && c != 0 && read_u32(c + 0x1db) != u32::MAX {
                crate::comparacao::ramo("cqi_1 ja com vaga");
                p += 12; // cqi1: pula o clube que já tem vaga
            } else if sem_humano != 0 && c != 0 && humano(c, 0, 1, 0, 0) != 0 {
                p += 12;
            } else {
                if falta == 0 {
                    achou = true;
                    break;
                }
                p += 12;
                falta = falta.wrapping_sub(1);
            }
            if p >= ultimo {
                break;
            }
        }
    }
    if !achou && falta != 0 {
        return 0;
    }
    if read_u32(p + 4) == nacao {
        return read_u32(p) as usize;
    }
    0
}




/// Increments the byte at an address.
pub(crate) unsafe fn increment_byte(a: usize) {
    *(a as *mut u8) = read_u8(a).wrapping_add(1);
}


/// Gives club c a place in competition comp at a phase if it has a nation and no place yet; counts it for nation i.
///
/// [pt] O clube `c` ganha a vaga na competição `comp`, na fase `fase`, se tem nação e ainda não tem vaga (+0x1db = −1):
/// +0x1db = comp, +0x1df = fase e um clube a mais da nação `i` (+0x11). Devolve se ganhou. (O caso da nação sem entrada,
/// −1, que a original procura pela nação do clube, não acontece: `i` vem sempre da volta pelas nações.)
pub(crate) unsafe fn assign_comp_place(this: usize, c: usize, comp: u32, fase: u8, i: i32) -> bool {
    if c == 0 || read_u32(c + 0x53) == 0 || read_u32(c + 0x1db) != u32::MAX {
        return false;
    }
    *((c + 0x1df) as *mut u8) = fase;
    *((c + 0x1db) as *mut u32) = comp;
    increment_byte(seeding_nation_entry(this, i) + 0x11);
    true
}

/// the table of European clubs (national teams excluded), each without a European place, sorted by nation and, within
/// it, by division, position and reputation [inference: the division +0x69 and club +0x5f fields]; in 2001 euro.cfg can
/// move clubs to the front of their nation
/// Original: `UefaSeeding::collect_european_clubs` (0x904040, uefa_seeding.cpp, thiscall, ret 4).
///
/// [pt] { 0x904040 } (thiscall, ret 4, uefa_seeding.cpp, `UefaSeeding::collect_european_clubs`): a tabela dos clubes
/// europeus: aloca 12 bytes por clube do jogo (0x944e46; sem memória, a caixa da linha 0x2ed e devolve 0) e grava o
/// ponteiro em `*saida`; para cada clube (registros de 0x245 em [0xae23b4], [0xae235c] clubes) com nação, que não é
/// seleção (0x53b900) e cuja nação é da Europa (o continente +0x71 com o id de [0x9cfa10]): tira a vaga europeia (+0x1db
/// = −1 e o dword da linha do clube em [0xae2d14], 6 bytes por clube, = −1) e grava o registro (+0 o clube, +4 o id da
/// nação, +8 a reputação +0x80 / 500, +9 o +0x69 da divisão +0x5b [inferência: o nível da divisão], +0xb o +0x5f [inferência: a posição nela]; sem divisão, −1 e 0x7f;
/// com a linha de uma competição sem o +0x11c, a linha e o +0x5f do clube zeram). Ordena pela nação (qsort 0x945333 com
/// 0x905ce0) e, em 2001, aplica a ordem do euro.cfg (0x9059e0, original). Devolve o número de clubes (só os 16 bits
/// baixos contam: quem chama lê o si).
pub(crate) unsafe extern "thiscall" fn collect_european_clubs(this: usize, saida: *mut usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let euro_cfg: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_UEFASEEDING_APPLY_EURO_CFG_ORDER);
    let t = alocar((read_u32(G_CLUB_COUNT) as i32).wrapping_mul(12) as u32);
    *saida = t;
    if t == 0 {
        show_error_at(0xaa1a1c, 0x2ed);
        return 0;
    }
    let mut k = 0u32;
    let mut c = read_u32(G_CLUB_TABLE) as usize;
    let mut i = 0u32;
    if read_u32(G_CLUB_COUNT) as i32 > 0 {
        loop {
            let nacao = read_u32(c + 0x53) as usize;
            if nacao != 0 && selecao(c) == 0 {
                let cont = read_u32(nacao + nation::CONTINENT) as usize;
                if cont != 0 && read_u32(cont) == read_u32(KEY_CONTINENT_EUROPE) {
                    *((c + 0x1db) as *mut u32) = u32::MAX;
                    core::ptr::write_unaligned((read_u32(G_CLUB_STATE_TABLE) as usize).wrapping_add((read_u32(c) as usize).wrapping_mul(6)) as *mut u32, u32::MAX);
                    let r = t.wrapping_add((k as u16 as i16 as i32 * 12) as usize);
                    core::ptr::write_unaligned(r as *mut u32, c as u32);
                    core::ptr::write_unaligned((r + 4) as *mut u32, read_u32(nacao));
                    *((r + 8) as *mut u8) = (core::ptr::read_unaligned((c + 0x80) as *const i16) as i32 / 500) as u8;
                    let linha = read_u32(c + 0x5b) as usize;
                    if linha != 0 {
                        crate::comparacao::ramo("cqi_2 nivel da divisao");
                        core::ptr::write_unaligned((r + 9) as *mut u16, core::ptr::read_unaligned((linha + 0x69) as *const u16)); // cqi2: o nível da divisão
                        *((r + 0xb) as *mut u8) = read_u8(c + 0x5f);
                        let comp = read_u32(read_u32(c + 0x5b) as usize + 0x5d) as usize;
                        if comp == 0 || read_u8(comp + 0x11c) == 0 {
                            core::ptr::write_unaligned((c + 0x5b) as *mut u32, 0);
                            *((c + 0x5f) as *mut u8) = 0;
                        }
                    } else {
                        core::ptr::write_unaligned((r + 9) as *mut u16, 0xffff);
                        *((r + 0xb) as *mut u8) = 0x7f;
                    }
                    k += 1;
                }
            }
            c += 0x245;
            i += 1;
            if (i as u16 as i16 as i32) >= read_u32(G_CLUB_COUNT) as i32 {
                break;
            }
        }
    }
    ordenar(t, k as u16 as i16 as i32, 0xc, 0x905ce0);
    if core::ptr::read_unaligned((this + 0x439) as *const i16) == 0x7d1 {
        euro_cfg(this, t, k);
    }
    // simplification: a original devolve o ax com os bytes altos do eax que sobraram (quem chama lê só o si); a cópia, zero
    k as u16 as u32
}


/// Comparer regions: club table, club state table, random seed and competition history table.
///
/// [pt] Os clubes do jogo, a linha de cada clube em [0xae2d14], a marca da caixa e o histórico das competições ([0xae23d4],\n/// [0xae237c] linhas de 0x1a: a 0x4afd80 o reordena, e com linhas empatadas a ordem de antes muda o clube achado).
pub(crate) unsafe fn club_regions() -> Vec<(usize, usize)> {
    let n = (read_u32(G_CLUB_COUNT) as i32).max(0) as usize;
    vec![(read_u32(G_CLUB_TABLE) as usize, n * club::SIZE), (read_u32(G_CLUB_STATE_TABLE) as usize, n * 6), (0xb67a34, 4), (read_u32(G_CLUB_COMP_HISTORY_TABLE) as usize, (read_u32(G_CLUB_COMP_HISTORY_COUNT) as i32).max(0) as usize * 0x1a)]
}


/// the Champions League rule (the game's rounds 1 to 4 read as groups, third, second and first qualifying round
/// [inference]): the last winner goes straight to the groups; nations, by coefficient position: 1st to 3rd 2 clubs in
/// the groups and 2 in the third qualifying round; 4th to 6th 2 and 1; 7th to 9th 1 and 1; 10th 1 in the groups and 1
/// in the second qualifying round; 11th to 15th 1 in the third and 1 in the second; 16th 1 in the third; 17th to 29th 1
/// in the second; 30th onwards the champion in the first qualifying round. Clubs go in the nation's table order
/// (champion, runner-up...); each round has a capacity (groups 16, or 15 with the holder; third 18, second 19, first
/// 18) and a place in a full round moves to the next one; a club that already has a place loses its own
/// Original: `UefaSeeding::assign_champions_league_places` (0x904210, uefa_seeding.cpp, thiscall, ret 8).
///
/// [pt] { 0x904210 } (thiscall, ret 8, uefa_seeding.cpp, `UefaSeeding::assign_champions_league_places`): as vagas da Liga dos
/// Campeões ([0x9cf6e8]). O campeão da última edição (0x4afd80 com a linha da competição em [0xae23d0], 0x6b bytes, e o
/// lugar 0) vai direto à fase de grupos (fase 1; a fase de grupos fica com 15 vagas); sem ele, a caixa da linha 0x35d.
/// Depois, nação por nação na ordem do coeficiente (a posição `r`), as vagas por fase (1 = grupos, 2 = terceira
/// preliminar, 3 = segunda, 4 = primeira) pelas faixas fixas: 1ª a 3ª nação 2 nos grupos e 2 na terceira preliminar; 4ª
/// a 6ª 2 e 1; 7ª a 9ª 1 e 1; 10ª 1 nos grupos e 1 na segunda; 11ª a 15ª 1 na terceira e 1 na segunda; 16ª 1 na
/// terceira; 17ª a 29ª 1 na segunda; da 30ª em diante 1 na primeira. Para k = 1..7, o k-ésimo clube da nação
/// (0x9050d0, sem pular ninguém) vai à primeira fase com vaga pedida; fase cheia (as capacidades 16/15, 18, 19 e 18) passa
/// uma vaga para a fase seguinte; clube sem nação ou já com vaga perde a vaga.
pub(crate) unsafe extern "thiscall" fn assign_champions_league_places(this: usize, clubes: usize, conta: u32) {
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_UEFA_FIND_NTH_UNPLACED_CLUB_OF_NATION);
    let lc = read_u32(KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE);
    let mut cap: [u8; 5] = [0, 0x10, 0x12, 0x13, 0x12];
    let h = campeao((read_u32(G_CLUB_COMP_TABLE) as usize).wrapping_add(lc.wrapping_mul(0x6b) as usize), 0);
    if h != 0 && read_u32(h + 0x53) != 0 && read_u32(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = lc;
        let id = read_u32(read_u32(h + 0x53) as usize);
        let n = *((this + 0x438) as *const i8) as i32;
        let mut i = -1i32;
        for k in 0..n {
            if read_u32(seeding_nation_entry(this, k)) == id {
                i = k;
                break;
            }
        }
        increment_byte(seeding_nation_entry(this, i) + 0x11);
        cap[1] = 0xf;
    } else {
        show_error_at(0xaa1a1c, 0x35d);
    }
    let mut r = 0i8;
    while r < *((this + 0x438) as *const i8) {
        let (band, ultima): ([u8; 4], u8) = if r < 3 {
            crate::comparacao::ramo("cqi_2 duas nos grupos");
            ([0, 2, 2, 0], 0) // cqi2: 1ª a 3ª nação
        } else if r < 6 {
            ([0, 2, 1, 0], 0)
        } else if r < 9 {
            ([0, 1, 1, 0], 0)
        } else if r < 0xa {
            ([0, 1, 0, 1], 0)
        } else if r < 0xf {
            ([0, 0, 1, 1], 0)
        } else if r < 0x10 {
            ([0, 0, 1, 0], 0)
        } else if r < 0x1d {
            ([0, 0, 0, 1], 0)
        } else {
            ([0, 0, 0, 0], 1)
        };
        let mut vagas = [band[0], band[1], band[2], band[3], ultima];
        let e = seeding_nation_entry(this, r as i32);
        let mut k = 1u8;
        while k < 8 {
            let c = enesimo(this, clubes, conta, read_u32(e), k as u32, 0, 0);
            let mut f = 1usize;
            while f < 5 {
                if vagas[f] != 0 {
                    if cap[f] != 0 {
                        break;
                    }
                    crate::comparacao::ramo("cqi_2 fase cheia");
                    if f + 1 < 5 {
                        vagas[f + 1] = vagas[f + 1].wrapping_add(1); // cqi2: a vaga passa para a fase seguinte
                    }
                    vagas[f] = vagas[f].wrapping_sub(1);
                }
                f += 1;
            }
            if f < 5 {
                if assign_comp_place(this, c, lc, f as u8, r as i32) {
                    cap[f] = cap[f].wrapping_sub(1);
                }
                vagas[f] = vagas[f].wrapping_sub(1);
            }
            k += 1;
        }
        r += 1;
    }
}


/// the Intertoto rule: 60 clubs; it cycles through the nations in coefficient order, each nation with its places (from
/// 0x903cc0) and its first clubs still without a European place; the first 8 nations enter round 1, the 9th to 12th
/// round 2, then round 3 (a nation's second club one round later). England puts in the volunteer club (0x903890) or the
/// first one without a place and without a human manager, and a second one when it has 2 places
/// Original: `UefaSeeding::assign_intertoto_places` (0x904e30, uefa_seeding.cpp, thiscall, ret 8).
///
/// [pt] { 0x904e30 } (thiscall, ret 8, uefa_seeding.cpp, `UefaSeeding::assign_intertoto_places`): as vagas da Intertoto
/// ([0x9cf6f8]) até 60 clubes: dá voltas pelas nações na ordem do coeficiente (a conta vai a `n` e volta a 0, e a fase
/// sai da conta antes de voltar: abaixo de 8 a fase 1, de 8 a 11 a 2, de 12 em diante a 3). A Inglaterra: um clube (o
/// voluntário da 0x903890; sem ele, o primeiro sem vaga e sem humano, 0x9050d0) e, com 2 vagas (+0xe), o segundo, na
/// fase seguinte; depois solta a lista dos voluntários (+0x43b, +0x43f). As outras: tantos clubes quanto as vagas +0xe,
/// o primeiro sem vaga de cada vez (humanos podem), na fase + a ordem do clube, enquanto houver menos de 60.
pub(crate) unsafe extern "thiscall" fn assign_intertoto_places(this: usize, clubes: usize, conta: u32) {
    let voluntario: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_UEFASEEDING_PICK_ENGLISH_INTERTOTO_CLUB);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_UEFA_FIND_NTH_UNPLACED_CLUB_OF_NATION);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let it = read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP);
    let mut colocados = 0i32;
    let mut cl = 0u8;
    loop {
        let fase: u8 = if (cl as i8) < 8 { 1 } else if (cl as i8) >= 0xc { 3 } else { 2 };
        let mut i = cl;
        if cl == read_u8(this + 0x438) {
            i = 0;
        }
        let e = seeding_nation_entry(this, i as i8 as i32);
        if read_u32(e) == read_u32(KEY_NATION_ENGLAND) {
            let mut c = voluntario(this);
            if c == 0 {
                c = enesimo(this, clubes, conta, read_u32(e), 1, 1, 1);
            }
            if assign_comp_place(this, c, it, fase, i as i8 as i32) {
                colocados += 1;
            }
            if read_u8(e + 0xe) == 2 {
                let mut c = voluntario(this);
                if c == 0 {
                    c = enesimo(this, clubes, conta, read_u32(e), 1, 1, 1);
                }
                if assign_comp_place(this, c, it, fase.wrapping_add(1), i as i8 as i32) {
                    colocados += 1;
                }
            }
            let lista = read_u32(this + 0x43b) as usize;
            if lista != 0 {
                soltar(lista);
                core::ptr::write_unaligned((this + 0x43b) as *mut u32, 0);
                *((this + 0x43f) as *mut u8) = 0;
            }
        } else if (read_u8(e + 0xe) as i8) > 0 {
            let mut j = 0i8;
            loop {
                if colocados as i16 >= 0x3c {
                    break;
                }
                let c = enesimo(this, clubes, conta, read_u32(e), 1, 1, 0);
                if assign_comp_place(this, c, it, (j as u8).wrapping_add(fase), i as i8 as i32) {
                    crate::comparacao::ramo("cqi_2 clube na intertoto");
                    colocados += 1; // cqi2: mais um na Intertoto
                }
                j += 1;
                if j >= read_u8(e + 0xe) as i8 {
                    break;
                }
            }
        }
        cl = i.wrapping_add(1);
        if colocados as i16 >= 0x3c {
            break;
        }
    }
}






/// Club c (with nation, no place) enters the UEFA Cup at a phase; counts it for nation r.
///
/// [pt] O clube `c` (com nação e ainda sem vaga) entra na Copa da UEFA na fase `fase`; um clube a mais da nação `r`. Devolve
/// se entrou. (O caso da nação −1, que a original procura pela nação do clube, não acontece: `r` vem da volta.)
pub(crate) unsafe fn enter_uefa_cup(this: usize, c: usize, fase: u8, r: i32, colocados: &mut i32) -> bool {
    if read_u32(c + 0x53) == 0 || read_u32(c + 0x1db) != u32::MAX {
        return false;
    }
    *((c + 0x1db) as *mut u32) = read_u32(KEY_CLUB_COMP_UEFA_CUP);
    *((c + 0x1df) as *mut u8) = fase;
    *colocados += 1;
    let p = seeding_nation_entry(this, r) + 0x11;
    *(p as *mut u8) = read_u8(p).wrapping_add(1);
    true
}

/// the UEFA Cup rule (the game's round 1 = the first round, round 2 = the qualifying round [inference]): the last
/// winner enters the first round and its nation loses one place in that round; three nations drawn at random (fair
/// play) get one place in the qualifying round; by coefficient position, places in the first round and the qualifying
/// round: 1st to 6th 3 and 0, 7th and 8th 4 and 0, 9th to 15th 1 and 1, 16th and 17th 1 and 2, 18th to 21st 0 and 3,
/// 22nd to 48th 0 and 2, 49th onwards 0 and 1. A nation's first place goes to the national cup winner (or the runner-up
/// if the winner is already in the Champions League; with both already qualified, to the finalists of the cup still to
/// be played stored in the seeding, except Spain and Belgium), the second to the league cup winner, the others to the
/// top clubs of the table still without a place
/// Original: `UefaSeeding::assign_uefa_cup_places` (0x904580, uefa_seeding.cpp, thiscall, ret 8).
///
/// [pt] { 0x904580 } (thiscall, ret 8, uefa_seeding.cpp, `UefaSeeding::assign_uefa_cup_places`): as vagas da Copa da UEFA
/// ([0x9cf6f0]). O campeão da última edição (0x4afd80 com a linha da competição e o lugar 0) na fase 1, e a nação dele
/// com uma vaga a menos na fase 1. Três nações sorteadas (0x90cfc0 com o número de nações; a 2ª e a 3ª de novo até as
/// três serem distintas) ganham uma vaga a mais na preliminar (o fair play). Por posição da nação no coeficiente, as
/// vagas na fase 1 e na preliminar (fase 2): 1ª a 6ª 3 e 0, 7ª e 8ª 4 e 0, 9ª a 15ª 1 e 1, 16ª e 17ª 1 e 2, 18ª a 21ª 0
/// e 3, 22ª a 48ª 0 e 2, da 49ª em diante 0 e 1. Os clubes de cada nação, na ordem: 1º o campeão da copa nacional (a
/// copa da nação em 0xb63d7e + linha × 0x48, linha = 0x669340), ou o vice se o campeão já tem vaga (a Bélgica e o vice
/// sem nação: o seguinte da tabela), e, com os dois já com vaga, a final guardada em +0x440 (50 registros: copa,
/// campeão, vice) quando a copa da temporada ainda não foi disputada (o ano +0x40 da copa ≠ o da nação em 0xb63d7b;
/// fora Espanha e Bélgica); 2º o campeão da copa da liga (0xb63d82), se ainda sem vaga; os outros, o primeiro sem vaga da
/// tabela (0x9050d0(.., 1, 1, 0)); sem copa nacional na linha, a caixa da linha 0x418 e a vaga é tentada de novo.
/// Com a final guardada e os dois finalistas: uma vaga a mais para quem dos dois ainda não tem (os dois sem vaga entram
/// na fase 10 × a conta + 3; os dois com vaga, o primeiro sem vaga da tabela). Para quando os colocados menos a conta
/// passam de 116.
pub(crate) unsafe extern "thiscall" fn assign_uefa_cup_places(this: usize, clubes: usize, conta: u32) {
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_UEFA_FIND_NTH_UNPLACED_CLUB_OF_NATION);
    let linha: unsafe extern "C" fn(usize) -> i8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_NATION);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let n8 = || *((this + 0x438) as *const i8);
    let uc = read_u32(KEY_CLUB_COMP_UEFA_CUP);
    let mut conta_extra = 1i8;
    let mut colocados = 0i32;
    let mut nacao_do_campeao = 0usize;
    let h = campeao((read_u32(G_CLUB_COMP_TABLE) as usize).wrapping_add(uc.wrapping_mul(0x6b) as usize), 0);
    if h != 0 && read_u32(h + 0x53) != 0 && read_u32(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = uc;
        colocados = 1;
        let id = read_u32(read_u32(h + 0x53) as usize);
        let mut i = -1i32;
        for k in 0..n8() as i32 {
            if read_u32(seeding_nation_entry(this, k)) == id {
                i = k;
                break;
            }
        }
        let p = seeding_nation_entry(this, i) + 0x11;
        *(p as *mut u8) = read_u8(p).wrapping_add(1);
        nacao_do_campeao = read_u32(h + 0x53) as usize;
    }
    let s1 = sorteio(n8() as i32) as u8 as i8;
    let mut s2 = sorteio(n8() as i32) as u8 as i8;
    let mut s3 = sorteio(n8() as i32) as u8 as i8;
    while s1 == s2 || s1 == s3 || s2 == s3 {
        s2 = sorteio(n8() as i32) as u8 as i8;
        s3 = sorteio(n8() as i32) as u8 as i8;
    }
    let mut ultima_fase = 0u8;
    let mut r = 0i8;
    while r < n8() {
        let (f1, f2): (u8, u8) = if r < 6 {
            crate::comparacao::ramo("cqi_3 tres na primeira fase");
            (3, 0) // cqi3: 1ª a 6ª nação
        } else if r < 8 {
            (4, 0)
        } else if r < 0xf {
            (1, 1)
        } else if r < 0x11 {
            (1, 2)
        } else if r < 0x15 {
            (0, 3)
        } else if r < 0x30 {
            (0, 2)
        } else {
            (0, 1)
        };
        let mut vagas = [0u8, f1, f2];
        let e = seeding_nation_entry(this, r as i32);
        let nacao = read_u32(e + 4) as usize;
        if nacao == nacao_do_campeao {
            vagas[1] = vagas[1].wrapping_sub(1);
        }
        if r == s1 || r == s2 || r == s3 {
            crate::comparacao::ramo("cqi_3 fair play");
            vagas[2] = vagas[2].wrapping_add(1); // cqi3: a vaga do fair play
        }
        let um = || enesimo(this, clubes, conta, read_u32(e), 1, 1, 0);
        let mut escolhidos = 0u8;
        let (mut vencedor, mut vice) = (0usize, 0usize);
        let mut f = 1u8;
        while f < 3 {
            let lim = vagas[f as usize] as i8;
            if lim > 0 {
                ultima_fase = f;
                let mut j = 0i8;
                loop {
                    let mut tentar_de_novo = false;
                    let c = if escolhidos == 0 {
                        if read_u8(nacao + nation::NATION_LEAGUE_SELECTED) == 0 {
                            um()
                        } else {
                            let copa = read_u32(G_LEAGUE_WORLD_CUP_CUP_COMP + (linha(nacao) as i32 * 0x48) as usize) as usize;
                            if copa == 0 {
                                show_error_at(0xaa1a1c, 0x418);
                                tentar_de_novo = true;
                                0
                            } else {
                                let mut c = campeao(copa, 0);
                                if c == 0 || read_u32(c + 0x53) == 0 {
                                    c = um();
                                }
                                if read_u32(c + 0x1db) == u32::MAX {
                                    c
                                } else {
                                    let mut c = campeao(copa, 1);
                                    if c == 0 || read_u32(c + 0x53) == 0 || read_u32(nacao) == read_u32(KEY_NATION_BELGIUM) {
                                        c = um();
                                    }
                                    if read_u32(c + 0x1db) == u32::MAX {
                                        c
                                    } else if read_u32(G_COMP_OBJECT_TABLE) == 0 {
                                        um()
                                    } else {
                                        let id = read_u32(copa);
                                        let ano = core::ptr::read_unaligned((0xb63d7b + (linha(nacao) as i32 * 0x48) as usize) as *const u16) as i32;
                                        let comp = read_u32((read_u32(G_COMP_OBJECT_TABLE) as usize).wrapping_add(id as usize * 4)) as usize;
                                        if read_i16(comp + 0x40) as i32 == ano || read_u32(nacao) == read_u32(KEY_NATION_SPAIN) || read_u32(nacao) == read_u32(KEY_NATION_BELGIUM) {
                                            um()
                                        } else {
                                            let (mut v1, mut v2) = (-1i32, -1i32);
                                            for k in 0..0x32usize {
                                                if read_u32(this + 0x440 + k * 12) == id {
                                                    v1 = read_u32(this + 0x444 + k * 12) as i32;
                                                    v2 = read_u32(this + 0x448 + k * 12) as i32;
                                                    break;
                                                }
                                            }
                                            let base = read_u32(G_CLUB_TABLE) as usize;
                                            vencedor = if v1 != -1 { base.wrapping_add((v1 as usize).wrapping_mul(club::SIZE)) } else { 0 };
                                            vice = if v2 != -1 { base.wrapping_add((v2 as usize).wrapping_mul(club::SIZE)) } else { 0 };
                                            if vencedor == 0 {
                                                um()
                                            } else if vice != 0 {
                                                0
                                            } else {
                                                c
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else if escolhidos == 1 {
                        if read_u8(nacao + nation::NATION_LEAGUE_SELECTED) == 0 {
                            um()
                        } else {
                            let copa = read_u32(G_LEAGUE_WORLD_CUP_SECOND_CUP_COMP + (linha(nacao) as i32 * 0x48) as usize) as usize;
                            if copa == 0 {
                                um()
                            } else {
                                let c = campeao(copa, 0);
                                if c == 0 || read_u32(c + 0x53) == 0 || read_u32(c + 0x1db) != u32::MAX {
                                    um()
                                } else {
                                    c
                                }
                            }
                        }
                    } else {
                        um()
                    };
                    if tentar_de_novo {
                        j -= 1;
                    } else {
                        escolhidos = escolhidos.wrapping_add(1);
                        if c != 0 {
                            enter_uefa_cup(this, c, f, r as i32, &mut colocados);
                            if colocados as i16 as i32 - conta_extra as i32 + 1 >= 0x76 {
                                return;
                            }
                        }
                    }
                    j += 1;
                    if j >= lim {
                        break;
                    }
                }
            }
            f += 1;
        }
        if vencedor != 0 && vice != 0 {
            conta_extra += 1;
            let (a, b) = (read_u32(vencedor + 0x1db) == u32::MAX, read_u32(vice + 0x1db) == u32::MAX);
            if !a && !b {
                let c = um();
                if c != 0 {
                    enter_uefa_cup(this, c, ultima_fase, r as i32, &mut colocados);
                }
            } else if a && b {
                let fase = (conta_extra as u8).wrapping_mul(10).wrapping_add(3);
                ultima_fase = fase;
                if read_u32(vencedor + 0x53) != 0 {
                    enter_uefa_cup(this, vencedor, fase, r as i32, &mut colocados);
                }
                enter_uefa_cup(this, vice, fase, r as i32, &mut colocados);
            } else {
                enter_uefa_cup(this, if a { vencedor } else { vice }, ultima_fase, r as i32, &mut colocados);
            }
        }
        r += 1;
    }
}



/// England's Intertoto club: outside 2001, up to 6 volunteers drawn among Premier clubs without a human manager or a
/// European place; the volunteer from the highest division goes (the best placed [inference: +0x5f]); without
/// volunteers, a random club without a place; with the news to the division. In 2001 England uses the table (0x904e30)
/// Original: `UefaSeeding::pick_english_intertoto_club` (0x903890, uefa_seeding.cpp, thiscall, ret 0).
///
/// [pt] { 0x903890 } (thiscall, ret 0, uefa_seeding.cpp, `UefaSeeding::pick_english_intertoto_club`; quadro SEH pela notícia
/// local, sem destrutor no caminho normal): o clube inglês da Intertoto. Em 2001, ou sem a Premier ([0xadadfc] +
/// [0x9cf5bc] × 4), devolve 0. Senão completa a lista dos voluntários (+0x43b, 30 ids, alocada com 0x78 bytes; +0x43f a
/// conta) até 6, sorteando clubes da Premier (0x90cfc0 com +0x3e, até 80 sorteios) sem técnico humano e sem vaga
/// europeia; escolhe o voluntário sem vaga da divisão mais alta (a da Premier +4 conta o +0x5f; as outras +30); sem
/// nenhum, sorteia clubes da Premier até um sem vaga. Entrega a notícia 0x17 com o clube (0x77bbe0, 0x785790, 0x7862d0
/// à divisão da Premier) e devolve o clube.
pub(crate) unsafe extern "thiscall" fn pick_english_intertoto_club(this: usize) -> usize {
    if read_i16(G_CURRENT_DATE_YEAR) == 0x7d1 {
        return 0;
    }
    let tabela = read_u32(G_COMP_OBJECT_TABLE) as usize;
    if tabela == 0 {
        return 0;
    }
    let liga = read_u32(tabela.wrapping_add(read_u32(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) as usize * 4)) as usize;
    if liga == 0 {
        return 0;
    }
    crate::comparacao::ramo("cqi_3 voluntario ingles");
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let a_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_DIVISION_MANAGERS);
    let clube_sorteado = || {
        let s = sorteio(read_i16(liga + 0x3e) as i32) as i16 as i32;
        read_u32((read_u32(liga + 0xb1) as usize).wrapping_add((s * 0x3b) as usize)) as usize
    };
    let mut tentativas = 0i8;
    while (read_u8(this + 0x43f) as i8) < 6 && tentativas < 0x50 {
        tentativas += 1;
        let c = clube_sorteado();
        if humano(c, 0, 1, 0, 0) == 0 && read_u32(c + 0x1db) == u32::MAX {
            if read_u32(this + 0x43b) == 0 {
                *((this + 0x43f) as *mut u8) = 0;
                let l = alocar(0x78);
                core::ptr::write_unaligned((this + 0x43b) as *mut u32, l as u32);
                if l == 0 {
                    show_error_at(0xaa1a1c, 0xf9);
                    continue;
                }
            }
            let k = read_u8(this + 0x43f) as i8;
            if k < 0x1e {
                core::ptr::write_unaligned((read_u32(this + 0x43b) as usize + k as usize * 4) as *mut u32, read_u32(c));
                *((this + 0x43f) as *mut u8) = (k as u8).wrapping_add(1);
            }
        }
    }
    let mut melhor = 0usize;
    let mut nivel = 0x64i8;
    let k = read_u8(this + 0x43f) as i8 as i32;
    if k > 0 {
        let lista = read_u32(this + 0x43b) as usize;
        let base = read_u32(G_CLUB_TABLE) as usize;
        for q in 0..k as usize {
            let c = base.wrapping_add((read_u32(lista + q * 4) as usize).wrapping_mul(club::SIZE));
            if read_u32(c + 0x1db) == u32::MAX {
                let v = read_u8(c + 0x5f) as i8;
                if read_u32(c + 0x5b) == read_u32(liga + 4) {
                    if v < nivel {
                        melhor = c;
                        nivel = v;
                    }
                } else if v as i32 + 0x1e < nivel as i32 {
                    melhor = c;
                    nivel = v.wrapping_add(0x1e);
                }
            }
        }
    }
    while melhor == 0 {
        let c = clube_sorteado();
        if read_u32(c + 0x1db) == u32::MAX {
            melhor = c;
        }
    }
    let mut noticia = [0u8; 0xe8];
    let n = noticia.as_mut_ptr() as usize;
    construir(n, 0x17, 0);
    gravar(n, 0, read_u32(melhor));
    a_divisao(read_u32(G_NEWS) as usize, n, read_u32(liga + 4) as usize);
    melhor
}


/// the seeding calendar: every new game year it redoes the coefficients, the places per nation and the Champions
/// League, UEFA Cup and Intertoto places, and posts the qualifiers news; on day 100 of the year it asks Premier human
/// managers for Intertoto volunteers; every day, the European registration deadline
/// Original: `UefaSeeding::update_season` (0x9035a0, uefa_seeding.cpp, thiscall, ret 4).
///
/// [pt] { 0x9035a0 } (thiscall, ret 4, uefa_seeding.cpp, `UefaSeeding::update_season`; quadro SEH pela notícia local, sem
/// destrutor no caminho normal): chamada pelas copas europeias. Sem `diario`: quando o ano do jogo ([0xae2c92]) é outro
/// que o +0x439, grava o ano e refaz a classificação: os coeficientes (0x903b50), as vagas por nação (0x903cc0), a tabela
/// dos clubes (0x904040; vazia, a caixa da linha 0x2b7), as vagas da Liga dos Campeões (0x904210), da Copa da UEFA
/// (0x904580) e da Intertoto (0x904e30), solta a tabela e entrega a notícia dos classificados (0x905f00). Com `diario`:
/// no dia 100 do ano, com a Premier e a Intertoto no jogo, a notícia 0x16 do pedido de voluntários (com o ano) a cada
/// técnico humano de clube da Premier (sem a Premier, volta sem mais nada); e o prazo das inscrições europeias (0x9058e0).
pub(crate) unsafe extern "thiscall" fn update_season(this: usize, diario: u32) {
    if diario == 0 {
        let ano = read_i16(G_CURRENT_DATE_YEAR);
        if ano == read_i16(this + 0x439) {
            return;
        }
        crate::comparacao::ramo("cqi_3 virada da classificacao");
        core::ptr::write_unaligned((this + 0x439) as *mut i16, ano);
        let coeficientes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_UEFASEEDING_ROLL_COEFFICIENTS);
        let vagas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_UEFASEEDING_SET_PLACE_COUNTS);
        let juntar: unsafe extern "thiscall" fn(usize, *mut usize) -> u32 = core::mem::transmute(FN_UEFASEEDING_COLLECT_EUROPEAN_CLUBS);
        let campeoes: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_UEFASEEDING_ASSIGN_CHAMPIONS_LEAGUE_PLACES);
        let copa_uefa: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_UEFASEEDING_ASSIGN_UEFA_CUP_PLACES);
        let intertoto: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_UEFASEEDING_ASSIGN_INTERTOTO_PLACES);
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        let classificados: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_UEFA_POST_QUALIFIERS_NEWS);
        coeficientes(this);
        vagas(this);
        let mut t = 0usize;
        let n = juntar(this, &mut t);
        if n as u16 == 0 {
            show_error_at(0xaa1a1c, 0x2b7);
        } else {
            campeoes(this, t, n);
            copa_uefa(this, t, n);
            intertoto(this, t, n); // cqi3: as vagas da Intertoto
            soltar(t);
        }
        classificados(this);
        return;
    }
    if *(G_CURRENT_DATE as *const i16) == 100 {
        let tabela = read_u32(G_COMP_OBJECT_TABLE) as usize;
        let liga = read_u32(tabela.wrapping_add(read_u32(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) as usize * 4)) as usize;
        if liga == 0 {
            return;
        }
        if read_u32(tabela.wrapping_add(read_u32(KEY_CLUB_COMP_INTER_TOTO_CUP) as usize * 4)) != 0 {
            let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
            let gravar: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
            let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NEWS_CLUB_HAS_HUMAN_MANAGER);
            let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_NEWS_SEND_TO_CLUB_MANAGER);
            let mut noticia = [0u8; 0xe8];
            let p = noticia.as_mut_ptr() as usize;
            construir(p, 0x16, 0);
            gravar(p, 0, 0);
            gravar(p, 2, read_i16(this + 0x439) as u16 as u32);
            let mut i = 0i16;
            while i < read_i16(liga + 0x3e) {
                let c = read_u32((read_u32(liga + 0xb1) as usize).wrapping_add(i as i32 as usize * 0x3b)) as usize;
                if humano(read_u32(G_NEWS) as usize, c) != 0 {
                    gravar(p, 1, read_u32(c));
                    entregar(read_u32(G_NEWS) as usize, p, c);
                }
                i += 1;
            }
        }
    }
    let prazos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_UEFA_CHECK_REGISTRATION_DEADLINES);
    prazos(this);
}



/// Reads a signed byte count as u32 (movsx).
///
/// [pt] A contagem de um byte com sinal (a original usa `movsx`).
pub(crate) unsafe fn signed_byte_count(a: usize) -> u32 {
    *(a as *const i8) as i32 as u32
}

/// Opens CM3_TEMP\<name> with "wb" through the game's path builder and fopen.
///
/// [pt] Abre `CM3_TEMP\<nome>` para gravar ("wb"): o caminho pela 0x90bde0 e o fopen do jogo (0x945b00).
pub(crate) unsafe fn open_temp_file_for_write(nome: usize) -> usize {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    abrir(caminho(0x987da0, nome), 0x9a7f28)
}

/// Calls the exe's CRT fwrite.
pub(crate) unsafe fn crt_fwrite(dados: usize, tamanho: u32, quantos: u32, f: usize) -> u32 {
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FWRITE);
    escrever(dados, tamanho, quantos, f)
}

/// Calls the exe's CRT fclose.
pub(crate) unsafe fn uefa_seeding_crt_fclose(f: usize) {
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    fechar(f);
}

/// writes CM3_TEMP\uefa_seeding.tmp: the country count (+0x438), 0x12 bytes per country from +0, +0x439 (u16), as many
/// u32 as +0x43f from [+0x43b] and 50 records of 0xc from +0x440; on error the "Error" box without closing the file
/// Original: `UefaSeeding::save_to_temp_file` (0x905510, uefa_seeding.cpp, thiscall).
///
/// [pt] { 0x905510 } (thiscall, ret 0, uefa_seeding.cpp): `UefaSeeding::save_to_temp_file`: grava `CM3_TEMP\uefa_seeding.tmp`:
/// a quantidade de países (+0x438, i8), os registros de 0x12 bytes de +0, +0x439 (u16), a quantidade +0x43f (i8) e, se
/// não for zero, essa quantidade de u32 de [+0x43b], e os 50 registros fixos de 0xc bytes de +0x440. Erro ao abrir ou
/// gravar: a caixa "Error" (uefa_seeding.cpp 0x64f, 0x657, 0x660, 0x668, 0x66f, 0x679, 0x682), sem fechar o arquivo
/// (como a original).
pub(crate) unsafe extern "thiscall" fn uefa_seeding_save_to_temp_file(this: usize) {
    const FONTE: usize = 0xaa1a1c;
    crate::comparacao::ramo("cpx_6 gravar sementes da uefa");
    let f = open_temp_file_for_write(0xaa1a50);
    if f == 0 {
        show_error_at(FONTE, 0x64f);
        return;
    }
    if crt_fwrite(this + 0x438, 1, 1, f) != 1 {
        show_error_at(FONTE, 0x657);
        return;
    }
    if crt_fwrite(this, 0x12, signed_byte_count(this + 0x438), f) != signed_byte_count(this + 0x438) {
        show_error_at(FONTE, 0x660);
        return;
    }
    if crt_fwrite(this + 0x439, 2, 1, f) != 1 {
        show_error_at(FONTE, 0x668);
        return;
    }
    if crt_fwrite(this + 0x43f, 1, 1, f) != 1 {
        show_error_at(FONTE, 0x66f);
        return;
    }
    if *((this + 0x43f) as *const u8) != 0 && crt_fwrite(read_usize(this + 0x43b), 4, signed_byte_count(this + 0x43f), f) != signed_byte_count(this + 0x43f) {
        show_error_at(FONTE, 0x679);
        return;
    }
    if crt_fwrite(this + 0x440, 0xc, 0x32, f) != 0x32 { // cpx6: os 50 registros fixos
        show_error_at(FONTE, 0x682);
        return;
    }
    uefa_seeding_crt_fclose(f);
}






