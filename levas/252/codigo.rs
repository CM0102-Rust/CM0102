// ---------- 252a leva: as datas de decisão e as ofertas do clube: a ordem das datas, decidir as datas vencidas e as ofertas de contrato do clube (contract_manager.cpp) ----------

unsafe fn cpp1_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpp1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// As comparações de data do jogo (thiscall na data `a`, a outra por valor): 0x456740 `>`, 0x456770 `<`, 0x4a7f20 `==`.
unsafe fn cpp1_data(f: usize, a: usize, b: usize) -> u32 {
    let g: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(f);
    g(a, cpp1_d(b), cpp1_d(b + 4))
}

/// { 0x4dc910 } (cdecl, contract_manager.cpp): `compare_decision_dates(a, b)`: a ordem do qsort das datas de decisão
/// (entradas de 14 bytes: +0 pessoa, +4 data, +0xc fatia do dia, +0xd tipo): a data de `a` antes (0x456770): −1;
/// depois (0x456740): 1; no mesmo dia, a fatia de `a` menos a de `b` (com sinal). Quente (o qsort): sem contador.
unsafe extern "C" fn comparar_datas_de_decisao(a: usize, b: usize) -> i32 {
    if cpp1_data(0x456770, a + 4, b + 4) != 0 {
        return -1; // a antes de b
    }
    if cpp1_data(0x456740, a + 4, b + 4) != 0 {
        return 1;
    }
    cpp1_b(a + 0xc) as i8 as i32 - cpp1_b(b + 0xc) as i8 as i32
}

unsafe extern "C" fn comparar_comparar_datas_de_decisao(a: usize, b: usize) -> i32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1006);
    let r = o(a, b);
    let n = comparar_datas_de_decisao(a, b);
    PLACAR.registrar(r == n, || format!("comparar_datas_de_decisao original={} rust={}", r, n));
    r
}

/// { 0x4cd9c0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::process_due_decision_dates(data, fatia)`:
/// do índice +0x18 até a conta +0x14 do vetor das datas de decisão (+0xc, entradas de 14 bytes, ordenadas pela
/// 0x4dc910): para na primeira data depois de `data` (0x456740) ou, no mesmo dia (0x4a7f20), com a fatia da entrada
/// maior que `fatia`. Cada entrada vencida com pessoa (+0 ≠ −1) fica com −1 e, sendo do tipo 3 (+0xd), a pessoa decide
/// as ofertas de contrato (0x4ce4e0). O índice +0x18 sobe a cada entrada (relido depois da decisão).
unsafe extern "thiscall" fn decidir_as_datas_vencidas(this: usize, data: usize, fatia: u32) {
    let n = |c: usize| cpp1_d(c) as i32;
    if n(this + 0x18) >= n(this + 0x14) {
        return;
    }
    let resolver: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4ce4e0usize);
    loop {
        let e = (cpp1_d(this + 0xc) as usize).wrapping_add((n(this + 0x18) as usize).wrapping_mul(14));
        if cpp1_data(0x456740, e + 4, data) != 0 {
            return; // ainda não venceu
        }
        if cpp1_data(0x4a7f20, e + 4, data) != 0 && cpp1_b(e + 0xc) as i8 > fatia as u8 as i8 {
            return; // hoje, numa fatia depois desta
        }
        let id = n(e);
        if id != -1 {
            let pessoa = (cpp1_d(0xae23bc) as usize).wrapping_add((id as usize).wrapping_mul(0x6e));
            core::ptr::write_unaligned(e as *mut i32, -1);
            if cpp1_b(e + 0xd) == 3 {
                crate::comparacao::ramo("cpp_1 decidir ofertas");
                resolver(this, pessoa); // a pessoa decide as ofertas
            }
        }
        let i = n(this + 0x18) + 1;
        core::ptr::write_unaligned((this + 0x18) as *mut i32, i);
        if i >= n(this + 0x14) {
            return;
        }
    }
}

/// Comparador: a decisão das ofertas (0x4ce4e0) mexe nos contratos, nas ofertas e no próprio vetor: o plano (as datas
/// lidas antes, sem rodar nada) diz se alguma entrada do tipo 3 com pessoa vence; com ela, só a original. Sem ela (o
/// caso de todo dia), lado a lado pelo índice +0x18 e pelas entradas que vencem (ficam com −1).
unsafe extern "thiscall" fn comparar_decidir_as_datas_vencidas(this: usize, data: usize, fatia: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1007);
    let (i0, n) = (cpp1_d(this + 0x18) as i32, cpp1_d(this + 0x14) as i32);
    let mut i = i0;
    let mut decide = false;
    while i < n {
        let e = (cpp1_d(this + 0xc) as usize).wrapping_add((i as usize).wrapping_mul(14));
        if cpp1_data(0x456740, e + 4, data) != 0
            || (cpp1_data(0x4a7f20, e + 4, data) != 0 && cpp1_b(e + 0xc) as i8 > fatia as u8 as i8) {
            break;
        }
        decide |= cpp1_d(e) as i32 != -1 && cpp1_b(e + 0xd) == 3;
        i += 1;
    }
    if decide {
        return o(this, data, fatia);
    }
    let mut reg = vec![(this + 0x18, 4)];
    if i > i0 {
        reg.push(((cpp1_d(this + 0xc) as usize).wrapping_add((i0 as usize).wrapping_mul(14)), ((i - i0) as usize) * 14));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, data, fatia), || decidir_as_datas_vencidas(this, data, fatia));
    PLACAR.registrar(igual, || format!("decidir_as_datas_vencidas(fatia {}) {:x?} x {:x?}", fatia as u8 as i8, mo, mr));
}

/// { 0x4ca300 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::get_current_contract_offers(clube, saida)`:
/// percorre a lista das ofertas (+0x34; cursor reposto, 0x68d1b0) e copia cada oferta do clube (+4 = id) fora dos
/// estados 5 e 6 (+0x35 & 0x3f) para um vetor novo de 111 bytes por oferta, que cresce de 20 em 20 (realloc 0x945667
/// com a capacidade em byte com sinal). Grava o vetor em `saida` e devolve quantas (byte). A original confere a lista
/// (+0x34), não o vetor, depois do realloc: só com a lista nula vai à caixa "cannot allocate memory" (linha 0x64c) e
/// devolve 0 com a saída nula. Quem a chama (0x598a06, o orçamento de transferências) libera o vetor.
unsafe extern "thiscall" fn ofertas_de_contrato_do_clube(this: usize, clube: usize, saida: usize) -> u8 {
    let mut n: i8 = 0;
    let mut cap: i8 = 0;
    let mut vetor: usize = 0;
    let lista = cpp1_d(this + 0x34) as usize;
    if lista != 0 {
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, cpp1_d(lista + 2));
        let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let mut item = proximo(lista);
        while item != 0 {
            let estado = cpp1_b(item + 0x35) & 0x3f;
            if cpp1_d(item + 4) == cpp1_d(clube) && estado != 5 && estado != 6 {
                if cap == n {
                    cap = cap.wrapping_add(20);
                    vetor = realocar(vetor, (cap as i32 * 111) as u32);
                    if cpp1_d(this + 0x34) == 0 {
                        cpl4_sem_espaco(0x64c, 0x9a8220); // "cannot allocate memory"
                        core::ptr::write_unaligned(saida as *mut u32, 0);
                        return 0;
                    }
                }
                crate::comparacao::ramo("cpp_1 oferta do clube");
                core::ptr::copy(item as *const u8, vetor.wrapping_add((n as i32 * 111) as usize) as *mut u8, 111);
                n = n.wrapping_add(1);
            }
            item = proximo(cpp1_d(this + 0x34) as usize);
        }
    }
    core::ptr::write_unaligned(saida as *mut u32, vetor as u32);
    n as u8
}

/// Comparador: a original aloca o vetor que quem chama libera: roda só ela e confere o plano feito antes (as ofertas
/// do clube lidas da lista, sem alocar): a conta e os 111 bytes de cada uma, na ordem.
unsafe extern "thiscall" fn comparar_ofertas_de_contrato_do_clube(this: usize, clube: usize, saida: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(1008);
    let lista = cpp1_d(this + 0x34) as usize;
    if lista == 0 || clube == 0 || saida == 0 {
        return o(this, clube, saida);
    }
    let mut plano: Vec<u8> = Vec::new();
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cpp1_d(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut item = proximo(lista);
    while item != 0 {
        let estado = cpp1_b(item + 0x35) & 0x3f;
        if cpp1_d(item + 4) == cpp1_d(clube) && estado != 5 && estado != 6 {
            plano.extend_from_slice(core::slice::from_raw_parts(item as *const u8, 111));
        }
        item = proximo(lista);
    }
    let r = o(this, clube, saida);
    let vetor = cpp1_d(saida) as usize;
    let k = plano.len() / 111;
    let ok = r as usize == k && (k == 0 || core::slice::from_raw_parts(vetor as *const u8, k * 111) == &plano[..]);
    PLACAR.registrar(ok, || format!("ofertas_de_contrato_do_clube(clube {:x}) original {} ofertas, plano {}", clube, r, k));
    r
}

// ---------- 252a leva: os empréstimos e o lugar no elenco: assinar o contrato de empréstimo, encerrar o empréstimo e trocar o lugar no elenco (contract_manager.cpp) ----------

unsafe fn cpp2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpp2_b(a: usize) -> u8 {
    // ponytail: leitura volátil porque a original lê [0 + 0x35] quando o contrato principal falta (cai como ela)
    core::ptr::read_volatile(a as *const u8)
}

unsafe fn cpp2_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// O registro de 79 bytes da pessoa em [0xae2d0c] (+0 o contrato principal, +4 o de empréstimo; −1 sem).
unsafe fn cpp2_vaga(pessoa: usize) -> usize {
    (cpp2_d(0xae2d0c) as usize).wrapping_add((cpp2_d(pessoa) as usize).wrapping_mul(0x4f))
}

/// A entrada `k` (0x50 bytes) do vetor de contratos que começa em `vetor`.
fn cpp2_contrato(vetor: u32, k: i32) -> usize {
    (vetor as usize).wrapping_add((k as usize).wrapping_mul(0x50))
}

/// O clube de id `id` (registro de 0x245 bytes em [0xae23b4]).
unsafe fn cpp2_clube(id: u32) -> usize {
    (cpp2_d(0xae23b4) as usize).wrapping_add((id as usize).wrapping_mul(0x245))
}

/// O contrato principal da pessoa no CONTRACT_MANAGER global ([[0xae1050]]), como as originais o refazem: pessoa fora
/// da faixa (id ≥ [0xae2364]) ou sem contrato principal: 0. Grava em [0xae1080] (a ficha global).
unsafe fn cpp2_ficha(pessoa: usize) -> usize {
    let c = if cpp2_d(pessoa) as i32 >= cpp2_d(0xae2364) as i32 {
        0
    } else {
        let k = cpp2_d(cpp2_vaga(pessoa)) as i32;
        if k < 0 { 0 } else { cpp2_contrato(cpp2_d(cpp2_d(0xae1050) as usize), k) }
    };
    cpp2_por(0xae1080, c as u32);
    c
}

/// A caixa "Error" do contract_manager.cpp na `linha`: "v%s %s.%s %d" em 0xae24d0 (0x944cff separa o caminho 0x9a8190,
/// 0x90d130 a versão; a original não desempilha os argumentos dela, que são o resto do formato), a caixa 0x5e8290 e
/// [0xb67a34] = 0. O jogo segue.
unsafe fn cpp2_erro(linha: u32) {
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
unsafe fn cpp2_parceiro(clube: usize, eh_a: *mut u32) -> usize {
    let f: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    f(clube, eh_a, 1)
}
unsafe fn cpp2_marcar(pessoa: usize, elenco: u32) {
    let f: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x540980usize);
    f(pessoa, elenco);
}
unsafe fn cpp2_tatica(clube: usize, pessoa: usize, a: u32, b: u32) {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x896c60usize);
    f(cpp2_d(0xde1d08) as usize, clube, pessoa, a, b);
}
unsafe fn cpp2_listas(pessoa: usize, a4: u32, clube: i32) {
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, i32, u32) = core::mem::transmute(0x7c5fe0usize);
    f(cpp2_d(0xb74350) as usize, pessoa, -1, 0, a4, clube, 0);
}
unsafe fn cpp2_tirar(clube: usize, pessoa: usize, data: usize, desligar: u32, como_jogador: u32) {
    let f: unsafe extern "C" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x53c190usize);
    f(clube, pessoa, data, desligar, como_jogador);
}
unsafe fn cpp2_por_no_clube(clube: usize, pessoa: usize, cargo: u32, data: usize, reduzir: u32, gravar: u32) {
    let f: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x53bd10usize);
    f(clube, pessoa, cargo, data, reduzir, gravar);
}
unsafe fn cpp2_reservas(clube: usize) -> u32 {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x542740usize);
    f(clube)
}
/// Refaz a folha de salários (0x598370) das finanças do clube (0x5a0590 em [0xae2a38]; a original chama mesmo sem).
unsafe fn cpp2_folha(clube: usize) {
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x598370usize);
    folha(financas(cpp2_d(0xae2a38) as usize, clube));
}
unsafe fn cpp2_data(destino: usize, origem: usize) {
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
unsafe extern "thiscall" fn assinar_contrato_de_emprestimo(this: usize, contrato: usize) {
    let mut guardada = [0u8; 8];
    let construir: unsafe extern "thiscall" fn(*mut [u8; 8]) -> usize = core::mem::transmute(0x54a110usize);
    construir(&mut guardada);
    let pessoa = (cpp2_d(0xae23bc) as usize).wrapping_add((cpp2_d(contrato) as usize).wrapping_mul(0x6e));
    let ficha = if pessoa == 0 { cpp2_por(0xae1080, 0); 0 } else { cpp2_ficha(pessoa) };
    if ficha == 0 {
        return;
    }
    let dono = cpp2_clube(cpp2_d(ficha + 4));
    if dono == 0 {
        return;
    }
    cpp2_data(guardada.as_mut_ptr() as usize, pessoa + 0x3e);
    let novo = cpp2_clube(cpp2_d(contrato + 4));
    cpp2_listas(pessoa, 1, cpp2_d(novo) as i32);
    let principal = if pessoa != 0 && (cpp2_d(pessoa) as i32) < cpp2_d(0xae2364) as i32 && (cpp2_d(cpp2_vaga(pessoa)) as i32) >= 0 {
        cpp2_contrato(cpp2_d(this), cpp2_d(cpp2_vaga(pessoa)) as i32)
    } else {
        0
    };
    if principal == 0 {
        cpp2_erro(0x16a9);
        return;
    }
    crate::comparacao::ramo("cpp_2 assinar emprestimo");
    let m = principal + 0x35;
    let reserva = if cpp2_reservas(dono) != 0 {
        cpp2_b(pessoa + 0x5e) & 1 == 0
    } else if cpp2_parceiro(dono, core::ptr::null_mut()) != 0 {
        dono as u32 != cpp2_d(pessoa + 0x39)
    } else {
        false
    };
    let cl = cpp2_b(m);
    *(m as *mut u8) = if reserva { cl | 0x40 } else { cl & 0xbf };
    let avisar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x90d860usize);
    avisar(cpp2_d(0xde2f70) as usize, novo, 0);
    cpp2_tatica(cpp2_d(pessoa + 0x39) as usize, pessoa, 0, 1);
    cpp2_tatica(cpp2_d(pessoa + 0x39) as usize, pessoa, 1, 1);
    cpp2_por_no_clube(novo, pessoa, 0xb, 0xae2c90, 1, 0); // o jogador no clube novo
    cpp2_data(pessoa + 0x3e, guardada.as_ptr() as usize);
    let emp = cpp2_d(cpp2_vaga(pessoa) + 4) as i32;
    if emp > -1 {
        // ponytail: a original copia em linha, igual ao STAFF_CONTRACT::operator= (0x4cc810, portado)
        copiar_contrato_de_pessoal(cpp2_contrato(cpp2_d(this), emp), contrato);
    } else {
        let n = cpp2_d(this + 8) as i32;
        let vetor = cpp2_d(this);
        let mut i = 0i32;
        while i < n {
            if cpp2_d(cpp2_contrato(vetor, i)) as i32 == -1 {
                copiar_contrato_de_pessoal(cpp2_contrato(vetor, i), contrato);
                cpp2_por(cpp2_vaga(pessoa) + 4, i as u32);
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
                cpp2_erro(0x1708);
            }
        }
        let n = cpp2_d(this + 8);
        cpp2_por(cpp2_vaga(pessoa) + 4, n);
        let destino = cpp2_contrato(cpp2_d(this), n as i32);
        cpp2_por(this + 8, n + 1);
        let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cc810usize);
        copiar(destino, contrato);
    }
    cpp2_folha(dono);
    cpp2_folha(novo);
}

unsafe extern "thiscall" fn comparar_assinar_contrato_de_emprestimo(this: usize, contrato: usize) {
    // ponytail: mexe no elenco, nas táticas, nas listas e realoca o vetor: só a original; a cópia prova no modo Rust
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1009)(this, contrato)
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
unsafe extern "thiscall" fn encerrar_o_emprestimo(this: usize, pessoa: usize) {
    let mut clube = cpp2_d(pessoa + 0x39) as usize;
    if clube == 0 {
        cpp2_erro(0x1729);
        return;
    }
    let mut eh_a = 0u32; // só lida quando a 0x540a50 a grava
    if cpp2_parceiro(clube, &mut eh_a) != 0 && eh_a == 0 {
        clube = cpp2_parceiro(clube, core::ptr::null_mut()); // o time A
    }
    let principal = if pessoa != 0 && (cpp2_d(pessoa) as i32) < cpp2_d(0xae2364) as i32 && (cpp2_d(cpp2_vaga(pessoa)) as i32) >= 0 {
        cpp2_contrato(cpp2_d(this), cpp2_d(cpp2_vaga(pessoa)) as i32)
    } else {
        0
    };
    if principal == 0 {
        cpp2_erro(0x1735);
        return;
    }
    crate::comparacao::ramo("cpp_2 encerrar emprestimo");
    let ficha = cpp2_contrato(cpp2_d(cpp2_d(0xae1050) as usize), cpp2_d(cpp2_vaga(pessoa)) as i32);
    cpp2_por(0xae1080, ficha as u32);
    let dono = if ficha == 0 { 0 } else { cpp2_clube(cpp2_d(ficha + 4)) };
    cpp2_listas(pessoa, 0, cpp2_d(dono) as i32);
    let disputas: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x556570usize);
    disputas(cpp2_d(0xae28f0) as usize, pessoa);
    cpp2_tatica(cpp2_d(pessoa + 0x39) as usize, pessoa, 0, 1);
    cpp2_tatica(cpp2_d(pessoa + 0x39) as usize, pessoa, 1, 1);
    let emp = cpp2_d(cpp2_vaga(pessoa) + 4) as i32;
    if emp != -1 {
        cpp2_por(cpp2_contrato(cpp2_d(this), emp), u32::MAX);
        cpp2_por(cpp2_vaga(pessoa) + 4, u32::MAX);
    }
    cpp2_tirar(cpp2_d(pessoa + 0x39) as usize, pessoa, 0, 1, 0);
    cpp2_folha(clube);
    let f = cpp2_ficha(pessoa);
    cpp2_folha(if f == 0 { 0 } else { cpp2_clube(cpp2_d(f + 4)) });
    let lista = cpp2_d(cpp2_d(0xde1f54) as usize + 0x856) as usize;
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    if entrada(lista, clube, pessoa, 0, 0) != 0 {
        let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x843e10usize);
        tirar(cpp2_d(cpp2_d(0xde1f54) as usize + 0x856) as usize, pessoa, clube, 4, 0);
    }
    let f = cpp2_ficha(pessoa);
    let volta = if f == 0 { 0 } else { cpp2_clube(cpp2_d(f + 4)) };
    cpp2_por(pessoa + 0x39, volta as u32); // a pessoa volta ao dono
    if cpp2_reservas(volta) != 0 {
        cpp2_marcar(pessoa, if cpp2_b(principal + 0x35) & 0x40 != 0 { 2 } else { 1 });
    } else {
        let parceiro = cpp2_parceiro(cpp2_d(pessoa + 0x39) as usize, core::ptr::null_mut());
        cpp2_marcar(pessoa, 1);
        if parceiro != 0 {
            for k in 0..0x32 {
                if cpp2_d(parceiro + 0xd7 + 4 * k) as usize == pessoa {
                    cpp2_por(pessoa + 0x39, parceiro as u32);
                }
            }
        }
    }
    if cpp2_b(pessoa + 0x6d) == 0x10 {
        *((pessoa + 0x6d) as *mut u8) = 4;
    }
}

unsafe extern "thiscall" fn comparar_encerrar_o_emprestimo(this: usize, pessoa: usize) {
    // ponytail: mexe no elenco, nas táticas, nas listas e nas disputas (0x556570 libera itens): só a original
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1010)(this, pessoa)
}

