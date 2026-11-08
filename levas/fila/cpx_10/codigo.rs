// ---------- %%LEVA%%a leva: as notícias do técnico: a diretoria pensa no cargo do técnico, a saída do técnico e o técnico novo (manager_manager.cpp) ----------

unsafe fn cpx10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpx10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpx10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpx10_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpx10_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A data das notícias de técnico ([0xdd6ec8]: dia com sinal em +0, ano sem sinal em +2) ainda não chegou (é depois de
/// hoje, [0xae2c90])? Então nenhuma notícia sai.
unsafe fn before_news_start_date() -> bool {
    let (dia, ano) = (cpx10_w(0xdd6ec8), cpx10_w(0xdd6eca) as u16);
    let hoje = cpx10_w(0xae2c92) as u16;
    ano > hoje || (ano == hoje && dia > cpx10_w(0xae2c90))
}

/// A linha do clube na tabela da diretoria (0x49 bytes por id, a partir de [this]).
unsafe fn board_row_of(this: usize, clube: usize) -> usize {
    cpx10_d(this).wrapping_add(cpx10_i(clube).wrapping_mul(0x49) as usize)
}

unsafe fn news_set_slot(n: usize, k: u32, v: u32) {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x785790usize);
    f(n, k, v);
}

/// O id e as três chaves (+0x33 de +4, +8 e +0xc) da pessoa `p` nos dados `k`..`k`+3 da notícia.
unsafe fn news_set_person(n: usize, k: u32, p: usize) {
    news_set_slot(n, k, cpx10_d(p) as u32);
    news_set_slot(n, k + 1, cpx10_d(cpx10_d(p + 4) + 0x33) as u32);
    news_set_slot(n, k + 2, cpx10_d(cpx10_d(p + 8) + 0x33) as u32);
    news_set_slot(n, k + 3, cpx10_d(cpx10_d(p + 0xc) + 0x33) as u32);
}

/// O técnico humano (uma das 16 últimas pessoas: id ≥ [0xae2364] − 16)?
unsafe fn is_human_manager(p: usize) -> bool {
    cpx10_i(p) >= cpx10_i(0xae2364).wrapping_sub(0x10)
}

/// Os dois nomes do técnico humano (os textos de +4 e +8, 0x7858c0) a partir do dado `primeiro`; o próximo dado livre
/// depois do primeiro nome vai no dado `k_proximo`.
unsafe fn news_set_manager_names(n: usize, p: usize, primeiro: u8, k_proximo: u32) {
    let texto: unsafe extern "thiscall" fn(usize, u32, usize, *mut u8) -> u32 = core::mem::transmute(0x7858c0usize);
    // ponytail: a original guarda o próximo dado no byte baixo de uma local que tinha a data [0xdd6ec8] e empurra o
    // dword inteiro como campo; a 0x7858c0 lê só o byte baixo, e a cópia passa o valor limpo
    let mut proximo = primeiro;
    texto(n, primeiro as u32, cpx10_d(p + 4), &mut proximo);
    news_set_slot(n, k_proximo, proximo as i8 as i32 as u32);
    texto(n, proximo as u32, cpx10_d(p + 8), &mut proximo);
}

/// O ensaio do comparador: a cópia roda sem entregar a notícia (as entregas copiam a notícia nas caixas dos técnicos
/// humanos, alocam e podem dar a resposta automática) e guarda o alcance que daria (+0xde: 4 todos, 2 os do país, 1 os
/// da divisão; 0 nenhuma) e a notícia montada.
static mut MANAGER_NEWS_DRY_RUN: bool = false;
static mut MANAGER_NEWS_REACH: u8 = 0;
static mut MANAGER_NEWS_BUILT: [u8; 0xe0] = [0; 0xe0];

/// A entrega da notícia `n` pelo `alcance` (4: 0x786780; 2: 0x7863f0 com a nação; 1: 0x7862d0 com a divisão) pelas
/// notícias [0xb7431c]; no ensaio, só o plano.
unsafe fn deliver_news(n: usize, alcance: u8, alvo: usize) {
    if MANAGER_NEWS_DRY_RUN {
        MANAGER_NEWS_REACH = alcance;
        core::ptr::copy_nonoverlapping(n as *const u8, core::ptr::addr_of_mut!(MANAGER_NEWS_BUILT) as *mut u8, 0xe0);
        return;
    }
    let noticias = cpx10_d(0xb7431c);
    match alcance {
        4 => {
            let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
            f(noticias, n);
        }
        2 => {
            let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
            f(noticias, n, alvo);
        }
        _ => {
            let f: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7862d0usize);
            f(noticias, n, alvo);
        }
    }
}

