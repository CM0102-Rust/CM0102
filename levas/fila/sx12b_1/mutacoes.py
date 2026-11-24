# Mutacoes (sx12b_1). A tabela das nacoes-chave e montada uma vez na criacao do jogo (a carreira ENG cria o jogo do
# zero): o comparador lado a lado acusa na primeira chamada. As datas de inicio e fim das linhas ativas sao reescritas
# depois por key_nation_create_all_comps; a marca de temporada dividida e o dia do fim ficam (lidos pelo ano da
# temporada de cada clube e pela forma fisica): a carreira tambem pode ver.
MUTACOES = [
    ('sx12b_temporada_dividida_trocada',
     'write_u32(at + key_nation_record::SPLIT_SEASON_FLAG, row.split_season as u32); // sx12b_1: the split season flag',
     'write_u32(at + key_nation_record::SPLIT_SEASON_FLAG, !row.split_season as u32); // mutation: the flag inverted',
     'sx12b_1 key nation table built'),
    ('sx12b_fim_da_temporada_7_dias',
     'write_u16(at + key_nation_record::SEASON_END_DAY, end[0]); // sx12b_1: the season end day',
     'write_u16(at + key_nation_record::SEASON_END_DAY, end[0] + 7); // mutation: a week later',
     'sx12b_1 key nation table built'),
]
