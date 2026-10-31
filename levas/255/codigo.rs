// ---------- 255a leva: o pedido de transferência do jogador, os cargos com contrato e quem não é da diretoria (contract_manager.cpp) ----------

unsafe fn cps1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps1_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x4d8610 } (thiscall, ret 4, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// handle_transfer_request(contrato)`: contrato nulo: a caixa "Error" 0x2999. Pessoa sem ficha de jogador (+0x61): nada.
/// Clube humano (0x7875e0): a notícia do pedido (0xbbf), montada na pilha. No clube do computador, fora do status de
/// elenco 1 (+0x4f & 0xf0 = 0x10): se o valor (0x4d0c60) × 1,5 passa do preço pedido (0x4d1600), o clube o põe na lista
/// (+0x4e: tira o bit 2 e liga o 8; moral +12 em 0x544710; as listas dos clubes 0x842c00; 0x4d71b0). Senão um sorteio
/// pela força do pedido (+0x44) e pelo ânimo (+0x59) contra outro de 50 dá a queixa 0x10000 (0x867700); a moral cai 12
/// e a força do pedido sobe ânimo × 5 + 56, presa em ±100.
unsafe extern "thiscall" fn pedido_de_transferencia(this: usize, contrato: usize) {
    if contrato == 0 {
        return aviso_em(0x9a8190, 0x2999);
    }
    let clube = cps1_d(0xae23b4).wrapping_add(cps1_d(contrato + 4).wrapping_mul(0x245));
    let pessoa = cps1_d(0xae23bc).wrapping_add(cps1_d(contrato).wrapping_mul(0x6e));
    if cps1_d(pessoa + 0x61) == 0 {
        return;
    }
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    if humano(cps1_d(0xb7431c), clube) != 0 {
        // ponytail: o clube humano recebe a notícia montada na pilha (0x77bbe0 e a entrega estão desligadas): esse
        // caso fica com a original (que refaz a pergunta 0x7875e0, sem efeito).
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(1116)(this, contrato);
    }
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x544710usize);
    if cps1_b(contrato + 0x4f) & 0xf0 != 0x10 {
        let preco: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
        let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
        let a = preco(this, clube, 0, pessoa, 0, 1, 1);
        let v = valor(this, pessoa, clube, 0, -1, 0);
        // o 1,5 é o double 0x968940
        if (v as f64) * 1.5 > a as f64 { // cps1: o valor passa do preço
            crate::comparacao::ramo("cps_1 listado pelo clube");
            let m = (contrato + 0x4e) as *mut u8;
            if *m & 2 != 0 {
                *m &= 0xfd;
            }
            *m |= 8;
            moral(pessoa, 0xc, 0);
            let listas: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x842c00usize);
            listas(cps1_d(cps1_d(0xde1f54) + 0x856), pessoa, 1);
            let listar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4d71b0usize);
            listar(this, pessoa, clube);
            return;
        }
    }
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let r = sorteio(cps1_b(contrato + 0x44) as i8 as i32 + (cps1_b(pessoa + 0x59) as i8 as i32 + 5) * 20);
    if r < sorteio(0x32) { // cps1: o sorteio da queixa
        crate::comparacao::ramo("cps_1 queixa do pedido");
        let queixa: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x867700usize);
        queixa(contrato, 0x10000, 0);
    }
    crate::comparacao::ramo("cps_1 pedido no clube do computador");
    moral(pessoa, 0xffff_fff4, 0); // cps1: a moral cai 12
    // relidos depois das chamadas, como na original (o `imul` de 8 bits fica com o byte baixo)
    let al = cps1_b(pessoa + 0x59).wrapping_mul(5).wrapping_add(0x38);
    let cl = cps1_b(contrato + 0x44);
    let s = cl as i8 as i32 + al as i8 as i32;
    let m = (contrato + 0x44) as *mut u8;
    *m = if s > 100 {
        100
    } else if s < -100 {
        0x9c
    } else {
        cl.wrapping_add(al)
    };
}

/// { 0x4cbb20 } (stdcall, ret 4, contract_manager.cpp): `is_contract_staff_job(cargo)`: 1 para os cargos (byte) 0, 6 a
/// 0xd e 0xf, os que têm contrato pelo gerente de contratos; senão 0.
unsafe extern "stdcall" fn cargo_com_contrato(cargo: u32) -> u32 {
    matches!(cargo as u8, 0 | 6..=0xd | 0xf) as u32
}

/// { 0x4cb960 } (stdcall, ret 4, contract_manager.cpp): `staff_is_not_board_member(pessoa)`: 0 se o cargo (+0x3d) é de
/// diretoria (1 a 4) num clube (+0x39); senão 1.
unsafe extern "stdcall" fn fora_da_diretoria(pessoa: usize) -> u32 {
    let cargo = cps1_b(pessoa + 0x3d);
    if (1..=4).contains(&cargo) && cps1_d(pessoa + 0x39) != 0 {
        return 0;
    }
    1
}

// ponytail: sorteia (0x90cfc0), põe na lista (0x842c00, 0x4d71b0), muda moral e queixas (que podem chamar o pedido de
// novo): só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_pedido_de_transferencia(this: usize, contrato: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1116)(this, contrato)
}

/// Comparador: pelo valor.
unsafe extern "stdcall" fn comparar_cargo_com_contrato(cargo: u32) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(u32) -> u32>(1117)(cargo);
    let n = cargo_com_contrato(cargo);
    PLACAR.registrar(r == n, || format!("cargo_com_contrato({}) original={} rust={}", cargo as u8, r, n));
    r
}

/// Comparador: pelo valor.
unsafe extern "stdcall" fn comparar_fora_da_diretoria(pessoa: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(1118)(pessoa);
    let n = fora_da_diretoria(pessoa);
    PLACAR.registrar(r == n, || format!("fora_da_diretoria(pessoa {}, cargo {}) original={} rust={}", cps1_d(pessoa), cps1_b(pessoa + 0x3d), r, n));
    r
}

// ---------- 255a leva: as permissões de trabalho do começo do jogo, a contagem da cláusula e o prêmio por jogo do jogador (transfer_manager.cpp) ----------

unsafe fn cps2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps2_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps2_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// { 0x8c6e60 } (thiscall, ret 0, transfer_manager.cpp, quadro SEH só da data local): `TRANSFER_MANAGER::
/// init_work_permits()`: pela criação do jogo (0x8bba30). Para cada pessoa: com mais de 13 jogos pela seleção (+0x22),
/// 0x3fff na palavra +3 do registro de 8 bytes ([this+0x86e] + i × 8); com mais de 5, 0x1fff. Jogador (+0x3d = 0xb) de
/// clube (+0x39) que precisa de permissão de trabalho (0x8c7560 sem ignorar a dada): o ano mais antigo na nação do
/// clube pela carreira (0x7c3e40, um grupo de 6 entradas de 0x2b bytes por temporada; para no primeiro clube de outra
/// nação), começando pelo ano de +0x40. Menos de 5 anos até o ano de hoje ([0xae2c92]): a permissão (0x8c7070, sem
/// ordenar; antes a original leva a data +0x3e de 52 em 52 semanas até a data [0xdd6ec8], sem usar o resultado); senão a
/// segunda nacionalidade (+0x1e) vira a nação do clube. No fim ordena as permissões (qsort 0x945333 com 0x8e55c0).
unsafe extern "thiscall" fn iniciar_permissoes_de_trabalho(this: usize) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mais_semanas: unsafe extern "thiscall" fn(usize, usize, i32) -> usize = core::mem::transmute(0x54c770usize);
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    let carreira: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(0x7c3e40usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let dar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x8c7070usize);
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let mut d = [0u8; 8];
    data_padrao(d.as_mut_ptr() as usize);
    let mut i = 0usize;
    let mut p = cps2_d(0xae23bc);
    while (i as i32) < cps2_d(0xae2364) as i32 {
        let jogos = cps2_b(p + 0x22);
        let marca = (cps2_d(this + 0x86e).wrapping_add(i.wrapping_mul(8)) + 3) as *mut u16;
        if jogos > 0xd {
            core::ptr::write_unaligned(marca, 0x3fff);
        } else if jogos > 5 {
            core::ptr::write_unaligned(marca, 0x1fff);
        }
        let clube = cps2_d(p + 0x39);
        if clube != 0 && cps2_b(p + 0x3d) == 0xb && precisa(this, p, clube, 0) != 0 {
            let mut ano = cps2_w(p + 0x40);
            let mut conta = 0i32;
            let t = carreira(cps2_d(0xb74350), p, &mut conta);
            let (mut k, mut e) = (0i32, t + 2);
            while k < conta {
                let id = cps2_d(e) as i32;
                if id >= 0 && id < cps2_d(0xae235c) as i32 {
                    let c = cps2_d(0xae23b4).wrapping_add((id as usize).wrapping_mul(0x245));
                    if cps2_d(c + 0x53) != cps2_d(cps2_d(p + 0x39) + 0x53) {
                        break; // clube de outra nação
                    }
                    let a = cps2_w(e - 2);
                    if a < ano { // cps2: o ano mais antigo
                        crate::comparacao::ramo("cps_2 ano mais antigo");
                        ano = a;
                    }
                }
                k += 6;
                e += 0x102;
            }
            if t != 0 {
                liberar(t);
            }
            if (cps2_w(0xae2c92) as i32) - (ano as i32) < 5 { // cps2: menos de cinco anos na nação
                // a data +0x3e levada de 52 em 52 semanas até a data [0xdd6ec8] (a original não usa o resultado)
                let (mut t1, mut t2, mut t3) = ([0u8; 8], [0u8; 8], [0u8; 8]);
                let dp = d.as_mut_ptr() as usize;
                copiar(dp, t1.as_mut_ptr() as usize, p + 0x3e);
                loop {
                    let (ah, dh) = (cps2_w(0xdd6eca), cps2_w(dp + 2));
                    if !(ah > dh || (ah == dh && (cps2_w(0xdd6ec8) as i16) > (cps2_w(dp) as i16))) {
                        break;
                    }
                    let r = mais_semanas(dp, t2.as_mut_ptr() as usize, 0x34);
                    copiar(dp, t3.as_mut_ptr() as usize, r);
                }
                crate::comparacao::ramo("cps_2 permissao pela carreira");
                // ponytail: a original empurra o registrador do ano com os 16 bits altos de um ponteiro; a 0x8c7070
                // só grava a palavra baixa
                dar(this, p, cps2_d(cps2_d(p + 0x39) + 0x53), ano as u32, 1, 0);
            } else {
                crate::comparacao::ramo("cps_2 segunda nacionalidade");
                core::ptr::write_unaligned((p + 0x1e) as *mut usize, cps2_d(cps2_d(p + 0x39) + 0x53)); // cps2: a segunda nacionalidade
            }
        }
        i += 1;
        p += 0x6e;
    }
    qsort(cps2_d(this + 0x93a), cps2_d(this + 0x93e) as u32, 0xe, 0x8e55c0);
}

/// { 0x8c3e50 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::get_clause_count(pessoa, tipo)`: no tipo 4,
/// os jogos pela seleção (+0x22) menos os da época do acordo (byte +7 do registro de 8 bytes [this+0x86e] + id × 8);
/// senão os jogos pelo clube (+0x3b) do contrato (0x4cf5a0, pedido duas vezes como na original), ou 0 sem contrato.
unsafe extern "thiscall" fn contagem_da_clausula(this: usize, pessoa: usize, tipo: u32) -> u16 {
    if tipo as u8 == 4 {
        let r = cps2_d(this + 0x86e).wrapping_add(cps2_d(pessoa).wrapping_mul(8));
        return (cps2_b(pessoa + 0x22) as u16).wrapping_sub(cps2_b(r + 7) as u16);
    }
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    if contrato_de(cps2_d(0xae1050), pessoa) == 0 {
        return 0;
    }
    cps2_w(contrato_de(cps2_d(0xae1050), pessoa) + 0x3b)
}

/// { 0x8d0ea0 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_first_team_appearance_money(pessoa)`:
/// com o bit 2 do registro de 8 bytes ([this+0x86e] + id × 8), busca o prêmio por jogo da pessoa no vetor [this+0x88c]
/// (conta +0x894; busca binária 0x946464 com 0x8b4e00, registros de 13 bytes) e o devolve; sem achar, tira o bit (soma
/// 0xfe ao byte) e devolve 0.
unsafe extern "thiscall" fn premio_por_jogo_do_jogador(this: usize, pessoa: usize) -> usize {
    let id = cps2_d(pessoa);
    if cps2_b(cps2_d(this + 0x86e).wrapping_add(id.wrapping_mul(8))) & 2 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const i32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = id as i32;
    let r = bsearch(&chave, cps2_d(this + 0x88c), cps2_d(this + 0x894), 0xd, 0x8b4e00);
    if r != 0 {
        return r;
    }
    let m = cps2_d(this + 0x86e).wrapping_add(cps2_d(pessoa).wrapping_mul(8)) as *mut u8;
    *m = (*m).wrapping_add(0xfe);
    0
}

// ponytail: cresce o vetor das permissões (0x8c7070) e grava em todas as pessoas: só a original; a cópia leva os ramos
// (roda uma vez, na criação do jogo).
unsafe extern "thiscall" fn comparar_iniciar_permissoes_de_trabalho(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1119)(this)
}

/// Comparador: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_contagem_da_clausula(this: usize, pessoa: usize, tipo: u32) -> u16 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u16>(1120)(this, pessoa, tipo);
    let n = contagem_da_clausula(this, pessoa, tipo);
    PLACAR.registrar(r == n, || format!("contagem_da_clausula(pessoa {}, tipo {}) original={} rust={}", cps2_d(pessoa), tipo as u8, r, n));
    r
}

/// Comparador: lado a lado pelo byte do registro da pessoa, e pela devolução.
unsafe extern "thiscall" fn comparar_premio_por_jogo_do_jogador(this: usize, pessoa: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1121);
    let m = cps2_d(this + 0x86e).wrapping_add(cps2_d(pessoa).wrapping_mul(8));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(m, 1)], || o(this, pessoa), || premio_por_jogo_do_jogador(this, pessoa));
    PLACAR.registrar(igual && r == n, || format!("premio_por_jogo_do_jogador(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", cps2_d(pessoa), r, n, mo, mr));
    r
}

// ---------- 255a leva: a fila da transferência livre, a próxima janela aberta e o empréstimo negociado (transfer_manager.cpp) ----------

