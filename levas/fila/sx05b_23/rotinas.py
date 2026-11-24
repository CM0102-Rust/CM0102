# O id da moeda da tela e a troca de bytes de 64 bits (cash.cpp; leitor sx05b, rodada 5, 23/11/2026). Usa CURRENCY_ID
# (sx05b_21): integrar depois dele.
# cash_get_currency roda na carga das preferencias (game_config.cpp 0x5d85d6); swap_bytes_64_array so no
# Formation::load_from_disk (0x5b5d10, ja em Rust) de um arquivo de tatica da outra ordem de bytes (nunca na carreira:
# os .pct do jogo sao da ordem nativa; a mutacao dela escapa por nao rodar). Item de simulacao sem LIGAS.
# Convencoes conferidas na desmontagem:
# - 0x4415e0 cdecl() -> al: `mov al, [0x98b140]; ret` (6 bytes; o prologo de 5 e o mov inteiro);
# - 0x43f350 cdecl(buf, conta): conta 0 nada; senao `dec` ate 0 (a conta como sem sinal); cada qword montado por
#   __allshl 0x945e20 e __allshr 0x945df0 (aritmetico) e mascaras: conferido por emulacao em Python que da a inversao
#   completa dos 8 bytes.
# Comparadores: o valor lado a lado; a troca lado a lado no buffer.
SAVE = False
ROTINAS = [('cash_get_currency', 0x4415e0, [0xa0, 0x40, 0xb1, 0x98, 0x00]),
           ('swap_bytes_64_array', 0x43f350, [0x8b, 0x44, 0x24, 0x08, 0x8b, 0xc8])]
DOCS = ('o id da moeda da tela e a troca de bytes de 64 bits',
        'the display currency id and the 64-bit byte swap',
        [('cash_get_currency', 'cash_get_currency', '0x4415e0', 'cash.cpp', 'cdecl',
          'devolve o id da moeda da tela',
          'returns the id of the display currency'),
         ('swap_bytes_64_array', 'swap_bytes_64_array', '0x43f350', 'cash.cpp', 'cdecl',
          'inverte a ordem dos bytes de cada palavra de 64 bits de um buffer, no lugar (a carga de um arquivo de tática da outra ordem de bytes)',
          'reverses the byte order of each 64-bit word of a buffer, in place (loading a tactics file of the other byte order)')])
CAMPOS = []
INTERESSA = []
