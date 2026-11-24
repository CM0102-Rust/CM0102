# O texto da noticia de financas da diretoria (fin_prm.cpp pelo mapa, finance.CPP pelos asserts; leitor sbA, rodada
# 5, 24/11/2026). Carreira ENG (sem LIGAS). Escritor de texto: roda quando a noticia e aberta e, com CM_TEXTO_NOTICIAS=1
# (CM_TEXTOS=1 na comparacao da cadeia), quando ela e enviada; so a comparacao o prova. Chamado pelo
# NewsItem::write_text pelo tipo 0x1779 (news.rs: (6, 0x1779, Global(0xae2a38, 0x59f3e0))).
# - 0x59f3e0 FinanceTable::write_finance_boost_news_text thiscall(noticia, corpo, saida) ret 0xc (o ecx nao e lido):
#   manchete pelos slots 0 (clube), 1 (primeira quantia) e 3 (tipo, tabela de saltos 0x59f8c8: 1 verba de
#   transferencias, 2 nova diretoria com a ligacao do nome curto +0x38/+0x52, 3 e 4 injecao com a quantia, 5 acoes, 6
#   patrocinio, 7 virou plc com a ligacao); corpo pelos slots 0 a 3 (tabela 0x59f8e4), as duas quantias como dinheiro
#   (Cash::get_display_amount 0x43f460 com 4, cash_format_money 0x441c10 em 1000 bytes), o limite reputacao² × 0,025
#   (x87, 0x967878, _ftol) para o tipo 1 e 2, a ligacao do nome longo (+4/+0x37) nos tipos 2 e 7.
# Convencoes conferidas: NewsItem::get_slot 0x785830 (thiscall, ret 4), langlib_sprintf 0x66f4e0 (cdecl, os generos
# antes do nome, a quantia depois), Cash::get_display_amount (thiscall, ret 4: o 4 de cima fica para o
# cash_format_money, cdecl de 5), Text::add_link 0x4ae8a0 (thiscall, ret 0xc), Text::set_string 0x4ae660 (thiscall, ret
# 4). Buffer local do texto de 0x1388 bytes (quadro de 0x1b68, _chkstk). Prologo mov eax, fs:[0] (6 bytes). Igual no
# cm0102-1080.exe (inteira).
# Defeito do original: um tipo fora de 1 a 7 nao formata nada e o texto fica com o lixo da pilha (o jogo so posta 1 a
# 7); a copia o deixa vazio.
# Comparador: compare_text_writer (a original no Text de verdade, a copia num local).
# Depende do cx10_7 (FINANCE_TEXT_SIZE e club_long_name): integrar o cx10_7 antes.
SAVE = False
ROTINAS = [('write_finance_boost_news_text', 0x59f3e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o texto da notícia de finanças da diretoria',
        "the text of the board's finance news",
        [('write_finance_boost_news_text', 'FinanceTable::write_finance_boost_news_text', '0x59f3e0', 'fin_prm.cpp (finance.CPP)', 'thiscall, ret 0xc',
          'escreve a notícia de finanças da diretoria pelo tipo: verba de transferências, nova diretoria, injeção de dinheiro, emissão de ações, patrocínio ou abertura de capital (plc); no corpo, as duas quantias como dinheiro e, na verba e na nova diretoria, a segunda quantia comparada com reputação² × 0,025 e com 50 mil; a ligação do clube na nova diretoria e na plc',
          "writes the board's finance news by kind: transfer funds, a new board, a cash injection, a share issue, a sponsorship or the PLC flotation; in the body, both amounts as money and, for the funds and the new board, the second amount checked against reputation² × 0.025 and 50k; the club's link for the new board and the PLC")])
CAMPOS = []
INTERESSA = []
