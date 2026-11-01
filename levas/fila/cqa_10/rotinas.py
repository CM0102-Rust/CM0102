# A diretoria e os jogadores depois do jogo: os jogadores que ela cita (com a confianca), o veredito da nota na partida
# e o texto dela (manager_manager.cpp).
# - 0x6aaee0 (sem nome no mapa; 10 chamadas ate 2002-06-01 na cobertura de 17/10, pela noticia da diretoria 0x6a5fa0):
#   dois sorteios, as saidas da noticia pela tabela da liga, os jogadores preferidos/rejeitados do tecnico com nota
#   abaixo de 7 e a confianca da diretoria (+6) por eles.
# - 0x6ab670 (get_player_match_rating_verdict, 10 chamadas): o veredito da nota pela leitura do registro da partida.
# - 0x6ab2d0 (build_board_player_performance_text, 1.397 chamadas, pelo texto da diretoria 0x6a69c0): o texto (so
#   alimenta a noticia: passageira das mutacoes).
# Convencoes conferidas nos pontos de chamada da original (licao da 258a):
#   0x53b900 selecao: cdecl (add esp, 4), 1 arg. 0x90cfc0 sorteio: cdecl (os dois saem com um add esp, 8), 1 arg.
#   0x4a4850 linha da tabela: thiscall (mov ecx, [tabela + 8]), 1 arg. 0x548090 preferidos / 0x5480f0 rejeitados: cdecl
#   (add esp, 0xc), 3 args. 0x4cf5a0 contrato: thiscall (mov ecx, [0xae1050]), 1 arg. 0x6ab670 veredito: thiscall (mov
#   ecx, ebp = this; a rotina nao le o ecx), 2 args. 0x9463af / 0x9462bb iteradores de vetor da CRT: stdcall (sem add
#   esp), 5 e 4 args. 0x729600 ler o registro: thiscall (mov ecx, [0xb6b7c0]), 2 args. 0x53dba0 nome: cdecl (add esp,
#   0x10), 4 args. 0x66f4e0 formatar: cdecl (add esp, 0xc no elogio, 0x10 na culpa), 3 ou 4 args.
# Comparadores: 0x6aaee0 lado a lado (sorteador, [0xae1080], diretoria do clube, ids e as 5 saidas; a copia sem contar
# os sorteios); 0x6ab670 pelo valor; 0x6ab2d0 lado a lado nos 0x200 primeiros bytes da saida (sem o primeiro id e com
# um dos outros, a original le a pilha sem valor: so ela). Iguais no cm0102-1080.exe.
SAVE = False
ROTINAS = [('jogadores_que_a_diretoria_cita', 0x6aaee0, [0x81, 0xec, 0xdc, 0x00, 0x00, 0x00]),
           ('veredito_da_nota_na_partida', 0x6ab670, [0x6a, 0xff, 0x68, 0x1b, 0xd0, 0x95, 0x00]),
           ('texto_da_diretoria_sobre_os_jogadores', 0x6ab2d0, [0x8b, 0x4c, 0x24, 0x04, 0x81, 0xec, 0x64, 0x02, 0x00, 0x00])]
DOCS = ('a diretoria e os jogadores depois do jogo: os jogadores que ela cita (com a confiança), o veredito da nota na partida e o texto dela',
        'the board and the players after the match: the players it names (with the confidence), the match rating verdict and its text',
        [('jogadores_que_a_diretoria_cita', 'board_named_players_after_match', '0x6aaee0', 'manager_manager.cpp', 'thiscall',
          'depois do jogo, por sorteio, a posição na liga para a notícia e os jogadores preferidos ou rejeitados do técnico que jogaram mal, com a confiança da diretoria subindo ou caindo por eles',
          'after the match, by a draw, the league position for the news and the manager\'s favoured or rejected players who played badly, with the board confidence rising or falling by them'),
         ('veredito_da_nota_na_partida', 'get_player_match_rating_verdict', '0x6ab670', 'manager_manager.cpp', 'thiscall',
          'acha o jogador entre os titulares do registro da partida: −1 com nota abaixo de 7, 1 acima, senão 0',
          'finds the player among the starters of the match record: −1 for a rating under 7, 1 above, else 0'),
         ('texto_da_diretoria_sobre_os_jogadores', 'build_board_player_performance_text', '0x6ab2d0', 'manager_manager.cpp', 'stdcall',
          'a frase da diretoria elogiando ou culpando o jogador citado, pela variante',
          'the board sentence praising or blaming the named player, by variant')])
CAMPOS = [('diretoria (0x49 bytes em [gerência dos técnicos] + id × 0x49) +6 confiança; valor; lê e grava (jogadores_que_a_diretoria_cita); +0x2c; valor; lê',
           'board (0x49 bytes at [manager manager] + id × 0x49) +6 confidence; value; reads and writes (jogadores_que_a_diretoria_cita); +0x2c; value; reads'),
          ('clube +0xbf técnico (+0x57, +0x69 ficha +0x16), +0x57 divisão, +0x80 reputação; ponteiro/valor; lê', 'club +0xbf manager (+0x57, +0x69 record +0x16), +0x57 division, +0x80 reputation; pointer/value; reads'),
          ('linha da tabela da liga +4 jogos, +0x19, +0x2d, +0x73, +0x74; valor; lê', 'league table row +4 games, +0x19, +0x2d, +0x73, +0x74; value; reads'),
          ('pessoa +0x18 idade, +0x3d cargo, +0x61 ficha (+0xb); valor/ponteiro; lê', 'person +0x18 age, +0x3d job, +0x61 player record (+0xb); value/pointer; reads'),
          ('registro da partida (0xd7a bytes): lados de 0x625 em +0x11f, vagas em +0x71 (0x49 bytes: +0 id, +0x35 nota); valor; lê (veredito_da_nota_na_partida)',
           'match record (0xd7a bytes): 0x625-byte sides at +0x11f, slots at +0x71 (0x49 bytes: +0 id, +0x35 rating); value; reads (veredito_da_nota_na_partida)'),
          ('[0xae1080] contrato consultado; ponteiro; grava', '[0xae1080] contract looked up; pointer; writes')]
INTERESSA = ['CMRTE']
