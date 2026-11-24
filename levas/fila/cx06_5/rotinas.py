# A tabela dos segundos colocados das eliminatorias da Eurocopa (euro_champ_qual.cpp; leitor cx06, rodada 5,
# 24/11/2026). Simulacao com as 8 ligas ate 2002-06-01 (docs/cobertura_competicoes.txt: 1 chamada cada, no dia do
# sorteio, 10/03/2002 [inferencia]); nos dias de jogo das eliminatorias (a partir de set/2002) a tabela e refeita pelo
# update_for_date (cx06_4).
# - 0x57f920 EuroChampQual::create_runners_up_stage thiscall() ret 0: os desempates locais 1, 2, 5, 0; +0xbe = 0 enquanto
#   a fase e montada; operator new 0x944cf1 de 0xee e League_Stage 0x68b100 (thiscall, ret 0x58: a copa, 9, sem lista, 0,
#   +4, sem tabela, 0 rodadas, +0xc2, +0xc3, +0xc4, os desempates locais, &+0xbe, o ano, o byte +0x100, 0x3fc, 0, 1, 0, 2,
#   -1, 0, 2) em [+0xc][+0x100]; +0xbe = 1, +0x30 = +0x100 e update_runners_up_table (direto).
# - 0x57fa10 EuroChampQual::update_runners_up_table thiscall() ret 0: o periodo (Date::Date 0x54a110 vazias; na edicao
#   1998 de 1/6/1998 (0x549f70), senao da primeira rodada (0x549ef0); ate uma semana (Date::add_weeks 0x54c770) depois
#   da ultima; Date::operator= 0x417c10); para cada grupo (a propria copa e [+0xc][0..7]): League::sort_table 0x6827d0
#   (push 0), o segundo (League::get_table_row 0x6826d0, linha 1), os ids dos 4 primeiros num malloc(0x10) (nulo: caixa
#   0x55b e volta, sem a ordenacao final), SubLeague::SubLeague 0x88c6d0 (thiscall, ret 0x30: 4, os ids, -1, -1, as datas,
#   o id da competicao, +0xc2, +0xc3, &+0xc5, 9 x n x (n - 1), a lista +0x10) dos jogos entre eles; a linha do segundo na
#   sub-liga e copiada (0x3b bytes) para a linha k + 1 da tabela dos segundos, com +5 e +9 zerados; a sub-liga destruida
#   (virtual 0 com 1), o malloc liberado; no fim a tabela dos segundos ordenada.
# Defeito do original: sem memoria para a sub-liga, o ponteiro nulo e lido como uma liga (get_table_row(0, j)).
# Iguais no cm0102-1080.exe (todas inteiras).
# Comparadores: a fase so a original (aloca), com o plano (9 linhas, +0xbe 1, +0x30); a tabela dos segundos lado a lado
# nas 9 linhas.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('euro_champ_qual_create_runners_up_stage', 0x57f920, [0x6a, 0xff, 0x68, 0x1b, 0x89, 0x95, 0x00]),
           ('euro_champ_qual_update_runners_up_table', 0x57fa10, [0x6a, 0xff, 0x68, 0x80, 0x89, 0x95, 0x00])]
DOCS = ('a tabela dos segundos colocados das eliminatórias da Eurocopa', "the European Championship qualifiers' runners-up table",
        [('euro_champ_qual_create_runners_up_stage', 'EuroChampQual::create_runners_up_stage', '0x57f920', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'cria a fase dos segundos colocados: uma liga de 9 linhas, sem jogos próprios, com os desempates saldo de gols, gols pró e gols fora, e a preenche',
          'creates the runners-up stage: a 9-row league with no games of its own, with the tie-breaks goal difference, goals for and away goals, and fills it'),
         ('euro_champ_qual_update_runners_up_table', 'EuroChampQual::update_runners_up_table', '0x57fa10', 'euro_champ_qual.cpp', 'thiscall, ret 0',
          'refaz a tabela dos segundos colocados: o segundo de cada grupo entra com a campanha só contra os 4 primeiros do seu grupo (uma sub-liga dos jogos entre eles, do início das eliminatórias até uma semana depois da última rodada), e a tabela é ordenada',
          'rebuilds the runners-up table: each group\'s runner-up enters with its record only against the top four of its group (a sub-league of their games, from the start of the qualifiers to a week after the last round), and the table is sorted')])
CAMPOS = []
INTERESSA = []
