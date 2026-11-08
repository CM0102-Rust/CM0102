// ---------- %%LEVA%%a leva: a busca de novas aposentadorias, a saída dos cargos de quem se aposenta e o registro da liberação (player_regen.cpp, staff_records.cpp) ----------

unsafe fn cpr9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpr9_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn zero_dword(a: usize) {
    core::ptr::write_unaligned(a as *mut u32, 0);
}

/// { 0x7a5790 } (thiscall, player_regen.cpp, SEH só das datas locais, de destrutor vazio):
/// `PLAYER_REGEN::find_new_retirements` (nos dias da 0x5d6250): cada pessoa (0x6e bytes em [0xae23bc], até o fim
/// calculado com a conta [0xae2364] do começo). Ficam de fora: menos de 30 anos (+0x18) em clube de competição que roda
/// no jogo ([[0xadadfc] + id × 4] da competição +0x57); id a partir de [0xae2364] − 16 (relida a cada passo); os bits
/// 0x30 do +0xb do registro de 79 bytes ([0xae2d0c]); quem já está na lista ([this+4], 26 bytes, conta +8); contrato
/// (0x4cf5a0) com +0x49. Os outros: aposentadoria pela idade (0x7a7310) ou, sem ela, do desempregado (0x7a8050); quem
/// se aposenta entra na lista (0x7a8510). No fim o qsort da lista (0x7ae2a0).
unsafe extern "thiscall" fn find_new_retirements(this: usize) {
    // ponytail: a original constrói as duas datas da entrada (0x54a110) e deixa o resto como estava na pilha; as duas
    // decisões gravam tudo o que a 0x7a8510 copia antes de devolver 1 (a terceira data local nunca é lida: fica de fora)
    let mut e = [0u8; 0x1a];
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    padrao(e.as_mut_ptr().add(8));
    padrao(e.as_mut_ptr().add(0x10));
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let pela_idade: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u8 = core::mem::transmute(0x7a7310usize);
    let desempregado: unsafe extern "stdcall" fn(usize, *mut u8) -> u8 = core::mem::transmute(0x7a8050usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8) = core::mem::transmute(0x7a8510usize);
    let mut p = cpr9_d(0xae23bc);
    let fim = p.wrapping_add(cpr9_d(0xae2364).wrapping_mul(0x6e));
    while p < fim {
        'x: {
            if cpr9_b(p + 0x18) < 0x1e {
                let c = cpr9_d(p + 0x39);
                if c != 0 {
                    let lg = cpr9_d(c + 0x57);
                    if lg != 0 && cpr9_d(cpr9_d(0xadadfc).wrapping_add(cpr9_d(lg).wrapping_mul(4))) != 0 {
                        break 'x;
                    }
                }
            }
            let id = cpr9_d(p) as i32;
            if id >= (cpr9_d(0xae2364) as i32).wrapping_sub(0x10) {
                break 'x;
            }
            if *((cpr9_d(0xae2d0c).wrapping_add((id as usize).wrapping_mul(79)) + 0xb) as *const u8) & 0x30 != 0 {
                break 'x;
            }
            let k = cpr9_d(this + 8) as i32;
            if k != 0 {
                let l = cpr9_d(this + 4);
                for i in 0..k.max(0) as usize {
                    if cpr9_d(l + i * 0x1a) as i32 == id {
                        break 'x;
                    }
                }
            }
            let c = de(cpr9_d(0xae1050), p);
            if c != 0 && *((c + 0x49) as *const u8) != 0 {
                break 'x;
            }
            if pela_idade(this, p, e.as_mut_ptr()) != 0 {
                acrescentar(this, e.as_ptr());
            } else if desempregado(p, e.as_mut_ptr()) != 0 {
                crate::comparison::branch("cpr_9 desempregado");
                acrescentar(this, e.as_ptr());
            }
        }
        p = p.wrapping_add(0x6e);
    }
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpr9_d(this + 4), cpr9_d(this + 8) as u32, 0x1a, 0x7ae2a0);
}

