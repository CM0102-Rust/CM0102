// ---------- 244a leva: o lugar do jogador no elenco e as subidas e descidas entre o principal e os reservas (contract_manager.cpp) ----------

unsafe fn cph2_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// O contrato da pessoa como a 0x4cbb60 acha por dentro (sem chamar a 0x4cf5a0): pessoa não nula, id abaixo de
/// [0xae2364], índice da tabela [0xae2d0c] (79 bytes por pessoa) não negativo; registro de 80 bytes em [objeto].
unsafe fn cph2_contrato(objeto: usize, pessoa: usize) -> usize {
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
unsafe extern "thiscall" fn lugar_no_elenco(this: usize, pessoa: usize, clube: usize) -> u8 {
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
    let c = cph2_contrato(cph2_d(0xae1050) as usize, pessoa);
    *(0xae1080 as *mut usize) = c; // a ficha do contrato
    if c == 0 {
        return 0;
    }
    let i = cph2_d(c + 4) as usize;
    let outro = (cph2_d(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
    if outro == 0 || outro == cph2_d(pessoa + 0x39) as usize || outro == b {
        return 0;
    }
    let c2 = cph2_contrato(this, pessoa);
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
unsafe fn cph2_mover(this: usize, pessoa: usize, clube: usize, de: [u8; 2]) -> u32 {
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
unsafe extern "thiscall" fn subir_para_o_principal(this: usize, pessoa: usize, clube: usize) -> u32 {
    cph2_mover(this, pessoa, clube, [1, 3]) // reservas ou time B
}

/// { 0x4cc0f0 } (thiscall, ret 8, contract_manager.cpp): `CONTRACT_MANAGER::move_to_reserves(pessoa, clube)`: no
/// principal (2) ou no clube com time B (4), troca (0x4cbd20) e devolve o que ela devolve; senão 0.
unsafe extern "thiscall" fn descer_para_os_reservas(this: usize, pessoa: usize, clube: usize) -> u32 {
    cph2_mover(this, pessoa, clube, [2, 4]) // principal ou clube com time B
}

/// Comparador: lado a lado pela ficha [0xae1080] (a vaga da saída é o argumento na pilha de cada uma).
unsafe extern "thiscall" fn comparar_lugar_no_elenco(this: usize, pessoa: usize, clube: usize) -> u8 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(829);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, pessoa, clube), || lugar_no_elenco(this, pessoa, clube));
    PLACAR.registrar(igual && r == n, || format!("lugar_no_elenco({:x}, {:x}) original={} rust={} {:x?} x {:x?}", pessoa, clube, r, n, mo, mr));
    r
}

/// Comparador das duas trocas: o lugar (0x4cbb60, pelo desvio) decide antes; fora dos lugares da troca, lado a lado
/// pela ficha [0xae1080] (as duas devolvem 0); nos lugares da troca, só a original (a 0x4cbd20 mexe no elenco).
unsafe fn cph2_comparar(i: usize, nome: &str, this: usize, pessoa: usize, clube: usize, de: [u8; 2]) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(i);
    let lugar: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x4cbb60usize);
    let ficha = cph2_d(0xae1080);
    let l = lugar(this, pessoa, clube);
    *(0xae1080 as *mut u32) = ficha;
    if l == de[0] || l == de[1] {
        return o(this, pessoa, clube);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(0xae1080, 4)], || o(this, pessoa, clube), || cph2_mover(this, pessoa, clube, de));
    PLACAR.registrar(igual && r == n, || format!("{}({:x}, {:x}) lugar {} original={} rust={} {:x?} x {:x?}", nome, pessoa, clube, l, r, n, mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_subir_para_o_principal(this: usize, pessoa: usize, clube: usize) -> u32 {
    cph2_comparar(830, "subir_para_o_principal", this, pessoa, clube, [1, 3])
}

unsafe extern "thiscall" fn comparar_descer_para_os_reservas(this: usize, pessoa: usize, clube: usize) -> u32 {
    cph2_comparar(831, "descer_para_os_reservas", this, pessoa, clube, [2, 4])
}

// ---------- 244a leva: a cópia do contrato de pessoal, a moral depois da boa partida e tirar as conversas de contrato (contract_manager.cpp) ----------

unsafe fn cph3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cph3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// Date::operator= (0x417c10): copia a data de `origem` para `destino`, com o retorno numa local (gravada pela própria).
unsafe fn cph3_data(destino: usize, origem: usize) {
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut local = [0u8; 8];
    copiar(destino, &mut local, origem);
}

/// { 0x4cc810 } (thiscall, ret 4, contract_manager.cpp): `STAFF_CONTRACT::operator=`: copia o contrato (0x50 bytes) de
/// `origem`: quatro duplas, três duplas (+0x10), cinco bytes (+0x1c), +0x21, as datas de início (+0x25) e do fim
/// (+0x2d) pela 0x417c10, o byte de campos de bits +0x35 em três passos (6 bits, o 7º, o 8º: no fim é o de `origem`),
/// +0x36, +0x3a, +0x3b, +0x3d, +0x3e, +0x3f, +0x40, +0x44, +0x45, +0x49, +0x4a, +0x4e e o +0x4f em dois passos (nibble
/// baixo e depois o resto). Devolve `this`.
unsafe extern "thiscall" fn copiar_contrato_de_pessoal(this: usize, origem: usize) -> usize {
    let d = |o: usize| core::ptr::write_unaligned((this + o) as *mut u32, cph3_d(origem + o));
    let w = |o: usize| core::ptr::write_unaligned((this + o) as *mut u16, core::ptr::read_unaligned((origem + o) as *const u16));
    let b = |o: usize| *((this + o) as *mut u8) = cph3_b(origem + o);
    for o in [0, 4, 8, 0xc, 0x10, 0x14, 0x18] {
        d(o);
    }
    for o in 0x1c..0x21 {
        b(o); // os cinco bytes
    }
    d(0x21);
    cph3_data(this + 0x25, origem + 0x25);
    cph3_data(this + 0x2d, origem + 0x2d);
    let m = (this + 0x35) as *mut u8;
    let s = cph3_b(origem + 0x35);
    let cl = ((((s << 2) as i8 >> 2) as u8 ^ *m) & 0x3f) ^ *m; // os 6 bits baixos
    *m = cl;
    let al = ((((cph3_b(origem + 0x35) << 1) as i8 >> 1) as u8 ^ cl) & 0x40) ^ cl; // o 7º bit
    *m = al;
    let s = cph3_b(origem + 0x35);
    *m = ((al ^ s) & 0x7f) ^ s; // o 8º bit
    d(0x36);
    b(0x3a);
    w(0x3b);
    b(0x3d);
    b(0x3e);
    b(0x3f);
    d(0x40);
    b(0x44);
    d(0x45);
    b(0x49);
    d(0x4a);
    b(0x4e);
    let m = (this + 0x4f) as *mut u8;
    let c = *m;
    let al = ((cph3_b(origem + 0x4f) ^ c) & 0xf) ^ c; // o nibble baixo
    *m = al;
    let s = cph3_b(origem + 0x4f);
    *m = ((al ^ s) & 0xf) ^ s;
    this
}

/// { 0x4cb0a0 } (stdcall, ret 0xc, contract_manager.cpp): `contract_manager_raise_match_morale(pessoa, clube,
/// contrato)`: com os três e o registro de jogador (+0x61): x = (nota +0xb − reputação do clube +0x80) + 417 × +0x18
/// − 11680; y = 1 + x × (6,6e-5 se x < 0, senão 6,6667e-5), entre 0,01 e 2; o ganho = ftol((+0x3f + 5) / y), no máximo
/// 10. O teto: 100 (50 com a queixa 2), −50 por queixa 1, 8, 0x20, 0x40, 0x80, 0x100 enquanto o teto não passa de −50
/// para baixo. A moral (+0x44) soma o ganho entre −100 e 100 e fica no teto. Sem +0x49: com ganho > 5 e moral + ganho
/// > 0, ou felicidade (+0x3d) ≥ 75 − ganho, apaga as queixas 4 e 0x10 (0x8674e0). Por fim confere a retirada do pedido
/// (0x867240 com 0). Constantes do .rdata; o x87 em dupla precisão, como as outras do contrato.
unsafe extern "stdcall" fn subir_moral_pela_partida(pessoa: usize, clube: usize, contrato: usize) {
    if contrato == 0 || clube == 0 || pessoa == 0 {
        return;
    }
    let jogador = cph3_d(pessoa + 0x61) as usize;
    if jogador == 0 {
        return;
    }
    let a = core::ptr::read_unaligned((jogador + 0xb) as *const i16) as i32 - core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32;
    let x = (a as f64 + cph3_b(pessoa + 0x18) as i8 as f64 * f64_em(0x968d90)) - f64_em(0x968d88);
    let mut y = (if x < f64_em(0x9689a0) { x * f64_em(0x968d80) } else { x * f64_em(0x968d78) }) + f64_em(0x967890);
    if y < f64_em(0x967898) {
        y = f64_em(0x967898);
    } else if y > f64_em(0x968950) {
        y = f64_em(0x968950);
    }
    let felicidade = (cph3_b(contrato + 0x3f) as i8 as i32 + 5) as f64; // +0x3f + 5
    let mut ganho = ftol(felicidade / y) as u8 as i8;
    if ganho > 10 {
        ganho = 10;
    }
    let queixas = cph3_d(contrato + 0x45);
    let mut teto: i8 = if queixas & 2 != 0 { 50 } else { 100 };
    if queixas & 1 != 0 {
        teto = teto.wrapping_sub(50);
    }
    for q in [8u32, 0x20, 0x40, 0x80, 0x100] {
        if teto < -50 {
            break;
        }
        if queixas & q != 0 {
            teto = teto.wrapping_sub(50); // menos 50 por queixa
        }
    }
    let moral = (contrato + 0x44) as *mut i8;
    let cl = *moral;
    let s = cl as i32 + ganho as i32;
    *moral = if s > 100 { 100 } else if s < -100 { -100 } else { cl.wrapping_add(ganho) };
    if *moral > teto {
        *moral = teto;
    }
    if cph3_b(contrato + 0x49) == 0 {
        let apaga = if ganho > 5 && *moral as i32 + ganho as i32 > 0 {
            true
        } else {
            cph3_b(contrato + 0x3d) as i8 as i32 >= 75 - ganho as i32
        };
        if apaga {
            let apagar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x8674e0usize);
            if queixas & 4 != 0 {
                apagar(contrato, 4);
            }
            if cph3_b(contrato + 0x45) & 0x10 != 0 {
                apagar(contrato, 0x10);
            }
        }
    }
    let retirada: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x867240usize);
    retirada(contrato, 0);
}

