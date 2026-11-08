// ---------- %%LEVA%%a leva: os empréstimos e o lugar no elenco: assinar o contrato de empréstimo, encerrar o empréstimo e trocar o lugar no elenco (contract_manager.cpp) ----------

unsafe fn cpp2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn read_byte_volatile(a: usize) -> u8 {
    // ponytail: leitura volátil porque a original lê [0 + 0x35] quando o contrato principal falta (cai como ela)
    core::ptr::read_volatile(a as *const u8)
}

unsafe fn cpp2_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// O registro de 79 bytes da pessoa em [0xae2d0c] (+0 o contrato principal, +4 o de empréstimo; −1 sem).
unsafe fn staff_state_entry(pessoa: usize) -> usize {
    (cpp2_d(0xae2d0c) as usize).wrapping_add((cpp2_d(pessoa) as usize).wrapping_mul(0x4f))
}

/// A entrada `k` (0x50 bytes) do vetor de contratos que começa em `vetor`.
fn contract_at_index(vetor: u32, k: i32) -> usize {
    (vetor as usize).wrapping_add((k as usize).wrapping_mul(0x50))
}

/// O clube de id `id` (registro de 0x245 bytes em [0xae23b4]).
unsafe fn contract_manager_club_by_id(id: u32) -> usize {
    (cpp2_d(0xae23b4) as usize).wrapping_add((id as usize).wrapping_mul(0x245))
}

/// O contrato principal da pessoa no CONTRACT_MANAGER global ([[0xae1050]]), como as originais o refazem: pessoa fora
/// da faixa (id ≥ [0xae2364]) ou sem contrato principal: 0. Grava em [0xae1080] (a ficha global).
unsafe fn main_contract_lookup(pessoa: usize) -> usize {
    let c = if cpp2_d(pessoa) as i32 >= cpp2_d(0xae2364) as i32 {
        0
    } else {
        let k = cpp2_d(staff_state_entry(pessoa)) as i32;
        if k < 0 { 0 } else { contract_at_index(cpp2_d(cpp2_d(0xae1050) as usize), k) }
    };
    cpp2_por(0xae1080, c as u32);
    c
}

/// A caixa "Error" do contract_manager.cpp na `linha`: "v%s %s.%s %d" em 0xae24d0 (0x944cff separa o caminho 0x9a8190,
/// 0x90d130 a versão; a original não desempilha os argumentos dela, que são o resto do formato), a caixa 0x5e8290 e
/// [0xb67a34] = 0. O jogo segue.
unsafe fn show_contract_error_box(linha: u32) {
    let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
    let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
    let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
    separar(0x9a8190, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
    let v = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), linha);
    formatar(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), linha);
    caixa(0x9870e0, 0xae24d0, 0);
    cpp2_por(0xb67a34, 0);
}

// As rotinas do banco que as três chamam (pelo endereço; as portadas passam pelo desvio).
unsafe fn contract_manager_linked_b_team(clube: usize, eh_a: *mut u32) -> usize {
    let f: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    f(clube, eh_a, 1)
}
unsafe fn set_squad_flags(pessoa: usize, elenco: u32) {
    let f: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x540980usize);
    f(pessoa, elenco);
}
unsafe fn remove_from_tactics_selection(clube: usize, pessoa: usize, a: u32, b: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x896c60usize);
    f(cpp2_d(0xde1d08) as usize, clube, pessoa, a, b);
}
unsafe fn contract_manager_archive_player_stats(pessoa: usize, a4: u32, clube: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, i32, u32) = core::mem::transmute(0x7c5fe0usize);
    f(cpp2_d(0xb74350) as usize, pessoa, -1, 0, a4, clube, 0);
}
unsafe fn contract_manager_remove_staff_from_club(clube: usize, pessoa: usize, data: usize, desligar: u32, como_jogador: u32) {
    let f: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53c190usize);
    f(clube, pessoa, data, desligar, como_jogador);
}
unsafe fn add_staff_to_club(clube: usize, pessoa: usize, cargo: u32, data: usize, reduzir: u32, gravar: u32) {
    let f: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x53bd10usize);
    f(clube, pessoa, cargo, data, reduzir, gravar);
}
unsafe fn club_has_reserve_squad(clube: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x542740usize);
    f(clube)
}
/// Refaz a folha de salários (0x598370) das finanças do clube (0x5a0590 em [0xae2a38]; a original chama mesmo sem).
unsafe fn update_club_wage_bill(clube: usize) {
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x598370usize);
    folha(financas(cpp2_d(0xae2a38) as usize, clube));
}
unsafe fn contract_manager_assign_date(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut ret = [0u8; 8];
    copiar(destino, &mut ret, origem);
}

