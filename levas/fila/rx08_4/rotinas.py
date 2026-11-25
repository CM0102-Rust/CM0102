# As eliminatorias europeias da Copa do Mundo: os registros de carreira do destino de uma selecao, o texto da noticia e
# o sorteio dos grupos (wc_europe_league.cpp; leitor rx08, rodada 6, 25/11/2026). Simulacao: na carreira de teste a
# secao europeia e a campanha de 2004 para a Copa de 2006 (o construtor, ja em Rust, leva o ano ao ciclo, como a
# sul-americana); o sorteio roda no dia do sorteio dela (update_for_date, ja em Rust, chama a 0x918ec0 pelo endereco), os
# registros e o texto no fim dos grupos e da repescagem (virtuais +0x44 e +0x48). LIGAS = 'ENG' e ATE = '2005-12-31',
# como os cx16_7..12 e os cx17_2/3 (pedido da rodada 6: as eliminatorias europeias de 2006 levam o ATE certo). A
# cobertura do mundo ate 2002-08-01 nao as viu (0 chamadas).
# - 0x918980 WcEuropeLeague::update_staff_records_for_status thiscall(clube, desfecho, fase, saida, jogo, a6) ret 0x18:
#   repescagem +0x101 = -1 ou fase < repescagem (bytes com sinal): 7 -> Staff_Records::add_qualified_for_round_record
#   0x86bdd0 (thiscall em [0xdd7f00]; clube, o registro da Copa [0xae23d0] + 0x6b x [0x9cf79c], 0, 0, 0x64),
#   WorldCup::add_qualified_team 0x92e740 (ecx = [0xadadfc + 4 x [0x9cf79c]]), com saida *saida = 4,
#   NationalTeams::add_qualified_nation 0x7779b0 (push registro; push clube; mov ecx, [0xb74318]; ret 8), -1; 1 e 0xfe ->
#   -1 sem registro; outro 0. Fase = repescagem: 1 -> o mesmo com 0x32 e sem *saida, -1; 2 ->
#   Staff_Records::add_knocked_out_record 0x86c000 (clube, [this+4], a palavra +0x32 e a +0x30 do jogo, 0x32; as palavras
#   vao com as metades altas dos registros, o chamado le palavras) e NationalTeams::clear_current_competition 0x775000
#   (push [clube+0x53]), 0. Prologo mov al, [ecx+0x101] (6 bytes).
# - 0x918b00 WcEuropeLeague::build_fate_news_text thiscall(clube_indice, destino, fase, a3, a4, a5, a6, a7, frase_inteira,
#   texto) ret 0x28 (os argumentos do world_cup_build_fate_news_text): a3 (palavra com sinal) 0x3e8..0x3f1 = os grupos;
#   0x3fc = os segundos; a4 o jogo (palavra +0x30 = 0xa4: a repescagem). langlib_sprintf 0x66f4e0 em 0xde1f64,
#   Text::set_string 0x4ae660, Text::add_link 0x4ae8a0; comp_util_write_group_qualify_news 0x4b4590 (cdecl; clube_indice,
#   a3, [this+4], 7 ou -2, frase_inteira, texto); comp_util_write_cup_tie_result_news 0x4b0b80 (cdecl; clube_indice, a4,
#   a7, destino, a5, texto). Prologo mov eax, [esp+0x10]; push esi (5 bytes).
# - 0x918ec0 WcEuropeLeague::draw_groups thiscall() ret 0: n = byte +0xff + 2 grupos; malloc(n x 6 x 4) em +0xf7 (caixa
#   0x27b); ate 6 potes enquanto o indice do bolo (byte com sinal) < +0xf2: 20 trocas da ordem dos grupos (utils_random
#   0x90cfc0 (n) duas vezes, a primeira e a posicao a), depois cada grupo na ordem (k palavra < n) recebe a proxima
#   selecao se a contagem dele < WcEuropeLeague::get_group_size 0x918dd0 (thiscall(g - 1), palavra com sinal). Prologo
#   sub esp, 0x224 (6 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os registros so a original com o plano (o retorno, *saida = 4 e a
# Copa com mais uma); o texto lado a lado (compare_text_writer); o sorteio: a original na tabela dela, o gerador de
# volta, a copia numa tabela propria (sorteios nao contados), as vagas preenchidas de cada grupo iguais.
# Defeitos do original [inferencia]: (1) o texto curto do perdedor da repescagem diz "out of" a Eurocopa (o registro de
# [0x9cf7a4]) e o vinculo e o da Copa do Mundo; (2) o sorteio pede o tamanho do grupo g pelo indice g - 1 (o grupo da
# propria secao e o -1 da get_group_size): coerente com a init_own_group_table, nao e defeito, so a convencao; (3) as
# vagas nao usadas da tabela do sorteio ficam com o que o malloc deixou.
# Depende do cx16_11 (FN_WC_EUROPE_LEAGUE_GET_GROUP_SIZE) e do cx17_2 (FN_NATIONALTEAMS_ADD_QUALIFIED_NATION): integrar
# depois deles (INTEGRAR_ANTES.txt).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_europe_league_update_staff_records_for_status', 0x918980, [0x8a, 0x81, 0x01, 0x01, 0x00, 0x00]),
           ('wc_europe_league_build_fate_news_text', 0x918b00, [0x8b, 0x44, 0x24, 0x10, 0x56]),
           ('wc_europe_league_draw_groups', 0x918ec0, [0x81, 0xec, 0x24, 0x02, 0x00, 0x00])]