unsafe fn cps3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps3_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cps3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// { 0x8c18c0 } (thiscall, ret 0, transfer_manager.cpp, quadro SEH das duas LLIST locais 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::process_free_transfer_queue()`: pelo dia a dia das transferências (0x8c5880). Percorre uma cópia
/// do cabeçalho da fila (+0x92c, 14 bytes, com o cursor no começo); para cada item (+0 id da pessoa, +4 motivo, +5
/// compensação) com contrato (0x4cf5a0, guardado em [0xae1080]) e clube: no clube humano (0x7875e0) sem motivo, a
/// diretoria pode barrar (0x696940; barrando, 0x6a0990 e o próximo); senão o clube entra nos rejeitados (0x547e70) e o
/// técnico (+0xcf) nos desafetos (0x547af0), e a diretoria registra (0x697430). Sem motivo, um sorteio pelo +0x5b contra
/// outro de 20 (ou gostar do clube, 0x547d60) põe o técnico nos desafetos. Depois os registros da liberação (0x86d800),
/// as ofertas canceladas (0x8df9b0) e a liberação (0x8c1af0 com motivo e compensação). No fim a cópia vira a lista
/// vazia, a fila é limpa (0x68d080) e as duas locais são destruídas.
unsafe extern "thiscall" fn fila_da_transferencia_livre(this: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let barra: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x696940usize);
    let barrado: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6a0990usize);
    let rejeitado: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547e70usize);
    let desafeto: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
    let diretoria: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x697430usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    let registros: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x86d800usize);
    let cancelar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8df9b0usize);
    let liberar: unsafe extern "thiscall" fn(usize, usize, usize, i32, usize, u32) = core::mem::transmute(0x8c1af0usize);
    let (mut l1, mut l2) = ([0u8; 14], [0u8; 14]);
    let (a, b) = (l1.as_mut_ptr() as usize, l2.as_mut_ptr() as usize);
    nova(a);
    nova(b);
    let fila = this + 0x92c;
    // a cópia do cabeçalho: os 10 primeiros bytes e o cursor (+0xa) no começo (+2)
    core::ptr::copy_nonoverlapping(fila as *const u8, a as *mut u8, 0xa);
    cps3_por(a + 0xa, cps3_d(a + 2));
    let mut e = proximo(a);
    while e != 0 {
        let p = cps3_d(0xae23bc).wrapping_add(cps3_d(e).wrapping_mul(0x6e));
        let c = contrato_de(cps3_d(0xae1050), p);
        cps3_por(0xae1080, c);
        let clube = if c != 0 { cps3_d(0xae23b4).wrapping_add(cps3_d(c + 4).wrapping_mul(0x245)) } else { 0 };
        if clube != 0 {
            let mut seguir = true;
            if humano(cps3_d(0xb7431c), clube) != 0 && cps3_b(e + 4) == 0 {
                let r = barra(cps3_d(0xb67a44), p, clube);
                if r as u16 != 0 {
                    barrado(cps3_d(0xb67a44), p, clube, r);
                    seguir = false;
                } else {
                    if cps3_b(e + 4) == 0 {
                        rejeitado(p, clube, 0);
                        if cps3_d(clube + 0xcf) != 0 {
                            desafeto(p, cps3_d(clube + 0xcf), 1);
                        }
                    }
                    diretoria(cps3_d(0xb67a44), p, clube);
                }
            }
            if seguir {
                if cps3_b(e + 4) == 0 {
                    let r = sorteio(cps3_b(p + 0x5b) as i8 as i32);
                    if r < sorteio(0x14) || gosta(p, clube) != 0 { // cps3: o sorteio do desafeto
                        crate::comparacao::ramo("cps_3 desafeto do tecnico");
                        desafeto(p, cps3_d(clube + 0xcf), 1);
                    }
                }
                crate::comparacao::ramo("cps_3 liberado");
                registros(cps3_d(0xdd7f00), p); // cps3: os registros da liberação
                cancelar(this, p);
                let comp = cps3_d(e + 5); // cps3: a compensação ao clube
                if comp != 0 {
                    crate::comparacao::ramo("cps_3 compensacao");
                }
                liberar(this, p, clube, cps3_b(e + 4) as i8 as i32, comp, 0);
            }
        }
        e = proximo(a);
    }
    // a cópia vira a lista vazia (os 14 bytes), a fila é limpa e as duas locais saem
    core::ptr::copy_nonoverlapping(b as *const u8, a as *mut u8, 14);
    limpar(fila);
    soltar(b);
    soltar(a);
}

/// { 0x8cf470 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH só das datas locais): `TRANSFER_MANAGER::
/// find_next_window_open_date(clube, saida)`: a regra da nação do clube (+0x53) entre as 28 de +0x8ac (+0xa); de hoje
/// ([0xae2c90]) até um ano depois (0x54c5b0 com 365), o primeiro dia com a janela aberta (0x8e8e50; sem regra, hoje) vai
/// para `saida`; sem achar, a caixa "Error" 0x32c6. A contratação desejada (0x85fc50) regrava a data com hoje logo depois.
unsafe extern "thiscall" fn proxima_janela_aberta(this: usize, clube: usize, saida: usize) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let aberta: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8e8e50usize);
    let (mut d, mut lim, mut t1, mut t2) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let (dp, lp, p1, p2) = (d.as_mut_ptr() as usize, lim.as_mut_ptr() as usize, t1.as_mut_ptr() as usize, t2.as_mut_ptr() as usize);
    data_padrao(dp);
    mais(0xae2c90, lp, 0x16d);
    let nacao = cps3_d(clube + 0x53);
    let mut regra = 0usize;
    for k in 0..28 {
        let r = cps3_d(this + 0x8ac + k * 4);
        if cps3_d(r + 0xa) == nacao {
            regra = r;
            break;
        }
    }
    copiar(dp, p1, 0xae2c90);
    loop {
        let (la, da) = (cps3_w(lp + 2), cps3_w(dp + 2));
        if !(la > da || (la == da && (cps3_w(lp) as i16) > (cps3_w(dp) as i16))) {
            return aviso_em(0xa8d514, 0x32c6);
        }
        if regra == 0 || aberta(regra, dp, 0) != 0 {
            copiar(saida, p2, dp);
            return;
        }
        let r = mais(dp, p1, 1);
        copiar(dp, p2, r);
    }
}

/// { 0x8c72d0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_negotiated_loan_contract(pessoa,
/// clube)`: na lista [this+0x946] (o cursor +0xa volta ao começo), o item com a pessoa (+0) e o clube (+4) pelos ids;
/// sem lista ou sem achar, 0.
unsafe extern "thiscall" fn emprestimo_negociado(this: usize, pessoa: usize, clube: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = cps3_d(this + 0x946);
    if l == 0 {
        return 0;
    }
    cps3_por(l + 0xa, cps3_d(l + 2));
    let mut e = proximo(cps3_d(this + 0x946));
    while e != 0 {
        if cps3_d(pessoa) == cps3_d(e) && cps3_d(clube) == cps3_d(e + 4) {
            return e;
        }
        e = proximo(cps3_d(this + 0x946));
    }
    0
}

// ponytail: sorteia (0x90cfc0) e libera os jogadores (0x8c1af0, 0x8df9b0), limpando a fila: só a original; a cópia
// leva os ramos.
unsafe extern "thiscall" fn comparar_fila_da_transferencia_livre(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1122)(this)
}

/// Comparador: lado a lado pela data de saída (a data estática da 0x54c5b0 é rascunho). Sem janela num ano, a caixa
/// sairia duas vezes (não acontece com as regras do jogo).
unsafe extern "thiscall" fn comparar_proxima_janela_aberta(this: usize, clube: usize, saida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1123);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, 8)], || o(this, clube, saida), || proxima_janela_aberta(this, clube, saida));
    PLACAR.registrar(igual, || format!("proxima_janela_aberta(clube {:x}) {:x?} x {:x?}", clube, mo, mr));
}

/// Comparador: pelo valor (as duas voltas deixam o cursor da lista no mesmo lugar).
unsafe extern "thiscall" fn comparar_emprestimo_negociado(this: usize, pessoa: usize, clube: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(1124)(this, pessoa, clube);
    let n = emprestimo_negociado(this, pessoa, clube);
    PLACAR.registrar(r == n, || format!("emprestimo_negociado(pessoa {}, clube {}) original={:x} rust={:x}", cps3_d(pessoa), cps3_d(clube), r, n));
    r
}

// ---------- 255a leva: a contraproposta com cláusula e os motivos da recusa do lance (transfer_manager.cpp) ----------

unsafe fn cps4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps4_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cps4_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O valor da oferta (+0x1b) × `f`, truncado (_ftol 0x945640; o x87 em precisão dupla).
unsafe fn cps4_vezes(o: usize, f: f64) -> usize {
    ftol(cps4_d(o + 0x1b) as i32 as f64 * f) as usize
}

/// O clube do contrato da pessoa (0x4cf5a0, guardado em [0xae1080]), ou 0.
unsafe fn cps4_clube_atual(pessoa: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cps4_d(0xae1050), pessoa);
    cps4_por(0xae1080, c);
    if c == 0 {
        0
    } else {
        cps4_d(0xae23b4).wrapping_add(cps4_d(c + 4).wrapping_mul(0x245))
    }
}

/// { 0x8c8fd0 } (thiscall com o `this` sem uso, ret 0x14, transfer_manager.cpp, quadro SEH do `new`):
/// `TRANSFER_MANAGER::build_clause_offer_option(clube, jogador, saida, apelo, valor)`: pela resposta do clube do computador
/// ao lance (0x8c8090). Com o clube do lance mais de 3 acima na reputação (+0x80) do clube atual (0x4cf5a0), ficha de
/// jogador (+0x61) com potencial (+7) ao menos 25 acima da habilidade (+5) e até 30 anos (+0x18): uma oferta nova
/// (0x944cf1 de 0x41 bytes, 0x8e5680) em `*saida`, iniciada com o valor (0x8e78f0), e apelo 50. Clube do lance abaixo de
/// 15 com a reputação da ficha (+0xb / 500) mais de 2 acima: 20% da próxima venda (+0x3f), o valor × 0,8 e apelo 80.
/// Senão, jogador sem jogos pela seleção (+0x22) de nação (+0x1a) acima de 5000 (+0x8e): o prêmio por jogo na seleção
/// (+0x3a, valor × 0,3, por 10 jogos +0x3e), o valor × 0,8 e apelo 70. Senão, reputação da ficha mais de 3 abaixo da do
/// clube: o prêmio por jogo no time (+0x35, valor × 0,3, por 20 jogos +0x39), o valor × 0,8 e apelo 60. Nenhum: a
/// oferta é destruída (0x8e5710, 0x944c94; `*saida` fica com o ponteiro, como na original) e 0.
unsafe extern "thiscall" fn oferta_com_clausula(_this: usize, clube: usize, jogador: usize, saida: *mut usize, apelo: *mut u8, valor: u32) -> u32 {
    let atual = cps4_clube_atual(jogador);
    if (cps4_w(clube + 0x80) as i16 as i32) - (cps4_w(atual + 0x80) as i16 as i32) <= 3 {
        return 0;
    }
    let ficha = cps4_d(jogador + 0x61);
    if ficha == 0 || (cps4_w(ficha + 7) as i16 as i32) - (cps4_w(ficha + 5) as i16 as i32) < 0x19 || cps4_b(jogador + 0x18) as i8 > 0x1e {
        return 0;
    }
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x8e5680usize);
    let iniciar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x8e78f0usize);
    let m = novo(0x41);
    let o = if m != 0 { construir(m) } else { 0 };
    *saida = o;
    iniciar(o, jogador, clube, valor);
    *apelo = 0x32;
    let rep = cps4_w(clube + 0x80) as i16;
    // a reputação da ficha (+0xb) / 500, com o sinal (a multiplicação mágica da original)
    let fama = || (cps4_w(cps4_d(jogador + 0x61) + 0xb) as i16 as i32) / 500;
    if rep < 0xf && fama() - (rep as i32) > 2 {
        crate::comparacao::ramo("cps_4 percentual da venda");
        let o = *saida;
        cps4_por(o + 0x1b, cps4_vezes(o, 0.8)); // cps4: o valor do percentual
        *((*saida + 0x3f) as *mut u8) = 0x14;
        *apelo = 0x50;
        return 1;
    }
    let nacao = cps4_d(jogador + 0x1a);
    if nacao != 0 && cps4_w(nacao + 0x8e) as i16 > 0x1388 && cps4_b(jogador + 0x22) == 0 {
        crate::comparacao::ramo("cps_4 premio na selecao");
        let o = *saida;
        cps4_por(o + 0x3a, cps4_vezes(o, 0.3));
        *((*saida + 0x3e) as *mut u8) = 0xa;
        let o = *saida;
        cps4_por(o + 0x1b, cps4_vezes(o, 0.8)); // cps4: o valor do prêmio na seleção
        *apelo = 0x46;
        return 1;
    }
    if fama() - (rep as i32) < -3 {
        crate::comparacao::ramo("cps_4 premio no time");
        let o = *saida;
        cps4_por(o + 0x35, cps4_vezes(o, 0.3));
        *((*saida + 0x39) as *mut u8) = 0x14;
        let o = *saida;
        cps4_por(o + 0x1b, cps4_vezes(o, 0.8)); // cps4: o valor do prêmio no time
        *apelo = 0x3c;
        return 1;
    }
    let o = *saida;
    if o != 0 {
        let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        destruir(o);
        liberar(o);
    }
    0
}

/// { 0x8c5560 } (stdcall, ret 8, transfer_manager.cpp): `transfer_manager_is_rejection_reason_valid(motivo, oferta)`: 0
/// quando o motivo não cabe na oferta (pessoa +0xc, clube do lance +8): 0x2000 (promessa) com 24 anos ou mais (+0x18);
/// 0x1000 com o clube atual (0x4cf5a0) e o do lance em divisões diferentes (+0x57); 0x800 (rival) com os dois clubes,
/// sem os dois humanos (0x600a50), sem confronto na liga (0x4b02e0) e sem um ser parceiro (+0xb3/+0xb7/+0xbb) do outro.
/// Senão 1.
unsafe extern "stdcall" fn motivo_da_recusa_vale(motivo: u32, oferta: usize) -> u32 {
    let pessoa = cps4_d(0xae23bc).wrapping_add(cps4_d(oferta + 0xc).wrapping_mul(0x6e));
    let atual = cps4_clube_atual(pessoa);
    let lance = cps4_d(0xae23b4).wrapping_add(cps4_d(oferta + 8).wrapping_mul(0x245));
    let r = match motivo as u16 {
        0x2000 => (cps4_b(pessoa + 0x18) as i8) < 0x18,
        0x1000 => atual == 0 || cps4_d(atual + 0x57) == cps4_d(lance + 0x57),
        0x800 => {
            if lance == 0 || atual == 0 {
                return 1;
            }
            let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
            let confronto: unsafe extern "C" fn(usize, usize) -> u8 = core::mem::transmute(0x4b02e0usize);
            if humano(atual, 0, 1, 0, 0) != 0 && humano(lance, 0, 1, 0, 0) != 0 {
                return 1;
            }
            confronto(lance, atual) != 0
                || [0xb3, 0xb7, 0xbb].iter().any(|&k| cps4_d(lance + k) == atual)
                || [0xb3, 0xb7, 0xbb].iter().any(|&k| cps4_d(atual + k) == lance)
        }
        _ => true,
    };
    r as u32
}

/// { 0x8c54c0 } (stdcall, ret 4, transfer_manager.cpp): `transfer_manager_get_squad_status_reason(oferta)`: o motivo de
/// recusa pelo status no elenco (+0x4f >> 4) do contrato da pessoa (+0xc) com o clube atual (0x4cf5f0): 1 → 0x200, 2 e
/// 3 → 0x400, 5 → 0x2000; senão 0.
unsafe extern "stdcall" fn motivo_pelo_status_no_elenco(oferta: usize) -> u16 {
    let pessoa = cps4_d(0xae23bc).wrapping_add(cps4_d(oferta + 0xc).wrapping_mul(0x6e));
    let atual = cps4_clube_atual(pessoa);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let c = com_clube(cps4_d(0xae1050), pessoa, atual);
    if c == 0 {
        return 0;
    }
    match cps4_b(c + 0x4f) >> 4 {
        1 => 0x200,
        2 | 3 => 0x400,
        5 => 0x2000,
        _ => 0,
    }
}

// ponytail: aloca e solta a oferta (0x944cf1, 0x944c94): só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_oferta_com_clausula(this: usize, clube: usize, jogador: usize, saida: *mut usize, apelo: *mut u8, valor: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, *mut usize, *mut u8, u32) -> u32>(1125)(this, clube, jogador, saida, apelo, valor)
}

/// Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
unsafe extern "stdcall" fn comparar_motivo_da_recusa_vale(motivo: u32, oferta: usize) -> u32 {
    let o = orig::<unsafe extern "stdcall" fn(u32, usize) -> u32>(1126);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(0xae1080, 4)], || o(motivo, oferta), || motivo_da_recusa_vale(motivo, oferta));
    PLACAR.registrar(igual && r == n, || format!("motivo_da_recusa_vale({:x}, pessoa {}) original={} rust={} {:x?} x {:x?}", motivo as u16, cps4_d(oferta + 0xc), r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelo contrato da vez ([0xae1080]), e pela devolução.
unsafe extern "stdcall" fn comparar_motivo_pelo_status_no_elenco(oferta: usize) -> u16 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> u16>(1127);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(0xae1080, 4)], || o(oferta), || motivo_pelo_status_no_elenco(oferta));
    PLACAR.registrar(igual && r == n, || format!("motivo_pelo_status_no_elenco(pessoa {}) original={:x} rust={:x} {:x?} x {:x?}", cps4_d(oferta + 0xc), r, n, mo, mr));
    r
}