/// { 0x4cbd20 } (thiscall, ret 0xc, contract_manager.cpp): `CONTRACT_MANAGER::toggle_squad_placement(pessoa, clube,
/// saida)`: pelo lugar (0x4cbb60): 0 nada. 1 e 2 (principal / reserva no clube): emprestada a outro clube (contrato de
/// empréstimo +4 com outro clube), só apaga (1) ou marca (2) o bit 0x40 de +0x35 do contrato principal; senão a marca de
/// elenco 1 ou 2 (0x540980) e as táticas do clube (0x896c60 com 1 ou 0). 3 (time B) e os outros (time A): o clube de
/// destino é o clube (3) ou o parceiro (0x540a50), gravado em `saida`; sem vaga para o cargo (+0x3d, 0x53b9a0) devolve
/// 0. Emprestada a este clube: sai das listas, do elenco e das táticas do outro e entra no destino com o cargo; com
/// contrato de empréstimo de outro clube: troca o bit 0x40 e passa ao destino como jogador (0xb); senão como o
/// emprestado, sem gravar. Devolve 1 (clube nulo também). Sem contrato principal a original lê [0x35] e cai.
unsafe extern "thiscall" fn trocar_lugar_no_elenco(this: usize, pessoa: usize, clube: usize, saida: usize) -> u32 {
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
        let k = cpp2_d(cpp2_vaga(p) + 4) as i32;
        if k < 0 { 0 } else { cpp2_contrato(cpp2_d(cpp2_d(0xae1050) as usize), k) }
    };
    let principal = |p: usize| {
        let k = cpp2_d(cpp2_vaga(p)) as i32;
        if k < 0 { 0 } else { cpp2_contrato(cpp2_d(this), k) }
    };
    if l == 1 || l == 2 {
        if na_faixa(pessoa) {
            let e = emprestimo(pessoa);
            if e != 0 && cpp2_d(e + 4) != cpp2_d(clube) {
                let m = principal(pessoa) + 0x35;
                let cl = cpp2_b(m);
                *(m as *mut u8) = if l == 1 { cl & 0xbf } else { cl | 0x40 };
                return 1;
            }
        }
        crate::comparacao::ramo("cpp_2 trocar no clube");
        cpp2_marcar(pessoa, l as u32); // 1 principal, 2 reserva
        cpp2_tatica(clube, pessoa, if l == 1 { 1 } else { 0 }, 1);
        return 1;
    }
    let (origem, destino) = if l == 3 {
        (cpp2_parceiro(clube, core::ptr::null_mut()), clube)
    } else {
        (clube, cpp2_parceiro(clube, core::ptr::null_mut()))
    };
    if saida != 0 {
        cpp2_por(saida, destino as u32);
    }
    let vaga: unsafe extern "C" fn(usize, u32, usize) -> u32 = core::mem::transmute(0x53b9a0usize);
    // ponytail: a original empurra o cargo num registrador com lixo nos bytes altos; a 0x53b9a0 lê só o byte
    if vaga(destino, cpp2_b(pessoa + 0x3d) as u32, pessoa) == 0 {
        return 0;
    }
    if na_faixa(pessoa) {
        let e = emprestimo(pessoa);
        if e != 0 && cpp2_d(e + 4) == cpp2_d(clube) {
            // emprestada a este clube: passa do outro ao destino com o cargo
            cpp2_listas(pessoa, 0, -1);
            cpp2_tirar(origem, pessoa, 0, 1, 1);
            cpp2_tatica(origem, pessoa, 1, 1);
            cpp2_tatica(origem, pessoa, 0, 1);
            // ponytail: o cargo vai com os bytes altos do eax da 0x896c60; a 0x53bd10 lê só o byte
            cpp2_por_no_clube(destino, pessoa, cpp2_b(pessoa + 0x3d) as u32, 0, 1, 1);
            return 1;
        }
        if na_faixa(pessoa) && e != 0 {
            let m = principal(pessoa) + 0x35;
            let cl = cpp2_b(m);
            *(m as *mut u8) = if cl & 0x40 == 0 { cl | 0x40 } else { cl & 0xbf };
            crate::comparacao::ramo("cpp_2 trocar emprestado");
            cpp2_tirar(origem, pessoa, 0, 0, 1);
            cpp2_por_no_clube(destino, pessoa, 0xb, 0, 0, 1);
            return 1;
        }
    }
    crate::comparacao::ramo("cpp_2 trocar de time");
    cpp2_listas(pessoa, 0, -1);
    cpp2_tirar(origem, pessoa, 0, 1, 0);
    cpp2_tatica(origem, pessoa, 1, 1);
    cpp2_tatica(origem, pessoa, 0, 1);
    // ponytail: o cargo vai com os bytes altos do edx deixados pela 0x896c60; a 0x53bd10 lê só o byte
    cpp2_por_no_clube(destino, pessoa, cpp2_b(pessoa + 0x3d) as u32, 0, 1, 0);
    1
}

unsafe extern "thiscall" fn comparar_trocar_lugar_no_elenco(this: usize, pessoa: usize, clube: usize, saida: usize) -> u32 {
    // ponytail: mexe no elenco, nas táticas e nas listas: só a original; a cópia prova no modo Rust
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(1011)(this, pessoa, clube, saida)
}

// ---------- 252a leva: a diretoria e o técnico: a diretoria do técnico novo, o motivo da saída do técnico e o registro do técnico (manager_manager.cpp, staff_records.cpp) ----------

unsafe fn cpp3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpp3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpp3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpp3_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cpp3_por_w(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

/// A linha do clube de id `id` na tabela da diretoria do MANAGER_MANAGER ([this] + 0x49 × id; relida a cada uso, como
/// na original): +1 a faixa de riqueza, +2/+3 marcas, +6 confiança, +8/+0xa/+0xc os humores, +0x10..+0x16 as mudanças,
/// +0x28, +0x30/+0x38 doubles, +0x40 marcas.
unsafe fn cpp3_linha(this: usize, id: u32) -> usize {
    (cpp3_d(this) as usize).wrapping_add((id as usize).wrapping_mul(0x49))
}

/// O humor novo (`mov word`): ftol do float `f32(sorteio1 + q)` menos `sorteio2` (os dois de 0..499), em dupla precisão.
unsafe fn cpp3_humor(q: f32) -> i16 {
    let s1 = sortear(0x1f4);
    let base = (s1 as f64 + q as f64) as f32; // fstp dword
    let s2 = sortear(0x1f4);
    ftol(base as f64 - s2 as f64) as i16
}

/// { 0x694c80 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::reset_board_for_new_manager(clube)`: marca o
/// bit 1 de +2 da linha da diretoria, recalcula as expectativas da temporada (0x6a3770 com a notícia) e marca de novo;
/// a lista dos clubes do Shortlist_Manager (+0xdc, 9 bytes por clube, +4) fica com 2. A reputação do técnico (+0xcf,
/// ficha +0x69): na mesma nação (+0x53 = +0x1a do técnico) a atual (+0xa) e a doméstica (+8) andam 10% para a do clube
/// (+0x80); senão a atual anda 10% a partir da de outra nação (+0xc). A base: a reputação atual (zero: a metade da do
/// clube, ou 5000 sem; abaixo de 500: 500 + sorteio de 1000) sobre a do clube (zero: ela mesma; abaixo de 500: 250 +
/// sorteio de 750) × 7500, em float; acima do double de 0x96d1b0 (lido como double: um denormal positivo) vira 10000,
/// abaixo de 6500, 6500. A confiança (+6) e os humores +8/+0xa/+0xc saem da base com dois sorteios de 500 cada (limites
/// 6250..9500, 6250..10000, 5250..10000, 5750..10000, com sorteios nos de baixo); zera as mudanças +0x10..+0x16, os
/// doubles +0x38, +0x28, +4, +5 e marcas (+2 &~0x2e, +3 &0xe0, +0x40 &7). Registra o técnico novo (0x869aa0 com 1). Com o
/// técnico humano (id ≥ [0xae2364] − 16): cancela as ofertas do clube (0x8c4950), refaz a lista do clube (0x84abe0) e
/// as táticas das 50 vagas (0x896c60). +1 = a faixa de riqueza das finanças (0x5986b0), 0 nas seleções ou sem finanças.
unsafe extern "thiscall" fn diretoria_do_tecnico_novo(this: usize, clube: usize) {
    let id = cpp3_d(clube);
    *(((cpp3_linha(this, id)) + 2) as *mut u8) |= 1;
    let expectativas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
    expectativas(this, clube, 1);
    *(((cpp3_linha(this, id)) + 2) as *mut u8) |= 1;
    let lista = cpp3_d(cpp3_d(cpp3_d(0xde1f54) as usize + 0x856) as usize + 0xdc) as usize;
    *(lista.wrapping_add((id as usize).wrapping_mul(9)) as *mut u8).add(4) = 2;
    let k = f64_em(0x967880); // 0,1
    let rep = cpp3_w(clube + 0x80) as i32;
    let tecnico = cpp3_d(clube + 0xcf) as usize;
    let ficha = cpp3_d(tecnico + 0x69) as usize;
    if cpp3_d(clube + 0x53) == cpp3_d(tecnico + 0x1a) {
        let a = cpp3_w(ficha + 0xa) as i32;
        cpp3_por_w(ficha + 0xa, ftol((rep - a) as f64 * k + a as f64) as i16);
        let ficha = cpp3_d(cpp3_d(clube + 0xcf) as usize + 0x69) as usize;
        let b = cpp3_w(ficha + 8) as i32;
        cpp3_por_w(ficha + 8, ftol((rep - b) as f64 * k + b as f64) as i16);
    } else {
        let c = cpp3_w(ficha + 0xc) as i32;
        let a = cpp3_w(ficha + 0xa) as i32;
        cpp3_por_w(ficha + 0xa, ftol((rep - c) as f64 * k + a as f64) as i16);
    }
    let ficha = cpp3_d(cpp3_d(clube + 0xcf) as usize + 0x69) as usize;
    let v = cpp3_w(ficha + 0xa) as i32;
    let mut atual = v as f32; // fst dword
    let zero = f32_em(0x968970) as f64;
    if v as f64 == zero {
        let r = cpp3_w(clube + 0x80) as i32;
        atual = if r != 0 { (r / 2) as f32 } else { 5000.0 };
    } else if (atual as f64) < f32_em(0x96d1a8) as f64 {
        atual = (sortear(0x3e8) + 0x1f4) as f32;
    }
    let d = cpp3_w(clube + 0x80) as i32;
    let divisor = if d as f64 == zero {
        atual as f64
    } else if (d as f64) < f32_em(0x96d1a8) as f64 {
        (sortear(0x2ee) + 0xfa) as f64
    } else {
        d as f64
    };
    let mut q = ((atual as f64 / divisor) * f32_em(0x96d1a4) as f64) as f32; // fstp dword
    if q as f64 > f64_em(0x96d1b0) {
        crate::comparacao::ramo("cpp_3 base no teto");
        q = 10000.0;
    } else if !(q as f64 >= f32_em(0x96d1ac) as f64) {
        q = 6500.0;
    }
    // a confiança e os humores, cada um com os seus limites
    let h = cpp3_humor(q);
    cpp3_por_w(cpp3_linha(this, id) + 6, h);
    let c = cpp3_w(cpp3_linha(this, id) + 6);
    if c < 0x186a {
        cpp3_por_w(cpp3_linha(this, id) + 6, (sortear(0x4e2) + 0x186a) as i16);
    } else if c > 0x251c {
        cpp3_por_w(cpp3_linha(this, id) + 6, (0x2710 - sortear(0x1f4)) as i16);
    }
    let s1 = sortear(0x1f4);
    let parcial = cpp3_w(cpp3_linha(this, id) + 6).wrapping_sub(s1 as i16);
    let s2 = sortear(0x1f4);
    cpp3_por_w(cpp3_linha(this, id) + 8, parcial.wrapping_add(s2 as i16));
    let c = cpp3_w(cpp3_linha(this, id) + 8);
    if c < 0x186a {
        cpp3_por_w(cpp3_linha(this, id) + 8, (sortear(0x4e2) + 0x186a) as i16);
    } else if c > 0x2710 {
        cpp3_por_w(cpp3_linha(this, id) + 8, 0x2710);
    }
    let h = cpp3_humor(q);
    cpp3_por_w(cpp3_linha(this, id) + 0xa, h);
    let c = cpp3_w(cpp3_linha(this, id) + 0xa);
    if c < 0x1482 {
        cpp3_por_w(cpp3_linha(this, id) + 0xa, (sortear(0xfa) + 0x1482) as i16);
    } else if c > 0x2710 {
        cpp3_por_w(cpp3_linha(this, id) + 0xa, 0x2710);
    }
    let h = cpp3_humor(q);
    cpp3_por_w(cpp3_linha(this, id) + 0xc, h);
    let c = cpp3_w(cpp3_linha(this, id) + 0xc);
    if c < 0x1676 {
        cpp3_por_w(cpp3_linha(this, id) + 0xc, (sortear(0xfa) + 0x1676) as i16);
    } else if c > 0x2710 {
        cpp3_por_w(cpp3_linha(this, id) + 0xc, 0x2710);
    }
    let e = cpp3_linha(this, id);
    for o in [0x10, 0x12, 0x14, 0x16] {
        cpp3_por_w(e + o, 0); // as mudanças
    }
    let m = |o: usize, mascara: u8| *((cpp3_linha(this, id) + o) as *mut u8) &= mascara;
    m(2, 0xf7);
    m(2, 0xfb);
    m(2, 0xfd);
    cpp3_por(e + 0x38, 0);
    cpp3_por(e + 0x3c, 0);
    *((e + 4) as *mut u8) = 0;
    m(3, 0xe1);
    m(2, 0xdf);
    cpp3_por(e + 0x28, 0);
    *((e + 5) as *mut u8) = 0;
    m(3, 0xfe);
    m(0x40, 7);
    let registrar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x869aa0usize);
    registrar(cpp3_d(0xdd7f00) as usize, cpp3_d(clube + 0xcf) as usize, clube, 1);
    if cpp3_d(cpp3_d(clube + 0xcf) as usize) as i32 >= (cpp3_d(0xae2364) as i32).wrapping_sub(0x10) {
        let cancelar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c4950usize);
        cancelar(cpp3_d(0xde1f54) as usize, clube);
        let refazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x84abe0usize);
        refazer(cpp3_d(cpp3_d(0xde1f54) as usize + 0x856) as usize, clube);
        let tatica: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x896c60usize);
        for k in 0..0x32 {
            let p = cpp3_d(clube + 0xd7 + 4 * k) as usize;
            if p != 0 {
                tatica(cpp3_d(0xde1d08) as usize, clube, p, 1, 1);
            }
        }
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let mut faixa = 0u8;
    if selecao(clube) == 0 {
        let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
        let fin = financas(cpp3_d(0xae2a38) as usize, clube);
        if fin != 0 {
            let riqueza: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x5986b0usize);
            faixa = riqueza(fin);
        }
    }
    *((cpp3_linha(this, cpp3_d(clube)) + 1) as *mut u8) = faixa;
}

unsafe extern "thiscall" fn comparar_diretoria_do_tecnico_novo(this: usize, clube: usize) {
    // ponytail: as expectativas (0x6a3770) postam notícia e o registro (0x869aa0) grava o histórico em arquivo: não
    // rodam duas vezes; só a original, a cópia prova pela carreira no modo Rust (o ramo)
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1012)(this, clube)
}

/// { 0x69b350 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::get_departure_reason(id)`: o motivo (0..11) da
/// saída do técnico do clube `id`. Seleção (0x53b900): 0. Mais de 2 jogadores do elenco com a queixa 0x10000000 no
/// contrato (+0x45): 9. Na temporada (0x547770): na liga com a nação marcada (+0x11c), linha da tabela com mais de 25
/// (+4) e mais de 1 jogo (+0x55), pela posição 0x9c da competição: progresso (+0x73) dentro e a posição projetada
/// (0x4ae300) fora das (+0x3e − +0xc1) vagas: 11; dívida (double +0x30 < +0x38) com a mudança +0x12 abaixo de −300: 10.
/// Depois: +0x12 abaixo de −750 fora de uma sequência do tipo 1 (0x690c30) com sorteio de 3 = 0: 2; na temporada com o
/// bit 8 de +2: 7; +0x10 < −350: 3; +0x14 < −750: 5; humor +0xa abaixo da confiança +6 e sorteio(+0xa) < sorteio(+6):
/// 8; +0x16 < −750 com sorteio de 3 ≠ 0: 4; o técnico há mais dias (0x54cdb0 desde +0x3e) que 1000 + sorteio de 2000:
/// 6. Senão 0.
unsafe extern "thiscall" fn motivo_da_saida_do_tecnico(this: usize, id: u32) -> u8 {
    let clube = (cpp3_d(0xae23b4) as usize).wrapping_add((id as usize).wrapping_mul(0x245));
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        return 0;
    }
    let em_temporada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547770usize);
    crate::comparacao::ramo("cpp_3 motivo da saida");
    let temporada = em_temporada(clube, 0xae2c90);
    let (mut tipo, mut tamanho) = (0u8, 0u8);
    let sequencia: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x690c30usize);
    sequencia(this, clube, &mut tipo, &mut tamanho);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let mut queixosos = 0u8;
    for k in 0..0x32 {
        let p = cpp3_d(clube + 0xd7 + 4 * k) as usize;
        if p != 0 && cpp3_d(p + 0x61) != 0 {
            let c = contrato(cpp3_d(0xae1050) as usize, p, clube);
            if c != 0 && cpp3_d(c + 0x45) & 0x1000_0000 != 0 {
                queixosos = queixosos.wrapping_add(1);
            }
        }
    }
    if queixosos as i8 > 2 {
        return 9;
    }
    if temporada != 0 {
        let liga = cpp3_d(clube + 0x57) as usize;
        let nacao = cpp3_d(clube + 0x53) as usize;
        if liga != 0 && nacao != 0 && cpp3_b(nacao + 0x11c) != 0 {
            let comp = cpp3_d((cpp3_d(0xadadfc) as usize).wrapping_add((cpp3_d(liga) as usize).wrapping_mul(4))) as usize;
            if comp != 0 {
                let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
                let linha = busca(cpp3_d(comp + 8) as usize, clube);
                let (mut pa, mut pb, mut pc) = (0u8, 0u8, 0u8); // a 0x4ae300 grava as três (com a linha achada)
                let projetar: unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8, *mut u8) -> u32 = core::mem::transmute(0x4ae300usize);
                let comp = cpp3_d((cpp3_d(0xadadfc) as usize).wrapping_add((cpp3_d(cpp3_d(clube + 0x57) as usize) as usize).wrapping_mul(4))) as usize;
                projetar(cpp3_d(comp + 8) as usize, clube, &mut pa, &mut pb, &mut pc);
                if cpp3_b(linha + 4) > 0x19 && cpp3_d(linha + 0x55) as i32 > 1 {
                    let comp = cpp3_d((cpp3_d(0xadadfc) as usize).wrapping_add((cpp3_d(cpp3_d(clube + 0x57) as usize) as usize).wrapping_mul(4))) as usize;
                    let f: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cpp3_d(cpp3_d(comp) as usize + 0x9c) as usize);
                    if f(comp) != 0 {
                        let comp = cpp3_d((cpp3_d(0xadadfc) as usize).wrapping_add((cpp3_d(cpp3_d(clube + 0x57) as usize) as usize).wrapping_mul(4))) as usize;
                        let vagas = cpp3_w(comp + 0x3e) as i32 - cpp3_b(comp + 0xc1) as i8 as i32;
                        if cpp3_b(linha + 0x73) as i32 >= vagas && (pa as i32) < vagas {
                            return 0xb;
                        }
                    }
                }
            }
        }
        let e = cpp3_linha(this, id);
        let (gasto, teto) = (core::ptr::read_unaligned((e + 0x30) as *const f64), core::ptr::read_unaligned((e + 0x38) as *const f64));
        if !(gasto >= teto) && cpp3_w(e + 0x12) < -300 {
            return 0xa;
        }
    }
    let e = cpp3_linha(this, id);
    if cpp3_w(e + 0x12) < -750 && tipo != 1 && sortear(3) == 0 {
        crate::comparacao::ramo("cpp_3 motivo 2");
        return 2;
    }
    if temporada != 0 && cpp3_b(e + 2) & 8 != 0 {
        return 7;
    }
    if cpp3_w(e + 0x10) < -350 {
        return 3;
    }
    if cpp3_w(e + 0x14) < -750 {
        return 5;
    }
    let humor = cpp3_w(e + 0xa);
    if humor < cpp3_w(e + 6) {
        let a = sortear(cpp3_w(cpp3_linha(this, id) + 6) as i32);
        let b = sortear(humor as i32);
        if b < a {
            return 8;
        }
    }
    if cpp3_w(cpp3_linha(this, id) + 0x16) < -750 && sortear(3) != 0 {
        return 4;
    }
    let tecnico = cpp3_d(clube + 0xcf) as usize;
    if tecnico != 0 {
        let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
        let d = dias(0xae2c90, cpp3_d(tecnico + 0x3e), cpp3_d(tecnico + 0x42));
        if d > sortear(0x7d0) + 0x3e8 {
            return 6;
        }
    }
    0
}

/// Comparador: lado a lado a partir do mesmo estado do sorteador (a rotina sorteia em vários caminhos), pela devolução;
/// a cópia sem contar os sorteios.
unsafe extern "thiscall" fn comparar_motivo_da_saida_do_tecnico(this: usize, id: u32) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u8>(1013);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&crate::aleatorio::REGIOES_DO_ESTADO, || o(this, id),
        || crate::aleatorio::sem_contar(|| motivo_da_saida_do_tecnico(this, id)));
    PLACAR.registrar(igual && r == n, || format!("motivo_da_saida_do_tecnico(clube {}) original={} rust={} {:x?} x {:x?}", id, r, n, mo, mr));
    r
}

