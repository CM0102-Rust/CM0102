// ---------- centesima quinquagesima leva: a escalacao automatica (0x6dbc50) ----------

/// A caixa de erro "v%s %s.%s %d" de match_man.cpp na linha `linha` (o jogo segue).
unsafe fn aviso_de_match_man(linha: u32) {
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let juntar: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0xa15da4, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let s = juntar(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, s);
    caixa(0x9870e0, 0xae24d0, 0);
    *(0xb67a34 as *mut u32) = 0;
}

/// { 0x6dbc50 } (thiscall, ret 8, match_man.cpp, ~7,7 KB): a escalacao automatica da comissao `t`
/// (+0x12 motor, +0x18 lado, +0x19 humano, +0x1a peso do jogo, +0x1b tecnico, +0x1f ficha, +0x27
/// linhas, +0x2b tatica, +0x2f clube, +0x37 partida) contra `outro`; `modo` 1 respeita as vagas
/// marcadas na tatica do clube (0x8972c0), 2 refaz tudo. Fora do humano, com clube de reputacao
/// (+0x80 >= 6000) e forca (0x53c590) > 30, mede o jogo contra o proximo da lista (0x5acbe0) para
/// poupar (+0x1a 0/1/2, com as mascaras de 15 vagas). Filtra os 50 das linhas (estado 0x62c400,
/// suspensao 0x55d270, ja jogou 0x8cf5e0, impedido 0x7757d0, emprestado 0x4cf6d0/0x4cf5a0), escala
/// (0x6e2000), escolhe a formacao (0x6e0230), as tres referencias (+6/+0xa/+0xe pela ficha +0xd
/// + +0xb), o rodizio (0x8cf880/0x6e8910), poe nas vagas (0x6e0d00, 0x896a90), o capitao (0x897760),
/// as instrucoes (0x6ddab0) e as ordens gerais pela postura (0x6df460, 0x5b50d0). Devolve quantos
/// escalou. Os locais vivem num quadro com os deslocamentos da pilha original (zerado, como a
/// original determinizada): os vetores se sobrepoem como la (o curto da 1a vaga extra, as vagas
/// marcadas alem de 20).
unsafe extern "thiscall" fn escalar_automatico(t: usize, outro: usize, modo: u32) -> i8 {
    let mut quadro = [0u8; 0x750];
    let f = quadro.as_mut_ptr() as usize;
    let b = |o: usize| (f + o) as *mut u8;
    let rd = |o: usize| core::ptr::read_unaligned((f + o) as *const u32);
    let wd = |o: usize, v: u32| core::ptr::write_unaligned((f + o) as *mut u32, v);
    let rw = |o: usize| core::ptr::read_unaligned((f + o) as *const i16);
    let ww = |o: usize, v: i16| core::ptr::write_unaligned((f + o) as *mut i16, v);
    let u = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const usize);
    let h16 = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const i16);
    let tb = |o: usize| (t + o) as *mut u8;

    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let forca: unsafe extern "C" fn(usize, u32, u32) -> u8 = core::mem::transmute(0x53c590usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x5acbe0usize);
    let rival_de: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b02e0usize);
    let na_competicao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x48f150usize);
    let conta: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let estado: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let ja_jogou: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x8cf5e0usize);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x7757d0usize);
    let emprestado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x4cf6d0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let melhor: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x6e1940usize);
    let escalar: Escalar = core::mem::transmute(0x6e2000usize);
    let consultar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
    let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
    let guardar_instrucoes: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x896fe0usize);
    let vaga_marcada: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u8 = core::mem::transmute(0x8972c0usize);
    let marcar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x896c60usize);
    let cedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let formacao: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u8 = core::mem::transmute(0x6e0230usize);
    let casa: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(0x5b2ae0usize);
    let marcas: unsafe extern "thiscall" fn(usize, u32) -> u64 = core::mem::transmute(0x5b5270usize);
    let marca_b: unsafe extern "thiscall" fn(usize, u32) -> u8 = core::mem::transmute(0x5b57c0usize);
    let rodizio: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32, usize) -> u32 = core::mem::transmute(0x8cf880usize);
    #[allow(clippy::type_complexity)]
    let reavaliar: unsafe extern "C" fn(usize, usize, usize, u32, u32, u32, u32, usize, usize, usize, usize, usize, usize, usize, usize, u32) = core::mem::transmute(0x6e8910usize);
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let free: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let da_tatica: unsafe extern "thiscall" fn(usize, usize, u32, i32) -> usize = core::mem::transmute(0x8968f0usize);
    let colocar: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> u8 = core::mem::transmute(0x6e0d00usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(0x896a90usize);
    let capitao: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x897760usize);
    let instrucoes: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x6ddab0usize);
    let postura: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x6df460usize);
    let geral: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x5b50d0usize);

    let clube = u(t, 0x2f);
    let jogo = u(t, 0x37);
    let m = u(t, 0x12);
    let lado = byte(t, 0x18) as isize;
    let peso_do_lado = || *(((m as isize) + 0x1ad + lado) as *const i8);
    let competicao = |id: i32| *((u(0xadadfc, 0) as isize + 4 * id as isize) as *const usize);
    let tatica = *(0xde1d08 as *const usize);
    let gerente = *(0xb63c98 as *const usize);
    let mut modo = modo;

    lista_nova(f + 0x80);
    data_padrao(f + 0x38);
    mais_dias(0xae2c90, f + 0xac, 2);
    if outro == 0 {
        erro_de_match_man(0x174);
    }
    *b(0x13) = (*((u(m, 0x4792) + 0x4d) as *const u8) & 0xf).wrapping_add(0xb);
    let forte = forca(clube, 0, 1) as i8;
    *b(0x12) = forte as u8;
    *tb(0x16) = 0;

    // fora do humano: pesa o jogo de hoje contra o proximo da lista (poupar para o mais importante?)
    if byte(t, 0x19) == 0 && h16(clube, 0x80) >= 0x1770 && *(jogo as *const i32) != -2 && humano(clube, 0, 1, 0, 0) == 0 && selecao(clube) == 0 && forte > 0x1e {
        mais_dias(0xae2c90, f + 0xa4, 1);
        mais_dias(0xae2c90, f + 0x40, 7);
        let r = jogos(u(0xae2a58, 0), rd(0xa4), rd(0xa6), rd(0x40), rd(0x42), *(clube as *const u32), f + 0x80, u32::MAX, 0xffff_fffe, 1);
        if r as u16 != 0 {
            // 333 x a reputacao da competicao (.69; a da [0x9cf6f8] na tabela [0xae23d0]) + reputacao/3
            let peso = |comp: usize, rival: usize| -> i32 {
                let c = if *(comp as *const u32) == *(0x9cf6f8 as *const u32) { u(0xae23d0, 0) + u(0x9cf6f0, 0).wrapping_mul(0x6b) } else { comp };
                (h16(c, 0x69) as i32 * 333).wrapping_add(h16(rival, 0x80) as i32 / 3)
            };
            let comp = u(jogo, 0x14);
            let id = *(comp as *const u32);
            let mut l14 = peso(comp, outro);
            if id != *(0x9cf6f8 as *const u32) {
                if byte(jogo, 0x3a) == 8 {
                    let k = *((jogo + 0x30) as *const u16);
                    if byte(t, 0x18) == 1 || (0x78..=0x7c).contains(&k) || (0x82..=0x87).contains(&k) || (0x96..=0x9b).contains(&k) {
                        *tb(0x16) = 1;
                    }
                } else if id == *(0x9cf6e8 as *const u32) && byte(jogo, 0x42) == -1 {
                    *tb(0x16) = 1;
                }
            }
            let mut fator = peso_do_lado() as f64 * f64_em(0x96d438) + f64_em(0x967890);
            if fator > f64_em(0x968dc0) {
                fator = f64_em(0x968dc0);
            }
            let mut esi = ftol(l14 as f64 * fator);
            l14 = esi;
            if rival_de(outro, clube) != 0 {
                esi = ftol(l14 as f64 * f64_em(0x968f98));
                l14 = esi;
                *tb(0x16) = 1;
            } else if outro == u(clube, 0xb3) || outro == u(clube, 0xb7) || outro == u(clube, 0xbb) {
                esi = ftol(l14 as f64 * f64_em(0x968940));
                l14 = esi;
            }
            if na_competicao(competicao(*(jogo as *const i32)), clube) == 0 {
                *tb(0x16) = 0;
                let tec = u(t, 0x1b);
                l14 = if tec != 0 { ftol(byte(tec, 0x5c) as f64 * f64_em(0x968a90) * l14 as f64) } else { esi / 2 };
            }
            wd(0x14, l14 as u32);
            wd(0x8a, rd(0x82));
            let x = proximo(f + 0x80);
            wd(0x20, x as u32);
            if x != 0 && *(x as *const i32) != -2 {
                let d = data_nova(f + 0x24, *((x + 0x2a) as *const u16) as u32, *((x + 0x28) as *const u16) as u32);
                copiar(f + 0x38, f + 0x18, d);
                let rival = if clube == u(x, 0x1c) { u(x, 0x20) } else { u(x, 0x1c) };
                let mut l18 = peso(u(x, 0x14), rival);
                let mut esi = l18;
                if rival_de(rival, clube) != 0 {
                    esi = ftol(l18 as f64 * f64_em(0x968940));
                    l18 = esi;
                } else if rival == u(clube, 0xb3) || rival == u(clube, 0xb7) || rival == u(clube, 0xbb) {
                    esi = ftol(l18 as f64 * f64_em(0x969588));
                    l18 = esi;
                }
                if u(x, 0x18) != u(clube, 0x69) {
                    let c = if u(clube, 0x53) != u(rival, 0x53) { f64_em(0x968978) } else { f64_em(0x9689c0) };
                    esi = ftol(l18 as f64 * c);
                    l18 = esi;
                }
                if na_competicao(competicao(*(x as *const i32)), clube) == 0 {
                    l18 = esi / 3;
                } else if conta(*(jogo as *const i32)) != 0 && (byte(x, 0x3a) == 8 || byte(x, 0x3a) == 7) && *(x as *const i32) > -1 {
                    let c = competicao(*(x as *const i32));
                    if c != 0 && u(c, 8) != 0 {
                        let r = busca(u(c, 8), clube);
                        let k = if r != 0 { *((r + 0x72) as *const u8) } else { 0 };
                        if k == 0 {
                            l18 = esi / 2;
                        } else if k > 6 {
                            l18 = esi.wrapping_mul(6) / k as i32;
                        }
                    }
                }
                wd(0x18, l18 as u32);
                // o proximo jogo daqui a 1..7 dias pesa mais (o de 4 dias, nada)
                for (k, c) in [Some(0x96d4c0usize), Some(0x96d4b8), Some(0x96d4b0), None, Some(0x96d4a8), Some(0x96d4a0), Some(0x96d498)].iter().enumerate() {
                    let p = mais_dias(0xae2c90, f + 0x48, k as u32 + 1);
                    wd(0x24, *(p as *const u32));
                    wd(0x28, *((p + 4) as *const u32));
                    if rw(0x26) == rw(0x3a) && rw(0x24) == rw(0x38) {
                        if let Some(c) = c {
                            l18 = ftol(l18 as f64 * f64_em(*c));
                            wd(0x18, l18 as u32);
                        }
                        break;
                    }
                }
                let (a, b2) = (l18 as f64, l14 as f64);
                if a > b2 + b2 && forte > 0x28 && l14 < 0x1b58 {
                    *tb(0x1a) = 0;
                } else if a > b2 * f64_em(0x968dc0) && forte > 0x23 && l14 < 0x2134 {
                    *tb(0x1a) = 1;
                } else if a > b2 * f64_em(0x9689c0) && l14 < 0x2710 {
                    *tb(0x1a) = 2;
                }
            }
        }
    }

    // poupando: as mascaras das vagas e a primeira escala, com os que podem jogar
    let emprestado_fora = |p: usize| -> bool {
        // emprestado (0x4cf6d0) a outro clube que nao o parceiro (0x540a50)?
        let dono = |p: usize| {
            let c = contrato(u(0xae1050, 0), p);
            *(0xae1080 as *mut usize) = c;
            if c != 0 { u(0xae23b4, 0).wrapping_add(u(c, 4).wrapping_mul(0x245)) } else { 0 }
        };
        if emprestado(u(0xae1050, 0), p) == 0 {
            return false;
        }
        if dono(p) == clube {
            return true;
        }
        dono(p) == parceiro(clube, 0, 1)
    };
    if byte(t, 0x1a) != 3 && humano(clube, 0, 1, 0, 0) == 0 && selecao(clube) == 0 {
        match byte(t, 0x1a) {
            0 => {
                for (o, v) in [(0x50, 1), (0x52, 0x804), (0x54, 0x84), (0x56, 0x204), (0x58, 0x204), (0x5a, 0x204), (0x5c, 0x810), (0x5e, 0x90), (0x60, 0x210), (0x62, 0x210), (0x64, 0x210), (0x66, 0x40), (0x68, 0x40), (0x6a, 0x40), (0x6c, 0x40)] {
                    ww(o, v);
                }
                *b(0x30) = 0xe;
            }
            1 => {
                for (o, v) in [(0x50, 4), (0x52, 4), (0x54, 0x204), (0x56, 0x10), (0x58, 0x10), (0x5a, 0x210), (0x5c, 0x40), (0x5e, 0x40)] {
                    ww(o, v);
                }
                *b(0x30) = 7;
            }
            2 => {
                for (o, v) in [(0x50, 4), (0x52, 0x10), (0x54, 0x40)] {
                    ww(o, v);
                }
                *b(0x30) = 2;
            }
            _ => {}
        }
        let x = rd(0x20) as usize;
        *b(0x18) = 0;
        for k in 0..50usize {
            let p = u(u(t, 0x27), 4 * k);
            if p == 0 || u(p, 0x61) == 0 {
                continue;
            }
            if byte(p, 0x18) < 0x10 {
                let c = u(jogo, 0x14);
                if c != 0 && h16(c, 0x69) > 10 {
                    continue;
                }
            }
            if estado(u(0xb63d28, 0), p) == 2 {
                continue;
            }
            if byte(t, 0x19) != 0 {
                if *(jogo as *const i32) != -2 && suspenso(u(0xae28f0, 0), p, u(jogo, 0x14), 1, 0) != 0 {
                    continue;
                }
            } else if x != 0 {
                if u(jogo, 0x14) != u(x, 0x14) && suspenso(u(0xae28f0, 0), p, u(x, 0x14), 1, 0) != 0 {
                    continue;
                }
                if ja_jogou(u(0xde1f54, 0), p, x) != 0 || impedido(u(0xb74318, 0), p, f + 0x38, clube, 0, 0) != 0 {
                    continue;
                }
            }
            let dia = if *(jogo as *const u32) == *(0x9cf684 as *const u32) { f + 0xac } else { 0xae2c90 };
            if impedido(u(0xb74318, 0), p, dia, clube, 0, 0) != 0 || emprestado_fora(p) {
                continue;
            }
            let c = *b(0x18) as usize;
            wd(0x278 + 4 * c, p as u32);
            *b(0x18) = (c + 1) as u8;
        }
        if *b(0x18) != 0 {
            let r = melhor(t, f + 0x278, rd(0x18));
            *((t + 0x3b) as *mut usize) = r;
            if r == 0 {
                erro_de_match_man(0x302);
            }
        }
        let cf = u(clube, 0xcf);
        let a7 = if byte(t, 0x19) == 0 {
            0xa520c2
        } else if humano(clube, 1, 1, 0, 0) != 0 && cf != 0 && consultar(gerente, cf) != 0 && opcao(gerente, 3, u(clube, 0xcf)) != 0 {
            0xc520c2
        } else {
            0x8520c2
        };
        escalar(f + 0x278, rd(0x18), f + 0x50, rd(0x30), 0, f + 0x56c, a7, clube, t, u(t, 0x1b) as u32, 0, 0, 0, 0, 0);
    }

    // os candidatos: quem pode jogar entra na lista; no modo 1 os das vagas marcadas so as ocupam
    let humano19 = byte(t, 0x19) as i32 as u32;
    let bl = modo as u8;
    if bl == 2 {
        guardar_instrucoes(tatica, clube, humano19, 0, 0);
    }
    let vaga = |p: usize| vaga_marcada(tatica, clube, p, humano19) as i8;
    let marcar_se = |p: usize| {
        if bl == 1 && vaga(p) > -1 {
            marcar(tatica, clube, p, humano19, 1);
        }
    };
    *b(0x18) = 0;
    for k in 0..50usize {
        wd(0x20, 4 * k as u32);
        let p = u(u(t, 0x27), 4 * k);
        if p == 0 || u(p, 0x61) == 0 {
            continue;
        }
        if byte(t, 0x19) == 0 && byte(p, 0x18) < 0x10 {
            let c = u(jogo, 0x14);
            if c != 0 && h16(c, 0x69) > 10 {
                continue;
            }
        }
        let m5e = |p: usize| *((p + 0x5e) as *const u8);
        let segue = if byte(t, 0x1a) == 3 {
            !(selecao(clube) != 0 && m5e(p) & 8 != 0)
        } else if humano(clube, 0, 1, 0, 0) != 0 {
            if selecao(clube) != 0 {
                m5e(p) & 8 != 0
            } else if byte(t, 0x19) != 0 && m5e(p) & 1 != 0 && vaga(p) == -1 {
                false
            } else {
                byte(t, 0x19) != 0 || m5e(p) & 2 == 0
            }
        } else if *(p as *const i32) >= *(0xae2364 as *const i32) {
            true
        } else if selecao(clube) != 0 {
            m5e(p) & 4 == 0
        } else {
            let mut fora = false;
            let mut j = 0i8;
            while j < *b(0x30) as i8 {
                if rd(0x56c + 4 * j as usize) as usize == p {
                    fora = true;
                    break;
                }
                j += 1;
            }
            !fora
        };
        if !segue {
            continue;
        }
        if selecao(clube) == 0 && cedido(u(0xde1f54, 0), p, clube, 0) != 0 {
            marcar_se(p);
            continue;
        }
        if estado(u(0xb63d28, 0), p) == 2 || (byte(t, 0x19) != 0 && estado(u(0xb63d28, 0), p) != 0) {
            marcar_se(p);
            continue;
        }
        let fora = (*(jogo as *const i32) != -2 && suspenso(u(0xae28f0, 0), p, u(jogo, 0x14), 1, 0) != 0)
            || impedido(u(0xb74318, 0), p, 0xae2c90, clube, 0, 0) != 0
            || (*(jogo as *const i32) != -2 && ja_jogou(u(0xde1f54, 0), p, jogo) != 0)
            || emprestado_fora(p);
        if fora {
            marcar_se(p);
            continue;
        }
        if bl == 1 {
            let v = vaga(p);
            if v > -1 && v < *b(0x13) as i8 {
                *b(0x2e) = 1;
                *b(0x90 + v as usize) = 1;
                continue;
            }
        }
        let c = *b(0x18) as usize;
        wd(0x278 + 4 * c, p as u32);
        *b(0x18) = (c + 1) as u8;
    }
    wd(0x20, 0xc8);
    let mut bl = bl;
    if bl == 1 && *b(0x2e) == 0 {
        bl = 2;
        modo = (modo & !0xff) | 2;
    }

    // as marcas da escala
    let mut mascara: u32 = if selecao(clube) != 0 { 0x208a09c } else { 0xca0ca };
    if byte(t, 0x19) != 0 {
        mascara = (mascara | 0x20100) ^ 0x80000;
    } else {
        let dif = || h16(clube, 0x80) as i32 - h16(outro, 0x80) as i32;
        let a = byte(jogo, 0x3a);
        let mut feito = false;
        if a == 1 || a == 9 {
            let lim = sorteio(2000).wrapping_add(sorteio(-4000));
            if dif() > lim {
                mascara |= 0x100;
                feito = true;
            }
        }
        if !feito {
            let lim = sorteio(3000) + 3000;
            if dif() > lim {
                mascara |= 0x100;
                if selecao(clube) == 0 {
                    mascara |= 0x10000;
                }
            } else {
                let j0 = *(jogo as *const i32);
                if byte(t, 0x1a) != 3 || (j0 > -1 && competicao(j0) != 0 && na_competicao(competicao(j0), clube) == 0) {
                    mascara = (mascara | 0x100) ^ 0x80000;
                } else if selecao(clube) == 0 {
                    mascara |= 0x10000;
                }
            }
        }
    }
    if byte(t, 0x16) != 0 {
        mascara |= 0x800000;
    }
    wd(0x14, mascara);

    // as tres referencias (a maior ficha +0xd + +0xb, sem repetir)
    if *b(0x18) != 0 && bl == 2 {
        let r = melhor(t, f + 0x278, rd(0x18));
        *((t + 0x3b) as *mut usize) = r;
        if r == 0 {
            erro_de_match_man(0x42b);
        }
        if mascara & 0x80000 != 0 {
            for s in 0..3usize {
                ww(0x40 + 2 * s, 0xd8f0u16 as i16);
                ww(0x24 + 2 * s, -1);
                let mut c = 0i8;
                while c < *b(0x18) as i8 {
                    let p = rd(0x278 + 4 * c as usize) as usize;
                    let c16 = c as i16;
                    let pula = p == 0
                        || *(p as *const i32) >= *(0xae2364 as *const i32)
                        || u(p, 0x61) == 0
                        || (s == 1 && rw(0x24) == c16)
                        || (s == 2 && (rw(0x24) == c16 || rw(0x26) == c16));
                    if !pula {
                        let fi = u(p, 0x61);
                        let v = h16(fi, 0xd).wrapping_add(h16(fi, 0xb));
                        if v > rw(0x40 + 2 * s) {
                            ww(0x24 + 2 * s, c16);
                            ww(0x40 + 2 * s, v);
                        }
                    }
                    c += 1;
                }
                let a = rw(0x24 + 2 * s);
                if a > -1 {
                    core::ptr::write_unaligned((t + 6 + 4 * s) as *mut u32, rd(0x278 + 4 * a as usize));
                }
            }
        }
    }
    formacao(t, f + 0x278, rd(0x18), rd(0x14), modo);

    // as vagas: as 11 da formacao (casas, marcas) e as extras sorteadas
    let n = *b(0x13) as i8 as i32;
    wd(0x30, n as u32);
    *b(0x12) = 0;
    *b(0x20) = 0;
    *b(0x40) = 0;
    let tat = || u(t, 0x2b);
    let extra = |o: usize, k: usize, primeiro: i32| {
        if k == 12 && *b(0x2f) == 0 && (*b(0x13) as i8 > 14 || sorteio(primeiro) == 0) {
            *b(0x2f) = 1;
            true
        } else {
            let v = rw(o);
            ww(o, if v < 3 { 4 } else if v >= 6 { 0x40 } else { 0x10 });
            false
        }
    };
    for k in 0..n.max(0) as usize {
        if modo as u8 == 1 {
            if *b(0x90 + k) != 0 {
                continue;
            }
            let c = *b(0x12) as i8 as usize;
            if k <= 10 {
                ww(0xe8 + 2 * c, casa(tat(), k as u32, 0) as i16);
                ww(0x214 + 2 * c, casa(tat(), k as u32, 1) as i16);
                let q = marcas(tat(), k as u32);
                wd(0x5bc + 8 * c, q as u32);
                wd(0x5c0 + 8 * c, (q >> 32) as u32);
                *b(0xb4 + c) = marca_b(tat(), k as u32);
                *b(0x20) = (*b(0x20)).wrapping_add(1);
            } else {
                wd(0x5bc + 8 * c, 0);
                wd(0x5c0 + 8 * c, 0);
                *b(0xb4 + c) = 0;
                ww(0xe8 + 2 * c, sorteio(10) as i16);
                ww(0x214 + 2 * c, 0);
                if extra(0xe8 + 2 * c, k, 0x14) {
                    ww(0xe8 + 2 * c, 1);
                }
                *b(0x40) = (*b(0x40)).wrapping_add(1);
            }
            *b(0x12) = (*b(0x12)).wrapping_add(1);
        } else if k <= 10 {
            *b(0x12) = (*b(0x12)).wrapping_add(1);
            *b(0x20) = (*b(0x20)).wrapping_add(1);
            ww(0xe8 + 2 * k, casa(tat(), k as u32, 0) as i16);
            ww(0x214 + 2 * k, casa(tat(), k as u32, 1) as i16);
            let q = marcas(tat(), k as u32);
            wd(0x5bc + 8 * k, q as u32);
            wd(0x5c0 + 8 * k, (q >> 32) as u32);
            *b(0xb4 + k) = marca_b(tat(), k as u32);
        } else {
            *b(0x40) = (*b(0x40)).wrapping_add(1);
            *b(0x12) = (*b(0x12)).wrapping_add(1);
            wd(0x5bc + 8 * k, 0);
            wd(0x5c0 + 8 * k, 0);
            *b(0xb4 + k) = 0;
            ww(0xe8 + 2 * k, sorteio(10) as i16);
            // como na original: zera o curto da vaga seguinte (o desta fica com o que havia)
            ww(0x214 + 2 * (*b(0x12) as i8 as usize), 0);
            if extra(0xe8 + 2 * k, k, 10) {
                ww(0x100, 1);
            }
        }
    }
    let cf = u(clube, 0xcf);
    if humano(clube, 1, 1, 0, 0) != 0 && byte(t, 0x19) == 0 && cf != 0 && consultar(gerente, cf) != 0 && opcao(gerente, 3, u(clube, 0xcf)) != 0 {
        wd(0x14, rd(0x14) | 0x400000);
    }
    escalar(f + 0x278, rd(0x18), f + 0xe8, rd(0x20), rd(0x40), f + 0x14c, rd(0x14), clube, t, u(t, 0x1b) as u32, 0, f + 0x5bc, f + 0xb4, *((u(t, 0x2b) + 0x584) as *const u32), f + 0x214);
    let n = rd(0x30) as i32;
    if n - 1 > 0 {
        for i in 0..(n - 1) as usize {
            for j in i + 1..n as usize {
                let a = rd(0x14c + 4 * i);
                if a != 0 && a == rd(0x14c + 4 * j) {
                    aviso_de_match_man(0x4c9);
                }
            }
        }
        wd(0x20, n as u32);
    }

    // o rodizio (0x8cf880 aponta quem descansa; 0x6e8910 reavalia), no maximo duas voltas
    if *(jogo as *const i32) != -2 {
        *b(0x2e) = 0;
        let mut r = rodizio(u(0xde1f54, 0), clube, jogo, 0, f + 0x14c, 2, f + 0x78);
        while r != 0 && *b(0x2e) != 2 {
            let nb = *b(0x13);
            *b(0x2e) = (*b(0x2e)).wrapping_add(1);
            core::ptr::copy(b(0x14c), b(0x340), 44);
            let v = if nb as i8 > 0xb {
                let k = rd(0x30) - 0xb;
                let v = calloc(k, 4);
                core::ptr::copy(b(0x178), v as *mut u8, 4 * k as usize);
                v
            } else {
                0
            };
            wd(0x24, v as u32);
            *b(0x20) = 0;
            let w = if *b(0x18) as i8 > nb as i8 {
                let w = calloc(0x32, 4);
                for c in 0..(*b(0x18) as i8).max(0) as usize {
                    let p = rd(0x278 + 4 * c);
                    if p == 0 {
                        continue;
                    }
                    let dentro = nb as i8 > 0 && (0..rd(0x30) as usize).any(|j| rd(0x14c + 4 * j) == p);
                    if !dentro {
                        *((w + 4 * *b(0x20) as i8 as usize) as *mut u32) = p;
                        *b(0x20) = (*b(0x20)).wrapping_add(1);
                    }
                }
                w
            } else {
                0
            };
            // ponytail: o 6o argumento leva so o byte (n - 11); a original passa sobra de ebx nos altos (lidos como byte)
            reavaliar(clube, u(t, 0x1b), f + 0x78, rd(0x14), 0xb, nb.wrapping_sub(0xb) as u32, rd(0x20), f + 0x340, v, w, f + 0x14c, f + 0xe8, f + 0x214, f + 0x5bc, f + 0xb4, 0);
            free(v);
            if w != 0 {
                free(w);
            }
            r = rodizio(u(0xde1f54, 0), clube, jogo, 0, f + 0x14c, 2, f + 0x78);
        }
    }
    if modo as u8 == 1 {
        *b(0x12) = 0;
        *b(0x24) = 0;
        let nn = *b(0x13) as i8;
        if nn > 0 {
            for k in 0..nn as usize {
                let p = if *b(0x90 + k) != 0 {
                    da_tatica(tatica, clube, rd(0x24), byte(t, 0x19) as i32) as u32
                } else {
                    let c = *b(0x12) as i8 as usize;
                    *b(0x12) = (*b(0x12)).wrapping_add(1);
                    rd(0x14c + 4 * c)
                };
                wd(0x36c + 4 * k, p);
                *b(0x24) = (k + 1) as u8;
            }
            core::ptr::copy(b(0x36c), b(0x14c), 4 * rd(0x30) as usize);
        }
    }

    // nas vagas, o capitao, as instrucoes
    let cap = melhor(t, f + 0x14c, 0xb);
    let n = rd(0x30) as i32;
    for k in 0..n.max(0) as usize {
        colocar(t, k as u32, rd(0x14c + 4 * k) as usize, k as u32);
        let p = rd(0x14c + 4 * k) as usize;
        if p != 0 {
            gravar(tatica, clube, k as u32, p, humano19);
            *b(0x37) = (*b(0x37)).wrapping_add(1);
            if rd(0x14c + 4 * k) as usize == cap {
                capitao(tatica, clube, k as u32, humano19);
            }
        }
    }
    let n_final = n.max(0) as usize;
    instrucoes(t);
    let nome = if u(t, 0x2b) != 0 { u(t, 0x2b) } else { 0xa15934 };
    let nome = core::ffi::CStr::from_ptr(nome as *const core::ffi::c_char).to_bytes();
    let especial = nome != b"3-4-3" && nome != b"3-5-2 Attacking";

    // as ordens gerais pela postura (0x5b50d0)
    let g = |x: u32| geral(u(t, 0x2b), x);
    let a = |x: u32| {
        g(x);
        g(0x800);
        g(0x2000);
    };
    let bx = |x: u32| {
        g(x);
        g(0x2000);
    };
    let cx = || {
        g(0x800);
        g(0x2000);
    };
    let ficha = u(t, 0x1f);
    let lim = |base: i32| if peso_do_lado() > 0 { base + 3 } else { base };
    let disposicao = |i: isize| *(((m as isize) + 0x476a + 4 * i) as *const f32) as f64;
    let e = |o: usize| byte(ficha, o) as i32;
    match postura(t) as i8 {
        1 => {
            let d = disposicao(lado);
            let x = if ficha != 0 && !(d >= f64_em(0x968e18)) {
                if e(0xe) > 6 && especial { 0x10000 } else { 0x20000 }
            } else if especial || d > f64_em(0x9688b0) {
                0x10000
            } else {
                0x20000
            };
            a(x);
        }
        3 => {
            if ficha == 0 {
                g(0x8000);
                bx(0x1000);
            } else {
                g(if e(0xe) > 0x10 { 0x20000 } else { 0x8000 });
                if e(0x13) < lim(10) { bx(0x1000) } else { cx() }
            }
        }
        4 => {
            if ficha != 0 {
                g(if e(0xe) > 0x12 { 0x20000 } else { 0x8000 });
            } else {
                g(0x8000);
            }
            if ficha != 0 && e(0xe) > 0xc {
                if e(0x13) < lim(10) { bx(0x1000) } else { cx() }
            } else {
                g(0x800);
                g(0x4000);
            }
        }
        5 => {
            if ficha == 0 {
                g(0x20000);
                bx(0x1000);
            } else {
                g(if e(0xe) > 0x10 { 0x10000 } else if e(0xe) > 0xc { 0x20000 } else { 0x8000 });
                if e(0x13) < lim(10) { bx(0x1000) } else { cx() }
            }
        }
        _ => {
            if ficha == 0 {
                a(0x20000);
            } else if e(0xe) > lim(0xf) && especial {
                a(0x10000);
            } else if e(0xe) <= 5 && peso_do_lado() != 0 && !(disposicao(n_final as isize) > f64_em(0x967890)) {
                // como na original: le a disposicao no indice do total de vagas (o ebx do laco), nao do lado
                g(0x8000);
                if e(0x13) < lim(4) { bx(0x1000) } else { cx() }
            } else {
                a(0x20000);
            }
        }
    }
    destruir_lista(f + 0x80);
    *b(0x37) as i8
}

// ponytail: aloca a lista e os vetores do rodizio e chama 24 rotinas ainda originais; sem comparacao
// propria, validada pela carreira (Rust x original), como as da 145a, 148a e 149a.
unsafe extern "thiscall" fn comparar_escalar_automatico(t: usize, outro: usize, modo: u32) -> i8 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> i8>(331)(t, outro, modo)
}