/// Tira a pessoa da vaga do `cargo` (tipo com sinal) no clube `c` (o da pessoa em +`campo`: 0x39 o clube, 0x24 a
/// seleção): 1 presidente (+0xbf); 2 as três de +0xc3; 5 e 12 técnico (+0xcf, só se é ele): no 5, sendo humano (id a
/// partir de [0xae2364] − 16), as ofertas do clube caem (0x8c4950); sai do cargo (0x69b790 com 3, 0, 0) e, se o clube
/// pode ser dirigido (0x544820), a vaga abre (0x69ba50); 6 auxiliar (+0xd3); 8 e 15 as cinco de +0x19f, depois da
/// distribuição dos treinadores do clube humano (0x600a50 com 1, 1; 0x8b2880 com 0); 9 as sete de +0x1b3 e, no
/// clube humano, +`campo` = 0 e as listas dos olheiros do técnico (0x7f7270); 10 as três de +0x1cf.
unsafe fn leave_job_slot(pessoa: usize, c: usize, cargo: i8, campo: usize) {
    let limpar = |base: usize, n: usize| {
        for k in 0..n {
            let a = c + base + 4 * k;
            if cpr9_d(a) == pessoa {
                zero_dword(a);
            }
        }
    };
    let humano = || {
        let f: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
        f(c, 1, 1, 0, 0) != 0
    };
    match cargo {
        1 => zero_dword(c + 0xbf),
        2 => limpar(0xc3, 3),
        5 | 12 => {
            if cpr9_d(c + 0xcf) != pessoa {
                return;
            }
            crate::comparison::branch("cpr_9 tecnico");
            if cargo == 5 && (cpr9_d(pessoa) as i32) >= (cpr9_d(0xae2364) as i32).wrapping_sub(0x10) {
                let cancelar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c4950usize);
                cancelar(cpr9_d(0xde1f54), c);
            }
            let sair: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x69b790usize);
            sair(cpr9_d(0xb67a44), c, 3, 0, 0);
            let pode: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x544820usize);
            if pode(c) as u8 != 0 {
                let abrir: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x69ba50usize);
                abrir(cpr9_d(0xb67a44), c, pessoa); // cpr9: a vaga do técnico abre
            }
        }
        6 => zero_dword(c + 0xd3),
        8 | 15 => {
            if humano() {
                let treinadores: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8b2880usize);
                treinadores(cpr9_d(0xde1f44), c, 0);
            }
            crate::comparison::branch("cpr_9 treinador");
            limpar(0x19f, 5); // cpr9: as vagas de treinador
        }
        9 => {
            limpar(0x1b3, 7);
            if humano() {
                zero_dword(pessoa + campo);
                let listas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7f7270usize);
                listas(cpr9_d(0xb74448), cpr9_d(c + 0xcf));
            }
        }
        10 => limpar(0x1cf, 3),
        _ => {}
    }
}

/// { 0x7aaeb0 } (stdcall, ret 4, player_regen.cpp): `player_regen_retire_from_non_playing_staff(pessoa)` (pela
/// 0x7a59e0, na aposentadoria): sai da vaga do cargo (+0x3d) no clube (+0x39) e da do cargo na seleção (+0x28) na
/// seleção (+0x24) (leave_job_slot); clube e seleção zerados, e os dois tipos de cargo.
unsafe extern "stdcall" fn player_regen_retire_from_non_playing_staff(pessoa: usize) {
    let c = cpr9_d(pessoa + 0x39);
    if c != 0 {
        leave_job_slot(pessoa, c, cpr9_b(pessoa + 0x3d), 0x39);
        zero_dword(pessoa + 0x39);
    }
    let s = cpr9_d(pessoa + 0x24);
    if s != 0 {
        leave_job_slot(pessoa, s, cpr9_b(pessoa + 0x28), 0x24);
        zero_dword(pessoa + 0x24);
    }
    *((pessoa + 0x3d) as *mut u8) = 0;
    *((pessoa + 0x28) as *mut u8) = 0;
}

