// ---------- %%LEVA%%a leva: o veto dos fisioterapeutas e da diretoria à transferência, a notícia do ultimato da diretoria e o texto dela (manager_manager.cpp) ----------

unsafe fn cpy12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy12_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpy12_clube(id: usize) -> usize {
    cpy12_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O registro da diretoria do clube na gerência dos técnicos ([this] + id × 0x49).
unsafe fn cpy12_diretoria(this: usize, clube: usize) -> usize {
    cpy12_d(this).wrapping_add(cpy12_d(clube).wrapping_mul(0x49))
}

/// Soma `delta` à palavra `o` (+6, +0x14, +0x16: as confianças) da diretoria do clube.
unsafe fn add_board_confidence(this: usize, clube: usize, o: usize, delta: i16) {
    let a = (cpy12_diretoria(this, clube) + o) as *mut i16;
    core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(delta));
}

/// O piso da confiança +6 em 1 e o veto 0x800.
unsafe fn clamp_confidence_floor(this: usize, clube: usize) -> u32 {
    let a = (cpy12_diretoria(this, clube) + 6) as *mut i16;
    if core::ptr::read_unaligned(a) < 1 {
        core::ptr::write_unaligned(a, 1);
    }
    0x800
}

/// O valor do jogador (CONTRACT_MANAGER::calc_player_value 0x4d0c60 em [0xae1050] com 0, −1 e `modo`).
unsafe fn player_value(j: usize, clube: usize, modo: u32) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    f(cpy12_d(0xae1050), j, clube, 0, -1, modo)
}

/// O clube do contrato do jogador (0x4cf5a0, guardado em [0xae1080]) ou 0.
unsafe fn contract_club_cached(j: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cpy12_d(0xae1050), j);
    *(0xae1080 as *mut usize) = c;
    if c == 0 { 0 } else { cpy12_clube(cpy12_d(c + 4)) }
}

/// Se a cópia semeou o rand da CRT pelo relógio (os fisioterapeutas avaliaram): o comparador não confere o estado do
/// rand (o relógio anda entre a original e a cópia no jogo real).
static mut CPY12_SEMEOU: bool = false;

