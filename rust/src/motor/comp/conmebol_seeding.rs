use super::*;



/// Club (non-null and without a place, +0x1db = -1) enters the Copa Libertadores at stage `fase`; returns whether it
/// entered.
///
/// [pt] O clube `c` (não nulo e ainda sem vaga, +0x1db = −1) entra na Libertadores na fase `fase`. Devolve se entrou.
pub(crate) unsafe fn enter_libertadores(c: usize, fase: u8) -> bool {
    if c == 0 || read_u32(c + 0x1db) != u32::MAX {
        return false;
    }
    *((c + 0x1db) as *mut u32) = read_u32(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES);
    *((c + 0x1df) as *mut u8) = fase;
    true
}

/// the Libertadores rule: the last winner enters round 1 and the others round 2 [inference: groups and preliminary];
/// the 11 nations of the seeding table (with Brazil, Mexico and the South American ones), each with its quota; Brazil,
/// up to 2002, with Palmeiras and Cruzeiro fixed, from 2003 with the Copa do Brasil winner (or runner-up) and the Copa
/// dos Campeões winner; and always with the Brasileirão champion and runner-up; each nation fills its quota with the
/// top clubs of the table still without a place (no second try: an already qualified club makes the place be lost)
/// Original: `ConmebolSeeding::assign_libertadores_places` (0x4c6430, conmebol_seeding.cpp, thiscall, ret 8).
///
/// [pt] { 0x4c6430 } (thiscall, ret 8, conmebol_seeding.cpp, `ConmebolSeeding::assign_libertadores_places`): as vagas da
/// Libertadores ([0x9cf63c]). O campeão da última edição (0x4afd80 com a linha da competição e o lugar 0) entra na fase 1
/// (sem ele ou já com vaga, a caixa da linha 0x114). Depois, as 11 nações da tabela (registros de 10 bytes a partir do
/// objeto: +0 id da nação, +8 a cota), cada uma até a cota, na fase 2: o Brasil, até 2002, o Palmeiras e o Cruzeiro (as
/// chaves 0x9d0274 e 0x9d026c); de 2003 em diante o campeão da Copa do Brasil (ou o vice, se o campeão já tem vaga) e o
/// campeão da Copa dos Campeões (+0x71); e sempre o campeão e o vice do Brasileirão (0x9cf9ec); cada nação completa a
/// cota com os primeiros da tabela ainda sem vaga (0x4c6730(.., 1, 1, 0)), uma tentativa por vaga.
pub(crate) unsafe extern "thiscall" fn assign_libertadores_places(this: usize, clubes: usize, conta: u32) {
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CONMEBOLSEEDING_FIND_NTH_UNQUALIFIED_CLUB_OF_NATION);
    let linha = |chave: usize| (read_u32(G_CLUB_COMP_TABLE) as usize).wrapping_add(read_u32(chave).wrapping_mul(0x6b) as usize);
    let h = campeao(linha(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES), 0);
    if h != 0 && read_u32(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = read_u32(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES);
    } else {
        show_error_at(0x9a7ed8, 0x114);
    }
    for k in 0..0xbusize {
        let e = this + k * 10;
        let mut n = 0u8;
        if read_u32(e) == read_u32(KEY_NATION_BRAZIL) {
            if *(G_CURRENT_DATE_YEAR as *const u16) > 0x7d2 {
                let mut c = campeao(linha(KEY_CLUB_COMP_BRAZILIAN_CUP), 0);
                if c == 0 || read_u32(c + 0x1db) != u32::MAX {
                    c = campeao(linha(KEY_CLUB_COMP_BRAZILIAN_CUP), 1);
                }
                if enter_libertadores(c, 2) {
                    n = 1;
                }
                let id = read_u32(this + 0x71);
                if id != u32::MAX && enter_libertadores(contract_manager_club_by_id(id), 2) {
                    n = n.wrapping_add(1);
                }
            } else {
                if enter_libertadores(contract_manager_club_by_id(read_u32(G_KEY_CLUB_SOCIEDADE_ESPORTIVA_PALMEIRAS)), 2) {
                    crate::comparacao::ramo("cqj_1 brasil fixo");
                    n = 1; // cqj1: o Palmeiras
                }
                if enter_libertadores(contract_manager_club_by_id(read_u32(G_KEY_CLUB_CRUZEIRO_ESPORTE_CLUBE)), 2) {
                    n = n.wrapping_add(1);
                }
            }
            for lugar in [0u32, 1] {
                if enter_libertadores(campeao(linha(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION), lugar), 2) {
                    n = n.wrapping_add(1);
                }
            }
        }
        while (n as i8) < *((e + 8) as *const i8) {
            let c = enesimo(this, clubes, conta, read_u32(e), 1, 1, 0);
            if c != 0 && read_u32(c + 0x1db) == u32::MAX {
                crate::comparacao::ramo("cqj_1 cota da nacao");
                enter_libertadores(c, 2); // cqj1: a vaga da cota
            }
            n = n.wrapping_add(1);
        }
    }
}


