// ---------- centesima sexagesima leva: estatisticas do jogador (player_stats.cpp) e zagueiros na formacao ----------

/// statistic `qual` 1..22 of the record (bytes, percentages ×100.0f, per-match averages, the two ratings completed with
/// under 20 matches ×0.05f), in x87 like the original
/// Original: `player_statistic` (0x7c2150, player_stats.cpp, stdcall).
///
/// [pt] { 0x7c2150 } (stdcall, ret 0xc, player_stats.cpp): a estatística `qual` do registro `r` (jogos
/// em +0, bytes +2..+0xd, palavras +0xe..+0x1a, +0x1c inteiro), como a original a deixa no x87:
/// 1..12 os bytes; 13 e 16 as porcentagens (+0x12/+0x10 e +0x1a/+0x18, vezes 100.0f); 14, 15, 17 e
/// 21 as médias por jogo; 18 a média de +0xe com menos de 20 jogos completada por `peso` (vezes
/// 0.05f); 20 a nota (2×+9 − +0xa + jogos, completada por 1.5 com menos de 20 jogos; 1.5f sem
/// jogos); 22 o inteiro. Sem divisor: 0.0. `r` nulo, 19 ou fora de 1..22: a original (caixa de erro).
unsafe extern "stdcall" fn estatistica_do_jogador(r: *const u8, qual: i8, peso: f32) -> f64 {
    if r.is_null() || !(1..=22).contains(&qual) || qual == 19 {
        return orig::<unsafe extern "stdcall" fn(*const u8, i8, f32) -> f64>(352)(r, qual, peso);
    }
    let b = |o: usize| *r.add(o) as f64;
    let w = |o: usize| core::ptr::read_unaligned(r.add(o) as *const i16) as f64;
    let jogos = *r;
    let cem = f32_em(0x968960) as f64;
    let vinte_avos = f32_em(0x96d61c) as f64;
    match qual {
        1 => b(0),
        2..=4 => b(qual as usize),
        5..=12 => b(qual as usize + 1),
        13 | 16 => {
            let (n, d) = if qual == 13 { (0x12, 0x10) } else { (0x1a, 0x18) };
            if core::ptr::read_unaligned(r.add(d) as *const i16) > 0 {
                w(n) / w(d) * cem
            } else {
                0.0
            }
        }
        14 | 15 | 17 | 21 => {
            let o = match qual { 14 => 0x14, 15 => 0x16, 17 => 0xe, _ => 0x18 };
            if jogos != 0 { w(o) / jogos as f64 } else { 0.0 }
        }
        18 => {
            if jogos < 20 {
                ((20 - jogos as i32) as f64 * peso as f64 + w(0xe)) * vinte_avos
            } else {
                w(0xe) / jogos as f64
            }
        }
        20 => {
            if jogos == 0 {
                return f32_em(0x96b004) as f64;
            }
            let nota = (*r.add(9) as i32 * 2 - *r.add(10) as i32 + jogos as i32) as f64;
            if jogos < 20 {
                (nota + (20 - jogos as i32) as f64 * f64_em(0x968940)) * vinte_avos
            } else {
                nota / jogos as f64
            }
        }
        _ => core::ptr::read_unaligned(r.add(0x1c) as *const i32) as f64,
    }
}

/// the average of the record's five 4-bit ratings (zero counts 27), 16-bit, divided by 4
/// Original: `ratings_average` (0x7c1950, player_stats.cpp, thiscall).
///
/// [pt] { 0x7c1950 } (thiscall, ret 4, player_stats.cpp): a média das cinco notas de 4 bits do registro
/// `*indice` (0x27 bytes cada, a partir de [this+0x30]): os dois meios de +0x24 e de +0x25 e o meio
/// baixo de +0x26, cada um ×4 (zero conta 27), somados em 16 bits e divididos por 4 (truncando).
/// `indice` nulo: a original (caixa de erro).
unsafe extern "thiscall" fn media_das_notas(this: usize, indice: *const i32) -> i32 {
    if indice.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, *const i32) -> i32>(353)(this, indice);
    }
    let reg = (*indice).wrapping_mul(0x27).wrapping_add(*((this + 0x30) as *const i32)) as usize;
    let nota = |n: u8| if n == 0 { 0x1b } else { n as u16 * 4 };
    let (a, c, e) = (*((reg + 0x24) as *const u8), *((reg + 0x25) as *const u8), *((reg + 0x26) as *const u8));
    let soma = nota(a & 0xf)
        .wrapping_add(nota(a >> 4))
        .wrapping_add(nota(c & 0xf))
        .wrapping_add(nota(c >> 4))
        .wrapping_add(nota(e & 0xf));
    soma as i16 as i32 / 4
}

/// 0 if any slot with an open side is a centre-back's or defensive midfielder's
/// Original: `centre_backs_without_conflict` (0x5b8dc0, formation.cpp, thiscall).
///
/// [pt] { 0x5b8dc0 } (thiscall, formation.cpp): como a 0x5b8960, para os zagueiros: devolve 0 se alguma
/// das 11 vagas com lado aberto (0x880) é de zagueiro (4, na casa +0x12d ou +0x14e) ou de volante
/// (8 na +0x12d; na +0x14e pela 0x5b2ae0); senão 1. Devolve só `al`.
unsafe extern "thiscall" fn zagueiros_sem_conflito(this: usize) -> u8 {
    let casa: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(FN_FORMATION_GET_SLOT_POSITION);
    let w = |o: usize| core::ptr::read_unaligned((this + o) as *const u16);
    for i in 0..=10u32 {
        let a = w(0x12d + 2 * i as usize);
        let b = w(0x14e + 2 * i as usize);
        if a & 0x880 != 0 && a & 4 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && b & 4 != 0 {
            return 0;
        }
        if a & 0x880 != 0 && a & 8 != 0 {
            return 0;
        }
        if b & 0x880 != 0 && casa(this, i, 1) & 8 != 0 {
            return 0;
        }
    }
    1
}

unsafe extern "stdcall" fn comparar_estatistica_do_jogador(r: *const u8, qual: i8, peso: f32) -> f64 {
    let o = orig::<unsafe extern "stdcall" fn(*const u8, i8, f32) -> f64>(352)(r, qual, peso);
    let n = estatistica_do_jogador(r, qual, peso);
    PLACAR.registrar(o.to_bits() == n.to_bits(), || format!("estatistica_do_jogador({:p}, {}, {}) original={} rust={}", r, qual, peso, o, n));
    o
}

comparar_valor!(comparar_media_das_notas, media_das_notas, 353, (indice: *const i32) -> i32);
comparar_valor!(comparar_zagueiros_sem_conflito, zagueiros_sem_conflito, 354, () -> u8);

// ---------- 271a leva: as estatísticas do jogador arquivadas no histórico (a troca de clube, o empréstimo, a aposentadoria) e os jogos e gols pelo clube recontados (player_stats.cpp) ----------


unsafe fn cqh8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// O ensaio do comparador: a cópia roda sem gravar no arquivo de histórico (FILE_LLIST::add_node 0x58e340, que escreve
/// no arquivo e move as posições da ficha); os nós que ela gravaria entram direto na soma dos jogos e gols pelo clube.
static mut CQH8_ENSAIO: bool = false;
static mut CQH8_PENDENTES: (u16, u16) = (0, 0);
static mut CQH8_CLUBE: i32 = -1;

/// O nó do histórico (0x2b bytes montados na pilha): +0 o ano da temporada do clube (0x669950), +2 o clube (−1 sem),
/// +6 os 32 bytes da categoria, +0x26 o valor, +0x2a o tipo (3 bits) e a categoria × 8.
unsafe fn cqh8_no(clube: usize, dados: Option<usize>, valor: u32, marca: u8) -> [u8; 0x2b] {
    let ano: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_KEY_NATION_GET_CLUB_SEASON_YEAR);
    let mut r = [0u8; 0x2b];
    r[0..2].copy_from_slice(&ano(clube).to_le_bytes());
    let id = if clube == 0 { 0xffff_ffffu32 } else { read_u32_as_usize(clube) as u32 };
    r[2..6].copy_from_slice(&id.to_le_bytes());
    if let Some(d) = dados {
        core::ptr::copy_nonoverlapping(d as *const u8, r.as_mut_ptr().add(6), 32);
    }
    r[0x26..0x2a].copy_from_slice(&valor.to_le_bytes());
    r[0x2a] = marca;
    r
}

