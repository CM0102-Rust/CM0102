// ---------- 251a leva: a confiança da diretoria: o negócio da transferência, a confiança depois da transferência e o ultimato (manager_manager.cpp) ----------

unsafe fn cpo1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo1_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo1_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

unsafe fn cpo1_ou(a: usize, v: u8) {
    *(a as *mut u8) |= v;
}

/// Soma `v` à palavra de 16 bits (com a volta do `add word`).
unsafe fn cpo1_somar(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, cpo1_w(a).wrapping_add(v));
}

/// Um `double` da tabela (sem alinhamento garantido).
unsafe fn cpo1_f(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

unsafe fn cpo1_f_por(a: usize, v: f64) {
    core::ptr::write_unaligned(a as *mut f64, v);
}

/// A constante `double` do próprio exe.
unsafe fn cpo1_k(a: usize) -> f64 {
    cpo1_f(a)
}

/// A linha do clube na tabela da diretoria do MANAGER_MANAGER ([this] + 0x49 × id): +2 marcas, +6 confiança (1..10000),
/// +0x14 e +0x16 os humores da diretoria, +0x38 o gasto líquido (double).
unsafe fn cpo1_linha(this: usize, id: usize) -> usize {
    cpo1_d(this).wrapping_add(id.wrapping_mul(0x49))
}

/// O valor do jogador para o clube (CONTRACT_MANAGER 0x4d0c60 em [0xae1050], pode sortear) como a 0x696570 o pede:
/// com o clube do jogador (+0x39) quando ele tem; senão o `clube`, com o último argumento 1 se o `clube` é de humano
/// (0x600a50).
unsafe fn cpo1_valor(pessoa: usize, clube: usize) -> i32 {
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x600a50usize);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let h = humano(clube, 0, 1, 0, 0);
    let atual = cpo1_d(pessoa + 0x39);
    let (alvo, ultimo) = if h != 0 {
        (if atual != 0 { atual } else { clube }, 1)
    } else if atual != 0 {
        (atual, 1)
    } else {
        (clube, 0)
    };
    valor(cpo1_d(0xae1050), pessoa, alvo, 0, -1, ultimo)
}

/// O limite da taxa pela reputação (+0x80) do clube: (reputação × 0,2)² truncado (_ftol 0x945640).
unsafe fn cpo1_limite(clube: usize) -> i32 {
    let x = cpo1_w(clube + 0x80) as i32 as f64 * cpo1_k(0x968918);
    ftol(x * x)
}

/// { 0x696570 } (thiscall, ret 0xc, manager_manager.cpp): `MANAGER_MANAGER::assess_transfer_deal(transf, f1, f2)`:
/// com o jogador (+0xc) que tem ficha (+0x61), zera as duas palavras de marcas e: pelo clube comprador (+8, registro de
/// 0x245 bytes) soma o valor do jogador (0x4d0c60, 1 se 0) ao gasto líquido (+0x38) e marca `f1` pela razão taxa
/// (0x8e7a70) / valor: taxa até o limite (reputação × 0,2)² = 4; valor acima de 250.000: < 0,2 = 0x10, < 0,5 = 8, < 2 =
/// 4, < 6 = 2, senão 1; até 250.000: < 0,125 = 8, < 8 = 4, senão 2; o jogador favorito ou rejeitado do clube ou do
/// presidente (+0xbf; 0x547790/0x5477d0/0x547810/0x547840) = 0x40 ou 0x100. Pelo clube vendedor (+0x39 do jogador)
/// tira o valor do gasto e marca `f2` com as faixas espelhadas (0,25/0,5/2/4; 0,125/8) e 0x20 ou 0x80. Transferência
/// já avaliada (+0x1f = 1): nada (as marcas ficam como estão).
unsafe extern "thiscall" fn avaliar_o_negocio_da_transferencia(this: usize, transf: usize, f1: usize, f2: usize) {
    if *((transf + 0x1f) as *const u8) == 1 {
        return;
    }
    core::ptr::write_unaligned(f2 as *mut u16, 0);
    core::ptr::write_unaligned(f1 as *mut u16, 0);
    let pessoa = cpo1_d(0xae23bc).wrapping_add(cpo1_d(transf + 0xc).wrapping_mul(0x6e));
    if cpo1_d(pessoa + 0x61) == 0 {
        return;
    }
    let comprador = cpo1_d(0xae23b4).wrapping_add(cpo1_d(transf + 8).wrapping_mul(0x245));
    let mut valor = cpo1_valor(pessoa, comprador);
    if valor == 0 {
        valor = 1;
    }
    let oferta: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x8e7a70usize);
    let taxa = oferta(transf, 0, 0, 0, 1);
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547790usize);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5477d0usize);
    let favorito_do_presidente: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let rejeitado_do_presidente: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    if comprador != 0 {
        let g = cpo1_linha(this, cpo1_d(comprador)) + 0x38;
        cpo1_f_por(g, valor as f64 + cpo1_f(g));
        let limite = cpo1_limite(comprador);
        let razao = if valor == 0 { *(0x968f18 as *const f32) as f64 } else { taxa as f64 / valor as f64 };
        if taxa <= limite {
            crate::comparacao::ramo("cpo_1 comprador no limite");
            cpo1_ou(f1, 4); // o comprador: taxa dentro do limite
        } else if valor > 0x3d090 {
            let m = if razao < cpo1_k(0x968918) {
                0x10
            } else if razao < cpo1_k(0x968968) {
                8
            } else if razao < cpo1_k(0x968950) {
                4
            } else if razao < cpo1_k(0x969000) {
                2
            } else {
                1
            };
            cpo1_ou(f1, m);
        } else {
            let m = if razao < cpo1_k(0x968fd0) { 8 } else if razao < cpo1_k(0x968f10) { 4 } else { 2 };
            cpo1_ou(f1, m);
        }
        if favorito(pessoa, comprador) != 0 {
            cpo1_ou(f1, 0x40);
        } else if rejeitado(pessoa, comprador) != 0 {
            cpo1_ou(f1 + 1, 1);
        }
        if favorito_do_presidente(pessoa, cpo1_d(comprador + 0xbf)) != 0 {
            cpo1_ou(f1, 0x40);
        } else if rejeitado_do_presidente(pessoa, cpo1_d(comprador + 0xbf)) != 0 {
            cpo1_ou(f1 + 1, 1);
        }
    }
    let vendedor = cpo1_d(pessoa + 0x39);
    if vendedor == 0 {
        return;
    }
    let v2 = cpo1_valor(pessoa, vendedor); // aqui sem o mínimo 1
    let g = cpo1_linha(this, cpo1_d(vendedor)) + 0x38;
    cpo1_f_por(g, cpo1_f(g) - v2 as f64);
    let limite = cpo1_limite(vendedor);
    let razao = if v2 == 0 { *(0x968f18 as *const f32) as f64 } else { taxa as f64 / v2 as f64 };
    if taxa <= limite {
        crate::comparacao::ramo("cpo_1 vendedor no limite");
        cpo1_ou(f2, 4); // o vendedor: taxa dentro do limite
    } else if v2 > 0x3d090 {
        let m = if razao < cpo1_k(0x9689e0) {
            1
        } else if razao < cpo1_k(0x968968) {
            2
        } else if razao < cpo1_k(0x968950) {
            4
        } else if razao < cpo1_k(0x968938) {
            8
        } else {
            0x10
        };
        cpo1_ou(f2, m);
    } else {
        let m = if razao < cpo1_k(0x968fd0) { 2 } else if razao < cpo1_k(0x968f10) { 4 } else { 8 };
        cpo1_ou(f2, m);
    }
    if favorito(pessoa, vendedor) != 0 {
        cpo1_ou(f2, 0x20);
        return;
    }
    if rejeitado(pessoa, vendedor) != 0 {
        cpo1_ou(f2, 0x80);
        return;
    }
    if favorito_do_presidente(pessoa, cpo1_d(vendedor + 0xbf)) != 0 {
        cpo1_ou(f2, 0x20);
        return;
    }
    if rejeitado_do_presidente(pessoa, cpo1_d(vendedor + 0xbf)) != 0 {
        cpo1_ou(f2, 0x80);
    }
}

/// As faixas de confiança (+6, 1..10000) e o limite de baixo do humor +0x16 (−10000).
unsafe fn cpo1_limitar(e: usize) {
    if cpo1_w(e + 6) < 1 {
        core::ptr::write_unaligned((e + 6) as *mut i16, 1);
    }
    if cpo1_w(e + 6) > 10000 {
        core::ptr::write_unaligned((e + 6) as *mut i16, 10000);
    }
    if cpo1_w(e + 0x16) <= -10000 {
        core::ptr::write_unaligned((e + 0x16) as *mut i16, -10000);
    }
}

/// A parte da razão nas marcas (bits 1..0x10, o primeiro que vale): a confiança (+6) e o humor +0x14.
unsafe fn cpo1_pela_razao(e: usize, f: u32) {
    if f & 1 != 0 {
        cpo1_somar(e + 6, -0x7d);
        cpo1_somar(e + 0x14, -500);
    } else if f & 2 != 0 {
        cpo1_somar(e + 6, -0x19);
        cpo1_somar(e + 0x14, -250);
    } else if f & 4 != 0 {
        // o negócio justo: nada
    } else if f & 8 != 0 {
        cpo1_somar(e + 6, 0x19);
        cpo1_somar(e + 0x14, 0x19);
    } else if f & 0x10 != 0 {
        cpo1_somar(e + 6, 0x4b);
        cpo1_somar(e + 0x14, 0x4b);
    }
}

/// { 0x6978f0 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::apply_transfer_confidence_changes(transf)`:
/// avalia o negócio (0x696570) com as marcas do comprador na vaga do próprio argumento e as do vendedor na vaga do
/// `ecx` empurrado (avaliado antes, +0x1f = 1: ficam os bits do ponteiro, como na original) e muda a linha da diretoria
/// do comprador (+8) e, se o vendedor (+0x39 do jogador) não é seleção (0x53b900) nem tem as finanças com +0x165 = 1
/// (0x5a0590), a dele: razão 1: −125/−500, 2: −25/−250, 8: +25/+25, 0x10: +75/+75 (confiança +6 e humor +0x14);
/// favorito e rejeitado no humor +0x16 (e +0x14). Limita a confiança a 1..10000 e +0x16 a −10000 ou mais; no fim o
/// jogador sai das pessoas favoritas do presidente do vendedor (0x547c10).
unsafe extern "thiscall" fn confianca_da_diretoria_na_transferencia(this: usize, transf: usize) {
    let pessoa = cpo1_d(0xae23bc).wrapping_add(cpo1_d(transf + 0xc).wrapping_mul(0x6e));
    let mut fc: usize = transf; // a vaga do argumento
    let mut fv: usize = this; // o ecx empurrado no prólogo
    let avaliar: unsafe extern "thiscall" fn(usize, usize, usize, usize) = core::mem::transmute(0x696570usize);
    avaliar(this, transf, &mut fc as *mut usize as usize, &mut fv as *mut usize as usize);
    let f = fc as u32;
    let e = cpo1_linha(this, cpo1_d(transf + 8));
    cpo1_pela_razao(e, f);
    if f & 0x20 != 0 {
        cpo1_somar(e + 0x14, -100);
        cpo1_somar(e + 0x16, -100);
    } else if f & 0x40 != 0 {
        cpo1_somar(e + 0x16, 0x32);
    } else if f & 0x80 != 0 {
        cpo1_somar(e + 0x16, 0xa);
    } else if f & 0x100 != 0 {
        cpo1_somar(e + 0x16, -100);
        cpo1_somar(e + 0x14, -0x32);
    }
    cpo1_limitar(e);
    let vendedor = cpo1_d(pessoa + 0x39);
    if vendedor == 0 {
        return;
    }
    let id = cpo1_d(vendedor);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if selecao(vendedor) != 0 {
        return;
    }
    let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let fin = financas_do_clube(cpo1_d(0xae2a38), cpo1_d(pessoa + 0x39));
    if fin != 0 && *((fin + 0x165) as *const u8) == 1 {
        return;
    }
    let f = fv as u32;
    let e = cpo1_linha(this, id);
    cpo1_pela_razao(e, f);
    if f & 0x20 != 0 {
        cpo1_somar(e + 0x16, -100);
        cpo1_somar(e + 0x14, -0x19);
    } else if f & 0x40 != 0 {
        cpo1_somar(e + 0x16, 0x32);
        cpo1_somar(e + 0x14, 0x19);
    } else if f & 0x80 != 0 {
        cpo1_somar(e + 0x16, 0xa);
    } else if f & 0x100 != 0 {
        cpo1_somar(e + 0x16, -100);
    }
    cpo1_limitar(e);
    let tirar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x547c10usize);
    crate::comparacao::ramo("cpo_1 tirar favorito");
    tirar(cpo1_d(cpo1_d(pessoa + 0x39) + 0xbf), pessoa); // o presidente do vendedor
}

/// { 0x6ac470 } (thiscall, ret 4, manager_manager.cpp): `MANAGER_MANAGER::resolve_board_ultimatum(clube)`: percorre a
/// lista dos ultimatos (+0x10, até a conta, que cai a cada retirada) e tira o item do clube (0x68d1f0); se o técnico
/// (+0xcf) está no cargo há mais de 100 dias (0x54cdb0 desde +0x3e), põe a saída na lista (+4, 0x68d0d0, 6 bytes: 1, o
/// motivo 2, ou 7 se a linha do clube na tabela da divisão (0x4a4850) não tem jogos (+0x55), e o id do clube), marca
/// o bit 2 em +2 da linha da diretoria e devolve 1. Senão 0.
unsafe extern "thiscall" fn resolver_o_ultimato_da_diretoria(this: usize, clube: usize) -> u8 {
    let lista = cpo1_d(this + 0x10);
    if cpo1_w(lista) == 0 {
        return 0;
    }
    cpo1_por(lista + 0xa, cpo1_d(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let remover: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    let dias: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(0x54cdb0usize);
    let mut k = 0i32;
    while k < cpo1_w(cpo1_d(this + 0x10)) as i32 {
        let item = proximo(cpo1_d(this + 0x10));
        if item != 0 && cpo1_d(clube) == cpo1_d(item) {
            remover(cpo1_d(this + 0x10), item);
            let tecnico = cpo1_d(clube + 0xcf);
            if tecnico != 0 && dias(0xae2c90, cpo1_d(tecnico + 0x3e), cpo1_d(tecnico + 0x42)) > 0x64 {
                crate::comparacao::ramo("cpo_1 ultimato cumprido");
                let mut reg = [0u8; 8];
                reg[0] = 1;
                reg[2..6].copy_from_slice(&(cpo1_d(clube) as u32).to_le_bytes());
                let mut motivo = 2u8;
                let divisao = cpo1_d(clube + 0x57);
                if divisao != 0 {
                    let comp = cpo1_d(cpo1_d(0xadadfc).wrapping_add(cpo1_d(divisao).wrapping_mul(4)));
                    if comp != 0 && cpo1_d(comp + 8) != 0 {
                        let busca: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
                        let linha = busca(cpo1_d(comp + 8), clube);
                        if linha == 0 || cpo1_i(linha + 0x55) <= 0 {
                            motivo = 7;
                        }
                    }
                }
                reg[1] = motivo;
                let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x68d0d0usize);
                acrescentar(cpo1_d(this + 4), reg.as_mut_ptr() as usize, 6);
                cpo1_ou(cpo1_linha(this, cpo1_d(clube)) + 2, 2);
                return 1;
            }
        }
        k += 1;
    }
    0
}

/// Comparador: a avaliação pede o valor do jogador (0x4d0c60, que sorteia): lado a lado com o estado do sorteador, as
/// duas palavras de marcas e os gastos líquidos do comprador e do vendedor; a cópia sem contar os sorteios.
unsafe extern "thiscall" fn comparar_avaliar_o_negocio_da_transferencia(this: usize, transf: usize, f1: usize, f2: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(967);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(f1, 2), (f2, 2)]);
    let pessoa = cpo1_d(0xae23bc).wrapping_add(cpo1_d(transf + 0xc).wrapping_mul(0x6e));
    let comprador = cpo1_d(0xae23b4).wrapping_add(cpo1_d(transf + 8).wrapping_mul(0x245));
    reg.push((cpo1_linha(this, cpo1_d(comprador)) + 0x38, 8));
    let vendedor = cpo1_d(pessoa + 0x39);
    if vendedor != 0 {
        reg.push((cpo1_linha(this, cpo1_d(vendedor)) + 0x38, 8));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, transf, f1, f2),
        || crate::aleatorio::sem_contar(|| avaliar_o_negocio_da_transferencia(this, transf, f1, f2)));
    PLACAR.registrar(igual, || format!("avaliar_o_negocio_da_transferencia({:x}) {:x?} x {:x?}", transf, mo, mr));
}

/// Comparador: lado a lado com o estado do sorteador (a avaliação 0x696570 sorteia), as linhas da diretoria do
/// comprador e do vendedor e as três pessoas favoritas do presidente do vendedor (0x547c10); a cópia sem contar.
unsafe extern "thiscall" fn comparar_confianca_da_diretoria_na_transferencia(this: usize, transf: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(968);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    let pessoa = cpo1_d(0xae23bc).wrapping_add(cpo1_d(transf + 0xc).wrapping_mul(0x6e));
    reg.push((cpo1_linha(this, cpo1_d(transf + 8)), 0x49));
    let vendedor = cpo1_d(pessoa + 0x39);
    if vendedor != 0 {
        reg.push((cpo1_linha(this, cpo1_d(vendedor)), 0x49));
        let pref = dbp_preferencias(cpo1_d(vendedor + 0xbf));
        if pref != 0 {
            reg.push((pref + 0x1c, 12));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, transf),
        || crate::aleatorio::sem_contar(|| confianca_da_diretoria_na_transferencia(this, transf)));
    PLACAR.registrar(igual, || format!("confianca_da_diretoria_na_transferencia({:x}) {:x?} x {:x?}", transf, mo, mr));
}

/// Comparador: a original tira o item da lista (e o libera) e acrescenta a saída: roda só ela e confere o plano: a
/// lista das saídas (+4) cresce de 1 quando devolve 1 (e a linha da diretoria fica com o bit 2) e fica igual com 0.
unsafe extern "thiscall" fn comparar_resolver_o_ultimato_da_diretoria(this: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(969);
    let saidas = cpo1_d(this + 4);
    if saidas == 0 || cpo1_d(this + 0x10) == 0 {
        return o(this, clube);
    }
    let antes = cpo1_w(saidas) as i32;
    let r = o(this, clube);
    let depois = cpo1_w(cpo1_d(this + 4)) as i32;
    let ok = depois == antes + (r != 0) as i32 && (r == 0 || *((cpo1_linha(this, cpo1_d(clube)) + 2) as *const u8) & 2 != 0);
    PLACAR.registrar(ok, || format!("resolver_o_ultimato_da_diretoria(clube {:x}) devolveu {}, saídas {} -> {}", clube, r, antes, depois));
    r
}

// ---------- 251a leva: os contratos: tirar o contrato principal, reenviar as ofertas guardadas e o mês do fim da temporada do clube (contract_manager.cpp) ----------

unsafe fn cpo2_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo2_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo2_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A vaga do contrato principal da pessoa: o registro de 79 bytes dela em [0xae2d0c] (+0 o índice do contrato, −1 sem).
unsafe fn cpo2_vaga(pessoa: usize) -> usize {
    cpo2_d(0xae2d0c).wrapping_add(cpo2_d(pessoa).wrapping_mul(0x4f))
}

/// { 0x4d0b50 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_contract(pessoa)`: com o contrato
/// principal (índice no registro de 79 bytes da pessoa), libera a entrada dele (+0 = −1 no vetor de 0x50 bytes do
/// CONTRACT_MANAGER), põe −1 na vaga da pessoa e refaz a folha de salários (0x598370) das finanças (0x5a0590) do clube
/// do contrato (+4). Sem contrato: nada.
unsafe extern "thiscall" fn tirar_o_contrato_principal(this: usize, pessoa: usize) {
    let indice = cpo2_i(cpo2_vaga(pessoa));
    if indice == -1 {
        return;
    }
    let e = cpo2_d(this).wrapping_add((indice as usize).wrapping_mul(0x50));
    let id_clube = cpo2_d(e + 4);
    cpo2_por(e, usize::MAX);
    let clube = cpo2_d(0xae23b4).wrapping_add(id_clube.wrapping_mul(0x245));
    crate::comparacao::ramo("cpo_2 tirar contrato");
    cpo2_por(cpo2_vaga(pessoa), usize::MAX); // a pessoa fica sem contrato
    let financas_do_clube: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let f = financas_do_clube(cpo2_d(0xae2a38), clube);
    if f != 0 {
        let folha: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x598370usize);
        folha(f);
    }
}

/// { 0x4cf4a0 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::resubmit_held_offers(id)`: percorre a lista
/// das ofertas guardadas (+0x38) e, para cada uma da pessoa `id` (+0): com a transferência (+0x59, 0x8c61e0 em
/// [0xde1f54]) cujo clube (+0x24) é o da oferta (+0x5d), põe o estado 0xf em 8 e reenvia a oferta (0x4cc490 com a
/// transferência, o clube, o byte +0x61, 1, 1, 0); sem transferência (+0x59 = −1) reenvia com −1, −1. Tira a oferta da
/// lista (0x68d1f0) e conta. O byte +0x61 vai empurrado no registrador inteiro (os bits de cima do que estava nele:
/// o ponteiro lido ou −1). Devolve quantas.
unsafe extern "thiscall" fn reenviar_ofertas_guardadas(this: usize, id: usize) -> u32 {
    let lista = cpo2_d(this + 0x38);
    if lista == 0 {
        return 0;
    }
    cpo2_por(lista + 0xa, cpo2_d(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let remover: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x68d1f0usize);
    let reenviar: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x4cc490usize);
    let mut n = 0u32;
    let mut item = proximo(cpo2_d(this + 0x38));
    while item != 0 {
        if cpo2_d(item) == id {
            let t = cpo2_d(item + 0x59);
            let byte = *((item + 0x61) as *const u8) as u32;
            if t as i32 != -1 {
                let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8c61e0usize);
                let p = achar(cpo2_d(0xde1f54), t);
                if p != 0 {
                    let q = cpo2_d(p);
                    if q != 0 && cpo2_d(q + 0x24) == cpo2_d(item + 0x5d) {
                        if *((q + 0x2c) as *const u8) == 0xf {
                            *((q + 0x2c) as *mut u8) = 8;
                        }
                        crate::comparacao::ramo("cpo_2 reenviar oferta");
                        reenviar(this, item, t, cpo2_d(item + 0x5d), (q as u32 & 0xffff_ff00) | byte, 1, 1, 0); // com a transferência
                    }
                }
            } else {
                reenviar(this, item, usize::MAX, usize::MAX, 0xffff_ff00 | byte, 1, 1, 0);
            }
            remover(cpo2_d(this + 0x38), item);
            n += 1;
        }
        item = proximo(cpo2_d(this + 0x38));
    }
    n
}

/// { 0x4d45f0 } (stdcall, ret 4, contract_manager.cpp, quadro SEH sem destrutor no caminho normal):
/// `contract_manager_season_end_month(clube)`: a data do fim da temporada do clube (Data() 0x54a110 e 0x669bd0 com 0),
/// mais 10 dias (0x54c5b0), e o mês dela (0x54cfb0). Devolve o eax da 0x54cfb0 (o mês 0..11).
unsafe extern "stdcall" fn mes_do_fim_da_temporada_do_clube(clube: usize) -> u32 {
    let mut d1 = [0u32; 2];
    let mut d2 = [0u32; 2];
    let construir: unsafe extern "thiscall" fn(*mut u32) -> usize = core::mem::transmute(0x54a110usize);
    let fim: unsafe extern "C" fn(usize, u32, *mut u32) = core::mem::transmute(0x669bd0usize);
    let somar: unsafe extern "thiscall" fn(*mut u32, *mut u32, i32) -> usize = core::mem::transmute(0x54c5b0usize);
    let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cfb0usize);
    construir(d1.as_mut_ptr());
    fim(clube, 0, d1.as_mut_ptr());
    let p = somar(d1.as_mut_ptr(), d2.as_mut_ptr(), 0xa); // dez dias depois
    mes(p)
}

/// Comparador: lado a lado pela vaga da pessoa e pela entrada do contrato (a folha 0x598370 refeita duas vezes dá o
/// mesmo, pelo contrato já tirado).
unsafe extern "thiscall" fn comparar_tirar_o_contrato_principal(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(970);
    let vaga = cpo2_vaga(pessoa);
    let indice = cpo2_i(vaga);
    if indice == -1 {
        return o(this, pessoa);
    }
    let e = cpo2_d(this).wrapping_add((indice as usize).wrapping_mul(0x50));
    let reg = [(vaga, 4), (e, 8)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, pessoa), || tirar_o_contrato_principal(this, pessoa));
    PLACAR.registrar(igual, || format!("tirar_o_contrato_principal(pessoa {:x}, contrato {}) {:x?} x {:x?}", cpo2_d(pessoa), indice, mo, mr));
}

