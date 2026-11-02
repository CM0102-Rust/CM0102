# As estatísticas arquivadas (player_stats.cpp), pedido de 02/11 (agente C, família das pessoas):
# - 0x7c5fe0 (PlayerStats::archive_player_stats, 1648 bytes): a cada saída de clube (transferência, empréstimo, fim,
#   aposentadoria, seleção), as seis categorias de estatísticas da temporada vão para o arquivo de histórico (nós de 0x2b
#   bytes pela FILE_LLIST::add_node 0x58e340) e são zeradas; um nó vazio quando a sexta categoria não tem registro; e os
#   jogos e gols pelo clube (+0x20/+0x22 do registro da ficha) recontados do histórico. Chamada pelas Rust 0x4cbd20,
#   0x4d01e0, 0x4d07e0, 0x690730, 0x7c1350 e pela 0x4c7ac0 em toda troca de clube da carreira automática. Item de uma
#   rotina média (brief cqe: rotina grande e passageiras; as vizinhas que restam em player_stats.cpp são de texto e de
#   tela e não rodam).
# Igual no cm0102-1080.exe (inteira, conferido byte a byte).
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
#   0x7c5fe0: thiscall (mov ecx, [0xb74350] nas Rust 0x4cbd20/0x690730 e na 0x4c7ac0), 6 na pilha (ret 0x18: pessoa,
#   valor, empréstimo (byte), fim (byte), clube (id, −1 nenhum), outro). Dentro: 0x669950 cdecl (add esp, 4), 1;
#   0x58e340 thiscall (mov ecx, [this]; push &nó; push reg + 0x1c; push reg + 0x18), 3; 0x669340 cdecl (add esp, 4),
#   1; 0x54cdb0 thiscall (mov ecx, 0xae2c90; push +0x42; push +0x3e), 2; 0x58f120 thiscall (mov ecx, [this]), 2;
#   0x68d1b0 thiscall (mov ecx, a lista), 0; 0x68d040 thiscall, 0; 0x944c94 cdecl (add esp, 4), 1; a caixa (aviso_em).
# Leitura de pilha: o byte do tipo do nó (+0x2a) é lido antes de gravado ((x & 7) | categoria × 8), mas os 3 bits de
#   baixo são trocados pelo tipo logo depois: o valor sai sempre categoria × 8 + tipo, sem sobra.
# Comparador em ensaio (modelo CPV6_ENSAIO): a cópia roda sem gravar no arquivo (os nós que gravaria entram direto na
# soma dos jogos e gols pelo clube), depois a original do mesmo estado; compara a devolução, os seis registros de 32
# bytes e os jogos e gols pelo clube. As posições do histórico (+0x18/+0x1c), que só a gravação move, ficam fora.
SAVE = False
ROTINAS = [('arquivar_estatisticas', 0x7c5fe0, [0x81, 0xec, 0x3c, 0x02, 0x00, 0x00])]
DOCS = ('as estatísticas do jogador arquivadas no histórico (a troca de clube, o empréstimo, a aposentadoria) e os jogos e gols pelo clube recontados',
        'the player stats archived into the history (the club change, the loan, the retirement) and the games and goals for the club recounted',
        [('arquivar_estatisticas', 'PlayerStats::archive_player_stats', '0x7c5fe0', 'player_stats.cpp', 'thiscall',
          'na saída do jogador de um clube: as estatísticas de cada categoria da temporada vão para o arquivo de histórico (com o ano, o clube, o valor e o tipo da saída) e são zeradas; um nó vazio quando falta a última categoria; e os jogos e gols pelo clube recontados do histórico',
          'when the player leaves a club: the stats of each category of the season go to the history file (with the year, the club, the fee and the kind of exit) and are cleared; an empty node when the last category is missing; and the games and goals for the club recounted from the history')])
CAMPOS = [('PlayerStats ([0xb74350]) +4 registros de 32 bytes das categorias (zerados), +0x30 registros de 39 bytes por ficha (+0..+0x14 as seis categorias, −1 sem, +0x18/+0x1c o primeiro e o último nó do histórico, +0x20 jogos e +0x22 gols pelo clube); valor; lê e grava (arquivar_estatisticas)',
           'PlayerStats ([0xb74350]) +4 32-byte category records (cleared), +0x30 39-byte records per player record (+0..+0x14 the six categories, −1 none, +0x18/+0x1c the first and last history node, +0x20 games and +0x22 goals for the club); value; reads and writes (arquivar_estatisticas)'),
          ('nó do histórico (0x2b): +0 ano, +2 clube, +6 os 32 bytes da categoria (+6 jogos, +8 gols), +0x26 valor da transferência, +0x2a categoria × 8 + tipo (0 transferência, 1 fim, 2 empréstimo, 3 outro); valor; grava no arquivo',
           'history node (0x2b): +0 year, +2 club, +6 the 32 category bytes (+6 games, +8 goals), +0x26 transfer fee, +0x2a category × 8 + kind (0 transfer, 1 end, 2 loan, 3 other); value; writes to the file'),
          ('pessoa +0x39 clube, +0x3e/+0x42 a data de entrada no clube, +0x61 ficha; [0xb63d8a] + 0x48 × linha da nação o começo (+0/+2) e o fim (+0xf/+0x11) da temporada; valor; lê',
           'person +0x39 club, +0x3e/+0x42 the club joined date, +0x61 record; [0xb63d8a] + 0x48 × nation row the season start (+0/+2) and end (+0xf/+0x11); value; reads')]
INTERESSA = ['CMRTE', 'SGE']