/// Grava o nó no fim da lista do histórico da ficha (0x58e340 no arquivo [this], posições +0x18/+0x1c do registro);
/// no ensaio, só soma.
unsafe fn cqh8_gravar(this: usize, reg: usize, no: &[u8; 0x2b]) {
    if CQH8_ENSAIO {
        if i32::from_le_bytes([no[2], no[3], no[4], no[5]]) == CQH8_CLUBE && no[0x2a] & 0xf8 == 0x10 {
            CQH8_PENDENTES.0 = CQH8_PENDENTES.0.wrapping_add(no[6] as u16);
            CQH8_PENDENTES.1 = CQH8_PENDENTES.1.wrapping_add(no[8] as u16);
        }
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    f(read_u32_as_usize(this), reg + 0x18, reg + 0x1c, no.as_ptr());
}

/// when the player leaves a club: the stats of each category of the season go to the history file (with the year, the
/// club, the fee and the kind of exit) and are cleared; an empty node when the last category is missing; and the games
/// and goals for the club recounted from the history
/// Original: `PlayerStats::archive_player_stats` (0x7c5fe0, player_stats.cpp, thiscall).
///
/// [pt] { 0x7c5fe0 } (thiscall, ret 0x18, player_stats.cpp): `PlayerStats::archive_player_stats(pessoa, valor, empréstimo,
/// fim, clube, outro)`, pelas saídas de clube (contratos 0x4c7ac0/0x4cbd20/0x4d01e0/0x4d07e0, 0x690730 o elenco, 0x7c1350
/// as seleções). O tipo do nó: 2 com `empréstimo`, 1 com `fim`, 3 com `outro`, senão 0. Pessoa nula: a caixa (linha
/// 0xf28) e 0; sem ficha: 0. Cada uma das seis categorias com registro (o registro de 39 bytes da ficha em [this+0x30])
/// vai para o histórico (o ano e o clube atuais, os 32 bytes do registro em [this+4], `valor`, a categoria e o tipo) e
/// o registro é zerado. Sem a sexta categoria, um nó vazio da categoria 6 com `valor` quando: `valor` ≥ 0 ou `fim`; ou
/// `clube` ≥ 0 e: fora da temporada da nação do clube (a tabela 0xb63d8a); ou, sem a nação ou na temporada dela, menos de
/// 365 dias desde a entrada no clube (+0x3e/+0x42, 0x54cdb0). Com `clube` ≥ 0 ou `valor` ≥ 0: recontam-se os jogos (+0x20) e os gols (+0x22) pelo
/// `clube` nos nós da categoria 2 do histórico (0x58f120). Devolve 1.
unsafe extern "thiscall" fn arquivar_estatisticas(this: usize, p: usize, valor: u32, emprestimo: u32, fim: u32, clube_id: u32, outro: u32) -> u32 {
    let tipo: u8 = if emprestimo as u8 != 0 {
        2
    } else if fim as u8 != 0 {
        1
    } else if outro != 0 {
        3
    } else {
        0
    };
    if p == 0 {
        aviso_em(0xa78310, 0xf28);
        return 0;
    }
    let clube = read_u32_as_usize(p + 0x39);
    let f = read_u32_as_usize(p + 0x61);
    if f == 0 {
        return 0;
    }
    let reg = read_u32_as_usize(this + 0x30).wrapping_add((cqh8_i(f) as isize * 0x27) as usize);
    let mut sexta = false;
    for cat in 0..6usize {
        let id = cqh8_i(reg + 4 * cat);
        if id == -1 {
            continue;
        }
        let dados = read_u32_as_usize(this + 4).wrapping_add((id as isize * 0x20) as usize);
        let no = cqh8_no(clube, Some(dados), valor, (((cat + 1) as u8) << 3) | tipo); // cqh8: a categoria e o tipo do nó
        crate::comparacao::ramo("cqh_8 categoria arquivada");
        cqh8_gravar(this, reg, &no);
        if cat == 5 {
            sexta = true;
        }
        core::ptr::write_bytes(dados as *mut u8, 0, 32); // cqh8: o registro da categoria zerado
    }
    // sem nação do clube (ou fora da tabela) ou na temporada dela: a marca que pede os 365 dias
    let marca = if clube == 0 || read_u32_as_usize(clube + 0x53) == 0 {
        true
    } else {
        let linha: unsafe extern "C" fn(usize) -> i8 = core::mem::transmute(FN_KEY_NATION_INDEX_OF_NATION);
        let k = linha(read_u32_as_usize(clube + 0x53));
        if !(0..0x23).contains(&k) {
            true
        } else {
            let t = 0xb63d8a + k as usize * 0x48;
            let (dia, ano) = (core::ptr::read_unaligned(0xae2c90 as *const i16), core::ptr::read_unaligned(0xae2c92 as *const u16));
            let (d0, a0) = (core::ptr::read_unaligned(t as *const i16), core::ptr::read_unaligned((t + 2) as *const u16));
            let (d1, a1) = (core::ptr::read_unaligned((t + 0xf) as *const i16), core::ptr::read_unaligned((t + 0x11) as *const u16));
            let depois_do_inicio = (dia > d0 && ano == a0) || ano > a0;
            let antes_do_fim = (dia < d1 && ano == a1) || ano < a1;
            depois_do_inicio && antes_do_fim
        }
    };
    if !sexta {
        let gravar = if (valor as i32) > -1 || fim as u8 != 0 {
            true
        } else if (clube_id as i32) <= -1 {
            false
        } else if !marca {
            true
        } else {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
            dias(0xae2c90, read_u32_as_usize(p + 0x3e) as u32, read_u32_as_usize(p + 0x42) as u32) < 0x16d
        };
        if gravar {
            let no = cqh8_no(clube, None, valor, 0x30 | tipo); // cqh8: o nó vazio da sexta categoria
            crate::comparacao::ramo("cqh_8 no vazio");
            cqh8_gravar(this, reg, &no);
        }
    }
    if (clube_id as i32) > -1 || (valor as i32) > -1 {
        core::ptr::write_unaligned((reg + 0x20) as *mut u16, 0);
        core::ptr::write_unaligned((reg + 0x22) as *mut u16, 0);
        let nos: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_FILE_LLIST_GET_NODES);
        let lista = nos(read_u32_as_usize(this), read_u32_as_usize(reg + 0x18) as u32, read_u32_as_usize(reg + 0x1c) as u32);
        if lista != 0 {
            core::ptr::write_unaligned((lista + 0xa) as *mut u32, read_u32_as_usize(lista + 2) as u32);
            let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
            let mut no = proximo(lista);
            while no != 0 {
                if read_u32_as_usize(no + 2) as u32 == clube_id && *((no + 0x2a) as *const u8) & 0xf8 == 0x10 {
                    crate::comparacao::ramo("cqh_8 jogos pelo clube");
                    let a = (reg + 0x20) as *mut u16;
                    core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(*((no + 6) as *const u8) as u16)); // cqh8: os jogos pelo clube
                    let g = (reg + 0x22) as *mut u16;
                    core::ptr::write_unaligned(g, core::ptr::read_unaligned(g).wrapping_add(*((no + 8) as *const u8) as u16));
                }
                no = proximo(lista);
            }
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
            let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
            destruir(lista);
            apagar(lista);
        }
        if CQH8_ENSAIO {
            let a = (reg + 0x20) as *mut u16;
            core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(CQH8_PENDENTES.0));
            let g = (reg + 0x22) as *mut u16;
            core::ptr::write_unaligned(g, core::ptr::read_unaligned(g).wrapping_add(CQH8_PENDENTES.1));
        }
    }
    1
}

/// Comparador em ensaio (modelo CPV6_ENSAIO): a cópia roda primeiro sem gravar no arquivo (os nós que gravaria entram na
/// soma dos jogos e gols), depois a original a partir do mesmo estado; compara a devolução, os seis registros de 32 bytes
/// das categorias e os jogos e gols pelo clube (+0x20/+0x22 do registro da ficha). As posições do histórico (+0x18,
/// +0x1c), que só a gravação da original move, ficam fora. Pessoa nula ou sem ficha: só a original.
unsafe extern "thiscall" fn comparar_arquivar_estatisticas(this: usize, p: usize, valor: u32, emprestimo: u32, fim: u32, clube_id: u32, outro: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) -> u32>(1642);
    if p == 0 || read_u32_as_usize(p + 0x61) == 0 {
        return o(this, p, valor, emprestimo, fim, clube_id, outro);
    }
    let reg = read_u32_as_usize(this + 0x30).wrapping_add((cqh8_i(read_u32_as_usize(p + 0x61)) as isize * 0x27) as usize);
    let mut regioes = vec![(reg + 0x20, 4)];
    for cat in 0..6usize {
        let id = cqh8_i(reg + 4 * cat);
        if id != -1 {
            regioes.push((read_u32_as_usize(this + 4).wrapping_add((id as isize * 0x20) as usize), 32));
        }
    }
    let antes = crate::comparacao::Memoria::ler(&regioes);
    CQH8_ENSAIO = true;
    CQH8_PENDENTES = (0, 0);
    CQH8_CLUBE = clube_id as i32;
    let n = crate::aleatorio::pausar_rastro(|| arquivar_estatisticas(this, p, valor, emprestimo, fim, clube_id, outro));
    CQH8_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&regioes);
    antes.escrever();
    let r = o(this, p, valor, emprestimo, fim, clube_id, outro);
    let mo = crate::comparacao::Memoria::ler(&regioes);
    PLACAR.registrar(r == n && mo == mr, || format!("arquivar_estatisticas(pessoa {:x}, valor {}, clube {}) original={} rust={} {:x?} x {:x?}",
        p, valor as i32, clube_id as i32, r, n, mo.0, mr.0));
    r
}

// ---------- 270a leva: o histórico da carreira do jogador (a tabela das temporadas por clube montada do arquivo de histórico) e os anos desde que jogou num clube (player_stats.cpp) ----------


unsafe fn cqf8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}





/// O tamanho de uma entrada da tabela (uma categoria de uma temporada num clube): +0 o ano, +2 o id do clube (−1
/// nenhum), +6..+0x26 os 32 bytes da categoria (+6 jogos, +7 ..., +0x14 a palavra), +0x26 o valor da transferência
/// (−1 nenhum), +0x2a categoria × 8 e o tipo nos 3 bits de baixo. Seis entradas (categorias 1..6) por temporada.
const CQF8_E: usize = 0x2b;

/// A entrada montada na pilha pela original (0x2b bytes, zerada antes de cada uma).
struct Cqf8Reg([u8; CQF8_E]);

impl Cqf8Reg {
    fn nova(ano: u16, clube: u32, cat: u8, transf: u32) -> Cqf8Reg {
        let mut r = [0u8; CQF8_E];
        r[0..2].copy_from_slice(&ano.to_le_bytes());
        r[2..6].copy_from_slice(&clube.to_le_bytes());
        r[0x26..0x2a].copy_from_slice(&transf.to_le_bytes());
        r[0x2a] = cat << 3;
        Cqf8Reg(r)
    }
    unsafe fn gravar(&self, a: usize) {
        core::ptr::copy_nonoverlapping(self.0.as_ptr(), a as *mut u8, CQF8_E);
    }
}

/// A caixa "Error" de player_stats.cpp (0xa78310) na linha `linha`.
unsafe fn cqf8_erro(linha: u32) {
    aviso_em(0xa78310, linha);
}

/// A entrada `k` (contando de 1) a partir da conta `n`: buf + (n + k) × 0x2b − 0x2b.
fn cqf8_lugar(buf: usize, n: i32, k: i32) -> usize {
    buf.wrapping_add(((n + k) as isize * CQF8_E as isize - CQF8_E as isize) as usize)
}