// ---------- 255a leva: as relações do elenco no dia a dia: as brigas e preferências entre colegas, o atrito com o técnico e o moral espalhado pela liderança (game.cpp) ----------

unsafe fn cps5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps5_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cps5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cps5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O clube de índice `i` (registros de 0x245 bytes em [0xae23b4]; a conta em 32 bits, como a original).
unsafe fn cps5_clube(i: usize) -> usize {
    cps5_d(0xae23b4).wrapping_add(i.wrapping_mul(0x245))
}

/// O contrato da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz a cada chamada.
unsafe fn cps5_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cps5_d(0xae1050), p);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    c
}

/// O clube do contrato (pelo id em +4), ou 0 sem contrato.
unsafe fn cps5_clube_do_contrato(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        cps5_clube(cps5_d(c + 4))
    }
}

/// O vínculo da pessoa com o clube (0x4cf5f0).
unsafe fn cps5_vinculo(p: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    f(cps5_d(0xae1050), p, clube)
}

/// `pessoa` não gosta de `alvo` (0x547840)?
unsafe fn cps5_nao_gosta(alvo: usize, pessoa: usize) -> u32 {
    let f: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    f(alvo, pessoa)
}

/// O fim comum do atrito (0x5d6d10 e 0x5d7190): o primeiro colega do elenco (+0xd7, 50 vagas) de quem a pessoa não
/// gosta; achado, e só quando dia % 14 + id da pessoa dá zero (na prática a pessoa de id 0 a cada 14 dias), a queixa
/// (0x587c60) contra o técnico (+0xcf, tipo 0), o auxiliar (+0xd3, 1) ou outro (2) e a marca no vínculo `k` (0x867700
/// com 0x2000, 0x1000 ou 0x800).
unsafe fn cps5_atrito(p: usize, clube: usize, k: usize) {
    let mut j = 0usize;
    loop {
        if j >= 0x32 {
            return;
        }
        let q = cps5_d(clube + 0xd7 + j * 4);
        if q != p && q != 0 && cps5_nao_gosta(q, p) != 0 {
            break;
        }
        j += 1;
    }
    let q = cps5_d(clube + 0xd7 + j * 4);
    if q == 0 {
        return;
    }
    if ((cps5_w(0xae2c90) as i32) % 0xe).wrapping_add(cps5_d(p) as i32) != 0 {
        return;
    }
    crate::comparacao::ramo("cps_5 atrito com o tecnico");
    let queixa: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x587c60usize);
    let marcar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x867700usize);
    let (tipo, marca) = if q == cps5_d(clube + 0xcf) {
        (0, 0x2000)
    } else if q == cps5_d(clube + 0xd3) {
        (1, 0x1000)
    } else {
        (2, 0x800)
    };
    queixa(p, q, tipo);
    marcar(k, marca, 0);
}

/// { 0x5d6d10 } (cdecl, game.cpp): `game_update_squad_relationships()`, todo dia pelo laço do dia do jogo. (1) Dez
/// vezes: um clube sorteado e duas vagas do elenco sorteadas (50); pessoas diferentes e sorteio(+0x5b da primeira) = 0:
/// se a primeira não gosta da segunda, deixa de não gostar (0x547c50) e, no mesmo clube de contrato e com o vínculo
/// marcado em +0x45 & 0x800, a briga (0x5888a0); senão, 1/3 passa a não gostar (0x547af0), 1/3 do resto passa a gostar
/// (0x547a10) e o resto deixa de gostar (0x547c10). (2) Dia múltiplo de 50: [0xae235c]/100 clubes sorteados passam
/// pelo atrito no vestiário (0x5d7190). Nos outros dias, cada pessoa (menos as 17 últimas) com ficha de jogador e
/// preferências, contrato e o vínculo do clube com a pessoa de +0xff: três sorteios (+0x5b, +0x5d, +0x5a, cada um + 5)
/// filtram; a soma de cinco atributos menos 3, 5, 15, 15 por sorteios zerados e 5 se a reputação do jogador (+0xb)
/// passa de 4500 e da do clube (+0x80) dá o sorteio que, zerado, leva ao atrito com o técnico.
unsafe extern "C" fn relacoes_do_elenco() {
    let tirar_rejeitada: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x547c50usize);
    let briga: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x5888a0usize);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
    let favorecer: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547a10usize);
    let tirar_favorita: unsafe extern "C" fn(usize, usize) = core::mem::transmute(0x547c10usize);
    for _ in 0..10 {
        let clube = cps5_clube(cps5_sortear(cps5_d(0xae235c) as i32) as usize);
        let a = cps5_d(clube.wrapping_add(cps5_sortear(0x32) as usize * 4) + 0xd7);
        let b = cps5_d(clube.wrapping_add(cps5_sortear(0x32) as usize * 4) + 0xd7);
        if a == 0 || b == 0 || a == b {
            continue;
        }
        if cps5_sortear(cps5_sb(a + 0x5b)) != 0 {
            continue;
        }
        if cps5_nao_gosta(b, a) != 0 {
            tirar_rejeitada(a, b);
            let c = cps5_contrato(a);
            if c == 0 || cps5_clube_do_contrato(c) == 0 {
                continue;
            }
            let ca = cps5_clube_do_contrato(cps5_contrato(a));
            let cb = cps5_clube_do_contrato(cps5_contrato(b));
            if ca != cb {
                continue;
            }
            let k = cps5_vinculo(a, cps5_clube_do_contrato(cps5_contrato(a)));
            if k == 0 || cps5_d(k + 0x45) & 0x800 == 0 {
                continue;
            }
            briga(a, b);
        } else if cps5_sortear(3) == 0 {
            rejeitar(a, b, 1);
        } else if cps5_sortear(3) == 0 {
            favorecer(a, b, 1);
        } else {
            tirar_favorita(a, b);
        }
    }
    if (cps5_w(0xae2c90) as i32) % 0x32 == 0 {
        let atrito: unsafe extern "C" fn(usize) = core::mem::transmute(0x5d7190usize);
        let mut i = 0i32;
        while i < cps5_d(0xae235c) as i32 / 100 {
            atrito(cps5_clube(cps5_sortear(cps5_d(0xae235c) as i32) as usize));
            i += 1;
        }
        return;
    }
    let base = cps5_d(0xae23bc);
    let fim = base.wrapping_add((cps5_d(0xae2364).wrapping_sub(17)).wrapping_mul(0x6e));
    let mut p = base;
    while p < fim {
        cps5_pessoa_e_tecnico(p);
        p += 0x6e;
    }
}

/// Uma pessoa do laço de fora dos dias múltiplos de 50 (0x5d6f15..0x5d7115).
unsafe fn cps5_pessoa_e_tecnico(p: usize) {
    if cps5_d(p + 0x61) == 0 || cps5_d(p + 0x65) == 0 {
        return;
    }
    let c = cps5_contrato(p);
    if c == 0 {
        return;
    }
    let clube = cps5_clube_do_contrato(c);
    if clube == 0 {
        return;
    }
    let k = cps5_vinculo(cps5_d(clube + 0xff), clube);
    if k == 0 {
        return;
    }
    if cps5_sortear(cps5_sb(p + 0x5b) + 5) != 0 && cps5_sortear(cps5_sb(p + 0x5d) + 5) != 0 && cps5_sortear(cps5_sb(p + 0x5a) + 5) != 0 {
        return;
    }
    let mut e = cps5_sb(p + 0x5b) + (cps5_sb(p + 0x5a) + cps5_sb(p + 0x59)) + (cps5_sb(p + 0x5c) + cps5_sb(p + 0x18));
    crate::comparacao::ramo("cps_5 tecnico sorteado");
    if cps5_sortear(cps5_sb(p + 0x59)) == 0 { // cps5: o primeiro desconto
        e -= 3;
    }
    if cps5_sortear(cps5_sb(p + 0x5a)) == 0 {
        e -= 5;
    }
    if cps5_d(k + 0x45) != 0 || cps5_sortear(0x64) == 0 {
        if cps5_sortear(cps5_sb(p + 0x5b)) == 0 {
            e -= 0xf;
        }
        if cps5_sortear(cps5_sb(p + 0x5d)) == 0 {
            e -= 0xf;
        }
    }
    let rep = cps5_w(cps5_d(p + 0x61) + 0xb);
    if rep > 0x1194 && rep > cps5_w(clube + 0x80) {
        e -= 5;
    }
    if cps5_sortear(e) >= 1 {
        return;
    }
    if cps5_d(p + 0x65) == 0 {
        return;
    }
    cps5_atrito(p, clube, k);
}

// ponytail: sorteia e mexe nas preferências (0x547a10, 0x547af0, 0x547c10, 0x547c50), briga e queixa: só a original;
// a cópia leva os ramos (roda todo dia).
unsafe extern "C" fn comparar_relacoes_do_elenco() {
    orig::<unsafe extern "C" fn()>(1128)()
}

/// { 0x5d7190 } (cdecl, game.cpp): `game_check_club_dressing_room_unrest(clube)`: cada pessoa do elenco (+0xd7, 50
/// vagas) com vínculo com o clube (0x4cf5f0) e preferências (+0x65) passa pelo atrito com o técnico (o primeiro colega
/// de quem não gosta; a queixa só com dia % 14 + id = 0). Pelas relações do elenco (0x5d6d10) nos dias múltiplos de 50.
unsafe extern "C" fn atrito_com_o_tecnico(clube: usize) {
    for i in 0..0x32usize {
        let p = cps5_d(clube + 0xd7 + i * 4);
        if p == 0 {
            continue;
        }
        let k = cps5_vinculo(p, clube);
        if k == 0 {
            continue;
        }
        if cps5_d(p + 0x65) == 0 {
            continue;
        }
        cps5_atrito(p, clube, k);
    }
}

// ponytail: a queixa (0x587c60) e a marca no vínculo (0x867700) não rodam duas vezes: só a original; a cópia leva o
// ramo do atrito.
unsafe extern "C" fn comparar_atrito_com_o_tecnico(clube: usize) {
    orig::<unsafe extern "C" fn(usize)>(1129)(clube)
}

/// A pessoa conta para a liderança do clube: id válido (< [0xae2364]), contrato com o clube (+0x39) e ficha de jogador.
unsafe fn cps5_do_elenco(p: usize, clube: usize) -> bool {
    p != 0 && (cps5_d(p) as i32) < cps5_d(0xae2364) as i32 && cps5_d(p + 0x39) == clube && cps5_d(p + 0x61) != 0
}

/// { 0x5ce0b0 } (cdecl, game.cpp): `game_spread_squad_morale_by_leadership()`, todo dia pelo laço do dia do jogo. Cada
/// clube que não é seleção (0x53b900), com sorteio(7) = 0: soma a liderança (jogador +0x2f) e liderança × moral do
/// vínculo (+0x44) do elenco (+0xd7, 50 vagas); menos de 16 jogadores contam 10 cada um que falta. Com soma não nula,
/// cada jogador sorteia (2 × Σ liderança × moral) / (liderança dele + soma) mais sorteio(moral / 12), em byte com sinal:
/// negativo, sorteio(s) < sorteio(−5) baixa o moral (0x544710 com −1); positivo, sorteio(s) > sorteio(5) sobe (+1).
unsafe extern "C" fn moral_pela_lideranca() {
    let e_selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let moral: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(0x544710usize);
    let n = cps5_d(0xae235c);
    let mut clube = cps5_d(0xae23b4);
    let fim = clube.wrapping_add(n.wrapping_mul(0x245));
    while clube < fim {
        let vagas = clube + 0xd7;
        if e_selecao(clube) == 0 && cps5_sortear(7) == 0 {
            let (mut soma, mut lideranca, mut q) = (0i32, 0i32, 0i32);
            for i in 0..0x32usize {
                let p = cps5_d(vagas + i * 4);
                if !cps5_do_elenco(p, clube) {
                    continue;
                }
                let c = cps5_vinculo(p, clube);
                if c == 0 {
                    continue;
                }
                let l = cps5_sb(cps5_d(p + 0x61) + 0x2f);
                soma = soma.wrapping_add(cps5_sb(c + 0x44) * l);
                lideranca += l;
                q += 1;
            }
            if q < 0x10 {
                lideranca += (0x10 - q) * 10;
            }
            for i in 0..0x32usize {
                if lideranca == 0 {
                    break;
                }
                let p = cps5_d(vagas + i * 4);
                if !cps5_do_elenco(p, clube) {
                    continue;
                }
                let c = cps5_vinculo(p, clube);
                let m = if c == 0 { 0 } else { cps5_sb(c + 0x44) };
                let l = cps5_sb(cps5_d(p + 0x61) + 0x2f);
                crate::comparacao::ramo("cps_5 moral sorteado");
                let r1 = cps5_sortear(soma.wrapping_mul(2) / (l + lideranca));
                let r2 = cps5_sortear(m / 12); // cps5: a parte do moral
                let s = (r1 as u8).wrapping_add(r2 as u8) as i8;
                if s < 0 {
                    let e = cps5_sortear(s as i32);
                    if e < cps5_sortear(-5) {
                        moral(p, -1, 1);
                    }
                } else if s > 0 {
                    let e = cps5_sortear(s as i32);
                    if e > cps5_sortear(5) {
                        crate::comparacao::ramo("cps_5 moral sobe");
                        moral(p, 1, 1); // cps5: sobe o moral
                    }
                }
            }
        }
        clube = clube.wrapping_add(0x245);
    }
}

// ponytail: sorteia e muda o moral (0x544710, que também sorteia): só a original; a cópia leva os ramos.
unsafe extern "C" fn comparar_moral_pela_lideranca() {
    orig::<unsafe extern "C" fn()>(1130)()
}

// ---------- 255a leva: o dia do jogo: as faltas ao treino, as competições do dia e a confiança da diretoria no clube (game.cpp) ----------

