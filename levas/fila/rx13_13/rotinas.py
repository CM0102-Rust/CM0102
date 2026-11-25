# O Campeonato de Clubes da Oceania na criação do jogo (oceania_club_champ.cpp; leitor rx13, rodada 6, 25/11/2026).
# Simulação com as 8 ligas (a Austrália dá a Oceania; o create_oceania_club_comps do setup.rs o constrói por endereço
# quando o continente tem nação escolhida); a edição de 2001 é sorteada e jogada no segundo semestre de 2001.
# - 0x796fd0 OceaniaClubChamp::OceaniaClubChamp thiscall(ano, registro) ret 8 (SEH só do new das estatísticas):
#   League::League 0x682200; vtable 0x96e018, +0x40 = ano (palavra), +4 = registro, +0x42 = 2, +0x1c = +0x20 = -1; ano
#   par (resto com sinal) e < 2001: sobe um a um até o primeiro ímpar (para em 2001); +0x50 = 0x23, +0x51 = 0xf;
#   League::load_from_temp_file 0x687b10(1) != 0: fim. Senão +0x44 = 0xff, +0x30 = -1, +0x2c = 2, +0xc = malloc(8)
#   (sem teste), +0x45 = 0; new 0x5ce e Comp_Stats 0x49ee70(this) em +8 (nulo: caixa 0x51 e fim);
#   OceaniaClubChamp::set_up_rules 0x7979b0 (retorno não lido). Prólogo push -1; push imm32 (7 bytes).
# Igual no cm0102-1080.exe. Comparador: só a original (aloca e pode ler o estado salvo), contra o plano (vtable, edição,
# registro, disciplina; num jogo novo a fase atual -1, as duas fases e o vetor, as estatísticas).
# O destrutor escalar 0x797150 fica no rx13_14 (telas: roda quando o jogo é desmontado).
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('construct_oceania_club_champ', 0x796fd0, [0x6a, 0xff, 0x68, 0xa9, 0xea, 0x95, 0x00])]
DOCS = ('o Campeonato de Clubes da Oceania na criação do jogo',
        'the Oceania Club Championship at career creation',
        [('construct_oceania_club_champ', 'OceaniaClubChamp::OceaniaClubChamp', '0x796fd0', 'oceania_club_champ.cpp', 'thiscall, ret 8',
          'regra: o Campeonato de Clubes da Oceania é uma competição de clubes sobre a base de liga, de dois em dois anos, nos anos ímpares (um ano par antes de 2001 passa ao ímpar seguinte); regra de disciplina 0x23 e importância 0xf; de um jogo salvo lê o estado, senão começa com duas fases além da liga, as estatísticas e as regras',
          'rule: the Oceania Club Championship is a club competition on the league base, played every two years in odd years (an even year before 2001 moves on to the next odd one); discipline rule 0x23 and importance 0xf; from a saved game it loads its state, otherwise it starts with two stages beyond the league, its statistics and its rules')])
CAMPOS = []
INTERESSA = []
