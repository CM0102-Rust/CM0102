// ---------- %%LEVA%%a leva: os jogos e as faltas das permissões de trabalho e os prêmios por jogos depois da partida, o prêmio por jogos na seleção guardado e a notícia da transferência finalizada (transfer_manager.cpp) ----------

unsafe fn cpy11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy11_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpy11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpy11_clube(id: usize) -> usize {
    cpy11_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpy11_pessoa(id: usize) -> usize {
    cpy11_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O byte das marcas da pessoa no registro de 8 bytes da gerência de transferências ([this+0x86e] + id × 8).
unsafe fn staff_flags_record(this: usize, p: usize) -> usize {
    cpy11_d(this + 0x86e).wrapping_add(cpy11_d(p).wrapping_mul(8))
}

/// A permissão de trabalho da pessoa (0xe bytes, [this+0x93a], conta +0x93e), como a original procura em linha: id fora
/// dos [0xae2364] (com sinal) ou sem a marca 0x20, 0; senão a busca binária (bsearch 0x946464 com 0x8e55c0).
unsafe fn transfer_manager_find_work_permit(this: usize, p: usize) -> usize {
    if p != 0 && cpy11_d(p) as i32 >= cpy11_d(0xae2364) as i32 {
        return 0;
    }
    if cpy11_b(staff_flags_record(this, p)) & 0x20 == 0 {
        return 0;
    }
    let bsearch: unsafe extern "C" fn(*const u32, usize, usize, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let chave = [cpy11_d(p) as u32];
    bsearch(chave.as_ptr(), cpy11_d(this + 0x93a), cpy11_d(this + 0x93e), 0xe, 0x8e55c0)
}

/// Soma 1 à palavra `o` da permissão (+4 jogos, +6 faltas). A original soma sem conferir a busca (a marca garante a
/// permissão); sem ela, a original cairia: aqui nada.
unsafe fn increment_permit_counter(r: usize, o: usize) {
    if r != 0 {
        let a = (r + o) as *mut u16;
        core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).wrapping_add(1));
    }
}

/// O ensaio do comparador: a cópia roda sem pagar os prêmios por jogos (0x8cb340 dá notícia, aloca e paga), como se
/// nenhum pagasse (sem tirar o bônus); a contagem das permissões não depende deles.
static mut APPEARANCE_CLAUSES_DRY_RUN: bool = false;

/// TRANSFER_MANAGER::pay_appearance_money (0x8cb340: thiscall(this, pessoa, jogos, tipo, 0), ret 0x10); no ensaio, 0.
unsafe fn pay_appearance_money_unless_dry_run(this: usize, p: usize, jogos: i32, tipo: u32) -> u32 {
    if APPEARANCE_CLAUSES_DRY_RUN {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, i32, u32, u32) -> u32 = core::mem::transmute(0x8cb340usize);
    f(this, p, jogos, tipo, 0)
}

/// { 0x8c39f0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::update_appearance_clauses_after_match(jogo,
/// partida)`, pelo fim de cada partida (0x6b5f00 e as estatísticas simuladas 0x84be20). Para os dois lados (+0x1c casa,
/// +0x20 fora): o jogo conta para as permissões de trabalho com competição (+0x14) que não é de seleção (casa pela
/// 0x53b900) nem amistoso (0x5c6710) e o byte do lado (+0x40/+0x41) ligado, ou com +0 = −2. Clube que não é seleção: o
/// elenco (0x541f90, 50 vagas; seleção: o vetor fica como estava) e, contando, os jogadores dele com a marca 0x20 (o
/// registro de 8 bytes em +0x86e). Nas 20 vagas da partida (+0x190 o id, +0x1ad ≠ −5, reservas com +0x1c2 ≠ −1; sem
/// ficha de jogador +0x61, fim das vagas): fora do empréstimo (0x4cf6d0, só no clube) e com a marca da cláusula (2
/// clube, 4 seleção), os jogos (seleção: +0x22 menos o byte +7 do registro; clube: +0x3b do contrato 0x4cf5a0) múltiplos
/// de 10 até 50, ou 1, pagam o prêmio (0x8cb340 com o tipo 5 ou 2) e, pago, o bônus sai (0x8cb630 ou 0x8cb590). Contando,
/// o jogador marcado soma um jogo (+4 da permissão) e sai da lista; os que sobram, sem lesão (0x62c400), suspensão
/// (0x55d270), convocação (0x7757d0) nem inelegibilidade (0x8cf5e0), e não emprestados por outro clube (0x4cf6d0 com o
/// contrato guardado em [0xae1080]), somam uma falta (+6).
unsafe extern "thiscall" fn update_appearance_clauses_after_match(this: usize, jogo: usize, partida: usize) {
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let amistoso: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
    let reunir: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let tirar_selecao: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb630usize);
    let tirar_time: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8cb590usize);
    let lesao: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x62c400usize);
    let suspenso: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x55d270usize);
    let convocado: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, usize) -> usize = core::mem::transmute(0x7757d0usize);
    let inelegivel: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x8cf5e0usize);
    // ponytail: o elenco local da original (esp+0x110) não é preenchido no lado da seleção e fica com o que a pilha tinha
    // (o do outro lado ou lixo); aqui começa zerado (vaga vazia, que o jogo também produz) e passa de um lado ao outro
    let mut elenco = [0usize; 50];
    let mut lado: i8 = 0;
    while lado < 2 {
        let clube = cpy11_d(jogo + if lado == 0 { 0x1c } else { 0x20 });
        let comp = cpy11_d(jogo + 0x14);
        let fora = comp == 0 || is_national_team(cpy11_d(jogo + 0x1c)) != 0 || amistoso(cpy11_d(0xae2c74), comp) != 0;
        let contando = (!fora && cpy11_b(jogo + 0x40 + lado as usize) != 0) || cpy11_d(jogo) as i32 == -2;
        let mascara: u8 = if is_national_team(clube) != 0 {
            4
        } else {
            reunir(clube, elenco.as_mut_ptr(), 0);
            2
        };
        let mut seguidos = [0usize; 50];
        let mut n: i8 = 0;
        if contando {
            for &p in elenco.iter() {
                if p != 0 && (cpy11_d(p) as i32) < cpy11_d(0xae2364) as i32 && cpy11_b(staff_flags_record(this, p)) & 0x20 != 0 {
                    seguidos[n as usize] = p;
                    n += 1;
                }
            }
        }
        let base = partida + lado as usize * 0x625;
        let mut k: i8 = 0;
        while k < 0x14 {
            let v = base + k as usize * 0x49;
            let id = cpy11_d(v + 0x190) as i32;
            if id != -1 && cpy11_b(v + 0x1ad) != 0xfb && (k < 0xb || cpy11_b(v + 0x1c2) != 0xff) {
                let p = cpy11_pessoa(id as usize);
                if cpy11_d(p + 0x61) == 0 {
                    break;
                }
                if (p == 0 || (cpy11_d(p) as i32) < cpy11_d(0xae2364) as i32) && !(mascara == 2 && ficha(cpy11_d(0xae1050), p) != 0) {
                    let reg = staff_flags_record(this, p);
                    if cpy11_b(reg) & mascara != 0 {
                        let (jogos, tipo): (i16, u8) = if mascara == 4 {
                            ((cpy11_b(p + 0x22) as u16).wrapping_sub(cpy11_b(reg + 7) as u16) as i16, 5)
                        } else {
                            let mut j = 0i16;
                            if contrato(cpy11_d(0xae1050), p) != 0 {
                                j = cpy11_w(contrato(cpy11_d(0xae1050), p) + 0x3b);
                            }
                            (j, 2)
                        };
                        // ponytail: a original empurra o dword do local do tipo, com os 3 bytes altos de lixo da pilha;
                        // a 0x8cb340 e a notícia só leem o byte: vai limpo
                        if (jogos as i32 % 10 == 0 && jogos <= 0x32) || jogos == 1 { // cpy11: os jogos do prêmio
                            if pay_appearance_money_unless_dry_run(this, p, jogos as i32, tipo as u32) != 0 {
                                crate::comparison::branch("cpy_11 premio pago");
                                if tipo == 5 {
                                    tirar_selecao(this, p);
                                } else {
                                    tirar_time(this, p);
                                }
                            }
                        }
                    }
                    if contando && cpy11_b(staff_flags_record(this, p)) & 0x20 != 0 {
                        let r = transfer_manager_find_work_permit(this, p);
                        crate::comparison::branch("cpy_11 jogo contado");
                        increment_permit_counter(r, 4); // cpy11: o jogo conta na permissão
                        let mut i: i8 = 0;
                        while i < n {
                            if seguidos[i as usize] == p {
                                seguidos[i as usize] = 0;
                                break;
                            }
                            i += 1;
                        }
                    }
                }
            }
            k += 1;
        }
        if contando && n > 0 {
            for i in 0..n as usize {
                let p = seguidos[i];
                if p == 0 || lesao(cpy11_d(0xb63d28), p) != 0 {
                    continue;
                }
                let comp = cpy11_d(jogo + 0x14);
                if comp != 0 && suspenso(cpy11_d(0xae28f0), p, comp, 1, 0) != 0 {
                    continue;
                }
                if convocado(cpy11_d(0xb74318), p, 0xae2c90, clube, 0, 0) != 0 {
                    continue;
                }
                if cpy11_d(jogo + 0x14) != 0 && inelegivel(this, p, jogo) != 0 {
                    continue;
                }
                if ficha(cpy11_d(0xae1050), p) != 0 {
                    let c = contrato(cpy11_d(0xae1050), p);
                    *(0xae1080 as *mut usize) = c;
                    let dono = if c == 0 { 0 } else { cpy11_clube(cpy11_d(c + 4)) };
                    if dono == clube {
                        continue;
                    }
                }
                let r = transfer_manager_find_work_permit(this, p);
                crate::comparison::branch("cpy_11 falta contada");
                increment_permit_counter(r, 6); // cpy11: a falta conta na permissão
            }
        }
        lado += 1;
    }
}

