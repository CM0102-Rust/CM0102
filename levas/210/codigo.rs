// ---------- 210a leva: a história punível da imprensa, o tipo de lesão sorteado e a categoria da notícia (media.cpp, injury.cpp, news.cpp) ----------

/// { 0x7357c0 } (thiscall, ret 4, media.cpp): o tipo da história guardada na imprensa sobre o jogador
/// quando é de indisciplina (lista de tipos em 0xa17be0, terminada em 0xff), senão 0. Histórias: [+0]
/// vetor de 0x17 bytes, [+8] quantas; em +8 o id do jogador, em +0x11 o tipo (palavra). Jogador nulo ou
/// com +0x39 zerado: 0.
// ponytail: só o ax é fiel; os 16 bits altos do eax da original são resto de registrador (quem chama usa ax)
unsafe extern "thiscall" fn historia_punivel_do_jogador(midia: usize, jogador: usize) -> u16 {
    if jogador == 0 || *((jogador + 0x39) as *const u32) == 0 {
        return 0;
    }
    let n = *((midia + 8) as *const i32);
    let base = *(midia as *const usize);
    let id = *(jogador as *const u32);
    for i in 0..n.max(0) as usize {
        let e = base.wrapping_add(i * 0x17);
        if core::ptr::read_unaligned((e + 8) as *const u32) != id || *(0xa17be0 as *const u8) == 0xff {
            continue;
        }
        let tipo = core::ptr::read_unaligned((e + 0x11) as *const u16);
        let mut p = 0xa17be0usize;
        loop {
            if tipo == *(p as *const i8) as i16 as u16 {
                return tipo;
            }
            p += 1;
            if *(p as *const u8) == 0xff {
                break;
            }
        }
    }
    0 // nenhuma história de indisciplina
}

/// { 0x62d140 } (thiscall, ret 0xc, injury.cpp): o tipo de lesão da categoria (parte do corpo) e da
/// gravidade pedidas: se o jogador já tem lesão atual (0x62ed70) de tipo fora da lista de leves, devolve
/// esse tipo. Senão sorteia cada tipo da categoria (primeiro/último em 0x9e7f05/06 + 3 × categoria; a
/// tabela de tipos tem 13 bytes a partir de 0x9e7ad8: +1 categoria, +2 peso, +0xc gravidade, 0xff =
/// qualquer) e fica com o menor sorteio (menor que 100); o tipo recorrente do jogador ([this] + 31 × id +
/// 0x12) sorteia só 0..2 − 1. Gravidade 3: nenhum (0xff).
// ponytail: só o al é fiel (a original devolve char)
unsafe extern "thiscall" fn escolher_tipo_de_lesao(lesoes: usize, categoria: i8, gravidade: i8, jogador: usize) -> u8 {
    let atual: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mut melhor: i8 = 0x64;
    let mut escolhido: u8 = 0xff;
    let mut recorrente: u8 = 0xff;
    let a = atual(lesoes, jogador);
    if a != 0 {
        let t = *(a as *const u8);
        if !matches!(t, 0 | 1 | 2 | 3 | 4 | 0x4c | 0x4d | 0x4e | 0x4f | 0x50 | 0x51) {
            return t;
        }
    }
    let tipo = |k: i8, o: usize| *((0x9e7ad8 + o) as *const u8).offset(k as isize * 13);
    if jogador != 0 {
        let id = *(jogador as *const u32);
        let r = *((*(lesoes as *const u32)).wrapping_add(id.wrapping_mul(31)).wrapping_add(0x12) as usize as *const u8);
        if r != 0xff && tipo(r as i8, 1) as i8 == categoria {
            recorrente = r;
        }
    }
    let c = categoria as isize * 3;
    let mut bl = *((0x9e7f05 + c) as *const i8);
    let ultimo = (0x9e7f06 + c) as *const i8;
    if bl > *ultimo {
        return escolhido;
    }
    loop {
        let g = tipo(bl, 0xc) as i8;
        let serve = (gravidade == -1 || g == gravidade || g == -1) && gravidade != 3;
        if serve {
            let r = if bl as u8 == recorrente {
                (sorteio(3) as u8).wrapping_sub(1) as i8
            } else {
                sorteio(tipo(bl, 2) as i8 as i32) as i8
            };
            if r < melhor {
                melhor = r;
                escolhido = bl as u8;
            }
        }
        bl = bl.wrapping_add(1);
        if bl > *ultimo {
            return escolhido;
        }
    }
}