/// { 0x4d01e0 } (thiscall, ret 4, contract_manager.cpp, quadro SEH só com uma data local, sem destrutor):
/// `CONTRACT_MANAGER::sign_loan_contract(contrato)`: a pessoa (+0) do contrato de empréstimo novo e o clube dela pelo
/// contrato principal (a ficha [0xae1080]); sem ele, nada. Guarda a data +0x3e da pessoa, tira-a das listas (0x7c5fe0
/// com o clube novo +4), confere o contrato principal no próprio vetor (sem ele, caixa da linha 0x16a9 e nada mais) e
/// marca nele o bit 0x40 de +0x35 (no reserva): com time reserva (0x542740), pela marca de elenco +0x5e & 1; com clube
/// parceiro (0x540a50), se o clube atual (+0x39) não é o dono; senão apaga. Avisa o clube novo (0x90d860), tira a pessoa
/// das táticas do clube atual (0x896c60 com 0 e 1), põe no clube novo como jogador (0x53bd10, cargo 0xb com a data de
/// hoje) e devolve a data +0x3e. O contrato vai para a vaga de empréstimo da pessoa (+4 do registro de 79 bytes): na
/// que ela já tem; senão na primeira livre (−1) do vetor, que fica com o índice (e aí a original sai sem refazer as
/// folhas); senão no fim (cheio: +100 vagas, realloc 0x945667 ou malloc 0x944e46; sem memória, caixa da linha 0x1708
/// e segue), pelo STAFF_CONTRACT::operator= 0x4cc810. Por fim refaz as folhas de salários do clube dono e do novo.
unsafe extern "thiscall" fn sign_loan_contract(this: usize, contrato: usize) {
    let mut guardada = [0u8; 8];
    let construir: unsafe extern "thiscall" fn(*mut [u8; 8]) -> usize = core::mem::transmute(0x54a110usize);
    construir(&mut guardada);
    let pessoa = (cpp2_d(0xae23bc) as usize).wrapping_add((cpp2_d(contrato) as usize).wrapping_mul(0x6e));
    let ficha = if pessoa == 0 { cpp2_por(0xae1080, 0); 0 } else { main_contract_lookup(pessoa) };
    if ficha == 0 {
        return;
    }
    let dono = contract_manager_club_by_id(cpp2_d(ficha + 4));
    if dono == 0 {
        return;
    }
    contract_manager_assign_date(guardada.as_mut_ptr() as usize, pessoa + 0x3e);
    let novo = contract_manager_club_by_id(cpp2_d(contrato + 4));
    contract_manager_archive_player_stats(pessoa, 1, cpp2_d(novo) as i32);
    let principal = if pessoa != 0 && (cpp2_d(pessoa) as i32) < cpp2_d(0xae2364) as i32 && (cpp2_d(staff_state_entry(pessoa)) as i32) >= 0 {
        contract_at_index(cpp2_d(this), cpp2_d(staff_state_entry(pessoa)) as i32)
    } else {
        0
    };
    if principal == 0 {
        show_contract_error_box(0x16a9);
        return;
    }
    crate::comparison::branch("cpp_2 assinar emprestimo");
    let m = principal + 0x35;
    let reserva = if club_has_reserve_squad(dono) != 0 {
        read_byte_volatile(pessoa + 0x5e) & 1 == 0
    } else if contract_manager_linked_b_team(dono, core::ptr::null_mut()) != 0 {
        dono as u32 != cpp2_d(pessoa + 0x39)
    } else {
        false
    };
    let cl = read_byte_volatile(m);
    *(m as *mut u8) = if reserva { cl | 0x40 } else { cl & 0xbf };
    let avisar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x90d860usize);
    avisar(cpp2_d(0xde2f70) as usize, novo, 0);
    remove_from_tactics_selection(cpp2_d(pessoa + 0x39) as usize, pessoa, 0, 1);
    remove_from_tactics_selection(cpp2_d(pessoa + 0x39) as usize, pessoa, 1, 1);
    add_staff_to_club(novo, pessoa, 0xb, 0xae2c90, 1, 0); // o jogador no clube novo
    contract_manager_assign_date(pessoa + 0x3e, guardada.as_ptr() as usize);
    let emp = cpp2_d(staff_state_entry(pessoa) + 4) as i32;
    if emp > -1 {
        // ponytail: a original copia em linha, igual ao STAFF_CONTRACT::operator= (0x4cc810, portado)
        contract_manager_operator(contract_at_index(cpp2_d(this), emp), contrato);
    } else {
        let n = cpp2_d(this + 8) as i32;
        let vetor = cpp2_d(this);
        let mut i = 0i32;
        while i < n {
            if cpp2_d(contract_at_index(vetor, i)) as i32 == -1 {
                contract_manager_operator(contract_at_index(vetor, i), contrato);
                cpp2_por(staff_state_entry(pessoa) + 4, i as u32);
                return; // a original sai sem refazer as folhas
            }
            i += 1;
        }
        let cap = cpp2_d(this + 4) as i32;
        if n >= cap {
            let cap = cap + 100;
            cpp2_por(this + 4, cap as u32);
            let novo_vetor = if vetor != 0 {
                let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
                realocar(vetor as usize, (cap * 0x50) as u32)
            } else {
                let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
                alocar((cap * 0x50) as u32)
            };
            cpp2_por(this, novo_vetor as u32);
            if novo_vetor == 0 {
                show_contract_error_box(0x1708);
            }
        }
        let n = cpp2_d(this + 8);
        cpp2_por(staff_state_entry(pessoa) + 4, n);
        let destino = contract_at_index(cpp2_d(this), n as i32);
        cpp2_por(this + 8, n + 1);
        let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cc810usize);
        copiar(destino, contrato);
    }
    update_club_wage_bill(dono);
    update_club_wage_bill(novo);
}

