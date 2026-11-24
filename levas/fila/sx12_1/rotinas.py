# A lesao atual de uma pessoa e duas contas da tabela das nacoes-chave (injury.cpp, key_nation.cpp; leitor sx12,
# rodada 5, 23/11/2026). Convencoes conferidas na desmontagem:
# - Injury::get_player_injury (0x62ed70..0x62ee2b): thiscall(pessoa), ret 4; pessoa nula: a caixa "Error" (linha
#   0xe3a, o mesmo trecho do show_error_at) e 0; id >= [0xae2364]: 0; senao 0x62c680 thiscall(this, pessoa) ret 4 e a
#   entrada de 0x30 bytes de [this+4] pelo dword +0xe do registro de 31 bytes de [this] (-1: 0). 54 chamadores (a
#   escolha da lesao 0x62d159, a partida, a escalacao, as telas): roda o tempo todo na carreira.
# - key_nation_continent_has_selected_nation (0x6699d0..0x669a05): cdecl(continente), eax 0/1.
# - key_nation_number_active_entries (0x669140..0x66917a): cdecl, sem argumentos, eax = 1; chamada pela criacao do
#   jogo (0x82893b, 0x82b8ab), uma vez por carreira.
SAVE = False
ROTINAS = [('get_player_injury', 0x62ed70, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('key_nation_continent_has_selected_nation', 0x6699d0, [0x8b, 0x54, 0x24, 0x04, 0x53]),
           ('key_nation_number_active_entries', 0x669140, [0x32, 0xd2, 0x53, 0x88, 0x15, 0x38, 0x47, 0xb6, 0x00])]
DOCS = ('a lesao atual de uma pessoa e as contas da tabela das nacoes-chave',
        "a person's current injury and the key nation table counts",
        [('get_player_injury', 'Injury::get_player_injury', '0x62ed70', 'injury.cpp', 'thiscall, ret 4',
          'devolve a lesao atual da pessoa: pessoa nula, a caixa "Error" e nada; pessoa fora da tabela, nada; senao atualiza a forma do dia (uma vez por dia) e devolve a entrada de lesao apontada pelo registro fisico da pessoa, ou nada quando ela nao esta lesionada',
          "returns the person's current injury: a null person, the \"Error\" box and nothing; a person outside the table, nothing; otherwise it updates the day's fitness (once a day) and returns the injury entry the person's physical record points to, or nothing when he is not injured"),
         ('key_nation_continent_has_selected_nation', 'key_nation_continent_has_selected_nation', '0x6699d0', 'key_nation.cpp', 'cdecl',
          'diz se a tabela das 35 nacoes-chave tem uma nacao do continente dado escolhida para ser jogada',
          'tells whether the table of the 35 key nations holds a nation of the given continent chosen to be played'),
         ('key_nation_number_active_entries', 'key_nation_number_active_entries', '0x669140', 'key_nation.cpp', 'cdecl',
          'numera as linhas ativas da tabela das nacoes-chave (as linhas sem nacao, do mundo e dos continentes, e as das nacoes escolhidas), na ordem, e guarda quantas sao; as inativas ficam com o numero antigo',
          'numbers the active rows of the key nation table (the rows without a nation, of the world and continents, and those of the chosen nations), in order, and stores how many there are; the inactive ones keep their old number')])
