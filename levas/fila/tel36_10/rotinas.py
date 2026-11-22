# The search filters' attribute count and the filter bit index (search_screens.cpp; reader tel36, the screens round,
# 21/11/2026). Helpers of the search screens: the attribute counters are called by 0x818100 (the search filters screen
# handler), the bit/index conversions by 0x8133a0 and 0x818100.
# Conventions checked in the disassembly: the counters thiscall(filters, which) ret 4 -> eax (signed byte compares);
# bit_to_index cdecl(low, high) -> al; index_to_bit cdecl(char) -> edx:eax. All four checked against the original by
# emulating it (unicorn) on every index 0..255, on the 64 single bits, on 0x3_8000_0000 and on 300 random masks and
# 117 two-bit masks (bit_to_index), and on every pair of signed counter/maximum bytes from -3 to 5 (the counters).
# Equal in cm0102-1080.exe.
# Comparers: side by side (the counters over the filters' two byte pairs; the conversions by value).
TELAS = True
ROTINAS = [('search_filters_increase_attributes_to_match', 0x8084c0, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('search_filters_decrease_attributes_to_match', 0x808500, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('search_screens_filter_bit_to_index', 0x81b0e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('search_screens_filter_index_to_bit', 0x81b4e0, [0x8a, 0x44, 0x24, 0x04, 0x84, 0xc0])]
DOCS = ('os atributos a combinar dos filtros da busca e o índice do bit de um filtro',
        "the search filters' attributes to match and a filter's bit index",
        [('search_filters_increase_attributes_to_match', 'SearchFilters::increase_attributes_to_match', '0x8084c0', 'search_screens.cpp', 'thiscall, ret 4',
          'mais um atributo a combinar (o contador de linha, ou o de goleiro), até o máximo; devolve 1 se subiu',
          'one more attribute to match (the outfield counter, or the goalkeeper one), up to the maximum; returns 1 when it went up'),
         ('search_filters_decrease_attributes_to_match', 'SearchFilters::decrease_attributes_to_match', '0x808500', 'search_screens.cpp', 'thiscall, ret 4',
          'um atributo a menos, até 1; devolve 1 se desceu',
          'one attribute fewer, down to 1; returns 1 when it went down'),
         ('search_screens_filter_bit_to_index', 'search_screens_filter_bit_to_index', '0x81b0e0', 'search_screens.cpp', 'cdecl',
          'a máscara de 64 bits de um filtro com um bit só dá o índice dele (0 a 63); a máscara dupla 0x3_8000_0000 dá 0x40; outra, -1',
          "a filter's 64-bit mask with a single bit gives its index (0 to 63); the double mask 0x3_8000_0000 gives 0x40; anything else -1"),
         ('search_screens_filter_index_to_bit', 'search_screens_filter_index_to_bit', '0x81b4e0', 'search_screens.cpp', 'cdecl',
          'o índice de um filtro dá a máscara: 1 << k de 0 a 63, 0x3_8000_0000 para 0x40, 0 para o resto',
          "a filter's index gives its mask: 1 << k for 0 to 63, 0x3_8000_0000 for 0x40, 0 for the rest")])