/// { 0x4ca1c0 } (thiscall, ret 0xc, contract_manager.cpp): `CONTRACT_MANAGER::remove_talks_records(pessoa, tecnico,
/// clube)`: no vetor de conversas ([this+0x28], [this+0x2c] registros de 21 bytes), cada um da pessoa (+0 = id) e,
/// quando dados, do clube (+4) e do técnico (+8) recebe por cima o último (5 duplas e um byte), a contagem desce e o
/// mesmo lugar é visto de novo. Sobrando algum, reordena (qsort 0x945333 com 0x4dc970).
unsafe extern "thiscall" fn tirar_conversas_de_contrato(this: usize, pessoa: usize, tecnico: usize, clube: usize) {
    let n = (this + 0x2c) as *mut i32;
    let mut i = 0i32;
    let mut off = 0usize;
    if *n > 0 {
        loop {
            let base = cph3_d(this + 0x28) as usize;
            let r = base.wrapping_add(off);
            let casa = cph3_d(r) == cph3_d(pessoa)
                && (clube == 0 || cph3_d(r + 4) == cph3_d(clube)) // o clube
                && (tecnico == 0 || cph3_d(r + 8) == cph3_d(tecnico));
            if casa {
                let ultimo = base.wrapping_add((*n as usize).wrapping_mul(21)).wrapping_sub(21);
                core::ptr::copy(ultimo as *const u8, r as *mut u8, 21);
                *n -= 1;
                i -= 1;
                off = off.wrapping_sub(21);
            }
            i += 1;
            off = off.wrapping_add(21);
            if i >= *n {
                break;
            }
        }
    }
    if *n != 0 {
        let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
        qsort(cph3_d(this + 0x28) as usize, *n, 0x15, 0x4dc970);
    }
}

