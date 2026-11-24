# Os textos das noticias da saida da crise e dos planos de ampliar o estadio (fin_prm.cpp pelo mapa, finance.CPP pelos
# asserts; leitor cx10, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS). Escritores de texto: rodam quando a noticia e
# aberta e, com CM_TEXTO_NOTICIAS=1 (CM_TEXTOS=1 na comparacao da cadeia), quando ela e enviada; so a comparacao as
# prova. Chamados pelo NewsItem::write_text (0x7816e5) pelos tipos 0x1783 e 0x1780.
# - 0x5a0380 FinanceTable::write_receivers_appeased_news_text thiscall(noticia, corpo, saida) ret 0xc (o ecx nao e
#   lido): corpo pelo slot 0 (clube, nome longo e genero).
# - 0x5a04c0 FinanceTable::write_ground_upgrade_plans_news_text: corpo pelos slots 1 (competicao de clubes, a tabela
#   [0xae23d0], 0x6b bytes, nome em +4 e genero em +0x37) e 2 (tipo 0 reforma, 1 ampliacao, byte com sinal).
# Convencoes conferidas: NewsItem::get_slot 0x785830 (thiscall, ret 4), langlib_sprintf 0x66f4e0 (cdecl),
# Text::set_string 0x4ae660 (thiscall, ret 4). Buffer local de 0x1388 bytes. Prologo mov eax, 0x1388 (5 bytes). Iguais
# no cm0102-1080.exe (inteiras).
# Defeito do original: os planos do estadio com tipo fora de 0/1 nao formatam nada (o texto fica com o lixo da pilha; a
# copia o deixa vazio; os dois chamadores na 0x599e00 passam 0 ou 1).
# Comparadores: compare_text_writer (a original no Text de verdade, a copia num local).
# Depende do cx10_7 (finance_news_text, club_long_name): integrar depois dele.
SAVE = False
ROTINAS = [('write_receivers_appeased_news_text', 0x5a0380, [0xb8, 0x88, 0x13, 0x00, 0x00]),
           ('write_ground_upgrade_plans_news_text', 0x5a04c0, [0xb8, 0x88, 0x13, 0x00, 0x00])]
DOCS = ('os textos das notícias da saída da crise e dos planos de ampliar o estádio',
        'the texts of the receivers appeased and ground upgrade plans news',
        [('write_receivers_appeased_news_text', 'FinanceTable::write_receivers_appeased_news_text', '0x5a0380', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0xc',
          'escreve a notícia de que os credores do clube o deixam de novo recusar e negociar propostas',
          'writes the news that the club\'s bankers let it refuse and negotiate offers again'),
         ('write_ground_upgrade_plans_news_text', 'FinanceTable::write_ground_upgrade_plans_news_text', '0x5a04c0', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0xc',
          'escreve a notícia dos planos da diretoria de reformar (tipo 0) ou ampliar (tipo 1) o estádio para disputar a competição',
          'writes the news of the board\'s plans to redevelop (kind 0) or enlarge (kind 1) the ground to enter the competition')])
CAMPOS = []
INTERESSA = []
