# O destino de uma seleção na Copa do Mundo no noticiário; as notícias dos prêmios e as listas de indicados
# (world_cup.cpp, award_manager.cpp; leitor mwg, rodada 3 do mundo inteiro, 19/11/2026). build_fate_news_text roda 4
# vezes até 2002-08-01 (as notícias da Copa de 2002, junho), write_award_news 76 (as notícias dos prêmios do fim da
# temporada) e add_shortlist 1 (cobertura_mundo.txt); a Copa e os prêmios ingleses rodam na carreira só com a
# Inglaterra: LIGAS = 'ENG'.
# Convenções conferidas na desmontagem (0x92cea0..0x92d0e2, 0x4143f0..0x414418, 0x4147f0..0x4148d1):
# - WorldCup::build_fate_news_text: thiscall, ret 0x28, os argumentos de GoldCup::build_fate_news_text (0x5e0f50,
#   cqz_14); fase (byte com sinal) < 7: destino 6 ou 0xfe na frase de grupo 0x4b4590 (cdecl, 6 argumentos: clube, a3,
#   [+4], 6 ou -2, completo, texto), com ou sem `completo`; outro destino nada; fase 7: com `completo` a frase de mata-mata
#   0x4b0b80 (clube, a4, a7, destino, a5, texto), sem ele os mesmos seis modelos da Copa América (0x98732c..0x987198)
#   por langlib_sprintf em 0xde1f64, Text::set_string e os dois vínculos (0x7d5 o clube, 0x7d0 a competição); fase > 7
#   nada;
# - AWARD_MANAGER::write_award_news: thiscall, ret 0x14; o objeto [[[this+8] + 4 x nação] + 4][prêmio] e a virtual +0xc
#   dele (write_news(item, texto, corpo), thiscall, ret 0xc), cujo eax devolve;
# - AWARD_MANAGER::add_shortlist: thiscall, ret 4; a contagem +0x18 (palavra com sinal) >= capacidade +0x1a: +5, malloc
#   (0x944e46) ou realloc (0x945667) de 4 x capacidade em +0x14; sem memória a caixa (linha 0x39a, 0x987e24) e ax = -1;
#   senão [+0x14][contagem] = lista, contagem + 1 e eax = (esi com a palavra baixa = a contagem nova) - 1 (a palavra
#   alta é a de this).
# Comparadores: build_fate_news_text e write_award_news lado a lado em dois Text (compare_text_writer); add_shortlist só
# no original (pode realocar), com o plano (a contagem + 1, a lista na última vaga, ax = a contagem antiga).
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('world_cup_build_fate_news_text', 0x92cea0, [0x8a, 0x44, 0x24, 0x0c, 0x57]),
           ('write_award_news', 0x4143f0, [0x8b, 0x41, 0x08, 0x8b, 0x4c, 0x24, 0x04]),
           ('add_shortlist', 0x4147f0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o destino de uma seleção na Copa do Mundo no noticiário; as notícias dos prêmios e as listas de indicados',
        "a nation's World Cup fate in the news; the award news and the award shortlists",
        [('world_cup_build_fate_news_text', 'WorldCup::build_fate_news_text', '0x92cea0', 'world_cup.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino de uma seleção na Copa do Mundo: na fase de grupos só a classificação às oitavas e a eliminação no grupo têm texto (a frase de grupo); no mata-mata (fase 7) o título, o vice, o terceiro e o quarto lugares, a eliminação e qualquer outro destino ("segue adiante"), como os da Copa América; depois da fase 7 nada',
          "the news text of a nation's World Cup fate: in the group stage only the round-of-16 qualification and the group exit have a text (the group sentence); in the knockout (stage 7) the title, the runners-up, third and fourth places, the elimination and any other fate (\"through to the next round\"), as the Copa América's; past stage 7 nothing"),
         ('write_award_news', 'AWARD_MANAGER::write_award_news', '0x4143f0', 'award_manager.cpp', 'thiscall, ret 0x14',
          'repassa o pedido de texto de uma notícia ao prêmio escolhido pela nação e pelo índice do prêmio (o write_news dele: manchete ou corpo)',
          "passes a news text request on to the award chosen by the nation and the award index (its write_news: headline or body)"),
         ('add_shortlist', 'AWARD_MANAGER::add_shortlist', '0x4147f0', 'award_manager.cpp', 'thiscall, ret 4',
          'acrescenta uma lista de indicados ao vetor do gerente de prêmios (que cresce de 5 em 5; sem memória, a caixa da linha 0x39a e −1) e devolve o índice dela',
          "appends a shortlist to the award manager's array (grown 5 at a time; out of memory, the box at line 0x39a and −1) and returns its index")])
