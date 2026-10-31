// ---------- %%LEVA%%a leva: as relações do elenco no dia a dia: as brigas e preferências entre colegas, o atrito com o técnico e o moral espalhado pela liderança (game.cpp) ----------

unsafe fn cps5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps5_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cps5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cps5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O clube de índice `i` (registros de 0x245 bytes em [0xae23b4]; a conta em 32 bits, como a original).
unsafe fn cps5_clube(i: usize) -> usize {
    cps5_d(0xae23b4).wrapping_add(i.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz a cada chamada.
unsafe fn cps5_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cps5_d(0xae1050), p);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    c
}

/// O clube do contrato (pelo id em +4), ou 0 sem contrato.
unsafe fn cps5_clube_do_contrato(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        cps5_clube(cps5_d(c + 4))
    }
}

/// O vínculo da pessoa com o clube (0x4cf5f0).
unsafe fn cps5_vinculo(p: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    f(cps5_d(0xae1050), p, clube)
}

/// `pessoa` não gosta de `alvo` (0x547840)?
unsafe fn cps5_nao_gosta(alvo: usize, pessoa: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    f(alvo, pessoa)
}

/// O fim comum do atrito (0x5d6d10 e 0x5d7190): o primeiro colega do elenco (+0xd7, 50 vagas) de quem a pessoa não
/// gosta; achado, e só quando dia % 14 + id da pessoa dá zero (na prática a pessoa de id 0 a cada 14 dias), a queixa
/// (0x587c60) contra o técnico (+0xcf, tipo 0), o auxiliar (+0xd3, 1) ou outro (2) e a marca no vínculo `k` (0x867700
/// com 0x2000, 0x1000 ou 0x800).
unsafe fn cps5_atrito(p: usize, clube: usize, k: usize) {
    let mut j = 0usize;
    loop {
        if j >= 0x32 {
            return;
        }
        let q = cps5_d(clube + 0xd7 + j * 4);
        if q != p && q != 0 && cps5_nao_gosta(q, p) != 0 {
            break;
        }
        j += 1;
    }
    let q = cps5_d(clube + 0xd7 + j * 4);
    if q == 0 {
        return;
    }
    if ((cps5_w(0xae2c90) as i32) % 0xe).wrapping_add(cps5_d(p) as i32) != 0 {
        return;
    }
    crate::comparacao::ramo("cps_5 atrito com o tecnico");
    let queixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x587c60usize);
    let marcar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x867700usize);
    let (tipo, marca) = if q == cps5_d(clube + 0xcf) {
        (0, 0x2000)
    } else if q == cps5_d(clube + 0xd3) {
        (1, 0x1000)
    } else {
        (2, 0x800)
    };
    queixa(p, q, tipo);
    marcar(k, marca, 0);
}

/// { 0x5d6d10 } (cdecl, game.cpp): `game_update_squad_relationships()`, todo dia pelo laço do dia do jogo. (1) Dez
/// vezes: um clube sorteado e duas vagas do elenco sorteadas (50); pessoas diferentes e sorteio(+0x5b da primeira) = 0:
/// se a primeira não gosta da segunda, deixa de não gostar (0x547c50) e, no mesmo clube de contrato e com o vínculo
/// marcado em +0x45 & 0x800, a briga (0x5888a0); senão, 1/3 passa a não gostar (0x547af0), 1/3 do resto passa a gostar
/// (0x547a10) e o resto deixa de gostar (0x547c10). (2) Dia múltiplo de 50: [0xae235c]/100 clubes sorteados passam
/// pelo atrito no vestiário (0x5d7190). Nos outros dias, cada pessoa (menos as 17 últimas) com ficha de jogador e
/// preferências, contrato e o vínculo do clube com a pessoa de +0xff: três sorteios (+0x5b, +0x5d, +0x5a, cada um + 5)
/// filtram; a soma de cinco atributos menos 3, 5, 15, 15 por sorteios zerados e 5 se a reputação do jogador (+0xb)
/// passa de 4500 e da do clube (+0x80) dá o sorteio que, zerado, leva ao atrito com o técnico.
unsafe extern "C" fn relacoes_do_elenco() {
    let tirar_rejeitada: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x547c50usize);
    let briga: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x5888a0usize);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
    let favorecer: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547a10usize);
    let tirar_favorita: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x547c10usize);
    for _ in 0..10 {
        let clube = cps5_clube(cps5_sortear(cps5_d(0xae235c) as i32) as usize);
        let a = cps5_d(clube.wrapping_add(cps5_sortear(0x32) as usize * 4) + 0xd7);
        let b = cps5_d(clube.wrapping_add(cps5_sortear(0x32) as usize * 4) + 0xd7);
        if a == 0 || b == 0 || a == b {
            continue;
        }
        if cps5_sortear(cps5_sb(a + 0x5b)) != 0 {
            continue;
        }
        if cps5_nao_gosta(b, a) != 0 {
            tirar_rejeitada(a, b);
            let c = cps5_contrato(a);
            if c == 0 || cps5_clube_do_contrato(c) == 0 {
                continue;
            }
            let ca = cps5_clube_do_contrato(cps5_contrato(a));
            let cb = cps5_clube_do_contrato(cps5_contrato(b));
            if ca != cb {
                continue;
            }
            let k = cps5_vinculo(a, cps5_clube_do_contrato(cps5_contrato(a)));
            if k == 0 || cps5_d(k + 0x45) & 0x800 == 0 {
                continue;
            }
            briga(a, b);
        } else if cps5_sortear(3) == 0 {
            rejeitar(a, b, 1);
        } else if cps5_sortear(3) == 0 {
            favorecer(a, b, 1);
        } else {
            tirar_favorita(a, b);
        }
    }
    if (cps5_w(0xae2c90) as i32) % 0x32 == 0 {
        let atrito: unsafe extern "C" fn(usize) = core::mem::transmute(0x5d7190usize);
        let mut i = 0i32;
        while i < cps5_d(0xae235c) as i32 / 100 {
            atrito(cps5_clube(cps5_sortear(cps5_d(0xae235c) as i32) as usize));
            i += 1;
        }
        return;
    }
    let base = cps5_d(0xae23bc);
    let fim = base.wrapping_add((cps5_d(0xae2364).wrapping_sub(17)).wrapping_mul(0x6e));
    let mut p = base;
    while p < fim {
        cps5_pessoa_e_tecnico(p);
        p += 0x6e;
    }
}

