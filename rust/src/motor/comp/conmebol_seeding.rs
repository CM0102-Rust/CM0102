// ---------- 273a leva: a classificação da Conmebol: as vagas da Libertadores, o n-ésimo clube da nação e os convidados da Mercosul (conmebol_seeding.cpp) ----------


/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cqj1_clube(id: u32) -> usize {
    (read_u32(0xae23b4) as usize).wrapping_add((id as usize).wrapping_mul(0x245))
}

/// O clube `c` (não nulo e ainda sem vaga, +0x1db = −1) entra na Libertadores na fase `fase`. Devolve se entrou.
unsafe fn cqj1_entra(c: usize, fase: u8) -> bool {
    if c == 0 || read_u32(c + 0x1db) != u32::MAX {
        return false;
    }
    *((c + 0x1db) as *mut u32) = read_u32(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES);
    *((c + 0x1df) as *mut u8) = fase;
    true
}

/// { 0x4c6430 } (thiscall, ret 8, conmebol_seeding.cpp, `ConmebolSeeding::assign_libertadores_places`): as vagas da
/// Libertadores ([0x9cf63c]). O campeão da última edição (0x4afd80 com a linha da competição e o lugar 0) entra na fase 1
/// (sem ele ou já com vaga, a caixa da linha 0x114). Depois, as 11 nações da tabela (registros de 10 bytes a partir do
/// objeto: +0 id da nação, +8 a cota), cada uma até a cota, na fase 2: o Brasil, até 2002, o Palmeiras e o Cruzeiro (as
/// chaves 0x9d0274 e 0x9d026c); de 2003 em diante o campeão da Copa do Brasil (ou o vice, se o campeão já tem vaga) e o
/// campeão da Copa dos Campeões (+0x71); e sempre o campeão e o vice do Brasileirão (0x9cf9ec); cada nação completa a
/// cota com os primeiros da tabela ainda sem vaga (0x4c6730(.., 1, 1, 0)), uma tentativa por vaga.
unsafe extern "thiscall" fn conmebol_vagas_libertadores(this: usize, clubes: usize, conta: u32) {
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CONMEBOLSEEDING_FIND_NTH_UNQUALIFIED_CLUB_OF_NATION);
    let linha = |chave: usize| (read_u32(0xae23d0) as usize).wrapping_add(read_u32(chave).wrapping_mul(0x6b) as usize);
    let h = campeao(linha(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES), 0);
    if h != 0 && read_u32(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = read_u32(KEY_CLUB_COMP_SOUTH_AMERICAN_COPA_LIBERTADORES);
    } else {
        aviso_em(0x9a7ed8, 0x114);
    }
    for k in 0..0xbusize {
        let e = this + k * 10;
        let mut n = 0u8;
        if read_u32(e) == read_u32(KEY_NATION_BRAZIL) {
            if *(0xae2c92 as *const u16) > 0x7d2 {
                let mut c = campeao(linha(KEY_CLUB_COMP_BRAZILIAN_CUP), 0);
                if c == 0 || read_u32(c + 0x1db) != u32::MAX {
                    c = campeao(linha(KEY_CLUB_COMP_BRAZILIAN_CUP), 1);
                }
                if cqj1_entra(c, 2) {
                    n = 1;
                }
                let id = read_u32(this + 0x71);
                if id != u32::MAX && cqj1_entra(cqj1_clube(id), 2) {
                    n = n.wrapping_add(1);
                }
            } else {
                if cqj1_entra(cqj1_clube(read_u32(0x9d0274)), 2) {
                    crate::comparacao::ramo("cqj_1 brasil fixo");
                    n = 1; // cqj1: o Palmeiras
                }
                if cqj1_entra(cqj1_clube(read_u32(0x9d026c)), 2) {
                    n = n.wrapping_add(1);
                }
            }
            for lugar in [0u32, 1] {
                if cqj1_entra(campeao(linha(KEY_CLUB_COMP_BRAZILIAN_NATIONAL_FIRST_DIVISION), lugar), 2) {
                    n = n.wrapping_add(1);
                }
            }
        }
        while (n as i8) < *((e + 8) as *const i8) {
            let c = enesimo(this, clubes, conta, read_u32(e), 1, 1, 0);
            if c != 0 && read_u32(c + 0x1db) == u32::MAX {
                crate::comparacao::ramo("cqj_1 cota da nacao");
                cqj1_entra(c, 2); // cqj1: a vaga da cota
            }
            n = n.wrapping_add(1);
        }
    }
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn cqj1_dif(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Lado a lado: os clubes do jogo e a marca da caixa.
unsafe extern "thiscall" fn comparar_conmebol_vagas_libertadores(this: usize, clubes: usize, conta: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1667);
    let n = (read_u32(0xae235c) as i32).max(0) as usize;
    // o histórico das competições também: a 0x4afd80 o reordena (linhas empatadas: a ordem de antes muda o clube achado)
    let reg = [(read_u32(0xae23b4) as usize, n * 0x245), (0xb67a34, 4), (read_u32(0xae23d4) as usize, (read_u32(0xae237c) as i32).max(0) as usize * 0x1a)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clubes, conta), || conmebol_vagas_libertadores(this, clubes, conta));
    PLACAR.registrar(igual, || {
        let base = read_u32(0xae23b4) as usize;
        let l: Vec<String> = cqj1_dif(&mo, &mr).iter().map(|&a| {
            if a < base || a - base >= mo.0[0].1.len() {
                return format!("{:x}", a);
            }
            let (q, k) = ((a - base) / 0x245, (a - base) % 0x245);
            let (x, y) = (mo.0[0].1[a - base], mr.0[0].1[a - base]);
            format!("clube {} +{:x}: {:02x}/{:02x}", q, k, x, y)
        }).collect();
        format!("conmebol_vagas_libertadores (ano {}): {}", *(0xae2c92 as *const u16), l.join(", "))
    });
}

