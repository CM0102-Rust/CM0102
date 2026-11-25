# Os registros dos tecnicos, o texto das noticias e os codigos de progresso das eliminatorias asiaticas da Copa de 2006
# (wc_asia_league.cpp; leitor rx09, rodada 6, 25/11/2026). Simulacao com a Inglaterra ate 2005-12-31 (o fim de cada fase
# em 2005, pelas virtuais +0x44, +0x48 e +0x68).
# - 0x912500 WcAsiaLeague::update_staff_records_for_status thiscall(clube, desfecho, fase, saida, jogo, a6) ret 0x18:
#   cada ramo conferido (fase < 9; 9..10; 11 pelo byte +0xee; 12); Staff_Records 0x86bdd0 (clube, registro, fase,
#   rodada, peso) e 0x86c1d0 (clube, registro, fase, peso) no G_STAFF_RECORDS; para a Copa o registro da Copa ([0xae23d0]
#   + 0x6b x [0x9cf79c]), WorldCup::add_qualified_team 0x92e740, *saida = 4 sem conferir o ponteiro e
#   NationalTeams::add_qualified_nation 0x7779b0, -1; eliminada: NationalTeams::clear_current_competition 0x775000
#   (push [clube+0x53]); os demais devolvem o byte +0xee com sinal.
# - 0x912780 WcAsiaLeague::build_fate_news_text thiscall(..) ret 0x28: os pushes conferidos (0x4b4590 (clube, a4,
#   registro, desfecho ou -2, noticia ou 0, texto); 0x4b0b80 (clube, a5, a8, desfecho, a6, texto)); os textos 0xad47bc,
#   0xad4780, 0xad4700 com o clube e 0xad4738, 0xad46c0 com o clube e a Copa (generos curtos +0x52 e nomes curtos +0x38,
#   vinculos 0x7d5 e 0x7d0).
# - 0x914730 WcAsiaLeague::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: as seis tabelas de
#   saltos 0x914980..0x9149d8 conferidas nos bytes; a fase 11 pelo byte +0xee (comparacoes sem sinal).
# Prologos mov al, [esp+0xc]; push esi (5 bytes) e mov eax, [esp+4]; push ebx (5 bytes). Iguais no cm0102-1080.exe.
# Comparadores: os registros so a original com o plano (o retorno e *saida = 4); os textos proprios lado a lado
# (compare_text_writer); o progresso lado a lado.
# Depende do rx09_16 (WC_ASIA_EXEMPT), do cx17_2 (FN_NATIONALTEAMS_ADD_QUALIFIED_NATION), do cx13_6 (play_off_club_text) e
# do cx14_4 (club_and_competition_short_text). O rx08_4 tem uma copia privada do mesmo passo de classificar para a Copa
# (wc_europe_league_qualify_nation, com peso variavel): ao integrar os dois, pode usar o wc_asia_qualify_nation.
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_asia_league_update_staff_records_for_status', 0x912500, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('wc_asia_league_build_fate_news_text', 0x912780, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('wc_asia_league_set_club_progress', 0x914730, [0x8b, 0x44, 0x24, 0x04, 0x53])]
DOCS = ('os registros dos técnicos, o texto das notícias e os códigos de progresso das eliminatórias asiáticas',
        'the staff records, the fate news text and the progress codes of the Asian World Cup qualifying',
        [('wc_asia_league_update_staff_records_for_status', 'WcAsiaLeague::update_staff_records_for_status', '0x912500', 'wc_asia_league.cpp', 'thiscall, ret 0x18',
          'regra: na primeira fase quem passa ganha o registro de classificação e quem cai o de eliminação; na segunda, o primeiro vai à Copa, o seguinte à terceira fase; na terceira e na repescagem, o vencedor vai à Copa (ou à repescagem) e o perdedor sai; quem sai encerra a campanha da seleção',
          'rule: in the first round a team through gets the qualified record and one out the failed record; in the second round, the winner goes to the World Cup and the next to the third round; in the third round and the play-off the winner goes to the World Cup (or to the play-off) and the loser is out; a team out ends its campaign'),
         ('wc_asia_league_build_fate_news_text', 'WcAsiaLeague::build_fate_news_text', '0x912780', 'wc_asia_league.cpp', 'thiscall, ret 0x28',
          'regra: sem notícia, "classifica-se para a segunda fase", "para a terceira fase", "para a repescagem", "para a Copa do Mundo" ou "está fora da Copa do Mundo"; com notícia, o texto comum dos grupos ou do mata-mata',
          'rule: without a news item, "qualify for second round", "for third round", "for playoff", "for the World Cup" or "out of the World Cup"; with one, the common group or cup tie text'),
         ('wc_asia_league_set_club_progress', 'WcAsiaLeague::set_club_progress', '0x914730', 'wc_asia_league.cpp', 'thiscall, ret 0x14',
          'regra: os códigos de progresso de uma seleção viram posições gerais: na primeira fase 1 a 4 viram 1 (melhor) ou 9 (atual e pior), 11, 21 e 31; na segunda, 1 e 2 viram 1 ou 4, depois 5, 7 e 9; na terceira, conforme os isentos, os primeiros viram 1',
          'rule: a team\'s progress codes become overall places: in the first round 1 to 4 become 1 (best) or 9 (current and worst), 11, 21 and 31; in the second round 1 and 2 become 1 or 4, then 5, 7 and 9; in the third round, by the exempt count, the leaders become 1')])
CAMPOS = []
INTERESSA = []
