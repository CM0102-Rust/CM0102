# A equipe da semana dos prêmios argentinos (award/argentina_awards.cpp; leitor rx13, rodada 6, 25/11/2026). Simulação
# com a Argentina carregada (a virtual +0x10 dos prêmios, chamada a cada semana com o prêmio): LIGAS do grupo da
# Argentina e ATE = '2002-08-01', como o rx13_3. Usa o nation_awards_team_of_the_week_of do sx05b_5 (já no motor).
# - 0x40a560 ArgentinaAwards::update_team_of_the_week thiscall(prêmio) ret 4: o prêmio igual a +4[0] ou +4[1]:
#   0x769c70(id da competição [[prêmio+9]], prêmio) no mesmo ecx. Prólogo mov edx, [ecx+4]; mov eax, [esp+4] (7 bytes).
# Igual no cm0102-1080.exe. Comparador: só a original (a equipe da semana é escolhida uma vez).
SAVE = False
LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'
ATE = '2002-08-01'
ROTINAS = [('argentina_awards_update_team_of_the_week', 0x40a560, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('a equipe da semana dos prêmios argentinos',
        "the Argentine awards' team of the week",
        [('argentina_awards_update_team_of_the_week', 'ArgentinaAwards::update_team_of_the_week', '0x40a560', 'award/argentina_awards.cpp', 'thiscall, ret 4',
          'regra: a cada semana, a equipe da semana da primeira e da segunda divisão argentina, quando o prêmio pedido é um dos dois',
          'rule: each week, the team of the week of the Argentine first and second divisions, when the award asked for is one of the two')])
CAMPOS = []
INTERESSA = []
