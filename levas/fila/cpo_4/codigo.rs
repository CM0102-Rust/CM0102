// ---------- %%LEVA%%a leva: as saídas dos clubes: as saídas pendentes dos técnicos, tirar o jogador do elenco e tirar a pessoa do cargo (manager_manager.cpp) ----------

unsafe fn cpo4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpo4_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O registro do clube pelo id (0x245 bytes em [0xae23b4]).
unsafe fn cpo4_clube(id: usize) -> usize {
    cpo4_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// { 0x69b650 } (thiscall, ret 0, manager_manager.cpp): `MANAGER_MANAGER::process_pending_departures()`: percorre a lista
/// das saídas pendentes (+4, itens de 6 bytes: dois bytes e o id do clube em +2) até a conta (que cai a cada retirada:
/// a original passa só por parte dos itens) e, para cada item, com técnico no clube (+0xcf) chama a saída (0x69b790
/// com os dois bytes empurrados nos registradores inteiros: os bits de cima são os do id e de 145 × id) e, se o clube
/// abre vaga (0x544820), a vaga (0x69ba50 com o técnico que saiu); tira o item (0x68d1f0). Item nulo: a caixa "Error".
unsafe extern "thiscall" fn process_pending_departures(this: usize) {
    let lista = cpo4_d(this + 4);
    if lista == 0 {
        return;
    }
    cpo4_por(lista + 0xa, cpo4_d(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let remover: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    let sair: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x69b790usize);
    let abre_vaga: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x544820usize);
    let vaga: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x69ba50usize);
    let mut k = 0i32;
    while k < cpo4_w(cpo4_d(this + 4)) as i32 {
        let item = proximo(cpo4_d(this + 4));
        if item == 0 {
            show_error_at(0x9f18a4, 0x2b70);
        } else {
            let id = cpo4_d(item + 2) as u32;
            let clube = cpo4_clube(id as usize);
            let tecnico = cpo4_d(clube + 0xcf);
            if tecnico != 0 {
                let a = (id & 0xffff_ff00) | cpo4_b(item) as u32;
                let b = (id.wrapping_mul(145) & 0xffff_ff00) | cpo4_b(item + 1) as u32;
                sair(this, clube, a, b, 0);
                if abre_vaga(clube) as u8 != 0 {
                    crate::comparison::branch("cpo_4 abrir a vaga");
                    vaga(this, clube, tecnico); // a vaga do clube
                }
            }
            remover(cpo4_d(this + 4), item);
        }
        k += 1;
    }
}

/// A função sem vaga no elenco depois de sair (12 → 5, 13 → 6, 15 → 8: a pessoa fica no clube) ou 0xb (sai).
fn role_after_leaving(f: u8) -> (u8, bool) {
    match f {
        0xc => (5, true),
        0xd => (6, true),
        0xf => (8, true),
        _ => (0xb, false),
    }
}

/// Tira a pessoa da primeira das 50 vagas do elenco do clube (+0xd7) em que ela está.
unsafe fn remove_from_squad(clube: usize, pessoa: usize) {
    for k in 0..0x32usize {
        let v = clube + 0xd7 + 4 * k;
        if cpo4_d(v) == pessoa {
            crate::comparison::branch("cpo_4 tirar do elenco");
            cpo4_por(v, 0); // a vaga do elenco
            return;
        }
    }
}

