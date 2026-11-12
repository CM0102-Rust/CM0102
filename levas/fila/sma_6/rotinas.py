# Os textos das expectativas e da confiança da diretoria (manager_manager.cpp; leitor sma, 12/11/2026). Chamados pelas
# notícias (0x77bc80, news.cpp, o texto de cada tipo); a 0x694440 também pela 0x6a5120 (original). Convenções:
# - 0x694440: thiscall(this ignorado, clube, marcas, buffer, tamanho), ret 0x10; o tamanho não é usado;
# - 0x6a4ae0 e 0x6a4d40: thiscall(this, notícia, corpo, Text), ret 0xc;
# - 0x785830 (newsitem_get_slot): push k; ecx = notícia -> thiscall; 0x66f4e0 (formato da língua): cdecl variádica;
# - 0x4ae660 (text_set_string): push texto; ecx = Text -> thiscall; 0x4ae8a0 (text_add_link): push dado; push tipo;
#   push texto; ecx = Text -> thiscall;
# - 0x690d80: push 0x1388; push buf; push marcas; push clube; ecx = this -> thiscall (original); 0x693850: push &ids;
#   push clube; push buf; ecx = this -> thiscall, al = quantos (original);
# - 0x53dba0 (nome): push 1; push 0x96; push buf; push pessoa -> cdecl; 0x945a60 (strstr) e 0x944c9f (sprintf): cdecl;
# - a virtual +0x98 do objeto da competição da divisão: mov ecx, objeto; call [vtable + 0x98].
# Comparadores: compare_text_writer (os dois textos); a 0x694440 lado a lado no buffer.
SAVE = False
ROTINAS = [('build_board_expectation_text', 0x694440, [0x8b, 0x44, 0x24, 0x08, 0x56]),
           ('news_board_expectations_text', 0x6a4ae0, [0xb8, 0x88, 0x13, 0x00, 0x00]),
           ('news_board_confidence_update_text', 0x6a4d40, [0xb8, 0xe8, 0x27, 0x00, 0x00])]
DOCS = ('os textos das expectativas e da confiança da diretoria',
        'the board\'s season expectations and confidence texts',
        [('build_board_expectation_text', 'MANAGER_MANAGER::build_board_expectation_text', '0x694440', 'manager_manager.cpp', 'thiscall, ret 0x10',
          'escreve a frase das expectativas da diretoria pelas marcas: título (1, com versão para fora da Europa), alto da tabela (2, pela divisão de cima), e as marcas 4, 8, 0x40, 0x10 e 0x20, cada uma por 0x400 / 0x100 / nenhuma e por 0x800; sem nenhuma delas e sem 0x20, "melhorar o elenco"; sempre com o gênero e o nome do clube',
          'writes the board\'s expectation sentence by the flags: title (1, with a version outside Europe), top of the table (2, by the top division), and flags 4, 8, 0x40, 0x10 and 0x20, each by 0x400 / 0x100 / neither and by 0x800; with none of them and no 0x20, "improve the squad"; always with the club\'s gender and name'),
         ('news_board_expectations_text', 'MANAGER_MANAGER::news_board_expectations_text', '0x6a4ae0', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'o título ou o corpo da notícia das expectativas: o corpo pela frase acima; a manchete pela seleção ("F.A. Announce Expectations") ou pelas marcas (glória, título, temporada de sucesso, difícil, ansiosa, ou só "anuncia"); o vínculo do clube nos dois',
          'the headline or body of the expectations news: the body by the sentence above; the headline by the national team ("F.A. Announce Expectations") or by the flags (glory, title, successful season, difficult, looking forward, or just "announce"); the club\'s link in both'),
         ('news_board_confidence_update_text', 'MANAGER_MANAGER::news_board_confidence_update_text', '0x6a4d40', 'manager_manager.cpp', 'thiscall, ret 0xc',
          'o título ou o corpo da atualização de confiança: manchete da federação ou da diretoria; corpo com a frase de confiança e, se houver, a dos jogadores citados (até 15) depois de uma linha em branco; vínculos para cada jogador citado cujo nome aparece no texto e para o clube',
          'the headline or body of the confidence update: the FA\'s or the board\'s headline; body with the confidence sentence and, if any, the named players\' sentence (up to 15) after a blank line; links for each named player whose name is in the text and for the club')])