unsafe extern "thiscall" fn comparar_copiar_contrato_de_pessoal(this: usize, origem: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(832);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x50)], || o(this, origem), || copiar_contrato_de_pessoal(this, origem));
    PLACAR.registrar(igual && r == n, || format!("copiar_contrato_de_pessoal {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: a 0x8674e0 e a 0x867240 mexem nas queixas, nos vínculos e na transferência (não podem rodar duas
/// vezes): só a original; a prova é a carreira e o ramo da cópia no modo Rust.
// ponytail: só a original (153.222 chamadas); comparar pedia separar a moral das chamadas do fim.
unsafe extern "stdcall" fn comparar_subir_moral_pela_partida(pessoa: usize, clube: usize, contrato: usize) {
    orig::<unsafe extern "stdcall" fn(usize, usize, usize)>(833)(pessoa, clube, contrato)
}

unsafe extern "thiscall" fn comparar_tirar_conversas_de_contrato(this: usize, pessoa: usize, tecnico: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(834);
    let n = (*((this + 0x2c) as *const i32)).max(0) as usize;
    let reg = [(this + 0x28, 8), (cph3_d(this + 0x28) as usize, n * 21)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, tecnico, clube), || tirar_conversas_de_contrato(this, pessoa, tecnico, clube));
    PLACAR.registrar(igual, || format!("tirar_conversas_de_contrato({:x}, {:x}, {:x}) {:x?} x {:x?}", pessoa, tecnico, clube, mo, mr));
}

// ---------- 244a leva: tirar de vetores ordenados: o interesse de transferência da mídia, o pedido de observação e o apelido de todos os técnicos humanos ----------

unsafe fn cph4_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cph4_qsort(base: usize, n: i32, tamanho: u32, comparar: usize) {
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(base, n, tamanho, comparar);
}

unsafe fn cph4_bsearch(chave: usize, base: usize, n: u32, tamanho: u32, comparar: usize) -> usize {
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    bsearch(chave, base, n, tamanho, comparar)
}

/// { 0x7549b0 } (thiscall, ret 4, media.cpp): `Media::remove_transfer_interest(pessoa)`: no vetor de interesses
/// ([this+0xc], [this+0x14] registros de 25 bytes), cada um da pessoa (+0 = id) recebe por cima o último (6 duplas e um
/// byte), a contagem desce e o mesmo lugar é visto de novo. Sobrando algum, reordena (qsort 0x945333 com 0x757ea0).
unsafe extern "thiscall" fn tirar_interesses_de_transferencia(this: usize, pessoa: usize) {
    let n = (this + 0x14) as *mut i32;
    let mut i = 0i32;
    let mut off = 0usize;
    if *n > 0 {
        loop {
            let base = cph4_d(this + 0xc) as usize;
            let r = base.wrapping_add(off);
            if cph4_d(r) == cph4_d(pessoa) {
                let ultimo = base.wrapping_add((*n as usize).wrapping_mul(25)).wrapping_sub(25); // o último interesse
                core::ptr::copy(ultimo as *const u8, r as *mut u8, 25);
                *n -= 1;
                i -= 1;
                off = off.wrapping_sub(25);
            }
            i += 1;
            off = off.wrapping_add(25);
            if i >= *n {
                break;
            }
        }
    }
    if *n != 0 {
        cph4_qsort(cph4_d(this + 0xc) as usize, *n, 0x19, 0x757ea0);
    }
}

/// { 0x7f8ee0 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::remove_scout_request(pessoa, clube)`: o clube
/// nulo é o do técnico humano atual ([0xb74c26 + 0x300 × [0xb77c7a]], +0x39; sem ele, nada). Com a pessoa, busca
/// (bsearch 0x946464 com 0x7fc270, que lê só +0 clube e +4 pessoa; a data padrão da chave em +8, o byte +0x10 sem valor
/// e a cópia usa zero) o pedido no vetor ([this+0x5c], [this+0x64] registros de 17 bytes); achou: puxa os seguintes um
/// lugar (+0, +4, a data +8 pela 0x417c10 e o byte +0x10), a contagem desce e, sobrando algum, reordena (qsort).
unsafe extern "thiscall" fn tirar_pedido_de_observacao(this: usize, pessoa: usize, clube: usize) {
    let mut chave = [0u8; 0x11];
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    padrao(chave.as_mut_ptr() as usize + 8);
    let mut c = clube;
    if c == 0 {
        let i = *(0xb77c7a as *const i16) as i32;
        let tecnico = cph4_d((0xb74c26 + i * 0x300) as usize) as usize; // o técnico humano atual
        if tecnico == 0 {
            return;
        }
        c = cph4_d(tecnico + 0x39) as usize;
        if c == 0 {
            return;
        }
    }
    if pessoa == 0 {
        return;
    }
    let id_clube = if c != 0 { cph4_d(c) } else { u32::MAX };
    chave[..4].copy_from_slice(&id_clube.to_le_bytes());
    chave[4..8].copy_from_slice(&cph4_d(pessoa).to_le_bytes());
    let base = cph4_d(this + 0x5c) as usize;
    let s = cph4_bsearch(chave.as_ptr() as usize, base, cph4_d(this + 0x64), 0x11, 0x7fc270);
    if s == 0 {
        return;
    }
    let mut k = ((s.wrapping_sub(base) as u32 as u64 * 0xf0f0f0f1) >> 36) as i32; // o lugar do pedido
    let copiar_data: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    while k < cph4_d(this + 0x64) as i32 - 1 {
        let r = (cph4_d(this + 0x5c) as usize).wrapping_add(k as usize * 17);
        core::ptr::write_unaligned(r as *mut u32, cph4_d(r + 0x11));
        core::ptr::write_unaligned((r + 4) as *mut u32, cph4_d(r + 0x15));
        let mut local = [0u8; 8];
        copiar_data(r + 8, &mut local, r + 0x19);
        *((r + 0x10) as *mut u8) = *((r + 0x21) as *const u8);
        k += 1;
    }
    let n = cph4_d(this + 0x64).wrapping_sub(1);
    core::ptr::write_unaligned((this + 0x64) as *mut u32, n); // um pedido a menos
    if n != 0 {
        cph4_qsort(cph4_d(this + 0x5c) as usize, n as i32, 0x11, 0x7fc270);
    }
}

/// { 0x5fff30 } (thiscall, ret 4, human_manager.cpp): `HUMAN_MANAGER::remove_nickname_from_all(id)`: em cada técnico
/// humano ([this], [this+4] registros de 0xa6ef bytes; contador em byte com sinal) não nulo, sem vetor de apelidos
/// (+0xa6e3) sai de vez; senão busca (bsearch com 0x601a10, que lê só o id em +0; a chave tem o byte +4 zerado e o resto
/// sem valor, a cópia usa zero) o apelido da pessoa `*id` entre os [+0xa6eb] de 30 bytes; achou: puxa os seguintes um
/// lugar, o último fica com id −1 e +4 = 0 e a contagem desce. No fim reordena (qsort) só o vetor do último técnico
/// visto (sem técnicos, o `id` faz as vezes do técnico, como no original).
unsafe extern "thiscall" fn tirar_apelido_de_todos(this: usize, id: usize) {
    if id == 0 {
        return;
    }
    let mut ultimo = id;
    if cph4_d(this + 4) as i32 > 0 {
        let mut c = 0i8;
        loop {
            let g = (cph4_d(this) as usize).wrapping_add((c as i32 as usize).wrapping_mul(0xa6ef));
            ultimo = g;
            if g != 0 {
                if cph4_d(g + 0xa6e3) == 0 {
                    return;
                }
                let mut chave = [0u8; 0x1e];
                chave[..4].copy_from_slice(&cph4_d(id).to_le_bytes());
                let vetor = cph4_d(g + 0xa6e3) as usize;
                let s = cph4_bsearch(chave.as_ptr() as usize, vetor, cph4_d(g + 0xa6eb), 0x1e, 0x601a10);
                if s != 0 {
                    let mut k = ((s.wrapping_sub(vetor) as u32 as u64 * 0x88888889) >> 36) as i32;
                    while k < cph4_d(g + 0xa6eb) as i32 - 1 {
                        let r = (cph4_d(g + 0xa6e3) as usize).wrapping_add(k as usize * 30);
                        core::ptr::copy((r + 30) as *const u8, r as *mut u8, 30); // o seguinte um lugar acima
                        k += 1;
                    }
                    let n = cph4_d(g + 0xa6eb) as usize;
                    let fim = (cph4_d(g + 0xa6e3) as usize).wrapping_add(n.wrapping_mul(30));
                    *((fim - 0x1a) as *mut u8) = 0;
                    core::ptr::write_unaligned((fim - 0x1e) as *mut u32, u32::MAX);
                    core::ptr::write_unaligned((g + 0xa6eb) as *mut u32, (n as u32).wrapping_sub(1));
                }
            }
            c = c.wrapping_add(1);
            if c as i32 >= cph4_d(this + 4) as i32 {
                break;
            }
        }
    }
    cph4_qsort(cph4_d(ultimo + 0xa6e3) as usize, cph4_d(ultimo + 0xa6eb) as i32, 0x1e, 0x601a10);
}

unsafe extern "thiscall" fn comparar_tirar_interesses_de_transferencia(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(835);
    let n = (*((this + 0x14) as *const i32)).max(0) as usize;
    let reg = [(this + 0xc, 0xc), (cph4_d(this + 0xc) as usize, n * 25)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa), || tirar_interesses_de_transferencia(this, pessoa));
    PLACAR.registrar(igual, || format!("tirar_interesses_de_transferencia({:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

unsafe extern "thiscall" fn comparar_tirar_pedido_de_observacao(this: usize, pessoa: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(836);
    let n = (*((this + 0x64) as *const i32)).max(0) as usize;
    let reg = [(this + 0x5c, 0xc), (cph4_d(this + 0x5c) as usize, n * 17)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, clube), || tirar_pedido_de_observacao(this, pessoa, clube));
    PLACAR.registrar(igual, || format!("tirar_pedido_de_observacao({:x}, {:x}) {:x?} x {:x?}", pessoa, clube, mo, mr));
}

/// Comparador: lado a lado pelos vetores de apelidos e contagens de todos os técnicos humanos; sem técnicos (o qsort
/// leria pelo `id`), só a original.
unsafe extern "thiscall" fn comparar_tirar_apelido_de_todos(this: usize, id: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(837);
    let n = cph4_d(this + 4) as i32;
    if id == 0 || n <= 0 {
        return o(this, id);
    }
    let mut reg = Vec::new();
    for c in 0..n.min(128) {
        let g = (cph4_d(this) as usize).wrapping_add(c as usize * 0xa6ef);
        if g != 0 && cph4_d(g + 0xa6e3) != 0 {
            reg.push((g + 0xa6e3, 0xc));
            reg.push((cph4_d(g + 0xa6e3) as usize, (cph4_d(g + 0xa6eb) as i32).max(0) as usize * 30));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, id), || tirar_apelido_de_todos(this, id));
    PLACAR.registrar(igual, || format!("tirar_apelido_de_todos({:x}) {:x?} x {:x?}", id, mo, mr));
}

// ---------- 244a leva: as aposentadorias (player_regen.cpp): tirar a pendente e a decisão do desempregado ----------

unsafe fn cph5_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// { 0x7a8690 } (thiscall, ret 4, player_regen.cpp): `PLAYER_REGEN::remove_retirement(chave)`: busca (bsearch 0x946464
/// com 0x7ae2a0) a aposentadoria pendente no vetor ([this+4], [this+8] registros de 26 bytes); achou: puxa as
/// seguintes um lugar (0x7a8780, a cópia do registro), a última fica com pessoa −1 e +0x18, +8, +0xa, +0x10, +0x12
/// zerados, e a contagem desce. Sem reordenar.
unsafe extern "thiscall" fn tirar_aposentadoria_pendente(this: usize, chave: usize) {
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let vetor = cph5_d(this + 4) as usize;
    let s = bsearch(chave, vetor, cph5_d(this + 8), 0x1a, 0x7ae2a0);
    if s == 0 {
        return;
    }
    let mut k = ((s.wrapping_sub(vetor) as u32 as u64 * 0x4ec4ec4f) >> 35) as i32; // o lugar (/ 26)
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7a8780usize);
    while k < cph5_d(this + 8) as i32 - 1 {
        let r = (cph5_d(this + 4) as usize).wrapping_add(k as usize * 26);
        copiar(r, r + 0x1a);
        k += 1;
    }
    let fim = (cph5_d(this + 4) as usize).wrapping_add((cph5_d(this + 8) as usize).wrapping_mul(26)).wrapping_sub(26);
    core::ptr::write_unaligned(fim as *mut u32, u32::MAX); // a última sem pessoa
    *((fim + 0x18) as *mut u8) = 0;
    for o in [8usize, 0xa, 0x10, 0x12] {
        core::ptr::write_unaligned((fim + o) as *mut u16, 0);
    }
    core::ptr::write_unaligned((this + 8) as *mut u32, cph5_d(this + 8).wrapping_sub(1));
}

/// Está na lista de observados (0x842080 no gerente da lista [[0xde1f54]+0x856], sem clube)?
unsafe fn cph5_observado(pessoa: usize) -> bool {
    let observado: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x842080usize);
    observado(cph5_d(cph5_d(0xde1f54) as usize + 0x856) as usize, pessoa, 0) != 0
}

