// ---------- %%LEVA%%a leva: os amistosos marcados: criar os amistosos planejados da semana, acrescentar um amistoso e sortear a data livre do amistoso (friendly.cpp) ----------

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
unsafe fn alloc_empty_list() -> usize {
    let novo: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944cf1usize);
    let criar: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let p = novo(0xe);
    if p != 0 { criar(p) } else { 0 }
}

/// Acrescenta à lista `l` uma cópia de `n` bytes de `p` (0x68d0d0); devolve a cópia (0 se falhou).
unsafe fn list_append_copy(l: usize, p: usize, n: u32) -> usize {
    let copiar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x68d0d0usize);
    copiar(l, p, n)
}

/// A situação da data `data` para o clube (0x5c75a0 Friendly::get_club_date_status, com as três saídas em locais, sem
/// datas escolhidas, tipo 0 e conferindo o estádio): 1 = livre.
unsafe fn club_date_status(this: usize, clube: usize, data: usize) -> u8 {
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
unsafe extern "thiscall" fn create_due_planned_friendlies(this: usize) {
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
            if club_date_status(this, cps12_clube(b), pdata) == 1 && club_date_status(this, cps12_clube(c), pdata) == 1 {
                let e = cps12_d(this + 0x34d8).wrapping_add(cps12_d(this + 0x34dc).wrapping_mul(20));
                crate::comparison::branch("cps_12 amistoso planejado");
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
unsafe fn stadium_busy_on(clube: usize, data: usize) -> bool {
    let estadio = cps12_d(clube + 0x69);
    if estadio == 0 {
        return false;
    }
    let ocupado: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x5ae870usize);
    // ponytail: a original empurra o dia e o ano (palavras) com sobra de registrador em cima; a 0x5ae870 lê as palavras
    ocupado(cps12_d(0xae2a58), estadio, cps12_w(data) as u32, cps12_w(data + 2) as u32, u32::MAX) != 0
}

/// O estádio de quem joga em casa (o clube com `em_casa`, senão o adversário, se houver) está ocupado na data?
unsafe fn home_stadium_busy_on(clube: usize, data: usize, adversario: usize, em_casa: u32) -> bool {
    if em_casa != 0 {
        stadium_busy_on(clube, data)
    } else {
        adversario != 0 && stadium_busy_on(adversario, data)
    }
}

/// { 0x5c6f00 } (thiscall, ret 0x10, friendly.cpp): `Friendly::add_friendly_match(clube, data, adversario, em_casa)`:
/// reaproveita o convite aberto do clube (0x5c88f0 sem data) ou monta um amistoso novo de 0x19 bytes (0x944cf1;
/// 0x54a110; sem memória: a caixa, linha 0xe6b, e 0). Se o estádio de quem joga em casa está ocupado na data (0x5ae870):
/// 0 (o novo fica sem liberar, como na original). Grava a data (0x417c10), +0xc o clube, +0x18 = 1, +8 = 0 e o mandante
/// (+0x10) e o visitante (+0x14). Sem a lista dos amistosos (+0x34c8), cria (0x68d020; sem memória: a caixa, linha
/// 0xe9f, libera o amistoso e 0). O reaproveitado volta; o novo vai em cópia para a lista (0x68d0d0; falhou: a caixa,
/// linha 0xeac, e 0), é liberado (0x944c94) e a cópia volta.
unsafe extern "thiscall" fn add_friendly_match(this: usize, clube: usize, data: usize, adversario: usize, em_casa: u32) -> usize {
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
            show_error_at(0x9cd0a0, 0xe6b);
            return 0;
        }
    }
    if home_stadium_busy_on(clube, data, adversario, em_casa) {
        return 0;
    }
    let atribuir: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut copia = [0u8; 8];
    cps12_pd(m + 0xc, clube);
    atribuir(m, &mut copia, data);
    *((m + 0x18) as *mut u8) = 1;
    cps12_pd(m + 8, 0);
    let (mandante, visitante) = if em_casa != 0 {
        crate::comparison::branch("cps_12 amistoso em casa");
        (clube, adversario) // cps12: o clube em casa
    } else {
        (adversario, clube)
    };
    cps12_pd(m + 0x10, mandante);
    cps12_pd(m + 0x14, visitante);
    if cps12_d(this + 0x34c8) == 0 {
        let l = alloc_empty_list();
        cps12_pd(this + 0x34c8, l);
        if l == 0 {
            show_error_at(0x9cd0a0, 0xe9f);
            liberar(m);
            return 0;
        }
    }
    if reaproveitado {
        return m;
    }
    let c = list_append_copy(cps12_d(this + 0x34c8), m, 0x19);
    if c == 0 {
        show_error_at(0x9cd0a0, 0xeac);
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
unsafe extern "thiscall" fn plan_friendly_on_free_date(this: usize, anfitriao: usize, casa: usize, fora: usize) -> u32 {
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
    crate::comparison::branch("cps_12 data livre");
    marcar(this, anfitriao, casa, fora, pdata);
    if cps12_d(casa + 0x12) == 0 {
        cps12_pd(casa + 0x12, alloc_empty_list());
    }
    if cps12_d(fora + 0x12) == 0 {
        cps12_pd(fora + 0x12, alloc_empty_list());
    }
    list_append_copy(cps12_d(casa + 0x12), pdata, 8);
    list_append_copy(cps12_d(fora + 0x12), pdata, 8);
    if cps12_d(casa + 0xa) == 0 {
        cps12_pd(casa + 0xa, alloc_empty_list());
    }
    list_append_copy(cps12_d(casa + 0xa), fora, 4);
    if cps12_d(fora + 0xa) == 0 {
        cps12_pd(fora + 0xa, alloc_empty_list());
    }
    list_append_copy(cps12_d(fora + 0xa), casa, 4);
    *((casa + 4) as *mut u8) = (*((casa + 4) as *const u8)).wrapping_add(1);
    *((fora + 4) as *mut u8) = (*((fora + 4) as *const u8)).wrapping_add(1);
    1
}

// ponytail: cria amistosos (aloca), manda os pedidos e libera o plano: não roda duas vezes. Só a original; a cópia leva o
// ramo (pré-temporada da IA, 515 chamadas até 2002-06-01).
unsafe extern "thiscall" fn compare_create_due_planned_friendlies(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// Comparador: aloca e acrescenta à lista (não roda duas vezes): a original faz e a cópia confere o resultado com o
/// estádio de quem joga em casa visto antes (0x5ae870, só lê): ocupado → 0; senão o amistoso devolvido tem a data,
/// +0xc o clube, +8 = 0, +0x18 = 1 e o mandante e o visitante pelo `em_casa`.
unsafe extern "thiscall" fn compare_add_friendly_match(this: usize, clube: usize, data: usize, adversario: usize, em_casa: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32) -> usize>(%%I1%%);
    let ocupada = home_stadium_busy_on(clube, data, adversario, em_casa);
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
    SCOREBOARD.record(igual, || format!("add_friendly_match(clube {:x}, {}/{}, adversario {:x}, em casa {}) ocupada {} original={:x}",
        clube, dia, ano, adversario, em_casa, ocupada, r));
    r
}

// ponytail: sorteia a data e acrescenta às listas: não roda duas vezes. Só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_plan_friendly_on_free_date(this: usize, anfitriao: usize, casa: usize, fora: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32>(%%I2%%)(this, anfitriao, casa, fora)
}
