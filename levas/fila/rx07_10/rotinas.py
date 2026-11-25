# O texto da notícia de que um jogador quer garantias (CONTRACT_MANAGER::write_seeks_assurances_news_text 0x4d9a90,
# contract_manager.cpp, 1.264 bytes; leitor rx07, rodada 6, 25/11/2026). Simulação, carreira ENG (sem LIGAS), no
# molde dos escritores de texto do contract_manager. Contagem 0 na carreira automática e no mundo: o texto só é
# montado quando a caixa de um técnico humano mostra a notícia 0xbc1; a prova é a comparação numa carreira do Lucas.
# Igual no cm0102-1080.exe. Vai de 0x4d9a90 ao ret 0xc de 0x4d9ecc; as seis tabelas de salto de 7 entradas
# (0x4d9ed0, 0x4d9eec, 0x4d9f08, 0x4d9f24, 0x4d9f40, 0x4d9f5c) vêm logo depois (as duas últimas iguais).
# Convenções conferidas na desmontagem: thiscall(notícia, completo, texto) ret 0xc; get_slot 0x785830 (os slots 0 e 2
# a 11 lidos antes de tudo; o 11 não é usado); database_format_name_from_ids 0x53e140 (modo 1 no corpo, 5 na
# manchete); langlib_sprintf 0x66f4e0 em 0xde1f64; Text::set_string 0x4ae660; Text::append_string 0x4ae770
# thiscall(texto, buf) ret 4; Text::add_link 0x4ae8a0; database_staff_matches_names 0x53e990. Prólogo sub esp, 0x148
# (6).
# Nota: o post_seeks_assurances_news (rx07_8) grava 0 nos slots 6, 7 e 10; com ele, só a primeira frase sai.
# Comparador: lado a lado (compare_text_writer).
SAVE = False
ROTINAS = [('write_seeks_assurances_news_text', 0x4d9a90, [0x81, 0xec, 0x48, 0x01, 0x00, 0x00])]
_F = 'contract_manager.cpp'
DOCS = ('o texto da notícia de que um jogador quer garantias',
        'the text of the news that a player seeks assurances',
        [('write_seeks_assurances_news_text', 'CONTRACT_MANAGER::write_seeks_assurances_news_text', '0x4d9a90', _F, 'thiscall, ret 0xc',
          'o texto da notícia 0xbc1: o corpo "<nome> quer garantias imediatas sobre o seu futuro." e, com mais a dizer e uma situação nova, a frase do papel no elenco (1 a 7) pela situação de transferência antiga e nova e a frase de fecho, com o clube ligado; a manchete "<nome> quer garantias"; e o vínculo da pessoa quando os nomes ainda batem',
          'the text of news item 0xbc1: the body "<name> is seeking immediate assurances regarding his future." and, with more to say and a new status, the squad role sentence (1 to 7) by the old and new transfer status and the closing sentence, with the club linked; the headline "<name> seeks assurances"; and the person\'s link when the names still match')])
CAMPOS = []
INTERESSA = []
