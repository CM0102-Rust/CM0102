# A tabela de caracteres da lingua: maiuscula, minuscula e a comparacao de nomes (utils.cpp; leitor sx21, rodada 5,
# 24/11/2026). Funcoes-folha usadas pela montagem dos textos (a primeira letra das frases das noticias) e pelas
# ordenacoes de nomes; na carreira ENG, pelas noticias do tecnico humano e pelas listas ordenadas. Se a cadeia mostrar
# que nao rodam ate 2002-06-01, o item vai para as telas (as listas ordenadas das telas de busca e de elenco).
# Convencoes conferidas: 0x90c2d0 e 0x90c2f0 cdecl(c), al (abaixo de 0x20 devolvem o argumento como veio); 0x90c1d0
# cdecl(a, b, diferenciar_maiusculas, tamanho); 0x66f4d0 cdecl, eax (a lingua; 0xa = sem a chave de ordenacao).
# A tabela de 3 bytes por caractere em 0xaa2154 (+0 a chave de ordenacao, +1 a maiuscula, +2 a minuscula).
# Comparadores: pelo valor (al nas duas primeiras).
SAVE = False
ROTINAS = [
    ('utils_char_to_upper', 0x90c2d0, [0x8b, 0x44, 0x24, 0x04, 0x3c, 0x20]),
    ('utils_char_to_lower', 0x90c2f0, [0x8b, 0x44, 0x24, 0x04, 0x3c, 0x20]),
    ('utils_string_compare', 0x90c1d0, [0x51, 0x53, 0x55, 0x56, 0x57]),
]
DOCS = ('a tabela de caracteres da língua: maiúscula, minúscula e a comparação de nomes',
        "the language's character table: upper case, lower case and the name comparison",
        [('utils_char_to_upper', 'utils_char_to_upper', '0x90c2d0', 'utils.cpp', 'cdecl',
          'regra: a maiúscula da tabela da língua (+1 da entrada de 3 bytes); códigos de controle (abaixo de 0x20) ficam como estão',
          "rule: the upper case from the language table (+1 of the 3-byte entry); control codes (below 0x20) stay as they are"),
         ('utils_char_to_lower', 'utils_char_to_lower', '0x90c2f0', 'utils.cpp', 'cdecl',
          'regra: a minúscula da tabela da língua (+2 da entrada); códigos de controle ficam como estão',
          'rule: the lower case from the language table (+2 of the entry); control codes stay as they are'),
         ('utils_string_compare', 'utils_string_compare', '0x90c1d0', 'utils.cpp', 'cdecl',
          'regra: -1, 0 ou 1 pela chave de ordenação da língua (menos na língua 0xa), sem diferenciar maiúsculas se pedido, até o tamanho dado; a mais curta vem antes',
          "rule: -1, 0 or 1 by the language's sort key (except in language 0xa), ignoring case when asked, up to the given length; the shorter comes first")])
CAMPOS = []
INTERESSA = []
