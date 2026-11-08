// ---------- %%LEVA%%a leva: o olheiro na partida observada e o histórico do clube observado (player_search.cpp) ----------

unsafe fn cqf10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqf10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqf10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf10_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf10_pessoa(id: usize) -> usize {
    cqf10_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqf10_clube(id: usize) -> usize {
    cqf10_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O clube do contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) ou 0.
unsafe fn player_search_contract_club(pessoa: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    // 0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let c = contract_of(cqf10_d(0xae1050), pessoa);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    if c != 0 { cqf10_clube(cqf10_d(c + 4)) } else { 0 }
}

/// { 0x7bd310 } (thiscall, ret 8, player_search.cpp): `player_search_record_scouted_club_match(jogo, resultado)`, pela
/// partida observada (0x7b5be0) quando a lista acompanha um clube (+0x190) que joga nela. `this` é o registro do clube
/// observado ([lista+0x170]: +0 o id do clube, 50 vagas de 0x1a2 bytes a partir de +4). Primeiro as vagas de quem saiu
/// do clube (id −1, o contador +4 e os bytes +0x1a0/+0x1a1 zerados); depois, do lado do clube no resultado (+0x190, 20
/// registros de 0x49, o lado de fora a 0x625), cada jogador que jogou (+4 ≠ −1; reservas além do 11º só com nota, +0x32)
/// ganha uma linha no histórico de 10 partidas da vaga dele (0x7bd700 com 1; sem vaga, a caixa da linha 0x2065): gols,
/// nota, data, adversário, placar, competição, o lado (0x4afa80) e as estatísticas; o tipo +0x2e (1 titular, 2 reserva,
/// 5/6 com a nota pelo +0x30) e a nota +0x15. Com o técnico do clube humano e as máscaras de atributos ligadas, o
/// técnico passa a conhecer o jogador (0x5b09d0, que sorteia).
unsafe extern "thiscall" fn player_search_record_scouted_club_match(this: usize, jogo: usize, resultado: usize) {
    let clube_obs = cqf10_clube(cqf10_d(this));
    for i in 0..0x32usize {
        let v = this + 4 + i * 0x1a2;
        let id = cqf10_i(v);
        if id == -1 {
            continue;
        }
        let p = cqf10_pessoa(id as usize);
        if p != 0 && cqf10_i(p) >= cqf10_i(0xae2364) {
            continue;
        }
        let c = cqf10_d(p + 0x39);
        if c != 0 && cqf10_d(c) == cqf10_d(this) {
            continue;
        }
        core::ptr::write_unaligned(v as *mut i32, -1);
        core::ptr::write_unaligned((v + 4) as *mut u16, 0);
        *((v + 0x1a0) as *mut u8) = 0;
        *((v + 0x1a1) as *mut u8) = 0;
    }
    // o lado do clube: 0 se ele é o +0xc do jogo (o adversário é o +0x10), senão 1 (adversário +0xc)
    let (lado, adversario): (i8, u32) = if cqf10_d(jogo + 0xc) == cqf10_d(this) {
        (0, cqf10_d(jogo + 0x10) as u32)
    } else {
        (1, cqf10_d(jogo + 0xc) as u32)
    };
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let quadro: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5afde0usize);
    let conhecer: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x5b09d0usize);
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7bd700usize);
    let lado_do_jogo: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x4afa80usize);
    // 0x600a50: 5 pushes (clube, 0, 1, 0, 0); add esp, 0x14 -> cdecl; 0x5afde0: push técnico; mov ecx, [0xae2a60] ->
    // thiscall; 0x5b09d0: push 0; push jogador; mov ecx, eax -> thiscall, ret 8; 0x7bd700: push 1; push jogador; mov ecx,
    // this -> thiscall, ret 8; 0x4afa80: push jogo; push clube; add esp, 8 -> cdecl
    for j in 0..0x14i8 {
        let e = resultado + 0x190 + lado as usize * 0x625 + j as usize * 0x49;
        let id = cqf10_i(e);
        if id == -1 || cqf10_i(e + 4) == -1 {
            continue;
        }
        if j > 10 && cqf10_b(e + 0x32) == 0xff {
            continue;
        }
        let p = cqf10_pessoa(id as usize);
        if p != 0 && cqf10_i(p) >= cqf10_i(0xae2364) {
            continue;
        }
        if cqf10_d(clube_obs + 0xcf) != 0 && humano(clube_obs, 0, 1, 0, 0) != 0 && cqf10_b(0x9cc270) != 0 {
            let q = quadro(cqf10_d(0xae2a60), cqf10_d(clube_obs + 0xcf));
            if q != 0 {
                conhecer(q, p, 0);
            }
        }
        let r = vaga_de(this, p, 1);
        if r == 0 {
            show_error_at(0xa701b0, 0x2065);
            return;
        }
        let quantos = cqf10_b(r + 0x1a1);
        core::ptr::write_unaligned((r + 4) as *mut u16, (cqf10_w(r + 4) as u16).wrapping_add(1));
        let mut k = cqf10_b(r + 0x1a0);
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
        por(0x17, cqf10_b(e + 0x27));
        por(0x18, cqf10_b(e + 0x22));
        core::ptr::write_unaligned((l + 6) as *mut u16, cqf10_w(jogo + 0x2a) as u16);
        core::ptr::write_unaligned((l + 8) as *mut u16, cqf10_w(jogo + 0x28) as u16);
        por(0x16, cqf10_b(e + 0x21));
        por(0x1b, cqf10_b(e + 0x35));
        por(0x1a, cqf10_b(e + 0x2d));
        por(0x19, cqf10_b(e + 0x2c));
        let (m, n) = if lado == 0 { (0x43, 0x44) } else { (0x44, 0x43) };
        por(0xe, cqf10_b(jogo + m));
        por(0xf, cqf10_b(jogo + n));
        core::ptr::write_unaligned((l + 0x10) as *mut u32, cqf10_d(jogo + 8) as u32);
        por(0x14, lado_do_jogo(cqf10_clube(cqf10_d(this)), jogo) as u8);
        for t in 0..10 {
            por(0x1c + t, cqf10_b(e + 0x3e + t));
        }
        por(0x26, cqf10_b(e + 0x28));
        por(0x27, cqf10_b(e + 0x25));
        por(0x28, cqf10_b(e + 0x26));
        por(0x29, cqf10_b(e + 0x29));
        por(0x2a, cqf10_b(e + 0x2a));
        por(0x2b, cqf10_b(e + 0x2b));
        por(0x2c, cqf10_b(e + 0x23));
        por(0x2d, cqf10_b(e + 0x24));
        por(0x2e, 0);
        let adv = if cqf10_d(jogo) == cqf10_d(0x9cf684) { adversario.wrapping_neg() } else { adversario };
        core::ptr::write_unaligned((l + 0xa) as *mut u32, adv);
        if j < 0xb {
            por(0x2e, 1);
            let a = cqf10_b(e + 0x34);
            if (a as i8) > -1 {
                por(0x15, a);
            } else if (cqf10_b(e + 0x30) as i8) > -1 {
                por(0x2e, 5);
                por(0x15, cqf10_b(e + 0x30));
            } else {
                por(0x15, cqf10_b(resultado + 0x4d));
            }
        } else {
            por(0x2e, 2);
            let c = cqf10_b(e + 0x32);
            if c == 0xff {
                por(0x15, 0);
            } else {
                let a = cqf10_b(e + 0x34);
                if (a as i8) > -1 {
                    por(0x15, a.wrapping_sub(c));
                } else if (cqf10_b(e + 0x30) as i8) > -1 {
                    por(0x2e, 6);
                    por(0x15, cqf10_b(e + 0x30).wrapping_sub(c));
                } else {
                    por(0x15, cqf10_b(resultado + 0x4d).wrapping_sub(c));
                }
            }
        }
    }
}

