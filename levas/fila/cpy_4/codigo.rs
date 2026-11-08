// ---------- %%LEVA%%a leva: os amistosos internacionais do dia e a lista escolhida do olheiro do técnico humano (friendly.cpp, scout_manager.cpp) ----------

unsafe fn cpy4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpy4_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

/// As datas estáticas da 0x54c770 (valor e guarda) e da 0x54c5b0, 0x54c8e0 e 0x54cdb0.
const SCOUT_MANAGER_DATE_STATICS: [(usize, usize); 8] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1), (0xae2470, 8), (0xae2418, 1)];

/// O corpo da 0x5cbb00. No `ensaio` (o comparador), nada se grava e nada se chama além das consultas: devolve o par
/// (primeira entrada, quantas) que iria à 0x5cbdf0, ou None se a original não a chamaria.
unsafe fn update_friendlies_body(this: usize, ensaio: bool) -> Option<(usize, u32)> {
    // 0x54a110: thiscall(data) (lea ecx); 0x54c770: thiscall(0xae2c90, saída, 0xc) (mov ecx, 0xae2c90; 2 push);
    // 0x68d080: thiscall(lista) (mov ecx, lista; sem push); 0x945333: cdecl(vetor, conta, tamanho, comparação) (add esp,
    // 0x10); 0x5cbdd0: thiscall(entrada, ano) (mov ecx, entrada; 1 push); 0x417c10: thiscall(destino, saída, origem)
    // (lea ecx, destino; 2 push); 0x946464: cdecl(chave, vetor, conta, 0x14, 0x5cc610) (add esp, 0x14); 0x456770 e
    // 0x456740: thiscall(data, baixo, alto) (lea ecx, data; 2 push); 0x5cbdf0: thiscall(this, primeira, quantas) (mov
    // ecx, esi; 2 push)
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let semanas: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) -> usize = core::mem::transmute(0x54c770usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let data_da_entrada: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x5cbdd0usize);
    let copiar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let marcar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x5cbdf0usize);
    // a chave da busca: a data em +3, como as entradas de 20 bytes (a original passa a data local − 3)
    let mut chave = [0u8; 0x14];
    let d = chave.as_mut_ptr() as usize + 3;
    data_padrao(d);
    let mut lim = [0u8; 8];
    semanas(0xae2c90, &mut lim, 0xc); // cpy4: hoje + 12 semanas
    let hoje = cpy4_w(0xae2c90);
    if hoje == 0 && !ensaio {
        crate::comparison::branch("cpy_4 virada do ano");
        let mut i = 0i32;
        while i < cpy4_d(0xae2350) as i32 {
            let l = cpy4_d(cpy4_d(this + 0x34f6).wrapping_add(i as usize * 4));
            if l != 0 {
                limpar(l);
            }
            i += 1;
        }
        ordenar(cpy4_d(this + 0x34fa), cpy4_d(this + 0x34fe) as u32, 4, 0x5cc5f0); // cpy4: as nações pela reputação
        // o primeiro ano de Copa (ano − 2 múltiplo de 4) a partir deste; a metade alta do dword que a original soma é
        // sobra de pilha: aqui, zero
        let mut ano = cpy4_w(0xae2c92) as u32;
        while ((ano & 0xffff) as i32 - 2) % 4 != 0 {
            ano = ano.wrapping_add(1);
        }
        let mut k = 0i32;
        while k < cpy4_d(this + 0x34f0) as i32 {
            let e = cpy4_d(this + 0x34ec).wrapping_add(k as usize * 0x14);
            let mut a = ((*((e + 0xb) as *const i8)) as i16 as u16 as u32).wrapping_add(ano);
            if (a & 0xffff) as i32 - cpy4_w(0xae2c92) as i32 > 3 {
                a = a.wrapping_add(0xfffc);
            }
            data_da_entrada(e, a);
            k += 1;
        }
        ordenar(cpy4_d(this + 0x34ec), cpy4_d(this + 0x34f0) as u32, 0x14, 0x5cc610);
    }
    // a data de 0xdd6ec8 depois de hoje + 12 semanas: nada
    let x_ano = cpy4_w(0xdd6eca);
    let l_ano = core::ptr::read_unaligned(lim.as_ptr().add(2) as *const u16);
    if x_ano > l_ano || (x_ano == l_ano && *(0xdd6ec8 as *const i16) > core::ptr::read_unaligned(lim.as_ptr() as *const i16)) {
        return None;
    }
    if cpy4_w(this + 0x34f4) as u32 == hoje as i16 as i32 as u32 {
        return None;
    }
    let mut lixo = [0u8; 8];
    copiar(d, &mut lixo, lim.as_ptr() as usize);
    let base = cpy4_d(this + 0x34ec);
    let n = cpy4_d(this + 0x34f0);
    let p = bsearch(chave.as_ptr() as usize, base, n as u32, 0x14, 0x5cc610);
    if p == 0 {
        return None;
    }
    let igual = |e: usize| antes(d, cpy4_d(e + 3) as u32, cpy4_d(e + 7) as u32) == 0 && depois(d, cpy4_d(e + 3) as u32, cpy4_d(e + 7) as u32) == 0;
    // volta até a primeira entrada da data
    let mut e = p;
    while e > base {
        if !igual(e - 0x14) {
            break;
        }
        e -= 0x14;
    }
    let primeira = e;
    let ultima = base.wrapping_add(n.wrapping_mul(0x14)).wrapping_sub(0x14);
    let mut quantas = 0u32;
    while e <= ultima && igual(e) {
        quantas += 1;
        e += 0x14;
    }
    if ensaio {
        return Some((primeira, quantas));
    }
    crate::comparison::branch("cpy_4 amistosos marcados");
    marcar(this, primeira, quantas); // cpy4: marcar os amistosos da data
    *((this + 0x34f4) as *mut u16) = cpy4_w(0xae2c90);
    Some((primeira, quantas))
}

