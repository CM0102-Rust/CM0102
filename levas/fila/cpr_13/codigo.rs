// ---------- %%LEVA%%a leva: o clube livre para o amistoso, o clube na copa e a nação parceira do calendário (friendly.cpp, Cup.cpp, fix_man.cpp) ----------

unsafe fn cpr13_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// { 0x5c6b70 } (stdcall, ret 0xc, friendly.cpp): `friendly_is_club_free_on_date(data, clube, tipo)` (da marcação dos
/// amistosos: excursões, torneios, respostas aos convites e a situação das datas do clube): a janela vai da véspera
/// (0x54c8e0) até `folga` dias depois (0x54c5b0; folga 2 nos tipos 1 e 10, 3 no tipo 2, 1 nos tipos 0, 3, 4 e 11;
/// outro tipo: a caixa, linha 0xf18, e folga 1). Com tipo ≠ 0, clube que não é seleção e nação-chave ativa, a janela
/// que cai na temporada da nação (início em 0xb63d8a, fim em 0xb63d99, registro de 72 bytes da 0x669340) não serve.
/// Senão, livre se a 0x5acbe0 não acha jogo do clube (+0 id) na janela (com uma lista provisória, destruída no fim).
/// Clube ou data nulos: 0.
unsafe extern "stdcall" fn friendly_is_club_free_on_date(data: usize, clube: usize, tipo: u32) -> u32 {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let criar_lista: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir_lista: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let menos_dias: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c8e0usize);
    let mais_dias: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let (mut a, mut b, mut t0, mut t1) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let mut lista = [0u8; 0x10];
    let (pa, pb, p0, p1, pl) = (a.as_mut_ptr() as usize, b.as_mut_ptr() as usize, t0.as_mut_ptr() as usize,
        t1.as_mut_ptr() as usize, lista.as_mut_ptr() as usize);
    data_padrao(pa);
    data_padrao(pb);
    criar_lista(pl);
    let sai = |r: u32| -> u32 {
        destruir_lista(pl);
        r
    };
    if clube == 0 || data == 0 {
        return sai(0);
    }
    let k = tipo as u8 as i8;
    let folga: u8 = match k {
        1 | 10 => 2,
        2 => 3,
        0 | 3 | 4 | 11 => 1,
        _ => {
            show_error_at(0x9cd0a0, 0xf18);
            1
        }
    };
    crate::comparacao::ramo("cpr_13 clube livre");
    let r = menos_dias(data, p0, 1); // a véspera
    copiar(pa, p1, r);
    // ponytail: a original empurra a folga com os 16 bits altos do ponteiro que a 0x417c10 devolveu; a 0x54c5b0 só lê
    // a palavra
    let r = mais_dias(data, p1, folga as u32);
    copiar(pb, p0, r);
    let w = |p: usize, o: usize| core::ptr::read_unaligned((p + o) as *const u16);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let nacao = cpr13_d(clube + 0x53) as usize;
    if k != 0 && selecao(clube) == 0 && nacao != 0 && *((nacao + 0x11c) as *const u8) != 0 {
        let indice: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669340usize);
        let reg = (indice(nacao) as u8 as i8 as isize * 72) as usize;
        let inicio = 0xb63d8ausize.wrapping_add(reg); // a temporada da nação-chave
        let (iano, idia) = (w(inicio, 2), w(inicio, 0) as i16);
        if iano < w(pb, 2) || (iano == w(pb, 2) && idia < w(pb, 0) as i16) {
            let fim = 0xb63d99usize.wrapping_add(reg);
            let (fano, fdia) = (w(fim, 2), w(fim, 0) as i16);
            if fano > w(pa, 2) || (fano == w(pa, 2) && fdia > w(pa, 0) as i16) {
                return sai(0); // a janela cai na temporada
            }
        }
    }
    // os dias e anos vão como os dwords da original (a palavra e os 2 bytes seguintes da data)
    let jogos: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, usize, u32, u32, u32) -> u32 =
        core::mem::transmute(0x5acbe0usize);
    let n = jogos(cpr13_d(0xae2a58) as usize, cpr13_d(pa), cpr13_d(pa + 2), cpr13_d(pb), cpr13_d(pb + 2), cpr13_d(clube),
        pl, u32::MAX, 0xffff_fffe, u32::MAX);
    if n as u16 != 0 {
        crate::comparacao::ramo("cpr_13 ocupado");
        return sai(0); // tem jogo na janela
    }
    sai(1)
}

