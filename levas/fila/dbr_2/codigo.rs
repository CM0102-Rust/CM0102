// ---------- %%LEVA%%a leva: a nação da Polinésia, o nome do cargo de treino e os anos entre duas datas em texto (Database.cpp) ----------

/// { 0x53d940 } (cdecl, Database.cpp): `database_is_polynesian_nation(nacao)`: 1 se a nação tem o id de uma das
/// chaves 0x9cf4d0, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514 (as polinésias), senão 0 (a nação não é conferida). Nas
/// copas da Oceania (oceania_club_champ 0x798220, oceania_nations 0x799e30).
unsafe extern "C" fn database_is_polynesian_nation(nacao: usize) -> u32 {
    let id = *(nacao as *const u32);
    let chaves = [0x9cf4d0usize, 0x9cf52c, 0x9cf4b4, 0x9cf2a4, 0x9cf514]; // polinésias
    chaves.iter().any(|&k| *(k as *const u32) == id) as u32
}

unsafe extern "C" fn comparar_database_is_polynesian_nation(nacao: usize) -> u32 {
    let r = original_fn::<unsafe extern "C" fn(usize) -> u32>(%%I0%%)(nacao);
    let n = database_is_polynesian_nation(nacao);
    SCOREBOARD.record(r == n, || format!("database_is_polynesian_nation({:x}) original={} rust={}", nacao, r, n));
    r
}

/// { 0x53ac40 } (cdecl, Database.cpp): `database_get_coaching_job_name(cargo, genero)`: o nome do cargo (byte com
/// sinal) no texto fixo 0xae1980, pela tabela de saltos 0x53acc8: 5 e 12 "Manager", 6 e 13 "Assistant Manager",
/// 8 e 15 "Coach" (traduzidos pela 0x66f4e0); os demais, texto vazio. Com `genero` não nulo e o idioma (0x66f4d0)
/// 4, grava nele 4 (técnico, auxiliar) ou 1 (treinador). Devolve 0xae1980. Na regeneração (player_regen 0x7a9650).
unsafe extern "C" fn database_get_coaching_job_name(cargo: u32, genero: usize) -> usize {
    let (texto, marca) = match (cargo as i8 as i32).wrapping_sub(5) {
        0 | 7 => (0x9b6440usize, 4u8), // "Manager"
        1 | 8 => (0x9b642c, 4),        // "Assistant Manager"
        3 | 10 => (0x9b6448, 1),       // "Coach"
        _ => {
            *(0xae1980 as *mut u8) = 0;
            return 0xae1980;
        }
    };
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    let idioma: unsafe extern "C" fn() -> i32 = core::mem::transmute(0x66f4d0usize);
    formatar(0xae1980, texto);
    if genero != 0 && idioma() == 4 {
        *(genero as *mut u8) = marca;
    }
    0xae1980
}

unsafe extern "C" fn comparar_database_get_coaching_job_name(cargo: u32, genero: usize) -> usize {
    let o = original_fn::<unsafe extern "C" fn(u32, usize) -> usize>(%%I1%%);
    // o texto (os nomes traduzidos cabem em 64) e o byte do gênero
    let com = [(0xae1980usize, 64usize), (genero, 1)];
    let regioes = if genero != 0 { &com[..] } else { &com[..1] };
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(regioes, || o(cargo, genero), || database_get_coaching_job_name(cargo, genero));
    SCOREBOARD.record(igual && r == n, || format!("database_get_coaching_job_name({}) original={:x} rust={:x} {:x?} x {:x?}", cargo as i8, r, n, mo, mr));
    r
}

/// { 0x540340 } (cdecl, Database.cpp): `database_format_years_between_dates(fim, inicio, texto, tamanho)`: os anos
/// completos entre as datas (+0 dia do ano, +2 ano, palavras): ano(fim) − ano(início), menos 1 se o dia do início é
/// maior (com sinal); escrito em `texto` com "%d" (0x98806c) como palavra sem sinal (negativo vira 65535). Devolve 1.
/// Texto nulo, tamanho < 5 ou uma das datas nula: a original (caixa e 0). Nas telas de recordes do clube e das
/// competições (0x44aa20, 0x4a5690, 0x4a7f50).
unsafe extern "C" fn database_format_years_between_dates(fim: usize, inicio: usize, texto: usize, tamanho: i32) -> u32 {
    if texto == 0 || tamanho < 5 || fim == 0 || inicio == 0 {
        return original_fn::<unsafe extern "C" fn(usize, usize, usize, i32) -> u32>(%%I2%%)(fim, inicio, texto, tamanho);
    }
    let w = |p: usize, d: usize| core::ptr::read_unaligned((p + d) as *const i16);
    let mut anos = w(fim, 2).wrapping_sub(w(inicio, 2));
    if w(inicio, 0) > w(fim, 0) {
        anos = anos.wrapping_sub(1); // o aniversário ainda não chegou
    }
    let fmt: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
    fmt(texto, 0x98806c, anos as u16 as u32);
    1
}

unsafe extern "C" fn comparar_database_format_years_between_dates(fim: usize, inicio: usize, texto: usize, tamanho: i32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize, usize, i32) -> u32>(%%I2%%);
    if texto == 0 || tamanho < 5 || fim == 0 || inicio == 0 {
        return o(fim, inicio, texto, tamanho); // caixa: só a original
    }
    // "%d" de uma palavra: até 6 bytes com o zero
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(texto, 6)], || o(fim, inicio, texto, tamanho),
        || database_format_years_between_dates(fim, inicio, texto, tamanho));
    SCOREBOARD.record(igual && r == n, || format!("database_format_years_between_dates({:x}, {:x}) original={} rust={} {:x?} x {:x?}", fim, inicio, r, n, mo, mr));
    r
}
