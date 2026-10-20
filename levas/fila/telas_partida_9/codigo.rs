// ---------- %%LEVA%%a leva: as telas da partida (9): a cópia do bloco do time e da linha do jogador e o placar ao vivo (match_screens.cpp) ----------

/// Copia de `s` para `d` os campos de `tam` bytes (1, 2 ou 4) de `ini` a `fim`, em ordem crescente, como a original.
unsafe fn copiar_campos(d: usize, s: usize, ini: usize, fim: usize, tam: usize) {
    let mut k = ini;
    while k < fim {
        match tam {
            1 => *((d + k) as *mut u8) = *((s + k) as *const u8),
            2 => core::ptr::write_unaligned((d + k) as *mut u16, core::ptr::read_unaligned((s + k) as *const u16)),
            _ => core::ptr::write_unaligned((d + k) as *mut u32, core::ptr::read_unaligned((s + k) as *const u32)),
        }
        k += tam;
    }
}

/// { 0x71dd70 } (thiscall, match_screens.cpp): copia o bloco do time da partida (0x625 bytes) campo a
/// campo: bytes 0..0x11, palavras 0x11..0x1f, os quatro vetores de 5 bytes e os bytes 0x33/0x34, as duas
/// listas de 11 palavras (0x35, 0x4b), as duplas 0x61/0x65, as palavras 0x69..0x71 e as 20 linhas de
/// jogador de 0x49 bytes (0x71df00). Devolve `this`.
unsafe extern "thiscall" fn atribuir_bloco_do_time(this: usize, src: usize) -> usize {
    copiar_campos(this, src, 0, 0x11, 1);
    copiar_campos(this, src, 0x11, 0x1f, 2);
    copiar_campos(this, src, 0x1f, 0x35, 1);
    copiar_campos(this, src, 0x35, 0x61, 2);
    copiar_campos(this, src, 0x61, 0x69, 4);
    copiar_campos(this, src, 0x69, 0x71, 2);
    let linha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x71df00usize);
    for i in 0..20 {
        linha(this + 0x71 + i * 0x49, src + 0x71 + i * 0x49);
    }
    this
}

unsafe extern "thiscall" fn comparar_atribuir_bloco_do_time(this: usize, src: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I0%%);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(this, 0x625)], || o(this, src), || atribuir_bloco_do_time(this, src));
    PLACAR.registrar(igual && r == n, || format!("atribuir_bloco_do_time({:#x} <- {:#x}): igual {} devolve {:#x}/{:#x}", this, src, igual, r, n));
    r
}

/// { 0x71df00 } (thiscall, match_screens.cpp): copia a linha de jogador do bloco do time (0x49 bytes):
/// duplas 0..0x14, a data em +0x14 pela Date::operator= (0x417c10, com o retorno numa local), bytes
/// 0x1c..0x36, palavras 0x36..0x3e e bytes 0x3e..0x49. Devolve `this`.
unsafe extern "thiscall" fn atribuir_linha_do_jogador(this: usize, src: usize) -> usize {
    copiar_campos(this, src, 0, 0x14, 4);
    let atribuir_data: unsafe extern "thiscall" fn(usize, *mut [u8; 8], usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut volta = [0u8; 8];
    atribuir_data(this + 0x14, &mut volta, src + 0x14);
    copiar_campos(this, src, 0x1c, 0x36, 1);
    copiar_campos(this, src, 0x36, 0x3e, 2);
    copiar_campos(this, src, 0x3e, 0x49, 1);
    this
}

unsafe extern "thiscall" fn comparar_atribuir_linha_do_jogador(this: usize, src: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I1%%);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado(&[(this, 0x49)], || o(this, src), || atribuir_linha_do_jogador(this, src));
    PLACAR.registrar(igual && r == n, || format!("atribuir_linha_do_jogador({:#x} <- {:#x}): igual {} devolve {:#x}/{:#x}", this, src, igual, r, n));
    r
}

/// { 0x728e90 } (cdecl, match_screens.cpp): conta no resultado `res` os gols da partida `indice` do dia
/// ([[0xb67a50]+0x19] + 0x69 x indice + 0x5f; lances de 4 bytes a partir de +0x57, até 50 ou lado 0xff)
/// até o minuto atual da partida `partida` ([[partida+0x5f]+0x4d], 16 bits com sinal; sem partida ao vivo,
/// [partida+0x67] = 0xff ou [partida+0x5f] nulo, não conta nada). Tipo 1/3: gol do lado (0 vai a +0x43,
/// senão +0x44); 2: gol contra (invertido); 7: gol nos pênaltis (+0x49/+0x4a, que começam em 0xff e vão
/// a 1 no primeiro); 8 e os outros: nada. Zera antes +0x43..+0x46 e +0x4b/+0x4c. Resultado ou partida
/// nulos: nada.
unsafe extern "C" fn contar_placar_ao_vivo(partida: usize, indice: u32, res: usize) {
    if res == 0 || partida == 0 {
        return;
    }
    let b = |k: usize| (res + k) as *mut u8;
    for k in [0x43, 0x44, 0x4b, 0x4c] {
        *b(k) = 0;
    }
    *b(0x49) = 0xff;
    *b(0x4a) = 0xff;
    *b(0x46) = 0;
    *b(0x45) = 0;
    let dia = core::ptr::read_unaligned((*(0xb67a50 as *const usize) + 0x19) as *const u32);
    let jogo = core::ptr::read_unaligned(dia.wrapping_add(indice.wrapping_mul(0x69)).wrapping_add(0x5f) as usize as *const usize);
    if jogo == 0 {
        return;
    }
    let mut lance = jogo + 0x59;
    for _ in 0..0x32 {
        let lado = *((lance - 2) as *const u8);
        if lado == 0xff {
            break;
        }
        let ao_vivo = core::ptr::read_unaligned((partida + 0x5f) as *const usize);
        if *((partida + 0x67) as *const u8) != 0xff
            && ao_vivo != 0
            && *((lance + 1) as *const u8) as i16 <= core::ptr::read_unaligned((ao_vivo + 0x4d) as *const i16)
        {
            let tipo = *(lance as *const u8);
            let campo = match tipo {
                1 | 3 => Some(if lado == 0 { 0x43 } else { 0x44 }),
                2 => Some(if lado == 0 { 0x44 } else { 0x43 }),
                _ => None,
            };
            if let Some(k) = campo {
                *b(k) = (*b(k)).wrapping_add(1);
            } else if tipo == 7 {
                let k = if lado == 0 { 0x49 } else { 0x4a };
                *b(k) = if *b(k) == 0xff { 1 } else { *b(k) + 1 };
            }
        }
        lance += 4;
    }
}

unsafe extern "C" fn comparar_contar_placar_ao_vivo(partida: usize, indice: u32, res: usize) {
    let o = orig::<unsafe extern "C" fn(usize, u32, usize)>(%%I2%%);
    if res == 0 {
        return o(partida, indice, res);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(res + 0x43, 0xa)], || o(partida, indice, res), || contar_placar_ao_vivo(partida, indice, res));
    PLACAR.registrar(igual, || format!("contar_placar_ao_vivo(partida {:#x}, indice {}): {:02x?} / {:02x?}", partida, indice, mo.0, mr.0));
}
