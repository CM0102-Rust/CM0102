# Os pedidos de observacao do tecnico humano e as ordens dos relatorios (scout_manager.cpp; leitor sx19b, rodada 5,
# 24/11/2026). Rotinas inteiras: 0x7f8ff0..0x7f9185 (ret 8), 0x7f9290..0x7f9516 (ret 8), 0x7f9520..0x7f9685 (ret 8),
# 0x7fc270..0x7fc29e (ret), 0x7fc2a0..0x7fc306 (ret) e 0x7fc310..0x7fc345 (ret 4).
# Quem chama: a ficha do jogador (status e pedido de observacao: 0x889880, 0x88b043, 0x88c419, sempre com clube 0); a
# observacao de uma busca do PLAYER_SEARCH (0x7b64a6: o relatorio feito); o bsearch e o qsort dos pedidos
# (has_scout_request, remove_scout_request, collect_club_scout_targets... com 0x7fc270) e o qsort dos relatorios
# (send_scout_reports 0x7f997b, com 0x7fc2a0); a saida de um tecnico humano (0x6b2ce2). A carreira de ferias nao tem
# pedidos de observacao (cobertura 0).
# TELAS: o caminho no jogo: na ficha de um jogador de outro clube, o botao de pedir aos olheiros (a mensagem de status
# e o pedido); dias depois os relatorios dos olheiros (o relatorio conta no pedido e as noticias ordenam os
# relatorios); um tecnico humano saindo do jogo (Opcoes > tirar um tecnico, num jogo com dois humanos).
# Convencoes conferidas na desmontagem:
# - 0x7f8ff0 thiscall(gerente, pessoa, clube) ret 8 -> al; 0x7f8d70 has_scout_request thiscall(gerente, pessoa,
#   clube, 0) ret 0xc (ja Rust); 0x7f6cd0 scout_record (ja Rust; o tecnico e um dword da pilha escrito so quando o clube
#   veio nulo); 0x7ae950 get_scout thiscall(busca) e 0x7b7490 scout_covers_nation thiscall(busca, nacao) ret 4 (ja
#   Rust); 0x7f9190 can_club_scout_nation thiscall(gerente, nacao, clube) ret 8 (ja Rust); resposta neg/sbb/and 0xfd/
#   add 4: 1 ou 4;
# - 0x7f9290 thiscall(gerente, &id da pessoa, &id do clube) ret 8: 0x944e46 malloc(0xaa), 0x945667 realloc((cap+10) x
#   17) (falhando: a caixa nas linhas 0x6f2/0x6fe, 0x5cc7d0, 0x5a8f60 e exit(-1)); 0x417c10 operator=(pedido+8, &tmp,
#   0xae2c90); 0x945333 qsort(pedidos, n, 0x11, 0x7fc270);
# - 0x7f9520 thiscall(gerente, &id da pessoa, &id do clube) ret 8; quadro SEH so para a data local; 0x946464
#   bsearch(&chave {clube, pessoa}, pedidos, n, 0x11, 0x7fc270); 0x549f70 Date::Date thiscall(tmp, 0x1b, 4, 0x7b1, -1)
#   ret 0x10; 0x417c10 operator=(pedido+8, ...);
# - 0x7fc270 / 0x7fc2a0 cdecl(a, b) -> eax;
# - 0x7fc310 thiscall(gerente, tecnico) ret 4; 0x7b14a0 PLAYER_SEARCH::clear_results thiscall(busca, 1) ret 4 (sx17_2,
#   pelo endereco).
# Defeito do original: no get_scout_request_status com clube dado, o tecnico passado ao scout_record e lixo da pilha
# (os chamadores passam sempre 0; a copia usa o tecnico da vez).
# Comparadores: o status e as duas ordens lado a lado; pedido, relatorio e limpeza so a original, conferidos depois.
TELAS = True
ROTINAS = [('get_scout_request_status', 0x7f8ff0, [0x81, 0xec, 0x10, 0x02, 0x00, 0x00]),
           ('add_scout_request', 0x7f9290, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('record_scout_report', 0x7f9520, [0x6a, 0xff, 0x68, 0x1b, 0x10, 0x96, 0x00]),
           ('scout_manager_compare_report_keys', 0x7fc270, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('scout_manager_compare_report_records', 0x7fc2a0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('clear_manager_lists', 0x7fc310, [0x51, 0x53, 0x56, 0x32, 0xdb])]
DOCS = ('os pedidos de observação do técnico humano e as ordens dos relatórios',
        'the human manager\'s scout requests and the report orders',
        [('get_scout_request_status', 'SCOUT_MANAGER::get_scout_request_status', '0x7f8ff0', 'scout_manager.cpp', 'thiscall, ret 8',
          'diz se dá para pedir aos olheiros que observem o jogador: 3 já pedido; a nação olhada é a do clube dele, senão a dele (nenhuma: 2); 0 se uma das 10 buscas do técnico tem olheiro que cobre a nação; sem busca de olheiro (tipo 4 com olheiro): 2; senão 1 se o clube alcança a nação, 4 se não',
          'tells whether the scouts can be asked to watch the player: 3 already asked; the nation looked at is his club\'s, else his own (none: 2); 0 when one of the manager\'s 10 searches has a scout covering the nation; no scout search (type 4 with a scout): 2; otherwise 1 when the club reaches the nation, 4 when not'),
         ('add_scout_request', 'SCOUT_MANAGER::add_scout_request', '0x7f9290', 'scout_manager.cpp', 'thiscall, ret 8',
          'acrescenta o pedido (jogador, clube; sem clube, o do técnico da vez) com a data de hoje e nenhum relatório, crescendo o vetor de 10 em 10, e reordena por jogador e clube',
          'appends the request (player, club; no club: the current manager\'s) with today\'s date and no reports, growing the array by ten, and sorts it by player and club'),
         ('record_scout_report', 'SCOUT_MANAGER::record_scout_report', '0x7f9520', 'scout_manager.cpp', 'thiscall, ret 8',
          'marca um relatório feito no pedido do par (se houver): a data volta para 27/04/1969 e o total de relatórios sobe um',
          'marks a report made on the pair\'s request (if any): the date goes back to 27/04/1969 and the report count goes up by one'),
         ('scout_manager_compare_report_keys', 'scout_manager_compare_report_keys', '0x7fc270', 'scout_manager.cpp', 'cdecl',
          'ordem dos pedidos de observação para o qsort e o bsearch: nulo primeiro, depois pelo jogador e pelo clube',
          'the scout request order for qsort and bsearch: null first, then by player and club'),
         ('scout_manager_compare_report_records', 'scout_manager_compare_report_records', '0x7fc2a0', 'scout_manager.cpp', 'cdecl',
          'ordem dos relatórios de olheiro (0x67 bytes): nulo e sem relatório primeiro, depois jogador, clube, nota atual e nota de potencial',
          'the scout report order (0x67 bytes): null and empty first, then player, club, current rating and potential rating'),
         ('clear_manager_lists', 'SCOUT_MANAGER::clear_manager_lists', '0x7fc310', 'scout_manager.cpp', 'thiscall, ret 4',
          'esvazia os resultados das 10 buscas do técnico humano que sai do jogo',
          'empties the results of the 10 searches of the human manager leaving the game')])
CAMPOS = []
INTERESSA = []
