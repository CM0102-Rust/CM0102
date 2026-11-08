// ---------- %%LEVA%%a leva: a verba de transferências repartida pelas vagas da lista de observação, o estatuto permitido pela idade e o nome das listas do olheiro (shortlist_manager.cpp, scout_manager.cpp) ----------

unsafe fn cpy3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpy3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy3_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

unsafe fn cpy3_f64(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// A pessoa de índice `i` ([0xae23bc], 0x6e bytes; a conta da original em 32 bits, com índice negativo inclusive).
unsafe fn shortlist_manager_staff_by_index(i: i32) -> usize {
    cpy3_d(0xae23bc).wrapping_add((i as usize).wrapping_mul(0x6e))
}

/// { 0x83b6e0 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::allocate_transfer_budget(clube)`, pela
/// atualização da lista do clube (0x83a730, já em Rust), pelas listas iniciais (0x83a1b0) e pela lista refeita
/// (0x84abe0). Clube do humano (0x600a50 com 0, 1), seleção (0x53b900), sem finanças (0x5a0590) ou lista fora do estado
/// 1: nada. A verba (0x598800 com 0, 0, 0, 1, 0; negativa: 0) se reparte pelas vagas abertas (alvo −1) depois de
/// ordenar as 40 (qsort 0x83b690): entre as primeiras 8 (ou até a quarta vaga de jogador aberta), cada vaga aberta
/// recebe em +7 a verba / (abertas × fator do tipo e do estatuto); o resto da verba passa pelas vagas ainda sem alvo
/// (reduções pelos lances guardados, pelo valor de +0xb sobre a soma e pelos fatores de 0x969540, 0x968968), cada uma
/// preenchida pela 0x83f3a0; o que sobra vai à primeira vaga de jogador do próprio clube que o técnico aceitaria vender
/// (0x846920 acima de 0x968e58, sorteios de 10 e 100 dias, ofertas 0x8c5cf0).
unsafe extern "thiscall" fn allocate_transfer_budget(this: usize, clube: usize) {
    // 0x600a50: cdecl(clube, 0, 1, 0, 0) (add esp, 0x14); 0x53b900: cdecl(clube) (add esp, 4); 0x5a0590:
    // thiscall([0xae2a38], clube); 0x598800: thiscall(finanças, 0, 0, 0, 1, 0) (5 push); 0x945333: cdecl(vetor, 0x28,
    // 0x25, 0x83b690) (add esp, 0x10); 0x8c61e0: thiscall([0xde1f54], lance) (1 push); 0x83f3a0: thiscall(this, vaga,
    // clube, 0) (mov ecx, [this local]; 3 push); 0x4cf5a0: thiscall([0xae1050], pessoa); 0x544590: cdecl(pessoa, 0)
    // (add esp, 8); 0x4cf5f0: thiscall([0xae1050], pessoa, clube) (2 push); 0x846920: thiscall(this, pessoa, clube, 1, 0,
    // estatuto, 0) (6 push, devolve em st0); 0x90cfc0: cdecl(n) (add esp, 4); 0x85fa00: thiscall(contrato) (sem push,
    // movsx ax); 0x8c5cf0: thiscall([0xde1f54], pessoa, 0, 0) (3 push); 0x54c8e0: thiscall(0xae2c90, saída, dias)
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let financas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5a0590usize);
    let verba_de: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x598800usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let lance_de: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(0x8c61e0usize);
    let preencher: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x83f3a0usize);
    if humano(clube, 0, 1, 0, 0) != 0 || selecao(clube) != 0 {
        return;
    }
    let fin = financas(cpy3_d(0xae2a38), clube);
    if fin == 0 {
        return;
    }
    let mut verba = verba_de(fin, 0, 0, 0, 1, 0) as i32;
    if verba < 0 {
        verba = 0;
    }
    let entrada = cpy3_d(this + 0xdc).wrapping_add(cpy3_d(clube).wrapping_mul(9));
    if cpy3_b(entrada + 4) != 1 {
        return;
    }
    ordenar(cpy3_d(entrada), 0x28, 0x25, 0x83b690);
    let vaga = |i: i32| cpy3_d(entrada).wrapping_add((i as usize).wrapping_mul(0x25));
    // as vagas abertas entre as primeiras 8, ou até a quarta vaga de jogador aberta
    let mut limit: i8 = 8;
    let mut abertas: i8 = 0;
    let mut jogadores: u8 = 0;
    let mut soma: i32 = 0;
    let mut i: i8 = 0;
    loop {
        let v = vaga(i as i32);
        if cpy3_i(v + 0xf) == -1 {
            if cpy3_b(v) == 0xb {
                jogadores = jogadores.wrapping_add(1);
                if jogadores == 4 {
                    limit = i.wrapping_add(1);
                }
            }
            soma = soma.wrapping_add(cpy3_i(v + 0xb));
            abertas = abertas.wrapping_add(1);
        }
        i = i.wrapping_add(1);
        if i >= limit {
            break;
        }
    }
    let a = abertas as i32;
    let mut repartido: i32 = 0;
    if limit > 0 {
        for k in 0..limit as i32 {
            let v = vaga(k);
            if cpy3_i(v + 0xf) != -1 {
                continue;
            }
            let t = cpy3_b(v);
            let divisor = if t == 0xb {
                match ((cpy3_b(v + 0x24) >> 1) & 0xf).wrapping_sub(2) {
                    0 | 1 => {
                        crate::comparacao::ramo("cpy_3 verba da vaga de titular");
                        Some(a * 2) // cpy3: a verba da vaga de titular
                    }
                    2 => Some(a * 4),
                    3 => Some(a * 10),
                    _ => None,
                }
            } else {
                if cpy3_i(v + 0x13) != -1 {
                    continue;
                }
                let rico = cpy3_b(clube + 0x64) == 1;
                match (t as i8 as i32).wrapping_sub(6) {
                    0 | 2 => Some(if rico { a * 10 } else { 0 }),
                    1 => Some(if rico { a * 1000 } else { 0 }),
                    3 | 4 => Some(if rico { a * 20 } else { 0 }),
                    _ => None,
                }
            };
            let Some(dv) = divisor else { continue };
            let x = if dv == 0 { 0 } else { verba.wrapping_div(dv) };
            cpy3_pi(v + 7, x);
            repartido = repartido.wrapping_add(x);
        }
    }
    let mut resto = verba.wrapping_sub(repartido);
    let mut baixos: i8 = 0;
    if limit > 0 {
        for k in 0..limit as i32 {
            let v = vaga(k);
            let t = cpy3_b(v);
            if t != 0xb && cpy3_i(v + 0x13) > -1 {
                continue;
            }
            let alvo = cpy3_i(v + 0xf);
            if alvo > -1 || t == 7 {
                continue;
            }
            let lance = cpy3_i(v + 0x17);
            if lance > -1 {
                if alvo == -1 {
                    cpy3_pi(v + 0x17, -1);
                } else if t == 0xb {
                    let l = cpy3_d(lance_de(cpy3_d(0xde1f54), lance));
                    if l != 0 {
                        resto = resto.wrapping_sub(cpy3_i(l + 0x1b));
                    }
                } else {
                    resto = resto.wrapping_sub(cpy3_i(shortlist_manager_staff_by_index(alvo) + 0x52));
                }
                continue;
            }
            if cpy3_b(clube + 0x64) != 1 && t != 0xb {
                cpy3_pi(v + 7, 0);
            } else {
                let base = if repartido >= 1000 { repartido } else { 1000 };
                let e = cpy3_i(v + 7);
                let x = match_pl_ftol((e as f64) / (base as f64) * (resto as f64) * cpy3_f64(0x969540));
                crate::comparacao::ramo("cpy_3 reducao pela sobra");
                cpy3_pi(v + 7, e.wrapping_sub(x)); // cpy3: a redução da vaga pela sobra
            }
            let marca = cpy3_b(v + 0x24) & 0x1e;
            if marca <= 6 {
                if baixos < 3 {
                    baixos = baixos.wrapping_add(1);
                    let s = if soma >= 1 { soma } else { 1 };
                    let b = cpy3_i(v + 0xb);
                    let b = if b >= 1 { b } else { 1 };
                    let x = match_pl_ftol((resto as f64) / (s as f64) * (b as f64));
                    cpy3_pi(v + 7, cpy3_i(v + 7).wrapping_add(x));
                }
            } else {
                let s = if soma >= 1 { soma } else { 1 };
                let b = cpy3_i(v + 0xb);
                let b = if b >= 1 { b } else { 1 };
                let x = match_pl_ftol((resto as f64) / (s as f64) * (b as f64) * cpy3_f64(0x968968));
                cpy3_pi(v + 7, cpy3_i(v + 7).wrapping_add(x));
            }
            *((v + 0x24) as *mut u8) &= 0xfe;
            crate::comparacao::ramo("cpy_3 vaga preenchida");
            preencher(this, v, clube, 0); // cpy3: a vaga é preenchida
            let alvo = cpy3_i(v + 0xf);
            if alvo > -1 && cpy3_b(v + 0x24) & 1 == 0 {
                resto = resto.wrapping_add(match_pl_ftol(cpy3_i(shortlist_manager_staff_by_index(alvo) + 0x52) as f64 * cpy3_f64(0x96f878)));
            }
            soma = soma.wrapping_sub(cpy3_i(v + 0xb));
        }
    }
    // a sobra vai à primeira vaga de jogador do próprio clube que o técnico aceitaria vender
    if limit >= 0x28 {
        return;
    }
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestado: unsafe extern "C" fn(usize, u32) -> u32 = core::mem::transmute(0x544590usize);
    let contrato_no: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let interesse: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, u32) -> f64 = core::mem::transmute(0x846920usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let fim: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x85fa00usize);
    let ofertas: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c5cf0usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mut k = limit;
    while resto > 0 {
        let v = vaga(k as i32);
        let alvo = cpy3_i(v + 0xf);
        let candidata = 'c: {
            if cpy3_b(v) != 0xb || alvo <= -1 || cpy3_i(v + 0x17) != -1 {
                break 'c false;
            }
            let p = shortlist_manager_staff_by_index(alvo);
            if cpy3_d(p + 0x39) != clube {
                break 'c false;
            }
            let c = contract_of(cpy3_d(0xae1050), p);
            *(0xae1080 as *mut usize) = c;
            let do_contrato = if c != 0 { cpy3_d(0xae23b4).wrapping_add(cpy3_d(c + 4).wrapping_mul(0x245)) } else { 0 };
            if do_contrato == clube || emprestado(p, 0) != 0 {
                break 'c false;
            }
            let c2 = contrato_no(cpy3_d(0xae1050), p, clube);
            let estatuto = if c2 != 0 { (cpy3_b(c2 + 0x4f) >> 4) as u32 } else { 4 };
            let f = interesse(this, p, clube, 1, 0, estatuto, 0);
            if !(f >= cpy3_f64(0x968e58)) {
                break 'c false;
            }
            if c2 != 0 {
                let lim = 20 - sorteio(10);
                if (fim(c2) as i16 as i32) < lim {
                    break 'c true;
                }
            }
            if ofertas(cpy3_d(0xde1f54), p, 0, 0) != 0 {
                break 'c true;
            }
            if c2 == 0 {
                break 'c false;
            }
            let mut buf = [0u8; 8];
            let r = menos_dias(0xae2c90, &mut buf, (sorteio(100) + 0x1e) as u32);
            let dia = core::ptr::read_unaligned(r as *const i16);
            let ano = core::ptr::read_unaligned((r + 2) as *const u16);
            let ano_c = core::ptr::read_unaligned((c2 + 0x27) as *const u16);
            ano > ano_c || (ano == ano_c && dia > core::ptr::read_unaligned((c2 + 0x25) as *const i16))
        };
        if candidata {
            crate::comparacao::ramo("cpy_3 sobra ao jogador do clube");
            cpy3_pi(v + 0xb, cpy3_i(v + 3));
            cpy3_pi(v + 7, resto);
            return;
        }
        k = k.wrapping_add(1);
        if k >= 0x28 {
            return;
        }
    }
}

