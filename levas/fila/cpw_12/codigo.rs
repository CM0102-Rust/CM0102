// ---------- %%LEVA%%a leva: o jogo entre seleções: as convocações, os jogos e gols pela seleção, o nome do elenco e as rodadas conhecidas (national_teams.cpp, comp.cpp) ----------

unsafe fn cpw12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O ensaio do comparador da 0x76b600: a cópia roda sem o ranking da FIFA (0x58cf70, acrescenta pontos ao histórico),
/// sem a primeira convocação (0x86cc50, vai para o arquivo), sem o jogo pela seleção no registro de transferências
/// (0x8ce920) e sem a notícia aos clubes (0x7706c0, aloca).
static mut CPW12_ENSAIO: bool = false;

/// A vaga `k` (0..20) da escalação do lado `lado` no resumo do jogo `m`: m + 0x625 × lado + 0x190 + 0x49 × k.
fn cpw12_vaga(m: usize, lado: usize, k: usize) -> usize {
    m.wrapping_add(0x625 * lado + 0x190 + 0x49 * k)
}

/// A pessoa da vaga, se a vaga conta (id ≠ −1 e titular, ou reserva que entrou: +0x32 ≠ 0xff) e a pessoa está na
/// faixa (id < [0xae2364]; o ponteiro nulo passa sem o teste, como na original).
unsafe fn cpw12_pessoa(vaga: usize, k: usize) -> Option<usize> {
    let id = cpw12_d(vaga) as i32;
    if id == -1 || (k >= 11 && *((vaga + 0x32) as *const u8) == 0xff) {
        return None;
    }
    let p = cpw12_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    if p != 0 && cpw12_d(p) as i32 >= cpw12_d(0xae2364) as i32 {
        return None;
    }
    Some(p)
}

/// O registro geral da pessoa (79 bytes em [0xae2d0c]).
unsafe fn cpw12_geral(p: usize) -> usize {
    cpw12_d(0xae2d0c).wrapping_add(cpw12_d(p).wrapping_mul(0x4f))
}

/// O registro da nação do clube nas seleções: [this + 4] + 0x6e × id.
unsafe fn cpw12_nacao(this: usize, clube: usize) -> usize {
    cpw12_d(this + 4).wrapping_add(cpw12_d(cpw12_d(clube + 0x53)).wrapping_mul(0x6e))
}

/// Jogou pela nação do clube: se a segunda nação (+0x1e) é a dele, ela vira a primeira (+0x1a) e a antiga a segunda.
unsafe fn cpw12_trocar_nacao(p: usize, clube: usize) {
    if cpw12_d(clube + 0x53) == cpw12_d(p + 0x1e) {
        core::ptr::write_unaligned((p + 0x1e) as *mut usize, cpw12_d(p + 0x1a));
        core::ptr::write_unaligned((p + 0x1a) as *mut usize, cpw12_d(clube + 0x53));
    }
}

/// O jogo de nação por lado (o seu código de competição em [p + 0x14]): conta o jogo da nação (+2 do registro dela,
/// curto) e cada pessoa que conta na escalação.
unsafe fn cpw12_lados(this: usize, p: usize, m: usize, mut f: impl FnMut(usize, usize, usize, usize)) {
    for lado in 0..2usize {
        let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
        let jogos = (cpw12_nacao(this, clube) + 2) as *mut i16;
        core::ptr::write_unaligned(jogos, core::ptr::read_unaligned(jogos).wrapping_add(1));
        for k in 0..20usize {
            let vaga = cpw12_vaga(m, lado, k);
            if let Some(pessoa) = cpw12_pessoa(vaga, k) {
                f(clube, vaga, k, pessoa);
            }
        }
    }
}