/// { 0x690730 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::remove_player_from_squad(pessoa, clube)`: no
/// clube da pessoa (+0x39): a função (+0x3d) de jogador-técnico volta à de técnico (12 → 5, 13 → 6, 15 → 8) ou vira
/// 0xb e ela sai do clube; tira das listas (0x7c5fe0), da mídia (0x843e10 com 4, 2), das duas táticas do clube (0x896c60)
/// e das 50 vagas do elenco (+0xd7). Na seleção (+0x24, função +0x28) o mesmo sem a 0x7c5fe0 e a 0x843e10. No fim o
/// contrato do empréstimo (0x4cf5a0) fica com o estado +0x4e = 4. Devolve o da 0x4cf5a0.
unsafe extern "thiscall" fn manager_manager_remove_player_from_squad(_this: usize, pessoa: usize, clube: usize) -> usize {
    let tatica: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x896c60usize);
    if cpo4_d(pessoa + 0x39) == clube {
        let (f, fica) = role_after_leaving(cpo4_b(pessoa + 0x3d));
        *((pessoa + 0x3d) as *mut u8) = f;
        let listas: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, i32, u32) = core::mem::transmute(0x7c5fe0usize);
        listas(cpo4_d(0xb74350), pessoa, -1, 0, 0, -1, 0);
        let midia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x843e10usize);
        midia(cpo4_d(cpo4_d(0xde1f54) + 0x856), pessoa, clube, 4, 2);
        tatica(cpo4_d(0xde1d08), clube, pessoa, 0, 1);
        tatica(cpo4_d(0xde1d08), clube, pessoa, 1, 1);
        remove_from_squad(clube, pessoa);
        if !fica {
            cpo4_por(pessoa + 0x39, 0);
        }
    }
    if cpo4_d(pessoa + 0x24) == clube {
        let (f, fica) = role_after_leaving(cpo4_b(pessoa + 0x28));
        *((pessoa + 0x28) as *mut u8) = f;
        tatica(cpo4_d(0xde1d08), clube, pessoa, 0, 1);
        tatica(cpo4_d(0xde1d08), clube, pessoa, 1, 1);
        remove_from_squad(clube, pessoa);
        if !fica {
            cpo4_por(pessoa + 0x24, 0);
        }
    }
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let r = emprestimo(cpo4_d(0xae1050), pessoa);
    if r != 0 {
        *((r + 0x4e) as *mut u8) = 4;
    }
    r
}

/// Zera as `n` vagas de `c + base` que guardam a pessoa (todas).
unsafe fn clear_slots_holding(c: usize, base: usize, n: usize, pessoa: usize) {
    for k in 0..n {
        if cpo4_d(c + base + 4 * k) == pessoa {
            cpo4_por(c + base + 4 * k, 0);
        }
    }
}

/// O corpo da 0x690890 sem as chamadas: a pessoa `pessoa` (campos em `s`) sai do cargo no clube `clube` (campos em `c`).
/// `s` e `c` são os próprios registros na cópia e as cópias deles no plano do comparador. Pela função no clube (+0x3d,
/// tabela 0x690af8): 1 presidente (+0xbf), 2 diretores (+0xc3, 3), 5 técnico (+0xcf, se for ele), 6 auxiliar (+0xd3),
/// 8 treinadores (+0x19f, 5), 9 olheiros (+0x1b3, 7), 10 fisioterapeutas (+0x1cf, 3): a vaga zerada e +0x39 = 0;
/// 12, 13, 15 (jogador-técnico, jogador-auxiliar, jogador-treinador): a vaga zerada e a função 0xb (fica no clube);
/// outras: +0x39 = 0 só se o byte baixo do ponteiro da pessoa é 0 (o `[esp+0x10]` da original). Na seleção (+0x24,
/// função +0x28, tabela 0x690b34) o mesmo com +0x24, sem a conferência do técnico, e as outras funções saem.
unsafe fn detach_staff_from_club(pessoa: usize, s: usize, clube: usize, c: usize) {
    if cpo4_d(s + 0x39) == clube {
        match (cpo4_b(s + 0x3d) as i8 as i32).wrapping_sub(1) as u32 {
            0 => {
                cpo4_por(c + 0xbf, 0);
                cpo4_por(s + 0x39, 0);
            }
            1 => {
                clear_slots_holding(c, 0xc3, 3, pessoa);
                cpo4_por(s + 0x39, 0);
            }
            4 => {
                if cpo4_d(c + 0xcf) == pessoa {
                    crate::comparison::branch("cpo_4 tirar o tecnico");
                    cpo4_por(c + 0xcf, 0); // o técnico do clube
                }
                cpo4_por(s + 0x39, 0);
            }
            5 => {
                cpo4_por(c + 0xd3, 0);
                cpo4_por(s + 0x39, 0);
            }
            7 => {
                clear_slots_holding(c, 0x19f, 5, pessoa);
                cpo4_por(s + 0x39, 0);
            }
            8 => {
                clear_slots_holding(c, 0x1b3, 7, pessoa);
                cpo4_por(s + 0x39, 0);
            }
            9 => {
                clear_slots_holding(c, 0x1cf, 3, pessoa);
                cpo4_por(s + 0x39, 0);
            }
            11 => {
                *((s + 0x3d) as *mut u8) = 0xb;
                if cpo4_d(c + 0xcf) == pessoa {
                    cpo4_por(c + 0xcf, 0);
                }
            }
            12 => {
                *((s + 0x3d) as *mut u8) = 0xb;
                cpo4_por(c + 0xd3, 0);
            }
            14 => {
                *((s + 0x3d) as *mut u8) = 0xb;
                clear_slots_holding(c, 0x19f, 5, pessoa);
            }
            _ => {
                if pessoa as u8 == 0 {
                    cpo4_por(s + 0x39, 0);
                }
            }
        }
    } else if cpo4_d(s + 0x24) == clube {
        match (cpo4_b(s + 0x28) as i8 as i32).wrapping_sub(1) as u32 {
            0 => cpo4_por(c + 0xbf, 0),
            1 => clear_slots_holding(c, 0xc3, 3, pessoa),
            4 => cpo4_por(c + 0xcf, 0),
            5 => cpo4_por(c + 0xd3, 0),
            7 => clear_slots_holding(c, 0x19f, 5, pessoa),
            8 => clear_slots_holding(c, 0x1b3, 7, pessoa),
            9 => clear_slots_holding(c, 0x1cf, 3, pessoa),
            11 => {
                cpo4_por(c + 0xcf, 0);
                *((s + 0x28) as *mut u8) = 0xb;
                return;
            }
            12 => {
                *((s + 0x28) as *mut u8) = 0xb;
                cpo4_por(c + 0xd3, 0);
                return;
            }
            14 => {
                *((s + 0x28) as *mut u8) = 0xb;
                clear_slots_holding(c, 0x19f, 5, pessoa);
                return;
            }
            _ => {}
        }
        cpo4_por(s + 0x24, 0);
    }
}

