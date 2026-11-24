# As ordens e as escolhas por posicao das listas do olheiro (player_search.cpp; leitor sx17, rodada 5, 24/11/2026):
# a ordem por nota atual (0x7bce80) e potencial (0x7bceb0), a ordem das entradas do relatorio (0x7be640) e os meias
# (0x7bfbd0), atacantes (0x7bfc50) e zagueiros (0x7bfcd0) do relatorio do proximo adversario. Rotinas inteiras
# (0x7bce80..0x7bcea2, 0x7bceb0..0x7bced2, 0x7be640..0x7be66e, 0x7bfbd0..0x7bfc44, 0x7bfc50..0x7bfcc4,
# 0x7bfcd0..0x7bfd44, todas com ret; o resto e enchimento).
# Quem chama: as duas ordens por nota, so o build_scout_targets (por endereco, 0x7b0a3d e 0x7b0a57, ao aparar a lista
# do olheiro); a ordem das entradas, o dia de trabalho do olheiro (qsort no relatorio final do clube-alvo, ja em Rust:
# scout_day_best_players); as escolhas por posicao, o texto do relatorio do proximo adversario (0x7be670). A carreira
# automatica nao passa.
# TELAS: a prova e o Lucas jogando comparando: um olheiro com tarefa (nacao, regiao, competicao, base) e um olheiro
# observando o proximo adversario ate o relatorio final.
# Convencoes conferidas na desmontagem: as ordens cdecl(a, b) -> eax; as escolhas cdecl(quantas: byte, entradas,
# saida) -> al.
# Defeito do original: as escolhas por posicao aceitam o id igual ao total de pessoas (um alem da tabela).
TELAS = True
ROTINAS = [('player_search_compare_results_by_current_rating', 0x7bce80, [0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9]),
           ('player_search_compare_results_by_potential_rating', 0x7bceb0, [0x8b, 0x4c, 0x24, 0x04, 0x85, 0xc9]),
           ('player_search_compare_report_entries', 0x7be640, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('player_search_collect_report_midfielders', 0x7bfbd0, [0x8a, 0x4c, 0x24, 0x04, 0x32, 0xc0]),
           ('player_search_collect_report_strikers', 0x7bfc50, [0x8a, 0x4c, 0x24, 0x04, 0x32, 0xc0]),
           ('player_search_collect_report_defenders', 0x7bfcd0, [0x8a, 0x4c, 0x24, 0x04, 0x32, 0xc0])]
_POSICAO = ('copia as entradas do relatorio marcadas como %s (bit %s das bandeiras +0x14) e nao excluidas (0x40000), com id de 0 ao total de pessoas, e devolve quantas copiou',
            'copies the report entries flagged as %s (bit %s of the +0x14 flags) and not excluded (0x40000), with an id from 0 to the staff count, and returns how many it copied')
DOCS = ('as ordens e as escolhas por posicao das listas do olheiro',
        'the scout list orders and position picks',
        [('player_search_compare_results_by_current_rating', 'player_search_compare_results_by_current_rating', '0x7bce80', 'player_search.cpp', 'cdecl',
          'ordena resultados pela nota atual (+0x21), a maior primeiro; o nulo depois de todos',
          'orders results by current rating (+0x21), highest first; a null after everything'),
         ('player_search_compare_results_by_potential_rating', 'player_search_compare_results_by_potential_rating', '0x7bceb0', 'player_search.cpp', 'cdecl',
          'ordena resultados pela nota potencial (+0x25), a maior primeiro; o nulo depois de todos',
          'orders results by potential rating (+0x25), highest first; a null after everything'),
         ('player_search_compare_report_entries', 'player_search_compare_report_entries', '0x7be640', 'player_search.cpp', 'cdecl',
          'ordena as entradas do relatorio pela nota (+0) e depois pelo id (+4), os maiores primeiro; o nulo depois de todos',
          'orders the report entries by rating (+0) then by id (+4), highest first; a null after everything'),
         ('player_search_collect_report_midfielders', 'player_search_collect_report_midfielders', '0x7bfbd0', 'player_search.cpp', 'cdecl',
          _POSICAO[0] % ('meias', '0x400'), _POSICAO[1] % ('midfielders', '0x400')),
         ('player_search_collect_report_strikers', 'player_search_collect_report_strikers', '0x7bfc50', 'player_search.cpp', 'cdecl',
          _POSICAO[0] % ('atacantes', '0x100'), _POSICAO[1] % ('strikers', '0x100')),
         ('player_search_collect_report_defenders', 'player_search_collect_report_defenders', '0x7bfcd0', 'player_search.cpp', 'cdecl',
          _POSICAO[0] % ('zagueiros', '0x200'), _POSICAO[1] % ('defenders', '0x200'))])
