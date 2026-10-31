# O clube livre para o amistoso, o clube na copa e a nação parceira do calendário (friendly.cpp, Cup.cpp, fix_man.cpp):
# friendly_is_club_free_on_date (0x5c6b70, 430 chamadas na cobertura de 17/10: excursões, torneios, convites e a
# situação das datas do clube na marcação dos amistosos), Cup::has_club (0x522910, a virtual +0x80 das copas, que a
# projeção da diretoria 0x4ae300 consulta; Cup.cpp fora da cobertura: o ramo diz) e fix_man_get_partner_key_nation_index
# (0x5af5b0, das buscas de jogos 0x5acbe0/0x5acee0/0x5ad780; fora do mapa da cobertura). A 0x5c6b70 tem SEH só pela
# lista provisória (0x68d020; a cópia a destrói pela 0x68d040 em toda saída, como o desenrolar) e empurra à 0x5acbe0 os
# dwords das datas como a original (a palavra e os 2 bytes seguintes). A folga vai à 0x54c5b0 com os 16 bits altos de
# um ponteiro na original: ela só lê a palavra. Iguais no cm0102-1080.exe. Simulação: as mutações pela carreira e a
# confirmação no jogo real comparando. Comparadores: as três pela devolução (a 0x5c6b70 roda a cópia com a lista
# provisória dela).
SAVE = False
ROTINAS = [('clube_livre_na_data', 0x5c6b70, [0x6a, 0xff, 0x68, 0x31, 0xa1, 0x95, 0x00]),
           ('copa_tem_o_clube', 0x522910, [0x56, 0x66, 0x8b, 0x71, 0x3e]),
           ('nacao_parceira_do_clube', 0x5af5b0, [0x8b, 0x44, 0x24, 0x04, 0x53])]
DOCS = ('o clube livre para o amistoso, o clube na copa e a nação parceira do calendário',
        'the club free for a friendly, the club in the cup and the calendar partner nation',
        [('clube_livre_na_data', 'friendly_is_club_free_on_date', '0x5c6b70', 'friendly.cpp', 'stdcall, ret 0xc',
          'diz se o clube está livre para um amistoso na data: sem jogo da véspera até a folga do tipo e, fora das seleções, fora da temporada da nação-chave',
          'tells whether the club is free for a friendly on the date: no fixture from the day before to the kind rest days and, outside national sides, outside the key-nation season'),
         ('copa_tem_o_clube', 'Cup::has_club', '0x522910', 'Cup.cpp', 'thiscall, ret 4',
          'diz se o clube está na tabela de times da copa',
          "tells whether the club is in the cup's team table"),
         ('nacao_parceira_do_clube', 'fix_man_get_partner_key_nation_index', '0x5af5b0', 'fix_man.cpp', 'cdecl',
          'para clube inglês ou galês, o índice da nação-chave ativa da outra nação; senão 0xff',
          'for an English or Welsh club, the active key-nation index of the other nation; otherwise 0xff')])
CAMPOS = [('clube +0: o id; +0x53 a nação; id e ponteiro; lê (clube_livre_na_data, nacao_parceira_do_clube)',
           'club +0: the id; +0x53 the nation; id and pointer; reads (clube_livre_na_data, nacao_parceira_do_clube)'),
          ('nação +0: o id; +0x11c ativa (byte); id e valor; lê (clube_livre_na_data, nacao_parceira_do_clube)',
           'nation +0: the id; +0x11c active (byte); id and value; reads (clube_livre_na_data, nacao_parceira_do_clube)'),
          ('nação-chave (72 bytes em 0xb63d60) +0: a nação; +0x14 o índice; +0x2a o início da temporada; +0x39 o fim; ponteiro, valor e data; lê (clube_livre_na_data, nacao_parceira_do_clube)',
           'key nation (72 bytes at 0xb63d60) +0: the nation; +0x14 the index; +0x2a season start; +0x39 season end; pointer, value and date; reads (clube_livre_na_data, nacao_parceira_do_clube)'),
          ('[0x9cf2e4] a Inglaterra; [0x9cf510] o País de Gales; [0xae23a8] as nações (0x122 bytes); id e ponteiro; lê (nacao_parceira_do_clube)',
           '[0x9cf2e4] England; [0x9cf510] Wales; [0xae23a8] the nations (0x122 bytes); id and pointer; reads (nacao_parceira_do_clube)'),
          ('copa +0x3e: a conta dos times; +0xa7 a tabela (6 bytes: +0 o clube); valor e ponteiro; lê (copa_tem_o_clube)',
           'cup +0x3e: the team count; +0xa7 the table (6 bytes: +0 the club); value and pointer; reads (copa_tem_o_clube)')]
INTERESSA = ['CMRTE']