/// { 0x690890 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::remove_staff_from_club(pessoa, clube)`: tira o
/// contrato (0x4d0b50 em [0xae1050]), tira a pessoa do cargo no clube ou na seleção (`detach_staff_from_club`), da mídia
/// (0x843e10 com 4, 2) e, se ela é humana (entre as 16 últimas pessoas), das listas do humano (0x7f7270 em
/// [0xb74448]) e do clube (0x8b2470 em [0xde1f44] com 0). Devolve o eax da última chamada.
unsafe extern "thiscall" fn remove_staff_from_club(_this: usize, pessoa: usize, clube: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d0b50usize);
    contrato(cpo4_d(0xae1050), pessoa);
    detach_staff_from_club(pessoa, pessoa, clube, clube);
    let midia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x843e10usize);
    let mut r = midia(cpo4_d(cpo4_d(0xde1f54) + 0x856), pessoa, clube, 4, 2);
    if cpo4_i(pessoa) >= cpo4_i(0xae2364).wrapping_sub(0x10) {
        let humano: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7f7270usize);
        humano(cpo4_d(0xb74448), pessoa);
        let lista: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> usize = core::mem::transmute(0x8b2470usize);
        r = lista(cpo4_d(0xde1f44), pessoa, clube, 0);
    }
    r
}