// ponytail: a 0x83f3a0 (chamada por cada vaga ainda sem alvo) sorteia, acrescenta pares aos vetores com realocação e dá
// notícias: não pode rodar duas vezes. Só a original; a cópia leva os ramos e as mutações.
unsafe extern "thiscall" fn comparar_allocate_transfer_budget(this: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%)(this, clube)
}

/// { 0x84b4d0 } (stdcall, ret 8, shortlist_manager.cpp; chamada com ecx = o Shortlist_Manager, que ela não lê):
/// `shortlist_manager_squad_status_allowed(pessoa, estatuto)`, pela tela de contrato (0x4dd0f0) e pela de transferência
/// (0x8f6770). Os estatutos 5 e 6 (jovens) só abaixo de 24 anos (+0x18, com sinal); os outros, 1. Sem ficha de jogador
/// (+0x61): a original (caixa "Error", 0).
unsafe extern "stdcall" fn shortlist_manager_squad_status_allowed(pessoa: usize, estatuto: u32) -> u32 {
    if cpy3_d(pessoa + 0x61) == 0 {
        return original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u32>(%%I1%%)(pessoa, estatuto); // caixa: só a original
    }
    match estatuto as u8 {
        5 | 6 => ((cpy3_b(pessoa + 0x18) as i8) < 0x18) as u32,
        _ => 1,
    }
}