/// Comparador: a original reenvia (0x4cc490, ofertas e transferências) e tira as ofertas da lista (libera): roda só ela
/// e confere o plano feito antes: devolve quantas ofertas da pessoa havia na lista.
unsafe extern "thiscall" fn comparar_reenviar_ofertas_guardadas(this: usize, id: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(971);
    let lista = cpo2_d(this + 0x38);
    if lista == 0 {
        return o(this, id);
    }
    // o plano: a original recomeça o cursor (+0xa) antes de percorrer
    cpo2_por(lista + 0xa, cpo2_d(lista + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut esperado = 0u32;
    let mut item = proximo(lista);
    while item != 0 {
        if cpo2_d(item) == id {
            esperado += 1;
        }
        item = proximo(lista);
    }
    let r = o(this, id);
    PLACAR.registrar(r == esperado, || format!("reenviar_ofertas_guardadas(id {:x}) devolveu {}, esperava {}", id, r, esperado));
    r
}

/// Comparador: pelo valor (datas e a 0x669bd0 só calculam).
unsafe extern "stdcall" fn comparar_mes_do_fim_da_temporada_do_clube(clube: usize) -> u32 {
    let r = orig::<unsafe extern "stdcall" fn(usize) -> u32>(972)(clube);
    let n = mes_do_fim_da_temporada_do_clube(clube);
    PLACAR.registrar(r == n, || format!("mes_do_fim_da_temporada_do_clube({:x}) original={} rust={}", clube, r, n));
    r
}

// ---------- 251a leva: os pontos dos técnicos: a virada do mês e do ano, o fim da temporada da competição e o técnico do mês (staff_records.cpp) ----------

unsafe fn cpo3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O registro de pontos de cada técnico (Staff_Records +8): 0xd2 bytes por pessoa que não é jogador ([0xae2368]) e
/// pelos 16 humanos. +0x86 id da pessoa (−1 vazio), +0xb6 pontos do ano, +0xba/+0xbe pontos na liga e na copa (+0xc2 e
/// +0xc6 as competições deles, −1 sem), +0xca/+0xce os do mês.
const CPO3_REGISTRO: usize = 0xd2;

unsafe fn cpo3_quantos() -> i32 {
    cpo3_i(0xae2368).wrapping_add(0x10)
}

unsafe fn cpo3_registro(this: usize, k: i32) -> usize {
    cpo3_d(this + 8).wrapping_add((k as usize).wrapping_mul(CPO3_REGISTRO))
}

/// A pessoa pelo id (registro de 0x6e bytes em [0xae23bc]; −1 dá o registro antes da tabela, como na original).
unsafe fn cpo3_pessoa(id: usize) -> usize {
    cpo3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O mês da data (0x54cfb0, o al com sinal).
unsafe fn cpo3_mes(data: usize) -> i16 {
    let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cfb0usize);
    mes(data) as u8 as i8 as i16
}

/// { 0x86dda0 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::daily_update(data)`: se o mês (0x54cfb0) ou o ano
/// (+2) da data não são os guardados (+0x14 mês, +0x16 ano), zera os pontos do mês (+0xca, +0xce) de todos os
/// registros e guarda o mês e o ano novos. No primeiro dia do ano (dia +0 = 0) passa os pontos do ano (+0xb6) de cada
/// registro com pessoa (+0x86) ao hall da fama (0x5ef270 em [0xb5ded8]) e os zera.
unsafe extern "thiscall" fn pontos_dos_tecnicos_no_dia(this: usize, data: usize) {
    if cpo3_mes(data) as i32 != cpo3_w(this + 0x14) as u16 as i32 || cpo3_w(data + 2) != cpo3_w(this + 0x16) {
        crate::comparacao::ramo("cpo_3 virada do mes");
        let base = cpo3_d(this + 8);
        let n = cpo3_quantos();
        for k in 0..n.max(0) as usize {
            let e = base.wrapping_add(k * CPO3_REGISTRO);
            cpo3_por(e + 0xca, 0);
            cpo3_por(e + 0xce, 0); // os pontos do mês
        }
        let m = cpo3_mes(data);
        core::ptr::write_unaligned((this + 0x14) as *mut i16, m);
        core::ptr::write_unaligned((this + 0x16) as *mut i16, cpo3_w(data + 2));
    }
    if cpo3_w(data) == 0 {
        let base = cpo3_d(this + 8);
        let hall: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x5ef270usize);
        let mut k = 0i32;
        while k < cpo3_quantos() {
            let e = base.wrapping_add((k as usize).wrapping_mul(CPO3_REGISTRO));
            let id = cpo3_d(e + 0x86);
            if id as i32 != -1 {
                hall(cpo3_d(0xb5ded8), cpo3_pessoa(id), cpo3_d(e + 0xb6));
            }
            cpo3_por(e + 0xb6, 0);
            k += 1;
        }
    }
}

/// { 0x86d6e0 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::end_season_for_nation(entrada)`: com a competição
/// da entrada (+0, id em +0 dela; sem competição −1; sem competição e sem +4 igual a [0x9cfa10], nada), para cada
/// registro: liga (+0xc2) da competição: soma a copa (+0xbe) se ela também é dessa competição (+0xc6 = −1, +0xbe = 0),
/// passa os pontos (+0xba) ao hall da fama (0x5ef330) se a competição guarda (+0x11c), zera e +0xc2 = −1; só a copa da
/// competição: passa +0xbe, zera e +0xc6 = −1.
unsafe extern "thiscall" fn pontos_dos_tecnicos_no_fim_da_competicao(this: usize, entrada: usize) {
    let p = cpo3_d(entrada + 4);
    let pela_global = p != 0 && cpo3_d(p) == cpo3_d(0x9cfa10);
    if !pela_global && cpo3_d(entrada) == 0 {
        return;
    }
    let comp = cpo3_d(entrada);
    let id = if comp != 0 { cpo3_i(comp) } else { -1 };
    let hall: unsafe extern "thiscall" fn(usize, usize, i32, usize) = core::mem::transmute(0x5ef330usize);
    let guarda = |c: usize| c != 0 && *((c + 0x11c) as *const u8) != 0;
    let base = cpo3_d(this + 8);
    let mut k = 0i32;
    while k < cpo3_quantos() {
        let s = base.wrapping_add((k as usize).wrapping_mul(CPO3_REGISTRO)) + 0xba;
        let copa = cpo3_i(s + 0xc);
        if cpo3_i(s + 8) == id {
            crate::comparacao::ramo("cpo_3 fim da liga");
            if copa == id {
                cpo3_por(s, cpo3_d(s).wrapping_add(cpo3_d(s + 4)));
                cpo3_por(s + 0xc, usize::MAX);
                cpo3_por(s + 4, 0);
            }
            if guarda(cpo3_d(entrada)) {
                hall(cpo3_d(0xb5ded8), cpo3_pessoa(cpo3_d(s - 0x34)), id, cpo3_d(s));
            }
            cpo3_por(s, 0);
            cpo3_por(s + 8, usize::MAX);
        } else if copa == id {
            if guarda(cpo3_d(entrada)) {
                hall(cpo3_d(0xb5ded8), cpo3_pessoa(cpo3_d(s - 0x34)), id, cpo3_d(s + 4));
            }
            cpo3_por(s + 4, 0);
            cpo3_por(s + 0xc, usize::MAX);
        }
        k += 1;
    }
}

/// { 0x86f950 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::find_manager_of_the_month(indice)`: na liga
/// ([0xadadfc] pelo índice; +0x43 tem de ser 2, senão a caixa "Error" e 0), entre os clubes da tabela (+0xb1, 0x3b
/// bytes, conta +0x3e) com técnico (+0xcf) no clube (+0x39) desde duas semanas atrás (0x54cab0 da data do jogo; ano
/// +0x40 sem sinal, dia +0x3e com sinal), o de mais pontos no mês (+0xce do registro 0x86f5b0, criado se falta), só
/// acima de 0. Devolve o técnico ou 0.
unsafe extern "thiscall" fn tecnico_do_mes_na_liga(this: usize, indice: i32) -> usize {
    let liga = cpo3_d(cpo3_d(0xadadfc).wrapping_add((indice as usize).wrapping_mul(4)));
    if *((liga + 0x43) as *const u8) != 2 {
        aviso_em(0xa83b78, 0x104e);
        return 0;
    }
    let antes: unsafe extern "thiscall" fn(usize, *mut u32, i32) -> usize = core::mem::transmute(0x54cab0usize);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
    let mut melhor_pontos = 0i32;
    let mut melhor = 0usize;
    let mut k = 0i32;
    while k < cpo3_w(liga + 0x3e) as i32 {
        let clube = cpo3_d(cpo3_d(liga + 0xb1).wrapping_add((k as usize).wrapping_mul(0x3b)));
        let tecnico = cpo3_d(clube + 0xcf);
        if tecnico != 0 && cpo3_d(tecnico + 0x39) != 0 {
            let mut d = [0u32; 2];
            let q = antes(0xae2c90, d.as_mut_ptr(), 2); // duas semanas atrás
            let lo = cpo3_d(q) as u32;
            let ano = (lo >> 16) as u16;
            let ano_tecnico = cpo3_w(tecnico + 0x40) as u16;
            let recente = ano < ano_tecnico || (ano == ano_tecnico && (lo as u16 as i16) < cpo3_w(tecnico + 0x3e));
            if recente {
                crate::comparacao::ramo("cpo_3 tecnico recente");
            } else {
                let r = registro(this, tecnico, 1);
                if r != 0 {
                    let pontos = cpo3_i(r + 0xce);
                    if pontos > melhor_pontos { // o melhor do mês
                        crate::comparacao::ramo("cpo_3 melhor do mes");
                        melhor_pontos = pontos;
                        melhor = cpo3_d(clube + 0xcf);
                    }
                }
            }
        }
        k += 1;
    }
    melhor
}

/// Comparador: os pontos vão ao hall da fama (0x5ef270, soma): roda só a original e confere o plano: na virada, os pontos
/// do mês zerados e o mês e o ano guardados; no primeiro dia do ano, os pontos do ano zerados.
unsafe extern "thiscall" fn comparar_pontos_dos_tecnicos_no_dia(this: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(973);
    let virada = cpo3_mes(data) as i32 != cpo3_w(this + 0x14) as u16 as i32 || cpo3_w(data + 2) != cpo3_w(this + 0x16);
    let ano = cpo3_w(data) == 0;
    let (m, a) = (cpo3_mes(data), cpo3_w(data + 2));
    o(this, data);
    if !virada && !ano {
        return;
    }
    let mut ok = !virada || (cpo3_w(this + 0x14) == m && cpo3_w(this + 0x16) == a);
    let mut primeiro = -1i32;
    for k in 0..cpo3_quantos().max(0) {
        let e = cpo3_registro(this, k);
        let certo = (!virada || (cpo3_d(e + 0xca) == 0 && cpo3_d(e + 0xce) == 0)) && (!ano || cpo3_d(e + 0xb6) == 0);
        if !certo && primeiro < 0 {
            primeiro = k;
        }
        ok &= certo;
    }
    PLACAR.registrar(ok, || format!("pontos_dos_tecnicos_no_dia(mês {}, ano {}) virada {} ano novo {}: registro {} não zerado", m, a, virada, ano, primeiro));
}

/// Comparador: os pontos vão ao hall da fama (0x5ef330, soma): roda só a original e confere o plano feito antes: os
/// quatro campos (+0xba, +0xbe, +0xc2, +0xc6) de cada registro.
unsafe extern "thiscall" fn comparar_pontos_dos_tecnicos_no_fim_da_competicao(this: usize, entrada: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(974);
    let p = cpo3_d(entrada + 4);
    if !(p != 0 && cpo3_d(p) == cpo3_d(0x9cfa10)) && cpo3_d(entrada) == 0 {
        return o(this, entrada);
    }
    let comp = cpo3_d(entrada);
    let id = if comp != 0 { cpo3_i(comp) } else { -1 };
    let n = cpo3_quantos().max(0);
    let mut plano = Vec::with_capacity(n as usize);
    for k in 0..n {
        let s = cpo3_registro(this, k) + 0xba;
        let (mut liga, mut copa, mut cl, mut cc) = (cpo3_d(s), cpo3_d(s + 4), cpo3_i(s + 8), cpo3_i(s + 0xc));
        if cl == id {
            if cc == id {
                cc = -1; // a copa somada à liga, que vai ao hall da fama e zera
                copa = 0;
            }
            liga = 0;
            cl = -1;
        } else if cc == id {
            copa = 0;
            cc = -1;
        }
        plano.push((liga, copa, cl, cc));
    }
    o(this, entrada);
    let errado = (0..n).find(|&k| {
        let s = cpo3_registro(this, k) + 0xba;
        plano[k as usize] != (cpo3_d(s), cpo3_d(s + 4), cpo3_i(s + 8), cpo3_i(s + 0xc))
    });
    PLACAR.registrar(errado.is_none(), || format!("pontos_dos_tecnicos_no_fim_da_competicao(competição {}) registro {:?} difere do plano", id, errado));
}

/// Comparador: pelo valor (o registro 0x86f5b0 criado pela original já existe para a cópia).
unsafe extern "thiscall" fn comparar_tecnico_do_mes_na_liga(this: usize, indice: i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(975);
    let liga = cpo3_d(cpo3_d(0xadadfc).wrapping_add((indice as usize).wrapping_mul(4)));
    if *((liga + 0x43) as *const u8) != 2 {
        return o(this, indice);
    }
    let r = o(this, indice);
    let n = tecnico_do_mes_na_liga(this, indice);
    PLACAR.registrar(r == n, || format!("tecnico_do_mes_na_liga({}) original={:x} rust={:x}", indice, r, n));
    r
}

// ---------- 251a leva: as saídas dos clubes: as saídas pendentes dos técnicos, tirar o jogador do elenco e tirar a pessoa do cargo (manager_manager.cpp) ----------

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
unsafe extern "thiscall" fn saidas_pendentes_dos_tecnicos(this: usize) {
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
            aviso_em(0x9f18a4, 0x2b70);
        } else {
            let id = cpo4_d(item + 2) as u32;
            let clube = cpo4_clube(id as usize);
            let tecnico = cpo4_d(clube + 0xcf);
            if tecnico != 0 {
                let a = (id & 0xffff_ff00) | cpo4_b(item) as u32;
                let b = (id.wrapping_mul(145) & 0xffff_ff00) | cpo4_b(item + 1) as u32;
                sair(this, clube, a, b, 0);
                if abre_vaga(clube) as u8 != 0 {
                    crate::comparacao::ramo("cpo_4 abrir a vaga");
                    vaga(this, clube, tecnico); // a vaga do clube
                }
            }
            remover(cpo4_d(this + 4), item);
        }
        k += 1;
    }
}

/// A função sem vaga no elenco depois de sair (12 → 5, 13 → 6, 15 → 8: a pessoa fica no clube) ou 0xb (sai).
fn cpo4_funcao_depois(f: u8) -> (u8, bool) {
    match f {
        0xc => (5, true),
        0xd => (6, true),
        0xf => (8, true),
        _ => (0xb, false),
    }
}

/// Tira a pessoa da primeira das 50 vagas do elenco do clube (+0xd7) em que ela está.
unsafe fn cpo4_tirar_das_50(clube: usize, pessoa: usize) {
    for k in 0..0x32usize {
        let v = clube + 0xd7 + 4 * k;
        if cpo4_d(v) == pessoa {
            crate::comparacao::ramo("cpo_4 tirar do elenco");
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
unsafe extern "thiscall" fn tirar_jogador_do_elenco_do_clube(_this: usize, pessoa: usize, clube: usize) -> usize {
    let tatica: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x896c60usize);
    if cpo4_d(pessoa + 0x39) == clube {
        let (f, fica) = cpo4_funcao_depois(cpo4_b(pessoa + 0x3d));
        *((pessoa + 0x3d) as *mut u8) = f;
        let listas: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32, i32, u32) = core::mem::transmute(0x7c5fe0usize);
        listas(cpo4_d(0xb74350), pessoa, -1, 0, 0, -1, 0);
        let midia: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x843e10usize);
        midia(cpo4_d(cpo4_d(0xde1f54) + 0x856), pessoa, clube, 4, 2);
        tatica(cpo4_d(0xde1d08), clube, pessoa, 0, 1);
        tatica(cpo4_d(0xde1d08), clube, pessoa, 1, 1);
        cpo4_tirar_das_50(clube, pessoa);
        if !fica {
            cpo4_por(pessoa + 0x39, 0);
        }
    }
    if cpo4_d(pessoa + 0x24) == clube {
        let (f, fica) = cpo4_funcao_depois(cpo4_b(pessoa + 0x28));
        *((pessoa + 0x28) as *mut u8) = f;
        tatica(cpo4_d(0xde1d08), clube, pessoa, 0, 1);
        tatica(cpo4_d(0xde1d08), clube, pessoa, 1, 1);
        cpo4_tirar_das_50(clube, pessoa);
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
unsafe fn cpo4_zerar_vagas(c: usize, base: usize, n: usize, pessoa: usize) {
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
unsafe fn cpo4_desligar(pessoa: usize, s: usize, clube: usize, c: usize) {
    if cpo4_d(s + 0x39) == clube {
        match (cpo4_b(s + 0x3d) as i8 as i32).wrapping_sub(1) as u32 {
            0 => {
                cpo4_por(c + 0xbf, 0);
                cpo4_por(s + 0x39, 0);
            }
            1 => {
                cpo4_zerar_vagas(c, 0xc3, 3, pessoa);
                cpo4_por(s + 0x39, 0);
            }
            4 => {
                if cpo4_d(c + 0xcf) == pessoa {
                    crate::comparacao::ramo("cpo_4 tirar o tecnico");
                    cpo4_por(c + 0xcf, 0); // o técnico do clube
                }
                cpo4_por(s + 0x39, 0);
            }
            5 => {
                cpo4_por(c + 0xd3, 0);
                cpo4_por(s + 0x39, 0);
            }
            7 => {
                cpo4_zerar_vagas(c, 0x19f, 5, pessoa);
                cpo4_por(s + 0x39, 0);
            }
            8 => {
                cpo4_zerar_vagas(c, 0x1b3, 7, pessoa);
                cpo4_por(s + 0x39, 0);
            }
            9 => {
                cpo4_zerar_vagas(c, 0x1cf, 3, pessoa);
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
                cpo4_zerar_vagas(c, 0x19f, 5, pessoa);
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
            1 => cpo4_zerar_vagas(c, 0xc3, 3, pessoa),
            4 => cpo4_por(c + 0xcf, 0),
            5 => cpo4_por(c + 0xd3, 0),
            7 => cpo4_zerar_vagas(c, 0x19f, 5, pessoa),
            8 => cpo4_zerar_vagas(c, 0x1b3, 7, pessoa),
            9 => cpo4_zerar_vagas(c, 0x1cf, 3, pessoa),
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
                cpo4_zerar_vagas(c, 0x19f, 5, pessoa);
                return;
            }
            _ => {}
        }
        cpo4_por(s + 0x24, 0);
    }
}

/// { 0x690890 } (thiscall, ret 8, manager_manager.cpp): `MANAGER_MANAGER::remove_staff_from_club(pessoa, clube)`: tira o
/// contrato (0x4d0b50 em [0xae1050]), tira a pessoa do cargo no clube ou na seleção (`cpo4_desligar`), da mídia
/// (0x843e10 com 4, 2) e, se ela é humana (entre as 16 últimas pessoas), das listas do humano (0x7f7270 em
/// [0xb74448]) e do clube (0x8b2470 em [0xde1f44] com 0). Devolve o eax da última chamada.
unsafe extern "thiscall" fn tirar_pessoa_do_cargo_no_clube(_this: usize, pessoa: usize, clube: usize) -> usize {
    let contrato: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4d0b50usize);
    contrato(cpo4_d(0xae1050), pessoa);
    cpo4_desligar(pessoa, pessoa, clube, clube);
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
unsafe extern "thiscall" fn comparar_saidas_pendentes_dos_tecnicos(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(976)(this)
}

/// Os campos da pessoa e do clube que a 0x690730/0x690890 gravam: (pessoa +0x24..+0x29, +0x39..+0x3e) e as vagas do
/// clube (+0xbf..+0x1db).
unsafe fn cpo4_campos(pessoa: usize, clube: usize) -> ([u8; 6], [u8; 5], [u8; 0x11c]) {
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
unsafe extern "thiscall" fn comparar_tirar_jogador_do_elenco_do_clube(this: usize, pessoa: usize, clube: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(977);
    let no_clube = cpo4_d(pessoa + 0x39) == clube;
    let na_selecao = cpo4_d(pessoa + 0x24) == clube;
    let (mut f39, mut c39) = (cpo4_b(pessoa + 0x3d), cpo4_d(pessoa + 0x39));
    let (mut f28, mut c24) = (cpo4_b(pessoa + 0x28), cpo4_d(pessoa + 0x24));
    if no_clube {
        let (f, fica) = cpo4_funcao_depois(f39);
        f39 = f;
        if !fica {
            c39 = 0;
        }
    }
    if c24 == clube {
        let (f, fica) = cpo4_funcao_depois(f28);
        f28 = f;
        if !fica {
            c24 = 0;
        }
    }
    let r = o(this, pessoa, clube);
    let fora = !(no_clube || na_selecao) || (0..0x32usize).all(|k| cpo4_d(clube + 0xd7 + 4 * k) != pessoa);
    let ok = fora && cpo4_b(pessoa + 0x3d) == f39 && cpo4_d(pessoa + 0x39) == c39 && cpo4_b(pessoa + 0x28) == f28 && cpo4_d(pessoa + 0x24) == c24;
    PLACAR.registrar(ok, || format!("tirar_jogador_do_elenco_do_clube(pessoa {:x}, clube {:x}): função {:x}/{:x} clube {:x}/{:x} (esperava {:x}/{:x} {:x}/{:x}), fora das vagas {}",
        cpo4_d(pessoa), cpo4_d(clube), cpo4_b(pessoa + 0x3d), cpo4_b(pessoa + 0x28), cpo4_d(pessoa + 0x39), cpo4_d(pessoa + 0x24), f39, f28, c39, c24, fora));
    r
}

/// Comparador: a original tira o contrato e tira a pessoa da mídia e das listas do humano (não pode rodar duas vezes):
/// roda só ela e confere o plano feito antes com o corpo (`cpo4_desligar`) sobre cópias dos campos da pessoa e das vagas
/// do clube. Pessoa humana: só a original (as listas do humano podem mexer no clube).
unsafe extern "thiscall" fn comparar_tirar_pessoa_do_cargo_no_clube(this: usize, pessoa: usize, clube: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(978);
    if pessoa == 0 || clube == 0 || cpo4_i(pessoa) >= cpo4_i(0xae2364).wrapping_sub(0x10) {
        return o(this, pessoa, clube);
    }
    // o plano: os campos em cópias com os mesmos deslocamentos
    let mut s = [0u8; 0x40];
    let mut c = [0u8; 0x1e0];
    core::ptr::copy_nonoverlapping(pessoa as *const u8, s.as_mut_ptr(), 0x40);
    core::ptr::copy_nonoverlapping(clube as *const u8, c.as_mut_ptr(), 0x1e0);
    cpo4_desligar(pessoa, s.as_mut_ptr() as usize, clube, c.as_mut_ptr() as usize);
    let esperado = cpo4_campos(s.as_ptr() as usize, c.as_ptr() as usize);
    let r = o(this, pessoa, clube);
    let depois = cpo4_campos(pessoa, clube);
    PLACAR.registrar(depois == esperado, || format!("tirar_pessoa_do_cargo_no_clube(pessoa {:x}, função {}, clube {:x}): pessoa {:x?}{:x?} esperava {:x?}{:x?}",
        cpo4_d(pessoa), cpo4_b(pessoa + 0x3d) as i8, cpo4_d(clube), depois.0, depois.1, esperado.0, esperado.1));
    r
}

// ---------- 251a leva: a notícia nova e a confiança da torcida e da diretoria depois de uma história da mídia (news.cpp, media.cpp) ----------

unsafe fn cpo5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo5_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo5_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cpo5_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// A linha do clube no gerente dos técnicos: [[0xb67a44]] + id × 0x49 (+6 confiança da diretoria, +8 da torcida, as duas
/// entre 1 e 10000; +0xe outra palavra).
unsafe fn cpo5_linha(clube: usize) -> usize {
    cpo5_d(cpo5_d(0xb67a44)).wrapping_add((cpo5_i(clube) as usize).wrapping_mul(0x49))
}

/// Os amortecedores comuns das duas confianças: com o preferido (`gosta`) a queda vale metade e a alta o dobro; com o
/// rejeitado (`nao_gosta`) o contrário; fora de ±1000, o excesso vale 0,2. Devolve a palavra baixa (o original usa `di`).
unsafe fn cpo5_amortecer(v: i32, gosta: bool, nao_gosta: bool) -> i16 {
    let mut v = v;
    if gosta {
        let x = v as i16 as f64;
        v = if (v as i16) < 0 { ftol(x * f64_em(0x968968)) } else { ftol(x + x) };
    } else if nao_gosta {
        let x = v as i16 as f64;
        v = if (v as i16) > 0 { ftol(x * f64_em(0x968968)) } else { ftol(x + x) };
    }
    let x = v as i16 as i32;
    if x < -1000 {
        v = ftol((x + 1000) as f64 * f64_em(0x968918) - f64_em(0x968908));
    } else if x > 1000 {
        v = ftol((x - 1000) as f64 * f64_em(0x968918) + f64_em(0x968908));
    }
    v as i16
}

/// A variação da confiança da torcida (0x738340): conf = +8 da linha (0 sem clube); d = alvo − trunc((conf − 5000) ×
/// −0,05) (16 bits); v = trunc(d × 0,75); o técnico (+0xcf) favorito (0x547790) ou rejeitado (0x5477d0) pelo clube
/// amortece. As duas consultas são puras.
unsafe fn cpo5_variacao_da_torcida(clube: usize, alvo: u32) -> i16 {
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547790usize);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5477d0usize);
    let conf = if clube != 0 { cpo5_w(cpo5_linha(clube) + 8) as i32 } else { 0 };
    let t = ftol((conf - 5000) as f64 * f64_em(0x96d280));
    let d = (alvo as i32).wrapping_sub(t) as i16;
    let v = ftol(d as f64 * f64_em(0x969040));
    let tecnico = cpo5_d(clube + 0xcf);
    let fav = favorito(tecnico, clube) != 0;
    let rej = !fav && rejeitado(tecnico, clube) != 0;
    cpo5_amortecer(v, fav, rej)
}

/// A variação da confiança da diretoria (0x7386f0): conf = +6 da linha (0 sem clube); s = (conf − 6000) / 20 + alvo
/// (16 bits); v = trunc(s × 0,75); o presidente (+0xbf) gosta (0x547810) ou não gosta (0x547840) do técnico (+0xcf).
unsafe fn cpo5_variacao_da_diretoria(clube: usize, alvo: u32) -> i16 {
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let conf = if clube != 0 { cpo5_w(cpo5_linha(clube) + 6) as i32 } else { 0 };
    let s = ((conf - 6000) / 20).wrapping_add(alvo as i32) as i16;
    let v = ftol(s as f64 * f64_em(0x969040));
    let (tecnico, presidente) = (cpo5_d(clube + 0xcf), cpo5_d(clube + 0xbf));
    let g = gosta(tecnico, presidente) != 0;
    let n = !g && nao_gosta(tecnico, presidente) != 0;
    cpo5_amortecer(v, g, n)
}

/// Soma `v` na palavra da linha, entre 1 e 10000.
unsafe fn cpo5_somar(p: usize, v: i16) {
    let x = cpo5_w(p).wrapping_add(v);
    core::ptr::write_unaligned(p as *mut i16, x.clamp(1, 10000));
}

/// { 0x77bbe0 } (thiscall, ret 8, news.cpp, quadro SEH só da data, destrutor vazio 0x5a53c0): `NewsItem::NewsItem(tipo,
/// importante)`: inicia a data (+0xcd, 0x54a110), grava o tipo (+0), o byte importante (+4) e zera os 50 dados (+5);
/// a data de hoje ([0xae2c90], pela atribuição 0x417c10 com retorno numa temporária), o byte [0xae2c88] em +0xd5, o id
/// (+0xd6) e +0xda zerados. Devolve a notícia.
unsafe extern "thiscall" fn construir_noticia(this: usize, tipo: u32, importante: u32) -> usize {
    let iniciar_data: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let atribuir_data: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    iniciar_data(this + 0xcd);
    cpo5_por(this, tipo as usize);
    *((this + 4) as *mut u8) = importante as u8;
    for k in 0..50 {
        cpo5_por(this + 5 + 4 * k, 0);
    }
    let mut temporaria = [0u32; 2];
    atribuir_data(this + 0xcd, temporaria.as_mut_ptr() as usize, 0xae2c90);
    *((this + 0xd5) as *mut u8) = *(0xae2c88 as *const u8);
    cpo5_por(this + 0xd6, 0);
    cpo5_por(this + 0xda, 0);
    this
}

/// Os dados do técnico (id e as três chaves +0x33 de +4, +8, +0xc) a partir de `k`.
unsafe fn cpo5_dados_da_pessoa(gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32, n: usize, k: i8, p: usize) {
    gravar(n, k, cpo5_d(p) as u32);
    gravar(n, k + 1, cpo5_d(cpo5_d(p + 4) + 0x33) as u32);
    gravar(n, k + 2, cpo5_d(cpo5_d(p + 8) + 0x33) as u32);
    gravar(n, k + 3, cpo5_d(cpo5_d(p + 0xc) + 0x33) as u32);
}

/// { 0x738340 } (thiscall, ret 0x14, media.cpp, quadro SEH sem destrutor): `Media::adjust_supporter_confidence(clube,
/// sujeito, tópico, alvo, variante)`: soma na confiança da torcida (+8 da linha do clube, entre 1 e 10000) a variação
/// (ver `cpo5_variacao_da_torcida`). Fora de ±500, a notícia 0xbe0 (local) ao técnico do clube (0x7861e0): 0 = 0, 1 =
/// tópico, 2 = variante, 3 = sorteio(4), 4 = id do clube, 5 = o grau (5 abaixo de −750, 4 negativa, 3 até 750, 2
/// acima), 6..9 o técnico; com sujeito, 10..13 o sujeito e 14 = 6 favorito, 7 rejeitado, 8 nenhum; depois −1, a
/// confiança nova e 0.
unsafe extern "thiscall" fn ajustar_confianca_da_torcida(_this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32) {
    let v = cpo5_variacao_da_torcida(clube, alvo);
    crate::comparacao::ramo("cpo_5 torcida");
    if clube != 0 {
        cpo5_somar(cpo5_linha(clube) + 8, v);
    }
    if (-500..=500).contains(&v) {
        return;
    }
    let favorito: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547790usize);
    let rejeitado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5477d0usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbe0, 0);
    gravar(n, 0, 0);
    gravar(n, 1, topico as i16 as i32 as u32);
    gravar(n, 2, variante as i8 as i32 as u32);
    let s = sorteio(4);
    gravar(n, 3, s as u32);
    gravar(n, 4, cpo5_d(clube) as u32);
    let grau = if v < -750 { 5 } else if v < 0 { 4 } else if v > 750 { 2 } else { 3 };
    gravar(n, 5, grau);
    cpo5_dados_da_pessoa(gravar, n, 6, cpo5_d(clube + 0xcf));
    let b: i8 = if sujeito != 0 {
        cpo5_dados_da_pessoa(gravar, n, 10, sujeito);
        let r = if favorito(sujeito, clube) != 0 {
            6
        } else if rejeitado(sujeito, clube) != 0 {
            7
        } else {
            8
        };
        gravar(n, 14, r);
        0xf
    } else {
        gravar(n, 10, u32::MAX);
        0xb
    };
    gravar(n, b, u32::MAX);
    let conf = if clube != 0 { cpo5_w(cpo5_linha(clube) + 8) as i32 } else { 0 };
    gravar(n, b + 1, conf as u32);
    gravar(n, b + 2, 0);
    entregar(cpo5_d(0xb7431c), n, clube);
}