/// Uma pessoa do laço de fora dos dias múltiplos de 50 (0x5d6f15..0x5d7115).
unsafe fn cps5_pessoa_e_tecnico(p: usize) {
    if cps5_d(p + 0x61) == 0 || cps5_d(p + 0x65) == 0 {
        return;
    }
    let c = cps5_contrato(p);
    if c == 0 {
        return;
    }
    let clube = cps5_clube_do_contrato(c);
    if clube == 0 {
        return;
    }
    let k = cps5_vinculo(cps5_d(clube + 0xff), clube);
    if k == 0 {
        return;
    }
    if cps5_sortear(cps5_sb(p + 0x5b) + 5) != 0 && cps5_sortear(cps5_sb(p + 0x5d) + 5) != 0 && cps5_sortear(cps5_sb(p + 0x5a) + 5) != 0 {
        return;
    }
    let mut e = cps5_sb(p + 0x5b) + (cps5_sb(p + 0x5a) + cps5_sb(p + 0x59)) + (cps5_sb(p + 0x5c) + cps5_sb(p + 0x18));
    crate::comparacao::ramo("cps_5 tecnico sorteado");
    if cps5_sortear(cps5_sb(p + 0x59)) == 0 { // cps5: o primeiro desconto
        e -= 3;
    }
    if cps5_sortear(cps5_sb(p + 0x5a)) == 0 {
        e -= 5;
    }
    if cps5_d(k + 0x45) != 0 || cps5_sortear(0x64) == 0 {
        if cps5_sortear(cps5_sb(p + 0x5b)) == 0 {
            e -= 0xf;
        }
        if cps5_sortear(cps5_sb(p + 0x5d)) == 0 {
            e -= 0xf;
        }
    }
    let rep = cps5_w(cps5_d(p + 0x61) + 0xb);
    if rep > 0x1194 && rep > cps5_w(clube + 0x80) {
        e -= 5;
    }
    if cps5_sortear(e) >= 1 {
        return;
    }
    if cps5_d(p + 0x65) == 0 {
        return;
    }
    cps5_atrito(p, clube, k);
}

// ponytail: sorteia e mexe nas preferências (0x547a10, 0x547af0, 0x547c10, 0x547c50), briga e queixa: só a original;
// a cópia leva os ramos (roda todo dia).
unsafe extern "C" fn comparar_relacoes_do_elenco() {
    orig::<unsafe extern "C" fn()>(%%I0%%)()
}