unsafe fn cph5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// { 0x7a8050 } (stdcall, ret 8, player_regen.cpp): `player_regen_check_unemployed_retirement(pessoa, saida)`: quem
/// está na lista de observados não se aposenta. Com registro de técnico (+0x69): sem clube (+0x39) e sem +0x24, com
/// +0xa do registro abaixo de 50 e sorteio(5) = 0. Sem ele, com registro de jogador (+0x61): fora da lista (conferida
/// de novo), sem clube e sem +0x24, desempregado há mais de 365 dias (365 × (ano − ano de +0x40) − dia de +0x3e + dia)
/// e sorteio(nota +0xb do jogador) < 750. Sem nenhum dos dois registros: sempre. Aposenta: a saída (26 bytes) recebe a
/// pessoa, o motivo 0x10, +0x19 = 0, o clube (−1 sem) e a data do jogo em +8 e +0x10; devolve 1. Senão 0.
unsafe extern "stdcall" fn aposentadoria_do_desempregado(pessoa: usize, saida: usize) -> u8 {
    let mut sem_registro = false;
    if cph5_observado(pessoa) {
        return 0;
    }
    let tecnico = cph5_d(pessoa + 0x69) as usize;
    if tecnico == 0 && cph5_d(pessoa + 0x61) == 0 {
        sem_registro = true;
    }
    let aposenta = if tecnico != 0 {
        if cph5_d(pessoa + 0x39) != 0 || cph5_d(pessoa + 0x24) != 0 {
            return 0;
        }
        *((tecnico + 0xa) as *const i16) < 50 && cph5_sortear(5) == 0 // o técnico fraco
    } else if cph5_d(pessoa + 0x61) == 0 {
        false
    } else {
        if cph5_observado(pessoa) {
            return 0;
        }
        if cph5_d(pessoa + 0x39) != 0 || cph5_d(pessoa + 0x24) != 0 {
            false
        } else {
            let anos = (*(0xae2c92 as *const u16) as i32).wrapping_sub(*((pessoa + 0x40) as *const u16) as i32);
            let dias = anos.wrapping_mul(365).wrapping_sub(*((pessoa + 0x3e) as *const i16) as i32).wrapping_add(*(0xae2c90 as *const i16) as i32);
            if dias > 0x16d {
                let nota = *((cph5_d(pessoa + 0x61) as usize + 0xb) as *const i16) as i32;
                cph5_sortear(nota) < 0x2ee // o sorteio pela nota
            } else {
                false
            }
        }
    };
    if !aposenta && !sem_registro {
        return 0;
    }
    core::ptr::write_unaligned(saida as *mut u32, cph5_d(pessoa));
    *((saida + 0x18) as *mut u8) = 0x10; // o motivo: desempregado
    *((saida + 0x19) as *mut u8) = 0;
    let clube = cph5_d(pessoa + 0x39) as usize;
    core::ptr::write_unaligned((saida + 4) as *mut u32, if clube != 0 { cph5_d(clube) } else { u32::MAX });
    for o in [8usize, 0x10] {
        core::ptr::write_unaligned((saida + o) as *mut u16, *(0xae2c90 as *const u16));
        core::ptr::write_unaligned((saida + o + 2) as *mut u16, *(0xae2c92 as *const u16));
    }
    1
}

