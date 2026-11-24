# A noticia do destino e os registros dos tecnicos na Copa da Suecia (swe_cup.cpp; leitor cx03, rodada 5, 24/11/2026).
# Competicoes da Suecia: LIGAS do grupo ENG SWE FIN NOR POR, ate 2002-08-01 (os registros rodam 718 vezes no mundo ate
# essa data; a noticia, escritora de texto, pelo modo CM_TEXTO_NOTICIAS).
# - 0x88d910 SweCup::build_fate_news_text thiscall(clube, destino, fase, a4..a8, noticia, texto) ret 0x28: com noticia,
#   0x4b0b80(clube, a5, a8, destino, a6, texto) (6 pushes, cdecl); sem: fase < 7 e destino 1, o texto 0xa8b4d4 "{clube}
#   through to {competicao} Third Round." (0x66f4e0, 8 pushes: o byte +0x52 do clube duas vezes, o byte +0x52 da
#   competicao duas vezes, os nomes curtos +0x38), Text::set_string e os dois links (0x7d5 e 0x7d0): european_cup_fate_text;
#   senao Comp::build_fate_news_text 0x48c6d0 com o a7 e a noticia trocados por 0. Prologo mov eax, [esp+0x24]; push ebx.
# - 0x88e2f0 SweCup::update_staff_records_for_status thiscall(clube, destino, fase, a4, partida, final) ret 0x18 (a
#   virtual 0x44) -> 0: fase < 7, peso = 0x4bf850(1, 0x50, a palavra +0x34 da partida, 2) (cdecl): destino 0xff ou 2,
#   Staff_Records::add_knocked_out_record 0x86c000 (ecx [0xdd7f00]; clube, competicao +4, +0x32, +0x30, peso; ret 0x14);
#   0, add_qualified_for_round_record 0x86bdd0 (clube, competicao, +0x32, o codigo +7 da rodada seguinte da tabela +0xa3,
#   peso); 1, 0x86bdd0(clube, competicao, 0, 0x1e, 0x5a); fase 7: 1, 0x86a800 e 2, 0x86b0b0 (clube, partida, final; ret
#   0xc). Prologo mov al, [esp+0xc]; push esi (5 bytes).
# Iguais no cm0102-1080.exe (as duas inteiras).
# Comparadores: a noticia nos caminhos so de texto lado a lado (compare_text_writer), a generica so a original; os
# registros so a original (vao para o historico), conferindo o retorno 0.
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
SAVE = False
ROTINAS = [('swe_cup_build_fate_news_text', 0x88d910, [0x8b, 0x44, 0x24, 0x24, 0x53]),
           ('swe_cup_update_staff_records_for_status', 0x88e2f0, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('a notícia do destino e os registros dos técnicos na Copa da Suécia', "the Swedish Cup's fate news and staff records",
        [('swe_cup_build_fate_news_text', 'SweCup::build_fate_news_text', '0x88d910', 'swe_cup.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de um clube na Copa da Suécia: com objeto de notícia, o resultado do confronto; sem, um clube que vence uma seção preliminar recebe "{clube} through to {competição} Third Round."; senão a notícia genérica da competição',
          "rule: the Swedish Cup news of a club's fate: with a news object, the tie result; without one, a club winning a preliminary section gets \"{club} through to {competition} Third Round.\"; otherwise the generic competition news"),
         ('swe_cup_update_staff_records_for_status', 'SweCup::update_staff_records_for_status', '0x88e2f0', 'swe_cup.cpp', 'thiscall, ret 0x18',
          'regra: os registros dos técnicos na Copa da Suécia: nas seções preliminares quem sai ou perde ganha o registro de eliminado, quem passa o da rodada seguinte e o vencedor da seção o de chegar à chave principal (rodada 0x1e, peso 0x5a); os pesos de 1 a 80 pela rodada; na chave principal o campeão e o vice ganham os registros da final',
          "rule: the Swedish Cup's staff records: in the preliminary sections a club going out or beaten gets the knocked-out record, a club going through the next round's record and a section winner the record of reaching the main draw (round 0x1e, weight 0x5a); the weights from 1 to 80 by round; in the main draw the winner and the runner-up get the final records")])
CAMPOS = []
INTERESSA = []
