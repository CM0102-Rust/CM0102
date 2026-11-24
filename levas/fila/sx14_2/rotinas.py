# O texto da noticia da reacao da diretoria a uma partida (MANAGER_MANAGER::news_board_match_reaction_text 0x6a6310,
# manager_manager.cpp; leitor sx14, rodada 5, 23/11/2026). Chamado pelas noticias (0x77bc80, tipo 0x138c, postado por
# post_board_match_reaction_news 0x6a5fa0); 115 chamadas na carreira ENG ate 2002-06-01. Convencoes conferidas:
# - a rotina: thiscall(MANAGER_MANAGER, noticia, corpo, Text), ret 0xc; quadro SEH sem destrutor (estado 0);
# - 0x785830 (NewsItem::get_slot): thiscall, ret 4; manchete com os campos 0, 1, 2 lidos direto; corpo com o contador
#   em byte de 0 a 17 e o registro da partida depois (0x7e4fa0 cdecl(noticia, &contador, registro), registro com
#   Date::Date 0x54a110 na entrada);
# - 0x6a69c0 thiscall com 15 argumentos (ret 0x3c), chamada pelo endereco (com ou sem o sx14_1);
# - 0x729500 thiscall([0xb6b7c0], indice, clube, adversario, dia, ano), ret 0x14 (o original empilha os dwords de
#   +0 e +2 do registro; a rotina le so a palavra baixa: RecordMatch::link_score); 0x53dba0 cdecl(pessoa, buf, 200, 1);
#   0x4ae660/0x4ae8a0 thiscall do Text; 0x66f4e0 cdecl.
# Prologo `push -1; push 0x95cf2b` (7 bytes, fronteira de instrucao, sem relativo).
# Comparador: compare_text_writer.
SAVE = False
ROTINAS = [('news_board_match_reaction_text', 0x6a6310, [0x6a, 0xff, 0x68, 0x2b, 0xcf, 0x95, 0x00])]
DOCS = ('o texto da notícia da reação da diretoria a uma partida',
        "the news text of the board's reaction to a match",
        [('news_board_match_reaction_text', 'MANAGER_MANAGER::news_board_match_reaction_text', '0x6a6310', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'a manchete ou o corpo da notícia da reação da diretoria a uma partida: a manchete pelas marcas (clássico: encantada, feliz, furiosa, descontente ou só reação, sem clube; senão a do adversário, menos a de fúria, que cita o próprio clube), com os vínculos dos dois clubes; o corpo relê os 18 campos e o registro da partida, monta o placar "maior-menor" e o veredito da diretoria (0x6a69c0), com os vínculos dos dois clubes, da competição, de cada jogador citado e do placar (se a partida ainda está nas estatísticas)',
          "the headline or body of the news of the board's reaction to a match: the headline by the marks (derby: delighted, happy, furious, unhappy or just the reaction, with no club; otherwise the opponent's, except the fury one, which quotes the club itself), with the links to both clubs; the body reads the 18 slots and the match record again, builds the score \"higher-lower\" and the board's verdict (0x6a69c0), with the links to both clubs, the competition, each named player and the score (when the match is still in the stats)")])
