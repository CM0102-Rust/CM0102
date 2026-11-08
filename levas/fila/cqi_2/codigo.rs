// ---------- %%LEVA%%a leva: a classificação da UEFA: a tabela dos clubes europeus, as vagas da Liga dos Campeões e da Intertoto (uefa_seeding.cpp) ----------

unsafe fn cqi2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cqi2_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn increment_byte(a: usize) {
    *(a as *mut u8) = cqi2_b(a).wrapping_add(1);
}

/// A entrada `i` da tabela das nações da classificação (registros de 0x12 bytes; +0 id da nação, +0xe vagas na
/// Intertoto, +0x11 clubes da nação nas copas).
fn cqi2_e(this: usize, i: i32) -> usize {
    this.wrapping_add((i * 0x12) as usize)
}

/// O clube `c` ganha a vaga na competição `comp`, na fase `fase`, se tem nação e ainda não tem vaga (+0x1db = −1):
/// +0x1db = comp, +0x1df = fase e um clube a mais da nação `i` (+0x11). Devolve se ganhou. (O caso da nação sem entrada,
/// −1, que a original procura pela nação do clube, não acontece: `i` vem sempre da volta pelas nações.)
unsafe fn assign_comp_place(this: usize, c: usize, comp: u32, fase: u8, i: i32) -> bool {
    if c == 0 || cqi2_d(c + 0x53) == 0 || cqi2_d(c + 0x1db) != u32::MAX {
        return false;
    }
    *((c + 0x1df) as *mut u8) = fase;
    *((c + 0x1db) as *mut u32) = comp;
    increment_byte(cqi2_e(this, i) + 0x11);
    true
}

