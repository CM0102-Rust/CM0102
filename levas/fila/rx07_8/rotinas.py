# A notícia de que um jogador quer garantias e os textos da proposta e do teste cancelados (contract_manager.cpp;
# leitor rx07, rodada 6, 25/11/2026). Simulação, carreira ENG (sem LIGAS), no molde do cpw_1 e dos escritores de
# texto do contract_manager. Contagem 0 na carreira automática e no mundo: a notícia sai pelo dia das pessoas
# (game_daily_staff_update 0x5ce2d0, original) só em condições de técnico humano, e os textos só quando a caixa de um
# técnico humano mostra a notícia; a prova é a comparação numa carreira do Lucas. Iguais no cm0102-1080.exe.
# Convenções conferidas na desmontagem:
# - 0x4d9840 CONTRACT_MANAGER::post_seeks_assurances_news thiscall(contrato) ret 4: sem contrato a caixa "Error"
#   (linha 0x2c36, [0xb67a34] = 0); TRANSFER_MANAGER 0x8c1870 is_queued_for_free_transfer thiscall(pessoa) ret 4,
#   0x8c5cf0 get_offers_for_player thiscall(pessoa, 0, 0) ret 0xc, 0x8c5e40 get_exchange_offers_for_player
#   thiscall(pessoa, 0) ret 8; a busca do contrato principal grava [0xae1080]; a notícia 0xbc1 (slot 1 não é escrito).
#   Prólogo mov eax, fs:[0] (6).
# - 0x4d65c0 CONTRACT_MANAGER::write_offer_cancelled_no_space_news_text thiscall(notícia, completo, texto) ret 0xc:
#   get_slot 0x785830 thiscall(k); database_format_name_from_ids 0x53e140 cdecl(s1, s2, s3, buf, 0x12c, modo, pessoa)
#   (modo 1 no corpo, 5 na manchete); langlib_sprintf 0x66f4e0 em 0xde1f64; Text::set_string 0x4ae660;
#   Text::add_link 0x4ae8a0 thiscall(texto, link, tipo, objeto) ret 0xc; database_staff_matches_names 0x53e990
#   cdecl(pessoa, s1, s2, s3, 0). Prólogo sub esp, 0x130 (6).
# - 0x4daec0 CONTRACT_MANAGER::write_trial_cancelled_news_text, a mesma convenção; o nome sempre no modo 1; o corpo com
#   o gênero do nome do clube (+0x37) quatro vezes e o nome completo (+4) duas. Prólogo mov eax, [esp+8]; sub esp,
#   0x130 (10).
# Comparadores: a notícia, só a original (vai à caixa do técnico); os textos lado a lado (compare_text_writer).
SAVE = False
ROTINAS = [('post_seeks_assurances_news', 0x4d9840, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('write_offer_cancelled_no_space_news_text', 0x4d65c0, [0x81, 0xec, 0x30, 0x01, 0x00, 0x00]),
           ('write_trial_cancelled_news_text', 0x4daec0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x30, 0x01, 0x00, 0x00])]
_F = 'contract_manager.cpp'
DOCS = ('a notícia de que um jogador quer garantias e os textos da proposta e do teste cancelados',
        'the news that a player seeks assurances and the texts of the cancelled offer and trial',
        [('post_seeks_assurances_news', 'CONTRACT_MANAGER::post_seeks_assurances_news', '0x4d9840', _F, 'thiscall, ret 4',
          'um jogador pede garantias sobre o futuro: a notícia 0xbc1 ao técnico do clube do contrato, menos quando ele está na fila de transferência livre, tem propostas, está numa troca ou o contrato principal é do tipo 5; com o clube, a pessoa e os nomes, a situação de transferência duas vezes e a idade; sem contrato, a caixa de erro',
          "a player seeks assurances about his future: news item 0xbc1 to his contract club's manager, unless he is queued for a free transfer, has offers, is in an exchange or his main contract is of type 5; with the club, the person and his names, the transfer status twice and the age; without a contract, the error box"),
         ('write_offer_cancelled_no_space_news_text', 'CONTRACT_MANAGER::write_offer_cancelled_no_space_news_text', '0x4d65c0', _F, 'thiscall, ret 0xc',
          'o texto da notícia 0xbc4: o corpo com o nome curto do clube e o nome da pessoa, ligado ao clube; a manchete "<nome> proposta de contrato cancelada"; e o vínculo da pessoa quando os nomes ainda batem',
          'the text of news item 0xbc4: the body with the club\'s short name and the person\'s name, linked to the club; the headline "<name> contract offer cancelled"; and the person\'s link when the names still match'),
         ('write_trial_cancelled_news_text', 'CONTRACT_MANAGER::write_trial_cancelled_news_text', '0x4daec0', _F, 'thiscall, ret 0xc',
          'o texto da notícia 0xbbc: o corpo com o nome da pessoa e o nome do clube, ligado ao clube; a manchete "<nome> teste cancelado"; e o vínculo da pessoa quando os nomes ainda batem',
          'the text of news item 0xbbc: the body with the person\'s name and the club\'s name, linked to the club; the headline "<name> trial cancelled"; and the person\'s link when the names still match')])
CAMPOS = []
INTERESSA = []