unsafe fn cps6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps6_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cps6_sb(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cps6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cps6_pw(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cps6_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O contrato da pessoa (0x4cf5a0), guardado em [0xae1080] como a original faz a cada chamada.
unsafe fn cps6_contrato(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cps6_d(0xae1050), p);
    core::ptr::write_unaligned(0xae1080 as *mut usize, c);
    c
}

/// O clube do contrato (registros de 0x245 bytes em [0xae23b4], pelo id em +4), ou 0 sem contrato.
unsafe fn cps6_clube_do_contrato(c: usize) -> usize {
    if c == 0 {
        0
    } else {
        cps6_d(0xae23b4).wrapping_add(cps6_d(c + 4).wrapping_mul(0x245))
    }
}

/// Dias desde a data de +0x3e da pessoa até hoje (0x54cdb0 sobre a data do jogo 0xae2c90, a outra por valor).
unsafe fn cps6_dias_desde(p: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    f(0xae2c90, cps6_d(p + 0x3e) as u32, cps6_d(p + 0x42) as u32)
}

/// { 0x5d6aa0 } (cdecl, game.cpp): `game_check_players_missing_training()`, todo dia pelo laço do dia do jogo. A partir
/// de uma pessoa sorteada (sorteio(n − n/250 − 1)), n/250 pessoas seguidas: jogador (+0x61, função +0x3d = 11) com
/// contrato e clube, presente (0x418f60), com o registro de treino (0x8b39f0, +0x15) e sem lesão grave (0x62c400 ≠ 2).
/// (1) Com +0x5b < 8, jogador +0x2f < 8 e a soma (jogador +0x2f, +0x18, +0x5b, vínculo +0x44, −5 se +0x5b ou o moral
/// valem 1, −5 se não gosta do técnico, +2 se gosta) < 43: sorteio(+0x5b + 100, 50 ou 25 conforme os dias desde +0x3e:
/// < 30, < 90, < 180) zerado é falta. (2) Senão, com +0x5b < 10 e vínculo com o clube: as marcas do vínculo (+0x45:
/// 0x8000, 0x30000; com +0x5b < 5, 0x7800) dão sorteios(+0x5b) que, zerados, são falta. A falta: 0x41a360.
unsafe extern "C" fn faltas_ao_treino() {
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
    let treino: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8b39f0usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let vinculo: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let falta: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x41a360usize);
    let n = cps6_d(0xae2364) as i32;
    let q = n / 250;
    crate::comparacao::ramo("cps_6 faltas do dia");
    let r = cps6_sortear(n - q - 1); // cps6: a primeira pessoa
    let mut atual = cps6_d(0xae23bc).wrapping_add((r as usize).wrapping_mul(0x6e));
    for _ in 0..q {
        let p = atual;
        atual = atual.wrapping_add(0x6e);
        if cps6_d(p + 0x61) == 0 || cps6_b(p + 0x3d) != 0xb {
            continue;
        }
        let c = cps6_contrato(p);
        if c == 0 {
            continue;
        }
        let clube = cps6_clube_do_contrato(c);
        if clube == 0 {
            continue;
        }
        if ausente(cps6_d(0xad9c6c), p, 0) != 0 {
            continue;
        }
        let t = treino(cps6_d(0xde1f44), p);
        if t == 0 || cps6_b(t + 0x15) == 0 {
            continue;
        }
        if lesao(cps6_d(0xb63d28), p) == 2 {
            continue;
        }
        let jog = cps6_d(p + 0x61);
        let cl = cps6_b(p + 0x5b);
        let dl = cps6_b(jog + 0x44);
        let mut bl = cps6_b(jog + 0x2f).wrapping_add(cps6_b(p + 0x18)).wrapping_add(cl).wrapping_add(dl);
        if cl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if dl == 1 {
            bl = bl.wrapping_add(0xfb);
        }
        if nao_gosta(cps6_d(clube + 0xcf), p) != 0 {
            bl = bl.wrapping_add(0xfb);
        }
        if gosta(cps6_d(clube + 0xcf), p) != 0 {
            bl = bl.wrapping_add(2);
        }
        let mut e_falta = false;
        let a = cps6_b(p + 0x5b);
        if (a as i8) < 8 && (cps6_b(cps6_d(p + 0x61) + 0x2f) as i8) < 8 && (bl as i8) < 0x2b {
            let mut s = a;
            if cps6_dias_desde(p) < 0x1e {
                s = s.wrapping_add(0x64);
            } else if cps6_dias_desde(p) < 0x5a {
                s = s.wrapping_add(0x32);
            } else if cps6_dias_desde(p) < 0xb4 {
                s = s.wrapping_add(0x19);
            }
            crate::comparacao::ramo("cps_6 falta pela soma");
            if cps6_sortear(s as i8 as i32) == 0 { // cps6: o sorteio da soma
                e_falta = true;
            }
        }
        if !e_falta {
            if cps6_sb(p + 0x5b) >= 0xa {
                continue;
            }
            let clube = cps6_clube_do_contrato(cps6_contrato(p));
            let v = vinculo(cps6_d(0xae1050), p, clube);
            if v == 0 {
                continue;
            }
            if cps6_d(v + 0x45) & 0x8000 != 0 && cps6_sortear(cps6_sb(p + 0x5b)) == 0 {
                e_falta = true;
            } else if cps6_d(v + 0x45) & 0x30000 != 0 && cps6_sortear(cps6_sb(p + 0x5b)) == 0 {
                e_falta = true;
            } else {
                let a = cps6_sb(p + 0x5b);
                if a >= 5 || cps6_d(v + 0x45) & 0x7800 == 0 || cps6_sortear(a) != 0 {
                    continue;
                }
                e_falta = true;
            }
        }
        if e_falta {
            crate::comparacao::ramo("cps_6 falta ao treino");
            falta(cps6_d(0xad9c6c), p); // cps6: a falta
        }
    }
}

// ponytail: sorteia e a falta (0x41a360) multa, sorteia e mexe na forma: só a original; a cópia leva os ramos.
unsafe extern "C" fn comparar_faltas_ao_treino() {
    orig::<unsafe extern "C" fn()>(1131)()
}

/// { 0x5ce060 } (cdecl, game.cpp): `game_update_comps_for_date(marca)`: cada competição da lista ([0xadadfc], conta
/// [0xadadf8] relida a cada volta) recebe o dia (virtual +0x14 com a data do jogo 0xae2c90 e a marca); com a marca, o
/// passo seguinte do [0xae2a58] (0x5aaf00, que a original chama em salto de cauda). Pelo laço do dia do jogo (com 1).
unsafe extern "C" fn competicoes_do_dia(marca: u32) {
    let m = marca as u8 as i8;
    let mut i = 0i32;
    while i < cps6_d(0xadadf8) as i32 {
        let c = cps6_d(cps6_d(0xadadfc) + i as usize * 4);
        if c != 0 {
            let dia: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(cps6_d(cps6_d(c) + 0x14));
            dia(c, 0xae2c90, m as i32);
        }
        i += 1;
    }
    if m != 0 {
        crate::comparacao::ramo("cps_6 competicoes do dia");
        let seguinte: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x5aaf00usize);
        seguinte(cps6_d(0xae2a58));
    }
}

// ponytail: passa o dia de todas as competições (virtuais) e do [0xae2a58]: só a original; a cópia leva o ramo.
unsafe extern "C" fn comparar_competicoes_do_dia(marca: u32) {
    orig::<unsafe extern "C" fn(u32)>(1132)(marca)
}

/// A confiança da diretoria no clube: a palavra +6 da linha do clube (73 bytes por id em [this]).
unsafe fn cps6_confianca(this: usize, clube: usize) -> usize {
    cps6_d(this).wrapping_add(cps6_d(clube).wrapping_mul(0x49)) + 6
}

/// { 0x5cdde0 } (thiscall, ret 8, game.cpp): `manager_manager_adjust_board_confidence(clube, delta)`: soma `delta`
/// (palavra) à confiança da diretoria no clube (+6 da linha do MANAGER_MANAGER [0xb67a44]), presa em 1..10000. Clube
/// nulo: nada. Pelo laço do dia do jogo (com −5).
unsafe extern "thiscall" fn mudar_confianca_da_diretoria(this: usize, clube: usize, delta: u32) {
    if clube == 0 {
        return;
    }
    let c = cps6_confianca(this, clube);
    cps6_pw(c, cps6_w(c).wrapping_add(delta as u16 as i16));
    let v = cps6_w(c);
    if v < 1 {
        cps6_pw(c, 1);
    } else if v > 0x2710 {
        cps6_pw(c, 0x2710);
    }
}

/// Comparador: lado a lado pela confiança (2 bytes).
unsafe extern "thiscall" fn comparar_mudar_confianca_da_diretoria(this: usize, clube: usize, delta: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32)>(1133);
    if clube == 0 {
        return o(this, clube, delta);
    }
    let c = cps6_confianca(this, clube);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(c, 2)], || o(this, clube, delta), || mudar_confianca_da_diretoria(this, clube, delta));
    PLACAR.registrar(igual, || format!("mudar_confianca_da_diretoria(clube {:x}, {}) {:x?} x {:x?}", clube, delta as i32, mo, mr));
}

// ---------- 255a leva: os estádios: o dia dos estádios (o estádio nacional fechado e as reservas vencidas), reservar um estádio numa data e se ele está livre (stadium.cpp) ----------

unsafe fn cps7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps7_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cps7_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// A nação da Inglaterra ([0x9cf2e4]; registros de 0x122 bytes em [0xae23a8]).
unsafe fn cps7_inglaterra() -> usize {
    cps7_d(0xae23a8).wrapping_add(cps7_d(0x9cf2e4).wrapping_mul(0x122))
}

/// A seleção principal da nação (0x53b3d0).
unsafe fn cps7_selecao(nacao: usize) -> usize {
    let f: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    f(nacao)
}

/// O estádio de id `id` (registros de 0x4e bytes em [0xae23b0]).
unsafe fn cps7_estadio(id: usize) -> usize {
    cps7_d(0xae23b0).wrapping_add(id.wrapping_mul(0x4e))
}

unsafe fn cps7_ordenar(lista: usize, n: u32) {
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(lista, n, 0xc, 0x85e380);
}

/// { 0x85b820 } (thiscall, ret 4, stadium.cpp): `Stadium::daily_update(hoje)`, todo dia pelo laço do dia do jogo
/// ([0xdd7ef4]). No dia 0 do ano refaz a lista dos estádios (0x85dfc0). Antes de 2003, o estádio nacional da Inglaterra
/// (nação +0x80) fecha: o id vai para [0xa835a4], a nação e a seleção principal (+0x69) ficam sem estádio. Em
/// 1/6/2003 (0x549f70) ele volta com 90000 lugares (+0x3c, +0x40). A cada 30 dias, as reservas (lista em +0, conta em
/// +4, 12 bytes: id e data) antes de hoje saem: a original conta as k primeiras (ano sem sinal, dia com sinal), copia
/// por cima delas, para a frente, as k que começam em n − k − 1 (não em n − k) e reordena as n − k (0x945333 com
/// 0x85e380).
unsafe extern "thiscall" fn estadios_do_dia(this: usize, hoje: usize) {
    if cps7_w(hoje) == 0 {
        let refazer: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x85dfc0usize);
        refazer(this);
    }
    if cps7_w(hoje + 2) < 0x7d3 {
        let nacao = cps7_inglaterra();
        let e = cps7_d(nacao + 0x80);
        if e != 0 {
            crate::comparacao::ramo("cps_7 fecha o estadio nacional");
            cps7_pd(0xa835a4, cps7_d(e) as u32); // cps7: guarda o id do estádio fechado
            cps7_pd(nacao + 0x80, 0);
            cps7_pd(cps7_selecao(cps7_inglaterra()) + 0x69, 0); // cps7: a seleção sem casa
        }
    }
    let nova: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut t = [0u8; 8];
    let r = nova(t.as_mut_ptr() as usize, 1, 6, 0x7d3, 0xffff_ffff);
    let reabre = cps7_d(r) as u32;
    if (reabre >> 16) as u16 == cps7_w(hoje + 2) && reabre as u16 == cps7_w(hoje) && cps7_d(0xa835a4) as i32 != -1 {
        crate::comparacao::ramo("cps_7 reabre o estadio nacional");
        cps7_pd(cps7_inglaterra() + 0x80, cps7_estadio(cps7_d(0xa835a4)) as u32);
        let nacao = cps7_inglaterra();
        cps7_pd(cps7_selecao(nacao) + 0x69, cps7_d(nacao + 0x80) as u32);
        cps7_pd(cps7_estadio(cps7_d(0xa835a4)) + 0x3c, 0x15f90);
        cps7_pd(cps7_estadio(cps7_d(0xa835a4)) + 0x40, 0x15f90);
        cps7_pd(0xa835a4, 0xffff_ffff);
    }
    if (cps7_w(hoje) as i16 as i32) % 0x1e != 0 {
        return;
    }
    let lista = cps7_d(this);
    if lista == 0 || (cps7_d(this + 4) as i32) <= 0 {
        return;
    }
    let (dia, ano) = (cps7_w(hoje) as i16, cps7_w(hoje + 2));
    // k em 32 bits, o índice em 16 com sinal (como a original: [esp+0x10] e si)
    let mut k = 0u32;
    let mut i = 0i16;
    loop {
        let e = lista.wrapping_add((i as i32 as usize).wrapping_mul(12));
        let ano_e = cps7_w(e + 6);
        let antes = ano > ano_e || (ano == ano_e && dia > cps7_w(e + 4) as i16);
        if !antes {
            break;
        }
        k = k.wrapping_add(1);
        i = i.wrapping_add(1);
        if i as i32 >= cps7_d(this + 4) as i32 {
            break;
        }
    }
    if k as u16 == 0 {
        return;
    }
    crate::comparacao::ramo("cps_7 reservas vencidas");
    let k = k as i16 as i32;
    let n = cps7_d(this + 4) as i32;
    if k < n {
        let de = lista.wrapping_add(((n - k - 1) as usize).wrapping_mul(12));
        // rep movsd para a frente: as regiões podem se sobrepor
        for j in 0..(k as usize * 3) {
            cps7_pd(lista + j * 4, cps7_d(de + j * 4) as u32);
        }
    }
    cps7_pd(this + 4, (n - k) as u32);
    cps7_ordenar(cps7_d(this), (n - k) as u32);
}

/// Comparador: no dia 0 do ano (refaz a lista dos estádios, aloca) só a original. Nos outros, lado a lado pelo cabeçalho
/// (0x1c bytes), pelas reservas, por [0xa835a4], pelo estádio nacional da Inglaterra, pela casa da seleção e pelos
/// lugares do estádio fechado.
unsafe extern "thiscall" fn comparar_estadios_do_dia(this: usize, hoje: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1134);
    if cps7_w(hoje) == 0 {
        return o(this, hoje);
    }
    let mut regioes = [(0usize, 0usize); 6];
    let mut m = 0;
    regioes[m] = (this, 0x1c);
    m += 1;
    let n = cps7_d(this + 4) as i32;
    if cps7_d(this) != 0 && n > 0 {
        regioes[m] = (cps7_d(this), n as usize * 12);
        m += 1;
    }
    regioes[m] = (0xa835a4, 4);
    m += 1;
    let nacao = cps7_inglaterra();
    regioes[m] = (nacao + 0x80, 4);
    m += 1;
    let s = cps7_selecao(nacao);
    if s != 0 {
        regioes[m] = (s + 0x69, 4);
        m += 1;
    }
    if cps7_d(0xa835a4) as i32 != -1 {
        regioes[m] = (cps7_estadio(cps7_d(0xa835a4)) + 0x3c, 8);
        m += 1;
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes[..m], || o(this, hoje), || estadios_do_dia(this, hoje));
    PLACAR.registrar(igual, || format!("estadios_do_dia(dia {}, ano {}) {:x?} x {:x?}", cps7_w(hoje) as i16, cps7_w(hoje + 2), mo, mr));
}

/// { 0x85de20 } (thiscall, ret 8, stadium.cpp): `Stadium::add_booking(estadio, data)`: acrescenta a reserva (id do
/// estádio, cópia da data por 0x417c10) à lista (+0; conta +4, capacidade +8), que nasce com 30 (0x944e46, 0x168
/// bytes) e cresce de 30 em 30 (0x945667), e reordena (0x945333 com 0x85e380). Estádio ou data nulos, ou sem memória: a
/// caixa "Error" (linhas 0x810, 0x81b, 0x82b). Pela escolha do estádio (0x85c260, 0x85c800).
unsafe extern "thiscall" fn reservar_estadio(this: usize, estadio: usize, data: usize) {
    if estadio == 0 || data == 0 {
        return aviso_em(0xa835b0, 0x810);
    }
    if cps7_d(this) == 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
        let l = alocar(0x168);
        cps7_pd(this, l as u32);
        if l == 0 {
            return aviso_em(0xa835b0, 0x81b);
        }
        cps7_pd(this + 8, 0x1e);
    } else if cps7_d(this + 4) == cps7_d(this + 8) {
        let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let l = realocar(cps7_d(this), (cps7_d(this + 8) as u32).wrapping_mul(3).wrapping_add(0x5a).wrapping_mul(4));
        cps7_pd(this, l as u32);
        if l == 0 {
            return aviso_em(0xa835b0, 0x82b);
        }
        cps7_pd(this + 8, (cps7_d(this + 8) as u32).wrapping_add(0x1e));
    }
    let e = cps7_d(this).wrapping_add(cps7_d(this + 4).wrapping_mul(12));
    cps7_pd(e, cps7_d(estadio) as u32);
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut t = [0u8; 8];
    copia(e + 4, t.as_mut_ptr() as usize, data);
    let n = (cps7_d(this + 4) as u32).wrapping_add(1);
    cps7_pd(this + 4, n);
    cps7_ordenar(cps7_d(this), n);
}

