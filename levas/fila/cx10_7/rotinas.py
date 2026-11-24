# Os textos das noticias do setor interditado, das obras canceladas e da crise financeira (fin_prm.cpp pelo mapa,
# finance.CPP pelos asserts; leitor cx10, rodada 5, 24/11/2026). Carreira ENG (sem LIGAS). Escritores de texto: rodam
# quando a noticia e aberta e, com CM_TEXTO_NOTICIAS=1 (CM_TEXTOS=1 na comparacao da cadeia), quando ela e enviada; so a
# comparacao as prova. Chamados pelo NewsItem::write_text (0x7816e5) pelos tipos 0x177a, 0x1788 e 0x177b.
# - 0x59f9c0 FinanceTable::write_stand_closed_news_text thiscall(noticia, corpo, saida) ret 0xc (o ecx nao e lido):
#   titulo pelo slot 3; corpo pelos slots 1 (estadio, -1 sem), 2 (lugares), 3 (tipo, byte com sinal); a tabela de
#   estadios [0xae23b0], 0x4e bytes, +0x40 os lugares sentados, +0x3c a capacidade.
# - 0x59fb90 FinanceTable::write_works_cancelled_news_text: slots 1 (assunto: 1..4 e 6 o estadio) e 2 (motivo), lidos
#   antes do titulo.
# - 0x59fcf0 FinanceTable::write_financial_crisis_news_text: titulo pelo slot 1 (palavra); corpo pelos slots 0 (clube) e
#   1 (redacao 0/1); a ligacao do clube (Text::add_link 0x4ae8a0, nome longo, 0x7d5) antes do Text::set_string.
# Convencoes conferidas: NewsItem::get_slot 0x785830 (mov ecx, noticia; push k -> thiscall, ret 4), langlib_sprintf
# 0x66f4e0 (cdecl, os generos antes dos nomes), Text::set_string 0x4ae660 (thiscall, ret 4), Text::add_link 0x4ae8a0
# (thiscall, ret 0xc). Buffer local de 0x1388 bytes (_chkstk 0x945610). Prologo mov eax, 0x1388 (5 bytes). Iguais no
# cm0102-1080.exe (inteiras).
# Defeitos do original: sem estadio (-1) o corpo do setor interditado le os lugares do registro anterior a tabela
# ([0xae23b0] - 0xe) e imprime o tipo (1) como lugares restantes; a crise com redacao fora de 0/1 nao formata nada (o
# texto fica com o lixo da pilha; a copia o deixa vazio; a redacao e um sorteio de 2, entao nao acontece).
# Comparadores: compare_text_writer (a original no Text de verdade, a copia num local).
# Os auxiliares finance_news_text, club_long_name e stadium_name ficam aqui; o cx10_8 e o cx10_9 os usam (integrar
# este antes deles).
SAVE = False
ROTINAS = [('write_stand_closed_news_text', 0x59f9c0, [0xb8, 0x88, 0x13, 0x00, 0x00]),
           ('write_works_cancelled_news_text', 0x59fb90, [0xb8, 0x88, 0x13, 0x00, 0x00]),
           ('write_financial_crisis_news_text', 0x59fcf0, [0xb8, 0x88, 0x13, 0x00, 0x00])]
DOCS = ('os textos das notícias do setor interditado, das obras canceladas e da crise financeira',
        'the texts of the stand closed, works cancelled and financial crisis news',
        [('write_stand_closed_news_text', 'FinanceTable::write_stand_closed_news_text', '0x59f9c0', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0xc',
          'escreve a notícia do setor interditado: na manchete, arquibancada ou geral fechada; no corpo, pelo estádio e pelo tipo, a arquibancada oeste com os lugares a menos, a norte sem lugares ou a geral leste com a capacidade que sobra',
          'writes the closed stand news: in the headline, stand or terracing closed; in the body, by the stadium and the kind, the west stand with the seats lost, the north stand with no seats left or the east terrace with the capacity left'),
         ('write_works_cancelled_news_text', 'FinanceTable::write_works_cancelled_news_text', '0x59fb90', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0xc',
          'escreve a notícia das obras suspensas, canceladas ou barradas pela prefeitura: o estádio (assuntos 1 a 4 e 6) pelo motivo, ou o centro de treino',
          'writes the news of works shelved, cancelled or refused by the council: the stadium (subjects 1 to 4 and 6) by the reason, or the training ground'),
         ('write_financial_crisis_news_text', 'FinanceTable::write_financial_crisis_news_text', '0x59fcf0', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0xc',
          'escreve a notícia da crise financeira: na manchete, credores chamados ou clube em crise; no corpo, a reunião de emergência da diretoria ou os credores nomeados, com a ligação do clube',
          'writes the financial crisis news: in the headline, receivers called in or club in crisis; in the body, the emergency board meeting or the receivers appointed, with the club\'s link')])
CAMPOS = []
INTERESSA = []