/// { 0x76b600 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::record_international_result(partida, resumo)`:
/// só entre duas seleções (0x53b900). Pela competição da partida ([+0x14]): a de [0x9cf794] marca +0xd = 1 no registro
/// geral de quem jogou; a de [0x9cf798] liga o bit 2 do +0xc e troca a nação (+0x1a/+0x1e); a de [0x9cf7a0] nada; as
/// outras: o ranking da FIFA (0x58cf70 em [this+0x1c]) e, de quem jogou, a primeira convocação (0x86cc50 com o
/// adversário, se +0x22 = 0, depois da troca da nação), um jogo (+0x22) e os gols (+0x23 += +0x21 da vaga) pela
/// seleção e o jogo no registro (0x8ce920 com o número de jogos da nação). Em todas, o jogo da nação (+2) sobe. Depois,
/// por lado: o tipo (1 se +0x40 = 0; senão 2 na segunda seleção 0x5409b0, senão 0); na data do tipo (+0x3a + 20 ×
/// tipo, 0x4a7f20 com a de hoje) zera +0x44 + 20 × tipo; a notícia aos clubes (0x7706c0) com o destaque (vaga +0x153) e
/// a lista de quem jogou. Sem SEH, sem caixa, sem sorteio.
unsafe extern "thiscall" fn record_international_result(this: usize, p: usize, m: usize) {
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if is_national_team(cpw12_d(p + 0x1c)) == 0 || is_national_team(cpw12_d(p + 0x20)) == 0 {
        return;
    }
    let comp = cpw12_d(cpw12_d(p + 0x14));
    if comp == cpw12_d(0x9cf794) {
        cpw12_lados(this, p, m, |_, _, _, pessoa| *((cpw12_geral(pessoa) + 0xd) as *mut u8) = 1);
    } else if comp == cpw12_d(0x9cf798) {
        cpw12_lados(this, p, m, |clube, _, _, pessoa| {
            *((cpw12_geral(pessoa) + 0xc) as *mut u8) |= 2;
            cpw12_trocar_nacao(pessoa, clube);
        });
    } else if comp != cpw12_d(0x9cf7a0) {
        let fifa: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x58cf70usize);
        let primeira: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86cc50usize);
        let jogo_pela_selecao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8ce920usize);
        if !CPW12_ENSAIO {
            fifa(cpw12_d(this + 0x1c), p);
        }
        for lado in 0..2usize {
            let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
            let jogos = (cpw12_nacao(this, clube) + 2) as *mut i16;
            core::ptr::write_unaligned(jogos, core::ptr::read_unaligned(jogos).wrapping_add(1));
            for k in 0..20usize {
                let vaga = cpw12_vaga(m, lado, k);
                let id = cpw12_d(vaga) as i32;
                if id == -1 {
                    continue;
                }
                if k >= 11 && *((vaga + 0x32) as *const u8) == 0xff { // cpw12: a reserva que não entrou
                    crate::comparacao::ramo("cpw_12 reserva sem entrar");
                    continue;
                }
                let pessoa = cpw12_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
                if pessoa != 0 && cpw12_d(pessoa) as i32 >= cpw12_d(0xae2364) as i32 {
                    continue;
                }
                if *((pessoa + 0x22) as *const u8) == 0 {
                    cpw12_trocar_nacao(pessoa, clube);
                    let adversario = if clube == cpw12_d(p + 0x1c) { cpw12_d(p + 0x20) } else { cpw12_d(p + 0x1c) };
                    if !CPW12_ENSAIO {
                        primeira(cpw12_d(0xdd7f00), pessoa, adversario);
                    }
                }
                crate::comparacao::ramo("cpw_12 jogo pela seleção");
                *((pessoa + 0x22) as *mut u8) = (*((pessoa + 0x22) as *const u8)).wrapping_add(1); // cpw12: um jogo
                *((pessoa + 0x23) as *mut u8) = (*((pessoa + 0x23) as *const u8)).wrapping_add(*((vaga + 0x21) as *const u8)); // cpw12: os gols
                if !CPW12_ENSAIO {
                    // a original empurra o edx com o curto dos jogos embaixo e o alto de 55 × id da nação
                    let id_nacao = cpw12_d(cpw12_d(clube + 0x53)) as u32;
                    let n = core::ptr::read_unaligned(jogos) as u16 as u32;
                    jogo_pela_selecao(cpw12_d(0xde1f54), pessoa, (id_nacao.wrapping_mul(0x37) & 0xffff_0000) | n);
                }
            }
        }
    }
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let mesma_data: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4a7f20usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, *mut usize, u32) = core::mem::transmute(0x7706c0usize);
    for lado in 0..2usize {
        let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
        let b = *((p + 0x40) as *const u8) == 0;
        let tipo: usize = if segunda(clube) == 0 { if b { 1 } else { 0 } } else if b { 1 } else { 2 };
        let reg = cpw12_nacao(this, clube) + 20 * tipo;
        if mesma_data(reg + 0x3a, cpw12_d(0xae2c90) as u32, cpw12_d(0xae2c94) as u32) != 0 {
            *((reg + 0x44) as *mut u8) = 0;
        }
        let mut lista = [0usize; 20];
        let mut q = 0usize;
        for k in 0..20usize {
            if let Some(pessoa) = cpw12_pessoa(cpw12_vaga(m, lado, k), k) {
                lista[q] = pessoa;
                q += 1;
            }
        }
        let destaque_k = *((m + 0x625 * lado + 0x153) as *const i8) as isize;
        let destaque_id = cpw12_d((m + 0x625 * lado + 0x190) .wrapping_add((0x49 * destaque_k) as usize));
        let destaque = cpw12_d(0xae23bc).wrapping_add(destaque_id.wrapping_mul(0x6e));
        if !CPW12_ENSAIO {
            // ponytail: a original empurra a conta num dword da local com só o byte de baixo gravado; a 0x7706c0 lê o byte
            noticia(this, clube, p, m, destaque, lista.as_mut_ptr(), q as u32);
        }
    }
}