/// the player career table, six entries (categories) per season and club: the current season, the ones from the history
/// file from newest to oldest with the empty years filled, the repeated entries of the same season and club summed and
/// the empty groups removed
/// Original: `PlayerStats::build_career_history` (0x7c3e40, player_stats.cpp, thiscall).
///
/// [pt] { 0x7c3e40 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::build_career_history(pessoa, &conta)`: a tabela
/// (malloc 0x944e46 de 500 entradas, realloc 0x945667 de 500 em 500; quem chama libera com 0x9452ca) com seis entradas
/// por temporada e clube, e a conta em `*conta`. Pessoa nula (linha 0xb98) ou sem ficha (0xbaa): a caixa e 0; id a
/// partir de [0xae2364]: 0; sem memória: a caixa (0xbb2, 0xc00, 0xc26, 0xc41) e 0.
/// (1) A temporada atual: as seis categorias com o ano da temporada do clube (0x669950), o clube (+0x39; −1 sem ele), a
/// transferência −1 e os 32 bytes da categoria (0x7c3920) quando há. (2) O histórico do arquivo (FILE_LLIST::get_nodes
/// 0x58f120 do [this] pelas posições +0x18/+0x1c do registro de 0x27 bytes da ficha em [this+0x30]), do fim para o
/// começo (0x68d1d0): cada nó de outra temporada, clube ou tipo abre um grupo: a conta anda 6; anos vazios entre os dois
/// (até a conta passar de 500) viram grupos vazios (clube −1, transferência −1); o grupo do nó leva o ano, o clube, a
/// transferência e o tipo do nó; o nó vai para a entrada da sua categoria (+0x2a / 8) no grupo. A lista é destruída
/// (0x68d040 e 0x944c94). (3) A conta anda 6; cada entrada leva a transferência e o tipo da entrada seis adiante (as
/// do último grupo, −1 e 0). (4) Um grupo com um seguinte do mesmo ano e clube, sem transferência nos dois, tipo do
/// seguinte diferente de 1 e a palavra +0x14 diferente de −1 nos dois, soma os bytes +6..+0x13 e a palavra +0x14 no
/// seguinte e some (ano 0). (5) O grupo sem jogos (+6, +7 da primeira e da última entrada), com ano, sem transferência
/// e de tipo 0 some quando sem clube ou quando outro grupo tem o ano dele e o grupo seguinte é do mesmo clube; o de
/// jogos com a última entrada sem clube (+0xd9) também some. (6) Os grupos de ano 0 saem da tabela (os seguintes
/// descem) e a conta cai 6 por grupo.
unsafe extern "thiscall" fn historico_da_carreira(this: usize, p: usize, conta: *mut i32) -> usize {
    let malloc: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let realloc: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_REALLOC);
    let ano_do_clube: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(FN_KEY_NATION_GET_CLUB_SEASON_YEAR);
    let categoria: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_PLAYERSTATS_GET_CATEGORY_RECORD);
    *conta = 0;
    if p == 0 {
        cqf8_erro(0xb98);
        return 0;
    }
    if cqf8_i(p) >= cqf8_i(0xae2364) {
        return 0;
    }
    let f = read_u32_as_usize(p + 0x61);
    if f == 0 {
        cqf8_erro(0xbaa);
        return 0;
    }
    let mut buf = malloc(0x53fc);
    if buf == 0 {
        cqf8_erro(0xbb2);
        return 0;
    }
    let mut cap: i32 = 500;
    let clube = read_u32_as_usize(p + 0x39);
    let mut ultimo = Cqf8Reg::nova(0, 0, 0, 0);
    for cat in 1..=6u8 {
        let ano = ano_do_clube(clube);
        let id = if clube == 0 { 0xffff_ffff } else { read_u32_as_usize(clube) as u32 }; // cqf8: o clube da temporada atual
        crate::comparacao::ramo("cqf_8 temporada atual");
        let mut r = Cqf8Reg::nova(ano, id, cat, 0xffff_ffff);
        let c = categoria(this, f, cat as u32);
        if c != 0 {
            core::ptr::copy_nonoverlapping(c as *const u8, r.0.as_mut_ptr().add(6), 32);
        }
        r.gravar(cqf8_lugar(buf, *conta, cat as i32));
        ultimo = r;
    }
    // o ano, o clube e o tipo do grupo aberto (o tipo começa em 0xff: o primeiro nó sempre abre grupo)
    let mut ano_atual = u16::from_le_bytes([ultimo.0[0], ultimo.0[1]]);
    let mut clube_atual = u32::from_le_bytes([ultimo.0[2], ultimo.0[3], ultimo.0[4], ultimo.0[5]]);
    let mut tipo_atual: u8 = 0xff;
    let reg_ficha = read_u32_as_usize(this + 0x30).wrapping_add((cqf8_i(f) as isize * 0x27) as usize);
    let nos: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_FILE_LLIST_GET_NODES);
    let lista = nos(read_u32_as_usize(this), read_u32_as_usize(reg_ficha + 0x18) as u32, read_u32_as_usize(reg_ficha + 0x1c) as u32);
    if lista != 0 {
        let anterior: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_PREV);
        write_u32(lista + 0xa, read_u32_as_usize(lista + 6) as u32);
        let mut no = anterior(lista);
        while no != 0 {
            let tipo = read_u8(no + 0x2a);
            if !(read_u16(no) == ano_atual && read_u32_as_usize(no + 2) as u32 == clube_atual && tipo & 7 == tipo_atual) {
                *conta += 6;
                ano_atual = ano_atual.wrapping_sub(1);
                if ano_atual > read_u16(no) {
                    loop {
                        let mut tam = cap as usize * CQF8_E;
                        for cat in 1..=6u8 {
                            let r = Cqf8Reg::nova(ano_atual, 0xffff_ffff, cat, 0xffff_ffff);
                            if *conta + cat as i32 - 1 >= cap {
                                tam += 0x53fc;
                                cap += 500;
                                buf = realloc(buf, tam);
                                if buf == 0 {
                                    cqf8_erro(0xc00);
                                    return 0;
                                }
                            }
                            r.gravar(cqf8_lugar(buf, *conta, cat as i32));
                        }
                        *conta += 6;
                        if *conta > 500 {
                            break;
                        }
                        ano_atual = ano_atual.wrapping_sub(1);
                        if ano_atual <= read_u16(no) {
                            break;
                        }
                    }
                }
                let mut tam = cap as usize * CQF8_E;
                let ano_no = read_u16(no); // cqf8: o ano do grupo do nó
                let clube_no = read_u32_as_usize(no + 2) as u32; // cqf8: o clube do grupo do nó
                for cat in 1..=6u8 {
                    let mut r = Cqf8Reg::nova(ano_no, clube_no, cat, read_u32_as_usize(no + 0x26) as u32);
                    r.0[0x2a] = (cat << 3) | (tipo & 7);
                    crate::comparacao::ramo("cqf_8 temporada do historico");
                    if *conta + cat as i32 - 1 >= cap {
                        tam += 0x53fc;
                        cap += 500;
                        buf = realloc(buf, tam);
                        if buf == 0 {
                            cqf8_erro(0xc26);
                            return 0;
                        }
                    }
                    r.gravar(cqf8_lugar(buf, *conta, cat as i32));
                }
                ano_atual = read_u16(no);
                clube_atual = read_u32_as_usize(no + 2) as u32;
                tipo_atual = tipo & 7;
            }
            if (tipo >> 3) as i32 - 1 + *conta >= cap {
                cap += 500;
                buf = realloc(buf, cap as usize * CQF8_E);
                if buf == 0 {
                    cqf8_erro(0xc41);
                    return 0;
                }
            }
            core::ptr::copy_nonoverlapping(no as *const u8, cqf8_lugar(buf, *conta, (tipo >> 3) as i32) as *mut u8, CQF8_E);
            no = anterior(lista);
        }
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
        let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
        destruir(lista);
        apagar(lista);
    }
    *conta += 6;
    // (3) a transferência e o tipo descem do grupo seguinte
    let n = *conta;
    for i in 0..n {
        let e = buf + i as usize * CQF8_E;
        if i + 6 < n {
            write_u32(e + 0x26, read_u32_as_usize(e + 6 * CQF8_E + 0x26) as u32);
            *((e + 0x2a) as *mut u8) = (read_u8(e + 0x2a) & 0xf8) | (read_u8(e + 6 * CQF8_E + 0x2a) & 7);
        } else {
            write_u32(e + 0x26, 0xffff_ffff);
            *((e + 0x2a) as *mut u8) = read_u8(e + 0x2a) & 0xf8;
        }
    }
    // (4) as entradas iguais do grupo seguinte somam e a de cima some
    let mut k = 6i32;
    while k - 6 < *conta {
        let ei = buf + (k - 6) as usize * CQF8_E;
        let mut j = k;
        while j < *conta {
            let ej = buf + j as usize * CQF8_E;
            if read_u16(ei) == read_u16(ej)
                && read_u8(ej + 0x2a) & 7 != 1
                && cqf8_i(ej + 0x26) == -1
                && cqf8_i(ei + 0x26) == -1
                && read_u32_as_usize(ei + 2) == read_u32_as_usize(ej + 2)
                && read_u16(ei + 0x14) != 0xffff
                && read_u16(ej + 0x14) != 0xffff
            {
                for o in 6..0x14 {
                    *((ej + o) as *mut u8) = read_u8(ej + o).wrapping_add(read_u8(ei + o)); // cqf8: soma da entrada repetida
                }
                write_u16(ej + 0x14, read_u16(ej + 0x14).wrapping_add(read_u16(ei + 0x14)));
                if cqf8_i(ei + 0x26) != -1 || (read_u8(ei + 0x2a) & 7 == 1 && cqf8_i(ej + 0x26) == -1) {
                    write_u32(ej + 0x26, read_u32_as_usize(ei + 0x26) as u32);
                    *((ej + 0x2a) as *mut u8) = (read_u8(ej + 0x2a) & 0xf8) | (read_u8(ei + 0x2a) & 7);
                }
                write_u16(ei, 0);
                break;
            }
            j += 6;
        }
        k += 1;
    }
    // (5) os grupos vazios somem (ano 0)
    let mut g = 0i32;
    while g < *conta {
        let e = buf + g as usize * CQF8_E;
        let zerar = if read_u8(e + 6) == 0
            && read_u8(e + 7) == 0
            && read_u8(e + 0xdd) == 0
            && read_u8(e + 0xde) == 0
            && read_u16(e) != 0
            && cqf8_i(e + 0x26) == -1
            && read_u8(e + 0x2a) & 7 == 0
        {
            let mut z = false;
            let mut j = 0i32;
            while j < *conta {
                if g != j {
                    let c = cqf8_i(e + 2);
                    if c == -1 {
                        z = true;
                        break;
                    }
                    let ej = buf + (j / 6) as usize * 0x102;
                    if read_u16(ej) == read_u16(e) && g + 6 < *conta && cqf8_i(e + 0x104) == c {
                        z = true;
                        break;
                    }
                }
                j += 6;
            }
            z
        } else {
            cqf8_i(e + 0xd9) == -1
        };
        if zerar {
            for t in 0..6 {
                write_u16(e + t * CQF8_E, 0); // cqf8: o grupo some
            }
        }
        g += 6;
    }
    // (6) os grupos de ano 0 saem e os seguintes descem
    let mut i = 0i32;
    while i < *conta {
        let e = buf + (i / 6) as usize * 0x102;
        if read_u16(e) == 0 {
            let mut d = i + 6;
            let mut a = e + 0x102;
            while d < *conta {
                core::ptr::copy(a as *const u8, (a - 0x102) as *mut u8, 0x102);
                d += 6;
                a += 0x102;
            }
            *conta -= 6;
            i -= 6;
        }
        i += 6;
    }
    buf
}

