# O texto da notícia do destino de uma seleção nas eliminatórias sul-americanas e a ordem dos potes do sorteio da Copa
# (wc_south_american_league.cpp, world_cup.cpp; leitor
# cx17, rodada 5, 24/11/2026). Simulação: a virtual +0x48 roda no fim da liga sul-americana, que na carreira de teste é
# a campanha de 2004 (de 29 de março de 2004 a 15 de novembro de 2005): LIGAS = 'ENG' e ATE = '2005-12-31', como os
# cx16_7..9 e o cx17_2. A cobertura do mundo até 2002-08-01 não a viu (0 chamadas).
# - 0x920b40 WcSouthAmericanLeague::build_fate_news_text thiscall(clube_indice, destino, fase, a3..a7, frase_inteira,
#   texto) ret 0x28 (os argumentos do world_cup_build_fate_news_text, 0x92cea0, já em Rust). Destino 7 com frase
#   inteira: langlib_sprintf 0x66f4e0 (cdecl) em 0xde1f64 com 0xad4d6c "{} have qualified from the {}." (gênero curto
#   do clube 2x, gênero curto da competição +0x52 2x, nome curto do clube +0x38, nome longo da competição +4),
#   Text::set_string 0x4ae660 e Text::add_link 0x4ae8a0 do clube (0x7d5) e do nome longo da competição (0x7d0). Destino 7
#   sem ela: 0xad44cc "{} qualify for {}" com a Copa do Mundo ([0xae23d0] + 0x6b x [0x9cf79c]; gênero curto e nome
#   curto), os vínculos da Copa (0x7d0) primeiro e do clube (0x7d5) depois. Destino 1: 0xad4d2c "{} qualify for Oceania
#   playoff" e o vínculo do clube. Destino 0xfe: comp_util_write_group_qualify_news 0x4b4590 (cdecl; clube_indice, o
#   dword com a palavra +0x3a sobre a metade alta do texto, [this+4], -2, frase_inteira, texto). Outro: nada.
# - 0x92f660 compare_entries_by_pot cdecl(a, b) (world_cup.cpp): o comparador da qsort 0x945333 das entradas do sorteio
#   (byte com sinal +4 de a menos o de b). Quem chama: WorldCup::seed_teams 0x92f230 (já em Rust; passa 0x92f660 à
#   qsort, que entra pelo desvio) e SpaCup::collect_entrants 0x84d910 (só com a Espanha). Na carreira ENG o seed_teams
#   não ordena em 2002 (cqf_6: sem cabeças de chave em 2002): a primeira ordenação é a da Copa de 2006, na semeadura de
#   15/12/2005 (world_cup_fields::SEEDING_DAY, 15/12 do ano anterior) [inferência]: por isso vem neste item, com a
#   mesma data. Prólogo mov eax, [esp+4]; mov ecx, [esp+8] (8 bytes).
# Prólogo da 0x920b40 mov al, [esp+8]; push ebx (5 bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores:
# compare_text_writer (a original no Text de verdade, a cópia num Text local; o texto, os vínculos e o gerador de
# sorteios têm de bater); o comparador dos potes lado a lado no retorno.
SAVE = False
LIGAS = 'ENG'
ATE = '2005-12-31'
ROTINAS = [('wc_south_american_league_build_fate_news_text', 0x920b40, [0x8a, 0x44, 0x24, 0x08, 0x53]),
           ('compare_entries_by_pot', 0x92f660, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08])]
DOCS = ('o texto da notícia do destino de uma seleção nas eliminatórias sul-americanas e a ordem dos potes do sorteio da Copa do Mundo',
        'the news text of a nation fate in the South American qualifying and the pot order of the World Cup draw',
        [('wc_south_american_league_build_fate_news_text', 'WcSouthAmericanLeague::build_fate_news_text', '0x920b40', 'wc_south_american_league.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de uma seleção nas eliminatórias sul-americanas: a classificada "se classificou pelas eliminatórias sul-americanas" na frase inteira e "se classifica para a Copa do Mundo" na curta; a da repescagem "se classifica para a repescagem da Oceania"; a eliminada, a frase comum de grupo; outro destino, sem texto',
          'rule: the news of a nation fate in the South American qualifying: a qualified nation "have qualified from the" South American qualifying in the full sentence and "qualify for" the World Cup in the short one; a play-off nation "qualify for Oceania playoff"; an eliminated one, the shared group sentence; any other fate, no text'),
         ('compare_entries_by_pot', 'compare_entries_by_pot', '0x92f660', 'world_cup.cpp', 'cdecl',
          'regra: as entradas do sorteio são ordenadas pelo pote, do menor para o maior',
          'rule: the draw entries are sorted by pot, lowest first')])
CAMPOS = []
INTERESSA = []
