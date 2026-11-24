# O texto do destino de um clube e o progresso no Mundial de Clubes (world_club_champ.cpp; leitor cx04, rodada 5,
# 24/11/2026). Simulacao com as 8 ligas e ATE 2003-08-15: a primeira edicao e em 2003 (o construtor 0x929140 leva o ano
# ao primeiro impar a partir de 2003), o sorteio em 5 de fevereiro e os jogos de 29 de julho a 12 de agosto de 2003; na
# cobertura do mundo ate 2002-08-01 as duas tem 0 chamadas. Sem a data longa, o item nao prova nada: se a cadeia nao
# puder ir ate la, deixar para a auditoria (ou tirar da fila).
# - 0x9298a0 WorldClubChamp::build_fate_news_text thiscall(clube, desfecho, fase, a4, partida, a6, a7, a8, noticia,
#   Text) ret 0x28 (virtual 0x48): o molde do da Oceania (cx04_5, group_championship_fate_text) com os grupos nas fases
#   < 2, sem o texto geral no meio, o texto 0xad580c de quem passa do grupo e, na fase 2, 1 -> 0x98732c, 2 -> 0x9872c8,
#   3 -> 0xad57a8. Prologo mov al, [esp+0xc]; push esi (5 bytes).
# - 0x92ad50 WorldClubChamp::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual 0x68): o
#   molde do da Oceania com fase < 2 e as tabelas de salto 0x92ae2c, 0x92ae3c e 0x92ae4c. Prologo mov eax, [esp+4]; mov
#   ecx, [ecx+8] (7 bytes).
# Defeito do original: na fase final, sem noticia e com desfecho fora de 1 a 3, o Text recebe o que estiver no buffer
# compartilhado 0xde1f64 (texto velho); a copia faz o mesmo.
# Iguais no cm0102-1080.exe. Usa os auxiliares do cx04_5: integrar depois dele.
# Comparadores: o texto lado a lado nos caminhos so de texto (compare_text_writer; com o modo CM_TEXTO_NOTICIAS); o
# progresso lado a lado nos 3 bytes da entrada.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ATE = '2003-08-15'
ROTINAS = [('world_club_champ_build_fate_news_text', 0x9298a0, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('world_club_champ_set_club_progress', 0x92ad50, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('o texto do destino e o progresso dos clubes no Mundial de Clubes', "a club's fate text and progress in the Club World Championship",
        [('world_club_champ_build_fate_news_text', 'WorldClubChamp::build_fate_news_text', '0x9298a0', 'world_club_champ.cpp', 'thiscall, ret 0x28',
          'escreve o texto da notícia do destino de um clube no Mundial de Clubes: nos grupos, quem passa "classifica-se para a final" (ou a notícia do grupo) e quem cai a notícia do grupo; na fase final o campeão, o vice ou o terceiro (ou o resultado do confronto). Na fase final, um desfecho sem modelo deixa no texto o que estava no buffer compartilhado (defeito do original)',
          'writes the news text of a club\'s fate in the Club World Championship: in the groups, one that goes through "qualifies for the final" (or the group news) and one that goes out gets the group news; in the final stage the winner, the runner-up or the third-placed club (or the tie result). In the final stage, an outcome without a template leaves in the text whatever the shared buffer held (a defect of the original)'),
         ('world_club_champ_set_club_progress', 'WorldClubChamp::set_club_progress', '0x92ad50', 'world_club_champ.cpp', 'thiscall, ret 0x14',
          'guarda o progresso de um clube nas estatísticas da competição: nas fases de grupos as colocações 1 a 4 viram os códigos do campeonato (atual 1, 5, 7, 10; melhor 1, 1, 7, 10; pior 4, 5, 7, 10); depois, como vêm',
          'stores a club\'s progress in the competition statistics: in the group stages places 1 to 4 become the championship\'s codes (current 1, 5, 7, 10; best 1, 1, 7, 10; worst 4, 5, 7, 10); afterwards, as they come')])
CAMPOS = []
INTERESSA = []