/// Comparador: as duas montam a tabela (a original e a cópia, a partir do mesmo estado; o arquivo de histórico só é
/// lido) e comparam a conta e as entradas; a da cópia é liberada (0x9452ca) e o jogo segue com a da original. Os casos
/// de erro (caixa): só a original.
unsafe extern "thiscall" fn comparar_historico_da_carreira(this: usize, p: usize, conta: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize>(1615);
    if p == 0 || (cqf8_i(p) < cqf8_i(0xae2364) && read_u32_as_usize(p + 0x61) == 0) {
        return o(this, p, conta);
    }
    let r = o(this, p, conta);
    let mut n = 0i32;
    let t = historico_da_carreira(this, p, &mut n);
    let igual = if r == 0 || t == 0 {
        r == t && *conta == n
    } else {
        *conta == n
            && core::slice::from_raw_parts(r as *const u8, n.max(0) as usize * CQF8_E)
                == core::slice::from_raw_parts(t as *const u8, n.max(0) as usize * CQF8_E)
    };
    PLACAR.registrar(igual, || {
        let k = (*conta).max(n).max(0) as usize;
        let dif = (0..k).find(|&i| {
            r == 0 || t == 0 || core::slice::from_raw_parts((r + i * CQF8_E) as *const u8, CQF8_E) != core::slice::from_raw_parts((t + i * CQF8_E) as *const u8, CQF8_E)
        });
        format!("historico_da_carreira(pessoa {:x}) conta original={} rust={}, primeira entrada diferente {:?}", p, *conta, n, dif)
    });
    if t != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        liberar(t);
    }
    r
}

/// how many years since the person played for the club: 0 when he is there, −1 when never
/// Original: `player_stats_years_since_at_club` (0x7c6e30, player_stats.cpp, stdcall).
///
/// [pt] { 0x7c6e30 } (stdcall, ret 8, player_stats.cpp): `player_stats_years_since_at_club(pessoa, clube)`: quantos anos
/// desde que a pessoa jogou pelo clube: 0 se está nele (+0x39), −1 se nunca; senão o ano de hoje (0xae2c92) menos o
/// maior ano dos grupos do histórico (0x7c3e40 em [0xb74350]; a última entrada de cada grupo, +0xd7 o ano, +0xd9 o
/// clube) com o clube. Pessoa nula (linha 0x133f), clube nulo (0x1347) ou sem ficha (0x135f): a caixa e −1; id a partir
/// de [0xae2364]: −1. Quem chama a põe com o ecx do PlayerStats, que ela não usa. Pela 0x5b03e0 (não roda na carreira
/// automática: passageira).
unsafe extern "stdcall" fn anos_desde_o_clube(p: usize, clube: usize) -> i16 {
    if p == 0 {
        cqf8_erro(0x133f);
        return -1;
    }
    if clube == 0 {
        cqf8_erro(0x1347);
        return -1;
    }
    if cqf8_i(p) >= cqf8_i(0xae2364) {
        return -1;
    }
    if read_u32_as_usize(p + 0x39) == clube {
        return 0;
    }
    if read_u32_as_usize(p + 0x61) == 0 {
        cqf8_erro(0x135f);
        return -1;
    }
    let montar: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(FN_PLAYERSTATS_BUILD_CAREER_HISTORY);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut n = 0i32;
    let t = montar(read_u32_as_usize(0xb74350), p, &mut n);
    let mut maior = 0x7b3i32;
    if t != 0 {
        let mut i = 0i32;
        let mut e = t + 0xd7;
        while i < n {
            let c = cqf8_i(e + 2);
            if c >= 0 && c == cqf8_i(clube) && read_u16(e) as i32 > maior {
                maior = read_u16(e) as i32;
            }
            i += 6;
            e += 0x102;
        }
    }
    liberar(t);
    if maior == 0x7b3 {
        return -1;
    }
    read_u16(0xae2c92).wrapping_sub(maior as u16) as i16
}

/// Comparador: a devolução (16 bits); a tabela é montada e liberada pelas duas. Os casos de erro: só a original.
unsafe extern "stdcall" fn comparar_anos_desde_o_clube(p: usize, clube: usize) -> i16 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize) -> i16>(1616);
    if p == 0 || clube == 0 || (cqf8_i(p) < cqf8_i(0xae2364) && read_u32_as_usize(p + 0x39) != clube && read_u32_as_usize(p + 0x61) == 0) {
        return o(p, clube);
    }
    let r = o(p, clube);
    let n = anos_desde_o_clube(p, clube);
    PLACAR.registrar(r == n, || format!("anos_desde_o_clube(pessoa {:x}, clube {:x}) original={} rust={}", p, clube, r, n));
    r
}

/// the qsort order of the history records: by person, by year and by the first field
/// Original: `player_stats_compare_history_by_staff` (0x7c6de0, player_stats.cpp, cdecl).
///
/// [pt] { 0x7c6de0 } (cdecl, player_stats.cpp): `player_stats_compare_history_by_staff(a, b)`, a ordem do qsort dos
/// registros do staff_history.dat na criação das estatísticas (0x7bfd50, ponteiro em 0x7c05ad): `a` nulo 1, `b` nulo −1;
/// pela pessoa (+4, sem sinal), depois pelo ano (+8, palavra com sinal), depois a diferença do primeiro dword.
unsafe extern "C" fn ordem_do_historico_por_pessoa(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (pa, pb) = (read_u32_as_usize(a + 4) as u32, read_u32_as_usize(b + 4) as u32);
    if pa > pb {
        return 1;
    }
    if pa < pb {
        return -1;
    }
    let (aa, ab) = (read_u16(a + 8) as i16, read_u16(b + 8) as i16);
    if aa > ab {
        return 1;
    }
    if aa != ab {
        return -1;
    }
    cqf8_i(a).wrapping_sub(cqf8_i(b))
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_ordem_do_historico_por_pessoa(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1617)(a, b);
    let n = ordem_do_historico_por_pessoa(a, b);
    PLACAR.registrar(r == n, || format!("ordem_do_historico_por_pessoa({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 257a leva: as estatísticas do jogador: a estatística de uma categoria e as notas combinadas, a ordem dos idiomas e a ordem da história por clube (player_stats.cpp) ----------



/// the requested statistic of the category record (without a record, the default for the rating and the average), or
/// the combined ratings of categories 2 to 5
/// Original: `PlayerStats::get_stat` (0x7c1e30, player_stats.cpp, thiscall).
///
/// [pt] { 0x7c1e30 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::get_stat(jogador, categoria, qual, padrão)`, como
/// a original a deixa no x87. Sem jogador: a caixa "Error" (linha 0x659) e 0.0; id (+0) fora da tabela ([0xae236c]):
/// 0.0. Com `qual` ≠ 0x13 e a categoria (byte com sinal) até 6: o registro da categoria (0x7c3920) dá a estatística
/// (0x7c2150 com `qual` e `padrão`); sem registro, `padrão` para 0x12 e 0x14 e 0.0 para as outras. Senão as notas
/// combinadas pelas categorias 2+3 (jogos n e total t de +0xe: n < 20, ((20 − n)·5 + t)·0.05f; senão t/n; guardado em
/// f32), 4 (n < 8, ((8 − n)·3 + t)·0.125; senão t/n; f32) e 5 (n < 5, ((5 − n) + t)·0.2f; senão t/n; no x87): categoria
/// 100, (c5·1.5 + c4·2.5 + c23·6)·0.1; 101, (c5·3 + c4·5 + 2·c23)·0.1; 102, (c5·5 + c4·4 + c23)·0.1; as outras 0.0. Sem
/// o registro 4 ou 5 a original lê locais sem valor (a cópia usa zero; o comparador deixa esses casos com a original).
/// Pela escolha da escalação 0x6e2f20, pelo MATCH_PLAYER::setup 0x6eaea0, pela reputação do clube 0x545e90, pela força
/// do jogador 0x546ab0, pelo dia da comissão 0x5ce2d0 e por outras. Quente: sem contador.
unsafe extern "thiscall" fn estatistica_da_categoria(this: usize, jogador: usize, categoria: u32, qual: u32, padrao: f32) -> f64 {
    if jogador == 0 {
        aviso_em(0xa78310, 0x659);
        return 0.0;
    }
    if read_usize(jogador) as i32 >= read_usize(0xae236c) as i32 {
        return 0.0;
    }
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_PLAYERSTATS_GET_CATEGORY_RECORD);
    if qual as u8 != 0x13 && categoria as u8 as i8 <= 6 {
        let r = registro(this, jogador, categoria);
        if r != 0 {
            let valor: unsafe extern "stdcall" fn(usize, u32, u32) -> f64 = core::mem::transmute(FN_PLAYER_STATS_GET_RECORD_VALUE);
            return valor(r, qual, padrao.to_bits()); // cpu3: o valor do registro
        }
        if qual as u8 == 0x12 || qual as u8 == 0x14 {
            return padrao as f64; // cpu3: sem registro, o padrao
        }
        return 0.0; // cpu3: sem registro, zero
    }
    let (mut n, mut t) = (0u8, 0i16);
    let r2 = registro(this, jogador, 2);
    if r2 != 0 {
        n = *(r2 as *const u8);
        t = read_i16(r2 + 0xe);
    }
    let r3 = registro(this, jogador, 3);
    if r3 != 0 {
        n = n.wrapping_add(*(r3 as *const u8));
        t = t.wrapping_add(read_i16(r3 + 0xe));
    }
    let c23 = if n < 0x14 {
        (((0x14 - n as i32) as f64 * 5.0 + t as f64) * (0.05f32 as f64)) as f32
    } else {
        (t as f64 / n as f64) as f32
    };
    // ponytail: sem o registro 4 a original lê a vaga do byte n com os 3 bytes de cima sem valor; a cópia usa zero
    let mut c4 = f32::from_bits(n as u32);
    let r4 = registro(this, jogador, 4);
    if r4 != 0 {
        let k = *(r4 as *const u8);
        let v = read_i16(r4 + 0xe) as f64;
        c4 = if k < 8 { (((8 - k as i32) as f64 * 3.0 + v) * 0.125) as f32 } else { (v / k as f64) as f32 };
    }
    let mut c5 = 0.0f64; // ponytail: sem o registro 5, a original carrega uma vaga sem valor; a cópia usa zero
    let r5 = registro(this, jogador, 5);
    if r5 != 0 {
        let k = *(r5 as *const u8);
        let v = read_i16(r5 + 0xe) as f64;
        c5 = if k < 5 { ((5 - k as i32) as f64 + v) * (0.2f32 as f64) } else { v / k as f64 };
    }
    let (c23, c4) = (c23 as f64, c4 as f64);
    match categoria as u8 as i8 as i32 {
        100 => ((c5 * 1.5 + c4 * 2.5) + c23 * 6.0) * 0.1,
        101 => ((c5 * 3.0 + c4 * 5.0) + (c23 + c23)) * 0.1,
        102 => ((c5 * 5.0 + c4 * 4.0) + c23) * 0.1,
        _ => 0.0,
    }
}

unsafe extern "thiscall" fn comparar_estatistica_da_categoria(this: usize, jogador: usize, categoria: u32, qual: u32, padrao: f32) -> f64 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64>(1200);
    if jogador == 0 || (read_usize(jogador) as i32) >= read_usize(0xae236c) as i32 {
        return o(this, jogador, categoria, qual, padrao);
    }
    let c = categoria as u8 as i8;
    let simples = qual as u8 != 0x13 && c <= 6;
    // ponytail: categoria fora de 1..6 cai na caixa da 0x7c3920; nas combinadas sem o registro 4 ou 5, locais sem valor
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_PLAYERSTATS_GET_CATEGORY_RECORD);
    if (simples && !(1..=6).contains(&c))
        || (!simples && (100..=102).contains(&(c as i32)) && (registro(this, jogador, 4) == 0 || registro(this, jogador, 5) == 0))
    {
        return o(this, jogador, categoria, qual, padrao);
    }
    let r = o(this, jogador, categoria, qual, padrao);
    let n = estatistica_da_categoria(this, jogador, categoria, qual, padrao);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || {
        format!("estatistica_da_categoria(jogador {}, categoria {}, qual {}, padrão {}) original={} rust={}", read_usize(jogador) as i32, c, qual as u8, padrao, r, n)
    });
    r
}

