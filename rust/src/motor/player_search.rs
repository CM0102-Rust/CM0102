use super::*;







/// Person -> club of their contract (stored in G_CONTRACT_LOOKUP_RESULT), or 0.
///
/// [pt] O clube do contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) ou 0.
pub(crate) unsafe fn player_search_contract_club(pessoa: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    // 0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let c = contract_of(read_usize(G_CONTRACT_MANAGER), pessoa);
    core::ptr::write_unaligned(G_CONTRACT_LOOKUP_RESULT as *mut usize, c);
    if c != 0 { friendly_club_by_id(read_usize(c + 4)) } else { 0 }
}

/// the slots of players who left the watched club are cleared; every club player who played gets a line in the 10-match
/// history: goals, rating, date, opponent, score, side, statistics and the type (starter, substitute, rating from
/// +0x30)
/// Original: `player_search_record_scouted_club_match` (0x7bd310, player_search.cpp, thiscall, ret 8).
///
/// [pt] { 0x7bd310 } (thiscall, ret 8, player_search.cpp): `player_search_record_scouted_club_match(jogo, resultado)`, pela
/// partida observada (0x7b5be0) quando a lista acompanha um clube (+0x190) que joga nela. `this` é o registro do clube
/// observado ([lista+0x170]: +0 o id do clube, 50 vagas de 0x1a2 bytes a partir de +4). Primeiro as vagas de quem saiu
/// do clube (id −1, o contador +4 e os bytes +0x1a0/+0x1a1 zerados); depois, do lado do clube no resultado (+0x190, 20
/// registros de 0x49, o lado de fora a 0x625), cada jogador que jogou (+4 ≠ −1; reservas além do 11º só com nota, +0x32)
/// ganha uma linha no histórico de 10 partidas da vaga dele (0x7bd700 com 1; sem vaga, a caixa da linha 0x2065): gols,
/// nota, data, adversário, placar, competição, o lado (0x4afa80) e as estatísticas; o tipo +0x2e (1 titular, 2 reserva,
/// 5/6 com a nota pelo +0x30) e a nota +0x15. Com o técnico do clube humano e as máscaras de atributos ligadas, o
/// técnico passa a conhecer o jogador (0x5b09d0, que sorteia).
pub(crate) unsafe extern "thiscall" fn player_search_record_scouted_club_match(this: usize, jogo: usize, resultado: usize) {
    let clube_obs = friendly_club_by_id(read_usize(this));
    for i in 0..0x32usize {
        let v = this + 4 + i * 0x1a2;
        let id = read_i32(v);
        if id == -1 {
            continue;
        }
        let p = staff_records_staff_by_id(id as usize);
        if p != 0 && read_i32(p) >= read_i32(0xae2364) {
            continue;
        }
        let c = read_usize(p + 0x39);
        if c != 0 && read_usize(c) == read_usize(this) {
            continue;
        }
        core::ptr::write_unaligned(v as *mut i32, -1);
        core::ptr::write_unaligned((v + 4) as *mut u16, 0);
        *((v + 0x1a0) as *mut u8) = 0;
        *((v + 0x1a1) as *mut u8) = 0;
    }
    // o lado do clube: 0 se ele é o +0xc do jogo (o adversário é o +0x10), senão 1 (adversário +0xc)
    let (lado, adversario): (i8, u32) = if read_usize(jogo + 0xc) == read_usize(this) {
        (0, read_usize(jogo + 0x10) as u32)
    } else {
        (1, read_usize(jogo + 0xc) as u32)
    };
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(FN_HUMAN_MANAGER_IS_HUMAN_CLUB);
    let quadro: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FOG_OF_WAR_GET_MANAGER_MAP);
    let conhecer: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_FOG_OF_WAR_MAP_RAISE_KNOWLEDGE);
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_PLAYER_SEARCH_FIND_OR_CLAIM_PLAYER_SLOT);
    let lado_do_jogo: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_COMP_UTIL_GET_FIXTURE_VENUE_SIDE);
    // 0x600a50: 5 pushes (clube, 0, 1, 0, 0); add esp, 0x14 -> cdecl; 0x5afde0: push técnico; mov ecx, [0xae2a60] ->
    // thiscall; 0x5b09d0: push 0; push jogador; mov ecx, eax -> thiscall, ret 8; 0x7bd700: push 1; push jogador; mov ecx,
    // this -> thiscall, ret 8; 0x4afa80: push jogo; push clube; add esp, 8 -> cdecl
    for j in 0..0x14i8 {
        let e = resultado + 0x190 + lado as usize * 0x625 + j as usize * 0x49;
        let id = read_i32(e);
        if id == -1 || read_i32(e + 4) == -1 {
            continue;
        }
        if j > 10 && read_u8(e + 0x32) == 0xff {
            continue;
        }
        let p = staff_records_staff_by_id(id as usize);
        if p != 0 && read_i32(p) >= read_i32(0xae2364) {
            continue;
        }
        if read_usize(clube_obs + club::MANAGER) != 0 && humano(clube_obs, 0, 1, 0, 0) != 0 && read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
            let q = quadro(read_usize(G_FOG_OF_WAR), read_usize(clube_obs + club::MANAGER));
            if q != 0 {
                conhecer(q, p, 0);
            }
        }
        let r = vaga_de(this, p, 1);
        if r == 0 {
            show_error_at(0xa701b0, 0x2065);
            return;
        }
        let quantos = read_u8(r + 0x1a1);
        core::ptr::write_unaligned((r + 4) as *mut u16, (read_i16(r + 4) as u16).wrapping_add(1));
        let mut k = read_u8(r + 0x1a0);
        if quantos == 0xa {
            // o histórico cheio: a linha mais antiga (+0x1a0) é a da vez, e o começo anda
            let n = k.wrapping_add(1);
            *((r + 0x1a0) as *mut u8) = if n == 0xa { 0 } else { n };
        } else {
            k = k.wrapping_add(quantos);
            if (k as i8) >= 0xa {
                k = k.wrapping_add(0xf6);
            }
            *((r + 0x1a1) as *mut u8) = quantos.wrapping_add(1);
        }
        let l = r.wrapping_add((k as i8 as isize * 0x29) as usize);
        let por = |o: usize, v: u8| *((l + o) as *mut u8) = v;
        por(0x17, read_u8(e + 0x27));
        por(0x18, read_u8(e + 0x22));
        core::ptr::write_unaligned((l + 6) as *mut u16, read_i16(jogo + 0x2a) as u16);
        core::ptr::write_unaligned((l + 8) as *mut u16, read_i16(jogo + 0x28) as u16);
        por(0x16, read_u8(e + 0x21));
        por(0x1b, read_u8(e + 0x35));
        por(0x1a, read_u8(e + 0x2d));
        por(0x19, read_u8(e + 0x2c));
        let (m, n) = if lado == 0 { (0x43, 0x44) } else { (0x44, 0x43) };
        por(0xe, read_u8(jogo + m));
        por(0xf, read_u8(jogo + n));
        core::ptr::write_unaligned((l + 0x10) as *mut u32, read_usize(jogo + 8) as u32);
        por(0x14, lado_do_jogo(friendly_club_by_id(read_usize(this)), jogo) as u8);
        for t in 0..10 {
            por(0x1c + t, read_u8(e + 0x3e + t));
        }
        por(0x26, read_u8(e + 0x28));
        por(0x27, read_u8(e + 0x25));
        por(0x28, read_u8(e + 0x26));
        por(0x29, read_u8(e + 0x29));
        por(0x2a, read_u8(e + 0x2a));
        por(0x2b, read_u8(e + 0x2b));
        por(0x2c, read_u8(e + 0x23));
        por(0x2d, read_u8(e + 0x24));
        por(0x2e, 0);
        let adv = if read_usize(jogo) == read_usize(KEY_CLUB_COMP_RESERVE) { adversario.wrapping_neg() } else { adversario };
        core::ptr::write_unaligned((l + 0xa) as *mut u32, adv);
        if j < 0xb {
            por(0x2e, 1);
            let a = read_u8(e + 0x34);
            if (a as i8) > -1 {
                por(0x15, a);
            } else if (read_u8(e + 0x30) as i8) > -1 {
                por(0x2e, 5);
                por(0x15, read_u8(e + 0x30));
            } else {
                por(0x15, read_u8(resultado + 0x4d));
            }
        } else {
            por(0x2e, 2);
            let c = read_u8(e + 0x32);
            if c == 0xff {
                por(0x15, 0);
            } else {
                let a = read_u8(e + 0x34);
                if (a as i8) > -1 {
                    por(0x15, a.wrapping_sub(c));
                } else if (read_u8(e + 0x30) as i8) > -1 {
                    por(0x2e, 6);
                    por(0x15, read_u8(e + 0x30).wrapping_sub(c));
                } else {
                    por(0x15, read_u8(resultado + 0x4d).wrapping_sub(c));
                }
            }
        }
    }
}

