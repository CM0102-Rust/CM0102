# As regras por nação do Database.cpp com nomes fixos: o clube fora da Argentina (0x5487b0: disciplina 0x55e300,
# diário dos técnicos 0x6ab800, notícias 0x782f70), fora da Argentina, França e Croácia (0x5487d0: diário dos
# técnicos) e a nação da Melanésia (0x53d900: as copas da Oceania 0x798220 e 0x799e30). Simulação: as mutações pela
# carreira até 2002-06-01 e a confirmação no jogo real comparando. Sem SEH, sem sorteio, sem variável local, sem
# chamada. As duas do clube devolvem a resposta em `al` com o resto do eax vindo do id da nação: a cópia reproduz o
# eax inteiro.
SAVE = False
ROTINAS = [('clube_fora_da_argentina', 0x5487b0, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('clube_fora_de_argentina_franca_croacia', 0x5487d0, [0x8b, 0x44, 0x24, 0x04, 0x85, 0xc0]),
           ('nacao_da_melanesia', 0x53d900, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x0d, 0x1c, 0xf4, 0x9c, 0x00])]
DOCS = ('as regras por nação: o clube fora da Argentina, fora da Argentina, França e Croácia, e a nação da Melanésia',
        'the nation rules: club outside Argentina, outside Argentina, France and Croatia, and Melanesian nation',
        [('clube_fora_da_argentina', 'club_nation_is_not_argentina', '0x5487b0', 'Database.cpp', 'cdecl',
          '1 se a nação do clube (+0x53) não é a Argentina (chave 0x9cf218); clube ou nação nulos: 0; o resto do eax é o do id',
          '1 when the club nation (+0x53) is not Argentina (key 0x9cf218); null club or nation: 0; the rest of eax comes from the id'),
         ('clube_fora_de_argentina_franca_croacia', 'club_nation_not_argentina_france_croatia', '0x5487d0', 'Database.cpp', 'cdecl',
          '1 se a nação do clube não é Argentina, França nem Croácia (chaves 0x9cf218, 0x9cf300, 0x9cf2ac); nulos: 0',
          '1 when the club nation is not Argentina, France or Croatia (keys 0x9cf218, 0x9cf300, 0x9cf2ac); null: 0'),
         ('nacao_da_melanesia', 'database_is_melanesian_nation', '0x53d900', 'Database.cpp', 'cdecl',
          '1 se a nação é uma das cinco melanésias (chaves 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500)',
          '1 when the nation is one of the five Melanesian ones (keys 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500)')])
CAMPOS = [('TClub +0x53: nação; ponteiro; lê (clube_fora_da_argentina, clube_fora_de_argentina_franca_croacia)',
           'TClub +0x53: nation; pointer; reads (clube_fora_da_argentina, clube_fora_de_argentina_franca_croacia)'),
          ('TNation +0: id; inteiro; lê (as três). Chaves 0x9cf218, 0x9cf300, 0x9cf2ac (Argentina, França, Croácia), 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500 (Melanésia); lê',
           'TNation +0: id; integer; reads (all three). Keys 0x9cf218, 0x9cf300, 0x9cf2ac (Argentina, France, Croatia), 0x9cf41c, 0x9cf478, 0x9cf530, 0x9cf2f8, 0x9cf500 (Melanesia); reads')]
INTERESSA = ['SGE', 'CMRTE']
