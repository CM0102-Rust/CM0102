# The tactics screens' 48-bit mask and bit index (tactics_screens.cpp; reader tel36, the screens round, 21/11/2026).
# Helpers of the tactics screens (mask64_to_bit_index called by 0x899ea0, bit_index_to_mask64 by 0x89f710).
# Conventions checked in the disassembly: mask64_to_bit_index cdecl(low, high) -> al; bit_index_to_mask64 cdecl(char)
# -> edx:eax. Both checked against the original by emulating it (unicorn) on every index 0..255, on the 64 single bits
# and on random and two-bit masks.
# Equal in cm0102-1080.exe.
# Comparers: by value.
TELAS = True
ROTINAS = [('mask64_to_bit_index', 0x8adbb0, [0x8b, 0x4c, 0x24, 0x04, 0x8b, 0x44, 0x24, 0x08]),
           ('bit_index_to_mask64', 0x8ade80, [0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0])]
DOCS = ('a máscara de 48 bits das telas de tática e o índice do bit',
        "the tactics screens' 48-bit mask and its bit index",
        [('mask64_to_bit_index', 'mask64_to_bit_index', '0x8adbb0', 'tactics_screens.cpp', 'cdecl',
          'a máscara com um bit só entre os 48 de baixo dá o índice dele (0 a 47); outra, -1',
          'a mask with a single bit among the low 48 gives its index (0 to 47); anything else -1'),
         ('bit_index_to_mask64', 'bit_index_to_mask64', '0x8ade80', 'tactics_screens.cpp', 'cdecl',
          'o índice dá a máscara 1 << k de 0 a 47; outro, 0',
          'the index gives the mask 1 << k for 0 to 47; anything else 0')])
