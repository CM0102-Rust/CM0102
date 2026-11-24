# Os inscritos e o dia de montagem da Copa da Holanda (hol_cup.cpp; leitor sbF, rodada 5, 24/11/2026). Competicoes da
# Holanda; ATE 2002-08-01 (a copa e montada no primeiro dia completo da temporada de 2001; a de 2002 na virada).
# - 0x5f4820 HolCup::select_entrants thiscall() ret 0 -> 1/0 (1,3 KB): 86 vagas (calloc 4 x 86, caixa 0x32c); os
#   amadores holandeses num pool local de ate 300 (a divisao A_LOWER_DIVISION e a nacao pelo endereco do registro); a
#   Toto (holandeses) e a KPN sem copa europeia (+0x1db = -1); os 50 primeiros repetidos (sem conferir o limite),
#   ordenados pela longitude (qsort 0x945333 com 0x53fe50) e as copias zeradas (indices de 16 bits); os 50 melhores
#   amadores embaralhados (0x4afbd0) nos buracos; 9 potes de 8 e o resto embaralhados (0x4afb40); os da KPN na
#   Intertoto, na UEFA e na Liga dos Campeoes no fim; os buracos que sobrarem com amadores que ainda nao estao.
# - 0x5f4020 HolCup::update_for_date thiscall(hoje, completo) ret 8: uma vez (+0x45 = 0), quando o ano em
#   [G_UEFA_SEEDING]+0x439 (palavra sem sinal) e o da copa e completo != 0: os inscritos, as posicoes previstas, o grupo
#   da copa (0x5f3d70, cx02_24), os jogos, a ordem (caixas 0x224, 0x22a, 0x231) e as 19 fases (0x5f4220, sbF_6);
#   depois o dia da liga (0x6847c0). O molde da SpaCup::update_for_date para a condicao.
# Prologos: sub esp, 0x6b8; sub esp, 0x200 (6 bytes). Iguais no cm0102-1080.exe.
# Comparadores: so a original (aloca, embaralha e monta), com o plano (os profissionais na lista e os europeus na
# ordem; ativo, a lista e as 19 fases no dia de montagem).
# Defeitos do original: os 50 primeiros sao copiados sem conferir as 86 vagas; o pool de amadores e a pilha sem
# iniciar (com menos de 50 amadores, os buracos recebem lixo).
LIGAS = 'ENG BEL HOL GER DEN'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('hol_cup_select_entrants', 0x5f4820, [0x81, 0xec, 0xb8, 0x06, 0x00, 0x00]),
           ('hol_cup_update_for_date', 0x5f4020, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os inscritos e o dia de montagem da Copa da Holanda',
        'the Dutch Cup entrants and its first full day',
        [('hol_cup_select_entrants', 'HolCup::select_entrants', '0x5f4820', 'hol_cup.cpp', 'thiscall, ret 0',
          'regra: a Copa da Holanda tem 86 clubes: os da Eerste Divisie e os da Eredivisie sem copa europeia, 50 amadores holandeses (os melhores, embaralhados) postos nos buracos deixados entre os profissionais ordenados pela longitude da cidade, a lista embaralhada em potes de oito e, no fim (entram depois), os da Eredivisie na Intertoto, na Copa UEFA e na Liga dos Campeões; vaga vazia fica com outro amador holandês',
          'rule: the Dutch Cup takes 86 clubs: the Eerste Divisie clubs and the Eredivisie clubs with no European cup, 50 Dutch amateur clubs (the best ones, shuffled) placed in the gaps left among the professionals sorted by the longitude of their city, the list shuffled in pots of eight and, at the end (they enter later), the Eredivisie clubs in the Intertoto, the UEFA Cup and the Champions League; an empty place goes to another Dutch amateur club'),
         ('hol_cup_update_for_date', 'HolCup::update_for_date', '0x5f4020', 'hol_cup.cpp', 'thiscall, ret 8',
          'regra: a Copa da Holanda é montada no seu primeiro dia completo da temporada (a temporada do jogo igual ao ano da copa): os 86 inscritos, as posições previstas, o grupo da própria copa com os jogos e a ordem e as 19 fases de grupo; depois, todo dia, o dia da liga',
          'rule: the Dutch Cup is set up on its first full day of the season (the game\'s season equal to the cup\'s year): the 86 entrants, the predicted positions, the cup\'s own group with its fixtures and order, and the 19 group stages; then, every day, the league day')])
CAMPOS = []
INTERESSA = []
