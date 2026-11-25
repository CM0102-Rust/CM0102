# O objeto dos estadios: reservas e sedes neutras (stadium.cpp; leitor sbI, rodada 5, 24/11/2026). Simulacao, carreira
# ENG (sem LIGAS).
# - 0x85b670 Stadium::Stadium thiscall() ret 0 -> this: na criacao do jogo (setup, passo 0x19, new 0x19) e na carga
#   (passo 0x1d, so this: le o Stadium.tmp pela 0x85ba40, ja em Rust). O CD pelo find_game_cd (0x85b6b0 e 0x85b6f4: o
#   patch sem CD do cm0102-1080 e do portatil troca esses 3 bytes; a rotina e desviada nos dois exes pelos dois
#   hashes; fora eles, igual no cm0102-1080.exe). No jogo novo: club_is_national_team 0x5409b0 (cdecl) nas linhas das
#   selecoes, de tras para frente, o +0x69 (estadio) zerado, e o build_venue_list 0x85dfc0 (Rust). Usa o find_game_cd
#   (sx02b_1, ja no HEAD desde a leva 330).
# Prologo sub esp, 0x474 (6 bytes). Sem relativas.
# Comparador: so a original (espera o CD, le o arquivo, aloca as sedes), com as reservas dentro da capacidade e as sedes
# montadas.
SAVE = False
ROTINAS = [('new_stadium_manager', 0x85b670, [0x81, 0xec, 0x74, 0x04, 0x00, 0x00])]
DOCS = ('o objeto dos estádios: reservas e sedes neutras', 'the stadium object: bookings and neutral venues',
        [('new_stadium_manager', 'Stadium::Stadium', '0x85b670', 'stadium.cpp', 'thiscall, ret 0',
          'monta o objeto dos estádios: confere o CD e zera as reservas e as sedes; carrega o arquivo temporário ou, no jogo novo, tira o estádio das seleções (as linhas das seleções no fim da tabela de clubes) e monta a lista das sedes neutras',
          'builds the stadium object: checks the CD and clears the bookings and the venues; loads the temporary file or, in a new game, takes the ground away from the national teams (their rows at the end of the club table) and builds the neutral venues list')])
CAMPOS = []
INTERESSA = []