/// { 0x522910 } (thiscall, ret 4, Cup.cpp; virtual +0x80 das copas, chamada pela 0x4ae300 da projeção da diretoria):
/// `Cup::has_club(clube)`: 1 se o clube está na tabela de times da copa (+0xa7, 6 bytes por time, +0 o clube; conta
/// +0x3e); senão 0.
unsafe extern "thiscall" fn has_club(this: usize, clube: u32) -> u32 {
    let n = *((this + 0x3e) as *const i16);
    let times = cpr13_d(this + 0xa7) as usize;
    for i in 0..n.max(0) as usize {
        if cpr13_d(times + i * 6) == clube {
            crate::comparacao::ramo("cpr_13 copa tem o clube");
            return 1; // está na copa
        }
    }
    0
}

/// { 0x5af5b0 } (cdecl, fix_man.cpp; das buscas de jogos 0x5acbe0, 0x5acee0 e 0x5ad780): `fix_man_get_partner_key_nation_index(clube)`:
/// para clube da Inglaterra ([0x9cf2e4]) ou do País de Gales ([0x9cf510]) (nação +0x53, id +0), o índice da nação-chave
/// ativa da outra (os 35 registros de 72 bytes em 0xb63d60: +0 a nação, ativa em +0x11c, +0x14 o índice); senão 0xff.
unsafe extern "C" fn fix_man_get_partner_key_nation_index(clube: usize) -> u8 {
    let nacao = cpr13_d(clube + 0x53) as usize;
    if nacao == 0 {
        return 0xff;
    }
    let (id, inglaterra, gales) = (cpr13_d(nacao), cpr13_d(0x9cf2e4), cpr13_d(0x9cf510));
    let outra = if id == inglaterra {
        gales
    } else if id == gales {
        inglaterra
    } else {
        return 0xff;
    };
    let alvo = cpr13_d(0xae23a8).wrapping_add(outra.wrapping_mul(0x122)) as usize;
    for i in 0..0x23usize {
        let p = cpr13_d(0xb63d60 + i * 72) as usize;
        if p != 0 && *((p + 0x11c) as *const u8) != 0 && p == alvo {
            return *((0xb63d74 + i * 72) as *const u8);
        }
    }
    0xff
}

/// Comparador: a cópia roda com a lista provisória dela, como a original: pela devolução.
unsafe extern "stdcall" fn comparar_friendly_is_club_free_on_date(data: usize, clube: usize, tipo: u32) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize, usize, u32) -> u32>(%%I0%%)(data, clube, tipo);
    let n = friendly_is_club_free_on_date(data, clube, tipo);
    PLACAR.registrar(r == n, || format!("friendly_is_club_free_on_date(data {:x}, clube {:x}, tipo {}) original={} rust={}", data, clube, tipo as u8 as i8, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "thiscall" fn comparar_has_club(this: usize, clube: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I1%%)(this, clube);
    let n = has_club(this, clube);
    PLACAR.registrar(r == n, || format!("has_club(copa {:x}, clube {:x}) original={} rust={}", this, clube, r, n));
    r
}

/// Comparador: pelo valor (só lê).
unsafe extern "C" fn comparar_fix_man_get_partner_key_nation_index(clube: usize) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u8>(%%I2%%)(clube);
    let n = fix_man_get_partner_key_nation_index(clube);
    PLACAR.registrar(r == n, || format!("fix_man_get_partner_key_nation_index({:x}) original={:x} rust={:x}", clube, r, n));
    r
}