/// O registro do técnico (210 bytes) que a 0x86f5b0 olha: humano (id em [0xae2364] − 16 .. [0xae2364]) pela vaga dele
/// depois dos [0xae2368] registros; senão pelo id da ficha de não jogador (+0x69). None: sem ficha, ou humano sem
/// situação (+0x5f 0 ou 5).
unsafe fn cpp3_registro(this: usize, pessoa: usize) -> Option<usize> {
    let n = cpp3_d(0xae2364) as i32;
    let id = cpp3_d(pessoa) as i32;
    let k = if id >= n.wrapping_sub(0x10) && id < n {
        let s = cpp3_b(pessoa + 0x5f);
        if s == 0 || s == 5 {
            return None;
        }
        id.wrapping_sub(n).wrapping_add(cpp3_d(0xae2368) as i32).wrapping_add(0x10)
    } else {
        let ficha = cpp3_d(pessoa + 0x69) as usize;
        if ficha == 0 {
            return None;
        }
        cpp3_d(ficha) as i32
    };
    Some((cpp3_d(this + 8) as usize).wrapping_add((k as usize).wrapping_mul(0xd2)))
}

unsafe fn cpp3_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut ret = [0u8; 8];
    copiar(destino, &mut ret, origem);
}

/// { 0x86f5b0 } (thiscall, ret 8, staff_records.cpp): `Staff_Records::get_manager_record(pessoa, criar)`: o registro de
/// 210 bytes do técnico (vetor +8). Pessoa nula: caixa da linha 0xf6e e 0. Vaga livre (+0x86 = −1): sem `criar`, 0;
/// com ele, a vaga fica com o id. Vaga de outro: zera o registro da ficha dele (−1 em +0x86, +0xc2, +0xc6, +0x4a, +0x5e,
/// +0x72) e olha de novo. Atualiza as nações do clube (+0x39) e da seleção (+0x24) em +0xc2/+0xc6, zerando os pontos
/// (+0xba/+0xbe) quando mudam. Com a data de [0xdd6ec8] até hoje: sem passagem (+0x4a = −1), abre a do clube atual
/// (desde +0x3e da pessoa; +0x56 = a data 13/11/1975 da 0x549f70); a mais longa (+0x5e, +0x62..+0x6a) e a mais curta
/// (+0x72, +0x76..+0x7e) pela 0x54cdb0, a de hoje em +0x6a / +0x7e. Devolve o registro.
unsafe extern "thiscall" fn registro_do_tecnico(this: usize, pessoa: usize, criar: u32) -> usize {
    if pessoa == 0 {
        let separar: unsafe extern "C" fn(usize, usize, usize, *mut u8, *mut u8) = core::mem::transmute(0x944cffusize);
        let versao: unsafe extern "C" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x90d130usize);
        let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
        let caixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x5e8290usize);
        let (mut nome, mut ext) = ([0u8; 256], [0u8; 256]);
        separar(0xa83b78, 0, 0, nome.as_mut_ptr(), ext.as_mut_ptr());
        let v = versao(nome.as_mut_ptr(), ext.as_mut_ptr(), 0xf6e);
        formatar(0xae24d0, 0x9870e8, v, nome.as_ptr(), ext.as_ptr(), 0xf6eu32);
        caixa(0x9870e0, 0xae24d0, 0);
        cpp3_por(0xb67a34, 0);
        return 0;
    }
    let r = loop {
        let Some(r) = cpp3_registro(this, pessoa) else {
            return 0;
        };
        let dono = cpp3_d(r + 0x86) as i32;
        if dono == -1 {
            if criar == 0 {
                return 0;
            }
            crate::comparacao::ramo("cpp_3 criar registro");
            cpp3_por(r + 0x86, cpp3_d(pessoa)); // a vaga fica com ele
            break r;
        }
        if dono == cpp3_d(pessoa) as i32 {
            break r;
        }
        // a vaga é de outro: zera o registro da ficha dele e olha de novo
        let z = (cpp3_d(this + 8) as usize).wrapping_add((cpp3_d(cpp3_d(pessoa + 0x69) as usize) as usize).wrapping_mul(0xd2));
        core::ptr::write_bytes(z as *mut u8, 0, 0xd2);
        for o in [0x86, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
            cpp3_por(z + o, u32::MAX);
        }
    };
    let clube = cpp3_d(pessoa + 0x39) as usize;
    if clube != 0 {
        let nacao = cpp3_d(clube + 0x53) as usize;
        if nacao != 0 && cpp3_d(nacao) != cpp3_d(r + 0xc2) {
            cpp3_por(r + 0xc2, cpp3_d(nacao));
            cpp3_por(r + 0xba, 0);
        }
    }
    let selecao = cpp3_d(pessoa + 0x24) as usize;
    if selecao != 0 {
        let nacao = cpp3_d(selecao + 0x53) as usize;
        if nacao != 0 && cpp3_d(nacao) != cpp3_d(r + 0xc6) {
            cpp3_por(r + 0xc6, cpp3_d(nacao));
            cpp3_por(r + 0xbe, 0);
        }
    }
    let (dia, ano) = (cpp3_w(0xdd6ec8), core::ptr::read_unaligned(0xdd6eca as *const u16));
    let hoje_ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
    if ano > hoje_ano || (ano == hoje_ano && dia > cpp3_w(0xae2c90)) {
        return r; // a data de [0xdd6ec8] ainda não chegou
    }
    if cpp3_d(r + 0x4a) as i32 == -1 {
        let clube = cpp3_d(pessoa + 0x39) as usize;
        if clube != 0 {
            crate::comparacao::ramo("cpp_3 abrir passagem");
            cpp3_por(r + 0x4a, cpp3_d(clube));
            cpp3_data(r + 0x4e, pessoa + 0x3e);
            let mut local = [0u8; 8];
            let construir: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
            let p = construir(&mut local, 0xd, 0xb, 0x7b7, -1);
            cpp3_data(r + 0x56, p);
        }
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    if cpp3_d(r + 0x5e) as i32 == -1 {
        cpp3_por(r + 0x5e, cpp3_d(r + 0x4a));
        cpp3_data(r + 0x62, r + 0x4e);
        cpp3_data(r + 0x6a, 0xae2c90);
    } else if cpp3_d(r + 0x4a) as i32 != -1 {
        let a = dias(r + 0x6a, cpp3_d(r + 0x62), cpp3_d(r + 0x66));
        let b = dias(0xae2c90, cpp3_d(r + 0x4e), cpp3_d(r + 0x52));
        if b > a {
            cpp3_por(r + 0x5e, cpp3_d(r + 0x4a)); // a mais longa
            cpp3_data(r + 0x62, r + 0x4e);
            cpp3_data(r + 0x6a, 0xae2c90);
        }
    }
    if cpp3_d(r + 0x72) as i32 == -1 {
        cpp3_por(r + 0x72, cpp3_d(r + 0x4a));
        cpp3_data(r + 0x76, r + 0x4e);
        cpp3_data(r + 0x7e, 0xae2c90);
        return r;
    }
    let atual = cpp3_d(r + 0x4a);
    if cpp3_d(r + 0x72) == atual && cpp3_w(r + 0x50) == cpp3_w(r + 0x78) && cpp3_w(r + 0x4e) == cpp3_w(r + 0x76) {
        cpp3_data(r + 0x7e, 0xae2c90); // a mais curta é a atual: vai até hoje
        return r;
    }
    if atual as i32 == -1 {
        return r;
    }
    let a = dias(r + 0x7e, cpp3_d(r + 0x76), cpp3_d(r + 0x7a));
    let b = dias(0xae2c90, cpp3_d(r + 0x4e), cpp3_d(r + 0x52));
    if b < a {
        cpp3_por(r + 0x72, cpp3_d(r + 0x4a)); // a mais curta
        cpp3_data(r + 0x76, r + 0x4e);
        cpp3_data(r + 0x7e, 0xae2c90);
    }
    r
}

/// Comparador: lado a lado pelo registro do técnico (e, quando a vaga é de outro, pelo registro que a original zera),
/// pela devolução. Pessoa nula (a caixa) e o humano sem situação: só a original.
unsafe extern "thiscall" fn comparar_registro_do_tecnico(this: usize, pessoa: usize, criar: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> usize>(1014);
    if pessoa == 0 {
        return o(this, pessoa, criar);
    }
    let Some(r) = cpp3_registro(this, pessoa) else {
        return o(this, pessoa, criar);
    };
    let mut reg = vec![(r, 0xd2)];
    let dono = cpp3_d(r + 0x86) as i32;
    if dono != -1 && dono != cpp3_d(pessoa) as i32 {
        let z = (cpp3_d(this + 8) as usize).wrapping_add((cpp3_d(cpp3_d(pessoa + 0x69) as usize) as usize).wrapping_mul(0xd2));
        if z != r {
            reg.push((z, 0xd2));
        }
    }
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa, criar), || registro_do_tecnico(this, pessoa, criar));
    PLACAR.registrar(igual && a == b, || format!("registro_do_tecnico(pessoa {:x}, criar {}) original={:x} rust={:x} {:x?} x {:x?}", pessoa, criar, a, b, mo, mr));
    a
}

// ---------- 252a leva: quem serve para o cargo e a moral depois da partida ruim: o candidato a técnico, o cargo que a pessoa pode assumir e baixar a moral pela partida (manager_manager.cpp, contract_manager.cpp) ----------

unsafe fn cpp4_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cpp4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpp4_b(a: usize) -> u8 {
    // ponytail: volátil porque a original lê [0 + 0x53] quando falta o clube do contrato (cai como ela)
    core::ptr::read_volatile(a as *const u8)
}

/// A reputação "vista" de um clube para o técnico: com nação (+0x53), o peso dela (+0x85) × reputação (+0x80) × 0,05
/// (0x967888); sem, a reputação × 0,65 (0x96a5d0); truncadas (16 bits) e, acima de 0x203a, 0x203a − (excesso × −1,1
/// truncado).
unsafe fn cpp4_reputacao_vista(clube: usize) -> i16 {
    let r = cpp4_w(clube + 0x80) as i32;
    let nacao = cpp4_d(clube + 0x53) as usize;
    let v = if nacao != 0 {
        ftol((cpp4_b(nacao + 0x85) as i8 as i32 * r) as f64 * f64_em(0x967888))
    } else {
        ftol(r as f64 * f64_em(0x96a5d0))
    } as i16;
    if v > 0x203a {
        (0x203a - ftol((v as i32 - 0x203a) as f64 * f64_em(0x96d278))) as i16
    } else {
        v
    }
}

/// { 0x69f810 } (thiscall, ret 0x10, manager_manager.cpp): `MANAGER_MANAGER::is_staff_suitable_for_job(pessoa, clube,
/// nota_minima)`: sem ficha de não jogador (+0x69) ou sem clube: 0. Os vetos (0): reputação de outra nação (+0xc) acima
/// de 4250 de outra nação com peso (+0x85) abaixo de 10; doméstica (+8) acima de 6250, técnico (+0x3d = 5) de outra
/// nação e clube abaixo de 5750; seleção que já é uma das três dele (+0x65 +0x10..+0x18); jogador (0xb) de nota
/// (+0x61 +0xb) acima de 4750 num clube abaixo de 6750, com contrato (0x4cf5a0) de faixa 1..3 (+0x4f >> 4), menos de
/// 180 dias, +0x3f acima de 1, +0x3e zero, menor de 35 com +0x3d positivo, ou de outra nação em clube abaixo de 7250
/// que não é o dele; técnico de clube numa divisão melhor (+0x57 +0x69), ou mais forte, ou recente (dias desde +0x3e
/// abaixo de (+0x59 + 21) × 10) num clube acima de 5250; técnico de seleção (+0x24, +0x28 = 5); clube abaixo de 2250
/// (com reputação +0xa acima de 4750 ou sem ficha); +0x22 acima de 75 num clube abaixo de 3250; não jogador com a
/// aposentadoria pendente (0x7ae350). Os sim (1): clube abaixo de 2500 da nação dele, doméstica abaixo de 1500, sem
/// clube nem seleção; seleção da nação dele com reputação abaixo de 6750, sem clube, ou com o clube dele mais fraco
/// (abaixo de 7750 e 750 abaixo da seleção) há tempo; clube acima de 7750 com a reputação vista acima da do clube dele
/// + 25 × +0x59 + 250. Senão, o interesse dele no cargo (0x844780, float) não abaixo de `nota_minima`.
unsafe extern "thiscall" fn serve_para_o_cargo(_this: usize, pessoa: usize, clube: usize, nota_minima: f64) -> u8 {
    let ficha = cpp4_d(pessoa + 0x69) as usize;
    if ficha == 0 || clube == 0 {
        return 0;
    }
    let rep = |c: usize| cpp4_w(c + 0x80);
    if cpp4_w(ficha + 0xc) > 0x109a {
        let (n1, n2) = (cpp4_d(pessoa + 0x1a) as usize, cpp4_d(clube + 0x53) as usize);
        if n1 != 0 && n2 != 0 && n1 != n2 && (cpp4_b(n2 + 0x85) as i8) < 0xa {
            return 0;
        }
    }
    if cpp4_w(ficha + 8) > 0x186a && cpp4_d(pessoa + 0x1a) != cpp4_d(clube + 0x53) && cpp4_b(pessoa + 0x3d) == 5 && rep(clube) < 0x1676 {
        return 0;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(clube) != 0 {
        let s = cpp4_d(pessoa + 0x65) as usize;
        if s != 0 && (cpp4_d(s + 0x10) as usize == clube || cpp4_d(s + 0x14) as usize == clube || cpp4_d(s + 0x18) as usize == clube) {
            return 0;
        }
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let tempo = |p: usize| dias(0xae2c90, cpp4_d(p + 0x3e), cpp4_d(p + 0x42));
    let limite = |p: usize| (cpp4_b(p + 0x59) as i8 as i32 + 0x15) * 10;
    if cpp4_b(pessoa + 0x3d) == 0xb {
        let jogador = cpp4_d(pessoa + 0x61) as usize;
        if jogador != 0 && cpp4_w(jogador + 0xb) > 0x128e && rep(clube) < 0x1a5e {
            let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
            let c = contrato_de(cpp4_d(0xae1050) as usize, pessoa);
            if c != 0 {
                let faixa = cpp4_b(c + 0x4f) >> 4;
                if faixa != 0 && faixa <= 3 {
                    return 0;
                }
                if dias(0xae2c90, cpp4_d(c + 0x25), cpp4_d(c + 0x29)) < 0xb4 {
                    return 0;
                }
                if cpp4_b(c + 0x3f) as i8 > 1 || cpp4_b(c + 0x3e) == 0 {
                    return 0;
                }
                if (cpp4_b(pessoa + 0x18) as i8) < 0x23 && cpp4_b(c + 0x3d) as i8 > 0 {
                    return 0;
                }
                if cpp4_d(pessoa + 0x1a) != cpp4_d(clube + 0x53) && rep(clube) < 0x1c52 && cpp4_d(pessoa + 0x39) as usize != clube {
                    return 0;
                }
            }
        }
    }
    if cpp4_b(pessoa + 0x3d) == 5 {
        let liga = cpp4_d(clube + 0x57) as usize;
        let atual = cpp4_d(pessoa + 0x39) as usize;
        if liga != 0 && atual != 0 && cpp4_d(atual + 0x57) != 0 {
            let liga2 = cpp4_d(atual + 0x57) as usize;
            if cpp4_w(liga + 0x69) < cpp4_w(liga2 + 0x69) {
                return 0; // o clube dele está numa divisão melhor
            }
            if liga == liga2 && (rep(clube) as i32 - 0x4e2) < rep(atual) as i32 {
                return 0;
            }
            if rep(atual) > rep(clube) {
                return 0;
            }
            if tempo(pessoa) < limite(pessoa) && rep(cpp4_d(pessoa + 0x39) as usize) > 0x1482 {
                return 0;
            }
        }
    }
    if cpp4_d(pessoa + 0x24) != 0 && cpp4_b(pessoa + 0x28) == 5 {
        return 0;
    }
    let ficha = cpp4_d(pessoa + 0x69) as usize;
    if (ficha == 0 || cpp4_w(ficha + 0xa) > 0x128e) && rep(clube) < 0x8ca {
        return 0;
    }
    if cpp4_b(pessoa + 0x22) > 0x4b && rep(clube) < 0xcb2 {
        return 0;
    }
    if cpp4_d(pessoa + 0x61) == 0 {
        let pendente: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x7ae350usize);
        if pendente(cpp4_d(0xb7434c) as usize, pessoa) != 0 {
            return 0;
        }
    }
    if rep(clube) < 0x9c4 && cpp4_d(pessoa + 0x1a) == cpp4_d(clube + 0x53) && cpp4_w(cpp4_d(pessoa + 0x69) as usize + 8) < 0x5dc
        && cpp4_d(pessoa + 0x39) == 0 && cpp4_d(pessoa + 0x24) == 0 {
        return 1;
    }
    if selecao(clube) != 0 && cpp4_d(pessoa + 0x1a) == cpp4_d(clube + 0x53) {
        if cpp4_w(cpp4_d(pessoa + 0x69) as usize + 0xa) < 0x1a5e {
            return 1;
        }
        if cpp4_d(pessoa + 0x39) == 0 {
            return 1;
        }
        if tempo(pessoa) > limite(pessoa) {
            let r = rep(cpp4_d(pessoa + 0x39) as usize);
            if r < 0x1e46 && (r as i32 + 0x2ee) < rep(clube) as i32 {
                return 1;
            }
        }
    }
    if rep(clube) > 0x1e46 {
        let vista = cpp4_reputacao_vista(clube);
        let atual = cpp4_d(pessoa + 0x39) as usize;
        if atual != 0 {
            let dele = cpp4_reputacao_vista(atual);
            let teto = (cpp4_b(pessoa + 0x59) as i8 as i16).wrapping_mul(25) as i32 + dele as i32 + 0xfa;
            if vista as i32 > teto {
                return 1;
            }
        }
    }
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> f64 = core::mem::transmute(0x844780usize);
    let nota = interesse(cpp4_d(cpp4_d(0xde1f54) as usize + 0x856) as usize, pessoa, clube, 0, 5, 0);
    if nota >= nota_minima { 1 } else { 0 } // fcomp: abaixo (ou sem ordem) é 0
}

/// Comparador: a rotina só lê (o interesse 0x844780 sem a saída das razões): pela devolução.
unsafe extern "thiscall" fn comparar_serve_para_o_cargo(this: usize, pessoa: usize, clube: usize, nota_minima: f64) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, f64) -> u8>(1015)(this, pessoa, clube, nota_minima);
    let n = serve_para_o_cargo(this, pessoa, clube, nota_minima);
    PLACAR.registrar(r == n, || format!("serve_para_o_cargo(pessoa {:x}, clube {:x}, {}) original={} rust={}", pessoa, clube, nota_minima, r, n));
    r
}

/// { 0x4cb990 } (stdcall, ret 8, contract_manager.cpp): `staff_can_take_job(cargo, pessoa)`: pela tabela de salto
/// 0x4cbaf4 (cargo − 6, 0..9; fora: 0); 7, 12 e 14: 0; pessoa nula: 0 (o regen de não jogador 0x7ab9f2 pergunta sempre
/// com nula). 6: com 30 anos (+0x18) ou mais, não jogador (+0x69), acima de 36, ou com +0x57 abaixo de 10 e +0x2f do
/// jogador 12 ou mais: 1; senão, não jogador no cargo 8, 6 ou 5. 8: de 30 anos, não jogador ou acima de 36 ou (+0x2f ≥ 12
/// e +0x57 < 10) fora dos cargos 5 e 10: 1; senão como o 6. 9: não jogador no cargo 9. 10: não jogador no cargo 10 ou com
/// +0x28 = 10. 11: jogador (+0x61). 13 e 15: jogador e não jogador, ou acima de 34, ou de 30 anos com +0x2f ≥ 12.
unsafe extern "stdcall" fn pode_assumir_o_cargo(cargo: u32, pessoa: usize) -> u32 {
    let k = (cargo as u8 as i8 as i32).wrapping_sub(6);
    if k as u32 > 9 || k == 1 || k == 6 || k == 8 {
        return 0;
    }
    if pessoa == 0 {
        crate::comparacao::ramo("cpp_4 pessoa nula");
        return 0; // pessoa nula
    }
    let p = pessoa;
    let idade = cpp4_b(p + 0x18) as i8;
    let nao_jogador = cpp4_d(p + 0x69);
    let funcao = cpp4_b(p + 0x3d);
    let forca = || cpp4_b(cpp4_d(p + 0x61) as usize + 0x2f) as i8; // +0x2f do jogador
    // o fim comum (0x4cb9db): não jogador nos cargos 8, 6 ou 5
    let comum = || nao_jogador != 0 && (funcao == 8 || funcao == 6 || funcao == 5);
    let r = match k {
        0 => {
            if idade >= 0x1e {
                if nao_jogador != 0 || idade > 0x24 {
                    return 1;
                }
                if (cpp4_b(p + 0x57) as i8) < 0xa && forca() >= 0xc {
                    return 1;
                }
            }
            comum()
        }
        2 => {
            if idade < 0x1e {
                comum()
            } else if nao_jogador == 0 && idade <= 0x24 && (forca() < 0xc || cpp4_b(p + 0x57) as i8 >= 0xa) {
                comum()
            } else if funcao == 5 || funcao == 0xa {
                comum()
            } else {
                true
            }
        }
        3 => nao_jogador != 0 && funcao == 9,
        4 => nao_jogador != 0 && (funcao == 0xa || cpp4_b(p + 0x28) == 0xa),
        5 => cpp4_d(p + 0x61) != 0,
        _ => {
            // 13 e 15
            let jogador = cpp4_d(p + 0x61) as usize;
            jogador != 0 && (nao_jogador != 0 || idade > 0x22 || (idade >= 0x1e && cpp4_b(jogador + 0x2f) as i8 >= 0xc))
        }
    };
    r as u32
}

