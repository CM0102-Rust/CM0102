# A escolha da equipe da semana das nacoes do sx04c: Italia, Japao, Irlanda do Norte, Noruega, Finlandia, Franca e
# Irlanda antigas, Polonia, Portugal, Russia, Escocia e Espanha (os *_awards.cpp; leitor sx04c, rodada 5, 23/11/2026).
# Usa nation_award, awards_lower_divisions e build_team_of_the_week (sx03_2), nation_awards_team_of_the_week_of (sx05b_5)
# e TEAM_OF_THE_WEEK_BUILDS (sx04c_24): integrar depois deles (na mesma leva ou depois do sx04c_24).
# RODAM SO PELAS TELAS: a virtual update_team_of_the_week (+0x10) so e chamada pela tela dos premios (a 0x769c70 que
# elas chamam tem 0 chamadas na cobertura do mundo). Item TELAS (os das outras nacoes, sx05b_13 e sx05b_19, ficaram na
# simulacao). Caminho para o Lucas, jogando comparando com a nacao carregada: abrir os premios da nacao e escolher a
# equipe da semana de cada divisao; as classes antigas (Finlandia, Franca, Irlanda) so com uma base sem as chaves novas.
# Convencoes conferidas na desmontagem: thiscall(premio) ret 4; [ecx+4] a tabela, o premio comparado com as vagas na
# ordem; a Italia so le o bit 4 de nacao+0x11c depois das quatro primeiras; com uma delas, 0x769c70 thiscall([[premio
# +9]], premio) ret 8 (o ecx e o this). Prologos de 7 bytes sem chamada relativa; iguais no cm0102-1080.exe.
# Comparadores: a original roda e tem de ter montado a equipe (o contador TEAM_OF_THE_WEEK_BUILDS do comparador da
# 0x769c70) exatamente quando a copia montaria.
TELAS = True
SAVE = False
ROTINAS = [('italy_awards_team_of_the_week', 0x661330, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('japan_awards_team_of_the_week', 0x666bb0, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('northern_ireland_awards_team_of_the_week', 0x7936b0, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('norway_awards_team_of_the_week', 0x7944d0, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('old_finland_awards_team_of_the_week', 0x79dcd0, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('old_france_awards_team_of_the_week', 0x79e560, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('old_ireland_awards_team_of_the_week', 0x79f510, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('poland_awards_team_of_the_week', 0x7cd810, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('portugal_awards_team_of_the_week', 0x7d3d60, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('russia_awards_team_of_the_week', 0x7ec6a0, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('scotland_awards_team_of_the_week', 0x7f6360, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('spain_awards_team_of_the_week', 0x8593e0, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('a escolha da equipe da semana das nações',
        "the nations' team of the week choice",
        [('italy_awards_team_of_the_week', 'ItalyAwards::update_team_of_the_week', '0x661330', 'award/italy_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana quando o prêmio é a equipe da semana da Série A, B, C1 A ou C1 B (e dos grupos da C2, com as divisões inferiores)',
          'builds the team of the week when the award is the team of the week of Serie A, B, C1 A or C1 B (and the C2 groups, with the lower divisions)'),
         ('japan_awards_team_of_the_week', 'JapanAwards::update_team_of_the_week', '0x666bb0', 'award/japan_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana (Japão) quando o prêmio é uma das equipes da semana: J1 e J2 (prêmios 4 e 6)',
          'builds the team of the week when the award is one of the J1 and J2 League teams of the week (awards 4 and 6)'),
         ('northern_ireland_awards_team_of_the_week', 'NorthernIrelandAwards::update_team_of_the_week', '0x7936b0', 'award/northern_ireland_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana (Irlanda do Norte) quando o prêmio é uma das equipes da semana: Premier e First Division (prêmios 1 e 7)',
          'builds the team of the week when the award is one of the Premier and First Division teams of the week (awards 1 and 7)'),
         ('norway_awards_team_of_the_week', 'NorwayAwards::update_team_of_the_week', '0x7944d0', 'award/norway_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana (Noruega) quando o prêmio é uma das equipes da semana: Premier e First Division (prêmios 5 e 8)',
          'builds the team of the week when the award is one of the Premier and First Division teams of the week (awards 5 and 8)'),
         ('old_finland_awards_team_of_the_week', 'OldFinlandAwards::update_team_of_the_week', '0x79dcd0', 'award/old_finland_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana (Finlândia, prêmios antigos) quando o prêmio é uma das equipes da semana: Premier e First Division (prêmios 7 e 10)',
          'builds the team of the week when the award is one of the Premier and First Division teams of the week (awards 7 and 10)'),
         ('old_france_awards_team_of_the_week', 'OldFranceAwards::update_team_of_the_week', '0x79e560', 'award/old_france_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana (França, prêmios antigos) quando o prêmio é uma das equipes da semana: Primeira e Segunda Divisão (prêmios 6 e 10)',
          'builds the team of the week when the award is one of the First and Second Division teams of the week (awards 6 and 10)'),
         ('old_ireland_awards_team_of_the_week', 'OldIrelandAwards::update_team_of_the_week', '0x79f510', 'award/old_ireland_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana (Irlanda, prêmios antigos) quando o prêmio é uma das equipes da semana: Premier e First Division (prêmios 3 e 10)',
          'builds the team of the week when the award is one of the Premier and First Division teams of the week (awards 3 and 10)'),
         ('poland_awards_team_of_the_week', 'PolandAwards::update_team_of_the_week', '0x7cd810', 'award/poland_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana (Polônia) quando o prêmio é uma das equipes da semana: Primeira e Segunda Divisão (prêmios 0 e 5)',
          'builds the team of the week when the award is one of the First and Second Division teams of the week (awards 0 and 5)'),
         ('portugal_awards_team_of_the_week', 'PortugalAwards::update_team_of_the_week', '0x7d3d60', 'award/portugal_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana (Portugal) quando o prêmio é uma das equipes da semana: Primeira e Segunda Divisão (prêmios 0 e 6)',
          'builds the team of the week when the award is one of the Premier and Second Division teams of the week (awards 0 and 6)'),
         ('russia_awards_team_of_the_week', 'RussiaAwards::update_team_of_the_week', '0x7ec6a0', 'award/russia_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana (Rússia) quando o prêmio é uma das equipes da semana: Premier e First Division (prêmios 1 e 4)',
          'builds the team of the week when the award is one of the Premier and First Division teams of the week (awards 1 and 4)'),
         ('scotland_awards_team_of_the_week', 'ScotlandAwards::update_team_of_the_week', '0x7f6360', 'award/scotland_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana (Escócia) quando o prêmio é uma das equipes da semana: quatro divisões (prêmios 3, 9, 15 e 21)',
          "builds the team of the week when the award is one of the four divisions' teams of the week (awards 3, 9, 15 and 21)"),
         ('spain_awards_team_of_the_week', 'SpainAwards::update_team_of_the_week', '0x8593e0', 'award/spain_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana (Espanha) quando o prêmio é uma das equipes da semana: Primeira e Segunda Divisão (prêmios 2 e 5)',
          'builds the team of the week when the award is one of the First and Second Division teams of the week (awards 2 and 5)')])
CAMPOS = []
INTERESSA = []
