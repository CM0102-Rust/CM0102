// ---------- %%LEVA%%a leva: a Primera espanhola (a tabela, a TV, a divisão de baixo genérica e a situação nas estatísticas) e os registros da Série C2/A italiana pela situação na tabela, no play-off e no play-out (spa_first.cpp, ita_ser_c2a.cpp) ----------

unsafe fn cqk5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqk5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
unsafe fn cqk5_registro(g: usize) -> usize {
    cqk5_d(0xae23d0).wrapping_add((cqk5_d(g) as i32 as usize).wrapping_mul(0x6b))
}

/// { 0x84fc70 } (thiscall, ret 0, spa_first.cpp): `SpaFirst::create_table()`. +0x3e = 20 e a tabela (malloc 0x944e46
/// de 0x49c; sem memória: a caixa do fonte 0xa82844, linha 0x250, e 0). Antes de encher, os clubes da divisão de
/// baixo espanhola ([0x9cf6b0]) passam à divisão de baixo genérica (+0x57 = o registro de [0x9cf764]). Depois os
/// clubes da Primera ([0x9cf624]) na ordem do banco, cada um na linha seguinte (League::init_table_row 0x687430:
/// thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher (sem encher: 0). Cheia: o fundo de prêmios (+0x28 =
/// League::calc_prize_fund 0x682f70 pela cota de 250.000), +0xe1 = 1, a TV de 5.000.000 a cada clube (0x683010:
/// thiscall(liga, valor, 0), ret 8) e 1.
unsafe extern "thiscall" fn spa_first_create_table(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x14);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(0x49c);
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(0xa82844, 0x250);
        return 0;
    }
    // a divisão de baixo espanhola vira a genérica
    let mut i: i16 = 0;
    let mut campo = cqk5_d(0xae23b4) + 0x57;
    while (i as i32) < cqk5_d(0xae235c) as i32 {
        let comp = cqk5_d(campo);
        if comp != 0 && cqk5_d(comp) == cqk5_d(0x9cf6b0) {
            core::ptr::write_unaligned(campo as *mut usize, cqk5_registro(0x9cf764));
        }
        i = i.wrapping_add(1);
        campo = campo.wrapping_add(0x245);
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = cqk5_d(0xae23b4);
    while (i as i32) < cqk5_d(0xae235c) as i32 {
        let comp = cqk5_d(clube + 0x57);
        if comp != 0 && cqk5_d(comp) == cqk5_d(0x9cf624) {
            let linha = k; // cqk5: a linha k recebe o k-ésimo clube da Primera
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqk5_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    if k as i8 as i16 != cqk5_w(this + 0x3e) {
        return 0;
    }
    crate::comparacao::ramo("cqk_5 tabela da primera");
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
    let f = premio(this, 0x3d090); // cqk5: a cota da Primera
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1; // cqk5: o prêmio pela tabela no fim da liga
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(0x683010usize);
    pagar(this, 0x4c4b40, 0); // cqk5: a TV da Primera
    1
}

/// Comparador da 0x84fc70: a original aloca a tabela, muda a divisão dos clubes e paga a TV (com notícia): só ela. O
/// plano, feito antes: os clubes da divisão de baixo espanhola (que passam à genérica) e os da Primera na ordem do
/// banco (até 20). Depois: os primeiros com +0x57 = o registro de [0x9cf764]; +0x3e = 20, a devolução, as linhas com
/// os clubes da Primera na ordem e, cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
unsafe extern "thiscall" fn comparar_spa_first_create_table(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let (mut baixo, mut plano): (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    for i in 0..(cqk5_d(0xae235c) as i32).max(0) as usize {
        let clube = cqk5_d(0xae23b4) + i * 0x245;
        let comp = cqk5_d(clube + 0x57);
        if comp != 0 && cqk5_d(comp) == cqk5_d(0x9cf6b0) {
            baixo.push(clube);
        } else if comp != 0 && cqk5_d(comp) == cqk5_d(0x9cf624) && plano.len() < 0x14 {
            plano.push(clube);
        }
    }
    let r = o(this);
    let t = cqk5_d(this + 0xb1);
    let generica = cqk5_registro(0x9cf764);
    let mut igual = r == (plano.len() == 0x14) as u32 && cqk5_w(this + 0x3e) == 0x14 && t != 0 && baixo.iter().all(|&c| cqk5_d(c + 0x57) == generica);
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| cqk5_d(t + k * 0x3b)).collect();
        igual = achou == plano;
        if r == 1 {
            let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x682f70usize);
            igual = igual && *((this + 0xe1) as *const u8) == 1 && core::ptr::read_unaligned((this + 0x28) as *const i32) == premio(this, 0x3d090);
        }
    }
    PLACAR.registrar(igual, || format!("spa_first_create_table({:#x}) original={} baixo={} plano={:x?} tabela={:x?}", this, r, baixo.len(), plano, achou));
    r
}