/// { 0x5cbb00 } (thiscall, ret 0, friendly.cpp, quadro SEH da data local de destrutor vazio): `Friendly::
/// update_international_friendlies()`, pelo dia dos amistosos (0x5c2460, já em Rust). No primeiro dia do ano: as listas
/// de nações ([+0x34f6], [0xae2350] delas) se esvaziam (0x68d080), a lista em +0x34fa (conta +0x34fe) se ordena pela
/// reputação (0x5cc5f0) e cada data internacional (+0x34ec, 20 bytes, conta +0x34f0) ganha o ano pelo ciclo da Copa
/// (ano − 2 múltiplo de 4, mais o deslocamento +0xb, menos 4 se passar de 3 anos à frente; 0x5cbdd0), reordenadas pela
/// data (0x5cc610). Depois, se a data de 0xdd6ec8 não passa de hoje + 12 semanas (0x54c770) e o dia não foi feito
/// (+0x34f4), as entradas com a data de hoje + 12 semanas (bsearch e as vizinhas iguais) vão à 0x5cbdf0 e o dia fica
/// marcado.
unsafe extern "thiscall" fn update_international_friendlies(this: usize) {
    update_friendlies_body(this, false);
}

/// Comparador: no primeiro dia do ano (as listas se esvaziam, liberando) e quando há amistosos a marcar (a 0x5cbdf0
/// cria jogos e sorteia), só a original, conferindo o plano: o dia marcado em +0x34f4. Nos outros dias, lado a lado
/// pelo dia marcado e pelas datas estáticas.
unsafe extern "thiscall" fn comparar_update_international_friendlies(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    if cpy4_w(0xae2c90) == 0 {
        return o(this);
    }
    if let Some((primeira, quantas)) = update_friendlies_body(this, true) {
        o(this);
        let dia = cpy4_w(this + 0x34f4);
        SCOREBOARD.record(dia == cpy4_w(0xae2c90), || format!("update_international_friendlies(primeira {:x}, quantas {}) dia marcado={} hoje={}", primeira, quantas, dia, cpy4_w(0xae2c90)));
        return;
    }
    let mut reg = SCOUT_MANAGER_DATE_STATICS.to_vec();
    reg.push((this + 0x34f4, 2));
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || update_international_friendlies(this));
    SCOREBOARD.record(igual, || format!("update_international_friendlies(sem amistosos) {:x?} x {:x?}", mo, mr));
}

