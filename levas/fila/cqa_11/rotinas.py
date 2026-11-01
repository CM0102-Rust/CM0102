# Os registros dos tecnicos (staff_records.cpp):
# - 0x868830 (Staff_Records::record_match_result): pelo fechamento da partida do dia 0x6b5f00 ("fechar historia" 2048
#   na base da 261a), a cada jogo; os pontos do mes (+0xce) sao lidos pelo tecnico do mes 0x86f950 e os da temporada
#   (+0xba) pelos candidatos a tecnico do ano (0x870220).
# - 0x86a070 (Staff_Records::add_league_finish_records): pela liga completa 0x689ad0 ("liga completa: fim da liga" 16).
# - 0x86dc30 (Staff_Records::add_supporters_player_of_year_record): pelo fim da temporada dos recordes 0x447880 (pode nao
#   rodar ate 2002-06-01: passageira).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x86f5b0 registro_do_tecnico: thiscall (mov ecx, ebx = this em 0x86887b; a primeira com o ecx de entrada), 2 args
#   (push 1, push tecnico), ret 8. 0x48e190 importancia_do_jogo_na_competicao: thiscall (mov ecx, [[0xadadfc] + 4 x
#   [jogo]] em 0x868a6a/0x868b6b), 3 args (push &vaga do arg jogo, push 0 / 1, push jogo), ret 0xc; o eax so pelo al
#   (movsx ecx, al). 0x53b900 e_selecao: cdecl (add esp, 4 em 0x868b02), 1 arg.
#   0x6826d0 registro_da_liga: thiscall (mov ecx, edi = liga em 0x86a0e6), 1 arg (push esi = k com sinal), ret 4.
#   0x58e340 (FILE_LLIST, historico): thiscall (mov ecx, [this] em 0x86a119 e 0x86dcc5), 3 args (push &entrada, push
#   lista + 4, push lista), pelo cpq4_historico. 0x54a110/0x417c10 (a data da entrada): thiscall, pelo cpq4_entrada e
#   cpq4_datar. 0x90cfc0 sorteio: cdecl (add esp, 8 em 0x86dd50; add esp, 0x20 no fim cobre os dois de 2 e as duas
#   chamadas). 0x547d90 staff_add_favoured_club: cdecl (pessoa, clube, forcar), 3 args. 0x547870
#   clube_acrescentar_pessoa_favorita: cdecl (clube, pessoa, forcar), 3 args (add esp, 0x20 em 0x86dd75).
#   As proprias: 0x868830 thiscall ret 8 (o chamador 0x6b5f00 poe ecx = [0xdd7f00]); 0x86a070 thiscall ret 4 (ecx =
#   [0xdd7f00] em 0x689ad0); 0x86dc30 thiscall ret 0x10.
# Comparadores: a 0x868830 lado a lado nos dois registros (criados antes pela 0x86f5b0, como a original faria); a
# 0x86a070 so a original (so o historico em arquivo); a 0x86dc30 em ensaio (a copia sem o historico; o sorteio, as
# favoritas do clube e os clubes da pessoa). Defeito da original reproduzido na 0x868830: os pontos da temporada do
# visitante (+0xba/+0xbe) recebem a nota do mandante (o [esp+0x10] reaproveitado). Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('resultado_nos_registros_dos_tecnicos', 0x868830, [0x83, 0xec, 0x08, 0x53, 0x55]),
           ('posicoes_no_fim_da_liga', 0x86a070, [0x6a, 0xff, 0x68, 0xdb, 0x2b, 0x96, 0x00]),
           ('jogador_do_ano_da_torcida_no_registro', 0x86dc30, [0x6a, 0xff, 0x68, 0xbb, 0x2d, 0x96, 0x00])]
