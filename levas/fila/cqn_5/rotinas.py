# O calendário real de 2001/02 da Série A e da Série B italianas (Agente B da frente das competições, 04/11/2026).
# Rodam uma vez cada na criação da carreira de 8 ligas, chamadas pelos construtores (docs/cobertura_competicoes.txt:
# 0x642a70 1 vez; 0x647230 1 vez):
# - 0x642a70 (ItaSerA::create_fixed_fixtures, 11.650 bytes: 306 chamadas a 0x68aff0) e 0x647230
#   (ItaSerB::set_up_fixed_fixtures, 14.388 bytes: 380 chamadas), quadro SEH só para o construtor de vetor.
# Iguais no cm0102-1080.exe (inteiras). O corpo das duas é o mesmo; os jogos (dia, mês, ano e os dois nomes, ponteiros
# para o .rdata do exe) foram extraídos da desmontagem com um leitor (capstone) e conferidos: as 306/380 chamadas
# empurram 6 valores constantes (a liga no esi) e gravam nos lugares 0, 16, 32... do vetor, em sequência; não há outra
# chamada no meio além das do vetor e da caixa.
# Convenções conferidas nos pontos de chamada da original:
# - 0x944cf1 (operator new): push n × 16 + 4; add esp, 4 -> cdecl;
# - 0x9463af (eh vector constructor iterator): push 0x5a53c0; push 0x645800; push n; push 0x10; push vetor (sem add
#   esp) -> stdcall, ret 0x14;
# - 0x68aff0 (fixed_fixture_set_by_club_names): push fora; push casa; push ano; push mês; push dia; push esi; mov ecx,
#   [esi + 0xea] (+ 16 × k) -> thiscall, ret 0x18;
# - 0x9462bb (eh vector destructor iterator): push 0x5a53c0; push conta; push 0x10; push vetor -> stdcall, ret 0x10;
# - 0x944c94 (operator delete): push; add esp, 4 -> cdecl;
# - a caixa: aviso_em.
# Comparador: aloca: só a original, conferindo que a lista ficou com a conta e os clubes achados ou foi derrubada.
# Determinismo: só constantes do exe e os campos da liga.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('calendario_fixo_da_serie_a', 0x642a70, [0x6a, 0xff, 0x68, 0xde, 0xb9, 0x95, 0x00]),
           ('calendario_fixo_da_serie_b', 0x647230, [0x6a, 0xff, 0x68, 0x6e, 0xba, 0x95, 0x00])]
DOCS = ('o calendário real de 2001/02 da Série A e da Série B italianas',
 'the real 2001/02 fixture list of the Italian Serie A and Serie B',
 [('calendario_fixo_da_serie_a', 'ItaSerA::create_fixed_fixtures', '0x642a70', 'ita_ser_a.cpp', 'thiscall, ret 0',
   'a Série A de 2001/02 não sorteia o calendário: os 306 jogos (34 rodadas de 9) vêm fixos, com a data e os dois clubes '
   'pelo nome; se algum nome não existe no banco, a lista inteira cai e a liga volta ao calendário sorteado',
   'the 2001/02 Serie A does not draw its fixtures: the 306 matches (34 rounds of 9) are fixed, with date and both '
   'clubs by name; if any name is missing from the database, the whole list is dropped and the league falls back to '
   'the drawn schedule'),
  ('calendario_fixo_da_serie_b', 'ItaSerB::set_up_fixed_fixtures', '0x647230', 'ita_ser_b.cpp', 'thiscall, ret 0',
   'a Série B de 2001/02: os 380 jogos (38 rodadas de 10) fixos, com a mesma regra',
   'the 2001/02 Serie B: the 380 matches (38 rounds of 10) fixed, with the same rule')])
CAMPOS = [('liga +0xea a lista dos jogos fixos (16 bytes cada; a conta antes, em −4); +8/+0xc os ids dos clubes, −1 sem '
           'achar; ponteiro e valor; grava (as duas)',
           'league +0xea the fixed fixture list (16 bytes each; count before, at −4); +8/+0xc the club ids, −1 if not '
           'found; pointer and value; writes'),
          ('liga +0x3e clubes, +0xa9 rodadas; valor; lê (as duas)', 'league +0x3e clubs, +0xa9 rounds; value; reads')]
INTERESSA = ['CMRTE', 'SGE']