unsafe extern "stdcall" fn comparar_pode_assumir_o_cargo(cargo: u32, pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(u32, usize) -> u32>(1016)(cargo, pessoa);
    let n = pode_assumir_o_cargo(cargo, pessoa);
    PLACAR.registrar(r == n, || format!("pode_assumir_o_cargo(cargo {}, pessoa {:x}) original={} rust={}", cargo as u8 as i8, pessoa, r, n));
    r
}

/// Os dias (0..400) desde a data (dia +0, ano +2 sem sinal) até hoje ([0xae2c90]), com o ano de 365 dias.
unsafe fn cpp4_dias_desde(dia: usize, ano: usize) -> i32 {
    let hoje = core::ptr::read_unaligned(0xae2c92 as *const u16) as i32;
    let a = core::ptr::read_unaligned(ano as *const u16) as i32;
    let d = (hoje - a) * 365 - cpp4_w(dia) as i32 + cpp4_w(0xae2c90) as i32;
    d.clamp(0, 400)
}

/// A queda da moral (o byte `m` da original) pela partida ruim, ou None quando a rotina sai sem mexer (algum nulo, o
/// técnico ou o auxiliar do clube +0xcf/+0xd3, ou fora do cargo 0xb). x = (nota +0xb − reputação do clube) + 50 × dias
/// no clube (desde +0x3e, até 400) + 417 × idade − 31680; com os dias do contrato (desde +0x2d) acima de 400 (nunca: o
/// teto é 400) e idade 24..31: + 250 × +0x57 + 5000; senão, com +0x22 e a reputação da nação (+0x1a +0x8e) acima de 3500
/// e menos de 32 anos: + 250 × +0x57 + 25 × +0x22 + 2500; senão pela faixa do contrato (+0x4f >> 4): 1: + 250 × +0x57 +
/// 5000; 2: + 125 × +0x57 + 2500; 3: + 1000 × +0x3e; nesses casos no máximo 15000. y = 1 − x × (6,667e-4 se x < 0, senão
/// 6e-5), entre 0,1 e 10; m = ftol((+0x3e + 4) / y + 0,5) (byte). Dupla precisão; as constantes do .rdata.
unsafe fn cpp4_queda_da_moral(pessoa: usize, clube: usize, contrato: usize) -> Option<i8> {
    if contrato == 0 || clube == 0 || pessoa == 0 {
        return None;
    }
    if pessoa == cpp4_d(clube + 0xcf) as usize || pessoa == cpp4_d(clube + 0xd3) as usize || cpp4_b(pessoa + 0x3d) != 0xb {
        return None;
    }
    let a = cpp4_dias_desde(pessoa + 0x3e, pessoa + 0x40);
    let b = cpp4_dias_desde(contrato + 0x2d, contrato + 0x2f);
    let idade = cpp4_b(pessoa + 0x18) as i8;
    let jogador = cpp4_d(pessoa + 0x61) as usize;
    let dif = cpp4_w(jogador + 0xb) as i32 - cpp4_w(clube + 0x80) as i32;
    let mut x = ((dif as f64 + (a * 50) as f64) + idade as f64 * f64_em(0x968d90)) - f64_em(0x968db8);
    let c57 = cpp4_b(pessoa + 0x57) as i8 as i32;
    let teto = f32_em(0x968da8) as f64;
    let limitar = |x: f64| if x >= teto { teto } else { x };
    let al = cpp4_b(pessoa + 0x22);
    if b > 0x190 && idade > 0x17 && idade < 0x20 {
        x = limitar(x + (c57 * 250) as f64 + f32_em(0x968db0) as f64); // nunca: b vai até 400
    } else if al != 0 && cpp4_w(cpp4_d(pessoa + 0x1a) as usize + 0x8e) > 0xdac && idade < 0x20 {
        x = limitar(x + (c57 * 250 + al as i32 * 25) as f64 + f32_em(0x968dac) as f64);
    } else {
        match (cpp4_b(contrato + 0x4f) >> 4) as i8 {
            1 => x = limitar(x + (c57 * 250) as f64 + f32_em(0x968db0) as f64),
            2 => x = limitar(x + (c57 * 125) as f64 + f32_em(0x968dac) as f64),
            3 => x = limitar(x + (cpp4_b(contrato + 0x3e) as i8 as i32 * 1000) as f64),
            _ => {}
        }
    }
    let y = if !(x >= f64_em(0x9689a0)) { x * f64_em(0x968da0) } else { x * f64_em(0x968d98) };
    let mut y = f64_em(0x967890) - y;
    if !(y >= f64_em(0x967880)) {
        y = f64_em(0x967880);
    } else if y > f64_em(0x9688b0) {
        y = f64_em(0x9688b0);
    }
    let d = (cpp4_b(contrato + 0x3e) as i8 as i32 + 4) as f64;
    Some(ftol(d / y + f64_em(0x968968)) as u8 as i8)
}

/// A moral (+0x44) menos `m`, entre −100 e 100 (a soma em byte quando cabe).
fn cpp4_moral_nova(moral: i8, m: i8) -> i8 {
    let menos = m.wrapping_neg();
    let s = moral as i32 + menos as i32;
    if s > 100 {
        100
    } else if s < -100 {
        -100
    } else {
        moral.wrapping_add(menos)
    }
}

/// O contrato principal da pessoa no CONTRACT_MANAGER global e o clube dele, como a original os refaz (a ficha
/// [0xae1080] gravada); sem contrato o clube é nulo.
unsafe fn cpp4_clube_do_contrato(pessoa: usize) -> usize {
    let c = if cpp4_d(pessoa) as i32 >= cpp4_d(0xae2364) as i32 {
        0
    } else {
        let k = cpp4_d((cpp4_d(0xae2d0c) as usize).wrapping_add((cpp4_d(pessoa) as usize).wrapping_mul(0x4f))) as i32;
        if k < 0 { 0 } else { (cpp4_d(cpp4_d(0xae1050) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) }
    };
    core::ptr::write_unaligned(0xae1080 as *mut u32, c as u32);
    if c == 0 { 0 } else { (cpp4_d(0xae23b4) as usize).wrapping_add((cpp4_d(c + 4) as usize).wrapping_mul(0x245)) }
}

/// A janela de transferências aberta para o clube do contrato da pessoa: o índice da nação-chave (0x669340 da nação do
/// clube; sem nação, o byte baixo do ponteiro do clube que a original lê da vaga do argumento) dá as duas datas da
/// tabela 0xb63d8a (72 bytes por nação): hoje depois da primeira e antes da segunda.
unsafe fn cpp4_janela_aberta(pessoa: usize, clube: usize) -> bool {
    let c1 = cpp4_clube_do_contrato(pessoa);
    let k = if cpp4_d(c1 + 0x53) != 0 {
        let c2 = cpp4_clube_do_contrato(pessoa);
        let indice: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669340usize);
        indice(cpp4_d(c2 + 0x53) as usize) as u8 as i8
    } else {
        // ponytail: a original lê o byte baixo da vaga do argumento `clube` (o ponteiro), não um índice
        clube as u8 as i8
    };
    if k < 0 || k >= 0x23 {
        return false;
    }
    let t = 0xb63d8ausize + (k as usize) * 0x48;
    let dia = cpp4_w(0xae2c90);
    let ano = core::ptr::read_unaligned(0xae2c92 as *const u16);
    let ano_de = |a: usize| core::ptr::read_unaligned(a as *const u16);
    let depois = (dia > cpp4_w(t) && ano == ano_de(t + 2)) || ano > ano_de(t + 2);
    let antes = (dia < cpp4_w(t + 0xf) && ano == ano_de(t + 0x11)) || ano < ano_de(t + 0x11);
    depois && antes
}

/// { 0x4cb250 } (thiscall, ret 0xc, contract_manager.cpp): `contract_manager_lower_match_morale(pessoa, clube,
/// contrato)`: a queda `m` (cpp4_queda_da_moral). Machucado (0x62c400), emprestado (contrato de empréstimo), aposentando
/// (0x7a7eb0), menor de 18 ou com +0x49: apaga as queixas 4 e 0x10 (0x8674e0). Senão, com m > 5 e a moral − m abaixo de
/// −30 − +0x59: acima de 21 anos, mais forte (+0xb) que o clube e acima de 2250, ou de potencial (+0xd) acima da
/// reputação − 1500 e de 500 com sorteio(+0x57) ≠ 0 e +0x3d negativo: na janela de transferências do clube do contrato,
/// a queixa 4 (0x8674e0 e 0x867700 com 4); no principal (0x4cbb60 = 1), nota acima de 1250, mais de 23 anos e
/// sorteio(+0x59 + 5) = 0: a queixa 0x10; até 21 anos, no principal com sorteio(+0x57) > 5, mais forte que o clube e
/// acima de 1250: a queixa 0x10. Por fim a moral (+0x44) cai m, entre −100 e 100.
unsafe extern "thiscall" fn baixar_moral_pela_partida(this: usize, pessoa: usize, clube: usize, contrato: usize) {
    let Some(m) = cpp4_queda_da_moral(pessoa, clube, contrato) else {
        return;
    };
    let apagar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8674e0usize);
    let marcar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x867700usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let aposentando: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x7a7eb0usize);
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4cbb60usize);
    let emprestado = || {
        if cpp4_d(pessoa) as i32 >= cpp4_d(0xae2364) as i32 {
            return false;
        }
        let k = cpp4_d((cpp4_d(0xae2d0c) as usize).wrapping_add((cpp4_d(pessoa) as usize).wrapping_mul(0x4f)) + 4) as i32;
        k >= 0 && (cpp4_d(cpp4_d(0xae1050) as usize) as usize).wrapping_add((k as usize).wrapping_mul(0x50)) != 0
    };
    let idade = cpp4_b(pessoa + 0x18) as i8;
    if lesao(cpp4_d(0xb63d28) as usize, pessoa) != 0 || emprestado() || aposentando(cpp4_d(0xb7434c) as usize, pessoa) != 0
        || idade < 0x12 || cpp4_b(contrato + 0x49) != 0 {
        if cpp4_d(contrato + 0x45) & 4 != 0 {
            apagar(contrato, 4);
        }
        if cpp4_b(contrato + 0x45) & 0x10 != 0 {
            apagar(contrato, 0x10);
        }
    } else if m > 5 && (cpp4_b(contrato + 0x44) as i8 as i32 - m as i32) < -30 - cpp4_b(pessoa + 0x59) as i8 as i32 {
        let jogador = cpp4_d(pessoa + 0x61) as usize;
        if idade > 0x15 {
            let nota = cpp4_w(jogador + 0xb);
            if nota > cpp4_w(clube + 0x80) && nota > 0x8ca {
                if cpp4_janela_aberta(pessoa, clube) {
                    crate::comparacao::ramo("cpp_4 queixa na janela");
                    apagar(contrato, 4);
                    marcar(contrato, 4, 0);
                }
            } else {
                let mut feito = false;
                if lugar(this, pessoa, clube) == 1 && cpp4_w(jogador + 0xb) > 0x4e2 && idade > 0x17
                    && sortear(cpp4_b(pessoa + 0x59) as i8 as i32 + 5) == 0 {
                    apagar(contrato, 4);
                    marcar(contrato, 0x10, 0);
                    feito = true;
                }
                if !feito {
                    let pot = cpp4_w(jogador + 0xd);
                    if (pot as i32) > cpp4_w(clube + 0x80) as i32 - 0x5dc && pot > 0x1f4
                        && sortear(cpp4_b(pessoa + 0x57) as i8 as i32) != 0 && (cpp4_b(contrato + 0x3d) as i8) < 0
                        && cpp4_janela_aberta(pessoa, clube) {
                        apagar(contrato, 4);
                        marcar(contrato, 4, 0);
                    }
                }
            }
        } else if lugar(this, pessoa, clube) == 1 && sortear(cpp4_b(pessoa + 0x57) as i8 as i32) > 5
            && cpp4_w(jogador + 0xb) > cpp4_w(clube + 0x80) && cpp4_w(jogador + 0xb) > 0x4e2 {
            apagar(contrato, 4);
            marcar(contrato, 0x10, 0);
        }
    }
    let p = (contrato + 0x44) as *mut i8;
    *p = cpp4_moral_nova(*p, m); // a moral cai
}

/// Comparador: as queixas (0x867700) sorteiam e podem abrir pedido de transferência: só a original; o plano (a queda
/// `m`, calculada antes, sem efeito) diz a moral +0x44 que ela deixa (as queixas não a gravam) ou que ela não mexe.
unsafe extern "thiscall" fn comparar_baixar_moral_pela_partida(this: usize, pessoa: usize, clube: usize, contrato: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1017);
    if contrato == 0 {
        return o(this, pessoa, clube, contrato);
    }
    let antes = cpp4_b(contrato + 0x44) as i8;
    let plano = cpp4_queda_da_moral(pessoa, clube, contrato);
    // apagar uma queixa (0x8674e0) recalcula o contrato (0x867240), que pode regravar a moral: com queixa marcada antes
    // ou queda acima de 5 (os ramos que apagam sempre), só a original
    if cpp4_b(contrato + 0x45) & 0x14 != 0 || plano.is_some_and(|m| m > 5) {
        return o(this, pessoa, clube, contrato);
    }
    o(this, pessoa, clube, contrato);
    let depois = cpp4_b(contrato + 0x44) as i8;
    let esperada = plano.map_or(antes, |m| cpp4_moral_nova(antes, m));
    PLACAR.registrar(depois == esperada, || format!("baixar_moral_pela_partida(queda {:?}) moral {} -> original {} plano {}", plano, antes, depois, esperada));
}

// ---------- 252a leva: a lista de alvos dos clubes: as vagas da formação, refazer a lista do clube e suspender os clubes que miram o jogador (shortlist_manager.cpp) ----------

unsafe fn cpp5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp5_w(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// A entrada de 9 bytes do clube na Shortlist_Manager (+0xdc; +0 as 40 vagas de 0x25 bytes, +8 o estado).
unsafe fn cpp5_entrada(this: usize, clube: usize) -> usize {
    cpp5_d(this + 0xdc).wrapping_add(cpp5_d(clube).wrapping_mul(9))
}

/// A primeira diferença entre duas leituras de memória (endereço, original, Rust), para a mensagem.
fn cpp5_dif(a: &crate::comparacao::Memoria, b: &crate::comparacao::Memoria) -> Option<(usize, u8, u8)> {
    for ((ea, va), (_, vb)) in a.0.iter().zip(b.0.iter()) {
        if let Some(k) = (0..va.len()).find(|&k| va[k] != vb[k]) {
            return Some((ea + k, va[k], vb[k]));
        }
    }
    None
}

/// As máscaras de posição (+1, 16 bits) das vagas 1..24 de jogador, por formação (+7 da entrada); a vaga 0 é sempre
/// o goleiro (1). A linha 0 vale para formação fora de 1..9 (o `default` do switch).
static CPP5_MASCARAS: [[u16; 24]; 10] = [
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x440, 0x140, 0x4, 0x8, 0x10, 0x10, 0x20, 0x40, 0x4, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // fora de 1..9
    [0x808, 0x88, 0x404, 0x2, 0x104, 0x410, 0x220, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x2, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formação 1
    [0x808, 0x88, 0x404, 0x204, 0x104, 0x410, 0x220, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x204, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formação 2
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x440, 0x140, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formação 3
    [0x804, 0x84, 0x404, 0x104, 0x210, 0x90, 0x810, 0x140, 0x440, 0x240, 0x4, 0x4, 0x10, 0x10, 0x140, 0x440, 0x4, 0x10, 0x1, 0x40, 0x4, 0x10, 0x40, 0x1], // formação 4
    [0x804, 0x84, 0x404, 0x104, 0x820, 0xa0, 0x410, 0x110, 0x440, 0x140, 0x4, 0x4, 0x10, 0x20, 0x440, 0x140, 0x204, 0x10, 0x1, 0x40, 0x4, 0x20, 0x10, 0x1], // formação 5
    [0x810, 0x90, 0x404, 0x204, 0x104, 0x410, 0x208, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x204, 0x8, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 6
    [0x804, 0x84, 0x404, 0x104, 0x208, 0x220, 0x410, 0x110, 0x140, 0x440, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 7
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x210, 0x240, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 8
    [0x204, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x210, 0x140, 0x440, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 9
];

/// { 0x83c090 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_set_up_formation_slots(entrada)`: nas 40 vagas de
/// 0x25 bytes da entrada (+0), as 25 primeiras viram vagas de jogador (+0 = 0xb) e as 15 seguintes recebem o papel
/// (6, 7, 8 ×5, 10, 9 ×7) com +0x24 = (+0x24 & 0xe3) | 2; depois grava a máscara de posição (+1, 16 bits) das 25 vagas
/// de jogador pela formação (+7, byte com sinal; fora de 1..9 a linha padrão). A tabela saiu da emulação do switch
/// (cada caso grava as 25 vagas uma vez só).
unsafe extern "stdcall" fn montar_vagas_da_formacao(entrada: usize) {
    for k in 0..40usize {
        let s = cpp5_d(entrada) + k * 0x25;
        if k < 25 {
            *(s as *mut u8) = 0xb;
        } else {
            let b = (s + 0x24) as *mut u8;
            *b = (*b & 0xe3) | 2;
            *(s as *mut u8) = match k - 25 {
                0 => 6,
                1 => 7,
                2..=6 => 8,
                7 => 10,
                _ => 9,
            };
        }
    }
    let f = *((entrada + 7) as *const i8) as i32;
    let linha = if (1..=9).contains(&f) { f as usize } else { 0 };
    crate::comparacao::ramo("cpp_5 vagas da formacao");
    let v = cpp5_d(entrada);
    cpp5_w(v + 1, 1); // o goleiro
    for k in 1..25usize {
        cpp5_w(v + 1 + k * 0x25, CPP5_MASCARAS[linha][k - 1]);
    }
}

/// { 0x83a730 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::update_club_shortlist(clube)`: solta os
/// alvos velhos das vagas do clube (0x844660 com a entrada), reavalia o elenco (0x83cb40 com a entrada recalculada,
/// o clube, diário 1, refazer 0) e redistribui a verba de transferências (0x83b6e0).
unsafe extern "thiscall" fn refazer_lista_do_clube(this: usize, clube: usize) {
    let soltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x844660usize);
    let avaliar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x83cb40usize);
    let verba: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x83b6e0usize);
    soltar(this, clube, cpp5_entrada(this, clube));
    avaliar(this, cpp5_entrada(this, clube), clube, 1, 0);
    crate::comparacao::ramo("cpp_5 refazer lista");
    verba(this, clube); // a verba de transferências
}

/// { 0x842b00 } (thiscall, ret 8, shortlist_manager.cpp, quadro SEH só para a lista nova):
/// `Shortlist_Manager::suspend_clubs_shortlisting_player(jogador, limpar)`: numa LLIST nova (0x944cf1(0xe) e 0x68d020)
/// junta os clubes que miram o jogador (0x842180 com ignorar 1, modo 3, todos); para cada item (0x68d1b0, depois de
/// recomeçar o cursor +0xa = +2) põe o estado da entrada do clube (+8) em 100 e, com `limpar`, tira a marca de lance
/// (+0x17 = −1 quando > −1) das vagas que têm o jogador (+0xf = id). Destrói e libera a lista (0x68d040, 0x944c94).
/// A contagem (+0, 16 bits com sinal) é relida a cada volta, como na original.
unsafe extern "thiscall" fn suspender_clubes_que_miram_o_jogador(this: usize, jogador: usize, limpar: u8) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let p = novo(0xe);
    let lista = if p != 0 { lista_nova(p) } else { 0 };
    interessados(this, jogador, lista, 1, 3, 0);
    let n = *(lista as *const i16);
    if n != 0 {
        core::ptr::write_unaligned((lista + 0xa) as *mut usize, cpp5_d(lista + 2));
    }
    if n > 0 {
        let mut i = 0i32;
        while i < *(lista as *const i16) as i32 {
            let item = proximo(lista);
            if item != 0 {
                let e = cpp5_d(this + 0xdc).wrapping_add(cpp5_d(item + 4).wrapping_mul(9));
                crate::comparacao::ramo("cpp_5 suspender clube");
                *((e + 8) as *mut u8) = 0x64; // o clube suspende o jogador
                if limpar != 0 {
                    for k in 0..0x28usize {
                        let s = cpp5_d(e) + k * 0x25;
                        if cpp5_d(s + 0xf) == cpp5_d(jogador) && core::ptr::read_unaligned((s + 0x17) as *const i32) > -1 {
                            core::ptr::write_unaligned((s + 0x17) as *mut i32, -1);
                        }
                    }
                }
            }
            i += 1;
        }
    }
    if lista != 0 {
        destruir(lista);
        liberar(lista);
    }
}

/// Comparador: lado a lado pelas 40 vagas da entrada (sem chamadas).
unsafe extern "stdcall" fn comparar_montar_vagas_da_formacao(entrada: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(1018);
    let reg = [(cpp5_d(entrada), 40 * 0x25)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(entrada), || montar_vagas_da_formacao(entrada));
    PLACAR.registrar(igual, || format!("montar_vagas_da_formacao(formacao {}) primeira diferença {:x?}", *((entrada + 7) as *const i8), cpp5_dif(&mo, &mr)));
}

/// Comparador: a 0x83cb40 reavalia o elenco (sorteios, ofertas, vagas de muitos clubes): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_refazer_lista_do_clube(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1019)(this, clube)
}

/// Comparador: lado a lado pelas entradas (9 bytes) e, com `limpar`, pelas vagas (40 × 0x25) dos clubes do plano: os
/// clubes que a 0x842180 (pura) junta antes, numa lista própria. Cada lado cria e libera a sua lista.
unsafe extern "thiscall" fn comparar_suspender_clubes_que_miram_o_jogador(this: usize, jogador: usize, limpar: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u8)>(1020);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let p = novo(0xe);
    if p == 0 {
        return o(this, jogador, limpar);
    }
    let lista = lista_nova(p);
    interessados(this, jogador, lista, 1, 3, 0);
    // os itens (8 bytes: id do jogador, id do clube) pelos nós (+2 o primeiro; nó: +0 o dado, +6 o próximo)
    let mut reg: Vec<(usize, usize)> = Vec::new();
    let mut no = cpp5_d(lista + 2);
    while no != 0 {
        let e = cpp5_d(this + 0xdc).wrapping_add(cpp5_d(cpp5_d(no) + 4).wrapping_mul(9));
        reg.push((e, 9));
        if limpar != 0 {
            reg.push((cpp5_d(e), 0x28 * 0x25));
        }
        no = cpp5_d(no + 6);
    }
    destruir(lista);
    liberar(lista);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador, limpar), || suspender_clubes_que_miram_o_jogador(this, jogador, limpar));
    PLACAR.registrar(igual, || format!("suspender_clubes_que_miram_o_jogador(jogador {:x}, limpar {}, {} regiões) primeira diferença {:x?}", cpp5_d(jogador), limpar, reg.len(), cpp5_dif(&mo, &mr)));
}

