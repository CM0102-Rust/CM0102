# Os números de camisa (squad_manager.cpp), parte 3: numerar as ligas no começo do jogo (0x859dc0, 1 chamada até
# 2002-06-01 com 8 ligas, na criação do jogo), numerar uma liga (0x859e20, 32) e a inscrição nas copas europeias
# (0x85aae0, ~280) (docs/cobertura_competicoes.txt). Inteiras iguais no cm0102-1080.exe.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x859dc0: 0x829db4 mov ecx, [0xdd7ef0]; call (sem push) -> thiscall(this), ret 0.
# - 0x859e20: 0x859dfa push 0; push 1; push liga; mov ecx, ebp (ebp = o this) -> thiscall(this, liga, inicial, automatico), ret 0xc.
# - 0x85aae0: 0x85aa8a mov ecx, [0xdd7ef0]; push 0 (marca); push etapa; push clube -> thiscall de 3, ret 0xc, devolve al.
# - dentro: o método +0x58 da competição (push 0; push &lista; mov ecx, liga; call [eax+0x58] -> thiscall de 2,
#   devolve ax); 0x540a50 (push 1; push &principal; push clube; add esp, 0xc -> cdecl de 3); 0x541f90 (cdecl de 3);
#   0x7875e0 (mov ecx, [0xb7431c]; push clube -> o ecx não é lido, ret 4: chamada como thiscall com o mesmo ecx);
#   0x85aec0 (push clube; mov ecx, this -> thiscall de 1, original: a notícia do pedido dos números); 0x85a760,
#   0x85a850, 0x859fe0 (thiscall de 2, 3 e 2); 0x9452ca (push; add esp, 4 -> cdecl de 1); 0x4b0700 (push id; add esp, 4
#   -> cdecl de 1); 0x4cf5a0 e 0x4cf6d0 (thiscall de 1 em [0xae1050]); 0x6e2000 (cdecl de 15; as contas de jogadores e
#   de reservas vão com lixo nos bytes altos, a cópia Rust dela lê só o byte).
# Comparadores: 0x859dc0 só a original (numera pelas 0x859e20, que mandam notícia ao técnico humano), com o plano da
# cópia (as ligas a numerar) conferido contra as ligas que a original mandou numerar (gravadas pelo comparador da
# 0x859e20) e o +0 zerado; 0x859e20 lado a lado no +0x3a dos contratos de quem está nos elencos dos clubes da liga e
# dos ligados (só a original quando um clube tem técnico humano e não é automático: a notícia); 0x85aae0 lado a lado
# na marca de inscrição (+0x6d) do elenco, no [0xae1080] e na devolução.
# Determinismo: nada lê pilha, heap ou endereço como valor; a escalação 0x6e2000 não sorteia.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('numerar_no_comeco', 0x859dc0, [0x55, 0x8b, 0xe9, 0x57, 0x33, 0xff]),
           ('numerar_liga', 0x859e20, [0x81, 0xec, 0xd8, 0x00, 0x00, 0x00]),
           ('inscrever_na_europa', 0x85aae0, [0x81, 0xec, 0x9c, 0x02, 0x00, 0x00])]
DOCS = ('os números de camisa: numerar as ligas no começo do jogo e a inscrição nas copas europeias',
        'squad numbers: numbering the leagues at the start of the game and the European cup registration',
        [('numerar_no_comeco', 'SQUAD_MANAGER::init_new_game', '0x859dc0', 'squad_manager.cpp', 'thiscall, ret 0',
          'a regra: no começo do jogo, cada liga de divisão que usa números de camisa (bit 2 das opções) é numerada a partir do banco',
          'the rule: at the start of the game, each division league that uses squad numbers (bit 2 of its options) is numbered from the database'),
         ('numerar_liga', 'SQUAD_MANAGER::assign_league_squad_numbers', '0x859e20', 'squad_manager.cpp', 'thiscall, ret 0xc',
          'a regra: cada clube da liga (fora o time B) é numerado; no começo, cada jogador recebe antes o número do banco (de 0 a 50); o clube de técnico humano não é numerado pelo jogo: os seus números ficam positivos e o técnico recebe o pedido da lista de números',
          'the rule: each club of the league (except B teams) is numbered; at the start each player first gets the database number (0 to 50); a human-managed club is not numbered by the game: its numbers become positive and the manager gets the squad numbers request'),
         ('inscrever_na_europa', 'SQUAD_MANAGER::select_euro_registered_players', '0x85aae0', 'squad_manager.cpp', 'thiscall, ret 0xc (this não lido)',
          'a regra da inscrição nas copas europeias: entram os melhores da escalação de referência (11 titulares e 14 reservas, ou 9 na primeira etapa sem a marca [inferência: lista B]); não podem ser inscritos os emprestados, os de contrato do tipo 5 e os já marcados 8 ou 0x10; na etapa 3, até 2 inscritos fora da escalação saem da lista (mínimo de 21 inscritos); depois da primeira etapa só entra quem cabe nas vagas',
          'the European cup registration rule: the best of the reference line-up are registered (11 starters and 14 substitutes, or 9 at the first stage without the mark [inference: list B]); loaned players, type-5 contracts and those already marked 8 or 0x10 cannot be registered; at stage 3 up to 2 registered players outside the line-up drop out (minimum of 21 registered); after the first stage only those who fit the places get in')])
CAMPOS = [('SQUAD_MANAGER +0 ano das inscrições (word); valor; grava 0 (numerar_no_comeco)', 'SQUAD_MANAGER +0 registration year (word); value; writes 0 (numerar_no_comeco)'),
          ('competição +4 (dword +0 o id), +0x52 opções (bit 2: números de camisa), método +0x58 a lista dos clubes; id e valor; lê (numerar_no_comeco, numerar_liga)',
           'competition +4 (dword +0 the id), +0x52 options (bit 2: squad numbers), method +0x58 the club list; id and value; reads (numerar_no_comeco, numerar_liga)'),
          ('ficha de jogador (pessoa +0x61) +4 número de camisa do banco; valor; lê (numerar_liga)', 'player record (person +0x61) +4 database squad number; value; reads (numerar_liga)'),
          ('pessoa +0x6d marca de inscrição europeia (1 ou 2 inscrito, 4, 8, 0x10) [inferência: as listas A e B]; valor; lê e grava (inscrever_na_europa)',
           'person +0x6d European registration mark (1 or 2 registered, 4, 8, 0x10) [inference: lists A and B]; value; reads and writes (inscrever_na_europa)'),
          ('contrato +0x35 tipo (& 0x3f; 5 não se inscreve) [inferência]; valor; lê (inscrever_na_europa); [0xae1080] o último contrato achado; ponteiro; grava (inscrever_na_europa)',
           'contract +0x35 type (& 0x3f; 5 cannot register) [inference]; value; reads (inscrever_na_europa); [0xae1080] last contract found; pointer; writes (inscrever_na_europa)')]
INTERESSA = ['CMRTE', 'SGE']
