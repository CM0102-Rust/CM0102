# O texto da notícia de que um jogador quer um contrato melhor (CONTRACT_MANAGER::write_wants_new_contract_news_text
# 0x4da190, contract_manager.cpp, 1.024 bytes; leitor rx07, rodada 6, 25/11/2026). Simulação, carreira ENG (sem LIGAS),
# no molde dos escritores de texto do contract_manager. Contagem 0 na carreira automática e no mundo: o texto só é
# montado quando a caixa de um técnico humano mostra a notícia 0xbc2 (post_wants_new_contract_news, já Rust); a prova
# é a comparação numa carreira do Lucas. Igual no cm0102-1080.exe. Vai de 0x4da190 ao ret 0xc de 0x4da584.
# Convenções conferidas na desmontagem: thiscall(notícia, completo, texto) ret 0xc, quadro SEH (só a Date local);
# prólogo push -1; push 0x956fcb (7), sem relativas. get_slot 0x785830 (0, 2 a 7 e 11 antes de tudo; 9, 8 e 10 só no
# corpo com o slot 7 limpo); Date::Date 0x549ef0 thiscall(dia, ano) ret 8; o contrato principal pelo
# [G_CONTRACT_MANAGER]; LLIST::get_next 0x68d1b0 (o cursor fica depois da proposta achada); News::set_item_slot
# 0x786d30 thiscall([0xb7431c], [0xb74c26 + vaga × 0x300], [notícia + 0xd6], 7, 1, notícia) ret 0x14;
# database_format_name_from_ids 0x53e140 (modo 1 no corpo, 5 na manchete); langlib_sprintf 0x66f4e0; Text::set_string
# 0x4ae660; Text::add_link 0x4ae8a0; database_staff_matches_names 0x53e990.
# Comparador: lado a lado (compare_text_writer); a original roda primeiro e a marca dela na caixa (o slot 7 = 1, o
# mesmo valor) faz a cópia pular a marca.
SAVE = False
ROTINAS = [('write_wants_new_contract_news_text', 0x4da190, [0x6a, 0xff, 0x68, 0xcb, 0x6f, 0x95, 0x00])]
_F = 'contract_manager.cpp'
DOCS = ('o texto da notícia de que um jogador quer um contrato melhor',
        'the text of the news that a player wants a better contract',
        [('write_wants_new_contract_news_text', 'CONTRACT_MANAGER::write_wants_new_contract_news_text', '0x4da190', _F, 'thiscall, ret 0xc',
          'o texto da notícia 0xbc2: no corpo, a notícia que já não vale (o contrato principal mudou de salário ou de fim, ou o clube já fez uma proposta) fica marcada na caixa do técnico humano; o texto pela situação no elenco (1, 2, 3 ou outra) e por quem pede (o jogador ou o agente), com o clube ligado; a manchete "<nome> quer contrato novo"; e o vínculo da pessoa quando os nomes ainda batem',
          'the text of news item 0xbc2: in the body, a news item no longer current (the main contract changed its wage or end, or the club already made an offer) is marked in the human manager\'s inbox; the text by the squad status (1, 2, 3 or other) and by who asks (the player or his agent), with the club linked; the headline "<name> wants new contract"; and the person\'s link when the names still match')])
CAMPOS = []
INTERESSA = []