// ---------- 252a leva: pôr o jogador à venda, pô-lo nas listas de alvos dos clubes e pagar o prêmio por jogos da transferência (shortlist_manager.cpp, transfer_manager.cpp) ----------

unsafe fn cpp6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp6_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpp6_clube(id: usize) -> usize {
    cpp6_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0 no CONTRACT_MANAGER [0xae1050]).
unsafe fn cpp6_contrato(pessoa: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    contrato_de(cpp6_d(0xae1050), pessoa)
}

/// O contrato da pessoa, guardado em [0xae1080] como a original faz depois de cada chamada.
unsafe fn cpp6_contrato_guardado(pessoa: usize) -> usize {
    let c = cpp6_contrato(pessoa);
    *(0xae1080 as *mut usize) = c;
    c
}

/// { 0x842c00 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::add_player_to_club_shortlists(jogador,
/// forçar)`: com ficha de posições (+0x61; sem forçar, nota +0xb ≥ 0x157c), fora da marca 0x10 dos bits 0x30 do registro
/// de 79 bytes (+0xb), jogador (+0x3d = 0xb), não em fim de carreira (+0x24 com +0x28 = 5), fora da nação [0x9cf458]
/// (+0x1a) e sem ser intocável (0x8cca10 = 0): se chegou ao clube há menos de 200 dias (hoje − 200, 0x54c8e0, antes da
/// data +0x3e/+0x40) e o contrato não está à venda (+0x4f nibble alto ≠ 7) nem tem +0x45, sai. Senão, com o clube do
/// contrato (guardado em [0xae1080]), o contador (nibble baixo do +0xb do registro de 79 bytes) e a máscara de
/// posições (0x5b7d20 com os vizinhos: 2→4, 8→0x10, 0x20→0x10, 0x80→0x100, 0x800→0x400), percorre os clubes do vetor
/// [0xae2d1c] ([0xae235c] relido a cada volta) até o contador chegar a 5: fora seleção (0x53b900), o clube do contrato e
/// o do jogador (+0x39), tenta a 0x842e10 e conta cada clube que o pôs na lista.
unsafe extern "thiscall" fn por_jogador_nas_listas_dos_clubes(this: usize, jogador: usize, forcar: u32) {
    let fichas = cpp6_d(jogador + 0x61);
    if fichas == 0 {
        return;
    }
    if forcar == 0 && core::ptr::read_unaligned((fichas + 0xb) as *const i16) < 0x157c {
        return;
    }
    let registro = |j: usize| cpp6_d(0xae2d0c).wrapping_add(cpp6_d(j).wrapping_mul(0x4f));
    if cpp6_b(registro(jogador) + 0xb) & 0x30 == 0x10 {
        return;
    }
    if cpp6_b(jogador + 0x3d) != 0xb {
        return;
    }
    if cpp6_d(jogador + 0x24) != 0 && cpp6_b(jogador + 0x28) == 5 {
        return;
    }
    let nacao = cpp6_d(jogador + 0x1a);
    if nacao != 0 && cpp6_d(nacao) == cpp6_d(0x9cf458) {
        return;
    }
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    if fora(cpp6_d(0xde1f54), jogador, 0) != 0 {
        return;
    }
    if cpp6_d(jogador + 0x39) != 0 {
        let menos: unsafe extern "thiscall" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x54c8e0usize);
        let mut t = [0u32; 2];
        let p = menos(0xae2c90, t.as_mut_ptr(), 200);
        let lo = cpp6_d(p) as u32;
        let (dia, ano) = (lo as u16 as i16, (lo >> 16) as u16);
        let ano_j = core::ptr::read_unaligned((jogador + 0x40) as *const u16);
        let dia_j = core::ptr::read_unaligned((jogador + 0x3e) as *const i16);
        let recente = if ano != ano_j { ano < ano_j } else { dia < dia_j };
        if recente {
            let c = cpp6_contrato(jogador);
            if c != 0 && cpp6_b(c + 0x4f) & 0xf0 != 0x70 && cpp6_d(c + 0x45) == 0 {
                return;
            }
        }
    }
    let c = cpp6_contrato_guardado(jogador);
    let clube_do_contrato = if c != 0 { cpp6_clube(cpp6_d(c + 4)) } else { 0 };
    let mut conta = (cpp6_b(registro(jogador) + 0xb) & 0xf) as i8;
    let mascara: unsafe extern "C" fn(usize) -> u16 = core::mem::transmute(0x5b7d20usize);
    // ponytail: a 0x5b7d20 devolve só ax e a 0x842e10 usa 16 bits da máscara: os 16 de cima vão zerados
    let mut m = mascara(cpp6_d(jogador + 0x61)) as u32;
    if m & 2 != 0 {
        m |= 4;
    }
    if m & 8 != 0 {
        m |= 0x10;
    }
    if m & 0x20 != 0 {
        m |= 0x10;
    }
    if m & 0x80 != 0 {
        m |= 0x100;
    }
    if m & 0x800 != 0 {
        m |= 0x400;
    }
    if *(0xae235c as *const i32) <= 0 {
        return;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x842e10usize);
    let clubes = cpp6_d(0xae2d1c);
    let mut i = 0i32;
    loop {
        let clube = cpp6_clube(cpp6_d(clubes + i as usize * 4));
        if conta >= 5 {
            return;
        }
        if selecao(clube) == 0 && clube_do_contrato != clube && cpp6_d(jogador + 0x39) != clube && acrescentar(this, jogador, clube, m) != 0 {
            crate::comparacao::ramo("cpp_6 clube acrescentou");
            conta = conta.wrapping_add(1); // mais um clube com ele na lista
        }
        i += 1;
        if i >= *(0xae235c as *const i32) {
            return;
        }
    }
}

/// { 0x842320 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::transfer_list_player(jogador, notícia)`:
/// se o clube ainda não o listou (0x5444a0 com 0) e há contrato: marca o contrato à venda (+0x4e bit 1, +0x4f nibble
/// alto 7); se não é intocável (0x8cca10): com o jogador gostando do clube (0x547d60) ou com a lealdade (+0x44 do
/// contrato, com sinal) acima da diferença de dois sorteios(100), o jogador passa a desgostar do técnico do clube
/// (0x547af0 com o +0xcf do clube, 1); com `notícia`, põe o jogador nas listas dos clubes (0x842c00 com 1) e dá a
/// notícia (0x4d71b0 com o clube +0x39).
unsafe extern "thiscall" fn por_jogador_a_venda(this: usize, jogador: usize, noticia: u32) {
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    if listada(jogador, 0) != 0 {
        return;
    }
    let c = cpp6_contrato(jogador);
    if c == 0 {
        return;
    }
    let a = cpp6_b(c + 0x4f);
    let b = cpp6_b(c + 0x4e);
    *((c + 0x4e) as *mut u8) = b | 1;
    crate::comparacao::ramo("cpp_6 por a venda");
    *((c + 0x4f) as *mut u8) = (a & 0xf) | 0x70; // à venda
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    if fora(cpp6_d(0xde1f54), jogador, 0) != 0 {
        return;
    }
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    let desgostar = if gosta(jogador, cpp6_d(jogador + 0x39)) != 0 {
        true
    } else {
        let lealdade = cpp6_b(c + 0x44) as i8 as i32;
        let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
        let s1 = sorteio(100);
        let s2 = sorteio(100);
        lealdade > s1.wrapping_sub(s2)
    };
    if desgostar {
        let antipatia: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
        antipatia(jogador, cpp6_d(cpp6_d(jogador + 0x39) + 0xcf), 1);
    }
    if noticia != 0 {
        let nas_listas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842c00usize);
        let dar_noticia: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4d71b0usize);
        nas_listas(this, jogador, 1);
        dar_noticia(cpp6_d(0xae1050), jogador, cpp6_d(jogador + 0x39));
    }
}

/// O registro de prêmio por jogos (0xd bytes: +0 id da pessoa, +4 valor, +8 jogos, +9 id do clube antigo) da pessoa,
/// pela busca binária (0x946464 com 0x8b4e00) no vetor das seleções (tipo 5: +0x898, conta +0x8a0) ou do clube
/// (+0x88c, conta +0x894) do TRANSFER_MANAGER.
unsafe fn cpp6_registro_do_premio(this: usize, jogador: usize, tipo: u32) -> usize {
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = [cpp6_d(jogador) as u32];
    let (vetor, conta) = if tipo as u8 == 5 { (cpp6_d(this + 0x898), cpp6_d(this + 0x8a0)) } else { (cpp6_d(this + 0x88c), cpp6_d(this + 0x894)) };
    bsearch(chave.as_ptr(), vetor, conta, 0xd, 0x8b4e00)
}

/// { 0x8cb340 } (thiscall, ret 0x10, transfer_manager.cpp): `TRANSFER_MANAGER::pay_appearance_money(jogador, jogos,
/// tipo, arg)`: com contrato (guardado em [0xae1080]) e o registro do prêmio da pessoa (o `tipo` vai inteiro, como a
/// original o empurra) cujos jogos (+8, com sinal) são os `jogos` (> 0): dá a notícia (0x8db8a0), soma o valor (+4)
/// aos totais de transferência (0x448ad0 em [0xad9dac], com o clube atual e o antigo), o clube antigo (+9) recebe
/// (0x599800) e o atual paga (0x599890), em 64 bits com sinal. Devolve 1 se pagou.
unsafe extern "thiscall" fn pagar_premio_por_jogos(this: usize, jogador: usize, jogos: i32, tipo: u32, arg: u32) -> u32 {
    let c = cpp6_contrato_guardado(jogador);
    if c == 0 || cpp6_clube(cpp6_d(c + 4)) == 0 {
        return 0;
    }
    let r = cpp6_registro_do_premio(this, jogador, tipo);
    if r == 0 {
        return 0;
    }
    // a original: jogos diferentes só seguem com −1, que cai no teste seguinte (≤ 0)
    if cpp6_b(r + 8) as i8 as i32 != jogos || jogos <= 0 {
        return 0;
    }
    let noticia: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) = core::mem::transmute(0x8db8a0usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let somar: unsafe extern "thiscall" fn(usize, i32, usize, usize) -> u32 = core::mem::transmute(0x448ad0usize);
    let receber: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x599800usize);
    let pagar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x599890usize);
    noticia(this, r, jogos, tipo, arg);
    let f_antigo = financas(cpp6_d(0xae2a38), cpp6_clube(cpp6_d(r + 9)));
    let c = cpp6_contrato_guardado(jogador);
    let f_atual = financas(cpp6_d(0xae2a38), if c != 0 { cpp6_clube(cpp6_d(c + 4)) } else { 0 });
    let c = cpp6_contrato_guardado(jogador);
    let atual = if c != 0 { cpp6_clube(cpp6_d(c + 4)) } else { 0 };
    let valor = core::ptr::read_unaligned((r + 4) as *const i32);
    somar(cpp6_d(0xad9dac), valor, atual, cpp6_clube(cpp6_d(r + 9)));
    crate::comparacao::ramo("cpp_6 premio por jogos");
    receber(f_antigo, valor as u32, (valor >> 31) as u32); // o clube antigo recebe
    pagar(f_atual, valor as u32, (valor >> 31) as u32);
    1
}

/// Comparador: a 0x842e10 mexe nas vagas de muitos clubes (e as listas não cabem numa cópia barata): roda só a
/// original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_por_jogador_nas_listas_dos_clubes(this: usize, jogador: usize, forcar: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1021)(this, jogador, forcar)
}

/// Comparador: sorteia e dá notícia: roda só a original e confere o plano feito antes: sem listagem do clube e com
/// contrato, o contrato fica à venda (+0x4e bit 1, +0x4f nibble alto 7).
unsafe extern "thiscall" fn comparar_por_jogador_a_venda(this: usize, jogador: usize, noticia: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1022);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    let marca = listada(jogador, 0) == 0 && cpp6_contrato(jogador) != 0;
    o(this, jogador, noticia);
    if marca {
        let c = cpp6_contrato(jogador);
        let ok = c != 0 && cpp6_b(c + 0x4e) & 1 != 0 && cpp6_b(c + 0x4f) & 0xf0 == 0x70;
        PLACAR.registrar(ok, || format!("por_jogador_a_venda(jogador {:x}) contrato {:x} sem a marca de à venda", cpp6_d(jogador), c));
    }
}

/// Comparador: dá notícia e mexe nas finanças: roda só a original e confere a devolução com o plano (contrato, clube,
/// registro do prêmio e jogos, sem efeitos).
unsafe extern "thiscall" fn comparar_pagar_premio_por_jogos(this: usize, jogador: usize, jogos: i32, tipo: u32, arg: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> u32>(1023);
    let c = cpp6_contrato(jogador);
    let esperado = if c == 0 || cpp6_clube(cpp6_d(c + 4)) == 0 {
        0
    } else {
        let r = cpp6_registro_do_premio(this, jogador, tipo);
        (r != 0 && cpp6_b(r + 8) as i8 as i32 == jogos && jogos > 0) as u32
    };
    let r = o(this, jogador, jogos, tipo, arg);
    PLACAR.registrar(r == esperado, || format!("pagar_premio_por_jogos(jogador {:x}, jogos {}, tipo {}) devolveu {}, esperava {}", cpp6_d(jogador), jogos, tipo as u8, r, esperado));
    r
}

// ---------- 252a leva: as ofertas de transferência: o clube recusa a oferta, soltar os jogadores da oferta e montar a oferta em dinheiro (transfer_manager.cpp) ----------

unsafe fn cpp7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp7_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpp7_pessoa(id: usize) -> usize {
    cpp7_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpp7_clube(id: usize) -> usize {
    cpp7_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// { 0x8c36b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::club_rejects_offer(oferta, clube,
/// motivo)`: fora dos estados 0x11/0x12 (+0x2c), conta a tentativa (+0x2e). Recusa do clube vendedor (id = +8): estado
/// 0x12; motivo 8 (16 bits) reagenda (0x8c6220 com 7, 0) e sai; senão, sem clube humano no contrato do jogador
/// (+0xc; contrato em [0xae1080], +0x35 & 0x3f ≠ 5, 0x600a50) e com mais de uma tentativa, um sorteio(2 × tentativas)
/// diferente de 0 rejeita a oferta (0x8bf680 com o endereço da oferta, 0, 6) e sai. Recusa do outro lado: estado
/// 0x11, desassossega o jogador (0x8c1290); com o clube vendedor do computador e o sorteio, retira a oferta (0x8bfcb0);
/// com motivo 8 o resto reagenda e sai (sem o teste de tentativas). No fim: reagenda e dá a notícia do adiamento
/// (0x8dda90 com o motivo inteiro, como a original o empurra).
unsafe extern "thiscall" fn clube_recusa_a_oferta(this: usize, oferta: usize, clube: usize, motivo: u32) {
    let estado = cpp7_b(oferta + 0x2c);
    if estado == 0x11 || estado == 0x12 {
        return;
    }
    crate::comparacao::ramo("cpp_7 recusa");
    *((oferta + 0x2e) as *mut u8) = cpp7_b(oferta + 0x2e).wrapping_add(1); // mais uma tentativa
    let agendar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x8c6220usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    // a original passa o endereço do próprio argumento
    let mut o = oferta;
    if cpp7_d(clube) == cpp7_d(oferta + 8) {
        *((oferta + 0x2c) as *mut u8) = 0x12;
        if motivo as u16 == 8 {
            agendar(this, oferta, 7, 0);
            return;
        }
        let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let c = contrato_de(cpp7_d(0xae1050), cpp7_pessoa(cpp7_d(oferta + 0xc)));
        *(0xae1080 as *mut usize) = c;
        let mut no_fim = false;
        if c != 0 && cpp7_b(c + 0x35) & 0x3f != 5 {
            let cl = cpp7_clube(cpp7_d(c + 4));
            no_fim = cl != 0 && humano(cl, 0, 1, 0, 0) != 0;
        }
        if !no_fim {
            let n = cpp7_b(oferta + 0x2e);
            if n > 1 && sorteio(n as i32 * 2) != 0 {
                let rejeitar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(0x8bf680usize);
                rejeitar(this, &mut o, 0, 6);
                return;
            }
        }
    } else {
        *((oferta + 0x2c) as *mut u8) = 0x11;
        let desassossegar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c1290usize);
        desassossegar(this, oferta);
        let retirar: unsafe extern "thiscall" fn(usize, *mut usize, u32, u32) -> u32 = core::mem::transmute(0x8bfcb0usize);
        let cl = cpp7_clube(cpp7_d(oferta + 8));
        if motivo as u16 == 8 {
            if humano(cl, 0, 1, 0, 0) == 0 && sorteio(cpp7_b(oferta + 0x2e) as i32 * 2) != 0 {
                retirar(this, &mut o, 0, 6);
                return;
            }
            agendar(this, oferta, 7, 0);
            return;
        }
        if humano(cl, 0, 1, 0, 0) == 0 {
            let n = cpp7_b(oferta + 0x2e);
            if n > 1 && sorteio(n as i32 * 2) != 0 {
                retirar(this, &mut o, 0, 6);
                return;
            }
        }
    }
    let noticia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8dda90usize);
    agendar(this, oferta, 7, 0);
    noticia(this, oferta, motivo);
}

/// { 0x8c6ae0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da lista local): `TRANSFER_MANAGER::
/// detach_offer_players(oferta)`: constrói uma LLIST local que não usa (0x68d020, destruída no fim por 0x68d040), tira
/// das outras ofertas o jogador da oferta (+0xc; 0x8c41c0 com a oferta, 0, 0) e, pela lista dos jogadores de troca
/// (+4, relida a cada chamada, cursor recomeçado), tira cada um (0x8c41c0) e desfaz as ofertas por ele (0x8c47d0). O
/// id do jogador de troca é relido do item depois da 0x8c41c0, como na original.
unsafe extern "thiscall" fn soltar_jogadores_da_oferta(this: usize, oferta: usize) {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x8c41c0usize);
    let desfazer: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8c47d0usize);
    let mut local = [0u32; 4]; // a LLIST (0xe bytes) da original
    lista_nova(local.as_mut_ptr() as usize);
    crate::comparacao::ramo("cpp_7 soltar jogadores");
    tirar(this, cpp7_pessoa(cpp7_d(oferta + 0xc)), oferta, 0, 0); // o jogador da oferta
    let trocas = cpp7_d(oferta + 4);
    if trocas != 0 {
        core::ptr::write_unaligned((trocas + 0xa) as *mut usize, cpp7_d(trocas + 2));
        let mut item = proximo(cpp7_d(oferta + 4));
        while item != 0 {
            tirar(this, cpp7_pessoa(cpp7_d(item)), oferta, 0, 0);
            desfazer(this, cpp7_pessoa(cpp7_d(item)));
            item = proximo(cpp7_d(oferta + 4));
        }
    }
    destruir(local.as_mut_ptr() as usize);
}