/// O técnico (nulo: o humano da vez, [0xb74c26 + 0x300 × [0xb77c7a]]) e o índice dele entre os 16 humanos (id −
/// [0xae2364] + 16), ou None nos casos de erro da original (sem técnico ou fora de 0..15).
unsafe fn manager_human_index(tecnico: usize) -> Option<usize> {
    let t = if tecnico != 0 {
        tecnico
    } else {
        cpy4_d(0xb74c26usize.wrapping_add((*(0xb77c7a as *const i16) as i32 as usize).wrapping_mul(0x300)))
    };
    if t == 0 {
        return None;
    }
    let i = (cpy4_d(t) as i32).wrapping_sub(cpy4_d(0xae2364) as i32).wrapping_add(0x10);
    if !(0..=0xf).contains(&i) {
        return None;
    }
    Some(i as usize)
}

/// { 0x7f6e30 } (thiscall, ret 4, scout_manager.cpp): `SCOUT_MANAGER::get_selected_list(tecnico)`, pelas telas de
/// busca (0x804248). A lista escolhida pelo técnico humano (byte em +0x4c + índice). Sem técnico ou fora de 0..15: a
/// original (caixa "Error", 0xff).
unsafe extern "thiscall" fn get_selected_list(this: usize, tecnico: usize) -> u8 {
    let Some(i) = manager_human_index(tecnico) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%)(this, tecnico); // caixa: só a original
    };
    *((this + 0x4c + i) as *const u8)
}

/// Comparador: a devolução (rotina pura); nos casos de erro, só a original (caixa).
unsafe extern "thiscall" fn comparar_get_selected_list(this: usize, tecnico: usize) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize) -> u8>(%%I1%%);
    if manager_human_index(tecnico).is_none() {
        return o(this, tecnico);
    }
    let r = o(this, tecnico);
    let n = get_selected_list(this, tecnico);
    SCOREBOARD.record(r == n, || format!("get_selected_list(tecnico {:x}) original={} rust={}", tecnico, r, n));
    r
}

/// { 0x7f6f20 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::set_selected_list(lista, tecnico)`, pelas telas
/// de busca (0x804460). Grava a lista escolhida (byte) em +0x4c + índice do técnico humano e devolve 1. Sem técnico ou
/// fora de 0..15: a original (caixa "Error", 0).
unsafe extern "thiscall" fn set_selected_list(this: usize, lista: u32, tecnico: usize) -> u32 {
    let Some(i) = manager_human_index(tecnico) else {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(%%I2%%)(this, lista, tecnico); // caixa: só a original
    };
    *((this + 0x4c + i) as *mut u8) = lista as u8;
    1
}

/// Comparador: lado a lado pelos 16 bytes das listas escolhidas, e a devolução; nos casos de erro, só a original.
unsafe extern "thiscall" fn comparar_set_selected_list(this: usize, lista: u32, tecnico: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> u32>(%%I2%%);
    if manager_human_index(tecnico).is_none() {
        return o(this, lista, tecnico);
    }
    let reg = [(this + 0x4c, 0x10)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, lista, tecnico), || set_selected_list(this, lista, tecnico));
    SCOREBOARD.record(igual && r == n, || format!("set_selected_list(lista {}, tecnico {:x}) original={} rust={} {:x?} x {:x?}", lista as u8, tecnico, r, n, mo, mr));
    r
}