/// { 0x5d7190 } (cdecl, game.cpp): `game_check_club_dressing_room_unrest(clube)`: cada pessoa do elenco (+0xd7, 50
/// vagas) com vínculo com o clube (0x4cf5f0) e preferências (+0x65) passa pelo atrito com o técnico (o primeiro colega
/// de quem não gosta; a queixa só com dia % 14 + id = 0). Pelas relações do elenco (0x5d6d10) nos dias múltiplos de 50.
unsafe extern "C" fn atrito_com_o_tecnico(clube: usize) {
    for i in 0..0x32usize {
        let p = cps5_d(clube + 0xd7 + i * 4);
        if p == 0 {
            continue;
        }
        let k = cps5_vinculo(p, clube);
        if k == 0 {
            continue;
        }
        if cps5_d(p + 0x65) == 0 {
            continue;
        }
        cps5_atrito(p, clube, k);
    }
}

// ponytail: a queixa (0x587c60) e a marca no vínculo (0x867700) não rodam duas vezes: só a original; a cópia leva o
// ramo do atrito.
unsafe extern "C" fn comparar_atrito_com_o_tecnico(clube: usize) {
    orig::<unsafe extern "C" fn(usize)>(%%I1%%)(clube)
}

/// A pessoa conta para a liderança do clube: id válido (< [0xae2364]), contrato com o clube (+0x39) e ficha de jogador.
unsafe fn cps5_do_elenco(p: usize, clube: usize) -> bool {
    p != 0 && (cps5_d(p) as i32) < cps5_d(0xae2364) as i32 && cps5_d(p + 0x39) == clube && cps5_d(p + 0x61) != 0
}

/// { 0x5ce0b0 } (cdecl, game.cpp): `game_spread_squad_morale_by_leadership()`, todo dia pelo laço do dia do jogo. Cada
/// clube que não é seleção (0x53b900), com sorteio(7) = 0: soma a liderança (jogador +0x2f) e liderança × moral do
/// vínculo (+0x44) do elenco (+0xd7, 50 vagas); menos de 16 jogadores contam 10 cada um que falta. Com soma não nula,
/// cada jogador sorteia (2 × Σ liderança × moral) / (liderança dele + soma) mais sorteio(moral / 12), em byte com sinal:
/// negativo, sorteio(s) < sorteio(−5) baixa o moral (0x544710 com −1); positivo, sorteio(s) > sorteio(5) sobe (+1).
unsafe extern "C" fn moral_pela_lideranca() {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let moral: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(0x544710usize);
    let n = cps5_d(0xae235c);
    let mut clube = cps5_d(0xae23b4);
    let fim = clube.wrapping_add(n.wrapping_mul(0x245));
    while clube < fim {
        let vagas = clube + 0xd7;
        if e_selecao(clube) == 0 && cps5_sortear(7) == 0 {
            let (mut soma, mut lideranca, mut q) = (0i32, 0i32, 0i32);
            for i in 0..0x32usize {
                let p = cps5_d(vagas + i * 4);
                if !cps5_do_elenco(p, clube) {
                    continue;
                }
                let c = cps5_vinculo(p, clube);
                if c == 0 {
                    continue;
                }
                let l = cps5_sb(cps5_d(p + 0x61) + 0x2f);
                soma = soma.wrapping_add(cps5_sb(c + 0x44) * l);
                lideranca += l;
                q += 1;
            }
            if q < 0x10 {
                lideranca += (0x10 - q) * 10;
            }
            for i in 0..0x32usize {
                if lideranca == 0 {
                    break;
                }
                let p = cps5_d(vagas + i * 4);
                if !cps5_do_elenco(p, clube) {
                    continue;
                }
                let c = cps5_vinculo(p, clube);
                let m = if c == 0 { 0 } else { cps5_sb(c + 0x44) };
                let l = cps5_sb(cps5_d(p + 0x61) + 0x2f);
                crate::comparacao::ramo("cps_5 moral sorteado");
                let r1 = cps5_sortear(soma.wrapping_mul(2) / (l + lideranca));
                let r2 = cps5_sortear(m / 12); // cps5: a parte do moral
                let s = (r1 as u8).wrapping_add(r2 as u8) as i8;
                if s < 0 {
                    let e = cps5_sortear(s as i32);
                    if e < cps5_sortear(-5) {
                        moral(p, -1, 1);
                    }
                } else if s > 0 {
                    let e = cps5_sortear(s as i32);
                    if e > cps5_sortear(5) {
                        crate::comparacao::ramo("cps_5 moral sobe");
                        moral(p, 1, 1); // cps5: sobe o moral
                    }
                }
            }
        }
        clube = clube.wrapping_add(0x245);
    }
}

// ponytail: sorteia e muda o moral (0x544710, que também sorteia): só a original; a cópia leva os ramos.
unsafe extern "C" fn comparar_moral_pela_lideranca() {
    orig::<unsafe extern "C" fn()>(%%I2%%)()
}
