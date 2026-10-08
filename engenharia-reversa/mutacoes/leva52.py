# Mutacoes da leva52 (ver docs/ROTINAS.md). Uso: python mutacoes.py mutacoes\leva52.py
MUTACOES = [
    ('t_ultimo', "*lista(byte(j, 0x28) as isize) = *lista(*n as isize - 1);", "*lista(byte(j, 0x28) as isize) = *lista(*n as isize - 2);"),
    ('t_movido', "        *((movido + 0x28) as *mut i8) = idx;", "        *((movido + 0x28) as *mut i8) = idx + 1;"),
    ('t_na', "        if na(*marcado, j) != 0 {", "        if na(*marcado, j) == 0 {"),
    ('t_lista', "            if *((q + 0x1aa) as *const usize) == j {\n                *((q + 0x1aa) as *mut usize) = 0;", "            if *((q + 0x1aa) as *const usize) != 0 {\n                *((q + 0x1aa) as *mut usize) = 0;"),
    ('t_modo', "    if modo != -3 && j != *((this + 0xf5a2)", "    if modo != -2 && j != *((this + 0xf5a2)"),
    ('t_atac', "    if f717410(j) & 0x40 != 0 || f717410(j) & 0x20 != 0 {", "    if f717410(j) & 0x40 != 0 {"),
    ('t_904b', "            if *((bloco_tec + 0x904b) as *const u8) == 0 {", "            if *((bloco_tec + 0x904b) as *const u8) != 0 {"),
    ('t_sel', "            if selecao(clube) != 0 {", "            if selecao(clube) == 0 {"),
    ('t_faixa', "if id >= npes - 16 && id < npes &&", "if id >= npes - 15 && id < npes &&"),
    ('t_filtro', "if escolhido != 0 && (f717410(q) & 1 != 0 || f717410(q) & 4 != 0 || f717410(q) & 2 != 0)", "if escolhido != 0 && (f717410(q) & 1 != 0 || f717410(q) & 4 != 0)"),
    ('t_15000', "        } else if r & 0x10 != 0 {\n            15000", "        } else if r & 0x10 != 0 {\n            20000"),
    ('t_lados', "        if r & 0x80 != 0 || r & 0x800 != 0 {", "        if r & 0x80 != 0 {"),
    ('t_maior', "        if v > melhor || escolhido == 0 {", "        if v >= melhor || escolhido == 0 {"),
    ('t_goleiro', "    if f717410(j) == 1 {\n        let campo = if lado_j == 0", "    if f717410(j) & 1 != 0 {\n        let campo = if lado_j == 0"),
]