/// Comparador: a cópia em ensaio (sem pagar os prêmios) e a original a partir do mesmo estado do vetor das permissões
/// de trabalho; os jogos (+4) e as faltas (+6) têm de bater. Seleção em casa: o elenco local da original começa com o
/// lixo da pilha, só a original roda. Os prêmios se provam pela própria 0x8cb340 (comparada) e pela carreira.
unsafe extern "thiscall" fn comparar_update_appearance_clauses_after_match(this: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let v = cpy11_d(this + 0x93a);
    let n = cpy11_d(this + 0x93e) as i32;
    if is_national_team(cpy11_d(jogo + 0x1c)) != 0 || v == 0 || n <= 0 {
        return o(this, jogo, partida); // ponytail: só a original (lixo da pilha ou sem permissões)
    }
    let regioes = [(v, n as usize * 0xe)];
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    APPEARANCE_CLAUSES_DRY_RUN = true;
    crate::rng::pause_trace(|| crate::rng::without_counting(|| update_appearance_clauses_after_match(this, jogo, partida)));
    APPEARANCE_CLAUSES_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regioes);
    antes.write_back();
    o(this, jogo, partida);
    // a original não realoca o vetor (os prêmios mexem em outros vetores)
    let mo = crate::comparison::MemorySnapshot::read(&regioes);
    PLACAR.record(mo == mr, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).take(16).collect();
        format!("update_appearance_clauses_after_match(jogo {:x}) permissões diferentes nos bytes {:x?}", jogo, dif)
    });
}

