// ---------- %%LEVA%%a leva: o jogador que sai da seleção, a faixa de horário do jogo e o teto da lista de observados (national_teams.cpp, fix_man.cpp, shortlist_manager.cpp) ----------

/// { 0x775ba0 } (thiscall, ret 4, national_teams.cpp): tira o `jogador` da seleção. Com nação (+0x1a)
/// e ficha (+0x61): no registro de 110 bytes da nação em [this+4], o nível (0x76d810 com o registro e
/// 0); se a nota do nível (palavra +0xf + nível x 4) for menor que a da ficha (+9), procura o jogador
/// na faixa do nível da lista [this+0xc] (início = dword +8 + bytes +0xe dos níveis anteriores,
/// tamanho no byte +0xd + nível x 4): achou, o último da faixa vai para a vaga dele e o tamanho
/// diminui. Depois, convocado (+0x24): a vaga dele entre as 50 (+0xd7) zera, +0x24 zera e o byte +0x5e
/// fica só com os bits 0..1.
unsafe extern "thiscall" fn national_teams_remove_player(this: usize, jogador: usize) {
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let nacao = u(jogador + 0x1a);
    if nacao == 0 || u(jogador + 0x61) == 0 {
        return;
    }
    let nivel: unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8 = core::mem::transmute(0x76d810usize);
    let reg = u(this + 4).wrapping_add((*(nacao as *const u32) as usize).wrapping_mul(110));
    let n = nivel(jogador, reg as *const i16, 0) as i8 as isize;
    let campo = |o: isize| (reg as isize).wrapping_add(n * 4 + o) as usize;
    if core::ptr::read_unaligned(campo(0xf) as *const i16) < core::ptr::read_unaligned((u(jogador + 0x61) + 9) as *const i16) {
        let mut inicio = u(reg + 8) as i32;
        for k in 0..n.max(0) as usize {
            inicio = inicio.wrapping_add(*((reg + 0xe + 4 * k) as *const u8) as i32);
        }
        let tamanho = campo(0xd) as *mut u8;
        let fim = inicio.wrapping_add(*tamanho as i32);
        let lista = |i: i32| (u(this + 0xc) as isize).wrapping_add(i as isize * 4) as *mut usize;
        let mut i = inicio;
        while i < fim {
            if core::ptr::read_unaligned(lista(i)) == jogador {
                *tamanho = (*tamanho).wrapping_sub(1);
                core::ptr::write_unaligned(lista(i), core::ptr::read_unaligned(lista(fim - 1)));
                break;
            }
            i += 1;
        }
    }
    let selecao = u(jogador + 0x24);
    if selecao == 0 {
        return;
    }
    if let Some(k) = (0..50).find(|&k| u(selecao + 0xd7 + 4 * k) == jogador) {
        core::ptr::write_unaligned((selecao + 0xd7 + 4 * k) as *mut usize, 0);
    }
    core::ptr::write_unaligned((jogador + 0x24) as *mut usize, 0);
    *((jogador + 0x5e) as *mut u8) &= 3;
}

/// { 0x5af2f0 } (thiscall, ret 8, fix_man.cpp; `this` não é lido): a faixa de horário do jogo da
/// competição `comp` no dia `dia` (palavra): Holanda, Irlanda, Bélgica e França (id da nação +0x5d
/// em [0x9cf338], [0x9cf2dc], [0x9cf244], [0x9cf300]) só têm 1 no dia 6; as outras, 1 nos dias 5 e 6;
/// senão 2. Devolve só `al`.
unsafe extern "thiscall" fn get_time_slice(_this: usize, comp: usize, dia: u32) -> u8 {
    let dia = dia as i16;
    if comp != 0 {
        let nacao = core::ptr::read_unaligned((comp + 0x5d) as *const usize);
        if nacao != 0 {
            let id = *(nacao as *const u32);
            if [0x9cf338usize, 0x9cf2dc, 0x9cf244, 0x9cf300].iter().any(|&g| id == *(g as *const u32)) {
                return if dia != 6 { 2 } else { 1 };
            }
        }
    }
    if dia == 6 || dia == 5 { 1 } else { 2 }
}

