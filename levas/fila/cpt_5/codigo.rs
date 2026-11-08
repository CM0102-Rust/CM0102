// ---------- %%LEVA%%a leva: o tempo das cidades: o tempo do dia, a região do weather.cfg e o período da região (weather.cpp) ----------

unsafe fn cpt5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpt5_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A linha seguinte do arquivo (0x90cba0: FILE*, buffer), 0 no fim.
unsafe fn read_config_line(f: usize, buf: *mut u8) -> i32 {
    let ler: unsafe extern "C" fn(usize, *mut u8) -> i32 = core::mem::transmute(0x90cba0usize);
    ler(f, buf)
}

/// { 0x922ee0 } (thiscall, weather.cpp): `Weather::update_city_weather()`, todo dia pelo laço do dia do jogo (0x5cd9aa,
/// [0xde2f74]). Sem a tabela das cidades (+4, 4 bytes por cidade): nada. Cada cidade ([0xae2354], relida a cada volta):
/// as três medidas (+0, +1, +2) viram (medida + sorteio(100)) × 0,5 truncado, presas em 0..100; sorteio(10) zerado dá
/// o vento novo (+3) = sorteio(3).
unsafe extern "thiscall" fn update_city_weather(this: usize) {
    if cpt5_d(this + 4) == 0 || cpt5_d(0xae2354) as i32 <= 0 {
        return;
    }
    crate::comparacao::ramo("cpt_5 tempo do dia");
    let mut i = 0usize;
    loop {
        for k in 0..3usize { // cpt5: as três medidas
            let p = cpt5_d(this + 4) + i * 4 + k;
            let r = cpt5_sortear(100);
            *(p as *mut u8) = match_pl_ftol((r as i16 as i32 + *(p as *const i8) as i32) as f64 * read_f64_at(0x968968)) as u8;
            let p = cpt5_d(this + 4) + i * 4 + k;
            let v = *(p as *const i8);
            if v < 0 {
                *(p as *mut u8) = 0;
            } else if v > 0x64 {
                *(p as *mut u8) = 0x64;
            }
        }
        if cpt5_sortear(10) == 0 { // cpt5: um em dez muda o vento
            crate::comparacao::ramo("cpt_5 vento novo");
            let r = cpt5_sortear(3); // cpt5: o vento novo
            *((cpt5_d(this + 4) + i * 4 + 3) as *mut u8) = r as u8;
        }
        i += 1;
        if i as i32 >= cpt5_d(0xae2354) as i32 {
            break;
        }
    }
}

// ponytail: sorteia 3 ou 4 vezes por cidade: só a original; a cópia leva os ramos.
unsafe extern "thiscall" fn comparar_update_city_weather(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// { 0x922c40 } (thiscall, ret 8, weather.cpp): `Weather::read_region(f, regiao)`: uma linha do weather.cfg (0x90cba0,
/// buffer de 2000) com "%ld %s" (0xad4e24) no id (+0) e no nome (+4) da região (0x6e bytes); depois os quatro
/// períodos (0x922d20) em +0x1e, +0x31, +0x44 e +0x57. Devolve 1 se leu tudo (só o al, como a original).
unsafe extern "thiscall" fn read_region(this: usize, f: usize, regiao: usize) -> u8 {
    let ler: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(0x946fd8usize);
    let periodo: unsafe extern "thiscall" fn(usize, usize, usize) -> u8 = core::mem::transmute(0x922d20usize);
    crate::comparacao::ramo("cpt_5 regiao do clima");
    let mut linha = [0u8; 2000];
    if read_config_line(f, linha.as_mut_ptr()) == 0 {
        return 0;
    }
    if ler(linha.as_ptr(), 0xad4e24, regiao, regiao + 4) != 2 {
        return 0;
    }
    for k in [0x1e, 0x31, 0x44, 0x57] {
        if periodo(this, f, regiao + k) == 0 {
            return 0;
        }
    }
    1
}

// ponytail: lê do arquivo (não dá para ler duas vezes): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_read_region(this: usize, f: usize, regiao: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I1%%)(this, f, regiao)
}

/// { 0x922d20 } (thiscall, ret 8, weather.cpp): `Weather::read_region_period(f, periodo)`: arquivo nulo é a caixa
/// "Error" (weather.cpp, 0xea) e 0. Uma linha (0x90cba0) com o formato de 0xad4e2c (o nome e 18 inteiros: o dia,
/// cinco W, cinco P e sete H); com os 19 lidos, o período (0x13 bytes) recebe o dia (palavra em +0) e os 17 bytes
/// seguintes (+2..+0x12). Devolve 1 se leu (só o al).
unsafe extern "thiscall" fn read_region_period(_this: usize, f: usize, periodo: usize) -> u8 {
    let ler: unsafe extern "C" fn(*const u8, usize, ...) -> i32 = core::mem::transmute(0x946fd8usize);
    if f == 0 {
        show_error_at(0xad4dec, 0xea);
        return 0;
    }
    crate::comparacao::ramo("cpt_5 periodo do clima");
    let mut linha = [0u8; 2000];
    if read_config_line(f, linha.as_mut_ptr()) == 0 {
        return 0;
    }
    // ponytail: o nome tem 200 bytes na original; aqui 2000 para o %s não estourar (com os nomes do arquivo, igual).
    let mut nome = [0u8; 2000];
    let mut v = [0i32; 18];
    let p = v.as_mut_ptr();
    let n = ler(linha.as_ptr(), 0xad4e2c, nome.as_mut_ptr(), p, p.add(1), p.add(2), p.add(3), p.add(4), p.add(5),
                p.add(6), p.add(7), p.add(8), p.add(9), p.add(10), p.add(11), p.add(12), p.add(13), p.add(14),
                p.add(15), p.add(16), p.add(17));
    if n != 0x13 {
        return 0;
    }
    core::ptr::write_unaligned(periodo as *mut u16, v[0] as u16);
    for k in 1..18usize {
        *((periodo + 1 + k) as *mut u8) = v[k] as u8;
    }
    1
}

// ponytail: lê do arquivo: só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn comparar_read_region_period(this: usize, f: usize, periodo: usize) -> u8 {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> u8>(%%I2%%)(this, f, periodo)
}
