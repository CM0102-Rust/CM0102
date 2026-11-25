# A escolha da equipe da semana do Brasil (award/brazil_awards.cpp; leitor rx09, rodada 6, 25/11/2026). Como as das
# nacoes do sx04c_23: a virtual update_team_of_the_week (+0x10) so e chamada pela tela dos premios (a 0x769c70 tem 0
# chamadas na cobertura do mundo). Item TELAS. Caminho para o Lucas, jogando comparando com o Brasil carregado: abrir os
# premios do Brasil e escolher a equipe da semana da Primeira, da Segunda e de um estadual.
# - 0x43f070 BrazilAwards::update_team_of_the_week thiscall(premio) ret 4: [ecx+4] a tabela; o premio comparado com as
#   vagas 0, 5, 10, 14, ..., 54 (offsets 0, 0x14, 0x28, 0x38 ... 0xd8) na ordem; com uma delas, 0x769c70
#   thiscall([[premio+9]], premio) ret 8.
# Prologo mov edx, [ecx+4]; mov eax, [esp+4] (7 bytes, sem relativo); igual no cm0102-1080.exe.
# Comparador: a original roda e tem de ter montado a equipe exatamente quando a copia montaria (compare_team_of_the_week_of
# do sx04c_23, o contador do sx04c_24). Depende do sx05b_5 (nation_awards_team_of_the_week_of) e do sx04c_23.
TELAS = True
SAVE = False
ROTINAS = [('brazil_awards_update_team_of_the_week', 0x43f070, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('a escolha da equipe da semana do Brasil', 'the Brazilian team of the week choice',
        [('brazil_awards_update_team_of_the_week', 'BrazilAwards::update_team_of_the_week', '0x43f070', 'award/brazil_awards.cpp', 'thiscall, ret 4',
          'regra: a equipe da semana é montada só para os prêmios de equipe da semana: os da Primeira e da Segunda Divisão e os dos doze estaduais',
          'rule: the team of the week is built only for the team of the week awards: the First and Second Divisions\' and the twelve state championships\'')])
CAMPOS = []
INTERESSA = []