/// Comparer for `player_search_record_scouted_club_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7bd310: lado a lado nas 50 vagas (o registro inteiro). Com as máscaras ligadas e o técnico do clube
/// humano, a 0x5b09d0 grava o que se sabe do jogador e sorteia: aí só a original roda (ponytail: o caso de um humano
/// observando o próprio clube; a carreira automática não chega a esta rotina).
pub(crate) unsafe extern "thiscall" fn comparar_player_search_record_scouted_club_match(this: usize, jogo: usize, resultado: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(IDX_PLAYER_SEARCH_RECORD_SCOUTED_CLUB_MATCH);
    let clube_obs = friendly_club_by_id(read_usize(this));
    if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 && read_usize(clube_obs + club::MANAGER) != 0 {
        o(this, jogo, resultado);
        return;
    }
    let regioes = [(this, 4 + 0x32 * 0x1a2), (0xae24d0, 0x100), (0xb67a34, 4)];
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogo, resultado),
        || player_search_record_scouted_club_match(this, jogo, resultado));
    PLACAR.registrar(igual, || format!("player_search_record_scouted_club_match(clube {:#x}, jogo {:#x}) {:x?} x {:x?}", read_usize(this), jogo, mo, mr));
}

/// O ensaio da 0x7b5be0 (só no comparador): a cópia decide tudo até o relatório sem os efeitos (o histórico 0x7bd310 e o
/// relatório não rodam) e anota se chegaria ao relatório.
pub(crate) static mut SCOUT_MATCH_DRY_RUN: bool = false;
pub(crate) static mut SCOUT_MATCH_WOULD_REPORT: bool = false;

/// the matching scout list (type 4) watches the played match: the history of the followed club; within the dates, with
/// the followed player in the match or with the club asking for a player of either club, the list start moves to
/// tomorrow and every player of another club rated at least 15 (and asked for, followed or with a seen reputation below
/// 4000) becomes a report in the output
/// Original: `PLAYER_SEARCH::scout_match` (0x7b5be0, player_search.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x7b5be0 } (thiscall, ret 0xc, player_search.cpp, quadro SEH da lista local e das datas): `PLAYER_SEARCH::
/// scout_match(jogo, resultado, saida)`, pela partida observada (0x7f8c00). `this` é a lista de busca do olheiro: +0 o
/// dono, +0xa o tipo (4 = observar partidas), +0x170 o registro do clube observado, +0x174..+0x184 os nomes do olheiro,
/// +0x180 o id dele, +0x190 o clube acompanhado, +0x195 o jogador acompanhado, +0x1a5/+0x1ad o começo e o fim. Sem jogo
/// ou resultado: a caixa da linha 0xef1. Do tipo 4, com olheiro que confere (0x53e990) e resultado de partida jogada
/// (+0x4c zero): o clube acompanhado que joga recebe o histórico (0x7bd310); dentro das datas (antes do fim só com
/// jogador acompanhado), se o jogo é do jogador acompanhado, relata; senão a busca tem de cobrir a nação da competição
/// (0x7b7490) e o clube do olheiro tem de ter pedido (0x7f9690, 0x7f8d70 com 1) algum jogador de um dos dois clubes. O
/// relatório: o começo da lista vai a amanhã; dos dois lados (+0x1c0 / +0x1c0+0x625, 20 registros de 0x49), cada jogador
/// válido de outro clube (reservas além do 11º só com nota) é conhecido pelo dono (0x5b09d0, com as máscaras) e, pedido
/// (0x7f8d70 com 0), acompanhado ou com reputação vista abaixo de 4000 (0x5407e0) e nota de ao menos 15 (90 com o
/// resultado sem notas), vira um relatório (0x7b62a0, modo 0 acompanhado, 1 pedido, 2 com +5 ≥ 8) que entra na saída
/// (0x68d0d0, 0x67 bytes) se o byte +0x66 ficou ≥ 0. Devolve 1 com o relatório, senão 0.
pub(crate) unsafe extern "thiscall" fn player_search_scout_match(this: usize, jogo: usize, resultado: usize, saida: usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    // 0x68d020 / 0x68d040: lea ecx, local -> thiscall
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let r = scout_match_body(this, jogo, resultado, saida, l);
    soltar(l);
    r
}

