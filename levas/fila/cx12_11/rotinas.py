# Os registros pela situacao, o texto da noticia do destino e o progresso dos clubes da Primeira Divisao norueguesa e
# da repescagem com a Premier (nor_first.cpp; leitor cx12, rodada 5, 24/11/2026). Simulacao com a Noruega (LIGAS e ATE
# dos itens mwe): a temporada de 2001 acaba em outubro e a repescagem (fase 0) e jogada a partir de 25 de outubro de
# 2001. O texto so com o modo CM_TEXTO_NOTICIAS e um tecnico humano como destinatario.
# - 0x791080 NorFirst::update_staff_records_for_status, 0x791270 NorFirst::build_fate_news_text e 0x7913f0
#   NorFirst::set_club_progress: os corpos do NirFirst (0x78c300, 0x78c4f0, 0x78c670; conferidos instrucao a
#   instrucao: so as chaves [0x9cf6bc] Premier e [0x9cf6c0] Primeira, a faixa da liga 3 (melhor 3 -> 2, pior 3 -> 4) e os
#   codigos da Premier na repescagem 1 -> 11 e 2 -> 14 mudam; a ordem dos registradores tambem). O rebaixamento do
#   clube da Premier que perde a repescagem usa a propria Premier norueguesa (no NirFirst, a da Republica da Irlanda).
#   Os auxiliares first_division_* sao os do cx12_9. Convencoes: thiscall, ret 0x18 / 0x28 / 0x14. Prologos mov edx,
#   [0x9cf6bc] (6 bytes), push ebx; mov ebx, [esp+0x28] (5) e mov ecx, [ecx+8]; push ebx; push esi (5).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os do cx12_9.
# Depende do cx12_9 (FirstDivisionPlayOff e os auxiliares first_division_*) e, por ele, do cx04_5: integrar depois deles.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('nor_first_update_staff_records_for_status', 0x791080, [0x8b, 0x15, 0xbc, 0xf6, 0x9c, 0x00]),
           ('nor_first_build_fate_news_text', 0x791270, [0x53, 0x8b, 0x5c, 0x24, 0x28]),
           ('nor_first_set_club_progress', 0x7913f0, [0x8b, 0x49, 0x08, 0x53, 0x56])]
DOCS = ('os registros pela situação, o texto da notícia do destino e o progresso dos clubes da Primeira Divisão norueguesa',
        'the Norwegian First Division staff records by outcome, fate news text and club progress',
        [('nor_first_update_staff_records_for_status', 'NorFirst::update_staff_records_for_status', '0x791080', 'nor_first.cpp', 'thiscall, ret 0x18',
          'regra: a mesma da Primeira norte-irlandesa: na liga os registros de título, acesso, vaga na repescagem e queda; na repescagem contra a Premier, o clube da Primeira que vence sobe e o da Premier que perde cai',
          'rule: the same as the Northern Irish First Division: in the league the title, promotion, play-off place and relegation records; in the play-off against the Premier, the First Division club that wins goes up and the Premier club that loses goes down'),
         ('nor_first_build_fate_news_text', 'NorFirst::build_fate_news_text', '0x791270', 'nor_first.cpp', 'thiscall, ret 0x28',
          'o texto da notícia da repescagem norueguesa, como o norte-irlandês: escapa do rebaixamento, rebaixado ou consegue o acesso; os outros destinos com o texto base',
          'the Norwegian play-off news text, as the Northern Irish one: avoids relegation, relegated or wins promotion; the other fates with the base text'),
         ('nor_first_set_club_progress', 'NorFirst::set_club_progress', '0x7913f0', 'nor_first.cpp', 'thiscall, ret 0x14',
          'a previsão de colocação na Primeira norueguesa: na liga, a melhor 3 vira 2 e a pior 3 vira 4; na repescagem, o clube da Premier grava nas estatísticas da Premier (1 vira 11 e 2 vira 14) e o da Primeira dobra os códigos',
          'the place forecast in the Norwegian First Division: in the league, best 3 becomes 2 and worst 3 becomes 4; in the play-off, the Premier club writes into the Premier statistics (1 becomes 11 and 2 becomes 14) and the First Division club doubles its codes')])
CAMPOS = []
INTERESSA = []