unsafe extern "thiscall" fn comparar_sign_loan_contract(this: usize, contrato: usize) {
    // ponytail: mexe no elenco, nas táticas, nas listas e realoca o vetor: só a original; a cópia prova no modo Rust
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, contrato)
}

/// { 0x4d07e0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::end_loan(pessoa)`: o clube do empréstimo
/// (+0x39; nulo: caixa da linha 0x1729), ou o time A dele quando é o B (0x540a50). Com o contrato principal no próprio
/// vetor (sem ele: caixa da linha 0x1735): a ficha [0xae1080] dá o clube dono; tira a pessoa das listas (0x7c5fe0 com o
/// dono), das disputas (0x556570), das táticas do clube do empréstimo (0x896c60 com 0 e 1); libera o contrato de
/// empréstimo (−1 na entrada e na vaga +4); tira do elenco do clube do empréstimo (0x53c190); refaz as folhas do clube
/// do empréstimo e do dono; tira da lista de observação do clube do empréstimo (0x849ce0, 0x843e10 com 4). A pessoa
/// volta ao dono (+0x39) e às marcas de elenco (0x540980): com time reserva (0x542740), 2 se o contrato principal tem o
/// bit 0x40 de +0x35, senão 1; senão 1 e, com clube parceiro, fica no parceiro se está nas 50 vagas dele. A situação
/// +0x6d 0x10 vira 4.
unsafe extern "thiscall" fn end_loan(this: usize, pessoa: usize) {
    let mut clube = cpp2_d(pessoa + 0x39) as usize;
    if clube == 0 {
        show_contract_error_box(0x1729);
        return;
    }
    let mut eh_a = 0u32; // só lida quando a 0x540a50 a grava
    if contract_manager_linked_b_team(clube, &mut eh_a) != 0 && eh_a == 0 {
        clube = contract_manager_linked_b_team(clube, core::ptr::null_mut()); // o time A
    }
    let principal = if pessoa != 0 && (cpp2_d(pessoa) as i32) < cpp2_d(0xae2364) as i32 && (cpp2_d(staff_state_entry(pessoa)) as i32) >= 0 {
        contract_at_index(cpp2_d(this), cpp2_d(staff_state_entry(pessoa)) as i32)
    } else {
        0
    };
    if principal == 0 {
        show_contract_error_box(0x1735);
        return;
    }
    crate::comparison::branch("cpp_2 encerrar emprestimo");
    let ficha = contract_at_index(cpp2_d(cpp2_d(0xae1050) as usize), cpp2_d(staff_state_entry(pessoa)) as i32);
    cpp2_por(0xae1080, ficha as u32);
    let dono = if ficha == 0 { 0 } else { contract_manager_club_by_id(cpp2_d(ficha + 4)) };
    contract_manager_archive_player_stats(pessoa, 0, cpp2_d(dono) as i32);
    let disputas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x556570usize);
    disputas(cpp2_d(0xae28f0) as usize, pessoa);
    remove_from_tactics_selection(cpp2_d(pessoa + 0x39) as usize, pessoa, 0, 1);
    remove_from_tactics_selection(cpp2_d(pessoa + 0x39) as usize, pessoa, 1, 1);
    let emp = cpp2_d(staff_state_entry(pessoa) + 4) as i32;
    if emp != -1 {
        cpp2_por(contract_at_index(cpp2_d(this), emp), u32::MAX);
        cpp2_por(staff_state_entry(pessoa) + 4, u32::MAX);
    }
    contract_manager_remove_staff_from_club(cpp2_d(pessoa + 0x39) as usize, pessoa, 0, 1, 0);
    update_club_wage_bill(clube);
    let f = main_contract_lookup(pessoa);
    update_club_wage_bill(if f == 0 { 0 } else { contract_manager_club_by_id(cpp2_d(f + 4)) });
    let lista = cpp2_d(cpp2_d(0xde1f54) as usize + 0x856) as usize;
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    if entrada(lista, clube, pessoa, 0, 0) != 0 {
        let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x843e10usize);
        tirar(cpp2_d(cpp2_d(0xde1f54) as usize + 0x856) as usize, pessoa, clube, 4, 0);
    }
    let f = main_contract_lookup(pessoa);
    let volta = if f == 0 { 0 } else { contract_manager_club_by_id(cpp2_d(f + 4)) };
    cpp2_por(pessoa + 0x39, volta as u32); // a pessoa volta ao dono
    if club_has_reserve_squad(volta) != 0 {
        set_squad_flags(pessoa, if read_byte_volatile(principal + 0x35) & 0x40 != 0 { 2 } else { 1 });
    } else {
        let parceiro = contract_manager_linked_b_team(cpp2_d(pessoa + 0x39) as usize, core::ptr::null_mut());
        set_squad_flags(pessoa, 1);
        if parceiro != 0 {
            for k in 0..0x32 {
                if cpp2_d(parceiro + 0xd7 + 4 * k) as usize == pessoa {
                    cpp2_por(pessoa + 0x39, parceiro as u32);
                }
            }
        }
    }
    if read_byte_volatile(pessoa + 0x6d) == 0x10 {
        *((pessoa + 0x6d) as *mut u8) = 4;
    }
}

