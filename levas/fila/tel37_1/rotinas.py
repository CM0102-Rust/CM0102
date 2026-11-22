# The network buffer (display.cpp, the NetworkBuffer methods of network.h; reader tel37, the screens round, 21/11/2026).
# The screen manager writes every screen as command packets into a NetworkBuffer and the local Display reads them back
# (Display::process_command_packet 0x564080; scrman: "a local record gets the packet applied to the local Display"):
# these run on every screen of a local game too, not only in network games.
# Conventions checked in the disassembly: all thiscall (ecx = the buffer: data, position, capacity), ret 4 (the value
# or the output pointer). Each one checks the room (position + size <= capacity, unsigned) and shows the network.h box
# (tel26_error_box with NETWORK_H, line 0x85..0x97) when there is none.
# Depends on tel26_1 (tel26_error_box, first_differences) and tel29_1 (NETWORK_H).
# The 1080 exe does not change these routines.
# Defects of the original: get_sized_block copies with an uninitialised length after the error box (the copy: 0).
TELAS = True
ROTINAS = [('network_buffer_put_char', 0x560140, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_put_short', 0x5601e0, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_put_ushort', 0x560290, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_put_ulong', 0x560340, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_put_int', 0x567b40, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_get_char', 0x567be0, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_get_short', 0x567c80, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_get_long', 0x567d30, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_get_int', 0x567de0, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_get_ushort', 0x567e90, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_get_ulong', 0x567f40, [0x8b, 0x41, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('network_buffer_get_sized_block', 0x567ff0, [0x81, 0xec, 0x04, 0x02, 0x00, 0x00])]
_P = [('put_char', '0x560140', 'um byte', 'one byte', 1, 133),
      ('put_short', '0x5601e0', 'uma palavra com sinal', 'a signed word', 2, 134),
      ('put_ushort', '0x560290', 'uma palavra sem sinal', 'an unsigned word', 2, 139),
      ('put_ulong', '0x560340', 'uma palavra dupla sem sinal', 'an unsigned dword', 4, 141),
      ('put_int', '0x567b40', 'uma palavra dupla com sinal', 'a signed dword', 4, 137)]
_G = [('get_char', '0x567be0', 'um byte', 'one byte', 1, 143),
      ('get_short', '0x567c80', 'uma palavra com sinal', 'a signed word', 2, 144),
      ('get_long', '0x567d30', 'uma palavra dupla com sinal', 'a signed dword', 4, 146),
      ('get_int', '0x567de0', 'uma palavra dupla (int)', 'a signed dword (int)', 4, 147),
      ('get_ushort', '0x567e90', 'uma palavra sem sinal', 'an unsigned word', 2, 149),
      ('get_ulong', '0x567f40', 'uma palavra dupla sem sinal', 'an unsigned dword', 4, 151)]
DOCS = ('o buffer de rede: pôr e tirar bytes, palavras, palavras duplas e blocos com tamanho (os pacotes de comando das telas)',
        'the network buffer: putting and getting bytes, words, dwords and sized blocks (the screen command packets)',
        [('network_buffer_' + n, 'NetworkBuffer::' + n, a, 'display.cpp', 'thiscall, ret 4',
          'põe %s na posição do buffer e avança %d; sem espaço (posição + %d acima da capacidade) a caixa de erro de network.h %d e nada muda' % (pt, k, k, ln),
          'puts %s at the buffer position and advances by %d; without room (position + %d above the capacity) the network.h %d error box and nothing changes' % (en, k, k, ln))
         for n, a, pt, en, k, ln in _P]
        + [('network_buffer_' + n, 'NetworkBuffer::' + n, a, 'display.cpp', 'thiscall, ret 4',
            'tira %s da posição do buffer para a saída e avança %d; sem espaço a caixa de erro de network.h %d e a saída fica como estava' % (pt, k, ln),
            'gets %s from the buffer position into the output and advances by %d; without room the network.h %d error box and the output is left as it was' % (en, k, ln))
           for n, a, pt, en, k, ln in _G]
        + [('network_buffer_get_sized_block', 'NetworkBuffer::get_sized_block', '0x567ff0', 'display.cpp', 'thiscall, ret 4',
            'tira um bloco: o tamanho (palavra com sinal) e depois os bytes, copiados para a saída; a posição avança 2 mais o tamanho. Defeito do original: depois da caixa de erro (sem espaço para o tamanho) copia com um tamanho não iniciado da pilha (a cópia usa 0)',
            'gets a block: the length (signed word) and then the bytes, copied to the output; the position advances by 2 plus the length. Defect of the original: after the error box (no room for the length) it copies with an uninitialised length from the stack (the copy uses 0)')])
