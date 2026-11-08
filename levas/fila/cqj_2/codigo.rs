// ---------- %%LEVA%%a leva: a classificação da Conmebol: a tabela dos clubes sul-americanos, a virada do ano e o campeão da Copa dos Campeões (conmebol_seeding.cpp) ----------

unsafe fn cqj2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// { 0x4c6280 } (stdcall, ret 4, conmebol_seeding.cpp, `collect_conmebol_clubs`): a tabela dos clubes da Conmebol: aloca
/// 12 bytes por clube do jogo (0x944e46; sem memória, a caixa da linha 0xbb e devolve 0) e grava o ponteiro em `*saida`;
/// para cada clube com nação, que não é seleção (0x53b900), do México ([0x9cf3d0]) ou de nação da América do Sul (o
/// continente +0x71 com o id de [0x9cfa1c]): tira a vaga continental (+0x1db = −1) e grava o registro (+0 o clube, +4 o
/// id da nação, +8 a reputação +0x80 / 500, +9 o +0x69 da divisão +0x5b, +0xb o +0x5f; sem divisão −1 e 0x7f; com a
/// divisão de uma competição sem o +0x11c, a divisão e o +0x5f do clube zeram). Ordena pela nação (qsort 0x945333 com
/// 0x905ce0). Devolve o número de clubes (só os 16 bits baixos contam).
unsafe extern "stdcall" fn collect_conmebol_clubs(saida: *mut usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let t = alocar((cqj2_d(0xae235c) as i32).wrapping_mul(12) as u32);
    *saida = t;
    if t == 0 {
        show_error_at(0x9a7ed8, 0xbb);
        return 0;
    }
    let mut k = 0u32;
    let mut c = cqj2_d(0xae23b4) as usize;
    let mut i = 0u32;
    if cqj2_d(0xae235c) as i32 > 0 {
        loop {
            let nacao = cqj2_d(c + 0x53) as usize;
            if nacao != 0 && selecao(c) == 0 {
                let mexico = cqj2_d(nacao) == cqj2_d(0x9cf3d0); // cqj2: o México entra
                let cont = if mexico { 0 } else { cqj2_d(nacao + 0x71) as usize };
                if mexico || (cont != 0 && cqj2_d(cont) == cqj2_d(0x9cfa1c)) {
                    if mexico {
                        crate::comparison::branch("cqj_2 clube mexicano");
                    }
                    crate::comparison::branch("cqj_2 tira a vaga");
                    *((c + 0x1db) as *mut u32) = u32::MAX; // cqj2: sem vaga continental
                    let r = t.wrapping_add((k as u16 as i16 as i32 * 12) as usize);
                    core::ptr::write_unaligned(r as *mut u32, c as u32);
                    core::ptr::write_unaligned((r + 4) as *mut u32, cqj2_d(nacao));
                    *((r + 8) as *mut u8) = (core::ptr::read_unaligned((c + 0x80) as *const i16) as i32 / 500) as u8;
                    let divisao = cqj2_d(c + 0x5b) as usize;
                    if divisao != 0 {
                        crate::comparison::branch("cqj_2 nivel da divisao");
                        core::ptr::write_unaligned((r + 9) as *mut u16, core::ptr::read_unaligned((divisao + 0x69) as *const u16)); // cqj2: o nível da divisão
                        *((r + 0xb) as *mut u8) = *((c + 0x5f) as *const u8);
                        let comp = cqj2_d(cqj2_d(c + 0x5b) as usize + 0x5d) as usize;
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
            if (i as u16 as i16 as i32) >= cqj2_d(0xae235c) as i32 {
                break;
            }
        }
    }
    ordenar(t, k as u16 as i16 as i32, 0xc, 0x905ce0);
    // ponytail: a original devolve o ax com os bytes altos do eax que sobraram do qsort (quem chama lê o ax); a cópia, zero
    k as u16 as u32
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn cqj2_dif(mo: &crate::comparison::MemorySnapshot, mr: &crate::comparison::MemorySnapshot) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Os clubes do jogo, a marca da caixa e o histórico das competições (a 0x4afd80 o reordena: ver a cqj_1).
unsafe fn club_seeding_regions() -> Vec<(usize, usize)> {
    let n = (cqj2_d(0xae235c) as i32).max(0) as usize;
    vec![(cqj2_d(0xae23b4) as usize, n * 0x245), (0xb67a34, 4), (cqj2_d(0xae23d4) as usize, (cqj2_d(0xae237c) as i32).max(0) as usize * 0x1a)]
}

/// Lado a lado: a original com a saída de quem chama, a cópia com uma saída própria (liberada depois); os clubes, a
/// devolução e os registros das duas tabelas.
unsafe extern "stdcall" fn comparar_collect_conmebol_clubs(saida: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(*mut usize) -> u32>(%%I0%%);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut t2 = 0usize;
    let reg = club_seeding_regions();
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(saida), || collect_conmebol_clubs(&mut t2));
    let (n, m) = (a as u16 as i16, b as u16 as i16);
    let t1 = *saida;
    let tabela = n == m && (n <= 0 || t1 == 0 || t2 == 0 || core::slice::from_raw_parts(t1 as *const u8, n as usize * 12) == core::slice::from_raw_parts(t2 as *const u8, n as usize * 12));
    if t2 != 0 {
        soltar(t2);
    }
    PLACAR.record(igual && tabela, || format!("collect_conmebol_clubs: {} e {} clubes, tabela {}, diferenças em {:x?}", n, m, tabela, cqj2_dif(&mo, &mr)));
    a
}

/// { 0x4c61b0 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::update_for_new_season`): chamada pela
/// Libertadores e pela Mercosul; sem `pular` e com o ano do jogo ([0xae2c92]) outro que o +0x6f: grava o ano, monta a
/// tabela dos clubes (0x4c6280; vazia, a caixa da linha 0x9b e volta sem soltar), dá as vagas da Libertadores (0x4c6430)
/// e solta a tabela.
unsafe extern "thiscall" fn update_for_new_season(this: usize, pular: u32) {
    if pular != 0 {
        return;
    }
    let ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
    if ano == core::ptr::read_unaligned((this + 0x6f) as *const u16) {
        return;
    }
    crate::comparison::branch("cqj_2 virada do ano");
    core::ptr::write_unaligned((this + 0x6f) as *mut u16, ano);
    let juntar: unsafe extern "stdcall" fn(*mut usize) -> u32 = core::mem::transmute(0x4c6280usize);
    let vagas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4c6430usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut t = 0usize;
    let n = juntar(&mut t);
    if n as u16 == 0 {
        show_error_at(0x9a7ed8, 0x9b);
        return;
    }
    vagas(this, t, n);
    soltar(t);
}

/// Lado a lado: o ano gravado (+0x6f), os clubes e a marca da caixa (cada uma monta e solta a própria tabela).
unsafe extern "thiscall" fn comparar_update_for_new_season(this: usize, pular: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I1%%);
    let mut reg = club_seeding_regions();
    reg.push((this + 0x6f, 2));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, pular), || update_for_new_season(this, pular));
    PLACAR.record(igual, || format!("update_for_new_season({}): diferenças em {:x?}", pular, cqj2_dif(&mo, &mr)));
}

/// { 0x4c6960 } (thiscall, ret 4, conmebol_seeding.cpp, `ConmebolSeeding::set_bra_champ_cup_winner`): guarda o id do
/// campeão da Copa dos Campeões em +0x71 (a vaga dele na Libertadores, de 2003 em diante).
unsafe extern "thiscall" fn set_bra_champ_cup_winner(this: usize, clube: usize) {
    core::ptr::write_unaligned((this + 0x71) as *mut u32, cqj2_d(clube));
}

/// Lado a lado: o +0x71.
unsafe extern "thiscall" fn comparar_set_bra_champ_cup_winner(this: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let reg = [(this + 0x71, 4)];
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this, clube), || set_bra_champ_cup_winner(this, clube));
    PLACAR.record(igual, || format!("set_bra_champ_cup_winner({:x})", clube));
}
