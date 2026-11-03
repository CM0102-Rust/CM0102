# O player_setup.cfg (player_regen.cpp), pedido de 02/11 (agente C, família das pessoas):
# - 0x7a50f0 (player_setup_find_staff, 656 bytes): a pessoa de cada linha pelos nomes (em todas as pessoas ou no
#   elenco e no técnico do clube nomeado). 290 chamadas na carreira automática até 2002-06-01 (cobertura de 02/11: o
#   player_setup.cfg do jogo_testes aplicado na criação do jogo pela 0x7a3a10).
# - 0x7a5090 (player_setup_read_quoted_token, 80 bytes): o texto entre aspas de cada campo da linha (3694 chamadas).
# - 0x7a3140 (player_setup_type_from_name, 400 bytes): o tipo da linha (2320 chamadas).
# As três mudam quais lesões, suspensões, aposentadorias, transferências, empréstimos e fugas da criação do jogo são
# aplicadas e a quem: as mutações estão nelas. (A escolha inicial deste item, a fuga AWOL::start_absence 0x419840 e
# a notícia 0x419ad0, foi trocada: a carreira automática de 02/11 não passou por elas.)
# Iguais no cm0102-1080.exe (as três inteiras, conferido byte a byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x7a50f0: cdecl (add esp, 0x14 em 0x7a3b08), 5 (primeiro, comum, segundo, clube, no_elenco byte). Dentro: 0x545880
#   cdecl (add esp, 4), 1 (o clube); 0x946fa0 strncmp cdecl (add esp, 0xc), 3.
#   0x7a5090: cdecl (as chamadas de 0x7a3a63 em diante), 2 (origem, destino), devolve o ponteiro depois do fim.
#   0x7a3140: cdecl (call em 0x7a3a76), 1. Dentro: 0x946fa0 strncmp cdecl (add esp, 0xc), 3 (com 100).
# Leitura de memória: só as linhas e os registros (pessoas, clubes); nenhuma local lida antes de gravada.
# Comparadores: 0x7a50f0 e 0x7a3140 pela devolução (só leem); 0x7a5090 lado a lado no destino e na devolução.
SAVE = False
ROTINAS = [('pessoa_pelo_nome', 0x7a50f0, [0x53, 0x8b, 0x5c, 0x24, 0x14]),
           ('texto_entre_aspas', 0x7a5090, [0x55, 0x8b, 0x6c, 0x24, 0x0c]),
           ('tipo_da_linha', 0x7a3140, [0x56, 0x8b, 0x74, 0x24, 0x08])]
DOCS = ('o player_setup.cfg: a pessoa pelo nome (no clube ou em todas), o texto entre aspas da linha e o tipo da linha pelo nome',
        'the player_setup.cfg: the person by name (in the club or in all), the quoted text of the line and the line type by name',
        [('pessoa_pelo_nome', 'player_setup_find_staff', '0x7a50f0', 'player_regen.cpp', 'cdecl',
          'acha a pessoa de uma linha do player_setup.cfg pelos três nomes: em todas as pessoas ou, com o nome do clube, no elenco (se pedido) e no técnico dele',
          'finds the person of a player_setup.cfg line by the three names: among all people or, with the club name, in its squad (when asked) and its manager'),
         ('texto_entre_aspas', 'player_setup_read_quoted_token', '0x7a5090', 'player_regen.cpp', 'cdecl',
          'copia o próximo texto entre aspas da linha (pulando espaços e quebras antes dele) e devolve onde continuar',
          'copies the next quoted text of the line (skipping spaces and line breaks before it) and returns where to continue'),
         ('tipo_da_linha', 'player_setup_type_from_name', '0x7a3140', 'player_regen.cpp', 'cdecl',
          'o bit do tipo de uma linha do player_setup.cfg (lesão, técnico, aposentadoria, transferência, empréstimo, doping, conduta, passaporte, táticas, daltonismo, idioma, fuga)',
          'the type bit of a player_setup.cfg line (injury, manager, retirement, transfer, loan, doping, misconduct, passport, tactics, colour blindness, language, absence)')])
CAMPOS = [('pessoa ([0xae23bc], 0x6e bytes) +4 primeiro nome, +8 segundo nome, +0xc nome comum (registros de nome que começam pelo texto); ponteiro; lê (pessoa_pelo_nome)',
           'person ([0xae23bc], 0x6e bytes) +4 first name, +8 second name, +0xc common name (name records that start with the text); pointer; reads (pessoa_pelo_nome)'),
          ('clube ([0xae23b4], 0x245 bytes) o nome (0x545880), +0xcf técnico, +0xd7 elenco (50); ponteiro; lê',
           'club ([0xae23b4], 0x245 bytes) the name (0x545880), +0xcf manager, +0xd7 squad (50); pointer; reads')]
INTERESSA = ['CMRTE', 'SGE']
