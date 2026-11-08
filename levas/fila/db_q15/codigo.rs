// ---------- %%LEVA%%a leva: a entrada da pessoa no clube (Database.cpp) ----------

/// Põe a `pessoa` na primeira das `n` casas vazias a partir de `clube + d`; false se não há.
unsafe fn ocupar_casa_do_clube(clube: usize, d: usize, n: usize, pessoa: usize) -> bool {
    let casa = |i: usize| (clube + d + 4 * i) as *mut usize;
    match (0..n).find(|&i| core::ptr::read_unaligned(casa(i)) == 0) {
        Some(i) => {
            core::ptr::write_unaligned(casa(i), pessoa);
            true
        }
        None => false,
    }
}

/// { 0x53bd10 } (cdecl, Database.cpp): põe a `pessoa` no `clube` no `cargo` (byte, tabela de salto
/// 0x53c14c). Pessoa da faixa dos humanos (id ≥ [0xae2364] − 16) só no cargo 5. Com `data`: as 8 bytes
/// dela vão para a pessoa (+0x29 se o clube é seleção, id ≥ [0xae235c] − 2 × [0xae2350]; senão +0x3e;
/// 0x417c10 copia +2, +0 e +4 e repete numa local que ninguém lê). Com `reduzir`: cargos 11, 12, 13 e 15
/// viram 11, os outros 0. Jogador (+0x61) com habilidade atual (+5) > 85 num clube sem técnico humano
/// (0x600a50 com 0, 1, 0, 0): 6 vira 13 e 8 vira 15. Casas: 1 +0xbf; 2..4 +0xc3 (3); 5 +0xcf; 6, 7 +0xd3;
/// 8 +0x19f (5); 9 +0x1b3 (7); 10 +0x1cf (3); 11 elenco +0xd7 (50); 12 +0xcf e o elenco (sem elenco: 0
/// com o +0xcf já gravado); 13 +0xd3 e o elenco (sem elenco fica 6); 15 +0x19f e o elenco (sem elenco
/// fica 8); 0 nenhuma; 14 e fora: 0. Com `gravar`: clube e cargo na pessoa (+0x24/+0x28 seleção,
/// +0x39/+0x3d clube). Devolve 1. Clube ou pessoa nulos: a original (caixa "Error" e 0).
unsafe extern "C" fn database_add_staff_to_club(clube: usize, pessoa: usize, cargo: u32, data: usize, gravar: u32, reduzir: u32) -> u32 {
    if clube == 0 || pessoa == 0 {
        return orig::<unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32>(%%I0%%)(clube, pessoa, cargo, data, gravar, reduzir);
    }
    let gi = |a: usize| *(a as *const i32);
    let selecao = || gi(clube) >= gi(0xae235c).wrapping_sub(gi(0xae2350).wrapping_mul(2));
    if gi(pessoa) >= gi(0xae2364).wrapping_sub(16) && cargo as u8 != 5 {
        return 0;
    }
    if data != 0 {
        let d = if selecao() { pessoa + 0x29 } else { pessoa + 0x3e };
        core::ptr::write_unaligned((d + 2) as *mut u16, core::ptr::read_unaligned((data + 2) as *const u16));
        core::ptr::write_unaligned(d as *mut u16, core::ptr::read_unaligned(data as *const u16));
        core::ptr::write_unaligned((d + 4) as *mut u32, core::ptr::read_unaligned((data + 4) as *const u32));
    }
    let mut c = cargo as u8;
    if reduzir != 0 {
        c = if matches!(c, 0xb | 0xc | 0xd | 0xf) { 0xb } else { 0 };
    }
    if core::ptr::read_unaligned((pessoa + 0x61) as *const usize) != 0 {
        let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
        if humano(clube, 0, 1, 0, 0) == 0 {
            let jogador = core::ptr::read_unaligned((pessoa + 0x61) as *const usize);
            if core::ptr::read_unaligned((jogador + 5) as *const i16) > 0x55 {
                if c == 6 {
                    c = 0xd;
                } else if c == 8 {
                    c = 0xf;
                }
            }
        }
    }
    let ocupar = |d: usize, n: usize| ocupar_casa_do_clube(clube, d, n, pessoa);
    let unica = |d: usize| {
        let p = (clube + d) as *mut usize;
        if core::ptr::read_unaligned(p) != 0 {
            return false;
        }
        core::ptr::write_unaligned(p, pessoa);
        true
    };
    let ok = match c as i8 {
        0 => true,
        1 => unica(0xbf),
        2..=4 => ocupar(0xc3, 3),
        5 => unica(0xcf),
        6 | 7 => unica(0xd3),
        8 => ocupar(0x19f, 5),
        9 => ocupar(0x1b3, 7),
        10 => ocupar(0x1cf, 3),
        11 => ocupar(0xd7, 50),
        12 => unica(0xcf) && ocupar(0xd7, 50),
        13 => {
            if !unica(0xd3) {
                return 0;
            }
            if !ocupar(0xd7, 50) {
                c = 6; // sem vaga no elenco: fica só na casa +0xd3
            }
            true
        }
        15 => {
            if !ocupar(0x19f, 5) {
                return 0;
            }
            if !ocupar(0xd7, 50) {
                c = 8; // sem vaga no elenco: fica só em +0x19f
            }
            true
        }
        _ => false, // 14 e fora de 0..15
    };
    if !ok {
        return 0;
    }
    if gravar != 0 {
        if selecao() {
            core::ptr::write_unaligned((pessoa + 0x24) as *mut usize, clube);
            *((pessoa + 0x28) as *mut u8) = c;
        } else {
            core::ptr::write_unaligned((pessoa + 0x39) as *mut usize, clube);
            *((pessoa + 0x3d) as *mut u8) = c; // clube e cargo do clube
        }
    }
    1
}

unsafe extern "C" fn comparar_database_add_staff_to_club(clube: usize, pessoa: usize, cargo: u32, data: usize, gravar: u32, reduzir: u32) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32>(%%I0%%);
    if clube == 0 || pessoa == 0 {
        return o(clube, pessoa, cargo, data, gravar, reduzir); // caixa: só a original
    }
    // grava só no clube (581 bytes) e na pessoa (110); 0x600a50 só consulta
    let regioes = [(clube, 0x245), (pessoa, 0x6e)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado(&regioes, || o(clube, pessoa, cargo, data, gravar, reduzir),
        || database_add_staff_to_club(clube, pessoa, cargo, data, gravar, reduzir));
    PLACAR.registrar(igual && r == n, || format!("database_add_staff_to_club({:x}, {:x}, {}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}",
        clube, pessoa, cargo as u8, data, gravar, reduzir, r, n, mo, mr));
    r
}
