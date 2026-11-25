# As eliminatorias europeias da Copa do Mundo: as fases de grupo, a tabela dos segundos colocados e a repescagem
# (wc_europe_league.cpp; leitor rx08, rodada 6, 25/11/2026). Simulacao, LIGAS 'ENG', ATE 2005-12-31 (a campanha de 2004
# para a Copa de 2006, como o rx08_4 e os cx16_7..12): as fases de grupo no dia do sorteio (update_for_date, ja em Rust,
# pelo exe::wceuropeleague_create_group_stage), a tabela dos segundos nos dias de resultado e no fim dos grupos, a
# repescagem pelo advance_stage (cx16_11, pelo endereco).
# - 0x9196d0 WcEuropeLeague::create_group_stage thiscall(grupo, lista, contagem) ret 0xc (quadro SEH so para o new): a
#   virtual +0x3c (grupo, &rodadas, &id da fase, 0) escreve palavras; sem lista: get_group_size 0x918dd0 (grupo), malloc
#   (caixa 0x3a7) e as vagas do grupo + 1 de +0xf7 (seis por grupo); com lista a tabela de rodadas e liberada e vai 0.
#   operator new 0x944cf1 de 0xee e League_Stage::League_Stage 0x68b100 (thiscall, ret 0x58: a secao, a contagem, a lista,
#   2, +4, a tabela, as rodadas, +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, +0x40, o grupo, o id da fase, +0x51, 4, 0, +0xd9,
#   -1, 0, 2; as palavras e bytes com lixo nas metades altas, que o construtor nao le) em [+0xc][grupo]; nulo: caixa 0x3c3
#   e 0; free da lista (sem lista dada) e da tabela; +0x30 = grupo; League::send_group_draw_news 0x684230; 1. Prologo
#   push -1; push 0x9659de (7 bytes).
# - 0x919910 WcEuropeLeague::create_play_off_stage thiscall() ret 0: sem +0x100 ou sem +0x101 (-1): cada linha de cada
#   grupo fora da situacao 7 recebe Staff_Records::add_failed_to_qualify_record 0x86c1d0 (clube, +4, 0x41c, 0x64) e
#   NationalTeams::clear_current_competition 0x775000. Senao N = 8 (isentas 0 ou 1) ou 6; malloc((N+1) x 4) (caixa
#   0x444); os clubes das linhas 0..N da fase dos segundos (League::get_table_row 0x6826d0); utils_random(N+1) sorteia
#   o da Asia e o ultimo toma o lugar dele; nas linhas dos grupos: na lista (0..N-1) ou o da Asia -> situacao 1 e
#   add_qualified_for_round_record 0x86bdd0 (clube, a competicao ou o registro da secao asiatica [0x9cf774] so quando a
#   entrada achada e a da Asia, 0, 0xa0, 0x1e); fora da lista e da situacao 7 -> eliminada. Depois a virtual +0x68
#   (clube, -1, 1, 1, 1) nas primeiras (linhas - N) linhas do ULTIMO grupo processado; a virtual +0x3c da repescagem;
#   operator new de 0xb2 e Cup_Stage::Cup_Stage 0x522e00 (thiscall, ret 0x3c: a secao, N, a lista, as rodadas, +4, a
#   tabela, +0x40, +0x101, 4, o id, 0x14, 0, 0, 0, 0) em [+0xc][+0x101]; free da lista e da tabela; nulo: caixa 0x49f;
#   senao Cup::set_up_round 0x51c800 (0) e +0x30 = +0x101. Prologo mov eax, fs:[0] (6 bytes).
# - 0x91a240 WcEuropeLeague::fill_runners_up_table thiscall() ret 0 (quadro SEH so para o new): as datas da primeira e da
#   ultima rodada (Date::Date 0x549ef0); para cada grupo: League::sort_table 0x6827d0 (0), o segundo (linha 1), os ids dos
#   4 primeiros num malloc(0x10) (caixa 0x57c e retorno, sem a ordenacao final), SubLeague::SubLeague 0x88c6d0 (thiscall,
#   ret 0x30: 4, os ids, -1, -1, &de, &ate, o id da competicao, +0xc2, +0xc3, &+0xc5, 2 x n x (n - 1), +0x10); achado o
#   segundo entre as 4 linhas, a linha (0x3b bytes) copiada na proxima da fase dos segundos com +5 e +9 zerados; a
#   sub-liga destruida pela virtual 0 (1); free; no fim League::sort_table da fase dos segundos. Prologo push -1; push
#   0x965a54 (7 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: as fases de grupo e a repescagem so a original, com o plano (a
# fase no lugar, com as linhas certas, e a fase corrente); a tabela dos segundos lado a lado nas linhas da tabela.
# Defeitos do original [inferencia]: (1) na repescagem o sorteado para a Asia so recebe o registro da secao asiatica
# quando o sorteio cai na ultima posicao da lista: nas outras a entrada achada e a que foi copiada para o lugar dele, e
# ele recebe o registro da propria secao europeia; (2) a virtual +0x68 dos melhores segundos e aplicada nas linhas do
# ultimo grupo processado (o registro esi foi reusado no laco dos grupos), com (linhas do grupo - N) <= 0: nunca roda;
# (3) a tabela dos segundos so anda quando o segundo e achado entre os 4 primeiros da sub-liga (sempre, na pratica).
# Depende do rx08_4 (WC_EUROPE_PLAY_OFF_STAGE) e do cx16_11 (FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE): integrar depois deles.
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_europe_league_create_group_stage', 0x9196d0, [0x6a, 0xff, 0x68, 0xde, 0x59, 0x96, 0x00]),
           ('wc_europe_league_create_play_off_stage', 0x919910, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('wc_europe_league_fill_runners_up_table', 0x91a240, [0x6a, 0xff, 0x68, 0x54, 0x5a, 0x96, 0x00])]
