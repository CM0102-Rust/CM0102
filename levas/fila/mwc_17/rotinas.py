# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01: os construtores rodam uma vez,
# na criacao do jogo. thiscall ret 8 (ecx = a copa; ano e registro na pilha). Os seis construtores conferidos entre si
# na desmontagem: so mudam a tabela virtual, o banco, a disciplina, a rotina dos participantes, o teste da falha
# (jge ou jne), as posicoes previstas (a Supercopa holandesa nao tem), o arquivo e as linhas das caixas.
# So o original roda no comparador: a mutacao (a disciplina inglesa) so a carreira pode pegar.
# Depende do ajudante do mwc_16 (construct_nation_cup).
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_bel_super', 0x420ef0, [0x6a, 0xff, 0x68, 0xa9, 0x40, 0x95, 0x00]),
           ('construct_hol_super', 0x5f8820, [0x6a, 0xff, 0x68, 0x39, 0xae, 0x95, 0x00]),
           ('construct_ger_lge_cup', 0x5dc0e0, [0x6a, 0xff, 0x68, 0xd9, 0xa5, 0x95, 0x00])]
DOCS = ('os construtores das Supercopas da Bélgica e da Holanda e da Copa da Liga alemã',
        'the constructors of the Belgian and Dutch Super Cups and the German League Cup',
        [('construct_bel_super', 'BelSuper::BelSuper', '0x420ef0', 'bel_super.cpp', 'thiscall, ret 8', 'regra: a Supercopa da Bélgica nasce com a regra de disciplina 5, o banco (7 reservas, 7 substituições); sem estado gravado, os participantes (BelSuper::select_teams), a tabela de rodadas, a árvore de confrontos (falha quando devolve negativo), as estatísticas e as posições previstas', 'rule: the Belgian Super Cup is born with discipline rule 5, the bench (7 substitutes named, 7 substitutions); with no saved state, the entrants (BelSuper::select_teams), the round table, the tie tree (a failure is a negative return), the statistics and the predicted positions'),
         ('construct_hol_super', 'HolSuper::HolSuper', '0x5f8820', 'hol_super.cpp', 'thiscall, ret 8', 'regra: a Supercopa da Holanda nasce com a regra de disciplina 15; sem estado gravado, os participantes (HolSuper::select_teams), a tabela de rodadas, a árvore de confrontos (falha quando devolve negativo), as estatísticas (sem posições previstas)', 'rule: the Dutch Super Cup is born with discipline rule 15; with no saved state, the entrants (HolSuper::select_teams), the round table, the tie tree (a failure is a negative return), the statistics (no predicted positions)'),
         ('construct_ger_lge_cup', 'GerLgeCup::GerLgeCup', '0x5dc0e0', 'ger_lge_cup.cpp', 'thiscall, ret 8', 'regra: a Copa da Liga alemã nasce com a regra de disciplina 13; sem estado gravado, os participantes (GerLgeCup::collect_entrants), a tabela de rodadas, a árvore de confrontos (falha quando devolve negativo), as estatísticas e as posições previstas', 'rule: the German League Cup is born with discipline rule 13; with no saved state, the entrants (GerLgeCup::collect_entrants), the round table, the tie tree (a failure is a negative return), the statistics and the predicted positions')])