/// the order of the language records: by person id and by language, with 0xff matching any language
/// Original: `player_stats_compare_language_entries` (0x7c6d40, player_stats.cpp, cdecl).
///
/// [pt] { 0x7c6d40 } (cdecl, player_stats.cpp): `player_stats_compare_language_entries(a, b)` (o qsort e o bsearch dos
/// idiomas, registros de 6 bytes {id, idioma, fluência}): a nulo, −1; b nulo, 1; ids (+0) diferentes, a diferença;
/// idioma 0xff em qualquer um, 0; senão a diferença dos idiomas (+4, com sinal). Pela fluência no idioma 0x7c6830 e pelos
/// idiomas acrescentados, tirados e melhorados.
unsafe extern "C" fn ordem_dos_idiomas(a: usize, b: usize) -> i32 {
    if a == 0 {
        return -1;
    }
    if b == 0 {
        return 1;
    }
    let (ia, ib) = (read_usize(a) as i32, read_usize(b) as i32);
    if ia != ib {
        return ia.wrapping_sub(ib);
    }
    let (la, lb) = (*((a + 4) as *const u8), *((b + 4) as *const u8));
    if la == 0xff || lb == 0xff {
        return 0;
    }
    la as i8 as i32 - lb as i8 as i32
}

unsafe extern "C" fn comparar_ordem_dos_idiomas(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1201)(a, b);
    let n = ordem_dos_idiomas(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_idiomas({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// the order of the player history by the dword +0xa and then by +4
/// Original: `player_stats_compare_history_by_club` (0x7c6d90, player_stats.cpp, cdecl).
///
/// [pt] { 0x7c6d90 } (cdecl, player_stats.cpp): `player_stats_compare_history_by_club(a, b)` (o qsort da história na carga
/// das estatísticas 0x7bfd50): a nula, 1; b nula, −1; pelo dword +0xa e depois pelo +4, sem sinal: maior 1, menor −1,
/// iguais 0.
unsafe extern "C" fn ordem_da_historia_por_clube(a: usize, b: usize) -> i32 {
    if a == 0 {
        return 1;
    }
    if b == 0 {
        return -1;
    }
    let (ua, ub) = (read_usize(a + 0xa) as u32, read_usize(b + 0xa) as u32);
    if ua != ub {
        return if ua > ub { 1 } else { -1 };
    }
    let (va, vb) = (read_usize(a + 4) as u32, read_usize(b + 4) as u32);
    if va > vb {
        1
    } else if va < vb {
        -1
    } else {
        0
    }
}

unsafe extern "C" fn comparar_ordem_da_historia_por_clube(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1202)(a, b);
    let n = ordem_da_historia_por_clube(a, b);
    PLACAR.registrar(r == n, || format!("ordem_da_historia_por_clube({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 255a leva: os gols na carreira do jogador, o registro do empréstimo e a ordem da fila das voltas aos treinos (player_stats.cpp, staff_records.cpp, injury.cpp) ----------


unsafe fn cps9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// sums the goals of every season and club of the player's career table (0x7c3e40), in 16 bits
/// Original: `player_stats_get_career_goals` (0x7c3d20, player_stats.cpp, stdcall, ret 4).
///
/// [pt] { 0x7c3d20 } (stdcall, ret 4, player_stats.cpp; a original recebe o PlayerStats no ecx e não o lê):
/// `player_stats_get_career_goals(pessoa)` (pela MATCH_PLAYER::setup 0x6eaea0, que grava em +0x43 do jogador da
/// partida): pessoa nula, a caixa "Error" (linha 0xb4b) e 0; id a partir de [0xae2364], 0; sem a ficha de jogador
/// (+0x61), a caixa (linha 0xb5d) e 0. Senão a tabela da carreira (0x7c3e40 do [0xb74350]: entradas de 0x2b bytes,
/// seis por temporada e clube) e a soma, em 16 bits, do byte +8 da sexta entrada de cada grupo (+0xdf); a tabela é
/// liberada (0x9452ca, também quando nula). Quente (cada jogador de cada partida): sem contador.
unsafe extern "stdcall" fn gols_na_carreira(pessoa: usize) -> u16 {
    if pessoa == 0 {
        aviso_em(0xa78310, 0xb4b);
        return 0;
    }
    if cps9_i(pessoa) >= cps9_i(0xae2364) {
        return 0;
    }
    if read_u32_as_usize(pessoa + 0x61) == 0 {
        aviso_em(0xa78310, 0xb5d);
        return 0;
    }
    let montar: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(FN_PLAYERSTATS_BUILD_CAREER_HISTORY);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut n = 0i32;
    let t = montar(read_u32_as_usize(0xb74350), pessoa, &mut n);
    let mut s = 0u16;
    if t != 0 {
        let mut i = 0i32; // cps9: do primeiro grupo
        while i < n {
            s = s.wrapping_add(*((t + 0xdf + (i / 6) as usize * 0x102) as *const u8) as u16);
            i += 6;
        }
    }
    liberar(t);
    s // cps9: os gols na carreira
}

/// Comparador: a devolução (a tabela é montada e liberada pelas duas), sem os casos de erro (só a original).
unsafe extern "stdcall" fn comparar_gols_na_carreira(pessoa: usize) -> u16 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> u16>(1140);
    if pessoa == 0 || (cps9_i(pessoa) < cps9_i(0xae2364) && read_u32_as_usize(pessoa + 0x61) == 0) {
        return o(pessoa);
    }
    let r = o(pessoa);
    let n = gols_na_carreira(pessoa);
    PLACAR.registrar(r == n, || format!("gols_na_carreira(pessoa {:x}) original={} rust={}", pessoa, r, n));
    r
}

/// the loan histories: the one of the receiving club manager (type 0x69) and the one of the person (type 0x6a)
/// Original: `Staff_Records::add_loan_records` (0x86da20, staff_records.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x86da20 } (thiscall, ret 0xc, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_loan_records(pessoa, de, clube)` (pela 0x8c2180, a transferência aceita): com o clube e o técnico
/// dele (+0xcf), o histórico do técnico (tipo 0x69: o clube, a pessoa e os três ids +0x33 de +4, +8 e +0xc dela, e o id
/// de `de`); depois, na mesma entrada, o da pessoa (tipo 0x6a: o clube e o id de `de`), no arquivo (0x58e340). O clube
/// é lido antes do teste da pessoa, como na original; pessoa nula: a caixa "Error" (linha 0x1010).
unsafe extern "thiscall" fn registrar_o_emprestimo(this: usize, pessoa: usize, de: usize, clube: usize) {
    crate::comparacao::ramo("cps_9 emprestimo");
    let mut e = cpq4_entrada();
    if clube != 0 && read_u32_as_usize(clube + 0xcf) != 0 {
        let tecnico = read_u32_as_usize(clube + 0xcf);
        cpq4_datar(e.as_mut_ptr());
        e[0xc] = 0x69;
        cpq4_por(&mut e, 8, cps9_i(clube));
        cpq4_quem(&mut e, 0xd, pessoa);
        cpq4_por(&mut e, 0x1d, cps9_i(de));
        cpq4_historico(true, this, read_u32_as_usize(tecnico), &e);
    }
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x6a;
    cpq4_por(&mut e, 8, cps9_i(clube));
    cpq4_por(&mut e, 0xd, cps9_i(de));
    if pessoa != 0 {
        cpq4_historico(true, this, read_u32_as_usize(pessoa), &e);
    } else {
        aviso_em(0xa83b78, 0x1010);
    }
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia não grava mais nada: só a original; a
// cópia leva o ramo.
unsafe extern "thiscall" fn comparar_registrar_o_emprestimo(this: usize, pessoa: usize, de: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1141)(this, pessoa, de, clube)
}

/// compares the key date with the one of a 0x16-byte return to training queue entry, for bsearch
/// Original: `injury_compare_entry_dates` (0x632040, injury.cpp, cdecl).
///
/// [pt] { 0x632040 } (cdecl, injury.cpp): `injury_compare_entry_dates(chave, item)`, a ordem do bsearch (0x946464) da fila
/// das voltas aos treinos (entradas de 0x16 bytes) em Injury::post_due_recovery_news (0x62e010, todo dia): a data em
/// +8 da chave antes da do item (0x456770), −1; depois (0x456740), 1; igual, 0.
unsafe extern "C" fn ordem_da_volta_aos_treinos(chave: usize, item: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3C);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_DATE_OPERATOR_3E);
    if antes(chave + 8, read_u32_as_usize(item + 8) as u32, read_u32_as_usize(item + 0xc) as u32) != 0 {
        return -1;
    }
    if depois(chave + 8, read_u32_as_usize(item + 8) as u32, read_u32_as_usize(item + 0xc) as u32) != 0 {
        return 1;
    }
    crate::comparacao::ramo("cps_9 data igual");
    0 // cps9: a volta de hoje achada
}

unsafe extern "C" fn comparar_ordem_da_volta_aos_treinos(chave: usize, item: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1142)(chave, item);
    let n = ordem_da_volta_aos_treinos(chave, item);
    PLACAR.registrar(r == n, || format!("ordem_da_volta_aos_treinos(chave {:x}, item {:x}) original={} rust={}", chave, item, r, n));
    r
}

// ---------- 253a leva: as estatísticas dos jogadores e a aposentadoria marcada: o registro da categoria, criar o registro e se a pessoa está na lista de aposentadorias (player_stats.cpp, player_regen.cpp) ----------


unsafe fn cpq10_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A vaga da categoria `categoria` (1..6, byte com sinal) no índice do jogador: o registro de 39 bytes do id dele
/// (+0) em [this+0x30], dword 4 × (categoria − 1). None sem jogador ou fora de 1..6.
unsafe fn cpq10_vaga(this: usize, jogador: usize, categoria: u32) -> Option<usize> {
    let k = (categoria as u8 as i8 as i32).wrapping_sub(1);
    if jogador == 0 || k as u32 > 5 {
        return None;
    }
    Some(read_usize(this + 0x30).wrapping_add(read_usize(jogador).wrapping_mul(39)) + 4 * k as usize)
}

/// A caixa "Memory Allocation Error" de player_stats.cpp (0xa78310): "v%s %s.%s %d (%s)" (versão 0x90d130, nome e
/// extensão, `linha`, `texto`) em 0xae24d0, o título traduzido (0x66f4e0) em 0xae28bc, o corpo (0x9a7ff8 com
/// 0xae24d0) em 0xae26c4 e a caixa 0x5e8290. O jogo segue.
unsafe fn cpq10_sem_memoria(linha: u32, texto: usize) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(FN_CRT_STRTOK);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32, usize) -> usize = core::mem::transmute(FN_UTILS_GET_VERSION_STRING);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_CRT_SPRINTF);
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(FN_LANGLIB_SPRINTF);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_GUI_SHOW_MESSAGE_BOX);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa78310, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha, texto);
    formatar(0xae24d0, 0x9a8120, s, nome.as_ptr(), ext.as_ptr(), linha, texto);
    traduzir(0xae28bc, 0x9a8108);
    traduzir(0xae26c4, 0x9a7ff8, 0xae24d0usize);
    caixa(0xae28bc, 0xae26c4, 0);
}

