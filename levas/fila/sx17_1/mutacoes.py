# Mutacoes (sx17_1) pela prova do save (mutacoes.py --prova-save N), sem o 4o campo (a base da prova roda com as
# rotinas desligadas):
# - o construtor com o tipo de lista 3 em vez de 4: toda busca do SCOUT_MANAGER criada no jogo novo sai com +0xa = 3 e o
#   scout_manager.dat do .qst muda;
# - a gravacao com o fim e o inicio da observacao trocados de ordem: o item muda (as duas datas sao diferentes no jogo
#   novo: a de inicio e 27/04/1969, a de fim a data vazia);
# - a carga sem pôr o registro do clube observado na lista global: so acusa se o save de ferias tem lista com clube
#   atribuido (o Lucas de ferias nao atribui olheiros; pode escapar, e o comparador nao a ve: so a original roda).
MUTACOES = [
    ('sx17_1_tipo_de_lista',
     'write_u8(this + player_search_fields::LIST_TYPE, 4); // sx17_1: the default list type',
     'write_u8(this + player_search_fields::LIST_TYPE, 3); // mutation: list type 3'),
    ('sx17_1_datas_trocadas',
     '    (player_search_fields::SCOUTING_END_DATE, 8, 1, 0x95a), (player_search_fields::SCOUTING_START_DATE, 8, 1, 0x961),',
     '    (player_search_fields::SCOUTING_START_DATE, 8, 1, 0x95a), (player_search_fields::SCOUTING_END_DATE, 8, 1, 0x961), // mutation'),
    ('sx17_1_registro_fora_da_lista',
     'exe::llist_add_tail(G_SCOUT_CLUB_TABLE_LIST, record, 4); // sx17_1: the record joins the global list',
     '// mutation: the record stays off the global list'),
]