/// { 0x8cad90 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::add_international_team_appearance_money(
/// registro)`: acrescenta o prêmio por jogos na seleção (13 bytes copiados de `registro`) ao vetor [this+0x898]
/// (capacidade +0x89c, conta +0x8a0); cheio, cresce de 50 em 50 (sem memória: caixa 0x25af e nada). Reordena pela pessoa
/// (qsort 0x945333 com 0x8b4e00). A gêmea do time principal é a 0x8cac40.
unsafe extern "thiscall" fn add_international_team_appearance_money(this: usize, registro: usize) {
    crate::comparison::branch("cpy_11 premio na selecao guardado");
    if !grow_record_vector(this, 0x898, 4, 50, 0xd, 0x25af, 0xa8ebe0) {
        return;
    }
    let n = cpy11_d(this + 0x8a0);
    core::ptr::copy_nonoverlapping(registro as *const u8, cpy11_d(this + 0x898).wrapping_add(n.wrapping_mul(0xd)) as *mut u8, 0xd);
    core::ptr::write_unaligned((this + 0x8a0) as *mut usize, n.wrapping_add(1));
    let qsort: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpy11_d(this + 0x898), cpy11_d(this + 0x8a0) as u32, 0xd, 0x8b4e00);
}

/// Comparador: a original acrescenta (pode realocar) e não pode rodar duas vezes: o plano antes (a conta e os 13
/// bytes), a original roda; confere a conta e que o registro está no vetor (reordenado).
unsafe extern "thiscall" fn comparar_add_international_team_appearance_money(this: usize, registro: usize) {
    let n = cpy11_d(this + 0x8a0);
    let alvo = core::slice::from_raw_parts(registro as *const u8, 0xd).to_vec();
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, registro);
    if cpy11_d(this + 0x898) == 0 {
        return; // sem memória: a caixa já saiu
    }
    let conta = cpy11_d(this + 0x8a0);
    let ok = conta == n.wrapping_add(1) && vector_contains_record(cpy11_d(this + 0x898), conta, 0xd, &alvo);
    PLACAR.record(ok, || format!("add_international_team_appearance_money: conta {} -> {}, registro {:x?}", n, conta, alvo));
}

