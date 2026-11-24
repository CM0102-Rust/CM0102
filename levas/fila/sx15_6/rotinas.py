# As seleções saindo de uma competição e a remarcação dos jogos de clube que batem com convocações
# (NationalTeams::clear_competition_for_comp_clubs 0x775420 e NationalTeams::reschedule_clashing_club_fixtures
# 0x7777e0, national_teams.cpp, 160 e 464 bytes; leitor sx15, rodada 5, 23/11/2026).
# - 0x775420: chamada no fim de competições de seleções (0x57b748, 0x90fbea, 0x92d328; 1 chamada na carreira ENG até
#   2002-06-01); thiscall(objeto, &id da competição), ret 4; a lista de clubes pelo virtual +0x58 do objeto da
#   competição (etapa 0; contagem como palavra com sinal; a lista liberada por 0x9452ca), reaproveitado de
#   for_each_comp_club; 0x53b900 cdecl(clube); a nação lida antes do teste (+0x53); 0x6a5610 thiscall([0xb67a44],
#   &competição, seleção), ret 8;
# - 0x7777e0: chamada pelo dia das seleções (0x76b579 em daily_update) e por 0x77a070; thiscall(objeto, data), ret 4;
#   SEH só pelas datas locais; 0x54c8e0/0x54c5b0 thiscall(data, saída, n) ret 8; 0x417c10 thiscall(destino, cópia,
#   origem) ret 8; 0x5abb10 thiscall([0xae2a58], clube, data) ret 8; as comparações de data: ano como palavra sem
#   sinal (ja/jb), dia com sinal (setg/setl); a soma dos convocados é byte comparado com sinal (> 2).
# Comparadores: só a original nas duas (a 0x775420 avisa a FA, que sorteia e pode demitir; a 0x7777e0 monta a tabela
# de marcas e remarca jogos), com o plano conferido depois: nenhuma seleção da competição com competição atual; a
# tabela de marcas existe.
ROTINAS = [('clear_competition_for_comp_clubs', 0x775420, [0x83, 0xec, 0x08, 0x8b, 0x44, 0x24, 0x0c]),
           ('reschedule_clashing_club_fixtures', 0x7777e0, [0x6a, 0xff, 0x68, 0x98, 0xdf, 0x95, 0x00])]
DOCS = ('as seleções saindo de uma competição e a remarcação dos jogos de clube que batem com convocações',
        'the national teams leaving a competition and the club fixtures that clash with call-ups',
        [('clear_competition_for_comp_clubs', 'NationalTeams::clear_competition_for_comp_clubs', '0x775420', 'national_teams.cpp', 'thiscall, ret 4',
          'cada seleção da lista de clubes da competição fica sem competição atual (-1 no registro da nação) e, com o gerente de técnicos, a FA avalia o técnico pelo desempenho na competição',
          "each national team of the competition's club list is left without a current competition (-1 in the nation record) and, with the manager manager, the FA reviews the manager's performance in the competition"),
         ('reschedule_clashing_club_fixtures', 'NationalTeams::reschedule_clashing_club_fixtures', '0x7777e0', 'national_teams.cpp', 'thiscall, ret 4',
          'sem a tabela de marcas, marca os clubes com jogo na data, na véspera e no dia seguinte; depois, cada clube marcado (1 ou 2) com mais de dois jogadores convocados em datas da véspera ao dia seguinte tem o jogo perto da data remarcado pelo gerente de jogos',
          'without the club marks, marks the clubs with a fixture on the date, the day before and the day after; then each marked club (1 or 2) with more than two players called up on dates from the day before to the day after has its fixture near the date rescheduled by the fixture manager')])
