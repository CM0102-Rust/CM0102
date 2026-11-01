# O fim da falta ao treino e da ausencia sem licenca e a noticia da falta ao treino (awol.cpp):
# - 0x4197c0 (AWOL::clear_missed_training): pela falta ao treino de quem ficou sem contrato (0x418ff0, que os jogadores
#   do dia 0x5a69b0 consultam) e pelas aposentadorias (0x7a59e0); 242 chamadas na carreira automatica ate 2002-06-01
#   (simular.py --cobertura, motor ligado, 01/11/2026). Troca o bit 2 da marca e tira o registro da lista das faltas.
# - 0x4196a0 (AWOL::clear_awol): o mesmo para a ausencia (bit 1, lista [this]); 1212 chamadas (quase todas sem o bit:
#   ninguem sumiu na carreira, a 0x419840 nao rodou).
# - 0x41a630 (awol_post_missed_training_news): a noticia da falta ao treino; 81 chamadas.
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   As proprias: 0x4196a0 thiscall (mov ecx, edi / [0xad9c6c]; push pessoa; ret 4), 0x4197c0 thiscall (o mesmo; ret 4),
#   0x41a630 thiscall (push clube; push jogador; mov ecx, ebp em 0x41a4fb; ret 8).
#   Nas duas primeiras: 0x68d1b0 thiscall (mov ecx, [edi] / [edi+4]), 0x68d1f0 thiscall (push item; mov ecx, lista;
#   ret 4). Na 0x41a630: 0x77bbe0 thiscall (push 0; push 0xbc9; lea ecx, noticia), 0x785790 thiscall (push v; push k;
#   lea ecx, noticia; ret 8), 0x90cfc0 cdecl (push n; add esp, 4), 0x600a50 cdecl (5 pushes; add esp, 0x14: (clube, 0,
#   1, 0, 0) e (clube, 1, 1, 0, 0)), 0x7875e0 thiscall (mov ecx, [0xb7431c]; push clube; ret 4), 0x7861e0 thiscall
#   (push clube +0x39; push noticia; mov ecx, [0xb7431c]; ret 8), 0x7863f0 thiscall (push nacao; push noticia; mov ecx,
#   [0xb7431c]; ret 8).
# Comparadores: 0x4196a0 e 0x4197c0 so a original (a 0x68d1f0 libera o no tirado), com o plano montado antes: a marca
# depois e os itens que ficam na lista. 0x41a630 so a original (a entrega aloca na caixa) com os sorteios rastreados; a
# copia em ensaio refaz o caminho com a fita (sem entregar) e confere que os sorteios dela sao o comeco da fita com os
# mesmos n. Iguais no cm0102-1080.exe (as tres inteiras). Deterministicas: nenhuma le pilha sem valor nem usa endereco
# como valor (a noticia local e construida pela 0x77bbe0 antes de qualquer leitura).
SAVE = False
ROTINAS = [('limpar_ausencia', 0x4196a0, [0x55, 0x8b, 0x6c, 0x24, 0x08]),
           ('limpar_falta_ao_treino', 0x4197c0, [0x55, 0x8b, 0x6c, 0x24, 0x08]),
           ('noticia_da_falta_ao_treino', 0x41a630, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('o fim da falta ao treino e da ausência sem licença e a notícia da falta ao treino',
        'the end of the missed training and of the absence without leave and the missed training news',
        [('limpar_ausencia', 'AWOL::clear_awol', '0x4196a0', 'awol.cpp', 'thiscall',
          'a ausência acaba: com o bit 1 da marca da pessoa, o bit troca e o registro dela sai da lista das ausências',
          'the absence ends: with bit 1 of the person\'s flag, the bit flips and her record leaves the absence list'),
         ('limpar_falta_ao_treino', 'AWOL::clear_missed_training', '0x4197c0', 'awol.cpp', 'thiscall',
          'a falta ao treino acaba: com o bit 2 da marca da pessoa, o bit troca e o registro dela sai da lista das faltas',
          'the missed training ends: with bit 2 of the person\'s flag, the bit flips and her record leaves the missed training list'),
         ('noticia_da_falta_ao_treino', 'awol_post_missed_training_news', '0x41a630', 'awol.cpp', 'thiscall',
          'a notícia 0xbc9 da falta ao treino: o jogador, o clube, sorteio(3), se o clube é de humano e quem relata (o técnico do clube da IA, ou por sorteio um dos 5 treinadores ou o auxiliar); ao clube do jogador se ele é de humano, senão à nação com a reputação do jogador acima de 7250',
          'the 0xbc9 missed training news: the player, the club, a draw of 3, whether the club is human-run and who reports it (the AI club\'s manager, or by draw one of the 5 coaches or the assistant); to the player\'s club if it is human-run, else to the nation when the player\'s reputation is above 7250')])
CAMPOS = [('AWOL +0xc tabela de marcas (um byte por pessoa: bit 1 ausente, bit 2 faltou ao treino); valor; lê e grava (limpar_ausencia, limpar_falta_ao_treino)',
           'AWOL +0xc flag table (one byte per person: bit 1 absent, bit 2 missed training); value; reads and writes (limpar_ausencia, limpar_falta_ao_treino)'),
          ('AWOL +0 lista das ausências, +4 lista das faltas (LLIST: +0 conta, +2 cabeça, +0xa cursor; registro +0 id da pessoa); ponteiro; lê e grava (limpar_ausencia, limpar_falta_ao_treino)',
           'AWOL +0 absence list, +4 missed training list (LLIST: +0 count, +2 head, +0xa cursor; record +0 person id); pointer; reads and writes (limpar_ausencia, limpar_falta_ao_treino)'),
          ('pessoa +0 id, +4/+8/+0xc chaves (+0x33), +0x39 clube, +0x61 ficha de jogador (+0xb reputação); id/ponteiro/valor; lê (noticia_da_falta_ao_treino)',
           'person +0 id, +4/+8/+0xc keys (+0x33), +0x39 club, +0x61 player record (+0xb reputation); id/pointer/value; reads (noticia_da_falta_ao_treino)'),
          ('clube +0xcf técnico, +0xd3 auxiliar, +0x19f os 5 treinadores, +0x53 nação; ponteiro; lê (noticia_da_falta_ao_treino)',
           'club +0xcf manager, +0xd3 assistant, +0x19f the 5 coaches, +0x53 nation; pointer; reads (noticia_da_falta_ao_treino)')]
INTERESSA = ['CMRTE']
