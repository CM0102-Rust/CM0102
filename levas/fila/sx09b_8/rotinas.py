# O texto do aviso da diretoria sobre multar demais (write_board_fine_warning_news_text 0x5a7ed0, Fine.cpp, 72 bytes ate
# o ret 0xc de 0x5a7f15; leitor sx09b, rodada 5, 23/11/2026). Chamada so pelo texto das noticias (0x77bc80, ja Rust,
# NEWS_WRITERS: categoria 6, tipo 0x1784, objeto [0xae2a3c], de 0x781815).
# Convencoes conferidas na desmontagem: thiscall(Fine, noticia, corpo, Text), ret 0xc; o ecx nao e lido (vira a noticia
# para o 0x785830 NewsItem::get_slot thiscall(noticia, 0) ret 4); o modelo por 0x66f4e0 langlib_sprintf
# cdecl(0xde1f64, modelo) e 0x4ae660 Text::set_string thiscall(Text, 0xde1f64) ret 4.
# Prologo: mov eax,[esp+8]; test eax,eax (6 bytes, sem salto).
# Comparador: compare_text_writer (texto, vinculos e gerador).
# A noticia 0x1784 so nasce quando um tecnico humano multa os jogadores demais (a contagem de multas por clube, ver
# sx09b_2): na carreira de ferias ninguem multa, entao a rotina provavelmente nao roda ate 2002-06-01.
SAVE = False
ROTINAS = [('write_board_fine_warning_news_text', 0x5a7ed0, [0x8b, 0x44, 0x24, 0x08, 0x85, 0xc0])]
DOCS = ('o texto do aviso da diretoria sobre multar demais',
        "the text of the board's warning about fining players too often",
        [('write_board_fine_warning_news_text', 'write_board_fine_warning_news_text', '0x5a7ed0', 'Fine.cpp', 'thiscall, ret 0xc',
          'escreve a manchete ("a diretoria manifesta preocupacao") ou o corpo da noticia 6020 (0x1784): com a vaga 0 ligada, o primeiro aviso (as multas frequentes vao abalar o moral do elenco); desligada, o ultimo (a diretoria pode procurar um novo tecnico)',
          'writes the headline ("the board make concerns known") or the body of news item 6020 (0x1784): with slot 0 set, the first warning (frequent fines will damage the squad\'s morale); clear, the last one (the board may look for a new manager)')])
