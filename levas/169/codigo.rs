// ---------- 169a leva: o fechamento do motor (3): a fila da narracao (match_events.cpp) ----------

/// { 0x6d5d00 } (thiscall, ret 0x1c, match_events.cpp): põe um evento na frente da fila da narração:
/// a posição de escrita (+8, anel de 12) recua uma casa (de 0 vai a 11); se o último da fila (+6
/// itens, palavra +0x712 de 14 bytes) é o mesmo evento da nova posição (+0x30), a fila perde um;
/// depois enfileira pela 0x6d6f70 com os mesmos 7 argumentos.
unsafe extern "thiscall" fn evento_na_frente(t: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    let w = |o: usize| core::ptr::read_unaligned((t + o) as *const i16);
    let pos = w(8);
    let pos = if pos == 0 { 0xb } else { pos.wrapping_sub(1) };
    core::ptr::write_unaligned((t + 8) as *mut i16, pos);
    let n = w(6);
    if n > 0 && w(0x712 + n as usize * 14) == w(0x30 + pos as usize * 14) {
        core::ptr::write_unaligned((t + 6) as *mut i16, n - 1);
    }
    let enfileirar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x6d6f70usize);
    enfileirar(t, a, b, c, d, e, f, g)
}

/// { 0x6d5bf0 } (thiscall, ret 4, match_events.cpp): o construtor da fila da narração: zera a fila e
/// os campos (+0x2db0 = 125, +1 e +0xa = −1, +0x2d68 = −1), os globais de evento (0xb68670..), lê os
/// eventos (0x6d9e50), marca as 16 vagas (+0x10, +0x20) com −1 e zera os 16 tempos (+0x2d6c); com
/// `torcida`, toca o loop da torcida do lado da vez ([0xb77c7a]) pela 0x8025e0. Devolve `this`.
unsafe extern "thiscall" fn construir_fila_da_narracao(t: usize, torcida: u8) -> usize {
    let b = |o: usize, v: u8| *((t + o) as *mut u8) = v;
    let w = |o: usize, v: u16| core::ptr::write_unaligned((t + o) as *mut u16, v);
    w(0xa, 0xffff);
    core::ptr::write_unaligned((t + 0x2d68) as *mut u32, u32::MAX);
    for o in [6, 8, 0xc, 0xe] {
        w(o, 0);
    }
    b(5, 0);
    b(2, 0);
    b(0, 1);
    b(0x2db0, 0x7d);
    b(1, 0xff);
    b(3, 0);
    core::ptr::write_unaligned((t + 0x2dac) as *mut u32, 0);
    b(4, 0);
    *(0xb68670 as *mut u16) = 1;
    *(0xb68672 as *mut u16) = 0x2bc;
    *(0xb68674 as *mut u8) = 0;
    *(0xb68778 as *mut u32) = 0xad9c64;
    *(0xb6877c as *mut u32) = 0xad9c64;
    let ler_eventos_cfg: unsafe extern "C" fn() = core::mem::transmute(0x6d9e50usize);
    ler_eventos_cfg();
    for k in 0..16 {
        b(0x10 + k, 0xff);
        b(0x20 + k, 0xff);
        core::ptr::write_unaligned((t + 0x2d6c + 4 * k) as *mut u32, 0);
    }
    if torcida != 0 {
        let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
        let tocar: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32) = core::mem::transmute(0x8025e0usize);
        let lado = *(0xb77c7a as *const i16) as isize;
        let nome = caminho(0x9c382c, 0xa159a0);
        tocar(0xb74450, (t as isize + 0x20 + lado) as usize, 2, nome, 1);
    }
    t
}

/// { 0x6d5b50 } (cdecl, match_events.cpp): libera as tabelas de eventos carregadas: em cada uma das
/// entradas de 0x12 bytes de 0xb68790 a 0xb6b72a com tabela própria (nem nula, nem a padrão
/// 0xb68670), os dois textos (+0x108, +0x10c) de cada registro de 0x110 bytes e a tabela; depois
/// zera 0xb6b723 e põe −1 em 0xa1599c.
unsafe extern "C" fn liberar_tabelas_de_eventos() {
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut e = 0xb68790usize;
    while e < 0xb6b72a {
        let tabela = core::ptr::read_unaligned(e as *const usize);
        if tabela != 0xb68670 && tabela != 0 {
            let mut k = 0i32;
            while k < core::ptr::read_unaligned((e + 4) as *const i16) as i32 {
                let r = tabela + k as usize * 0x110;
                for o in [0x108, 0x10c] {
                    let p = core::ptr::read_unaligned((r + o) as *const usize);
                    if p != 0 {
                        liberar(p);
                    }
                }
                k += 1;
            }
            liberar(tabela);
            core::ptr::write_unaligned(e as *mut usize, 0);
        }
        e += 0x12;
    }
    *(0xb6b723 as *mut u8) = 0;
    *(0xa1599c as *mut u32) = u32::MAX;
}

// ponytail: as tres enfileiram narracao, tocam som ou liberam memoria (nao rodam duas vezes): sem
// comparacao propria, provadas pela carreira (fila de 19414 partidas, 285 mil eventos na frente).
unsafe extern "thiscall" fn comparar_evento_na_frente(t: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) -> u32>(379)(t, a, b, c, d, e, f, g)
}

unsafe extern "thiscall" fn comparar_construir_fila_da_narracao(t: usize, torcida: u8) -> usize {
    orig::<unsafe extern "thiscall" fn(usize, u8) -> usize>(380)(t, torcida)
}

unsafe extern "C" fn comparar_liberar_tabelas_de_eventos() {
    orig::<unsafe extern "C" fn()>(381)()
}