unsafe extern "thiscall" fn comparar_end_loan(this: usize, pessoa: usize) {
    // ponytail: mexe no elenco, nas táticas, nas listas e nas disputas (0x556570 libera itens): só a original
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, pessoa)
}

/// { 0x4cbd20 } (thiscall, ret 0xc, contract_manager.cpp): `CONTRACT_MANAGER::toggle_squad_placement(pessoa, clube,
/// saida)`: pelo lugar (0x4cbb60): 0 nada. 1 e 2 (principal / reserva no clube): emprestada a outro clube (contrato de
/// empréstimo +4 com outro clube), só apaga (1) ou marca (2) o bit 0x40 de +0x35 do contrato principal; senão a marca de
/// elenco 1 ou 2 (0x540980) e as táticas do clube (0x896c60 com 1 ou 0). 3 (time B) e os outros (time A): o clube de
/// destino é o clube (3) ou o parceiro (0x540a50), gravado em `saida`; sem vaga para o cargo (+0x3d, 0x53b9a0) devolve
/// 0. Emprestada a este clube: sai das listas, do elenco e das táticas do outro e entra no destino com o cargo; com
/// contrato de empréstimo de outro clube: troca o bit 0x40 e passa ao destino como jogador (0xb); senão como o
/// emprestado, sem gravar. Devolve 1 (clube nulo também). Sem contrato principal a original lê [0x35] e cai.
unsafe extern "thiscall" fn toggle_squad_placement(this: usize, pessoa: usize, clube: usize, saida: usize) -> u32 {
    if clube == 0 {
        return 1;
    }
    if saida != 0 {
        cpp2_por(saida, 0);
    }
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4cbb60usize);
    let l = lugar(this, pessoa, clube);
    if l == 0 {
        return 1;
    }
    let na_faixa = |p: usize| p != 0 && (cpp2_d(p) as i32) < cpp2_d(0xae2364) as i32;
    // o contrato de empréstimo (no CONTRACT_MANAGER global) e o principal (no próprio vetor; 0 sem)
    let emprestimo = |p: usize| {
        let k = cpp2_d(staff_state_entry(p) + 4) as i32;
        if k < 0 { 0 } else { contract_at_index(cpp2_d(cpp2_d(0xae1050) as usize), k) }
    };
    let principal = |p: usize| {
        let k = cpp2_d(staff_state_entry(p)) as i32;
        if k < 0 { 0 } else { contract_at_index(cpp2_d(this), k) }
    };
    if l == 1 || l == 2 {
        if na_faixa(pessoa) {
            let e = emprestimo(pessoa);
            if e != 0 && cpp2_d(e + 4) != cpp2_d(clube) {
                let m = principal(pessoa) + 0x35;
                let cl = read_byte_volatile(m);
                *(m as *mut u8) = if l == 1 { cl & 0xbf } else { cl | 0x40 };
                return 1;
            }
        }
        crate::comparison::branch("cpp_2 trocar no clube");
        set_squad_flags(pessoa, l as u32); // 1 principal, 2 reserva
        remove_from_tactics_selection(clube, pessoa, if l == 1 { 1 } else { 0 }, 1);
        return 1;
    }
    let (origem, destino) = if l == 3 {
        (contract_manager_linked_b_team(clube, core::ptr::null_mut()), clube)
    } else {
        (clube, contract_manager_linked_b_team(clube, core::ptr::null_mut()))
    };
    if saida != 0 {
        cpp2_por(saida, destino as u32);
    }
    let vaga: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x53b9a0usize);
    // ponytail: a original empurra o cargo num registrador com lixo nos bytes altos; a 0x53b9a0 lê só o byte
    if vaga(destino, read_byte_volatile(pessoa + 0x3d) as u32, pessoa) == 0 {
        return 0;
    }
    if na_faixa(pessoa) {
        let e = emprestimo(pessoa);
        if e != 0 && cpp2_d(e + 4) == cpp2_d(clube) {
            // emprestada a este clube: passa do outro ao destino com o cargo
            contract_manager_archive_player_stats(pessoa, 0, -1);
            contract_manager_remove_staff_from_club(origem, pessoa, 0, 1, 1);
            remove_from_tactics_selection(origem, pessoa, 1, 1);
            remove_from_tactics_selection(origem, pessoa, 0, 1);
            // ponytail: o cargo vai com os bytes altos do eax da 0x896c60; a 0x53bd10 lê só o byte
            add_staff_to_club(destino, pessoa, read_byte_volatile(pessoa + 0x3d) as u32, 0, 1, 1);
            return 1;
        }
        if na_faixa(pessoa) && e != 0 {
            let m = principal(pessoa) + 0x35;
            let cl = read_byte_volatile(m);
            *(m as *mut u8) = if cl & 0x40 == 0 { cl | 0x40 } else { cl & 0xbf };
            crate::comparison::branch("cpp_2 trocar emprestado");
            contract_manager_remove_staff_from_club(origem, pessoa, 0, 0, 1);
            add_staff_to_club(destino, pessoa, 0xb, 0, 0, 1);
            return 1;
        }
    }
    crate::comparison::branch("cpp_2 trocar de time");
    contract_manager_archive_player_stats(pessoa, 0, -1);
    contract_manager_remove_staff_from_club(origem, pessoa, 0, 1, 0);
    remove_from_tactics_selection(origem, pessoa, 1, 1);
    remove_from_tactics_selection(origem, pessoa, 0, 1);
    // ponytail: o cargo vai com os bytes altos do edx deixados pela 0x896c60; a 0x53bd10 lê só o byte
    add_staff_to_club(destino, pessoa, read_byte_volatile(pessoa + 0x3d) as u32, 0, 1, 0);
    1
}

unsafe extern "thiscall" fn comparar_toggle_squad_placement(this: usize, pessoa: usize, clube: usize, saida: usize) -> u32 {
    // ponytail: mexe no elenco, nas táticas e nas listas: só a original; a cópia prova no modo Rust
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(%%I2%%)(this, pessoa, clube, saida)
}