/// Body of player_search_scout_match over a built local list: checks the scout/search and produces scouting reports for
/// the match.
pub(crate) unsafe fn scout_match_body(this: usize, jogo: usize, resultado: usize, saida: usize, l: usize) -> u32 {
    if jogo == 0 || resultado == 0 {
        show_error_at(0xa701b0, 0xef1);
        return 0;
    }
    let dono = staff_records_staff_by_id(read_usize(this));
    // a nação da competição do jogo (+0x18 → +0x38 → +0x1f) ou a do mandante (+0x1c → +0x53)
    let mut nacao = 0usize;
    let t = read_usize(jogo + 0x18);
    if t != 0 && read_usize(t + 0x38) != 0 && read_usize(read_usize(t + 0x38) + 0x1f) != 0 {
        nacao = read_usize(read_usize(t + 0x38) + 0x1f);
    }
    if nacao == 0 {
        nacao = read_usize(read_usize(jogo + 0x1c) + 0x53);
    }
    if read_u8(this + 0xa) != 4 {
        return 0;
    }
    let id_olheiro = read_i32(this + 0x180);
    if id_olheiro <= -1 {
        return 0;
    }
    let confere: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_STAFF_MATCHES_NAMES);
    // 0x53e990: 5 pushes; add esp, 0x14 -> cdecl
    if confere(staff_records_staff_by_id(id_olheiro as usize), read_usize(this + 0x174), read_usize(this + 0x178), read_usize(this + 0x17c), this + 0x184) == 0 {
        return 0;
    }
    let olheiro = staff_records_staff_by_id(read_i32(this + 0x180) as usize);
    if olheiro == 0 || read_u8(resultado + 0x4c) != 0 {
        return 0;
    }
    let acompanhado = read_i32(this + 0x190);
    if acompanhado > -1 && read_usize(this + 0x170) != 0 && (read_i32(jogo + 0xc) == acompanhado || read_i32(jogo + 0x10) == acompanhado) {
        if !SCOUT_MATCH_DRY_RUN {
            crate::comparacao::ramo("cqf_10 historico do clube");
            let historico: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_PLAYER_SEARCH_RECORD_SCOUTED_CLUB_MATCH);
            // 0x7bd310: push resultado; push jogo; mov ecx, [this+0x170] -> thiscall, ret 8
            historico(read_usize(this + 0x170), jogo, resultado);
        }
    }
    // as datas: hoje antes do começo (+0x1a5: dia, +0x1a7: ano) sai; antes do fim (+0x1ad/+0x1af) só com jogador
    // acompanhado; ano sem sinal, dia com sinal, como a original
    let (dia, ano) = (read_i16(G_CURRENT_DATE), read_i16(G_CURRENT_DATE_YEAR) as u16);
    let (dia_ini, ano_ini) = (read_i16(this + 0x1a5), read_i16(this + 0x1a7) as u16);
    if ano < ano_ini || (ano == ano_ini && dia < dia_ini) {
        return 0;
    }
    let (dia_fim, ano_fim) = (read_i16(this + 0x1ad), read_i16(this + 0x1af) as u16);
    let antes_do_fim = ano < ano_fim || (ano == ano_fim && dia < dia_fim);
    if antes_do_fim && read_i32(this + 0x195) == -1 {
        return 0;
    }
    let alvo = read_i32(this + 0x195);
    let do_alvo = alvo > -1 && read_i32(jogo) == alvo;
    if !do_alvo {
        let cobre: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_PLAYER_SEARCH_SCOUT_COVERS_NATION);
        // 0x7b7490: push nação; mov ecx, this -> thiscall, ret 4
        if cobre(this, nacao) == 0 {
            return 0;
        }
        let alvos: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_SCOUT_MANAGER_COLLECT_CLUB_SCOUT_TARGETS);
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
        let pedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_HAS_SCOUT_REQUEST);
        // 0x7f9690: push &lista; push clube do olheiro; mov ecx, [0xb74448] -> thiscall, ret 8; 0x68d1b0: lea ecx, lista
        // -> thiscall; 0x7f8d70: push 1; push clube; push jogador; mov ecx, [0xb74448] -> thiscall, ret 0xc
        alvos(read_usize(G_SCOUT_MANAGER), read_usize(olheiro + 0x39), l);
        let conta = read_i16(l);
        if conta == 0 {
            return 0;
        }
        core::ptr::write_unaligned((l + 0xa) as *mut usize, read_usize(l + 2)); // o cursor volta à cabeça
        let mut livre = true; // ebp da original: nenhum pedido achado
        // ponytail: a original usa o mesmo registrador (edi) para contar os itens e as 20 vagas de cada lado; depois de
        // um item de um dos clubes o contador vira o número de vagas olhadas na última volta (o laço de fora acaba antes)
        let mut i: i32 = 0;
        if conta > 0 {
            loop {
                let item = proximo(l);
                if item != 0 {
                    let (m, v) = (read_usize(jogo + 0x1c), read_usize(jogo + 0x20));
                    let c39 = read_usize(item + 0x39);
                    let c24 = read_usize(item + 0x24);
                    if c39 == m || c39 == v || c24 == m || c24 == v {
                        let mut lado: i8 = 0;
                        while livre {
                            i = 0;
                            let mut k = 0usize;
                            while livre {
                                let id = read_i32(resultado + 0x190 + lado as usize * 0x625 + k);
                                if id != -1 && pedido(read_usize(G_SCOUT_MANAGER), staff_records_staff_by_id(id as usize), read_usize(olheiro + 0x39), 1) != 0 {
                                    livre = false;
                                }
                                k += 0x49;
                                i += 1;
                                if k >= 0x5b4 {
                                    break;
                                }
                            }
                            lado += 1;
                            if lado >= 2 {
                                break;
                            }
                        }
                    }
                }
                i += 1;
                if i >= read_i16(l) as i32 {
                    break;
                }
            }
        }
        if livre {
            return 0;
        }
    }
    crate::comparacao::ramo("cqf_10 relatorio");
    // o começo da lista vai a amanhã (0x54c5b0 com 1, 0x417c10 grava em +0x1a5); o byte baixo do dword da data copiada
    // vira o contador dos dois lados e vai inteiro como 7º argumento da 0x7b62a0
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    // 0x54c5b0: push 1; push &tmp; mov ecx, 0xae2c90 -> thiscall, ret 8; 0x417c10: push data; push &copia; lea ecx,
    // [this+0x1a5] -> thiscall, ret 8
    let (mut tmp, mut copia) = ([0u32; 2], [0u32; 2]);
    let amanha = mais(G_CURRENT_DATE, tmp.as_mut_ptr() as usize, 1); // cqf10: amanhã
    if SCOUT_MATCH_DRY_RUN {
        SCOUT_MATCH_WOULD_REPORT = true;
        return 1;
    }
    copiar(this + 0x1a5, copia.as_mut_ptr() as usize, amanha);
    let mut marca = copia[0] & !0xff;
    let quadro: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FOG_OF_WAR_GET_MANAGER_MAP);
    let conhecer: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_FOG_OF_WAR_MAP_RAISE_KNOWLEDGE);
    let pedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_HAS_SCOUT_REQUEST);
    let injury_word_0c: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(FN_DATABASE_GET_STAFF_REPUTATION_FOR_VIEWER);
    let relatorio: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, usize, usize, u32) = core::mem::transmute(FN_PLAYER_SEARCH_REPORT_ON_PLAYER);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_LLIST_ADD_TAIL);
    // 0x5afde0: push dono; mov ecx, [0xae2a60] -> thiscall, ret 4; 0x5b09d0: push olheiro; push jogador; mov ecx, eax ->
    // thiscall, ret 8; 0x7f8d70: push 0; push clube; push jogador; mov ecx, [0xb74448] -> thiscall, ret 0xc; 0x5407e0:
    // push 0; push olheiro; push jogador; add esp, 0xc -> cdecl; 0x7b62a0: 7 pushes; mov ecx, this -> thiscall, ret 0x1c;
    // 0x68d0d0: push 0x67; push &registro; mov ecx, saida -> thiscall, ret 8
    // o registro local (0x67 bytes), reaproveitado entre os jogadores como na original, com as partes que o construtor da
    // original monta na entrada: a data em +0 (0x54a110) e os dois recordistas em +0x1f e +0x37 (0x450d40); o resto
    // começa zerado (na original, sobra da pilha que a 0x7b62a0 sobrescreve)
    let mut registro = [0u8; 0x68];
    let reg = registro.as_mut_ptr() as usize;
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let recordista: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_RECORD_HOLDER_CONSTRUCT);
    // lea ecx, local -> thiscall nas duas
    data_padrao(reg);
    recordista(reg + 0x1f);
    recordista(reg + 0x37);
    let clube_olheiro = read_usize(olheiro + 0x39);
    for lado in 0..2usize {
        let mut e = resultado + 0x1c0 + lado * 0x625;
        for j in 0..0x14i32 {
            let atual = e;
            e += 0x49;
            if j > 10 && read_u8(atual + 2) == 0xff {
                continue;
            }
            let id = read_i32(atual - 0x30);
            if id == -1 {
                continue;
            }
            let p = staff_records_staff_by_id(id as usize);
            if p != 0 && read_i32(p) >= read_i32(0xae2364) {
                continue;
            }
            if player_search_contract_club(p) == clube_olheiro || read_usize(p + 0x61) == 0 {
                continue;
            }
            if read_u8(G_ATTRIBUTE_MASKING_ENABLED) != 0 {
                let q = quadro(read_usize(G_FOG_OF_WAR), dono);
                if q != 0 {
                    conhecer(q, p, olheiro);
                }
            }
            let pediu = pedido(read_usize(G_SCOUT_MANAGER), p, clube_olheiro, 0);
            if pediu == 0 {
                let alvo = read_i32(this + 0x195);
                if !(alvo != -1 && read_i32(jogo) == alvo) && injury_word_0c(p, olheiro, 0) >= 0xfa0 { // cqf10: reputação vista
                    continue;
                }
            }
            // a nota: titulares (até o 11º) pela nota +4, senão +0, senão a do resultado (+0x4d); reservas descontam a
            // entrada (+2); resultado sem notas (+0x4c): titular vale 90
            let nota_de = |x: usize| -> u8 {
                let a = read_u8(x + 4);
                if (a as i8) > -1 { a } else if (read_u8(x) as i8) > -1 { read_u8(x) } else { read_u8(resultado + 0x4d) }
            };
            let nota: u8 = if j < 0xb {
                if read_u8(resultado + 0x4c) != 0 { 0x5a } else { nota_de(atual) }
            } else {
                let s = read_u8(atual + 2);
                if s == 0xff {
                    continue;
                }
                nota_de(atual).wrapping_sub(s)
            };
            if (nota as i8) < 0xf { // cqf10: a nota mínima
                continue;
            }
            let alvo = read_i32(this + 0x195);
            let modo: u32 = if alvo > -1 && alvo == read_i32(jogo) {
                0
            } else if pediu != 0 {
                1
            } else {
                if read_u8(atual + 5) < 8 {
                    continue;
                }
                2
            };
            crate::comparacao::ramo("cqf_10 avaliar jogador");
            let n = ((read_u8(atual + 5) as i32 * 0x28 - 0x104).wrapping_mul(nota as i8 as i32)) / 2;
            relatorio(this, p, reg, n, modo, jogo, resultado, marca);
            if (read_u8(reg + 0x66) as i8) > -1 {
                acrescentar(saida, reg, 0x67);
            }
        }
        marca = (marca & !0xff) | ((marca as u8).wrapping_add(1)) as u32;
    }
    1
}