/// { 0x785670 } (thiscall, news.cpp): a categoria da notícia pelo milhar do tipo (+0): 1..999 → 2,
/// 1000.. → 7, 2000.. → 3, 3000.. → 4, 4000.. → 5, 5000.. → 1, 6000..0x1790 → 6. Fora disso: a original
/// (caixa de erro). Devolve o eax inteiro como a original (o tipo com o al trocado).
unsafe extern "thiscall" fn categoria_da_noticia(noticia: usize) -> u32 {
    let t = *(noticia as *const i32);
    let c = match t {
        1..=999 => 2,
        1000..=1999 => 7,
        2000..=2999 => 3,
        3000..=3999 => 4,
        4000..=4999 => 5,
        5000..=5999 => 1,
        6000..=0x1790 => 6,
        _ => return orig::<unsafe extern "thiscall" fn(usize) -> u32>(507)(noticia),
    };
    (t as u32 & 0xffff_ff00) | c
}

unsafe extern "thiscall" fn comparar_historia_punivel_do_jogador(midia: usize, jogador: usize) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u16>(505)(midia, jogador);
    let n = historia_punivel_do_jogador(midia, jogador);
    PLACAR.registrar(r == n, || format!("historia_punivel_do_jogador original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_escolher_tipo_de_lesao(lesoes: usize, categoria: i8, gravidade: i8, jogador: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u8>(506);
    // ponytail: jogador nulo abre a caixa de erro do 0x62ed70; não roda duas vezes
    if jogador == 0 {
        return o(lesoes, categoria, gravidade, jogador);
    }
    // ponytail: a 0x62ed70 (primeiro efeito da 0x62d140, antes de qualquer sorteio dela) passa pela
    // 0x62c680, que sorteia só na primeira consulta do dia e guarda em cache; chamá-la aqui antes enche o
    // cache na mesma ordem da original, e os dois lados do lado_a_lado acham o cache cheio
    let atual: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    atual(lesoes, jogador);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO,
        || o(lesoes, categoria, gravidade, jogador), || crate::aleatorio::sem_contar(|| escolher_tipo_de_lesao(lesoes, categoria, gravidade, jogador)));
    PLACAR.registrar(igual && r == n, || format!("escolher_tipo_de_lesao({}, {}) original={} rust={} sorteio igual={}", categoria, gravidade, r, n, igual));
    r
}

unsafe extern "thiscall" fn comparar_categoria_da_noticia(noticia: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(507);
    let t = *(noticia as *const i32);
    if !(1..=0x1790).contains(&t) {
        return o(noticia);
    }
    let r = o(noticia);
    let n = categoria_da_noticia(noticia);
    PLACAR.registrar(r == n, || format!("categoria_da_noticia({}) original={:x} rust={:x}", t, r, n));
    r
}

// ---------- 210a leva: as convocações repetidas das seleções, a vaga do escalado e o comentarista ex-internacional (national_teams.cpp, tactics.cpp, media.cpp) ----------

/// As 50 vagas de convocação (dwords desalinhados em +0xd7) das duas seleções da nação: a principal
/// (0x53b3d0) e a B (0x53b4f0), nessa ordem.
unsafe fn convocacoes_das_selecoes(nacao: usize) -> [usize; 2] {
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let b: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b4f0usize);
    [principal(nacao).wrapping_add(0xd7), b(nacao).wrapping_add(0xd7)]
}

/// { 0x76c060 } (stdcall, ret 4, national_teams.cpp): apaga as convocações repetidas das duas
/// seleções da nação: em cada uma, todo id não nulo zera as vagas seguintes com o mesmo id.
unsafe extern "stdcall" fn apagar_repetidos_das_convocacoes(nacao: usize) {
    for vagas in convocacoes_das_selecoes(nacao) {
        let vaga = |i: usize| (vagas + 4 * i) as *mut u32;
        for i in 0..50 {
            let v = core::ptr::read_unaligned(vaga(i));
            if v == 0 {
                continue;
            }
            for j in i + 1..50 {
                if core::ptr::read_unaligned(vaga(j)) == v {
                    core::ptr::write_unaligned(vaga(j), 0);
                }
            }
        }
    }
}