unsafe extern "thiscall" fn comparar_tirar_aposentadoria_pendente(this: usize, chave: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(838);
    let n = (*((this + 8) as *const i32)).max(0) as usize;
    let reg = [(this + 4, 8), (cph5_d(this + 4) as usize, n * 26)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, chave), || tirar_aposentadoria_pendente(this, chave));
    PLACAR.registrar(igual, || format!("tirar_aposentadoria_pendente {:x?} x {:x?}", mo, mr));
}

/// Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio e pela saída (26 bytes), a cópia em sem_contar.
unsafe extern "stdcall" fn comparar_aposentadoria_do_desempregado(pessoa: usize, saida: usize) -> u8 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize) -> u8>(839);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((saida, 0x1a));
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(pessoa, saida),
        || crate::aleatorio::sem_contar(|| aposentadoria_do_desempregado(pessoa, saida)));
    PLACAR.registrar(igual && r == n, || format!("aposentadoria_do_desempregado({:x}) original={} rust={} {:x?} x {:x?}", pessoa, r, n, mo, mr));
    r
}

// ---------- 244a leva: as permissões de trabalho (transfer_manager.cpp): a oferta que precisa de permissão e a atualização anual ----------

unsafe fn cph6_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cph6_pessoa(i: u32) -> usize {
    (cph6_d(0xae23bc) as usize).wrapping_add((i as usize).wrapping_mul(110))
}

