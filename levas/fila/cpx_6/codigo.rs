// ---------- %%LEVA%%a leva: os temporários do save: as sementes da UEFA e da CONMEBOL e os países anfitriões (uefa_seeding.cpp, conmebol_seeding.cpp, host_country.cpp) ----------

unsafe fn cpx6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A contagem de um byte com sinal (a original usa `movsx`).
unsafe fn signed_byte_count(a: usize) -> u32 {
    *(a as *const i8) as i32 as u32
}

/// Abre `CM3_TEMP\<nome>` para gravar ("wb"): o caminho pela 0x90bde0 e o fopen do jogo (0x945b00).
unsafe fn open_temp_file_for_write(nome: usize) -> usize {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    abrir(caminho(0x987da0, nome), 0x9a7f28)
}

unsafe fn crt_fwrite(dados: usize, tamanho: u32, quantos: u32, f: usize) -> u32 {
    let escrever: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    escrever(dados, tamanho, quantos, f)
}

unsafe fn uefa_seeding_crt_fclose(f: usize) {
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    fechar(f);
}

/// { 0x905510 } (thiscall, ret 0, uefa_seeding.cpp): `UefaSeeding::save_to_temp_file`: grava `CM3_TEMP\uefa_seeding.tmp`:
/// a quantidade de países (+0x438, i8), os registros de 0x12 bytes de +0, +0x439 (u16), a quantidade +0x43f (i8) e, se
/// não for zero, essa quantidade de u32 de [+0x43b], e os 50 registros fixos de 0xc bytes de +0x440. Erro ao abrir ou
/// gravar: a caixa "Error" (uefa_seeding.cpp 0x64f, 0x657, 0x660, 0x668, 0x66f, 0x679, 0x682), sem fechar o arquivo
/// (como a original).
unsafe extern "thiscall" fn uefa_seeding_save_to_temp_file(this: usize) {
    const FONTE: usize = 0xaa1a1c;
    crate::comparison::branch("cpx_6 gravar sementes da uefa");
    let f = open_temp_file_for_write(0xaa1a50);
    if f == 0 {
        show_error_at(FONTE, 0x64f);
        return;
    }
    if crt_fwrite(this + 0x438, 1, 1, f) != 1 {
        show_error_at(FONTE, 0x657);
        return;
    }
    if crt_fwrite(this, 0x12, signed_byte_count(this + 0x438), f) != signed_byte_count(this + 0x438) {
        show_error_at(FONTE, 0x660);
        return;
    }
    if crt_fwrite(this + 0x439, 2, 1, f) != 1 {
        show_error_at(FONTE, 0x668);
        return;
    }
    if crt_fwrite(this + 0x43f, 1, 1, f) != 1 {
        show_error_at(FONTE, 0x66f);
        return;
    }
    if *((this + 0x43f) as *const u8) != 0 && crt_fwrite(cpx6_d(this + 0x43b), 4, signed_byte_count(this + 0x43f), f) != signed_byte_count(this + 0x43f) {
        show_error_at(FONTE, 0x679);
        return;
    }
    if crt_fwrite(this + 0x440, 0xc, 0x32, f) != 0x32 { // cpx6: os 50 registros fixos
        show_error_at(FONTE, 0x682);
        return;
    }
    uefa_seeding_crt_fclose(f);
}

/// Comparador da 0x905510: só a original.
unsafe extern "thiscall" fn compare_uefa_seeding_save_to_temp_file(this: usize) {
    // ponytail: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    // (o .tmp vai no .qst pelo zadd_matching_files)
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// { 0x4c6b70 } (thiscall, ret 0, conmebol_seeding.cpp): `ConmebolSeeding::save_to_temp_file`: grava
/// `CM3_TEMP\conmebol_seeding.tmp`: a quantidade de países (+0x6e, i8), os registros de 0xa bytes de +0, +0x6f (u16) e
/// +0x71 (u32). Erro: a caixa "Error" (conmebol_seeding.cpp 0x2ee, 0x2f6, 0x2ff, 0x307, 0x30f), sem fechar o arquivo.
unsafe extern "thiscall" fn conmebol_seeding_save_to_temp_file(this: usize) {
    const FONTE: usize = 0x9a7ed8;
    crate::comparison::branch("cpx_6 gravar sementes da conmebol");
    let f = open_temp_file_for_write(0x9a7f10);
    if f == 0 {
        show_error_at(FONTE, 0x2ee);
        return;
    }
    if crt_fwrite(this + 0x6e, 1, 1, f) != 1 {
        show_error_at(FONTE, 0x2f6);
        return;
    }
    if crt_fwrite(this, 0xa, signed_byte_count(this + 0x6e), f) != signed_byte_count(this + 0x6e) { // cpx6: os registros dos países
        show_error_at(FONTE, 0x2ff);
        return;
    }
    if crt_fwrite(this + 0x6f, 2, 1, f) != 1 {
        show_error_at(FONTE, 0x307);
        return;
    }
    if crt_fwrite(this + 0x71, 4, 1, f) != 1 {
        show_error_at(FONTE, 0x30f);
        return;
    }
    uefa_seeding_crt_fclose(f);
}

/// Comparador da 0x4c6b70: só a original.
unsafe extern "thiscall" fn compare_conmebol_seeding_save_to_temp_file(this: usize) {
    // ponytail: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// { 0x5fb320 } (thiscall, ret 0, host_country.cpp): `HostCountry::save_temp_file`: grava `CM3_TEMP\HostCountry.tmp`:
/// a quantidade (+4, i16) e os registros de 0x22 bytes de [+0], sem conferir a gravação. Erro ao abrir: a caixa
/// "Error" (host_country.cpp 0x475).
unsafe extern "thiscall" fn save_temp_file(this: usize) {
    crate::comparison::branch("cpx_6 gravar paises anfitrioes");
    let f = open_temp_file_for_write(0x9cf10c);
    if f == 0 {
        show_error_at(0x9cf0d8, 0x475);
        return;
    }
    crt_fwrite(this + 4, 2, 1, f);
    crt_fwrite(cpx6_d(this), 0x22, *((this + 4) as *const i16) as i32 as u32, f); // cpx6: os anfitriões
    uefa_seeding_crt_fclose(f);
}

/// Comparador da 0x5fb320: só a original.
unsafe extern "thiscall" fn compare_save_temp_file(this: usize) {
    // ponytail: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%)(this)
}
