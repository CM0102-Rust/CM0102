# Do Database.cpp, textos de tela que faltavam: o nome da seleção (0x542100: telas do clube e da seleção, escalação
# da partida 0x725f70, já em Rust, hall da fama, impressões, Setup), o nome do atributo do jogador (0x5428a0, com o
# pedaço 0x542f48 do mapa, que é o meio dela: fronteira real 0x5431e0) e o texto das posições (0x53ee50: telas do
# clube, busca, pessoa, tática, treino, impressões). Simulação (SAVE = False). Sem SEH, sem sorteio. As locais de
# 0x200 bytes da 0x542100 e da 0x5428a0 só servem à caixa (tamanho curto, clube nulo, atributo fora da faixa), que
# fica com a original; a 0x53ee50 grava a linha, os lados e a nota mínima antes de ler (as de 0x100 bytes só servem à
# caixa). A 0x53ee50 grava no jogador (+0x13 = 15 quando nada passa e +0x14 >= 15) e usa o texto global [0xde1f64]:
# o comparador confere os dois. Os textos são imediatos do código (push 0x9b....), as cópias usam os mesmos
# endereços; a tabela de salto da 0x5428a0 é do próprio código (conferido pela assinatura da rotina inteira).
SAVE = False
ROTINAS = [('nome_da_selecao', 0x542100, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nome_do_atributo_do_jogador', 0x5428a0, [0x8a, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('texto_das_posicoes', 0x53ee50, [0xb8, 0xa4, 0x11, 0x00, 0x00])]
DOCS = ('o nome da seleção, o nome do atributo do jogador e o texto das posições',
        'the national team name, the player attribute name and the positions text',
        [('nome_da_selecao', 'club_get_national_team_name', '0x542100', 'Database.cpp', 'cdecl',
          'o nome do clube de seleção pela forma (1..4): o nome, "Reserves", "B"/"\'B\'", "U21"/"Under 21"/"Under 21\'s"/"Under 21 Squad" (ou 23) e "Squad"/"Main Squad"',
          'the national team club name by style (1..4): the name, "Reserves", "B"/"\'B\'", "U21"/"Under 21"/"Under 21\'s"/"Under 21 Squad" (or 23) and "Squad"/"Main Squad"'),
         ('nome_do_atributo_do_jogador', 'database_get_player_attribute_name', '0x5428a0', 'Database.cpp', 'cdecl',
          'o nome traduzido dos 31 atributos de jogador, abreviado ou inteiro; devolve 1',
          'the translated name of the 31 player attributes, short or full; returns 1'),
         ('texto_das_posicoes', 'database_format_player_positions', '0x53ee50', 'Database.cpp', 'cdecl',
          'o texto das posições pelas notas (15 a 10) e os lados (15), abreviado ("D/DM RC"), inteiro ou em letras',
          'the positions text from the ratings (15 to 10) and the sides (15), short ("D/DM RC"), full or as letters')])
CAMPOS = [('TClub +0 id, +4 nome longo, +0x37 e +0x52 gênero, +0x38 nome, +0x53 nação; [0xae2350] nações, [0xae235c] clubes, [0xb74318] +0x36 sub-23 por nação; lê (nome_da_selecao)',
           'TClub +0 id, +4 long name, +0x37 and +0x52 gender, +0x38 name, +0x53 nation; [0xae2350] nations, [0xae235c] clubs, [0xb74318] +0x36 under-23 by nation; reads (nome_da_selecao)'),
          ('jogador +0xf..+0x1a notas das posições; lê; +0x13 meia: escreve 15 (texto_das_posicoes); [0xde1f64] texto global: escreve',
           'player +0xf..+0x1a position ratings; reads; +0x13 midfielder: writes 15 (texto_das_posicoes); [0xde1f64] global text: writes')]
INTERESSA = ['SGE', 'CMRTE']
