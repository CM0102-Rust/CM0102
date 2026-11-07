// ---------- 270a leva: o olheiro na partida observada e o histórico do clube observado (player_search.cpp) ----------


unsafe fn cqf10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}



unsafe fn cqf10_pessoa(id: usize) -> usize {
    read_usize(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

unsafe fn cqf10_clube(id: usize) -> usize {
    read_usize(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O clube do contrato da pessoa (0x4cf5a0, guardado em [0xae1080]) ou 0.
unsafe fn cqf10_clube_do_contrato(pessoa: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    // 0x4cf5a0: push pessoa; mov ecx, [0xae1050] -> thiscall, ret 4
    let c = contrato_de(read_usize(0xae1050), pessoa);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    if c != 0 { cqf10_clube(read_usize(c + 4)) } else { 0 }
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
unsafe extern "thiscall" fn cqf10_historico_do_clube_observado(this: usize, jogo: usize, resultado: usize) {
    let clube_obs = cqf10_clube(read_usize(this));
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
        if j > 10 && read_u8(e + 0x32) == 0xff {
            continue;
        }
        let p = cqf10_pessoa(id as usize);
        if p != 0 && cqf10_i(p) >= cqf10_i(0xae2364) {
            continue;
        }
        if read_usize(clube_obs + 0xcf) != 0 && humano(clube_obs, 0, 1, 0, 0) != 0 && read_u8(0x9cc270) != 0 {
            let q = quadro(read_usize(0xae2a60), read_usize(clube_obs + 0xcf));
            if q != 0 {
                conhecer(q, p, 0);
            }
        }
        let r = vaga_de(this, p, 1);
        if r == 0 {
            aviso_em(0xa701b0, 0x2065);
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
        por(0x14, lado_do_jogo(cqf10_clube(read_usize(this)), jogo) as u8);
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
        let adv = if read_usize(jogo) == read_usize(0x9cf684) { adversario.wrapping_neg() } else { adversario };
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

/// Comparador da 0x7bd310: lado a lado nas 50 vagas (o registro inteiro). Com as máscaras ligadas e o técnico do clube
/// humano, a 0x5b09d0 grava o que se sabe do jogador e sorteia: aí só a original roda (ponytail: o caso de um humano
/// observando o próprio clube; a carreira automática não chega a esta rotina).
unsafe extern "thiscall" fn comparar_cqf10_historico_do_clube_observado(this: usize, jogo: usize, resultado: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1622);
    let clube_obs = cqf10_clube(read_usize(this));
    if read_u8(0x9cc270) != 0 && read_usize(clube_obs + 0xcf) != 0 {
        o(this, jogo, resultado);
        return;
    }
    let regioes = [(this, 4 + 0x32 * 0x1a2), (0xae24d0, 0x100), (0xb67a34, 4)];
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogo, resultado),
        || cqf10_historico_do_clube_observado(this, jogo, resultado));
    PLACAR.registrar(igual, || format!("cqf10_historico_do_clube_observado(clube {:#x}, jogo {:#x}) {:x?} x {:x?}", read_usize(this), jogo, mo, mr));
}

/// O ensaio da 0x7b5be0 (só no comparador): a cópia decide tudo até o relatório sem os efeitos (o histórico 0x7bd310 e o
/// relatório não rodam) e anota se chegaria ao relatório.
static mut CQF10_ENSAIO: bool = false;
static mut CQF10_RELATORIO: bool = false;

/// { 0x7b5be0 } (thiscall, ret 0xc, player_search.cpp, quadro SEH da lista local e das datas): `PLAYER_SEARCH::
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
unsafe extern "thiscall" fn cqf10_olheiro_no_jogo(this: usize, jogo: usize, resultado: usize, saida: usize) -> u32 {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    // 0x68d020 / 0x68d040: lea ecx, local -> thiscall
    let mut lista = [0u8; 16];
    let l = lista.as_mut_ptr() as usize;
    nova(l);
    let r = cqf10_corpo(this, jogo, resultado, saida, l);
    soltar(l);
    r
}

unsafe fn cqf10_corpo(this: usize, jogo: usize, resultado: usize, saida: usize, l: usize) -> u32 {
    if jogo == 0 || resultado == 0 {
        aviso_em(0xa701b0, 0xef1);
        return 0;
    }
    let dono = cqf10_pessoa(read_usize(this));
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
    let id_olheiro = cqf10_i(this + 0x180);
    if id_olheiro <= -1 {
        return 0;
    }
    let confere: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(0x53e990usize);
    // 0x53e990: 5 pushes; add esp, 0x14 -> cdecl
    if confere(cqf10_pessoa(id_olheiro as usize), read_usize(this + 0x174), read_usize(this + 0x178), read_usize(this + 0x17c), this + 0x184) == 0 {
        return 0;
    }
    let olheiro = cqf10_pessoa(cqf10_i(this + 0x180) as usize);
    if olheiro == 0 || read_u8(resultado + 0x4c) != 0 {
        return 0;
    }
    let acompanhado = cqf10_i(this + 0x190);
    if acompanhado > -1 && read_usize(this + 0x170) != 0 && (cqf10_i(jogo + 0xc) == acompanhado || cqf10_i(jogo + 0x10) == acompanhado) {
        if !CQF10_ENSAIO {
            crate::comparacao::ramo("cqf_10 historico do clube");
            let historico: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x7bd310usize);
            // 0x7bd310: push resultado; push jogo; mov ecx, [this+0x170] -> thiscall, ret 8
            historico(read_usize(this + 0x170), jogo, resultado);
        }
    }
    // as datas: hoje antes do começo (+0x1a5: dia, +0x1a7: ano) sai; antes do fim (+0x1ad/+0x1af) só com jogador
    // acompanhado; ano sem sinal, dia com sinal, como a original
    let (dia, ano) = (read_i16(0xae2c90), read_i16(0xae2c92) as u16);
    let (dia_ini, ano_ini) = (read_i16(this + 0x1a5), read_i16(this + 0x1a7) as u16);
    if ano < ano_ini || (ano == ano_ini && dia < dia_ini) {
        return 0;
    }
    let (dia_fim, ano_fim) = (read_i16(this + 0x1ad), read_i16(this + 0x1af) as u16);
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
        alvos(read_usize(0xb74448), read_usize(olheiro + 0x39), l);
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
                                let id = cqf10_i(resultado + 0x190 + lado as usize * 0x625 + k);
                                if id != -1 && pedido(read_usize(0xb74448), cqf10_pessoa(id as usize), read_usize(olheiro + 0x39), 1) != 0 {
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
    let reputacao: unsafe extern "C" fn(usize, usize, u32) -> i16 = core::mem::transmute(0x5407e0usize);
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
    let clube_olheiro = read_usize(olheiro + 0x39);
    for lado in 0..2usize {
        let mut e = resultado + 0x1c0 + lado * 0x625;
        for j in 0..0x14i32 {
            let atual = e;
            e += 0x49;
            if j > 10 && read_u8(atual + 2) == 0xff {
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
            if cqf10_clube_do_contrato(p) == clube_olheiro || read_usize(p + 0x61) == 0 {
                continue;
            }
            if read_u8(0x9cc270) != 0 {
                let q = quadro(read_usize(0xae2a60), dono);
                if q != 0 {
                    conhecer(q, p, olheiro);
                }
            }
            let pediu = pedido(read_usize(0xb74448), p, clube_olheiro, 0);
            if pediu == 0 {
                let alvo = cqf10_i(this + 0x195);
                if !(alvo != -1 && cqf10_i(jogo) == alvo) && reputacao(p, olheiro, 0) >= 0xfa0 { // cqf10: reputação vista
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
            let alvo = cqf10_i(this + 0x195);
            let modo: u32 = if alvo > -1 && alvo == cqf10_i(jogo) {
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

/// Comparador da 0x7b5be0: o relatório sorteia (0x7b62a0, 0x5b09d0) e põe registros na saída (aloca): só a original
/// roda. Antes, a cópia em ensaio decide até o relatório sem os efeitos (sem o histórico e sem o relatório, com o estado
/// do sorteador guardado e devolvido) e diz a devolução e se o começo da lista vai a amanhã; a original tem de bater.
unsafe extern "thiscall" fn comparar_cqf10_olheiro_no_jogo(this: usize, jogo: usize, resultado: usize, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(1621);
    if jogo == 0 || resultado == 0 {
        return o(this, jogo, resultado, saida);
    }
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = (read_usize(this + 0x1a5), read_usize(this + 0x1a9));
    CQF10_ENSAIO = true;
    CQF10_RELATORIO = false;
    let n = crate::aleatorio::sem_contar(|| cqf10_olheiro_no_jogo(this, jogo, resultado, saida));
    CQF10_ENSAIO = false;
    let relata = CQF10_RELATORIO;
    estado.escrever();
    let previsto = if relata {
        let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let mut t = [0u32; 2];
        let a = mais(0xae2c90, t.as_mut_ptr() as usize, 1);
        (read_i16(a + 2) as u16, read_i16(a))
    } else {
        (read_i16(this + 0x1a7) as u16, read_i16(this + 0x1a5))
    };
    let r = o(this, jogo, resultado, saida);
    let depois = (read_i16(this + 0x1a7) as u16, read_i16(this + 0x1a5));
    PLACAR.registrar(r == n && depois == previsto, || format!("cqf10_olheiro_no_jogo(lista {:#x}, jogo {:#x}) original={} ensaio={} começo antes {:x?} previsto {:x?} depois {:x?}",
        this, jogo, r, n, antes, previsto, depois));
    r
}

// ---------- 263a leva: a lista de observação do técnico humano: o jogador na busca, as listas dos olheiros acertadas com os olheiros do clube e a vaga do jogador no registro do clube observado (player_search.cpp, scout_manager.cpp) ----------



/// { 0x7af600 } (thiscall, ret 4, player_search.cpp, quadro SEH só pelas duas datas locais, sem destrutor):
/// `PLAYER_SEARCH::contains_player(pessoa)`, da lista do olheiro do técnico humano (0x7f7490, que a avaliação dos
/// jogadores e os clubes interessados consultam a cada dia) e de telas. A chave local de 0x2d bytes começa com as duas
/// datas padrão (+0x10 e +0x18, 0x54a110); pessoa nula: a caixa da linha 0x33e e 0. Sem resultados (+0x1f6 = conta):
/// 0. A referência ao nome da pessoa na chave (0x7e4920: +0xc o id, o único campo que a comparação 0x7bce50 lê) e a
/// busca binária nos resultados (0x946464 com o vetor +0x1ee, a conta, 0x2d e 0x7bce50); achada e sem a marca de
/// removida (bit 0 de +0x20), 1; senão 0.
unsafe extern "thiscall" fn jogador_na_busca(this: usize, pessoa: usize) -> u32 {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mut chave = [0u8; 0x30];
    let k = chave.as_mut_ptr() as usize;
    data_padrao(k + 0x10);
    data_padrao(k + 0x18);
    if pessoa == 0 {
        aviso_em(0xa701b0, 0x33e);
        return 0;
    }
    if read_usize(this + 0x1f6) == 0 {
        crate::comparacao::ramo("cqa_6 busca vazia");
        return 0; // cqa6: sem resultados, não está
    }
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
    referencia(pessoa, k);
    let bsearch: unsafe extern "C" fn(usize, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
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

/// Comparador da 0x7af600: pela devolução (só lê; as datas e a chave são locais). Pessoa nula: a caixa, só a original.
unsafe extern "thiscall" fn comparar_jogador_na_busca(this: usize, pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1434);
    if pessoa == 0 {
        return o(this, pessoa);
    }
    let r = o(this, pessoa);
    let n = jogador_na_busca(this, pessoa);
    PLACAR.registrar(r == n, || format!("jogador_na_busca(busca {:x}, pessoa {:x}) original={} rust={}", this, pessoa, r, n));
    r
}

/// A busca `i` (0..9) do técnico `m` (0x7f6cd0: thiscall(gerência, i, técnico), ret 8; lê só o byte do índice).
unsafe fn cqa6_busca(this: usize, i: u8, m: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    f(this, i as u32, m)
}

/// O olheiro da busca (0x7ae950: thiscall(busca)).
unsafe fn cqa6_olheiro(l: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x7ae950usize);
    f(l)
}

/// Põe o olheiro na busca (0x7ae8a0: thiscall(busca, olheiro), ret 4).
unsafe fn cqa6_trocar(l: usize, olheiro: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7ae8a0usize);
    f(l, olheiro);
}

/// { 0x7f7270 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::sync_scout_lists(técnico)`, quando a comissão do
/// clube humano muda (a saída do cargo 0x690890 e a da vaga dos olheiros, as transferências e contratações de pessoal).
/// Sem técnico, o humano da vez; sem ele, a caixa da linha 0x1ba e 0. Das 10 buscas do técnico, as listas de olheiro
/// (+0xa = 4) cujo olheiro (0x7ae950) não é mais olheiro (+0x3d = 9) do clube do técnico (+0x39) ficam sem olheiro
/// (0x7ae8a0 com 0; sem clube, todas; com clube, as sem olheiro ficam como estão). Com clube, cada um dos 7 olheiros do
/// clube (0x53cd60) que não tem lista vai para a primeira lista de olheiro sem olheiro. Depois a vaga do técnico (byte
/// +0 − byte de [0xae2364] + 16, 0..15; fora, a caixa da linha 0x212 e 0): com a edição aberta ([this + vaga × 4 +
/// 0xc]), ela refaz os olheiros das listas (0x8043c0). Devolve 1.
unsafe extern "thiscall" fn sincronizar_listas_dos_olheiros(this: usize, tecnico: usize) -> u32 {
    let m = if tecnico != 0 { tecnico } else { cps8_gerente_da_vez() };
    if m == 0 {
        aviso_em(0xa7b094, 0x1ba);
        return 0;
    }
    let clube = read_usize(m + 0x39);
    for i in 0..10u8 {
        let l = cqa6_busca(this, i, m);
        if l == 0 || read_u8(l + 0xa) != 4 {
            continue;
        }
        let o = cqa6_olheiro(l);
        if clube != 0 {
            if o == 0 || (read_usize(o + 0x39) == clube && read_u8(o + 0x3d) == 9) {
                continue;
            }
        }
        crate::comparacao::ramo("cqa_6 lista sem olheiro");
        cqa6_trocar(l, 0);
    }
    if clube != 0 {
        let olheiros: unsafe extern "C" fn(usize, *mut usize) -> u32 = core::mem::transmute(0x53cd60usize);
        let mut v = [0usize; 7];
        olheiros(clube, v.as_mut_ptr());
        for &s in v.iter() {
            if s == 0 {
                continue;
            }
            let mut livre: i8 = -1;
            let mut tem = false;
            for i in 0..10u8 {
                let l = cqa6_busca(this, i, m);
                if l == 0 || read_u8(l + 0xa) != 4 {
                    continue;
                }
                if cqa6_olheiro(l) == s {
                    tem = true;
                    break;
                }
                if cqa6_olheiro(l) == 0 && livre == -1 {
                    livre = i as i8;
                }
            }
            if !tem && livre > -1 {
                crate::comparacao::ramo("cqa_6 olheiro na lista livre");
                cqa6_trocar(cqa6_busca(this, livre as u8, m), s);
            }
        }
    }
    let vaga = read_u8(m).wrapping_sub(read_u8(0xae2364)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) {
        aviso_em(0xa7b094, 0x212);
        return 0;
    }
    let s = read_usize(this + vaga as usize * 4 + 0xc);
    if s != 0 {
        let refazer: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8043c0usize);
        refazer(s);
    }
    1
}

/// Comparador da 0x7f7270: lado a lado nas 10 buscas do técnico (0x2b0 bytes cada, achadas antes pela 0x7f6cd0, que
/// só lê) e na devolução. Sem técnico ou com a edição aberta (a 0x8043c0 mexe na edição): só a original.
unsafe extern "thiscall" fn comparar_sincronizar_listas_dos_olheiros(this: usize, tecnico: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1435);
    let m = if tecnico != 0 { tecnico } else { cps8_gerente_da_vez() };
    if m == 0 {
        return o(this, tecnico);
    }
    let vaga = read_u8(m).wrapping_sub(read_u8(0xae2364)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) || read_usize(this + vaga as usize * 4 + 0xc) != 0 {
        return o(this, tecnico);
    }
    let mut regioes = Vec::new();
    for i in 0..10u8 {
        let l = cqa6_busca(this, i, m);
        if l != 0 {
            regioes.push((l, 0x2b0usize));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, tecnico), || sincronizar_listas_dos_olheiros(this, tecnico));
    PLACAR.registrar(igual && r == n, || format!("sincronizar_listas_dos_olheiros(técnico {:x}) original={} rust={} {:x?} x {:x?}", m, r, n, mo, mr));
    r
}

/// { 0x7bd700 } (thiscall, ret 8, player_search.cpp): `player_search_find_or_claim_player_slot(pessoa, reservar)`, do
/// registro de 50 vagas (0x1a2 bytes a partir de +4: +0 o id, −1 livre) do clube observado, pela partida dele
/// (0x7bd310) e pelas telas (0x7f8610). A vaga com o id da pessoa; senão, com `reservar`, a primeira livre recebe o id
/// (pessoa nula: −1), zera a palavra +4 e os bytes +0x1a0 e +0x1a1 e é devolvida (nenhuma livre: a caixa da linha
/// 0x210d e 0); sem `reservar`, 0.
unsafe extern "thiscall" fn vaga_do_jogador_observado(this: usize, pessoa: usize, reservar: u32) -> usize {
    if pessoa == 0 {
        // ponytail: a original lê o id da pessoa antes de conferir o ponteiro (cairia); fica com ela
        return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1436)(this, pessoa, reservar);
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
        aviso_em(0xa701b0, 0x210d);
        return 0;
    }
    let a = vaga(livre);
    core::ptr::write_unaligned(a as *mut i32, id);
    core::ptr::write_unaligned((a + 4) as *mut u16, 0);
    *((a + 0x1a0) as *mut u8) = 0;
    *((a + 0x1a1) as *mut u8) = 0;
    a
}

/// Comparador da 0x7bd700: sem `reservar`, pela devolução (só lê); com ele, lado a lado nas 50 vagas e na devolução.
unsafe extern "thiscall" fn comparar_vaga_do_jogador_observado(this: usize, pessoa: usize, reservar: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1436);
    if pessoa == 0 {
        return o(this, pessoa, reservar);
    }
    if reservar == 0 {
        let r = o(this, pessoa, reservar);
        let n = vaga_do_jogador_observado(this, pessoa, reservar);
        PLACAR.registrar(r == n, || format!("vaga_do_jogador_observado(registro {:x}, pessoa {:x}) original={:x} rust={:x}", this, pessoa, r, n));
        return r;
    }
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(this + 4, 0x32 * 0x1a2)], || o(this, pessoa, reservar), || vaga_do_jogador_observado(this, pessoa, reservar));
    PLACAR.registrar(igual && r == n, || format!("vaga_do_jogador_observado(registro {:x}, pessoa {:x}, reservar) original={:x} rust={:x} iguais={}", this, pessoa, r, n, igual));
    r
}

// ---------- 257a leva: a nota-alvo da busca de jogadores do clube e o histórico de jogos do jogador acompanhado: o jogo da vez e a soma dos gols (player_search.cpp) ----------



/// { 0x7bca90 } (cdecl, player_search.cpp): `player_search_get_club_target_rating(clube, *base, *forcada)`: a nota que
/// o clube procura nos jogadores. A base (sem `forcada`): a reputação (+0x80), a do dinheiro (+0x77 × 0,43 − 430, até
/// 10000, se maior que a reputação), a da nação (+0x57: +0x69 × 500, se maior que reputação / 500) e a da liga (+0x1db
/// em [0xae23d0], 0x6b bytes: +0x69 × 600, idem), pesadas e / 16; metade no clube não profissional (+0x64 ≠ 1). A
/// nota sai da base por faixas (até 3000, 5000, 7000, 9000 e acima). Sem `forcada`, pelo elenco (0x53c590) e pela
/// faixa de riqueza das finanças (0x5a0590 de [0xae2a38], 0x5986b0) nos clubes com nação em [0xadadfc]: elenco curto
/// (< 16) metade; < 20 metade ou 2/3 nos pobres; senão encolhe pela falta de jogadores frente à nação (+0x69 × 1,35 +
/// 20) e a riqueza puxa (−2..2: × 0,909 ou − 1000, × 0,952 ou − 500, nada, × 1,0125 ou + 125, × 1,025 ou + 250).
/// No mínimo 500. A base vai em `*base` (se não nulo). As contas de ponto flutuante em f64 (o x87 do jogo em 53 bits).
unsafe extern "C" fn nota_alvo_do_clube(clube: usize, base: *mut i32, forcada: *const i16) -> i32 {
    let contar: unsafe extern "C" fn(usize, u32, u32) -> i8 = core::mem::transmute(0x53c590usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let riqueza: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let li = read_i32(clube + 0x1db);
    let liga = if li > -1 { (read_i32(0xae23d0) as usize).wrapping_add((li as usize).wrapping_mul(0x6b)) } else { 0 };
    let nacao = read_i32(clube + 0x57) as usize;
    let mut esi: i32;
    if !forcada.is_null() {
        esi = *forcada as i32;
    } else {
        let rep = read_i16(clube + 0x80) as i32;
        let t = ftol(read_i32(clube + 0x77) as f64 * f64_em(0x96e2b0) - f64_em(0x96e2a8)) as i16 as i32;
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
    if *((clube + 0x64) as *const u8) != 1 {
        esi /= 2;
    }
    let mut edi: i32 = if esi > 9000 {
        esi.wrapping_mul(3).wrapping_add(7500)
    } else if esi > 7000 {
        ftol(esi as f64 * f64_em(0x96d688) - f64_em(0x96d760))
    } else if esi > 5000 {
        ftol(esi as f64 * f64_em(0x968e18) + f64_em(0x9690b8))
    } else if esi > 3000 {
        crate::comparacao::ramo("cpu_13 faixa de 3000 a 5000");
        esi.wrapping_mul(5).wrapping_sub(5000) // cpu13: a faixa do meio
    } else {
        ftol(esi as f64 * f64_em(0x969018) + f64_em(0x9688e8))
    };
    if forcada.is_null() {
        let elenco = contar(clube, 0, 0);
        let com_nacao = |c: usize| c != 0 && read_i32((read_i32(0xadadfc) as usize).wrapping_add(read_i32(c) as usize * 4)) != 0;
        if elenco < 16 && com_nacao(nacao) {
            edi /= 2;
        } else if elenco < 20 && com_nacao(nacao) {
            let fin = financas(read_i32(0xae2a38) as usize, clube);
            if fin != 0 {
                match riqueza(fin) as i8 {
                    -2 => edi /= 2,
                    -1 => edi = ftol(edi as f64 * f64_em(0x96a670)),
                    _ => {}
                }
            }
        } else {
            let nacao = read_i32(clube + 0x57) as usize;
            if com_nacao(nacao) {
                let t2 = ftol(read_i16(nacao + 0x69) as i32 as f64 * f64_em(0x969588) + f64_em(0x968ea8)) as u8;
                let falta = (elenco as u8).wrapping_sub(t2) as i8;
                if falta < 0 {
                    let v = if falta > -10 { falta as i32 } else { -10 };
                    edi = ftol((v as f64 * f64_em(0x967888) + f64_em(0x967890)) * edi as f64);
                }
            }
            let fin = financas(read_i32(0xae2a38) as usize, clube);
            if fin != 0 {
                let v = edi as f64;
                match riqueza(fin) as i8 {
                    -2 => {
                        let (x, y) = (v * f64_em(0x96d718), edi.wrapping_sub(1000) as f64);
                        edi = ftol(if x < y { x } else { y });
                    }
                    -1 => {
                        let (x, y) = (v * f64_em(0x96e2a0), edi.wrapping_sub(500) as f64);
                        edi = ftol(if x < y { x } else { y });
                    }
                    1 => {
                        let (x, y) = (v * f64_em(0x96d6b8), edi.wrapping_add(125) as f64);
                        edi = ftol(if x > y { x } else { y });
                    }
                    2 => {
                        crate::comparacao::ramo("cpu_13 clube rico");
                        let (x, y) = (v * f64_em(0x968fa0), edi.wrapping_add(250) as f64); // cpu13: o rico
                        edi = ftol(if x > y { x } else { y });
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

/// { 0x7bd830 } (thiscall, ret 4, player_search.cpp): `player_search_slot_get_match_entry(i)`: o jogo `i` (0x29
/// bytes, a partir de +6) do anel de 10 que começa em +0x1a0 (byte com sinal), ou 0 se `i` ≥ a conta (+0x1a1).
unsafe extern "thiscall" fn jogo_do_historico(this: usize, i: u32) -> usize {
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

/// { 0x7bd870 } (thiscall, ret 0, player_search.cpp): `player_search_slot_sum_goals()`: a soma (16 bits) do byte com
/// sinal +0x16 dos jogos do anel (+0x1a1 deles, a partir de +0x1a0, dando a volta em 10).
unsafe extern "thiscall" fn gols_do_historico(this: usize) -> u16 {
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

/// Comparador: pura (lê o clube, o elenco e as finanças; grava só `*base`): as duas sobre bases próprias.
unsafe extern "C" fn comparar_nota_alvo_do_clube(clube: usize, base: *mut i32, forcada: *const i16) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, *mut i32, *const i16) -> i32>(1230);
    let (mut bo, mut br) = (0x7fff_0001_i32, 0x7fff_0002_i32);
    let r = o(clube, &mut bo, forcada);
    let n = nota_alvo_do_clube(clube, &mut br, forcada);
    PLACAR.registrar(r == n && bo == br, || format!("nota_alvo_do_clube({:x}, forcada {}) {}/{} base {}/{}", clube, if forcada.is_null() { -1 } else { *forcada as i32 }, r, n, bo, br));
    if !base.is_null() {
        *base = bo;
    }
    r
}

/// Comparador: pura: compara a devolução.
unsafe extern "thiscall" fn comparar_jogo_do_historico(this: usize, i: u32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(1231)(this, i);
    let n = jogo_do_historico(this, i);
    PLACAR.registrar(r == n, || format!("jogo_do_historico({:x}, {}) {:x}/{:x}", this, i as u8 as i8, r, n));
    r
}

/// Comparador: pura: compara a devolução.
unsafe extern "thiscall" fn comparar_gols_do_historico(this: usize) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize) -> u16>(1232)(this);
    let n = gols_do_historico(this);
    PLACAR.registrar(r == n, || format!("gols_do_historico({:x}) {}/{}", this, r, n));
    r
}

// ---------- 244a leva: o olheiro da busca, a sequência atual do clube, construir o recordista e o técnico rival que comenta na mídia ----------


unsafe fn cph7_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// { 0x7ae950 } (thiscall, player_search.cpp): `PLAYER_SEARCH::get_scout`: o olheiro da busca: o índice em +0x180 (> −1)
/// no vetor de pessoas [0xae23bc] (110 bytes), se ainda confere com os nomes e a data guardados (0x53e990 com +0x174,
/// +0x178, +0x17c e a data em +0x184); senão 0.
unsafe extern "thiscall" fn olheiro_da_busca(this: usize) -> usize {
    let i = read_u32(this + 0x180) as i32;
    if i <= -1 {
        return 0;
    }
    let pessoa = (read_u32(0xae23bc) as usize).wrapping_add((i as usize).wrapping_mul(110));
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x53e990usize);
    if confere(pessoa, read_u32(this + 0x174), read_u32(this + 0x178), read_u32(this + 0x17c), this + 0x184) == 0 {
        return 0; // a vaga mudou de pessoa
    }
    pessoa
}

/// { 0x690c30 } (thiscall, ret 0xc, manager_manager.cpp): `MANAGER_MANAGER::get_current_run(clube, tipo, tamanho)`:
/// a sequência atual do clube na liga (+0x57; a competição pelo id em [0xadadfc], estatísticas em +8, a linha do clube
/// pela 0x4a4850): a primeira de mais de 1 jogo cujo último foi há menos de 10 dias (0x54cdb0 da data do jogo), na
/// ordem +0x19 (tipo 1, data +0x25), +0x2d (3, +0x39), +0x55 (4, +0x61) e +0x41 (2, +0x4d): o tipo e o tamanho (byte).
/// Nenhuma (ou sem liga, competição, estatísticas ou linha): tipo 5 e tamanho 0. O `this` não é usado.
unsafe extern "thiscall" fn sequencia_atual_do_clube(_this: usize, clube: usize, tipo: *mut u8, tamanho: *mut u8) {
    let liga = read_u32(clube + 0x57) as usize;
    let comp = if liga != 0 { read_u32((read_u32(0xadadfc) as usize).wrapping_add((read_u32(liga) as usize).wrapping_mul(4))) as usize } else { 0 };
    let stats = if comp != 0 { read_u32(comp + 8) as usize } else { 0 };
    if stats != 0 {
        let linha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
        let e = linha(stats, clube);
        if e != 0 {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
            for (n, data, t) in [(0x19usize, 0x25usize, 1u8), (0x2d, 0x39, 3), (0x55, 0x61, 4), (0x41, 0x4d, 2)] {
                if read_u32(e + n) as i32 > 1 && dias(0xae2c90, read_u32(e + data), read_u32(e + data + 4)) < 10 { // há menos de 10 dias
                    *tipo = t;
                    *tamanho = *((e + n) as *const u8);
                    return;
                }
            }
        }
    }
    *tipo = 5; // nenhuma
    *tamanho = 0;
}

/// { 0x450d40 } (thiscall, club_records.cpp): `record_holder_construct`: constrói o recordista: só a data em +0x10
/// (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recordista(this: usize) -> usize {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    padrao(this + 0x10); // a data do recordista
    this
}

/// O técnico (+0xcf) de um clube rival passa nos sorteios (sorteio(+0x5c) = 0 e sorteio(+0x57) > 5)?
unsafe fn cph7_sorteado(c: usize) -> bool {
    cph7_sortear(*((read_u32(c + 0xcf) as usize + 0x5c) as *const i8) as i32) == 0
        && cph7_sortear(*((read_u32(c + 0xcf) as usize + 0x57) as *const i8) as i32) > 5
}

/// { 0x72b940 } (thiscall, ret 4, media.cpp): `Media::pick_rival_manager_id(clube)`: com técnico (+0xcf) com contrato
/// (+0x69) e liga (+0x57; sem liga, só seleção, 0x53b900): o primeiro dos rejeitados do técnico (0x5480f0) com
/// registro de técnico e nota + 750 < nota (nunca: o sorteio(3) dali não acontece); depois cada rival (+0xb3, +0xb7,
/// +0xbb) com técnico que passa nos sorteios, da mesma liga e de reputação (+0x80) acima da do clube + 750, com
/// sorteio(3) ≠ 0; depois até 50 clubes sorteados (de [0xae235c] − 2 × [0xae2350]) que dão confronto na liga
/// (0x4b02e0, com o limite do sorteio como 3º argumento, resto da pilha do original) com técnico que passa nos
/// sorteios. Devolve o id do técnico, ou −1.
unsafe extern "thiscall" fn tecnico_rival_para_comentar(_this: usize, clube: usize) -> u32 {
    if read_u32(clube + 0xcf) == 0 {
        return u32::MAX;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if read_u32(clube + 0x57) == 0 && selecao(clube) == 0 {
        return u32::MAX;
    }
    let tecnico = read_u32(clube + 0xcf) as usize;
    if read_u32(tecnico + 0x69) == 0 {
        return u32::MAX;
    }
    let mut rejeitados = [0usize; 20];
    let mut n = clube as u32; // a vaga do argumento: a 0x5480f0 zera o byte antes de contar
    let rejeitadas: unsafe extern "C" fn(usize, usize, usize) -> usize = core::mem::transmute(0x5480f0usize);
    rejeitadas(tecnico, rejeitados.as_mut_ptr() as usize, &mut n as *mut u32 as usize);
    let mut k = 0i8;
    while k < n as u8 as i8 {
        let p = *rejeitados.as_ptr().add(k as u8 as usize); // como o original, sem conferir o tamanho (a 0x5480f0 dá até 3)
        if p != 0 && read_u32(p + 0x69) != 0 {
            let nota = *((read_u32(p + 0x69) as usize + 0xa) as *const i16) as i32;
            if nota.wrapping_add(0x2ee) < nota && cph7_sortear(3) != 0 {
                return read_u32(p);
            }
        }
        k = k.wrapping_add(1);
    }
    for rival in [0xb3usize, 0xb7, 0xbb] {
        let r = read_u32(clube + rival) as usize;
        if r == 0 || read_u32(r + 0xcf) == 0 || !cph7_sorteado(r) {
            continue;
        }
        if read_u32(r + 0x57) != read_u32(clube + 0x57) {
            continue;
        }
        let reputacao = |c: usize| *((c + 0x80) as *const i16) as i32;
        if reputacao(clube) + 0x2ee >= reputacao(r) {
            continue; // o rival não é bem maior
        }
        if cph7_sortear(3) != 0 {
            return read_u32(read_u32(r + 0xcf) as usize);
        }
    }
    let confronto: unsafe extern "C" fn(usize, usize, i32) -> u8 = core::mem::transmute(0x4b02e0usize);
    let mut b = 0i8;
    while b < 0x32 {
        let limite = (read_u32(0xae235c) as i32).wrapping_sub((read_u32(0xae2350) as i32).wrapping_mul(2));
        let i = cph7_sortear(limite) as usize;
        let c = (read_u32(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
        if confronto(clube, c, limite) != 0 && read_u32(c + 0xcf) != 0 && cph7_sorteado(c) {
            return read_u32(read_u32(c + 0xcf) as usize);
        }
        b += 1;
    }
    u32::MAX
}

// a 0x53e990 só lê: compara o valor
comparar_valor!(comparar_olheiro_da_busca, olheiro_da_busca, 842, () -> usize);

/// Comparador: a original grava nas saídas de verdade, a cópia em locais; a 0x4a4850 pode reordenar as estatísticas
/// (a original primeiro; a cópia acha a tabela já em ordem e a mesma linha).
unsafe extern "thiscall" fn comparar_sequencia_atual_do_clube(this: usize, clube: usize, tipo: *mut u8, tamanho: *mut u8) {
    orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(843)(this, clube, tipo, tamanho);
    let (mut t, mut n) = (0u8, 0u8);
    sequencia_atual_do_clube(this, clube, &mut t, &mut n);
    PLACAR.registrar(*tipo == t && *tamanho == n, || format!("sequencia_atual_do_clube({:x}) original={}/{} rust={}/{}", clube, *tipo, *tamanho, t, n));
}

unsafe extern "thiscall" fn comparar_construir_recordista(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(844);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x10, 8)], || o(this), || construir_recordista(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recordista {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio, a cópia em sem_contar (as escritas são só nas
/// locais da pilha de cada uma).
unsafe extern "thiscall" fn comparar_tecnico_rival_para_comentar(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(845);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(this, clube),
        || crate::aleatorio::sem_contar(|| tecnico_rival_para_comentar(this, clube)));
    PLACAR.registrar(igual && r == n, || format!("tecnico_rival_para_comentar({:x}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}

// ---------- 195a leva: a última nota do jogador e a vaga e o tipo da busca de jogadores (player_stats.cpp, player_search.cpp) ----------

/// { 0x7c1a60 } (thiscall, ret 4, player_stats.cpp): a nota da última partida do jogador `*id` (os 4
/// bits baixos do byte +0x26 do registro de 39 bytes em [estatisticas+0x30]); sem nota, 6,5
/// ([0x96e2b8]). Jogador nulo: a original (caixa de erro). Devolve em st0.
unsafe extern "thiscall" fn ultima_nota_do_jogador(estatisticas: usize, id: *const i32) -> f32 {
    if id.is_null() {
        return orig::<unsafe extern "thiscall" fn(usize, *const i32) -> f32>(457)(estatisticas, id);
    }
    let registro = (*((estatisticas + 0x30) as *const usize) as isize + *id as isize * 39) as usize;
    let nota = *((registro + 0x26) as *const u8) & 0xf;
    if nota != 0 { nota as f32 } else { *(0x96e2b8 as *const f32) }
}

/// { 0x7ae640 } (thiscall, ret 4, player_search.cpp): grava a vaga `vaga` (0 a 9) da busca em +4 e
/// devolve 1. Fora da faixa: a original (caixa de erro, devolve 0).
unsafe extern "thiscall" fn vaga_da_busca(busca: usize, vaga: i8) -> u32 {
    if !(0..=9).contains(&vaga) {
        return orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(458)(busca, vaga);
    }
    *((busca + 4) as *mut i8) = vaga;
    1
}

/// { 0x7ae6e0 } (thiscall, ret 4, player_search.cpp): grava o tipo de lista `tipo` (1 a 4) da busca em
/// +0xa e devolve 1. Fora da faixa: a original (caixa de erro, devolve 0).
unsafe extern "thiscall" fn tipo_da_lista_da_busca(busca: usize, tipo: i8) -> u32 {
    if !(1..=4).contains(&tipo) {
        return orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(459)(busca, tipo);
    }
    *((busca + 0xa) as *mut i8) = tipo;
    1
}

unsafe extern "thiscall" fn comparar_ultima_nota_do_jogador(estatisticas: usize, id: *const i32) -> f32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const i32) -> f32>(457)(estatisticas, id);
    if id.is_null() {
        return r;
    }
    let n = ultima_nota_do_jogador(estatisticas, id);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("ultima_nota_do_jogador({}) original={} rust={}", *id, r, n));
    r
}

unsafe extern "thiscall" fn comparar_vaga_da_busca(busca: usize, vaga: i8) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(458);
    if !(0..=9).contains(&vaga) {
        return o(busca, vaga);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(busca + 4, 1)], || o(busca, vaga), || vaga_da_busca(busca, vaga));
    PLACAR.registrar(igual && ro == rr, || format!("vaga_da_busca({}) original={} rust={} {:x?} x {:x?}", vaga, ro, rr, mo, mr));
    ro
}

unsafe extern "thiscall" fn comparar_tipo_da_lista_da_busca(busca: usize, tipo: i8) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(459);
    if !(1..=4).contains(&tipo) {
        return o(busca, tipo);
    }
    let (ro, rr, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(busca + 0xa, 1)], || o(busca, tipo), || tipo_da_lista_da_busca(busca, tipo));
    PLACAR.registrar(igual && ro == rr, || format!("tipo_da_lista_da_busca({}) original={} rust={} {:x?} x {:x?}", tipo, ro, rr, mo, mr));
    ro
}