/// { 0x8da5e0 } (thiscall, ret 4, transfer_manager.cpp, quadro SEH da data e da notícia locais): `TRANSFER_MANAGER::
/// post_transfer_finalised_news(oferta)`, pela transferência acertada (0x8c2f50). O vendedor: o clube do contrato da
/// pessoa (0x4cf5a0, guardado em [0xae1080]) fora do tipo 5 (pedido de novo, sem guardar). A notícia 0xfa5: a pessoa (o
/// id e as três chaves), o comprador, o vendedor (ou −1), +0x20, +0x24, o valor +0x1b, o estado +0x2c, o total
/// (0x8e7a70 com 0, 1, 0, 1), +0x2f, +0x11; a data de conclusão (0x8cee70) depois de hoje vai nos campos 0xe (dia), 0xf
/// (ano) e 0xd (o nibble baixo de +0x30, com sinal), senão 0, 0, 0; a conta da troca (+4) no 0xb. Ao vendedor humano
/// (0x7875e0), fora do estado 0xc e sem troca pendente (+0x2f com nibble +0x30), e ao comprador humano, cada um no
/// campo 0x10.
unsafe extern "thiscall" fn post_transfer_finalised_news(this: usize, o: usize) {
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let data_padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x8e7a70usize);
    let conclusao: unsafe extern "thiscall" fn(usize, usize, *mut u8) -> u16 = core::mem::transmute(0x8cee70usize);
    // a original põe o jornal no ecx; a 0x7875e0 é stdcall (ret 4) e não o lê
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let p = cpy11_pessoa(cpy11_d(o + 0xc));
    let c = contrato(cpy11_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    let mut vendedor = if c == 0 || cpy11_b(c + 0x35) & 0x3f == 5 { 0 } else { cpy11_clube(cpy11_d(c + 4)) };
    let comprador = cpy11_clube(cpy11_d(o + 8));
    let mut data = [0u8; 8];
    data_padrao(data.as_mut_ptr());
    if vendedor != 0 && cpy11_b(contrato(cpy11_d(0xae1050), p) + 0x35) & 0x3f == 5 {
        vendedor = 0;
    }
    crate::comparison::branch("cpy_11 noticia da transferencia finalizada");
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nt = local.as_mut_ptr() as usize;
    construir(nt, 0xfa5, 0);
    gravar(nt, 0, cpy11_d(p) as u32);
    gravar(nt, 1, cpy11_d(cpy11_d(p + 4) + 0x33) as u32);
    gravar(nt, 2, cpy11_d(cpy11_d(p + 8) + 0x33) as u32);
    gravar(nt, 3, cpy11_d(cpy11_d(p + 0xc) + 0x33) as u32);
    gravar(nt, 4, cpy11_d(o + 8) as u32);
    gravar(nt, 5, if vendedor != 0 { cpy11_d(vendedor) as u32 } else { u32::MAX });
    gravar(nt, 6, cpy11_d(o + 0x20) as u32);
    gravar(nt, 7, cpy11_d(o + 0x24) as u32);
    gravar(nt, 8, cpy11_d(o + 0x1b) as u32);
    gravar(nt, 9, cpy11_b(o + 0x2c) as i8 as u32);
    gravar(nt, 0xa, total(o, 0, 1, 0, 1));
    gravar(nt, 0xc, cpy11_b(o + 0x2f) as i8 as u32);
    gravar(nt, 0x11, cpy11_b(o + 0x11) as i8 as u32);
    conclusao(this, o, data.as_mut_ptr());
    let (dia, ano) = (i16::from_le_bytes([data[0], data[1]]), u16::from_le_bytes([data[2], data[3]]));
    let (hoje_dia, hoje_ano) = (cpy11_w(0xae2c90), cpy11_w(0xae2c92) as u16);
    if hoje_ano < ano || (hoje_ano == ano && hoje_dia < dia) {
        gravar(nt, 0xe, dia as i32 as u32);
        gravar(nt, 0xf, ano as u32);
        gravar(nt, 0xd, (((cpy11_b(o + 0x30) << 4) as i8) >> 4) as i32 as u32);
    } else {
        gravar(nt, 0xe, 0);
        gravar(nt, 0xf, 0);
        gravar(nt, 0xd, 0);
    }
    let troca = cpy11_d(o + 4);
    gravar(nt, 0xb, if troca != 0 { cpy11_b(troca) as i8 as u32 } else { 0 });
    let jornal = cpy11_d(0xb7431c);
    if vendedor != 0 && humano(jornal, vendedor) != 0 && cpy11_b(o + 0x2c) != 0xc && (cpy11_b(o + 0x2f) == 0 || cpy11_b(o + 0x30) & 0xf == 0) {
        gravar(nt, 0x10, cpy11_d(vendedor) as u32);
        entregar(cpy11_d(0xb7431c), nt, vendedor);
    }
    if humano(cpy11_d(0xb7431c), comprador) != 0 {
        gravar(nt, 0x10, cpy11_d(comprador) as u32);
        entregar(cpy11_d(0xb7431c), nt, comprador);
    }
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_post_transfer_finalised_news(this: usize, o: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, o)
}
