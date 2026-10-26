# Do Database.cpp, consultas das telas: a comparação do qsort das pessoas pelo sobrenome (0x53ff60: tela de oferta de
# contrato 0x4de5d0), a dos ponteiros de clube pela reputação (0x53ffe0: menu dos times do clube 0x482070 e as abas
# de Escolher Amistosos 0x5a1480/0x5a1e10/0x5a2250) e se a nação do clube está na CEE (0x53d7d0: aba de informações
# da tela do clube, os estrangeiros 0x462070). Simulação (SAVE = False), rotinas só de tela. Sem SEH, sem sorteio;
# as locais de 0x7d0 da 0x53ff60 têm o primeiro byte zerado antes de lidas. A 0x53dba0 (nome) é chamada pelo
# endereço do jogo; a comparação de textos é a `dbr_comparar_textos` (235a).
SAVE = False
ROTINAS = [('ordem_das_pessoas_pelo_sobrenome', 0x53ff60, [0x81, 0xec, 0xa0, 0x0f, 0x00, 0x00]),
           ('ordem_dos_ponteiros_de_clube_pela_reputacao', 0x53ffe0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x4c, 0x24, 0x08]),
           ('nacao_do_clube_na_cee', 0x53d7d0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x40, 0x53])]
DOCS = ('as ordens das pessoas pelo sobrenome e dos clubes pela reputação, e o clube da CEE',
        'the staff order by surname and the club order by reputation, and the EEC club',
        [('ordem_das_pessoas_pelo_sobrenome', 'database_compare_staff_by_surname', '0x53ff60', 'Database.cpp', 'cdecl',
          'comparação do qsort de pessoas pelo nome no estilo 4 (0x53dba0) sem maiúsculas; pessoa nula: texto vazio',
          'qsort comparison of staff by style-4 name (0x53dba0), case-insensitive; null person: empty text'),
         ('ordem_dos_ponteiros_de_clube_pela_reputacao', 'database_compare_clubs_by_reputation', '0x53ffe0', 'Database.cpp', 'cdecl',
          'comparação do qsort de ponteiros de clube pela reputação (+0x80), a maior primeiro; clube nulo: 0',
          'qsort comparison of club pointers by reputation (+0x80), highest first; null club: 0'),
         ('nacao_do_clube_na_cee', 'database_club_nation_is_eec', '0x53d7d0', 'Database.cpp', 'cdecl',
          '1 se a nação do clube (+0x53) está na CEE (+0x7f = 2)',
          '1 when the club nation (+0x53) is in the EEC (+0x7f = 2)')])
CAMPOS = [('TClub +0x80: reputação; curto com sinal; lê (ordem_dos_ponteiros_de_clube_pela_reputacao)', 'TClub +0x80: reputation; signed short; reads (ordem_dos_ponteiros_de_clube_pela_reputacao)'),
          ('TClub +0x53: nação; ponteiro; TNation +0x7f: bloco econômico (2 = CEE); byte; lê (nacao_do_clube_na_cee)',
           'TClub +0x53: nation; pointer; TNation +0x7f: economic bloc (2 = EEC); byte; reads (nacao_do_clube_na_cee)')]
INTERESSA = ['SGE', 'CMRTE']