/// A entrega das notícias de saída e de técnico novo de um técnico da máquina: clube que pode ser dirigido (0x544820);
/// seleção (0x53b900) ou reputação (+0x80) acima de 7250: a todos; senão aos do país (+0x53), se houver.
unsafe fn deliver_news_for_club(n: usize, clube: usize) {
    let pode: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if pode(clube) == 0 {
        return;
    }
    if selecao(clube) != 0 || cpx10_w(clube + 0x80) > 0x1c52 {
        deliver_news(n, 4, 0);
    } else {
        let nacao = cpx10_d(clube + 0x53);
        if nacao != 0 {
            deliver_news(n, 2, nacao);
        }
    }
}

/// O comparador das três: a cópia em ensaio (sem entregar) e a original a partir do mesmo estado (as `regioes` e o
/// sorteio). Iguais: as `regioes`; os sorteios da cópia são o começo dos da original (a resposta automática da entrega
/// pode sortear depois); e cada técnico humano cuja caixa (News +8: 12 bytes por humano, início do anel em +0, próxima
/// vaga em +4) andou recebeu a notícia da cópia: tipo (+0), dados e data (+5..+0xd6) e o alcance (+0xde); sem entrega
/// planejada, nenhuma caixa anda.
unsafe fn compare_manager_news(nome: &str, regioes: Vec<(usize, usize)>, original: impl FnOnce(), copia: impl FnOnce()) {
    let noticias = cpx10_d(0xb7431c);
    let caixas = if noticias != 0 { cpx10_d(noticias + 8) } else { 0 };
    let antes_caixas: Vec<(i32, i32)> = if caixas != 0 { (0..0x10).map(|k| (cpx10_i(caixas + 12 * k), cpx10_i(caixas + 12 * k + 4))).collect() } else { Vec::new() };
    let mut todas = regioes.clone();
    todas.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&todas);
    MANAGER_NEWS_DRY_RUN = true;
    MANAGER_NEWS_REACH = 0;
    let (_, rr) = crate::aleatorio::rastrear(|| crate::aleatorio::sem_contar(copia));
    MANAGER_NEWS_DRY_RUN = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    let (alcance, noticia) = (MANAGER_NEWS_REACH, *core::ptr::addr_of!(MANAGER_NEWS_BUILT));
    antes.escrever();
    let (_, ro) = crate::aleatorio::rastrear(original);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    let mut entregues = 0;
    let mut noticia_igual = true;
    let caixas = if noticias != 0 { cpx10_d(noticias + 8) } else { 0 };
    if caixas != 0 && antes_caixas.len() == 0x10 {
        for (k, &(inicio, vaga)) in antes_caixas.iter().enumerate() {
            let e = caixas + 12 * k;
            if cpx10_i(e + 4) == vaga && cpx10_i(e) == inicio {
                continue;
            }
            entregues += 1;
            let v = if inicio == -1 { cpx10_i(e) } else { vaga }; // a primeira notícia abre o anel no início
            let s = cpx10_d(noticias).wrapping_add((v as usize).wrapping_mul(0xdf));
            let g = core::slice::from_raw_parts(s as *const u8, 0xdf);
            if g[0..4] != noticia[0..4] || g[5..0xd6] != noticia[5..0xd6] || g[0xde] != alcance {
                noticia_igual = false;
            }
        }
    }
    let igual = mo == mr && ro.len() >= rr.len() && ro[..rr.len()] == rr[..] && noticia_igual && (alcance != 0 || entregues == 0);
    PLACAR.registrar(igual, || format!("{} original {:x?} sorteios {:?} rust {:x?} sorteios {:?}; alcance {} entregues {} noticia igual {}", nome, mo, ro, mr, rr, alcance, entregues, noticia_igual));
}