/// O registro de staff ([0xdd7f00]).
unsafe fn cqk5_registros() -> usize {
    cqk5_d(0xdd7f00)
}

/// { 0x654180 } (thiscall, ret 0x18, ita_ser_c2a.cpp): `ItaSerC2a::update_staff_records_for_status(clube, situação,
/// fase, _, jogo, _)`, a virtual +0x44 (anunciada pela 0x686670). Devolve 0. Na liga (fase 0xff), pela situação: 5,
/// o título (Staff_Records::add_league_champions_record 0x868c50: thiscall(registros, clube, comp +4), ret 8); 0, o
/// acesso (0x869480: thiscall(registros, clube, comp, 0x64), ret 0xc); 1, a vaga no play-off (0x86bdd0:
/// thiscall(registros, clube, comp, 0xa0, 0x82, 0x28), ret 0x14); 3, o rebaixamento (0x86a1c0: thiscall(registros,
/// clube, comp), ret 8). No play-off de acesso (fase 0), na primeira linha do clube com situação 1, 2 ou 0xff: 1
/// (venceu), o acesso com peso 0x32 e +0x37 = 0; 2 ou 0xff (caiu), a eliminação (0x86c000: thiscall(registros, clube,
/// comp, fase do jogo +0x32, rodada +0x30, peso), ret 0x14; o peso = comp_util_scale_by_round 0x4bf850: cdecl(0x1e,
/// 0x32, rodada do jogo +0x34, 2)) e +0x37 = 0xfe. No play-out (fase 1): 1 (salvou-se), +0x37 = 0xfe; 2 ou 0xff,
/// o rebaixamento e +0x37 = 3.
unsafe extern "thiscall" fn spa_first_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, _a4: u32, jogo: usize, _a6: u32) -> u32 {
    let s = situacao as u8;
    let comp = cqk5_d(this + 4);
    let f = fase as u8;
    if f == 0xff {
        match s {
            5 => {
                let titulo: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x868c50usize);
                titulo(cqk5_registros(), clube, comp);
            }
            0 => {
                let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
                acesso(cqk5_registros(), clube, comp, 0x64);
            }
            1 => {
                crate::comparacao::ramo("cqk_5 vaga no play-off da c2a");
                let vaga: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86bdd0usize);
                vaga(cqk5_registros(), clube, comp, 0xa0, 0x82, 0x28);
            }
            3 => {
                crate::comparacao::ramo("cqk_5 rebaixamento na c2a");
                let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
                cair(cqk5_registros(), clube, comp); // cqk5: o rebaixamento pela tabela da C2/A
            }
            _ => {}
        }
        return 0;
    }
    if f > 1 {
        return 0;
    }
    let n = cqk5_w(this + 0x3e) as i32;
    let mut i: i32 = 0;
    while i < n {
        let linha = cqk5_d(this + 0xb1) + (i as usize) * 0x3b;
        if cqk5_d(linha) == clube {
            if f == 0 {
                match s {
                    1 => {
                        let acesso: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869480usize);
                        acesso(cqk5_registros(), clube, comp, 0x32);
                        *((cqk5_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0;
                        return 0;
                    }
                    2 | 0xff => {
                        let escala: unsafe extern "C" fn(u32, u32, u32, u32) -> u32 = core::mem::transmute(0x4bf850usize);
                        let peso = escala(0x1e, 0x32, cqk5_w(jogo + 0x34) as u16 as u32, 2);
                        // ponytail: a original empurra a fase (+0x32) e a rodada (+0x30) do jogo como palavras em
                        // registradores com lixo nos 16 bits de cima; a 0x86c000 só lê as palavras.
                        let fora: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x86c000usize);
                        fora(cqk5_registros(), clube, comp, cqk5_w(jogo + 0x32) as u16 as u32, cqk5_w(jogo + 0x30) as u16 as u32, peso);
                        *((cqk5_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                        return 0;
                    }
                    _ => {}
                }
            } else {
                match s {
                    1 => {
                        *((cqk5_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 0xfe;
                        return 0;
                    }
                    2 | 0xff => {
                        let cair: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x86a1c0usize);
                        cair(cqk5_registros(), clube, comp);
                        *((cqk5_d(this + 0xb1) + (i as usize) * 0x3b + 0x37) as *mut u8) = 3;
                        return 0;
                    }
                    _ => {}
                }
            }
        }
        i += 1;
    }
    0
}

/// Comparador da 0x654180: os registros vão ao histórico das pessoas e mexem nas reputações e no moral (não rodam duas
/// vezes): só a original. O plano, feito antes, nas fases 0 e 1: a primeira linha do clube com situação 1, 2 ou 0xff
/// decide o +0x37 que a rotina grava (fase 0: 0 com 1, 0xfe com 2/0xff; fase 1: 0xfe com 1, 3 com 2/0xff); depois a
/// linha tem de estar assim. Na liga ela só registra: a devolução 0.
unsafe extern "thiscall" fn comparar_spa_first_update_staff_records_for_status(this: usize, clube: usize, situacao: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%);
    let (s, f) = (situacao as u8, fase as u8);
    let mut plano: Option<(usize, u8)> = None;
    if f <= 1 {
        let n = cqk5_w(this + 0x3e) as i32;
        for i in 0..n.max(0) as usize {
            let l = cqk5_d(this + 0xb1) + i * 0x3b;
            if cqk5_d(l) == clube && (s == 1 || s == 2 || s == 0xff) {
                plano = Some((l, match (f, s) {
                    (0, 1) => 0,
                    (0, _) => 0xfe,
                    (_, 1) => 0xfe,
                    _ => 3,
                }));
                break;
            }
        }
    }
    let r = o(this, clube, situacao, fase, a4, jogo, a6);
    let igual = r == 0 && plano.map_or(true, |(l, v)| *((l + 0x37) as *const u8) == v);
    PLACAR.registrar(igual, || format!("spa_first_update_staff_records_for_status({:#x}, {:#x}, {}, {}) original={} plano={:x?}", this, clube, s, f, r, plano));
    r
}

/// { 0x850880 } (thiscall, ret 0x14, spa_first.cpp): `SpaFirst::set_club_progress(clube, fase, atual, melhor, pior)`,
/// a virtual +0x68 (da 0x684cd0 a cada linha da tabela): na entrada do clube nas estatísticas (Comp_Stats::
/// find_club_entry 0x4a4850: thiscall(+8, clube), ret 4) grava a posição atual (+0x73), a melhor possível (+0x74;
/// 17 e 18 viram 16) e a pior (+0x75; 17 e 18 viram 19): a faixa de 17 a 18 da Primera não conta como faixa própria
/// [inferência: as posições de 17 a 18 juntam-se à vizinha de cima na melhor e à de baixo na pior].
unsafe extern "thiscall" fn spa_first_set_club_progress(this: usize, clube: usize, _fase: u32, atual: u32, melhor: u32, pior: u32) {
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = achar(cqk5_d(this + 8), clube);
    let m = match melhor as u8 {
        0x11 | 0x12 => 0x10,
        v => v,
    };
    let p = match pior as u8 {
        0x11 | 0x12 => 0x13,
        v => v,
    };
    *((e + 0x73) as *mut u8) = atual as u8;
    *((e + 0x74) as *mut u8) = m;
    *((e + 0x75) as *mut u8) = p;
}

/// Comparador da 0x850880: a entrada é achada antes (a original e a cópia acham a mesma) e as duas rodam lado a lado
/// nos 3 bytes dela. Sem entrada (a original gravaria no nulo): só a original.
unsafe extern "thiscall" fn comparar_spa_first_set_club_progress(this: usize, clube: usize, fase: u32, atual: u32, melhor: u32, pior: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32)>(%%I2%%);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let e = achar(cqk5_d(this + 8), clube);
    if e == 0 {
        return o(this, clube, fase, atual, melhor, pior);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(e + 0x73, 3)], || o(this, clube, fase, atual, melhor, pior), || spa_first_set_club_progress(this, clube, fase, atual, melhor, pior));
    PLACAR.registrar(igual, || format!("spa_first_set_club_progress({:#x}, {:#x}) original={:x?} rust={:x?}", this, clube, mo, mr));
}
