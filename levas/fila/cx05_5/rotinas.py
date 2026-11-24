# A edicao seguinte da Copa Africana, da Copa da Asia e da Eurocopa (african_nations.cpp, asia_nations.cpp,
# euro_champ.cpp; leitor cx05, rodada 5, 24/11/2026). Simulacao com as 8 ligas e ATE 2004-11-01: as tres viradas
# (virtual +8) tem 0 chamadas na cobertura do mundo ate 2002-08-01 (a Copa Africana de 2002, em fevereiro, nao vira
# dentro dessa janela); a Eurocopa de 2004 acaba em 30 de junho de 2004 e a Copa da Asia em 29 de outubro de 2004.
# Duvida para a prova: a data em que o jogo chama a virada de cada uma nao foi determinada (a da Copa America, molde
# destas, roda logo depois da edicao, cqs_12); se nenhuma rodar ate 2004-11-01, deixar o item para a auditoria.
# - 0x401d20 AfricanNations::start_new_season thiscall() ret 0: o molde da Copa America (goldcup_start_new_season) com
#   o ano + 2 e a caixa da linha 0x2a8. Prologo sub esp, 0x200 (6 bytes).
# - 0x40f720 AsiaNations::start_new_season thiscall() ret 0: o mesmo com o ano + 4 e a linha 0x29b. Prologo sub esp,
#   0x200.
# - 0x57b750 EuroChamp::start_new_season thiscall() ret 0: a virada da eliminatoria (a competicao de
#   [0x9cf784] na tabela [0xadadfc], virtual +8, thiscall, ret 0) antes; a tabela, +0x3e = 0, +0x45 = 0, o calendario,
#   o arquivo das estatisticas, as fases, o ano + 4, +0x44 = 0xff, +0x30 = -1, o livro de inscritas solto e refeito
#   (EuroChamp::set_up_team_list 0x57bd80, ja em Rust, thiscall, ret 0), as regras (virtual +0x8c; 0: a caixa da linha
#   0x283 e mais nada) e +0xb1 = 0. Prologo sub esp, 0x200.
# Dentro: League::free_table 0x687970 (push 0 -> thiscall, ret 4), o free 0x9452ca (cdecl), Comp_Stats::
# archive_season_and_reset 0x4a1c50 (push 1; mov ecx, [+8] -> thiscall, ret 4), o destrutor escalar de cada fase
# (virtual 0 com 1), a virtual +0x8c (thiscall, ret 0).
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: so a original (libera e destroi), com o plano: o ano + 2/+4, +0x45 = 0, +0x30 = -1, +0x4c = 0; na
# Eurocopa o livro refeito com 16 vagas, +0x44 = 0xff e +0x3e = 0.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('african_nations_start_new_season', 0x401d20, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('asia_nations_start_new_season', 0x40f720, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('euro_champ_start_new_season', 0x57b750, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a edição seguinte das copas de seleções', 'the next edition of the national-team cups',
        [('african_nations_start_new_season', 'AfricanNations::start_new_season', '0x401d20', 'african_nations.cpp', 'thiscall, ret 0',
          'regra: a Copa Africana de Nações é de dois em dois anos: quando acaba, a tabela e as fases são desfeitas, a temporada vai para o histórico das estatísticas, o ano sobe 2 e as regras são montadas de novo',
          'rule: the African Nations Cup is every two years: when it ends the table and stages are undone, the season goes to the statistics history, the year goes up 2 and the rules are set up again'),
         ('asia_nations_start_new_season', 'AsiaNations::start_new_season', '0x40f720', 'asia_nations.cpp', 'thiscall, ret 0',
          'regra: a Copa da Ásia é de quatro em quatro anos: quando acaba, a tabela e as fases são desfeitas, a temporada vai para o histórico, o ano sobe 4 e as regras são montadas de novo',
          'rule: the Asian Cup is every four years: when it ends the table and stages are undone, the season goes to the history, the year goes up 4 and the rules are set up again'),
         ('euro_champ_start_new_season', 'EuroChamp::start_new_season', '0x57b750', 'euro_champ.cpp', 'thiscall, ret 0',
          'regra: a Eurocopa é de quatro em quatro anos: quando acaba, a eliminatória começa a sua edição seguinte, a tabela e as fases da fase final são desfeitas, a temporada vai para o histórico, o ano sobe 4, a lista de inscritas é refeita (as sedes e o campo padrão) e as regras são montadas de novo',
          'rule: the European Championship is every four years: when it ends the qualifying competition starts its next edition, the finals\' table and stages are undone, the season goes to the history, the year goes up 4, the entrant list is rebuilt (the hosts and the default field) and the rules are set up again')])
CAMPOS = []
INTERESSA = []
