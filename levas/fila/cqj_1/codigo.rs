// ---------- %%LEVA%%a leva: a classificação da Conmebol: as vagas da Libertadores, o n-ésimo clube da nação e os convidados da Mercosul (conmebol_seeding.cpp) ----------

unsafe fn cqj1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cqj1_clube(id: u32) -> usize {
    (cqj1_d(0xae23b4) as usize).wrapping_add((id as usize).wrapping_mul(0x245))
}

/// O clube `c` (não nulo e ainda sem vaga, +0x1db = −1) entra na Libertadores na fase `fase`. Devolve se entrou.
unsafe fn enter_libertadores(c: usize, fase: u8) -> bool {
    if c == 0 || cqj1_d(c + 0x1db) != u32::MAX {
        return false;
    }
    *((c + 0x1db) as *mut u32) = cqj1_d(0x9cf63c);
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
unsafe extern "thiscall" fn assign_libertadores_places(this: usize, clubes: usize, conta: u32) {
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x4c6730usize);
    let linha = |chave: usize| (cqj1_d(0xae23d0) as usize).wrapping_add(cqj1_d(chave).wrapping_mul(0x6b) as usize);
    let h = campeao(linha(0x9cf63c), 0);
    if h != 0 && cqj1_d(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = cqj1_d(0x9cf63c);
    } else {
        show_error_at(0x9a7ed8, 0x114);
    }
    for k in 0..0xbusize {
        let e = this + k * 10;
        let mut n = 0u8;
        if cqj1_d(e) == cqj1_d(0x9cf260) {
            if *(0xae2c92 as *const u16) > 0x7d2 {
                let mut c = campeao(linha(0x9cf9e8), 0);
                if c == 0 || cqj1_d(c + 0x1db) != u32::MAX {
                    c = campeao(linha(0x9cf9e8), 1);
                }
                if enter_libertadores(c, 2) {
                    n = 1;
                }
                let id = cqj1_d(this + 0x71);
                if id != u32::MAX && enter_libertadores(cqj1_clube(id), 2) {
                    n = n.wrapping_add(1);
                }
            } else {
                if enter_libertadores(cqj1_clube(cqj1_d(0x9d0274)), 2) {
                    crate::comparison::branch("cqj_1 brasil fixo");
                    n = 1; // cqj1: o Palmeiras
                }
                if enter_libertadores(cqj1_clube(cqj1_d(0x9d026c)), 2) {
                    n = n.wrapping_add(1);
                }
            }
            for lugar in [0u32, 1] {
                if enter_libertadores(campeao(linha(0x9cf9ec), lugar), 2) {
                    n = n.wrapping_add(1);
                }
            }
        }
        while (n as i8) < *((e + 8) as *const i8) {
            let c = enesimo(this, clubes, conta, cqj1_d(e), 1, 1, 0);
            if c != 0 && cqj1_d(c + 0x1db) == u32::MAX {
                crate::comparison::branch("cqj_1 cota da nacao");
                enter_libertadores(c, 2); // cqj1: a vaga da cota
            }
            n = n.wrapping_add(1);
        }
    }
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn cqj1_dif(mo: &crate::comparison::MemorySnapshot, mr: &crate::comparison::MemorySnapshot) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Lado a lado: os clubes do jogo e a marca da caixa.
unsafe extern "thiscall" fn compare_assign_libertadores_places(this: usize, clubes: usize, conta: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let n = (cqj1_d(0xae235c) as i32).max(0) as usize;
    // o histórico das competições também: a 0x4afd80 o reordena (linhas empatadas: a ordem de antes muda o clube achado)
    let reg = [(cqj1_d(0xae23b4) as usize, n * 0x245), (0xb67a34, 4), (cqj1_d(0xae23d4) as usize, (cqj1_d(0xae237c) as i32).max(0) as usize * 0x1a)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clubes, conta), || assign_libertadores_places(this, clubes, conta));
    SCOREBOARD.record(igual, || {
        let base = cqj1_d(0xae23b4) as usize;
        let l: Vec<String> = cqj1_dif(&mo, &mr).iter().map(|&a| {
            if a < base || a - base >= mo.0[0].1.len() {
                return format!("{:x}", a);
            }
            let (q, k) = ((a - base) / 0x245, (a - base) % 0x245);
            let (x, y) = (mo.0[0].1[a - base], mr.0[0].1[a - base]);
            format!("clube {} +{:x}: {:02x}/{:02x}", q, k, x, y)
        }).collect();
        format!("assign_libertadores_places (ano {}): {}", *(0xae2c92 as *const u16), l.join(", "))
    });
}

