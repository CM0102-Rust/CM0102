# O texto do destino de um clube e o progresso no Campeonato de Clubes da Oceania (oceania_club_champ.cpp; leitor cx04,
# rodada 5, 24/11/2026). Simulacao com as 8 ligas (a competicao roda com a Australia: OceaniaClubChamp
# update_staff_records_for_status 16 chamadas e start_new_season 1 na cobertura de 8 ligas ate 2002-06-01).
# - 0x7976b0 OceaniaClubChamp::build_fate_news_text thiscall(clube, desfecho, fase, a4, partida, a6, a7, a8, noticia,
#   Text) ret 0x28 (virtual 0x48): fase < 1 (com sinal): desfecho 6 com noticia 0x4b4590 cdecl(clube, a4, competicao, 6,
#   noticia, Text), sem noticia o texto 0xa6a0ac com o nome curto do clube e o vinculo 0x7d5; desfecho 0xfe 0x4b4590 com
#   -2 (com ou sem noticia: o salto volta depois do teste); fase 1: Comp::build_fate_news_text 0x48c6d0 (a7 = 0); fase 2:
#   com noticia 0x4b0b80 cdecl(clube, partida, a8, desfecho, a6, Text), sem noticia 1 -> 0x98732c, 2 -> 0x9872c8 e, em
#   qualquer desfecho, o Text recebe o buffer 0xde1f64 e os vinculos do clube e da competicao. Prologo mov al,
#   [esp+0xc]; push esi (5 bytes).
# - 0x7987a0 OceaniaClubChamp::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual 0x68):
#   Comp_Stats::find_club_entry 0x4a4850 (thiscall em [this+8], push clube); fase < 1: as tabelas de salto 0x798878,
#   0x798888 e 0x798898 (1 a 4); grava +0x74, +0x73, +0x75. Prologo mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# Defeito do original: na fase final, sem noticia e com desfecho fora de 1 e 2, o Text recebe o que estiver no buffer
# compartilhado 0xde1f64 (texto velho); a copia faz o mesmo.
# Iguais no cm0102-1080.exe. Os auxiliares (group_championship_fate_text, group_championship_progress e os
# comparadores) servem tambem ao Mundial de Clubes (cx04_6).
# Comparadores: o texto lado a lado nos caminhos so de texto (compare_text_writer; com o modo CM_TEXTO_NOTICIAS); o
# progresso lado a lado nos 3 bytes da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('oceania_club_champ_build_fate_news_text', 0x7976b0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('oceania_club_champ_set_club_progress', 0x7987a0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('o texto do destino e o progresso dos clubes no Campeonato da Oceania', "a club's fate text and progress in the Oceania Club Championship",
        [('oceania_club_champ_build_fate_news_text', 'OceaniaClubChamp::build_fate_news_text', '0x7976b0', 'oceania_club_champ.cpp', 'thiscall, ret 0x28',
          'escreve o texto da notícia do destino de um clube no Campeonato da Oceania: nos grupos, quem passa "classifica-se para a semifinal" (ou a notícia do grupo) e quem cai a notícia do grupo; na fase 1 o texto geral da competição; na fase final o campeão ou o vice (ou o resultado do confronto). Na fase final, um desfecho sem modelo deixa no texto o que estava no buffer compartilhado (defeito do original)',
          'writes the news text of a club\'s fate in the Oceania Club Championship: in the groups, one that goes through "qualifies for the semi final" (or the group news) and one that goes out gets the group news; at stage 1 the competition\'s general text; in the final stage the winner or the runner-up (or the tie result). In the final stage, an outcome without a template leaves in the text whatever the shared buffer held (a defect of the original)'),
         ('oceania_club_champ_set_club_progress', 'OceaniaClubChamp::set_club_progress', '0x7987a0', 'oceania_club_champ.cpp', 'thiscall, ret 0x14',
          'guarda o progresso de um clube nas estatísticas da competição: na fase de grupos as colocações 1 a 4 viram os códigos do campeonato (atual 1, 3, 5, 7; melhor 1, 1, 5, 7; pior 3, 3, 5, 7); depois, como vêm',
          'stores a club\'s progress in the competition statistics: in the group stage places 1 to 4 become the championship\'s codes (current 1, 3, 5, 7; best 1, 1, 5, 7; worst 3, 3, 5, 7); afterwards, as they come')])
CAMPOS = []
INTERESSA = []
