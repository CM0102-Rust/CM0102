# As listas do historico dos premios e a formacao do desenho da equipe do premio (award/team_award.cpp,
# year_award.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa team_award_obj, year_award_obj (sx05b_5) e TeamAward
# (sx05_6): integrar depois deles.
# So rodam nas telas dos premios (award_screens.cpp: 0x415ff7 e 0x4156e3 pedem o historico, 0x416693 a formacao do
# desenho da equipe). TELAS = True. Caminho para o Lucas provar (jogando comparando): Nacao -> Premios (ou Competicao
# -> Premios) -> um premio do ano (o historico dos vencedores) e uma equipe da semana/temporada (o historico das
# equipes e o desenho da equipe no campo).
# Convencoes conferidas na desmontagem:
# - 0x8b00a0 thiscall(this) -> eax: AWARD_MANAGER 0x414320 thiscall([0xad9c60], [+0x254], [+0x258]) ret 8 (a lista
#   nova do FILE_LLIST 0x58f120); preenchidas (+0x252) >= 11 + reservas (+0x253), bytes com sinal: sem lista, operator
#   new 0x944cf1(0xe) e LLIST::LLIST 0x68d020; LLIST::add_tail 0x68d0d0 thiscall(+0x4e, 0x205) ret 8;
# - 0x9304d0 thiscall(this) -> eax: 0x4142e0 thiscall([0xad9c60], [+0x23], [+0x27]) ret 8;
# - 0x8b0730 thiscall(formacao) ret 4: onze Formation::set_slot_position 0x5b2ba0 thiscall(vaga, codigo, 0) ret 0xc
#   pelo modo +0x25c (0, 2, 1 e o resto, nessa ordem de testes).
# Comparadores: as listas lado a lado pelo conteudo (cada lado monta a sua; a da copia e liberada); a formacao lado a
# lado.
TELAS = True
SAVE = False
ROTINAS = [('team_award_get_history_list', 0x8b00a0, [0x6a, 0xff, 0x68, 0xeb, 0x37, 0x96, 0x00]),
           ('year_award_get_history_list', 0x9304d0, [0x8b, 0x41, 0x27, 0x8b, 0x49, 0x23]),
           ('team_award_apply_formation', 0x8b0730, [0x8a, 0x81, 0x5c, 0x02, 0x00, 0x00])]
DOCS = ('as listas do histórico dos prêmios e a formação da equipe do prêmio',
        'the award history lists and the team award formation',
        [('team_award_get_history_list', 'TEAM_AWARD::get_history_list', '0x8b00a0', 'award/team_award.cpp', 'thiscall',
          'as equipes passadas do prêmio: os registros do histórico entre os deslocamentos do prêmio e, com a seleção completa, a equipe atual acrescentada (numa lista nova se não havia); devolve a lista',
          "the award's past teams: the history records between the award's offsets and, with the selection complete, the current team appended (to a new list when there was none); returns the list"),
         ('year_award_get_history_list', 'YEAR_AWARD::get_history_list', '0x9304d0', 'award/year_award.cpp', 'thiscall',
          'os vencedores passados do prêmio do ano: os registros do histórico entre os deslocamentos do prêmio',
          "the year award's past winners: the history records between the award's offsets"),
         ('team_award_apply_formation', 'TEAM_AWARD::apply_formation', '0x8b0730', 'award/team_award.cpp', 'thiscall, ret 4',
          'põe nas onze vagas da formação os códigos de posição do modo de formação do prêmio (0, 1 e 2 os seus, os outros o quarto), para o desenho da equipe na tela',
          "sets the formation's eleven slots to the position codes of the award's formation mode (0, 1 and 2 their own, any other the fourth), for the team drawing on screen")])
CAMPOS = []
INTERESSA = []