/// { 0x86d800 } (thiscall, ret 4, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_free_release_records(pessoa)`: o contrato da pessoa (0x4cf5a0) vai para [0xae1080] e dá o clube
/// (registro de 0x245 bytes pelo +4). Com técnico no clube (+0xcf), o histórico dele (tipo 0x68: o clube, a pessoa e os
/// três ids +0x33 de +4, +8 e +0xc dela); depois o da pessoa (tipo 0x67: o clube ou −1), no arquivo (0x58e340).
/// Pessoa nula: a caixa "Error" (linha 0x1010).
unsafe extern "thiscall" fn add_free_release_records(this: usize, pessoa: usize) {
    crate::comparison::branch("cpr_9 liberado");
    let mut e = history_entry();
    let de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = de(cpr9_d(0xae1050), pessoa);
    *(0xae1080 as *mut usize) = c;
    let clube = if c != 0 { cpr9_d(0xae23b4).wrapping_add(cpr9_d(c + 4).wrapping_mul(0x245)) } else { 0 };
    if clube != 0 && cpr9_d(clube + 0xcf) != 0 {
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x68;
        put_i32(&mut e, 8, cpq4_i(clube));
        put_person_keys(&mut e, 0xd, pessoa);
        append_person_history_by_id(true, this, cpr9_d(cpr9_d(clube + 0xcf)), &e);
    }
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x67;
    put_i32(&mut e, 8, staff_records_id_or_minus_one(clube));
    if pessoa != 0 {
        append_person_history_by_id(true, this, cpr9_d(pessoa), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
}

// ponytail: a busca acrescenta (realloc) e sorteia em cada teste: não roda duas vezes. Só a original; o modo Rust e a
// carreira provam a cópia.
unsafe extern "thiscall" fn comparar_find_new_retirements(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// A saída dos cargos só mexe na pessoa e nos dois clubes? Não quando é técnico do clube (5, 12) nem nos cargos que
/// chamam rotinas do clube humano (8, 15, 9).
unsafe fn job_exit_is_memory_only(pessoa: usize) -> bool {
    for (campo, tipo) in [(0x39usize, 0x3dusize), (0x24, 0x28)] {
        let c = cpr9_d(pessoa + campo);
        if c == 0 {
            continue;
        }
        let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
        match cpr9_b(pessoa + tipo) {
            5 | 12 if cpr9_d(c + 0xcf) == pessoa => return false,
            8 | 15 | 9 if humano(c, 1, 1, 0, 0) != 0 => return false,
            _ => {}
        }
    }
    true
}

/// Comparador: sem rotinas com efeito (job_exit_is_memory_only), lado a lado pela pessoa (0x6e bytes) e pelos dois clubes
/// (0x245 bytes cada); senão (vaga do técnico, cancelamento de ofertas, listas do humano) só a original.
unsafe extern "stdcall" fn comparar_player_regen_retire_from_non_playing_staff(pessoa: usize) {
    let o = original_fn::<unsafe extern "stdcall" fn(usize)>(%%I1%%);
    if !job_exit_is_memory_only(pessoa) {
        return o(pessoa);
    }
    let mut reg = job_exit_regions(pessoa);
    reg.retain(|r| r.0 != 0);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(pessoa), || player_regen_retire_from_non_playing_staff(pessoa));
    SCOREBOARD.record(igual, || format!("player_regen_retire_from_non_playing_staff(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

unsafe fn job_exit_regions(pessoa: usize) -> Vec<(usize, usize)> {
    vec![(pessoa, 0x6e), (cpr9_d(pessoa + 0x39), 0x245), (cpr9_d(pessoa + 0x24), 0x245)]
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia sem ele só grava [0xae1080]: só a
// original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_add_free_release_records(this: usize, pessoa: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, pessoa)
}
