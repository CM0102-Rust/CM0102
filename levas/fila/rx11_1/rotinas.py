# Os textos das notícias do destino de uma seleção nas eliminatórias africanas e da CONCACAF (wc_african_cup.cpp,
# wc_concacaf_cup.cpp; leitor rx11, rodada 6, 25/11/2026). Simulação na campanha de 2006 (as eliminatórias africanas e
# da CONCACAF de 2004 e 2005 na carreira de teste, que começa em 2001): LIGAS = 'ENG' e ATE = '2005-12-31', como os
# cx16_7..12.
# - 0x90fe00 WcAfricanCup::build_fate_news_text thiscall(clube, destino, fase, codigo, a5..a8, frase, texto) ret 0x28
#   (virtual +0x48, vtable 0x970ad0): nos grupos (códigos 0x407..0x40b e 0x472) a frase comum de grupo 0x4b4590 ou, curto,
#   "qualify for Final" (6) e "qualify for World Cup" (7, vínculos clube e Copa); na primeira fase (-1) e nos mata-matas
#   (código 0) a frase comum de confronto 0x4b0b80 ou, curto, "2nd round"/"knocked out in 1st round" e "qualify
#   for"/"knocked out of" a Copa (vínculos Copa e clube, nessa ordem). Prólogo mov dl, [esp+0xc]; push esi (5 bytes).
# - 0x9159b0 WcConcacafCup::build_fate_news_text (mesma assinatura, virtual +0x48 da vtable da CONCACAF): no mata-mata
#   caribenho (0x40f) e na repescagem (fase 2) a frase de confronto ou "qualify for Semi Finals" e a frase de vitória ou
#   derrota 0x4b4420; nos grupos semifinais (0x40c..0x40e) e no grupo final (0x96) "have qualified from the ... stage"
#   (nome longo da competição) ou, curto, "qualify for final"/"qualify for World Cup", e a frase de grupo com -2.
#   Prólogo mov eax, [esp+0x10]; push ebx (5 bytes).
# Iguais no cm0102-1080.exe. Comparadores: lado a lado em dois Texts (compare_text_writer).
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_african_cup_build_fate_news_text', 0x90fe00, [0x8a, 0x54, 0x24, 0x0c, 0x56]),
           ('wc_concacaf_cup_build_fate_news_text', 0x9159b0, [0x8b, 0x44, 0x24, 0x10, 0x53])]
DOCS = ('os textos das notícias do destino de uma seleção nas eliminatórias africanas e da CONCACAF',
        "the news texts of a nation's fate in the African and CONCACAF qualifying",
        [('wc_african_cup_build_fate_news_text', 'WcAfricanCup::build_fate_news_text', '0x90fe00', 'wc_african_cup.cpp', 'thiscall, ret 0x28',
          'regra: nas eliminatórias africanas, nos grupos a notícia diz que a seleção vai à fase final ou à Copa do Mundo (ou a frase comum de grupo); na primeira fase, que passou à segunda fase ou caiu na primeira; nos mata-matas, que vai à Copa ou caiu dela',
          'rule: in the African qualifying, in the groups the news says the nation reaches the final stage or the World Cup (or the shared group sentence); in the first round, that it reached the second round or went out in the first; in the ties, that it qualifies for the World Cup or is knocked out of it'),
         ('wc_concacaf_cup_build_fate_news_text', 'WcConcacafCup::build_fate_news_text', '0x9159b0', 'wc_concacaf_cup.cpp', 'thiscall, ret 0x28',
          'regra: nas eliminatórias da CONCACAF, no mata-mata caribenho e na repescagem o vencedor vai às semifinais; nos grupos semifinais o classificado vai à fase final e no grupo final à Copa do Mundo; quem cai num grupo tem a frase comum de grupo',
          'rule: in the CONCACAF qualifying, in the Caribbean knockout and the play-off the winner reaches the semi finals; in the semi-final groups a qualifier reaches the final stage, and in the final group the World Cup; one going out of a group gets the shared group sentence')])
CAMPOS = []
INTERESSA = []