/// { 0x696b10 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::check_board_transfer_veto(jogador, clube,
/// valor, comprando)`, pela 0x8cdb80 (oferta sem troca). Comprando um jogador com lesão (0x62ed70) de gravidade +0x28 > 20,
/// cada um dos três fisioterapeutas do clube (+0x1cf, com ficha +0x69) semeia o rand da CRT pelos dois ids e sorteia:
/// aposentadoria anunciada pela lesão (0x7a7ed0) contra a nota dele (0x544410 do byte +0x1e), a gravidade contra
/// sorteio(200) + 25 e (41 − nota) × 5, a recaída (0x62ece0), a nota³ + 100 (pow 0x945ff0) e a nota contra a do jogador
/// (+0x61 +0x2d): qualquer um veta (0x2000); no fim o rand volta ao relógio (0x946ebb). Depois, só com contrato fora dos
/// tipos 1 e 3, clube atual (+0x39) e clube humano (0x600a50): o limite (reputação +0x80 × 0,2)² entre 150 mil e 2
/// milhões. Vendendo: liga com divisão e elenco abaixo de 15, a diretoria perde 1250/1250/500 (+6/+0x14/+0x16, piso 1 em
/// +6) e veta 0x200; senão o pedido (0x4d1600, no máximo o valor pelo clube do contrato) acima do limite e o valor pelo
/// clube atual não abaixo do `valor`: pedido > 2 × valor com +0x3d do contrato > 25, ou pedido > 5 milhões com +0x3d > 0 e
/// > 2,5 × valor, perdem 1250/1250/500; > 6 × valor, 500/500/250; veto 0x800. Comprando: valor acima de 1,2 × a verba
/// (0x598800) veta 0x800; acima do limite, o menor valor (pelo clube e pelo do contrato) + 0,65 × limite + a faixa de
/// riqueza (0x5986b0: 1,25 milhão, 650 mil ou 65 mil) truncado abaixo de 0,275 × valor perde 250/500/500 e veta 0x800.
/// Senão 0.
unsafe extern "thiscall" fn check_board_transfer_veto(this: usize, j: usize, clube: usize, valor: i32, comprando: u32) -> u32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let semear: unsafe extern "C" fn(u32) = core::mem::transmute(0x9469fausize);
    let relogio: unsafe extern "C" fn(*mut i32) -> i32 = core::mem::transmute(0x946ebbusize);
    let nota: unsafe extern "C" fn(usize, u32) -> i8 = core::mem::transmute(0x544410usize);
    let aposentadoria: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7a7ed0usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x62ed70usize);
    let recaida: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62ece0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    CPY12_SEMEOU = false;
    if comprando as u8 != 0 {
        let l = lesao(cpy12_d(0xb63d28), j);
        if l != 0 && cpy12_w(l + 0x28) > 0x14 {
            let mut semeou = false;
            for k in 0..3usize {
                let f = cpy12_d(clube + 0x1cf + k * 4);
                if f == 0 || cpy12_d(f + 0x69) == 0 {
                    continue;
                }
                semear((cpy12_d(j) as u32).wrapping_add(1).wrapping_mul((cpy12_d(f) as u32).wrapping_add(1)));
                semeou = true;
                CPY12_SEMEOU = true;
                crate::comparacao::ramo("cpy_12 fisioterapeuta avalia");
                let np = cpy12_d(f + 0x69);
                // a original empurra o registrador inteiro com o byte +0x1e no baixo; a 0x544410 só lê o byte
                let n = || nota(np, cpy12_b(np + 0x1e) as u32) as i32;
                if aposentadoria(cpy12_d(0xb7434c), j, 1) != 0 {
                    if sorteio(n()) != 0 {
                        return 0x2000;
                    }
                    if aposentadoria(cpy12_d(0xb7434c), j, 0) != 0 {
                        return 0x2000;
                    }
                }
                let r = sorteio(200); // cpy12: o sorteio da gravidade
                if cpy12_w(l + 0x28) as i32 > r.wrapping_add(0x19) {
                    let a = sorteio((0x29 - n()) * 5);
                    let b = sorteio(cpy12_w(l + 0x28) as i32);
                    if a < b {
                        return 0x2000;
                    }
                }
                if recaida(cpy12_d(0xb63d28), j) != 0 {
                    let c = n();
                    let rr = recaida(cpy12_d(0xb63d28), j) as i8 as i32;
                    let a = sorteio(c + rr);
                    let b = sorteio(10);
                    if a > b {
                        return 0x2000;
                    }
                }
                let c = n();
                if sorteio(match_pl_ftol(game_pow(c as f64, read_f64_at(0x968930)) + read_f64_at(0x9688a8))) == 0 {
                    return 0x2000;
                }
                let a = sorteio(n());
                let b = sorteio(200);
                if a > b {
                    let a = sorteio(cpy12_b(cpy12_d(j + 0x61) + 0x2d) as i8 as i32);
                    let b = sorteio(200);
                    if a > b {
                        return 0x2000;
                    }
                }
            }
            if semeou {
                let mut t = 0i32;
                semear(relogio(&mut t) as u32);
            }
        }
    }
    let c = contrato(cpy12_d(0xae1050), j);
    if c == 0 {
        return 0;
    }
    let tipo = cpy12_b(c + 0x35) & 0x3f;
    if tipo == 1 || tipo == 3 || cpy12_d(j + 0x39) == 0 || humano(clube, 1, 1, 0, 0) == 0 {
        return 0;
    }
    let x = cpy12_w(clube + 0x80) as f64 * read_f64_at(0x968918);
    let mut limit = match_pl_ftol(x * x);
    if limit < 0x249f0 {
        limit = 0x249f0;
    } else if limit > 0x1e8480 {
        limit = 0x1e8480;
    }
    if comprando as u8 == 0 {
        let contar: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
        let nacao = cpy12_d(clube + 0x53);
        let divisao = cpy12_d(clube + 0x57);
        if nacao != 0 && cpy12_b(nacao + 0x11c) != 0 && divisao != 0 && cpy12_d(divisao) != cpy12_d(0x9cf764) && (contar(clube, 0, 0) as i8) < 0xf {
            add_board_confidence(this, clube, 6, -0x4e2);
            add_board_confidence(this, clube, 0x14, -0x4e2);
            add_board_confidence(this, clube, 0x16, -500);
            clamp_confidence_floor(this, clube);
            return 0x200;
        }
        let preco: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
        let mut pedido = preco(cpy12_d(0xae1050), cpy12_d(j + 0x39), clube, j, 0, 0, 1);
        let pelo_contrato = player_value(j, contract_club_cached(j), 1);
        if pedido > pelo_contrato {
            pedido = pelo_contrato;
        }
        let pelo_atual = player_value(j, cpy12_d(j + 0x39), 0);
        if pedido <= limit || pelo_atual < valor {
            return 0;
        }
        let (p, v) = (pedido as f64, valor as f64);
        let mais = cpy12_b(c + 0x3d) as i8;
        crate::comparacao::ramo("cpy_12 diretoria vendendo");
        if p > v + v && mais > 0x19 {
            add_board_confidence(this, clube, 6, -0x4e2);
            add_board_confidence(this, clube, 0x14, -0x4e2);
        } else if pedido > 0x4c4b40 && mais > 0 && p > v * read_f64_at(0x968e18) {
            add_board_confidence(this, clube, 6, -0x4e2);
            add_board_confidence(this, clube, 0x14, -0x4e2);
        } else if p > v * read_f64_at(0x969000) {
            add_board_confidence(this, clube, 6, -500);
            add_board_confidence(this, clube, 0x14, -500);
            add_board_confidence(this, clube, 0x16, -0xfa);
            return clamp_confidence_floor(this, clube);
        } else {
            return 0;
        }
        add_board_confidence(this, clube, 0x16, -500);
        return clamp_confidence_floor(this, clube);
    }
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let verba: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) -> i64 = core::mem::transmute(0x598800usize);
    let orcamento = verba(financas(cpy12_d(0xae2a38), clube), j, 0, 0, 1, 0);
    if valor as i64 > orcamento.wrapping_mul(12) / 10 { // cpy12: a verba da diretoria
        crate::comparacao::ramo("cpy_12 diretoria sem verba");
        return 0x800;
    }
    if valor <= limit {
        return 0;
    }
    let pelo_clube = player_value(j, clube, 0);
    let pelo_contrato = player_value(j, contract_club_cached(j), 1);
    let menor = if pelo_clube > pelo_contrato { pelo_contrato } else { pelo_clube };
    let fin = financas(cpy12_d(0xae2a38), clube);
    if fin == 0 {
        return 0;
    }
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
    let k = match band(fin) {
        2 => read_f64_at(0x96d1f8),
        1 => read_f64_at(0x96d1f0),
        _ => read_f64_at(0x968ea0),
    };
    let teto = match_pl_ftol(limit as f64 * read_f64_at(0x96a5d0) + menor as f64 + k);
    if !(valor as f64 * read_f64_at(0x96d1e8) > teto as f64) {
        return 0;
    }
    crate::comparacao::ramo("cpy_12 diretoria acha caro");
    add_board_confidence(this, clube, 6, -0xfa);
    add_board_confidence(this, clube, 0x14, -500);
    add_board_confidence(this, clube, 0x16, -500);
    clamp_confidence_floor(this, clube)
}