/// Comparador da 0x7bd310: lado a lado nas 50 vagas (o registro inteiro). Com as máscaras ligadas e o técnico do clube
/// humano, a 0x5b09d0 grava o que se sabe do jogador e sorteia: aí só a original roda (ponytail: o caso de um humano
/// observando o próprio clube; a carreira automática não chega a esta rotina).
unsafe extern "thiscall" fn comparar_player_search_record_scouted_club_match(this: usize, jogo: usize, resultado: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    let clube_obs = cqf10_clube(cqf10_d(this));
    if cqf10_b(0x9cc270) != 0 && cqf10_d(clube_obs + 0xcf) != 0 {
        o(this, jogo, resultado);
        return;
    }
    let regioes = [(this, 4 + 0x32 * 0x1a2), (0xae24d0, 0x100), (0xb67a34, 4)];
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogo, resultado),
        || player_search_record_scouted_club_match(this, jogo, resultado));
    PLACAR.registrar(igual, || format!("player_search_record_scouted_club_match(clube {:#x}, jogo {:#x}) {:x?} x {:x?}", cqf10_d(this), jogo, mo, mr));
}

/// O ensaio da 0x7b5be0 (só no comparador): a cópia decide tudo até o relatório sem os efeitos (o histórico 0x7bd310 e o
/// relatório não rodam) e anota se chegaria ao relatório.
static mut CQF10_ENSAIO: bool = false;
static mut CQF10_RELATORIO: bool = false;

