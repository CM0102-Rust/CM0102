// ---------- %%LEVA%%a leva: a formação lida do arquivo .pct (as táticas da IA e as do humano) e a troca da ordem dos bytes de palavras e dwords (formation.cpp, cash.cpp) ----------

unsafe fn cqg8_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn cqg8_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

/// fread (0x945bfe, cdecl): `n` registros de `tam` bytes; devolve quantos leu.
unsafe fn cqg8_ler(buf: usize, tam: u32, f: usize) -> u32 {
    let fread: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    fread(buf, tam, 1, f)
}

/// fclose (0x945a09, cdecl).
unsafe fn cqg8_fechar(f: usize) {
    let fclose: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    fclose(f);
}

/// A caixa de erro de formation.cpp (0x9cc3dc) na linha `linha`. A original também formata a mensagem num texto local
/// (sprintf 0x944c9f) que não sai dali.
unsafe fn cqg8_erro(linha: u32) {
    aviso_em(0x9cc3dc, linha);
}

/// A troca dos bytes de um dword (a original faz em linha).
fn cqg8_inverter(v: u32) -> u32 {
    v.swap_bytes()
}

/// A leitura que falha e encerra: sem `quieto`, a caixa; o arquivo fecha e devolve 0.
unsafe fn cqg8_falhou(quieto: u32, linha: u32, f: usize) -> u32 {
    if quieto == 0 {
        cqg8_erro(linha);
    }
    cqg8_fechar(f);
    0
}

