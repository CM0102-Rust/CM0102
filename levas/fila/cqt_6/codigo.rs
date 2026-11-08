// ---------- %%LEVA%%a leva: a Copa dos Campeões do Brasil: a fase preliminar (a tabela dos 3 e os jogos) e a fase seguinte (os grupos e as semifinais) (bra_champ_cup.cpp) ----------

unsafe fn cqt6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqt6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

const CQT6_F: usize = 0x98a1e8;

/// { 0x426130 } (thiscall, ret 0, bra_champ_cup.cpp): `BraChampCup::create_preliminary_table()`: a preliminar é a própria
/// copa, com 3 times (+0x3e = 3); a tabela (0x944e46 de 0xb1 em +0xb1; sem memória, a caixa da linha 0x195 e 0); os 3
/// primeiros da lista +0x14 (entradas de 6 bytes, +0 o clube) nas linhas 0..2 (League::init_table_row 0x687430: push 0;
/// push 0; push clube; push o dword do byte da linha com lixo em cima (só o byte é lido); mov ecx -> thiscall, ret 0x10),
/// cada um zerado na lista ao entrar; a notícia do grupo (0x684230, thiscall). Devolve 1.
unsafe fn cqt6_tabela(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 3);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(0xb1);
    core::ptr::write_unaligned((this + 0xb1) as *mut u32, t as u32);
    if t == 0 {
        show_error_at(CQT6_F, 0x195);
        return 0;
    }
    crate::comparison::branch("cqt_6 tabela da preliminar");
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let mut i: i8 = 0;
    if cqt6_w(this + 0x3e) > 0 {
        loop {
            let e = i as isize as usize * 6;
            iniciar(this, i as u8 as u32, cqt6_d(cqt6_d(this + 0x14) + e), 0, 0);
            core::ptr::write_unaligned((cqt6_d(this + 0x14) + e) as *mut u32, 0); // cqt6: o time sai da lista
            i = i.wrapping_add(1);
            if (i as i16) >= cqt6_w(this + 0x3e) {
                break;
            }
        }
    }
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x684230usize);
    noticia(this);
    1
}

unsafe extern "thiscall" fn cqt6_campeoes_tabela_da_preliminar(this: usize) -> u32 {
    cqt6_tabela(this)
}

/// Só a original (a 0x687430 aloca e a notícia vai aos humanos), conferindo o plano feito antes: devolve 1, +0x3e = 3, os 3
/// primeiros da lista +0x14 nas linhas, na ordem, e zerados na lista.
unsafe extern "thiscall" fn comparar_cqt6_campeoes_tabela_da_preliminar(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let plano: Vec<usize> = (0..3).map(|k| cqt6_d(cqt6_d(this + 0x14) + k * 6)).collect();
    let r = o(this);
    let t = cqt6_d(this + 0xb1);
    let ok = r == 1 && cqt6_w(this + 0x3e) == 3 && t != 0
        && plano.iter().enumerate().all(|(k, &c)| cqt6_d(t + k * 0x3b) == c && cqt6_d(cqt6_d(this + 0x14) + k * 6) == 0);
    SCOREBOARD.record(ok, || format!("cqt6_campeoes_tabela_da_preliminar({:#x}) devolveu {}; plano {:x?}", this, r, plano));
    r
}

/// { 0x427320 } (thiscall, ret 0, bra_champ_cup.cpp): `BraChampCup::start_preliminary_stage()`: as posições previstas
/// (virtual +0x5c); cada um com a sua caixa se devolver 0: a tabela da preliminar (BraChampCup::create_preliminary_table
/// 0x426130, thiscall; linha 0x491), os jogos (League::create_fixtures 0x6835c0, thiscall, ret 0; linha 0x497) e a ordem da
/// tabela (League::sort_table 0x6827d0: push 0; mov ecx -> thiscall, ret 4; linha 0x49e); por fim +0x45 = 1 (a copa começou).
unsafe extern "thiscall" fn cqt6_campeoes_preliminar(this: usize) {
    let previstas: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqt6_d(cqt6_d(this) + 0x5c));
    previstas(this);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x426130usize);
    if tabela(this) == 0 {
        return show_error_at(CQT6_F, 0x491);
    }
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    if jogos(this) == 0 {
        return show_error_at(CQT6_F, 0x497);
    }
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    if ordenar(this, 0) == 0 { // cqt6: a ordem da tabela da preliminar
        return show_error_at(CQT6_F, 0x49e);
    }
    crate::comparison::branch("cqt_6 preliminar dos campeoes");
    core::ptr::write_unaligned((this + 0x45) as *mut u32, 1);
}

/// Só a original (a tabela, os jogos e o sorteio da ordem alocam e sorteiam), conferindo o plano: +0x45 = 1 e +0x3e = 3.
unsafe extern "thiscall" fn comparar_cqt6_campeoes_preliminar(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    o(this);
    SCOREBOARD.record(cqt6_d(this + 0x45) == 1 && cqt6_w(this + 0x3e) == 3, || format!("cqt6_campeoes_preliminar({:#x}) +0x45 {} +0x3e {}", this, cqt6_d(this + 0x45), cqt6_w(this + 0x3e)));
}

/// { 0x4263c0 } (thiscall, ret 0, bra_champ_cup.cpp; a virtual +0x28): `BraChampCup::advance_stage()`: na última fase
/// (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x219 e nada mais; senão +0x30 sobe um: ao chegar a 0 (o fim da
/// preliminar), os grupos (BraChampCup::create_group_stages 0x426470); ao chegar a 2 (o fim dos grupos), as semifinais
/// (BraChampCup::create_semi_final_stage 0x426720); as duas com o ecx da copa -> thiscall, ret 0.
unsafe extern "thiscall" fn cqt6_campeoes_fase_seguinte(this: usize) {
    let fase = cqt6_d(this + 0x30) as i32;
    if fase >= (cqt6_d(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(CQT6_F, 0x219);
    }
    let fase = fase + 1;
    core::ptr::write_unaligned((this + 0x30) as *mut i32, fase);
    let alvo = match fase {
        0 => 0x426470usize, // cqt6: os grupos depois da preliminar
        2 => 0x426720,      // cqt6: as semifinais depois dos grupos
        _ => return,
    };
    crate::comparison::branch("cqt_6 fase seguinte dos campeoes");
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(alvo);
    f(this);
}

/// Só a original (os grupos e as semifinais alocam e sorteiam), conferindo o plano: +0x30 sobe um fora da última fase (ou
/// mais, quando os grupos ou as semifinais gravam a fase delas) e fica na última.
unsafe extern "thiscall" fn comparar_cqt6_campeoes_fase_seguinte(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let antes = cqt6_d(this + 0x30) as i32;
    let ultima = antes >= (cqt6_d(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = cqt6_d(this + 0x30) as i32;
    // os grupos (fase 0) gravam a fase deles em +0x30 depois (na comparação de 06/11: −1 -> 1): aí só confere que andou
    let ok = if ultima { depois == antes } else if antes + 1 == 0 || antes + 1 == 2 { depois > antes } else { depois == antes + 1 };
    SCOREBOARD.record(ok, || format!("cqt6_campeoes_fase_seguinte({:#x}) fase {} -> {}", this, antes, depois));
}
