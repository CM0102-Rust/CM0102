# O calendário real de 2001/02 dos dois grupos da Série C1 italiana e os dois tocos que devolvem 1 (Agente B da frente
# das competições, 04/11/2026). Rodam na carreira de 8 ligas (docs/cobertura_competicoes.txt: 0x64be90 e 0x650100 1
# vez cada, chamadas pelos construtores; 0x5bb540 2124 vezes; 0x5bb550 111 vezes):
# - 0x64be90 (ItaSerC1a::set_up_fixed_fixtures) e 0x650100 (ItaSerC1b::set_up_fixed_fixtures), 11.650 bytes cada (306
#   chamadas a 0x68aff0), quadro SEH só para o construtor de vetor; o corpo é o da cqn_5 (conferido pela desmontagem);
# - 0x5bb540 (stub_return_one_ret8) e 0x5bb550 (stub_return_one_ret4), 8 bytes cada: mov eax, 1; ret 8 / ret 4.
# Iguais no cm0102-1080.exe (inteiras). Os jogos foram extraídos da desmontagem (capstone) e conferidos como na cqn_5.
# Convenções conferidas nos pontos de chamada da original: as da cqn_5 (0x944cf1, 0x9463af, 0x68aff0, 0x9462bb,
# 0x944c94, a caixa); os tocos são virtuais (thiscall, ret 8 / ret 4: o `ret` deles).
# Comparadores: o calendário, só a original (aloca), conferindo a lista; os tocos, a devolução.
# Determinismo: só constantes do exe e os campos da liga.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('calendario_fixo_da_c1a', 0x64be90, [0x6a, 0xff, 0x68, 0x0e, 0xbb, 0x95, 0x00]),
           ('calendario_fixo_da_c1b', 0x650100, [0x6a, 0xff, 0x68, 0xae, 0xbb, 0x95, 0x00]),
           ('toco_um_com_dois', 0x5bb540, [0xb8, 0x01, 0x00, 0x00, 0x00]),
           ('toco_um_com_um', 0x5bb550, [0xb8, 0x01, 0x00, 0x00, 0x00])]
DOCS = ('o calendário real de 2001/02 dos dois grupos da Série C1 italiana; os tocos que devolvem 1',
 'the real 2001/02 fixture list of the two Italian Serie C1 groups; the stubs that return 1',
 [('calendario_fixo_da_c1a', 'ItaSerC1a::set_up_fixed_fixtures', '0x64be90', 'ita_ser_c1a.cpp', 'thiscall, ret 0',
   'a C1/A de 2001/02: os 306 jogos (34 rodadas de 9) fixos pelo nome dos clubes; faltando um nome no banco, a lista '
   'cai e a liga sorteia o calendário',
   'the 2001/02 C1/A: the 306 matches (34 rounds of 9) fixed by club name; with a name missing from the database, '
   'the list is dropped and the league draws its schedule'),
  ('calendario_fixo_da_c1b', 'ItaSerC1b::set_up_fixed_fixtures', '0x650100', 'ita_ser_c1b.cpp', 'thiscall, ret 0',
   'a C1/B de 2001/02, com a mesma regra', 'the 2001/02 C1/B, with the same rule'),
  ('toco_um_com_dois', 'stub_return_one_ret8', '0x5bb540', 'fra_first.cpp', 'thiscall, ret 8',
   'o toco do add_result das competições que não jogam partidas: devolve 1',
   'the add_result stub of competitions that play no matches: returns 1'),
  ('toco_um_com_um', 'stub_return_one_ret4', '0x5bb550', 'fra_first.cpp', 'thiscall, ret 4',
   'o padrão do slot 25 das competições e do slot 6 do órgão disciplinar: devolve 1',
   'the default of competition slot 25 and ruling body slot 6: returns 1')])
CAMPOS = [('liga +0xea a lista dos jogos fixos (16 bytes cada; a conta antes, em −4); +8/+0xc os ids dos clubes, −1 sem '
           'achar; ponteiro e valor; grava (calendario_fixo_da_c1a/c1b)',
           'league +0xea the fixed fixture list (16 bytes each; count before, at −4); +8/+0xc the club ids, −1 if not '
           'found; pointer and value; writes')]
INTERESSA = ['CMRTE', 'SGE']
