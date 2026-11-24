# O calendario real de 2001/02 da Serie C2/B italiana (ita_ser_c2b.cpp; leitor cx11, rodada 5, 24/11/2026). Simulacao com a
# Italia e as divisoes inferiores: roda uma vez na criacao do jogo (o construtor do cx11_2). Como no cx11_2, a C2 so
# existe com a base NORMAL ou MAXIMA (BASE abaixo): a prova pede DATABASE=NORMAL na carreira.
# - 0x658750 ItaSerC2b::load_fixed_fixtures thiscall() ret 0 (11664 bytes, uma rotina grande sozinha): o corpo do
#   ItaSerC1a::set_up_fixed_fixtures (conferido: as mesmas instrucoes fora das 306 chamadas, as mesmas chamadas, o
#   mesmo numero de cada mnemonico; so a tabela, o fonte e a linha da caixa 0x3d3 mudam): operator new 0x944cf1 (n x 16 +
#   4, cdecl), eh vector constructor iterator 0x9463af (stdcall: 0x645800, 0x5a53c0), as 306 chamadas de
#   fixed_fixture_set_by_club_names 0x68aff0 (push fora; push casa; push ano; push mes; push dia; push liga; ecx =
#   [+0xea] + 16k -> thiscall, ret 0x18) e, com um clube nao achado, eh vector destructor iterator 0x9462bb, operator
#   delete 0x944c94 e +0xea = 0: set_up_fixed_fixtures_body com SERIE_C2B_FIXTURES. A tabela foi extraida da
#   desmontagem por um script (as posicoes do vetor em passos de 16, 17 jogos em casa por clube) que, rodado na C1/A,
#   devolve o C1A_FIXTURES ja publicado igual. Prologo push -1; push imm32 (7 bytes).
# Iguais no cm0102-1080.exe (inteira). Comparador: so a original (aloca), a lista solta ou com todos os clubes achados
# (fra_first_compare_fixed_fixtures).
SAVE = False
LIGAS = 'ENG ITA'
ATE = '2002-09-30'
BASE = 'NORMAL'
ROTINAS = [('ita_ser_c2b_set_up_fixed_fixtures', 0x658750, [0x6a, 0xff, 0x68, 0xee, 0xbc, 0x95, 0x00])]
DOCS = ('o calendário real de 2001/02 da Série C2/B italiana', 'the real 2001/02 Italian Serie C2/B calendar',
        [('ita_ser_c2b_set_up_fixed_fixtures', 'ItaSerC2b::load_fixed_fixtures', '0x658750', 'ita_ser_c2b.cpp', 'thiscall, ret 0',
          'a Série C2/B de 2001/02: os 306 jogos (34 rodadas de 9) fixados pelos nomes dos clubes; com um nome que falte no banco, a lista cai e a liga sorteia o calendário',
          'the 2001/02 Serie C2/B: the 306 matches (34 rounds of 9) fixed by club name; with a name missing from the database, the list is dropped and the league draws its schedule')])
CAMPOS = []
INTERESSA = []