/// { 0x8c8740 } (thiscall, ret 0x14, transfer_manager.cpp, quadro SEH só para a oferta nova):
/// `TRANSFER_MANAGER::build_cash_offer_option(clube, jogador, saída, atração, valor)`: aloca a oferta (0x944cf1(0x41) e
/// 0x8e5680), guarda em `saída`, inicia pelo jogador, o clube comprador e o valor (0x8e78f0, que pode sortear) e põe a
/// atração em 50. Clube sem finanças (0x5a0590 = 0): a original (caixa "Error").
unsafe extern "thiscall" fn montar_oferta_em_dinheiro(this: usize, clube: usize, jogador: usize, saida: *mut usize, atracao: *mut u8, valor: i32) -> u32 {
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    // ponytail: a 0x5a0590 só procura; consultada antes de alocar para o caso de erro ir inteiro pela original
    if financas(cpp7_d(0xae2a38), clube) == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, i32) -> u32>(1026)(this, clube, jogador, saida, atracao, valor);
    }
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let iniciar: unsafe extern "thiscall" fn(usize, usize, usize, i32) = core::mem::transmute(0x8e78f0usize);
    let p = novo(0x41);
    let oferta = if p != 0 { construir(p) } else { 0 };
    *saida = oferta;
    crate::comparacao::ramo("cpp_7 oferta em dinheiro");
    iniciar(oferta, jogador, clube, valor); // o valor da oferta
    *atracao = 0x32;
    1
}

/// Comparador: sorteia, reagenda, rejeita ou retira (pode liberar a oferta) e dá notícia: roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_clube_recusa_a_oferta(this: usize, oferta: usize, clube: usize, motivo: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1024)(this, oferta, clube, motivo)
}

/// Comparador: tira jogadores de ofertas e desfaz ofertas (libera itens): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_soltar_jogadores_da_oferta(this: usize, oferta: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1025)(this, oferta)
}

/// Comparador: aloca a oferta e a 0x8e78f0 pode sortear: roda só a original e confere o resultado: devolve 1, a oferta
/// guardada em `saída` tem o valor (+0x1b) e a atração ficou em 50.
unsafe extern "thiscall" fn comparar_montar_oferta_em_dinheiro(this: usize, clube: usize, jogador: usize, saida: *mut usize, atracao: *mut u8, valor: i32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, i32) -> u32>(1026)(this, clube, jogador, saida, atracao, valor);
    let oferta = *saida;
    let ok = r == 1 && oferta != 0 && core::ptr::read_unaligned((oferta + 0x1b) as *const i32) == valor && *atracao == 0x32;
    PLACAR.registrar(ok, || format!("montar_oferta_em_dinheiro(jogador {:x}, valor {}) devolveu {}, oferta {:x}", cpp7_d(jogador), valor, r, oferta));
    r
}

// ---------- 252a leva: as doenças e a pré-temporada: acrescentar o surto de doença no clube, espalhar os surtos pelos elencos e o jogador fora da temporada (injury.cpp) ----------

unsafe fn cpp8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpp8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpp8_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpp8_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O clube pelo índice (registro de 0x245 bytes em [0xae23b4]).
unsafe fn cpp8_clube(i: i32) -> usize {
    cpp8_d(0xae23b4).wrapping_add((i as usize).wrapping_mul(0x245))
}

unsafe fn cpp8_selecao(clube: usize) -> bool {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    e_selecao(clube) != 0
}

/// Um surto (17 bytes, vetor [this+0x1c], capacidade +0x20, conta +0x24): +0 a data (8 bytes), +8 o tipo da doença,
/// +9 o índice do clube, +0xd o id da partida (−1 sem).
const CPP8_SURTO: usize = 0x11;

/// Se o surto entra (os sorteios da 0x62e820, na ordem): tipo 4 nunca; tipo 3 só com 1 em 15 (sorteio de 15 = 0); os
/// outros com 3 em 4 (sorteio de 4 ≠ 0); e nunca numa seleção (0x53b900).
unsafe fn cpp8_surto_entra(tipo: u8, clube_i: i32) -> bool {
    if tipo == 4 {
        return false;
    }
    if tipo == 3 {
        if cpp8_sorteio(0xf) != 0 {
            return false;
        }
    } else if cpp8_sorteio(4) == 0 { // 1 em 4 fica de fora
        return false;
    }
    !cpp8_selecao(cpp8_clube(clube_i))
}

/// { 0x62e820 } (thiscall, ret 0x10, injury.cpp): `Injury::add_illness_outbreak(data, tipo, clube, partida)`: o tipo
/// é o byte baixo; se o surto entra (`cpp8_surto_entra`), cresce o vetor de 30 em 30 quando cheio (0x945667, ou
/// 0x944e46 sem vetor; sem memória, a caixa "Error" da linha 0xd01 e nada), grava o surto no fim (data pela 0x417c10)
/// e reordena pela data (qsort 0x945333 com 0x778e90).
unsafe extern "thiscall" fn acrescentar_surto_de_doenca(this: usize, data: usize, tipo: u32, clube_i: i32, partida: usize) {
    let tipo = tipo as u8;
    if !cpp8_surto_entra(tipo, clube_i) {
        return;
    }
    crate::comparacao::ramo("cpp_8 surto novo");
    let cap = cpp8_i(this + 0x20);
    if cpp8_i(this + 0x24) >= cap {
        let cap = cap.wrapping_add(30);
        core::ptr::write_unaligned((this + 0x20) as *mut i32, cap);
        let tamanho = (cap as u32).wrapping_mul(CPP8_SURTO as u32);
        let v = cpp8_d(this + 0x1c);
        let novo = if v != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            realocar(v, tamanho)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            alocar(tamanho)
        };
        core::ptr::write_unaligned((this + 0x1c) as *mut usize, novo);
        if novo == 0 {
            aviso_em(0x9e7f28, 0xd01);
            return;
        }
    }
    let e = cpp8_d(this + 0x1c).wrapping_add((cpp8_i(this + 0x24) as usize).wrapping_mul(CPP8_SURTO));
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut rascunho = [0u8; 8];
    copiar(e, rascunho.as_mut_ptr(), data);
    *((e + 8) as *mut u8) = tipo;
    core::ptr::write_unaligned((e + 9) as *mut i32, clube_i);
    let id = if partida != 0 { cpp8_i(partida) } else { -1 };
    core::ptr::write_unaligned((e + 0xd) as *mut i32, id);
    let n = cpp8_i(this + 0x24).wrapping_add(1);
    core::ptr::write_unaligned((this + 0x24) as *mut i32, n);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(cpp8_d(this + 0x1c), n as u32, CPP8_SURTO as u32, 0x778e90);
}

/// A data de cinco dias atrás (0x54c8e0 sobre a data do jogo).
unsafe fn cpp8_cinco_dias_atras() -> [u8; 8] {
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) = core::mem::transmute(0x54c8e0usize);
    let mut d = [0u8; 8];
    datar(0xae2c90, &mut d, 5); // cinco dias atrás
    d
}

/// A data `limite` (dia com sinal em +0, ano sem sinal em +2) é depois da do surto `e`?
unsafe fn cpp8_depois(limite: &[u8; 8], e: usize) -> bool {
    let dia = i16::from_le_bytes([limite[0], limite[1]]);
    let ano = u16::from_le_bytes([limite[2], limite[3]]);
    let ano_e = cpp8_w(e + 2) as u16;
    ano > ano_e || (ano == ano_e && dia > cpp8_w(e))
}

/// { 0x62e9e0 } (thiscall, ret 0, injury.cpp, quadro SEH sem destrutor): `Injury::spread_illness_outbreaks()`: percorre
/// os surtos (ordenados pela data) até o primeiro depois de hoje. O de mais de cinco dias (0x54c8e0) conta para sair;
/// os outros espalham: tipo 3 só no próprio dia, com 1 a 4 doentes (sorteio de 4, +1); os outros 0 a 2 (sorteio de 3).
/// Fora das seleções (0x53b900), até 20 tentativas: uma das 50 vagas do elenco (sorteio de 50, +0xd7), jogador (id
/// abaixo de [0xae2364]) sem essa doença (+0x1d do registro de 31 bytes em [this]) fica doente (0x62d250: o tipo no
/// byte baixo da dword do deslocamento do surto, como a original; a partida do surto). No fim, os que saem vão
/// embora do começo do vetor (os outros descem) e a conta cai.
unsafe extern "thiscall" fn espalhar_surtos_de_doenca(this: usize) {
    let cinco = cpp8_cinco_dias_atras();
    let lesionar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, i32, u32, u32) -> u32 = core::mem::transmute(0x62d250usize);
    let mut removidos: i32 = 0;
    let mut k: i32 = 0;
    let mut off: usize = 0;
    while k < cpp8_i(this + 0x24) {
        let e = cpp8_d(this + 0x1c).wrapping_add(off);
        let hoje4 = cpp8_d(0xae2c90) as u32;
        let (ano_e, ano_h) = (cpp8_w(e + 2) as u16, (hoje4 >> 16) as u16);
        if ano_h < ano_e || (ano_h == ano_e && (hoje4 as u16 as i16) < cpp8_w(e)) {
            break; // o primeiro surto depois de hoje
        }
        if cpp8_depois(&cinco, e) {
            crate::comparacao::ramo("cpp_8 surto antigo");
            removidos += 1;
        } else {
            let tipo = cpp8_b(e + 8);
            let quantos = if tipo == 3 {
                if cpp8_d(0xae2c90) as u32 == cpp8_d(e) as u32 { Some((cpp8_sorteio(4) as u8).wrapping_add(1)) } else { None }
            } else {
                Some(cpp8_sorteio(3) as u8) // 0 a 2 doentes
            };
            if let Some(mut quantos) = quantos {
                crate::comparacao::ramo("cpp_8 espalhar");
                let clube = cpp8_clube(cpp8_i(cpp8_d(this + 0x1c).wrapping_add(off) + 9));
                if !cpp8_selecao(clube) {
                    let mut t = 0;
                    while t < 0x14 && (quantos as i8) > 0 {
                        let vaga = cpp8_sorteio(0x32) as i8 as isize;
                        let p = cpp8_d((clube + 0xd7).wrapping_add((vaga * 4) as usize));
                        if p != 0 && cpp8_i(p) < cpp8_i(0xae2364) {
                            let e = cpp8_d(this + 0x1c).wrapping_add(off);
                            let doenca = cpp8_b(e + 8);
                            let registro = cpp8_d(this).wrapping_add((cpp8_i(p) as usize).wrapping_mul(31));
                            if cpp8_b(registro + 0x1d) != doenca {
                                lesionar(this, (off as u32 & 0xffff_ff00) | doenca as u32, 0, p, 0, 0, cpp8_i(e + 0xd), 0, 0);
                                quantos = quantos.wrapping_sub(1);
                            }
                        }
                        t += 1;
                    }
                }
            }
        }
        k += 1;
        off += CPP8_SURTO;
    }
    let n = cpp8_i(this + 0x24);
    if removidos != 0 && removidos != n {
        let base = cpp8_d(this + 0x1c);
        core::ptr::copy((base + removidos as usize * CPP8_SURTO) as *const u8, base as *mut u8, (n - removidos) as usize * CPP8_SURTO);
    }
    core::ptr::write_unaligned((this + 0x24) as *mut i32, n - removidos);
}

/// { 0x62ce30 } (stdcall, ret 4, injury.cpp, quadro SEH sem destrutor): `injury_player_in_close_season(pessoa)`: sem
/// pessoa, 1. Com seleção (+0x24), a data dela (0x76e040 sobre [0xb74318], a nação +0x53 da seleção, a classe da
/// 0x76d780 pelo bit 3 do +0x5e) depois de hoje conta como convocado. Sem clube (+0x39), convocado ou clube com +0x1db
/// ≠ −1: 0. Senão, fora da temporada da nação do clube (0x669bd0 com as duas datas) e faltando menos de 40 dias
/// (0x54cdb0 de hoje até a segunda data, 16 bits sem sinal): 1; senão 0.
unsafe extern "stdcall" fn jogador_fora_da_temporada(pessoa: usize) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut u32) -> usize = core::mem::transmute(0x54a110usize);
    let mut inicio = [0u32; 2];
    let mut fim = [0u32; 2];
    construir(inicio.as_mut_ptr());
    construir(fim.as_mut_ptr());
    if pessoa == 0 {
        return 1;
    }
    let mut convocado = false;
    let selecao = cpp8_d(pessoa + 0x24);
    if selecao != 0 {
        let classe: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(0x76d780usize);
        let campo: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x76e040usize);
        let k = classe(selecao, (cpp8_b(pessoa + 0x5e) & 8) as u32);
        // ponytail: a original empurra a dword do argumento com só o byte baixo trocado pelo al; a 0x76e040 lê só o byte
        let q = campo(cpp8_d(0xb74318), cpp8_d(selecao + 0x53), k as u32);
        let lo = cpp8_d(q) as u32;
        let (ano, ano_h) = ((lo >> 16) as u16, cpp8_w(0xae2c92) as u16);
        convocado = ano > ano_h || (ano == ano_h && (lo as u16 as i16) > cpp8_w(0xae2c90));
    }
    let clube = cpp8_d(pessoa + 0x39);
    if clube == 0 || convocado || cpp8_i(clube + 0x1db) != -1 {
        return 0;
    }
    let temporada: unsafe extern "C" fn(usize, *mut u32, *mut u32) -> u32 = core::mem::transmute(0x669bd0usize);
    if temporada(clube, inicio.as_mut_ptr(), fim.as_mut_ptr()) != 0 {
        return 0;
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    if (dias(0xae2c90, fim[0], fim[1]) as u16) < 0x28 {
        return 1;
    }
    0
}

/// Comparador: a original sorteia, aloca e reordena: o plano antes (os sorteios e a seleção, com o estado do sorteio
/// guardado e devolvido); a original roda. Confere o mesmo estado do sorteio e, se o surto entrou, a conta +1 e o surto
/// (data, tipo, clube, partida) no vetor; senão a conta igual.
unsafe extern "thiscall" fn comparar_acrescentar_surto_de_doenca(this: usize, data: usize, tipo: u32, clube_i: i32, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, i32, usize)>(1027);
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let entra = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| cpp8_surto_entra(tipo as u8, clube_i)));
    let depois_do_plano = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    estado.escrever();
    let n0 = cpp8_i(this + 0x24);
    let mut esperado = [0u8; CPP8_SURTO];
    esperado[..8].copy_from_slice(&*(data as *const [u8; 8]));
    esperado[8] = tipo as u8;
    esperado[9..13].copy_from_slice(&clube_i.to_le_bytes());
    let id = if partida != 0 { cpp8_i(partida) } else { -1 };
    esperado[13..17].copy_from_slice(&id.to_le_bytes());
    o(this, data, tipo, clube_i, partida);
    let n = cpp8_i(this + 0x24);
    let mesmo_sorteio = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO) == depois_do_plano;
    let v = cpp8_d(this + 0x1c);
    let ok = if entra && v != 0 {
        n == n0 + 1 && (0..n.max(0) as usize).any(|k| *((v + k * CPP8_SURTO) as *const [u8; CPP8_SURTO]) == esperado)
    } else {
        n == n0
    };
    PLACAR.registrar(ok && mesmo_sorteio, || {
        format!("acrescentar_surto_de_doenca(tipo {}, clube {}) plano entra={}, conta {} -> {}, mesmo sorteio={}", tipo as u8, clube_i, entra, n0, n, mesmo_sorteio)
    });
}

/// Comparador: a original sorteia e adoece (0x62d250, que pode acrescentar surtos): roda só ela e confere o plano
/// feito antes: quantos surtos tinham mais de cinco dias (os que saem) e, depois, nenhum surto com mais de cinco dias e
/// a conta pelo menos a de antes menos os que saem.
unsafe extern "thiscall" fn comparar_espalhar_surtos_de_doenca(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1028);
    let cinco = cpp8_cinco_dias_atras();
    let n0 = cpp8_i(this + 0x24);
    let hoje4 = cpp8_d(0xae2c90) as u32;
    let mut saem = 0;
    for k in 0..n0.max(0) as usize {
        let e = cpp8_d(this + 0x1c) + k * CPP8_SURTO;
        let (ano_e, ano_h) = (cpp8_w(e + 2) as u16, (hoje4 >> 16) as u16);
        if ano_h < ano_e || (ano_h == ano_e && (hoje4 as u16 as i16) < cpp8_w(e)) {
            break;
        }
        if cpp8_depois(&cinco, e) {
            saem += 1;
        }
    }
    o(this);
    let n = cpp8_i(this + 0x24);
    let velho = (0..n.max(0) as usize).find(|&k| cpp8_depois(&cinco, cpp8_d(this + 0x1c) + k * CPP8_SURTO));
    PLACAR.registrar(velho.is_none() && n >= n0 - saem, || {
        format!("espalhar_surtos_de_doenca: conta {} -> {} (saem {}), surto velho {:?}", n0, n, saem, velho)
    });
}

/// Comparador: pelo valor (datas, a seleção e a temporada só calculam).
unsafe extern "stdcall" fn comparar_jogador_fora_da_temporada(pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(1029)(pessoa);
    let n = jogador_fora_da_temporada(pessoa);
    PLACAR.registrar(r == n, || format!("jogador_fora_da_temporada({:x}) original={} rust={}", pessoa, r, n));
    r
}

// ---------- 252a leva: a criação do jogo: a forma inicial dos elencos, os registros de lesão com as lesões iniciais e as listas de clubes por nação dos jovens (injury.cpp, player_regen.cpp) ----------

unsafe fn cpp9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpp9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpp9_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpp9_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

unsafe fn cpp9_selecao(clube: usize) -> bool {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    e_selecao(clube) != 0
}

/// Os clubes sem as seleções do fim do vetor: [0xae235c] − 2 × [0xae2350].
unsafe fn cpp9_clubes_sem_selecoes() -> i32 {
    cpp9_i(0xae235c).wrapping_sub(cpp9_i(0xae2350).wrapping_mul(2))
}

