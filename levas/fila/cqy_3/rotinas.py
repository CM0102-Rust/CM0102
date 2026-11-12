# Os nomes das rodadas, grupos e fases e o rótulo competição + fase + rodada (comp_util.cpp; leitor cqy, 12/11/2026).
# Rodam 270 e 9 vezes na carreira de 8 ligas até 2002-06-01 e 596 e 182 vezes na de ENG (docs/cobertura_*.txt): nas
# notícias das copas e das ligas (0x4b4420, 0x4b4590, 0x4b0b80) e nas telas.
# - 0x4b4850 comp_util_get_stage_name (24.736 bytes): quase tudo é um switch por forma e por faixa de id que escreve um
#   texto fixo. A tabela (forma, id) -> texto foi extraída seguindo a própria original com o unicorn (scratchpad
#   cqy/emu.py e stage_name.py: cada id de −40 a 0x6ff e mais 0x7fff, −0x8000, 0x1000 e 0x2000, nas formas 0, 1, 2, 3, 4,
#   0x80 e 0xff, com as chamadas trocadas por registradores): as formas 0, 1 e 3 têm 322, 322 e 321 ids de 1 a 1155
#   (cada um um langlib_sprintf(buf, texto) sem argumentos); o id 0 é sprintf(buf, [0xad9c64]); os outros escrevem
#   "Cannot find index %d" (o id com sinal) em 0xde1f64; as outras formas dão a caixa da linha 0x15e2; tamanho < 30, a
#   caixa da linha 0x110f; a única leitura fora da pilha e do código é o texto vazio 0xad9c64 (copiado no começo).
#   Os comparadores de faixa (cmp ax, 0x32/0x64/0xa0/0x3e8/0x3fc/0x40c/0x418/0x41f) ficam todos dentro da faixa varrida.
# - 0x4bfa30 comp_util_format_comp_stage_round_label (1.344 bytes): lido instrução a instrução (a desmontagem; o
#   pseudo-C do Ghidra confere a estrutura).
# Iguais no cm0102-1080.exe (inteiras).
# Convenções conferidas: 0x4b4850 (4 pushes: forma, id, tamanho, saída; add esp, 0x10 -> cdecl; devolve a saída);
# 0x4bfa30 (6 pushes; add esp -> cdecl; devolve 1 ou 0); dentro: 0x66f4e0 langlib_sprintf e 0x944c9f sprintf (cdecl
# variádicas: 2 argumentos nos textos fixos, 3 no "Cannot find index", 8, 10 e 11 nos rótulos), 0x4ba8f0 (push forma;
# push id -> cdecl, devolve o byte), 0x66f4d0 (cdecl, sem argumentos), 0x6701f0 (5 pushes -> cdecl), 0x9465fc tolower
# (push; add esp, 4 -> cdecl), a caixa (show_error_at, fonte 0x99ec20).
# Defeito do original: no rótulo, uma forma fora de 1, 2 e 3 usa os três textos sem escrevê-los (lixo da pilha); a
# cópia usa textos vazios (o comparador acusaria se acontecesse).
# Comparadores: lado a lado (são puras: escrevem só no texto de quem chama e, no "Cannot find index", o mesmo texto em
# 0xde1f64): a original no texto de quem chama, a cópia num texto próprio do mesmo tamanho (no rótulo, começando com os
# bytes de quem chama, porque sem fase nem rodada nem competição ele fica como estava); os textos (e as devoluções)
# iguais. Nos casos da caixa, só a original.
# Determinismo: só constantes e os registros da competição.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('comp_util_get_stage_name', 0x4b4850, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('comp_util_format_comp_stage_round_label', 0x4bfa30, [0x81, 0xec, 0x30, 0x03, 0x00, 0x00])]
DOCS = ('os nomes das rodadas, grupos e fases e o rótulo competição + fase + rodada',
        'the names of the rounds, groups and stages, and the competition, stage and round label',
        [('comp_util_get_stage_name', 'comp_util_get_stage_name', '0x4b4850', 'comp_util.cpp', 'cdecl',
          'regra: o nome de uma rodada, grupo ou fase pelo código (1 a 1155): forma 0 o nome completo ("First Round"), 1 o curto ("1st Rnd"), 3 o plural; 322, 322 e 321 códigos com nome; o código 0 dá o texto vazio do jogo; um código sem nome deixa o texto vazio e escreve "Cannot find index N" no texto global; outra forma ou um texto com menos de 30 bytes dão a caixa de erro',
          'rule: the name of a round, group or stage by its code (1 to 1155): form 0 the full name ("First Round"), 1 the short one ("1st Rnd"), 3 the plural; 322, 322 and 321 codes have a name; code 0 gives the game\'s empty text; a code without a name leaves the text empty and writes "Cannot find index N" into the global text; another form or a text under 30 bytes give the error box'),
         ('comp_util_format_comp_stage_round_label', 'comp_util_format_comp_stage_round_label', '0x4bfa30', 'comp_util.cpp', 'cdecl',
          'regra: o rótulo "competição fase rodada": forma 1 com o nome longo da competição, 2 e 3 com o curto (3 com os nomes das fases no plural); só as partes com código; sem competição, só fase e rodada; em alemão, o nome da competição no genitivo ("s") e os nomes das fases em minúsculas; a Regionalliga com as fases 0x453 e 0x456 vira a divisão Norte e a Sul; sem texto ou com menos de 0x67 bytes, a caixa de erro e 0',
          'rule: the label "competition stage round": form 1 with the competition\'s long name, 2 and 3 with the short one (3 with the plural stage names); only the parts with a code; with no competition, only stage and round; in German, the competition name in the genitive ("s") and the stage names in lower case; the Regionalliga with stages 0x453 and 0x456 becomes the North and the South division; with no text or under 0x67 bytes, the error box and 0')])
CAMPOS = [('competição (registro de 0x6b bytes da tabela 0xae23d0): +4 nome longo, +0x37 gênero do nome longo, +0x38 nome curto, +0x52 gênero do nome curto [inferência no gênero: o byte vai em pares para o langlib]; valor; lê',
           'competition (0x6b-byte record of table 0xae23d0): +4 long name, +0x37 long name gender, +0x38 short name, +0x52 short name gender [inference on the gender: the byte goes in pairs to the langlib]; value; reads')]
INTERESSA = []