/// { 0x5b5d10 } (thiscall, ret 0xc, formation.cpp): `Formation::load_from_disk(caminho, quieto, ofuscado)`, pela
/// criação das táticas (0x893dc0: as formações da IA e as pré-definidas, 45 na carreira) e pelas telas de tática.
/// Caminho nulo (linha 0x8d8) ou o arquivo que não abre (0x8e0): 0 (sem `quieto`, a caixa). Lê o identificador (4 bytes;
/// invertido se o marcador do arquivo 0x90c140 não é o nativo; menos 123456789 se ofuscado): fora de 0x98ec54..0x98ec5e
/// "não é formação" (0x8f7), abaixo de 0x98ec59 "versão antiga" (0x900). Depois, na ordem: a senha (+0x33, 50 bytes,
/// cada byte invertido, 0xff fica), o criador (+0x65, 200), a máscara (+0x187b, a partir de 0x98ec5a; antes 0), as 11
/// posições gerais (+0x12d, palavras) e a máscara refeita (0x5b59a0), a ordem de exibição (+0x143, 11 bytes, a partir de
/// 0x98ec5c; antes zeros), as 11 posições extras (+0x14e), as casas (+0x164, 0x420 bytes = 0x210 palavras), nas versões
/// 0x98ec59/0x98ec5a 0x108 bytes descartados, as instruções do time (+0x584, dword), as instruções dos jogadores (abaixo
/// de 0x98ec5e 11 dwords, cada um com 10 ao lado, em +0x188b de 8 em 8; depois 0x58 bytes em +0x188b) e os tipos de
/// marcação (+0x5b4, 11 bytes); as três últimas, se falham, só mostram a caixa. As instruções do time ganham 0x2000
/// (sem 0x4000) e, abaixo de 0x98ec5d, 0x20000. As 11 vagas refazem as posições pela casa guardada (0x5b3c70 e
/// 0x5b3560 com a fase 0 e a faixa 0); o nome (+0) é o do arquivo (_splitpath 0x944cff); o mapa das casas (0x5b7160);
/// +0x187f = 1, +0x1883 = senha não vazia, +0x1887 = 1. Fecha e devolve 1. Os arquivos não nativos trocam a ordem dos
/// bytes (0x43f2d0 palavras, 0x43f300 dwords, 0x43f350 qwords).
unsafe extern "thiscall" fn ler_formacao_do_disco(this: usize, caminho: usize, quieto: u32, ofuscado: u32) -> u32 {
    if caminho == 0 {
        if quieto == 0 {
            cqg8_erro(0x8d8);
        }
        return 0;
    }
    let fopen: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let f = fopen(caminho, 0x98f928);
    if f == 0 {
        if quieto == 0 {
            cqg8_erro(0x8e0);
        }
        return 0;
    }
    let marcador: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x90c140usize);
    let nativo = marcador(f) != 0xf1ff_fff1;
    let mut id_ = [0u8; 4];
    if cqg8_ler(id_.as_mut_ptr() as usize, 4, f) != 1 {
        return cqg8_falhou(quieto, 0x8ea, f);
    }
    let mut id = u32::from_le_bytes(id_);
    if !nativo {
        id = cqg8_inverter(id);
    }
    if ofuscado != 0 {
        id = id.wrapping_sub(0x75bcd15);
    }
    let id = id as i32;
    if !(0x98ec54..=0x98ec5e).contains(&id) {
        return cqg8_falhou(quieto, 0x8f7, f);
    }
    if id < 0x98ec59 {
        return cqg8_falhou(quieto, 0x900, f);
    }
    let senha = this + 0x33;
    if cqg8_ler(senha, 0x32, f) != 1 {
        return cqg8_falhou(quieto, 0x909, f);
    }
    let strlen = |a: usize| (0..).take_while(|&k| *((a + k) as *const u8) != 0).count();
    if strlen(senha) != 0 {
        let mut k = 0usize;
        loop {
            let b = *((senha + k) as *const u8);
            *((senha + k) as *mut u8) = if b == 0xff { 0xff } else { !b }; // cqg8: a senha invertida
            k += 1;
            if k >= strlen(senha) {
                break;
            }
        }
    }
    if cqg8_ler(this + 0x65, 200, f) != 1 {
        return cqg8_falhou(quieto, 0x915, f);
    }
    let troca16: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x43f2d0usize);
    if id >= 0x98ec5a {
        if cqg8_ler(this + 0x187b, 4, f) != 1 {
            return cqg8_falhou(quieto, 0x920, f);
        }
        if !nativo {
            cqg8_pd(this + 0x187b, cqg8_inverter(cqg8_d(this + 0x187b)));
        }
    } else {
        cqg8_pd(this + 0x187b, 0);
    }
    if cqg8_ler(this + 0x12d, 0x16, f) != 1 {
        return cqg8_falhou(quieto, 0x930, f);
    }
    if !nativo {
        troca16(this + 0x12d, 0xb);
    }
    let mascara: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5b59a0usize);
    mascara(this);
    if id >= 0x98ec5c {
        if cqg8_ler(this + 0x143, 0xb, f) != 1 {
            return cqg8_falhou(quieto, 0x940, f);
        }
    } else {
        core::ptr::write_bytes((this + 0x143) as *mut u8, 0, 0xb);
    }
    if cqg8_ler(this + 0x14e, 0x16, f) != 1 {
        return cqg8_falhou(quieto, 0x94f, f);
    }
    if !nativo {
        troca16(this + 0x14e, 0xb);
    }
    if cqg8_ler(this + 0x164, 0x420, f) != 1 {
        return cqg8_falhou(quieto, 0x959, f);
    }
    if !nativo {
        troca16(this + 0x164, 0x210);
    }
    if id == 0x98ec59 || id == 0x98ec5a {
        let mut descarte = [0u8; 0x108];
        if cqg8_ler(descarte.as_mut_ptr() as usize, 0x108, f) != 1 {
            return cqg8_falhou(quieto, 0x968, f);
        }
        if !nativo {
            troca16(descarte.as_mut_ptr() as usize, 0x84);
        }
    }
    let time = this + 0x584;
    if cqg8_ler(time, 4, f) == 1 {
        if !nativo {
            cqg8_pd(time, cqg8_inverter(cqg8_d(time)));
        }
    } else if quieto == 0 {
        cqg8_erro(0x971);
    }
    if id < 0x98ec5e {
        // ponytail: a original copia o vetor local mesmo quando a leitura falha (sobra de pilha num arquivo truncado);
        // a cópia começa com zeros
        let mut v = [0u32; 11];
        if cqg8_ler(v.as_mut_ptr() as usize, 0x2c, f) == 1 {
            if !nativo {
                let troca32: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x43f300usize);
                troca32(v.as_mut_ptr() as usize, 0xb);
            }
        } else if quieto == 0 {
            cqg8_erro(0x97b);
        }
        for (k, x) in v.iter().enumerate() {
            cqg8_pd(this + 0x188b + 8 * k, *x);
            cqg8_pd(this + 0x188b + 8 * k + 4, 0xa); // cqg8: a instrução padrão das versões antigas
        }
        crate::comparacao::ramo("cqg_8 instrucoes antigas");
    } else if cqg8_ler(this + 0x188b, 0x58, f) == 1 {
        if !nativo {
            let troca64: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x43f350usize);
            troca64(this + 0x188b, 0xb);
        }
    } else if quieto == 0 {
        cqg8_erro(0x988);
    }
    if cqg8_ler(this + 0x5b4, 0xb, f) != 1 && quieto == 0 {
        cqg8_erro(0x98f);
    }
    let mut v = cqg8_d(time);
    if v & 0x4000 == 0 {
        v |= 0x2000; // cqg8: a instrução 0x2000 sem a 0x4000
        cqg8_pd(time, v);
        crate::comparacao::ramo("cqg_8 instrucao 0x2000");
    }
    if id < 0x98ec5d {
        cqg8_pd(time, cqg8_d(time) | 0x20000);
    }
    let guardada: unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32 = core::mem::transmute(0x5b3c70usize);
    let refazer: unsafe extern "thiscall" fn(usize, i32, i32, u32, u32) -> u32 = core::mem::transmute(0x5b3560usize);
    let mut c = 0u32;
    for vaga in 0..0xbi32 {
        guardada(this, vaga as i8, 0, 0, &mut c); // cqg8: a casa guardada da fase 0
        crate::comparacao::ramo("cqg_8 posicoes refeitas");
        refazer(this, vaga, 0, 0, c); // cqg8: as posições refeitas pela casa
    }
    let separar: unsafe extern "C" fn(usize, usize, usize, usize, usize) = core::mem::transmute(0x944cffusize);
    separar(caminho, 0, 0, this, 0);
    let mapa: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5b7160usize);
    mapa(this);
    cqg8_pd(this + 0x187f, 1);
    cqg8_pd(this + 0x1883, (*((this + 0x33) as *const u8) != 0) as u32);
    cqg8_pd(this + 0x1887, 1);
    cqg8_fechar(f);
    1
}