unsafe fn cph6_clube(i: u32) -> usize {
    let i = i as usize;
    (cph6_d(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4))
}

/// A pessoa precisa de permissão para jogar no clube (0x8c7560, original, com 0 = sem ignorar as dadas)?
unsafe fn cph6_precisa(this: usize, pessoa: usize, clube: usize) -> bool {
    let precisa: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x8c7560usize);
    precisa(this, pessoa, clube, 0) != 0
}

/// { 0x8c7470 } (thiscall, ret 4, transfer_manager.cpp): `transfer_manager_offer_needs_work_permit(oferta)`: 1 se o
/// jogador da oferta (+0xc) precisa de permissão no clube da oferta (+8), ou se algum dos jogadores de troca (a lista
/// em +4, do começo: o cursor +0xa volta ao +2 e anda pela 0x68d1b0) precisa no clube do contrato dele (0x4cf5a0 em
/// [0xae1050], gravado em [0xae1080]; sem contrato, clube nulo); senão 0.
unsafe extern "thiscall" fn oferta_precisa_de_permissao(this: usize, oferta: usize) -> u32 {
    if cph6_precisa(this, cph6_pessoa(cph6_d(oferta + 0xc)), cph6_clube(cph6_d(oferta + 8))) {
        return 1; // o jogador da oferta
    }
    let lista = cph6_d(oferta + 4) as usize;
    if lista == 0 {
        return 0;
    }
    core::ptr::write_unaligned((lista + 0xa) as *mut u32, cph6_d(lista + 2)); // o cursor no começo
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let contrato_de: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let mut item = proximo(lista);
    while item != 0 {
        let c = contrato_de(cph6_d(0xae1050) as usize, cph6_pessoa(cph6_d(item)));
        *(0xae1080 as *mut usize) = c;
        let clube = if c != 0 { cph6_clube(cph6_d(c + 4)) } else { 0 };
        if cph6_precisa(this, cph6_pessoa(cph6_d(item)), clube) {
            return 1;
        }
        item = proximo(lista);
    }
    0
}

/// { 0x8c7330 } (thiscall, transfer_manager.cpp): `TRANSFER_MANAGER::update_work_permits`: no vetor de permissões
/// ([this+0x93a], [this+0x93e] registros de 14 bytes: +0 pessoa, +8 ano, +0xa nação), a marca da pessoa (byte em
/// [this+0x86e] + 8 × id) sem o bit 0x20 é zerada inteira; com mais de 4 anos (ano do jogo − +8) a pessoa ganha a
/// segunda nacionalidade (+0x1e = nação +0xa, 0x122 bytes em [0xae23a8]), perde o bit 0x20 e a permissão sai (pessoa
/// −1); quem não é jogador (+0x61) também perde. Saindo alguma: reordena (qsort 0x945333 com 0x8e55c0) e desconta as
/// do fim com pessoa −1.
unsafe extern "thiscall" fn atualizar_permissoes_de_trabalho(this: usize) {
    let n = (this + 0x93e) as *mut i32;
    let mut saiu = false;
    let mut i = 0i32;
    let mut off = 0usize;
    while i < *n {
        let r = (cph6_d(this + 0x93a) as usize).wrapping_add(off);
        let pessoa = cph6_pessoa(cph6_d(r));
        let marca = |p: usize| ((cph6_d(this + 0x86e) as usize).wrapping_add((cph6_d(p) as usize).wrapping_mul(8))) as *mut u8;
        if *marca(pessoa) & 0x20 == 0 {
            *marca(pessoa) = 0; // a marca sem permissão
        }
        let r = (cph6_d(this + 0x93a) as usize).wrapping_add(off);
        let anos = (*(0xae2c92 as *const u16) as i32).wrapping_sub(*((r + 8) as *const u16) as i32);
        if anos > 4 {
            let nacao = cph6_d(r + 0xa) as usize;
            core::ptr::write_unaligned((pessoa + 0x1e) as *mut usize, (cph6_d(0xae23a8) as usize).wrapping_add(nacao.wrapping_mul(0x122)));
            *marca(pessoa) &= 0xdf;
            core::ptr::write_unaligned(((cph6_d(this + 0x93a) as usize).wrapping_add(off)) as *mut u32, u32::MAX);
            saiu = true;
        } else if cph6_d(pessoa + 0x61) == 0 {
            *marca(pessoa) &= 0xdf;
            core::ptr::write_unaligned(((cph6_d(this + 0x93a) as usize).wrapping_add(off)) as *mut u32, u32::MAX);
            saiu = true; // quem não é jogador
        }
        i += 1;
        off = off.wrapping_add(0xe);
    }
    if !saiu {
        return;
    }
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cph6_d(this + 0x93a) as usize, *n, 0xe, 0x8e55c0);
    let mut k = *n - 1;
    while k >= 0 {
        let r = (cph6_d(this + 0x93a) as usize).wrapping_add(k as usize * 0xe);
        if cph6_d(r) != u32::MAX {
            break;
        }
        *n -= 1;
        k -= 1;
    }
}

