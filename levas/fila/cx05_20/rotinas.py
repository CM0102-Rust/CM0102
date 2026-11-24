# As telas dos amistosos: um torneio pelo indice, os pedidos de datas do clube humano e os convites do torneio dele
# (friendly.cpp; leitor cx05, rodada 5, 24/11/2026). Item de TELAS: as cinco so sao chamadas pelas telas dos amistosos
# (0x46931a, 0x47170e, 0x474806, 0x482b5a, 0x482e08, 0x482ed1, 0x482f3c) e pelo botao de acao das noticias (0x77d697,
# 0x77da8c, 0x782e80, 0x7843a4); 0 chamadas na carreira automatica.
# Caminho no jogo (o Lucas, jogando comparando): com um clube humano, abrir a tela de amistosos do clube na
# pre-temporada; organizar um torneio (sede), convidar e desconvidar clubes (set_tournament_invite); oferecer datas
# livres para amistosos (o pedido novo), abrir a lista delas (o pedido aberto) e cancelar o pedido; clicar o botao de
# uma noticia de convite para torneio (get_tournament).
# - 0x5c89a0 Friendly::get_tournament thiscall(indice, data) ret 8: Date::operator== 0x4a7f20 (2 pushes; lea ecx ->
#   thiscall, ret 8). Prologo lea eax, [ecx+0xb1] (6 bytes).
# - 0x5c6920 Friendly::find_human_club_open_dates_request thiscall() ret 0: LLIST::get_next 0x68d1b0 (mov ecx, lista
#   -> thiscall). Prologo push esi; mov esi, ecx; push edi; mov eax, [esi+0x34cc] (10 bytes).
# - 0x5c6990 Friendly::set_tournament_invite thiscall(clube, incluir) ret 8: o torneio do clube humano (o mesmo laco do
#   find_human_club_tournament, com a caixa da linha 0xd95 sem torneio), Friendly::send_tournament_invitations 0x5c5310
#   (push torneio; mov ecx -> thiscall, ret 4). Prologo movsx eax, word [0xb77c7a] (7 bytes).
# - 0x5c71f0 Friendly::add_friendly_dates_request thiscall(clube, valor, n, datas) ret 0x10 (da tela 0x47170e): operator
#   new 0x944cf1, o vector constructor iterator do CRT 0x9463af (5 pushes -> stdcall) com FixedFixture::FixedFixture
#   0x645800, Date::operator= 0x417c10, operator new e LLIST::LLIST, LLIST::add_tail 0x68d0d0 (copia 0x107 bytes), o
#   vector destructor iterator 0x9462bb (stdcall) com o corpo vazio 0x5a53c0 e o operator delete 0x944c94. Prologo push
#   -1; push imm32 (7 bytes).
# - 0x5c85d0 Friendly::cancel_friendly_dates_request thiscall(pedido) ret 4 (da tela 0x46931a): LLIST::LLIST/~LLIST
#   numa lista local, FIXTURE_MANAGER::find_fixtures 0x5acbe0 (9 pushes; mov ecx, [0xae2a58] -> thiscall, ret 0x24; o
#   ano empurrado como o dword em +2 da data), FIXTURE_MANAGER::remove_fixture 0x5abc30, Friendly::find_friendly_match
#   0x5c88f0 e Friendly::remove_friendly_match 0x5c87b0 (ja em Rust: chamadas pelo endereco), LLIST::remove 0x68d1f0.
#   Prologo mov eax, fs:[0] (6 bytes).
# Iguais no cm0102-1080.exe (as cinco inteiras).
# Comparadores: os dois leitores lado a lado no valor; os convites, o pedido novo e o cancelamento so a original (as
# noticias saem, a lista e o calendario mudam), com o plano: o pedido copiado na lista; o pedido fora da lista.
SAVE = False
TELAS = True
ROTINAS = [('friendly_get_tournament', 0x5c89a0, [0x8d, 0x81, 0xb1, 0x00, 0x00, 0x00]),
           ('find_human_club_open_dates_request', 0x5c6920, [0x56, 0x8b, 0xf1, 0x57, 0x8b, 0x86, 0xcc, 0x34, 0x00, 0x00]),
           ('set_tournament_invite', 0x5c6990, [0x0f, 0xbf, 0x05, 0x7a, 0x7c, 0xb7, 0x00]),
           ('add_friendly_dates_request', 0x5c71f0, [0x6a, 0xff, 0x68, 0x8c, 0xa1, 0x95, 0x00]),
           ('cancel_friendly_dates_request', 0x5c85d0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('as telas dos torneios e pedidos de amistosos', 'the friendly tournaments and requests screens',
        [('friendly_get_tournament', 'Friendly::get_tournament', '0x5c89a0', 'friendly.cpp', 'thiscall, ret 8',
          'devolve o torneio de amistosos de um índice quando a vaga tem sede e, se uma data é dada, o torneio começa nela; senão nada',
          'returns the friendly tournament at an index when the slot has a host and, if a date is given, the tournament starts on it; otherwise nothing'),
         ('find_human_club_open_dates_request', 'Friendly::find_human_club_open_dates_request', '0x5c6920', 'friendly.cpp', 'thiscall, ret 0',
          'devolve o pedido de datas de amistosos do clube do técnico humano que ainda espera adversários',
          'returns the friendly dates request of the human manager\'s club that still waits for opponents'),
         ('set_tournament_invite', 'Friendly::set_tournament_invite', '0x5c6990', 'friendly.cpp', 'thiscall, ret 8',
          'regra: no torneio que o clube humano organiza há quatro vagas de convidados; incluir põe o clube na primeira vaga livre (pendente até aceitar; um clube já convidado é erro), excluir esvazia a vaga dele; depois os convites saem de novo',
          'rule: the tournament the human club hosts has four invitation places; adding puts the club in the first free place (pending until it accepts; a club already invited is an error), removing empties its place; then the invitations go out again'),
         ('add_friendly_dates_request', 'Friendly::add_friendly_dates_request', '0x5c71f0', 'friendly.cpp', 'thiscall, ret 0x10',
          'regra: um clube oferece até dez datas livres para amistosos; o pedido guarda o clube, cada data sem adversário e nasce fechado; uma cópia vai para o fim da lista de pedidos',
          'rule: a club offers up to ten free dates for friendlies; the request keeps the club, each date with no opponent, and starts closed; a copy goes to the end of the requests list'),
         ('cancel_friendly_dates_request', 'Friendly::cancel_friendly_dates_request', '0x5c85d0', 'friendly.cpp', 'thiscall, ret 4',
          'regra: cancelar um pedido de datas tira do calendário os amistosos ainda não jogados do clube nessas datas (menos os de hoje) e o amistoso marcado em cada data, e o pedido sai da lista',
          "rule: cancelling a dates request removes from the calendar the club's unplayed friendlies on those dates (except today's) and the friendly arranged on each date, and the request leaves the list")])
CAMPOS = [('pedido de datas de amistosos (0x107 bytes) +0 clube, +4 valor, +8 quantas datas, +9 dez vagas de 0x19 bytes (+0 data, +0xc e +0x14 clube, +0x18), +0x103 aberto; lê e grava', 'friendly dates request (0x107 bytes) +0 club, +4 value, +8 date count, +9 ten slots of 0x19 bytes (+0 date, +0xc and +0x14 club, +0x18), +0x103 open; reads and writes')]
INTERESSA = []
