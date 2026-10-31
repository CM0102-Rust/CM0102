// ---------- %%LEVA%%a leva: os estádios: o dia dos estádios (o estádio nacional fechado e as reservas vencidas), reservar um estádio numa data e se ele está livre (stadium.cpp) ----------

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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
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
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
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
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%)(this, estadio, data);
    let n = estadio_livre_na_data(this, estadio, data);
    PLACAR.registrar(r == n, || format!("estadio_livre_na_data(estadio {:x}) original={} rust={}", estadio, r, n));
    r
}
