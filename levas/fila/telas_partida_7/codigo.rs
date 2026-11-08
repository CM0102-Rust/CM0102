// ---------- %%LEVA%%a leva: as telas da partida (7): o Continuar da partida, abrir a escalação e a resposta da escalação (match_screens.cpp) ----------

/// A vaga de tela do humano da vez ([0xb74c26 + 0x300 × humano]).
unsafe fn vaga_de_tela_do_humano() -> usize {
    core::ptr::read_unaligned((0xb74c26 + *(0xb77c7a as *const i16) as isize * 0x300) as *const usize)
}

/// O id do item `item` (−1 nenhum: o padrão [0xdd6bde]), como as respostas de match_screens.cpp leem.
unsafe fn id_do_item_ou_padrao(item: i16) -> u16 {
    if item == -1 { *(0xdd6bde as *const u16) } else { id_do_item(item) }
}

/// { 0x725ca0 } (cdecl, match_screens.cpp): a resposta ao Continuar da partida (item `item`, −1 nenhum).
/// Em rede ([0xb77c84]): com o comando pendente 1 ([0xdd6be0], zerado com [0xdd6be4]) e a escolha 3,
/// procura na lista de vagas do gerente (0x7ffd80) outra vaga livre (0x7ffdd0) do mesmo clube do humano
/// (0x765480 +0xe2): achou, guarda em [0xb77c4a] e devolve −6; senão, com outros jogadores a esperar
/// (0x803d80), avisa "Waiting for other players..." (0x802f80), abre a tela da partida (0x718aa0) e
/// devolve −4, ou −11. Comando pendente 0x43: abre a escalação (0x725f70) e devolve −4. Depois, só o
/// item de id 0x3e8: com a rede da partida ([0xb67a48]) abre a primeira entrada da lista [0xb67a50]
/// (+0xd, 0x54 bytes; primeiro as de +0x10, depois as de +0xc) que o humano ainda não marcou (0x717cc0,
/// −4) ou devolve −8; sem ela, com ficha e clube, abre a escalação se 0x474e00 negar e devolve −4.
unsafe extern "C" fn match_screens_handle_continue_command(item: u32) -> i32 {
    crate::comparacao::ramo("telas respond ao continuar");
    let g = |a: usize| *(a as *const u32);
    let humano = || *(0xb77c7a as *const i16);
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x765480usize);
    let abrir_escalacao: unsafe extern "C" fn() = core::mem::transmute(0x725f70usize);
    let r = ficha(0);
    let clube = if r != 0 { core::ptr::read_unaligned((r + 0xe2) as *const usize) } else { 0 };
    if g(0xb77c84) != 0 {
        let comando = g(0xdd6be0);
        if comando == 1 {
            let escolha = g(0xdd6be4);
            *(0xdd6be0 as *mut u32) = 0;
            *(0xdd6be4 as *mut u32) = 0;
            if escolha == 3 {
                let listar: unsafe extern "thiscall" fn(usize, *mut usize, *mut i16, u32) = core::mem::transmute(0x7ffd80usize);
                let ocupada: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7ffdd0usize);
                let (mut n, mut lista) = (0i16, [0usize; 16]);
                // a original empurra eax inteiro: os 16 bits altos são os da ficha devolvida por 0x765480
                listar(GERENTE_DE_TELAS, lista.as_mut_ptr(), &mut n, (r as u32 & 0xffff_0000) | humano() as u16 as u32);
                let mut i: i8 = 0;
                if n > 0 {
                    loop {
                        let e = *lista.as_ptr().offset(i as isize);
                        if e != vaga_de_tela_do_humano()
                            && ocupada(GERENTE_DE_TELAS, e) == 0
                            && core::ptr::read_unaligned((ficha(e) + 0xe2) as *const usize) == clube
                        {
                            core::ptr::write_unaligned(0xb77c4a as *mut usize, e);
                            return -6;
                        }
                        i = i.wrapping_add(1);
                        if i as i16 >= n {
                            break;
                        }
                    }
                }
                let esperando: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x803d80usize);
                if esperando(GERENTE_DE_TELAS) == 0 {
                    return -11;
                }
                let formatar_texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
                formatar_texto(0xde1f64, 0xa16d94);
                let aviso: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x802f80usize);
                // ponytail: a original empurra ecx com 16 bits altos que sobram de 0x66f4e0 (imprevisíveis); aqui zero
                aviso(GERENTE_DE_TELAS, 0xde1f64, 0, 0, humano() as u16 as u32);
                let abrir_partida: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x718aa0usize);
                abrir_partida(vaga_de_tela_do_humano(), clube);
                return -4;
            }
        } else if comando == 0x43 {
            *(0xdd6be0 as *mut u32) = 0;
            abrir_escalacao();
            return -4;
        }
    }
    if id_do_item_ou_padrao(item as u16 as i16) != 0x3e8 {
        return 0;
    }
    if g(0xb67a48) != 0 {
        let n = core::ptr::read_unaligned((*(0xb67a50 as *const usize) + 0x15) as *const i32);
        for modo in 1..=2u8 {
            let base = core::ptr::read_unaligned((*(0xb67a50 as *const usize) + 0xd) as *const usize);
            let campo = if modo == 1 { 0x10 } else { 0xc };
            let mut i: i8 = 0;
            if n > 0 {
                loop {
                    let e = (base as isize + i as isize * 0x54) as usize;
                    if core::ptr::read_unaligned((e + campo) as *const u32) != 0
                        && core::ptr::read_unaligned((e as isize + humano() as isize * 4 + 0x14) as *const u32) == 0
                    {
                        let abrir: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x717cc0usize);
                        abrir(vaga_de_tela_do_humano(), e);
                        return -4;
                    }
                    i = i.wrapping_add(1);
                    if i as i32 >= n {
                        break;
                    }
                }
            }
        }
        return -8;
    }
    if r != 0 && clube != 0 {
        let pronta: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x474e00usize);
        if pronta(r) == 0 {
            abrir_escalacao();
        }
        return -4;
    }
    0
}

