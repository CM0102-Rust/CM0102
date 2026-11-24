# A repescagem e os classificados das eliminatorias da Eurocopa (euro_champ_qual.cpp; leitor cx06, rodada 5,
# 24/11/2026). Simulacao com as 8 ligas e ATE 2004-11-01 (o mesmo dos itens da Copa da Asia do cx05, a mesma leva): os
# grupos acabam em 7/10/2003, a passagem de fase cria a repescagem (sorteio 11/10/2003, jogos de 11/11/2003) e, com a
# competicao completa, os vencedores sao marcados; na cobertura do mundo ate 2002-08-01 as tres tem 0 chamadas.
# - 0x57ec60 EuroChampQual::advance_stage thiscall() ret 0 (virtual 0x28): +0x30 >= +0x2c - 1: caixa 0x38b; senao +0x30 + 1
#   e, na fase +0x100 ou +0x101, create_playoff_stage (direto). Prologo mov edx, [ecx+0x2c]; mov eax, [ecx+0x30] (6
#   bytes).
# - 0x57ef50 EuroChampQual::create_playoff_stage thiscall() ret 0: sem a fase dos segundos (+0x100 = -1) quem nao tem a
#   situacao 7 nos grupos (a copa e [+0xc][0..+0xff]) recebe Staff_Records::add_failed_to_qualify_record 0x86c1d0 (0x41c,
#   0x64) e NationalTeams::clear_current_competition 0x775000, e so. Senao: os melhores segundos (3 com +0xf6 = 1, senao
#   1; as linhas 0.. da fase dos segundos, League::get_table_row 0x6826d0) ganham a situacao 7, o anuncio (virtual 0x40:
#   clube, 7, o byte +0x100, 0, 0), +0xbe 2 e +0xbf 0 no grupo; a repescagem (6 ou 8; malloc, nulo: caixa 0x441) com as
#   ultimas linhas da fase dos segundos; nos grupos, esses ganham a situacao 1 e add_qualified_for_round_record 0x86bdd0
#   (0, 0xa0, 0x1e) e os outros sem a situacao 7 falham; o progresso 1, 1, 1 na fase -1 (virtual 0x68) das primeiras
#   (linhas - repescagem) linhas do ultimo grupo varrido; a tabela de rodadas (virtual 0x3c com +0x101); com a data de
#   inicio do jogo [0xdd6ec8] depois da atual [0xae2c90] (a edicao de 1998) a lista fixa da Euro 2000 (Escocia x
#   Inglaterra, Israel x Dinamarca, Eslovenia x Ucrania, Irlanda x Turquia), o bit 2 no byte +0xe da rodada e os pares
#   mantidos (modo 1), senao sorteados (modo 0); operator new 0xb2 e Cup_Stage 0x522e00 (thiscall, ret 0x3c: a copa, o
#   tamanho, a lista, as rodadas, +4, a tabela, o ano, +0x101, 4, 0, 0x14, o modo, 0, 0, 0) em [+0xc][+0x101]; a lista e a
#   tabela liberadas; nula: caixa 0x4a3; Cup::set_up_round 0x51c800 (push 0) e +0x30 = +0x101. Prologo mov eax, fs:[0]
#   (6 bytes).
# - 0x580040 EuroChampQual::mark_qualified_teams thiscall() ret 0: com +0xfb zero, +0xfb = 1 e, em cada grupo, a linha de
#   cada selecao com +5 = 1 na tabela de times da repescagem (+0xa7, 6 bytes) ganha a situacao 7. Prologo sub esp, 0xc;
#   xor edx, edx (5 bytes).
# Defeitos do original: o progresso 1, 1, 1 vai para as linhas do ultimo grupo varrido (a variavel do grupo foi
# reaproveitada; o grupo tem 5 linhas, menos que a repescagem, entao nada acontece) e nao para os melhores segundos; na
# edicao de 1998 com uma sede (repescagem de 6) a lista fixa escreve 8 selecoes num malloc de 6.
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: a passagem e a repescagem so a original (alocam, sorteiam, registram), com o plano (+0x30; a fase com 6
# ou 8 times e +0x30); os classificados lado a lado em +0xfb e nas tabelas dos grupos.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2004-11-01'
ROTINAS = [('euro_champ_qual_advance_stage', 0x57ec60, [0x8b, 0x51, 0x2c, 0x8b, 0x41, 0x30]),
           ('euro_champ_qual_create_playoff_stage', 0x57ef50, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('euro_champ_qual_mark_qualified_teams', 0x580040, [0x83, 0xec, 0x0c, 0x33, 0xd2])]
DOCS = ('a repescagem e os classificados das eliminatórias da Eurocopa', "the European Championship qualifiers' play-off and qualified nations",
        [('euro_champ_qual_advance_stage', 'EuroChampQual::advance_stage', '0x57ec60', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'passa as eliminatórias à fase seguinte; ao entrar na fase dos segundos colocados ou na repescagem cria a repescagem; depois da última fase, a caixa de erro',
          'moves the qualifiers to the next stage; entering the runners-up stage or the play-off it creates the play-off; past the last stage, the error box'),
         ('euro_champ_qual_create_playoff_stage', 'EuroChampQual::create_playoff_stage', '0x57ef50', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'encerra os grupos das eliminatórias: os melhores segundos colocados (3 com uma sede europeia, 1 com duas) se classificam; os 6 ou 8 últimos da tabela dos segundos vão à repescagem; os outros que não se classificaram são eliminados (o registro dos técnicos e o fim da competição da seleção); a repescagem é sorteada (na edição de 1998, os pares fixos da Euro 2000)',
          'ends the qualifying groups: the best runners-up (3 with one European host, 1 with two) qualify; the last 6 or 8 of the runners-up table go to the play-off; the others that did not qualify are out (the managers\' record and the end of the nation\'s competition); the play-off is drawn (in the 1998 edition, the fixed Euro 2000 pairs)'),
         ('euro_champ_qual_mark_qualified_teams', 'EuroChampQual::mark_qualified_teams', '0x580040', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'uma vez só, marca como classificados nos grupos os vencedores da repescagem',
          'once, marks the play-off winners as qualified in their groups')])
CAMPOS = []
INTERESSA = []