/// Comparador: com lista e vaga (não aloca), lado a lado pelo cabeçalho (0xc bytes) e pelas reservas com a nova; senão
/// (aloca, realoca ou a caixa) só a original.
unsafe extern "thiscall" fn comparar_reservar_estadio(this: usize, estadio: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1135);
    let n = cps7_d(this + 4);
    if estadio == 0 || data == 0 || cps7_d(this) == 0 || n == cps7_d(this + 8) {
        return o(this, estadio, data);
    }
    let regioes = [(this, 0xc), (cps7_d(this), (n + 1) * 12)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(this, estadio, data), || reservar_estadio(this, estadio, data));
    PLACAR.registrar(igual, || format!("reservar_estadio(estadio {:x}, reservas {}) {:x?} x {:x?}", estadio, n, mo, mr));
}

/// { 0x85ddb0 } (thiscall, ret 8, stadium.cpp): `Stadium::is_stadium_free_on_date(estadio, data)`: 1 sem lista de
/// reservas; senão 1 se a busca binária (0x946464 com 0x85e380) não acha a reserva (id do estádio, data copiada por
/// 0x417c10). Pela escolha do estádio neutro (0x85c800).
unsafe extern "thiscall" fn estadio_livre_na_data(this: usize, estadio: usize, data: usize) -> u32 {
    if cps7_d(this) == 0 {
        return 1;
    }
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let buscar: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let mut chave = [0u8; 12];
    let mut t = [0u8; 8];
    core::ptr::write_unaligned(chave.as_mut_ptr() as *mut u32, cps7_d(estadio) as u32);
    copia(chave.as_mut_ptr() as usize + 4, t.as_mut_ptr() as usize, data);
    let r = buscar(chave.as_ptr() as usize, cps7_d(this), cps7_d(this + 4) as u32, 0xc, 0x85e380);
    crate::comparacao::ramo("cps_7 reserva conferida");
    (r == 0) as u32 // cps7: livre sem a reserva
}

/// Comparador: a original e a cópia (as duas só leem), pelo valor.
unsafe extern "thiscall" fn comparar_estadio_livre_na_data(this: usize, estadio: usize, data: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1136)(this, estadio, data);
    let n = estadio_livre_na_data(this, estadio, data);
    PLACAR.registrar(r == n, || format!("estadio_livre_na_data(estadio {:x}) original={} rust={}", estadio, r, n));
    r
}

// ---------- 255a leva: o jogo com jogador observado, a lista do olheiro do humano e o registro da primeira convocação (scout_manager.cpp, staff_records.cpp) ----------

unsafe fn cps8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// { 0x7f9720 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::fixture_has_scouted_player(partida)` (pela
/// partida_sem_humano 0x6b66a0): competição (+0) negativa, 0. Um clube da lista vigiada (LLIST estática 0xb74438,
/// cursor reposto; +0 do item) igual a +0xc ou +0x10 da partida, 1. Depois os pedidos de observação ([this+0x5c],
/// 0x11 bytes, a pessoa em +4, conta [this+0x64], relidos a cada passo): a pessoa cujo clube (+0x39) ou seleção (+0x24)
/// é um dos dois da partida (+0x1c primeiro, depois +0x20), que não está cedida (0x8c7560, fora de seleção 0x53b900),
/// nem lesionada (0x62c400 = 2), nem suspensa (0x55d270), nem impedida (0x7757d0), e que ainda não jogou (0x8cf5e0
/// = 0), dá 1. Senão 0.
unsafe extern "thiscall" fn jogo_com_observado(this: usize, partida: usize) -> u32 {
    let (c1, c2) = (cps8_d(partida + 0x1c), cps8_d(partida + 0x20));
    if cps8_i(partida) < 0 {
        crate::comparacao::ramo("cps_8 competicao negativa");
        return 0; // cps8: competição negativa
    }
    core::ptr::write_unaligned(0xb74442 as *mut u32, cps8_d(0xb7443a) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut e = proximo(0xb74438);
    while e != 0 {
        let v = cps8_d(e);
        if cps8_d(partida + 0xc) == v || cps8_d(partida + 0x10) == v {
            return 1;
        }
        e = proximo(0xb74438);
    }
    if cps8_d(this + 0x5c) == 0 || cps8_i(this + 0x64) <= 0 {
        crate::comparacao::ramo("cps_8 sem pedidos");
        return 0; // cps8: sem pedidos de observação
    }
    let selecao: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x53b900usize);
    let cedido: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let impedido: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x7757d0usize);
    let ja_jogou: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x8cf5e0usize);
    let mut k = 0i32;
    while k < cps8_i(this + 0x64) {
        let id = cps8_i(cps8_d(this + 0x5c).wrapping_add(k as usize * 0x11) + 4);
        k += 1;
        if id <= -1 {
            continue;
        }
        let p = cps8_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
        let (clube, sel) = (cps8_d(p + 0x39), cps8_d(p + 0x24));
        let x = if c1 == clube || c1 == sel {
            c1
        } else if c2 == clube || c2 == sel {
            c2
        } else {
            continue;
        };
        if selecao(x) == 0 && cedido(cps8_d(0xde1f54), p, x, 0) != 0 {
            continue;
        }
        if lesao(cps8_d(0xb63d28), p) == 2 {
            continue;
        }
        if suspenso(cps8_d(0xae28f0), p, cps8_d(partida + 0x14), 1, 0) != 0 {
            continue;
        }
        if impedido(cps8_d(0xb74318), p, 0xae2c90, x, 0, 0) != 0 {
            continue;
        }
        if ja_jogou(cps8_d(0xde1f54), p, partida) == 0 {
            return 1;
        }
    }
    0
}

unsafe extern "thiscall" fn comparar_jogo_com_observado(this: usize, partida: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1137)(this, partida);
    let n = jogo_com_observado(this, partida);
    PLACAR.registrar(r == n, || format!("jogo_com_observado(partida {:x}) original={} rust={}", partida, r, n));
    r
}

/// O gerente do humano da vez ([0xb74c26 + 0x300 × (i16)[0xb77c7a]]).
unsafe fn cps8_gerente_da_vez() -> usize {
    let k = core::ptr::read_unaligned(0xb77c7a as *const i16) as isize;
    cps8_d((0xb74c26isize + 0x300 * k) as usize)
}

/// { 0x7f7490 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::is_player_shortlisted(pessoa, gerente)` (pelos
/// clubes interessados 0x842180, pelo estatuto 0x849d60, pela 0x8cde70 e por telas): pessoa nula, a caixa "Error"
/// (linha 0x231) e 0; sem gerente, o do humano da vez (sem ele, a caixa da linha 0x23b e 0). A vaga do humano (byte
/// +0 do gerente − byte de [0xae2364] + 16, com sinal, 0..15): com a edição aberta ([this + vaga × 4 + 0xc]), a
/// resposta dela (0x804540); senão a primeira das 10 buscas (0x7f6cd0) de tipo 1 (+0xa) responde (0x7af600). Sem ela, 0.
unsafe extern "thiscall" fn na_lista_do_olheiro(this: usize, pessoa: usize, gerente: usize) -> u32 {
    if pessoa == 0 {
        aviso_em(0xa7b094, 0x231);
        return 0;
    }
    let m = if gerente != 0 { gerente } else { cps8_gerente_da_vez() };
    if m == 0 {
        aviso_em(0xa7b094, 0x23b);
        return 0;
    }
    let vaga = (*(m as *const u8)).wrapping_sub(*(0xae2364 as *const u8)).wrapping_add(0x10) as i8;
    if !(0..=15).contains(&vaga) {
        return 0;
    }
    let s = cps8_d(this + vaga as usize * 4 + 0xc);
    if s != 0 {
        let da_edicao: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x804540usize);
        return da_edicao(s, pessoa);
    }
    // ponytail: a original empurra um dword local com o índice só no byte baixo; a 0x7f6cd0 lê só esse byte
    let busca: unsafe extern "thiscall" fn(usize, u32, usize) -> usize = core::mem::transmute(0x7f6cd0usize);
    let tem: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7af600usize);
    for i in 0..10u32 {
        let l = busca(this, i, m);
        if l != 0 && *((l + 0xa) as *const u8) == 1 {
            let r = tem(l, pessoa);
            crate::comparacao::ramo("cps_8 lista do olheiro");
            return r; // cps8: a resposta da lista do olheiro
        }
    }
    0
}

unsafe extern "thiscall" fn comparar_na_lista_do_olheiro(this: usize, pessoa: usize, gerente: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(1138);
    if pessoa == 0 || (gerente == 0 && cps8_gerente_da_vez() == 0) {
        return o(this, pessoa, gerente);
    }
    let r = o(this, pessoa, gerente);
    let n = na_lista_do_olheiro(this, pessoa, gerente);
    PLACAR.registrar(r == n, || format!("na_lista_do_olheiro(pessoa {:x}, gerente {:x}) original={} rust={}", pessoa, gerente, r, n));
    r
}

/// { 0x86cc50 } (thiscall, ret 8, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_first_cap_record(pessoa, adversário)` (pelo resultado internacional 0x76b600): o histórico da
/// pessoa (tipo 6: o clube contratante +0x39 ou −1, o id do adversário na partida e a idade +0x18 com sinal), no arquivo (0x58e340).
/// O clube é lido antes do teste da pessoa, como na original; pessoa nula: a caixa "Error" (linha 0x1010).
unsafe extern "thiscall" fn registrar_a_primeira_convocacao(this: usize, pessoa: usize, adversario: usize) {
    crate::comparacao::ramo("cps_8 primeira convocacao");
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 6;
    cpq4_por(&mut e, 8, cpq4_clube_de(pessoa));
    cpq4_por(&mut e, 0xd, cps8_i(adversario));
    cpq4_por(&mut e, 0x11, *((pessoa + 0x18) as *const i8) as i32);
    if pessoa != 0 {
        cpq4_historico(true, this, cps8_d(pessoa), &e);
    } else {
        aviso_em(0xa83b78, 0x1010);
    }
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia não grava mais nada: só a original; a
// cópia leva o ramo.
unsafe extern "thiscall" fn comparar_registrar_a_primeira_convocacao(this: usize, pessoa: usize, adversario: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1139)(this, pessoa, adversario)
}

// ---------- 255a leva: os gols na carreira do jogador, o registro do empréstimo e a ordem da fila das voltas aos treinos (player_stats.cpp, staff_records.cpp, injury.cpp) ----------

unsafe fn cps9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// { 0x7c3d20 } (stdcall, ret 4, player_stats.cpp; a original recebe o PlayerStats no ecx e não o lê):
/// `player_stats_get_career_goals(pessoa)` (pela MATCH_PLAYER::setup 0x6eaea0, que grava em +0x43 do jogador da
/// partida): pessoa nula, a caixa "Error" (linha 0xb4b) e 0; id a partir de [0xae2364], 0; sem a ficha de jogador
/// (+0x61), a caixa (linha 0xb5d) e 0. Senão a tabela da carreira (0x7c3e40 do [0xb74350]: entradas de 0x2b bytes,
/// seis por temporada e clube) e a soma, em 16 bits, do byte +8 da sexta entrada de cada grupo (+0xdf); a tabela é
/// liberada (0x9452ca, também quando nula). Quente (cada jogador de cada partida): sem contador.
unsafe extern "stdcall" fn gols_na_carreira(pessoa: usize) -> u16 {
    if pessoa == 0 {
        aviso_em(0xa78310, 0xb4b);
        return 0;
    }
    if cps9_i(pessoa) >= cps9_i(0xae2364) {
        return 0;
    }
    if cps9_d(pessoa + 0x61) == 0 {
        aviso_em(0xa78310, 0xb5d);
        return 0;
    }
    let montar: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(0x7c3e40usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut n = 0i32;
    let t = montar(cps9_d(0xb74350), pessoa, &mut n);
    let mut s = 0u16;
    if t != 0 {
        let mut i = 0i32; // cps9: do primeiro grupo
        while i < n {
            s = s.wrapping_add(*((t + 0xdf + (i / 6) as usize * 0x102) as *const u8) as u16);
            i += 6;
        }
    }
    liberar(t);
    s // cps9: os gols na carreira
}

/// Comparador: a devolução (a tabela é montada e liberada pelas duas), sem os casos de erro (só a original).
unsafe extern "stdcall" fn comparar_gols_na_carreira(pessoa: usize) -> u16 {
    let o = orig::<unsafe extern "stdcall" fn(usize) -> u16>(1140);
    if pessoa == 0 || (cps9_i(pessoa) < cps9_i(0xae2364) && cps9_d(pessoa + 0x61) == 0) {
        return o(pessoa);
    }
    let r = o(pessoa);
    let n = gols_na_carreira(pessoa);
    PLACAR.registrar(r == n, || format!("gols_na_carreira(pessoa {:x}) original={} rust={}", pessoa, r, n));
    r
}

/// { 0x86da20 } (thiscall, ret 0xc, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_loan_records(pessoa, de, clube)` (pela 0x8c2180, a transferência aceita): com o clube e o técnico
/// dele (+0xcf), o histórico do técnico (tipo 0x69: o clube, a pessoa e os três ids +0x33 de +4, +8 e +0xc dela, e o id
/// de `de`); depois, na mesma entrada, o da pessoa (tipo 0x6a: o clube e o id de `de`), no arquivo (0x58e340). O clube
/// é lido antes do teste da pessoa, como na original; pessoa nula: a caixa "Error" (linha 0x1010).
unsafe extern "thiscall" fn registrar_o_emprestimo(this: usize, pessoa: usize, de: usize, clube: usize) {
    crate::comparacao::ramo("cps_9 emprestimo");
    let mut e = cpq4_entrada();
    if clube != 0 && cps9_d(clube + 0xcf) != 0 {
        let tecnico = cps9_d(clube + 0xcf);
        cpq4_datar(e.as_mut_ptr());
        e[0xc] = 0x69;
        cpq4_por(&mut e, 8, cps9_i(clube));
        cpq4_quem(&mut e, 0xd, pessoa);
        cpq4_por(&mut e, 0x1d, cps9_i(de));
        cpq4_historico(true, this, cps9_d(tecnico), &e);
    }
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x6a;
    cpq4_por(&mut e, 8, cps9_i(clube));
    cpq4_por(&mut e, 0xd, cps9_i(de));
    if pessoa != 0 {
        cpq4_historico(true, this, cps9_d(pessoa), &e);
    } else {
        aviso_em(0xa83b78, 0x1010);
    }
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia não grava mais nada: só a original; a
// cópia leva o ramo.
unsafe extern "thiscall" fn comparar_registrar_o_emprestimo(this: usize, pessoa: usize, de: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1141)(this, pessoa, de, clube)
}

/// { 0x632040 } (cdecl, injury.cpp): `injury_compare_entry_dates(chave, item)`, a ordem do bsearch (0x946464) da fila
/// das voltas aos treinos (entradas de 0x16 bytes) em Injury::post_due_recovery_news (0x62e010, todo dia): a data em
/// +8 da chave antes da do item (0x456770), −1; depois (0x456740), 1; igual, 0.
unsafe extern "C" fn ordem_da_volta_aos_treinos(chave: usize, item: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    if antes(chave + 8, cps9_d(item + 8) as u32, cps9_d(item + 0xc) as u32) != 0 {
        return -1;
    }
    if depois(chave + 8, cps9_d(item + 8) as u32, cps9_d(item + 0xc) as u32) != 0 {
        return 1;
    }
    crate::comparacao::ramo("cps_9 data igual");
    0 // cps9: a volta de hoje achada
}

unsafe extern "C" fn comparar_ordem_da_volta_aos_treinos(chave: usize, item: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1142)(chave, item);
    let n = ordem_da_volta_aos_treinos(chave, item);
    PLACAR.registrar(r == n, || format!("ordem_da_volta_aos_treinos(chave {:x}, item {:x}) original={} rust={}", chave, item, r, n));
    r
}