/// Comparador: lado a lado a partir do mesmo estado do registro da diretoria do clube, do contrato guardado em
/// [0xae1080] e do sorteio; a devolução e tudo isso têm de bater (o rand da CRT, só sem a semente do relógio).
unsafe extern "thiscall" fn comparar_check_board_transfer_veto(this: usize, j: usize, clube: usize, valor: i32, comprando: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, i32, u32) -> u32>(%%I0%%);
    let regioes = [(cpy12_diretoria(this, clube), 0x49), (0xae1080, 4), (0xde1f5c, 8), (0xad6c30, 4)];
    let (r, n, _, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, j, clube, valor, comprando),
        || crate::aleatorio::sem_contar(|| check_board_transfer_veto(this, j, clube, valor, comprando)));
    let partes = if CPY12_SEMEOU { 3 } else { 4 };
    let igual = (0..partes).all(|i| mo.0[i] == mr.0[i]);
    PLACAR.registrar(igual && r == n, || format!("check_board_transfer_veto(clube {:x}, valor {}, comprando {}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, valor, comprando as u8, r, n, mo, mr));
    r
}

/// { 0x6ac700 } (thiscall, ret 0xc, manager_manager.cpp, quadro SEH da notícia local): `MANAGER_MANAGER::
/// post_board_ultimatum_news(clube, adversário, competição)`, pelo ultimato da diretoria (0x6ac560). Clube com divisão
/// (+0x57), técnico (+0xcf) e a competição: a notícia 0x178d com o técnico (o id e as três chaves), o clube, o adversário,
/// a competição e um sorteio(5); clube de reputação (+0x80) abaixo de 5750, ao técnico do clube (0x7861e0); senão, por
/// sorteio(3), aos técnicos da divisão (0x7862d0) ou aos do país (+0x53, 0x7863f0).
unsafe extern "thiscall" fn post_board_ultimatum_news(_this: usize, clube: usize, adversario: usize, comp: usize) {
    if clube == 0 || cpy12_d(clube + 0x57) == 0 || cpy12_d(clube + 0xcf) == 0 || comp == 0 {
        return;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let ao_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let a_divisao: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7862d0usize);
    let ao_pais: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7863f0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0x178d, 0);
    let t = cpy12_d(clube + 0xcf);
    gravar(nt, 0, cpy12_d(t) as u32);
    gravar(nt, 1, cpy12_d(cpy12_d(t + 4) + 0x33) as u32);
    gravar(nt, 2, cpy12_d(cpy12_d(t + 8) + 0x33) as u32);
    gravar(nt, 3, cpy12_d(cpy12_d(t + 0xc) + 0x33) as u32);
    gravar(nt, 4, cpy12_d(clube) as u32);
    gravar(nt, 5, cpy12_d(adversario) as u32);
    gravar(nt, 6, cpy12_d(comp) as u32);
    crate::comparacao::ramo("cpy_12 noticia do ultimato");
    gravar(nt, 7, sorteio(5) as u32); // cpy12: o sorteio do ultimato
    if cpy12_w(clube + 0x80) < 0x1676 {
        ao_clube(cpy12_d(0xb7431c), nt, clube);
    } else if sorteio(3) != 0 {
        a_divisao(cpy12_d(0xb7431c), nt, cpy12_d(clube + 0x57));
    } else {
        ao_pais(cpy12_d(0xb7431c), nt, cpy12_d(clube + 0x53));
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca) e sorteia: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_post_board_ultimatum_news(this: usize, clube: usize, adversario: usize, comp: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I1%%)(this, clube, adversario, comp)
}

