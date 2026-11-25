# O construtor, as regras e a tabela da Segunda Divisao B portuguesa (por_second_b.cpp; leitor rx10, rodada 6,
# 25/11/2026). Simulacao com Portugal e a base NORMAL (a Segunda B e divisao inferior: so e carregada com a base NORMAL;
# sem ela a Liga de Honra troca direto com as zonas, sbE_2), com as ligas e a data dos itens mwe. Rodam na criacao do
# jogo (2001: a tabela junta as tres zonas antigas na Segunda B) e, as regras e a tabela, na virada portuguesa de 2002.
# - 0x7d1920 PorSecondB::PorSecondB thiscall(ano, registro) ret 8: construct_divided_league (rx10_4) com vtable
#   0x96e858, disciplina 0x17, +0x2c = 2 (malloc 8), regras 0x7d2610 (0x47), tabela 0x7d2400 (0x4e), ordem (0x55),
#   estatisticas (0x5d), jogos (0x66), as zonas 0x7d2870(0) e (1) (o indice empurrado com os bytes altos do ponteiro das
#   estatisticas) e as posicoes previstas 0x7d2cd0. Prologo push -1; push 0x95fac9 (7 bytes).
# - 0x7d2610 PorSecondB::set_up_rules thiscall() ret 0: os campos (acima a Liga de Honra 0x9cf610, abaixo -1, +0x52 =
#   2, 7 e 3 substitutos) e a tabela de rodadas pela virtual +0x3c. Prologo push ebx; push esi; mov esi, ecx; mov eax,
#   2 (9 bytes).
# - 0x7d2400 PorSecondB::create_table thiscall() ret 0: +0x3e = 20; em 2001 os clubes das zonas 0x9cf614/618/61c vao
#   para a Segunda B (+0x57 = o proprio registro) com a zona na reserva (+0x60); fill_section_table com a Segunda B
#   0x9cf7c0 e a zona Norte 0x9cf614, sem marcar times B; caixas 0x192 e 0x1a3. Prologo sub esp, 0x200 (6 bytes).
# Depende do rx10_4. Iguais no cm0102-1080.exe. Comparadores: o construtor e a tabela so a original com o plano; as
# regras em ensaio. A tabela fica sem mutacao propria (o fill_section_table e mutado no rx10_4; mudar a reuniao de 2001
# deixa a tabela incompleta e abre caixa).
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
BASE = 'NORMAL'
ROTINAS = [('construct_por_second_b', 0x7d1920, [0x6a, 0xff, 0x68, 0xc9, 0xfa, 0x95, 0x00]),
           ('por_second_b_set_up_rules', 0x7d2610, [0x53, 0x56, 0x8b, 0xf1, 0xb8, 0x02, 0x00, 0x00, 0x00]),
           ('por_second_b_create_table', 0x7d2400, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o construtor, as regras e a tabela da Segunda Divisão B portuguesa',
        'the Portuguese Segunda B constructor, rules and table',
        [('construct_por_second_b', 'PorSecondB::PorSecondB', '0x7d1920', 'por_second_b.cpp', 'thiscall, ret 8',
          'regra: a Segunda B é uma liga do ano do jogo com as regras disciplinares portuguesas, em três zonas (a Norte na tabela própria, a Centro e a Sul como fases); sem estado salvo, monta as regras, a tabela Norte, a ordem, as estatísticas, os jogos, as duas zonas e as posições previstas',
          'rule: the Segunda B is a league of the game\'s year under the Portuguese discipline rules, in three zones (North in its own table, Central and South as stages); with nothing saved, it builds the rules, the North table, the order, the statistics, the fixtures, the two zones and the predicted positions'),
         ('por_second_b_set_up_rules', 'PorSecondB::set_up_rules', '0x7d2610', 'por_second_b.cpp', 'thiscall, ret 0',
          'regra: a Segunda B tem turno e returno, 3 pontos por vitória e 1 por empate; o campeão de cada zona sobe para a Liga de Honra e quatro caem, sem liga abaixo; 7 reservas relacionados e 3 substituições',
          'rule: the Segunda B is played home and away, 3 points a win and 1 a draw; each zone\'s champion goes up to the Liga de Honra and four go down, with no league below; 7 substitutes named and 3 used'),
         ('por_second_b_create_table', 'PorSecondB::create_table', '0x7d2400', 'por_second_b.cpp', 'thiscall, ret 0',
          'regra: a Segunda B Norte tem 20 clubes, os da zona Norte na ordem da base; em 2001 os clubes das três zonas antigas são antes reunidos na Segunda B, com a zona guardada como seção',
          'rule: the Segunda B North has 20 clubs, those of the North zone in database order; in 2001 the clubs of the three old zone divisions are first gathered into the Segunda B, the zone kept as their section')])
CAMPOS = []
INTERESSA = []