/// { 0x7386f0 } (thiscall, ret 0x18, media.cpp, quadro SEH sem destrutor): `Media::adjust_board_confidence(clube,
/// sujeito, tópico, alvo, variante, sexto)`: soma na confiança da diretoria (+6 da linha do clube, entre 1 e 10000) a
/// variação (ver `cpo5_variacao_da_diretoria`). Fora de ±500, a notícia 0xbe0 (local) ao técnico do clube (0x7861e0):
/// 0 = 1, 1 = tópico, 2 = variante, 3 = sorteio(4), 4 = id do clube, 5 = o grau (5 abaixo de −750 com confiança abaixo
/// de 4500; 9 negativa com confiança acima de 6250, senão 4; 2 acima de 750 com confiança acima de 6250, senão 3), 6..9
/// o técnico; com sujeito 10..13 e 14 = 6/7/8 (o presidente gosta, não gosta, nenhum); depois o presidente (ou −1), a
/// confiança nova, o sexto (com ele 1 e a palavra +0xe), e as marcas do sujeito: contrato 0x10/0x20 acima de 25 anos,
/// média das notas acima da média da liga × 5, +0x59 e +0x5b acima de 14, mais de 1500 dias no clube, valor (0x4d0c60)
/// acima de 4.500.000 e reputação (+5 da ficha) acima de 140.
unsafe extern "thiscall" fn ajustar_confianca_da_diretoria(_this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32, sexto: u32) {
    let v = cpo5_variacao_da_diretoria(clube, alvo);
    crate::comparacao::ramo("cpo_5 diretoria");
    if clube != 0 {
        cpo5_somar(cpo5_linha(clube) + 6, v);
    }
    if (-500..=500).contains(&v) {
        return;
    }
    crate::comparacao::ramo("cpo_5 diretoria noticia");
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let media_das_notas: unsafe extern "thiscall" fn(usize, usize) -> i32 = core::mem::transmute(0x7c1950usize);
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    let valor: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d0c60usize);
    let conf = || if clube != 0 { cpo5_w(cpo5_linha(clube) + 6) } else { 0 };
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbe0, 0);
    gravar(n, 0, 1);
    gravar(n, 1, topico as i16 as i32 as u32);
    gravar(n, 2, variante as i8 as i32 as u32);
    let s = sorteio(4);
    gravar(n, 3, s as u32);
    gravar(n, 4, cpo5_d(clube) as u32);
    let grau = if v < -750 && (clube == 0 || conf() < 4500) {
        5
    } else if v < 0 {
        if clube != 0 && conf() > 6250 { 9 } else { 4 }
    } else if v > 750 && clube != 0 && conf() > 6250 {
        2
    } else {
        3
    };
    gravar(n, 5, grau);
    cpo5_dados_da_pessoa(gravar, n, 6, cpo5_d(clube + 0xcf));
    let mut b: i8 = if sujeito != 0 {
        cpo5_dados_da_pessoa(gravar, n, 10, sujeito);
        let presidente = cpo5_d(clube + 0xbf);
        let r = if gosta(sujeito, presidente) != 0 {
            6
        } else if nao_gosta(sujeito, presidente) != 0 {
            7
        } else {
            8
        };
        gravar(n, 14, r);
        0xf
    } else {
        gravar(n, 10, u32::MAX);
        0xb
    };
    let presidente = cpo5_d(clube + 0xbf);
    if presidente != 0 {
        cpo5_dados_da_pessoa(gravar, n, b, presidente);
        b += 4;
    } else {
        gravar(n, b, u32::MAX);
        b += 1;
    }
    gravar(n, b, conf() as i32 as u32);
    b += 1;
    if sexto != 0 {
        gravar(n, b, 1);
        let w = if clube != 0 { cpo5_w(cpo5_linha(clube) + 0xe) } else { 0 };
        gravar(n, b + 1, w as i32 as u32);
        b += 2;
    } else {
        gravar(n, b, 0);
        b += 1;
    }
    // as marcas do sujeito
    let mut m = 0u32;
    if sujeito != 0 && cpo5_d(sujeito + 0x61) != 0 {
        let c = contrato_de(cpo5_d(0xae1050), sujeito);
        if c != 0 && cpo5_c(sujeito + 0x18) > 0x19 {
            let t = *((c + 0x4f) as *const u8) & 0xf0;
            if t == 0x20 || t == 0x10 {
                m = 1;
            }
        }
    }
    gravar(n, b, m);
    b += 1;
    let mut m = 0u32;
    if sujeito != 0 && cpo5_d(sujeito + 0x61) != 0 {
        let base = cpo5_d(0xb74350);
        let notas = media_das_notas(base, cpo5_d(sujeito + 0x61)) as i8 as f64;
        let base = cpo5_d(0xb74350);
        let y = f64_em(base + 0x18);
        let c0 = f64_em(0x9689a0);
        let media = if !(y < c0 || y > c0) { f64_em(0x968998) } else { f64_em(base + 0x10) / y }; // sem jogos: 6,8
        if notas > media * f64_em(0x9688a0) {
            m = 1;
        }
    }
    gravar(n, b, m);
    b += 1;
    gravar(n, b, (sujeito != 0 && cpo5_c(sujeito + 0x59) > 0xe) as u32);
    b += 1;
    gravar(n, b, (sujeito != 0 && cpo5_c(sujeito + 0x5b) > 0xe) as u32);
    b += 1;
    let dias = sujeito != 0 && dias_ate(0xae2c90, cpo5_d(sujeito + 0x3e) as u32, cpo5_d(sujeito + 0x42) as u32) > 0x5dc;
    gravar(n, b, dias as u32);
    b += 1;
    let caro = sujeito != 0 && valor(cpo5_d(0xae1050), sujeito, clube, 0, u32::MAX, 0) > 0x44aa20;
    gravar(n, b, caro as u32);
    b += 1;
    let reputacao = sujeito != 0 && cpo5_d(sujeito + 0x61) != 0 && cpo5_w(cpo5_d(sujeito + 0x61) + 5) > 0x8c;
    gravar(n, b, reputacao as u32);
    entregar(cpo5_d(0xb7431c), n, clube);
}

/// Comparador: pela memória da notícia (0xde bytes; a original usa só uma temporária na pilha) e pela devolução.
unsafe extern "thiscall" fn comparar_construir_noticia(this: usize, tipo: u32, importante: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> usize>(979);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0xde)], || o(this, tipo, importante), || construir_noticia(this, tipo, importante));
    PLACAR.registrar(igual && r == n, || format!("construir_noticia(tipo {:#x}, {}) {:x?} x {:x?}", tipo, importante as u8, mo.0[0].1, mr.0[0].1));
    r
}

/// Comparador: a original sorteia e entrega a notícia (não pode rodar duas vezes): roda só ela e confere o plano feito
/// antes: a confiança da torcida nova (a variação é pura).
unsafe extern "thiscall" fn comparar_ajustar_confianca_da_torcida(this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(980);
    if clube == 0 {
        return o(this, clube, sujeito, topico, alvo, variante);
    }
    let p = cpo5_linha(clube) + 8;
    let antes = cpo5_w(p);
    let v = cpo5_variacao_da_torcida(clube, alvo);
    let esperado = antes.wrapping_add(v).clamp(1, 10000);
    o(this, clube, sujeito, topico, alvo, variante);
    let depois = cpo5_w(cpo5_linha(clube) + 8);
    PLACAR.registrar(depois == esperado, || format!("ajustar_confianca_da_torcida(clube {:x}, alvo {}) {} + {} = {} x original {}", clube, alvo as i16, antes, v, esperado, depois));
}

/// Comparador: como o da torcida, pela confiança da diretoria nova.
unsafe extern "thiscall" fn comparar_ajustar_confianca_da_diretoria(this: usize, clube: usize, sujeito: usize, topico: u32, alvo: u32, variante: u32, sexto: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32)>(981);
    if clube == 0 {
        return o(this, clube, sujeito, topico, alvo, variante, sexto);
    }
    let p = cpo5_linha(clube) + 6;
    let antes = cpo5_w(p);
    let v = cpo5_variacao_da_diretoria(clube, alvo);
    let esperado = antes.wrapping_add(v).clamp(1, 10000);
    o(this, clube, sujeito, topico, alvo, variante, sexto);
    let depois = cpo5_w(cpo5_linha(clube) + 6);
    PLACAR.registrar(depois == esperado, || format!("ajustar_confianca_da_diretoria(clube {:x}, alvo {}) {} + {} = {} x original {}", clube, alvo as i32, antes, v, esperado, depois));
}

// ---------- 251a leva: a resposta do técnico da máquina a uma história da mídia, marcar a notícia lida e achar a notícia pelo id (media.cpp, news.cpp) ----------

unsafe fn cpo6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo6_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O byte com sinal (os atributos de 1 a 20).
unsafe fn cpo6_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

/// O tipo do contrato (+0x4f, nibble alto).
unsafe fn cpo6_tipo(contrato: usize) -> u8 {
    *((contrato + 0x4f) as *const u8) & 0xf0
}

/// A notícia guardada: 0xdf bytes (+0xd6 id, +0xda lida).
const CPO6_NOTICIA: usize = 0xdf;

