// ---------- %%LEVA%%a leva: as diretorias no início da carreira, as apelações de suspensão e a partida que o humano pode ver (manager_manager.cpp, human_manager.cpp) ----------

unsafe fn cpr5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpr5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpr5_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cpr5_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cpr5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpr5_pb(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cpr5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O técnico (ou o selecionador) do clube (0x53ca90).
unsafe fn club_manager(clube: usize) -> usize {
    let f: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
    f(clube)
}

unsafe fn manager_manager_is_human_club(clube: usize, qualquer: u8, pessoa: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    f(clube, 0, qualquer, pessoa, 0)
}

/// Uma das confianças da linha (+6..+0xc): sorteio(`n1`) + `h` guardado em float, menos sorteio(500), truncado.
unsafe fn roll_confidence(c: usize, n1: i32, h: f32) {
    let r1 = cpr5_sortear(n1);
    let s = (r1 as f64 + h as f64) as f32;
    let r2 = cpr5_sortear(0x1f4);
    cpr5_pw(c, match_pl_ftol(s as f64 - r2 as f64) as i16);
}

/// Abaixo de 5500: 5500 + sorteio(250); acima de 10000: 10000.
unsafe fn clamp_confidence_5500(c: usize) {
    if cpr5_w(c) < 0x157c {
        cpr5_pw(c, (cpr5_sortear(0xfa) + 0x157c) as i16);
    } else if cpr5_w(c) > 0x2710 {
        cpr5_pw(c, 0x2710);
    }
}

/// Os campos da linha que as duas saídas zeram (+0x10..+0x16, os bits de +2, +3, +4, +5, +0x28 e +0x40).
unsafe fn clear_board_flags(l: usize) {
    for k in [0x10usize, 0x12, 0x14, 0x16] {
        cpr5_pw(l + k, 0);
    }
    cpr5_pb(l + 2, cpr5_b(l + 2) & 0xe0);
    cpr5_pb(l + 4, 0);
    cpr5_pb(l + 3, cpr5_b(l + 3) & 0xe1);
    cpr5_pb(l + 2, cpr5_b(l + 2) & 0x1f);
    cpr5_pd(l + 0x28, 0);
    cpr5_pb(l + 5, 0);
    cpr5_pb(l + 3, cpr5_b(l + 3) & 0xfe);
    cpr5_pb(l + 0x40, cpr5_b(l + 0x40) & 7);
    cpr5_pb(l + 0x40, cpr5_b(l + 0x40) & 0xf8);
}

/// { 0x68f130 } (thiscall, ret 0, manager_manager.cpp): `MANAGER_MANAGER::init_club_board_records()`: no começo da
/// carreira (pelo construtor 0x68dbf0). (1) Cada pessoa com clube (+0x24): função (+0x28) 5 ou 12 vira o técnico do
/// clube (+0xcf); 11 fica; 13 e 15 viram 11 (com ficha de jogador +0x61) ou 8; as outras perdem o clube; sem contrato
/// (+0x39) e sem função de origem (+0x3d), a função vai para +0x3d; fora de 11 a função zera. (2) Cada clube (as
/// seleções só com nação e +0x71): a segunda seleção tem a reputação (+0x80) presa a 200 × a da nação (+0x8e) e as
/// vagas zeradas; a linha dele (73 bytes em [this]) ganha as datas (23/07/1971 em +0x41, hoje em +0x18), a reputação² ×
/// 0,25 (double em +0x30) e zeros. Com técnico: a data 23/08/1971 em +0x20 (pela linha do id), a expectativa `f` (a
/// reputação, a nota da diretoria +0xa ou 4500, que cai a 0,75 × reputação), `h` = f / (reputação × 0,5 + 2500) ×
/// 7500 e as quatro confianças (+6, +8, +0xa, +0xc) por sorteios, presas embaixo (5750 ou 5500 com sorteio) e em
/// cima (10000; +6 acima de 9500 vira 10000 − sorteio(500)); zera as marcas e põe as expectativas da temporada
/// (0x6a3770 com 1). Sem técnico (fora do clube parceiro): confianças 5000, +0xe = 16, marcas zeradas; clube
/// dirigível (ou com competição e nação com +0x11c & 3) recebe as expectativas e a vaga (0x69ba50).
unsafe extern "thiscall" fn init_club_board_records(this: usize) {
    // (1) as pessoas
    let mut k = 0i32;
    while k < cpr5_d(0xae2364) as i32 {
        let p = cpr5_d(0xae23bc).wrapping_add(k as usize * 0x6e);
        k += 1;
        let clube = cpr5_d(p + 0x24);
        if clube == 0 {
            continue;
        }
        let f = cpr5_b(p + 0x28);
        match f {
            5 | 12 => {
                cpr5_pd(clube + 0xcf, p as u32);
                continue;
            }
            11 => continue,
            _ => {}
        }
        let mut tira = true;
        if f == 0xf || f == 0xd {
            if cpr5_d(p + 0x61) != 0 {
                cpr5_pb(p + 0x28, 0xb);
                tira = false;
            } else {
                cpr5_pb(p + 0x28, 8);
            }
        }
        if tira {
            cpr5_pd(p + 0x24, 0);
        }
        if cpr5_d(p + 0x39) == 0 && cpr5_b(p + 0x3d) == 0 {
            cpr5_pb(p + 0x3d, cpr5_b(p + 0x28));
        }
        if cpr5_b(p + 0x28) != 0xb {
            cpr5_pb(p + 0x28, 0);
        }
    }
    // (2) os clubes
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let segunda: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x5409b0usize);
    let data_dma: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let expectativas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
    let mut parceiro = 0u32; // a original guarda numa variável só, de clube em clube
    let mut i = 0i32;
    let mut ofs = 0usize;
    while i < cpr5_d(0xae235c) as i32 {
        let clube = cpr5_d(0xae23b4).wrapping_add(i as usize * 0x245);
        let l = cpr5_d(this).wrapping_add(ofs);
        i += 1;
        ofs += 0x49;
        if is_national_team(clube) != 0 {
            let n = cpr5_d(clube + 0x53);
            if n == 0 || cpr5_d(n + 0x71) == 0 {
                continue;
            }
        }
        if segunda(clube) != 0 {
            let n = cpr5_d(clube + 0x53);
            if n != 0 && cpr5_w(clube + 0x80) as i32 > cpr5_w(n + 0x8e) as i32 * 200 {
                cpr5_pw(clube + 0x80, cpr5_w(n + 0x8e).wrapping_mul(200));
            }
            cpr5_pd(clube + 0xcf, 0);
            cpr5_pd(clube + 0xbf, 0);
            cpr5_pd(clube + 0xd3, 0);
            core::ptr::write_bytes((clube + 0xc3) as *mut u8, 0, 12);
            core::ptr::write_bytes((clube + 0x19f) as *mut u8, 0, 0x14 + 0x1c + 12); // +0x19f..+0x1db
        }
        let (mut d, mut t) = ([0u8; 8], [0u8; 8]);
        let r = data_dma(d.as_mut_ptr() as usize, 0x17, 6, 0x7b3, 0xffff_ffff);
        copia(l + 0x41, t.as_mut_ptr() as usize, r);
        cpr5_pw(l + 0xe, 0);
        cpr5_pd(l + 0x2c, 0);
        copia(l + 0x18, t.as_mut_ptr() as usize, 0xae2c90);
        cpr5_pb(l, 0);
        let rep = cpr5_w(clube + 0x80) as i32;
        core::ptr::write_unaligned((l + 0x30) as *mut f64, rep.wrapping_mul(rep) as f64 * 0.25);
        cpr5_pd(l + 0x38, 0);
        cpr5_pd(l + 0x3c, 0);
        if club_manager(clube) == 0 {
            // sem técnico
            let parceiro_de: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
            if parceiro_de(clube, &mut parceiro, 1) != 0 && parceiro != 0 {
                continue;
            }
            crate::comparison::branch("cpr_5 diretoria sem tecnico");
            for c in [6usize, 8, 0xa, 0xc] {
                cpr5_pw(l + c, 0x1388);
            }
            cpr5_pw(l + 0xe, 0x10);
            clear_board_flags(l);
            let dirigivel: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x544820usize);
            if dirigivel(clube) == 0 {
                if is_national_team(clube) != 0 {
                    continue;
                }
                let objs = cpr5_d(0xadadfc);
                let comp = cpr5_d(clube + 0x57);
                let n = cpr5_d(clube + 0x53);
                if objs == 0 || comp == 0 || n == 0 || cpr5_d(objs + cpr5_d(comp) * 4) == 0 || cpr5_b(n + 0x11c) & 3 == 0 {
                    continue;
                }
            }
            expectativas(this, clube, 1);
            let vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x69ba50usize);
            vaga(this, clube, 0);
            continue;
        }
        // com técnico
        let r = data_dma(d.as_mut_ptr() as usize, 0x17, 7, 0x7b3, 0xffff_ffff);
        copia(cpr5_d(this).wrapping_add(cpr5_d(clube).wrapping_mul(0x49)) + 0x20, t.as_mut_ptr() as usize, r);
        let mut f: f32;
        let mut direto = false; // a 4500 salta o teste do zero e do 500
        if cpr5_d(club_manager(clube) + 0x69) == 0 && manager_manager_is_human_club(clube, 1, 0) == 0 {
            f = rep as f32;
        } else if manager_manager_is_human_club(clube, 1, 0) != 0 {
            if cpr5_d(clube + 0xcf) != 0 && cpr5_d(club_manager(clube) + 0x69) != 0 {
                f = cpr5_w(cpr5_d(club_manager(clube) + 0x69) + 0xa) as f32;
            } else {
                f = 4500.0;
                direto = true;
            }
        } else {
            f = cpr5_w(cpr5_d(club_manager(clube) + 0x69) + 0xa) as f32;
            let teto = rep as f64 * 0.75;
            if (cpr5_w(cpr5_d(club_manager(clube) + 0x69) + 0xa) as f64) < teto {
                let v = match_pl_ftol(teto) as i16;
                cpr5_pw(cpr5_d(club_manager(clube) + 0x69) + 0xa, v); // a nota da diretoria cai a 0,75 × reputação
            }
        }
        if !direto {
            if f == 0.0 {
                let r80 = cpr5_w(clube + 0x80);
                f = if r80 != 0 { (r80 as f64 * 0.75) as f32 } else { 5000.0 };
            } else if f < 500.0 {
                f = (cpr5_sortear(0x3e8) + 0x1f4) as f32;
            }
        }
        let mut g = rep as f64 * 0.5 + 2500.0;
        if g == 0.0 {
            g = f as f64 * 1.5;
        } else if g < 500.0 {
            g = (cpr5_sortear(0x2ee) + 0xfa) as f64;
        }
        crate::comparison::branch("cpr_5 diretoria com tecnico");
        let h = ((f as f64 / g) * 7500.0) as f32;
        roll_confidence(l + 6, 0x1f4, h);
        let c = cpr5_w(l + 6);
        if c < 0x1676 {
            crate::comparison::branch("cpr_5 confianca baixa");
            cpr5_pw(l + 6, (cpr5_sortear(0x2ee) + 0x1676) as i16);
        } else if c > 0x251c {
            cpr5_pw(l + 6, (0x2710 - cpr5_sortear(0x1f4)) as i16);
        }
        roll_confidence(l + 8, 0x1f4, h);
        if cpr5_w(l + 8) < 0x1676 {
            cpr5_pw(l + 8, (cpr5_sortear(0x1f4) + 0x1676) as i16);
        } else if cpr5_w(l + 8) > 0x2710 {
            cpr5_pw(l + 8, 0x2710);
        }
        roll_confidence(l + 0xa, 0x6d6, h);
        clamp_confidence_5500(l + 0xa);
        roll_confidence(l + 0xc, 0x9c4, h);
        clamp_confidence_5500(l + 0xc);
        clear_board_flags(l);
        expectativas(this, clube, 1);
    }
}

