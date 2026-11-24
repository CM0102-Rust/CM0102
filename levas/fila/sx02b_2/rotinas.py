# As transferencias passadas antigas para os arquivos anuais e as ofertas canceladas quando o tecnico de um clube sai
# (transfer_manager.cpp; leitor sx02b, rodada 5, 23/11/2026). Cada uma roda 1 vez na carreira ENG ate 2002-06-01.
# Convencoes conferidas:
# - 0x8c3eb0 (clear_old_past_transfers): thiscall(this), ret 0; chamada pela daily_update no dia 0 (1/1/2002 na carreira);
#   0x54cab0 (Date::sub_weeks) thiscall(G_CURRENT_DATE, &corte, 0x34); sprintf(0xde1f64, "transfers_%d.tmp", ano u16);
#   0x90bde0 cdecl("CM3_TEMP", 0xde1f64); fopen(caminho, "ab"); fwrite(registro, 0x25, 1, f); fclose; as caixas "Error"
#   (show_error_box, linhas 0x1332 e 0x133c, que zera G_MOUSE_LEFT_BUTTON_DOWN) e a de memoria (linha 0x135d); malloc,
#   memcpy (rep movs) e free do vetor; a capacidade em +0x866.
# - 0x8c4950 (cancel_club_offers): thiscall(this, clube), ret 4; chamada de 0x695135, 0x6b2c23, 0x7aaf2d e 0x7ab115;
#   0x53b900 (database_is_national_team) cdecl(clube); LLIST local (0x68d020/0x68d040), add_tail(&k, 4), get_next
#   (cursor +0xa = cabeca +2), clear; 0x541f90 (database_collect_club_squad) cdecl(clube, vetor de 50, 0); 0x8c5cf0
#   (get_offers_for_player) thiscall(this, pessoa, lista, 0); por oferta cancelada: 0x8c6db0 thiscall(this, oferta),
#   0x4cf5a0 (get_contract, guardado em G_CONTRACT_LOOKUP_RESULT), 0x84aeb0 thiscall(sm, clube, pessoa) (comprador e
#   alvo; vendedor e cada jogador da troca), 0x4cca70 thiscall(CM, alvo, comprador), 0x4ccb30 thiscall(CM, jogador da
#   troca, vaga da oferta), 0x8c6220 thiscall(this, oferta, 10, 0), 0x8de790 thiscall(this, oferta, clube, 0xb).
# Comparadores: o da clear_old_past_transfers roda so a original e confere o plano (os registros antes do corte, a conta,
# a capacidade e o vetor restante); o da cancel_club_offers so a original (agenda, desfaz ofertas de contrato e posta
# noticias).
SAVE = False
ROTINAS = [('clear_old_past_transfers', 0x8c3eb0, [0x6a, 0xff, 0x68, 0x4b, 0x3c, 0x96, 0x00]),
           ('cancel_club_offers', 0x8c4950, [0x6a, 0xff, 0x68, 0xab, 0x3c, 0x96, 0x00])]
DOCS = ('as transferências antigas para os arquivos anuais e as ofertas de um clube cujo técnico sai',
        'the old transfers to the yearly files and the offers of a club whose manager leaves',
        [('clear_old_past_transfers', 'TRANSFER_MANAGER::clear_old_past_transfers', '0x8c3eb0', 'transfer_manager.cpp', 'thiscall, ret 0',
          'regra: no primeiro dia do ano, as transferências passadas de mais de 52 semanas vão para o fim do arquivo transfers_<ano>.tmp do seu ano (um arquivo aberto por vez) e saem da memória; o vetor novo fica com as restantes e espaço para mais 100. Defeitos: depois de um arquivo que não abre, os outros registros daquele ano contam como movidos sem ir a lugar nenhum; um registro que falha na gravação fica de fora da conta, e o vetor guarda um registro já gravado a mais; sem memória para o vetor novo, os registros gravados ficam também na memória',
          'rule: on the first day of the year, the past transfers older than 52 weeks go to the end of the file transfers_<year>.tmp of their year (one file open at a time) and leave memory; the new array keeps the rest and room for 100 more. Defects: after a file that does not open, the other records of that year count as moved without going anywhere; a record that fails to be written is left out of the count, and the array keeps one already written record too many; without memory for the new array, the written records also stay in memory'),
         ('cancel_club_offers', 'TRANSFER_MANAGER::cancel_club_offers', '0x8c4950', 'transfer_manager.cpp', 'thiscall, ret 4',
          'regra: quando o técnico de um clube (não seleção) sai, cada oferta aberta que o clube fez (menos a transferência futura) e cada oferta aberta pelos jogadores do elenco (menos a transferência futura e o pré-contrato) é cancelada: fecham as ofertas de contrato, saem as marcas de proposta das listas de observação, volta em 10 dias e vai a notícia de negócio desfeito (motivo 0xb). Defeito: a lista das ofertas do elenco nunca é esvaziada entre jogadores, e as ofertas dos jogadores anteriores são canceladas de novo (nova agenda e nova notícia) a cada jogador com ofertas',
          "rule: when a club's manager (not a national team) leaves, each open offer the club made (except a future transfer) and each open offer for its squad players (except a future transfer and a pre-contract) is cancelled: the contract offers are closed, the bid markers leave the shortlists, it is due again in 10 days and the collapse news goes out (reason 0xb). Defect: the list of the squad's offers is never emptied between players, and the earlier players' offers are cancelled again (scheduled again, news again) for each player with offers")])
CAMPOS = []
INTERESSA = []
