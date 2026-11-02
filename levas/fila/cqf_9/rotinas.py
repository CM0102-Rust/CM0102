# A notícia da volta aos treinos e o que ela acerta (injury.cpp), pedido de 02/11 (agente C, família das pessoas):
# - 0x630ee0 (Injury::post_recovery_news, 1648 bytes, quadro SEH da notícia local sem destrutor): a notícia 2001 e, nas
#   categorias de tipo 12, o acerto do tratamento ('M': o registro de forma, a simpatia pelo técnico), da volta da
#   lesão ('O', 'N': a forma e a marca +0xc do registro de 79 bytes) e das férias ('Q': as marcas de férias do
#   contrato, a felicidade do contrato, a simpatia, a fuga 0x419840 e os atributos +0x57/+0x59/+0x5d e a ficha +0x44).
#   9 chamadas na carreira automática até 2002-06-01 (cobertura de 02/11), pelas voltas aos treinos (0x62e010) e pelo
#   fim da lesão (0x62e700), ambas já em Rust. As mutações estão nela.
# - 0x630c50 (injury_get_category_name, 656 bytes): o nome traduzido da categoria da lesão (1 chamada, pelo texto da
#   notícia 0x631580): passageira.
# Iguais no cm0102-1080.exe (as duas inteiras, conferido byte a byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x630ee0: thiscall (a Rust 0x62e010/0x62e700 com o this das lesões [0xb63d28]), 5 na pilha (ret 0x14: jogador,
#   estágio, tipo, forçar, dias; os três do meio são bytes, `forçar` vai inteiro para a entrega e o dia é palavra com
#   sinal). Dentro: 0x7875e0 (mov ecx, [0xb7431c]; push clube; ret 4: o ecx não é lido, stdcall na cópia); 0x77bbe0
#   thiscall (lea ecx, a notícia local), 2 (0x7d1, 0); 0x785790 thiscall (lea ecx, a notícia), 2 (k, valor); 0x90cfc0
#   cdecl, 1; 0x4cf5a0 thiscall (mov ecx, [0xae1050]), 1; 0x8674e0 thiscall (mov ecx, edi = o contrato), 1 (a marca);
#   0x547840 / 0x547810 / 0x547c50 / 0x547c10 cdecl (add esp, 8), 2 (jogador, técnico); 0x547a10 / 0x547af0 cdecl (add
#   esp, 0xc), 3 (jogador, técnico, 1); 0x419840 thiscall (mov ecx, [0xad9c6c]), 3 (jogador, clube, 4); 0x786090
#   thiscall (mov ecx, [0xb7431c]), 6 (a notícia, o clube, 0, a nação, o continente, `forçar` inteiro).
#   0x630c50: stdcall, 3 (ret 0xc: categoria byte, texto, tamanho); quem chama: 0x631884, 0x631cf4, 0x875cbc. Dentro:
#   0x66f4e0 cdecl (add esp, 8), 2; 0x66f4d0 cdecl, 0; a caixa (aviso_em, injury.cpp 0x9e7f28). A tabela de saltos
#   0x630eac (13 casos, em ordem) conferida.
# Leitura de memória: a notícia local é montada pela 0x77bbe0 antes de qualquer leitura; nenhuma local lida antes de
# gravada.
# Comparadores: 0x630ee0 em ensaio (modelo CPV6_ENSAIO: a cópia sem a entrega 0x786090, que aloca, e parando antes da
# fuga; depois a original do mesmo estado): o registro do jogador, a ficha, o contrato, o registro de forma, o registro
# de 79 bytes, o técnico do clube, [0xae1080] e o estado do sorteio. Com a fuga: só a original. 0x630c50 lado a lado
# no texto e na devolução (o tamanho curto: só a original).
SAVE = False
ROTINAS = [('noticia_da_volta_aos_treinos', 0x630ee0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00]),
           ('nome_da_categoria_da_lesao', 0x630c50, [0x8b, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('a notícia da volta aos treinos e o que ela acerta (o tratamento, as férias e a moral do jogador, a relação com o técnico, a fuga) e o nome da categoria da lesão',
        'the back-to-training news and what it settles (the treatment, the holidays and the player morale, the relation with the manager, the absence) and the injury category name',
        [('noticia_da_volta_aos_treinos', 'Injury::post_recovery_news', '0x630ee0', 'injury.cpp', 'thiscall',
          'a notícia da volta do jogador aos treinos leves ou completos; com a categoria de tipo 12: o resultado do tratamento, da volta e das férias (as marcas de férias do contrato, a felicidade, a simpatia pelo técnico, a chance de fugir e o acerto de atributos de personalidade)',
          'the news of the player return to light or full training; with a type 12 category: the outcome of the treatment, the return and the holidays (the contract holiday flags, the happiness, the liking for the manager, the chance of going absent and the personality attribute changes)'),
         ('nome_da_categoria_da_lesao', 'injury_get_category_name', '0x630c50', 'injury.cpp', 'stdcall',
          'o nome traduzido da categoria da lesão (interna, pé, canela, joelho, coxa, virilha, tronco, mão, braço, ombro, pescoço, cabeça, recuperação); devolve o gênero do nome no idioma 3',
          'the translated injury category name (internal, foot, shin, knee, thigh, groin, upper body, hand, arm, shoulder, neck, head, rehabilitation); returns the name gender in language 3')])
CAMPOS = [('pessoa +0x57, +0x59, +0x5b, +0x5d atributos de personalidade [inferência: os que as férias acertam]; valor; lê e grava (noticia_da_volta_aos_treinos)',
           'person +0x57, +0x59, +0x5b, +0x5d personality attributes [inference: the ones the holidays adjust]; value; reads and writes (noticia_da_volta_aos_treinos)'),
          ('ficha do jogador (pessoa +0x61) +0xb valor [inferência], +0x44 [inferência: atributo que as férias baixam; a moral é a +0x45]; valor; lê e grava',
           'player record (person +0x61) +0xb value [inference], +0x44 [inference: attribute the holidays lower; morale is +0x45]; value; reads and writes'),
          ('contrato +0x44 felicidade no contrato (i8, −100..100), +0x45 marcas de férias 0x20, 0x40, 0x20000 (caem pela 0x8674e0); valor; lê e grava',
           'contract +0x44 contract happiness (i8, −100..100), +0x45 holiday flags 0x20, 0x40, 0x20000 (cleared through 0x8674e0); value; reads and writes'),
          ('lesões ([0xb63d28]) [+0] + 31 × id registro de forma: +0x12 (0xff) e +0x13 [inferência: o andamento da recuperação]; valor; grava',
           'injuries ([0xb63d28]) [+0] + 31 × id fitness record: +0x12 (0xff) and +0x13 [inference: the recovery progress]; value; writes'),
          ('[0xae2d0c] + 0x4f × id +0xc bit 0 [inferência: em tratamento]; valor; grava 0; [0x9e7ad9] + 13 × tipo a categoria do tipo de lesão; valor; lê',
           '[0xae2d0c] + 0x4f × id +0xc bit 0 [inference: under treatment]; value; writes 0; [0x9e7ad9] + 13 × type the injury type category; value; reads'),
          ('notícia 2001: dados 0 estágio, 1 tipo, 2 id, 3..5 os nomes, 6 nascimento, 7 ano, 8 dias, 9 resultado, 10 a simpatia; valor; grava',
           'news 2001: data 0 stage, 1 type, 2 id, 3..5 the names, 6 birth, 7 year, 8 days, 9 outcome, 10 the liking; value; writes')]
INTERESSA = ['CMRTE', 'SGE']