/// Os dias de pré-temporada que contam na forma inicial: dentro da temporada da nação do clube (0x669bd0), 0; fora,
/// os dias do começo dela até hoje (0x54cdb0 sobre a primeira data): acima de 80, 40; de 41 a 80, 80 − dias (no byte,
/// com sinal); senão os dias.
unsafe fn cpp9_dias_de_preparo(clube: usize, inicio: &mut [u32; 2], fim: &mut [u32; 2]) -> i32 {
    let temporada: unsafe extern "C" fn(usize, *mut u32, *mut u32) -> u32 = core::mem::transmute(0x669bd0usize);
    if temporada(clube, inicio.as_mut_ptr(), fim.as_mut_ptr()) != 0 {
        return 0;
    }
    let dias: unsafe extern "thiscall" fn(*mut u32, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let d = dias(inicio.as_mut_ptr(), cpp9_d(0xae2c90) as u32, cpp9_d(0xae2c94) as u32);
    if d > 0x50 {
        0x28
    } else if d > 0x28 {
        (0x50u8.wrapping_sub(d as u8)) as i8 as i32
    } else {
        d
    }
}

/// { 0x62a920 } (thiscall, ret 0, injury.cpp, quadro SEH sem destrutor): `Injury::init_squad_fitness()`: para cada
/// clube (sem as seleções), os dias de preparo (`cpp9_dias_de_preparo`); cada uma das 50 vagas (+0xd7) com pessoa de
/// id abaixo de [0xae2364]: jogador (+0x61): v = dias − (+0x34 do jogador)/2 − (+0x5b da pessoa)/4 (com sinal,
/// truncado), entre 1 e 40, e a forma (+8 do registro de 31 bytes em [this]) = 10000 − 75 × v; quem não é jogador sai
/// da vaga (0), e se o clube dele (+0x39) é este: cargo (+0x3d) 0xb fica sem clube e 8, 0xf vira 8, 0xd vira 6.
unsafe extern "thiscall" fn forma_inicial_dos_elencos(this: usize) {
    let construir: unsafe extern "thiscall" fn(*mut u32) -> usize = core::mem::transmute(0x54a110usize);
    let mut inicio = [0u32; 2];
    let mut fim = [0u32; 2];
    construir(inicio.as_mut_ptr());
    construir(fim.as_mut_ptr());
    let base = cpp9_d(0xae23b4);
    let mut i = 0i32;
    while i < cpp9_clubes_sem_selecoes() {
        let clube = base.wrapping_add((i as usize).wrapping_mul(0x245));
        let dias = cpp9_dias_de_preparo(clube, &mut inicio, &mut fim);
        for s in 0..50usize {
            let vaga = clube + 0xd7 + 4 * s;
            let p = cpp9_d(vaga);
            if p == 0 || cpp9_i(p) >= cpp9_i(0xae2364) {
                continue;
            }
            let jogador = cpp9_d(p + 0x61);
            if jogador != 0 {
                crate::comparacao::ramo("cpp_9 forma inicial");
                let v = dias - (cpp9_b(jogador + 0x34) as i8 as i32) / 2 - (cpp9_b(p + 0x5b) as i8 as i32) / 4;
                let v = v.clamp(1, 0x28);
                let r = cpp9_d(this).wrapping_add((cpp9_i(p) as usize).wrapping_mul(31));
                core::ptr::write_unaligned((r + 8) as *mut u16, (10000 - v * 75) as u16); // a forma inicial
            } else {
                if cpp9_d(p + 0x39) == clube {
                    match cpp9_b(p + 0x3d) {
                        0xb => {
                            core::ptr::write_unaligned((p + 0x39) as *mut usize, 0);
                            *((p + 0x3d) as *mut u8) = 8;
                        }
                        0xf => *((p + 0x3d) as *mut u8) = 8,
                        0xd => *((p + 0x3d) as *mut u8) = 6,
                        _ => {}
                    }
                }
                core::ptr::write_unaligned(vaga as *mut usize, 0);
            }
        }
        i += 1;
    }
}

/// { 0x62cf70 } (thiscall, ret 0, injury.cpp): `Injury::init_records()`: [this] = 31 bytes por pessoa de [0xae2364]
/// (0x944e46; sem memória, a caixa "Error" da linha 0x7d0 e nada). Cada registro: a data de hoje (0x417c10), forma e
/// condição 10000 (+8, +0xa), desgaste 0 (+0xc), sem lesão (+0xe −1), sem recorrente (+0x12 0xff), histórico −1
/// (+0x14, +0x18), +0x1d 0xff, +0x1e 50. Jogador (+0x61): o tipo sorteado de 82 com peso (+8 da tabela de 13 bytes em
/// 0x9e7ad8) e 1 em 10 fica recorrente; 1 em 100 começa lesionado: categoria sorteada de 12, tipo pela 0x62d140
/// (gravidade −1) e a lesão nova 0x62d250. No fim os dois totais em 0xde1f64 ("%d players with ...").
unsafe extern "thiscall" fn registros_iniciais_de_lesao(this: usize) {
    let n = cpp9_i(0xae2364);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let v = alocar((n as u32).wrapping_mul(31));
    core::ptr::write_unaligned(this as *mut usize, v);
    if v == 0 {
        aviso_em(0x9e7f28, 0x7d0);
        return;
    }
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let escolher: unsafe extern "thiscall" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x62d140usize);
    let lesionar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, i32, u32, u32) -> u32 = core::mem::transmute(0x62d250usize);
    let (mut recorrentes, mut iniciais) = (0i32, 0i32);
    let mut r = v;
    let mut pessoa = cpp9_d(0xae23bc);
    let mut i = 0i32;
    while i < cpp9_i(0xae2364) {
        let mut rascunho = [0u8; 8];
        copiar(r, rascunho.as_mut_ptr(), 0xae2c90);
        core::ptr::write_unaligned((r + 0xc) as *mut u16, 0);
        core::ptr::write_unaligned((r + 8) as *mut u16, 10000);
        core::ptr::write_unaligned((r + 0xa) as *mut u16, 10000);
        *((r + 0x12) as *mut u8) = 0xff;
        core::ptr::write_unaligned((r + 0xe) as *mut i32, -1);
        core::ptr::write_unaligned((r + 0x14) as *mut i32, -1);
        core::ptr::write_unaligned((r + 0x18) as *mut i32, -1);
        *((r + 0x1d) as *mut u8) = 0xff;
        *((r + 0x1e) as *mut u8) = 0x32;
        if cpp9_d(pessoa + 0x61) != 0 {
            let t = cpp9_sorteio(0x52) as u8;
            if cpp9_d((0x9e7ae0usize).wrapping_add((t as i8 as isize * 13) as usize)) != 0 && cpp9_sorteio(10) == 0 {
                recorrentes += 1;
                *((r + 0x12) as *mut u8) = t;
            }
            if cpp9_sorteio(100) == 0 { // 1 em 100 começa lesionado
                crate::comparacao::ramo("cpp_9 lesao inicial");
                // ponytail: a original empurra dwords da pilha com só o byte baixo gravado (o resto é lixo); a 0x62d140
                // e a 0x62d250 leem só o byte: aqui vão limpos
                let categoria = cpp9_sorteio(0xc) as u8;
                let tipo = escolher(this, categoria as u32, 0xffff_ffff, pessoa) as u8;
                lesionar(this, tipo as u32, 1, pessoa, 0, 1, -1, 0, 0);
                iniciais += 1;
            }
        }
        i += 1;
        pessoa = pessoa.wrapping_add(0x6e);
        r = r.wrapping_add(31);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    formatar(0xde1f64, 0x9e7f98, recorrentes);
    formatar(0xde1f64, 0x9e7f74, iniciais);
}

/// A nação em cuja lista dos jovens o clube entra (0x7acfc0): nenhuma para seleção (0x53b900), sem nação (+0x53),
/// nação sem +0x71 ou os dois clubes de 0x9d00a4/0x9d00a8. Nação com +0x11c: só com +0x80 até 2 e divisão (+0x57);
/// sem: com divisão.
unsafe fn cpp9_nacao_da_lista(clube: usize) -> usize {
    if cpp9_selecao(clube) {
        return 0;
    }
    let nacao = cpp9_d(clube + 0x53);
    if nacao == 0 || cpp9_d(nacao + 0x71) == 0 {
        return 0;
    }
    if cpp9_b(nacao + 0x11c) != 0 {
        crate::comparacao::ramo("cpp_9 nacao chave");
        if !(cpp9_w(clube + 0x80) <= 2 && cpp9_d(clube + 0x57) != 0) { // nação com +0x11c
            return 0;
        }
    } else if cpp9_d(clube + 0x57) == 0 {
        return 0;
    }
    let id = cpp9_d(clube);
    if id == cpp9_d(0x9d00a4) || id == cpp9_d(0x9d00a8) {
        return 0;
    }
    nacao
}

/// { 0x7acfc0 } (thiscall, ret 0, player_regen.cpp, quadro SEH do `new`): `PLAYER_REGEN::build_nation_club_lists()`:
/// [this] = 14 bytes por nação de [0xae2350] (0x944e46, sem teste), com uma lista nova (0x944cf1 de 0xe e 0x68d020;
/// sem memória, 0) por nação; cada clube de [0xae235c] entra (0x68d0d0, os 4 bytes do id) na lista da nação dele
/// (`cpp9_nacao_da_lista`).
unsafe extern "thiscall" fn listas_de_clubes_por_nacao(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    let tabela = alocar((cpp9_i(0xae2350) as u32).wrapping_mul(14));
    core::ptr::write_unaligned(this as *mut usize, tabela);
    let mut i = 0i32;
    while i < cpp9_i(0xae2350) {
        let obj = novo(0xe);
        let l = if obj != 0 { lista_nova(obj) } else { 0 };
        core::ptr::write_unaligned(cpp9_d(this).wrapping_add(i as usize * 4) as *mut usize, l);
        i += 1;
    }
    let base = cpp9_d(0xae23b4);
    let mut j = 0i32;
    while j < cpp9_i(0xae235c) {
        let clube = base.wrapping_add((j as usize).wrapping_mul(0x245));
        let nacao = cpp9_nacao_da_lista(clube);
        if nacao != 0 {
            crate::comparacao::ramo("cpp_9 clube na lista");
            let l = cpp9_d(cpp9_d(this).wrapping_add((cpp9_i(nacao) as usize).wrapping_mul(4)));
            acrescentar(l, clube, 4);
        }
        j += 1;
    }
}

/// Comparador: lado a lado pelos registros de lesão (31 bytes × [0xae2364]), pelos clubes (as vagas) e pelo clube e
/// cargo (+0x39..+0x3d) de quem não é jogador nas vagas.
unsafe extern "thiscall" fn comparar_forma_inicial_dos_elencos(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1030);
    let base = cpp9_d(0xae23b4);
    let mut regioes = vec![(cpp9_d(this), (cpp9_i(0xae2364).max(0) as usize) * 31), (base, (cpp9_i(0xae235c).max(0) as usize) * 0x245)];
    for i in 0..cpp9_clubes_sem_selecoes().max(0) as usize {
        for s in 0..50usize {
            let p = cpp9_d(base + i * 0x245 + 0xd7 + 4 * s);
            if p != 0 && cpp9_i(p) < cpp9_i(0xae2364) && cpp9_d(p + 0x61) == 0 {
                regioes.push((p + 0x39, 5));
            }
        }
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(this), || forma_inicial_dos_elencos(this));
    PLACAR.registrar(igual, || format!("forma_inicial_dos_elencos: memória difere ({} regiões)", regioes.len()));
}

/// Comparador: a original aloca, sorteia e lesiona (0x62d250): roda só ela e confere o que a 0x62d250 não muda: o
/// vetor alocado e o +0x1e = 50 de todos os registros.
unsafe extern "thiscall" fn comparar_registros_iniciais_de_lesao(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1031)(this);
    let v = cpp9_d(this);
    if v == 0 {
        return;
    }
    let errado = (0..cpp9_i(0xae2364).max(0) as usize).find(|&k| cpp9_b(v + 31 * k + 0x1e) != 0x32);
    PLACAR.registrar(errado.is_none(), || format!("registros_iniciais_de_lesao: registro {:?} sem o +0x1e = 50", errado));
}

/// Comparador: a original aloca as listas: o plano antes (os ids dos clubes de cada nação, na ordem); a original roda;
/// confere os itens de cada lista.
unsafe extern "thiscall" fn comparar_listas_de_clubes_por_nacao(this: usize) {
    let n = cpp9_i(0xae2350).max(0) as usize;
    let mut plano: Vec<Vec<usize>> = vec![Vec::new(); n];
    let base = cpp9_d(0xae23b4);
    for j in 0..cpp9_i(0xae235c).max(0) as usize {
        let clube = base + j * 0x245;
        let nacao = cpp9_nacao_da_lista(clube);
        if nacao != 0 && (cpp9_i(nacao) as usize) < n {
            plano[cpp9_i(nacao) as usize].push(cpp9_d(clube));
        }
    }
    orig::<unsafe extern "thiscall" fn(usize)>(1032)(this);
    let errada = (0..n).find(|&k| {
        let l = cpp9_d(cpp9_d(this) + 4 * k);
        let ids: Vec<usize> = if l != 0 { cpd1_itens(l).into_iter().map(|e| cpp9_d(e)).collect() } else { Vec::new() };
        l != 0 && ids != plano[k]
    });
    PLACAR.registrar(errada.is_none(), || format!("listas_de_clubes_por_nacao: a lista da nação {:?} difere do plano", errada));
}

// ---------- 252a leva: o fim da temporada e as transferências nos registros: o jogador do ano da torcida, arquivar as estatísticas da nação e os recordes de valor de transferência (club_records.cpp, player_stats.cpp) ----------

unsafe fn cpp10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpp10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpp10_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Um double do .rdata do exe.
unsafe fn cpp10_q(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

unsafe fn cpp10_selecao(clube: usize) -> bool {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    e_selecao(clube) != 0
}

/// O clube pelo índice (registro de 0x245 bytes em [0xae23b4]).
unsafe fn cpp10_clube(i: i32) -> usize {
    cpp10_d(0xae23b4).wrapping_add((i as usize).wrapping_mul(0x245))
}

/// A nota da torcida para o jogador `p` do clube pela estatística `st` (0x452da0): média (+0xe com sinal / jogos +0)
/// + 0,025 × +2 + 1/60 × +6; o clube gosta dele (0x547790) +0,2, não gosta (0x5477d0) −2 (guardada); ele gosta do
/// clube (0x547d60) +0,2, não gosta (0x547d30) −1,5 (só na comparação).
unsafe fn cpp10_nota_da_torcida(p: usize, clube: usize, st: usize) -> f64 {
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547790usize);
    let desafeto: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5477d0usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d30usize);
    let media = cpp10_w(st + 0xe) as i32 as f64 / cpp10_b(st) as i32 as f64;
    let mut nota = media + cpp10_b(st + 2) as i32 as f64 * cpp10_q(0x967878);
    nota = nota + cpp10_b(st + 6) as i32 as f64 * cpp10_q(0x968958);
    if favorito(p, clube) != 0 {
        crate::comparacao::ramo("cpp_10 favorito do clube");
        nota = nota + cpp10_q(0x968918); // o clube gosta dele
    } else if desafeto(p, clube) != 0 {
        nota = nota - cpp10_q(0x968950);
    }
    if gosta(p, clube) != 0 {
        nota + cpp10_q(0x968918)
    } else if nao_gosta(p, clube) != 0 {
        nota - cpp10_q(0x968940)
    } else {
        nota
    }
}

/// { 0x452da0 } (thiscall, ret 8, club_records.cpp): `Club_Records::pick_fans_player_of_year(clube, registro)`: sem
/// jogos (+0x10 com sinal do registro de 106 bytes do clube em [this+0x1c]) e reputação (+0x80) abaixo de 3000, 0.
/// Com o fator 0,6 caindo de 0,1 enquanto ninguém foi escolhido e passa de 0,2: o mínimo de jogos é +0x10 × fator
/// (_ftol); dos 50 do elenco (+0xd7) jogadores (+0x61, id abaixo de [0xae2364]) do clube (+0x39) com a estatística 6
/// (0x7c3920 sobre [0xb74350]) de jogos acima de 0 e do mínimo (16 bits com sinal), o de maior nota
/// (`cpp10_nota_da_torcida`, acima de −1000). Ninguém: 0; senão a referência dele (0x7e4920) em registro +0x371 e 1.
unsafe extern "thiscall" fn jogador_do_ano_da_torcida(this: usize, clube: usize, registro: usize) -> u32 {
    let jogos_do_clube = || cpp10_b(cpp10_d(this + 0x1c).wrapping_add((cpp10_i(clube) as usize).wrapping_mul(106)) + 0x10) as i8;
    if jogos_do_clube() == 0 && cpp10_w(clube + 0x80) < 0xbb8 {
        return 0;
    }
    let estatistica: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x7c3920usize);
    let mut fator = 0.6f64;
    let mut melhor = -1000.0f64;
    let mut escolhido = 0usize;
    while escolhido == 0 {
        let minimo = ftol(jogos_do_clube() as i32 as f64 * fator) as i16;
        for s in 0..50usize {
            let p = cpp10_d(clube + 0xd7 + 4 * s);
            if p == 0 || cpp10_i(p) >= cpp10_i(0xae2364) {
                continue;
            }
            let jogador = cpp10_d(p + 0x61);
            if jogador == 0 || cpp10_d(p + 0x39) != clube {
                continue;
            }
            let st = estatistica(cpp10_d(0xb74350), jogador, 6);
            if st == 0 {
                continue;
            }
            let jogos = cpp10_b(st);
            if (jogos as i16) < minimo || jogos == 0 {
                continue;
            }
            let nota = cpp10_nota_da_torcida(p, clube, st);
            if nota > melhor {
                melhor = nota;
                escolhido = p;
            }
        }
        fator -= cpp10_q(0x967880);
        if !(fator > cpp10_q(0x968918)) {
            break;
        }
    }
    if escolhido == 0 {
        return 0;
    }
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
    referencia(escolhido, registro + 0x371);
    1
}

/// { 0x7c1350 } (thiscall, ret 4, player_stats.cpp): `PlayerStats::end_season_for_key_nation(nacao)`: sem nação, a
/// caixa "Error" da linha 0x396 e 0. Percorre os clubes de [0xae235c] até a primeira seleção (0x53b900: para ali);
/// clube da nação (0x669360, registro de 72 bytes em 0xb63d60): cada jogador (+0x61, id abaixo de [0xae2364]) das 50
/// vagas (+0xd7) sem empréstimo (0x4cf6d0 sobre [0xae1050]) ou emprestado ao próprio clube (+0x39) tem as estatísticas
/// arquivadas (0x7c5fe0 com −1, 0, 0, −1, 0). Devolve 1.
unsafe extern "thiscall" fn arquivar_estatisticas_da_nacao(this: usize, nacao: usize) -> u32 {
    if nacao == 0 {
        aviso_em(0xa78310, 0x396);
        return 0;
    }
    let indice: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669360usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let arquivar: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, i32, u32) = core::mem::transmute(0x7c5fe0usize);
    let mut i = 0i32;
    while i < cpp10_i(0xae235c) {
        let clube = cpp10_clube(i);
        if cpp10_selecao(clube) {
            break; // a primeira seleção encerra
        }
        let k = indice(clube) as u8 as i8 as isize; // só o al
        if (0xb63d60usize).wrapping_add((k * 72) as usize) == nacao {
            for s in 0..50usize {
                let p = cpp10_d(clube + 0xd7 + 4 * s);
                if p == 0 || cpp10_d(p + 0x61) == 0 || cpp10_i(p) >= cpp10_i(0xae2364) {
                    continue;
                }
                if emprestimo(cpp10_d(0xae1050), p) == 0 || cpp10_d(p + 0x39) == clube {
                    crate::comparacao::ramo("cpp_10 arquivar");
                    arquivar(this, p, -1, 0, 0, -1, 0);
                }
            }
        }
        i += 1;
    }
    1
}

/// O recorde de transferência já está na fila de notícias (6 bytes: id do clube, tipo) de [this+0x24], conta +0x2c?
unsafe fn cpp10_na_fila(this: usize, id: usize, tipo: u8) -> bool {
    let fila = cpp10_d(this + 0x24);
    (0..cpp10_i(this + 0x2c).max(0) as usize).any(|k| cpp10_d(fila + 6 * k) == id && cpp10_b(fila + 6 * k + 4) == tipo)
}

/// A transferência de recorde do clube passa de 1000 × (reputação (+0x80) + 300)?
unsafe fn cpp10_grande(valor: i32, clube: usize) -> bool {
    valor >= (cpp10_w(clube + 0x80) as i32 + 300).wrapping_mul(1000)
}

/// Copia o bloco de recorde de `o` (+`d`: valor, a referência de 0x18 bytes, o clube, a data, a marca) para `a`.
unsafe fn cpp10_copiar_recorde(a: usize, o: usize, d: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut rascunho = [0u8; 8];
    for k in 0..5 {
        core::ptr::write_unaligned((a + d + 4 * k) as *mut usize, cpp10_d(o + d + 4 * k));
    }
    copiar(a + d + 0x14, rascunho.as_mut_ptr(), o + d + 0x14);
    core::ptr::write_unaligned((a + d + 0x1c) as *mut usize, cpp10_d(o + d + 0x1c));
    copiar(a + d + 0x20, rascunho.as_mut_ptr(), o + d + 0x20);
    *((a + d + 0x28) as *mut u8) = cpp10_b(o + d + 0x28);
}

/// { 0x448760 } (thiscall, ret 4, club_records.cpp): `Club_Records::update_transfer_fee_records(transferencia)`: a
/// pessoa (+0xc) e o clube comprador (+8); o contrato dela (0x4cf5a0 sobre [0xae1050], guardado em [0xae1080]) dá o
/// vendedor (+4). Os registros da temporada (0x4528d0) e de sempre (0x452710) do comprador (criados); sem vendedor,
/// 1. Valor (+0x1b) acima do recorde pago da temporada (+0x313): ele, a referência da pessoa (0x7e4920), o vendedor,
/// a data de hoje e o empréstimo (+4 ≠ 0); acima do de sempre, copiado para ele, e se passa de 1000 × (reputação +
/// 300): o recorde na fila de notícias (0x452cb0, 0x1c, se ainda não está) e o técnico (+0xbf) passa a gostar da pessoa
/// (0x547a10, forçado). O mesmo para o vendedor no recorde recebido (+0x33c; a fila com 0x1d, sem o gostar). 1.
unsafe extern "thiscall" fn recordes_de_valor_de_transferencia(this: usize, t: usize) -> u32 {
    let pessoa = cpp10_d(0xae23bc).wrapping_add((cpp10_i(t + 0xc) as usize).wrapping_mul(0x6e));
    let comprador = cpp10_clube(cpp10_i(t + 8));
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contrato = contrato_de(cpp10_d(0xae1050), pessoa);
    core::ptr::write_unaligned(0xae1080 as *mut usize, contrato);
    let vendedor = if contrato != 0 { cpp10_clube(cpp10_i(contrato + 4)) } else { 0 };
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let referencia: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let noticia: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x452cb0usize);
    let mut rascunho = [0u8; 8];
    let s = temporada(this, comprador, 1);
    let a = sempre(this, comprador, 1);
    if vendedor == 0 {
        return 1;
    }
    let valor = cpp10_i(t + 0x1b);
    if valor != 0 && valor > cpp10_i(s + 0x313) {
        crate::comparacao::ramo("cpp_10 recorde pago");
        core::ptr::write_unaligned((s + 0x313) as *mut i32, valor);
        referencia(pessoa, s + 0x317);
        core::ptr::write_unaligned((s + 0x32f) as *mut usize, cpp10_d(vendedor));
        copiar(s + 0x333, rascunho.as_mut_ptr(), 0xae2c90);
        *((s + 0x33b) as *mut u8) = (cpp10_d(t + 4) != 0) as u8;
        if cpp10_i(t + 0x1b) > cpp10_i(a + 0x313) {
            cpp10_copiar_recorde(a, s, 0x313);
            if cpp10_grande(cpp10_i(a + 0x313), comprador) {
                crate::comparacao::ramo("cpp_10 transferencia grande");
                if !cpp10_na_fila(this, cpp10_d(comprador), 0x1c) {
                    noticia(this, cpp10_d(comprador) as u32, 0x1c);
                }
                let gostar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547a10usize);
                gostar(cpp10_d(comprador + 0xbf), pessoa, 1); // forçado
            }
        }
    }
    let s = temporada(this, vendedor, 1);
    let a = sempre(this, vendedor, 1);
    let valor = cpp10_i(t + 0x1b);
    if valor == 0 || valor <= cpp10_i(s + 0x33c) {
        return 1;
    }
    core::ptr::write_unaligned((s + 0x33c) as *mut i32, valor);
    referencia(pessoa, s + 0x340);
    core::ptr::write_unaligned((s + 0x358) as *mut usize, cpp10_d(comprador));
    copiar(s + 0x35c, rascunho.as_mut_ptr(), 0xae2c90);
    *((s + 0x364) as *mut u8) = (cpp10_d(t + 4) != 0) as u8;
    if cpp10_i(t + 0x1b) > cpp10_i(a + 0x33c) {
        cpp10_copiar_recorde(a, s, 0x33c);
        if cpp10_grande(cpp10_i(a + 0x33c), vendedor) && !cpp10_na_fila(this, cpp10_d(vendedor), 0x1d) {
            noticia(this, cpp10_d(vendedor) as u32, 0x1d);
        }
    }
    1
}

