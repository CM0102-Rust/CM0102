# As equipes da semana da Grecia e da Croacia (award/greece_awards.cpp, croatia_awards.cpp; leitor rx12, rodada 6,
# 25/11/2026). Usam nation_awards_team_of_the_week_of (sx05b_5), ja publicado.
# Chamadas pela virtual das equipes de premio (a cada rodada da divisao) com as duas nacoes carregadas.
# Convencoes conferidas na desmontagem: thiscall(premio) ret 4; comparam o premio com [[+4]+8], +0x18 (Grecia: 2, 6)
# e [[+4]], +0xc, +0x18 (Croacia: 0, 3, 6); 0x769c70 thiscall([[premio+9]], premio) ret 8.
# Comparadores: so a original (a selecao nao se refaz).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('greece_awards_team_of_the_week', 0x5ece90, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('croatia_awards_team_of_the_week', 0x5184d0, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('as equipes da semana da Grécia e da Croácia',
        'the Greek and Croatian teams of the week',
        [('greece_awards_team_of_the_week', 'GreeceAwards::update_team_of_the_week', '0x5ece90', 'award/greece_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana quando o prêmio é uma das duas equipes da semana gregas',
          'builds the team of the week when the award is one of the two Greek teams of the week'),
         ('croatia_awards_team_of_the_week', 'CroatiaAwards::update_team_of_the_week', '0x5184d0', 'award/croatia_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana quando o prêmio é uma das três equipes da semana croatas',
          'builds the team of the week when the award is one of the three Croatian teams of the week')])
CAMPOS = []
INTERESSA = []
