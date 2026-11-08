// ---------- %%LEVA%%a leva: a Copa do Mundo: a tabela das rodadas de cada fase, a nação classificada e as cabeças de chave (world_cup.cpp) ----------

unsafe fn cqf6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqf6_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqf6_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// Os jogos de cada fase de grupos na tabela das rodadas (0x68a1c0: rodada, jogo, a, b, c, valor), na ordem da
/// original, pela fase + 1 (a tabela de saltos 0x92ccd0: −1 é a própria Copa, o grupo 1; 0..6 os grupos 2..8).
const WORLD_CUP_GROUP_GAMES: [[(i32, i32, i32, i32, i32, u32); 6]; 8] = [
    [(0, 0, 2, 1, 3, 4), (0, 1, 2, 2, 3, 0x4000), (1, 0, 1, 1, 3, 0x1000), (1, 1, 1, 2, 3, 0x400), (2, 0, 1, 2, 3, 0x40), (2, 1, 1, 2, 3, 0x2000)],
    [(0, 0, 3, 1, 3, 0x1000), (0, 1, 3, 2, 3, 0x800), (1, 0, 2, 1, 3, 0x2000), (1, 1, 2, 2, 3, 0x4000), (2, 0, 1, 1, 3, 4), (2, 1, 1, 1, 3, 0x400)],
    [(0, 0, 4, 1, 3, 0x200), (0, 1, 4, 2, 3, 0x40), (1, 0, 3, 2, 3, 0x800), (1, 1, 3, 1, 3, 4), (2, 0, 2, 1, 3, 0x100), (2, 1, 2, 1, 3, 0x1000)],
    [(0, 0, 4, 1, 3, 0x4000), (0, 1, 5, 1, 3, 0x400), (1, 0, 5, 1, 3, 0x2000), (1, 1, 5, 2, 3, 0x80), (2, 0, 2, 2, 3, 0x200), (2, 1, 2, 2, 3, 0x800)],
    [(0, 0, 5, 1, 3, 0x100), (0, 1, 5, 2, 3, 4), (1, 0, 5, 1, 3, 0x1000), (1, 1, 5, 2, 3, 0x40), (2, 0, 3, 1, 3, 0x2000), (2, 1, 3, 1, 3, 0x80)],
    [(0, 0, 6, 1, 3, 0x2000), (0, 1, 0, 2, 3, 0x80), (1, 0, 6, 1, 3, 0x200), (1, 1, 6, 2, 3, 0x100), (2, 0, 3, 2, 3, 0x4000), (2, 1, 3, 2, 3, 0x400)],
    [(0, 0, 0, 1, 3, 0x40), (0, 1, 0, 2, 3, 0x100), (1, 0, 0, 1, 3, 0x4000), (1, 1, 0, 2, 3, 0x800), (2, 0, 4, 2, 3, 4), (2, 1, 4, 2, 3, 0x200)],
    [(0, 0, 6, 1, 3, 0x800), (0, 1, 6, 2, 3, 0x200), (1, 0, 5, 1, 3, 0x400), (1, 1, 6, 1, 3, 0x80), (2, 0, 4, 1, 3, 0x1000), (2, 1, 4, 1, 3, 0x100)],
];