/// As regiões que a 0x76b600 grava por conta própria: de cada lado, o registro da nação (o +2 e os +0x44 dos tipos) e,
/// de cada vaga com pessoa na faixa, os campos +0x1a..+0x24 dela e o +0xc/+0xd do registro geral.
unsafe fn cpw12_regioes(this: usize, p: usize, m: usize) -> Vec<(usize, usize)> {
    let mut reg = Vec::new();
    for lado in 0..2usize {
        let clube = cpw12_d(p + if lado == 0 { 0x1c } else { 0x20 });
        let r = cpw12_nacao(this, clube);
        reg.extend([(r + 2, 2), (r + 0x44, 1), (r + 0x58, 1), (r + 0x6c, 1)]);
        for k in 0..20usize {
            let vaga = cpw12_vaga(m, lado, k);
            let id = cpw12_d(vaga) as i32;
            if id == -1 {
                continue;
            }
            let pessoa = cpw12_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
            if pessoa != 0 && (cpw12_d(pessoa) as i32) < cpw12_d(0xae2364) as i32 {
                reg.push((pessoa + 0x1a, 10));
                reg.push((cpw12_geral(pessoa) + 0xc, 2));
            }
        }
    }
    reg
}

/// Comparador: o ensaio (a cópia sem o ranking, a primeira convocação, o registro de transferências e a notícia, que
/// alocam, vão para o arquivo ou gravam fora) e a original a partir do mesmo estado, conferidos nas regiões que a
/// rotina grava por conta própria (cpw12_regioes). Fora de jogo entre seleções: só a original.
unsafe extern "thiscall" fn comparar_record_international_result(this: usize, p: usize, m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if is_national_team(cpw12_d(p + 0x1c)) == 0 || is_national_team(cpw12_d(p + 0x20)) == 0 {
        return o(this, p, m); // ponytail: não grava nada, só a original
    }
    let regioes = cpw12_regioes(this, p, m);
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CPW12_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| record_international_result(this, p, m)));
    CPW12_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    o(this, p, m);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(mo == mr, || format!("record_international_result(partida {:x}, resumo {:x}) original={:x?} rust={:x?}", p, m, mo, mr));
}

