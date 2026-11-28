# A lista das transferencias futuras de um clube (transfer_manager.cpp; leitor r7_08, rodada 7, 28/11/2026): estava
# adiada no levas/fila/rxs1_12/FORA.md e rxg_5/FORA.md so por tamanho (o rxg_5 parou porque o Ghidra escondia o ecx das
# Date::operator=; a desmontagem linha a linha resolve). Rotina inteira:
# - 0x8ccb80 TRANSFER_MANAGER::get_club_future_transfer_list (1488 bytes, ate o ret 8 em 0x8ccfb2, mais as duas saidas
#   sem memoria ate 0x8cd10e e a tabela de saltos 0x8cd114/0x8cd134; quadro SEH da LLIST local).
# TELAS: a tela do clube (0x455945) e o orcamento de transferencias de um clube humano com `pending` (0x5988d8). A
# carreira automatica nao passa (0 chamadas na cobertura do mundo).
# Caminho no jogo para o Lucas, jogando comparando: com um clube humano que tenha propostas em andamento (feitas e
# recebidas, uma com jogador na troca), abrir as financas do clube (o orcamento de transferencias) e a tela do clube
# que lista as transferencias futuras.
# Prologo: push -1 / push 0x963ec6 (7 bytes), sem relativas. Igual no cm0102-1080.exe (a rotina inteira, conferido
# byte a byte).
# Convencoes conferidas na desmontagem:
#   0x8ccb80 thiscall(tm, clube, &saida) ret 8 -> a contagem em eax (os chamadores leem al).
#   Dentro: 0x68d020 / 0x68d040 / 0x68d0d0 (lista, &k, 4) / 0x68d1b0 (LLIST); 0x54a110 Date::Date();
#   0x8c6100 thiscall(tm, clube, &lista) ret 8 (ja Rust, chamada pelo endereco); 0x4cf5a0 thiscall([0xae1050],
#   pessoa); 0x417c10 Date::operator=; 0x945667 cdecl realloc; 0x945333 cdecl qsort(linhas, n, 0x1b, 0x8e55f0);
#   sem memoria: 0x944cff/0x90d130/0x944c9f/0x66f4e0/0x5e8290 (show_transfer_memory_error, linhas 0x2b2c e 0x2b63).
# Leitura de memoria: as ofertas, as transferencias futuras, as pessoas e os contratos. Defeito do original: a linha de
# um jogador da troca nao grava o +0x1a (fica o que a memoria do realloc tinha); a copia tambem nao grava.
# Comparador: lado a lado (a original e a copia so leem e alocam o vetor): a contagem, as linhas (sem o +0x1a nas de
# tipo 1) e o contrato guardado em 0xae1080; o vetor da copia e liberado e o chamador recebe o da original.
TELAS = True
SAVE = False
ROTINAS = [('get_club_future_transfer_list', 0x8ccb80, [0x6a, 0xff, 0x68, 0xc6, 0x3e, 0x96, 0x00])]
DOCS = ('a lista das transferências futuras de um clube, para a tela do clube e o orçamento de transferências',
        'a club\'s future transfer list, for the club screen and the transfer budget',
        [('get_club_future_transfer_list', 'TRANSFER_MANAGER::get_club_future_transfer_list', '0x8ccb80', 'transfer_manager.cpp', 'thiscall, ret 8',
          'monta as linhas (27 bytes) das transferências em andamento de um clube e devolve quantas são: as propostas que o clube fez ainda abertas (fora as recusadas, retiradas, caducas, canceladas e concluídas; uma desfeita só enquanto a rodada de resposta está abaixo de 3) e as propostas pelos jogadores dele; os empréstimos ficam de fora. Cada linha: a data da transferência futura da proposta (ou a data da próxima etapa), a etapa pelo estado (1 em espera; 2 aceita, pré-contrato ou esperando o jogador; 3 esperando o visto; 4 confirmações dos clubes; 5 recusada na aprovação; 6 confirmada ou transferência futura), o jogador, se ele já sabe, o outro clube (o dono do contrato quando o clube compra, -1 sem dono; o comprador quando vende) com a marca de saída, a taxa e o tipo (4 pré-contrato sem taxa no fim do contrato, 1 com jogadores na troca, 0 dinheiro). Cada jogador da troca acrescenta uma linha com a mesma data e etapa, taxa 0, tipo 1 e o sentido trocado. Ordena por etapa e data. Sem memória, a caixa e 0 linhas',
          'builds the rows (27 bytes) of a club\'s transfers in progress and returns how many: the club\'s still open bids (except refused, withdrawn, lapsed, cancelled and completed; a collapsed one only while its response round is below 3) and the bids for its players; loans are left out. Each row: the date of the bid\'s future transfer (or of its next step), the stage by status (1 on hold; 2 accepted, pre-contract or awaiting the player; 3 awaiting the work permit; 4 the clubs\' confirmations; 5 refused at approval; 6 confirmed or a future transfer), the player, whether he knows, the other club (the contract\'s owner when the club buys, -1 without one; the buyer when it sells) with the outgoing mark, the fee and the kind (4 a pre-contract without a fee at the contract end, 1 with exchange players, 0 cash). Each exchange player adds a row with the same date and stage, fee 0, kind 1 and the direction reversed. Sorted by stage and date. Out of memory, the box and 0 rows')])
CAMPOS = [('linha da lista de transferências futuras (0x1b bytes): +0 data, +0xc o outro clube (-1 sem), +0x18 tipo (0 dinheiro, 1 troca, 4 pré-contrato), +0x1a o jogador sabe (offer +0x2d); grava',
           'future transfer list row (0x1b bytes): +0 date, +0xc the other club (-1 none), +0x18 kind (0 cash, 1 exchange, 4 pre-contract), +0x1a the player knows (offer +0x2d); writes')]
INTERESSA = []