/// { 0x725f70 } (cdecl, match_screens.cpp): abre a tela da escalação da próxima partida do humano
/// (0x7fe600 com a montagem 0x7260f0 e a resposta 0x727c30) e grava os parâmetros: 0 o clube (ficha
/// 0x765480 +0xe6), 1 a partida ([ficha+0xe2] + 4) e 2 se o clube joga em casa (+0x1c sem a marca +0x40,
/// ou +0x20 sem a +0x41). Sem ficha, clube ou partida: a caixa de erro (linhas 0x1e1c, 0x1e24, 0x1e2c),
/// como a original (a tela já ficou aberta).
unsafe extern "C" fn match_screens_open_team_selection() {
    crate::comparacao::ramo("telas abrir a escalacao");
    let abrir: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(0x7fe600usize);
    if abrir(GERENTE_DE_TELAS, 0x7260f0, 0x727c30, 1, 0, 0) == 0 {
        return;
    }
    const FONTE: usize = 0xa161d4; // match_screens.cpp
    let ficha: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x765480usize);
    let r = ficha(0);
    if r == 0 {
        return aviso_em(FONTE, 0x1e1c);
    }
    let clube = core::ptr::read_unaligned((r + 0xe6) as *const usize);
    if clube == 0 {
        return aviso_em(FONTE, 0x1e24);
    }
    let partida = core::ptr::read_unaligned((r + 0xe2) as *const usize).wrapping_add(4);
    if partida == 0 {
        return aviso_em(FONTE, 0x1e2c);
    }
    let d = |o: usize| core::ptr::read_unaligned((partida + o) as *const usize);
    let b = |o: usize| *((partida + o) as *const u8);
    let em_casa = (clube == d(0x1c) && b(0x40) == 0) || (clube == d(0x20) && b(0x41) == 0);
    gravar_parametro_da_tela(0, clube);
    gravar_parametro_da_tela(1, partida);
    gravar_parametro_da_tela(2, em_casa as usize);
}

/// { 0x727c30 } (cdecl, match_screens.cpp): a resposta da tela da escalação ao item `item` (−1 nenhum).
/// Ids fora de 1, 2 e 3: a resposta comum (0x7615d0 e, se der 0, 0x7643d0) com o argumento inteiro. Id 3
/// (confirmar): se a tática [0xde1d08] não tem capitão (0x897590 com 1 negativo) ou ele não é titular
/// (0x897590 com 0 ≥ 11), nomeia o melhor titular (0x727e20) por 0x897760. Para 1, 2 e 3 deixa o
/// comando pendente 1 ([0xdd6be0]) com a escolha (o id, [0xdd6be4]) e devolve −9.
unsafe extern "C" fn match_screens_handle_team_selection_input(item: u32) -> i32 {
    crate::comparacao::ramo("telas respond a escalacao");
    let clube = parametro_da_tela(0);
    let p2 = parametro_da_tela(2);
    let item16 = item as u16 as i16;
    let id = id_do_item_ou_padrao(item16);
    if id != 1 && id != 2 && id != 3 {
        let comum: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7615d0usize);
        let r = comum(item);
        if r != 0 {
            return r;
        }
        let outra: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7643d0usize);
        return outra(item);
    }
    if id == 3 {
        let capitao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x897590usize);
        let tatica = || *(0xde1d08 as *const usize);
        if (capitao(tatica(), clube, p2, 1) as u8 as i8) < 0 || (capitao(tatica(), clube, p2, 0) as u8 as i8) >= 11 {
            let melhor: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x727e20usize);
            let m = melhor(clube, p2) as u8;
            if (m as i8) > -1 {
                // a original grava o byte por cima do argumento e empurra a palavra inteira
                let nomear: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x897760usize);
                nomear(tatica(), clube, (item & 0xffff_ff00) | m as u32, p2);
            }
        }
    }
    *(0xdd6be0 as *mut u32) = 1;
    *(0xdd6be4 as *mut i32) = id_do_item_ou_padrao(item16) as i16 as i32;
    -9
}

// ponytail: as três abrem telas, mexem na rede ou no capitão (não rodam duas vezes): no modo comparação
// só a original; a cópia é provada no modo Rust pelos ramos
unsafe extern "C" fn comparar_match_screens_handle_continue_command(item: u32) -> i32 {
    orig::<unsafe extern "C" fn(u32) -> i32>(%%I0%%)(item)
}

unsafe extern "C" fn comparar_match_screens_open_team_selection() {
    orig::<unsafe extern "C" fn()>(%%I1%%)()
}

unsafe extern "C" fn comparar_match_screens_handle_team_selection_input(item: u32) -> i32 {
    orig::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}