/// { 0x775610 } (thiscall, ret 0x14, national_teams.cpp; `this` não é lido): `NationalTeams::format_squad_label(tipo,
/// sub23, saida, tamanho, minusculo)`: tamanho abaixo de 20: a caixa (linha 0x1c98). Escreve (0x66f4e0) o nome do
/// elenco: tipo 0 "Main", 1 "'B'", outros "Under 23" (com sub23) ou "Under 21"; com minúsculo = 1, em minúsculas.
unsafe extern "thiscall" fn format_squad_label(this: usize, tipo: u32, sub23: u32, saida: usize, tamanho: i32, minusculo: u32) -> u32 {
    if tamanho < 0x14 {
        return orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32>(%%I1%%)(this, tipo, sub23, saida, tamanho, minusculo);
    }
    let texto: unsafe extern "C" fn(usize, usize, ...) -> u32 = core::mem::transmute(0x66f4e0usize);
    let s = match (minusculo as u8 == 1, tipo as u8, sub23 != 0) {
        (true, 0, _) => 0x9cd9b4,
        (true, 1, _) => 0xa62fa0,
        (true, _, true) => 0xa62f94,
        (true, _, false) => 0xa62f88,
        (false, 0, _) => 0xa62f80,
        (false, 1, _) => 0xa62fa0,
        (false, _, true) => 0xa62f74,
        (false, _, false) => 0xa62f68,
    };
    texto(saida, s)
}

/// Comparador: lado a lado no começo da saída (até 64 bytes) e na devolução. A caixa: só a original.
unsafe extern "thiscall" fn comparar_format_squad_label(this: usize, tipo: u32, sub23: u32, saida: usize, tamanho: i32, minusculo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, usize, i32, u32) -> u32>(%%I1%%);
    if tamanho < 0x14 || saida == 0 {
        return o(this, tipo, sub23, saida, tamanho, minusculo); // ponytail: a caixa, só a original
    }
    let reg = [(saida, (tamanho as usize).min(0x40))];
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, tipo, sub23, saida, tamanho, minusculo),
        || format_squad_label(this, tipo, sub23, saida, tamanho, minusculo));
    PLACAR.registrar(igual && ro == rr, || format!("format_squad_label({}, {}, {}) original={:x} {:x?} rust={:x} {:x?}", tipo as u8, sub23, minusculo as u8, ro, mo, rr, mr));
    ro
}

/// { 0x48d410 } (thiscall, ret 8, comp.cpp): `Comp::are_round_fixtures_known(fase, rodada)`: a fase (−1 = a própria
/// competição; acima de +0x30: 0; abaixo de −1: a caixa da linha 0x36e, pela original) é liga (+0x43 = 2): 1. Senão 1
/// se a rodada é antes da atual (+0xab, byte) ou é a atual, a última (+0x3c − 1) e com o código 0x8c (+7 do registro
/// de 0x68 bytes dela em +0xa3); senão 0.
unsafe extern "thiscall" fn are_round_fixtures_known(this: usize, fase: i32, rodada: i32) -> u32 {
    if fase > cpw12_d(this + 0x30) as i32 {
        return 0;
    }
    if fase < -1 {
        return orig::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(%%I2%%)(this, fase, rodada);
    }
    let c = if fase == -1 { this } else { cpw12_d(cpw12_d(this + 0xc).wrapping_add(4 * fase as usize)) };
    if *((c + 0x43) as *const u8) == 2 {
        return 1;
    }
    let atual = *((c + 0xab) as *const i8) as i32;
    if rodada < atual {
        return 1;
    }
    if rodada != atual {
        return 0;
    }
    let n = core::ptr::read_unaligned((c + 0x3c) as *const i16) as i32;
    if atual != n - 1 {
        return 0;
    }
    let cod = core::ptr::read_unaligned((cpw12_d(c + 0xa3).wrapping_add((0x68 * (n - 1)) as usize) + 7) as *const i16);
    (cod == 0x8c) as u32
}

/// Comparador: pelo valor (só lê). A caixa: só a original.
unsafe extern "thiscall" fn comparar_are_round_fixtures_known(this: usize, fase: i32, rodada: i32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(%%I2%%);
    if fase < -1 && fase <= cpw12_d(this + 0x30) as i32 {
        return o(this, fase, rodada); // ponytail: a caixa, só a original
    }
    let r = o(this, fase, rodada);
    let n = are_round_fixtures_known(this, fase, rodada);
    PLACAR.registrar(r == n, || format!("are_round_fixtures_known({:x}, {}, {}) original={} rust={}", this, fase, rodada, r, n));
    r
}