/// { 0x8972c0 } (thiscall, ret 0xc, tactics.cpp): a vaga (0..19) da escalação do clube em que está a
/// pessoa: linha tática do clube (0x8982b0 com `time`; -1 devolve 0) e as 20 vagas (dwords em +0x601 +
/// linha x 0x91 + 2) comparadas com o id; não está: 0xff. Clube ou pessoa nulos: a original (caixa
/// de erro).
unsafe extern "thiscall" fn vaga_do_escalado(tatica: usize, clube: usize, pessoa: *const i32, time: u32) -> u8 {
    if clube == 0 || pessoa.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32) -> u8>(509)(tatica, clube, pessoa, time);
    }
    let linha: unsafe extern "stdcall" fn(usize, u32) -> i32 = core::mem::transmute(0x8982b0usize);
    let k = linha(clube, time);
    if k == -1 {
        return 0;
    }
    let id = *pessoa;
    let vagas = core::ptr::read_unaligned((tatica + 0x601) as *const usize).wrapping_add((k as usize).wrapping_mul(0x91) + 2);
    match (0..20).find(|&i| core::ptr::read_unaligned((vagas + 4 * i) as *const i32) == id) {
        Some(i) => i as u8,
        None => 0xff,
    }
}

/// { 0x72b810 } (thiscall, ret 8, media.cpp; `this` não é lido): sorteia o comentarista
/// ex-internacional para o `clube` (ou para falar de `assunto`). 150 rodadas; em cada uma sorteia
/// pessoas (0x90cfc0 com [0xae2364], 110 bytes em [0xae23bc]) até achar uma com idade (+0x18) ≥ 37,
/// jogos pela seleção (+0x22) ≥ 50, ficha (+0x69), sem clube (+0x39) se tiver contrato (+0x61) e que
/// não seja o técnico do clube (+0xcf); a 100ª tentativa encerra a rodada. O peso é um sorteio de
/// reputação da nação (+0x1a, palavra +0x8e) × jogos, dobrado se for da nação do assunto (sem
/// assunto: tem de ser da nação do clube, +0x53). De outra nação: a reputação tem de vencer 6500 +
/// sorteio de 100, a ficha +0xc ≥ 6000 e o assunto não pode desgostar dele (0x547840). Fica o de
/// maior peso (o primeiro aceito sempre entra). Sem clube e sem assunto: 0.
unsafe extern "thiscall" fn escolher_comentarista(_this: usize, clube: usize, assunto: usize) -> usize {
    if assunto == 0 && clube == 0 {
        return 0;
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let reputacao = |nacao: usize| core::ptr::read_unaligned((nacao + 0x8e) as *const i16) as i32;
    let (mut melhor, mut maior) = (0usize, 0i32);
    for _ in 0..150 {
        let mut tentativas = 0;
        let p = loop {
            let r = sorteio(*(0xae2364 as *const i32));
            tentativas += 1;
            if tentativas >= 100 {
                break 0;
            }
            let p = (*(0xae23bc as *const usize)).wrapping_add((r as usize).wrapping_mul(110));
            if *((p + 0x18) as *const i8) < 0x25 || *((p + 0x22) as *const u8) < 0x32 || u(p + 0x69) == 0 {
                continue;
            }
            if u(p + 0x61) != 0 && u(p + 0x39) != 0 {
                continue;
            }
            if u(clube + 0xcf) == p {
                continue;
            }
            break p;
        };
        if p == 0 {
            continue;
        }
        let nacao = u(p + 0x1a);
        let mut peso = sorteio(reputacao(nacao).wrapping_mul(*((p + 0x22) as *const u8) as i32));
        let mesma_nacao = if assunto == 0 {
            if nacao != u(clube + 0x53) {
                continue;
            }
            true
        } else {
            nacao == u(assunto + 0x1a)
        };
        if mesma_nacao {
            peso = peso.wrapping_add(peso);
        } else {
            let corte = sorteio(100).wrapping_add(0x1964);
            if reputacao(nacao) < corte
                || core::ptr::read_unaligned((u(p + 0x69) + 0xc) as *const i16) < 0x1770
                || nao_gosta(assunto, p) != 0
            {
                continue;
            }
        }
        if melhor == 0 || peso > maior {
            maior = peso;
            melhor = p;
        }
    }
    melhor
}

unsafe extern "stdcall" fn comparar_apagar_repetidos_das_convocacoes(nacao: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(508);
    if nacao == 0 {
        return o(nacao);
    }
    let [a, b] = convocacoes_das_selecoes(nacao);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(a, 200), (b, 200)], || o(nacao), || apagar_repetidos_das_convocacoes(nacao));
    PLACAR.registrar(igual, || format!("apagar_repetidos_das_convocacoes {:x?} x {:x?}", mo, mr));
}