/// Comparer for `player_search_scout_match` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7b5be0: o relatório sorteia (0x7b62a0, 0x5b09d0) e põe registros na saída (aloca): só a original
/// roda. Antes, a cópia em ensaio decide até o relatório sem os efeitos (sem o histórico e sem o relatório, com o estado
/// do sorteador guardado e devolvido) e diz a devolução e se o começo da lista vai a amanhã; a original tem de bater.
pub(crate) unsafe extern "thiscall" fn comparar_player_search_scout_match(this: usize, jogo: usize, resultado: usize, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(IDX_PLAYER_SEARCH_SCOUT_MATCH);
    if jogo == 0 || resultado == 0 {
        return o(this, jogo, resultado, saida);
    }
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = (read_usize(this + 0x1a5), read_usize(this + 0x1a9));
    SCOUT_MATCH_DRY_RUN = true;
    SCOUT_MATCH_WOULD_REPORT = false;
    let n = crate::aleatorio::sem_contar(|| player_search_scout_match(this, jogo, resultado, saida));
    SCOUT_MATCH_DRY_RUN = false;
    let relata = SCOUT_MATCH_WOULD_REPORT;
    estado.escrever();
    let previsto = if relata {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_DATE_ADD_DAYS);
        let mut t = [0u32; 2];
        let a = mais(G_CURRENT_DATE, t.as_mut_ptr() as usize, 1);
        (read_i16(a + 2) as u16, read_i16(a))
    } else {
        (read_i16(this + 0x1a7) as u16, read_i16(this + 0x1a5))
    };
    let r = o(this, jogo, resultado, saida);
    let depois = (read_i16(this + 0x1a7) as u16, read_i16(this + 0x1a5));
    PLACAR.registrar(r == n && depois == previsto, || format!("player_search_scout_match(lista {:#x}, jogo {:#x}) original={} ensaio={} começo antes {:x?} previsto {:x?} depois {:x?}",
        this, jogo, r, n, antes, previsto, depois));
    r
}



/// looks the player up in the search results (binary search by id) and returns 1 when found and not marked as removed
/// Original: `PLAYER_SEARCH::contains_player` (0x7af600, player_search.cpp, thiscall, ret 4).
///
/// [pt] { 0x7af600 } (thiscall, ret 4, player_search.cpp, quadro SEH só pelas duas datas locais, sem destrutor):
/// `PLAYER_SEARCH::contains_player(pessoa)`, da lista do olheiro do técnico humano (0x7f7490, que a avaliação dos
/// jogadores e os clubes interessados consultam a cada dia) e de telas. A chave local de 0x2d bytes começa com as duas
/// datas padrão (+0x10 e +0x18, 0x54a110); pessoa nula: a caixa da linha 0x33e e 0. Sem resultados (+0x1f6 = conta):
/// 0. A referência ao nome da pessoa na chave (0x7e4920: +0xc o id, o único campo que a comparação 0x7bce50 lê) e a
/// busca binária nos resultados (0x946464 com o vetor +0x1ee, a conta, 0x2d e 0x7bce50); achada e sem a marca de
/// removida (bit 0 de +0x20), 1; senão 0.
pub(crate) unsafe extern "thiscall" fn contains_player(this: usize, pessoa: usize) -> u32 {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let mut chave = [0u8; 0x30];
    let k = chave.as_mut_ptr() as usize;
    data_padrao(k + 0x10);
    data_padrao(k + 0x18);
    if pessoa == 0 {
        show_error_at(0xa701b0, 0x33e);
        return 0;
    }
    if read_usize(this + 0x1f6) == 0 {
        crate::comparacao::ramo("cqa_6 busca vazia");
        return 0; // cqa6: sem resultados, não está
    }
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_FILL_STAFF_NAME_REF);
    referencia(pessoa, k);
    let bsearch: unsafe extern "C" fn(usize, usize, usize, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let r = bsearch(k, read_usize(this + 0x1ee), read_usize(this + 0x1f6), 0x2d, 0x7bce50);
    if r == 0 {
        return 0;
    }
    if read_u8(r + 0x20) & 1 != 0 { // cqa6: a removida não conta
        crate::comparacao::ramo("cqa_6 removido");
        return 0;
    }
    crate::comparacao::ramo("cqa_6 achado");
    1 // cqa6: achado
}

/// Comparer for `contains_player` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7af600: pela devolução (só lê; as datas e a chave são locais). Pessoa nula: a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_contains_player(this: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_CONTAINS_PLAYER);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let r = o(this, pessoa);
    let n = contains_player(this, pessoa);
    PLACAR.registrar(r == n, || format!("contains_player(busca {:x}, pessoa {:x}) original={} rust={}", this, pessoa, r, n));
    r
}

/// SCOUT_MANAGER get_search(i, manager).
///
/// [pt] A busca `i` (0..9) do técnico `m` (0x7f6cd0: thiscall(gerência, i, técnico), ret 8; lê só o byte do índice).
pub(crate) unsafe fn get_search(this: usize, i: u8, m: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(FN_SCOUT_MANAGER_GET_SEARCH);
    f(this, i as u32, m)
}

/// PLAYER_SEARCH get_scout of the search.
///
/// [pt] O olheiro da busca (0x7ae950: thiscall(busca)).
pub(crate) unsafe fn search_scout(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_PLAYER_SEARCH_GET_SCOUT);
    f(l)
}

/// PLAYER_SEARCH set_scout(search, scout).
///
/// [pt] Põe o olheiro na busca (0x7ae8a0: thiscall(busca, olheiro), ret 4).
pub(crate) unsafe fn set_search_scout(l: usize, olheiro: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_PLAYER_SEARCH_SET_SCOUT);
    f(l, olheiro);
}



