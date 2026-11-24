# O "suspenso nesta partida?" de um orgao disciplinar, a limpeza dos amarelos de um clube e a retirada das
# suspensoes pendentes de um jogador (ruling_body.cpp; leitor sx19, rodada 5, 24/11/2026). Usa as pecas do sx19_17
# (ruling_body_entry_index, ruling_body_entry_at, RULING_BODY_ENTRY_SIZE): integrar depois dele.
# Rodam na carreira ENG: is_player_banned em toda escalacao (11.088.735 vezes no mundo ate 2002-08-01, via
# Discipline::is_player_suspended 0x55d270); reset_bookings_for_club nas fases das copas europeias (0x7e9180 chamado
# pela Liga dos Campeoes ao montar as quartas, e pelas regras de transferencia); remove_pending_bans pela disciplina
# (0x55703a).
# Convencoes conferidas na desmontagem:
# - 0x7e85b0 thiscall(pessoa, so_marcadas, extra (palavra), partida) ret 0x10: 0x55cae0 table_penalty(id, +4, +5); -1:
#   0; +0xb nulo: caixa da linha 0x330 e 0; 0x556e90 update_ban_progress thiscall(id, regra, 0) ret 0xc; a suspensao
#   da entrada (+6, -1: 0) em [[0xae28f0]+4] + 20 * indice; so_marcadas e sem o bit 1 das marcas (+0x13): 0. Hoje antes
#   do inicio (+4; ano sem sinal, dia com sinal): sem partida 0; senao a data da partida (0x549ef0 com +0x2a, +0x28), a
#   suspensao comecando depois dela 0; senao o clube do jogador na partida (+0x39 ou +0x24 da pessoa igual ao mandante
#   +0x1c: mandante e a marca +0x40; senao visitante +0x20 e +0x41), 0x5acbe0 find_fixtures thiscall(dia e ano do
#   inicio, dia e ano da partida, id do clube, lista local (0x68d020), -1, -2, marca != 0) ret 0x24, e conta os jogos
#   da lista nao jogados (+0x43 == 0xff) de competicao (+0 >= 0) com a mesma regra disciplinar (+0x50 da tabela
#   0xadadfc) que a da partida; devolve jogos (+0, com sinal) >= contagem. Ja comecada: por dias (+2 != -1): sem partida
#   1; senao 0x54cdb0 days_since(data da partida; inicio) < dias. Por jogos: cumpridos (+1) + extra <= jogos;
# - 0x7e9180 thiscall(clube) ret 4: para cada entrada (contagem +0x13 relida) com id != -1 e (clube nulo ou +0x39 da
#   pessoa == clube): +4 = 0;
# - 0x7e91d0 thiscall(id) ret 4: a lista do orgao (+7; nula: nada), cursor no inicio, 0x68d1b0 get_next e 0x68d1f0
#   remove thiscall(no) ret 4 nos nos com +0x1c == id.
# Comparadores: is_player_banned roda a original e depois a copia (o update_ban_progress da original ja andou; o da
# copia nao muda mais nada) e compara a resposta; a limpeza lado a lado sobre uma copia das entradas; a retirada so a
# original, conferindo depois que nao sobrou no do jogador.
SAVE = False
ROTINAS = [('is_player_banned', 0x7e85b0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('reset_bookings_for_club', 0x7e9180, [0x8b, 0x41, 0x13, 0x57, 0x33, 0xff]),
           ('remove_pending_bans', 0x7e91d0, [0x56, 0x8b, 0xf1, 0x8b, 0x46, 0x07])]
DOCS = ('o "suspenso nesta partida?" de um órgão, a limpeza dos amarelos de um clube e a retirada das suspensões pendentes',
        'a ruling body\'s "banned for this match?", clearing a club\'s bookings and dropping the pending bans',
        [('is_player_banned', 'RulingBody::is_player_banned', '0x7e85b0', 'ruling_body/ruling_body.cpp', 'thiscall, ret 0x10',
          'diz se a suspensão do jogador neste órgão vale para uma partida: primeiro as suspensões dele andam; sem suspensão, não; uma que ainda não começou vale contra uma partida depois do início se os jogos dela cobrem os jogos ainda não disputados do clube, entre o início e a partida, de competições com a mesma regra disciplinar; uma por dias, se a partida cai antes de acabarem os dias; uma por jogos, se os cumpridos mais o extra não passam dos jogos',
          'tells whether the player\'s ban in this body covers a match: first his bans move on; without a ban, no; one not started yet covers a match after its start when its matches cover the club\'s unplayed matches, from the start to the match, of competitions under the same discipline rule set; one by days, when the match falls before the days run out; one by matches, when the matches served plus the extra do not pass its matches'),
         ('reset_bookings_for_club', 'RulingBody::reset_bookings_for_club', '0x7e9180', 'ruling_body/ruling_body.cpp', 'thiscall, ret 4',
          'zera os amarelos de todos os jogadores do clube neste órgão (de todos, com clube nulo)',
          'clears the bookings of every player of the club in this body (of everyone, with a null club)'),
         ('remove_pending_bans', 'RulingBody::remove_pending_bans', '0x7e91d0', 'ruling_body/ruling_body.cpp', 'thiscall, ret 4',
          'tira da lista do órgão todas as suspensões pendentes do jogador',
          'removes every pending ban of the player from the body\'s list')])
CAMPOS = []
INTERESSA = []
