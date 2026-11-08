// ---------- %%LEVA%%a leva: a ordem dos clubes da liga para a tabela de jogos e os clubes possíveis de cada lado de um jogo da copa (league.cpp, Cup.cpp) ----------

unsafe fn cpu12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpu12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de 59 bytes da tabela da liga.
const CUP_LEAGUE_TABLE_ROW_SIZE: usize = 0x3b;

/// Copia um registro de 59 bytes (o `rep movsd` de 14 dwords, a palavra e o byte da original).
unsafe fn copy_row_59(destino: usize, origem: usize) {
    core::ptr::copy(origem as *const u8, destino as *mut u8, CUP_LEAGUE_TABLE_ROW_SIZE);
}

/// Põe o par (i, j) da tabela nas vagas k e metade + k da grade, marca as vagas e os dois clubes.
unsafe fn place_pair(grade: usize, posto: *mut u32, usado: *mut u32, tab: usize, i: usize, j: usize, k: &mut i32, metade: i32) {
    copy_row_59(grade + *k as usize * CUP_LEAGUE_TABLE_ROW_SIZE, tab + i * CUP_LEAGUE_TABLE_ROW_SIZE);
    let m = metade + *k;
    copy_row_59(grade + m as usize * CUP_LEAGUE_TABLE_ROW_SIZE, tab + j * CUP_LEAGUE_TABLE_ROW_SIZE);
    *posto.add(m as usize) = 1;
    *posto.add(*k as usize) = 1;
    *usado.add(j) = 1;
    *usado.add(i) = 1;
    *k += 1;
}