/// { 0x4c6730 } (thiscall, ret 0x18, conmebol_seeding.cpp, `ConmebolSeeding::find_nth_unqualified_club_of_nation`; o
/// this não é lido; o mesmo código da 0x9050d0 da UEFA): na tabela dos clubes sul-americanos (registros de 12 bytes
/// ordenados pela nação: +0 o clube, +4 o id da nação), a busca binária (bsearch 0x946464 com 0x905ce0; a chave só com a
/// nação e +8 = 0xff) acha um clube da nação; recua até o primeiro da nação; anda até o `n`-ésimo (char, 1 = o primeiro)
/// que conta, pulando, se pedido, os que já têm vaga (+0x1db ≠ −1) e os de técnico humano (0x600a50(clube, 0, 1, 0, 0)).
/// O último registro da tabela nunca é examinado pela volta (se ela chega nele com a conta zerada, ele é o escolhido sem
/// os testes). Devolve o clube se for da nação, senão 0.
unsafe extern "thiscall" fn conmebol_enesimo_clube_da_nacao(_this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
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

/// A devolução das duas (a rotina só lê).
unsafe extern "thiscall" fn comparar_conmebol_enesimo_clube_da_nacao(this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize>(1668);
    let r = o(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    let c = conmebol_enesimo_clube_da_nacao(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    PLACAR.registrar(r == c, || format!("conmebol_enesimo_clube_da_nacao(nação {:x}, n {}, {} {}): original {:x} rust {:x}", nacao, n as u8, sem_vaga, sem_humano, r, c));
    r
}

/// Os 20 clubes convidados da Copa Mercosul (as chaves fixas: River Plate, Boca Juniors, Independiente, San Lorenzo,
/// Vélez Sarsfield, Cruzeiro, São Paulo, Palmeiras, Corinthians, Flamengo, Rosario Central, Vasco, Colo-Colo,
/// Universidad de Chile, Universidad Católica, Peñarol, Nacional, Olimpia, Cerro Porteño e Grêmio), na ordem da original.
const CQJ1_MERCOSUL: [usize; 20] = [
    0x9d00ac, 0x9d00d0, 0x9d00b4, 0x9d00d8, 0x9d00b0, 0x9d026c, 0x9d0270, 0x9d0274, 0x9d0278, 0x9d0358,
    0x9d00e0, 0x9d0354, 0x9d0280, 0x9d0284, 0x9d0288, 0x9d0298, 0x9d028c, 0x9d0290, 0x9d0294, 0x9d027c,
];

/// { 0x4c6890 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::is_mercosur_invited_club`; o this não é
/// lido): 1 se o id do clube é um dos 20 convidados da Copa Mercosul (`CQJ1_MERCOSUL`), senão 0 (sem clube, devolve o 0
/// do próprio ponteiro).
unsafe extern "thiscall" fn conmebol_convidado_da_mercosul(_this: usize, clube: usize) -> u32 {
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

/// A devolução das duas (a rotina só lê).
unsafe extern "thiscall" fn comparar_conmebol_convidado_da_mercosul(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1669);
    let r = o(this, clube);
    let c = conmebol_convidado_da_mercosul(this, clube);
    PLACAR.registrar(r == c, || format!("conmebol_convidado_da_mercosul({:x}): original {} rust {}", clube, r, c));
    r
}

// ---------- 273a leva: a classificação da Conmebol: a tabela dos clubes sul-americanos, a virada do ano e o campeão da Copa dos Campeões (conmebol_seeding.cpp) ----------


/// { 0x4c6280 } (stdcall, ret 4, conmebol_seeding.cpp, `collect_conmebol_clubs`): a tabela dos clubes da Conmebol: aloca
/// 12 bytes por clube do jogo (0x944e46; sem memória, a caixa da linha 0xbb e devolve 0) e grava o ponteiro em `*saida`;
/// para cada clube com nação, que não é seleção (0x53b900), do México ([0x9cf3d0]) ou de nação da América do Sul (o
/// continente +0x71 com o id de [0x9cfa1c]): tira a vaga continental (+0x1db = −1) e grava o registro (+0 o clube, +4 o
/// id da nação, +8 a reputação +0x80 / 500, +9 o +0x69 da divisão +0x5b, +0xb o +0x5f; sem divisão −1 e 0x7f; com a
/// divisão de uma competição sem o +0x11c, a divisão e o +0x5f do clube zeram). Ordena pela nação (qsort 0x945333 com
/// 0x905ce0). Devolve o número de clubes (só os 16 bits baixos contam).
unsafe extern "stdcall" fn conmebol_juntar_clubes(saida: *mut usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let t = alocar((read_u32(0xae235c) as i32).wrapping_mul(12) as u32);
    *saida = t;
    if t == 0 {
        aviso_em(0x9a7ed8, 0xbb);
        return 0;
    }
    let mut k = 0u32;
    let mut c = read_u32(0xae23b4) as usize;
    let mut i = 0u32;
    if read_u32(0xae235c) as i32 > 0 {
        loop {
            let nacao = read_u32(c + 0x53) as usize;
            if nacao != 0 && selecao(c) == 0 {
                let mexico = read_u32(nacao) == read_u32(KEY_NATION_MEXICO); // cqj2: o México entra
                let cont = if mexico { 0 } else { read_u32(nacao + 0x71) as usize };
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
            if (i as u16 as i16 as i32) >= read_u32(0xae235c) as i32 {
                break;
            }
        }
    }
    ordenar(t, k as u16 as i16 as i32, 0xc, 0x905ce0);
    // ponytail: a original devolve o ax com os bytes altos do eax que sobraram do qsort (quem chama lê o ax); a cópia, zero
    k as u16 as u32
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn cqj2_dif(mo: &crate::comparacao::Memoria, mr: &crate::comparacao::Memoria) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Os clubes do jogo, a marca da caixa e o histórico das competições (a 0x4afd80 o reordena: ver a cqj_1).
unsafe fn cqj2_regioes_dos_clubes() -> Vec<(usize, usize)> {
    let n = (read_u32(0xae235c) as i32).max(0) as usize;
    vec![(read_u32(0xae23b4) as usize, n * 0x245), (0xb67a34, 4), (read_u32(0xae23d4) as usize, (read_u32(0xae237c) as i32).max(0) as usize * 0x1a)]
}

/// Lado a lado: a original com a saída de quem chama, a cópia com uma saída própria (liberada depois); os clubes, a
/// devolução e os registros das duas tabelas.
unsafe extern "stdcall" fn comparar_conmebol_juntar_clubes(saida: *mut usize) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(*mut usize) -> u32>(1670);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut t2 = 0usize;
    let reg = cqj2_regioes_dos_clubes();
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(saida), || conmebol_juntar_clubes(&mut t2));
    let (n, m) = (a as u16 as i16, b as u16 as i16);
    let t1 = *saida;
    let tabela = n == m && (n <= 0 || t1 == 0 || t2 == 0 || core::slice::from_raw_parts(t1 as *const u8, n as usize * 12) == core::slice::from_raw_parts(t2 as *const u8, n as usize * 12));
    if t2 != 0 {
        soltar(t2);
    }
    PLACAR.registrar(igual && tabela, || format!("conmebol_juntar_clubes: {} e {} clubes, tabela {}, diferenças em {:x?}", n, m, tabela, cqj2_dif(&mo, &mr)));
    a
}

/// { 0x4c61b0 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::update_for_new_season`): chamada pela
/// Libertadores e pela Mercosul; sem `pular` e com o ano do jogo ([0xae2c92]) outro que o +0x6f: grava o ano, monta a
/// tabela dos clubes (0x4c6280; vazia, a caixa da linha 0x9b e volta sem soltar), dá as vagas da Libertadores (0x4c6430)
/// e solta a tabela.
unsafe extern "thiscall" fn conmebol_virada_do_ano(this: usize, pular: u32) {
    if pular != 0 {
        return;
    }
    let ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
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
        aviso_em(0x9a7ed8, 0x9b);
        return;
    }
    vagas(this, t, n);
    soltar(t);
}

/// Lado a lado: o ano gravado (+0x6f), os clubes e a marca da caixa (cada uma monta e solta a própria tabela).
unsafe extern "thiscall" fn comparar_conmebol_virada_do_ano(this: usize, pular: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32)>(1671);
    let mut reg = cqj2_regioes_dos_clubes();
    reg.push((this + 0x6f, 2));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pular), || conmebol_virada_do_ano(this, pular));
    PLACAR.registrar(igual, || format!("conmebol_virada_do_ano({}): diferenças em {:x?}", pular, cqj2_dif(&mo, &mr)));
}

/// { 0x4c6960 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::set_bra_champ_cup_winner`): guarda o id do
/// campeão da Copa dos Campeões em +0x71 (a vaga dele na Libertadores, de 2003 em diante).
unsafe extern "thiscall" fn conmebol_campeao_dos_campeoes(this: usize, clube: usize) {
    core::ptr::write_unaligned((this + 0x71) as *mut u32, read_u32(clube));
}

/// Lado a lado: o +0x71.
unsafe extern "thiscall" fn comparar_conmebol_campeao_dos_campeoes(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1672);
    let reg = [(this + 0x71, 4)];
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || conmebol_campeao_dos_campeoes(this, clube));
    PLACAR.registrar(igual, || format!("conmebol_campeao_dos_campeoes({:x})", clube));
}

