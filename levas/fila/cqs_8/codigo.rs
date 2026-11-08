// ---------- %%LEVA%%a leva: as eliminatórias da Eurocopa: as regras e a data do sorteio, o tamanho de cada grupo, a situação de um clube e a tabela do grupo 1 (euro_champ_qual.cpp) ----------

unsafe fn cqs8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs8_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// { 0x57e8f0 } (thiscall, ret 0, euro_champ_qual.cpp; a virtual +0x8c, do construtor em 2001-000; quadro SEH pela Date
/// local, sem destrutor): `EuroChampQual::set_up_rules()`: 3 pontos pela vitória (+0xc2) e 1 pelo empate (+0xc3), +0x4a =
/// 3, dois turnos (+0x3c = 2, word), a categoria 0xa dos jogos (+0xc4), +0x43 = 2, os desempates 1, 2, 4, 5 (+0xc5..+0xc8),
/// uma vaga direta (+0xbe = 1) e uma de play-off (+0xbf = 1), sem rebaixamento (+0xc0 = +0xc1 = 0), +0x49 = 7, +0xa7 =
/// 0xffff (word); a data do sorteio (+0xb6, o dia do ano) = Date(10, 2, ano +0x40, 5) (0x549f70: push 5; push ano; push 2;
/// push 0xa; lea ecx, local -> thiscall, ret 0x10: 10 de março levado à sexta-feira [inferência]); +0xb8 = 0 (word); a
/// tabela das rodadas pela virtual +0x3c (push 0; push &+0x3a; push &+0xa9; push −1; mov ecx -> thiscall, ret 0x10) em
/// +0xba. Devolve 1.
unsafe fn set_up_rules(this: usize) -> u32 {
    cqs8_pb(this + 0xc2, 3);
    cqs8_pb(this + 0x4a, 3);
    core::ptr::write_unaligned((this + 0x3c) as *mut i16, 2); // cqs8: dois turnos
    cqs8_pb(this + 0xc3, 1);
    cqs8_pb(this + 0xc4, 0xa);
    cqs8_pb(this + 0x43, 2);
    for (k, v) in [1u8, 2, 4, 5].into_iter().enumerate() {
        cqs8_pb(this + 0xc5 + k, v);
    }
    cqs8_pb(this + 0xbe, 1);
    cqs8_pb(this + 0xbf, 1);
    cqs8_pb(this + 0xc0, 0);
    cqs8_pb(this + 0xc1, 0);
    cqs8_pb(this + 0x49, 7);
    core::ptr::write_unaligned((this + 0xa7) as *mut u16, 0xffff);
    let construir: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut data = [0u32; 2];
    // ponytail: a original empurra o ano no ecx com os 16 bits altos do ponteiro da copa; a 0x549f70 lê a palavra
    construir(&mut data, 0xa, 2, cqs8_w(this + 0x40) as u16 as u32, 5); // cqs8: a data do sorteio
    core::ptr::write_unaligned((this + 0xb6) as *mut u16, data[0] as u16);
    core::ptr::write_unaligned((this + 0xb8) as *mut u16, 0);
    crate::comparison::branch("cqs_8 regras da eurocopa");
    let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(cqs8_d(cqs8_d(this) + 0x3c));
    let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xba) as *mut u32, t as u32);
    1
}

unsafe extern "thiscall" fn euro_champ_qual_set_up_rules(this: usize) -> u32 {
    set_up_rules(this)
}

/// Lado a lado a partir do mesmo estado (os campos +0x3a..+0xc9 da copa, fora o ponteiro +0xba); a tabela das rodadas que
/// cada lado aloca em +0xba (pela virtual +0x3c, original) só é conferida como montada, e a da cópia é liberada.
unsafe extern "thiscall" fn comparar_euro_champ_qual_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let antes = cqs8_d(this + 0xba);
    let reg = [(this + 0x3a, 0x80usize), (this + 0xbe, 0xb)];
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        to.set(cqs8_d(this + 0xba));
        core::ptr::write_unaligned((this + 0xba) as *mut u32, antes as u32);
        r
    }, || {
        let r = set_up_rules(this);
        tr.set(cqs8_d(this + 0xba));
        core::ptr::write_unaligned((this + 0xba) as *mut u32, to.get() as u32);
        r
    });
    let (to, tr) = (to.get(), tr.get());
    // a tabela é montada pela virtual +0x3c (original nos dois lados) e tem bytes que ela não grava (sobra do malloc):
    // confere só que os dois lados a montaram
    let tabela = to != 0 && tr != 0;
    if tr != 0 && tr != to {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(tr);
    }
    SCOREBOARD.record(igual && a == b && tabela, || format!("euro_champ_qual_set_up_rules({:#x}) devolveu {} x {} memória {} tabela {}", this, a, b, igual, tabela));
    a
}

/// { 0x57d9d0 } (thiscall, ret 4, euro_champ_qual.cpp): `EuroChampQual::get_group_team_count(g)`: com o formato +0xf6 = 1
/// (9 grupos [inferência]), os grupos 0, 2 e 5 têm 6 seleções e os outros 5; com o formato 2 (10 grupos [inferência]), os
/// grupos 0, 2, 5 e 7 têm 6 e os outros 5; outro formato, a caixa da linha 0x245 e 5. `g` é o byte do argumento.
unsafe fn group_team_count(this: usize, g: u32) -> i16 {
    let g = g as u8;
    match *((this + 0xf6) as *const i8) {
        1 => if matches!(g, 0 | 2 | 5) { 6 } else { 5 }, // cqs8: os grupos de 6 no formato 1
        2 => if matches!(g, 0 | 2 | 5 | 7) { 6 } else { 5 },
        _ => {
            show_error_at(0x9c4674, 0x245);
            5
        }
    }
}

