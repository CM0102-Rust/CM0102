# As miudezas das telas (background.cpp, comp_text.cpp, comp_screens.cpp, contract_screens.cpp, match_screens.cpp,
# search_screens.cpp; leitor tel39, rodada 2 das telas, 22/11/2026). Item de varios arquivos: as sobras de cada um
# sao de 1 a 3 rotinas.
# Convencoes conferidas na desmontagem e nos pontos de chamada:
# - 0x41bad0 background_free_config_tables cdecl(); chamada pela 0x5d7b70 (game_config.cpp, a limpeza: game_fatal_cleanup
#   0x5cc80a e a caixa das opcoes 0x825a6e/0x825acf); free da CRT 0x9452ca.
# - 0x41bf80 background_set_pictures_disabled cdecl(ligado) e 0x41bfa0 background_pictures_disabled cdecl() -> eax:
#   o global 0xdd6bda (a foto fixa do Display::draw_screen); chamadas pelo Config::Config (0x5d7aa7, na partida do
#   programa) e pelo Config::apply_and_save (0x5d87c5..0x5d87e2, a caixa das opcoes do jogo).
# - 0x497910 LLIST::scalar_deleting_destructor thiscall(lista, opcoes) ret 4 -> this; chamada pelo clique da tela da
#   competicao (0x496c6e...) e pelo Comp_Stats::build_season_records_list (0x4ab599). PROLOGO COM CALL: push esi;
#   mov esi, ecx; call 0x68d040 -> precisa de (0x497910, 3) no RELATIVAS do rust/src/detours.rs.
# - 0x4ae5c0 Text::Text (copia) thiscall(this, outro*) ret 4 -> this; so a confirm_push_text_object (file_screens.cpp,
#   0x591a1b, a caixa de confirmacao com Text) o chama.
# - 0x4dc9b0 e 0x4e5770 cdecl(tecnico, id da noticia) ret; por endereco nos botoes das noticias (0x7834ee; 0x783590,
#   0x7835e1, 0x783bf6); NewsItem local (0x77bbe0, sem destruidor), News::find_item_by_id 0x786b70, NewsItem::get_slot
#   0x785830, TRANSFER_MANAGER::get_offer_slot 0x8c61e0, contract_screens_offer_contract_to_staff 0x4dd0f0 (Rust, cdecl).
# - 0x727fa0 match_screens_draw_waiting_screen cdecl(); por endereco em 0x727f84 (a tela de espera dos jogadores em
#   rede). PROLOGO COM CALL: push 0; call 0x41c0d0 -> precisa de (0x727fa0, 2) no RELATIVAS.
# - 0x72aa70 match_stats_static_date_atexit: um ret (1 byte; o salto do desvio cai no enchimento, como as 0x7d4420 e
#   0x7d4450 do tel36_13).
# - 0x81b800 search_filter_record_init thiscall(this) -> this; chamada pelo PLAYER_SEARCH::PLAYER_SEARCH (0x7ae41a).
# Comparadores: a limpeza das tabelas so a original, conferindo depois as 11 secoes vazias; o global da foto e o
# registro dos filtros lado a lado; a copia do Text lado a lado num Text local; o destruidor da lista, os botoes das
# noticias (abrem tela) e o destruidor vazio so a original; a tela de espera lado a lado sobre o bloco de parametros.
# Depende de: tel39_6 (text_set_string, text_add_link, TEXT_*, comp_text_free), background_section/background_sub
# (motor.rs), screen_param_regions e first_differences (match_screens.rs), text_contents (compare/media.rs).
TELAS = True
ROTINAS = [('background_free_config_tables', 0x41bad0, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('background_set_pictures_disabled', 0x41bf80, [0x8b, 0x4c, 0x24, 0x04, 0x33, 0xc0]),
           ('background_pictures_disabled', 0x41bfa0, [0x8b, 0x0d, 0xda, 0x6b, 0xdd, 0x00]),
           ('llist_scalar_deleting_destructor', 0x497910, [0x56, 0x8b, 0xf1, 0xe8, 0x28, 0x57, 0x1f, 0x00]),
           ('text_construct_copy', 0x4ae5c0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('contract_screens_news_offer_contract_action', 0x4dc9b0, [0x6a, 0xff, 0x68, 0x9b, 0x70, 0x95, 0x00]),
           ('contract_screens_news_negotiate_action', 0x4e5770, [0x6a, 0xff, 0x68, 0x2b, 0x73, 0x95, 0x00]),
           ('match_screens_draw_waiting_screen', 0x727fa0, [0x6a, 0x00, 0xe8, 0x29, 0x41, 0xcf, 0xff]),
           ('match_stats_static_date_atexit', 0x72aa70, [0xc3, 0x90, 0x90, 0x90, 0x90]),
           ('search_filter_record_init', 0x81b800, [0x8b, 0xc1, 0x80, 0xc9, 0xff])]
DOCS = ('as miudezas das telas: as opções das fotos de fundo, a cópia de um Text, o destruidor de uma lista, os botões das notícias de contrato, a tela de espera, o destruidor de uma data estática e o registro dos filtros da busca',
        "the screens' small routines: the background picture settings, a Text copy, a list's deleting destructor, the contract news buttons, the waiting screen, a static date's destructor and the search filter record",
        [('background_free_config_tables', 'background_free_config_tables', '0x41bad0', 'background.cpp', 'cdecl',
          'libera as tabelas do pics.cfg: das 11 seções, cada subseção (as duas listas de fotos e o nome) e o vetor de subseções, zerando ponteiros e contagens',
          "frees the pics.cfg tables: of the 11 sections, each subsection (its two picture lists and its name) and the subsection array, clearing pointers and counts"),
         ('background_set_pictures_disabled', 'background_set_pictures_disabled', '0x41bf80', 'background.cpp', 'cdecl',
          'grava a opção da foto fixa (0xdd6bda) como o inverso do argumento',
          'sets the fixed picture option (0xdd6bda) to the inverse of the argument'),
         ('background_pictures_disabled', 'background_pictures_disabled', '0x41bfa0', 'background.cpp', 'cdecl',
          'devolve o inverso da opção da foto fixa (1 quando 0xdd6bda é 0)',
          'returns the inverse of the fixed picture option (1 when 0xdd6bda is 0)'),
         ('llist_scalar_deleting_destructor', 'LLIST::scalar_deleting_destructor', '0x497910', 'comp_screens.cpp', 'thiscall',
          'destrói uma lista ligada e libera a memória dela quando o bit 0 das opções está ligado',
          'destroys a linked list and frees its memory when bit 0 of the flags is set'),
         ('text_construct_copy', 'Text::Text', '0x4ae5c0', 'comp_text.cpp', 'thiscall',
          'constrói um Text como cópia de outro: o texto e cada vínculo',
          'constructs a Text as a copy of another: its string and each link'),
         ('contract_screens_news_offer_contract_action', 'contract_screens_news_offer_contract_action', '0x4dc9b0', 'contract_screens.cpp', 'cdecl',
          'o botão "Oferecer contrato" da notícia: abre a oferta de contrato ao jogador da vaga 2',
          'the news item\'s "Offer Contract" button: opens the contract offer to the player of slot 2'),
         ('contract_screens_news_negotiate_action', 'contract_screens_news_negotiate_action', '0x4e5770', 'contract_screens.cpp', 'cdecl',
          'o botão "Negociar" da notícia de contrato: abre a negociação com a pessoa da vaga 0; depois de uma proposta de transferência aceita (tipo 0xfa4), sobre a proposta da vaga 7',
          'the contract news item\'s "Negotiate" button: opens the negotiation with the person of slot 0; after an accepted transfer offer (type 0xfa4), on the offer of slot 7'),
         ('match_screens_draw_waiting_screen', 'match_screens_draw_waiting_screen', '0x727fa0', 'match_screens.cpp', 'cdecl',
          'a tela de espera dos jogadores (jogo em rede): só tira a foto de fundo',
          'the waiting for players screen (network game): only clears the background picture'),
         ('match_stats_static_date_atexit', 'match_stats_static_date_atexit', '0x72aa70', 'match_screens.cpp', 'cdecl',
          'o destruidor vazio da data estática da 0x729ff0',
          "the empty destructor of 0x729ff0's static date"),
         ('search_filter_record_init', 'search_filter_record_init', '0x81b800', 'search_screens.cpp', 'thiscall',
          'constrói o registro dos filtros da busca de jogadores (0x11 bytes em PLAYER_SEARCH +0x293) com os valores padrão',
          "constructs the player search's filter record (0x11 bytes at PLAYER_SEARCH +0x293) with the default values")])