/// the player's slot among the 50 of the scouted club record; when claiming, the first free one gets the id and its
/// counters are zeroed
/// Original: `player_search_find_or_claim_player_slot` (0x7bd700, player_search.cpp, thiscall, ret 8).
///
/// [pt] { 0x7bd700 } (thiscall, ret 8, player_search.cpp): `player_search_find_or_claim_player_slot(pessoa, reservar)`, do
/// registro de 50 vagas (0x1a2 bytes a partir de +4: +0 o id, −1 livre) do clube observado, pela partida dele
/// (0x7bd310) e pelas telas (0x7f8610). A vaga com o id da pessoa; senão, com `reservar`, a primeira livre recebe o id
/// (pessoa nula: −1), zera a palavra +4 e os bytes +0x1a0 e +0x1a1 e é devolvida (nenhuma livre: a caixa da linha
/// 0x210d e 0); sem `reservar`, 0.
pub(crate) unsafe extern "thiscall" fn player_search_find_or_claim_player_slot(this: usize, pessoa: usize, reservar: u32) -> usize {
    if pessoa == 0 {
        // ponytail: a original lê o id da pessoa antes de conferir o ponteiro (cairia); fica com ela
        return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_PLAYER_SEARCH_FIND_OR_CLAIM_PLAYER_SLOT)(this, pessoa, reservar);
    }
    let id = read_usize(pessoa) as i32;
    let vaga = |i: i8| this + 4 + i as usize * 0x1a2;
    let mut livre: i8 = -1;
    for i in 0..0x32i8 {
        let v = read_usize(vaga(i)) as i32;
        if v == id {
            return vaga(i);
        }
        if reservar != 0 && livre == -1 && v == -1 {
            livre = i;
        }
    }
    if reservar == 0 {
        return 0;
    }
    if livre == -1 {
        show_error_at(0xa701b0, 0x210d);
        return 0;
    }
    let a = vaga(livre);
    core::ptr::write_unaligned(a as *mut i32, id);
    core::ptr::write_unaligned((a + 4) as *mut u16, 0);
    *((a + 0x1a0) as *mut u8) = 0;
    *((a + 0x1a1) as *mut u8) = 0;
    a
}

/// Comparer for `player_search_find_or_claim_player_slot` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x7bd700: sem `reservar`, pela devolução (só lê); com ele, lado a lado nas 50 vagas e na devolução.
pub(crate) unsafe extern "thiscall" fn comparar_player_search_find_or_claim_player_slot(this: usize, pessoa: usize, reservar: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(IDX_PLAYER_SEARCH_FIND_OR_CLAIM_PLAYER_SLOT);
    if pessoa == 0 {
        return o(this, pessoa, reservar);
    }
    if reservar == 0 {
        let r = o(this, pessoa, reservar);
        let n = player_search_find_or_claim_player_slot(this, pessoa, reservar);
        PLACAR.registrar(r == n, || format!("player_search_find_or_claim_player_slot(registro {:x}, pessoa {:x}) original={:x} rust={:x}", this, pessoa, r, n));
        return r;
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(this + 4, 0x32 * 0x1a2)], || o(this, pessoa, reservar), || player_search_find_or_claim_player_slot(this, pessoa, reservar));
    PLACAR.registrar(igual && r == n, || format!("player_search_find_or_claim_player_slot(registro {:x}, pessoa {:x}, reservar) original={:x} rust={:x} iguais={}", this, pessoa, r, n, igual));
    r
}



/// the rating the club looks for: the base from reputation, money, nation and league, by bands, and the adjustments for
/// squad size and wealth; at least 500
/// Original: `player_search_get_club_target_rating` (0x7bca90, player_search.cpp, cdecl).
///
/// [pt] { 0x7bca90 } (cdecl, player_search.cpp): `player_search_get_club_target_rating(clube, *base, *forcada)`: a nota que
/// o clube procura nos jogadores. A base (sem `forcada`): a reputação (+0x80), a do dinheiro (+0x77 × 0,43 − 430, até
/// 10000, se maior que a reputação), a da nação (+0x57: +0x69 × 500, se maior que reputação / 500) e a da liga (+0x1db
/// em [0xae23d0], 0x6b bytes: +0x69 × 600, idem), pesadas e / 16; metade no clube não profissional (+0x64 ≠ 1). A
/// nota sai da base por faixas (até 3000, 5000, 7000, 9000 e acima). Sem `forcada`, pelo elenco (0x53c590) e pela
/// faixa de riqueza das finanças (0x5a0590 de [0xae2a38], 0x5986b0) nos clubes com nação em [0xadadfc]: elenco curto
/// (< 16) metade; < 20 metade ou 2/3 nos pobres; senão encolhe pela falta de jogadores frente à nação (+0x69 × 1,35 +
/// 20) e a riqueza puxa (−2..2: × 0,909 ou − 1000, × 0,952 ou − 500, nada, × 1,0125 ou + 125, × 1,025 ou + 250).
/// No mínimo 500. A base vai em `*base` (se não nulo). As contas de ponto flutuante em f64 (o x87 do jogo em 53 bits).
pub(crate) unsafe extern "C" fn player_search_get_club_target_rating(clube: usize, base: *mut i32, forcada: *const i16) -> i32 {
    let contar: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(FN_DATABASE_COUNT_CLUB_SQUAD);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_FINANCETABLE_GET_CLUB_FINANCE);
    let riqueza: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(FN_FINANCE_GET_WEALTH_BAND);
    let li = read_i32(clube + club::EURO_FLAG);
    let liga = if li > -1 { (read_i32(G_CLUB_COMP_TABLE) as usize).wrapping_add((li as usize).wrapping_mul(0x6b)) } else { 0 };
    let nacao = read_i32(clube + club::DIVISION) as usize;
    let mut esi: i32;
    if !forcada.is_null() {
        esi = *forcada as i32;
    } else {
        let rep = read_i16(clube + club::REPUTATION) as i32;
        let t = match_pl_ftol(read_i32(clube + club::MIN_ATTENDANCE) as f64 * read_f64_at(0x96e2b0) - read_f64_at(0x96e2a8)) as i16 as i32; // 0x96e2b0 = 0.43, 0x96e2a8 = 430
        let ebx = (if rep > t { rep } else if t > 10000 { 10000 } else { t }) as i16 as i32;
        let mut n = rep;
        let mut l = rep;
        if nacao != 0 {
            let c = read_i16(nacao + 0x69) as i32;
            if c > rep / 500 {
                n = c * 500;
                l = n;
            }
        }
        if liga != 0 {
            let c = read_i16(liga + 0x69) as i32;
            if c > rep / 500 {
                l = c * 600;
            }
        }
        let soma = if n < 8000 {
            crate::comparacao::ramo("cpu_13 base pela reputação");
            n.wrapping_mul(3).wrapping_add(ebx).wrapping_add(l.wrapping_add(rep.wrapping_mul(2)).wrapping_mul(4))
        } else {
            ebx.wrapping_add(rep.wrapping_mul(7)).wrapping_add(n.wrapping_add(l).wrapping_mul(4))
        };
        esi = soma / 16; // cpu13: a base
    }
    if *((clube + club::PROFESSIONAL_STATUS) as *const u8) != 1 {
        esi /= 2;
    }
    let mut edi: i32 = if esi > 9000 {
        esi.wrapping_mul(3).wrapping_add(7500)
    } else if esi > 7000 {
        match_pl_ftol(esi as f64 * read_f64_at(0x96d688) - read_f64_at(0x96d760)) // 0x96d688 = 4.75, 0x96d760 = 8250
    } else if esi > 5000 {
        match_pl_ftol(esi as f64 * read_f64_at(0x968e18) + read_f64_at(0x9690b8)) // 0x968e18 = 2.5, 0x9690b8 = 7500
    } else if esi > 3000 {
        crate::comparacao::ramo("cpu_13 band de 3000 a 5000");
        esi.wrapping_mul(5).wrapping_sub(5000) // cpu13: a faixa do meio
    } else {
        match_pl_ftol(esi as f64 * read_f64_at(0x969018) + read_f64_at(0x9688e8)) // 0x969018 = 1.6, 0x9688e8 = 5000
    };
    if forcada.is_null() {
        let elenco = contar(clube, 0, 0);
        let com_nacao = |c: usize| c != 0 && read_i32((read_i32(G_COMP_OBJECT_TABLE) as usize).wrapping_add(read_i32(c) as usize * 4)) != 0;
        if elenco < 16 && com_nacao(nacao) {
            edi /= 2;
        } else if elenco < 20 && com_nacao(nacao) {
            let fin = financas(read_i32(G_FINANCE_TABLE) as usize, clube);
            if fin != 0 {
                match riqueza(fin) as i8 {
                    -2 => edi /= 2,
                    -1 => edi = match_pl_ftol(edi as f64 * read_f64_at(0x96a670)), // 0x96a670 = 0.6666666667
                    _ => {}
                }
            }
        } else {
            let nacao = read_i32(clube + club::DIVISION) as usize;
            if com_nacao(nacao) {
                let t2 = match_pl_ftol(read_i16(nacao + 0x69) as i32 as f64 * read_f64_at(0x969588) + read_f64_at(0x968ea8)) as u8; // 0x969588 = 1.35, 0x968ea8 = 20
                let falta = (elenco as u8).wrapping_sub(t2) as i8;
                if falta < 0 {
                    let v = if falta > -10 { falta as i32 } else { -10 };
                    edi = match_pl_ftol((v as f64 * read_f64_at(0x967888) + read_f64_at(0x967890)) * edi as f64); // 0x967888 = 0.05, 0x967890 = 1
                }
            }
            let fin = financas(read_i32(G_FINANCE_TABLE) as usize, clube);
            if fin != 0 {
                let v = edi as f64;
                match riqueza(fin) as i8 {
                    -2 => {
                        let (x, y) = (v * read_f64_at(0x96d718), edi.wrapping_sub(1000) as f64); // 0x96d718 = 0.9090909091
                        edi = match_pl_ftol(if x < y { x } else { y });
                    }
                    -1 => {
                        let (x, y) = (v * read_f64_at(0x96e2a0), edi.wrapping_sub(500) as f64); // 0x96e2a0 = 0.9523809524
                        edi = match_pl_ftol(if x < y { x } else { y });
                    }
                    1 => {
                        let (x, y) = (v * read_f64_at(0x96d6b8), edi.wrapping_add(125) as f64); // 0x96d6b8 = 1.0125
                        edi = match_pl_ftol(if x > y { x } else { y });
                    }
                    2 => {
                        crate::comparacao::ramo("cpu_13 clube rico");
                        let (x, y) = (v * read_f64_at(0x968fa0), edi.wrapping_add(250) as f64); // cpu13: o rico; 0x968fa0 = 1.025
                        edi = match_pl_ftol(if x > y { x } else { y });
                    }
                    _ => {}
                }
            }
        }
    }
    if edi < 500 {
        edi = 500;
    }
    if !base.is_null() {
        *base = esi;
    }
    edi
}

