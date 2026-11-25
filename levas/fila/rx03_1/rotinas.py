# A T2K (library_without_asserts): as contas de ponto fixo e a ordenacao pequena (util.c da T2K; leitor rx03, rodada 6,
# 25/11/2026; o mapa em levas/reorganizacao/proximas/TRUETYPE.md).
# Item de TELAS: o texto desenhado e tela. No jogo rodam no desenho de cada letra nova (FONT 0x5b1ca3 ->
# T2K_RenderGlyph 0x93e8c0 -> a escala das coordenadas e o conversor) e na carga das fontes (T2K_NewTransformation,
# fixed_hypot e fixed_div). So a T2K as chama (nenhuma chamada fora de 0x933b20..0x944c10). Caminho no jogo: abrir o
# jogo comparando (as fontes carregam no inicio do video; cada letra nova passa por elas).
# Convencoes conferidas: todas cdecl (call direto, add esp do chamador), dois argumentos de 32 bits; o f2dot14_mul le
# so os 16 bits baixos do segundo (test cx, cx). sort_int16_array_ascending (vetor, n).
# - 0x9446a0 sort_int16_array_ascending, 0x9446e0 fixed_mul, 0x944760 f26dot6_mul, 0x9447e0 f2dot14_mul,
#   0x944840 fixed_div, 0x9448b0 fixed_hypot (tres passos de Newton; os dois primeiros com o fixed_mul e o fixed_div
#   escritos em linha pelo compilador, as mesmas contas).
# O rx01 chama estas pelo endereco (lib_fixed_mul, lib_f2dot14_mul, lib_fixed_div, lib_fixed_hypot): o desvio leva ao
# Rust. Iguais no cm0102-1080.exe (todas inteiras). Prologos sem call/jmp relativo.
# Comparadores: valor contra valor (puras); a ordenacao lado a lado no vetor.
# Prova sem o jogo: o emulador (rascunho rx03/check_rx03.py rx03_1), com as fronteiras (0, +-1, 0xffff, 0x10000,
# i32::MIN, iguais) e sorteio.
SAVE = False
TELAS = True
ROTINAS = [('sort_int16_array_ascending', 0x9446a0, [0x53, 0x8b, 0x5c, 0x24, 0x0c]),
           ('fixed_mul', 0x9446e0, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
           ('f26dot6_mul', 0x944760, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('f2dot14_mul', 0x9447e0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('fixed_div', 0x944840, [0x53, 0x56, 0x57, 0x8b, 0x7c, 0x24, 0x10]),
           ('fixed_hypot', 0x9448b0, [0x83, 0xec, 0x0c, 0x56, 0x8b, 0x74, 0x24, 0x14])]
_A = 'library_without_asserts'
DOCS = ('a T2K: as contas de ponto fixo e a ordenação pequena',
        'the T2K: the fixed-point helpers and the small sort',
        [('sort_int16_array_ascending', 'sort_int16_array_ascending', '0x9446a0', _A, 'cdecl',
          'regra: ordena n valores de 16 bits com sinal em ordem crescente, pelo método da bolha (passadas inteiras até nada mudar; iguais ficam onde estão)',
          'rule: sorts n signed 16-bit values ascending with a bubble sort (whole passes until nothing moves; equal values stay)'),
         ('fixed_mul', 'fixed_mul', '0x9446e0', _A, 'cdecl',
          'regra: produto de dois 16.16 pelas metades de 16 bits dos módulos, truncado (sem arredondar, estoura em volta), com o sinal do produto',
          'rule: product of two 16.16 numbers by the 16-bit halves of the magnitudes, truncated (no rounding, wraps), with the product sign'),
         ('f26dot6_mul', 'f26dot6_mul', '0x944760', _A, 'cdecl',
          'regra: produto dividido por 64 (26.6) sem intermediário de 64 bits, truncado, com o sinal do produto',
          'rule: product divided by 64 (26.6) with no 64-bit intermediate, truncated, with the product sign'),
         ('f2dot14_mul', 'f2dot14_mul', '0x9447e0', _A, 'cdecl',
          'regra: valor de 32 bits vezes um fator 2.14 (os 16 bits baixos do segundo argumento), deslocado 14 à direita, truncado, com o sinal do produto',
          'rule: a 32-bit value times a 2.14 factor (the low 16 bits of the second argument), shifted right 14, truncated, with the product sign'),
         ('fixed_div', 'fixed_div', '0x944840', _A, 'cdecl',
          'regra: quociente de dois 16.16, truncado, com o sinal do quociente; o resto é reduzido a 16 bits junto com o divisor antes da parte fracionária; divisor zero derruba (div)',
          'rule: quotient of two 16.16 numbers, truncated, with the quotient sign; the remainder is cut to 16 bits together with the divisor before the fraction; a zero divisor faults (div)'),
         ('fixed_hypot', 'fixed_hypot', '0x9448b0', _A, 'cdecl',
          'regra: o comprimento do vetor (x, y) em 16.16: com uma componente zero, o módulo da outra; senão o chute máx + mín/2 e três passos de Newton g = (g + x·(x/g) + y·(y/g) + 1) / 2',
          'rule: the length of the vector (x, y) in 16.16: with one component zero, the other\'s magnitude; else the guess max + min/2 and three Newton steps g = (g + x·(x/g) + y·(y/g) + 1) / 2')])
CAMPOS = []
INTERESSA = []
MUTACOES = []