/// { 0x92c290 } (thiscall, ret 0x10, world_cup.cpp, método +0x3c da Copa do Mundo): `WorldCup::create_schedule(fase,
/// &rodadas, &id, &é_liga)`, da criação das regras (0x92d200, fase −1) e das fases dos grupos (0x92d540, 0..6). Nas
/// fases de grupos (char < 7): é_liga (se pedido) = 1, rodadas = 3, id = 0x420 + fase; a tabela de 3 rodadas de 0x41
/// bytes (0x944e46(0xc3); sem memória, a caixa da linha 0xcc e devolve 0), as datas (0x68a160: 12, 19 e 24 de junho
/// do ano da Copa, levadas aos dias da semana 4, 4 e 2, código 1, valor 1), o jogo 2 de cada rodada vazio (−1) e os
/// dois jogos de cada rodada da fase (`WORLD_CUP_GROUP_GAMES`). O mata-mata (fase 7, com as rotinas da copa) e as fases acima:
/// a original. Devolve a tabela.
unsafe extern "thiscall" fn world_cup_create_schedule(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let f = fase as u8 as i8;
    if f >= 7 {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%)(this, fase, rodadas, id, liga);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(0x68a160usize);
    let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(0x68a1c0usize);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = 3; // cqf6: três rodadas
    *id = (f as i16).wrapping_add(0x420);
    let t = alocar(0xc3);
    if t == 0 {
        show_error_at(0xad5888, 0xcc);
        return 0;
    }
    crate::comparison::branch("cqf_6 tabela das rodadas");
    // o ano vai com lixo nos bytes altos na original (o cx do ano sobre o ecx de antes); a 0x68a160 só usa os 16 bits
    // de baixo (a 0x549f70 lê o ano como word e a palavra +2 guarda ano − base): a cópia passa o ano limpo
    let ano = cqf6_w(this + 0x40) as u16 as u32;
    data(t, 0, 0xc, 5, 0, 4, 1, ano, 1); // cqf6: a primeira rodada
    jogo(t, 0, 2, -1, -1, -1, 0);
    data(t, 1, 0x13, 5, 0, 4, 1, ano, 1); // cqf6: a segunda rodada
    jogo(t, 1, 2, -1, -1, -1, 0);
    data(t, 2, 0x18, 5, 0, 2, 1, ano, 1); // cqf6: a terceira rodada
    jogo(t, 2, 2, -1, -1, -1, 0);
    let k = (f as i32 + 1) as u32;
    if k <= 7 {
        for &(r, j, a, b, c, v) in WORLD_CUP_GROUP_GAMES[k as usize].iter() {
            jogo(t, r, j, a, b, c, v);
        }
    }
    t
}

/// Os bytes que a tabela das rodadas tem gravados (o resto da memória alocada não é escrito): de cada rodada, +0..+4 e
/// +0x3d..+0x40; de cada um dos 3 jogos, +5..+0xb.
unsafe fn written_group_round_bytes(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..3usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
        for j in 0..3usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + j * 7 + 5) as *const u8, 7));
        }
    }
    v
}

/// Lado a lado nas fases de grupos: a original com as saídas de quem chama e a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas alocadas têm de ser iguais (a da cópia é liberada).
/// O mata-mata: só a original.
unsafe extern "thiscall" fn comparar_world_cup_create_schedule(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    if fase as u8 as i8 >= 7 {
        return o(this, fase, rodadas, id, liga);
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let (mut r2, mut i2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(
        &reg,
        || o(this, fase, rodadas, id, liga),
        || world_cup_create_schedule(this, fase, &mut r2, &mut i2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }),
    );
    let saidas = *rodadas == r2 && *id == i2 && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { written_group_round_bytes(a) == written_group_round_bytes(b) } else { a == 0 && b == 0 };
    if b != 0 {
        soltar(b);
    }
    SCOREBOARD.record(igual && saidas && tabela, || format!("world_cup_create_schedule({}): saídas {} tabela {}", fase as u8 as i8, saidas, tabela));
    a
}

/// { 0x92e740 } (thiscall, ret 4, world_cup.cpp, sem SEH): `WorldCup::add_qualified_team(clube)`, das eliminatórias
/// (0x90f4f0, 0x911e10, 0x91538e, 0x9184c0, 0x91e200, 0x920500): com a lista das nações (+0x36) abaixo do máximo
/// (+0x38), a vaga +0x36 da tabela [+0x14] (6 bytes) recebe a situação 6 e o clube, e +0x36 sobe; cheia, a caixa da
/// linha 0x456.
unsafe extern "thiscall" fn add_qualified_team(this: usize, clube: usize) {
    let n = cqf6_w(this + 0x36);
    if n >= cqf6_w(this + 0x38) {
        show_error_at(0xad5888, 0x456);
        return;
    }
    crate::comparison::branch("cqf_6 nação classificada");
    let e = cqf6_d(this + 0x14).wrapping_add((n as i32 * 6) as usize);
    cqf6_pb(e + 4, 6);
    cqf6_pd(cqf6_d(this + 0x14).wrapping_add((cqf6_w(this + 0x36) as i32 * 6) as usize), clube);
    cqf6_pw(this + 0x36, (cqf6_w(this + 0x36) as u16).wrapping_add(1));
}

/// Lado a lado (+0x36 e a vaga da tabela); lista cheia (a caixa): só a original.
unsafe extern "thiscall" fn comparar_add_qualified_team(this: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
    let n = cqf6_w(this + 0x36);
    if n >= cqf6_w(this + 0x38) || cqf6_d(this + 0x14) == 0 {
        return o(this, clube);
    }
    let reg = [(this + 0x36, 2usize), (cqf6_d(this + 0x14).wrapping_add((n as i32 * 6) as usize), 6usize)];
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, clube), || add_qualified_team(this, clube));
    SCOREBOARD.record(igual, || format!("add_qualified_team: {:x?} x {:x?}", mo, mr));
}

