# A fuga do jogador e a notícia dela (awol.cpp), pedido de 02/11 (agente C, família das pessoas):
# - 0x419840 (AWOL::start_absence, 656 bytes, x87 nas reputações, quadro SEH só da data local): o jogador some: a marca
#   na tabela das fugas, as três reputações cortadas até 40%, as antipatias com o técnico, o elenco e o +0xbf do clube,
#   a volta marcada (12 bytes na lista das fugas) e a notícia. Pela falta ao treino (0x41a360), pelo lance de outro clube
#   (0x8c1290), pela volta das férias (0x630ee0), pelo diário (0x5ce2d0) e pela mídia (0x736c00). As mutações estão nela.
# - 0x419ad0 (awol_post_news, 400 bytes): a notícia 3013 da fuga, para todos (reputação mundial acima de 7500) ou para
#   o país do clube e o da nacionalidade: passageira.
# Iguais no cm0102-1080.exe (as duas inteiras, conferido byte a byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x419840: thiscall (mov ecx, [0xad9c6c]; push 4; push ebp; push esi em 0x6311c2..0x6311cc; e as Rust 0x41a360 e
#   0x8c1290), 3 (ret 0xc: jogador, clube, tipo byte). Dentro: 0x54a110 thiscall (lea ecx, a data local), 0; 0x90cfc0
#   cdecl, 1; 0x945640 _ftol; 0x547af0 cdecl (add esp, 0xc), 3 (dono, outro, 1); 0x600a50 cdecl (add esp, 0x20 junto
#   da 0x547af0), 5 (clube, 1, 1, 0, 0); 0x417c10 thiscall (lea ecx, a data), 2; 0x54c5b0 thiscall (lea ecx, a data;
#   push dias; push temporária), 2; 0x68d0d0 thiscall (mov ecx, [this]; push 0xc; push &registro), 2; 0x419ad0 (mov
#   ecx, ebp = this; push valor; push tipo; push clube; push jogador; ret 0x10; ela não lê o ecx), 4.
#   0x419ad0: thiscall como a 0x419840 a chama (o this não é lido), 4. Dentro: 0x77bbe0 thiscall (lea ecx, a notícia
#   local), 2 (0xbc5, 0); 0x785790 thiscall (lea ecx, a notícia), 2; 0x786780 thiscall (mov ecx, [0xb7431c]), 1;
#   0x7863f0 thiscall (mov ecx, [0xb7431c]), 2 (a notícia, a nação).
# Leitura de pilha: nenhuma local lida antes de gravada (o registro da volta é o id e a data montada).
# Comparadores: 0x419840 em ensaio (modelo CPV6_ENSAIO: a cópia sem a lista e sem a notícia, que alocam; depois a
# original do mesmo estado): a marca, o jogador, a ficha, o técnico, o +0xbf e o elenco do clube e o estado do sorteio.
# 0x419ad0 só a original (a entrega aloca a notícia).
SAVE = False
ROTINAS = [('comecar_a_fuga', 0x419840, [0x6a, 0xff, 0x68, 0xb0, 0x3e, 0x95, 0x00]),
           ('noticia_da_fuga', 0x419ad0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a fuga do jogador (as reputações, as antipatias do técnico e do elenco, a volta marcada) e a notícia dela',
        'the player going absent (the reputations, the dislikes of the manager and the squad, the scheduled return) and its news',
        [('comecar_a_fuga', 'AWOL::start_absence', '0x419840', 'awol.cpp', 'thiscall',
          'o jogador some: marca a fuga, corta as três reputações até 40%, cria antipatias com o técnico, o elenco e o auxiliar, marca a volta (2 a 32 dias no tipo 4, senão 21 a 170) e manda a notícia',
          'the player goes absent: marks the absence, cuts the three reputations by up to 40%, creates dislikes with the manager, the squad and the assistant, schedules the return (2 to 32 days for kind 4, else 21 to 170) and posts the news'),
         ('noticia_da_fuga', 'awol_post_news', '0x419ad0', 'awol.cpp', 'thiscall',
          'a notícia da fuga para todos os técnicos (reputação mundial acima de 7500) ou para os do país do clube e da nacionalidade',
          'the absence news to every manager (world reputation above 7500) or to the ones of the club country and of the nationality')])
CAMPOS = [('AWOL ([0xad9c6c]) +0 a lista das fugas (12 bytes: id e a data da volta), +0xc a tabela de marcas por id (bit 1 fugido); valor; lê e grava (comecar_a_fuga)',
           'AWOL ([0xad9c6c]) +0 the absence list (12 bytes: id and the return date), +0xc the flag table per id (bit 1 absent); value; reads and writes (comecar_a_fuga)'),
          ('ficha do jogador +9, +0xb, +0xd reputações [inferência: atual, local, mundial; a +0xd acima de 7500 leva a notícia a todos]; valor; lê e grava',
           'player record +9, +0xb, +0xd reputations [inference: current, home, world; +0xd above 7500 sends the news to everyone]; value; reads and writes'),
          ('pessoa +0x1a nacionalidade, +0x39 clube, +0x59/+0x5b atributos de personalidade [inferência]; clube +0xbf, +0xcf técnico, +0xd7 elenco (50); ponteiro/valor; lê',
           'person +0x1a nationality, +0x39 club, +0x59/+0x5b personality attributes [inference]; club +0xbf, +0xcf manager, +0xd7 squad (50); pointer/value; reads')]
INTERESSA = ['CMRTE', 'SGE']