DOCS = ('as eliminatórias europeias da Copa do Mundo: os registros do destino de uma seleção, o texto da notícia e o sorteio dos grupos',
        'the European World Cup qualifying: the staff records of a nation fate, the news text and the group draw',
        [('wc_europe_league_update_staff_records_for_status', 'WcEuropeLeague::update_staff_records_for_status', '0x918980', 'wc_europe_league.cpp', 'thiscall, ret 0x18',
          'regra: nas eliminatórias europeias o vencedor de grupo ganha o registro da Copa do Mundo (peso 100), entra na Copa e na lista das classificadas e fica com a situação 4; na repescagem o vencedor ganha o mesmo com peso 50 e o perdedor o registro de eliminado (peso 50) e encerra a campanha; os outros destinos dos grupos não deixam registro',
          'rule: in the European qualifying a group winner gets the World Cup record (weight 100), enters the World Cup and the list of qualified nations and its status becomes 4; in the play-off the winner gets the same with weight 50 and the loser the knocked-out record (weight 50) and ends its campaign; the other group fates leave no record'),
         ('wc_europe_league_build_fate_news_text', 'WcEuropeLeague::build_fate_news_text', '0x918b00', 'wc_europe_league.cpp', 'thiscall, ret 0x28',
          'regra: o texto do destino nas eliminatórias europeias: nos grupos o vencedor "se classifica" para a Copa (na frase inteira, a frase comum de grupo) e o eliminado a frase comum de grupo; o melhor segundo colocado "é o melhor segundo" (ou "um dos três melhores", com uma seleção isenta); na repescagem a frase comum de confronto, ou na curta "se classifica" para a Copa o vencedor e "fora" da Eurocopa o perdedor',
          'rule: the fate text in the European qualifying: in the groups the winner "qualify for" the World Cup (in the full sentence, the shared group one) and the eliminated the shared group sentence; the best runner-up "are the best second placed team" (or "one of the three best", with one exempt nation); in the play-off the shared tie sentence, or in short "qualify for" the World Cup for the winner and "out of" the European Championship for the loser'),
         ('wc_europe_league_draw_groups', 'WcEuropeLeague::draw_groups', '0x918ec0', 'wc_europe_league.cpp', 'thiscall, ret 0',
          'regra: o sorteio das eliminatórias europeias: as seleções, da mais forte para a mais fraca, vão para os grupos pote a pote; em cada pote a ordem dos grupos é embaralhada (20 trocas) e cada grupo com vaga recebe a próxima seleção, então cada pote dá uma seleção a cada grupo',
          'rule: the European qualifying draw: the nations, strongest first, go into the groups pot by pot; in each pot the group order is shuffled (20 swaps) and each group with room takes the next nation, so every pot gives one nation to each group')])
CAMPOS = []
INTERESSA = []
