# Mutações da 241a leva (itens cpa_1, cpa_2, cpa_3, cpc_2, cpc_1). Uso: python mutacoes.py mutacoes\leva241.py --carreira 2002-06-01
MUTACOES = [
    ('cpa_ordem_posicao', '            *((r + 4) as *mut i8) = c; // a posição', '            *((r + 4) as *mut i8) = c.wrapping_add(1); // mutacao: uma posicao abaixo'),
    ('cpa_mini_posicao', '                pos = pos.wrapping_sub(1); // a posição desce', '                pos = pos.wrapping_sub(0); // mutacao: a posicao nao desce'),
    ('cpa_liberar_propria', '    if propria { // a da liga: zera a contagem e a tabela', '    if !propria { // mutacao: zera a da liga ao soltar uma copia'),
    ('cpa_completa_rodadas', '            if ((read_u8(tab + CPA1_LINHA * i + 0x11) as i8) as i16) < rodadas { // ainda falta jogar', '            if ((read_u8(tab + CPA1_LINHA * i + 0x11) as i8) as i16) < rodadas - 1 { // mutacao: uma rodada antes'),
    ('cpa_tv_valor', '            somar_tv(financas_do_clube(*(0xae2a38 as *const usize), clube), valor); // o dinheiro da TV', '            somar_tv(financas_do_clube(*(0xae2a38 as *const usize), clube), valor.wrapping_mul(2)); // mutacao: o dobro'),
    ('cpa_honras_ordem', '    registrar(liga, campeao, vice, terceiro, destaque) // campeão, vice, terceiro', '    registrar(liga, campeao, terceiro, vice, destaque) // mutacao: vice e terceiro trocados'),
    ('cpa_historico_ano', '    core::ptr::write_unaligned((r + 8) as *mut u16, *(0xae2c92 as *const u16)); // o ano', '    core::ptr::write_unaligned((r + 8) as *mut u16, (*(0xae2c92 as *const u16)).wrapping_add(1)); // mutacao: o ano seguinte'),
    ('cpa_confronto_titulo', '            return 1; // briga pelo título', '            return 3; // mutacao: a zona no lugar do titulo'),
    ('cpa_forca_reputacao', '    forca = forca.wrapping_add((core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32) << 4); // a reputação', '    forca = forca.wrapping_add((core::ptr::read_unaligned((clube + 0x80) as *const i16) as i32) << 3); // mutacao: metade'),
    ('cpa_colocado_lugar', '    read_usize(p + [0xa, 0xe, 0x12][lugar as usize]) // campeão, vice, terceiro', '    read_usize(p + [0xe, 0xa, 0x12][lugar as usize]) // mutacao: campeao e vice trocados'),
    ('cpc_amistoso_do_clube', 'let do_clube = u(a + 0xc) == clube; // o clube do amistoso', 'let do_clube = u(a + 0xc) != clube; // mutacao: outro clube'),
    ('cpc_lista_da_data', 'lista(*(0xae2a58 as *const usize), ano, dia, fatia, grupo) // o gerenciador de partidas', 'lista(*(0xae2a58 as *const usize), ano, dia, (fatia + 1) % 3, grupo) // mutacao: a fatia seguinte'),
    ('cpc_proxima_competicao', '(comp == -1 || comp == d(0)) // a competição', '(comp == -1 || comp != d(0)) // mutacao: a competicao trocada'),
    ('cpc_estadio_em_uso', 'if e != 0 && e == estadio { // o estádio da partida', 'if e != 0 && e != estadio { // mutacao: outro estadio'),
    ('cpc_data_amistoso', 'acertar(this + 3, alto | b(1), mes, ano, semana) // a data com o ano dado', 'acertar(this + 3, alto | b(1), mes, ano.wrapping_add(1), semana) // mutacao: o ano seguinte'),
]
