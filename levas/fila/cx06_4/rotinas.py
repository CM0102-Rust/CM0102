# As tabelas de rodadas, as fases de grupo e o dia das eliminatorias da Eurocopa (euro_champ_qual.cpp; leitor cx06,
# rodada 5, 24/11/2026). Simulacao com as 8 ligas ate 2002-06-01 (docs/cobertura_competicoes.txt: create_schedule 9
# chamadas, create_group_stage 8 e update_for_date 2064, todas a partir do sorteio de 10/03/2002 [inferencia: a data do
# set_up_rules ja em Rust]; antes dele o dia devolve 0 sem rodar o dia da liga).
# - 0x57d190 EuroChampQual::create_schedule thiscall(fase, &rodadas, &id, &liga) ret 0x10 (virtual 0x3c): ate a ultima
#   fase de grupo (+0xff, com sinal) a tabela dos grupos (liga 1 se o ponteiro existir, 10 rodadas, id 0x3e9 + fase sem
#   conferir o ponteiro, malloc 0x28a, caixa 0xbb): 10 datas (league_round_set_date 0x68a160, cdecl) de 2/9 a 7/10 do ano
#   seguinte, cada uma com o jogo 0 vazio (0x68a1c0, cdecl); a fase +0x100 ou outra que nao seja +0x101: 0 sem escrever
#   nada; a repescagem (+0x101): liga 0, 1 rodada, id 0, malloc 0x68 (caixa 0xf5), sorteio 11/10 (0x521e60), jogo 11/11
#   (0x521eb0) e os campos da rodada; formato 1 (+0xf6) 6 selecoes, 2 oito, outro a caixa 0x11b (a tabela ainda volta).
#   A maquina das tabelas de rodadas da Copa America (build_phase_table, compare_phase_table) com o plano por fase.
# - 0x57ed10 EuroChampQual::create_group_stage thiscall(g, lista, n) ret 0xc: a tabela de rodadas da fase g (virtual
#   0x3c); sem lista, o tamanho do grupo (get_group_team_count 0x57d9d0, ja em Rust) e os sorteados de 6 x (g + 1) em
#   diante (malloc; nulo: caixa 0x3b4 e 0); com lista, a tabela de rodadas e liberada e a fase fica sem ela;
#   League_Stage 0x68b100 (thiscall, ret 0x58, 22 argumentos: a copa, n, a lista, 2, +4, a tabela, as rodadas, +0xc2,
#   +0xc3, +0xc4, &+0xc5, &+0xbe, o ano, g, o id, +0x51, 4, 0, +0xd9, -1, 0, 2) em [+0xc][g]; nula: caixa 0x3d0 e 0 (sem
#   liberar nada); a lista propria e a tabela liberadas, +0x30 = g, League::send_group_draw_news 0x684230 e 1.
# - 0x57fd60 EuroChampQual::update_for_date thiscall(hoje, completo) ret 8 (virtual 0x14): antes da data do sorteio
#   (Date 0x549ef0 de +0xb6, +0xb8 + ano) devolve 0; no dia, com +0x45 zero e completo: os inscritos (0x57f600, cx06_3),
#   o sorteio (0x57dab0) e a tabela do grupo 1 (0x57e7e0, ja em Rust; 0: caixa 0x5a8 e 0), os jogos
#   (League::create_fixtures 0x6835c0; 0: caixa 0x5af), a tabela ordenada (0x6827d0; 0: caixa 0x5b6), as fases dos grupos
#   2 a 9 (create_group_stage(g, 0, 6)), free(+0xf7) e zera, a fase dos segundos (0x57f920, cx06_5), a virtual 0x5c, free
#   (+0xee) e zera +0xee/+0xf2, +0x45 = 1; nos outros dias, com jogo da competicao na data
#   (fix_man_comp_has_fixture_on_date 0x5ae6d0, ja em Rust, sobre uma copia de hoje) a tabela dos segundos (0x57fa10,
#   cx06_5); depois League::update_for_date 0x6847c0 e o resultado dele.
# Iguais no cm0102-1080.exe (todas inteiras). As rotinas de outros itens sao chamadas pelo endereco (os desvios as
# levam ao Rust ou a original).
# Comparadores: as tabelas de rodadas lado a lado (compare_phase_table); a fase de grupo so a original (aloca e manda a
# noticia), com o plano (os times da fase, como conjunto, e +0x30); o dia so a original (sorteia e aloca), com o plano
# depois do sorteio.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('euro_champ_qual_create_schedule', 0x57d190, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('euro_champ_qual_create_group_stage', 0x57ed10, [0x6a, 0xff, 0x68, 0xce, 0x88, 0x95, 0x00]),
           ('euro_champ_qual_update_for_date', 0x57fd60, [0x6a, 0xff, 0x68, 0xa6, 0x89, 0x95, 0x00])]
DOCS = ('as tabelas de rodadas, as fases de grupo e o dia das eliminatórias da Eurocopa',
        "the European Championship qualifiers' round tables, group stages and day",
        [('euro_champ_qual_create_schedule', 'EuroChampQual::create_schedule', '0x57d190', 'euro_champ_qual.cpp', 'thiscall, ret 0x10',
          'as datas das eliminatórias: cada grupo tem 10 rodadas, em 2 e 6 de setembro, 7 e 11 de outubro do ano anterior à fase final, 24 e 28 de março, 6 de junho, 2 e 6 de setembro e 7 de outubro do ano seguinte; a repescagem é um confronto de ida e volta com sorteio em 11 de outubro e jogos a partir de 11 de novembro, com 6 seleções (uma sede europeia) ou 8 (duas)',
          'the qualifiers\' dates: each group has 10 matchdays, on 2 and 6 September, 7 and 11 October of the year before the finals, 24 and 28 March, 6 June, 2 and 6 September and 7 October of the next year; the play-off is a two-legged tie drawn on 11 October and played from 11 November, with 6 nations (one European host) or 8 (two)'),
         ('euro_champ_qual_create_group_stage', 'EuroChampQual::create_group_stage', '0x57ed10', 'euro_champ_qual.cpp', 'thiscall, ret 0xc',
          'cria a fase de um grupo das eliminatórias (os grupos 2 a 9; o grupo 1 é a própria competição): os sorteados do grupo, ou a lista dada (sem datas de rodada), numa liga de grupo com as regras das eliminatórias; e manda a notícia do sorteio do grupo',
          'creates the stage of a qualifying group (groups 2 to 9; group 1 is the competition itself): the group\'s drawn nations, or the given list (with no round dates), in a group league with the qualifiers\' rules; and sends the group draw news'),
         ('euro_champ_qual_update_for_date', 'EuroChampQual::update_for_date', '0x57fd60', 'euro_champ_qual.cpp', 'thiscall, ret 8',
          'o dia das eliminatórias: nada antes da data do sorteio (10 de março do ano da edição anterior); no dia do sorteio os inscritos, o sorteio dos 9 grupos, os jogos do grupo 1, as fases dos grupos 2 a 9, a fase dos segundos colocados e as posições previstas; nos dias com jogo das eliminatórias a tabela dos segundos colocados é refeita; depois o dia da liga',
          'the qualifiers\' day: nothing before the draw date (10 March of the year before the finals); on the draw date the entrants, the 9 groups draw, group 1\'s fixtures, the stages of groups 2 to 9, the runners-up stage and the predicted positions; on the qualifiers\' match days the runners-up table is rebuilt; then the league day')])
CAMPOS = []
INTERESSA = []