/// { 0x92f230 } (thiscall, ret 0, world_cup.cpp, sem SEH): `WorldCup::seed_teams()`, do dia da Copa (0x92f1a0, na
/// data das cabeças de chave). Na Copa de 2002 (+0x40 = 2002) volta sem fazer nada: as vagas do sorteio dela vêm do
/// banco de dados, e é por isso que uma Copa de 2002 criada por outro caminho trava o sorteio (o laço da 0x92e940, cqf_4).
/// Nas outras: +0x4b = 0; com menos de 32 nações (+0x36), a caixa da linha 0x59e e completa a lista com a seleção
/// principal (0x53b3d0) de cada nação ([0xae2350] nações de 0x122 bytes) que ainda não esteja nela, até 32; ordena a
/// tabela [+0x14] (6 bytes) pela reputação (qsort 0x945333 com 0x4b07b0); as sedes (0x5fa730 em [0xb5e134]: thiscall,
/// ret 0x14, com o id da Copa [0x9cf79c], o ano como word, as duas saídas e 1; saída −1 = sem sede) e a detentora (o
/// campeão mais recente, 0x4afd80 da competição [0xae23d0] + id × 0x6b) vão para as vagas 1, 2 e 0 (trocando de lugar),
/// e as outras são ordenadas de novo a partir da primeira livre; as 8 primeiras ficam cabeças de chave (pote 0,
/// situação 3) e as outras 24 vão para o pote do continente (+0x53 nação, +0x71 continente: [0x9cfa10] pote 1,
/// [0x9cfa18]/[0x9cfa0c]/[0x9cfa1c] pote 2, [0x9cfa08]/[0x9cfa14] pote 3; sem nação ou continente, a caixa da linha
/// 0x61e) com o id do clube nas vagas +0xee (5 bytes); por fim as 24 da tabela (a partir da 9ª) e as vagas +0x116 são
/// ordenadas pelo pote (0x92f660).
unsafe extern "thiscall" fn seed_teams(this: usize) {
    if cqf6_w(this + 0x40) == 0x7d2 {
        crate::comparison::branch("cqf_6 sem cabeças de chave em 2002");
        return;
    }
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let sedes: unsafe extern "thiscall" fn(usize, usize, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(0x5fa730usize);
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    cqf6_pb(this + 0x4b, 0);
    if cqf6_w(this + 0x36) < 0x20 {
        show_error_at(0xad5888, 0x59e);
        let mut k = 0i32;
        let mut desloc = 0usize;
        if cqf6_d(0xae2350) as i32 > 0 {
            while cqf6_w(this + 0x36) < 0x20 {
                let mut j = 0i16;
                let mut ja = false;
                while j < cqf6_w(this + 0x36) {
                    let c = principal(cqf6_d(0xae23a8).wrapping_add(desloc));
                    if cqf6_d(cqf6_d(this + 0x14) + j as usize * 6) == c {
                        ja = true;
                        break;
                    }
                    j += 1;
                }
                if !ja {
                    let c = principal(cqf6_d(0xae23a8).wrapping_add(desloc));
                    cqf6_pd(cqf6_d(this + 0x14) + cqf6_w(this + 0x36) as usize * 6, c);
                    cqf6_pw(this + 0x36, (cqf6_w(this + 0x36) as u16).wrapping_add(1));
                }
                k += 1;
                desloc += 0x122;
                if k >= cqf6_d(0xae2350) as i32 {
                    break;
                }
            }
        }
    }
    ordenar(cqf6_d(this + 0x14), cqf6_w(this + 0x36) as i32 as u32, 6, 0x4b07b0);
    // ponytail: as duas saídas começam em −1 (sem sede); a original as deixa com o resto da pilha quando a 0x5fa730 não
    // acha a Copa (ela só as grava achando)
    let (mut a, mut b) = (-1i32, -1i32);
    sedes(cqf6_d(0xb5e134), cqf6_d(0x9cf79c), cqf6_w(this + 0x40) as u16 as u32, &mut a, &mut b, 1);
    let sede1 = if a != -1 { principal(cqf6_d(0xae23a8).wrapping_add(a.wrapping_mul(0x122) as usize)) } else { 0 };
    let sede2 = if b != -1 { principal(cqf6_d(0xae23a8).wrapping_add(b.wrapping_mul(0x122) as usize)) } else { 0 };
    let detentor = campeao(cqf6_d(0xae23d0).wrapping_add((cqf6_d(0x9cf79c) as i32).wrapping_mul(0x6b) as usize), 0);
    let mut fixos = 0i8;
    for i in 0..32usize {
        for (vaga, alvo) in [(0usize, detentor), (1, sede1), (2, sede2)] {
            let t = cqf6_d(this + 0x14);
            let e = t + i * 6;
            if cqf6_d(e) == alvo {
                let (c, w) = (cqf6_d(e), cqf6_w(e + 4) as u16);
                let f = t + vaga * 6;
                cqf6_pd(e, cqf6_d(f));
                cqf6_pw(e + 4, cqf6_w(f + 4) as u16);
                let t = cqf6_d(this + 0x14) + vaga * 6;
                cqf6_pd(t, c);
                cqf6_pw(t + 4, w);
                if fixos < vaga as i8 + 1 {
                    fixos = vaga as i8 + 1;
                }
            }
        }
    }
    let mut n = fixos;
    if sede2 == sede1 || sede2 == detentor {
        n = 2;
    }
    if sede1 == detentor {
        n = 1;
    }
    ordenar(cqf6_d(this + 0x14) + n as i32 as usize * 6, (0x20 - n as i32) as u32, 6, 0x4b07b0);
    for i in 0..0x20usize {
        let clube = cqf6_d(cqf6_d(this + 0x14) + i * 6);
        let pote = this + 0xf2 + i * 5;
        cqf6_pd(pote - 4, cqf6_d(clube));
        if i < 8 {
            cqf6_pb(pote, 0);
            cqf6_pb(cqf6_d(this + 0x14) + i * 6 + 4, 3);
            continue;
        }
        let nacao = cqf6_d(clube + 0x53);
        let cont = if nacao != 0 { cqf6_d(nacao + 0x71) } else { 0 };
        if cont == 0 {
            show_error_at(0xad5888, 0x61e);
            continue;
        }
        let c = cqf6_d(cont);
        let (p, s) = if c == cqf6_d(0x9cfa10) {
            (1, 0xa)
        } else if c == cqf6_d(0x9cfa18) || c == cqf6_d(0x9cfa0c) || c == cqf6_d(0x9cfa1c) {
            (2, 0xb)
        } else if c == cqf6_d(0x9cfa08) || c == cqf6_d(0x9cfa14) {
            (3, 0xc)
        } else {
            continue;
        };
        cqf6_pb(pote, p);
        cqf6_pb(cqf6_d(this + 0x14) + i * 6 + 4, s);
    }
    ordenar(cqf6_d(this + 0x14) + 0x30, 0x18, 6, 0x92f660);
    ordenar(this + 0x116, 0x18, 5, 0x92f660);
}

/// Na Copa de 2002 (volta sem gravar nada) lado a lado em +0x4b; nas outras, só a original (caixas, e as saídas da
/// 0x5fa730 que a original pode deixar sem gravar).
unsafe extern "thiscall" fn comparar_seed_teams(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    if cqf6_w(this + 0x40) != 0x7d2 {
        return o(this);
    }
    let reg = [(this + 0x36, 0x16usize), (this + 0xee, 0xa0usize)];
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&reg, || o(this), || seed_teams(this));
    SCOREBOARD.record(igual, || "seed_teams: a Copa de 2002 mudou".to_string());
}