/// { 0x735870 } (thiscall, ret 0xc, media.cpp, 3,7 KB): `Media::pick_ai_manager_response(tecnico, jogador, tipo)`: a
/// resposta (1, 2 ou 3) do técnico da máquina a uma história do `tipo` sobre o jogador. Sem técnico, sem a ficha de
/// técnico (+0x69), jogador sem ficha (+0x61), o próprio técnico, sem clube (+0x39, senão a seleção +0x24) ou sem
/// contrato do jogador com ele (0x4cf5f0): 3. A nota é trunc(5 × 0x736720(jogador, clube, contrato, tipo, 3, 0)); sem
/// jogador, a palavra do argumento fica no lugar (o ponteiro do técnico, como no original). Depois, pelo tipo: 0x83
/// pelo tipo de contrato e o conhecimento (0x544410); 0x82 pelas preferências do técnico (0x547840/0x547810) e
/// sorteios; 0x1f pela lista de transferências (0x544520) e o contrato; os outros, antes, pelas preferências com a
/// nota, e depois por famílias de tipos, cada uma com a sua cadeia de sorteios (0x90cfc0); 8/0x25 e 3/4 podem pôr o
/// jogador entre os rejeitados do técnico (0x547af0).
unsafe extern "thiscall" fn resposta_do_tecnico_da_maquina(this: usize, tecnico: usize, jogador: usize, tipo: u32) -> u32 {
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let chance: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
    let contrato_com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let nota_da_historia: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x736720usize);
    let nao_gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547840usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547810usize);
    let rejeitar: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
    let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let listada_pelo_clube: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    let criterio: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x866f60usize);
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    if tecnico == 0 || cpo6_d(tecnico + 0x69) == 0 {
        return 3;
    }
    if jogador != 0 && cpo6_d(jogador + 0x61) == 0 {
        return 3;
    }
    if tecnico == jogador {
        return 3;
    }
    let mut clube = cpo6_d(tecnico + 0x39);
    if clube == 0 {
        clube = cpo6_d(tecnico + 0x24);
        if clube == 0 {
            return 3;
        }
    }
    let contrato = contrato_com_clube(cpo6_d(0xae1050), jogador, clube);
    if contrato == 0 {
        return 3;
    }
    let mut nota = tecnico as i32; // sem jogador: o argumento que ocupa a vaga
    if jogador != 0 {
        nota = ftol(nota_da_historia(this, jogador, clube, contrato, tipo, 3, 0.0) * *(0x968990 as *const f32) as f64);
    }
    let f = || cpo6_d(tecnico + 0x69); // a ficha de técnico
    let ch = |o: usize| chance(f(), *((f() + o) as *const i8)) as i8 as i32; // o conhecimento pelo atributo +o
    let k = tipo as i16;
    if k == 0x83 {
        let t = cpo6_tipo(contrato);
        if t == 0x10 || t == 0x20 {
            return 1;
        }
        if t == 0x30 && cpo6_c(contrato + 0x3f) > 1 {
            return 1;
        }
        if cpo6_w(cpo6_d(jogador + 0x61) + 0xb) < 0x1482 && ch(0x14) > 0xf {
            return 2;
        }
        if cpo6_tipo(contrato) == 0x70 && ch(0x14) > 0xa {
            return 2;
        }
        return if sorteio(ch(0x19)) != 0 { 1 } else { 3 };
    }
    if k == 0x82 {
        if nao_gosta(jogador, tecnico) != 0 {
            let a = cpo6_c(tecnico + 0x5b);
            if a < 0xf && sorteio(a) == 0 {
                let x = sorteio(ch(0x19));
                let y = sorteio(0x14);
                if x < y {
                    return 2;
                }
            }
            let a = cpo6_c(tecnico + 0x5b);
            if a <= 0xe || sorteio(a) <= 10 {
                return 3;
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(0x14);
            return if x <= y { 3 } else { 1 };
        }
        if gosta(jogador, tecnico) != 0 {
            let x = sorteio(cpo6_c(tecnico + 0x5b));
            let y = sorteio(10);
            if x >= y {
                return 3;
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(0xf);
            return if x <= y { 3 } else { 1 };
        }
        if cpo6_c(tecnico + 0x5d) < 6 {
            let y = sorteio(0x14);
            if cpo6_c(tecnico + 0x5d) < y {
                return 1;
            }
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        if x > y {
            return 1;
        }
        if cpo6_c(tecnico + 0x5c) <= 0xf {
            return 3;
        }
        let y = sorteio(10);
        return if cpo6_c(tecnico + 0x5c) <= y { 3 } else { 2 };
    }
    if k == 0x1f {
        if listada(jogador, clube) != 0 {
            return 2;
        }
        let b = cpo6_c(contrato + 0x3d);
        if b <= sorteio(100) {
            return 3;
        }
        if criterio(contrato) != 0 || *((contrato + 0x49) as *const u8) != 0 {
            return 3;
        }
        return if sorteio(3) != 0 { 1 } else { 3 };
    }
    // as preferências do técnico com a nota
    if nao_gosta(jogador, tecnico) != 0 && nota < 5 {
        if sorteio(cpo6_c(tecnico + 0x5b)) != 0 {
            return 3;
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        return if x >= y { 3 } else { 2 };
    }
    if gosta(jogador, tecnico) != 0 && nota > -5 {
        let x = sorteio(cpo6_c(tecnico + 0x5b));
        let y = sorteio(10);
        if x >= y {
            return 3;
        }
        let x = sorteio(ch(0x19));
        let y = sorteio(0x14);
        return if x >= y { 3 } else { 1 };
    }
    // a soma do conhecimento (+0x19), de +0x1d da ficha e de +0x59 do técnico
    let soma = || ch(0x19) + cpo6_c(f() + 0x1d) + cpo6_c(tecnico + 0x59);
    match k as i32 {
        0x3c | 0x3d => {
            let x = sorteio(ch(0x1b));
            let y = sorteio(0x17);
            if x > y {
                let x = sorteio(cpo6_c(f() + 0x1d));
                let y = sorteio(10);
                if x < y {
                    return 2;
                }
            }
            if sorteio(cpo6_c(tecnico + 0x5a)) == 0 {
                return 1;
            }
            let x = sorteio(cpo6_c(tecnico + 0x59));
            let y = sorteio(0x28);
            if x > y {
                return 1;
            }
            let x = sorteio(cpo6_c(f() + 0x1d));
            let y = sorteio(0x32);
            if x <= y { 3 } else { 1 }
        }
        0x28..=0x3b => {
            let x = sorteio(cpo6_c(jogador + 0x5b));
            let y = sorteio(0x14);
            if x > y {
                let a = sorteio(ch(0x19));
                let b = sorteio(0x14 - nota);
                if a > b {
                    return 1;
                }
                if sorteio(ch(0x19)) != 0 {
                    return 3;
                }
                return if nota >= 0xa { 3 } else { 2 };
            }
            let a = sorteio(ch(0x19));
            let b = sorteio(10);
            if a > b {
                let mut arg = 0;
                if *((cpo6_d(jogador + 0x61) + 0x44) as *const u8) != 0 {
                    // o registro de 0x4f bytes da pessoa em [0xae2d0c] (+0xa, +0xb bit 7)
                    let reg = |o: usize| *((cpo6_d(0xae2d0c).wrapping_add((cpo6_i(jogador) as usize).wrapping_mul(0x4f)) + o) as *const u8);
                    let e = sorteio((2 - (reg(0xb) >> 7) as i32) * 20);
                    if sorteio(reg(0xa) as i32) > e {
                        arg = 1;
                    }
                }
                if sorteio(arg) == 0 && nota < 0xa {
                    return 2;
                }
            }
            if sorteio(cpo6_c(cpo6_d(jogador + 0x61) + 0x45)) == 0 {
                return 1;
            }
            let x = sorteio(cpo6_c(tecnico + 0x59));
            let y = sorteio(0x14 - nota);
            if x <= y { 3 } else { 1 }
        }
        0 | 0xd | 0x11 | 0x52 | 0x23 | 0xe | 2 | 0x21 | 0x22 | 9 | 0xc => {
            crate::comparacao::ramo("cpo_6 tipos comuns");
            let t = cpo6_tipo(contrato);
            if t == 0x30 || t == 0x10 || t == 0x20 {
                let e = entrada(cpo6_d(cpo6_d(0xde1f54) + 0x856), clube, jogador, 0, 0);
                if e != 0 && cpo6_d(e + 0xf) == cpo6_d(jogador) && nota > -10 {
                    crate::comparacao::ramo("cpo_6 sorteio descartado");
                    sorteio(cpo6_c(tecnico + 0x59)); // o original descarta o resultado
                }
            }
            let x = sorteio(ch(0x19));
            let y = sorteio(10);
            if x < y {
                let x = sorteio(cpo6_c(tecnico + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(cpo6_c(f() + 0x1d));
                    let y = sorteio(10);
                    if x < y {
                        let x = sorteio(cpo6_c(tecnico + 0x59));
                        let y = sorteio(10);
                        if x < y {
                            let x = sorteio(nota);
                            let y = sorteio(-5);
                            if x < y {
                                return 2;
                            }
                        }
                    }
                }
            }
            let x = sorteio(soma());
            let y = sorteio(0x24);
            if x <= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x > y { 1 } else { 3 }
        }
        8 | 0x25 => {
            if sorteio(cpo6_c(f() + 0x1d) * 2) == 0 {
                rejeitar(tecnico, jogador, 1);
            }
            if ch(0x19) < 0xa {
                let x = sorteio(cpo6_c(tecnico + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(cpo6_c(f() + 0x1d));
                    let y = sorteio(10);
                    if x < y && sorteio(ch(0x19)) == 0 {
                        return 2;
                    }
                }
            }
            let x = sorteio(soma());
            let y = sorteio(0x24);
            if x <= y { 3 } else { 1 }
        }
        1 => {
            if sorteio(ch(0x19)) == 0 {
                let x = sorteio(cpo6_c(tecnico + 0x5d));
                let y = sorteio(10);
                if x < y {
                    let x = sorteio(cpo6_c(f() + 0x1d));
                    let y = sorteio(5);
                    if x < y && sorteio(ch(0x19)) == 0 {
                        return 2;
                    }
                }
            }
            let x = sorteio(soma());
            let y = sorteio(0x1e);
            if x <= y { 3 } else { 1 }
        }
        0x1e | 0x20 => {
            let x = sorteio(cpo6_c(tecnico + 0x5d));
            let y = sorteio(0x14);
            if x < y {
                let x = sorteio(cpo6_c(f() + 0x1d));
                let y = sorteio(0xf);
                if x < y {
                    return 2;
                }
            }
            let x = sorteio(cpo6_c(f() + 0x1d) + cpo6_c(tecnico + 0x59));
            let y = sorteio(0x96);
            if x > y {
                if cpo6_w(cpo6_d(jogador + 0x61) + 0xb) as i32 <= cpo6_w(clube + 0x80) as i32 + 0x1f4 {
                    return 2;
                }
                let x = sorteio(cpo6_c(f() + 0x14));
                let y = sorteio(5);
                return if x >= y { 3 } else { 1 };
            }
            let x = sorteio(cpo6_c(f() + 0x14));
            let y = sorteio(5);
            if x < y {
                return 1;
            }
            let x = sorteio(cpo6_c(f() + 0x14));
            let y = sorteio(0x14);
            if x <= y { 3 } else { 2 }
        }
        0x63 | 0x62 | 0x5d | 0x3f | 0x5f | 0x60 | 0x43 | 0x5c | 0x5a | 0x44 | 0x5b | 0x42 | 0x61 | 0x40 | 0x41 => {
            let mut base: i32 = match k {
                0x5d => 5,
                0x61 => 10,
                0x62 => -7,
                _ => 0,
            };
            if sorteio(cpo6_c(tecnico + 0x5a)) == 0 {
                base += sorteio(10);
                base -= sorteio(10);
            }
            let x = sorteio(cpo6_c(f() + 0x14));
            let y = sorteio(0xf - base);
            if x > y {
                return 2;
            }
            let x = sorteio(cpo6_c(tecnico + 0x57));
            let y = sorteio(base + 0xf);
            if x <= y { 3 } else { 1 }
        }
        3 | 4 => {
            let x = sorteio(cpo6_c(f() + 0x14));
            let y = sorteio(0x32);
            if x > y && sorteio(cpo6_c(f() + 0x1d)) == 0 {
                let x = sorteio(nota);
                let y = sorteio(-5);
                if x < y {
                    rejeitar(tecnico, jogador, 1);
                    return 2;
                }
            }
            let t = cpo6_tipo(contrato);
            if t != 0 && t != 0x70 && t != 0x80 && listada_pelo_clube(jogador, 0) == 0 {
                let x = sorteio(cpo6_c(tecnico + 0x59));
                let y = sorteio(6);
                if x > y {
                    let x = sorteio(ch(0x19));
                    let y = sorteio(6);
                    if x > y {
                        let x = sorteio(nota);
                        let y = sorteio(-5);
                        if x > y {
                            return 1;
                        }
                    }
                }
            }
            let t = cpo6_tipo(contrato);
            if t != 0x70 && t != 0x40 {
                return 3;
            }
            let x = sorteio(cpo6_c(tecnico + 0x59));
            let y = sorteio(5);
            if x >= y {
                return 3;
            }
            let x = sorteio(cpo6_c(tecnico + 0x5b));
            let y = sorteio(5);
            if x >= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        5 => {
            if sorteio(cpo6_c(tecnico + 0x59)) != 0 && sorteio(ch(0x19)) != 0 && sorteio(cpo6_c(jogador + 0x5b)) != 0 {
                let x = sorteio(nota);
                let y = sorteio(-5);
                if x > y {
                    return 1;
                }
            }
            if sorteio(ch(0x19)) != 0 {
                return 3;
            }
            let x = sorteio(cpo6_c(jogador + 0x5a));
            let y = sorteio(10);
            if x >= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        6 => {
            if sorteio(cpo6_c(tecnico + 0x59)) != 0 {
                let x = sorteio(cpo6_c(f() + 0x1f));
                let y = sorteio(10);
                if x > y {
                    let x = sorteio(cpo6_c(jogador + 0x5b));
                    let y = sorteio(5);
                    if x > y {
                        let x = sorteio(cpo6_c(cpo6_d(jogador + 0x61) + 0x25));
                        let y = sorteio(0x1e);
                        if x < y {
                            let x = sorteio(nota);
                            let y = sorteio(-5);
                            if x > y {
                                return 1;
                            }
                        }
                    }
                }
            }
            if sorteio(ch(0x19)) != 0 || sorteio(cpo6_c(f() + 0x1f)) != 0 {
                return 3;
            }
            let x = sorteio(cpo6_c(cpo6_d(jogador + 0x61) + 0x25));
            let y = sorteio(0x32);
            if x <= y {
                return 3;
            }
            let x = sorteio(nota);
            let y = sorteio(-5);
            if x >= y { 3 } else { 2 }
        }
        _ => 3,
    }
}

/// { 0x787140 } (thiscall, ret 0xc, news.cpp): `News::mark_item_read(tecnico, id, sincronizar)`: no anel de 100 vagas
/// do humano (News +8, pelo id − [0xae2364] + 16), a primeira vaga com o `id` ainda não lida (+0xda = 0) cujo botão
/// (0x782b70 em 0xde1f64) não pede resposta (2) fica lida e sai da conta (+8 da entrada). Sem anel: 0. Senão (mesmo
/// sem achar), com `sincronizar`, repete (sem sincronizar) para os outros humanos (+0x5f = 1) da mesma máquina da rede
/// (0x7fffa0) que o humano da vez; devolve 1. Sem técnico, sem id ou técnico da máquina: a caixa "Error" e 0.
unsafe extern "thiscall" fn marcar_noticia_lida(this: usize, tecnico: usize, id: u32, sincronizar: u32) -> u32 {
    if tecnico == 0 {
        aviso_em(0xa693ec, 0x2358);
        return 0;
    }
    if id == 0 {
        aviso_em(0xa693ec, 0x2360);
        return 0;
    }
    let idx = cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10);
    if idx < 0 {
        aviso_em(0xa693ec, 0x236b);
        return 0;
    }
    let e = cpo6_d(this + 8) + idx as usize * 12;
    if cpo6_i(e) == -1 {
        return 0;
    }
    let botao: unsafe extern "thiscall" fn(usize, usize, usize, u32, *mut u32) -> u8 = core::mem::transmute(0x782b70usize);
    let mut v = 0usize;
    for k in 0..0x64usize {
        v = cpo6_d(this).wrapping_add((cpo6_d(e).wrapping_add(k)).wrapping_mul(CPO6_NOTICIA));
        if cpo6_d(v + 0xd6) as u32 == id && cpo6_d(v + 0xda) == 0 {
            let mut tratador = 0u32;
            if botao(v, tecnico, 0xde1f64, 0x7d0, &mut tratador) != 2 {
                crate::comparacao::ramo("cpo_6 noticia lida");
                core::ptr::write_unaligned((v + 0xda) as *mut u32, 1);
                core::ptr::write_unaligned((e + 8) as *mut i32, cpo6_i(e + 8) - 1);
                break;
            }
        }
    }
    if v == 0 {
        return 0;
    }
    if sincronizar != 0 {
        let maquina: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7fffa0usize);
        let marcar: unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32 = core::mem::transmute(0x787140usize);
        let da_vez = cpo6_d(0xb74c26 + 0x300 * *(0xb77c7a as *const i16) as isize as usize);
        let m = maquina(0xb74450, da_vez) as u16;
        for j in 0..0x10i32 {
            let p = cpo6_d(0xae23bc).wrapping_add((cpo6_i(0xae2364) - j - 1).wrapping_mul(0x6e) as usize);
            if *((p + 0x5f) as *const u8) == 1 && p != tecnico && maquina(0xb74450, p) as u16 == m {
                marcar(this, p, id, 0);
            }
        }
    }
    1
}

/// { 0x786b70 } (thiscall, ret 0xc, news.cpp): `News::find_item_by_id(tecnico, id, saida)`: copia (0x786010) em `saida`
/// a notícia com o `id`: com técnico, entre as 100 vagas do anel dele (sem anel: 0); sem técnico, entre todas as vagas
/// (News +4). Achou: 1; senão 0. Sem id, sem saída ou técnico da máquina: a caixa "Error" e 0.
unsafe extern "thiscall" fn achar_noticia_pelo_id(this: usize, tecnico: usize, id: u32, saida: usize) -> u32 {
    if id == 0 {
        aviso_em(0xa693ec, 0x2252);
        return 0;
    }
    if saida == 0 {
        aviso_em(0xa693ec, 0x225a);
        return 0;
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x786010usize);
    let (inicio, n) = if tecnico == 0 {
        (0usize, cpo6_i(this + 4).max(0) as usize)
    } else {
        let idx = cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10);
        if idx < 0 {
            aviso_em(0xa693ec, 0x2268);
            return 0;
        }
        let ini = cpo6_i(cpo6_d(this + 8) + idx as usize * 12);
        if ini == -1 {
            return 0;
        }
        (ini as usize, 0x64)
    };
    for k in 0..n {
        let v = cpo6_d(this).wrapping_add(inicio.wrapping_add(k).wrapping_mul(CPO6_NOTICIA));
        if cpo6_d(v + 0xd6) as u32 == id {
            copiar(saida, v);
            return 1;
        }
    }
    0
}

/// Comparador: a resposta sorteia e pode pôr o jogador entre os rejeitados do técnico (0x547af0): lado a lado no
/// estado do gerador e no bloco das preferências do técnico (+0x65, +0x1c..+0x34), a cópia sem contar os sorteios.
unsafe extern "thiscall" fn comparar_resposta_do_tecnico_da_maquina(this: usize, tecnico: usize, jogador: usize, tipo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32>(982);
    let mut regioes = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    if tecnico != 0 && cpo6_d(tecnico + 0x65) != 0 {
        regioes.push((cpo6_d(tecnico + 0x65) + 0x1c, 0x18));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &regioes,
        || o(this, tecnico, jogador, tipo),
        || crate::aleatorio::sem_contar(|| resposta_do_tecnico_da_maquina(this, tecnico, jogador, tipo)),
    );
    PLACAR.registrar(igual && r == n, || format!("resposta_do_tecnico_da_maquina(tipo {:#x}) original={} rust={} {:x?} x {:x?}", tipo as i16, r, n, mo.0, mr.0));
    r
}

/// Comparador: lado a lado no vetor das notícias (News +0, +4 vagas), nas entradas dos 16 humanos (News +8) e no texto
/// do botão (0xde1f64; a 0x782b70 só consulta). Caixa "Error": só a original.
unsafe extern "thiscall" fn comparar_marcar_noticia_lida(this: usize, tecnico: usize, id: u32, sincronizar: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(983);
    if tecnico == 0 || id == 0 || cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10) < 0 || cpo6_d(this) == 0 {
        return o(this, tecnico, id, sincronizar);
    }
    let regioes = [
        (cpo6_d(this), cpo6_i(this + 4).max(0) as usize * CPO6_NOTICIA),
        (cpo6_d(this + 8), 0x10 * 12),
        (0xde1f64, 0x7d0),
    ];
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&regioes, || o(this, tecnico, id, sincronizar), || marcar_noticia_lida(this, tecnico, id, sincronizar));
    PLACAR.registrar(igual && r == n, || format!("marcar_noticia_lida(id {}, sincronizar {}) original={} rust={} memoria igual {}", id, sincronizar, r, n, igual));
    r
}

/// Comparador: pela notícia copiada em `saida` (0xdf bytes) e pela devolução. Caixa "Error": só a original.
unsafe extern "thiscall" fn comparar_achar_noticia_pelo_id(this: usize, tecnico: usize, id: u32, saida: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(984);
    if id == 0 || saida == 0 || (tecnico != 0 && cpo6_i(tecnico).wrapping_sub(cpo6_i(0xae2364)).wrapping_add(0x10) < 0) {
        return o(this, tecnico, id, saida);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(saida, CPO6_NOTICIA)], || o(this, tecnico, id, saida), || achar_noticia_pelo_id(this, tecnico, id, saida));
    PLACAR.registrar(igual && r == n, || format!("achar_noticia_pelo_id(id {}) original={} rust={} {:x?} x {:x?}", id, r, n, mo.0[0].1, mr.0[0].1));
    r
}

// ---------- 251a leva: as notícias do jogador depois de uma história da mídia: gratidão ao técnico, pedido de apoio e reação à crítica (media.cpp) ----------

unsafe fn cpo7_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O byte com sinal (os atributos de 1 a 20, a idade).
unsafe fn cpo7_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

type Cpo7Gravar = unsafe extern "thiscall" fn(usize, i8, u32) -> u32;

/// Os dados da pessoa a partir de `k`: id e as três chaves +0x33 de +4, +8 e +0xc (a de +0xc pode faltar: −1, só
/// quando `opcional`).
unsafe fn cpo7_dados(gravar: Cpo7Gravar, n: usize, k: i8, p: usize, opcional: bool) {
    gravar(n, k, cpo7_d(p) as u32);
    gravar(n, k + 1, cpo7_d(cpo7_d(p + 4) + 0x33) as u32);
    gravar(n, k + 2, cpo7_d(cpo7_d(p + 8) + 0x33) as u32);
    let q = cpo7_d(p + 0xc);
    gravar(n, k + 3, if opcional && q == 0 { u32::MAX } else { cpo7_d(q + 0x33) as u32 });
}

/// A dupla de transmutes comum: construir a notícia (0x77bbe0) e gravar um dado (0x785790).
unsafe fn cpo7_noticia() -> (unsafe extern "thiscall" fn(usize, u32, u32) -> usize, Cpo7Gravar) {
    (core::mem::transmute(0x77bbe0usize), core::mem::transmute(0x785790usize))
}

/// Os dias desde a chegada da pessoa (+0x3e/+0x42) até hoje (0x54cdb0 em 0xae2c90).
unsafe fn cpo7_dias(p: usize) -> i32 {
    let dias_ate: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    dias_ate(0xae2c90, cpo7_d(p + 0x3e) as u32, cpo7_d(p + 0x42) as u32)
}

/// O sorteio da gratidão pelo tipo (índices da tabela de bytes 0x732fcc, saltos 0x732fb4): 0 os tipos 0, 6, 14 e 17; 1
/// os 2 e 30 (sorteio(2)); 2 os 4, 9, 33, 34 e 37 (sorteio(3)); 3 os 5 e 7 (sorteio(4)); o resto 0.
const CPO7_CASO: [u8; 0x26] = [0, 5, 1, 5, 2, 3, 0, 3, 5, 2, 5, 5, 5, 4, 0, 5, 5, 0, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 1, 5, 5, 2, 2, 4, 5, 2];

/// { 0x732c80 } (thiscall, ret 0x10, media.cpp, quadro SEH sem destrutor): `Media::queue_player_gratitude_news(jogador,
/// clube, tipo, variante)`: com jogador com ficha (+0x61), clube com técnico (+0xcf) e o contrato dele com o clube
/// (0x4cf5f0), a notícia 0xbda (local) ao técnico (0x7861e0): 0..3 o jogador (3 = −1 sem +0xc), 4 = id do clube, 5 =
/// o tipo, 6 = a variante, 7 = o sorteio do tipo, 8..0xb o técnico, 0xc = 0x420e9, 0xd..0x10 o auxiliar (+0xd3) ou −1.
/// O tipo: se o jogador está na lista da mídia (this +0/+8, 0x17 bytes, id em +8 e marca 2 em +0x10) ou o contrato tem
/// 0x2000 ou 8 em +0x45, os tipos 0, 2, 4, 7, 9, 0xe e 0x23 viram 0x12 (+0x5d ≥ sorteio(10)) ou 0x21; lesionado
/// (0x62ed70): 0xc. Nos tipos 0, 6, 14 e 17: com +0x44 da ficha abaixo de 10, sorteio(3) + 2; senão sorteio(5), e 0 ou 1
/// sobe +0x44 da ficha (até 20).
unsafe extern "thiscall" fn noticia_de_gratidao_do_jogador(this: usize, jogador: usize, clube: usize, tipo: u32, variante: u32) {
    if jogador == 0 || cpo7_d(jogador + 0x61) == 0 || clube == 0 || cpo7_d(clube + 0xcf) == 0 {
        return;
    }
    let contrato_com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let contrato = contrato_com_clube(cpo7_d(0xae1050), jogador, clube);
    if contrato == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 gratidao");
    let (construir, gravar) = cpo7_noticia();
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let lesionado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x62ed70usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbda, 0);
    cpo7_dados(gravar, n, 0, jogador, true);
    gravar(n, 4, cpo7_d(clube) as u32);
    // a lista da mídia (this +0, conta em +8)
    let mut na_lista = false;
    for i in 0..core::ptr::read_unaligned((this + 8) as *const i32).max(0) as usize {
        let e = cpo7_d(this) + i * 0x17;
        if cpo7_d(e + 8) == cpo7_d(jogador) && *((e + 0x10) as *const u8) == 2 {
            na_lista = true;
            break;
        }
    }
    let f45 = cpo7_d(contrato + 0x45);
    let mut t = tipo;
    if na_lista || f45 & 0x2000 != 0 || f45 & 8 != 0 {
        if [0xe, 0, 2, 9, 0x23, 4, 7].contains(&(tipo as i16)) {
            crate::comparacao::ramo("cpo_7 gratidao tipo");
            let s = sorteio(10);
            t = if cpo7_c(jogador + 0x5d) >= s { 0x12 } else { 0x21 };
        }
    }
    if lesionado(cpo7_d(0xb63d28), jogador) != 0 {
        t = 0xc;
    }
    let t = t as i16 as i32;
    gravar(n, 5, t as u32);
    gravar(n, 6, variante as i8 as i32 as u32);
    let v = match if (t as u32) <= 0x25 { CPO7_CASO[t as usize] } else { 5 } {
        0 => {
            let ficha = cpo7_d(jogador + 0x61);
            if ficha != 0 && cpo7_c(ficha + 0x44) < 10 {
                (sorteio(3) as u8).wrapping_add(2) as i8 as i32
            } else {
                let s = sorteio(5);
                if s == 0 || s == 1 {
                    let ficha = cpo7_d(jogador + 0x61);
                    if ficha != 0 && cpo7_c(ficha + 0x44) < 0x14 {
                        crate::comparacao::ramo("cpo_7 gratidao sobe");
                        *((ficha + 0x44) as *mut u8) += 1;
                    }
                }
                s
            }
        }
        1 => sorteio(2),
        2 => {
            crate::comparacao::ramo("cpo_7 gratidao sorteio 3");
            sorteio(3)
        }
        3 => sorteio(4),
        _ => 0,
    };
    gravar(n, 7, v as u32);
    let tecnico = cpo7_d(clube + 0xcf);
    cpo7_dados(gravar, n, 8, tecnico, false);
    gravar(n, 0xc, 0x420e9);
    let auxiliar = cpo7_d(clube + 0xd3);
    if auxiliar != 0 {
        cpo7_dados(gravar, n, 0xd, auxiliar, false);
    } else {
        gravar(n, 0xd, u32::MAX);
    }
    entregar(cpo7_d(0xb7431c), n, clube);
}

/// { 0x743370 } (thiscall, ret 0x10, media.cpp, quadro SEH sem destrutor): `Media::post_player_backing_request_news(
/// jogador, clube, tipo, quarto)`: com jogador com ficha (+0x61) e clube humano (0x600a50), a notícia 0xbcf (local) ao
/// técnico do clube (0x7861e0): 0..3 o jogador (3 = −1 sem +0xc), 4 = id do clube, 5/6 = (1, ·) com reputação (+0xb da
/// ficha) acima de 4250, senão (2, ·): o segundo 1 se (+0x57 > 15 e +0x5d < 6, ou +0x5b < 10 e +0x59 < 6 na faixa de
/// baixo), 3 se o tipo é 0x20, 1 ou 0x1e com +0x59 > 15, +0x5b > 10 e mais de 210 dias no clube, senão 2; 7 = tipo, 8 =
/// quarto, 9 = listado para transferência (0x544520).
unsafe extern "thiscall" fn noticia_de_pedido_de_apoio(_this: usize, jogador: usize, clube: usize, tipo: u32, quarto: u32) {
    if jogador == 0 || cpo7_d(jogador + 0x61) == 0 || clube == 0 {
        return;
    }
    let clube_humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if clube_humano(clube, 0, 1, 0, 0) == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 pedido de apoio");
    let (construir, gravar) = cpo7_noticia();
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544520usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbcf, 0);
    cpo7_dados(gravar, n, 0, jogador, true);
    gravar(n, 4, cpo7_d(clube) as u32);
    let t = tipo as u8;
    let alvo = t == 0x20 || t == 1 || t == 0x1e;
    let (a, b): (i32, i32) = if cpo7_w(cpo7_d(jogador + 0x61) + 0xb) > 0x109a {
        if cpo7_c(jogador + 0x57) > 0xf && cpo7_c(jogador + 0x5d) < 6 {
            (1, 1)
        } else if alvo && cpo7_c(jogador + 0x59) > 0xf && cpo7_c(jogador + 0x5b) > 0xa && cpo7_dias(jogador) > 0xd2 {
            (1, 3)
        } else {
            (1, 2)
        }
    } else if cpo7_c(jogador + 0x5b) < 0xa && cpo7_c(jogador + 0x59) < 6 {
        (2, 1)
    } else if alvo && cpo7_c(jogador + 0x59) > 0xf && cpo7_c(jogador + 0x5b) > 0xa && cpo7_dias(jogador) > 0xd2 {
        (2, 3)
    } else {
        (2, 2)
    };
    gravar(n, 5, a as u32);
    gravar(n, 6, b as u32);
    gravar(n, 7, t as i8 as i32 as u32);
    gravar(n, 8, quarto as i16 as i32 as u32);
    gravar(n, 9, (listado(jogador, 0) != 0) as u32);
    entregar(cpo7_d(0xb7431c), n, clube);
}