// ---------- 255a leva: os gols do jogador pelo clube, o registro da seleção do ano e a ordem dos clubes pelos pontos do técnico (player_stats.cpp, staff_records.cpp) ----------

unsafe fn cps10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// { 0x7c3bb0 } (stdcall, ret 8, player_stats.cpp; a original recebe o PlayerStats no ecx e não o lê):
/// `player_stats_get_goals_for_club(pessoa, clube)` (pela MATCH_PLAYER::setup 0x6eaea0, que grava em +0x3d do jogador
/// da partida): pessoa nula, a caixa "Error" (linha 0xb04) e 0; clube nulo, a da linha 0xb0c e 0; id a partir de
/// [0xae2364], 0; sem a ficha de jogador (+0x61), a da linha 0xb1e e 0. Senão a tabela da carreira (0x7c3e40 do
/// [0xb74350]: entradas de 0x2b bytes, seis por temporada e clube) e a soma, em 16 bits, do byte +0xdf dos grupos cujo
/// clube (+0xd9, com sinal, não negativo) é o id do `clube`; a tabela é liberada (0x9452ca, também quando nula).
/// Quente (cada jogador de cada partida): sem contador.
unsafe extern "stdcall" fn gols_pelo_clube(pessoa: usize, clube: usize) -> u16 {
    if pessoa == 0 {
        aviso_em(0xa78310, 0xb04);
        return 0;
    }
    if clube == 0 {
        aviso_em(0xa78310, 0xb0c);
        return 0;
    }
    if cps10_i(pessoa) >= cps10_i(0xae2364) {
        return 0;
    }
    if cps10_d(pessoa + 0x61) == 0 {
        aviso_em(0xa78310, 0xb1e);
        return 0;
    }
    let montar: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(0x7c3e40usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut n = 0i32;
    let t = montar(cps10_d(0xb74350), pessoa, &mut n);
    let mut s = 0u16;
    if t != 0 {
        let mut i = 0i32;
        while i < n {
            let g = t + 0xd9 + (i / 6) as usize * 0x102;
            let v = cps10_i(g);
            if v >= 0 && v == cps10_i(clube) { // cps10: só os grupos do clube
                s = s.wrapping_add(*((g + 6) as *const u8) as u16);
            }
            i += 6;
        }
    }
    liberar(t);
    s // cps10: os gols pelo clube
}

/// Comparador: a devolução (a tabela é montada e liberada pelas duas), sem os casos de erro (só a original).
unsafe extern "stdcall" fn comparar_gols_pelo_clube(pessoa: usize, clube: usize) -> u16 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize) -> u16>(1143);
    if pessoa == 0 || clube == 0 || (cps10_i(pessoa) < cps10_i(0xae2364) && cps10_d(pessoa + 0x61) == 0) {
        return o(pessoa, clube);
    }
    let r = o(pessoa, clube);
    let n = gols_pelo_clube(pessoa, clube);
    PLACAR.registrar(r == n, || format!("gols_pelo_clube(pessoa {:x}, clube {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}

/// { 0x86c730 } (thiscall, ret 0x14, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_team_of_year_record(pessoa, clube, prêmio, a, b)` (pela TEAM_AWARD::record_winners 0x8affb0):
/// pessoa ou prêmio nulos, a caixa "Error" (linha 0x7ee). Senão o histórico da pessoa (tipo 0x32: o id do clube ou −1,
/// o id do prêmio, `a` em 16 bits sem sinal e `b` como byte com sinal), no arquivo (0x58e340).
unsafe extern "thiscall" fn registrar_a_selecao_do_ano(this: usize, pessoa: usize, clube: usize, premio: usize, a: u32, b: u32) {
    crate::comparacao::ramo("cps_10 selecao do ano");
    if pessoa == 0 || premio == 0 {
        aviso_em(0xa83b78, 0x7ee);
        return;
    }
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x32;
    cpq4_por(&mut e, 8, cpq4_id_ou_menos_1(clube));
    cpq4_por(&mut e, 0xd, cps10_i(premio));
    cpq4_por(&mut e, 0x11, (a & 0xffff) as i32);
    cpq4_por(&mut e, 0x15, b as u8 as i8 as i32);
    cpq4_historico(true, this, cps10_d(pessoa), &e);
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia não grava mais nada: só a original; a
// cópia leva o ramo.
unsafe extern "thiscall" fn comparar_registrar_a_selecao_do_ano(this: usize, pessoa: usize, clube: usize, premio: usize, a: u32, b: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(1144)(this, pessoa, clube, premio, a, b)
}

/// O registro de pontos do técnico do clube (+0xcf; 0x86f5b0 do [0xdd7f00], criando), ou 0 sem técnico.
unsafe fn cps10_registro(clube: usize) -> usize {
    let tecnico = cps10_d(clube + 0xcf);
    if tecnico == 0 {
        return 0;
    }
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    registro(cps10_d(0xdd7f00), tecnico, 1)
}

/// { 0x870220 } (cdecl, staff_records.cpp): `staff_records_compare_clubs_by_manager_points(a, b)`, a ordem do qsort dos
/// candidatos a técnico do ano (0x86fab0): os registros dos técnicos dos dois clubes (cps10_registro); sem os dois, 0;
/// só sem o de `a`, 1; só sem o de `b`, −1; com os dois, os pontos da temporada na liga (+0xba) de `b` menos os de
/// `a` (mais pontos primeiro).
unsafe extern "C" fn ordem_pelos_pontos_do_tecnico(a: usize, b: usize) -> i32 {
    let (ca, cb) = (cps10_d(a), cps10_d(b));
    let ra = cps10_registro(ca);
    let rb = cps10_registro(cb);
    if ra == 0 {
        return if rb != 0 { 1 } else { 0 };
    }
    if rb == 0 {
        return -1;
    }
    crate::comparacao::ramo("cps_10 pontos dos dois");
    cps10_i(rb + 0xba).wrapping_sub(cps10_i(ra + 0xba)) // cps10: mais pontos primeiro
}

// ponytail: a 0x86f5b0 cria o registro que falta; a original roda antes e a cópia só acha os que ela criou.
unsafe extern "C" fn comparar_ordem_pelos_pontos_do_tecnico(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1145)(a, b);
    let n = ordem_pelos_pontos_do_tecnico(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pelos_pontos_do_tecnico(clubes {:x}, {:x}) original={} rust={}", cps10_d(a), cps10_d(b), r, n));
    r
}

// ---------- 255a leva: o calendário guardado: achar a cópia guardada de um jogo, guardar um jogo e tirar um jogo (fix_man.cpp) ----------

unsafe fn cps11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps11_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// O bloco do ano do jogo `f` (+0x28, palavra) na corrente do calendário a partir de `fm` (o laço comum das 0x5ac7a0,
/// 0x5ab230 e 0x5abc30): o bloco seguinte (+0x112c) de outro ano é levado ao ano pedido (0x5aa2d0 set_year); sem
/// seguinte, um bloco novo (0x944cf1, 0x1184 bytes; 0x5a9cf0 com o ano, o bloco de antes e 0) entra na corrente. 0 se
/// faltou memória (a corrente fica com 0 em +0x112c, como na original).
unsafe fn cps11_bloco_do_ano(fm: usize, f: usize) -> usize {
    let mudar_ano: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5aa2d0usize);
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, usize, u32) -> usize = core::mem::transmute(0x5a9cf0usize);
    let mut b = fm;
    loop {
        let ano = cps11_w(f + 0x28);
        if ano == cps11_w(b + 0x1130) {
            return b;
        }
        let prox = cps11_d(b + 0x112c);
        if prox != 0 {
            if cps11_w(prox + 0x1130) != ano {
                // ponytail: a original empurra o eax com o ano na palavra baixa e sobra de registrador em cima; a
                // 0x5aa2d0 compara a palavra (o caminho só roda na virada do ano do calendário)
                mudar_ano(prox, ano as u32);
            }
        } else {
            let p = novo(0x1184);
            // o ecx da original era o +0x112c nulo: o ano vai limpo
            let r = if p != 0 { construir(p, ano as u32, b, 0) } else { 0 };
            core::ptr::write_unaligned((b + 0x112c) as *mut u32, r as u32);
            if r == 0 {
                return 0;
            }
        }
        b = cps11_d(b + 0x112c);
    }
}

/// A lista do dia, fatia (+0x3f) e nação-chave do jogo `f` no bloco `b`: [[b + (dia +0x2a × 3 + fatia) × 4] + nação × 4],
/// a nação pela 0x48dd70 (competição +0x14, o jogo), com sinal.
unsafe fn cps11_lista_no_bloco(b: usize, f: usize) -> usize {
    let nacao: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x48dd70usize);
    let n = nacao(cps11_d(f + 0x14), f) as u8 as i8 as isize;
    let k = cps11_w(f + 0x2a) as usize * 3 + *((f + 0x3f) as *const u8) as usize;
    cps11_d((cps11_d(b + k * 4) as isize + n * 4) as usize)
}

/// O próximo item da lista (0x68d1b0: o cursor +0xa anda).
unsafe fn cps11_proximo(l: usize) -> usize {
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    prox(l)
}

/// { 0x5ac7a0 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::find_stored_fixture(jogo)`: a cópia guardada no
/// calendário cujos 0x4f bytes são iguais aos do jogo, na lista do dia, fatia e nação-chave dele (o bloco do ano pela
/// corrente; o cursor da lista volta ao começo e anda). Jogo nulo ou nenhum igual: 0; sem memória para o bloco do ano:
/// a caixa (linha 0x7cf) e 0.
unsafe extern "thiscall" fn jogo_guardado_igual(this: usize, f: usize) -> usize {
    if f == 0 {
        return 0;
    }
    let b = cps11_bloco_do_ano(this, f);
    if b == 0 {
        aviso_em(0x9cc208, 0x7cf);
        return 0;
    }
    let l = cps11_lista_no_bloco(b, f);
    core::ptr::write_unaligned((l + 0xa) as *mut u32, cps11_d(l + 2) as u32);
    let alvo = core::slice::from_raw_parts(f as *const u8, 0x4f);
    let mut item = cps11_proximo(l);
    while item != 0 {
        if core::slice::from_raw_parts(item as *const u8, 0x4f) == alvo {
            crate::comparacao::ramo("cps_11 achou o jogo guardado");
            return item; // cps11: a cópia guardada
        }
        item = cps11_proximo(l);
    }
    0
}

/// { 0x5ab230 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::insert_fixture(jogo)`: acrescenta uma cópia dos 0x4f
/// bytes do jogo (0x68d0d0) à lista do dia, fatia e nação-chave dele no bloco do ano (pela corrente). Sem memória para o
/// bloco: a caixa (linha 0x3ff); a cópia que falha: a caixa (linha 0x3eb).
unsafe extern "thiscall" fn guardar_jogo_no_calendario(this: usize, f: usize) {
    let b = cps11_bloco_do_ano(this, f);
    if b == 0 {
        return aviso_em(0x9cc208, 0x3ff);
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    let l = cps11_lista_no_bloco(b, f);
    if copiar(l, f, 0x4f) == 0 { // cps11: guardar
        return aviso_em(0x9cc208, 0x3eb);
    }
    crate::comparacao::ramo("cps_11 jogo guardado");
}

/// { 0x5abc30 } (thiscall, ret 4, fix_man.cpp): `FIXTURE_MANAGER::remove_fixture(jogo)`: tira da lista do dia, fatia e
/// nação-chave do jogo (no bloco do ano, pela corrente) o primeiro item com os mesmos bytes (0x68d1f0). Devolve 1 se
/// tirou, 0 se não; sem memória para o bloco: a caixa (linha 0x5ef) e 0.
unsafe extern "thiscall" fn tirar_jogo_do_calendario(this: usize, f: usize) -> u32 {
    let b = cps11_bloco_do_ano(this, f);
    if b == 0 {
        aviso_em(0x9cc208, 0x5ef);
        return 0;
    }
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let r = tirar(cps11_lista_no_bloco(b, f), f); // cps11: tirar
    if r != 0 {
        crate::comparacao::ramo("cps_11 jogo tirado");
    }
    (r != 0) as u32
}

/// A lista do jogo no calendário, achada antes da original (o laço da corrente faz o mesmo que ela faria: depois dele
/// a original acha o bloco do ano de primeira); 0 sem memória.
unsafe fn cps11_lista_antes(fm: usize, f: usize) -> usize {
    let b = cps11_bloco_do_ano(fm, f);
    if b == 0 { 0 } else { cps11_lista_no_bloco(b, f) }
}

/// Comparador: a original e a cópia só andam o cursor da lista (e a primeira pode criar o bloco do ano, que a segunda
/// acha pronto): pela devolução.
unsafe extern "thiscall" fn comparar_jogo_guardado_igual(this: usize, f: usize) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1146)(this, f);
    let n = jogo_guardado_igual(this, f);
    PLACAR.registrar(r == n, || format!("jogo_guardado_igual(jogo {:x}) original={:x} rust={:x}", f, r, n));
    r
}

/// Comparador: acrescenta à lista (não roda duas vezes): a lista é achada antes pela cópia, a original guarda e a conta
/// tem de subir 1 com a cauda nova igual aos 0x4f bytes do jogo.
unsafe extern "thiscall" fn comparar_guardar_jogo_no_calendario(this: usize, f: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1147);
    let l = cps11_lista_antes(this, f);
    if l == 0 {
        return o(this, f); // ponytail: sem memória para o bloco, só a original (a caixa)
    }
    let conta = cps11_w(l);
    o(this, f);
    let cauda = cps11_d(l + 6);
    let igual = cps11_w(l) == conta.wrapping_add(1) && cauda != 0
        && core::slice::from_raw_parts(cps11_d(cauda) as *const u8, 0x4f) == core::slice::from_raw_parts(f as *const u8, 0x4f);
    PLACAR.registrar(igual, || format!("guardar_jogo_no_calendario(jogo {:x}) conta antes={} depois={}", f, conta, cps11_w(l)));
}

/// Comparador: tira da lista (não roda duas vezes): antes da original, a cópia acha a lista e vê se algum item tem os
/// bytes do jogo (o tamanho guardado em +4, como a 0x68d1f0); a original tira e devolve: a devolução e a conta.
unsafe extern "thiscall" fn comparar_tirar_jogo_do_calendario(this: usize, f: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1148);
    let l = cps11_lista_antes(this, f);
    if l == 0 {
        return o(this, f); // ponytail: sem memória para o bloco, só a original (a caixa)
    }
    let conta = cps11_w(l);
    let mut no = cps11_d(l + 2);
    let mut tem = false;
    while no != 0 && !tem {
        let n = *((no + 4) as *const i16) as isize as usize;
        tem = core::slice::from_raw_parts(cps11_d(no) as *const u8, n) == core::slice::from_raw_parts(f as *const u8, n);
        no = cps11_d(no + 6);
    }
    let r = o(this, f);
    let esperado = tem as u32;
    let igual = r == esperado && cps11_w(l) == conta.wrapping_sub(esperado as u16);
    PLACAR.registrar(igual, || format!("tirar_jogo_do_calendario(jogo {:x}) original={} esperado={} conta antes={} depois={}",
        f, r, esperado, conta, cps11_w(l)));
    r
}

// ---------- 255a leva: os amistosos marcados: criar os amistosos planejados da semana, acrescentar um amistoso e sortear a data livre do amistoso (friendly.cpp) ----------

unsafe fn cps12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps12_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O clube de índice `i` em [0xae23b4] (0x245 bytes).
unsafe fn cps12_clube(i: usize) -> usize {
    cps12_d(0xae23b4).wrapping_add(i.wrapping_mul(0x245))
}

