# O texto da notícia da transferência acertada (TRANSFER_MANAGER::write_transfer_agreed_news_text 0x8e08e0,
# transfer_manager.cpp, 2.160 bytes, 0x8e08e0..0x8e1146; leitor rx05, rodada 6, 25/11/2026). Igual no
# cm0102-1080.exe. Contagem 0 na carreira ENG; 59 chamadas no mundo até 2002-08-01 (docs/cobertura_mundo.txt).
# Usa os ajudantes de texto já no motor (news_text_person_name, news_text_money, news_text_link_club,
# news_text_link_person, news_text_append, news_text_format) e define NewsPlayerList (os jogadores da troca de uma
# notícia e o texto da lista), usado também por outros itens rx05.
# Convenções conferidas na desmontagem: thiscall(this, notícia, corpo, Text) ret 0xc, quadro SEH só para o estado
# das quantias; 0x785830 get_slot ret 4; 0x53e140 cdecl(n1, n2, n3, buf, 0x12c, modo, id); 0x66f4e0 cdecl
# (langlib_sprintf: os argumentos dos "{}" vêm antes dos "%s"; o 0x64 é o marcador depois do nome da pessoa);
# 0x90c520 cdecl(n, buf, 0x32, 1); 0x549ef0 Date::Date(dia, ano) ret 8; 0x54ba20 thiscall(data, buf, 0xc8, 0xff)
# ret 0xc; 0x43f460 thiscall(&valor, 6) ret 4 e 0x441c10 cdecl(buf, 0x32, valor i64, 6) (news_text_money);
# 0x4ae660/0x4ae770/0x4ae8a0 os métodos do Text; 0x53e990 cdecl(pessoa, n1, n2, n3, 0).
# As quantias das vagas 6 e 8 são lidas como float (fild/fstp) e comparadas em float; o _ftol trunca.
# Comparador: lado a lado do texto e dos vínculos (compare_text_writer).
SAVE = False
ROTINAS = [('write_transfer_agreed_news_text', 0x8e08e0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o texto da notícia da transferência acertada',
        'the news text of an agreed transfer',
        [('write_transfer_agreed_news_text', 'TRANSFER_MANAGER::write_transfer_agreed_news_text', '0x8e08e0', 'transfer_manager.cpp', 'thiscall, ret 0xc',
          'o título "<comprador> vai contratar <nome>" ou o corpo: com o valor total acima da parte em dinheiro, "num negócio de <valor>" (mais os jogadores da troca); senão, com dinheiro, "por <dinheiro>" (mais os jogadores); sem dinheiro, a troca de jogadores ou só os clubes (com ou sem o vendedor); depois "a transferência sai em <data>" e os vínculos ao comprador, ao vendedor, à pessoa e aos jogadores da troca nomeados',
          'the headline "<buyer> to sign <name>" or the body: with the total value above the cash part, "in a deal worth <value>" (plus the exchange players); otherwise, with cash, "for <cash>" (plus the players); without cash, the player swap or just the clubs (with or without the seller); then "the transfer will go through on <date>" and the links to the buyer, the seller, the person and the named exchange players')])