/// { 0x686a70 } (thiscall, ret 4, league.cpp): `League::draw_team_order(clubes)`: embaralha a tabela da liga
/// ([+0xb1], +0x3e registros de 59 bytes) para a grade de jogos: guarda um sorteio (0x90cfc0, 30000), semeia o rand
/// da CRT com +0x40 + [0xdd7120] e troca n pares sorteados (rand × n / 0x8000), e volta a semente (sorteio + 1). Liga
/// com a marca 0x100 em +0xd9: cada clube vai à vaga que a tabela virtual +0xa8 diz (se livre), os outros às
/// primeiras livres, e a +0xac recebe cada vaga. Senão: na Escócia ([+4] = [0x9cf5dc]) o Celtic e o Rangers
/// ([0x9d0118], [0x9d011c]) vão às vagas 0 e clubes / 2, em ordem sorteada (0x90cfc0, 2); os clubes do mesmo estádio
/// (+0x69) e depois os de estádios ligados (+0x48 do estádio) vão em pares às vagas k e clubes / 2 + k (passando da
/// metade dos clubes, a caixa); os outros, às primeiras vagas livres. A grade (0x944cf1, livre em 0x944c94) volta à
/// tabela. Sem memória: a caixa (linha 0xed3).
unsafe extern "thiscall" fn draw_team_order(this: usize, clubes: i32) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let semear: unsafe extern "C" fn(u32) = core::mem::transmute(0x9469fausize);
    let rand: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x946a04usize);
    let n = || cpu12_w(this + 0x3e) as i32;
    let tab = || cpu12_d(this + 0xb1);
    let metade_n = n() / 2;
    let usado = novo((n() * 4) as u32) as *mut u32;
    let posto = novo((n() << 2) as u32) as *mut u32;
    let grade = novo((n() * CUP_LEAGUE_TABLE_ROW_SIZE as i32) as u32);
    let mut k: i32 = 0;
    if grade == 0 || usado.is_null() || posto.is_null() {
        show_error_at(0x9f15f4, 0xed3);
        liberar(posto as usize);
        liberar(usado as usize);
        liberar(grade);
        return;
    }
    let mut i = 0;
    while i < n() {
        *usado.add(i as usize) = 0;
        *posto.add(i as usize) = 0;
        i += 1;
    }
    let semente = sorteio(30000).wrapping_add(1);
    semear((cpu12_w(this + 0x40) as i32).wrapping_add(*(0xdd7120 as *const i32)) as u32); // cpu12: a semente
    crate::comparison::branch("cpu_12 ordem sorteada");
    let mut i = 0;
    while i < n() {
        let a = ((rand() as i64 * n() as i64) / 0x8000) as i32 as usize;
        let b = ((rand() as i64 * n() as i64) / 0x8000) as i32 as usize;
        let mut tmp = [0u8; CUP_LEAGUE_TABLE_ROW_SIZE];
        let t = tab();
        core::ptr::copy(( t + a * CUP_LEAGUE_TABLE_ROW_SIZE) as *const u8, tmp.as_mut_ptr(), CUP_LEAGUE_TABLE_ROW_SIZE);
        copy_row_59(t + a * CUP_LEAGUE_TABLE_ROW_SIZE, t + b * CUP_LEAGUE_TABLE_ROW_SIZE);
        core::ptr::copy(tmp.as_ptr(), (tab() + b * CUP_LEAGUE_TABLE_ROW_SIZE) as *mut u8, CUP_LEAGUE_TABLE_ROW_SIZE); // cpu12: a troca
        i += 1;
    }
    semear(semente as u32);
    let metade = clubes / 2;
    if cpu12_w(this + 0xd9) as u16 & 0x100 != 0 {
        let mut i = 0;
        while i < n() {
            *((grade + i as usize * CUP_LEAGUE_TABLE_ROW_SIZE) as *mut u32) = 0;
            i += 1;
        }
        let vt = cpu12_d(this);
        let vaga_do: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(cpu12_d(vt + 0xa8));
        let mut i = 0usize;
        while (i as i32) < n() {
            let s = vaga_do(this, cpu12_d(tab() + i * CUP_LEAGUE_TABLE_ROW_SIZE)) as i8 as i32;
            if s >= 0 && s < n() && cpu12_d(grade + s as usize * CUP_LEAGUE_TABLE_ROW_SIZE) == 0 {
                copy_row_59(grade + s as usize * CUP_LEAGUE_TABLE_ROW_SIZE, tab() + i * CUP_LEAGUE_TABLE_ROW_SIZE);
                *((tab() + i * CUP_LEAGUE_TABLE_ROW_SIZE) as *mut u32) = 0;
            }
            i += 1;
        }
        let mut i = 0usize;
        while (i as i32) < n() {
            let lin = tab() + i * CUP_LEAGUE_TABLE_ROW_SIZE;
            if cpu12_d(lin) != 0 {
                if let Some(j) = (0..n().max(0) as usize).find(|&j| cpu12_d(grade + j * CUP_LEAGUE_TABLE_ROW_SIZE) == 0) {
                    copy_row_59(grade + j * CUP_LEAGUE_TABLE_ROW_SIZE, lin);
                }
            }
            i += 1;
        }
        let gravar: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(cpu12_d(vt + 0xac));
        let mut i = 0;
        while i < n() {
            gravar(this, cpu12_d(grade + i as usize * CUP_LEAGUE_TABLE_ROW_SIZE), i);
            i += 1;
        }
    } else {
        if cpu12_d(cpu12_d(this + 4)) == cpu12_d(0x9cf5dc) {
            let (mut rangers, mut celtic): (i8, i8) = (-1, -1); // bl e [esp+0x17]
            let total = cpu12_w(this + 0x3e);
            let mut c: i8 = 0;
            while total > 0 && (c as i16) < total {
                let id = cpu12_d(cpu12_d(tab() + c as isize as usize * CUP_LEAGUE_TABLE_ROW_SIZE));
                if id == cpu12_d(0x9d0118) {
                    celtic = c;
                }
                if id == cpu12_d(0x9d011c) {
                    rangers = c;
                }
                c = c.wrapping_add(1);
            }
            if rangers != -1 && celtic != -1 {
                let (primeiro, segundo) = if sorteio(2) != 0 { (celtic, rangers) } else { (rangers, celtic) };
                copy_row_59(grade, tab() + primeiro as isize as usize * CUP_LEAGUE_TABLE_ROW_SIZE);
                copy_row_59(grade + metade as usize * CUP_LEAGUE_TABLE_ROW_SIZE, tab() + segundo as isize as usize * CUP_LEAGUE_TABLE_ROW_SIZE);
                k = 1;
                *posto.add(metade as usize) = 1;
                *posto = 1;
                *usado.add(celtic as isize as usize) = 1;
                *usado.add(rangers as isize as usize) = 1;
            }
        }
        // os clubes do mesmo estádio (+0x69)
        let mut i = 0usize;
        while (i as i32) < n() - 1 {
            if *usado.add(i) == 0 {
                let mut j = i + 1;
                while (j as i32) < n() {
                    if *usado.add(j) == 0 {
                        let ei = cpu12_d(cpu12_d(tab() + i * CUP_LEAGUE_TABLE_ROW_SIZE) + 0x69);
                        if ei != 0 {
                            let ej = cpu12_d(cpu12_d(tab() + j * CUP_LEAGUE_TABLE_ROW_SIZE) + 0x69);
                            if ej != 0 && ei == ej {
                                if k >= metade_n {
                                    show_error_at(0x9f15f4, 0xf76);
                                } else {
                                    place_pair(grade, posto, usado, tab(), i, j, &mut k, metade);
                                }
                                break;
                            }
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        // os clubes de estádios ligados (+0x48 do estádio de um é o do outro)
        let mut i = 0usize;
        while (i as i32) < n() - 1 {
            if *usado.add(i) == 0 {
                let mut j = i + 1;
                while (j as i32) < n() {
                    if *usado.add(j) == 0 {
                        let ei = cpu12_d(cpu12_d(tab() + i * CUP_LEAGUE_TABLE_ROW_SIZE) + 0x69);
                        if ei == 0 {
                            break;
                        }
                        let ej = cpu12_d(cpu12_d(tab() + j * CUP_LEAGUE_TABLE_ROW_SIZE) + 0x69);
                        if ej == 0 {
                            break;
                        }
                        if cpu12_d(ei + 0x48) == ej || cpu12_d(ej + 0x48) == ei {
                            if k >= metade_n {
                                show_error_at(0x9f15f4, 0xfa3);
                            } else {
                                place_pair(grade, posto, usado, tab(), i, j, &mut k, metade);
                            }
                            break;
                        }
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        // os outros, às primeiras vagas livres
        let mut i = 0usize;
        while (i as i32) < n() {
            if *usado.add(i) == 0 {
                if let Some(j) = (0..n().max(0) as usize).find(|&j| *posto.add(j) == 0) { // cpu12: a primeira vaga livre
                    crate::comparison::branch("cpu_12 clube na vaga livre");
                    copy_row_59(grade + j * CUP_LEAGUE_TABLE_ROW_SIZE, tab() + i * CUP_LEAGUE_TABLE_ROW_SIZE);
                    *posto.add(j) = 1;
                    *usado.add(i) = 1;
                }
            }
            i += 1;
        }
    }
    let mut i = 0usize;
    while (i as i32) < n() {
        copy_row_59(tab() + i * CUP_LEAGUE_TABLE_ROW_SIZE, grade + i * CUP_LEAGUE_TABLE_ROW_SIZE);
        i += 1;
    }
    liberar(posto as usize);
    liberar(usado as usize);
    liberar(grade);
}

/// O núcleo da 0x5219b0: o jogo `jogo` da rodada `rodada` ([[+0xad] + rodada × 4] + jogo × 0x35): cada lado (+0 e
/// +2) com clube vai a `saida[*conta]` (e conta sobe); lado vazio (−1) desce ao jogo de onde ele vem (+0xe/+0xa e
/// +0x10/+0xc). Rodada negativa ou jogo −1 (também nas descidas): a caixa (linha 0x1592, se `caixa`) e a conta.
/// Devolve a conta.
unsafe fn collect_fixture_teams(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32, caixa: bool) -> i32 {
    if rodada < 0 || jogo == -1 {
        if caixa {
            show_error_at(0x9b55ac, 0x1592);
        }
        return *conta;
    }
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let a = cpu12_w(reg);
    if a == -1 {
        collect_fixture_teams(this, cpu12_w(reg + 0xe) as i32, cpu12_w(reg + 0xa) as i32, saida, conta, caixa);
    } else {
        *saida.offset(*conta as isize) = a;
        *conta += 1;
    }
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let b = cpu12_w(reg + 2);
    if b == -1 {
        collect_fixture_teams(this, cpu12_w(reg + 0x10) as i32, cpu12_w(reg + 0xc) as i32, saida, conta, caixa);
        return *conta;
    }
    *saida.offset(*conta as isize) = b;
    *conta += 1;
    *conta
}

/// { 0x5219b0 } (thiscall, ret 0x10, Cup.cpp): `Cup::collect_possible_teams(rodada, jogo, saida, *conta)`: acrescenta
/// a `saida` os clubes que ainda podem jogar esse jogo, descendo pelos jogos de onde vêm os lados vazios. Saída ou
/// conta nula: a original (a caixa).
unsafe extern "thiscall" fn collect_possible_teams(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32) -> i32 {
    if saida.is_null() || conta.is_null() {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(%%I2%%)(this, rodada, jogo, saida, conta);
    }
    collect_fixture_teams(this, rodada, jogo, saida, conta, true)
}

/// { 0x521790 } (thiscall, ret 0x10, Cup.cpp): `Cup::get_fixture_side_teams(rodada, jogo, lado, saida)`: o clube de um
/// lado do jogo (lado ≠ 0: +0; 0: +2) ou, vazio, os clubes que podem chegar a ele (0x5219b0 com a conta no global
/// 0xae1880). Devolve a conta (a caixa se nenhum). Rodada fora de 0..+0x3c, copa sem clubes (+0xa7) ou jogo −1: a
/// original (a caixa e 0).
unsafe extern "thiscall" fn get_fixture_side_teams(this: usize, rodada: i32, jogo: i32, lado: u32, saida: *mut i16) -> i32 {
    if rodada < 0 || rodada > cpu12_w(this + 0x3c) as i32 || cpu12_d(this + 0xa7) == 0 || jogo == -1 {
        return original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(%%I1%%)(this, rodada, jogo, lado, saida);
    }
    let coletar: unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32 = core::mem::transmute(0x5219b0usize);
    let conta = 0xae1880 as *mut i32;
    *conta = 0;
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let (c, r, j) = if lado as u8 != 0 { (cpu12_w(reg), reg + 0xe, reg + 0xa) } else { (cpu12_w(reg + 2), reg + 0x10, reg + 0xc) };
    if c != -1 {
        *saida = c;
        *conta = 1;
        return 1;
    }
    coletar(this, cpu12_w(r) as i32, cpu12_w(j) as i32, saida, conta);
    if *conta == 0 {
        show_error_at(0x9b55ac, 0x157f);
    }
    *conta
}

/// Comparador: embaralha com o sorteio e o rand, aloca e chama a tabela virtual: só a original.
unsafe extern "thiscall" fn comparar_draw_team_order(this: usize, clubes: i32) {
    // ponytail: sorteia e aloca (não roda duas vezes); a cópia se prova pela carreira (a grade de jogos de toda liga)
    original_fn::<unsafe extern "thiscall" fn(usize, i32)>(%%I0%%)(this, clubes)
}

/// Comparador: o esperado sai do núcleo sobre uma conta própria e uma saída própria antes da original; depois, a
/// devolução, a conta global e a saída têm de bater.
unsafe extern "thiscall" fn comparar_get_fixture_side_teams(this: usize, rodada: i32, jogo: i32, lado: u32, saida: *mut i16) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, u32, *mut i16) -> i32>(%%I1%%);
    if saida.is_null() || rodada < 0 || rodada > cpu12_w(this + 0x3c) as i32 || cpu12_d(this + 0xa7) == 0 || jogo == -1 {
        return o(this, rodada, jogo, lado, saida); // ponytail: a caixa, só a original
    }
    let reg = cpu12_d(cpu12_d(this + 0xad) + rodada as usize * 4) + (jogo as usize).wrapping_mul(0x35);
    let (c, r, j) = if lado as u8 != 0 { (cpu12_w(reg), reg + 0xe, reg + 0xa) } else { (cpu12_w(reg + 2), reg + 0x10, reg + 0xc) };
    let mut esperado = vec![0i16; 0x100];
    let mut n: i32 = 0;
    if c != -1 {
        esperado[0] = c;
        n = 1;
    } else {
        collect_fixture_teams(this, cpu12_w(r) as i32, cpu12_w(j) as i32, esperado.as_mut_ptr(), &mut n, false);
    }
    let res = o(this, rodada, jogo, lado, saida);
    let obtido = core::slice::from_raw_parts(saida, res.clamp(0, 0x100) as usize);
    let global = *(0xae1880 as *const i32);
    PLACAR.record(res == n && global == n && obtido == &esperado[..n.clamp(0, 0x100) as usize], || format!("get_fixture_side_teams({:x}, {}, {}, {}) {}/{} global {} {:?} x {:?}", this, rodada, jogo, lado as u8, res, n, global, obtido, &esperado[..n.clamp(0, 0x100) as usize]));
    res
}

/// Comparador: o esperado sai do núcleo sobre uma cópia da conta e uma saída própria antes da original; depois, a
/// devolução, a conta e os clubes acrescentados têm de bater.
unsafe extern "thiscall" fn comparar_collect_possible_teams(this: usize, rodada: i32, jogo: i32, saida: *mut i16, conta: *mut i32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32, i32, *mut i16, *mut i32) -> i32>(%%I2%%);
    if saida.is_null() || conta.is_null() || rodada < 0 || jogo == -1 || !(0..0x100).contains(&*conta) {
        return o(this, rodada, jogo, saida, conta); // ponytail: a caixa ou conta fora do esperado, só a original
    }
    let c0 = *conta;
    let mut esperado = vec![0i16; 0x200];
    let mut n = c0;
    collect_fixture_teams(this, rodada, jogo, esperado.as_mut_ptr(), &mut n, false);
    let res = o(this, rodada, jogo, saida, conta);
    let fim = (*conta).clamp(c0, 0x200);
    let obtido = core::slice::from_raw_parts(saida.offset(c0 as isize), (fim - c0) as usize);
    let ok = res == n && *conta == n && obtido == &esperado[c0 as usize..n.clamp(c0, 0x200) as usize];
    PLACAR.record(ok, || format!("collect_possible_teams({:x}, {}, {}) {}/{} conta {} {:?} x {:?}", this, rodada, jogo, res, n, *conta, obtido, &esperado[c0 as usize..n.clamp(c0, 0x200) as usize]));
    res
}