/// Uma lista nova (0x944cf1 de 0xe bytes + 0x68d020); 0 sem memória.
unsafe fn cps12_lista_nova() -> usize {
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let p = novo(0xe);
    if p != 0 { criar(p) } else { 0 }
}

/// Acrescenta à lista `l` uma cópia de `n` bytes de `p` (0x68d0d0); devolve a cópia (0 se falhou).
unsafe fn cps12_copiar(l: usize, p: usize, n: u32) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    copiar(l, p, n)
}

/// A situação da data `data` para o clube (0x5c75a0 Friendly::get_club_date_status, com as três saídas em locais, sem
/// datas escolhidas, tipo 0 e conferindo o estádio): 1 = livre.
unsafe fn cps12_situacao(this: usize, clube: usize, data: usize) -> u8 {
    let situacao: unsafe extern "thiscall" fn(usize, usize, usize, *mut u32, *mut u32, *mut u32, u32, u32, u32, u32) -> u8 =
        core::mem::transmute(0x5c75a0usize);
    let (mut t, mut m, mut q) = (0u32, 0u32, 0u32);
    situacao(this, clube, data, &mut t, &mut m, &mut q, 0, 0, 0, 1)
}

/// { 0x5c2ae0 } (thiscall, ret 0, friendly.cpp): `Friendly::create_due_planned_friendlies()`: com o plano dos amistosos
/// da IA (+0x34d8, entradas de 20 bytes: +0 a data, +8 o clube A, +0xc o B, +0x10 o C; o índice em +0x34dc e a conta em
/// +0x34e0), anda o índice: entrada de data anterior a hoje + 1 semana (0x54c770) passa; de data posterior para; na data,
/// com A, B e C válidos e B e C livres na data (0x5c75a0 = 1), cria o amistoso (0x5c6f00: A contra C em casa se A = B;
/// senão A contra B fora) e manda o pedido (0x5c5570). Plano acabado: libera o vetor (0x9452ca) e zera +0x34d8, +0x34e0 e
/// +0x34e4.
unsafe extern "thiscall" fn criar_amistosos_planejados(this: usize) {
    let semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c770usize);
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> usize = core::mem::transmute(0x5c6f00usize);
    let pedir: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x5c5570usize);
    let mut alvo = [0u8; 8];
    semanas(0xae2c90, &mut alvo, 1);
    if cps12_d(this + 0x34d8) == 0 {
        return;
    }
    let (dia, ano) = (u16::from_le_bytes([alvo[0], alvo[1]]), u16::from_le_bytes([alvo[2], alvo[3]]));
    while (cps12_d(this + 0x34dc) as i32) < cps12_d(this + 0x34e0) as i32 {
        let i = cps12_d(this + 0x34dc);
        let e = cps12_d(this + 0x34d8).wrapping_add(i.wrapping_mul(20));
        let (edia, eano) = (cps12_w(e), cps12_w(e + 2));
        if eano == ano && edia == dia {
            let (a, b, c) = (cps12_d(e + 8), cps12_d(e + 0xc), cps12_d(e + 0x10));
            if a == u32::MAX as usize || c == u32::MAX as usize || b == u32::MAX as usize {
                cps12_pd(this + 0x34dc, i.wrapping_add(1));
                continue;
            }
            let mut data = [0u8; 8]; // cópia da data da entrada
            core::ptr::copy_nonoverlapping(e as *const u8, data.as_mut_ptr(), 8);
            let pdata = data.as_mut_ptr() as usize;
            if cps12_situacao(this, cps12_clube(b), pdata) == 1 && cps12_situacao(this, cps12_clube(c), pdata) == 1 {
                let e = cps12_d(this + 0x34d8).wrapping_add(cps12_d(this + 0x34dc).wrapping_mul(20));
                crate::comparacao::ramo("cps_12 amistoso planejado");
                let m = if a == b { acrescentar(this, cps12_clube(a), e, cps12_clube(c), 1) } else { acrescentar(this, cps12_clube(a), e, cps12_clube(b), 0) }; // cps12: criar
                if m != 0 {
                    pedir(this, m);
                }
            }
            cps12_pd(this + 0x34dc, cps12_d(this + 0x34dc).wrapping_add(1));
            continue;
        }
        // datas: o ano sem sinal, o dia com sinal
        if ano < eano || (ano == eano && (dia as i16) < edia as i16) {
            return; // a entrada é de depois
        }
        if ano > eano || (dia as i16) > edia as i16 {
            cps12_pd(this + 0x34dc, i.wrapping_add(1)); // ficou para trás
        }
    }
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    liberar(cps12_d(this + 0x34d8));
    cps12_pd(this + 0x34d8, 0);
    cps12_pd(this + 0x34e0, 0);
    cps12_pd(this + 0x34e4, 0);
}

/// O estádio do clube (+0x69) já tem jogo na data em alguma fatia (0x5ae870 do calendário [0xae2a58] com −1)? Sem
/// estádio: não.
unsafe fn cps12_estadio_ocupado(clube: usize, data: usize) -> bool {
    let estadio = cps12_d(clube + 0x69);
    if estadio == 0 {
        return false;
    }
    let ocupado: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x5ae870usize);
    // ponytail: a original empurra o dia e o ano (palavras) com sobra de registrador em cima; a 0x5ae870 lê as palavras
    ocupado(cps12_d(0xae2a58), estadio, cps12_w(data) as u32, cps12_w(data + 2) as u32, u32::MAX) != 0
}

/// O estádio de quem joga em casa (o clube com `em_casa`, senão o adversário, se houver) está ocupado na data?
unsafe fn cps12_casa_ocupada(clube: usize, data: usize, adversario: usize, em_casa: u32) -> bool {
    if em_casa != 0 {
        cps12_estadio_ocupado(clube, data)
    } else {
        adversario != 0 && cps12_estadio_ocupado(adversario, data)
    }
}

/// { 0x5c6f00 } (thiscall, ret 0x10, friendly.cpp): `Friendly::add_friendly_match(clube, data, adversario, em_casa)`:
/// reaproveita o convite aberto do clube (0x5c88f0 sem data) ou monta um amistoso novo de 0x19 bytes (0x944cf1;
/// 0x54a110; sem memória: a caixa, linha 0xe6b, e 0). Se o estádio de quem joga em casa está ocupado na data (0x5ae870):
/// 0 (o novo fica sem liberar, como na original). Grava a data (0x417c10), +0xc o clube, +0x18 = 1, +8 = 0 e o mandante
/// (+0x10) e o visitante (+0x14). Sem a lista dos amistosos (+0x34c8), cria (0x68d020; sem memória: a caixa, linha
/// 0xe9f, libera o amistoso e 0). O reaproveitado volta; o novo vai em cópia para a lista (0x68d0d0; falhou: a caixa,
/// linha 0xeac, e 0), é liberado (0x944c94) e a cópia volta.
unsafe extern "thiscall" fn acrescentar_amistoso(this: usize, clube: usize, data: usize, adversario: usize, em_casa: u32) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x5c88f0usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let mut reaproveitado = false;
    let mut m = achar(this, 0, clube);
    if m != 0 {
        reaproveitado = true;
    } else {
        let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
        let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
        let p = novo(0x19);
        m = if p != 0 { data_padrao(p); p } else { 0 };
        if m == 0 {
            aviso_em(0x9cd0a0, 0xe6b);
            return 0;
        }
    }
    if cps12_casa_ocupada(clube, data, adversario, em_casa) {
        return 0;
    }
    let atribuir: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut copia = [0u8; 8];
    cps12_pd(m + 0xc, clube);
    atribuir(m, &mut copia, data);
    *((m + 0x18) as *mut u8) = 1;
    cps12_pd(m + 8, 0);
    let (mandante, visitante) = if em_casa != 0 {
        crate::comparacao::ramo("cps_12 amistoso em casa");
        (clube, adversario) // cps12: o clube em casa
    } else {
        (adversario, clube)
    };
    cps12_pd(m + 0x10, mandante);
    cps12_pd(m + 0x14, visitante);
    if cps12_d(this + 0x34c8) == 0 {
        let l = cps12_lista_nova();
        cps12_pd(this + 0x34c8, l);
        if l == 0 {
            aviso_em(0x9cd0a0, 0xe9f);
            liberar(m);
            return 0;
        }
    }
    if reaproveitado {
        return m;
    }
    let c = cps12_copiar(cps12_d(this + 0x34c8), m, 0x19);
    if c == 0 {
        aviso_em(0x9cd0a0, 0xeac);
        liberar(m);
        return 0;
    }
    liberar(m);
    c
}

/// { 0x5c4380 } (thiscall, ret 0xc, friendly.cpp): `Friendly::plan_friendly_on_free_date(anfitriao, casa, fora)`: a janela
/// é a do clube da casa (+0xe..+0x10, palavras com sinal) cortada pela do de fora (se +0xe ≠ −1); vazia: 0. Sorteia um
/// deslocamento (0x90cfc0) e a data = início da temporada (+0xa7) + início da janela + sorteio (0x54c5b0). Até 5 vezes,
/// os dois livres na data (0x5c45a0 = 0 para os dois) marcam achado; a marca começa no lugar do argumento de fora, que a
/// original zera antes. Sem achado: 0. Achado: o amistoso entra (0x5c4b60), a data vai para as listas de datas dos dois
/// (+0x12, criadas se faltam; 8 bytes), cada um entra na lista de adversários do outro (+0xa; os 4 primeiros bytes do
/// candidato) e as contas (+4) sobem. Devolve 1.
unsafe extern "thiscall" fn planejar_amistoso_em_data_livre(this: usize, anfitriao: usize, casa: usize, fora: usize) -> u32 {
    let mut ini = *((casa + 0xe) as *const i16);
    let mut fim = *((casa + 0x10) as *const i16);
    let fini = *((fora + 0xe) as *const i16);
    if fini != -1 {
        if fini > ini {
            ini = fini;
        }
        let ffim = *((fora + 0x10) as *const i16);
        if ffim < fim {
            fim = ffim;
        }
    }
    if ini >= fim {
        return 0;
    }
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let n = (ini as i32).wrapping_add(sortear(fim as i32 - ini as i32)); // cps12: o dia sorteado
    let mut data = [0u8; 8];
    // ponytail: a original empurra o esi inteiro (a palavra baixa é a soma, em cima sobra de registrador); a 0x54c5b0 lê
    // a palavra
    mais_dias(this + 0xa7, &mut data, n as u16 as u32);
    let pdata = data.as_mut_ptr() as usize;
    let ocupado: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x5c45a0usize);
    let mut achado = false;
    let mut k = 0u8;
    loop {
        if k == 5 {
            if !achado {
                return 0;
            }
            break;
        }
        if ocupado(this, casa, pdata) == 0 && ocupado(this, fora, pdata) == 0 {
            achado = true;
        }
        k += 1;
        if achado {
            break;
        }
    }
    let marcar: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(0x5c4b60usize);
    crate::comparacao::ramo("cps_12 data livre");
    marcar(this, anfitriao, casa, fora, pdata);
    if cps12_d(casa + 0x12) == 0 {
        cps12_pd(casa + 0x12, cps12_lista_nova());
    }
    if cps12_d(fora + 0x12) == 0 {
        cps12_pd(fora + 0x12, cps12_lista_nova());
    }
    cps12_copiar(cps12_d(casa + 0x12), pdata, 8);
    cps12_copiar(cps12_d(fora + 0x12), pdata, 8);
    if cps12_d(casa + 0xa) == 0 {
        cps12_pd(casa + 0xa, cps12_lista_nova());
    }
    cps12_copiar(cps12_d(casa + 0xa), fora, 4);
    if cps12_d(fora + 0xa) == 0 {
        cps12_pd(fora + 0xa, cps12_lista_nova());
    }
    cps12_copiar(cps12_d(fora + 0xa), casa, 4);
    *((casa + 4) as *mut u8) = (*((casa + 4) as *const u8)).wrapping_add(1);
    *((fora + 4) as *mut u8) = (*((fora + 4) as *const u8)).wrapping_add(1);
    1
}

// ponytail: cria amistosos (aloca), manda os pedidos e libera o plano: não roda duas vezes. Só a original; a cópia leva o
// ramo (pré-temporada da IA, 515 chamadas até 2002-06-01).
unsafe extern "thiscall" fn comparar_criar_amistosos_planejados(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1149)(this)
}

/// Comparador: aloca e acrescenta à lista (não roda duas vezes): a original faz e a cópia confere o resultado com o
/// estádio de quem joga em casa visto antes (0x5ae870, só lê): ocupado → 0; senão o amistoso devolvido tem a data,
/// +0xc o clube, +8 = 0, +0x18 = 1 e o mandante e o visitante pelo `em_casa`.
unsafe extern "thiscall" fn comparar_acrescentar_amistoso(this: usize, clube: usize, data: usize, adversario: usize, em_casa: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> usize>(1150);
    let ocupada = cps12_casa_ocupada(clube, data, adversario, em_casa);
    let (dia, ano) = (cps12_w(data), cps12_w(data + 2));
    let r = o(this, clube, data, adversario, em_casa);
    let (casa, fora) = if em_casa != 0 { (clube, adversario) } else { (adversario, clube) };
    let igual = if ocupada {
        r == 0
    } else {
        // ponytail: r = 0 sem estar ocupada só sem memória (a caixa já saiu): não confere
        r == 0 || (cps12_w(r) == dia && cps12_w(r + 2) == ano && cps12_d(r + 0xc) == clube && cps12_d(r + 8) == 0
            && *((r + 0x18) as *const u8) == 1 && cps12_d(r + 0x10) == casa && cps12_d(r + 0x14) == fora)
    };
    PLACAR.registrar(igual, || format!("acrescentar_amistoso(clube {:x}, {}/{}, adversario {:x}, em casa {}) ocupada {} original={:x}",
        clube, dia, ano, adversario, em_casa, ocupada, r));
    r
}

// ponytail: sorteia a data e acrescenta às listas: não roda duas vezes. Só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_planejar_amistoso_em_data_livre(this: usize, anfitriao: usize, casa: usize, fora: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(1151)(this, anfitriao, casa, fora)
}

// ---------- 255a leva: o dia a dia das copas e o risco de doença na viagem: a atualização da copa no dia, montar a rodada da copa e o risco de doença dos visitantes (Cup.cpp, fix_man.cpp) ----------

unsafe fn cps13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps13_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps13_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Uma data montada pela 0x549ef0 (dia, ano: ela lê as palavras; os dwords vão como os da original).
unsafe fn cps13_data(dia: u32, ano: u32) -> [u8; 8] {
    let montar: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let mut d = [0u8; 8];
    montar(&mut d, dia, ano);
    d
}

/// (dia, ano) de uma data.
fn cps13_dia_ano(d: &[u8; 8]) -> (u16, u16) {
    (u16::from_le_bytes([d[0], d[1]]), u16::from_le_bytes([d[2], d[3]]))
}

/// A rodada `k` da copa (0x68 bytes cada, em +0xa3).
unsafe fn cps13_rodada(this: usize, k: isize) -> usize {
    (cps13_d(this + 0xa3) as isize).wrapping_add(k.wrapping_mul(0x68)) as usize
}

/// A data da rodada `k` (+0 o dia, +2 o ano a partir do ano-base +0x40 da copa, byte com sinal), como nos blocos da
/// 0x51f890 em que a metade alta do dword do dia é a do ponteiro da rodada (a do ano é zero).
unsafe fn cps13_data_da_rodada(this: usize, k: isize) -> [u8; 8] {
    let r = cps13_rodada(this, k);
    let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
    cps13_data((r as u32 & 0xffff_0000) | cps13_w(r) as u32, ano)
}

/// A virtual `off` do objeto `p` (thiscall, sem argumentos).
unsafe fn cps13_virtual0(p: usize, off: usize) {
    let f: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cps13_d(cps13_d(p) + off));
    f(p)
}

/// A fase `i` da copa ([+0xc][i]).
unsafe fn cps13_fase(this: usize, i: usize) -> usize {
    cps13_d(cps13_d(this + 0xc) + i * 4)
}