/// the 0x20-byte record of the player's category (1..6), through the index in the 39-byte entry of his id; 0 without an
/// index
/// Original: `PlayerStats::get_category_record` (0x7c3920, player_stats.cpp, thiscall, ret 8).
///
/// [pt] { 0x7c3920 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::get_category_record(jogador, categoria)`: o
/// registro de 0x20 bytes ([this+4] + 32 × índice) da categoria do jogador (cpq10_vaga), ou 0 quando o índice é −1
/// ou menor. Sem jogador ou fora de 1..6, a caixa "Error" e 0: a original. Pela PlayerStats::record_match 0x7c10e6 e
/// pela PlayerStats::get_stat 0x7c1e30 (as médias do motor de partida). Quente: sem contador.
unsafe extern "thiscall" fn registro_da_categoria(this: usize, jogador: usize, categoria: u32) -> usize {
    let vaga = match cpq10_vaga(this, jogador, categoria) {
        Some(v) => v,
        _ => return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1072)(this, jogador, categoria),
    };
    let i = read_usize(vaga) as i32; // o índice da categoria
    if i <= -1 {
        return 0;
    }
    read_usize(this + 4).wrapping_add((i as usize) << 5)
}

/// the player's category record; without an index, the first free record (+0xe = 0xffff) or a new one at the end (grows
/// by 100), zeroed
/// Original: `PlayerStats::create_stats` (0x7c5d80, player_stats.cpp, thiscall, ret 8).
///
/// [pt] { 0x7c5d80 } (thiscall, ret 8, player_stats.cpp): `PlayerStats::create_stats(jogador, categoria)`: o registro da
/// categoria do jogador (cpq10_vaga); sem índice (−1), com registros soltos ([this+0x38]), o primeiro registro de
/// [this+4] (conta +0xc) com +0xe = 0xffff vira o dele (zerado; os soltos descem); senão um no fim (zerado; a conta
/// sobe): cheio (capacidade +8), cresce de 100 em 100 (realloc 0x945667; sem memória, a caixa 0xef4 e 0). Sem
/// jogador ou fora de 1..6, a caixa "Error" e 0: a original. Pela PlayerStats::record_match 0x7c10f9.
unsafe extern "thiscall" fn criar_estatisticas(this: usize, jogador: usize, categoria: u32) -> usize {
    let vaga = match cpq10_vaga(this, jogador, categoria) {
        Some(v) => v,
        _ => return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1073)(this, jogador, categoria),
    };
    if read_usize(vaga) as i32 != -1 {
        return read_usize(this + 4).wrapping_add(read_usize(vaga) << 5);
    }
    if read_usize(this + 0x38) != 0 {
        let n = read_usize(this + 0xc) as i32;
        let mut i = 0i32;
        while i < n {
            let r = read_usize(this + 4).wrapping_add((i as usize) << 5);
            if core::ptr::read_unaligned((r + 0xe) as *const u16) == 0xffff {
                crate::comparacao::ramo("cpq_10 registro solto");
                core::ptr::write_bytes(r as *mut u8, 0, 0x20);
                cpq10_por(vaga, i as usize);
                cpq10_por(this + 0x38, read_usize(this + 0x38).wrapping_sub(1));
                return r;
            }
            i += 1;
        }
    }
    let n = read_usize(this + 0xc) as i32;
    if n >= read_usize(this + 8) as i32 {
        let cap = read_usize(this + 8).wrapping_add(100);
        cpq10_por(this + 8, cap);
        let realocar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realocar(read_usize(this + 4), cap << 5);
        cpq10_por(this + 4, p);
        if p == 0 {
            cpq10_sem_memoria(0xef4, 0xa784fc); // "PLAYER_STATS::create_stats() : Unable to allocate enough memory"
            return 0;
        }
    }
    crate::comparacao::ramo("cpq_10 registro novo");
    cpq10_por(vaga, read_usize(this + 0xc)); // o índice novo
    cpq10_por(this + 0xc, read_usize(this + 0xc).wrapping_add(1));
    let r = read_usize(this + 4).wrapping_add(read_usize(vaga) << 5);
    core::ptr::write_bytes(r as *mut u8, 0, 0x20);
    r
}

/// 1 when the person's id is on the retirement list, else 0
/// Original: `PLAYER_REGEN::is_retirement_pending` (0x7ae350, player_regen.cpp, thiscall, ret 4).
///
/// [pt] { 0x7ae350 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::is_retirement_pending(pessoa)`: 1 quando o id da
/// pessoa (+0) está na lista de aposentadorias ([this+4], registros de 0x1a bytes, conta +8 com sinal), senão 0 (e 0
/// sem pessoa). Pela MANAGER_MANAGER::is_staff_suitable_for_job 0x69fa4b: quem não é jogador e vai se aposentar não
/// serve para técnico.
unsafe extern "thiscall" fn aposentadoria_marcada(this: usize, pessoa: usize) -> u8 {
    if pessoa == 0 {
        return 0;
    }
    let n = read_usize(this + 8) as i32;
    let id = read_usize(pessoa);
    let mut i = 0i32;
    while i < n {
        if read_usize(read_usize(this + 4) + 0x1a * i as usize) == id {
            crate::comparacao::ramo("cpq_10 aposentadoria marcada");
            return 1; // está na lista
        }
        i += 1;
    }
    0
}

/// Comparador: pelo valor (só lê); os casos de erro só na original.
unsafe extern "thiscall" fn comparar_registro_da_categoria(this: usize, jogador: usize, categoria: u32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1072)(this, jogador, categoria);
    if cpq10_vaga(this, jogador, categoria).is_none() {
        return r;
    }
    let n = registro_da_categoria(this, jogador, categoria);
    PLACAR.registrar(r == n, || format!("registro_da_categoria({:x}, {:x}, {}) original={:x} rust={:x}", this, jogador, categoria as u8 as i8, r, n));
    r
}

/// Comparador: a original realoca o vetor, então só ela roda; antes, o plano (o índice que o jogador vai ter, a
/// conta e os soltos depois); depois, o registro devolvido, o índice na vaga, a conta, os soltos e o registro
/// zerado quando é novo. ponytail: os casos de erro e a falta de memória só na original, sem conferir.
unsafe extern "thiscall" fn comparar_criar_estatisticas(this: usize, jogador: usize, categoria: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1073);
    let vaga = match cpq10_vaga(this, jogador, categoria) {
        Some(v) => v,
        _ => return o(this, jogador, categoria),
    };
    let atual = read_usize(vaga) as i32;
    let (n, soltos) = (read_usize(this + 0xc), read_usize(this + 0x38));
    let solto = if atual == -1 && soltos != 0 {
        (0..n as i32).find(|&i| core::ptr::read_unaligned((read_usize(this + 4) + ((i as usize) << 5) + 0xe) as *const u16) == 0xffff)
    } else {
        None
    };
    let (indice, n2, soltos2) = match (atual, solto) {
        (-1, Some(i)) => (i, n, soltos.wrapping_sub(1)),
        (-1, None) => (n as i32, n.wrapping_add(1), soltos),
        (i, _) => (i, n, soltos),
    };
    let r = o(this, jogador, categoria);
    if r == 0 {
        return r;
    }
    let esperado = read_usize(this + 4).wrapping_add((indice as usize) << 5);
    let zerado = atual != -1 || (0..0x20).all(|k| *((r + k) as *const u8) == 0);
    let igual = r == esperado && read_usize(vaga) as i32 == indice && read_usize(this + 0xc) == n2 && read_usize(this + 0x38) == soltos2 && zerado;
    PLACAR.registrar(igual, || format!("criar_estatisticas({:x}, {:x}, {}) original={:x} plano={:x} índice {} x {} conta {} x {} soltos {} x {} zerado={}",
        this, jogador, categoria as u8 as i8, r, esperado, read_usize(vaga) as i32, indice, read_usize(this + 0xc), n2, read_usize(this + 0x38), soltos2, zerado));
    r
}

comparar_valor!(comparar_aposentadoria_marcada, aposentadoria_marcada, 1074, (pessoa: usize) -> u8);

// ---------- 251a leva: os idiomas do jogador (acrescentar e melhorar) e zerar as estatísticas do jogador regenerado (player_stats.cpp) ----------