/// { 0x6ac880 } (stdcall, ret 0xc, manager_manager.cpp): `news_board_ultimatum_text(notícia, corpo, saída)`: lê os
/// campos 0 a 7 da notícia (0x785830: o técnico, as três chaves, o clube, o adversário, a competição); o nome do técnico
/// (0x53dba0, 500, estilo 1). Corpo: o texto 0xa12dac em 0xde1f64 (0x66f4e0) com os gêneros (+0x37 dos clubes, +0x52 da
/// competição), os nomes (+4 dos clubes, +0x38 da competição) e o do técnico, na `saída` (0x4ae660), e as ligações
/// (0x4ae8a0): o técnico (0x7da, se os nomes conferem pela 0x53e990), o clube e o adversário (0x7d5). Título: 0xa12d64
/// com o gênero (+0x52) e o nome curto (+0x38) do clube.
unsafe extern "stdcall" fn news_board_ultimatum_text(noticia: usize, corpo: u32, saida: usize) {
    let campo: unsafe extern "thiscall" fn(usize, i8) -> u32 = core::mem::transmute(0x785830usize);
    let nome_da_pessoa: unsafe extern "C" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53dba0usize);
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x53e990usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let por_texto: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4ae660usize);
    let ligar: unsafe extern "thiscall" fn(usize, usize, u32, usize) = core::mem::transmute(0x4ae8a0usize);
    crate::comparacao::ramo("cpy_12 texto do ultimato");
    let v: [u32; 7] = core::array::from_fn(|k| campo(noticia, k as i8));
    campo(noticia, 7);
    let pessoa = cpy12_d(0xae23bc).wrapping_add((v[0] as usize).wrapping_mul(0x6e));
    let clube = cpy12_clube(v[4] as usize);
    let adversario = cpy12_clube(v[5] as usize);
    let comp = cpy12_d(0xae23d0).wrapping_add((v[6] as usize).wrapping_mul(0x6b));
    let mut nome = [0u8; 0x200];
    let buf = nome.as_mut_ptr() as usize;
    nome_da_pessoa(pessoa, buf, 0x1f4, 1);
    if corpo != 0 {
        let gc = cpy12_b(comp + 0x52) as i8 as i32;
        let ga = cpy12_b(adversario + 0x37) as i8 as i32;
        let gl = cpy12_b(clube + 0x37) as i8 as i32;
        formatar(0xde1f64, 0xa12dac, gl, gl, ga, ga, gc, gc, clube + 4, buf, adversario + 4, comp + 0x38);
        por_texto(saida, 0xde1f64);
        if confere(pessoa, v[1], v[2], v[3], 0) != 0 {
            ligar(saida, buf, 0x7da, pessoa);
        }
        ligar(saida, clube + 4, 0x7d5, clube);
        ligar(saida, adversario + 4, 0x7d5, adversario);
    } else {
        let g = cpy12_b(clube + 0x52) as i8 as i32;
        formatar(0xde1f64, 0xa12d64, g, g, clube + 0x38); // "... manager given ultimatum by board"
        por_texto(saida, 0xde1f64);
    }
}

/// Comparador: o texto vai para o objeto de saída (0x4ae660/0x4ae8a0 acrescentam e alocam): roda só a original.
unsafe extern "stdcall" fn comparar_news_board_ultimatum_text(noticia: usize, corpo: u32, saida: usize) {
    // ponytail: o objeto de texto não pode receber o texto duas vezes; só a original roda aqui
    original_fn::<unsafe extern "stdcall" fn(usize, u32, usize)>(%%I2%%)(noticia, corpo, saida);
}
