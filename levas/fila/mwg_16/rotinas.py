# As listas de indicados aos prêmios: montar uma, acrescentar um candidato e anunciá-la (award_shortlist.cpp; leitor
# mwg, rodada 3 do mundo inteiro, 19/11/2026). Até 2002-08-01: o construtor 1 vez, add_entry 10, announce 1
# (cobertura_mundo.txt; a lista do prêmio de fim de temporada); a Premier tem os prêmios com lista: LIGAS = 'ENG'.
# Fica de fora pick_winners (0x417c50, a votação ponderada dos clubes do computador, 912 bytes): próxima rodada.
# Convenções conferidas na desmontagem (0x417440..0x41757f, 0x4178a0..0x417a1d, 0x417a20..0x417c0d):
# - AWARD_SHORTLIST::AWARD_SHORTLIST: thiscall, ret 8 (escopo, arquivo), devolve this; quadro SEH só para a Date
#   membro: Date::Date() (0x54a110) em +0x38; sem escopo a caixa (linha 0x30); +0x36 = -1, +6 = movsx byte escopo+0xd,
#   +4 = 0, +0 = malloc(14 x capacidade) (sem memória, a caixa 0x3e); o escopo (3 dwords e uma palavra) em +0x28; com
#   arquivo, AWARD_SHORTLIST::load (0x417590, thiscall, ret 4);
# - AWARD_SHORTLIST::add_entry: thiscall, ret 0xc (jogador, clube ou 0, nota float); cheia (contagem >= capacidade,
#   palavras com sinal): a caixa 0xbf e 1; sem jogador: a caixa 0xc6 e 0; a entrada (id, id do clube ou -1, a nota por
#   fld/fstp, 0 votos), contagem + 1, devolve contagem == capacidade;
# - AWARD_SHORTLIST::announce: thiscall, ret 0xc (data, a, b); quadro SEH para a NewsItem local (0xe0 bytes); qsort
#   (0x945333) por nome (0x418340); AWARD_MANAGER::add_shortlist ([0xad9c60], thiscall, ret 4) em +0x36; Date::operator=
#   (0x417c10) em +0x38; NewsItem::NewsItem(0x1c, 0) (0x77bbe0) e set_slot (0x785790) 0 a, 1 b, 2 o dia (movsx), 3 o
#   ano (movzx), 4 0; as 16 linhas de técnico humano (staff[[0xae2364] - k - 1], 0x6e bytes) com +0x5f e no escopo
#   recebem a notícia (News::send_to_club_manager 0x7861e0, ecx = [0xb7431c], ret 8) com o clube (+0x39) ou a seleção
#   (+0x24).
# Comparadores: o construtor só no original (aloca), com o plano; add_entry lado a lado na contagem e na entrada nova;
# announce só no original (manda notícias), com o índice e a data.
SAVE = False
LIGAS = 'ENG'
ATE = '2002-08-01'
ROTINAS = [('construct_award_shortlist', 0x417440, [0x6a, 0xff, 0x68, 0xce, 0x3d, 0x95, 0x00]),
           ('award_shortlist_add_entry', 0x4178a0, [0x66, 0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('award_shortlist_announce', 0x417a20, [0x6a, 0xff, 0x68, 0xeb, 0x3d, 0x95, 0x00])]
DOCS = ('as listas de indicados aos prêmios: montar, acrescentar um candidato e anunciar',
        'the award shortlists: building one, adding a candidate and announcing it',
        [('construct_award_shortlist', 'AWARD_SHORTLIST::AWARD_SHORTLIST', '0x417440', 'award_shortlist.cpp', 'thiscall, ret 8',
          'monta uma lista de indicados a partir do registro de escopo de 14 bytes: a data vazia, o índice no gerente −1, a capacidade do escopo, nenhum candidato e as entradas alocadas (sem escopo ou sem memória, a caixa de erro), o escopo copiado; com o arquivo de um jogo salvo, carrega a lista',
          'builds a shortlist from its 14-byte scope record: the date empty, the manager index −1, the capacity from the scope, no candidates and the entries allocated (without a scope or out of memory, the error box), the scope copied; with a saved game file, loads the list'),
         ('award_shortlist_add_entry', 'AWARD_SHORTLIST::add_entry', '0x4178a0', 'award_shortlist.cpp', 'thiscall, ret 0xc',
          'acrescenta um candidato à lista: o id do jogador, o do clube (ou −1 sem clube), a nota e nenhum voto; devolve se a lista encheu (cheia antes, a caixa e 1; sem jogador, a caixa e 0)',
          "appends a candidate to the list: the player's id, the club's (or −1 without a club), the score and no votes; returns whether the list is now full (already full, the box and 1; without a player, the box and 0)"),
         ('award_shortlist_announce', 'AWARD_SHORTLIST::announce', '0x417a20', 'award_shortlist.cpp', 'thiscall, ret 0xc',
          'anuncia a lista: os candidatos em ordem de nome, a lista registrada no gerente de prêmios, a data marcada e a notícia (tipo 0x1c) mandada a cada técnico humano no escopo dela (divisão, nação ou continente do clube, continente da seleção, todo técnico de clube, todo de seleção ou todos)',
          "announces the list: the candidates in name order, the list registered with the award manager, the date set and the news (type 0x1c) sent to each human manager in its scope (the club's division, nation or continent, the national team's continent, every club manager, every national manager or everyone)")])