unsafe fn cpo12_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// O registro de idioma (6 bytes: +0 id da pessoa, +4 idioma, +5 fluência) da `pessoa` no `idioma`: bsearch
/// (0x946464, comparação 0x7c6d40) na tabela [this+0x3c], [this+0x44] registros. 0 sem achar.
unsafe fn cpo12_idioma(this: usize, pessoa: usize, idioma: u8) -> usize {
    let bsearch: unsafe extern "C" fn(*const u8, usize, u32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let mut chave = [0u8; 8];
    core::ptr::write_unaligned(chave.as_mut_ptr() as *mut u32, read_u32_as_usize(pessoa) as u32);
    chave[4] = idioma; // a comparação só lê o byte +4 da chave
    bsearch(chave.as_ptr(), read_u32_as_usize(this + 0x3c), read_u32_as_usize(this + 0x44) as u32, 6, 0x7c6d40)
}

/// appends the new entry to the language table (6 bytes: id, language, fluency up to 10; starts at 100, grows by 100)
/// and sorts it when asked (qsort)
/// Original: `PlayerStats::add_language` (0x7c6a30, player_stats.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7c6a30 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::add_language(pessoa, idioma, fluencia,
/// ordenar)`: a tabela dos idiomas (6 bytes por registro em [this+0x3c], capacidade +0x40, conta +0x44) nasce com 100
/// (malloc 0x258) e cresce 100 (realloc) quando cheia; o registro novo no fim: o id da pessoa, o idioma e a fluência
/// (no máximo 10, com sinal); com `ordenar`, qsort pela comparação 0x7c6d40. Sem memória: caixa fatal.
unsafe extern "thiscall" fn acrescentar_idioma(this: usize, pessoa: usize, idioma: u32, fluencia: u32, ordenar: u32) {
    if read_u32_as_usize(this + 0x3c) == 0 {
        let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
        let p = malloc(0x258);
        cpo12_por(this + 0x3c, p as u32);
        if p == 0 {
            aviso_em(0xa78310, 0x11fd);
            erro_fatal();
        }
        cpo12_por(this + 0x40, 0x64);
    }
    let cap = read_u32_as_usize(this + 0x40);
    if cap == read_u32_as_usize(this + 0x44) {
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_CRT_REALLOC);
        let p = realloc(read_u32_as_usize(this + 0x3c), (cap as u32).wrapping_mul(3).wrapping_add(0x12c).wrapping_mul(2));
        cpo12_por(this + 0x3c, p as u32);
        if p == 0 {
            aviso_em(0xa78310, 0x1208);
            erro_fatal();
        }
        cpo12_por(this + 0x40, (read_u32_as_usize(this + 0x40) as u32).wrapping_add(0x64));
    }
    crate::comparacao::ramo("cpo_12 idioma novo");
    let n = read_u32_as_usize(this + 0x44);
    let r = read_u32_as_usize(this + 0x3c).wrapping_add(n.wrapping_mul(6));
    cpo12_por(r, read_u32_as_usize(pessoa) as u32);
    *((r + 4) as *mut u8) = idioma as u8;
    let f = fluencia as u8;
    *((r + 5) as *mut u8) = if (f as i8) > 10 { 10 } else { f };
    cpo12_por(this + 0x44, (n as u32).wrapping_add(1));
    if ordenar != 0 {
        crate::comparacao::ramo("cpo_12 idiomas ordenados");
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
        qsort(read_u32_as_usize(this + 0x3c), read_u32_as_usize(this + 0x44) as u32, 6, 0x7c6d40);
    }
}

/// finds the person's entry for the language (bsearch) and adds the increase to the fluency, up to 10; without one,
/// adds the language with the increase as fluency
/// Original: `PlayerStats::improve_language` (0x7c6ca0, player_stats.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7c6ca0 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::improve_language(pessoa, idioma, aumento,
/// ordenar)`: sem tabela, ou sem o registro da pessoa no idioma (`cpo12_idioma`), acrescenta-o com a fluência
/// `aumento` (0x7c6a30); com ele, soma o aumento à fluência (byte), no máximo 10 (com sinal).
unsafe extern "thiscall" fn melhorar_idioma(this: usize, pessoa: usize, idioma: u32, aumento: u32, ordenar: u32) {
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(FN_PLAYERSTATS_ADD_LANGUAGE);
    if read_u32_as_usize(this + 0x3c) == 0 {
        acrescentar(this, pessoa, idioma, aumento, ordenar);
        return;
    }
    let r = cpo12_idioma(this, pessoa, idioma as u8);
    if r == 0 {
        acrescentar(this, pessoa, idioma, aumento, ordenar);
        return;
    }
    crate::comparacao::ramo("cpo_12 idioma melhorado");
    let f = (*((r + 5) as *const u8)).wrapping_add(aumento as u8);
    *((r + 5) as *mut u8) = f;
    if (f as i8) > 10 {
        *((r + 5) as *mut u8) = 10;
    }
}

/// frees the player's 6 category records (+0xe = −1, +1 to the free count), clears the league totals and recent ratings
/// and empties the history; in DESLIGADAS since 2026-10-29: with the detour the career in Rust mode left the reference
/// be54a138 (bisection of batch 251's 39 routines; with 0x77bbe0, the news constructor, the game later crashed in
/// 0x7c6650 on a broken index); the cause (stack residue or a difference in the copy) is not isolated yet; the copy
/// stays in the code and does not run
/// Original: `PlayerStats::reset_player` (0x7c6650, player_stats.cpp, thiscall, ret 4).
///
/// [pt] { 0x7c6650 } (thiscall, ret 4, player_stats.cpp): `PlayerStats::reset_player(pessoa)`: no registro de 39 bytes da
/// pessoa ([this+0x30] + id da ficha de jogador [pessoa+0x61] × 39; corrigido em 31/10/2026, antes usava o id da pessoa), cada uma das 6 categorias (+0..+0x14) com registro (≠ −1) o libera (+0xe = −1 no
/// registro de 32 bytes de [this+4]; +1 nos livres [this+0x38]) e fica −1; zera +0x20, +0x22 (palavras) e +0x24..+0x26
/// (as notas recentes) e esvazia o histórico (0x58f110 sobre [this] com +0x18/+0x1c). Pessoa nula ou não jogadora
/// (+0x61): a original (caixa fatal). Devolve o que a 0x58f110 devolve. Do regen (0x7a59e0).
unsafe extern "thiscall" fn zerar_estatisticas_do_jogador(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 || read_u32_as_usize(pessoa + 0x61) == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1002)(this, pessoa); // caixa fatal: só a original
    }
    let r = read_u32_as_usize(this + 0x30).wrapping_add(read_u32_as_usize(read_u32_as_usize(pessoa + 0x61)).wrapping_mul(39)); // o id da ficha de jogador (+0x61), não o da pessoa
    for k in 0..6usize {
        let i = read_u32_as_usize(r + 4 * k);
        if i as u32 == 0xffff_ffff {
            continue;
        }
        core::ptr::write_unaligned((read_u32_as_usize(this + 4).wrapping_add(i << 5) + 0xe) as *mut u16, 0xffff);
        cpo12_por(this + 0x38, (read_u32_as_usize(this + 0x38) as u32).wrapping_add(1));
        cpo12_por(r + 4 * k, 0xffff_ffff);
    }
    core::ptr::write_unaligned((r + 0x20) as *mut u16, 0);
    core::ptr::write_unaligned((r + 0x22) as *mut u16, 0);
    *((r + 0x26) as *mut u8) = 0;
    *((r + 0x25) as *mut u8) = 0;
    *((r + 0x24) as *mut u8) = 0;
    let esvaziar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_FILE_LLIST_RESET_LAST_OFFSET);
    esvaziar(read_u32_as_usize(this), r + 0x18, r + 0x1c)
}

/// Comparador: a 0x7c6a30 aloca e realoca a tabela (não pode rodar duas vezes): o plano antes (a conta e a
/// capacidade depois, a fluência), a original roda; confere a conta, a capacidade e o registro novo (no fim sem ordenar;
/// com `ordenar`, em algum lugar da tabela).
unsafe extern "thiscall" fn comparar_acrescentar_idioma(this: usize, pessoa: usize, idioma: u32, fluencia: u32, ordenar: u32) {
    let (n, cap) = if read_u32_as_usize(this + 0x3c) == 0 { (read_u32_as_usize(this + 0x44), 100) } else { (read_u32_as_usize(this + 0x44), read_u32_as_usize(this + 0x40)) };
    let cap = if cap == n { cap.wrapping_add(100) } else { cap };
    let f = fluencia as u8;
    let novo = (read_u32_as_usize(pessoa) as u32, idioma as u8, if (f as i8) > 10 { 10 } else { f });
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(1000)(this, pessoa, idioma, fluencia, ordenar);
    let base = read_u32_as_usize(this + 0x3c);
    let e = |i: usize| {
        let r = base.wrapping_add(i * 6);
        (read_u32_as_usize(r) as u32, *((r + 4) as *const u8), *((r + 5) as *const u8))
    };
    let conta = read_u32_as_usize(this + 0x44);
    let mut ok = conta == n.wrapping_add(1) && read_u32_as_usize(this + 0x40) == cap;
    if ok {
        ok = if ordenar == 0 { e(n) == novo } else { (0..conta).any(|i| e(i) == novo) };
    }
    PLACAR.registrar(ok, || format!("acrescentar_idioma({:?}, ordenar {}) conta {} (esperava {}) capacidade {} (esperava {})",
        novo, ordenar, conta, n.wrapping_add(1), read_u32_as_usize(this + 0x40), cap));
}

/// Comparador: sem tabela ou sem o registro, só a original (o acréscimo 0x7c6a30 tem o comparador dele); com o
/// registro, lado a lado nos 6 bytes dele.
unsafe extern "thiscall" fn comparar_melhorar_idioma(this: usize, pessoa: usize, idioma: u32, aumento: u32, ordenar: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(1001);
    let r = if read_u32_as_usize(this + 0x3c) == 0 { 0 } else { cpo12_idioma(this, pessoa, idioma as u8) };
    if r == 0 {
        return o(this, pessoa, idioma, aumento, ordenar);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 6)],
        || o(this, pessoa, idioma, aumento, ordenar), || melhorar_idioma(this, pessoa, idioma, aumento, ordenar));
    PLACAR.registrar(igual, || format!("melhorar_idioma({}, idioma {}, +{}) {:x?} x {:x?}", read_u32_as_usize(pessoa), idioma as u8, aumento as u8, mo, mr));
}

/// Comparador: lado a lado no registro de 39 bytes, na conta dos livres (+0x38) e na marca +0xe dos registros de
/// categoria liberados (a 0x58f110 só grava +0x1c, dentro do registro). Pessoa nula ou não jogadora: só a original.
unsafe extern "thiscall" fn comparar_zerar_estatisticas_do_jogador(this: usize, pessoa: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1002);
    if pessoa == 0 || read_u32_as_usize(pessoa + 0x61) == 0 {
        return o(this, pessoa);
    }
    let r = read_u32_as_usize(this + 0x30).wrapping_add(read_u32_as_usize(read_u32_as_usize(pessoa + 0x61)).wrapping_mul(39)); // o id da ficha de jogador (+0x61), não o da pessoa
    let mut reg = vec![(r, 39), (this + 0x38, 4)];
    for k in 0..6usize {
        let i = read_u32_as_usize(r + 4 * k);
        if i as u32 != 0xffff_ffff {
            reg.push((read_u32_as_usize(this + 4).wrapping_add(i << 5) + 0xe, 2));
        }
    }
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa), || zerar_estatisticas_do_jogador(this, pessoa));
    PLACAR.registrar(igual && a == b, || format!("zerar_estatisticas_do_jogador({}) {:x?} x {:x?}", read_u32_as_usize(pessoa), mo, mr));
    a
}