/// { 0x84b230 } (stdcall, ret 0xc, shortlist_manager.cpp; `posicoes` não é lido): o teto de preço
/// para a vaga de tipo `tipo` na lista de observados, pela reputação do clube (palavra +0x80): tipo 4
/// = x550 em Itália, Inglaterra e Espanha ([0x9cf360], [0x9cf2e4], [0x9cf488], nação +0x53), x475 em
/// Alemanha e França ([0x9cf314], [0x9cf300]), x330 nas outras; tipo 5 = x550; tipo 6 = x110; os
/// outros, -1.
unsafe extern "stdcall" fn shortlist_manager_max_fee_for_squad_status(clube: usize, _posicoes: u32, tipo: u32) -> i32 {
    let rep = core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32;
    match tipo as i8 {
        4 => {
            let nacao = core::ptr::read_unaligned((clube + 0x53) as *const usize);
            let eh = |gs: &[usize]| nacao != 0 && gs.iter().any(|&g| *(nacao as *const u32) == *(g as *const u32));
            if eh(&[0x9cf360, 0x9cf2e4, 0x9cf488]) {
                rep.wrapping_mul(550)
            } else if eh(&[0x9cf314, 0x9cf300]) {
                rep.wrapping_mul(475)
            } else {
                rep.wrapping_mul(330)
            }
        }
        5 => rep.wrapping_mul(550),
        6 => rep.wrapping_mul(110),
        _ => -1,
    }
}

unsafe extern "thiscall" fn comparar_national_teams_remove_player(this: usize, jogador: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let u = |p: usize| core::ptr::read_unaligned(p as *const usize);
    let nacao = u(jogador + 0x1a);
    if nacao == 0 || u(jogador + 0x61) == 0 {
        return o(this, jogador);
    }
    // as regiões: o jogador, o registro da nação, a faixa do nível na lista e a seleção (o nível,
    // 0x76d810, só lê)
    let nivel: unsafe extern "stdcall" fn(usize, *const i16, u8) -> u8 = core::mem::transmute(0x76d810usize);
    let reg = u(this + 4).wrapping_add((*(nacao as *const u32) as usize).wrapping_mul(110));
    let n = nivel(jogador, reg as *const i16, 0) as i8 as isize;
    let mut regioes = [(jogador, 110), (reg, 110), (jogador, 0), (jogador, 0)];
    if (0..=3).contains(&n) {
        let inicio = (0..n as usize).fold(u(reg + 8) as i32, |s, k| s.wrapping_add(*((reg + 0xe + 4 * k) as *const u8) as i32));
        let tamanho = *((reg + 0xd + 4 * n as usize) as *const u8) as usize;
        if inicio >= 0 && tamanho > 0 {
            regioes[2] = (u(this + 0xc) + 4 * inicio as usize, 4 * tamanho);
        }
    }
    if u(jogador + 0x24) != 0 {
        regioes[3] = (u(jogador + 0x24) + 0xd7, 200);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, jogador), || national_teams_remove_player(this, jogador));
    PLACAR.registrar(igual, || format!("national_teams_remove_player(nivel {}) {:x?} x {:x?}", n, mo, mr));
}

unsafe extern "thiscall" fn comparar_get_time_slice(this: usize, comp: usize, dia: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u8>(%%I1%%)(this, comp, dia);
    let n = get_time_slice(this, comp, dia);
    PLACAR.registrar(r == n, || format!("get_time_slice(dia {}) original={} rust={}", dia as i16, r, n));
    r
}

unsafe extern "stdcall" fn comparar_shortlist_manager_max_fee_for_squad_status(clube: usize, posicoes: u32, tipo: u32) -> i32 {
    let r = orig::<unsafe extern "stdcall" fn(usize, u32, u32) -> i32>(%%I2%%)(clube, posicoes, tipo);
    let n = shortlist_manager_max_fee_for_squad_status(clube, posicoes, tipo);
    PLACAR.registrar(r == n, || format!("shortlist_manager_max_fee_for_squad_status(tipo {}) original={} rust={}", tipo as i8, r, n));
    r
}