unsafe extern "thiscall" fn comparar_vaga_do_escalado(tatica: usize, clube: usize, pessoa: *const i32, time: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, *const i32, u32) -> u8>(509)(tatica, clube, pessoa, time);
    if clube == 0 || pessoa.is_null() {
        return r;
    }
    let n = vaga_do_escalado(tatica, clube, pessoa, time);
    PLACAR.registrar(r == n, || format!("vaga_do_escalado(time {}) original={} rust={}", time, r, n));
    r
}

unsafe extern "thiscall" fn comparar_escolher_comentarista(this: usize, clube: usize, assunto: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(510);
    if assunto == 0 && clube == 0 {
        return o(this, clube, assunto);
    }
    // o comentarista sai só do sorteio
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO, || o(this, clube, assunto),
        || crate::aleatorio::sem_contar(|| escolher_comentarista(this, clube, assunto)));
    PLACAR.registrar(igual && r == n, || format!("escolher_comentarista {:#x} x {:#x} {:x?} x {:x?}", r, n, mo, mr));
    r
}

// ---------- 210a leva: as regras da rodada no jogo da copa, a chave do jogo e o país da excursão (Cup.cpp, friendly.cpp) ----------

/// A rodada `r` da copa: registro de 104 bytes em [copa+0xa3].
unsafe fn rodada_da_copa(copa: usize, r: i32) -> usize {
    (*((copa + 0xa3) as *const usize) as isize + r as isize * 104) as usize
}

/// A linha da chave `k` da rodada `r` da copa: [[copa+0xad] + 4·r] + 53·k.
unsafe fn linha_da_chave(copa: usize, r: i32, k: i32) -> usize {
    (*((*((copa + 0xad) as *const usize) as isize + r as isize * 4) as *const usize) as isize + k as isize * 53) as usize
}

/// O registro de 6 bytes da vaga `v` da copa: [copa+0xa7] + 6·v.
unsafe fn vaga_da_copa(copa: usize, v: i16) -> usize {
    (*((copa + 0xa7) as *const usize) as isize + v as isize * 6) as usize
}

/// { 0x521d60 } (thiscall, ret 8, Cup.cpp): passa as regras da rodada do jogo (palavra +0xf da rodada
/// [jogo+0x34]) para as marcas do jogo (+0x38, OU). Com a marca 0x800 (decide quem passa no empate), o
/// nível (byte +4 da vaga, com sinal) dos dois lados da chave `chave` decide: mandante menor tira 0x800 e
/// põe 0x1000; maior mantém 0x800; igual tira 0x800 e põe 1.
unsafe extern "thiscall" fn regras_da_rodada_no_jogo(copa: usize, jogo: usize, chave: i32) {
    let r = *((jogo + 0x34) as *const i16) as i32;
    let marcas = (jogo + 0x38) as *mut u16;
    let m = core::ptr::read_unaligned(marcas) | core::ptr::read_unaligned((rodada_da_copa(copa, r) + 0xf) as *const u16);
    core::ptr::write_unaligned(marcas, m);
    if m & 0x800 == 0 {
        return;
    }
    let linha = linha_da_chave(copa, r, chave);
    let nivel = |o: usize| *((vaga_da_copa(copa, core::ptr::read_unaligned((linha + o) as *const i16)) + 4) as *const i8);
    let (a, b) = (nivel(0), nivel(2));
    let m = if a < b { (m ^ 0x800) | 0x1000 } else if a > b { m | 0x800 } else { (m ^ 0x800) | 1 };
    core::ptr::write_unaligned(marcas, m);
}