/// Comparador: lado a lado pela ficha [0xae1080] e pelo cursor (+0xa) da lista de troca.
unsafe extern "thiscall" fn comparar_oferta_precisa_de_permissao(this: usize, oferta: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(840);
    let lista = cph6_d(oferta + 4) as usize;
    let mut reg = vec![(0xae1080, 4)];
    if lista != 0 {
        reg.push((lista + 0xa, 4));
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, oferta), || oferta_precisa_de_permissao(this, oferta));
    PLACAR.registrar(igual && r == n, || format!("oferta_precisa_de_permissao({:x}) original={} rust={} {:x?} x {:x?}", oferta, r, n, mo, mr));
    r
}

/// Comparador: lado a lado pelo vetor, pela contagem e, de cada permissão, pela marca da pessoa e pela segunda
/// nacionalidade (+0x1e). Permissões com pessoa negativa não entram nas regiões (o original leria fora do vetor).
unsafe extern "thiscall" fn comparar_atualizar_permissoes_de_trabalho(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(841);
    let n = (*((this + 0x93e) as *const i32)).max(0) as usize;
    let vetor = cph6_d(this + 0x93a) as usize;
    let mut reg = vec![(this + 0x93a, 8), (vetor, n * 0xe)];
    for k in 0..n {
        let id = cph6_d(vetor + k * 0xe);
        if (id as i32) < 0 {
            continue;
        }
        let pessoa = cph6_pessoa(id);
        reg.push(((cph6_d(this + 0x86e) as usize).wrapping_add((cph6_d(pessoa) as usize).wrapping_mul(8)), 1));
        reg.push((pessoa + 0x1e, 4));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || atualizar_permissoes_de_trabalho(this));
    PLACAR.registrar(igual, || format!("atualizar_permissoes_de_trabalho {:x?} x {:x?}", mo, mr));
}

// ---------- 244a leva: o olheiro da busca, a sequência atual do clube, construir o recordista e o técnico rival que comenta na mídia ----------

unsafe fn cph7_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cph7_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// { 0x7ae950 } (thiscall, player_search.cpp): `PLAYER_SEARCH::get_scout`: o olheiro da busca: o índice em +0x180 (> −1)
/// no vetor de pessoas [0xae23bc] (110 bytes), se ainda confere com os nomes e a data guardados (0x53e990 com +0x174,
/// +0x178, +0x17c e a data em +0x184); senão 0.
unsafe extern "thiscall" fn olheiro_da_busca(this: usize) -> usize {
    let i = cph7_d(this + 0x180) as i32;
    if i <= -1 {
        return 0;
    }
    let pessoa = (cph7_d(0xae23bc) as usize).wrapping_add((i as usize).wrapping_mul(110));
    let confere: unsafe extern "C" fn(usize, u32, u32, u32, usize) -> u32 = core::mem::transmute(0x53e990usize);
    if confere(pessoa, cph7_d(this + 0x174), cph7_d(this + 0x178), cph7_d(this + 0x17c), this + 0x184) == 0 {
        return 0; // a vaga mudou de pessoa
    }
    pessoa
}

/// { 0x690c30 } (thiscall, ret 0xc, manager_manager.cpp): `MANAGER_MANAGER::get_current_run(clube, tipo, tamanho)`:
/// a sequência atual do clube na liga (+0x57; a competição pelo id em [0xadadfc], estatísticas em +8, a linha do clube
/// pela 0x4a4850): a primeira de mais de 1 jogo cujo último foi há menos de 10 dias (0x54cdb0 da data do jogo), na
/// ordem +0x19 (tipo 1, data +0x25), +0x2d (3, +0x39), +0x55 (4, +0x61) e +0x41 (2, +0x4d): o tipo e o tamanho (byte).
/// Nenhuma (ou sem liga, competição, estatísticas ou linha): tipo 5 e tamanho 0. O `this` não é usado.
unsafe extern "thiscall" fn sequencia_atual_do_clube(_this: usize, clube: usize, tipo: *mut u8, tamanho: *mut u8) {
    let liga = cph7_d(clube + 0x57) as usize;
    let comp = if liga != 0 { cph7_d((cph7_d(0xadadfc) as usize).wrapping_add((cph7_d(liga) as usize).wrapping_mul(4))) as usize } else { 0 };
    let stats = if comp != 0 { cph7_d(comp + 8) as usize } else { 0 };
    if stats != 0 {
        let linha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
        let e = linha(stats, clube);
        if e != 0 {
            let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
            for (n, data, t) in [(0x19usize, 0x25usize, 1u8), (0x2d, 0x39, 3), (0x55, 0x61, 4), (0x41, 0x4d, 2)] {
                if cph7_d(e + n) as i32 > 1 && dias(0xae2c90, cph7_d(e + data), cph7_d(e + data + 4)) < 10 { // há menos de 10 dias
                    *tipo = t;
                    *tamanho = *((e + n) as *const u8);
                    return;
                }
            }
        }
    }
    *tipo = 5; // nenhuma
    *tamanho = 0;
}

/// { 0x450d40 } (thiscall, club_records.cpp): `record_holder_construct`: constrói o recordista: só a data em +0x10
/// (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recordista(this: usize) -> usize {
    let padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    padrao(this + 0x10); // a data do recordista
    this
}