/// { 0x7b5be0 } (thiscall, ret 0xc, player_search.cpp, quadro SEH da lista local e das datas): `PLAYER_SEARCH::
/// scout_match(jogo, resultado, saida)`, pela partida observada (0x7f8c00). `this` é a lista de busca do olheiro: +0 o
/// dono, +0xa o tipo (4 = observar partidas), +0x170 o registro do clube observado, +0x174..+0x184 os nomes do olheiro,
/// +0x180 o id dele, +0x190 o clube acompanhado, +0x195 o jogador acompanhado, +0x1a5/+0x1ad o começo e o fim. Sem jogo
/// ou resultado: a caixa da linha 0xef1. Do tipo 4, com olheiro que confere (0x53e990) e resultado de partida play
/// (+0x4c zero): o clube acompanhado que joga recebe o histórico (0x7bd310); dentro das datas (antes do fim só com
/// jogador acompanhado), se o jogo é do jogador acompanhado, relata; senão a busca tem de cobrir a nação da competição
/// (0x7b7490) e o clube do olheiro tem de ter pedido (0x7f9690, 0x7f8d70 com 1) algum jogador de um dos dois clubes. O
/// relatório: o começo da lista vai a amanhã; dos dois lados (+0x1c0 / +0x1c0+0x625, 20 registros de 0x49), cada jogador
/// válido de outro clube (reservas além do 11º só com nota) é conhecido pelo dono (0x5b09d0, com as máscaras) e, pedido
/// (0x7f8d70 com 0), acompanhado ou com reputação vista abaixo de 4000 (0x5407e0) e nota de ao menos 15 (90 com o
/// resultado sem notas), vira um relatório (0x7b62a0, modo 0 acompanhado, 1 pedido, 2 com +5 ≥ 8) que entra na saída
/// (0x68d0d0, 0x67 bytes) se o byte +0x66 ficou ≥ 0. Devolve 1 com o relatório, senão 0.
unsafe extern "thiscall" fn player_search_scout_match(this: usize, jogo: usize, resultado: usize, saida: usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    // 0x68d020 / 0x68d040: lea ecx, local -> thiscall
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let r = scout_match_body(this, jogo, resultado, saida, l);
    soltar(l);
    r
}