/// { 0x519c90 } (thiscall, ret 4, Cup.cpp): a chave da rodada do jogo (+0x34) em que está o clube
/// mandante do jogo (+0xc), olhando os dois lados de cada chave (53 bytes; vaga negativa é vazia); -1 se
/// nenhuma. O número de chaves é a palavra +0x1a da rodada. Jogo nulo: a original (caixa de erro).
unsafe extern "thiscall" fn chave_do_jogo_na_copa(copa: usize, jogo: usize) -> i32 {
    if jogo == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> i32>(512)(copa, jogo);
    }
    let r = *((jogo + 0x34) as *const i16) as i32;
    let n = core::ptr::read_unaligned((rodada_da_copa(copa, r) + 0x1a) as *const i16) as i32;
    let mandante = *((jogo + 0xc) as *const usize);
    for i in 0..n {
        let linha = linha_da_chave(copa, r, i);
        for o in [0, 2] {
            let v = core::ptr::read_unaligned((linha + o) as *const i16);
            if v >= 0 && *(core::ptr::read_unaligned(vaga_da_copa(copa, v) as *const usize) as *const usize) == mandante {
                return i;
            }
        }
    }
    -1
}

/// { 0x5c4680 } (thiscall, ret 4, friendly.cpp): sorteia o país da excursão do clube. Pela reputação
/// (+0x80) o alcance: ≥ 8000 qualquer país, ≥ 6000 e ≥ 4000 o mesmo continente (+0x71), ≥ 2000 o mesmo
/// continente e a mesma região (+0x75); abaixo, nenhum (0). Candidatos: os outros países (lista
/// [0xae23a8], 0x122 bytes, [0xae2350] países) com +0x86 ≥ 20, +0x7e = 1, +0x84 ≤ 2 e +0x8e ≥ 5; cada um
/// tira 0x90cfc0(100) e fica o maior (estrito, comparado como byte com sinal).
unsafe extern "thiscall" fn pais_da_excursao(_this: usize, clube: usize) -> usize {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let proprio = *((clube + 0x53) as *const usize);
    let alcance: i8 = match *((clube + 0x80) as *const i16) {
        8000.. => 5,
        6000.. => 4,
        4000.. => 3,
        2000.. => 2,
        _ => return 0,
    };
    let mut escolhido = 0usize;
    let mut melhor: i8 = -1;
    if *(0xae2350 as *const i32) <= 0 {
        return 0;
    }
    let mut p = *(0xae23a8 as *const usize);
    let mut i: i16 = 0;
    loop {
        let candidato = p != proprio
            && *((p + 0x86) as *const i16) >= 0x14
            && *((p + 0x7e) as *const u8) == 1
            && *((p + 0x84) as *const i8) <= 2
            && *((p + 0x8e) as *const i16) >= 5;
        if candidato {
            let continente = || *((p + 0x71) as *const u32) == *((proprio + 0x71) as *const u32);
            let perto = match alcance {
                2 => *((p + 0x75) as *const u8) == *((proprio + 0x75) as *const u8) && continente(),
                5.. => true,
                _ => continente(),
            };
            if perto {
                let r = sorteio(100) as i8;
                if r > melhor {
                    melhor = r;
                    escolhido = p;
                }
            }
        }
        i = i.wrapping_add(1);
        p += 0x122;
        if i as i32 >= *(0xae2350 as *const i32) {
            return escolhido;
        }
    }
}

unsafe extern "thiscall" fn comparar_regras_da_rodada_no_jogo(copa: usize, jogo: usize, chave: i32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32)>(511);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(jogo + 0x38, 2)], || o(copa, jogo, chave),
        || regras_da_rodada_no_jogo(copa, jogo, chave));
    PLACAR.registrar(igual, || format!("regras_da_rodada_no_jogo(chave {}) {:x?} x {:x?}", chave, mo, mr));
}

unsafe extern "thiscall" fn comparar_chave_do_jogo_na_copa(copa: usize, jogo: usize) -> i32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> i32>(512)(copa, jogo);
    if jogo == 0 {
        return r;
    }
    let n = chave_do_jogo_na_copa(copa, jogo);
    PLACAR.registrar(r == n, || format!("chave_do_jogo_na_copa original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_pais_da_excursao(this: usize, clube: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(513);
    let reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube),
        || crate::aleatorio::sem_contar(|| pais_da_excursao(this, clube)));
    PLACAR.registrar(igual && r == n, || format!("pais_da_excursao({:#x}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}