/// Lado a lado: os clubes do jogo e a marca da caixa.
pub(crate) unsafe extern "thiscall" fn comparar_assign_libertadores_places(this: usize, clubes: usize, conta: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(IDX_ASSIGN_LIBERTADORES_PLACES);
    let n = (read_u32(G_CLUB_COUNT) as i32).max(0) as usize;
    // o histórico das competições também: a 0x4afd80 o reordena (linhas empatadas: a ordem de antes muda o clube achado)
    let reg = [(read_u32(G_CLUB_TABLE) as usize, n * club::SIZE), (0xb67a34, 4), (read_u32(G_CLUB_COMP_HISTORY_TABLE) as usize, (read_u32(G_CLUB_COMP_HISTORY_COUNT) as i32).max(0) as usize * 0x1a)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clubes, conta), || assign_libertadores_places(this, clubes, conta));
    PLACAR.registrar(igual, || {
        let base = read_u32(G_CLUB_TABLE) as usize;
        let l: Vec<String> = memory_diff_addresses(&mo, &mr).iter().map(|&a| {
            if a < base || a - base >= mo.0[0].1.len() {
                return format!("{:x}", a);
            }
            let (q, k) = ((a - base) / 0x245, (a - base) % 0x245);
            let (x, y) = (mo.0[0].1[a - base], mr.0[0].1[a - base]);
            format!("clube {} +{:x}: {:02x}/{:02x}", q, k, x, y)
        }).collect();
        format!("assign_libertadores_places (ano {}): {}", *(G_CURRENT_DATE_YEAR as *const u16), l.join(", "))
    });
}