/// { 0x743a50 } (thiscall, ret 0x10, media.cpp, quadro SEH sem destrutor): `Media::post_player_criticism_reaction_news(
/// jogador, clube, tópico, resposta)`: com jogador com ficha, clube com técnico e o contrato (0x4cf5f0), a notícia 0xbdc
/// (local): 0..3 o jogador, 4 = id do clube, 5 = a reação (com reputação acima de 4250 e da do clube, categoria
/// 0x4ca470 1 ou 2 e mais de 22 anos: 1 ou 3 se +0x59 < 10, +0x57 > 15 e +0x5d < 6; 2 ou 4 com mais de 1000 dias, +0x59
/// > 15 e sem 0x181 em +0x45 do contrato; o maior quando o contrato tem 0x2000 ou 8, +0x5b < 10, +0x5d > 10, +0x59 < 15,
/// mais de 23 anos e a resposta é 2; senão 0), 6 = resposta, 7 = confiança da diretoria abaixo de 4000, 8 = tópico,
/// 9..0xc o técnico. Vai aos técnicos dos clubes que a 0x8d1da0 ([0xde1f54], 0x21a, até 30) devolve.
unsafe extern "thiscall" fn noticia_de_reacao_a_critica(_this: usize, jogador: usize, clube: usize, topico: u32, resposta: u32) {
    if jogador == 0 || cpo7_d(jogador + 0x61) == 0 || clube == 0 || cpo7_d(clube + 0xcf) == 0 {
        return;
    }
    let contrato_com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let contrato = contrato_com_clube(cpo7_d(0xae1050), jogador, clube);
    if contrato == 0 {
        return;
    }
    crate::comparacao::ramo("cpo_7 reacao a critica");
    let (construir, gravar) = cpo7_noticia();
    let categoria: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4ca470usize);
    let clubes_interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbdc, 0);
    let f45 = cpo7_d(contrato + 0x45);
    let marca = (f45 & 0x2000 != 0 || f45 & 8 != 0)
        && cpo7_c(jogador + 0x5b) < 0xa
        && cpo7_c(jogador + 0x5d) > 0xa
        && cpo7_c(jogador + 0x59) < 0xf
        && cpo7_c(jogador + 0x18) > 0x17;
    cpo7_dados(gravar, n, 0, jogador, true);
    gravar(n, 4, cpo7_d(clube) as u32);
    let r = resposta as u8;
    let rep = cpo7_w(cpo7_d(jogador + 0x61) + 0xb);
    let mut reacao = 0i8;
    if rep > 0x109a && rep > cpo7_w(clube + 0x80) {
        let gerente = cpo7_d(0xae1050);
        let cat_ok = categoria(gerente, jogador, contrato) == 1 || categoria(gerente, jogador, contrato) == 2;
        if cat_ok && cpo7_c(jogador + 0x18) > 0x16 {
            if cpo7_c(jogador + 0x59) < 0xa && cpo7_c(jogador + 0x57) > 0xf && cpo7_c(jogador + 0x5d) < 6 {
                reacao = if marca && r == 2 { 3 } else { 1 };
            } else if cpo7_dias(jogador) > 0x3e8 && cpo7_c(jogador + 0x59) > 0xf && cpo7_d(contrato + 0x45) & 0x181 == 0 {
                reacao = if marca && r == 2 { 4 } else { 2 };
            }
        }
    }
    gravar(n, 5, reacao as i32 as u32);
    gravar(n, 6, r as i8 as i32 as u32);
    let linha = cpo7_d(cpo7_d(0xb67a44)).wrapping_add((cpo7_d(clube)).wrapping_mul(0x49));
    gravar(n, 7, (cpo7_w(linha + 6) < 0xfa0) as u32); // confiança da diretoria abaixo de 4000
    gravar(n, 8, topico);
    cpo7_dados(gravar, n, 9, cpo7_d(clube + 0xcf), false);
    let mut clubes = [0usize; 0x1e];
    let tm = cpo7_d(0xde1f54);
    let k = if tm != 0 { clubes_interessados(tm, jogador, 0x21a, clubes.as_mut_ptr(), 0x1e) as i8 } else { 0 };
    for i in 0..k.max(0) as usize {
        entregar(cpo7_d(0xb7431c), n, clubes[i]);
    }
}

/// Comparador: a gratidão sorteia, sobe +0x44 da ficha e entrega a notícia: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_noticia_de_gratidao_do_jogador(this: usize, jogador: usize, clube: usize, tipo: u32, variante: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(985)(this, jogador, clube, tipo, variante)
}

/// Comparador: a notícia vai à caixa do técnico (não pode rodar duas vezes): só a original.
unsafe extern "thiscall" fn comparar_noticia_de_pedido_de_apoio(this: usize, jogador: usize, clube: usize, tipo: u32, quarto: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(986)(this, jogador, clube, tipo, quarto)
}

/// Comparador: a notícia vai às caixas dos técnicos (não pode rodar duas vezes): só a original.
unsafe extern "thiscall" fn comparar_noticia_de_reacao_a_critica(this: usize, jogador: usize, clube: usize, topico: u32, resposta: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(987)(this, jogador, clube, topico, resposta)
}

// ---------- 251a leva: as listas de posição da lista de observação: montar as listas, ordená-las a cada dia e os clubes interessados no jogador (shortlist_manager.cpp) ----------

unsafe fn cpo8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo8_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpo8_w(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpo8_clube(id: usize) -> usize {
    cpo8_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpo8_pessoa(id: usize) -> usize {
    cpo8_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0, guardado em [0xae1080]), o clube dele (id em +4) e o contrato com esse
/// clube (0x4cf5f0).
unsafe fn cpo8_contrato_com_o_clube(p: usize) -> usize {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let c = contrato_de(cpo8_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let clube = if c != 0 { cpo8_clube(cpo8_d(c + 4)) } else { 0 };
    com_clube(cpo8_d(0xae1050), p, clube)
}

/// { 0x839e60 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::build_position_lists(não usado)`: libera
/// os 32 vetores das listas de posição (+0x18: 16 pela pessoa, +0x58: 16 pela chave do contrato); o tamanho deles
/// (+0xd8) é o maior número de pessoas (todas, humanos inclusive) que servem para uma das 16 vagas (0x844300); aloca os
/// 32 com 12 bytes por entrada (sem memória: a caixa "Error" e o jogo para) e zera as contas (+0x98). Depois, cada
/// pessoa fora dos 16 humanos, em cada vaga que serve (conta no limite: a caixa e o jogo para): as duas entradas com o
/// id dela (+0), o id da nação (+4; sem nação em +0x1a, a 0x90cfc0 sorteia uma e a grava na pessoa), na primeira o valor
/// (+0x52) e na segunda o +0x3d do contrato com o clube do contrato (sem contrato: 100). Roda a cada 30 dias.
unsafe extern "thiscall" fn montar_listas_de_posicao(this: usize, _nao_usado: u32) {
    let livrar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    // ponytail: a original empurra a vaga num dword com 3 bytes de lixo da pilha acima; a 0x844300 só lê o byte
    let serve: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(0x844300usize);
    crate::comparacao::ramo("cpo_8 montar listas");
    for k in 0..32usize {
        let v = this + 0x18 + k * 4;
        if cpo8_d(v) != 0 {
            livrar(cpo8_d(v));
            cpo8_w(v, 0);
        }
    }
    cpo8_w(this + 0xd8, 0);
    for vaga in 0..16u32 {
        let base = cpo8_d(0xae23bc);
        let mut n = 0i32;
        let mut i = 0usize;
        while (i as i32) < cpo8_i(0xae2364) {
            if serve(base.wrapping_add(i * 0x6e), vaga) != 0 {
                n += 1;
            }
            i += 1;
        }
        if n > cpo8_i(this + 0xd8) {
            cpo8_w(this + 0xd8, n as u32);
        }
    }
    for k in 0..32usize {
        let v = alocar(cpo8_d(this + 0xd8).wrapping_mul(12) as u32);
        cpo8_w(this + 0x18 + k * 4, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x10c);
        }
    }
    for k in 0..16usize {
        cpo8_w(this + 0x98 + k * 4, 0);
    }
    let base = cpo8_d(0xae23bc);
    let mut i = 0usize;
    while (i as i32) < cpo8_i(0xae2364).wrapping_sub(0x10) {
        let p = base.wrapping_add(i * 0x6e);
        for vaga in 0..16u32 {
            if serve(p, vaga) == 0 {
                continue;
            }
            let pv = vaga as usize;
            let conta = cpo8_i(this + 0x98 + pv * 4);
            if conta >= cpo8_i(this + 0xd8) {
                erro_em(0xa825a4, 0x121);
            }
            let o = (conta as usize).wrapping_mul(12);
            let (l1, l2) = (cpo8_d(this + 0x18 + pv * 4).wrapping_add(o), cpo8_d(this + 0x58 + pv * 4).wrapping_add(o));
            cpo8_w(l1, i as u32);
            cpo8_w(l2, i as u32);
            let mut nacao = cpo8_d(p + 0x1a);
            if nacao == 0 {
                let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
                nacao = cpo8_d(0xae23a8).wrapping_add((sortear(cpo8_i(0xae2350)) as usize).wrapping_mul(0x122));
                cpo8_w(p + 0x1a, nacao as u32);
            }
            cpo8_w(l1 + 4, cpo8_d(nacao) as u32);
            cpo8_w(l2 + 4, cpo8_d(cpo8_d(p + 0x1a)) as u32);
            cpo8_w(l1 + 8, cpo8_d(p + 0x52) as u32);
            let k = cpo8_contrato_com_o_clube(p);
            cpo8_w(l2 + 8, if k != 0 { cpo8_b(k + 0x3d) as i8 as i32 as u32 } else { 100 });
            cpo8_w(this + 0x98 + pv * 4, (conta + 1) as u32);
        }
        i += 1;
    }
}

/// { 0x83a780 } (thiscall, ret 0, shortlist_manager.cpp): `Shortlist_Manager::sort_position_lists()`: a cada dia,
/// refaz a chave (+8) das entradas das 16 vagas e ordena os dois vetores de cada uma (0x945333 com 0x83b670: pela chave
/// e pelo +4). Na primeira lista: o valor da pessoa (+0x52) se o contrato com o clube existe e a faixa do vencimento
/// (0x867d30) é 0, senão 0; o jogador de clube humano (0x600a50) na lista de transferências (0x5444a0) perde 1.500.000,
/// no mínimo um terço do valor. Na vaga 15 só a primeira, com o valor. Na segunda: −1000 com o bit 2 do +0x4e do
/// contrato, 2000 com os bits 0 ou 3 ou sem jogador (+0x61), sem clube, sem divisão (+0x57) ou divisão sem marca em
/// [0xadadfc] com reputação abaixo de 6000, ou sem contrato; senão o +0x3d do contrato.
unsafe extern "thiscall" fn ordenar_listas_de_posicao(this: usize) {
    let faixa: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x867d30usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    for vaga in 0..16usize {
        let mut j = 0i32;
        let mut o = 0usize;
        while j < cpo8_i(this + 0x98 + vaga * 4) {
            let p = cpo8_pessoa(cpo8_d(cpo8_d(this + 0x18 + vaga * 4) + o));
            let k = cpo8_contrato_com_o_clube(p);
            if vaga == 15 {
                cpo8_w(cpo8_d(this + 0x54) + o + 8, cpo8_d(p + 0x52) as u32);
                j += 1;
                o += 12;
                continue;
            }
            if k != 0 && faixa(k) == 0 {
                cpo8_w(cpo8_d(this + 0x18 + vaga * 4) + o + 8, cpo8_d(p + 0x52) as u32);
                let clube = cpo8_d(p + 0x39);
                if clube != 0 && humano(clube, 0, 1, 0, 0) != 0 && listado(p, 0) != 0 {
                    let e = cpo8_d(this + 0x18 + vaga * 4) + o + 8;
                    cpo8_w(e, cpo8_i(e).wrapping_add(-0x16e360) as u32);
                    let terco = cpo8_i(p + 0x52) / 3;
                    if cpo8_i(e) < terco {
                        cpo8_w(e, terco as u32);
                    }
                }
            } else {
                if k != 0 {
                    crate::comparacao::ramo("cpo_8 faixa do contrato");
                }
                cpo8_w(cpo8_d(this + 0x18 + vaga * 4) + o + 8, 0);
            }
            // a segunda lista
            let p2 = cpo8_pessoa(cpo8_d(cpo8_d(this + 0x58 + vaga * 4) + o));
            let k2 = cpo8_contrato_com_o_clube(p2);
            let clube = cpo8_d(p2 + 0x39);
            let chave: i32 = if cpo8_d(p2 + 0x61) == 0 || clube == 0 || cpo8_d(clube + 0x57) == 0 {
                0x7d0
            } else if cpo8_d(cpo8_d(0xadadfc) + cpo8_d(cpo8_d(clube + 0x57)).wrapping_mul(4)) == 0
                && core::ptr::read_unaligned((clube + 0x80) as *const i16) < 0x1770
            {
                0x7d0
            } else if k2 == 0 {
                0x7d0
            } else if cpo8_b(k2 + 0x4e) & 2 != 0 {
                -1000
            } else if cpo8_b(k2 + 0x4e) & 9 != 0 {
                0x7d0
            } else {
                cpo8_b(k2 + 0x3d) as i8 as i32
            };
            cpo8_w(cpo8_d(this + 0x58 + vaga * 4) + o + 8, chave as u32);
            j += 1;
            o += 12;
        }
        let n = cpo8_d(this + 0x98 + vaga * 4) as u32;
        ordenar(cpo8_d(this + 0x18 + vaga * 4), n, 12, 0x83b670);
        let n = cpo8_d(this + 0x98 + vaga * 4) as u32;
        ordenar(cpo8_d(this + 0x58 + vaga * 4), n, 12, 0x83b670);
    }
}

/// { 0x842180 } (thiscall, ret 0x14, shortlist_manager.cpp): `Shortlist_Manager::collect_interested_clubs(jogador,
/// saída, ignorar, modo, só_humanos)`: esvazia a lista `saída` (0x68d080) e acrescenta (0x68d0d0, 8 bytes: id do
/// jogador, id do clube) os clubes interessados: com modo ≠ 3, os 16 humanos (do último para o primeiro) com +0x5f e
/// clube que têm o jogador na lista do olheiro (0x7f7490; com modo 2, fora os clubes que o humano da vez controla,
/// 0x600c90); sem `só_humanos`, os pares (jogador, clube) do vetor pela pessoa (+0, conta +0x10; 0x946464 até o primeiro
/// do jogador) cujo clube não está suspenso (+8 da entrada de 9 bytes do clube em +0xdc abaixo de 100, salvo com
/// `ignorar`), não é o do jogador e põe nele (+7 da vaga de 0x25 bytes com o jogador em +0xf; sem vaga, a primeira)
/// mais que o valor (+0x52).
unsafe extern "thiscall" fn clubes_interessados_no_jogador(this: usize, jogador: usize, saida: usize, ignorar: u32, modo: u32, so_humanos: u32) {
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    if saida != 0 {
        let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
        limpar(saida);
    }
    if modo as u8 != 3 {
        let controla: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x600c90usize);
        let na_lista: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7f7490usize);
        for k in 0..16i32 {
            let h = cpo8_pessoa(cpo8_i(0xae2364).wrapping_sub(k).wrapping_sub(1) as usize);
            let clube = cpo8_d(h + 0x39);
            if cpo8_b(h + 0x5f) == 0 || clube == 0 {
                continue;
            }
            if controla(clube, 0, 1) != 0 && modo as u8 == 2 {
                continue;
            }
            if na_lista(cpo8_d(0xb74448), jogador, h) != 0 {
                let par = [cpo8_d(jogador) as u32, cpo8_d(clube) as u32];
                acrescentar(saida, par.as_ptr() as usize, 8);
            }
        }
    }
    if so_humanos != 0 {
        return;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = [cpo8_d(jogador) as u32, 0xffff_ffff];
    let mut e = bsearch(chave.as_ptr(), cpo8_d(this), cpo8_i(this + 0x10), 8, 0x757ea0);
    if e == 0 {
        return;
    }
    let base = cpo8_d(this);
    let ultimo = base.wrapping_add((cpo8_i(this + 0x10) as usize).wrapping_mul(8)).wrapping_sub(8);
    let id = cpo8_d(jogador);
    if e > base {
        loop {
            let x = cpo8_d(e - 8);
            e -= 8;
            if x != id {
                e += 8;
                break;
            }
            if e <= base {
                break;
            }
        }
    }
    if cpo8_d(e) != id {
        return;
    }
    loop {
        let clube_id = cpo8_d(e + 4);
        let entrada = cpo8_d(this + 0xdc).wrapping_add(clube_id.wrapping_mul(9));
        let vagas = cpo8_d(entrada);
        let mut k = 0usize;
        while k < 0x28 && cpo8_d(vagas + 0xf + k * 0x25) != cpo8_d(jogador) {
            k += 1;
        }
        let valor = if k < 0x28 { cpo8_i(vagas + k * 0x25 + 7) } else { cpo8_i(vagas + 7) };
        if !(ignorar == 0 && cpo8_b(entrada + 8) >= 100) && valor > cpo8_i(jogador + 0x52) {
            let c = cpo8_d(jogador + 0x39);
            if !(c != 0 && cpo8_d(c) == clube_id) {
                crate::comparacao::ramo("cpo_8 clube interessado");
                acrescentar(saida, e, 8);
            }
        }
        if e == ultimo {
            return;
        }
        let prox = cpo8_d(e + 8);
        e += 8;
        if prox != cpo8_d(jogador) {
            return;
        }
    }
}

/// Os itens (8 bytes) de uma LLIST (+2 o primeiro nó; nó: +0 o dado, +6 o próximo).
unsafe fn cpo8_itens(lista: usize) -> Vec<(u32, u32)> {
    let mut v = Vec::new();
    let mut no = cpo8_d(lista + 2);
    while no != 0 {
        let x = cpo8_d(no);
        v.push((cpo8_d(x) as u32, cpo8_d(x + 4) as u32));
        no = cpo8_d(no + 6);
    }
    v
}

/// Comparador: só a original.
// ponytail: a original solta e aloca os 32 vetores (e pode sortear a nação de quem não tem); não roda duas vezes. Confere
// o resultado: as duas listas de cada vaga com os mesmos ids, cada entrada servindo para a vaga, a conta no limite.
unsafe extern "thiscall" fn comparar_montar_listas_de_posicao(this: usize, nao_usado: u32) {
    orig::<unsafe extern "thiscall" fn(usize, u32)>(988)(this, nao_usado);
    let serve: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(0x844300usize);
    let mut ruim = None;
    'f: for vaga in 0..16usize {
        let n = cpo8_i(this + 0x98 + vaga * 4);
        if n > cpo8_i(this + 0xd8) {
            ruim = Some((vaga, -1));
            break;
        }
        for j in 0..n.max(0) as usize {
            let (a, b) = (cpo8_d(this + 0x18 + vaga * 4) + j * 12, cpo8_d(this + 0x58 + vaga * 4) + j * 12);
            if cpo8_d(a) != cpo8_d(b) || serve(cpo8_pessoa(cpo8_d(a)), vaga as u32) == 0 {
                ruim = Some((vaga, j as i32));
                break 'f;
            }
        }
    }
    PLACAR.registrar(ruim.is_none(), || format!("montar_listas_de_posicao: vaga e entrada {:?}", ruim));
}

/// Comparador: lado a lado pelos 32 vetores (12 bytes por entrada) e por [0xae1080].
unsafe extern "thiscall" fn comparar_ordenar_listas_de_posicao(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(989);
    let mut reg = vec![(0xae1080usize, 4usize)];
    for vaga in 0..16usize {
        let n = cpo8_i(this + 0x98 + vaga * 4);
        if n > 0 {
            reg.push((cpo8_d(this + 0x18 + vaga * 4), n as usize * 12));
            reg.push((cpo8_d(this + 0x58 + vaga * 4), n as usize * 12));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || ordenar_listas_de_posicao(this));
    PLACAR.registrar(igual, || {
        let dif: Vec<usize> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).map(|(a, _)| a.0).take(8).collect();
        format!("ordenar_listas_de_posicao: vetores diferentes {:x?}", dif)
    });
}

/// Comparador: a original na lista de saída e a cópia numa lista nova (0x68d020, solta com 0x68d080); compara os pares.
unsafe extern "thiscall" fn comparar_clubes_interessados_no_jogador(this: usize, jogador: usize, saida: usize, ignorar: u32, modo: u32, so_humanos: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(990);
    o(this, jogador, saida, ignorar, modo, so_humanos);
    if saida == 0 {
        return;
    }
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let mut nova = [0u8; 0x10];
    let l = nova.as_mut_ptr() as usize;
    criar(l);
    clubes_interessados_no_jogador(this, jogador, l, ignorar, modo, so_humanos);
    let (a, b) = (cpo8_itens(saida), cpo8_itens(l));
    limpar(l);
    PLACAR.registrar(a == b, || format!("clubes_interessados_no_jogador({:#x}) original={:x?} rust={:x?}", jogador, a, b));
}

// ---------- 251a leva: a lista de observação e o empréstimo: acrescentar o par na lista, prever a resposta do clube vendedor e o período do empréstimo (shortlist_manager.cpp, transfer_manager.cpp) ----------

unsafe fn cpo9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo9_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo9_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpo9_w(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// Acrescenta o par (id da pessoa, id do clube) no fim do vetor `[this + vet]` (conta em `this + conta`) e o ordena
/// (0x945333 com `cmp`).
unsafe fn cpo9_inserir(this: usize, vet: usize, conta: usize, pessoa: usize, clube: usize, cmp: usize) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let n = cpo9_i(this + conta);
    let e = cpo9_d(this + vet).wrapping_add((n as usize).wrapping_mul(8));
    cpo9_w(e, cpo9_d(pessoa) as u32);
    cpo9_w(e + 4, cpo9_d(clube) as u32);
    cpo9_w(this + conta, (n + 1) as u32);
    ordenar(cpo9_d(this + vet), (n + 1) as u32, 8, cmp);
}

/// { 0x8423f0 } (thiscall, ret 8, shortlist_manager.cpp): `Shortlist_Manager::add_shortlist_entry(pessoa, clube)`: cria
/// os dois vetores de pares na primeira vez (0x944e46 de 0x320 bytes, capacidade +8/+0xc = 100; sem memória: a caixa
/// "Error" e o jogo para); com o par (pessoa, clube; sem clube −1) já no vetor pela pessoa (0x946464 com 0x757ea0), nada.
/// Senão cresce de 100 em 100 o vetor cheio (0x945667), acrescenta o par nos dois e os ordena (pela pessoa com
/// 0x757ea0, pelo clube com 0x844740) e, fora de clube humano (0x600a50), soma 1 no nibble baixo do +0xb do registro de
/// 79 bytes da pessoa ([0xae2d0c]: quantas listas a têm).
unsafe extern "thiscall" fn acrescentar_na_lista_de_observacao(this: usize, pessoa: usize, clube: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
    if cpo9_d(this) == 0 {
        let v = alocar(0x320);
        cpo9_w(this, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x1594);
        }
        cpo9_w(this + 8, 100);
    }
    if cpo9_d(this + 4) == 0 {
        let v = alocar(0x320);
        cpo9_w(this + 4, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x159d);
        }
        cpo9_w(this + 0xc, 100);
    }
    if cpo9_d(this) != 0 {
        let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
        let chave = [cpo9_d(pessoa) as u32, if clube != 0 { cpo9_d(clube) as u32 } else { 0xffff_ffff }];
        if bsearch(chave.as_ptr(), cpo9_d(this), cpo9_i(this + 0x10), 8, 0x757ea0) != 0 {
            return;
        }
    }
    crate::comparacao::ramo("cpo_9 nova entrada");
    if cpo9_i(this + 8) == cpo9_i(this + 0x10) {
        let v = realocar(cpo9_d(this), (cpo9_i(this + 8) as u32).wrapping_mul(8).wrapping_add(0x320));
        cpo9_w(this, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x15b5);
        }
        cpo9_w(this + 8, cpo9_i(this + 8).wrapping_add(100) as u32);
    }
    if cpo9_i(this + 0xc) == cpo9_i(this + 0x14) {
        let v = realocar(cpo9_d(this + 4), (cpo9_i(this + 0xc) as u32).wrapping_mul(8).wrapping_add(0x320));
        cpo9_w(this + 4, v as u32);
        if v == 0 {
            erro_em(0xa825a4, 0x15be);
        }
        cpo9_w(this + 0xc, cpo9_i(this + 0xc).wrapping_add(100) as u32);
    }
    cpo9_inserir(this, 0, 0x10, pessoa, clube, 0x757ea0); // o vetor pela pessoa
    cpo9_inserir(this, 4, 0x14, pessoa, clube, 0x844740); // o vetor pelo clube
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 0, 1, 0, 0) == 0 {
        let r = (cpo9_d(0xae2d0c).wrapping_add(cpo9_d(pessoa).wrapping_mul(0x4f)) + 0xb) as *mut u8;
        let a = *r;
        *r = (a & 0xf0) | (a.wrapping_add(1) & 0xf);
    }
}