/// Comparador: a devolução (rotina pura); sem ficha de jogador, só a original (caixa).
unsafe extern "stdcall" fn comparar_shortlist_manager_squad_status_allowed(pessoa: usize, estatuto: u32) -> u32 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize, u32) -> u32>(%%I1%%);
    if cpy3_d(pessoa + 0x61) == 0 {
        return o(pessoa, estatuto);
    }
    let r = o(pessoa, estatuto);
    let n = shortlist_manager_squad_status_allowed(pessoa, estatuto);
    PLACAR.registrar(r == n, || format!("shortlist_manager_squad_status_allowed(pessoa {:x}, estatuto {}) original={} rust={}", pessoa, estatuto as i8, r, n));
    r
}

/// { 0x7fc150 } (cdecl, scout_manager.cpp): `scout_manager_write_list_name(indice, texto, tamanho)`, pelo construtor do
/// SCOUT_MANAGER (0x7f6760) e pelas telas de busca. O nome traduzido (0x66f4e0) da lista `indice`: 0 "Short List", 1
/// "Player Search", 2 "Staff Search", 3..9 "Scout #<%d - number>" com `indice` − 2. Devolve 1. Índice fora de 0..9,
/// texto nulo ou tamanho abaixo de 50: a original (caixa "Error", 0).
unsafe extern "C" fn scout_manager_write_list_name(indice: u32, texto: usize, tamanho: i32) -> u32 {
    // 0x66f4e0: cdecl(texto, formato, ...) (add esp, 8 ou 0xc)
    let k = indice as u8 as i8;
    if !(0..=9).contains(&k) || texto == 0 || tamanho < 0x32 {
        return original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u32>(%%I2%%)(indice, texto, tamanho); // caixa: só a original
    }
    let traduzir: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    match k {
        0 => traduzir(texto, 0xa7dd94),
        1 => traduzir(texto, 0xa7dd84),
        2 => traduzir(texto, 0xa7dd74),
        _ => traduzir(texto, 0xa7dd5c, k as i32 - 2),
    };
    1
}

/// Comparador: lado a lado pelo texto (até 0x100 bytes do tamanho dado), e a devolução; nos casos de erro, só a
/// original (caixa).
unsafe extern "C" fn comparar_scout_manager_write_list_name(indice: u32, texto: usize, tamanho: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, i32) -> u32>(%%I2%%);
    let k = indice as u8 as i8;
    if !(0..=9).contains(&k) || texto == 0 || tamanho < 0x32 {
        return o(indice, texto, tamanho);
    }
    let reg = [(texto, (tamanho as usize).min(0x100))];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(indice, texto, tamanho), || scout_manager_write_list_name(indice, texto, tamanho));
    PLACAR.registrar(igual && r == n, || format!("scout_manager_write_list_name(indice {}) original={} rust={} {:x?} x {:x?}", k, r, n, mo, mr));
    r
}
