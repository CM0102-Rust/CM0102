// ---------- %%LEVA%%a leva: o fim da temporada das ligas: a cadeia de acessos e rebaixamentos; as notícias do prêmio e da TV de uma competição (league.cpp, comp.cpp) ----------

unsafe fn cqp1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

/// O slot 0xa4 da vtable da liga (thiscall, sem argumentos) [inferência: fecha a classificação final da temporada e
/// marca os que sobem e descem (+0x37 dos clubes), que a 0x689c80 lê].
unsafe fn finalize_league_table(liga: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqp1_d(cqp1_d(liga) + 0xa4));
    f(liga)
}

/// { 0x689c20 } (thiscall, ret 4, league.cpp, `League::process_promotion_relegation_chain(aos_pares)`; pelo slot do
/// fim de temporada da liga): só na liga do topo da pirâmide (+0x1c, a liga de cima, negativo): fecha a temporada
/// dela (slot 0xa4) e, descendo a cadeia (+0x20 o índice da liga de baixo na tabela [0xadadfc], até negativo), fecha a
/// de baixo e troca os que sobem e descem entre as duas (0x689c80 com contratos = 1, `aos_pares`, sem contagem
/// esperada: −1, −1; o ecx é o desta liga).
unsafe extern "thiscall" fn league_process_promotion_relegation_chain(this: usize, aos_pares: u32) {
    if (cqp1_d(this + 0x1c) as i32) >= 0 {
        return;
    }
    crate::comparacao::ramo("cqp_1 cadeia");
    let trocar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x689c80usize);
    let mut acima = this;
    finalize_league_table(this); // cqp1: fecha a liga do topo
    let mut i = cqp1_d(this + 0x20) as i32;
    while i >= 0 {
        let abaixo = cqp1_d(cqp1_d(0xadadfc) + i as usize * 4);
        finalize_league_table(abaixo); // cqp1: fecha a de baixo
        trocar(this, acima, abaixo, 1, aos_pares, u32::MAX, u32::MAX); // cqp1: sobem e descem
        i = cqp1_d(abaixo + 0x20) as i32;
        acima = abaixo;
    }
}

// ponytail: fecha temporadas e move clubes entre as ligas (contratos, técnicos, notícias): não roda duas vezes; só a
// original. A cópia leva o ramo.
unsafe extern "thiscall" fn comparar_league_process_promotion_relegation_chain(this: usize, aos_pares: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, u32)>(%%I0%%)(this, aos_pares)
}

/// O técnico do clube é humano? (0x7875e0, stdcall de 1, já em Rust; o ecx = [0xb7431c] da original não conta.)
unsafe fn cqp1_humano(clube: usize) -> bool {
    let f: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    f(clube) != 0
}

/// { 0x48e3e0 } (thiscall, ret 0x1c, comp.cpp, `Comp::send_prize_money_news(clube, valor (i64), posição, rodada, partida,
/// tipo)`; quadro SEH só pela notícia local): para o técnico humano do clube (0x7875e0), com o valor de pelo menos 50 ×
/// a reputação (+0x80), a notícia 0x1a (0x77bbe0): 0 a competição ([this+4]), 1 o valor, 2 o clube, 3 a posição, 4 a
/// rodada, 7 o tipo; com a partida, 5 o adversário e 6 se o clube venceu (+0x3d/+0x3e o vencedor nos pênaltis
/// [inferência], senão +0x4b/+0x4c, senão +0x43/+0x44 o placar); sem ela −1 e 0. Entregue a ele (0x7861e0).
unsafe extern "thiscall" fn send_prize_money_news(this: usize, clube: usize, lo: u32, hi: i32, posicao: u32, rodada: u32, partida: usize, tipo: u32) {
    if !cqp1_humano(clube) {
        return;
    }
    let minimo = *((clube + 0x80) as *const i16) as i64 * 50;
    let valor = ((hi as i64) << 32) | lo as i64;
    if valor < minimo {
        return;
    }
    crate::comparacao::ramo("cqp_1 noticia do premio");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let dado: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut n = [0u8; 0xe8];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x1a, 0);
    dado(p, 0, cqp1_d(cqp1_d(this + 4)) as u32);
    dado(p, 1, lo);
    dado(p, 2, cqp1_d(clube) as u32);
    dado(p, 3, posicao as u8 as i8 as i32 as u32);
    dado(p, 4, rodada as u16 as i16 as i32 as u32);
    dado(p, 7, tipo as u8 as i8 as i32 as u32);
    let b = |a: usize| *(a as *const i8);
    let venceu = if partida != 0 {
        if cqp1_d(partida + 0x1c) == clube {
            dado(p, 5, cqp1_d(cqp1_d(partida + 0x20)) as u32);
            b(partida + 0x3d) != 0 || b(partida + 0x4b) > b(partida + 0x4c) || b(partida + 0x43) > b(partida + 0x44)
        } else {
            dado(p, 5, cqp1_d(cqp1_d(partida + 0x1c)) as u32);
            b(partida + 0x3e) != 0 || b(partida + 0x4b) < b(partida + 0x4c) || b(partida + 0x43) < b(partida + 0x44)
        }
    } else {
        dado(p, 5, u32::MAX);
        false
    };
    dado(p, 6, venceu as u32);
    entregar(cqp1_d(0xb7431c), p, clube);
}

// ponytail: entrega a notícia (aloca); só a original. A cópia leva o ramo.
unsafe extern "thiscall" fn comparar_send_prize_money_news(this: usize, clube: usize, lo: u32, hi: i32, posicao: u32, rodada: u32, partida: usize, tipo: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32, u32, u32, usize, u32)>(%%I1%%)(this, clube, lo, hi, posicao, rodada, partida, tipo)
}

/// { 0x48ea40 } (thiscall, ret 8, comp.cpp, `Comp::send_tv_rights_news(clube, valor)`; quadro SEH só pela notícia local):
/// para o técnico humano do clube (0x7875e0), a notícia 0x1f (0x77bbe0): 0 o clube, 1 a competição ([this+4]), 2 o
/// valor, 3 o ano da competição (+0x40), entregue a ele (0x7861e0).
unsafe extern "thiscall" fn send_tv_rights_news(this: usize, clube: usize, valor: u32) {
    if !cqp1_humano(clube) {
        return;
    }
    crate::comparacao::ramo("cqp_1 noticia da tv");
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let dado: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut n = [0u8; 0xe8];
    let p = n.as_mut_ptr() as usize;
    construir(p, 0x1f, 0);
    dado(p, 0, cqp1_d(clube) as u32);
    dado(p, 1, cqp1_d(cqp1_d(this + 4)) as u32);
    dado(p, 2, valor);
    dado(p, 3, *((this + 0x40) as *const i16) as i32 as u32);
    entregar(cqp1_d(0xb7431c), p, clube);
}

// ponytail: entrega a notícia (aloca); só a original. A cópia leva o ramo.
unsafe extern "thiscall" fn comparar_send_tv_rights_news(this: usize, clube: usize, valor: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32)>(%%I2%%)(this, clube, valor)
}