unsafe fn scout_match_body(this: usize, jogo: usize, resultado: usize, saida: usize, l: usize) -> u32 {
    if jogo == 0 || resultado == 0 {
        show_error_at(0xa701b0, 0xef1);
        return 0;
    }
    let dono = cqf10_pessoa(cqf10_d(this));
    // a nação da competição do jogo (+0x18 → +0x38 → +0x1f) ou a do mandante (+0x1c → +0x53)
    let mut nacao = 0usize;
    let t = cqf10_d(jogo + 0x18);
    if t != 0 && cqf10_d(t + 0x38) != 0 && cqf10_d(cqf10_d(t + 0x38) + 0x1f) != 0 {
        nacao = cqf10_d(cqf10_d(t + 0x38) + 0x1f);
    }
    if nacao == 0 {
        nacao = cqf10_d(cqf10_d(jogo + 0x1c) + 0x53);
    }
    if cqf10_b(this + 0xa) != 4 {
        return 0;
    }
    let id_olheiro = cqf10_i(this + 0x180);
    if id_olheiro <= -1 {
        return 0;
    }
    let confere: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(0x53e990usize);
    // 0x53e990: 5 pushes; add esp, 0x14 -> cdecl
    if confere(cqf10_pessoa(id_olheiro as usize), cqf10_d(this + 0x174), cqf10_d(this + 0x178), cqf10_d(this + 0x17c), this + 0x184) == 0 {
        return 0;
    }
    let olheiro = cqf10_pessoa(cqf10_i(this + 0x180) as usize);
    if olheiro == 0 || cqf10_b(resultado + 0x4c) != 0 {
        return 0;
    }
    let acompanhado = cqf10_i(this + 0x190);
    if acompanhado > -1 && cqf10_d(this + 0x170) != 0 && (cqf10_i(jogo + 0xc) == acompanhado || cqf10_i(jogo + 0x10) == acompanhado) {
        if !CQF10_ENSAIO {
            crate::comparacao::ramo("cqf_10 historico do clube");
            let historico: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7bd310usize);
            // 0x7bd310: push resultado; push jogo; mov ecx, [this+0x170] -> thiscall, ret 8
            historico(cqf10_d(this + 0x170), jogo, resultado);
        }
    }
    // as datas: hoje antes do começo (+0x1a5: dia, +0x1a7: ano) sai; antes do fim (+0x1ad/+0x1af) só com jogador
    // acompanhado; ano sem sinal, dia com sinal, como a original
    let (dia, ano) = (cqf10_w(0xae2c90), cqf10_w(0xae2c92) as u16);
    let (dia_ini, ano_ini) = (cqf10_w(this + 0x1a5), cqf10_w(this + 0x1a7) as u16);
    if ano < ano_ini || (ano == ano_ini && dia < dia_ini) {
        return 0;
    }
    let (dia_fim, ano_fim) = (cqf10_w(this + 0x1ad), cqf10_w(this + 0x1af) as u16);
    let antes_do_fim = ano < ano_fim || (ano == ano_fim && dia < dia_fim);
    if antes_do_fim && cqf10_i(this + 0x195) == -1 {
        return 0;
    }
    let alvo = cqf10_i(this + 0x195);
    let do_alvo = alvo > -1 && cqf10_i(jogo) == alvo;
    if !do_alvo {
        let cobre: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7b7490usize);
        // 0x7b7490: push nação; mov ecx, this -> thiscall, ret 4
        if cobre(this, nacao) == 0 {
            return 0;
        }
        let alvos: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7f9690usize);
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        let pedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x7f8d70usize);
        // 0x7f9690: push &lista; push clube do olheiro; mov ecx, [0xb74448] -> thiscall, ret 8; 0x68d1b0: lea ecx, lista
        // -> thiscall; 0x7f8d70: push 1; push clube; push jogador; mov ecx, [0xb74448] -> thiscall, ret 0xc
        alvos(cqf10_d(0xb74448), cqf10_d(olheiro + 0x39), l);
        let conta = cqf10_w(l);
        if conta == 0 {
            return 0;
        }
        core::ptr::write_unaligned((l + 0xa) as *mut usize, cqf10_d(l + 2)); // o cursor volta à cabeça
        let mut livre = true; // ebp da original: nenhum pedido achado
        // ponytail: a original usa o mesmo registrador (edi) para contar os itens e as 20 vagas de cada lado; depois de
        // um item de um dos clubes o contador vira o número de vagas olhadas na última volta (o laço de fora acaba antes)
        let mut i: i32 = 0;
        if conta > 0 {
            loop {
                let item = proximo(l);
                if item != 0 {
                    let (m, v) = (cqf10_d(jogo + 0x1c), cqf10_d(jogo + 0x20));
                    let c39 = cqf10_d(item + 0x39);
                    let c24 = cqf10_d(item + 0x24);
                    if c39 == m || c39 == v || c24 == m || c24 == v {
                        let mut lado: i8 = 0;
                        while livre {
                            i = 0;
                            let mut k = 0usize;
                            while livre {
                                let id = cqf10_i(resultado + 0x190 + lado as usize * 0x625 + k);
                                if id != -1 && pedido(cqf10_d(0xb74448), cqf10_pessoa(id as usize), cqf10_d(olheiro + 0x39), 1) != 0 {
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
                if i >= cqf10_w(l) as i32 {
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
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    // 0x54c5b0: push 1; push &tmp; mov ecx, 0xae2c90 -> thiscall, ret 8; 0x417c10: push data; push &copia; lea ecx,
    // [this+0x1a5] -> thiscall, ret 8
    let (mut tmp, mut copia) = ([0u32; 2], [0u32; 2]);
    let amanha = mais(0xae2c90, tmp.as_mut_ptr() as usize, 1); // cqf10: amanhã
    if CQF10_ENSAIO {
        CQF10_RELATORIO = true;
        return 1;
    }
    copiar(this + 0x1a5, copia.as_mut_ptr() as usize, amanha);
    let mut marca = copia[0] & !0xff;
    let quadro: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5afde0usize);
    let conhecer: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x5b09d0usize);
    let pedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x7f8d70usize);
    let injury_word_0c: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(0x5407e0usize);
    let relatorio: unsafe extern "thiscall" fn(usize, usize, usize, i32, u32, usize, usize, u32) = core::mem::transmute(0x7b62a0usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    // 0x5afde0: push dono; mov ecx, [0xae2a60] -> thiscall, ret 4; 0x5b09d0: push olheiro; push jogador; mov ecx, eax ->
    // thiscall, ret 8; 0x7f8d70: push 0; push clube; push jogador; mov ecx, [0xb74448] -> thiscall, ret 0xc; 0x5407e0:
    // push 0; push olheiro; push jogador; add esp, 0xc -> cdecl; 0x7b62a0: 7 pushes; mov ecx, this -> thiscall, ret 0x1c;
    // 0x68d0d0: push 0x67; push &registro; mov ecx, saida -> thiscall, ret 8
    // o registro local (0x67 bytes), reaproveitado entre os jogadores como na original, com as partes que o construtor da
    // original monta na entrada: a data em +0 (0x54a110) e os dois recordistas em +0x1f e +0x37 (0x450d40); o resto
    // começa zerado (na original, sobra da pilha que a 0x7b62a0 sobrescreve)
    let mut registro = [0u8; 0x68];
    let reg = registro.as_mut_ptr() as usize;
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let recordista: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x450d40usize);
    // lea ecx, local -> thiscall nas duas
    data_padrao(reg);
    recordista(reg + 0x1f);
    recordista(reg + 0x37);
    let clube_olheiro = cqf10_d(olheiro + 0x39);
    for lado in 0..2usize {
        let mut e = resultado + 0x1c0 + lado * 0x625;
        for j in 0..0x14i32 {
            let atual = e;
            e += 0x49;
            if j > 10 && cqf10_b(atual + 2) == 0xff {
                continue;
            }
            let id = cqf10_i(atual - 0x30);
            if id == -1 {
                continue;
            }
            let p = cqf10_pessoa(id as usize);
            if p != 0 && cqf10_i(p) >= cqf10_i(0xae2364) {
                continue;
            }
            if player_search_contract_club(p) == clube_olheiro || cqf10_d(p + 0x61) == 0 {
                continue;
            }
            if cqf10_b(0x9cc270) != 0 {
                let q = quadro(cqf10_d(0xae2a60), dono);
                if q != 0 {
                    conhecer(q, p, olheiro);
                }
            }
            let pediu = pedido(cqf10_d(0xb74448), p, clube_olheiro, 0);
            if pediu == 0 {
                let alvo = cqf10_i(this + 0x195);
                if !(alvo != -1 && cqf10_i(jogo) == alvo) && injury_word_0c(p, olheiro, 0) >= 0xfa0 { // cqf10: reputação vista
                    continue;
                }
            }
            // a nota: starters (até o 11º) pela nota +4, senão +0, senão a do resultado (+0x4d); reservas descontam a
            // entrada (+2); resultado sem notas (+0x4c): titular vale 90
            let nota_de = |x: usize| -> u8 {
                let a = cqf10_b(x + 4);
                if (a as i8) > -1 { a } else if (cqf10_b(x) as i8) > -1 { cqf10_b(x) } else { cqf10_b(resultado + 0x4d) }
            };
            let nota: u8 = if j < 0xb {
                if cqf10_b(resultado + 0x4c) != 0 { 0x5a } else { nota_de(atual) }
            } else {
                let s = cqf10_b(atual + 2);
                if s == 0xff {
                    continue;
                }
                nota_de(atual).wrapping_sub(s)
            };
            if (nota as i8) < 0xf { // cqf10: a nota mínima
                continue;
            }
            let alvo = cqf10_i(this + 0x195);
            let modo: u32 = if alvo > -1 && alvo == cqf10_i(jogo) {
                0
            } else if pediu != 0 {
                1
            } else {
                if cqf10_b(atual + 5) < 8 {
                    continue;
                }
                2
            };
            crate::comparacao::ramo("cqf_10 avaliar jogador");
            let n = ((cqf10_b(atual + 5) as i32 * 0x28 - 0x104).wrapping_mul(nota as i8 as i32)) / 2;
            relatorio(this, p, reg, n, modo, jogo, resultado, marca);
            if (cqf10_b(reg + 0x66) as i8) > -1 {
                acrescentar(saida, reg, 0x67);
            }
        }
        marca = (marca & !0xff) | ((marca as u8).wrapping_add(1)) as u32;
    }
    1
}

/// Comparador da 0x7b5be0: o relatório sorteia (0x7b62a0, 0x5b09d0) e põe registros na saída (aloca): só a original
/// roda. Antes, a cópia em ensaio decide até o relatório sem os efeitos (sem o histórico e sem o relatório, com o estado
/// do sorteador guardado e devolvido) e diz a devolução e se o começo da lista vai a amanhã; a original tem de bater.
unsafe extern "thiscall" fn comparar_player_search_scout_match(this: usize, jogo: usize, resultado: usize, saida: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(%%I0%%);
    if jogo == 0 || resultado == 0 {
        return o(this, jogo, resultado, saida);
    }
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = (cqf10_d(this + 0x1a5), cqf10_d(this + 0x1a9));
    CQF10_ENSAIO = true;
    CQF10_RELATORIO = false;
    let n = crate::aleatorio::sem_contar(|| player_search_scout_match(this, jogo, resultado, saida));
    CQF10_ENSAIO = false;
    let relata = CQF10_RELATORIO;
    estado.escrever();
    let previsto = if relata {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let mut t = [0u32; 2];
        let a = mais(0xae2c90, t.as_mut_ptr() as usize, 1);
        (cqf10_w(a + 2) as u16, cqf10_w(a))
    } else {
        (cqf10_w(this + 0x1a7) as u16, cqf10_w(this + 0x1a5))
    };
    let r = o(this, jogo, resultado, saida);
    let depois = (cqf10_w(this + 0x1a7) as u16, cqf10_w(this + 0x1a5));
    PLACAR.registrar(r == n && depois == previsto, || format!("player_search_scout_match(lista {:#x}, jogo {:#x}) original={} ensaio={} começo antes {:x?} previsto {:x?} depois {:x?}",
        this, jogo, r, n, antes, previsto, depois));
    r
}