/// O técnico (+0xcf) de um clube rival passa nos sorteios (sorteio(+0x5c) = 0 e sorteio(+0x57) > 5)?
unsafe fn cph7_sorteado(c: usize) -> bool {
    cph7_sortear(*((cph7_d(c + 0xcf) as usize + 0x5c) as *const i8) as i32) == 0
        && cph7_sortear(*((cph7_d(c + 0xcf) as usize + 0x57) as *const i8) as i32) > 5
}

/// { 0x72b940 } (thiscall, ret 4, media.cpp): `Media::pick_rival_manager_id(clube)`: com técnico (+0xcf) com contrato
/// (+0x69) e liga (+0x57; sem liga, só seleção, 0x53b900): o primeiro dos rejeitados do técnico (0x5480f0) com
/// registro de técnico e nota + 750 < nota (nunca: o sorteio(3) dali não acontece); depois cada rival (+0xb3, +0xb7,
/// +0xbb) com técnico que passa nos sorteios, da mesma liga e de reputação (+0x80) acima da do clube + 750, com
/// sorteio(3) ≠ 0; depois até 50 clubes sorteados (de [0xae235c] − 2 × [0xae2350]) que dão confronto na liga
/// (0x4b02e0, com o limite do sorteio como 3º argumento, resto da pilha do original) com técnico que passa nos
/// sorteios. Devolve o id do técnico, ou −1.
unsafe extern "thiscall" fn tecnico_rival_para_comentar(_this: usize, clube: usize) -> u32 {
    if cph7_d(clube + 0xcf) == 0 {
        return u32::MAX;
    }
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    if cph7_d(clube + 0x57) == 0 && selecao(clube) == 0 {
        return u32::MAX;
    }
    let tecnico = cph7_d(clube + 0xcf) as usize;
    if cph7_d(tecnico + 0x69) == 0 {
        return u32::MAX;
    }
    let mut rejeitados = [0usize; 20];
    let mut n = clube as u32; // a vaga do argumento: a 0x5480f0 zera o byte antes de contar
    let rejeitadas: unsafe extern "C" fn(usize, usize, usize) -> usize = core::mem::transmute(0x5480f0usize);
    rejeitadas(tecnico, rejeitados.as_mut_ptr() as usize, &mut n as *mut u32 as usize);
    let mut k = 0i8;
    while k < n as u8 as i8 {
        let p = *rejeitados.as_ptr().add(k as u8 as usize); // como o original, sem conferir o tamanho (a 0x5480f0 dá até 3)
        if p != 0 && cph7_d(p + 0x69) != 0 {
            let nota = *((cph7_d(p + 0x69) as usize + 0xa) as *const i16) as i32;
            if nota.wrapping_add(0x2ee) < nota && cph7_sortear(3) != 0 {
                return cph7_d(p);
            }
        }
        k = k.wrapping_add(1);
    }
    for rival in [0xb3usize, 0xb7, 0xbb] {
        let r = cph7_d(clube + rival) as usize;
        if r == 0 || cph7_d(r + 0xcf) == 0 || !cph7_sorteado(r) {
            continue;
        }
        if cph7_d(r + 0x57) != cph7_d(clube + 0x57) {
            continue;
        }
        let reputacao = |c: usize| *((c + 0x80) as *const i16) as i32;
        if reputacao(clube) + 0x2ee >= reputacao(r) {
            continue; // o rival não é bem maior
        }
        if cph7_sortear(3) != 0 {
            return cph7_d(cph7_d(r + 0xcf) as usize);
        }
    }
    let confronto: unsafe extern "C" fn(usize, usize, i32) -> u8 = core::mem::transmute(0x4b02e0usize);
    let mut b = 0i8;
    while b < 0x32 {
        let limite = (cph7_d(0xae235c) as i32).wrapping_sub((cph7_d(0xae2350) as i32).wrapping_mul(2));
        let i = cph7_sortear(limite) as usize;
        let c = (cph7_d(0xae23b4) as usize).wrapping_add(i).wrapping_add(i.wrapping_mul(145).wrapping_mul(4));
        if confronto(clube, c, limite) != 0 && cph7_d(c + 0xcf) != 0 && cph7_sorteado(c) {
            return cph7_d(cph7_d(c + 0xcf) as usize);
        }
        b += 1;
    }
    u32::MAX
}

// a 0x53e990 só lê: compara o valor
comparar_valor!(comparar_olheiro_da_busca, olheiro_da_busca, 842, () -> usize);

/// Comparador: a original grava nas saídas de verdade, a cópia em locais; a 0x4a4850 pode reordenar as estatísticas
/// (a original primeiro; a cópia acha a tabela já em ordem e a mesma linha).
unsafe extern "thiscall" fn comparar_sequencia_atual_do_clube(this: usize, clube: usize, tipo: *mut u8, tamanho: *mut u8) {
    orig::<unsafe extern "thiscall" fn(usize, usize, *mut u8, *mut u8)>(843)(this, clube, tipo, tamanho);
    let (mut t, mut n) = (0u8, 0u8);
    sequencia_atual_do_clube(this, clube, &mut t, &mut n);
    PLACAR.registrar(*tipo == t && *tamanho == n, || format!("sequencia_atual_do_clube({:x}) original={}/{} rust={}/{}", clube, *tipo, *tamanho, t, n));
}

unsafe extern "thiscall" fn comparar_construir_recordista(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(844);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x10, 8)], || o(this), || construir_recordista(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recordista {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: sorteia (0x90cfc0): lado a lado pelo estado do sorteio, a cópia em sem_contar (as escritas são só nas
/// locais da pilha de cada uma).
unsafe extern "thiscall" fn comparar_tecnico_rival_para_comentar(this: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(845);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(this, clube),
        || crate::aleatorio::sem_contar(|| tecnico_rival_para_comentar(this, clube)));
    PLACAR.registrar(igual && r == n, || format!("tecnico_rival_para_comentar({:x}) original={:#x} rust={:#x} {:x?} x {:x?}", clube, r, n, mo, mr));
    r
}
