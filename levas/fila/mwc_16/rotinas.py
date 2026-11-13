# Leitor mwc (12/11/2026), carreira do mundo inteiro (BEL HOL GER DEN) ate 2002-08-01: os construtores rodam uma vez,
# na criacao do jogo. thiscall ret 8 (ecx = a copa; ano e registro na pilha). Os seis construtores conferidos entre si
# na desmontagem: so mudam a tabela virtual, o banco, a disciplina, a rotina dos participantes, o teste da falha
# (jge ou jne), as posicoes previstas (a Supercopa holandesa nao tem), o arquivo e as linhas das caixas.
# So o original roda no comparador: a mutacao (a disciplina inglesa) so a carreira pode pegar.
# Traz o ajudante comum dos construtores das copas (construct_nation_cup).
# 
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('construct_bel_fa_cup', 0x41d050, [0x6a, 0xff, 0x68, 0x59, 0x3f, 0x95, 0x00]),
           ('construct_den_cup', 0x5509b0, [0x6a, 0xff, 0x68, 0x89, 0x7a, 0x95, 0x00]),
           ('construct_ger_cup', 0x5d8c20, [0x6a, 0xff, 0x68, 0x39, 0xa5, 0x95, 0x00])]
DOCS = ('os construtores das copas da Bélgica, da Dinamarca e da Alemanha',
        'the constructors of the Belgian, Danish and German cups',
        [('construct_bel_fa_cup', 'BelFaCup::BelFaCup', '0x41d050', 'bel_fa_cup.cpp', 'thiscall, ret 8', 'regra: a Copa da Bélgica nasce com a regra de disciplina 5, o banco (5 reservas, 3 substituições); sem estado gravado, os participantes (BelFaCup::collect_entrants), a tabela de rodadas, a árvore de confrontos (falha quando devolve negativo), as estatísticas e as posições previstas', 'rule: the Belgian Cup is born with discipline rule 5, the bench (5 substitutes named, 3 substitutions); with no saved state, the entrants (BelFaCup::collect_entrants), the round table, the tie tree (a failure is a negative return), the statistics and the predicted positions'),
         ('construct_den_cup', 'DenCup::DenCup', '0x5509b0', 'den_cup.cpp', 'thiscall, ret 8', 'regra: a Copa da Dinamarca nasce com a regra de disciplina 8; sem estado gravado, os participantes (DenCup::collect_entrants), a tabela de rodadas, a árvore de confrontos (falha quando devolve 0), as estatísticas e as posições previstas', 'rule: the Danish Cup is born with discipline rule 8; with no saved state, the entrants (DenCup::collect_entrants), the round table, the tie tree (a failure is a 0 return), the statistics and the predicted positions'),
         ('construct_ger_cup', 'GerCup::GerCup', '0x5d8c20', 'ger_cup.cpp', 'thiscall, ret 8', 'regra: a Copa da Alemanha nasce com a regra de disciplina 14, o banco (6 reservas, 3 substituições); sem estado gravado, os participantes (GerCup::collect_entrants), a tabela de rodadas, a árvore de confrontos (falha quando devolve 0), as estatísticas e as posições previstas', 'rule: the German Cup is born with discipline rule 14, the bench (6 substitutes named, 3 substitutions); with no saved state, the entrants (GerCup::collect_entrants), the round table, the tie tree (a failure is a 0 return), the statistics and the predicted positions')])