DOCS = ('os registros dos técnicos: o resultado de cada jogo, a posição no fim da liga e o jogador do ano da torcida',
        "the managers' records: each match result, the final league position and the supporters' player of the year",
        [('resultado_nos_registros_dos_tecnicos', 'Staff_Records::record_match_result', '0x868830', 'staff_records.cpp', 'thiscall',
          'nos registros dos dois técnicos: o jogo, os gols, a vitória, o empate ou a derrota e a nota do jogo (importância × reputação do adversário × peso × pontos) no mês, no ano, na liga do clube e na temporada do clube ou da seleção; o visitante recebe na temporada a nota do mandante (defeito da original)',
          "in both managers' records: the match, the goals, the win, draw or loss and the match rating (importance × opponent reputation × weight × points) in the month, the year, the club's league and the club or national season; the away manager gets the home rating in the season total (original bug)"),
         ('posicoes_no_fim_da_liga', 'Staff_Records::add_league_finish_records', '0x86a070', 'staff_records.cpp', 'thiscall',
          'no fim da liga, a posição final de cada clube no histórico do técnico dele',
          "at the end of the league, each club's final position in its manager's history"),
         ('jogador_do_ano_da_torcida_no_registro', 'Staff_Records::add_supporters_player_of_year_record', '0x86dc30', 'staff_records.cpp', 'thiscall',
          'o jogador do ano da torcida no histórico dele e, pelo sorteio, o vínculo de favoritos entre ele e o clube',
          "the supporters' player of the year in his history and, by a draw, the favourite link between him and the club")])
CAMPOS = [('registro do técnico (0xd2 bytes em [0xdd7f00]+8) +0x8e jogos, +0x90 vitórias, +0x92 empates, +0x94 derrotas, +0x96 gols pró, +0x9a gols contra; valores; grava (resultado_nos_registros_dos_tecnicos)',
           "manager record (0xd2 bytes at [0xdd7f00]+8) +0x8e games, +0x90 wins, +0x92 draws, +0x94 losses, +0x96 goals for, +0x9a goals against; values; writes (resultado_nos_registros_dos_tecnicos)"),
          ('registro do técnico +0xb6 pontos do ano, +0xca pontos, +0xce pontos do mês (liga do clube), +0xba temporada no clube, +0xbe temporada na seleção; valores; grava (resultado_nos_registros_dos_tecnicos)',
           'manager record +0xb6 year points, +0xca points, +0xce month points (club league), +0xba club season, +0xbe national season; values; writes (resultado_nos_registros_dos_tecnicos)'),
          ('jogo +0x14 competição, +0x1c/+0x20 clubes, +0x3a não disputado, +0x43/+0x44 e +0x47/+0x48 placares; ponteiros/valores; lê (resultado_nos_registros_dos_tecnicos)',
           'fixture +0x14 competition, +0x1c/+0x20 clubs, +0x3a not played, +0x43/+0x44 and +0x47/+0x48 scores; pointers/values; reads (resultado_nos_registros_dos_tecnicos)'),
          ('partida +0x11f/+0x127/+0x128/+0x129 e +0x744/+0x74c/+0x74d/+0x74e contagens de cada lado; valores; lê (resultado_nos_registros_dos_tecnicos)',
           'match data +0x11f/+0x127/+0x128/+0x129 and +0x744/+0x74c/+0x74d/+0x74e counts for each side; values; reads (resultado_nos_registros_dos_tecnicos)'),
          ('clube +0xcf técnico, +0x80 reputação, +0x57/+0x60 ligas, +0x9b..+0xb3 pessoas favoritas e rejeitadas; ponteiros/valores; lê (resultado_nos_registros_dos_tecnicos, posicoes_no_fim_da_liga), grava as favoritas (jogador_do_ano_da_torcida_no_registro)',
           'club +0xcf manager, +0x80 reputation, +0x57/+0x60 leagues, +0x9b..+0xb3 favoured and disliked people; pointers/values; reads (resultado_nos_registros_dos_tecnicos, posicoes_no_fim_da_liga), writes the favourites (jogador_do_ano_da_torcida_no_registro)'),
          ('preferências da pessoa (+0x65) +4..+0x18 clubes favoritos e rejeitados; ponteiros; grava (jogador_do_ano_da_torcida_no_registro)',
           "person preferences (+0x65) +4..+0x18 favoured and disliked clubs; pointers; writes (jogador_do_ano_da_torcida_no_registro)"),
          ('liga +4 competição, +0x3a ano, +0x3e linhas da tabela; valores; lê (posicoes_no_fim_da_liga)', 'league +4 competition, +0x3a year, +0x3e table rows; values; reads (posicoes_no_fim_da_liga)')]
INTERESSA = ['CMRTE']
