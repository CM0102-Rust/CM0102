// ---------- %%LEVA%%a leva: a confiança da diretoria: o negócio da transferência, a confiança depois da transferência e o ultimato (manager_manager.cpp) ----------

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
unsafe extern "thiscall" fn assess_transfer_deal(this: usize, transf: usize, f1: usize, f2: usize) {
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
unsafe extern "thiscall" fn apply_transfer_confidence_changes(this: usize, transf: usize) {
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
    let get_club_finance: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let fin = get_club_finance(cpo1_d(0xae2a38), cpo1_d(pessoa + 0x39));
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
unsafe extern "thiscall" fn resolve_board_ultimatum(this: usize, clube: usize) -> u8 {
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
unsafe extern "thiscall" fn comparar_assess_transfer_deal(this: usize, transf: usize, f1: usize, f2: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I0%%);
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
        || crate::aleatorio::sem_contar(|| assess_transfer_deal(this, transf, f1, f2)));
    PLACAR.registrar(igual, || format!("assess_transfer_deal({:x}) {:x?} x {:x?}", transf, mo, mr));
}

/// Comparador: lado a lado com o estado do sorteador (a avaliação 0x696570 sorteia), as linhas da diretoria do
/// comprador e do vendedor e as três pessoas favoritas do presidente do vendedor (0x547c10); a cópia sem contar.
unsafe extern "thiscall" fn comparar_apply_transfer_confidence_changes(this: usize, transf: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%);
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
        || crate::aleatorio::sem_contar(|| apply_transfer_confidence_changes(this, transf)));
    PLACAR.registrar(igual, || format!("apply_transfer_confidence_changes({:x}) {:x?} x {:x?}", transf, mo, mr));
}

/// Comparador: a original tira o item da lista (e o libera) e acrescenta a saída: roda só ela e confere o plano: a
/// lista das saídas (+4) cresce de 1 quando devolve 1 (e a linha da diretoria fica com o bit 2) e fica igual com 0.
unsafe extern "thiscall" fn comparar_resolve_board_ultimatum(this: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I2%%);
    let saidas = cpo1_d(this + 4);
    if saidas == 0 || cpo1_d(this + 0x10) == 0 {
        return o(this, clube);
    }
    let antes = cpo1_w(saidas) as i32;
    let r = o(this, clube);
    let depois = cpo1_w(cpo1_d(this + 4)) as i32;
    let ok = depois == antes + (r != 0) as i32 && (r == 0 || *((cpo1_linha(this, cpo1_d(clube)) + 2) as *const u8) & 2 != 0);
    PLACAR.registrar(ok, || format!("resolve_board_ultimatum(clube {:x}) devolveu {}, saídas {} -> {}", clube, r, antes, depois));
    r
}
