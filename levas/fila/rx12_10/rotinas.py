# As equipes da semana da Belgica, da Holanda e da Dinamarca (award/belgium_awards.cpp, holland_awards.cpp,
# denmark_awards.cpp; leitor rx12, rodada 6, 25/11/2026). Usam nation_awards_team_of_the_week_of (sx05b_5), ja
# publicado.
# Chamadas pela virtual das equipes de premio (a cada rodada da divisao) com as tres nacoes carregadas.
# Convencoes conferidas na desmontagem: thiscall(premio) ret 4; comparam o premio com [[+4]+8], +0x18, +0x24 (Belgica:
# 2, 6, 9), +0xc, +0x1c (Holanda: 3, 7) e +4, +0x10, +0x1c (Dinamarca: 1, 4, 7); 0x769c70 thiscall([[premio+9]],
# premio) ret 8.
# Comparadores: so a original (a selecao nao se refaz).
SAVE = False
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
ROTINAS = [('belgium_awards_team_of_the_week', 0x424b60, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('holland_awards_team_of_the_week', 0x5f9670, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04]),
           ('denmark_awards_team_of_the_week', 0x555430, [0x8b, 0x51, 0x04, 0x8b, 0x44, 0x24, 0x04])]
DOCS = ('as equipes da semana da Bélgica, da Holanda e da Dinamarca',
        'the Belgian, Dutch and Danish teams of the week',
        [('belgium_awards_team_of_the_week', 'BelgiumAwards::update_team_of_the_week', '0x424b60', 'award/belgium_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana quando o prêmio é uma das três equipes da semana belgas (primeira, segunda e terceira divisões)',
          'builds the team of the week when the award is one of the three Belgian teams of the week (First, Second and Third Divisions)'),
         ('holland_awards_team_of_the_week', 'HollandAwards::update_team_of_the_week', '0x5f9670', 'award/holland_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana quando o prêmio é uma das duas equipes da semana holandesas',
          'builds the team of the week when the award is one of the two Dutch teams of the week'),
         ('denmark_awards_team_of_the_week', 'DenmarkAwards::update_team_of_the_week', '0x555430', 'award/denmark_awards.cpp', 'thiscall, ret 4',
          'monta a equipe da semana quando o prêmio é uma das três equipes da semana dinamarquesas',
          'builds the team of the week when the award is one of the three Danish teams of the week')])
CAMPOS = []
INTERESSA = []