DOCS = ('as eliminatórias europeias da Copa do Mundo: as fases de grupo, a tabela dos segundos colocados e a repescagem',
        'the European World Cup qualifying: the group stages, the runners-up table and the play-off',
        [('wc_europe_league_create_group_stage', 'WcEuropeLeague::create_group_stage', '0x9196d0', 'wc_europe_league.cpp', 'thiscall, ret 0xc',
          'regra: cada grupo das eliminatórias europeias depois do da própria seção é uma fase de liga das seleções sorteadas para ele (ou de uma lista dada), em turno e returno com os pontos, a categoria e os desempates da seção; a notícia do sorteio do grupo sai',
          'rule: each European qualifying group after the section\'s own is a league stage of the nations drawn into it (or of a given list), home and away with the section\'s points, category and tie-breaks; the group draw news goes out'),
         ('wc_europe_league_create_play_off_stage', 'WcEuropeLeague::create_play_off_stage', '0x919910', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'regra: no fim dos grupos europeus os melhores segundos colocados vão à repescagem: um deles, sorteado, joga contra a seção asiática e os outros entre si, em ida e volta; toda seleção não classificada fica fora e encerra a campanha',
          'rule: when the European groups end the best runners-up go to the play-off: one of them, drawn, plays the Asian section and the others play each other, home and away; every nation not through is out and ends its campaign'),
         ('wc_europe_league_fill_runners_up_table', 'WcEuropeLeague::fill_runners_up_table', '0x91a240', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'regra: os segundos colocados europeus são ordenados pela campanha contra os quatro primeiros do próprio grupo: a linha de cada um na tabela dos segundos é a de uma mini-liga dos jogos entre os quatro primeiros do grupo',
          'rule: the European runners-up are ranked by their record against the top four of their own group: each one\'s row in the runners-up table is that of a mini-league of the games among the group\'s top four')])
CAMPOS = []
INTERESSA = []