/// match entry i (0x29 bytes) of the tracked player's ring of 10, or 0 past the count
/// Original: `player_search_slot_get_match_entry` (0x7bd830, player_search.cpp, thiscall, ret 4).
///
/// [pt] { 0x7bd830 } (thiscall, ret 4, player_search.cpp): `player_search_slot_get_match_entry(i)`: o jogo `i` (0x29
/// bytes, a partir de +6) do anel de 10 que começa em +0x1a0 (byte com sinal), ou 0 se `i` ≥ a conta (+0x1a1).
pub(crate) unsafe extern "thiscall" fn player_search_slot_get_match_entry(this: usize, i: u32) -> usize {
    let i = i as u8 as i8;
    if i >= *((this + 0x1a1) as *const i8) {
        return 0;
    }
    let mut k = *((this + 0x1a0) as *const i8) as i32 + i as i32;
    if k >= 10 {
        k -= 10;
    }
    (this as isize + k as isize * 0x29 + 6) as usize
}

/// the sum of the goals of the matches stored in the ring
/// Original: `player_search_slot_sum_goals` (0x7bd870, player_search.cpp, thiscall, ret 0).
///
/// [pt] { 0x7bd870 } (thiscall, ret 0, player_search.cpp): `player_search_slot_sum_goals()`: a soma (16 bits) do byte com
/// sinal +0x16 dos jogos do anel (+0x1a1 deles, a partir de +0x1a0, dando a volta em 10).
pub(crate) unsafe extern "thiscall" fn player_search_slot_sum_goals(this: usize) -> u16 {
    let n = *((this + 0x1a1) as *const i8);
    let mut k = *((this + 0x1a0) as *const u8);
    let mut s: u16 = 0;
    for _ in 0..n.max(0) {
        let g = *((this as isize + (k as i8) as isize * 0x29 + 0x16) as *const i8);
        s = s.wrapping_add(g as i16 as u16);
        k = k.wrapping_add(1);
        if k == 10 {
            k = 0;
        }
    }
    s
}

/// Comparer for `player_search_get_club_target_rating` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura (lê o clube, o elenco e as finanças; grava só `*base`): as duas sobre bases próprias.
pub(crate) unsafe extern "C" fn comparar_player_search_get_club_target_rating(clube: usize, base: *mut i32, forcada: *const i16) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, *mut i32, *const i16) -> i32>(IDX_PLAYER_SEARCH_GET_CLUB_TARGET_RATING);
    let (mut bo, mut br) = (0x7fff_0001_i32, 0x7fff_0002_i32);
    let r = o(clube, &mut bo, forcada);
    let n = player_search_get_club_target_rating(clube, &mut br, forcada);
    PLACAR.registrar(r == n && bo == br, || format!("player_search_get_club_target_rating({:x}, forcada {}) {}/{} base {}/{}", clube, if forcada.is_null() { -1 } else { *forcada as i32 }, r, n, bo, br));
    if !base.is_null() {
        *base = bo;
    }
    r
}

/// Comparer for `player_search_slot_get_match_entry` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura: compara a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_player_search_slot_get_match_entry(this: usize, i: u32) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(IDX_PLAYER_SEARCH_SLOT_GET_MATCH_ENTRY)(this, i);
    let n = player_search_slot_get_match_entry(this, i);
    PLACAR.registrar(r == n, || format!("player_search_slot_get_match_entry({:x}, {}) {:x}/{:x}", this, i as u8 as i8, r, n));
    r
}

/// Comparer for `player_search_slot_sum_goals` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: pura: compara a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_player_search_slot_sum_goals(this: usize) -> u16 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize) -> u16>(IDX_PLAYER_SEARCH_SLOT_SUM_GOALS)(this);
    let n = player_search_slot_sum_goals(this);
    PLACAR.registrar(r == n, || format!("player_search_slot_sum_goals({:x}) {}/{}", this, r, n));
    r
}



