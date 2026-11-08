// ---------- %%LEVA%%a leva: os atributos vistos por quem olha: se o atributo do jogador aparece, o valor dele para quem olha e o nome do atributo de técnico (Database.cpp) ----------

/// { 0x543280 } (cdecl, Database.cpp): `staff_is_player_attribute_visible(quem_ve, pessoa, atributo)`: 1 se quem vê
/// enxerga o atributo (byte com sinal) da pessoa. Visível sem conta: `quem_ve` ou `pessoa` nulos, sem a tabela da
/// névoa ([0xae2a60]), névoa desligada ([0x9cc270] = 0), sem a ficha de quem vê (fog_of_war 0x5afde0, thiscall na
/// tabela) ou conhecimento (0x5b02f0, thiscall na ficha) 3. Conhecimento 1 ou 2: os atributos 1, 0x11, 0x15, 0x1a e
/// 0x1b sempre; os outros pelo resto do id da pessoa por 4 (`m`): o atributo deslocado (0, 1, 2 ou 3 conforme `m`)
/// divisível por 4 (conhecimento 1) ou por 2 (conhecimento 2), ou o seguinte (m = 0) / o mesmo (m ≠ 0) divisível
/// por 3. Outro conhecimento: 0. Só `al`. A 0x5b02f0 guarda o último clube achado em [0xae1080] (o mesmo valor
/// nas duas). Nas telas de busca e filtros (search_screens, SearchFilters 0x807450), da pessoa e do clube.
unsafe extern "C" fn staff_is_player_attribute_visible(quem_ve: usize, pessoa: usize, atributo: u32) -> u8 {
    let tabela = *(0xae2a60 as *const usize);
    if quem_ve == 0 || pessoa == 0 || tabela == 0 || *(0x9cc270 as *const u8) == 0 {
        return 1;
    }
    let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5afde0usize);
    let conhecimento: unsafe extern "thiscall" fn(usize, usize) -> u8 = core::mem::transmute(0x5b02f0usize);
    let f = ficha(tabela, quem_ve);
    if f == 0 {
        return 1;
    }
    let k = conhecimento(f, pessoa);
    if k == 3 {
        return 1;
    }
    if k != 1 && k != 2 {
        return 0;
    }
    let x = atributo as u8 as i8;
    if matches!(x, 1 | 0x11 | 0x15 | 0x1a | 0x1b) {
        return 1;
    }
    let x = x as i32;
    let divisor = if k == 2 { 2 } else { 4 }; // (dbt)
    let (a, b) = match *(pessoa as *const i32) % 4 {
        0 => (x, x + 1),
        1 => (x + 1, x + 1),
        2 => (x + 2, x + 2),
        _ => (x + 3, x + 3),
    };
    (a % divisor == 0 || b % 3 == 0) as u8
}

unsafe extern "C" fn comparar_staff_is_player_attribute_visible(quem_ve: usize, pessoa: usize, atributo: u32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(%%I0%%)(quem_ve, pessoa, atributo);
    let n = staff_is_player_attribute_visible(quem_ve, pessoa, atributo);
    PLACAR.registrar(r == n, || format!("staff_is_player_attribute_visible({:x}, {:x}, {}) original={} rust={}", quem_ve, pessoa, atributo as u8 as i8, r, n));
    r
}

/// { 0x543440 } (cdecl, Database.cpp): `staff_get_player_attribute_for_viewer(quem_ve, pessoa, atributo)`: o
/// atributo (1 a 0x1f, byte com sinal) do jogador (+0x61) da pessoa como quem vê o enxerga: com a névoa ligada
/// ([0x9cc270]), quem tem ficha (0x5afde0) e não vê o atributo (0x543280, pelo endereço do jogo) recebe 0xf9 (-7,
/// "desconhecido"); senão o valor de staff_compute_player_attribute (0x543540). Só `al`. Pessoa nula ou atributo fora
/// da faixa: a original (caixa e 0xff). Na comparação de jogadores e no perfil da pessoa (staff_screens 0x877470,
/// 0x879ac0, 0x87a754).
unsafe extern "C" fn staff_get_player_attribute_for_viewer(quem_ve: usize, pessoa: usize, atributo: u32) -> u8 {
    let x = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&x) {
        return original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(%%I1%%)(quem_ve, pessoa, atributo);
    }
    let calcular: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x543540usize);
    let jogador = core::ptr::read_unaligned((pessoa + 0x61) as *const usize);
    if *(0x9cc270 as *const u8) != 0 {
        let ficha: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x5afde0usize);
        let visivel: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543280usize);
        if ficha(*(0xae2a60 as *const usize), quem_ve) != 0 && visivel(quem_ve, pessoa, atributo) == 0 {
            return 0xf9; // desconhecido (dbt)
        }
    }
    calcular(pessoa, jogador, atributo) as u8
}