/// { 0x84af00 } (stdcall, ret 0x20, shortlist_manager.cpp): `shortlist_manager_predict_selling_club_response(vendedor,
/// jogador, oferta, comprador, pedido, empréstimo)`: 0 aceita, 1 negocia, 2 recusa. Sem vendedor ou com o contrato com
/// ele (0x4cf5f0) marcado em +0x49: 0; o técnico do vendedor é o jogador: 2; sem contrato de tipo 2 (+0x35): 2 no
/// empréstimo, 0 senão. Empréstimo: clube humano (0x600a50): 0; listado para empréstimo (0x544590) com a reputação do
/// comprador × [0x968928] acima do +0xb da ficha: 0; estatuto 6/7 (+0x4f) ou +0xf da ficha 20, com a reputação acima de
/// +0xb × [0x969040], +0x3d do contrato abaixo de −50 e o +0x1db do vendedor diferente de [0x9cf6e8]: 0; senão 2.
/// Compra: o preço (humano: 0x4d42d0 × [0x9689b0], sem cláusula 0x867b50 e com o técnico de férias 0x5fd210 a opção 0
/// 0x5fea50: 1 recusa, 2 o valor +0x52; senão o `pedido` não negativo, o valor sem ficha ou 0x4d1600); oferta no preço:
/// 0; humano com a oferta acima do preço − 1.500.000 e listado (0x5444a0): 1; preço × [0x96f9c0] acima da oferta: 2; 1.
unsafe extern "stdcall" fn prever_resposta_do_vendedor(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, emprestimo: u32) -> u8 {
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let k = com_clube(cpo9_d(0xae1050), jogador, vendedor);
    if vendedor == 0 || (k != 0 && cpo9_b(k + 0x49) != 0) {
        return 0;
    }
    if cpo9_d(vendedor + 0xcf) == jogador {
        return 2;
    }
    if k == 0 || cpo9_b(k + 0x35) & 0x3f != 2 {
        return if emprestimo as u8 != 0 { 2 } else { 0 };
    }
    if emprestimo as u8 != 0 {
        if humano(vendedor, 0, 1, 0, 0) != 0 {
            return 0;
        }
        let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x544590usize);
        let rep = cpo9_h(comprador + 0x80) as f64;
        if listado(jogador, 0) != 0 && rep * f64_em(0x968928) > cpo9_h(cpo9_d(jogador + 0x61) + 0xb) as f64 {
            return 0;
        }
        let st = cpo9_b(k + 0x4f) & 0xf0;
        if st == 0x60 || st == 0x70 || cpo9_b(cpo9_d(jogador + 0x61) + 0xf) == 0x14 {
            let v = cpo9_h(cpo9_d(jogador + 0x61) + 0xb) as f64 * f64_em(0x969040);
            if rep > v && (cpo9_b(k + 0x3d) as i8) < -50 && cpo9_d(vendedor + 0x1db) != cpo9_d(0x9cf6e8) {
                return 0;
            }
        }
        return 2;
    }
    let preco: i32 = if humano(vendedor, 0, 1, 0, 0) != 0 {
        let pedido_humano: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> i32 = core::mem::transmute(0x4d42d0usize);
        let mut v = ftol(pedido_humano(cpo9_d(0xae1050), vendedor, comprador, jogador) as f64 * f64_em(0x9689b0));
        let clausula: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x867b50usize);
        if clausula(k) == 0 {
            let tecnico: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53ca90usize);
            let ferias: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
            if ferias(cpo9_d(0xb63c98), tecnico(vendedor)) != 0 {
                let opcao: unsafe extern "thiscall" fn(usize, u32, usize) -> u8 = core::mem::transmute(0x5fea50usize);
                match opcao(cpo9_d(0xb63c98), 0, tecnico(vendedor)) as i8 {
                    1 => return 2,
                    2 => v = cpo9_i(jogador + 0x52),
                    _ => {}
                }
            }
        }
        v
    } else if pedido >= 0 {
        pedido as i32
    } else if cpo9_d(jogador + 0x61) == 0 {
        cpo9_i(jogador + 0x52)
    } else {
        let calcular: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> i32 = core::mem::transmute(0x4d1600usize);
        calcular(cpo9_d(0xae1050), vendedor, comprador, jogador, 0, 1, 1)
    };
    if oferta >= preco as i64 {
        return 0;
    }
    let listado: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
    if humano(vendedor, 0, 1, 0, 0) != 0 && (preco.wrapping_sub(0x16e360) as i64) < oferta && listado(jogador, 0) != 0 {
        return 1;
    }
    if preco as f64 * f64_em(0x96f9c0) > oferta as f64 {
        return 2;
    }
    crate::comparacao::ramo("cpo_9 negocia");
    1
}

/// O ano da data padrão (0x54a110: dia 40 de 1972).
unsafe fn cpo9_ano_padrao() -> u16 {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mut data = [0u8; 8];
    let p = padrao(data.as_mut_ptr() as usize);
    cpo9_h(p + 2) as u16
}

/// A data (dia do ano +0, ano +2) passa do fim do contrato `c` (+0x2d dia, +0x2f ano)?
unsafe fn cpo9_depois_do_contrato(dia: i16, ano: u16, c: usize) -> bool {
    let fim = cpo9_h(c + 0x2f) as u16;
    ano > fim || (ano == fim && dia > cpo9_h(c + 0x2d))
}

/// { 0x8d0590 } (thiscall, ret 0x1c, transfer_manager.cpp): `TRANSFER_MANAGER::get_loan_period(clube, a2, jogador,
/// início_mês, início_ano, fim_mês, fim_ano)`: as regras da nação do clube (+0x53; das 28 em +0x8ac, a primeira sem
/// achar) dão o período (método +0x28). Sem contrato de tipo 2 (0x4cf5a0), com o contrato já vencido hoje ou o fim mais
/// de 5 anos à frente: o fim vira o ano da data padrão (1972). Senão, enquanto a data (dia do mês de hoje, fim_mês,
/// fim_ano; 0x549f70) passar do fim do contrato, recua um mês (o início acompanha se ficar depois do fim); fim antes do
/// ano de hoje: o ano padrão.
unsafe extern "thiscall" fn periodo_do_emprestimo(this: usize, clube: usize, a2: u32, jogador: usize, ini_mes: *mut i8, ini_ano: *mut u16, mes: *mut i8, ano: *mut u16) {
    let nacao = cpo9_d(clube + 0x53);
    let mut regra = cpo9_d(this + 0x8ac);
    for k in 0..0x1cusize {
        let r = cpo9_d(this + 0x8ac + k * 4);
        if cpo9_d(r + 0xa) == nacao {
            if r != 0 {
                regra = r;
            }
            break;
        }
    }
    let periodo: unsafe extern "thiscall" fn(usize, usize, u32, *mut i8, *mut u16, *mut i8, *mut u16, usize) = core::mem::transmute(cpo9_d(cpo9_d(regra) + 0x28));
    periodo(regra, clube, a2, ini_mes, ini_ano, mes, ano, jogador);
    let dia_do_mes: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54cf70usize);
    let dia = dia_do_mes(0xae2c90);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contrato_de(cpo9_d(0xae1050), jogador);
    if c == 0 || cpo9_b(c + 0x35) & 0x3f != 2 || cpo9_depois_do_contrato(cpo9_h(0xae2c90), cpo9_h(0xae2c92) as u16, c) {
        *ano = cpo9_ano_padrao();
        return;
    }
    if *ano as i32 > cpo9_h(0xae2c92) as u16 as i32 + 5 {
        *ano = cpo9_ano_padrao();
        return;
    }
    // ponytail: a original empurra o mês e o ano em registradores com lixo acima; a 0x549f70 lê o byte e a palavra
    let data_de: unsafe extern "thiscall" fn(usize, i32, i32, u32, i32) -> usize = core::mem::transmute(0x549f70usize);
    loop {
        let mut data = [0u8; 8];
        let d = data_de(data.as_mut_ptr() as usize, dia, *mes as i32, *ano as u32, -1);
        if !cpo9_depois_do_contrato(cpo9_h(d), cpo9_h(d + 2) as u16, c) {
            return;
        }
        crate::comparacao::ramo("cpo_9 recua o mes");
        let m = (*mes).wrapping_sub(1);
        *mes = m;
        if m < 0 {
            *ano = (*ano).wrapping_sub(1);
            *mes = 11;
        }
        if *ini_ano > *ano || (*ini_ano == *ano && *ini_mes > *mes) {
            *ini_mes = *mes;
            *ini_ano = *ano;
        }
        if *ano < cpo9_h(0xae2c92) as u16 {
            *ano = cpo9_ano_padrao();
            return;
        }
    }
}

/// Comparador: só a original, conferindo o plano (feito antes): o par já estava (nada muda) ou entra nos dois vetores
/// (contas + 1, o par achado depois) e o nibble da pessoa soma 1 fora de clube humano.
// ponytail: a original realoca os vetores (0x945667) e os ordena; não roda duas vezes.
unsafe extern "thiscall" fn comparar_acrescentar_na_lista_de_observacao(this: usize, pessoa: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(991);
    let bsearch: unsafe extern "C" fn(*const u32, usize, i32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let chave = [cpo9_d(pessoa) as u32, if clube != 0 { cpo9_d(clube) as u32 } else { 0xffff_ffff }];
    let tinha = cpo9_d(this) != 0 && bsearch(chave.as_ptr(), cpo9_d(this), cpo9_i(this + 0x10), 8, 0x757ea0) != 0;
    let (n0, n1) = if cpo9_d(this) != 0 { (cpo9_i(this + 0x10), cpo9_i(this + 0x14)) } else { (0, 0) };
    let r = cpo9_d(0xae2d0c).wrapping_add(cpo9_d(pessoa).wrapping_mul(0x4f)) + 0xb;
    let nib = cpo9_b(r);
    let soma = !tinha && humano(clube, 0, 1, 0, 0) == 0;
    o(this, pessoa, clube);
    let esperado = if soma { (nib & 0xf0) | (nib.wrapping_add(1) & 0xf) } else { nib };
    let (d0, d1) = (if tinha { n0 } else { n0 + 1 }, if tinha { n1 } else { n1 + 1 });
    let achado = bsearch(chave.as_ptr(), cpo9_d(this), cpo9_i(this + 0x10), 8, 0x757ea0) != 0;
    let ok = cpo9_i(this + 0x10) == d0 && cpo9_i(this + 0x14) == d1 && cpo9_b(r) == esperado && (achado || clube == 0);
    PLACAR.registrar(ok, || {
        format!("acrescentar_na_lista_de_observacao({:#x}, {:#x}) tinha={} contas {}/{} -> {}/{} nibble {:#x} -> {:#x} (esperado {:#x})",
            pessoa, clube, tinha, n0, n1, cpo9_i(this + 0x10), cpo9_i(this + 0x14), nib, cpo9_b(r), esperado)
    });
}

/// Comparador: pelo valor (o byte baixo) e lado a lado pelo sorteio e [0xae1080] (as chamadas de preço podem sortear).
unsafe extern "stdcall" fn comparar_prever_resposta_do_vendedor(vendedor: usize, jogador: usize, oferta: i64, comprador: usize, pedido: i64, emprestimo: u32) -> u8 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, i64, usize, i64, u32) -> u8>(992);
    let mut reg = vec![(0xae1080usize, 4usize)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(vendedor, jogador, oferta, comprador, pedido, emprestimo),
        || crate::aleatorio::sem_contar(|| prever_resposta_do_vendedor(vendedor, jogador, oferta, comprador, pedido, emprestimo)),
    );
    PLACAR.registrar(igual && r == n, || format!("prever_resposta_do_vendedor({:#x}, {:#x}, {}, {:#x}, {}, {}) original={} rust={} {:x?} x {:x?}", vendedor, jogador, oferta, comprador, pedido, emprestimo as u8, r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelas quatro saídas, o sorteio e [0xae1080].
unsafe extern "thiscall" fn comparar_periodo_do_emprestimo(this: usize, clube: usize, a2: u32, jogador: usize, ini_mes: *mut i8, ini_ano: *mut u16, mes: *mut i8, ano: *mut u16) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, *mut i8, *mut u16, *mut i8, *mut u16)>(993);
    let mut reg = vec![(ini_mes as usize, 1usize), (ini_ano as usize, 2), (mes as usize, 1), (ano as usize, 2), (0xae1080, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(this, clube, a2, jogador, ini_mes, ini_ano, mes, ano),
        || crate::aleatorio::sem_contar(|| periodo_do_emprestimo(this, clube, a2, jogador, ini_mes, ini_ano, mes, ano)),
    );
    PLACAR.registrar(igual, || format!("periodo_do_emprestimo({:#x}, {:#x}) {:x?} x {:x?}", clube, jogador, mo, mr));
}

// ---------- 251a leva: as transferências: o prazo da resposta à oferta, pagar as prestações do mês e tirar a oferta (transfer_manager.cpp) ----------

unsafe fn cpo10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpo10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpo10_h(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo10_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpo10_w(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpo10_clube(id: usize) -> usize {
    cpo10_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpo10_pessoa(id: usize) -> usize {
    cpo10_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// A regra de transferência da nação `nacao` entre as 28 de `this + 0x8ac` (a de +0xa igual), ou 0.
unsafe fn cpo10_regra(this: usize, nacao: usize) -> usize {
    for k in 0..0x1cusize {
        let r = cpo10_d(this + 0x8ac + k * 4);
        if cpo10_d(r + 0xa) == nacao {
            return r;
        }
    }
    0
}

/// { 0x8cc5b0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::get_offer_response_delay(oferta, dias,
/// fatias)`: o prazo é o de fechar a janela (0x8e8ed0) da nação do comprador (+8; sem regra: 365) e, com o +4 da oferta,
/// o menor com o da nação do clube do contrato do jogador (+0xc; contrato de tipo 5 não conta). Abaixo de 3: 0 dia e 1
/// fatia; abaixo de 6 ou de 11: 1 ou 2 dias e 1..3 fatias sorteadas. Senão o valor pelo estatuto do contrato com o clube
/// (+0x3d; sem ele −100): o jogador (0x53b620 do +0x11; sem ficha, a caixa "Error") de habilidade +7 da ficha acima de
/// 130 e da reputação do clube / 60, menos de 26 anos, em menos de 3 listas e estatuto positivo, com o interesse
/// (0x846920) acima de [0x968e58]: o prazo × [0x968e18] até [0x968dd8]; senão um sorteio até estatuto/10 + 12 (o maior
/// com 1..3). Dias = valor / 3, fatias = valor % 3.
unsafe extern "thiscall" fn prazo_da_resposta_a_oferta(this: usize, oferta: usize, dias: *mut u8, fatias: *mut u8) {
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let com_clube: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let janela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8e8ed0usize);
    let sortear: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let comprador = cpo10_clube(cpo10_d(oferta + 8));
    let p = cpo10_pessoa(cpo10_d(oferta + 0xc));
    let c = contrato_de(cpo10_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let dono = if c != 0 && cpo10_b(c + 0x35) & 0x3f != 5 { cpo10_clube(cpo10_d(c + 4)) } else { 0 };
    let k = com_clube(cpo10_d(0xae1050), p, dono);
    let mut prazo: u16 = 0x16d;
    let r = cpo10_regra(this, cpo10_d(comprador + 0x53));
    if r != 0 {
        prazo = janela(r) as u16;
    }
    if cpo10_d(oferta + 4) != 0 {
        let r = cpo10_regra(this, cpo10_d(dono + 0x53));
        if r != 0 && prazo >= janela(r) as u16 {
            prazo = janela(r) as u16;
        }
    }
    if prazo < 3 {
        *dias = 0;
        *fatias = 1;
        return;
    }
    if prazo < 0xb {
        *dias = if prazo < 6 { 1 } else { 2 };
        *fatias = (sortear(3) as u8).wrapping_add(1);
        return;
    }
    let estatuto: i8 = if k != 0 { cpo10_b(k + 0x3d) as i8 } else { -100 };
    // ponytail: a original empurra o cargo num registrador com lixo acima; a 0x53b620 só lê o byte
    let e_jogador: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x53b620usize);
    if e_jogador(cpo10_b(oferta + 0x11) as u32) != 0 && cpo10_d(p + 0x61) == 0 {
        aviso_em(0xa8d514, 0x29c3);
    }
    let mut valor: Option<u8> = None;
    if dono != 0 && e_jogador(cpo10_b(oferta + 0x11) as u32) != 0 {
        let hab = cpo10_h(cpo10_d(p + 0x61) + 7);
        if hab > 0x82
            && hab as i32 > cpo10_h(dono + 0x80) as i32 / 60
            && (cpo10_b(p + 0x18) as i8) < 0x1a
            && cpo10_b(cpo10_d(0xae2d0c).wrapping_add(cpo10_d(p).wrapping_mul(0x4f)) + 0xb) & 0xf < 3
            && estatuto > 0
        {
            let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(0x846920usize);
            if interesse(cpo10_d(this + 0x856), p, comprador, 0, 0, 1, 0) > f64_em(0x968e58) {
                let x = prazo as i32 as f64 * f64_em(0x968e18);
                let teto = f64_em(0x968dd8);
                valor = Some(ftol(if teto < x || x.is_nan() { teto } else { x }) as u8);
            }
        }
    }
    let v = match valor {
        Some(v) => v,
        None => {
            crate::comparacao::ramo("cpo_10 prazo sorteado");
            let s = sortear(estatuto as i32 / 10 + 0xc) as i8;
            if sortear(3) + 1 > s as i32 {
                (sortear(3) as u8).wrapping_add(1)
            } else {
                s as u8
            }
        }
    } as i8 as i32;
    *dias = (v / 3) as u8;
    *fatias = (v % 3) as u8;
}

/// { 0x8caee0 } (thiscall, ret 0, transfer_manager.cpp): `TRANSFER_MANAGER::pay_transfer_instalments()`: no fim do mês,
/// cada prestação da lista (+0x872; 13 bytes: +0 o valor, +4 o clube que paga, +8 o que recebe, +0xc as que faltam) é
/// paga (0x599890 nas finanças do pagador, 0x599800 nas do recebedor, 0x5a0590) e conta uma a menos; faltando menos de
/// 4 ou de 8, a figura das prestações (+0x1c das finanças) recebe as frações ([0x968918]/[0x968f30] ou
/// [0x967880]/[0x969518]); acabada, sai da lista (0x68d1f0); senão os dois clubes ficam marcados (0x945501). Depois, em
/// cada clube com finanças, a figura zera: sem marca; positiva só pagando; negativa só recebendo.
unsafe extern "thiscall" fn pagar_prestacoes_das_transferencias(this: usize) {
    let calloc: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let livrar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let pagar: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599890usize);
    let receber: unsafe extern "thiscall" fn(usize, i64) = core::mem::transmute(0x599800usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let recebe = calloc(cpo10_d(0xae235c) as u32, 1);
    let paga = calloc(cpo10_d(0xae235c) as u32, 1);
    let lista = this + 0x872;
    cpo10_w(lista + 0xa, cpo10_d(this + 0x874) as u32);
    loop {
        let e = proximo(lista);
        if e == 0 {
            break;
        }
        let fr = financas(cpo10_d(0xae2a38), cpo10_clube(cpo10_d(e + 8)));
        let fp = financas(cpo10_d(0xae2a38), cpo10_clube(cpo10_d(e + 4)));
        pagar(fp, cpo10_i(e) as i64);
        receber(fr, cpo10_i(e) as i64);
        let n = (cpo10_b(e + 0xc) as i8).wrapping_sub(1);
        *((e + 0xc) as *mut i8) = n;
        let fracoes = if n < 4 { Some((0x968918, 0x968f30)) } else if n < 8 { Some((0x967880, 0x969518)) } else { None };
        if let Some((a, b)) = fracoes {
            cpo10_w(fp + 0x1c, cpo10_i(fp + 0x1c).wrapping_add(ftol(cpo10_i(e) as f64 * f64_em(a))) as u32);
            cpo10_w(fr + 0x1c, cpo10_i(fr + 0x1c).wrapping_add(ftol(cpo10_i(e) as f64 * f64_em(b))) as u32);
        }
        if cpo10_b(e + 0xc) == 0 {
            crate::comparacao::ramo("cpo_10 prestacao acabada");
            tirar(lista, e);
        } else {
            *((recebe + cpo10_d(cpo10_clube(cpo10_d(e + 8)))) as *mut u8) = 1;
            *((paga + cpo10_d(cpo10_clube(cpo10_d(e + 4)))) as *mut u8) = 1;
        }
    }
    let mut i = 0usize;
    while (i as i32) < cpo10_i(0xae235c) {
        let f = financas(cpo10_d(0xae2a38), cpo10_d(0xae23b4).wrapping_add(i * 0x245));
        if f != 0 {
            let (r, p) = (cpo10_b(recebe + i) != 0, cpo10_b(paga + i) != 0);
            let v = cpo10_i(f + 0x1c);
            let zerar = match (r, p) {
                (false, false) => true,
                (false, true) => v > 0,
                (true, false) => v < 0,
                (true, true) => false,
            };
            if zerar {
                cpo10_w(f + 0x1c, 0);
            }
        }
        i += 1;
    }
    if recebe != 0 {
        livrar(recebe);
    }
    if paga != 0 {
        livrar(paga);
    }
}

/// { 0x8d0f60 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::remove_offer(oferta)`: a oferta ligada
/// (+0x28 ≠ −1, em +0xc) é destruída (0x8e5710, 0x944c94), a vaga zera e a conta +0x14 baixa; com o +0x1f em 1, sai da
/// lista de empréstimos (+0x946) o par (clube +8, jogador +0xc); para cada jogador da troca (+4): a proposta de contrato
/// ligada à vaga +0x20 sai (0x4ccb30, se o jogador da oferta tem contrato) e o +2 do registro de 8 bytes dele (+0x86e)
/// baixa; sai a proposta de contrato do jogador com o comprador (0x4cca70), o +1 do registro do jogador baixa, a menor
/// vaga livre (+0x85e) acompanha, a vaga +0x20 do vetor (+0) zera, a oferta é destruída e a conta +8 baixa.
unsafe extern "thiscall" fn tirar_oferta_de_transferencia(this: usize, oferta: usize) {
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x8e5710usize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    crate::comparacao::ramo("cpo_10 tirar oferta");
    if cpo10_i(oferta + 0x28) != -1 {
        let x = cpo10_d(cpo10_d(this + 0xc).wrapping_add((cpo10_i(oferta + 0x28) as usize).wrapping_mul(4)));
        if x != 0 {
            destruir(x);
            apagar(x);
        }
        cpo10_w(cpo10_d(this + 0xc).wrapping_add((cpo10_i(oferta + 0x28) as usize).wrapping_mul(4)), 0);
        cpo10_w(this + 0x14, cpo10_i(this + 0x14).wrapping_sub(1) as u32);
    }
    if cpo10_b(oferta + 0x1f) == 1 && cpo10_d(this + 0x946) != 0 {
        let l = cpo10_d(this + 0x946);
        cpo10_w(l + 0xa, cpo10_d(l + 2) as u32);
        loop {
            let e = proximo(cpo10_d(this + 0x946));
            if e == 0 {
                break;
            }
            if cpo10_d(e + 4) == cpo10_d(oferta + 8) && cpo10_d(e) == cpo10_d(oferta + 0xc) {
                let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
                tirar(cpo10_d(this + 0x946), e);
                break;
            }
        }
    }
    if cpo10_d(oferta + 4) != 0 {
        let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
        let tirar_proposta: unsafe extern "thiscall" fn(usize, usize, i32) = core::mem::transmute(0x4ccb30usize);
        let l = cpo10_d(oferta + 4);
        cpo10_w(l + 0xa, cpo10_d(l + 2) as u32);
        loop {
            let e = proximo(cpo10_d(oferta + 4));
            if e == 0 {
                break;
            }
            let c = contrato_de(cpo10_d(0xae1050), cpo10_pessoa(cpo10_d(oferta + 0xc)));
            *(0xae1080 as *mut usize) = c;
            if c != 0 && cpo10_clube(cpo10_d(c + 4)) != 0 {
                tirar_proposta(cpo10_d(0xae1050), cpo10_pessoa(cpo10_d(e)), cpo10_i(oferta + 0x20));
            }
            let q = (cpo10_d(this + 0x86e).wrapping_add(cpo10_d(e).wrapping_mul(8)) + 2) as *mut u8;
            *q = (*q).wrapping_sub(1);
        }
    }
    let tirar_do_clube: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x4cca70usize);
    tirar_do_clube(cpo10_d(0xae1050), cpo10_pessoa(cpo10_d(oferta + 0xc)), cpo10_clube(cpo10_d(oferta + 8)));
    let q = (cpo10_d(this + 0x86e).wrapping_add(cpo10_d(oferta + 0xc).wrapping_mul(8)) + 1) as *mut u8;
    *q = (*q).wrapping_sub(1); // as ofertas pela pessoa
    if cpo10_i(oferta + 0x20) < cpo10_i(this + 0x85e) {
        cpo10_w(this + 0x85e, cpo10_d(oferta + 0x20) as u32);
    }
    cpo10_w(cpo10_d(this).wrapping_add((cpo10_i(oferta + 0x20) as usize).wrapping_mul(4)), 0);
    if oferta != 0 {
        destruir(oferta);
        apagar(oferta);
    }
    cpo10_w(this + 8, cpo10_i(this + 8).wrapping_sub(1) as u32);
}

/// Os dados dos nós de uma LLIST (+2 o primeiro nó; nó: +0 o dado, +6 o próximo).
unsafe fn cpo10_itens(lista: usize) -> Vec<usize> {
    let mut v = Vec::new();
    let mut no = cpo10_d(lista + 2);
    while no != 0 {
        v.push(cpo10_d(no));
        no = cpo10_d(no + 6);
    }
    v
}

/// Comparador: lado a lado pelas duas saídas, o sorteio, [0xae1080] e [0xb67a34] (a caixa da falta de ficha).
unsafe extern "thiscall" fn comparar_prazo_da_resposta_a_oferta(this: usize, oferta: usize, dias: *mut u8, fatias: *mut u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(994);
    let mut reg = vec![(dias as usize, 1usize), (fatias as usize, 1), (0xae1080, 4), (0xb67a34, 4)];
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(
        &reg,
        || o(this, oferta, dias, fatias),
        || crate::aleatorio::sem_contar(|| prazo_da_resposta_a_oferta(this, oferta, dias, fatias)),
    );
    PLACAR.registrar(igual, || format!("prazo_da_resposta_a_oferta({:#x}) {:x?} x {:x?}", oferta, mo, mr));
}

/// Comparador: só a original, conferindo o plano (feito antes): saem da lista as prestações que tinham 1 a pagar.
// ponytail: a original paga (finanças), tira itens da lista (libera) e aloca as marcas; não roda duas vezes. A cópia
// tem os ramos e a prova é pela carreira.
unsafe extern "thiscall" fn comparar_pagar_prestacoes_das_transferencias(this: usize) {
    let lista = this + 0x872;
    let antes = cpo10_itens(lista);
    let acabam = antes.iter().filter(|&&e| cpo10_b(e + 0xc) == 1).count();
    orig::<unsafe extern "thiscall" fn(usize)>(995)(this);
    let depois = cpo10_itens(lista).len();
    PLACAR.registrar(depois + acabam == antes.len(), || format!("pagar_prestacoes_das_transferencias: {} prestações, {} acabando, {} depois", antes.len(), acabam, depois));
}

/// Comparador: só a original, conferindo o plano (feito antes): a vaga zerada, as contas +8 e +0x14, a menor vaga livre
/// e o +1 do registro do jogador.
// ponytail: a original destrói e libera a oferta e mexe nas listas; não roda duas vezes.
unsafe extern "thiscall" fn comparar_tirar_oferta_de_transferencia(this: usize, oferta: usize) {
    let vaga = cpo10_i(oferta + 0x20);
    let ligada = cpo10_i(oferta + 0x28) != -1;
    let (n8, n14, livre) = (cpo10_i(this + 8), cpo10_i(this + 0x14), cpo10_i(this + 0x85e));
    let q = cpo10_d(this + 0x86e).wrapping_add(cpo10_d(oferta + 0xc).wrapping_mul(8)) + 1;
    let ofertas = cpo10_b(q);
    orig::<unsafe extern "thiscall" fn(usize, usize)>(996)(this, oferta);
    let ok = cpo10_d(cpo10_d(this).wrapping_add((vaga as usize).wrapping_mul(4))) == 0
        && cpo10_i(this + 8) == n8 - 1
        && cpo10_i(this + 0x14) == n14 - ligada as i32
        && cpo10_i(this + 0x85e) == livre.min(vaga)
        && cpo10_b(q) == ofertas.wrapping_sub(1);
    PLACAR.registrar(ok, || format!("tirar_oferta_de_transferencia: vaga {} contas {}/{} -> {}/{} livre {} -> {} ofertas {} -> {}",
        vaga, n8, n14, cpo10_i(this + 8), cpo10_i(this + 0x14), livre, cpo10_i(this + 0x85e), ofertas, cpo10_b(q)));
}

// ---------- 251a leva: as suspensões na troca de clube: acrescentar a suspensão, soltar as vagas que não valem no clube novo e devolver as suspensões guardadas no fim do empréstimo (discipline.cpp) ----------

unsafe fn cpo11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpo11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo11_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cpo11_por_w(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
unsafe fn cpo11_orgao(this: usize, regra: u8) -> usize {
    cpo11_d(cpo11_d(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)))
}

/// A entrada (14 bytes; +0 livre = −1, +6 o índice da suspensão, −1 sem) da pessoa no órgão: 0x7e82e0 sem criar
/// (só lê: acha pela vaga da pessoa no registro de 29 bytes, 0x55cae0).
unsafe fn cpo11_entrada(orgao: usize, id: u32) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x7e82e0usize);
    achar(orgao, id, 0)
}

/// O registro de 29 bytes da pessoa `id` em [this+0x14] (4 vagas de 5 bytes: a regra e o índice da entrada; +0x14/+0x18
/// o histórico; +0x1c quantas suspensões guardadas).
unsafe fn cpo11_registro(this: usize, id: u32) -> usize {
    cpo11_d(this + 0x14).wrapping_add((id as usize).wrapping_mul(29))
}

/// O órgão vale para o clube? Com nação (+0x17 ≠ −1): a nação do clube (+0x53) é ela; sem nação, com continente
/// (+0x1b ≠ −1): o continente da nação do clube (+0x71) é ele; sem os dois, vale sempre. Clube nulo ou sem nação: não.
unsafe fn cpo11_cobre(orgao: usize, clube: usize) -> bool {
    let nacao = cpo11_d(orgao + 0x17) as u32;
    if nacao == 0xffff_ffff {
        let continente = cpo11_d(orgao + 0x1b) as u32;
        if continente == 0xffff_ffff {
            return true;
        }
        if clube == 0 || cpo11_d(clube + 0x53) == 0 {
            return false;
        }
        let c = cpo11_d(cpo11_d(clube + 0x53) + 0x71);
        return c != 0 && cpo11_d(c) as u32 == continente;
    }
    clube != 0 && cpo11_d(clube + 0x53) != 0 && cpo11_d(cpo11_d(clube + 0x53)) as u32 == nacao
}

/// Solta a suspensão da entrada (se tem: +6 > −1) e libera a vaga `k` do registro da pessoa (regra 0xff, índice −1) e a
/// entrada (+0 = −1). O índice vai no ax; o alto do eax é o do ponteiro da entrada (a 0x556d00 só lê a palavra).
unsafe fn cpo11_liberar(this: usize, id: u32, e: usize) {
    let i = cpo11_w(e + 6);
    if i > -1 {
        let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x556d00usize);
        soltar(this, id, (e as u32 & 0xffff_0000) | i as u16 as u32);
    }
}

