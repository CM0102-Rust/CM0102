// ---------- %%LEVA%%a leva: o lugar do jogador no elenco e as subidas e descidas entre o principal e os reservas (contract_manager.cpp) ----------

unsafe fn cph2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// O contrato da pessoa como a 0x4cbb60 acha por dentro (sem chamar a 0x4cf5a0): pessoa não nula, id abaixo de
/// [0xae2364], índice da tabela [0xae2d0c] (79 bytes por pessoa) não negativo; registro de 80 bytes em [objeto].
unsafe fn squad_contract_of_staff(objeto: usize, pessoa: usize) -> usize {
    if pessoa == 0 {
        return 0;
    }
    let id = cph2_d(pessoa) as i32;
    if id >= cph2_d(0xae2364) as i32 {
        return 0;
    }
    let i = cph2_d((cph2_d(0xae2d0c) as usize).wrapping_add((id as usize).wrapping_mul(79))) as i32;
    if i < 0 {
        return 0;
    }
    (cph2_d(objeto) as usize).wrapping_add((i as usize).wrapping_mul(80))
}

/// { 0x4cbb60 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_squad_placement(pessoa, clube)`: o
/// lugar da pessoa no `clube`: 0 nenhum, 1 reservas, 2 principal, 3 no time B ligado, 4 no clube principal com time B.
/// Só para os cargos de jogador (+0x3d diferente de 12 e 0x53b620). No próprio clube (+0x39) que pode ter reservas
/// (0x542740): 2 com o bit 0 de +0x5e, senão 1. O clube parceiro (0x540a50 com 1; o byte de saída é a vaga da
/// pessoa, que fica com o ponteiro dela se a 0x540a50 não gravar): sendo A e `clube` o próprio, 4; sendo B e `clube` o
/// parceiro A, 3. Senão pelo contrato (0x4cbb60 acha por dentro em [0xae1050], gravado em [0xae1080]): o clube do
/// contrato, que não é o próprio nem o parceiro já visto; com o bit 6 de +0x35 do contrato (achado de novo em [this]):
/// no `clube` com reservas, 1 com o bit e 2 sem; ou com o parceiro (0x540a50) sendo o `clube` (ou o do contrato o
/// `clube`), 3 com o bit e 4 sem. O jogo lê só o al.
unsafe extern "thiscall" fn get_squad_placement(this: usize, pessoa: usize, clube: usize) -> u8 {
    let tipo = *((pessoa + 0x3d) as *const i8);
    let tipo_de_jogador: unsafe extern "C" fn(i8) -> u32 = core::mem::transmute(0x53b620usize);
    if tipo == 12 || tipo_de_jogador(tipo) == 0 {
        return 0;
    }
    let reservas: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x542740usize);
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let proprio = cph2_d(pessoa + 0x39) as usize;
    if proprio != 0 && proprio == clube && tipo_de_jogador(*((pessoa + 0x3d) as *const i8)) != 0 && reservas(clube) != 0 {
        return if *((pessoa + 0x5e) as *const u8) & 1 != 0 { 2 } else { 1 }; // principal ou reservas
    }
    let mut vaga = pessoa as u32; // a vaga do argumento: a 0x540a50 pode não gravar
    let mut b = 0usize;
    if proprio != 0 {
        b = parceiro(proprio, &mut vaga, 1);
        if b != 0 {
            if vaga != 0 {
                if clube == cph2_d(pessoa + 0x39) as usize {
                    return 4;
                }
            } else if clube == b {
                return 3;
            }
        }
    }
    let c = squad_contract_of_staff(cph2_d(0xae1050) as usize, pessoa);
    *(0xae1080 as *mut usize) = c; // a ficha do contrato
    if c == 0 {
        return 0;
    }
    let i = cph2_d(c + 4) as usize;
    let outro = (cph2_d(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
    if outro == 0 || outro == cph2_d(pessoa + 0x39) as usize || outro == b {
        return 0;
    }
    let c2 = squad_contract_of_staff(this, pessoa);
    let marca = (*((c2 + 0x35) as *const u8) << 1) as i8 >> 7; // o bit 6, estendido
    if outro == clube && reservas(clube) != 0 {
        return if marca == 0 { 2 } else { 1 };
    }
    let p = parceiro(outro, &mut vaga, 1);
    if p == 0 || (p != clube && outro != clube) {
        return 0;
    }
    if marca == 0 { 4 } else { 3 }
}

/// A 0x4cbb60 pelo endereço (passa pelo desvio) e a troca 0x4cbd20 (com 0 na saída do novo clube).
unsafe fn toggle_placement_if_in(this: usize, pessoa: usize, clube: usize, de: [u8; 2]) -> u32 {
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4cbb60usize);
    let l = lugar(this, pessoa, clube);
    if l != de[0] && l != de[1] {
        return 0;
    }
    let trocar: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x4cbd20usize);
    trocar(this, pessoa, clube, 0)
}

/// { 0x4cc0b0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::move_to_first_team(pessoa, clube)`: nos
/// reservas (1) ou no time B (3), troca (0x4cbd20) e devolve o que ela devolve; senão 0.
unsafe extern "thiscall" fn move_to_first_team(this: usize, pessoa: usize, clube: usize) -> u32 {
    toggle_placement_if_in(this, pessoa, clube, [1, 3]) // reservas ou time B
}

/// { 0x4cc0f0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::move_to_reserves(pessoa, clube)`: no
/// principal (2) ou no clube com time B (4), troca (0x4cbd20) e devolve o que ela devolve; senão 0.
unsafe extern "thiscall" fn move_to_reserves(this: usize, pessoa: usize, clube: usize) -> u32 {
    toggle_placement_if_in(this, pessoa, clube, [2, 4]) // principal ou clube com time B
}

/// Comparador: lado a lado pela ficha [0xae1080] (a vaga da saída é o argumento na pilha de cada uma).
unsafe extern "thiscall" fn compare_get_squad_placement(this: usize, pessoa: usize, clube: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I0%%);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae1080, 4)], || o(this, pessoa, clube), || get_squad_placement(this, pessoa, clube));
    SCOREBOARD.record(igual && r == n, || format!("get_squad_placement({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// Comparador das duas trocas: o lugar (0x4cbb60, pelo desvio) decide antes; fora dos lugares da troca, lado a lado
/// pela ficha [0xae1080] (as duas devolvem 0); nos lugares da troca, só a original (a 0x4cbd20 mexe no elenco).
unsafe fn compare_placement_toggle(i: usize, nome: &str, this: usize, pessoa: usize, clube: usize, de: [u8; 2]) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(i);
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4cbb60usize);
    let ficha = cph2_d(0xae1080);
    let l = lugar(this, pessoa, clube);
    *(0xae1080 as *mut u32) = ficha;
    if l == de[0] || l == de[1] {
        return o(this, pessoa, clube);
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae1080, 4)], || o(this, pessoa, clube), || toggle_placement_if_in(this, pessoa, clube, de));
    SCOREBOARD.record(igual && r == n, || format!("{}({:x}, {:x}) lugar {} original={} rust={} {:x?} x {:x?}", nome, pessoa, clube, l, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn compare_move_to_first_team(this: usize, pessoa: usize, clube: usize) -> u32 {
    compare_placement_toggle(%%I1%%, "move_to_first_team", this, pessoa, clube, [1, 3])
}

unsafe extern "thiscall" fn compare_move_to_reserves(this: usize, pessoa: usize, clube: usize) -> u32 {
    compare_placement_toggle(%%I2%%, "move_to_reserves", this, pessoa, clube, [2, 4])
}
