# Mutações da 241a leva (itens cpa_1, cpa_2, cpa_3, cpc_2, cpc_1). Uso: python mutacoes.py mutacoes\leva241.py --carreira 2002-06-01
MUTACOES = [
    ('cpa_ordem_posicao', '            *((r + 4) as *mut i8) = c; // a posição', '            *((r + 4) as *mut i8) = c.wrapping_add(1); // mutacao: uma posicao abaixo'),
    ('cpa_mini_posicao', '                pos = pos.wrapping_sub(1); // a posição desce', '                pos = pos.wrapping_sub(0); // mutacao: a posicao nao desce'),
    ('cpa_liberar_propria', '    if propria { // a da liga: zera a contagem e a tabela', '    if !propria { // mutacao: zera a da liga ao soltar uma copia'),
    ('cpa_completa_rodadas', '            if ((read_u8(table + LEAGUE_TABLE_ROW_SIZE * i + 0x11) as i8) as i16) < rounds { // ainda falta jogar', '            if ((read_u8(table + LEAGUE_TABLE_ROW_SIZE * i + 0x11) as i8) as i16) < rounds - 1 { // mutacao: uma rodada antes'),
    ('cpa_tv_valor', '            add_tv(get_club_finance(*(G_FINANCE_TABLE as *const usize), clube), value); // o dinheiro da TV', '            add_tv(get_club_finance(*(0xae2a38 as *const usize), clube), value.wrapping_mul(2)); // mutacao: o dobro'),
    ('cpa_honras_ordem', '    register(league, champion, runner_up, terceiro, destaque) // campeão, vice, terceiro', '    register(league, champion, terceiro, runner_up, destaque) // mutacao: vice e terceiro trocados'),
    ('cpa_historico_ano', '    core::ptr::write_unaligned((r + 8) as *mut u16, *(G_CURRENT_DATE_YEAR as *const u16)); // o ano', '    core::ptr::write_unaligned((r + 8) as *mut u16, (*(0xae2c92 as *const u16)).wrapping_add(1)); // mutacao: o ano seguinte'),
    ('cpa_confronto_titulo', '            return 1; // briga pelo título', '            return 3; // mutacao: a zona no lugar do titulo'),
    ('cpa_forca_reputacao', '    strength = strength.wrapping_add((core::ptr::read_unaligned((clube + club::REPUTATION) as *const i16) as i32) << 4); // a reputação', '    strength = strength.wrapping_add((core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32) << 3); // mutacao: metade'),
    ('cpa_colocado_lugar', '    read_usize(p + [0xa, 0xe, 0x12][place as usize]) // campeão, vice, terceiro', '    read_usize(p + [0xe, 0xa, 0x12][place as usize]) // mutacao: campeao e vice trocados'),
    ('cpc_amistoso_do_clube', 'let of_club = u(a + 0xc) == clube; // o clube do amistoso', 'let of_club = u(a + 0xc) != clube; // mutacao: outro clube'),
    ('cpc_lista_da_data', 'list(*(G_FIXTURE_MANAGER as *const usize), year, day, slice, group) // o gerenciador de partidas', 'list(*(0xae2a58 as *const usize), year, day, (slice + 1) % 3, group) // mutacao: a fatia seguinte'),
    ('cpc_proxima_competicao', '(comp == -1 || comp == d(0)) // a competição', '(comp == -1 || comp != d(0)) // mutacao: a competicao trocada'),
    ('cpc_estadio_em_uso', 'if e != 0 && e == estadio { // o estádio da partida', 'if e != 0 && e != estadio { // mutacao: outro estadio'),
    ('cpc_data_amistoso', 'acertar(this + 3, high | b(1), month, year, week) // a data com o ano dado', 'acertar(this + 3, high | b(1), month, year.wrapping_add(1), week) // mutacao: o ano seguinte'),
]