unsafe fn cpo11_limpar_vaga(this: usize, id: u32, k: usize, e: usize) {
    let v = cpo11_registro(this, id).wrapping_add(k * 5);
    *(v as *mut u8) = 0xff;
    cpo11_por(v + 1, 0xffff_ffff);
    cpo11_por(e, 0xffff_ffff);
}

/// { 0x556940 } (thiscall, ret 8, discipline.cpp): `Discipline::add_ban(id, suspensao)`: se alguma das 4 regras da
/// suspensão nova (+0xc..+0xf) já tem suspensão da pessoa (a entrada do órgão com +6 > −1), estende a velha (jogos +0
/// somados, ou, com algum dos dois por dias (0xff), os dias +2) e marca as duas (+0x13 bit 0); senão (o bit 0 da nova
/// limpo a cada regra sem suspensão) copia a nova (0x556c70) na primeira vaga livre do vetor (+0 = 0xff e +2 = −1), ou
/// no fim (conta +0xa, capacidade +8; cresce 100, até 0x7d00, malloc/realloc, as vagas novas livres), liga cada regra
/// ao índice (0x7e8510) e, com a quarta regra, põe {id, nação} no índice das nações (+0xc, capacidade +0x10, conta
/// +0x12; cresce 20; qsort 0x55e5f0). Caixas "Error" (capacidade no máximo, sem memória) pela aviso_em.
unsafe extern "thiscall" fn acrescentar_suspensao(this: usize, id: u32, ban: usize) {
    for k in 0..4usize {
        let regra = *((ban + 0xc + k) as *const u8);
        if regra == 0xff {
            continue;
        }
        let e = cpo11_entrada(cpo11_orgao(this, regra), id);
        if e != 0 && cpo11_w(e + 6) > -1 {
            crate::comparacao::ramo("cpo_11 suspensao estendida");
            let s = cpo11_d(this + 4).wrapping_add((cpo11_w(e + 6) as i32 as usize).wrapping_mul(20));
            let jogos = *(s as *const u8);
            let novos = *(ban as *const u8);
            if jogos != 0xff && novos != 0xff {
                *(s as *mut u8) = novos.wrapping_add(jogos);
            } else {
                cpo11_por_w(s + 2, (cpo11_w(s + 2) as u16).wrapping_add(cpo11_w(ban + 2) as u16));
            }
            *((ban + 0x13) as *mut u8) |= 1;
            *((s + 0x13) as *mut u8) |= 1;
            return;
        }
        *((ban + 0x13) as *mut u8) &= 0xfe;
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x556c70usize);
    let n = cpo11_w(this + 0xa);
    let mut vaga: i16 = -1;
    for i in 0..n.max(0) {
        let s = cpo11_d(this + 4).wrapping_add(i as usize * 20);
        if *(s as *const u8) == 0xff && cpo11_w(s + 2) == -1 {
            vaga = i;
            copiar(s, ban);
            break;
        }
    }
    if vaga == -1 {
        let conta = cpo11_w(this + 0xa);
        let cap = cpo11_w(this + 8);
        if conta < cap {
            cpo11_por_w(this + 0xa, conta.wrapping_add(1) as u16);
            copiar(cpo11_d(this + 4).wrapping_add((conta as i32 as usize).wrapping_mul(20)), ban);
            vaga = cpo11_w(this + 0xa).wrapping_sub(1);
        } else {
            if cap >= 0x7d00 {
                aviso_em(0x9b8f18, 0x401);
                return;
            }
            let nova = cap.wrapping_add(100);
            cpo11_por_w(this + 8, nova as u16);
            let tamanho = (nova as i32 as u32).wrapping_mul(20);
            let p = if cpo11_d(this + 4) == 0 {
                let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
                malloc(tamanho)
            } else {
                let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
                realloc(cpo11_d(this + 4), tamanho)
            };
            cpo11_por(this + 4, p as u32);
            if p == 0 {
                aviso_em(0x9b8f18, 0x410);
                return;
            }
            vaga = cpo11_w(this + 0xa);
            cpo11_por_w(this + 0xa, vaga.wrapping_add(1) as u16);
            copiar(p.wrapping_add((vaga as i32 as usize).wrapping_mul(20)), ban);
            let mut i = cpo11_w(this + 0xa);
            while i < cpo11_w(this + 8) {
                let s = cpo11_d(this + 4).wrapping_add((i as i32 as usize).wrapping_mul(20));
                *(s as *mut u8) = 0xff;
                cpo11_por_w(s + 2, 0xffff);
                i = i.wrapping_add(1);
            }
        }
    }
    crate::comparacao::ramo("cpo_11 suspensao nova");
    let ligar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x7e8510usize);
    for k in 0..4usize {
        let regra = *((ban + 0xc + k) as *const u8);
        if regra != 0xff {
            ligar(cpo11_orgao(this, regra), id, vaga as u16 as u32); // liga a regra à suspensão
        }
    }
    if *((ban + 0xf) as *const u8) == 0xff {
        return;
    }
    let cap = cpo11_w(this + 0x10);
    if cap == cpo11_w(this + 0x12) {
        let nova = cap.wrapping_add(0x14);
        cpo11_por_w(this + 0x10, nova as u16);
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realloc(cpo11_d(this + 0xc), (nova as i32 as u32).wrapping_mul(8));
        cpo11_por(this + 0xc, p as u32);
        if p == 0 {
            aviso_em(0x9b8f18, 0x43e);
            return;
        }
    }
    let n = cpo11_w(this + 0x12);
    let item = cpo11_d(this + 0xc).wrapping_add((n as i32 as usize).wrapping_mul(8));
    cpo11_por(item, id);
    let pessoa = cpo11_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    cpo11_por(item + 4, cpo11_d(cpo11_d(pessoa + 0x1a)) as u32); // a nação
    cpo11_por_w(this + 0x12, n.wrapping_add(1) as u16);
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpo11_d(this + 0xc), cpo11_w(this + 0x12) as i32, 8, 0x55e5f0);
}

/// { 0x556270 } (thiscall, ret 0xc, discipline.cpp): `Discipline::clear_bans_invalid_for_club(pessoa, _, clube)`: cada
/// vaga ocupada das 4 do registro da pessoa cujo órgão não vale para o clube novo (`cpo11_cobre`) é liberada: a
/// suspensão solta (0x556d00), a vaga livre e a entrada do órgão livre. Com suspensões guardadas (+0x1c ≠ 0), a caixa
/// "Error" antes (e segue). Da troca de clube (0x8c9720).
unsafe extern "thiscall" fn tirar_suspensoes_de_fora_do_clube(this: usize, pessoa: usize, _nada: u32, clube: usize) {
    let id = cpo11_d(pessoa) as u32;
    let r = cpo11_registro(this, id);
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if regra == 0xff {
            continue;
        }
        let orgao = cpo11_orgao(this, regra);
        if cpo11_cobre(orgao, clube) {
            continue;
        }
        let e = cpo11_entrada(orgao, id);
        if e == 0 {
            continue;
        }
        crate::comparacao::ramo("cpo_11 transferido solta a vaga");
        cpo11_liberar(this, id, e);
        if *((r + 0x1c) as *const u8) != 0 {
            aviso_em(0x9b8f18, 0x239);
        }
        cpo11_limpar_vaga(this, id, k, e);
    }
}

/// O que a 0x556570 vai fazer, lido antes (a lista das guardadas perde e libera os itens): o contrato e o clube, os
/// itens guardados da pessoa (a vaga e os 5 bytes) e, depois deles, as vagas 0..2 não devolvidas que o clube não cobre.
struct Cpo11Devolver {
    contrato: usize,
    clube: usize,
    guardadas: Vec<(usize, [u8; 5])>,
    resto: u8,
    soltas: Vec<(usize, usize)>,
}

/// O plano da 0x556570 (o cursor da lista é zerado pela original na entrada; a busca sem criar só lê).
unsafe fn cpo11_plano_devolver(this: usize, pessoa: usize) -> Cpo11Devolver {
    let id = cpo11_d(pessoa) as u32;
    let r = cpo11_registro(this, id);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contrato = contrato_de(cpo11_d(0xae1050), pessoa);
    let clube = if contrato == 0 { 0 } else { cpo11_d(0xae23b4).wrapping_add(cpo11_d(contrato + 4).wrapping_mul(0x245)) };
    let mut p = Cpo11Devolver { contrato, clube, guardadas: Vec::new(), resto: *((r + 0x1c) as *const u8), soltas: Vec::new() };
    let lista = this + 0x1c;
    cpo11_por(lista + 0xa, cpo11_d(this + 0x1e) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut vagas = [0u8; 20];
    core::ptr::copy_nonoverlapping(r as *const u8, vagas.as_mut_ptr(), 20);
    let mut marcas = [false; 4];
    let mut item = proximo(lista);
    while item != 0 && (p.resto as i8) > 0 {
        if cpo11_d(item) as u32 == id {
            let k = (*((item + 4) as *const u8) as i8 as isize as usize) & 3;
            let mut b = [0u8; 5];
            core::ptr::copy_nonoverlapping((item + 5) as *const u8, b.as_mut_ptr(), 5);
            vagas[k * 5..k * 5 + 5].copy_from_slice(&b);
            marcas[k] = true;
            p.guardadas.push((k, b));
            p.resto = p.resto.wrapping_sub(1);
        }
        item = proximo(lista);
    }
    for k in 0..3usize {
        if marcas[k] || vagas[k * 5] == 0xff {
            continue;
        }
        let orgao = cpo11_orgao(this, vagas[k * 5]);
        if !cpo11_cobre(orgao, clube) {
            let e = cpo11_entrada(orgao, id);
            if e != 0 {
                p.soltas.push((k, e));
            }
        }
    }
    p
}

/// { 0x556570 } (thiscall, ret 4, discipline.cpp): `Discipline::restore_held_bans(pessoa)`: no fim do empréstimo
/// (0x4d07e0): o contrato (0x4cf5a0, guardado em [0xae1080]) dá o clube; cada suspensão guardada da pessoa na lista
/// [this+0x1c] (id, vaga +4, os 5 bytes +5), enquanto houver guardadas (+0x1c do registro, com sinal): a vaga ocupada é
/// liberada antes (a suspensão solta, 0x556d00), a guardada volta para a vaga, sai da lista (0x68d1f0, que a libera) e a
/// conta cai. Depois, as vagas 0..2 não devolvidas (a quarta nunca: `k != 3` da original) cujo órgão não vale para o
/// clube são liberadas.
unsafe extern "thiscall" fn restaurar_suspensoes_guardadas(this: usize, pessoa: usize) {
    let id = cpo11_d(pessoa) as u32;
    let r = cpo11_registro(this, id);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contrato = contrato_de(cpo11_d(0xae1050), pessoa);
    cpo11_por(0xae1080, contrato as u32);
    let clube = if contrato == 0 { 0 } else { cpo11_d(0xae23b4).wrapping_add(cpo11_d(contrato + 4).wrapping_mul(0x245)) };
    let lista = this + 0x1c;
    cpo11_por(lista + 0xa, cpo11_d(this + 0x1e) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let mut marcas = [0u32; 4];
    let mut item = proximo(lista);
    while item != 0 && *((r + 0x1c) as *const i8) > 0 {
        if cpo11_d(item) as u32 == id {
            let k = *((item + 4) as *const u8) as i8 as isize as usize;
            let regra = *(r.wrapping_add(k.wrapping_mul(5)) as *const u8);
            if regra != 0xff {
                let e = cpo11_entrada(cpo11_orgao(this, regra), id);
                if e != 0 {
                    cpo11_liberar(this, id, e);
                    cpo11_limpar_vaga(this, id, k, e);
                }
            }
            crate::comparacao::ramo("cpo_11 guardada devolvida");
            marcas[k & 3] = 1;
            let v = r.wrapping_add(k.wrapping_mul(5));
            cpo11_por(v, cpo11_d(item + 5) as u32);
            *((v + 4) as *mut u8) = *((item + 9) as *const u8);
            remover(lista, item); // libera o item
            *((r + 0x1c) as *mut u8) = (*((r + 0x1c) as *const u8)).wrapping_sub(1);
        }
        item = proximo(lista);
    }
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if marcas[k] != 0 || k == 3 || regra == 0xff {
            continue;
        }
        let orgao = cpo11_orgao(this, regra);
        if cpo11_cobre(orgao, clube) {
            continue;
        }
        let e = cpo11_entrada(orgao, id);
        if e != 0 {
            crate::comparacao::ramo("cpo_11 fim do emprestimo solta a vaga");
            cpo11_liberar(this, id, e);
            cpo11_limpar_vaga(this, id, k, e);
        }
    }
}

/// O caminho da 0x556940, lido antes: a suspensão velha estendida (e os valores novos dela), ou a vaga da nova e a
/// conta do vetor depois; `None` nas caixas (capacidade no máximo).
enum Cpo11Plano {
    Estender { s: usize, jogos: u8, dias: i16 },
    Nova { vaga: i16, conta: i16 },
    Caixa,
}

unsafe fn cpo11_plano_suspensao(this: usize, id: u32, ban: usize) -> Cpo11Plano {
    for k in 0..4usize {
        let regra = *((ban + 0xc + k) as *const u8);
        if regra == 0xff {
            continue;
        }
        let e = cpo11_entrada(cpo11_orgao(this, regra), id);
        if e != 0 && cpo11_w(e + 6) > -1 {
            let s = cpo11_d(this + 4).wrapping_add((cpo11_w(e + 6) as i32 as usize).wrapping_mul(20));
            let (jogos, novos) = (*(s as *const u8), *(ban as *const u8));
            return if jogos != 0xff && novos != 0xff {
                Cpo11Plano::Estender { s, jogos: novos.wrapping_add(jogos), dias: cpo11_w(s + 2) }
            } else {
                Cpo11Plano::Estender { s, jogos, dias: cpo11_w(s + 2).wrapping_add(cpo11_w(ban + 2)) }
            };
        }
    }
    let n = cpo11_w(this + 0xa);
    for i in 0..n.max(0) {
        let s = cpo11_d(this + 4).wrapping_add(i as usize * 20);
        if *(s as *const u8) == 0xff && cpo11_w(s + 2) == -1 {
            return Cpo11Plano::Nova { vaga: i, conta: n };
        }
    }
    if n < cpo11_w(this + 8) || cpo11_w(this + 8) < 0x7d00 {
        return Cpo11Plano::Nova { vaga: n, conta: n.wrapping_add(1) };
    }
    Cpo11Plano::Caixa
}

/// Comparador: a 0x556940 realoca os vetores e liga os órgãos (não pode rodar duas vezes): o plano antes, a original
/// roda; confere a velha estendida (jogos, dias, bit 0 das duas), ou a nova na vaga (jogos, dias, as 4 regras), a
/// conta, as entradas das regras apontando para a vaga e, com a quarta regra, o índice das nações com um a mais.
unsafe extern "thiscall" fn comparar_acrescentar_suspensao(this: usize, id: u32, ban: usize) {
    let plano = cpo11_plano_suspensao(this, id, ban);
    let indice = cpo11_w(this + 0x12);
    let mut regras = [0u8; 4];
    core::ptr::copy_nonoverlapping((ban + 0xc) as *const u8, regras.as_mut_ptr(), 4);
    orig::<unsafe extern "thiscall" fn(usize, u32, usize)>(997)(this, id, ban);
    let (ok, detalhe) = match plano {
        Cpo11Plano::Caixa => return,
        Cpo11Plano::Estender { s, jogos, dias } => {
            let ok = *(s as *const u8) == jogos && cpo11_w(s + 2) == dias && *((s + 0x13) as *const u8) & 1 == 1
                && *((ban + 0x13) as *const u8) & 1 == 1;
            (ok, format!("estender: jogos {} (esperava {}), dias {} (esperava {})", *(s as *const u8), jogos, cpo11_w(s + 2), dias))
        }
        Cpo11Plano::Nova { vaga, conta } => {
            let s = cpo11_d(this + 4).wrapping_add((vaga as i32 as usize).wrapping_mul(20));
            let mut ok = cpo11_w(this + 0xa) == conta && *(s as *const u8) == *(ban as *const u8) && cpo11_w(s + 2) == cpo11_w(ban + 2);
            for k in 0..4usize {
                ok &= *((s + 0xc + k) as *const u8) == regras[k];
                if regras[k] != 0xff {
                    let e = cpo11_entrada(cpo11_orgao(this, regras[k]), id);
                    ok &= e != 0 && cpo11_w(e + 6) == vaga;
                }
            }
            if regras[3] != 0xff {
                ok &= cpo11_w(this + 0x12) == indice.wrapping_add(1);
            }
            (ok, format!("nova: vaga {} conta {} (esperava {}), índice {} (antes {})", vaga, cpo11_w(this + 0xa), conta, cpo11_w(this + 0x12), indice))
        }
    };
    PLACAR.registrar(ok, || format!("acrescentar_suspensao({}) {}", id, detalhe));
}

/// Comparador: a 0x556270 solta suspensões (índice das nações, órgãos): o plano antes (as vagas que o clube não cobre
/// e as entradas), a original roda; confere as vagas liberadas (0xff, −1, a entrada −1) e as outras intactas. Uma
/// entrada repetida em duas vagas não entra na conferência da segunda.
unsafe extern "thiscall" fn comparar_tirar_suspensoes_de_fora_do_clube(this: usize, pessoa: usize, nada: u32, clube: usize) {
    let id = cpo11_d(pessoa) as u32;
    let r = cpo11_registro(this, id);
    let mut antes = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, antes.as_mut_ptr(), 29);
    let mut plano: [Option<usize>; 4] = [None; 4];
    let mut duvida = [false; 4];
    for k in 0..4usize {
        let regra = antes[k * 5];
        if regra == 0xff {
            continue;
        }
        let orgao = cpo11_orgao(this, regra);
        if !cpo11_cobre(orgao, clube) {
            let e = cpo11_entrada(orgao, id);
            if e != 0 {
                duvida[k] = plano[..k].iter().any(|x| *x == Some(e));
                plano[k] = Some(e);
            }
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(998)(this, pessoa, nada, clube);
    let mut depois = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, depois.as_mut_ptr(), 29);
    let mut ok = depois[20..] == antes[20..];
    for k in 0..4usize {
        if duvida[k] {
            continue;
        }
        let v = &depois[k * 5..k * 5 + 5];
        ok &= match plano[k] {
            Some(e) => v == [0xff; 5] && cpo11_d(e) as u32 == 0xffff_ffff,
            None => *v == antes[k * 5..k * 5 + 5],
        };
    }
    PLACAR.registrar(ok, || format!("tirar_suspensoes_de_fora_do_clube({}) plano {:x?} antes {:02x?} depois {:02x?}", id, plano, antes, depois));
}

/// Comparador: a 0x556570 tira e libera itens da lista (lição 1: o esperado é montado ANTES; nenhum item é lido depois)
/// e solta suspensões: o plano antes, a original roda; confere [0xae1080], as vagas devolvidas (os 5 bytes guardados), as
/// soltas (0xff, −1, a entrada −1), a conta de guardadas e as outras vagas intactas.
unsafe extern "thiscall" fn comparar_restaurar_suspensoes_guardadas(this: usize, pessoa: usize) {
    let id = cpo11_d(pessoa) as u32;
    let r = cpo11_registro(this, id);
    let p = cpo11_plano_devolver(this, pessoa);
    let mut antes = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, antes.as_mut_ptr(), 29);
    orig::<unsafe extern "thiscall" fn(usize, usize)>(999)(this, pessoa);
    let mut depois = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, depois.as_mut_ptr(), 29);
    let mut esperado = antes;
    for (k, b) in &p.guardadas {
        esperado[k * 5..k * 5 + 5].copy_from_slice(b);
    }
    let mut ok = cpo11_d(0xae1080) == p.contrato;
    for (k, e) in &p.soltas {
        esperado[k * 5..k * 5 + 5].copy_from_slice(&[0xff; 5]);
        ok &= cpo11_d(*e) as u32 == 0xffff_ffff;
    }
    esperado[0x1c] = p.resto;
    ok &= depois == esperado;
    PLACAR.registrar(ok, || format!("restaurar_suspensoes_guardadas({}) clube {:x} esperado {:02x?} depois {:02x?}", id, p.clube, esperado, depois));
}

