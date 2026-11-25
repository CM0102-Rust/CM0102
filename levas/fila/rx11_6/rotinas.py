# Os prêmios ingleses: o objeto e os seus 26 ou 32 prêmios (award/england_awards.cpp; leitor rx11, rodada 6,
# 25/11/2026). Simulação: rodam na criação do jogo (o AWARD_MANAGER monta os prêmios de cada nação); a carreira de teste
# cria o jogo do zero. Usa o molde comum do sx05b_5: integrar depois dele (INTEGRAR_ANTES.txt).
# - 0x578780 EnglandAwards::EnglandAwards thiscall(arquivo) ret 4: NationAwards 0x768290, vtable 0x969f38,
#   [KEY_NATION_ENGLAND] na tabela das nações (x 0x122), +0x13 = 7, +0x12 = 1; com arquivo, NationAwards::load
#   0x76a280 (lido: fim); senão MONTH_RATINGS 0x767af0(0) em +0xa (new 0xf; nulo: caixa 0x6a e fim) e init_awards
#   0x5788c0(arquivo). Prólogo push -1; push imm32 (7 bytes).
# - 0x5788c0 EnglandAwards::init_awards thiscall(arquivo) ret 4: +8 = 0x20 com o bit 4 de +0x11c da nação (a
#   Conference), senão 0x1a; malloc(4 x n) (nulo: caixa 0x95); os prêmios pelos construtores 0x92fe80 (ano, 0x2b),
#   0x8af520 (equipe, 0x25d) e 0x766e10 (mês, 0xa2), argumentos extraídos por script e conferidos à mão: dois do ano dos
#   jogadores e, por divisão, equipe da semana, técnico, jogador e jovem do mês (agosto a maio), técnico do ano e a
#   seleção do ano. O índice só anda quando o new dá certo (a cópia fixa as vagas). Prólogo push -1; push imm32.
# Iguais no cm0102-1080.exe. Comparadores: só a original (alocam), com o plano do molde: a vtable, o id, o tipo e a
# contagem; cada prêmio da nação na sua vaga. A Conference (6 prêmios) só com a base NORMAL: fica sem prova aqui.
SAVE = False
ROTINAS = [('england_awards_construct', 0x578780, [0x6a, 0xff, 0x68, 0xf9, 0x84, 0x95, 0x00]),
           ('england_awards_init', 0x5788c0, [0x6a, 0xff, 0x68, 0xd0, 0x86, 0x95, 0x00])]
DOCS = ('os prêmios ingleses: o objeto e os seus 26 ou 32 prêmios',
        'the English awards: the object and its 26 or 32 awards',
        [('england_awards_construct', 'EnglandAwards::EnglandAwards', '0x578780', 'award/england_awards.cpp', 'thiscall, ret 4',
          'regra: os prêmios ingleses são da Inglaterra, com as notas do mês; de um jogo salvo são lidos, senão criados',
          'rule: the English awards object is built for England with month ratings; from a saved game it loads them, otherwise it creates its awards'),
         ('england_awards_init', 'EnglandAwards::init_awards', '0x5788c0', 'award/england_awards.cpp', 'thiscall, ret 4',
          'regra: a Inglaterra tem 26 prêmios (32 com a Conference): o jogador e o jovem do ano dos jogadores; depois, em cada divisão, a equipe da semana, o técnico, o jogador e o jovem do mês (de agosto a maio), o técnico do ano e a seleção do ano da divisão',
          "rule: England has 26 awards (32 with the Conference): the players' player and young player of the year; then for each division the team of the week, the manager, player and young player of the month (August to May), the manager of the year and the division's team of the year")])
CAMPOS = []
INTERESSA = []