unsafe extern "C" fn comparar_staff_get_player_attribute_for_viewer(quem_ve: usize, pessoa: usize, atributo: u32) -> u8 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize, u32) -> u8>(%%I1%%)(quem_ve, pessoa, atributo);
    let x = atributo as u8 as i8;
    if pessoa == 0 || !(1..=0x1f).contains(&x) {
        return r; // caixa: só a original
    }
    let n = staff_get_player_attribute_for_viewer(quem_ve, pessoa, atributo);
    PLACAR.registrar(r == n, || format!("staff_get_player_attribute_for_viewer({:x}, {:x}, {}) original={} rust={}", quem_ve, pessoa, x, r, n));
    r
}

/// Os nomes dos 12 atributos de técnico (abreviado, inteiro) da 0x543b10, pelos endereços dos textos no exe; o 8º
/// (gestão de pessoas) abreviado é um formato com o texto de [0xad9c64].
const STAFF_ATTRIBUTE_NAMES: [(usize, usize); 12] = [
    (0x9b7c90, 0x9b7c80), // Ada / Adaptability
    (0x9b7c7c, 0x9b7c64), // CGk / Coaching Goalkeepers
    (0x9b7c60, 0x9b7c44), // Coa / Coaching Outfield Players
    (0x9b7a68, 0x9b7a58), // Det / Determination
    (0x9b7c40, 0x9b7c28), // Jud / Judging Player Ability
    (0x9b7c24, 0x9b7c08), // JPt / Judging Player Potential
    (0x9b7c04, 0x9b7bf0), // Dis / Level of Discipline
    (0x9b7bcc, 0x9b7bbc), // Man<%s - COMMENT - man management> / Man Management
    (0x9b7bb8, 0x9b7bac), // Mot / Motivating
    (0x9b7ba8, 0x9b7b98), // Phy / Physiotherapy
    (0x9b7b94, 0x9b7b80), // Tct / Tactical Knowledge
    (0x9b7b7c, 0x9b7b64), // Yng / Working With Youngsters
];

/// { 0x543b10 } (cdecl, Database.cpp): `database_get_non_player_attribute_name(atributo, texto, tamanho, abreviado)`:
/// escreve em `texto` o nome do atributo de técnico (1 a 12, byte com sinal), abreviado ou inteiro, pelo texto
/// traduzido do Langlib (0x66f4e0, pelo endereço do jogo); devolve 1. Atributo fora da faixa, texto nulo ou tamanho
/// (curto com sinal) abaixo de 100: a original (caixa e 0). Nas telas de busca (search_screens) e no perfil da
/// pessoa (staff_screens 0x87ad97).
unsafe extern "C" fn database_get_non_player_attribute_name(atributo: u32, texto: usize, tamanho: u32, abreviado: u32) -> u32 {
    let x = atributo as u8 as i8;
    if !(1..=12).contains(&x) || texto == 0 || (tamanho as u16 as i16) < 100 {
        return original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(%%I2%%)(atributo, texto, tamanho, abreviado);
    }
    let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    *(texto as *mut u8) = 0;
    let (curto, longo) = STAFF_ATTRIBUTE_NAMES[x as usize - 1];
    if abreviado == 0 {
        formatar(texto, longo);
    } else if x == 8 {
        formatar(texto, curto, 0xad9c64usize);
    } else {
        formatar(texto, curto); // (dbt)
    }
    1
}

unsafe extern "C" fn comparar_database_get_non_player_attribute_name(atributo: u32, texto: usize, tamanho: u32, abreviado: u32) -> u32 {
    let o = original_fn::<unsafe extern "C" fn(u32, usize, u32, u32) -> u32>(%%I2%%);
    let x = atributo as u8 as i8;
    if !(1..=12).contains(&x) || texto == 0 || (tamanho as u16 as i16) < 100 {
        return o(atributo, texto, tamanho, abreviado);
    }
    // os 100 primeiros bytes (o tamanho mínimo aceito): os nomes cabem neles
    let reg = [(texto, 100)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(atributo, texto, tamanho, abreviado),
        || database_get_non_player_attribute_name(atributo, texto, tamanho, abreviado));
    PLACAR.registrar(igual && r == n, || format!("database_get_non_player_attribute_name({}, {:x}, {}, {}) original={} rust={} {:x?} x {:x?}", x, texto, tamanho as u16 as i16, abreviado, r, n, mo, mr));
    r
}
