# As disputas de contrato na Comissao do Estatuto do Jogador (dispute.cpp; leitor sx09b, rodada 5, 23/11/2026):
# - DISPUTE::is_in_dispute (0x569e30, 158 bytes ate o ret 8 de 0x569ecd): thiscall(disputa, pessoa, so a marca) ret 8,
#   devolve a marca em al (eax alto: lixo). Chamada pelo dia do jogo (0x5ce2d0, game.cpp), pela MANAGER_MANAGER
#   (0x696940) e pelas telas da pessoa; 0x56a090 remove_dispute (ja Rust, chamada direta); 0x4cf5a0
#   CONTRACT_MANAGER::get_contract thiscall([0xae1050], pessoa) ret 4, o resultado gravado em [0xae1080];
#   [0xae233c] = as regras da FIFA em vigor (byte);
# - DISPUTE::get_dispute_club_id (0x56a030, 92 bytes ate o ret 4 de 0x56a08b): thiscall(disputa, pessoa) ret 4, -1 sem
#   disputa. Chamada pelo dia do jogo (0x5cf3bd) e pela aba de transferencias da pessoa (0x87f24a);
# - DISPUTE::process_hearings (0x569ed0, 343 bytes ate o ret de 0x56a026): thiscall(disputa), sem argumentos. Chamada
#   pelo dia do jogo (0x5cdafa). 0x456770 Date::operator< thiscall(data da audiencia +8, dword [0xae2c90], dword
#   [0xae2c94]) ret 8; 0x8c16b0 TRANSFER_MANAGER::queue_free_transfer thiscall([0xde1f54], pessoa, 0, -1) ret 0xc;
#   0x547810 staff_is_favoured_by cdecl(pessoa, tecnico); 0x547c10 staff_remove_favoured_staff cdecl(pessoa, tecnico);
#   0x547790 club_favours_staff cdecl(pessoa, clube); 0x547bd0 club_remove_favoured_staff cdecl(clube, pessoa) (sem
#   invólucro: FN_ no item); 0x547af0 staff_add_disliked_staff cdecl(pessoa, tecnico, 1); 0x547940
#   club_add_disliked_staff cdecl(clube, pessoa, 1); 0x56a3b0 dispute_post_news thiscall(disputa, pessoa, clube,
#   resultado, 0, 0) ret 0x14 (pelo endereco); 0x68d1b0/0x68d1f0 LLIST.
# O registro da disputa (17 bytes, na lista em +0): +0 id da pessoa, +4 id do clube, +8 data da audiencia, +0x10 byte
# "o jogador fica no clube". A marca por pessoa: um byte por id na tabela em +4.
# Prologos sem salto: 0x569e30 push esi; mov esi,[esp+8] (5); 0x56a030 push ebx; mov ebx,[esp+8] (5); 0x569ed0 push ecx;
# push ebx; mov ebx,ecx; push ebp (5).
# Defeitos do original (registrados, a copia faz o mesmo): o process_hearings conta k contra a quantidade relida a cada
# passo, entao cada remocao encurta o passeio e a ultima audiencia vencida pode ficar para o dia seguinte; e recusa a
# ultima pessoa da tabela (id < quantidade - 1).
# Comparadores: is_in_dispute e get_dispute_club_id lado a lado (o resultado da busca de contrato e o cursor da lista);
# a disputa que cai (a lista muda) e o process_hearings so a original, com o plano conferido depois.
# Na carreira de ferias as disputas provavelmente nao acontecem (so nascem da file_complaint, pela multa e pela resposta
# a uma historia da imprensa): as tres rodam, mas so pelo caminho sem disputa (marca 0, lista vazia).
SAVE = False
ROTINAS = [('is_in_dispute', 0x569e30, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('get_dispute_club_id', 0x56a030, [0x53, 0x8b, 0x5c, 0x24, 0x08]),
           ('process_hearings', 0x569ed0, [0x51, 0x53, 0x8b, 0xd9, 0x55])]
DOCS = ('as disputas de contrato na Comissao do Estatuto do Jogador',
        "the contract disputes before the Player Status Committee",
        [('is_in_dispute', 'DISPUTE::is_in_dispute', '0x569e30', 'dispute.cpp', 'thiscall, ret 8',
          'devolve a marca de disputa da pessoa; a disputa cai (e devolve 0) quando as regras da FIFA estao desligadas ou, fora da consulta so da marca, quando a pessoa nao tem contrato',
          "returns the person's dispute flag; the dispute is dropped (and 0 returned) when the FIFA rules are off or, outside the flag-only query, when the person has no contract"),
         ('get_dispute_club_id', 'DISPUTE::get_dispute_club_id', '0x56a030', 'dispute.cpp', 'thiscall, ret 4',
          'devolve o clube da disputa da pessoa (procurado na lista desde o comeco), ou -1 sem disputa',
          "returns the club of the person's dispute (searched in the list from the start), or -1 without a dispute"),
         ('process_hearings', 'DISPUTE::process_hearings', '0x569ed0', 'dispute.cpp', 'thiscall',
          'resolve cada disputa com a audiencia ja passada: se o jogador ainda e do clube da disputa, ou fica (noticia com resultado 1) ou sai de graca (transferencia livre; o tecnico e o clube deixam de gostar dele e passam a nao gostar; noticia com resultado 2); depois a marca zera e a disputa sai da lista. O passeio conta contra a quantidade relida, entao cada remocao o encurta',
          'resolves every dispute whose hearing has passed: if the player is still at the dispute\'s club, he either stays (news with outcome 1) or leaves for free (a free transfer; the manager and the club stop favouring him and dislike him; news with outcome 2); then the flag is cleared and the dispute leaves the list. The walk counts against the re-read count, so each removal shortens it')])