/// { 0x6a0e70 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::post_manager_under_pressure_news(clube, tipo)`,
/// por pensar em demitir o técnico (0x69aba0: tipo 1 na troca pelo melhor candidato, 2 depois de 14 dias). Clube sem
/// técnico (+0xcf) ou antes da data [0xdd6ec8]: nada. A notícia 0x1777: o técnico (0..3), o clube (4), o presidente
/// (+0xbf: 5..7 e o id em 10; sem ele, −1). O motivo (8) pela linha da diretoria (+6 a confiança): avaliação +8, +0xa ou
/// +0xc abaixo da confiança − 500 com sorteio(3) zero dá 2, 1 ou 3 (+1 com sorteio(5) não zero); senão 0. O tipo (9) e
/// o boato (11): sorteio(10), ou 1 se o técnico conhece bem o clube (0x544410 do não jogador +0x69 com +0x1b acima de
/// 10). Entrega: técnico humano, a todos; clube de reputação acima de 5000, aos do país; senão aos da divisão (+0x57)
/// que não seja a [0x9cf764]. Por fim a data de hoje em +0x18 da linha (0x417c10).
unsafe extern "thiscall" fn post_manager_under_pressure_news(this: usize, clube: usize, tipo: u32) {
    if clube == 0 {
        return;
    }
    let tec = cpx10_d(clube + 0xcf);
    if tec == 0 || before_news_start_date() {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1777, 0);
    news_set_person(n, 0, tec);
    news_set_slot(n, 4, cpx10_d(clube) as u32);
    let pres = cpx10_d(clube + 0xbf);
    if pres != 0 {
        news_set_slot(n, 5, cpx10_d(cpx10_d(pres + 4) + 0x33) as u32);
        news_set_slot(n, 6, cpx10_d(cpx10_d(pres + 8) + 0x33) as u32);
        news_set_slot(n, 7, cpx10_d(cpx10_d(pres + 0xc) + 0x33) as u32);
        news_set_slot(n, 0xa, cpx10_d(pres) as u32);
    } else {
        for k in [5, 6, 7, 0xa] {
            news_set_slot(n, k, u32::MAX);
        }
    }
    let l = board_row_of(this, clube);
    let motivo: i8 = if (cpx10_w(l + 8) as i32) < cpx10_w(l + 6) as i32 - 500 /* cpx10: o limit pela avaliação +8 */ && {
        crate::comparacao::ramo("cpx_10 motivo pela avaliacao");
        cpx10_sortear(3) == 0
    } {
        2
    } else if (cpx10_w(l + 0xa) as i32) < cpx10_w(l + 6) as i32 - 500 && cpx10_sortear(3) == 0 {
        1
    } else if (cpx10_w(l + 0xc) as i32) < cpx10_w(l + 6) as i32 - 500 && cpx10_sortear(3) == 0 {
        3 + (cpx10_sortear(5) != 0) as i8
    } else {
        0
    };
    news_set_slot(n, 8, motivo as i32 as u32);
    news_set_slot(n, 9, tipo as u8 as i8 as i32 as u32);
    crate::comparacao::ramo("cpx_10 sorteio do boato");
    let mut boato = cpx10_sortear(10); // cpx10: o sorteio do boato
    let nj = cpx10_d(tec + 0x69);
    if nj != 0 {
        let conhecer: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
        if conhecer(nj, cpx10_b(nj + 0x1b) as i8) as u8 as i8 > 10 {
            boato = 1;
        }
    }
    news_set_slot(n, 0xb, boato as u32);
    if is_human_manager(tec) {
        deliver_news(n, 4, 0);
    } else if cpx10_w(clube + 0x80) > 0x1388 {
        deliver_news(n, 2, cpx10_d(clube + 0x53));
    } else {
        let div = cpx10_d(clube + 0x57);
        if div != 0 && cpx10_i(div) != cpx10_i(0x9cf764) {
            deliver_news(n, 1, div);
        }
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut data = [0u32; 2];
    crate::comparacao::ramo("cpx_10 data da pressure");
    copiar(board_row_of(this, clube) + 0x18, data.as_mut_ptr() as usize, 0xae2c90); // cpx10: a data da notícia na linha
}

unsafe extern "thiscall" fn comparar_post_manager_under_pressure_news(this: usize, clube: usize, tipo: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I0%%);
    let regioes = if clube != 0 { vec![(board_row_of(this, clube), 0x49)] } else { Vec::new() };
    compare_manager_news("post_manager_under_pressure_news", regioes, || o(this, clube, tipo), || post_manager_under_pressure_news(this, clube, tipo));
}