/// { 0x4c6730 } (thiscall, ret 0x18, conmebol_seeding.cpp, `ConmebolSeeding::find_nth_unqualified_club_of_nation`; o
/// this não é lido; o mesmo código da 0x9050d0 da UEFA): na tabela dos clubes sul-americanos (registros de 12 bytes
/// ordenados pela nação: +0 o clube, +4 o id da nação), a busca binária (bsearch 0x946464 com 0x905ce0; a chave só com a
/// nação e +8 = 0xff) acha um clube da nação; recua até o primeiro da nação; anda até o `n`-ésimo (char, 1 = o primeiro)
/// que conta, pulando, se pedido, os que já têm vaga (+0x1db ≠ −1) e os de técnico humano (0x600a50(clube, 0, 1, 0, 0)).
/// O último registro da tabela nunca é examinado pela volta (se ela chega nele com a conta zerada, ele é o escolhido sem
/// os testes). Devolve o clube se for da nação, senão 0.
unsafe extern "thiscall" fn find_nth_unqualified_club_of_nation(_this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
    let busca: unsafe extern "C" fn(*const u8, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let k = conta as u16 as i16 as i32;
    let ultimo = clubes.wrapping_add((k * 12 - 12) as usize);
    let mut chave = [0u8; 12];
    chave[4..8].copy_from_slice(&nacao.to_le_bytes());
    chave[8] = 0xff;
    let mut p = busca(chave.as_ptr(), clubes, k, 0xc, 0x905ce0);
    if p == 0 {
        return 0;
    }
    while p > clubes && cqj1_d(p - 8) == nacao {
        p -= 12;
    }
    let mut falta = (n as u8).wrapping_sub(1);
    let mut achou = false;
    if p < ultimo {
        loop {
            let c = cqj1_d(p) as usize;
            if sem_vaga != 0 && c != 0 && cqj1_d(c + 0x1db) != u32::MAX {
                crate::comparison::branch("cqj_1 ja com vaga");
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
    if cqj1_d(p + 4) == nacao {
        return cqj1_d(p) as usize;
    }
    0
}

/// A devolução das duas (a rotina só lê).
unsafe extern "thiscall" fn compare_find_nth_unqualified_club_of_nation(this: usize, clubes: usize, conta: u32, nacao: u32, n: u32, sem_vaga: u32, sem_humano: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize>(%%I1%%);
    let r = o(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    let c = find_nth_unqualified_club_of_nation(this, clubes, conta, nacao, n, sem_vaga, sem_humano);
    SCOREBOARD.record(r == c, || format!("find_nth_unqualified_club_of_nation(nação {:x}, n {}, {} {}): original {:x} rust {:x}", nacao, n as u8, sem_vaga, sem_humano, r, c));
    r
}

/// Os 20 clubes convidados da Copa Mercosul (as chaves fixas: River Plate, Boca Juniors, Independiente, San Lorenzo,
/// Vélez Sarsfield, Cruzeiro, São Paulo, Palmeiras, Corinthians, Flamengo, Rosario Central, Vasco, Colo-Colo,
/// Universidad de Chile, Universidad Católica, Peñarol, Nacional, Olimpia, Cerro Porteño e Grêmio), na ordem da original.
const MERCOSUR_INVITED_CLUBS: [usize; 20] = [
    0x9d00ac, 0x9d00d0, 0x9d00b4, 0x9d00d8, 0x9d00b0, 0x9d026c, 0x9d0270, 0x9d0274, 0x9d0278, 0x9d0358,
    0x9d00e0, 0x9d0354, 0x9d0280, 0x9d0284, 0x9d0288, 0x9d0298, 0x9d028c, 0x9d0290, 0x9d0294, 0x9d027c,
];

/// { 0x4c6890 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::is_mercosur_invited_club`; o this não é
/// lido): 1 se o id do clube é um dos 20 convidados da Copa Mercosul (`MERCOSUR_INVITED_CLUBS`), senão 0 (sem clube, devolve o 0
/// do próprio ponteiro).
unsafe extern "thiscall" fn is_mercosur_invited_club(_this: usize, clube: usize) -> u32 {
    if clube == 0 {
        return 0;
    }
    let id = cqj1_d(clube);
    for &chave in MERCOSUR_INVITED_CLUBS.iter() {
        if id == cqj1_d(chave) {
            return 1; // cqj1: convidado
        }
    }
    0
}

/// A devolução das duas (a rotina só lê).
unsafe extern "thiscall" fn compare_is_mercosur_invited_club(this: usize, clube: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    let r = o(this, clube);
    let c = is_mercosur_invited_club(this, clube);
    SCOREBOARD.record(r == c, || format!("is_mercosur_invited_club({:x}): original {} rust {}", clube, r, c));
    r
}
