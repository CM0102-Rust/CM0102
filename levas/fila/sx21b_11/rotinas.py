# Numeros nos textos: o ordinal, o numero por extenso e a quebra de linha perto do meio (utils.cpp, a pasta si\code;
# leitor sx21b, rodada 5, 24/11/2026). O ordinal e o extenso entram nas noticias e nos textos de recordes, premios e
# tabelas (centenas de chamadores; ja citados por itens da fila como gra_11..18, sx01e_4, sx09_5); a quebra de linha
# nos eventos da partida (0x6d892f) e na barra de menus (0x75e67e). Item de simulacao (carreira ENG).
# Convencoes conferidas: 0x90c310 cdecl(n u32, genero byte) devolve o buffer 0xde2734 (0 mostra a caixa da linha
# 0x3c6 e da ""); 0x90c520 cdecl(n u32, buffer, tamanho i16, minusculas) (1; sem buffer ou tamanho < 50: a caixa,
# linhas 0x45d e 0x464, e 0; tabelas de saltos de 21 entradas em 0x90ca6c e 0x90cac0); 0x90cb20 cdecl(texto,
# esquerda, direita) (1 ou 0). 0x66f4d0 langlib_get_language (2 frances; 4, 7, 9, 0xa, 0xb sem sufixo em certos
# generos); 0x944c9f sprintf ("%d", "1er", "1re", "%deme"); 0x66f4e0 langlib_sprintf (os sufixos ingleses e as
# palavras traduzidas).
# Comparadores: o buffer do ordinal (0x64 bytes), o texto escrito e o texto quebrado, lado a lado com o valor.
SAVE = False
ROTINAS = [
    ('utils_format_ordinal', 0x90c310, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
    ('utils_number_to_words', 0x90c520, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
    ('utils_break_line_near_middle', 0x90cb20, [0x53, 0x56, 0x8b, 0x74, 0x24, 0x0c]),
]
DOCS = ('números nos textos: o ordinal, o número por extenso e a quebra de linha perto do meio',
        'numbers in texts: the ordinal, the number in words and the line break near the middle',
        [('utils_format_ordinal', 'utils_format_ordinal', '0x90c310', 'utils.cpp', 'cdecl',
          'regra: em francês 1er/1re e %dème (só o número no gênero 1); em algumas línguas só o número; senão st, nd, rd pelo último algarismo e th de 10 a 20 e nos outros; 0 mostra a caixa',
          'rule: in French 1er/1re and %dème (just the number for gender 1); in some languages just the number; otherwise st, nd, rd by the last digit and th from 10 to 20 and for the rest; 0 shows the box'),
         ('utils_number_to_words', 'utils_number_to_words', '0x90c520', 'utils.cpp', 'cdecl',
          'regra: de zero a vinte por extenso (minúsculo ou com maiúscula, traduzido); os outros em algarismos',
          'rule: zero to twenty in words (lower case or capitalised, translated); the others in digits'),
         ('utils_break_line_near_middle', 'utils_break_line_near_middle', '0x90cb20', 'utils.cpp', 'cdecl',
          'regra: do meio para fora, um passo para cada lado por vez (esquerda primeiro), o primeiro espaço vira quebra de linha',
          'rule: from the middle outward, one step each way at a time (left first), the first space becomes a line break')])
CAMPOS = []
INTERESSA = []
