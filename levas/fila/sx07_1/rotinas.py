# A noticia de que uma pessoa respondeu a uma proposta de contrato (contract_manager_post_offer_response_news 0x4d49a0,
# contract_manager.cpp, 1.552 bytes; leitor sx07, rodada 5, 23/11/2026). A rotina vai de 0x4d49a0 ao ret 8 de 0x4d4fac
# (0x4d4faf e enchimento). Chamada pela decisao das propostas (resolve_contract_offers 0x4ce4e0, ja Rust) e pela
# renovacao recusada do dia dos contratos (contract_day_*, ja Rust), 1.417 vezes na carreira ENG ate 2002-06-01.
# Convencoes conferidas na desmontagem:
# - a rotina: thiscall(gerente de contratos, proposta, recusada), ret 8; o ecx so vai para 0x4cf5f0;
# - 0x4cf5f0 CONTRACT_MANAGER::get_contract_for_club thiscall(pessoa, clube) ret 8; 0x77bbe0 NewsItem::NewsItem
#   thiscall(tipo, importante) ret 8; 0x785790 NewsItem::set_slot thiscall(k, valor) ret 8;
# - 0x53b620 database_is_player_job cdecl(byte); 0x8c61e0 TRANSFER_MANAGER::get_offer_slot thiscall([0xde1f54], k)
#   ret 4 (devolve o endereco da vaga); 0x846920 Shortlist_Manager::staff_interest_in_joining_club thiscall
#   ([tm+0x856], pessoa, clube, 0, tipo, situacao, &motivos) ret 0x18 (o tipo e a situacao vao num byte com lixo em
#   cima; a 0x846920 le so o byte; o valor em ponto flutuante e descartado com fstp st(0));
# - 0x90cfc0 utils_random cdecl(3); 0x53ab90 database_get_staff_age cdecl(pessoa, &fim do contrato da proposta);
#   0x85fa00 STAFF_CONTRACT::days_to_end_date thiscall(contrato) (palavra com sinal);
# - 0x7861e0 News::send_to_club_manager thiscall([0xb7431c], noticia, clube) ret 8; 0x842180
#   collect_interested_clubs thiscall(pessoa, lista, 1, 1, 0) ret 0x14; 0x8d1da0
#   collect_player_news_recipients thiscall([0xde1f54], pessoa, 0x21a, vetor, 0x1e) ret 0x10 (al, com sinal);
# - 0x68d020/0x68d1b0/0x68d040 LLIST; o cursor = a cabeca (dword +2 copiado para +0xa).
# Defeitos do original: com `recusada` ligada e sem contrato com o clube, le a data de fim do contrato nulo (+0x2d) e
# cai; a copia faz o mesmo (le sem conferir).
# Comparador: ensaio (a copia sem mandar, guardando cada envio), depois a original do mesmo estado do gerador; compara
# o gerador e as noticias que chegaram as caixas dos tecnicos humanos (o send_to_club_manager so entrega a humano) com
# os envios da copia a clubes de tecnico humano. A vaga de transferencia vazia (a caixa de erro) so roda a original.
SAVE = False
ROTINAS = [('contract_manager_post_offer_response_news', 0x4d49a0, [0x6a, 0xff, 0x68, 0x66, 0x6e, 0x95, 0x00])]
DOCS = ('a noticia da resposta a uma proposta de contrato',
        'the news item of the answer to a contract offer',
        [('contract_manager_post_offer_response_news', 'contract_manager_post_offer_response_news', '0x4d49a0', 'contract_manager.cpp', 'thiscall, ret 8',
          'monta a noticia 3000 (0xbb8) de que uma pessoa aceitou ou recusou uma proposta de contrato: a pessoa e os nomes, o clube, o estado da proposta, o fim do contrato, a vaga da proposta de transferencia ligada (vaga vazia: a caixa de erro e nada e mandado; senao o numero de serie dela), o tipo de contrato, se a marca nao e cargo de jogador, os motivos que o interesse da lista de observacao devolve, o tecnico do clube com os nomes (-1 sem tecnico), o codigo do fim do contrato atual (so com "recusada": 0x3e7 se acaba antes da data fixa de +91 dias, 0x3e8 antes da de +365 dias, senao 0 e "recusada" se desliga), 0x420e9, um sorteio de 0 a 2, se e renovacao aceita de contrato existente, a marca, o salario, o valor +8, o inicio do contrato, se e veterano (goleiro 20 acima de 37 anos, ou defensor, meia ou atacante 15 ou mais acima de 36, 35 ou 34, pela idade no fim do contrato) e o bit 7 do registro de 79 bytes. Manda, com "recusada" ainda ligada ou na renovacao aceita com menos de 180 dias de contrato, aos clubes que o gerente de transferencias escolhe para as noticias do jogador (ate 30); senao ao clube e, no contrato de jogador aceito, aos clubes interessados nele, menos o proprio clube',
          'builds news item 3000 (0xbb8) that a person accepted or rejected a contract offer: the person and his names, the club, the offer state, the contract end, the linked transfer offer slot (an empty slot gives the error box and nothing is sent; otherwise its serial number), the contract type, whether the mark is not a player job, the reasons the shortlist interest returns, the club manager with his names (-1 without one), the current contract end code (only with "refused": 0x3e7 if it ends before the fixed date +91 days, 0x3e8 before the +365 days one, otherwise 0 and "refused" is cleared), 0x420e9, a draw from 0 to 2, whether it is the agreed renewal of an existing contract, the mark, the wage, the +8 value, the contract start, whether he is a veteran (goalkeeper 20 over 37, or defender, midfielder or attacker 15 or more over 36, 35 or 34, by his age at the contract end) and bit 7 of the 79-byte record. Sends it, with "refused" still set or for an agreed renewal with fewer than 180 days left, to the clubs the transfer manager picks for the player\'s news (up to 30); otherwise to the club and, for an agreed player contract, to the clubs interested in him, except the club itself')])