/// in the South American club table sorted by nation (and, within it, by division, position and reputation), the nth
/// club of the nation, skipping on request those already qualified and those with a human manager
/// Original: `ConmebolSeeding::find_nth_unqualified_club_of_nation` (0x4c6730, conmebol_seeding.cpp, thiscall, ret 0x18 (this não lido)).
///
/// [pt] { 0x4c6730 } (thiscall, ret 0x18, conmebol_seeding.cpp, `ConmebolSeeding::find_nth_unqualified_club_of_nation`; o
/// this não é lido; o mesmo código da 0x9050d0 da UEFA): na tabela dos clubes sul-americanos (registros de 12 bytes
/// ordenados pela nação: +0 o clube, +4 o id da nação), a busca binária (bsearch 0x946464 com 0x905ce0; a chave só com a
/// nação e +8 = 0xff) acha um clube da nação; recua até o primeiro da nação; anda até o `n`-ésimo (char, 1 = o primeiro)
/// que conta, pulando, se pedido, os que já têm vaga (+0x1db ≠ −1) e os de técnico humano (0x600a50(clube, 0, 1, 0, 0)).
/// O último registro da tabela nunca é examinado pela volta (se ela chega nele com a conta zerada, ele é o escolhido sem
/// os testes). Devolve o clube se for da nação, senão 0.
pub(crate) unsafe extern "thiscall" fn find_nth_unqualified_club_of_nation(_this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
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
                crate::comparacao::ramo("cqj_1 ja com vaga");
                p += 12; // cqj1: pula o clube que já tem vaga
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

/// Comparer for `find_nth_unqualified_club_of_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] A devolução das duas (a rotina só lê).
pub(crate) unsafe extern "thiscall" fn comparar_find_nth_unqualified_club_of_nation(this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize>(IDX_FIND_NTH_UNQUALIFIED_CLUB_OF_NATION);
    let r = o(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    let c = find_nth_unqualified_club_of_nation(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    PLACAR.registrar(r == c, || format!("find_nth_unqualified_club_of_nation(nação {:x}, n {}, {} {}): original {:x} rust {:x}", nacao, n as u8, sem_vaga, sem_humano, r, c));
    r
}

/// Os 20 clubes convidados da Copa Mercosul (as chaves fixas: River Plate, Boca Juniors, Independiente, San Lorenzo,
/// Vélez Sarsfield, Cruzeiro, São Paulo, Palmeiras, Corinthians, Flamengo, Rosario Central, Vasco, Colo-Colo,
/// Universidad de Chile, Universidad Católica, Peñarol, Nacional, Olimpia, Cerro Porteño e Grêmio), na ordem da original.
pub(crate) const CQJ1_MERCOSUL: [usize; 20] = [
    0x9d00ac, 0x9d00d0, 0x9d00b4, 0x9d00d8, 0x9d00b0, 0x9d026c, 0x9d0270, 0x9d0274, 0x9d0278, 0x9d0358,
    0x9d00e0, 0x9d0354, 0x9d0280, 0x9d0284, 0x9d0288, 0x9d0298, 0x9d028c, 0x9d0290, 0x9d0294, 0x9d027c,
];

/// the Copa Mercosur rule: the 20 fixed guests (River Plate, Boca Juniors, Independiente, San Lorenzo, Vélez Sarsfield,
/// Rosario Central; Cruzeiro, São Paulo, Palmeiras, Corinthians, Flamengo, Vasco, Grêmio; Colo-Colo, Universidad de
/// Chile, Universidad Católica; Peñarol, Nacional; Olimpia, Cerro Porteño)
/// Original: `ConmebolSeeding::is_mercosur_invited_club` (0x4c6890, conmebol_seeding.cpp, thiscall, ret 4 (this não lido)).
///
/// [pt] { 0x4c6890 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::is_mercosur_invited_club`; o this não é
/// lido): 1 se o id do clube é um dos 20 convidados da Copa Mercosul (`CQJ1_MERCOSUL`), senão 0 (sem clube, devolve o 0
/// do próprio ponteiro).
pub(crate) unsafe extern "thiscall" fn is_mercosur_invited_club(_this: usize, clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let id = read_u32(clube);
    for &chave in CQJ1_MERCOSUL.iter() {
        if id == read_u32(chave) {
            return 1; // cqj1: convidado
        }
    }
    0
}

/// Comparer for `is_mercosur_invited_club` (comparison mode: see the header of motor.rs).
///
/// [pt] A devolução das duas (a rotina só lê).
pub(crate) unsafe extern "thiscall" fn comparar_is_mercosur_invited_club(this: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_IS_MERCOSUR_INVITED_CLUB);
    let r = o(this, clube);
    let c = is_mercosur_invited_club(this, clube);
    PLACAR.registrar(r == c, || format!("is_mercosur_invited_club({:x}): original {} rust {}", clube, r, c));
    r
}


/// the CONMEBOL club table: South American and Mexican clubs (national teams excluded), each without a continental
/// place, sorted by nation and, within it, by division, position and reputation [inference: the division +0x69 and club
/// +0x5f fields]
/// Original: `collect_conmebol_clubs` (0x4c6280, conmebol_seeding.cpp, stdcall, ret 4).
///
/// [pt] { 0x4c6280 } (stdcall, ret 4, conmebol_seeding.cpp, `collect_conmebol_clubs`): a tabela dos clubes da Conmebol: aloca
/// 12 bytes por clube do jogo (0x944e46; sem memória, a caixa da linha 0xbb e devolve 0) e grava o ponteiro em `*saida`;
/// para cada clube com nação, que não é seleção (0x53b900), do México ([0x9cf3d0]) ou de nação da América do Sul (o
/// continente +0x71 com o id de [0x9cfa1c]): tira a vaga continental (+0x1db = −1) e grava o registro (+0 o clube, +4 o
/// id da nação, +8 a reputação +0x80 / 500, +9 o +0x69 da divisão +0x5b, +0xb o +0x5f; sem divisão −1 e 0x7f; com a
/// divisão de uma competição sem o +0x11c, a divisão e o +0x5f do clube zeram). Ordena pela nação (qsort 0x945333 com
/// 0x905ce0). Devolve o número de clubes (só os 16 bits baixos contam).
pub(crate) unsafe extern "stdcall" fn collect_conmebol_clubs(saida: *mut usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let t = alocar((read_u32(G_CLUB_COUNT) as i32).wrapping_mul(12) as u32);
    *saida = t;
    if t == 0 {
        show_error_at(0x9a7ed8, 0xbb);
        return 0;
    }
    let mut k = 0u32;
    let mut c = read_u32(G_CLUB_TABLE) as usize;
    let mut i = 0u32;
    if read_u32(G_CLUB_COUNT) as i32 > 0 {
        loop {
            let nacao = read_u32(c + 0x53) as usize;
            if nacao != 0 && selecao(c) == 0 {
                let mexico = read_u32(nacao) == read_u32(KEY_NATION_MEXICO); // cqj2: o México entra
                let cont = if mexico { 0 } else { read_u32(nacao + nation::CONTINENT) as usize };
                if mexico || (cont != 0 && read_u32(cont) == read_u32(KEY_CONTINENT_SOUTH_AMERICA)) {
                    if mexico {
                        crate::comparacao::ramo("cqj_2 clube mexicano");
                    }
                    crate::comparacao::ramo("cqj_2 tira a vaga");
                    *((c + 0x1db) as *mut u32) = u32::MAX; // cqj2: sem vaga continental
                    let r = t.wrapping_add((k as u16 as i16 as i32 * 12) as usize);
                    core::ptr::write_unaligned(r as *mut u32, c as u32);
                    core::ptr::write_unaligned((r + 4) as *mut u32, read_u32(nacao));
                    *((r + 8) as *mut u8) = (core::ptr::read_unaligned((c + 0x80) as *const i16) as i32 / 500) as u8;
                    let divisao = read_u32(c + 0x5b) as usize;
                    if divisao != 0 {
                        crate::comparacao::ramo("cqj_2 nivel da divisao");
                        core::ptr::write_unaligned((r + 9) as *mut u16, core::ptr::read_unaligned((divisao + 0x69) as *const u16)); // cqj2: o nível da divisão
                        *((r + 0xb) as *mut u8) = *((c + 0x5f) as *const u8);
                        let comp = read_u32(read_u32(c + 0x5b) as usize + 0x5d) as usize;
                        if comp == 0 || *((comp + 0x11c) as *const u8) == 0 {
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
    // ponytail: a original devolve o ax com os bytes altos do eax que sobraram do qsort (quem chama lê o ax); a cópia, zero
    k as u16 as u32
}


/// Regions compared: the whole club table, [0xb67a34] and the club competition history table.
///
/// [pt] Os clubes do jogo, a marca da caixa e o histórico das competições (a 0x4afd80 o reordena: ver a cqj_1).
pub(crate) unsafe fn club_seeding_regions() -> Vec<(usize, usize)> {
    let n = (read_u32(G_CLUB_COUNT) as i32).max(0) as usize;
    vec![(read_u32(G_CLUB_TABLE) as usize, n * club::SIZE), (0xb67a34, 4), (read_u32(G_CLUB_COMP_HISTORY_TABLE) as usize, (read_u32(G_CLUB_COMP_HISTORY_COUNT) as i32).max(0) as usize * 0x1a)]
}

/// Comparer for `collect_conmebol_clubs` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: a original com a saída de quem chama, a cópia com uma saída própria (liberada depois); os clubes, a
/// devolução e os registros das duas tabelas.
pub(crate) unsafe extern "stdcall" fn comparar_collect_conmebol_clubs(saida: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(*mut usize) -> u32>(IDX_COLLECT_CONMEBOL_CLUBS);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut t2 = 0usize;
    let reg = club_seeding_regions();
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(saida), || collect_conmebol_clubs(&mut t2));
    let (n, m) = (a as u16 as i16, b as u16 as i16);
    let t1 = *saida;
    let tabela = n == m && (n <= 0 || t1 == 0 || t2 == 0 || core::slice::from_raw_parts(t1 as *const u8, n as usize * 12) == core::slice::from_raw_parts(t2 as *const u8, n as usize * 12));
    if t2 != 0 {
        soltar(t2);
    }
    PLACAR.registrar(igual && tabela, || format!("collect_conmebol_clubs: {} e {} clubes, tabela {}, diferenças em {:x?}", n, m, tabela, memory_diff_addresses(&mo, &mr)));
    a
}

/// the Libertadores calendar: every new game year it builds the table and gives the places
/// Original: `ConmebolSeeding::update_for_new_season` (0x4c61b0, conmebol_seeding.cpp, thiscall, ret 4).
///
/// [pt] { 0x4c61b0 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::update_for_new_season`): chamada pela
/// Libertadores e pela Mercosul; sem `pular` e com o ano do jogo ([0xae2c92]) outro que o +0x6f: grava o ano, monta a
/// tabela dos clubes (0x4c6280; vazia, a caixa da linha 0x9b e volta sem soltar), dá as vagas da Libertadores (0x4c6430)
/// e solta a tabela.
pub(crate) unsafe extern "thiscall" fn update_for_new_season(this: usize, pular: u32) {
    if pular != 0 {
        return;
    }
    let ano = core::ptr::read_unaligned(G_CURRENT_DATE_YEAR as *const u16);
    if ano == core::ptr::read_unaligned((this + 0x6f) as *const u16) {
        return;
    }
    crate::comparacao::ramo("cqj_2 virada do ano");
    core::ptr::write_unaligned((this + 0x6f) as *mut u16, ano);
    let juntar: unsafe extern "stdcall" fn(*mut usize) -> u32 = core::mem::transmute(FN_COLLECT_CONMEBOL_CLUBS);
    let vagas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(FN_CONMEBOLSEEDING_ASSIGN_LIBERTADORES_PLACES);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut t = 0usize;
    let n = juntar(&mut t);
    if n as u16 == 0 {
        show_error_at(0x9a7ed8, 0x9b);
        return;
    }
    vagas(this, t, n);
    soltar(t);
}

/// Comparer for `update_for_new_season` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado: o ano gravado (+0x6f), os clubes e a marca da caixa (cada uma monta e solta a própria tabela).
pub(crate) unsafe extern "thiscall" fn comparar_update_for_new_season(this: usize, pular: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(IDX_UPDATE_FOR_NEW_SEASON);
    let mut reg = club_seeding_regions();
    reg.push((this + 0x6f, 2));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pular), || update_for_new_season(this, pular));
    PLACAR.registrar(igual, || format!("update_for_new_season({}): diferenças em {:x?}", pular, memory_diff_addresses(&mo, &mr)));
}

/// stores the Copa dos Campeões winner: from 2003 on it gets a Libertadores place
/// Original: `ConmebolSeeding::set_bra_champ_cup_winner` (0x4c6960, conmebol_seeding.cpp, thiscall, ret 4).
///
/// [pt] { 0x4c6960 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::set_bra_champ_cup_winner`): guarda o id do
/// campeão da Copa dos Campeões em +0x71 (a vaga dele na Libertadores, de 2003 em diante).
pub(crate) unsafe extern "thiscall" fn set_bra_champ_cup_winner(this: usize, clube: usize) {
    core::ptr::write_unaligned((this + 0x71) as *mut u32, read_u32(clube));
}

/// Lado a lado: o +0x71.
pub(crate) unsafe extern "thiscall" fn comparar_set_bra_champ_cup_winner(this: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_SET_BRA_CHAMP_CUP_WINNER);
    let reg = [(this + 0x71, 4)];
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || set_bra_champ_cup_winner(this, clube));
    PLACAR.registrar(igual, || format!("set_bra_champ_cup_winner({:x})", clube));
}

/// writes CM3_TEMP\conmebol_seeding.tmp: the country count (+0x6e), 0xa bytes per country from +0, +0x6f (u16) and
/// +0x71 (u32); on error the "Error" box without closing the file
/// Original: `ConmebolSeeding::save_to_temp_file` (0x4c6b70, conmebol_seeding.cpp, thiscall).
///
/// [pt] { 0x4c6b70 } (thiscall, ret 0, conmebol_seeding.cpp): `ConmebolSeeding::save_to_temp_file`: grava
/// `CM3_TEMP\conmebol_seeding.tmp`: a quantidade de países (+0x6e, i8), os registros de 0xa bytes de +0, +0x6f (u16) e
/// +0x71 (u32). Erro: a caixa "Error" (conmebol_seeding.cpp 0x2ee, 0x2f6, 0x2ff, 0x307, 0x30f), sem fechar o arquivo.
pub(crate) unsafe extern "thiscall" fn conmebol_seeding_save_to_temp_file(this: usize) {
    const FONTE: usize = 0x9a7ed8;
    crate::comparacao::ramo("cpx_6 gravar sementes da conmebol");
    let f = open_temp_file_for_write(0x9a7f10);
    if f == 0 {
        show_error_at(FONTE, 0x2ee);
        return;
    }
    if crt_fwrite(this + 0x6e, 1, 1, f) != 1 {
        show_error_at(FONTE, 0x2f6);
        return;
    }
    if crt_fwrite(this, 0xa, signed_byte_count(this + 0x6e), f) != signed_byte_count(this + 0x6e) { // cpx6: os registros dos países
        show_error_at(FONTE, 0x2ff);
        return;
    }
    if crt_fwrite(this + 0x6f, 2, 1, f) != 1 {
        show_error_at(FONTE, 0x307);
        return;
    }
    if crt_fwrite(this + 0x71, 4, 1, f) != 1 {
        show_error_at(FONTE, 0x30f);
        return;
    }
    uefa_seeding_crt_fclose(f);
}

/// Comparer for `conmebol_seeding_save_to_temp_file` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4c6b70: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_seeding_save_to_temp_file(this: usize) {
    // ponytail: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_SEEDING_SAVE_TO_TEMP_FILE)(this)
}
