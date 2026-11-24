# Mutacoes (sbF_2), carreira com a Grecia ate 2002-08-01. Nenhuma abre caixa.
# - a 1a rodada do mata-mata grego uma semana depois (22/12): a data dos jogos muda (o comparador acusa a cada pedido
#   da tabela; a carreira, no mata-mata de dezembro);
# - o grupo da propria copa com a coluna 1 do pool: os clubes repetem os do grupo 0 (o comparador acusa; a carreira
#   muda os grupos);
# - cada fase de grupo com a coluna seguinte (g + 2): os clubes trocam de grupo (o comparador acusa; a carreira muda).
MUTACOES = [
    ('sbF_2_grecia_semana_depois',
     'cql10r((0x10, 8, 0, 3), (0xf, 11, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (0xc, 6, 0xc, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // sbF2: the Greek Cup 1st round',
     'cql10r((0x10, 8, 0, 3), (0x16, 11, 0, 2, 2, 0), 0x14, (0, 3, 0), 4, (0xc, 6, 0xc, 0), (0, 1, 0), (0, 0xffff_ffff, 0)), // mutation',
     'sbF_2 greek cup knockout rounds'),
    ('sbF_2_primeiro_grupo_coluna_1',
     'exe::league_init_table_row(this, k as u8 as u32, greek_cup_pool_club(this, 0, k as usize), 0, 0);',
     'exe::league_init_table_row(this, k as u8 as u32, greek_cup_pool_club(this, 1, k as usize), 0, 0); // mutation',
     'sbF_2 greek cup first group'),
    ('sbF_2_fase_coluna_seguinte',
     'read_usize(cup_pool_entry(this, g as usize + 1 + k * GREEK_CUP_GROUPS)), // sbF2: the Greek group columns',
     'read_usize(cup_pool_entry(this, (g as usize + 2) % GREEK_CUP_GROUPS + k * GREEK_CUP_GROUPS)), // mutation',
     'sbF_2 greek cup group stage'),
]
