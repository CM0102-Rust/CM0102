# Mutacoes (sx16_9) pela prova do save (mutacoes.py --prova-save N): o .qst gravado com e sem as rotinas. Nenhuma abre
# caixa de erro (os tamanhos gravados continuam batendo com as contagens).
# - o news.dat com a contagem de caixas no lugar do ultimo id (16): as noticias depois da carga recebem outros ids;
# - o retire.dat com as duas tabelas de 16 bytes trocadas (fichas livres no lugar dos registros de nao jogador de
#   reserva e vice-versa): a carga as troca e as regeneracoes seguintes usam a tabela errada.
MUTACOES = [
    ('sx16_9_ultimo_id_trocado',
     '    if exe::zwrite(this + news_fields::LAST_ID, 4, 1, z) != 1 {',
     '    if exe::zwrite(this + 0xc, 4, 1, z) != 1 { // mutation: the inbox count as the last id'),
    ('sx16_9_tabelas_trocadas',
     '    (player_regen_fields::SPARE_NON_PLAYER_RECORDS, player_regen_fields::SPARE_NON_PLAYER_COUNT, 0x10, 0x1a71, 0x1a78),\n    (player_regen_fields::PLAYER_DATA_POOL, player_regen_fields::PLAYER_DATA_POOL_COUNT, 0x10, 0x1a7f, 0x1a86),',
     '    (player_regen_fields::PLAYER_DATA_POOL, player_regen_fields::PLAYER_DATA_POOL_COUNT, 0x10, 0x1a7f, 0x1a86), // mutation\n    (player_regen_fields::SPARE_NON_PLAYER_RECORDS, player_regen_fields::SPARE_NON_PLAYER_COUNT, 0x10, 0x1a71, 0x1a78),'),
]
