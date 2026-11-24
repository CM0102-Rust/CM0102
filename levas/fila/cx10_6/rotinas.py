# As noticias da saida da crise e dos planos de ampliar o estadio e o construtor dos registros que comecam com uma data
# (fin_prm.cpp pelo mapa, finance.CPP pelos asserts; ita_ser_a.cpp; leitor cx10, rodada 5, 24/11/2026). Carreira ENG
# (sem LIGAS).
# - 0x5a02f0 finance_post_receivers_appeased_news thiscall(id do clube) ret 4 (o ecx nao e lido): noticia 0x1783 com o
#   id, ao tecnico do clube do id (a tabela [0xae23b4] + id * 0x245). Pela saida da crise nos eventos de financas da
#   semana (leave_crisis, ja em Rust, pelo exe::finance_post_receivers_appeased_news).
# - 0x5a0410 finance_post_ground_upgrade_plans_news thiscall(id do clube, competicao, tipo) ret 0xc: noticia 0x1780 com o
#   id, a competicao e o tipo (byte com sinal), ao tecnico do clube do id. Chamada duas vezes pela
#   Finance::upgrade_stadium_for_league 0x599e00 (original; 0x59a204 e 0x59a328), que roda quando um clube entra numa
#   liga com lotacao minima (fim de temporada ou criacao do jogo; pode nao rodar ate 2002-06-01).
# - 0x645800 FixedFixture::FixedFixture thiscall() ret 0 -> this: Date::Date 0x54a110 (mov ecx -> thiscall; o dia 0x28,
#   o ano 0x7b4 e 1 em +4) e devolve o registro. Construtor comum dos registros que comecam com uma data (o jogo fixo de
#   16 bytes, os recordes de clube 0x4482a0.., as estatisticas de competicao 0x49ee9f.., e outros 30 lugares, alguns
#   pelo vector constructor iterator). Prologo push esi; mov esi, ecx; call rel32 Date::Date: RELATIVAS (0x645800, 3).
# Convencoes conferidas: NewsItem::NewsItem 0x77bbe0 (thiscall, ret 8), NewsItem::set_slot 0x785790 (thiscall, ret 8),
# News::send_to_club_manager 0x7861e0 (ecx = [0xb7431c]; thiscall, ret 8). Prologo das noticias push -1; push imm32
# (7 bytes; o quadro SEH so cobre o desenrolar de excecao do NewsItem local). Iguais no cm0102-1080.exe (inteiras).
# Comparadores: as noticias so a original (enviam; nao podem rodar duas vezes); o construtor lado a lado (a original no
# registro, a copia num local de 16 bytes: os 8 bytes da data e a devolucao).
SAVE = False
ROTINAS = [('finance_post_receivers_appeased_news', 0x5a02f0, [0x6a, 0xff, 0x68, 0x8b, 0x91, 0x95, 0x00]),
           ('finance_post_ground_upgrade_plans_news', 0x5a0410, [0x6a, 0xff, 0x68, 0xab, 0x91, 0x95, 0x00]),
           ('new_fixed_fixture', 0x645800, [0x56, 0x8b, 0xf1, 0xe8, 0x08, 0x49, 0xf0, 0xff])]
DOCS = ('as notícias da saída da crise e dos planos de ampliar o estádio e o construtor dos registros que começam com uma data',
        'the receivers appeased and ground upgrade plans news and the constructor of the Date-led records',
        [('finance_post_receivers_appeased_news', 'finance_post_receivers_appeased_news', '0x5a02f0', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 4',
          'envia ao técnico do clube a notícia de que os credores estão satisfeitos e o clube volta a poder recusar propostas',
          'sends the club\'s manager the news that the bankers are satisfied and the club may refuse offers again'),
         ('finance_post_ground_upgrade_plans_news', 'finance_post_ground_upgrade_plans_news', '0x5a0410', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0xc',
          'envia ao técnico do clube a notícia de que a diretoria planeja ampliar ou reformar o estádio para disputar uma competição, com a competição e o tipo',
          'sends the club\'s manager the news that the board plans to enlarge or redevelop the ground to enter a competition, with the competition and the kind'),
         ('new_fixed_fixture', 'FixedFixture::FixedFixture', '0x645800', 'ita_ser_a.cpp', 'thiscall, ret 0',
          'constrói um registro que começa com uma data (o jogo fixo e outros): a data padrão do jogo e devolve o registro',
          'constructs a record that starts with a date (the fixed fixture and others): the game\'s default date, and returns the record')])
CAMPOS = []
INTERESSA = []