unsafe extern "thiscall" fn get_group_team_count(this: usize, g: u32) -> i16 {
    group_team_count(this, g)
}

unsafe extern "thiscall" fn comparar_get_group_team_count(this: usize, g: u32) -> i16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> i16>(%%I1%%);
    let r = o(this, g);
    if !matches!(*((this + 0xf6) as *const i8), 1 | 2) {
        return r; // ponytail: o formato desconhecido mostra a caixa: só a original
    }
    let n = group_team_count(this, g);
    SCOREBOARD.record(r == n, || format!("get_group_team_count(formato {}, grupo {}) original={} rust={}", *((this + 0xf6) as *const i8), g as u8, r, n));
    r
}

/// { 0x580b50 } (thiscall, ret 4, euro_champ_qual.cpp; a virtual da situação de um clube): `EuroChampQual::
/// get_qualification_status(clube)`: a estatística 0x12 do clube nas estatísticas da copa (Comp_Stats::get_club_stat
/// 0x4a2e10: push 0x12; push clube; mov ecx, [this+8] -> thiscall, ret 8, devolve float), truncada (_ftol 0x945640): com o
/// byte de baixo menor que 14 (com sinal), −2; senão −3 (word).
unsafe fn qualification_status(this: usize, clube: usize) -> i16 {
    let estatistica: unsafe extern "thiscall" fn(usize, usize, u32) -> f64 = core::mem::transmute(0x4a2e10usize);
    let x = match_pl_ftol(estatistica(cqs8_d(this + 8), clube, 0x12));
    ((((x as u8 as i8) < 0xe) as i32) - 3) as i16 // cqs8: a situação pela estatística 0x12
}

unsafe extern "thiscall" fn get_qualification_status(this: usize, clube: usize) -> i16 {
    qualification_status(this, clube)
}

unsafe extern "thiscall" fn comparar_get_qualification_status(this: usize, clube: usize) -> i16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> i16>(%%I2%%);
    let r = o(this, clube);
    let n = qualification_status(this, clube);
    SCOREBOARD.record(r == n, || format!("get_qualification_status({:#x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x57e7e0 } (thiscall, ret 0, euro_champ_qual.cpp; do sorteio, 2002-067): `EuroChampQual::init_group_one_table()`: o
/// grupo 1 é a própria copa: +0x3e = o tamanho do grupo −1 (0x57d9d0: push −1; mov ecx -> thiscall, ret 4; o byte 0xff
/// não é grupo de 6: 5); a tabela (0x944e46 de +0x3e × 0x3b em +0xb1; sem memória, a caixa da linha 0x2d2 e 0); +0xa7 =
/// (+0x3e − 1) × +0x3c (words: as rodadas, ida e volta); cada seleção da lista +0xf7 (na ordem) na linha i
/// (League::init_table_row 0x687430: push 0; push 0; push clube; push o dword do byte i com lixo em cima (só o byte é lido);
/// mov ecx -> thiscall, ret 0x10); a notícia do grupo (0x684230, thiscall). Devolve 1.
unsafe extern "thiscall" fn init_group_one_table(this: usize) -> u32 {
    let tamanho: unsafe extern "thiscall" fn(usize, u32) -> i16 = core::mem::transmute(0x57d9d0usize);
    let n = tamanho(this, 0xffff_ffff);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar((n as i32 * 0x3b) as u32);
    core::ptr::write_unaligned((this + 0xb1) as *mut u32, t as u32);
    if t == 0 {
        show_error_at(0x9c4674, 0x2d2);
        return 0;
    }
    let n = cqs8_w(this + 0x3e);
    core::ptr::write_unaligned((this + 0xa7) as *mut i16, n.wrapping_sub(1).wrapping_mul(cqs8_w(this + 0x3c))); // cqs8: as rodadas do grupo 1
    crate::comparison::branch("cqs_8 grupo 1 da eurocopa");
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut i: i8 = 0;
    if n > 0 {
        loop {
            let c = cqs8_d(cqs8_d(this + 0xf7) + i as isize as usize * 4);
            iniciar(this, i as u8 as u32, c, 0, 0);
            i = i.wrapping_add(1);
            if (i as i16) >= cqs8_w(this + 0x3e) {
                break;
            }
        }
    }
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    noticia(this);
    1
}

/// Só a original (a 0x687430 aloca a forma e o confronto direto de cada linha e a notícia vai aos humanos), conferindo o
/// plano feito antes: devolve 1, +0x3e = o tamanho do grupo −1, +0xa7 = (+0x3e − 1) × +0x3c e as seleções de +0xf7 nas
/// linhas, na ordem.
unsafe extern "thiscall" fn comparar_init_group_one_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I3%%);
    let n = if matches!(*((this + 0xf6) as *const i8), 1 | 2) { 5i16 } else { return o(this) };
    let plano: Vec<usize> = (0..n as usize).map(|k| cqs8_d(cqs8_d(this + 0xf7) + k * 4)).collect();
    let r = o(this);
    let t = cqs8_d(this + 0xb1);
    let ok = r == 1 && cqs8_w(this + 0x3e) == n && cqs8_w(this + 0xa7) == (n - 1).wrapping_mul(cqs8_w(this + 0x3c)) && t != 0
        && plano.iter().enumerate().all(|(k, &c)| cqs8_d(t + k * 0x3b) == c);
    SCOREBOARD.record(ok, || format!("init_group_one_table({:#x}) devolveu {}; plano {:x?}", this, r, plano));
    r
}
