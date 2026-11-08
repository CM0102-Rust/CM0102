// ---------- %%LEVA%%a leva: tirar de vetores ordenados: o interesse de transferência da mídia, o pedido de observação e o apelido de todos os técnicos humanos ----------

unsafe fn cph4_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

unsafe fn media_crt_qsort(base: usize, n: i32, tamanho: u32, comparar: usize) {
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(base, n, tamanho, comparar);
}

unsafe fn crt_bsearch(chave: usize, base: usize, n: u32, tamanho: u32, comparar: usize) -> usize {
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    bsearch(chave, base, n, tamanho, comparar)
}

/// { 0x7549b0 } (thiscall, ret 4, media.cpp): `Media::remove_transfer_interest(pessoa)`: no vetor de interesses
/// ([this+0xc], [this+0x14] registros de 25 bytes), cada um da pessoa (+0 = id) recebe por cima o último (6 duplas e um
/// byte), a contagem desce e o mesmo lugar é visto de novo. Sobrando algum, reordena (qsort 0x945333 com 0x757ea0).
unsafe extern "thiscall" fn remove_transfer_interest(this: usize, pessoa: usize) {
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
        media_crt_qsort(cph4_d(this + 0xc) as usize, *n, 0x19, 0x757ea0);
    }
}

/// { 0x7f8ee0 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::remove_scout_request(pessoa, clube)`: o clube
/// nulo é o do técnico humano atual ([0xb74c26 + 0x300 × [0xb77c7a]], +0x39; sem ele, nada). Com a pessoa, busca
/// (bsearch 0x946464 com 0x7fc270, que lê só +0 clube e +4 pessoa; a data padrão da chave em +8, o byte +0x10 sem valor
/// e a cópia usa zero) o pedido no vetor ([this+0x5c], [this+0x64] registros de 17 bytes); achou: puxa os seguintes um
/// lugar (+0, +4, a data +8 pela 0x417c10 e o byte +0x10), a contagem desce e, sobrando algum, reordena (qsort).
unsafe extern "thiscall" fn remove_scout_request(this: usize, pessoa: usize, clube: usize) {
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
    let s = crt_bsearch(chave.as_ptr() as usize, base, cph4_d(this + 0x64), 0x11, 0x7fc270);
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
        media_crt_qsort(cph4_d(this + 0x5c) as usize, n as i32, 0x11, 0x7fc270);
    }
}

/// { 0x5fff30 } (thiscall, ret 4, human_manager.cpp): `HUMAN_MANAGER::remove_nickname_from_all(id)`: em cada técnico
/// humano ([this], [this+4] registros de 0xa6ef bytes; contador em byte com sinal) não nulo, sem vetor de apelidos
/// (+0xa6e3) sai de vez; senão busca (bsearch com 0x601a10, que lê só o id em +0; a chave tem o byte +4 zerado e o resto
/// sem valor, a cópia usa zero) o apelido da pessoa `*id` entre os [+0xa6eb] de 30 bytes; achou: puxa os seguintes um
/// lugar, o último fica com id −1 e +4 = 0 e a contagem desce. No fim reordena (qsort) só o vetor do último técnico
/// visto (sem técnicos, o `id` faz as vezes do técnico, como no original).
unsafe extern "thiscall" fn remove_nickname_from_all(this: usize, id: usize) {
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
                let s = crt_bsearch(chave.as_ptr() as usize, vetor, cph4_d(g + 0xa6eb), 0x1e, 0x601a10);
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
    media_crt_qsort(cph4_d(ultimo + 0xa6e3) as usize, cph4_d(ultimo + 0xa6eb) as i32, 0x1e, 0x601a10);
}

unsafe extern "thiscall" fn comparar_remove_transfer_interest(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I0%%);
    let n = (*((this + 0x14) as *const i32)).max(0) as usize;
    let reg = [(this + 0xc, 0xc), (cph4_d(this + 0xc) as usize, n * 25)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa), || remove_transfer_interest(this, pessoa));
    PLACAR.registrar(igual, || format!("remove_transfer_interest({:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

unsafe extern "thiscall" fn comparar_remove_scout_request(this: usize, pessoa: usize, clube: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    let n = (*((this + 0x64) as *const i32)).max(0) as usize;
    let reg = [(this + 0x5c, 0xc), (cph4_d(this + 0x5c) as usize, n * 17)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, pessoa, clube), || remove_scout_request(this, pessoa, clube));
    PLACAR.registrar(igual, || format!("remove_scout_request({:x}, {:x}) {:x?} x {:x?}", pessoa, clube, mo, mr));
}

/// Comparador: lado a lado pelos vetores de apelidos e contagens de todos os técnicos humanos; sem técnicos (o qsort
/// leria pelo `id`), só a original.
unsafe extern "thiscall" fn comparar_remove_nickname_from_all(this: usize, id: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
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
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, id), || remove_nickname_from_all(this, id));
    PLACAR.registrar(igual, || format!("remove_nickname_from_all({:x}) {:x?} x {:x?}", id, mo, mr));
}
