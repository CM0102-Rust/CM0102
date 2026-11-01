# Os olheiros dos tecnicos humanos na partida e a sessao de edicao das buscas (scout_manager.cpp):
# - 0x7f8c00 (SCOUT_MANAGER::scout_match): pelo fechamento da partida do dia 0x6b5f00 (Rust, fechar_partida_do_dia);
#   5160 chamadas na carreira automatica ate 2002-06-01 (docs/cobertura_simulacao.txt), com 50390 da 0x7b5be0 abaixo
#   (o tecnico humano de ferias tem os 10 observadores). Sorteia a ordem dos observadores (0x90cfc0).
# - 0x7f6be0 (SCOUT_MANAGER::get_edit_session) e 0x7f6af0 (SCOUT_MANAGER::set_edit_session): passageiras (pelas
#   telas das buscas e pela 0x7afeb7; nao rodam ate 2002-06-01).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   As proprias: 0x7f8c00 thiscall (mov [esp + 0x10], ecx na entrada, usado como o this da 0x7f6cd0 e da 0x7f9890;
#   ret 8: partida, resultado); 0x7f6be0 thiscall, 1 arg (push ebx; mov ecx, [0xb74448] em 0x7afead; ret 4); 0x7f6af0
#   thiscall, 2 args (push 0; push esi em 0x8085b1: sessao, tecnico; ret 8).
#   Na 0x7f8c00: 0x68d020/0x68d040 thiscall (lea ecx, lista local), 0x945501 calloc cdecl (push 4; push 0xa; add
#   esp, 8), 0x90cfc0 sorteio cdecl (push 0xa; add esp, 4), 0x9452ca free cdecl (add esp, 4), 0x7f6cd0 thiscall (push
#   pessoa; push o dword do observador; mov ecx, [esp + 0x18] = this; ret 8; le o byte), 0x7ae9c0 thiscall (push 1;
#   push 0; push -1 tres vezes; mov ecx, esi = registro; ret 0x14), 0x7b5be0 thiscall (push &lista; push resultado;
#   push partida; mov ecx, esi; ret 0xc), 0x7f9890 thiscall (push &lista; mov ecx, this; ret 4). Nas 0x7f6be0/0x7f6af0,
#   so a caixa (pela original).
# Comparadores: 0x7f8c00 so a original (a observacao, as buscas comecadas e os relatorios entregues nao podem rodar
# duas vezes; ela mesma so grava o vetor e a lista locais, que saem), conferindo a devolucao; 0x7f6be0 pelo valor e
# 0x7f6af0 lado a lado pelas 16 sessoes e a devolucao (com a caixa, so a original). Iguais no cm0102-1080.exe (as tres
# inteiras).
SAVE = False
ROTINAS = [('olheiros_na_partida', 0x7f8c00, [0x6a, 0xff, 0x68, 0xd8, 0x0f, 0x96, 0x00]),
           ('sessao_de_edicao', 0x7f6be0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('escolher_sessao_de_edicao', 0x7f6af0, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('os olheiros dos técnicos humanos na partida e a sessão de edição das buscas',
        'the human managers\' scouts at the match and the searches edit session',
        [('olheiros_na_partida', 'SCOUT_MANAGER::scout_match', '0x7f8c00', 'scout_manager.cpp', 'thiscall',
          'os olheiros na partida: para cada técnico humano, os 10 observadores numa ordem sorteada; o do clube que joga começa a observação, e cada um observa a partida; no fim os relatórios vão',
          'the scouts at the match: for each human manager, the 10 scouts in a drawn order; one for the playing club starts the assignment, and each watches the match; at the end the reports go out'),
         ('sessao_de_edicao', 'SCOUT_MANAGER::get_edit_session', '0x7f6be0', 'scout_manager.cpp', 'thiscall',
          'a sessão de edição das buscas do técnico humano',
          'the human manager\'s searches edit session'),
         ('escolher_sessao_de_edicao', 'SCOUT_MANAGER::set_edit_session', '0x7f6af0', 'scout_manager.cpp', 'thiscall',
          'grava a sessão de edição das buscas do técnico humano e devolve 1',
          'stores the human manager\'s searches edit session and returns 1')])
CAMPOS = [('pessoa +0x5f técnico humano (1), +0x39 clube; valor/ponteiro; lê (olheiros_na_partida)',
           'person +0x5f human manager (1), +0x39 club; value/pointer; reads (olheiros_na_partida)'),
          ('registro de observação +0x19d busca em curso; valor; lê (olheiros_na_partida)',
           'scout record +0x19d search in progress; value; reads (olheiros_na_partida)'),
          ('partida +0x1c/+0x20 clubes, +0x40/+0x41 marcas; ponteiro/valor; lê (olheiros_na_partida)',
           'fixture +0x1c/+0x20 clubs, +0x40/+0x41 flags; pointer/value; reads (olheiros_na_partida)'),
          ('SCOUT_MANAGER +0xc sessões dos 16 humanos; ponteiro; lê e grava (sessao_de_edicao, escolher_sessao_de_edicao)',
           'SCOUT_MANAGER +0xc sessions of the 16 humans; pointer; reads and writes (sessao_de_edicao, escolher_sessao_de_edicao)')]
INTERESSA = ['CMScout']