/// Comparador: a saída e a vaga (0x69b790, 0x69ba50) demitem e contratam: roda só a original.
// ponytail: sem plano (a 0x69ba50 pode acrescentar saídas na mesma lista); a prova é a carreira e o modo Rust.
unsafe extern "thiscall" fn compare_process_pending_departures(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// Os campos da pessoa e do clube que a 0x690730/0x690890 gravam: (pessoa +0x24..+0x29, +0x39..+0x3e) e as vagas do
/// clube (+0xbf..+0x1db).
unsafe fn staff_job_fields(pessoa: usize, clube: usize) -> ([u8; 6], [u8; 5], [u8; 0x11c]) {
    let mut a = [0u8; 6];
    let mut b = [0u8; 5];
    let mut c = [0u8; 0x11c];
    core::ptr::copy_nonoverlapping((pessoa + 0x24) as *const u8, a.as_mut_ptr(), 6);
    core::ptr::copy_nonoverlapping((pessoa + 0x39) as *const u8, b.as_mut_ptr(), 5);
    core::ptr::copy_nonoverlapping((clube + 0xbf) as *const u8, c.as_mut_ptr(), 0x11c);
    (a, b, c)
}

/// Comparador: a original tira a pessoa das listas, da mídia e das táticas (não pode rodar duas vezes): roda só ela e
/// confere o plano: a função e o clube (e a seleção) da pessoa e ela fora das 50 vagas do elenco.
unsafe extern "thiscall" fn compare_manager_manager_remove_player_from_squad(this: usize, pessoa: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I1%%);
    let no_clube = cpo4_d(pessoa + 0x39) == clube;
    let na_selecao = cpo4_d(pessoa + 0x24) == clube;
    let (mut f39, mut c39) = (cpo4_b(pessoa + 0x3d), cpo4_d(pessoa + 0x39));
    let (mut f28, mut c24) = (cpo4_b(pessoa + 0x28), cpo4_d(pessoa + 0x24));
    if no_clube {
        let (f, fica) = role_after_leaving(f39);
        f39 = f;
        if !fica {
            c39 = 0;
        }
    }
    if c24 == clube {
        let (f, fica) = role_after_leaving(f28);
        f28 = f;
        if !fica {
            c24 = 0;
        }
    }
    let r = o(this, pessoa, clube);
    let fora = !(no_clube || na_selecao) || (0..0x32usize).all(|k| cpo4_d(clube + 0xd7 + 4 * k) != pessoa);
    let ok = fora && cpo4_b(pessoa + 0x3d) == f39 && cpo4_d(pessoa + 0x39) == c39 && cpo4_b(pessoa + 0x28) == f28 && cpo4_d(pessoa + 0x24) == c24;
    SCOREBOARD.record(ok, || format!("manager_manager_remove_player_from_squad(pessoa {:x}, clube {:x}): função {:x}/{:x} clube {:x}/{:x} (esperava {:x}/{:x} {:x}/{:x}), fora das vagas {}",
        cpo4_d(pessoa), cpo4_d(clube), cpo4_b(pessoa + 0x3d), cpo4_b(pessoa + 0x28), cpo4_d(pessoa + 0x39), cpo4_d(pessoa + 0x24), f39, f28, c39, c24, fora));
    r
}

/// Comparador: a original tira o contrato e tira a pessoa da mídia e das listas do humano (não pode rodar duas vezes):
/// roda só ela e confere o plano feito antes com o corpo (`detach_staff_from_club`) sobre cópias dos campos da pessoa e das vagas
/// do clube. Pessoa humana: só a original (as listas do humano podem mexer no clube).
unsafe extern "thiscall" fn compare_remove_staff_from_club(this: usize, pessoa: usize, clube: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I2%%);
    if pessoa == 0 || clube == 0 || cpo4_i(pessoa) >= cpo4_i(0xae2364).wrapping_sub(0x10) {
        return o(this, pessoa, clube);
    }
    // o plano: os campos em cópias com os mesmos deslocamentos
    let mut s = [0u8; 0x40];
    let mut c = [0u8; 0x1e0];
    core::ptr::copy_nonoverlapping(pessoa as *const u8, s.as_mut_ptr(), 0x40);
    core::ptr::copy_nonoverlapping(clube as *const u8, c.as_mut_ptr(), 0x1e0);
    detach_staff_from_club(pessoa, s.as_mut_ptr() as usize, clube, c.as_mut_ptr() as usize);
    let esperado = staff_job_fields(s.as_ptr() as usize, c.as_ptr() as usize);
    let r = o(this, pessoa, clube);
    let depois = staff_job_fields(pessoa, clube);
    SCOREBOARD.record(depois == esperado, || format!("remove_staff_from_club(pessoa {:x}, função {}, clube {:x}): pessoa {:x?}{:x?} esperava {:x?}{:x?}",
        cpo4_d(pessoa), cpo4_b(pessoa + 0x3d) as i8, cpo4_d(clube), depois.0, depois.1, esperado.0, esperado.1));
    r
}
