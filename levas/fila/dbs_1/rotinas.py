# Do Database.cpp: o nome pelos ids (0x53e140: ~126 chamadores, notícias, telas, contratos, disciplina; o texto pela
# 0x53e2e0 e o apelido dado por humano pela 0x600740/0x53e990/0x6005e0), o nome do oficial (0x53eab0: eventos e painel
# da partida, chamada pelas cópias Rust 0x6d8d10, 0x6d7b00 e 0x71c270) e o texto da máscara da posição (0x5482f0: mídia
# 0x72d270 e tela da pessoa 0x87dd80). Simulação: as mutações pela carreira até 2002-06-01 e a confirmação no jogo real
# comparando. Sem SEH, sem sorteio; as locais de 0x200 bytes da 0x53e140 e da 0x53eab0 só servem à mensagem da caixa
# (texto nulo, tamanho < 0x69, forma fora de 1..6), caminho que fica com a original; as duas locais da 0x53e140 são
# gravadas antes de lidas. Os comparadores conferem o texto produzido (até 128 bytes) e o valor.
SAVE = False
ROTINAS = [('nome_por_ids', 0x53e140, [0x8b, 0x44, 0x24, 0x10, 0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('nome_oficial', 0x53eab0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('texto_da_mascara_da_posicao', 0x5482f0, [0x53, 0x57, 0x8b, 0x7c, 0x24, 0x10])]
DOCS = ('o nome pelos ids, o nome do oficial e o texto da máscara da posição',
        'the name from the ids, the official name and the position mask text',
        [('nome_por_ids', 'database_format_name_from_ids', '0x53e140', 'Database.cpp', 'cdecl',
          'os registros do nome, sobrenome e nome comum pelos índices ([0xae23f0], [0xae23f4], [0xae23f8]; o apelido dado por humano no lugar do comum) e o texto pela 0x53e2e0',
          'the first, second and common name records by index ([0xae23f0], [0xae23f4], [0xae23f8]; the nickname given by a human instead of the common name) and the text by 0x53e2e0'),
         ('nome_oficial', 'database_format_official_name', '0x53eab0', 'Database.cpp', 'cdecl',
          'o nome do oficial pela forma: 1 "%s %s", 2 "%c.%s", 3 "%s, %s", 4 "%s, %c", 5 o sobrenome, 6 as iniciais; nação [0x9cf488]: só o sobrenome',
          'the official name by format: 1 "%s %s", 2 "%c.%s", 3 "%s, %s", 4 "%s, %c", 5 the surname, 6 the initials; nation [0x9cf488]: surname only'),
         ('texto_da_mascara_da_posicao', 'position_mask_get_text', '0x5482f0', 'Database.cpp', 'cdecl',
          'o nome traduzido da posição da máscara (goleiro, apoio, atacante, ala, pontas, meias, zagueiro, laterais, líbero, reserva, defensor) e o gênero pelo idioma',
          'the translated position name of the mask (goalkeeper, support, striker, wing back, wingers, midfielders, centre back, full backs, sweeper, substitute, defender) and the gender by language')])
CAMPOS = [('[0xae23f0], [0xae23f4], [0xae23f8]: tabelas de nomes (60 bytes); [0xae23bc] pessoas (110 bytes); [0xb63c98] apelidos; lê (nome_por_ids)',
           '[0xae23f0], [0xae23f4], [0xae23f8]: name tables (60 bytes); [0xae23bc] people (110 bytes); [0xb63c98] nicknames; reads (nome_por_ids)'),
          ('oficial +4 nome, +8 sobrenome, +0x16 nação; [0x9cf488] chave da nação; lê (nome_oficial)',
           'official +4 first name, +8 surname, +0x16 nation; [0x9cf488] nation key; reads (nome_oficial)'),
          ('0xad9c64: texto do comentário da posição; lê (texto_da_mascara_da_posicao)', '0xad9c64: position comment text; reads (texto_da_mascara_da_posicao)')]
INTERESSA = ['SGE', 'CMRTE']
