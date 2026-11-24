# As equipes da semana da Turquia (award/turkey_awards.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa
# nation_awards_team_of_the_week_of (sx05b_5, que usa build_team_of_the_week do sx03_2): integrar depois deles.
# Chamada pela virtual das equipes de premio, toda semana, com a Turquia carregada: LIGAS = 'ENG POL TUR GRE HRV RUS'.
# Convencoes conferidas na desmontagem: thiscall(premio) ret 4; compara com [[+4]+8] e [[+4]+0x10] (premios 2 e 4);
# NationAwards::update_team_of_the_week 0x769c70 thiscall([[premio+9]], premio) ret 8.
# Comparador: so a original (a selecao nao se refaz).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ROTINAS = [('turkey_awards_team_of_the_week', 0x902080, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('as equipes da semana turcas',
        'the Turkish teams of the week',
        [('turkey_awards_team_of_the_week', 'TurkeyAwards::update_team_of_the_week', '0x902080', 'award/turkey_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana de um prêmio turco: só para as equipes da semana da primeira e da segunda divisão (prêmios 2 e 4), pela competição de clubes do prêmio',
          "builds the team of the week of a Turkish award: only for the first and second division teams of the week (awards 2 and 4), by the award's club competition")])
CAMPOS = []
INTERESSA = []