/// the person at index +0x180 when it still matches the stored names and date (0x53e990); else 0
/// Original: `PLAYER_SEARCH::get_scout` (0x7ae950, player_search.cpp, thiscall).
///
/// [pt] { 0x7ae950 } (thiscall, player_search.cpp): `PLAYER_SEARCH::get_scout`: o olheiro da busca: o índice em +0x180 (> −1)
/// no vetor de pessoas [0xae23bc] (110 bytes), se ainda confere com os nomes e a data guardados (0x53e990 com +0x174,
/// +0x178, +0x17c e a data em +0x184); senão 0.
pub(crate) unsafe extern "thiscall" fn get_scout(this: usize) -> usize {
    let i = read_u32(this + 0x180) as i32;
    if i <= -1 {
        return 0;
    }
    let pessoa = (read_u32(G_STAFF_TABLE) as usize).wrapping_add((i as usize).wrapping_mul(110));
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(FN_DATABASE_STAFF_MATCHES_NAMES);
    if confere(pessoa, read_u32(this + 0x174), read_u32(this + 0x178), read_u32(this + 0x17c), this + 0x184) == 0 {
        return 0; // a vaga mudou de pessoa
    }
    pessoa
}



/// Rival club's manager (+0xcf) passes the draws: random(+0x5c) == 0 and random(+0x57) > 5.
///
/// [pt] O técnico (+0xcf) de um clube rival passa nos sorteios (sorteio(+0x5c) = 0 e sorteio(+0x57) > 5)?
pub(crate) unsafe fn rival_manager_passes_draws(c: usize) -> bool {
    friendly_random(*((read_u32(c + 0xcf) as usize + 0x5c) as *const i8) as i32) == 0
        && friendly_random(*((read_u32(c + 0xcf) as usize + 0x57) as *const i8) as i32) > 5
}


// a 0x53e990 só lê: compara o valor
comparar_valor!(comparar_get_scout, get_scout, IDX_GET_SCOUT, () -> usize);





/// stores the player search slot (0 to 9)
/// Original: `PLAYER_SEARCH::set_slot_index` (0x7ae640, player_search.cpp, thiscall).
///
/// [pt] { 0x7ae640 } (thiscall, ret 4, player_search.cpp): grava a vaga `vaga` (0 a 9) da busca em +4 e
/// devolve 1. Fora da faixa: a original (caixa de erro, devolve 0).
pub(crate) unsafe extern "thiscall" fn set_slot_index(busca: usize, vaga: i8) -> u32 {
    if !(0..=9).contains(&vaga) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_SET_SLOT_INDEX)(busca, vaga);
    }
    *((busca + 4) as *mut i8) = vaga;
    1
}

/// stores the player search list type (1 to 4)
/// Original: `PLAYER_SEARCH::set_list_type` (0x7ae6e0, player_search.cpp, thiscall).
///
/// [pt] { 0x7ae6e0 } (thiscall, ret 4, player_search.cpp): grava o tipo de lista `tipo` (1 a 4) da busca em
/// +0xa e devolve 1. Fora da faixa: a original (caixa de erro, devolve 0).
pub(crate) unsafe extern "thiscall" fn set_list_type(busca: usize, tipo: i8) -> u32 {
    if !(1..=4).contains(&tipo) {
        return original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_SET_LIST_TYPE)(busca, tipo);
    }
    *((busca + 0xa) as *mut i8) = tipo;
    1
}


/// Comparer for `set_slot_index` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_slot_index(busca: usize, vaga: i8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_SET_SLOT_INDEX);
    if !(0..=9).contains(&vaga) {
        return o(busca, vaga);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(busca + 4, 1)], || o(busca, vaga), || set_slot_index(busca, vaga));
    PLACAR.registrar(igual && ro == rr, || format!("set_slot_index({}) original={} rust={} {:x?} x {:x?}", vaga, ro, rr, mo, mr));
    ro
}

/// Comparer for `set_list_type` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_list_type(busca: usize, tipo: i8) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i8) -> u32>(IDX_SET_LIST_TYPE);
    if !(1..=4).contains(&tipo) {
        return o(busca, tipo);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(busca + 0xa, 1)], || o(busca, tipo), || set_list_type(busca, tipo));
    PLACAR.registrar(igual && ro == rr, || format!("set_list_type({}) original={} rust={} {:x?} x {:x?}", tipo, ro, rr, mo, mr));
    ro
}

/// new scout: stores the ids (0x7e4920) or −1, and the nation, region and competition assignments go back to −1;
/// returns 1
/// Original: `PLAYER_SEARCH::set_scout` (0x7ae8a0, player_search.cpp, thiscall, ret 4).
///
/// [pt] { 0x7ae8a0 } (thiscall, ret 4, player_search.cpp): `PLAYER_SEARCH::set_scout(olheiro)`: se o olheiro muda (o atual
/// pela 0x7ae950), grava os ids dele em +0x174 (0x7e4920) ou, sem olheiro, o índice +0x180 = −1; e as missões de nação
/// (+0x18c), região (+0x194) e competição (+0x195) voltam a −1. Devolve 1.
pub(crate) unsafe extern "thiscall" fn set_scout(this: usize, olheiro: usize) -> u32 {
    if olheiro == get_scout(this) {
        return 1; // o mesmo olheiro
    }
    if olheiro != 0 {
        let gravar: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(FN_FILL_STAFF_NAME_REF);
        gravar(olheiro, this + 0x174);
    } else {
        core::ptr::write_unaligned((this + 0x180) as *mut u32, u32::MAX);
    }
    core::ptr::write_unaligned((this + 0x18c) as *mut u32, u32::MAX);
    *((this + 0x194) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x195) as *mut u32, u32::MAX);
    1
}

/// Comparer for `set_scout` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo olheiro e as missões (+0x174 a +0x199; a 0x53e990 só lê).
pub(crate) unsafe extern "thiscall" fn comparar_set_scout(this: usize, olheiro: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SET_SCOUT);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x174, 0x25)], || o(this, olheiro), || set_scout(this, olheiro));
    PLACAR.registrar(igual && r == n, || format!("set_scout({:x}) {:x?} x {:x?}", olheiro, mo, mr));
    r
}

/// stores the human manager id in the player search
/// Original: `PLAYER_SEARCH::set_manager` (0x7ae570, player_search.cpp, thiscall).
///
/// [pt] { 0x7ae570 } (thiscall, ret 4, player_search.cpp): grava na busca (+0) o id do técnico humano
/// `pessoa` e devolve 1. Pessoa nula ou que não é humana (id abaixo de [0xae2364] − 16): a original
/// (caixa de erro, 0).
pub(crate) unsafe extern "thiscall" fn set_manager(busca: usize, pessoa: usize) -> u32 {
    if pessoa == 0 || *(pessoa as *const i32) < *(G_STAFF_COUNT as *const i32) - 0x10 {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SET_MANAGER)(busca, pessoa);
    }
    *(busca as *mut i32) = *(pessoa as *const i32);
    1
}

/// Comparer for `set_manager` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_set_manager(busca: usize, pessoa: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SET_MANAGER);
    if pessoa == 0 || *(pessoa as *const i32) < *(G_STAFF_COUNT as *const i32) - 0x10 {
        return o(busca, pessoa);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(busca, 4)], || o(busca, pessoa), || set_manager(busca, pessoa));
    PLACAR.registrar(igual && ro == rr, || format!("set_manager original={} rust={} {:x?} x {:x?}", ro, rr, mo, mr));
    ro
}