/// { 0x6a1b00 } (thiscall, ret 0x14, manager_manager.cpp): `MANAGER_MANAGER::post_manager_departure_news(tecnico, tipo,
/// motivo, clube, cargo)`, por tirar o técnico do cargo (0x69b790). Técnico ou clube nulos, antes da data [0xdd6ec8] ou
/// tipo 5: nada. A notícia 6000: o técnico (0..3), o tipo (4), o clube (5), o cargo (7), as chaves do presidente (+0xbf:
/// 8..10; sem ele, −1) e o motivo (11). Técnico humano: os nomes dele a partir do dado 13 (o próximo livre no 12) e a
/// todos; senão pelo clube (0x544820; seleção ou reputação acima de 7250 a todos, senão aos do país).
unsafe extern "thiscall" fn post_manager_departure_news(this: usize, tec: usize, tipo: u32, motivo: u32, clube: usize, cargo: u32) {
    let _ = this;
    if tec == 0 || clube == 0 || before_news_start_date() {
        return;
    }
    let t = tipo as u8 as i8;
    if t == 5 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1770, 0);
    news_set_person(n, 0, tec);
    news_set_slot(n, 4, t as i32 as u32);
    news_set_slot(n, 5, cpx10_d(clube) as u32);
    news_set_slot(n, 7, cargo as u8 as i8 as i32 as u32);
    let pres = cpx10_d(clube + 0xbf);
    for (k, d) in [(8, 4), (9, 8), (0xa, 0xc)] {
        news_set_slot(n, k, if pres != 0 { cpx10_d(cpx10_d(pres + d) + 0x33) as u32 } else { u32::MAX });
    }
    news_set_slot(n, 0xb, motivo as u8 as i8 as i32 as u32);
    crate::comparacao::ramo("cpx_10 noticia da saida");
    if is_human_manager(tec) {
        news_set_manager_names(n, tec, 0xd, 0xc);
        deliver_news(n, 4, 0);
    } else {
        deliver_news_for_club(n, clube);
    }
}

unsafe extern "thiscall" fn comparar_post_manager_departure_news(this: usize, tec: usize, tipo: u32, motivo: u32, clube: usize, cargo: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, usize, u32)>(%%I1%%);
    compare_manager_news("post_manager_departure_news", Vec::new(), || o(this, tec, tipo, motivo, clube, cargo), || post_manager_departure_news(this, tec, tipo, motivo, clube, cargo));
}

/// { 0x6a2e00 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::post_manager_appointment_news(tecnico,
/// anterior, clube, cargo)`, pelo dia do mercado de técnicos (0x68f9c0) e pela nomeação (0x69ebc0). Técnico ou clube
/// nulos, antes da data [0xdd6ec8] ou cargo 5: nada. Só clube de nação ativa (+0x11c da nação) com a divisão no vetor
/// [0xadadfc], ou seleção (0x53b900) de nação ativa. A notícia 0x1787: o técnico (0..3), o clube anterior (4; −1), o
/// clube (5), o cargo (6) e a função anterior (7: +0x28 numa seleção, +0x3d num clube; sem anterior, −1). Técnico
/// humano: os nomes a partir do dado 8 (o próximo livre no 7) e a todos; senão pelo clube, como a da saída.
unsafe extern "thiscall" fn post_manager_appointment_news(this: usize, tec: usize, anterior: usize, clube: usize, cargo: u32) {
    let _ = this;
    if tec == 0 || clube == 0 || before_news_start_date() {
        return;
    }
    let c = cargo as u8 as i8;
    if c == 5 {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let nacao = cpx10_d(clube + 0x53);
    let ativa = nacao != 0 && cpx10_b(nacao + 0x11c) != 0 && {
        let div = cpx10_d(clube + 0x57);
        div != 0 && cpx10_d(cpx10_d(0xadadfc).wrapping_add(cpx10_i(div).wrapping_mul(4) as usize)) != 0
    };
    // a original lê +0x11c da nação da seleção sem conferir a nação (toda seleção tem uma)
    if !ativa && !(selecao(clube) != 0 && cpx10_b(cpx10_d(clube + 0x53) + 0x11c) != 0) {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0x1787, 0);
    news_set_person(n, 0, tec);
    news_set_slot(n, 4, if anterior != 0 { cpx10_d(anterior) as u32 } else { u32::MAX });
    news_set_slot(n, 5, cpx10_d(clube) as u32);
    news_set_slot(n, 6, c as i32 as u32);
    let funcao = if anterior == 0 {
        u32::MAX
    } else if selecao(anterior) != 0 {
        cpx10_b(tec + 0x28) as i8 as i32 as u32
    } else {
        cpx10_b(tec + 0x3d) as i8 as i32 as u32
    };
    news_set_slot(n, 7, funcao);
    crate::comparacao::ramo("cpx_10 noticia do tecnico novo");
    if is_human_manager(tec) {
        news_set_manager_names(n, tec, 8, 7);
        deliver_news(n, 4, 0);
    } else {
        deliver_news_for_club(n, clube);
    }
}

unsafe extern "thiscall" fn comparar_post_manager_appointment_news(this: usize, tec: usize, anterior: usize, clube: usize, cargo: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32)>(%%I2%%);
    compare_manager_news("post_manager_appointment_news", Vec::new(), || o(this, tec, anterior, clube, cargo), || post_manager_appointment_news(this, tec, anterior, clube, cargo));
}