/// { 0x51f890 } (thiscall, ret 8, Cup.cpp; a atualização diária de cada copa): `Cup::update_for_date(hoje, fase)`. Copa
/// sem pai (+0x44 = 0xff): com +0xb1, se as fases 0..=+0x30 estão completas (virtual +0x10 com 0, 1), passa à fase
/// seguinte (virtual +0x28, se +0x30 < +0x2c − 1) ou encerra uma vez (+0x4c: o histórico 0x442e80 de [0xad9da8], a
/// virtual +0x30, os recordes 0x4a19f0 do Comp_Stats +8); depois as notícias dos recordes (0x4a2450). Ainda sem pai,
/// cada fase recebe o dia (virtual +0x14 com 0xae2c90 e a fase). Fase 0 (com [0xae2c88] = 1): a última rodada final
/// (+7 = 0x8c) sorteia na data dela (0x51c410); sem +0xb1, sorteia as rodadas a partir de +0xac cuja data chegou, até
/// não avançar ou acabar. Fase ≠ 0 (com +0x45): o mesmo com a montagem das rodadas (0x51c800) a partir de +0xab, que
/// também monta na data pendente da rodada (+0x34 da fase da rodada). Devolve 0.
unsafe extern "thiscall" fn atualizar_copa_no_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let sortear: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51c410usize);
    let montar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51c800usize);
    if cps13_b(this + 0x44) == 0xff {
        let mut completas = cps13_b(this + 0xb1) != 0;
        if completas {
            let mut i = 0usize;
            while (i as i32) <= cps13_d(this + 0x30) as i32 {
                let p = cps13_fase(this, i);
                let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(cps13_d(cps13_d(p) + 0x10));
                if completa(p, 0, 1) == 0 {
                    completas = false;
                    break;
                }
                i += 1;
            }
        }
        if completas {
            if (cps13_d(this + 0x30) as i32) < (cps13_d(this + 0x2c) as i32).wrapping_sub(1) {
                cps13_virtual0(this, 0x28); // a fase seguinte
            } else if cps13_d(this + 0x4c) == 0 {
                let historico: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x442e80usize);
                historico(cps13_d(0xad9da8), this);
                cps13_virtual0(this, 0x30);
                if cps13_d(this + 8) != 0 {
                    let recordes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4a19f0usize);
                    recordes(cps13_d(this + 8));
                }
                core::ptr::write_unaligned((this + 0x4c) as *mut u32, 1);
            }
        }
        if cps13_d(this + 8) != 0 {
            let noticias: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x4a2450usize);
            noticias(cps13_d(this + 8));
        }
    }
    if cps13_b(this + 0x44) == 0xff {
        let mut i = 0usize;
        while (i as i32) <= cps13_d(this + 0x30) as i32 {
            let p = cps13_fase(this, i);
            let dia: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(cps13_d(cps13_d(p) + 0x14));
            dia(p, 0xae2c90, fase);
            i += 1;
        }
    }
    let (hdia, hano) = (cps13_w(hoje), cps13_w(hoje + 2));
    let n = cps13_w(this + 0x3c) as i16;
    if fase == 0 {
        if cps13_b(0xae2c88) != 1 {
            return 0;
        }
        if n > 0 && cps13_w(cps13_rodada(this, n as isize - 1) + 7) == 0x8c {
            let r = cps13_rodada(this, n as isize - 1);
            let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
            let d = cps13_data(cps13_w(r) as u32, ano);
            if cps13_dia_ano(&d) == (hdia, hano) {
                sortear(this, (n as i32 - 1) as u32); // a final na data dela
            }
        }
        if cps13_b(this + 0xb1) != 0 {
            return 0;
        }
        let mut k = cps13_b(this + 0xac) as i8;
        if k as i16 == cps13_w(this + 0x3c) as i16 {
            return 0;
        }
        loop {
            let (dia, ano) = cps13_dia_ano(&cps13_data_da_rodada(this, k as isize));
            // o ano sem sinal, o dia com sinal
            if ano > hano || (ano == hano && dia as i16 > hdia as i16) { // cps13: a data do sorteio
                return 0;
            }
            crate::comparacao::ramo("cps_13 sorteio da rodada");
            sortear(this, cps13_b(this + 0xac) as i8 as i32 as u32);
            let novo = cps13_b(this + 0xac) as i8;
            if novo == k || novo as i16 == cps13_w(this + 0x3c) as i16 {
                return 0;
            }
            k = novo;
        }
    }
    if cps13_d(this + 0x45) == 0 {
        return 0;
    }
    if n > 0 && cps13_w(cps13_rodada(this, n as isize - 1) + 7) == 0x8c {
        let r = cps13_rodada(this, n as isize - 1);
        let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
        let d = cps13_data(cps13_w(r) as u32, ano);
        if cps13_dia_ano(&d) == (hdia, hano) {
            montar(this, (n as i32 - 1) as u32); // a final na data dela
        }
    }
    if cps13_b(this + 0xb1) != 0 || cps13_d(this + 0x45) == 0 {
        return 0;
    }
    let mut k = cps13_b(this + 0xab) as i8;
    if k as i16 == cps13_w(this + 0x3c) as i16 {
        return 0;
    }
    loop {
        let (dia, ano) = cps13_dia_ano(&cps13_data_da_rodada(this, k as isize));
        if (ano, dia) != (hano, hdia) { // cps13: a data da montagem
            let atual = cps13_b(this + 0xab) as i8 as isize;
            let p = cps13_d((cps13_d(this + 0xad) as isize + atual * 4) as usize);
            if cps13_b(p + 0x34) == 0 {
                return 0; // nem a data nem pendente
            }
        }
        crate::comparacao::ramo("cps_13 montagem da rodada");
        montar(this, cps13_b(this + 0xab) as i8 as i32 as u32);
        let novo = cps13_b(this + 0xab) as i8;
        if novo == k || novo as i16 == cps13_w(this + 0x3c) as i16 {
            return 0;
        }
        k = novo;
    }
}

/// { 0x51c800 } (thiscall, ret 4, Cup.cpp): `Cup::set_up_round(rodada)`: sem +0x45: 1. Rodada fora de 0..+0x3c − 1: a
/// caixa (linha 0xa64) e −1. Sorteia a rodada se a fase dela não tem +0x34 (0x51c410). Na rodada atual (+0xab): se um
/// confronto (0x35 bytes na fase, +0/+2 os times ou 0xffff) tem time sem clube (+0xa7, 6 bytes), adia a rodada pedida e
/// as seguintes um dia (0x54c5b0, 0x417c10; as seguintes só se ficaram antes) e devolve 1. Senão cria os jogos de cada
/// confronto marcado (+0x34 = 1 adiado, 2 já: 0x51cb90; depois do 2, sem a copa em [0xadadfc][id]: 1), leva os jogos às
/// datas da rodada (data +3/+5 num bloco de 8 bytes, 0x51ebe0, liberado depois) e, com a copa em [0xadadfc][id], passa à
/// rodada seguinte (fora da final 0x8c) e leva +0xac junto. Devolve 1.
unsafe extern "thiscall" fn montar_rodada_da_copa(this: usize, rodada: u32) -> u32 {
    let id = cps13_d(cps13_d(this + 4));
    let ativa = || cps13_d(cps13_d(0xadadfc) + id * 4) != 0;
    if cps13_d(this + 0x45) == 0 {
        return 1;
    }
    let ri = rodada as i32;
    if ri > cps13_w(this + 0x3c) as i16 as i32 - 1 || ri < 0 {
        aviso_em(0x9b55ac, 0xa64);
        return u32::MAX;
    }
    let ad = cps13_d(this + 0xad);
    if cps13_b(cps13_d(ad + rodada as usize * 4) + 0x34) == 0 {
        let sortear: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51c410usize);
        sortear(this, rodada);
    }
    let atual = cps13_b(this + 0xab) as i8 as isize;
    let base = cps13_d(this + 0xa3);
    let off = atual * 0x68;
    let n = *((base as isize + off + 0x1a) as *const i16) as i32;
    let mut sem_clube = false;
    for i in 0..n.max(0) as usize {
        let t = cps13_d((cps13_d(this + 0xad) as isize + atual * 4) as usize) + i * 0x35;
        for lado in [0usize, 2] {
            let time = cps13_w(t + lado);
            if time != 0xffff && cps13_d(cps13_d(this + 0xa7) + (time as i16 as isize * 6) as usize) == 0 {
                sem_clube = true;
                break;
            }
        }
        if sem_clube {
            break;
        }
    }
    if sem_clube {
        // a rodada pedida (não a atual) e as seguintes ficam um dia depois
        let mais_dias: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c5b0usize);
        let atribuir: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
        let r = cps13_rodada(this, ri as isize);
        let ano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
        let mut d = cps13_data((cps13_d(this + 0xa7) as u32 & 0xffff_0000) | cps13_w(r) as u32, ano);
        let (mut amanha, mut copia) = ([0u8; 8], [0u8; 8]);
        let p = mais_dias(&mut d, &mut amanha, 1);
        atribuir(&mut d, &mut copia, p);
        let (dia, ano) = cps13_dia_ano(&d);
        crate::comparacao::ramo("cps_13 rodada adiada");
        let r = cps13_rodada(this, ri as isize);
        core::ptr::write_unaligned(r as *mut u16, dia);
        *((r + 2) as *mut u8) = (ano as u8).wrapping_sub(cps13_b(this + 0x40));
        let mut k = ri + 1;
        while (k as i16) < cps13_w(this + 0x3c) as i16 {
            let r = cps13_rodada(this, k as i16 as isize);
            let rano = (cps13_b(r + 2) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
            let (odia, oano) = cps13_dia_ano(&cps13_data((cps13_d(this + 0xa3) as u32 & 0xffff_0000) | cps13_w(r) as u32, rano));
            if ano > oano || (ano == oano && dia as i16 > odia as i16) {
                let r = cps13_rodada(this, k as i16 as isize);
                core::ptr::write_unaligned(r as *mut u16, dia);
                *((r + 2) as *mut u8) = (ano as u8).wrapping_sub(cps13_b(this + 0x40));
            }
            k += 1;
        }
        return 1;
    }
    let criar: unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32 = core::mem::transmute(0x51cb90usize);
    let mut i = 0i32;
    while n > 0 && i < *((cps13_d(this + 0xa3) as isize + off + 0x1a) as *const i16) as i32 {
        let marca = |i: i32| cps13_b(cps13_d((cps13_d(this + 0xad) as isize + atual * 4) as usize) + i as usize * 0x35 + 0x34);
        if marca(i) == 1 {
            criar(this, atual as i32 as u32, i as u32, 1);
        }
        if marca(i) == 2 {
            criar(this, atual as i32 as u32, i as u32, 0);
            if !ativa() {
                return 1;
            }
        }
        i += 1;
    }
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let montar_data: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let p = novo(8);
    let data = if p != 0 {
        let base = cps13_d(this + 0xa3);
        let r = (base as isize + off) as usize;
        let ano = (cps13_b(r + 5) as i8 as i16).wrapping_add(cps13_w(this + 0x40) as i16) as u16 as u32;
        let dia = cps13_w(r + 3) as u32;
        montar_data(p, (r as u32 & 0xffff_0000) | dia, (base as u32 & 0xffff_0000) | ano)
    } else {
        0
    };
    let levar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51ebe0usize);
    crate::comparacao::ramo("cps_13 jogos da rodada");
    levar(this, data, atual as i32 as u32);
    if data != 0 {
        let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
        liberar(data);
    }
    if !ativa() {
        return 1;
    }
    if cps13_w((cps13_d(this + 0xa3) as isize + off + 7) as usize) != 0x8c {
        *((this + 0xab) as *mut u8) = cps13_b(this + 0xab).wrapping_add(1);
    }
    if (cps13_b(this + 0xac) as i8) < cps13_b(this + 0xab) as i8 {
        *((this + 0xac) as *mut u8) = cps13_b(this + 0xab);
    }
    1
}

/// Põe o risco de doença (0x62e820 de [0xb63d28], tipo 3) na véspera da data do jogo `f` (+0x2a o dia, +0x28 o ano;
/// 0x549ef0, 0x54c8e0, 0x417c10) para o clube `id` com a nação do estádio. `alto_dia`/`alto_ano`: as metades altas dos
/// dwords que a original empurra à 0x549ef0.
unsafe fn cps13_doenca(f: usize, id: usize, nacao: usize, alto_dia: u32, alto_ano: u32) {
    let menos_dias: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let atribuir: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let doenca: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32, usize, usize) -> u32 = core::mem::transmute(0x62e820usize);
    let mut d = cps13_data((alto_dia & 0xffff_0000) | cps13_w(f + 0x2a) as u32, (alto_ano & 0xffff_0000) | cps13_w(f + 0x28) as u32);
    let (mut vespera, mut copia) = ([0u8; 8], [0u8; 8]);
    let p = menos_dias(&mut d, &mut vespera, 1);
    atribuir(&mut d, &mut copia, p);
    doenca(cps13_d(0xb63d28), &mut d, 3, id, nacao);
}

/// { 0x5ab3c0 } (thiscall, ret 4, fix_man.cpp; o `this` não é usado): `FIXTURE_MANAGER::add_away_trip_illness_risk(jogo)`:
/// com o estádio do jogo (+0x18), a cidade (+0x38) e a nação dela (+0x1f) num país do terceiro mundo (+0x7e = 3), cada
/// clube visitante de outra nação (+0x53) — o mandante (+0x1c, id +0xc) e o visitante (+0x20, id +0x10) — ganha o risco
/// de doença na véspera do jogo.
unsafe extern "thiscall" fn risco_de_doenca_na_viagem(_this: usize, f: usize) {
    let estadio = cps13_d(f + 0x18);
    if estadio == 0 {
        return;
    }
    let cidade = cps13_d(estadio + 0x38);
    if cidade == 0 {
        return;
    }
    let nacao = cps13_d(cidade + 0x1f);
    if nacao == 0 {
        return;
    }
    if cps13_b(nacao + 0x7e) != 3 { // cps13: só o terceiro mundo
        return;
    }
    let casa = cps13_d(f + 0x1c);
    if casa != 0 && cps13_d(casa + 0x53) != nacao {
        crate::comparacao::ramo("cps_13 doenca do mandante");
        // a original empurra o ano no eax do clube e o dia no ecx da nação (metades altas dos ponteiros)
        cps13_doenca(f, cps13_d(f + 0xc), cps13_d(cps13_d(cps13_d(f + 0x18) + 0x38) + 0x1f), nacao as u32, casa as u32);
    }
    let fora = cps13_d(f + 0x20);
    if fora != 0 && cps13_d(fora + 0x53) != cps13_d(cps13_d(cps13_d(f + 0x18) + 0x38) + 0x1f) {
        crate::comparacao::ramo("cps_13 doenca do visitante");
        // o ano no ecx do estádio e o dia no edx da cidade
        let (e, c) = (cps13_d(f + 0x18), cps13_d(cps13_d(f + 0x18) + 0x38));
        cps13_doenca(f, cps13_d(f + 0x10), cps13_d(c + 0x1f), c as u32, e as u32);
    }
}

// ponytail: sorteia e monta rodadas (aloca jogos): não roda duas vezes. Só a original; a cópia leva os ramos (sorteio e
// montagem; a atualização é diária em cada copa, 29.934 chamadas até 2002-06-01: sem ramo no caminho comum).
unsafe extern "thiscall" fn comparar_atualizar_copa_no_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1152)(this, hoje, fase)
}

// ponytail: sorteia, cria os jogos e passa a rodada: não roda duas vezes. Só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_montar_rodada_da_copa(this: usize, rodada: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(1153)(this, rodada)
}

// ponytail: acrescenta o risco de doença (não roda duas vezes; o caminho comum não faz nada). Só a original; a cópia leva
// os ramos.
unsafe extern "thiscall" fn comparar_risco_de_doenca_na_viagem(this: usize, f: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1154)(this, f)
}
