# A equipe da semana dos prêmios franceses (award/france_awards.cpp; leitor rx11, rodada 6, 25/11/2026). Simulação com
# a França carregada (a virtual +0x10 dos prêmios, chamada a cada semana com o prêmio): LIGAS com a França e ATE =
# '2002-08-01', como o rx11_7. Usa o nation_awards_team_of_the_week_of do sx05b_5: integrar depois dele.
# - 0x5c1540 FranceAwards::update_team_of_the_week thiscall(premio) ret 4: o prêmio igual a +4[6], +4[11] ou +4[16]:
#   0x769c70(id da competição [[premio+9]], premio). Prólogo mov edx, [ecx+4]; mov eax, [esp+4] (7 bytes).
# Igual no cm0102-1080.exe. Comparador: só a original (a equipe da semana é escolhida uma vez).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2002-08-01'
ROTINAS = [('france_awards_update_team_of_the_week', 0x5c1540, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('a equipe da semana dos prêmios franceses',
        "the French awards' team of the week",
        [('france_awards_update_team_of_the_week', 'FranceAwards::update_team_of_the_week', '0x5c1540', 'award/france_awards.cpp', 'thiscall, ret 4',
          'regra: a equipe da semana é montada para o prêmio da equipe da semana da Primeira Divisão, da Segunda ou do National franceses',
          'rule: the team of the week is built for the team of the week award of the French First Division, Second Division or National')])
CAMPOS = []
INTERESSA = []