/// { 0x904040 } (thiscall, ret 4, uefa_seeding.cpp, `UefaSeeding::collect_european_clubs`): a tabela dos clubes
/// europeus: aloca 12 bytes por clube do jogo (0x944e46; sem memória, a caixa da linha 0x2ed e devolve 0) e grava o
/// ponteiro em `*saida`; para cada clube (registros de 0x245 em [0xae23b4], [0xae235c] clubes) com nação, que não é
/// seleção (0x53b900) e cuja nação é da Europa (o continente +0x71 com o id de [0x9cfa10]): tira a vaga europeia (+0x1db
/// = −1 e o dword da linha do clube em [0xae2d14], 6 bytes por clube, = −1) e grava o registro (+0 o clube, +4 o id da
/// nação, +8 a reputação +0x80 / 500, +9 o +0x69 da divisão +0x5b [inferência: o nível da divisão], +0xb o +0x5f [inferência: a posição nela]; sem divisão, −1 e 0x7f;
/// com a linha de uma competição sem o +0x11c, a linha e o +0x5f do clube zeram). Ordena pela nação (qsort 0x945333 com
/// 0x905ce0) e, em 2001, aplica a ordem do euro.cfg (0x9059e0, original). Devolve o número de clubes (só os 16 bits
/// baixos contam: quem chama lê o si).
unsafe extern "thiscall" fn collect_european_clubs(this: usize, saida: *mut usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    let euro_cfg: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x9059e0usize);
    let t = alocar((cqi2_d(0xae235c) as i32).wrapping_mul(12) as u32);
    *saida = t;
    if t == 0 {
        show_error_at(0xaa1a1c, 0x2ed);
        return 0;
    }
    let mut k = 0u32;
    let mut c = cqi2_d(0xae23b4) as usize;
    let mut i = 0u32;
    if cqi2_d(0xae235c) as i32 > 0 {
        loop {
            let nacao = cqi2_d(c + 0x53) as usize;
            if nacao != 0 && selecao(c) == 0 {
                let cont = cqi2_d(nacao + 0x71) as usize;
                if cont != 0 && cqi2_d(cont) == cqi2_d(0x9cfa10) {
                    *((c + 0x1db) as *mut u32) = u32::MAX;
                    core::ptr::write_unaligned((cqi2_d(0xae2d14) as usize).wrapping_add((cqi2_d(c) as usize).wrapping_mul(6)) as *mut u32, u32::MAX);
                    let r = t.wrapping_add((k as u16 as i16 as i32 * 12) as usize);
                    core::ptr::write_unaligned(r as *mut u32, c as u32);
                    core::ptr::write_unaligned((r + 4) as *mut u32, cqi2_d(nacao));
                    *((r + 8) as *mut u8) = (core::ptr::read_unaligned((c + 0x80) as *const i16) as i32 / 500) as u8;
                    let linha = cqi2_d(c + 0x5b) as usize;
                    if linha != 0 {
                        crate::comparison::branch("cqi_2 division level");
                        core::ptr::write_unaligned((r + 9) as *mut u16, core::ptr::read_unaligned((linha + 0x69) as *const u16)); // cqi2: o nível da divisão
                        *((r + 0xb) as *mut u8) = cqi2_b(c + 0x5f);
                        let comp = cqi2_d(cqi2_d(c + 0x5b) as usize + 0x5d) as usize;
                        if comp == 0 || cqi2_b(comp + 0x11c) == 0 {
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
            if (i as u16 as i16 as i32) >= cqi2_d(0xae235c) as i32 {
                break;
            }
        }
    }
    ordenar(t, k as u16 as i16 as i32, 0xc, 0x905ce0);
    if core::ptr::read_unaligned((this + 0x439) as *const i16) == 0x7d1 {
        euro_cfg(this, t, k);
    }
    // ponytail: a original devolve o ax com os bytes altos do eax que sobraram (quem chama lê só o si); a cópia, zero
    k as u16 as u32
}

/// As diferenças entre duas memórias (os 16 primeiros endereços).
fn cqi2_dif(mo: &crate::comparison::MemorySnapshot, mr: &crate::comparison::MemorySnapshot) -> Vec<usize> {
    let (a, b) = (&mo.0, &mr.0);
    (0..a.len()).flat_map(|q| (0..a[q].1.len()).filter(move |&i| a[q].1[i] != b[q].1[i]).map(move |i| a[q].0 + i)).take(16).collect()
}

/// Os clubes do jogo, a linha de cada clube em [0xae2d14], a marca da caixa e o histórico das competições ([0xae23d4],\n/// [0xae237c] linhas de 0x1a: a 0x4afd80 o reordena, e com linhas empatadas a ordem de antes muda o clube achado).
unsafe fn club_regions() -> Vec<(usize, usize)> {
    let n = (cqi2_d(0xae235c) as i32).max(0) as usize;
    vec![(cqi2_d(0xae23b4) as usize, n * 0x245), (cqi2_d(0xae2d14) as usize, n * 6), (0xb67a34, 4), (cqi2_d(0xae23d4) as usize, (cqi2_d(0xae237c) as i32).max(0) as usize * 0x1a)]
}

/// Lado a lado: a original com a saída de quem chama, a cópia com uma saída própria (liberada depois); os clubes, a
/// linha de cada um em [0xae2d14], a devolução e os registros das duas tabelas.
unsafe extern "thiscall" fn compare_collect_european_clubs(this: usize, saida: *mut usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut usize) -> u32>(%%I0%%);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut t2 = 0usize;
    let reg = club_regions();
    let (a, b, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, saida), || collect_european_clubs(this, &mut t2));
    let (n, m) = (a as u16 as i16, b as u16 as i16);
    let t1 = *saida;
    let tabela = n == m && (n <= 0 || t1 == 0 || t2 == 0 || core::slice::from_raw_parts(t1 as *const u8, n as usize * 12) == core::slice::from_raw_parts(t2 as *const u8, n as usize * 12));
    if t2 != 0 {
        soltar(t2);
    }
    SCOREBOARD.record(igual && tabela, || format!("collect_european_clubs: {} e {} clubes, tabela {}, diferenças em {:x?}", n, m, tabela, cqi2_dif(&mo, &mr)));
    a
}

/// { 0x904210 } (thiscall, ret 8, uefa_seeding.cpp, `UefaSeeding::assign_champions_league_places`): as vagas da Liga dos
/// Campeões ([0x9cf6e8]). O campeão da última edição (0x4afd80 com a linha da competição em [0xae23d0], 0x6b bytes, e o
/// lugar 0) vai direto à fase de grupos (fase 1; a fase de grupos fica com 15 vagas); sem ele, a caixa da linha 0x35d.
/// Depois, nação por nação na ordem do coeficiente (a posição `r`), as vagas por fase (1 = grupos, 2 = terceira
/// preliminar, 3 = segunda, 4 = primeira) pelas faixas fixas: 1ª a 3ª nação 2 nos grupos e 2 na terceira preliminar; 4ª
/// a 6ª 2 e 1; 7ª a 9ª 1 e 1; 10ª 1 nos grupos e 1 na segunda; 11ª a 15ª 1 na terceira e 1 na segunda; 16ª 1 na
/// terceira; 17ª a 29ª 1 na segunda; da 30ª em diante 1 na primeira. Para k = 1..7, o k-ésimo clube da nação
/// (0x9050d0, sem pular ninguém) vai à primeira fase com vaga pedida; fase cheia (as capacidades 16/15, 18, 19 e 18) passa
/// uma vaga para a fase seguinte; clube sem nação ou já com vaga perde a vaga.
unsafe extern "thiscall" fn assign_champions_league_places(this: usize, clubes: usize, conta: u32) {
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x9050d0usize);
    let lc = cqi2_d(0x9cf6e8);
    let mut cap: [u8; 5] = [0, 0x10, 0x12, 0x13, 0x12];
    let h = campeao((cqi2_d(0xae23d0) as usize).wrapping_add(lc.wrapping_mul(0x6b) as usize), 0);
    if h != 0 && cqi2_d(h + 0x53) != 0 && cqi2_d(h + 0x1db) == u32::MAX {
        *((h + 0x1df) as *mut u8) = 1;
        *((h + 0x1db) as *mut u32) = lc;
        let id = cqi2_d(cqi2_d(h + 0x53) as usize);
        let n = *((this + 0x438) as *const i8) as i32;
        let mut i = -1i32;
        for k in 0..n {
            if cqi2_d(cqi2_e(this, k)) == id {
                i = k;
                break;
            }
        }
        increment_byte(cqi2_e(this, i) + 0x11);
        cap[1] = 0xf;
    } else {
        show_error_at(0xaa1a1c, 0x35d);
    }
    let mut r = 0i8;
    while r < *((this + 0x438) as *const i8) {
        let (band, ultima): ([u8; 4], u8) = if r < 3 {
            crate::comparison::branch("cqi_2 two in the groups");
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
        let e = cqi2_e(this, r as i32);
        let mut k = 1u8;
        while k < 8 {
            let c = enesimo(this, clubes, conta, cqi2_d(e), k as u32, 0, 0);
            let mut f = 1usize;
            while f < 5 {
                if vagas[f] != 0 {
                    if cap[f] != 0 {
                        break;
                    }
                    crate::comparison::branch("cqi_2 full stage");
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

/// Lado a lado: a tabela das nações, os clubes e a marca da caixa.
unsafe extern "thiscall" fn compare_assign_champions_league_places(this: usize, clubes: usize, conta: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I1%%);
    let mut reg = club_regions();
    reg.push((this, 0x440));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clubes, conta), || assign_champions_league_places(this, clubes, conta));
    SCOREBOARD.record(igual, || format!("assign_champions_league_places: diferenças em {:x?}", cqi2_dif(&mo, &mr)));
}

/// { 0x904e30 } (thiscall, ret 8, uefa_seeding.cpp, `UefaSeeding::assign_intertoto_places`): as vagas da Intertoto
/// ([0x9cf6f8]) até 60 clubes: dá voltas pelas nações na ordem do coeficiente (a conta vai a `n` e volta a 0, e a fase
/// sai da conta antes de voltar: abaixo de 8 a fase 1, de 8 a 11 a 2, de 12 em diante a 3). A Inglaterra: um clube (o
/// voluntário da 0x903890; sem ele, o primeiro sem vaga e sem humano, 0x9050d0) e, com 2 vagas (+0xe), o segundo, na
/// fase seguinte; depois solta a lista dos voluntários (+0x43b, +0x43f). As outras: tantos clubes quanto as vagas +0xe,
/// o primeiro sem vaga de cada vez (humanos podem), na fase + a ordem do clube, enquanto houver menos de 60.
unsafe extern "thiscall" fn assign_intertoto_places(this: usize, clubes: usize, conta: u32) {
    let voluntario: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x903890usize);
    let enesimo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(0x9050d0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let it = cqi2_d(0x9cf6f8);
    let mut colocados = 0i32;
    let mut cl = 0u8;
    loop {
        let fase: u8 = if (cl as i8) < 8 { 1 } else if (cl as i8) >= 0xc { 3 } else { 2 };
        let mut i = cl;
        if cl == cqi2_b(this + 0x438) {
            i = 0;
        }
        let e = cqi2_e(this, i as i8 as i32);
        if cqi2_d(e) == cqi2_d(0x9cf2e4) {
            let mut c = voluntario(this);
            if c == 0 {
                c = enesimo(this, clubes, conta, cqi2_d(e), 1, 1, 1);
            }
            if assign_comp_place(this, c, it, fase, i as i8 as i32) {
                colocados += 1;
            }
            if cqi2_b(e + 0xe) == 2 {
                let mut c = voluntario(this);
                if c == 0 {
                    c = enesimo(this, clubes, conta, cqi2_d(e), 1, 1, 1);
                }
                if assign_comp_place(this, c, it, fase.wrapping_add(1), i as i8 as i32) {
                    colocados += 1;
                }
            }
            let lista = cqi2_d(this + 0x43b) as usize;
            if lista != 0 {
                soltar(lista);
                core::ptr::write_unaligned((this + 0x43b) as *mut u32, 0);
                *((this + 0x43f) as *mut u8) = 0;
            }
        } else if (cqi2_b(e + 0xe) as i8) > 0 {
            let mut j = 0i8;
            loop {
                if colocados as i16 >= 0x3c {
                    break;
                }
                let c = enesimo(this, clubes, conta, cqi2_d(e), 1, 1, 0);
                if assign_comp_place(this, c, it, (j as u8).wrapping_add(fase), i as i8 as i32) {
                    crate::comparison::branch("cqi_2 club in intertoto");
                    colocados += 1; // cqi2: mais um na Intertoto
                }
                j += 1;
                if j >= cqi2_b(e + 0xe) as i8 {
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

/// Lado a lado em 2001 (a 0x903890 devolve 0 sem efeito) e sem a lista dos voluntários (soltá-la duas vezes não dá):
/// a tabela das nações, os clubes e o sorteio (a cópia sem contar). Fora disso, só a original (a 0x903890 sorteia e
/// entrega notícia).
unsafe extern "thiscall" fn compare_assign_intertoto_places(this: usize, clubes: usize, conta: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%);
    if *(0xae2c92 as *const i16) != 0x7d1 || cqi2_d(this + 0x43b) != 0 {
        return o(this, clubes, conta);
    }
    let mut reg = club_regions();
    reg.push((this, 0x440));
    reg.extend(crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clubes, conta), || crate::rng::without_counting(|| assign_intertoto_places(this, clubes, conta)));
    SCOREBOARD.record(igual, || format!("assign_intertoto_places: diferenças em {:x?}", cqi2_dif(&mo, &mr)));
}
