# As eliminatorias da Oceania para a Copa do Mundo: o texto da noticia do destino e a repescagem intercontinental
# (wc_oceania_league.cpp; leitor rx12, rodada 6, 25/11/2026). Usa club_of_index, qualifying_fate_line,
# link_fate_club e link_fate_competition do rx11_1 (ver INTEGRAR_ANTES.txt) e compare_text_writer (compare/media.rs).
# Simulacao na campanha de 2006 (as eliminatorias da Oceania de 2004 e a repescagem com a America do Sul em 2005):
# LIGAS = 'ENG' e ATE = '2005-12-31', como os rx11_1 e cx16_7..12.
# Convencoes conferidas na desmontagem:
# - 0x91e990 thiscall(clube, destino, fase, codigo, a5..a8, frase, texto) ret 0x28 (virtual +0x48): fase (byte com
#   sinal) < 1: destino 6 com frase 0xad4c48 (generos do clube x2 e da competicao +0x52 x2, nome curto do clube, nome
#   longo +4 da competicao; vinculos clube 0x7d5 e competicao 0x7d0) ou, curto, 0xad4c0c (vinculo do clube); 0xfe:
#   0x4b4590 (clube, codigo, registro, -2, frase, texto) cdecl; fase 1, ou codigo (word) 0xa0: com frase 0x4b0b80
#   (clube, a5, a8, destino, a6, texto) cdecl; sem, 0xad4be0/0xad4ba4 (final) ou 0xad4b78/0xad4b40 (repescagem),
#   vinculo do clube; sprintf 0x66f4e0 em 0xde1f64, Text::set_string 0x4ae660, Text::add_link 0x4ae8a0;
# - 0x91f800 thiscall(this) ret 0: o objeto da secao sul-americana lido antes; malloc(8), nulo: caixa linha 0x403; o
#   vencedor da final (1a entrada de 6 bytes com +5 == 1 na tabela +0xa7 de [[+0xc]+4], conta word +0x3e) e a 1a linha
#   com +0x37 == 1 da tabela da secao; sem os dois: caixa linha 0x41f e free da lista; a virtual +0x3c (o byte de +0x30,
#   &rodadas, &extra, 0); operator new(0xb2), Cup_Stage 0x522e00 (this, 2, lista, rodadas, competicao, tabela, ano, o
#   byte de +0x30, 4, extra, 0x14, 0, 0, 0, 0) em [+0xc][+0x30]; free da lista e da tabela; nula: caixa linha 0x431;
#   senao Cup::set_up_round 0x51c800 (0).
# Comparadores: o texto lado a lado em dois Texts (compare_text_writer); a repescagem so a original e as duas selecoes
# conferidas contra o plano.
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_oceania_league_build_fate_news_text', 0x91e990, [0x8a, 0x44, 0x24, 0x0c, 0x53]),
           ('wc_oceania_league_create_playoff_stage', 0x91f800, [0x6a, 0xff, 0x68, 0x7e, 0x5b, 0x96, 0x00])]
DOCS = ('as eliminatórias da Oceania: o texto do destino e a repescagem',
        'the Oceania qualifying: the fate text and the play-off',
        [('wc_oceania_league_build_fate_news_text', 'WcOceaniaLeague::build_fate_news_text', '0x91e990', 'wc_oceania_league.cpp', 'thiscall, ret 0x28',
          'regra: nas eliminatórias da Oceania, nos grupos o classificado vai às semifinais (ou a frase comum de grupo para quem cai); na final o vencedor vai à repescagem e o perdedor cai das eliminatórias; na repescagem intercontinental o vencedor vai à Copa do Mundo e o perdedor cai',
          'rule: in the Oceania qualifying, in the groups a qualifier reaches the semi finals (or the shared group sentence for one going out); in the final the winner reaches the play-off and the loser is knocked out of the qualifying; in the inter-continental play-off the winner qualifies for the World Cup and the loser is knocked out'),
         ('wc_oceania_league_create_playoff_stage', 'WcOceaniaLeague::create_playoff_stage', '0x91f800', 'wc_oceania_league.cpp', 'thiscall',
          'monta a repescagem intercontinental em dois jogos: o vencedor da final da Oceania contra a seleção da repescagem da América do Sul; sem as duas, a caixa de erro',
          "builds the two-legged inter-continental play-off: the Oceania final's winner against the South American play-off nation; without both, the error box")])
CAMPOS = []
INTERESSA = []