// ---------- 247a leva: a fluência da pessoa num idioma, recomeçar o histórico da pessoa e a posição de uma casa da formação ----------




/// from 0 to 10: the contract level, 10 for a nation (or the second one) speaking the language, or the learned language
/// (binary search)
/// Original: `PlayerStats::get_language_fluency` (0x7c6830, player_stats.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x7c6830 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::get_language_fluency(pessoa, clube, idioma,
/// pelo_contrato)`: quanto a pessoa fala o `idioma` (0 a 10). Técnico humano (id ≥ [0xae2364]): 10 se é o idioma
/// (+0x77) da nação dela (+0x1a), senão 0. Com `pelo_contrato`, o contrato com o clube (0x4cf5f0) de quem joga num
/// clube (+0x39) da mesma nação do `clube` (+0x53): o nível do contrato (+0x4f & 0xf). 10 se a nação (+0x1a) ou a
/// segunda (+0x1e) fala o idioma da nação do `clube` ou o `idioma`. Senão o idioma aprendido: a busca binária (bsearch
/// 0x946464, comparação 0x7c6d40, registros de 6 bytes em [+0x3c], [+0x44] deles) pela chave {id, idioma}, o byte +5;
/// sem a tabela ou sem o registro, 0. Os chamadores só leem o al.
unsafe extern "thiscall" fn fluencia_no_idioma(this: usize, pessoa: usize, clube: usize, idioma: u32, pelo_contrato: u32) -> u8 {
    let idioma = idioma as u8;
    if pessoa != 0 && read_usize(pessoa) as i32 >= read_usize(0xae2364) as i32 {
        return if read_u8(read_usize(pessoa + 0x1a) + 0x77) == idioma { 10 } else { 0 }; // o técnico humano
    }
    if pelo_contrato != 0 {
        let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
        let c = contrato(read_usize(0xae1050), pessoa, clube);
        if c != 0 {
            let cp = read_usize(pessoa + 0x39);
            if cp != 0 && read_usize(clube + 0x53) == read_usize(cp + 0x53) {
                return read_u8(c + 0x4f) & 0xf;
            }
        }
    }
    let fala = |l: u8| {
        if read_u8(read_usize(pessoa + 0x1a) + 0x77) == l {
            return true;
        }
        let s = read_usize(pessoa + 0x1e);
        s != 0 && read_u8(s + 0x77) == l // a segunda nação
    };
    if clube != 0 {
        let n = read_usize(clube + 0x53);
        if n != 0 && fala(read_u8(n + 0x77)) {
            return 10;
        }
    }
    if fala(idioma) {
        return 10;
    }
    let base = read_usize(this + 0x3c);
    if base == 0 {
        return 0;
    }
    let mut chave = [0u8; 8];
    chave[..4].copy_from_slice(&(read_usize(pessoa) as u32).to_le_bytes());
    chave[4] = idioma;
    let bsearch: unsafe extern "C" fn(*const u8, usize, usize, usize, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let e = bsearch(chave.as_ptr(), base, read_usize(this + 0x44), 6, 0x7c6d40);
    if e == 0 {
        return 0;
    }
    read_u8(e + 5)
}

/// the last offset of the person's history list (+4 of the pair at [+0xc] + 8 × id) = −1 (0x58f110)
/// Original: `Staff_Records::rewind_staff_history` (0x86fb80, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86fb80 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::rewind_staff_history(pessoa)`: a lista do
/// histórico da pessoa (o par de 8 bytes em [+0xc] + 8 × id) fica sem o último (0x58f110 sobre [this]: o segundo
/// deslocamento = −1). Pessoa nula: a original (caixa fatal: o jogo termina).
unsafe extern "thiscall" fn recomecar_historico_da_pessoa(this: usize, pessoa: usize) {
    if pessoa == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(902)(this, pessoa); // caixa fatal: só a original
    }
    crate::comparacao::ramo("cpk_7 recomecar historico");
    let par = read_usize(this + 0xc).wrapping_add(read_usize(pessoa).wrapping_mul(8));
    let sem_ultimo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_FILE_LLIST_RESET_LAST_OFFSET);
    sem_ultimo(read_usize(this), par, par + 4); // o último deslocamento
}

/// stores the general or extra position of the slot, the marks, and recomputes the pitch cells without and with the
/// ball (0x5b7e60)
/// Original: `Formation::set_slot_position` (0x5b2ba0, formation.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5b2ba0 } (thiscall, ret 0xc, formation.cpp): `Formation::set_slot_position(casa, posicao, extra)`: sem `extra`,
/// a posição geral da casa (+0x12d, palavras) muda para `posicao` (se mudar: a máscara 0x5b59a0, a marca +0x143 da casa
/// = 1 e a das outras casas com a mesma posição = 0); com ou sem, a posição extra (+0x14e) = `posicao`. Depois, para
/// cada faixa 0..2 e altura 0..3 (a palavra baixa e a alta do argumento da 0x5b7e60), as casas no campo sem e com a
/// bola (+0x164 e +0x194 + 4 × (altura + 4 × faixa + 24 × casa)): com geral = extra, as duas da geral; senão, pelos
/// setores (0x5b73a0): extra de setor abaixo do da geral, sem a bola a extra e com a bola a geral; senão o contrário.
/// Por fim +0x187f = 0. Devolve 1. Casa fora de 0..10: a original (caixa "Error" e 0).
unsafe extern "thiscall" fn posicao_na_formacao(this: usize, casa: u32, posicao: u32, extra: u32) -> u32 {
    let k = casa as u8 as i8;
    if !(0..=10).contains(&k) {
        return orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(903)(this, casa, posicao, extra); // caixa: só a original
    }
    let s = k as usize;
    let p = posicao as u16 as i16;
    let geral = this + 0x12d + 2 * s;
    let extra_da_casa = this + 0x14e + 2 * s;
    if extra == 0 && read_i16(geral) != p {
        core::ptr::write_unaligned(geral as *mut i16, p);
        let mascara: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_FORMATION_UPDATE_SHAPE_CODE);
        mascara(this);
        *((this + 0x143 + s) as *mut u8) = 1;
        for j in 0..11usize {
            if j != s && read_i16(this + 0x12d + 2 * j) == p {
                *((this + 0x143 + j) as *mut u8) = 0; // a outra casa com a mesma posição
            }
        }
    }
    core::ptr::write_unaligned(extra_da_casa as *mut i16, p);
    // a 0x5b7e60 e a 0x5b73a0 só leem a palavra baixa da máscara (a original empurra o eax com o alto que estiver)
    let casa_no_campo: unsafe extern "C" fn(u32, u32, u32, *mut u32) -> u32 = core::mem::transmute(FN_FORMATION_GET_DEFAULT_COORDINATE);
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(FN_FORMATION_SPLIT_POSITION_CODE);
    // as saídas valem de uma volta para a outra, como as locais da original (sem valor só se a 0x5b73a0 falhar
    // logo na primeira: zero aqui)
    let (mut setor_geral, mut setor_extra, mut lado, mut saida) = (0u16, 0u16, 0u16, 0u32);
    if read_i16(geral) == read_i16(extra_da_casa) {
        crate::comparacao::ramo("cpk_7 formacao mesma posicao");
    } else {
        crate::comparacao::ramo("cpk_7 formacao posicao extra");
    }
    for faixa in 0..3usize {
        for altura in 0..4usize {
            let arg = faixa as u32 | (altura as u32) << 16;
            let i = altura + 4 * (faixa + 6 * s);
            let (sem_bola, com_bola) = (this + 0x164 + 4 * i, this + 0x194 + 4 * i);
            let (g, e) = (read_i16(geral) as u16 as u32, read_i16(extra_da_casa) as u16 as u32);
            let (a, b) = if g == e {
                (g, g) // a mesma posição com e sem a bola
            } else {
                separar(g, &mut setor_geral, &mut lado);
                separar(e, &mut setor_extra, &mut lado);
                if (setor_extra as i16) < (setor_geral as i16) { // a extra mais atrás
                    (e, g)
                } else {
                    (g, e)
                }
            };
            casa_no_campo(a, 0, arg, &mut saida);
            core::ptr::write_unaligned(sem_bola as *mut u32, saida);
            casa_no_campo(b, 1, arg, &mut saida);
            core::ptr::write_unaligned(com_bola as *mut u32, saida);
        }
    }
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    1
}

/// Comparador: pelo valor (a 0x4cf5f0 e a busca só leem).
unsafe extern "thiscall" fn comparar_fluencia_no_idioma(this: usize, pessoa: usize, clube: usize, idioma: u32, pelo_contrato: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u8>(901)(this, pessoa, clube, idioma, pelo_contrato);
    let n = fluencia_no_idioma(this, pessoa, clube, idioma, pelo_contrato);
    PLACAR.registrar(r == n, || format!("fluencia_no_idioma({:x}, {:x}, {}, {}) original={} rust={}", pessoa, clube, idioma as u8, pelo_contrato, r, n));
    r
}

/// Comparador: lado a lado pelo par da pessoa (8 bytes; a 0x58f110 só grava o segundo).
unsafe extern "thiscall" fn comparar_recomecar_historico_da_pessoa(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(902);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let par = read_usize(this + 0xc).wrapping_add(read_usize(pessoa).wrapping_mul(8));
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(par, 8)], || o(this, pessoa), || recomecar_historico_da_pessoa(this, pessoa));
    PLACAR.registrar(igual, || format!("recomecar_historico_da_pessoa({}) {:x?} x {:x?}", read_usize(pessoa), mo, mr));
}

/// Comparador: lado a lado pela formação (+0x12d a +0x584: posições, marcas e casas no campo; a máscara +0x187b e
/// +0x187f) e [0xb67a34] (a 0x5b73a0 zera quando falha).
unsafe extern "thiscall" fn comparar_posicao_na_formacao(this: usize, casa: u32, posicao: u32, extra: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(903);
    if !(0..=10).contains(&(casa as u8 as i8)) {
        return o(this, casa, posicao, extra);
    }
    let reg = [(this + 0x12d, 0x584 - 0x12d), (this + 0x187b, 8), (0xb67a34, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, casa, posicao, extra), || posicao_na_formacao(this, casa, posicao, extra));
    PLACAR.registrar(igual && r == n, || format!("posicao_na_formacao({}, {:#x}, {}) {:x?} x {:x?}", casa as u8 as i8, posicao as u16, extra, mo, mr));
    r
}