/// Comparador: lado a lado pela referência do escolhido (registro +0x371, 0x18 bytes) e pela devolução.
unsafe extern "thiscall" fn comparar_jogador_do_ano_da_torcida(this: usize, clube: usize, registro: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1033);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(registro + 0x371, 0x18)], || o(this, clube, registro), || jogador_do_ano_da_torcida(this, clube, registro));
    PLACAR.registrar(igual && r == n, || format!("jogador_do_ano_da_torcida(clube {:x}) original={} {:x?} rust={} {:x?}", cpp10_d(clube), r, mo, n, mr));
    r
}

/// Comparador: a 0x7c5fe0 move as estatísticas para o histórico (aloca): roda só a original e confere a devolução
/// (1, ou 0 sem nação).
unsafe extern "thiscall" fn comparar_arquivar_estatisticas_da_nacao(this: usize, nacao: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1034)(this, nacao);
    PLACAR.registrar(r == (nacao != 0) as u32, || format!("arquivar_estatisticas_da_nacao({:x}) devolveu {}", nacao, r));
    r
}

/// Comparador: a original põe na fila de notícias (realoca) e o gostar sorteia: o plano antes (os quatro registros
/// criados na mesma ordem da original, e os recordes pago e recebido esperados); a original roda; pega os registros
/// de novo (o vetor pode ter sido realocado) e confere os valores e o clube de cada recorde e o [0xae1080].
unsafe extern "thiscall" fn comparar_recordes_de_valor_de_transferencia(this: usize, t: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1035);
    let pessoa = cpp10_d(0xae23bc).wrapping_add((cpp10_i(t + 0xc) as usize).wrapping_mul(0x6e));
    let comprador = cpp10_clube(cpp10_i(t + 8));
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contrato = contrato_de(cpp10_d(0xae1050), pessoa);
    let vendedor = if contrato != 0 { cpp10_clube(cpp10_i(contrato + 4)) } else { 0 };
    if vendedor == 0 {
        let r = o(this, t);
        PLACAR.registrar(r == 1 && cpp10_d(0xae1080) == 0, || format!("recordes_de_valor_de_transferencia sem vendedor: {} [0xae1080]={:x}", r, cpp10_d(0xae1080)));
        return r;
    }
    let temporada: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4528d0usize);
    let sempre: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x452710usize);
    let registros = || [temporada(this, comprador, 1), sempre(this, comprador, 1), temporada(this, vendedor, 1), sempre(this, vendedor, 1)];
    let valor = cpp10_i(t + 0x1b);
    let rg = registros();
    // (valor, clube) de cada recorde: pago da temporada e de sempre do comprador, recebido do vendedor
    let mut plano = [(cpp10_i(rg[0] + 0x313), cpp10_d(rg[0] + 0x32f)), (cpp10_i(rg[1] + 0x313), cpp10_d(rg[1] + 0x32f)),
                     (cpp10_i(rg[2] + 0x33c), cpp10_d(rg[2] + 0x358)), (cpp10_i(rg[3] + 0x33c), cpp10_d(rg[3] + 0x358))];
    let mut fila = cpp10_i(this + 0x2c);
    if valor != 0 {
        if valor > plano[0].0 {
            plano[0] = (valor, cpp10_d(vendedor));
            if valor > plano[1].0 {
                plano[1] = plano[0];
                if cpp10_grande(valor, comprador) && !cpp10_na_fila(this, cpp10_d(comprador), 0x1c) {
                    fila += 1;
                }
            }
        }
        if valor > plano[2].0 {
            plano[2] = (valor, cpp10_d(comprador));
            if valor > plano[3].0 {
                plano[3] = plano[2];
                if cpp10_grande(valor, vendedor) && !cpp10_na_fila(this, cpp10_d(vendedor), 0x1d) {
                    fila += 1;
                }
            }
        }
    }
    let r = o(this, t);
    let rg = registros();
    let agora = [(cpp10_i(rg[0] + 0x313), cpp10_d(rg[0] + 0x32f)), (cpp10_i(rg[1] + 0x313), cpp10_d(rg[1] + 0x32f)),
                 (cpp10_i(rg[2] + 0x33c), cpp10_d(rg[2] + 0x358)), (cpp10_i(rg[3] + 0x33c), cpp10_d(rg[3] + 0x358))];
    let fila_agora = cpp10_i(this + 0x2c);
    PLACAR.registrar(r == 1 && agora == plano && fila_agora == fila && cpp10_d(0xae1080) == contrato, || {
        format!("recordes_de_valor_de_transferencia(valor {}) plano {:x?} fila {} x {:x?} fila {}", valor, plano, fila, agora, fila_agora)
    });
    r
}

// ---------- 252a leva: os amistosos internacionais: o par de nações que não joga amistoso, a ordem das nações pela reputação e a dos amistosos pela data (friendly.cpp) ----------

unsafe fn cpp11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp11_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// { 0x5cc3e0 } (thiscall, ret 8, friendly.cpp): `Friendly::is_banned_nation_pair(a, b)`: percorre os pares proibidos
/// (vetor de 8 bytes em +0x3502, conta em byte com sinal em +0x3506) e devolve 1 quando (a, b), (a, −1) ou (−1, b) está
/// lá. Senão 0: o resto da original (Austrália, Nova Zelândia e Oceania, 0x9cf224/0x9cf3f4/0x9cfa18, pela tabela das
/// nações) só lê e cai sempre no `xor eax, eax`.
unsafe extern "thiscall" fn par_de_nacoes_sem_amistoso(this: usize, a: i32, b: i32) -> u32 {
    let n = *((this + 0x3506) as *const i8);
    let mut k: i8 = 0;
    while k < n {
        let p = cpp11_d(this + 0x3502).wrapping_add(k as isize as usize * 8);
        let (x, y) = (cpp11_i(p), cpp11_i(p + 4));
        if (x == a && (y == b || y == -1)) || (x == -1 && y == b) {
            crate::comparacao::ramo("cpp_11 par proibido");
            return 1; // o par proibido
        }
        k = k.wrapping_add(1);
    }
    // ponytail: as leituras da Oceania não mudam a devolução (sempre 0); a cópia não as repete.
    0
}

/// { 0x5cc5f0 } (cdecl, friendly.cpp): `compare_nations_by_reputation_desc(a, b)`: o qsort das listas de nações
/// (ponteiros) pela reputação (+0x8e, palavra com sinal), a maior primeiro. Quente (qsort): sem contador.
unsafe extern "C" fn ordem_das_nacoes_pela_reputacao(a: usize, b: usize) -> i32 {
    let ra = *((cpp11_d(a) + 0x8e) as *const i16) as i32;
    let rb = *((cpp11_d(b) + 0x8e) as *const i16) as i32;
    rb - ra // a maior reputação primeiro
}

/// { 0x5cc610 } (cdecl, friendly.cpp): `compare_fixture_records_by_date(a, b)`: o qsort dos registros de 20 bytes dos
/// amistosos pela data em +3: −1 se a de `a` é antes (0x456770), 1 se é depois (0x456740), senão 0. Quente (qsort):
/// sem contador.
unsafe extern "C" fn ordem_dos_amistosos_pela_data(a: usize, b: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let (d0, d1) = (cpp11_d(b + 3) as u32, cpp11_d(b + 7) as u32);
    let r = if antes(a + 3, d0, d1) != 0 { -1 } else { (depois(a + 3, d0, d1) != 0) as i32 };
    r // a data mais cedo primeiro
}

/// Comparador: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_par_de_nacoes_sem_amistoso(this: usize, a: i32, b: i32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i32, i32) -> u32>(1036)(this, a, b);
    let n = par_de_nacoes_sem_amistoso(this, a, b);
    PLACAR.registrar(r == n, || format!("par_de_nacoes_sem_amistoso({}, {}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_das_nacoes_pela_reputacao(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1037)(a, b);
    let n = ordem_das_nacoes_pela_reputacao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_nacoes_pela_reputacao({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_dos_amistosos_pela_data(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1038)(a, b);
    let n = ordem_dos_amistosos_pela_data(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_amistosos_pela_data({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

// ---------- 252a leva: as seleções e o calendário dos clubes: se o jogo do clube pode ser marcado com os convocados, a ordem das convocações pela data e a nação classificada para a fase final (national_teams.cpp) ----------

unsafe fn cpp12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Dia (palavra com sinal) e ano (palavra sem sinal) de uma data de 8 bytes.
unsafe fn cpp12_data(p: usize) -> (i16, u16) {
    (core::ptr::read_unaligned(p as *const i16), core::ptr::read_unaligned((p + 2) as *const u16))
}

/// Os convocados do clube (id < [0xae235c] − 2 × [0xae2350]: clube comum, não seleção) entre as datas `a` e `b`: a
/// lista do clube em [this + 0x20] + 6 × id (+0 os registros de 9 bytes em ordem de data, +5 a conta em byte com sinal);
/// pula os antes de `a`, para no primeiro depois de `b` e soma os bytes +8 (em byte, com a volta). Mais de 2: demais.
unsafe fn cpp12_muitos_convocados(this: usize, clube: usize, a: usize, b: usize) -> bool {
    if clube == 0 {
        return false;
    }
    let limite = cpp12_i(0xae235c).wrapping_sub(cpp12_i(0xae2350).wrapping_mul(2));
    let id = cpp12_i(clube);
    if id >= limite {
        return false;
    }
    let rec = cpp12_d(this + 0x20).wrapping_add((id as usize).wrapping_mul(6));
    let n = *((rec + 5) as *const i8);
    let lista = cpp12_d(rec);
    let (a_dia, a_ano) = cpp12_data(a);
    let (b_dia, b_ano) = cpp12_data(b);
    let mut soma: i8 = 0;
    let mut k: i8 = 0;
    while k < n {
        let e = lista.wrapping_add(k as isize as usize * 9);
        let (dia, ano) = cpp12_data(e);
        k = k.wrapping_add(1);
        if a_ano > ano || (a_ano == ano && a_dia > dia) {
            continue; // antes da janela
        }
        if b_ano < ano || (b_ano == ano && b_dia < dia) {
            break; // depois da janela: a lista está em ordem
        }
        crate::comparacao::ramo("cpp_12 convocado na janela");
        soma = soma.wrapping_add(*((e + 8) as *const i8));
    }
    soma > 2 // mais de dois convocados
}

/// { 0x776fc0 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::can_schedule_club_fixture(jogo)`: com a data
/// do jogo (0x549ef0 com +0x2a e +0x28), a janela de um dia antes (0x54c8e0) e um depois (0x54c5b0); jogo da competição
/// amistosa (0x5c6710 em [0xae2c74] com +0x14): 1. Senão 0 quando o mandante (+0x1c) ou o visitante (+0x20) tem mais
/// de 2 convocados na janela; senão 1. O quadro SEH só guarda o estado (as datas não têm destrutor).
unsafe extern "thiscall" fn pode_marcar_jogo_do_clube(this: usize, jogo: usize) -> u32 {
    let data_nova: unsafe extern "thiscall" fn(*mut u8, u32, u32) = core::mem::transmute(0x549ef0usize);
    let menos_dias: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais_dias: unsafe extern "thiscall" fn(*mut u8, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let amistosa: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let (mut d, mut a, mut b) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    // ponytail: a original empurra as palavras +0x2a e +0x28 com o alto do registro (lixo); a 0x549ef0 só lê as
    // palavras baixas: a cópia passa as palavras limpas.
    data_nova(d.as_mut_ptr(), *((jogo + 0x2a) as *const u16) as u32, *((jogo + 0x28) as *const u16) as u32);
    menos_dias(d.as_mut_ptr(), a.as_mut_ptr(), 1);
    mais_dias(d.as_mut_ptr(), b.as_mut_ptr(), 1);
    if amistosa(cpp12_d(0xae2c74), cpp12_d(jogo + 0x14)) != 0 {
        return 1;
    }
    let (pa, pb) = (a.as_ptr() as usize, b.as_ptr() as usize);
    if cpp12_muitos_convocados(this, cpp12_d(jogo + 0x1c), pa, pb) {
        return 0;
    }
    if cpp12_muitos_convocados(this, cpp12_d(jogo + 0x20), pa, pb) {
        crate::comparacao::ramo("cpp_12 visitante convocado");
        return 0; // o visitante com convocados demais
    }
    1
}

/// { 0x778e90 } (cdecl, national_teams.cpp): `compare_call_up_records_by_date(a, b)`: o qsort dos registros com a data
/// em +0 (as convocações de 9 bytes, as lesões, as transferências): −1 se a de `a` é antes (0x456770), 1 se é depois
/// (0x456740), senão 0. Quente (qsort): sem contador.
unsafe extern "C" fn ordem_das_convocacoes_pela_data(a: usize, b: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let (d0, d1) = (cpp12_d(b) as u32, cpp12_d(b + 4) as u32);
    let r = if antes(a, d0, d1) != 0 { -1 } else { (depois(a, d0, d1) != 0) as i32 };
    r // a convocação mais cedo primeiro
}

/// { 0x7779b0 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::add_qualified_nation(nacao, fase_final)`: com
/// as duas não nulas, acrescenta o registro de 8 bytes (nação, fase final) à lista das novas classificadas em
/// this + 0x28 (LLIST, 0x68d0d0).
unsafe extern "thiscall" fn acrescentar_nacao_classificada(this: usize, nacao: usize, fase_final: usize) {
    if nacao == 0 || fase_final == 0 {
        return;
    }
    let mut reg = [0u8; 8];
    reg[..4].copy_from_slice(&(nacao as u32).to_le_bytes());
    reg[4..].copy_from_slice(&(fase_final as u32).to_le_bytes());
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
    acrescentar(this + 0x28, reg.as_mut_ptr() as usize, 8);
}

/// Comparador: pelo valor (só lê; as datas são locais).
unsafe extern "thiscall" fn comparar_pode_marcar_jogo_do_clube(this: usize, jogo: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1039)(this, jogo);
    let n = pode_marcar_jogo_do_clube(this, jogo);
    PLACAR.registrar(r == n, || format!("pode_marcar_jogo_do_clube({:x}) original={} rust={}", jogo, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_das_convocacoes_pela_data(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1040)(a, b);
    let n = ordem_das_convocacoes_pela_data(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_convocacoes_pela_data({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: a original aloca o nó (não roda duas vezes): só ela, e confere o plano: a conta da lista (+0x28, palavra)
/// sobe 1 com as duas não nulas e fica igual senão.
unsafe extern "thiscall" fn comparar_acrescentar_nacao_classificada(this: usize, nacao: usize, fase_final: usize) {
    let antes = *((this + 0x28) as *const i16) as i32;
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1041)(this, nacao, fase_final);
    let depois = *((this + 0x28) as *const i16) as i32;
    let esperado = antes + (nacao != 0 && fase_final != 0) as i32;
    PLACAR.registrar(depois == esperado, || format!("acrescentar_nacao_classificada({:x}, {:x}) conta {} -> {}", nacao, fase_final, antes, depois));
}

// ---------- 252a leva: as copas e a divisão dos clubes: a ordem dos cabeças de chave do sorteio (decrescente e crescente) e o teto da divisão dos clubes no fim da temporada (Cup.cpp, league.cpp) ----------

unsafe fn cpp13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// O nível (+4, byte com sinal) e a reputação do clube (+0 → +0x80, palavra com sinal) de uma entrada de copa.
unsafe fn cpp13_nivel(e: usize) -> i32 {
    *((e + 4) as *const i8) as i32
}

unsafe fn cpp13_reputacao(e: usize) -> i32 {
    *((cpp13_d(e) + 0x80) as *const i16) as i32
}

/// { 0x522190 } (cdecl, Cup.cpp): `cup_compare_entries_by_level_desc(a, b)`: o qsort das entradas das copas (sorteios da
/// Copa da UEFA, Intertoto, copas do Japão e da Escócia) pelo nível, o maior primeiro; no empate pela reputação do
/// clube, a menor primeiro. Quente (qsort): sem contador.
unsafe extern "C" fn ordem_dos_cabecas_de_chave_decrescente(a: usize, b: usize) -> i32 {
    let r = cpp13_nivel(b) - cpp13_nivel(a); // o maior nível primeiro
    if r != 0 {
        return r;
    }
    cpp13_reputacao(a) - cpp13_reputacao(b)
}

/// { 0x5221c0 } (cdecl, Cup.cpp): `cup_compare_entries_by_level_asc(a, b)`: o qsort das entradas da copa da liga
/// inglesa (0x56ccc0) pelo nível, o menor primeiro; no empate pela reputação do clube, a menor primeiro. Quente
/// (qsort): sem contador.
unsafe extern "C" fn ordem_dos_cabecas_de_chave_crescente(a: usize, b: usize) -> i32 {
    let r = cpp13_nivel(a) - cpp13_nivel(b); // o menor nível primeiro
    if r != 0 {
        return r;
    }
    cpp13_reputacao(a) - cpp13_reputacao(b)
}

/// Leva o byte +0x64 do clube (com sinal) até `valor`: para baixo (`baixar` ≠ 0) se está acima, para cima se está
/// abaixo.
unsafe fn cpp13_ajustar(clube: usize, valor: i8, baixar: u32) {
    let p = (clube + 0x64) as *mut i8;
    let muda = if baixar != 0 { *p > valor } else { *p < valor };
    if muda {
        crate::comparacao::ramo("cpp_13 teto da divisao");
        *p = valor; // o teto novo
    }
}

/// { 0x68a980 } (thiscall, ret 0x10, league.cpp): `League::limit_club_division_tier(valor, fora, dentro, baixar)`
/// (chamada pelo fim de temporada de cada liga: Argentina, Bélgica, Brasil, Croácia, Dinamarca, Inglaterra, França,
/// Alemanha, Holanda, Itália, Rússia, Escócia, Suécia e Turquia): para cada linha da tabela (+0xb1, 0x3b bytes, conta
/// +0x3e), ajusta o byte +0x64 do clube (+0) a `valor` quando a situação da linha (+0x37) não é `fora` (0xfd = nenhuma)
/// e de novo quando é `dentro` (0xfd = nenhuma). Os três primeiros argumentos são bytes empurrados como dword: só o
/// byte baixo conta.
unsafe extern "thiscall" fn teto_da_divisao_dos_clubes(this: usize, valor: u32, fora: u32, dentro: u32, baixar: u32) {
    let (valor, fora, dentro) = (valor as u8 as i8, fora as u8, dentro as u8);
    let mut i: i16 = 0;
    while i < *((this + 0x3e) as *const i16) {
        let linha = || cpp13_d(this + 0xb1).wrapping_add(i as isize as usize * 0x3b);
        if fora != 0xfd && *((linha() + 0x37) as *const u8) != fora {
            cpp13_ajustar(cpp13_d(linha()), valor, baixar);
        }
        if dentro != 0xfd && *((linha() + 0x37) as *const u8) == dentro {
            cpp13_ajustar(cpp13_d(linha()), valor, baixar);
        }
        i = i.wrapping_add(1);
    }
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_dos_cabecas_de_chave_decrescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1042)(a, b);
    let n = ordem_dos_cabecas_de_chave_decrescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_cabecas_de_chave_decrescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_ordem_dos_cabecas_de_chave_crescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1043)(a, b);
    let n = ordem_dos_cabecas_de_chave_crescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_cabecas_de_chave_crescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// Comparador: grava só o byte +0x64 dos clubes da tabela: lado a lado nesses bytes.
unsafe extern "thiscall" fn comparar_teto_da_divisao_dos_clubes(this: usize, valor: u32, fora: u32, dentro: u32, baixar: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32)>(1044);
    let n = *((this + 0x3e) as *const i16);
    let tabela = cpp13_d(this + 0xb1);
    let reg: Vec<(usize, usize)> = (0..n.max(0) as usize).map(|i| (cpp13_d(tabela + i * 0x3b) + 0x64, 1)).collect();
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, valor, fora, dentro, baixar),
        || teto_da_divisao_dos_clubes(this, valor, fora, dentro, baixar));
    PLACAR.registrar(igual, || format!("teto_da_divisao_dos_clubes(valor {}, fora {:x}, dentro {:x}, baixar {}) {:x?} x {:x?}",
        valor as u8 as i8, fora as u8, dentro as u8, baixar, mo, mr));
}
