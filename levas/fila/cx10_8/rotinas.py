# Os textos das noticias dos dividendos, das obras do estadio e da mudanca de estadio (fin_prm.cpp pelo mapa,
# finance.CPP pelos asserts; leitor cx10, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS). Escritores de texto: rodam
# quando a noticia e aberta e, com CM_TEXTO_NOTICIAS=1 (CM_TEXTOS=1 na comparacao da cadeia), quando ela e enviada; so a
# comparacao as prova. Chamados pelo NewsItem::write_text (0x7816e5) pelos tipos 0x177c, 0x177d e o da mudanca.
# - 0x59feb0 FinanceTable::write_dividend_news_text thiscall(noticia, corpo, saida) ret 0xc (o ecx nao e lido; quadro
#   SEH de _chkstk 0x1774 so para o desenrolar): corpo pelos slots 1 (valor) e 2 (sociedade anonima, byte); o valor por
#   Cash::get_display_amount 0x43f460 (lea ecx, &valor; push 4 -> thiscall, ret 4; o primeiro push 4 fica como quarto
#   argumento) e cash_format_money 0x441c10 cdecl(buf, 0x3e8, valor64, 4). Os dois ramos do byte da sociedade anonima
#   chamam a mesma linha 0x9c96b8.
# - 0x5a0050 FinanceTable::write_stadium_works_news_text: titulo pelo slot 2 (tabela de saltos 0x5a01f8, tipos 4..8);
#   corpo pelos slots 1 (numero ou id do estadio) e 2 (tabela 0x5a020c, tipos 1..8); o estadio pela tabela [0xae23b0],
#   0x4e bytes, nome em +4 e genero em +0x37.
# - 0x5a0230 FinanceTable::write_club_relocation_news_text: corpo pelos slots 0 (clube), 1 (lido e sem uso) e 2
#   (estadio); a linha com os generos do clube e do estadio antes dos dois nomes.
# Convencoes conferidas: NewsItem::get_slot 0x785830 (thiscall, ret 4), langlib_sprintf 0x66f4e0 (cdecl),
# Text::set_string 0x4ae660 (thiscall, ret 4). Prologo mov eax, 0x1388 (5 bytes) ou mov eax, fs:[0] (6 bytes, a
# 0x59feb0). Iguais no cm0102-1080.exe (inteiras).
# Defeito do original: as obras do estadio com tipo fora de 1..8 no corpo nao formatam nada (o texto fica com o lixo da
# pilha; a copia o deixa vazio).
# Comparadores: compare_text_writer (a original no Text de verdade, a copia num local).
# Depende do cx10_7 (finance_news_text, club_long_name, stadium_name): integrar depois dele.
SAVE = False
ROTINAS = [('write_dividend_news_text', 0x59feb0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('write_stadium_works_news_text', 0x5a0050, [0xb8, 0x88, 0x13, 0x00, 0x00]),
           ('write_club_relocation_news_text', 0x5a0230, [0xb8, 0x88, 0x13, 0x00, 0x00])]
DOCS = ('os textos das notícias dos dividendos, das obras do estádio e da mudança de estádio',
        'the texts of the dividend, stadium works and club relocation news',
        [('write_dividend_news_text', 'FinanceTable::write_dividend_news_text', '0x59feb0', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0xc',
          'escreve a notícia dos dividendos aos acionistas: o total anunciado pela diretoria, em dinheiro como o jogo mostra',
          'writes the shareholder dividend news: the total the board announces, in money as the game shows it'),
         ('write_stadium_works_news_text', 'FinanceTable::write_stadium_works_news_text', '0x5a0050', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0xc',
          'escreve a notícia das obras do estádio pelo tipo: ampliação, geral, cadeiras, compra, venda, aquecimento do gramado, aquecimento quebrado ou centro de treino',
          'writes the stadium works news by the kind: expansion, standing, seats, purchase, sale, undersoil heating, failed heating or training ground'),
         ('write_club_relocation_news_text', 'FinanceTable::write_club_relocation_news_text', '0x5a0230', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0xc',
          'escreve a notícia da mudança do clube para o estádio novo, com os dois nomes',
          'writes the news of the club\'s move to its new stadium, with both names')])
CAMPOS = []
INTERESSA = []
