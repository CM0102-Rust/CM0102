# Os construtores dos premios de uma nacao, das notas do mes e do premio do mes (nation_awards.cpp, month_ratings.cpp,
# month_award.cpp; leitor sx04b, rodada 5, 23/11/2026). Rodam na criacao do jogo (a carreira de teste cria o jogo do
# zero a cada execucao): a base NationAwards no inicio do construtor de cada nacao (Inglaterra 0x5787a3, internacional
# 0x632ade...), as notas do mes logo depois (Inglaterra 0x578816, com arquivo 0) e os premios do mes no init_awards
# (Inglaterra 0x578aef.., 12 chamadas). Com um save carregado, as notas e os premios do mes recebem o arquivo (so a
# original no comparador: a carga aloca; as rotinas de carga sao de outro item).
# Convencoes conferidas na desmontagem:
# - 0x768290: thiscall sem argumentos, ret 0; devolve this (eax = ecx).
# - 0x767af0: thiscall(arquivo), ret 4; devolve this; MONTH_RATINGS::load_from_disk 0x767b20 thiscall(arquivo) ret 4.
# - 0x766e10: thiscall com 12 argumentos, ret 0x30; quadro SEH (estado 0 depois da base); AWARD::AWARD 0x412e40
#   thiscall(a1, a2, a3, a4, a5, a6, a11, a12) ret 0x20 (o a11 e o arquivo, o a12 a marca de jogador); mes a7 (byte,
#   cl), ano a8 (dword, comparado em 16 bits), mes final a9 (byte, comparado com sinal), ano final a10 (word, sem
#   sinal); MONTH_AWARD::load_from_disk 0x766f50 thiscall(arquivo) ret 4.
# Prologos sem chamada relativa; iguais no cm0102-1080.exe (0x768290 com 10 bytes: tres instrucoes).
# Depende do sx04_1 (MONTH_AWARD_SLOT_COUNT, MONTH_AWARD_SLOT_SIZE) e do sx05b_5 (award_construct_base, award_base,
# nation_awards_fields): integrar depois deles.
# Comparadores: lado a lado sobre o objeto (0xe, 0xf e 0xa2 bytes); com arquivo, so a original.
SAVE = False
ROTINAS = [('construct_nation_awards', 0x768290, [0x8b, 0xc1, 0x33, 0xc9, 0xc7, 0x00, 0x10, 0xda, 0x96, 0x00]),
           ('construct_month_ratings', 0x767af0, [0x8b, 0x44, 0x24, 0x04, 0x56]),
           ('construct_month_award', 0x766e10, [0x6a, 0xff, 0x68, 0x88, 0xdb, 0x95, 0x00])]
DOCS = ('os construtores dos prêmios de uma nação, das notas do mês e do prêmio do mês',
        "the constructors of a nation's awards, the month ratings and the month award",
        [('construct_nation_awards', 'NationAwards::NationAwards', '0x768290', 'nation_awards.cpp', 'thiscall, ret 0',
          'regra: a base dos prêmios de toda nação: a vtable base, sem tabela de prêmios, sem prêmios e sem notas do mês',
          "rule: the base of every nation's awards: the base vtable, no award table, no awards and no month ratings"),
         ('construct_month_ratings', 'MONTH_RATINGS::MONTH_RATINGS', '0x767af0', 'month_ratings.cpp', 'thiscall, ret 4',
          'regra: as notas do mês vazias, com o mês -1 e o ano 0 (o primeiro dia não fecha mês); com arquivo, lidas do save',
          'rule: empty month ratings, with month -1 and year 0 (the first day closes no month); with a file, read from the save'),
         ('construct_month_award', 'MONTH_AWARD::MONTH_AWARD', '0x766e10', 'month_award.cpp', 'thiscall, ret 0x30',
          'regra: o prêmio do mês sobre a base AWARD, com uma vaga (sem vencedor nem clube) para cada mês do início ao fim pedidos, no máximo doze (dezembro seguido de janeiro do ano seguinte); com arquivo, as vagas lidas do save',
          'rule: the month award on the AWARD base, with one slot (no winner, no club) for each month from the given start to the given end, at most twelve (December followed by January of the next year); with a file, the slots read from the save')])
CAMPOS = [('prêmio do mês (MONTH_AWARD) +4 tipo (2), +0xa1 vagas em uso; valores; grava (construct_month_award)',
           'month award (MONTH_AWARD) +4 kind (2), +0xa1 slots in use; values; writes (construct_month_award)'),
          ('notas do mês (MONTH_RATINGS) +0 entradas, +4 conta, +8 capacidade; valores; grava (construct_month_ratings)',
           'month ratings (MONTH_RATINGS) +0 entries, +4 count, +8 capacity; values; writes (construct_month_ratings)')]
INTERESSA = []
