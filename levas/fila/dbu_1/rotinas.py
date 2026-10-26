# Do Database.cpp, textos que faltavam: o nome pelos textos (0x53e2e0, chamada pela 0x53e140, já em Rust pelo
# endereço do jogo: o desvio vale também lá, ~126 chamadores; e pelas telas do clube e o manager_manager), o nome do
# cargo (0x53ad00: telas, mídia, regens e transferências 0x8dc370) e o nome do idioma (0x53cee0: busca e tela da
# pessoa). Simulação (SAVE = False). Sem SEH, sem sorteio; as locais de 0x200 bytes da 0x53e2e0 só servem à caixa
# (texto nulo, tamanho < 0x69, forma fora de 1..7), que fica com a original; a 0x53ad00 e a 0x53cee0 não têm locais.
# Os textos são imediatos do código (push 0x9b....), não tabelas do .rdata: as cópias usam os mesmos endereços. Os
# comparadores conferem o texto produzido (o do argumento ou os globais [0xae19b4] e [0xae21d4]), o byte do gênero
# e o valor.
SAVE = False
ROTINAS = [('nome_pelos_textos', 0x53e2e0, [0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('nome_do_cargo', 0x53ad00, [0x0f, 0xbe, 0x44, 0x24, 0x04]),
           ('nome_do_idioma', 0x53cee0, [0x0f, 0xbe, 0x44, 0x24, 0x04])]
DOCS = ('o nome pelos textos, o nome do cargo e o nome do idioma',
        'the name from strings, the job name and the language name',
        [('nome_pelos_textos', 'database_format_name_from_strings', '0x53e2e0', 'Database.cpp', 'cdecl',
          'o nome da pessoa pela forma (1..7) a partir do nome, sobrenome e nome comum: "%s %s", "%c.%s", "%s, %s", "%s, %c", só um deles ou as iniciais',
          'the person name by format (1..7) from the first, second and common names: "%s %s", "%c.%s", "%s, %s", "%s, %c", one of them or the initials'),
         ('nome_do_cargo', 'database_get_job_name', '0x53ad00', 'Database.cpp', 'cdecl',
          'o nome traduzido do cargo (0..0x11), curto ou inteiro, minúsculo ou maiúsculo, em [0xae19b4]; o gênero com o idioma 4',
          'the translated job name (0..0x11), short or full, lower or upper case, in [0xae19b4]; the gender with language 4'),
         ('nome_do_idioma', 'database_get_language_name', '0x53cee0', 'Database.cpp', 'cdecl',
          'o nome traduzido do idioma (1..64, senão "Unknown") em [0xae21d4]',
          'the translated language name (1..64, else "Unknown") in [0xae21d4]')])
CAMPOS = [('[0xae19b4]: texto do nome do cargo; escreve (nome_do_cargo); [0xae21d4]: texto do nome do idioma (100 bytes); escreve (nome_do_idioma)',
           '[0xae19b4]: job name text; writes (nome_do_cargo); [0xae21d4]: language name text (100 bytes); writes (nome_do_idioma)')]
INTERESSA = ['SGE', 'CMRTE']