// ponytail: sorteia, põe as expectativas (0x6a3770) e abre vagas (0x69ba50, aloca): só a original; a cópia leva os
// ramos (roda uma vez, no começo da carreira).
unsafe extern "thiscall" fn comparar_init_club_board_records(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// { 0x6abef0 } (thiscall, ret 0, manager_manager.cpp): `MANAGER_MANAGER::process_ban_appeals()`: percorre a lista das
/// apelações (+0x14, a conta em palavra com sinal relida a cada volta) e, para cada uma cuja data (+5) já não é depois
/// de hoje (0x456740), com resultado (+4): as até 4 regras (+0xd..+0x10, ≥ 0) recebem o resultado na disciplina
/// (0x557000 com os campos +0x11..+0x18); depois a notícia do resultado (0x6ac060) e a apelação sai (0x68d1f0). Item
/// nulo: a caixa "Error" (0x57f5) e para. Pelo dia a dia da diretoria (0x68f9c0); só as apelações do humano entram na
/// lista.
unsafe extern "thiscall" fn process_ban_appeals(this: usize) {
    let lista = || cpr5_d(this + 0x14);
    cpr5_pd(lista() + 0xa, cpr5_d(lista() + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let aplicar: unsafe extern "thiscall" fn(usize, u32, i8, u8, u8, i16, u8, i16, u8) = core::mem::transmute(0x557000usize);
    let noticia: unsafe extern "thiscall" fn(usize, usize, u8, u8, i16, u8) = core::mem::transmute(0x6ac060usize);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let mut k = 0i32;
    while k < cpr5_w(lista()) as i32 {
        let item = proximo(lista());
        if item == 0 {
            return show_error_at(0x9f18a4, 0x57f5);
        }
        if depois(item + 5, cpr5_d(0xae2c90) as u32, cpr5_d(0xae2c94) as u32) == 0 {
            crate::comparison::branch("cpr_5 apelacao decidida");
            let p = cpr5_d(0xae23bc).wrapping_add(cpr5_d(item).wrapping_mul(0x6e));
            if cpr5_b(item + 4) != 0 {
                for j in 0..4usize {
                    let regra = cpr5_b(item + 0xd + j) as i8;
                    if regra > -1 {
                        // ponytail: a original empurra os campos em registradores com lixo em cima; a 0x557000 lê só
                        // o byte ou a palavra de cada um.
                        aplicar(cpr5_d(0xae28f0), cpr5_d(p) as u32, regra, cpr5_b(item + 0x11), cpr5_b(item + 0x12),
                                cpr5_w(item + 0x13), cpr5_b(item + 0x15), cpr5_w(item + 0x16), cpr5_b(item + 0x18));
                    }
                }
            }
            noticia(this, p, cpr5_b(item + 4), cpr5_b(item + 0x15), cpr5_w(item + 0x16), cpr5_b(item + 0x18));
            remover(lista(), item);
        }
        k += 1;
    }
}

// ponytail: aplica a disciplina, solta a notícia e libera a apelação: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_process_ban_appeals(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// { 0x5fe700 } (thiscall, ret 8, human_manager.cpp): `HUMAN_MANAGER::can_watch_fixture(partida, humano)`: o humano
/// (ou, nulo, o da tela atual [0xb74c26 + 0x300 × [0xb77c7a]]) pode ver a partida se ela tem +0x43 negativo, nenhum dos
/// dois clubes (+0x1c, +0x20, com as marcas +0x40, +0x41) é dele (0x600a50) e, em cada grupo das nações-chave
/// ([0xb64738]), nenhuma partida da lista do dia dela (data +0x28/+0x2a, fatia +0x3f; 0x5ae6a0) tem um clube dele: 1.
/// Senão 0. Partida nula: a caixa "Error" (0x707) e 0. Pela limpeza das partidas assistidas (0x5fe8d0) e pela escolha
/// da partida (0x5fe500).
unsafe extern "thiscall" fn can_watch_fixture(this: usize, partida: usize, humano: usize) -> u32 {
    if partida == 0 {
        show_error_at(0x9cf11c, 0x707);
        return 0;
    }
    let mut humano = humano;
    if humano == 0 {
        humano = cpr5_d(0xb74c26usize.wrapping_add((cpr5_w(0xb77c7a) as isize as usize).wrapping_mul(0x300)));
        if humano == 0 {
            return 0;
        }
    }
    if cpr5_b(partida + 0x43) as i8 > -1 {
        return 0;
    }
    let dele = |p: usize| manager_manager_is_human_club(cpr5_d(p + 0x1c), cpr5_b(p + 0x40), humano) != 0
        || manager_manager_is_human_club(cpr5_d(p + 0x20), cpr5_b(p + 0x41), humano) != 0;
    if dele(partida) {
        return 0;
    }
    let data_nova: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let lista_de: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x5ae6a0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut grupo = 0u8;
    while (grupo as i8) < cpr5_b(0xb64738) as i8 {
        let mut d = [0u8; 8];
        let data = data_nova(d.as_mut_ptr() as usize, cpr5_w(partida + 0x2a) as u16 as u32, cpr5_w(partida + 0x28) as u16 as u32);
        // ponytail: a original empurra o grupo como palavra inteira (os bytes de cima são sobra de pilha); a 0x5ae6a0
        // repassa o tipo (char) e a 0x5abf10 lê só o byte.
        let lista = lista_de(data, cpr5_b(partida + 0x3f) as u32, grupo as u32);
        if lista != 0 {
            cpr5_pd(lista + 0xa, cpr5_d(lista + 2) as u32);
            let mut p = proximo(lista);
            while p != 0 {
                if dele(p) {
                    return 0;
                }
                p = proximo(lista);
            }
        }
        grupo = grupo.wrapping_add(1);
    }
    1
}

/// Comparador: a original e a cópia (as duas só leem; a volta da lista recomeça o cursor), pelo valor.
unsafe extern "thiscall" fn comparar_can_watch_fixture(this: usize, partida: usize, humano: usize) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%)(this, partida, humano);
    if partida == 0 {
        return r;
    }
    let n = can_watch_fixture(this, partida, humano);
    PLACAR.record(r == n, || format!("can_watch_fixture(partida {:x}, humano {:x}) original={} rust={}", partida, humano, r, n));
    r
}