/// with the scout checked and some assignment set (+0x190, +0x18c, +0x194, +0x195, +0x199, +0x19d), copies the six and
/// returns 1; else 0
/// Original: `PLAYER_SEARCH::get_scout_assignment` (0x7aefd0, player_search.cpp, thiscall, ret 0x18).
///
/// [pt] { 0x7aefd0 } (thiscall, ret 0x18, player_search.cpp): `PLAYER_SEARCH::get_scout_assignment(clube, nacao, regiao,
/// comp, base, adversario)`: com o olheiro conferido, se alguma das missões está marcada (clube +0x190, nação +0x18c,
/// região +0x194 byte, competição +0x195 acima de −1; base +0x199, adversário +0x19d não nulos), copia as seis para
/// as saídas e devolve 1; nenhuma: 0. Olheiro que não confere: a original (caixa "Error" e 0).
pub(crate) unsafe extern "thiscall" fn get_scout_assignment(this: usize, clube: *mut u32, nacao: *mut u32, regiao: *mut u8, comp: *mut u32, base: *mut u32, adversario: *mut u32) -> u32 {
    if check_search_scout(this).is_none() {
        return original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, *mut u32, *mut u32, *mut u32) -> u32>(IDX_GET_SCOUT_ASSIGNMENT)(this, clube, nacao, regiao, comp, base, adversario); // caixa: só a original
    }
    let marcada = read_usize(this + 0x190) as i32 > -1
        || read_usize(this + 0x18c) as i32 > -1
        || *((this + 0x194) as *const i8) > -1
        || read_usize(this + 0x195) as i32 > -1 // a competição
        || read_usize(this + 0x199) != 0
        || read_usize(this + 0x19d) != 0;
    if !marcada {
        return 0;
    }
    clube.write_unaligned(read_usize(this + 0x190) as u32);
    nacao.write_unaligned(read_usize(this + 0x18c) as u32);
    *regiao = *((this + 0x194) as *const u8);
    comp.write_unaligned(read_usize(this + 0x195) as u32);
    base.write_unaligned(read_usize(this + 0x199) as u32);
    adversario.write_unaligned(read_usize(this + 0x19d) as u32);
    1
}

/// Comparer for `get_scout_assignment` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelas seis saídas (a 0x53e990 só lê); com a caixa, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_get_scout_assignment(this: usize, clube: *mut u32, nacao: *mut u32, regiao: *mut u8, comp: *mut u32, base: *mut u32, adversario: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, *mut u32, *mut u32, *mut u32) -> u32>(IDX_GET_SCOUT_ASSIGNMENT);
    if check_search_scout(this).is_none() {
        return o(this, clube, nacao, regiao, comp, base, adversario);
    }
    let reg = [(clube as usize, 4), (nacao as usize, 4), (regiao as usize, 1), (comp as usize, 4), (base as usize, 4), (adversario as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, clube, nacao, regiao, comp, base, adversario),
        || get_scout_assignment(this, clube, nacao, regiao, comp, base, adversario));
    PLACAR.registrar(igual && r == n, || format!("get_scout_assignment original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// whether the scout search (type 4, scout checked, with a club) covers the nation: by the set assignments (club,
/// nation, region, base) or, without one, by the region of the club's nation, the scout's nation and the club's scouts
/// Original: `PLAYER_SEARCH::scout_covers_nation` (0x7b7490, player_search.cpp, thiscall, ret 4).
///
/// [pt] { 0x7b7490 } (thiscall, ret 4, player_search.cpp): `PLAYER_SEARCH::scout_covers_nation(nacao)`: a busca de olheiro
/// (tipo +0xa = 4) com o olheiro conferido (índice +0x180 > −1 e 0x53e990 com os nomes +0x174..+0x17c e a data +0x184)
/// e com clube (+0x39) cobre a `nacao`? Com missão marcada (0x7aefd0, que copia as missões +0x18c..+0x19d sobre elas
/// mesmas): a nação do clube da missão (+0x53 do clube +0x190), a nação da missão (+0x18c = id +0 da nação), a região
/// (+0x194, byte com sinal, = +0x76 da nação), ou com a base (+0x199) a mesma região. Sem missão: as duas nações com
/// +0x7f = 2, a mesma região (+0x76) da nação do clube, a nação não estrangeira para o olheiro (0x53d980 com 1) ou os
/// olheiros do clube cobrindo a nação (0x7f9190 do [0xb74448]). Devolve 1 ou 0.
pub(crate) unsafe extern "thiscall" fn scout_covers_nation(this: usize, nacao: usize) -> u32 {
    let mut pessoa = 0usize;
    let i = read_usize(this + 0x180) as i32;
    if i > -1 {
        let confere: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_DATABASE_STAFF_MATCHES_NAMES);
        let p = read_usize(G_STAFF_TABLE).wrapping_add((i as usize).wrapping_mul(staff::SIZE));
        if confere(p, read_usize(this + 0x174), read_usize(this + 0x178), read_usize(this + 0x17c), this + 0x184) != 0 {
            pessoa = read_usize(G_STAFF_TABLE).wrapping_add(read_usize(this + 0x180).wrapping_mul(staff::SIZE));
        }
    }
    if read_u8(this + 0xa) != 4 || pessoa == 0 || nacao == 0 {
        return 0;
    }
    let clube = read_usize(pessoa + staff::CLUB_JOB);
    if clube == 0 {
        return 0;
    }
    let missao: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(FN_PLAYER_SEARCH_GET_SCOUT_ASSIGNMENT);
    if missao(this, this + 0x190, this + 0x18c, this + 0x194, this + 0x195, this + 0x199, this + 0x19d) != 0 {
        let c = read_usize(this + 0x190) as i32;
        if c > -1 && read_usize(read_usize(G_CLUB_TABLE).wrapping_add((c as usize).wrapping_mul(club::SIZE)) + 0x53) == nacao {
            return 1;
        }
        let n = read_usize(this + 0x18c) as i32;
        if n > -1 && read_usize(nacao) as i32 == n {
            return 1;
        }
        let r = read_u8(this + 0x194);
        if (r as i8) > -1 && read_u8(nacao + nation::ACTUAL_REGION) == r {
            return 1;
        }
        // a original compara de novo o mesmo byte quando há base
        if read_usize(this + 0x199) != 0 && read_u8(nacao + nation::ACTUAL_REGION) == r {
            return 1;
        }
        return 0;
    }
    let nc = read_usize(clube + club::NATION);
    if read_u8(nc + 0x7f) == 2 && read_u8(nacao + nation::GROUP_MEMBERSHIP) == 2 {
        return 1;
    }
    if read_u8(nc + 0x76) == read_u8(nacao + nation::ACTUAL_REGION) {
        crate::comparacao::ramo("cpn_7 mesma regiao");
        return 1; // a mesma região da nação do clube
    }
    let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
    if estrangeira(pessoa, nacao, 1) == 0 {
        crate::comparacao::ramo("cpn_7 nacao do olheiro");
        return 1; // a nação não é estrangeira para o olheiro
    }
    let cobrem: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(FN_SCOUT_MANAGER_CAN_CLUB_SCOUT_NATION);
    (cobrem(read_usize(G_SCOUT_MANAGER), nacao, clube) != 0) as u32
}

/// Comparer for `scout_covers_nation` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador: lado a lado pelo valor e pelas missões +0x18c..+0x1a0 (a 0x7aefd0 as copia sobre elas mesmas).
pub(crate) unsafe extern "thiscall" fn comparar_scout_covers_nation(this: usize, nacao: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u32>(IDX_SCOUT_COVERS_NATION);
    let reg = [(this + 0x18c, 0x15)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, nacao), || scout_covers_nation(this, nacao));
    PLACAR.registrar(igual && r == n, || format!("scout_covers_nation(nacao {:#x}) original={} rust={} {:x?} x {:x?}", nacao, r, n, mo, mr));
    r
}
