# Os recordes de pessoal, o texto da noticia do destino e o progresso dos clubes da Primeira Divisao polonesa
# (pol_first.cpp; leitor cx13, rodada 5, 24/11/2026). Simulacao com a Polonia (as ligas e a data dos itens mwd): a fase
# de grupos acaba em novembro de 2001 e a fase final em maio/junho de 2002.
# - 0x7ca2a0 PolFirst::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18 (virtual
#   +0x44): fase < 1 com status 1, o recorde de classificado (0x86bdd0 com 0x44b, 0, 0x28: thiscall, ret 0x14) e 0;
#   outro status -1. Fase 1: status 5 o titulo (0x868c50, ret 8), 3 o rebaixamento (0x86a1c0, ret 8), ambos 0; outro
#   -1. Fase > 1: 0. Prologo mov al, [esp+0xc]; cmp al, 1 (6 bytes).
# - 0x7ca320 PolFirst::build_fate_news_text thiscall(10 dwords) ret 0x28: fase < 1 e destino 1, o texto do grupo do
#   titulo no buffer 0xde1f64 (langlib_sprintf 0x66f4e0, cdecl: com noticia, 0x9b54d0 com os bytes de genero do clube e
#   da competicao e os dois nomes curtos, 8 pushes; sem, 0x9b549c, 5 pushes); fase 1, Comp::build_fate_news_text
#   0x48c6d0 (this, a1, a2, 1, a4, a5, a6, 0, a8, a9, a10). O objeto Text nao recebe o texto (so o buffer). Prologo push
#   ebx; mov bl, [esp+0x10] (5 bytes).
# - 0x7ca3f0 PolFirst::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (virtual +0x68): fase < 1,
#   melhor = 1 se < 5 senao 9, pior = 8 se < 5 senao 16 (comparacoes sem sinal), atual * 2; senao como vieram. Prologo
#   mov eax, [esp+4]; mov ecx, [ecx+8] (7 bytes).
# Iguais no cm0102-1080.exe (inteiras). Comparadores: os recordes so a original contra o plano (o retorno); o texto lado
# a lado no buffer (so a original na fase final, que escreve o Text); o progresso lado a lado nos 3 bytes da entrada.
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('pol_first_update_staff_records_for_status', 0x7ca2a0, [0x8a, 0x44, 0x24, 0x0c, 0x3c, 0x01]),
           ('pol_first_build_fate_news_text', 0x7ca320, [0x53, 0x8a, 0x5c, 0x24, 0x10]),
           ('pol_first_set_club_progress', 0x7ca3f0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('os recordes de pessoal, o texto do destino e o progresso dos clubes da Primeira Divisão polonesa',
        'the Polish First Division\'s staff records, fate news text and club progress',
        [('pol_first_update_staff_records_for_status', 'PolFirst::update_staff_records_for_status', '0x7ca2a0', 'pol_first.cpp', 'thiscall, ret 0x18',
          'nos grupos, o clube que vai ao grupo do título ganha o recorde de classificado; na fase final, o campeão o recorde de título e o rebaixado o de rebaixamento',
          'in the groups, a club going into the championship group gets the qualified record; in the final stage, the champion gets the title record and a relegated club the relegation record'),
         ('pol_first_build_fate_news_text', 'PolFirst::build_fate_news_text', '0x7ca320', 'pol_first.cpp', 'thiscall, ret 0x28',
          'nos grupos, o texto "classificado para o grupo do título" (com a competição quando há notícia) vai para o buffer compartilhado; na fase final, o texto comum',
          'in the groups, the "qualify for championship group" text (with the competition when there is a news item) goes to the shared buffer; in the final stage, the common text'),
         ('pol_first_set_club_progress', 'PolFirst::set_club_progress', '0x7ca3f0', 'pol_first.cpp', 'thiscall, ret 0x14',
          'nos grupos, os dois grupos de oito viram uma escada só: o melhor código 1 quando abaixo de 5 (vaga no grupo do título ao alcance) e 9 senão, o pior 8 ou 16, o atual dobrado; na fase final, como vieram',
          'in the groups, the two groups of eight become a single ladder: the best code 1 when below 5 (a championship group place within reach) and 9 otherwise, the worst 8 or 16, the current one doubled; in the final stage, as they came')])
CAMPOS = []
INTERESSA = []
