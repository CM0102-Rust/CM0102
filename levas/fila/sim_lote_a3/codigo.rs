// ---------- %%LEVA%%a leva: as datas do sorteio e do jogo da rodada da copa e o clube que sai dos amistosos planejados (Cup.cpp, friendly.cpp) ----------

/// A data (Date::Date, 0x549f70, num Date local de 8 bytes) de `dia`/`mes`/`ano`, levada ao dia da
/// semana `semana` (−1 = qualquer): (palavra +0 = dia do ano, byte baixo do ano +2).
unsafe fn data_da_rodada(dia: u32, mes: u32, ano: u32, semana: u32) -> (u16, u8) {
    let construir: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u32; 2];
    construir(&mut d, dia, mes, ano, semana);
    (d[0] as u16, (d[0] >> 16) as u8)
}

/// O registro de 104 bytes da rodada `rodada` (palavra com sinal) a partir de `rodadas`.
fn registro_da_rodada(rodadas: usize, rodada: u32) -> usize {
    rodadas.wrapping_add((rodada as i16 as isize * 104) as usize)
}

/// { 0x521e60 } (cdecl, Cup.cpp): a data do sorteio da rodada: dia do ano na palavra +0 e ano − ano-base
/// (byte) em +2, da data de `dia`/`mes`/ano-base + `anos` levada ao dia da semana. Devolve o registro.
unsafe extern "C" fn data_do_sorteio_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, base: u32) -> usize {
    let (d, a) = data_da_rodada(dia, mes, base.wrapping_add(anos), semana);
    let r = registro_da_rodada(rodadas, rodada);
    core::ptr::write_unaligned(r as *mut u16, d);
    *((r + 2) as *mut u8) = a.wrapping_sub(base as u8); // ano do sorteio
    r
}

/// { 0x521eb0 } (cdecl, Cup.cpp): a data do jogo da rodada, como a do sorteio, em +3 (dia) e +5 (ano −
/// ano-base); o horário (byte) em +6 e as marcas do mando em +0x13. Devolve o registro.
unsafe extern "C" fn data_do_jogo_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, hora: u32, base: u32, mando: u32) -> usize {
    let (d, a) = data_da_rodada(dia, mes, base.wrapping_add(anos), semana);
    let r = registro_da_rodada(rodadas, rodada);
    core::ptr::write_unaligned((r + 3) as *mut u16, d); // dia do jogo
    *((r + 5) as *mut u8) = a.wrapping_sub(base as u8);
    *((r + 6) as *mut u8) = hora as u8;
    core::ptr::write_unaligned((r + 0x13) as *mut u32, mando);
    r
}

/// { 0x5c64e0 } (thiscall, ret 4, friendly.cpp): tira o `clube` dos amistosos planejados: no vetor
/// [+0x34d8] ([+0x34e0] registros de 0x14 bytes) o id do clube em +8 vira −1; na lista [+0x34c8]
/// (0x68d1b0), o ponteiro do clube em +0xc vira 0.
unsafe extern "thiscall" fn tirar_clube_dos_amistosos(amistosos: usize, clube: usize) {
    let vetor = (amistosos + 0x34d8) as *const usize;
    let n = (amistosos + 0x34e0) as *const i32;
    if *vetor != 0 {
        let mut i = 0;
        while i < *n {
            let id = (*vetor + 8 + 0x14 * i as usize) as *mut u32;
            if *id == *(clube as *const u32) {
                *id = 0xffff_ffff;
            }
            i += 1;
        }
    }
    let lista = (amistosos + 0x34c8) as *const usize;
    if *lista == 0 {
        return;
    }
    let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    core::ptr::write_unaligned((*lista + 0xa) as *mut u32, core::ptr::read_unaligned((*lista + 2) as *const u32));
    loop {
        let e = prox(*lista);
        if e == 0 {
            return;
        }
        if *((e + 0xc) as *const usize) == clube {
            *((e + 0xc) as *mut usize) = 0; // sai da lista
        }
    }
}

unsafe extern "C" fn comparar_data_do_sorteio_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, base: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize>(%%I0%%);
    let r = registro_da_rodada(rodadas, rodada);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r, 3)], || o(rodadas, rodada, dia, mes, anos, semana, base),
        || data_do_sorteio_da_rodada(rodadas, rodada, dia, mes, anos, semana, base));
    PLACAR.registrar(igual && a == b, || format!("data_do_sorteio_da_rodada({}, {}/{} +{}) {:x?} x {:x?}", rodada as i16, dia, mes, anos, mo, mr));
    a
}

unsafe extern "C" fn comparar_data_do_jogo_da_rodada(rodadas: usize, rodada: u32, dia: u32, mes: u32, anos: u32, semana: u32, hora: u32, base: u32, mando: u32) -> usize {
    let o = orig::<unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize>(%%I1%%);
    let r = registro_da_rodada(rodadas, rodada);
    let (a, b, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(r + 3, 0x14)], || o(rodadas, rodada, dia, mes, anos, semana, hora, base, mando),
        || data_do_jogo_da_rodada(rodadas, rodada, dia, mes, anos, semana, hora, base, mando));
    PLACAR.registrar(igual && a == b, || format!("data_do_jogo_da_rodada({}, {}/{} +{}) {:x?} x {:x?}", rodada as i16, dia, mes, anos, mo, mr));
    a
}

unsafe extern "thiscall" fn comparar_tirar_clube_dos_amistosos(amistosos: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let mut regioes = vec![];
    let vetor = *((amistosos + 0x34d8) as *const usize);
    let n = *((amistosos + 0x34e0) as *const i32);
    if vetor != 0 && n > 0 {
        regioes.push((vetor, n as usize * 0x14));
    }
    let lista = *((amistosos + 0x34c8) as *const usize);
    if lista != 0 {
        // o cursor e o ponteiro +0xc de cada amistoso da lista (percorrida aqui antes; as duas
        // rotinas reiniciam o cursor)
        regioes.push((lista + 0xa, 4));
        let prox: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
        core::ptr::write_unaligned((lista + 0xa) as *mut u32, core::ptr::read_unaligned((lista + 2) as *const u32));
        loop {
            let e = prox(lista);
            if e == 0 {
                break;
            }
            regioes.push((e + 0xc, 4));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(amistosos, clube),
        || tirar_clube_dos_amistosos(amistosos, clube));
    PLACAR.registrar(igual, || format!("tirar_clube_dos_amistosos({:#x}) {:x?} x {:x?}", clube, mo, mr));
}