// ---------- 251a leva: os idiomas do jogador (acrescentar e melhorar) e zerar as estatísticas do jogador regenerado (player_stats.cpp) ----------

unsafe fn cpo12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpo12_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// O registro de idioma (6 bytes: +0 id da pessoa, +4 idioma, +5 fluência) da `pessoa` no `idioma`: bsearch
/// (0x946464, comparação 0x7c6d40) na tabela [this+0x3c], [this+0x44] registros. 0 sem achar.
unsafe fn cpo12_idioma(this: usize, pessoa: usize, idioma: u8) -> usize {
    let bsearch: unsafe extern "C" fn(*const u8, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let mut chave = [0u8; 8];
    core::ptr::write_unaligned(chave.as_mut_ptr() as *mut u32, cpo12_d(pessoa) as u32);
    chave[4] = idioma; // a comparação só lê o byte +4 da chave
    bsearch(chave.as_ptr(), cpo12_d(this + 0x3c), cpo12_d(this + 0x44) as u32, 6, 0x7c6d40)
}

/// { 0x7c6a30 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::add_language(pessoa, idioma, fluencia,
/// ordenar)`: a tabela dos idiomas (6 bytes por registro em [this+0x3c], capacidade +0x40, conta +0x44) nasce com 100
/// (malloc 0x258) e cresce 100 (realloc) quando cheia; o registro novo no fim: o id da pessoa, o idioma e a fluência
/// (no máximo 10, com sinal); com `ordenar`, qsort pela comparação 0x7c6d40. Sem memória: caixa fatal.
unsafe extern "thiscall" fn acrescentar_idioma(this: usize, pessoa: usize, idioma: u32, fluencia: u32, ordenar: u32) {
    if cpo12_d(this + 0x3c) == 0 {
        let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
        let p = malloc(0x258);
        cpo12_por(this + 0x3c, p as u32);
        if p == 0 {
            aviso_em(0xa78310, 0x11fd);
            erro_fatal();
        }
        cpo12_por(this + 0x40, 0x64);
    }
    let cap = cpo12_d(this + 0x40);
    if cap == cpo12_d(this + 0x44) {
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realloc(cpo12_d(this + 0x3c), (cap as u32).wrapping_mul(3).wrapping_add(0x12c).wrapping_mul(2));
        cpo12_por(this + 0x3c, p as u32);
        if p == 0 {
            aviso_em(0xa78310, 0x1208);
            erro_fatal();
        }
        cpo12_por(this + 0x40, (cpo12_d(this + 0x40) as u32).wrapping_add(0x64));
    }
    crate::comparacao::ramo("cpo_12 idioma novo");
    let n = cpo12_d(this + 0x44);
    let r = cpo12_d(this + 0x3c).wrapping_add(n.wrapping_mul(6));
    cpo12_por(r, cpo12_d(pessoa) as u32);
    *((r + 4) as *mut u8) = idioma as u8;
    let f = fluencia as u8;
    *((r + 5) as *mut u8) = if (f as i8) > 10 { 10 } else { f };
    cpo12_por(this + 0x44, (n as u32).wrapping_add(1));
    if ordenar != 0 {
        crate::comparacao::ramo("cpo_12 idiomas ordenados");
        let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
        qsort(cpo12_d(this + 0x3c), cpo12_d(this + 0x44) as u32, 6, 0x7c6d40);
    }
}

/// { 0x7c6ca0 } (thiscall, ret 0x10, player_stats.cpp): `PlayerStats::improve_language(pessoa, idioma, aumento,
/// ordenar)`: sem tabela, ou sem o registro da pessoa no idioma (`cpo12_idioma`), acrescenta-o com a fluência
/// `aumento` (0x7c6a30); com ele, soma o aumento à fluência (byte), no máximo 10 (com sinal).
unsafe extern "thiscall" fn melhorar_idioma(this: usize, pessoa: usize, idioma: u32, aumento: u32, ordenar: u32) {
    let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) = core::mem::transmute(0x7c6a30usize);
    if cpo12_d(this + 0x3c) == 0 {
        acrescentar(this, pessoa, idioma, aumento, ordenar);
        return;
    }
    let r = cpo12_idioma(this, pessoa, idioma as u8);
    if r == 0 {
        acrescentar(this, pessoa, idioma, aumento, ordenar);
        return;
    }
    crate::comparacao::ramo("cpo_12 idioma melhorado");
    let f = (*((r + 5) as *const u8)).wrapping_add(aumento as u8);
    *((r + 5) as *mut u8) = f;
    if (f as i8) > 10 {
        *((r + 5) as *mut u8) = 10;
    }
}

/// { 0x7c6650 } (thiscall, ret 4, player_stats.cpp): `PlayerStats::reset_player(pessoa)`: no registro de 39 bytes da
/// pessoa ([this+0x30] + id × 39), cada uma das 6 categorias (+0..+0x14) com registro (≠ −1) o libera (+0xe = −1 no
/// registro de 32 bytes de [this+4]; +1 nos livres [this+0x38]) e fica −1; zera +0x20, +0x22 (palavras) e +0x24..+0x26
/// (as notas recentes) e esvazia o histórico (0x58f110 sobre [this] com +0x18/+0x1c). Pessoa nula ou não jogadora
/// (+0x61): a original (caixa fatal). Devolve o que a 0x58f110 devolve. Do regen (0x7a59e0).
unsafe extern "thiscall" fn zerar_estatisticas_do_jogador(this: usize, pessoa: usize) -> usize {
    if pessoa == 0 || cpo12_d(pessoa + 0x61) == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1002)(this, pessoa); // caixa fatal: só a original
    }
    let r = cpo12_d(this + 0x30).wrapping_add(cpo12_d(pessoa).wrapping_mul(39));
    for k in 0..6usize {
        let i = cpo12_d(r + 4 * k);
        if i as u32 == 0xffff_ffff {
            continue;
        }
        core::ptr::write_unaligned((cpo12_d(this + 4).wrapping_add(i << 5) + 0xe) as *mut u16, 0xffff);
        cpo12_por(this + 0x38, (cpo12_d(this + 0x38) as u32).wrapping_add(1));
        cpo12_por(r + 4 * k, 0xffff_ffff);
    }
    core::ptr::write_unaligned((r + 0x20) as *mut u16, 0);
    core::ptr::write_unaligned((r + 0x22) as *mut u16, 0);
    *((r + 0x26) as *mut u8) = 0;
    *((r + 0x25) as *mut u8) = 0;
    *((r + 0x24) as *mut u8) = 0;
    let esvaziar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x58f110usize);
    esvaziar(cpo12_d(this), r + 0x18, r + 0x1c)
}

/// Comparador: a 0x7c6a30 aloca e realoca a tabela (não pode rodar duas vezes): o plano antes (a conta e a
/// capacidade depois, a fluência), a original roda; confere a conta, a capacidade e o registro novo (no fim sem ordenar;
/// com `ordenar`, em algum lugar da tabela).
unsafe extern "thiscall" fn comparar_acrescentar_idioma(this: usize, pessoa: usize, idioma: u32, fluencia: u32, ordenar: u32) {
    let (n, cap) = if cpo12_d(this + 0x3c) == 0 { (cpo12_d(this + 0x44), 100) } else { (cpo12_d(this + 0x44), cpo12_d(this + 0x40)) };
    let cap = if cap == n { cap.wrapping_add(100) } else { cap };
    let f = fluencia as u8;
    let novo = (cpo12_d(pessoa) as u32, idioma as u8, if (f as i8) > 10 { 10 } else { f });
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(1000)(this, pessoa, idioma, fluencia, ordenar);
    let base = cpo12_d(this + 0x3c);
    let e = |i: usize| {
        let r = base.wrapping_add(i * 6);
        (cpo12_d(r) as u32, *((r + 4) as *const u8), *((r + 5) as *const u8))
    };
    let conta = cpo12_d(this + 0x44);
    let mut ok = conta == n.wrapping_add(1) && cpo12_d(this + 0x40) == cap;
    if ok {
        ok = if ordenar == 0 { e(n) == novo } else { (0..conta).any(|i| e(i) == novo) };
    }
    PLACAR.registrar(ok, || format!("acrescentar_idioma({:?}, ordenar {}) conta {} (esperava {}) capacidade {} (esperava {})",
        novo, ordenar, conta, n.wrapping_add(1), cpo12_d(this + 0x40), cap));
}

/// Comparador: sem tabela ou sem o registro, só a original (o acréscimo 0x7c6a30 tem o comparador dele); com o
/// registro, lado a lado nos 6 bytes dele.
unsafe extern "thiscall" fn comparar_melhorar_idioma(this: usize, pessoa: usize, idioma: u32, aumento: u32, ordenar: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(1001);
    let r = if cpo12_d(this + 0x3c) == 0 { 0 } else { cpo12_idioma(this, pessoa, idioma as u8) };
    if r == 0 {
        return o(this, pessoa, idioma, aumento, ordenar);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 6)],
        || o(this, pessoa, idioma, aumento, ordenar), || melhorar_idioma(this, pessoa, idioma, aumento, ordenar));
    PLACAR.registrar(igual, || format!("melhorar_idioma({}, idioma {}, +{}) {:x?} x {:x?}", cpo12_d(pessoa), idioma as u8, aumento as u8, mo, mr));
}

/// Comparador: lado a lado no registro de 39 bytes, na conta dos livres (+0x38) e na marca +0xe dos registros de
/// categoria liberados (a 0x58f110 só grava +0x1c, dentro do registro). Pessoa nula ou não jogadora: só a original.
unsafe extern "thiscall" fn comparar_zerar_estatisticas_do_jogador(this: usize, pessoa: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(1002);
    if pessoa == 0 || cpo12_d(pessoa + 0x61) == 0 {
        return o(this, pessoa);
    }
    let r = cpo12_d(this + 0x30).wrapping_add(cpo12_d(pessoa).wrapping_mul(39));
    let mut reg = vec![(r, 39), (this + 0x38, 4)];
    for k in 0..6usize {
        let i = cpo12_d(r + 4 * k);
        if i as u32 != 0xffff_ffff {
            reg.push((cpo12_d(this + 4).wrapping_add(i << 5) + 0xe, 2));
        }
    }
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa), || zerar_estatisticas_do_jogador(this, pessoa));
    PLACAR.registrar(igual && a == b, || format!("zerar_estatisticas_do_jogador({}) {:x?} x {:x?}", cpo12_d(pessoa), mo, mr));
    a
}

// ---------- 251a leva: as seleções: a competição atual da nação e os clubes com jogo na data; a disciplina: tirar o jogador regenerado (national_teams.cpp, discipline.cpp) ----------

unsafe fn cpo13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpo13_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// A competição (id) é uma das fases finais grandes ([0x9cf79c], [0x9cf7a0], [0x9cf7a4], [0x9cf7a8], [0x9cf7ac],
/// [0x9cf790], [0x9cf8a0], [0x9cf78c], [0x9cf964])?
unsafe fn cpo13_final(id: u32) -> bool {
    [0x9cf79c, 0x9cf7a0, 0x9cf7a4, 0x9cf7a8, 0x9cf7ac, 0x9cf790, 0x9cf8a0, 0x9cf78c, 0x9cf964usize]
        .iter()
        .any(|&g| cpo13_d(g) as u32 == id)
}

/// O que a 0x775220 grava: a competição atual da nação ([this+4] + id × 110 + 4), a da fila ([this+8] + id × 4) e se
/// avisa o gerenciador dos técnicos.
struct Cpo13Atual {
    vaga: usize,
    nova: Option<u32>,
    fila: usize,
    na_fila: Option<u32>,
    avisa: bool,
}

/// O plano da 0x775220 (só lê): com competição atual (≠ −1) que é fase final grande, a nova vai para a fila e a atual
/// fica (sem aviso); senão, se a nova é fase final grande e há atual, a atual vai para a fila; a nova vira a atual e o
/// gerenciador dos técnicos ([0xb67a44], se existe) é avisado.
unsafe fn cpo13_plano_atual(this: usize, nacao: usize, comp: usize) -> Cpo13Atual {
    let id = cpo13_d(nacao);
    let vaga = cpo13_d(this + 4).wrapping_add(id.wrapping_mul(110)) + 4;
    let fila = cpo13_d(this + 8).wrapping_add(id.wrapping_mul(4));
    let atual = cpo13_d(vaga) as u32;
    let nova = cpo13_d(comp) as u32;
    let mut p = Cpo13Atual { vaga, nova: Some(nova), fila, na_fila: None, avisa: cpo13_d(0xb67a44) != 0 };
    if atual != 0xffff_ffff {
        let r = cpo13_d(0xae23d0).wrapping_add((atual as usize).wrapping_mul(107)); // registros de 107 bytes
        if r != 0 && cpo13_final(cpo13_d(r) as u32) {
            return Cpo13Atual { vaga, nova: None, fila, na_fila: Some(nova), avisa: false };
        }
        if cpo13_final(nova) {
            p.na_fila = Some(atual);
        }
    }
    p
}

/// { 0x775220 } (thiscall, ret 8, national_teams.cpp): `NationalTeams::set_current_competition(nacao, comp)`: a
/// competição atual da nação vira `comp` (plano em `cpo13_plano_atual`: a fase final grande em curso fica na frente e a
/// outra vai para a fila) e o gerenciador dos técnicos é avisado (0x6a3770 com o clube da seleção principal, 0x53b3d0,
/// e 1). Nação ou competição nula: a caixa "Error" (e volta).
unsafe extern "thiscall" fn definir_competicao_da_selecao(this: usize, nacao: usize, comp: usize) {
    if nacao == 0 || comp == 0 {
        aviso_em(0xa5b2a4, 0x1bb2);
        return;
    }
    let p = cpo13_plano_atual(this, nacao, comp);
    if let Some(f) = p.na_fila {
        cpo13_por(p.fila, f);
    }
    let Some(nova) = p.nova else {
        crate::comparacao::ramo("cpo_13 fase final fica");
        return;
    };
    crate::comparacao::ramo("cpo_13 competicao atual");
    cpo13_por(p.vaga, nova);
    let g = cpo13_d(0xb67a44);
    if g != 0 {
        let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
        let avisar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x6a3770usize);
        avisar(g, principal(nacao), 1);
    }
}

/// Comparador: a 0x6a3770 (o aviso) não pode rodar duas vezes: o plano antes, a original roda; confere a competição
/// atual e a da fila. Nação ou competição nula: só a original (caixa).
unsafe extern "thiscall" fn comparar_definir_competicao_da_selecao(this: usize, nacao: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1003);
    if nacao == 0 || comp == 0 {
        return o(this, nacao, comp);
    }
    let p = cpo13_plano_atual(this, nacao, comp);
    let (atual, fila) = (cpo13_d(p.vaga) as u32, cpo13_d(p.fila) as u32);
    o(this, nacao, comp);
    let (atual2, fila2) = (cpo13_d(p.vaga) as u32, cpo13_d(p.fila) as u32);
    let ok = atual2 == p.nova.unwrap_or(atual) && fila2 == p.na_fila.unwrap_or(fila);
    PLACAR.registrar(ok, || format!("definir_competicao_da_selecao(nação {}, comp {}) atual {:#x} -> {:#x} (esperava {:x?}), fila {:#x} -> {:#x} (esperava {:x?})",
        cpo13_d(nacao), cpo13_d(comp), atual, atual2, p.nova, fila, fila2, p.na_fila));
}

/// Marca o clube `c` (se existe e o id < n) com o código na tabela [this+0x24].
unsafe fn cpo13_marcar(this: usize, c: usize, n: i32, codigo: u8) {
    if c != 0 && (cpo13_d(c) as i32) < n {
        *(cpo13_d(this + 0x24).wrapping_add(cpo13_d(c)) as *mut u8) = codigo;
    }
}

/// O clube tem nação marcada (+0x53, +0x11c ≠ 0)?
unsafe fn cpo13_nacao_marcada(c: usize) -> bool {
    let n = cpo13_d(c + 0x53);
    n != 0 && *((n + 0x11c) as *const u8) != 0
}

/// { 0x7775c0 } (thiscall, ret 4, national_teams.cpp): `NationalTeams::mark_clubs_with_fixtures(data)`: a tabela de um
/// byte por clube ([this+0x24], n = clubes [0xae235c] − 2 × nações [0xae2350]; calloc na primeira, sem memória a
/// caixa "Error") recebe, para cada jogo não amistoso (0x5c6710) da data nas 3 fatias (0x5ae6a0) de cada uma das 0x21
/// entradas de 0x48 bytes de 0xb63df0 (a nação +0 ativa em +0x11c, ou sem nação; o grupo +0x14): sem nação, 3 para os
/// clubes (+0x1c, +0x20) de nação ativa; com nação, 1 quando a competição do jogo (+0x14) é a da liga do mandante (+0x57)
/// ou a +0x60 do visitante, senão 2, para os dois. Lida pela remarcação dos jogos dos clubes com convocados (0x7777e0).
unsafe extern "thiscall" fn marcar_clubes_com_jogo(this: usize, data: usize) {
    let n = (cpo13_d(0xae235c) as i32).wrapping_sub((cpo13_d(0xae2350) as i32).wrapping_mul(2));
    if cpo13_d(this + 0x24) == 0 {
        let calloc: unsafe extern "C" fn(i32, u32) -> usize = core::mem::transmute(0x945501usize);
        let p = calloc(n, 1);
        cpo13_por(this + 0x24, p as u32);
        if p == 0 {
            aviso_em(0xa5b2a4, 0x2229);
            return;
        }
    }
    let lista_da_data: unsafe extern "stdcall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x5ae6a0usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    for i in 0..0x21usize {
        let e = 0xb63df0 + i * 0x48;
        let nacao = cpo13_d(e);
        if nacao != 0 && *((nacao + 0x11c) as *const u8) == 0 {
            continue;
        }
        for fatia in 0..3u32 {
            // a original empurra a fatia num dword de pilha com o alto de lixo e o grupo no al: a 0x5abf10 só lê os bytes
            let lista = lista_da_data(data, fatia, *((e + 0x14) as *const u8) as u32);
            if lista == 0 {
                continue;
            }
            cpo13_por(lista + 0xa, cpo13_d(lista + 2) as u32);
            let mut jogo = proximo(lista);
            while jogo != 0 {
                if amistoso(cpo13_d(0xae2c74), cpo13_d(jogo + 0x14)) == 0 {
                    let (casa, fora) = (cpo13_d(jogo + 0x1c), cpo13_d(jogo + 0x20));
                    if cpo13_d(e) == 0 {
                        if casa != 0 && cpo13_nacao_marcada(casa) {
                            cpo13_marcar(this, casa, n, 3);
                        }
                        if fora != 0 && cpo13_nacao_marcada(fora) {
                            cpo13_marcar(this, fora, n, 3);
                        }
                    } else {
                        let comp = cpo13_d(jogo + 0x14);
                        // o visitante +0x60 é lido sem conferir o ponteiro (como a original)
                        let codigo = if casa != 0 && (cpo13_d(casa + 0x57) == comp || cpo13_d(fora + 0x60) == comp) { 1 } else { 2 };
                        crate::comparacao::ramo("cpo_13 clubes com jogo");
                        cpo13_marcar(this, casa, n, codigo); // o mandante
                        cpo13_marcar(this, fora, n, codigo); // o visitante
                    }
                }
                jogo = proximo(lista);
            }
        }
    }
}

/// Comparador: na primeira (sem tabela: calloc) só a original; depois, lado a lado na tabela (n bytes). A lista do
/// dia e o amistoso só leem (o cursor da lista volta ao começo a cada uso).
unsafe extern "thiscall" fn comparar_marcar_clubes_com_jogo(this: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1004);
    let n = (cpo13_d(0xae235c) as i32).wrapping_sub((cpo13_d(0xae2350) as i32).wrapping_mul(2));
    if cpo13_d(this + 0x24) == 0 || n <= 0 {
        return o(this, data);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(cpo13_d(this + 0x24), n as usize)], || o(this, data), || marcar_clubes_com_jogo(this, data));
    PLACAR.registrar(igual, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<(usize, u8, u8)> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).map(|i| (i, a[i], b[i])).collect();
        format!("marcar_clubes_com_jogo({:x}) clube, original, rust: {:?}", core::ptr::read_unaligned(data as *const u32), dif)
    });
}

/// O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
unsafe fn cpo13_orgao(this: usize, regra: u8) -> usize {
    cpo13_d(cpo13_d(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)))
}

/// A entrada da pessoa no órgão pela vaga dela (0x7e82e0 sem criar: só lê).
unsafe fn cpo13_entrada(orgao: usize, id: u32) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x7e82e0usize);
    achar(orgao, id, 0)
}

/// { 0x556830 } (thiscall, ret 4, discipline.cpp): `Discipline::remove_player(id)`: do regen (0x7abac0): apaga o
/// histórico de suspensões (0x55e1d0), libera as 4 vagas do registro de 29 bytes (com entrada no órgão: a suspensão
/// solta, 0x556d00, a vaga livre e a entrada +0 = −1), tira da lista [this+0x1c] as suspensões guardadas dele
/// (0x68d1f0, enquanto a conta +0x1c > 0) e o registro fica 0xff com a conta 0.
unsafe extern "thiscall" fn tirar_jogador_da_disciplina(this: usize, id: u32) {
    let r = cpo13_d(this + 0x14).wrapping_add((id as usize).wrapping_mul(29));
    let apagar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x55e1d0usize);
    apagar(this, cpo13_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e)));
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if regra == 0xff {
            continue;
        }
        let e = cpo13_entrada(cpo13_orgao(this, regra), id);
        if e == 0 {
            continue;
        }
        crate::comparacao::ramo("cpo_13 regen solta a vaga");
        let i = core::ptr::read_unaligned((e + 6) as *const i16);
        if i > -1 {
            // o índice no ax; o alto do eax é o do ponteiro da entrada (a 0x556d00 só lê a palavra)
            let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x556d00usize);
            soltar(this, id, (e as u32 & 0xffff_0000) | i as u16 as u32);
        }
        let v = cpo13_d(this + 0x14).wrapping_add((id as usize).wrapping_mul(29) + k * 5);
        *(v as *mut u8) = 0xff;
        cpo13_por(v + 1, 0xffff_ffff);
        cpo13_por(e, 0xffff_ffff);
    }
    let lista = this + 0x1c;
    cpo13_por(lista + 0xa, cpo13_d(this + 0x1e) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let mut item = proximo(lista);
    while item != 0 && *((r + 0x1c) as *const i8) > 0 {
        if cpo13_d(item) as u32 == id {
            remover(lista, item); // libera o item
            *((r + 0x1c) as *mut u8) = (*((r + 0x1c) as *const u8)).wrapping_sub(1);
        }
        item = proximo(lista);
    }
    core::ptr::write_bytes(r as *mut u8, 0xff, 29);
    *((r + 0x1c) as *mut u8) = 0;
}

/// Comparador: a 0x556830 solta suspensões e tira (e libera) itens da lista: o plano antes (as entradas das vagas), a
/// original roda; confere o registro (0xff, conta 0) e as entradas livres (+0 = −1).
unsafe extern "thiscall" fn comparar_tirar_jogador_da_disciplina(this: usize, id: u32) {
    let r = cpo13_d(this + 0x14).wrapping_add((id as usize).wrapping_mul(29));
    let mut entradas = Vec::new();
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if regra != 0xff {
            let e = cpo13_entrada(cpo13_orgao(this, regra), id);
            if e != 0 {
                entradas.push(e);
            }
        }
    }
    orig::<unsafe extern "thiscall" fn(usize, u32)>(1005)(this, id);
    let mut depois = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, depois.as_mut_ptr(), 29);
    let ok = depois[..0x1c].iter().all(|&b| b == 0xff) && depois[0x1c] == 0
        && entradas.iter().all(|&e| cpo13_d(e) as u32 == 0xffff_ffff);
    PLACAR.registrar(ok, || format!("tirar_jogador_da_disciplina({}) registro {:02x?} entradas {:x?}", id, depois, entradas));
}