/// Comparador: lado a lado na formação (0x18e3 bytes: até as instruções dos jogadores em +0x188b + 0x58) e na
/// devolução; o arquivo é aberto e lido pelas duas (só leitura).
unsafe extern "thiscall" fn comparar_ler_formacao_do_disco(this: usize, caminho: usize, quieto: u32, ofuscado: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u32>(%%I0%%);
    if this == 0 {
        return o(this, caminho, quieto, ofuscado);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(this, 0x18e3)], || o(this, caminho, quieto, ofuscado),
        || ler_formacao_do_disco(this, caminho, quieto, ofuscado));
    PLACAR.registrar(igual && r == n, || {
        let (a, b) = (&mo.0[0].1, &mr.0[0].1);
        let dif: Vec<usize> = (0..a.len()).filter(|&k| a[k] != b[k]).take(8).collect();
        format!("ler_formacao_do_disco(this {:x}) original={} rust={}, bytes diferentes {:x?}", this, r, n, dif)
    });
    r
}

/// { 0x43f2d0 } (cdecl, cash.cpp): `swap_bytes_16_array(palavras, conta)`: troca os dois bytes de cada palavra.
unsafe extern "C" fn trocar_bytes_das_palavras(p: usize, conta: u32) {
    for k in 0..conta as usize {
        let a = (p + 2 * k) as *mut u16;
        core::ptr::write_unaligned(a, core::ptr::read_unaligned(a).swap_bytes());
    }
}

/// Comparador: lado a lado no vetor (até 0x420 palavras conferidas).
unsafe extern "C" fn comparar_trocar_bytes_das_palavras(p: usize, conta: u32) {
    let o = orig::<unsafe extern "C" fn(usize, u32)>(%%I1%%);
    if p == 0 || conta == 0 || conta > 0x1000 {
        return o(p, conta);
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(p, 2 * conta as usize)], || o(p, conta), || trocar_bytes_das_palavras(p, conta));
    PLACAR.registrar(igual, || format!("trocar_bytes_das_palavras({:x}, {})", p, conta));
}

/// { 0x43f300 } (cdecl, cash.cpp): `swap_bytes_32_array(dwords, conta)`: inverte os quatro bytes de cada dword.
unsafe extern "C" fn trocar_bytes_dos_dwords(p: usize, conta: u32) {
    for k in 0..conta as usize {
        cqg8_pd(p + 4 * k, cqg8_inverter(cqg8_d(p + 4 * k)));
    }
}

/// Comparador: lado a lado no vetor.
unsafe extern "C" fn comparar_trocar_bytes_dos_dwords(p: usize, conta: u32) {
    let o = orig::<unsafe extern "C" fn(usize, u32)>(%%I2%%);
    if p == 0 || conta == 0 || conta > 0x1000 {
        return o(p, conta);
    }
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&[(p, 4 * conta as usize)], || o(p, conta), || trocar_bytes_dos_dwords(p, conta));
    PLACAR.registrar(igual, || format!("trocar_bytes_dos_dwords({:x}, {})", p, conta));
}
