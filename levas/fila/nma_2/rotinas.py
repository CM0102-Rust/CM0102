# A passada diaria das 35 entradas de temporada (game_season_update_pass 0x5d5ae0, game.cpp; lista nma, 12/11/2026).
# Chamada uma vez por dia pelo laco do dia. Igual no cm0102-1080.exe (os 1072 bytes conferidos). Convencoes conferidas
# na desmontagem (0x5d5ae0..0x5d5f04):
# - a rotina: cdecl(void), ret 0, quadro SEH (so para o destrutor do LLIST local que nao existe aqui: o estado -1/0
#   em [esp+0x128] cerca o operator new do SimulatedStats);
# - 0x54a110 Date::Date (ecx = 0xae2c80) e 0x94627a atexit(0x5d5f10) na primeira vez (bit 0 de 0xae2c89);
# - 0x54c5b0 Date::add_days: push 1; push saida; mov ecx, 0xae2c90 -> ret 8;
# - 0x66f4e0 langlib_sprintf(buf, "Updating game data") cdecl; 0x802f80: push -1; push peso; push passo; push buf;
#   mov ecx, 0xb74450 -> ret 0x10;
# - 0x549f70 Date::Date(dia, mes, ano, dia da semana): ecx = data local -> ret 0x10; os bits altos dos registros
#   empurrados (lixo de [esi], do retorno do 0x417c10 e de edx) sao reproduzidos menos os de endereco de pilha; o chamado
#   le dia, ano e dia da semana como palavra e o mes como byte (0x54a2d0 e 0x54c8e0 testam si);
# - 0x417c10 Date::operator=(saida, origem): ecx = destino -> ret 8, devolve a saida, ecx = 0 e edx = o dword do destino;
# - vtable da competicao: +0xc all_stages_complete (bool em al), +0x2c skip_season_reset (eax; EuroChampQual e
#   WcConcacafCup marcam a si mesmas), +8 START_NEW_SEASON; 0x4b0700 comp_util_is_league_division(id) cdecl;
#   0x5d6280 (media de publico, cdecl(comp)) por estagio ligado em +0x20;
# - 0x556210: mov ecx, [0xae28f0]; push [comp+4] -> ret 4; 0x84be10 (ecx = SimulatedStats), 0x944c94 delete,
#   0x944cf1 new(8), 0x84b940: push 0; ecx = novo -> ret 4 (devolve this);
# - o dia anterior: 0x447880 (ecx = [0xad9dac]), 0x7c1350 (ecx = [0xb74350]), 0x59cdd0 (ecx = [0xae2a38]) e 0x86d6e0
#   (ecx = [0xdd7f00]), todos push entrada -> ret 4.
# As rotinas chamadas que ja sao Rust (0x556210, 0x7c1350, 0x59cdd0, 0x86d6e0, 0x5d6280) sao chamadas pelo endereco do
# exe: o desvio leva ao Rust (e ao comparador no modo de comparacao).
# Comparador: so a original (comeca temporadas e arquiva registros, nao roda duas vezes); a copia se prova no modo Rust.
SAVE = False
ROTINAS = [('game_season_update_pass', 0x5d5ae0, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a passada diaria das temporadas das nacoes',
        'the daily pass of the nations\' seasons',
        [('game_season_update_pass', 'game_season_update_pass', '0x5d5ae0', 'game.cpp', 'cdecl',
          'cada uma das 35 entradas de temporada (a tabela de 72 bytes das ligas) roda no seu dia de atualizacao (+0x15) e arquiva os registros na vespera; o peso da passada e 2 por entrada de hoje e 4 por entrada de amanha, e com peso 10 ou mais e a interface ativa vai o progresso "Updating game data"; no dia: a nao ser que a nacao exista com a liga nao selecionada, ou que o ano de inicio ja seja o de hoje, o ano de inicio e o de fim andam o passo da entrada (+0x1d) e as datas de inicio e fim sao refeitas (dia, mes, ano, dia da semana); a entrada 3 limpa a selecao de elenco (+0x6d) de todas as pessoas; cada competicao da entrada completa em todos os estagios que nao pula o reinicio recomeca: a media de publico de cada estagio de uma divisao de liga, o novo inicio de temporada da competicao, a nova temporada da disciplina para ela e, a primeira vez no dia, o SimulatedStats refeito; na vespera: recordes de clubes, estatisticas dos jogadores, financas e registros da comissao fecham a temporada da nacao',
          'each of the 35 season entries (the 72-byte league table) runs on its update day (+0x15) and archives its records the day before; the weight of the pass is 2 per entry of today and 4 per entry of tomorrow, and with a weight of 10 or more and the front end active the "Updating game data" progress is shown; on the day: unless the nation exists with its league not selected, or the start year is already today\'s, the start and end years move on by the entry\'s step (+0x1d) and the start and end dates are rebuilt (day, month, year, weekday); entry 3 clears every person\'s squad selection (+0x6d); each competition of the entry complete in every stage that does not skip the reset restarts: the average attendance of each stage of a league division, the competition\'s new season, the discipline\'s new season for it and, the first time on the day, a new SimulatedStats; the day before: club records, player stats, finances and staff records close the nation\'s season')])
CAMPOS = [('entrada de temporada (0xb63d60, 72 bytes) +0xc numero de competicoes, +0x10 vetor das competicoes, +0x32/+0x34/+0x35/+0x37 dia, mes, ano e dia da semana do inicio, +0x41/+0x43/+0x44/+0x46 os do fim [inferencia]; le e escreve (+0x35, +0x44, +0x2a, +0x39)',
           'season entry (0xb63d60, 72 bytes) +0xc competition count, +0x10 competition array, +0x32/+0x34/+0x35/+0x37 start day, month, year and weekday, +0x41/+0x43/+0x44/+0x46 the end ones [inference]; reads and writes (+0x35, +0x44, +0x2a, +0x39)')]
