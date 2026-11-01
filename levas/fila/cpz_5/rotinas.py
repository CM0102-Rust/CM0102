# As estatísticas das táticas (tactics.cpp): Tactics::record_tactic_result (0x896290: no fim de cada partida sem
# humano, do laço do motor 0x6ba9a0, soma o resultado na linha da tática de cada clube na tabela [0xde1d08]+0 de 0x28
# bytes: jogos, vitórias, empates, derrotas, gols, pontos e as três razões em double que a escolha da formação do
# computador 0x6e0230 lê, +2 e +0x10/+0x18/+0x20 pela postura), Tactics::get_captain (0x8976a0) e Tactics::set_captain
# (0x897880), estas duas só pelas telas de tática (0x8a7440, 0x8a80d0): passageiras. Iguais no cm0102-1080.exe; a
# 0x896290 tem quadro SEH só pela data local (destrutor vazio) e x87 (a ordem das contas está no codigo.rs); as outras,
# sem SEH. O que resta de tactics.cpp fora destas: o construtor, o destrutor e a carga/gravação do tactics.dat.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - 0x600a50 (clube humano): push 0; push 0; push 1; push 0; push clube; call; add esp, 0x14 -> cdecl(clube, 0, 1, 0, 0).
# - 0x54a110 (Date::Date): lea ecx, [registro]; call; sem push, sem add esp -> thiscall(this), ret 0.
# - 0x7e4750 (registro da partida pelo clube): push registro; push clube; push jogo; call; add esp, 0xc -> cdecl(jogo,
#   clube, registro).
# - 0x8982b0 (a linha do clube na tabela tática): push time; push clube; mov ecx, esi (this); call; sem add esp ->
#   thiscall(this, clube, time), ret 8 (a cópia Rust do motor.rs a declara stdcall porque não usa o this; a pilha é a
#   mesma e aqui vai thiscall, como na original).
# - a caixa de erro (aviso_em do motor.rs: 0x944cff cdecl 5, 0x90d130 cdecl 3, 0x944c9f cdecl, 0x5e8290 cdecl 3), com
#   o fonte 0xa8b908 (tactics.cpp).
# - quem chama: 0x6ba9a0 mov ecx, [0xde1d08]; push jogo -> thiscall(this, jogo), ret 4 (o motor.rs chama por em1).
# Comparadores: a 0x896290 lado a lado nas duas linhas das táticas e na devolução (o registro da partida é local; jogo
# ou clube nulos: a caixa, só a original); a 0x8976a0 pela devolução; a 0x897880 lado a lado no dword do capitão.
SAVE = False
ROTINAS = [('registrar_resultado_da_tatica', 0x896290, [0x55, 0x8b, 0xec, 0x83, 0xe4, 0xf8]),
           ('capitao_guardado', 0x8976a0, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('escolher_capitao', 0x897880, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('as estatísticas das táticas: o resultado de cada partida na linha da tática do clube do computador, o capitão guardado e o capitão escolhido',
        "the tactic statistics: each match result in the AI club's tactic row, the stored captain and the chosen captain",
        [('registrar_resultado_da_tatica', 'Tactics::record_tactic_result', '0x896290', 'tactics.cpp', 'thiscall, ret 4',
          'no fim da partida sem humano, soma na linha da tática de cada clube (0x28 bytes) jogos, vitórias, empates, derrotas, gols e pontos (3 por vitória, 1 por empate) e refaz as três razões em double (+0x10/+0x18/+0x20) que a escolha da formação do computador lê',
          "at the end of a match with no human, adds to each club's tactic row (0x28 bytes) the games, wins, draws, losses, goals and points (3 per win, 1 per draw) and redoes the three double ratios (+0x10/+0x18/+0x20) that the AI formation choice reads"),
         ('capitao_guardado', 'Tactics::get_captain', '0x8976a0', 'tactics.cpp', 'thiscall, ret 8',
          'o capitão guardado (+0x52) na linha do clube e do time na tabela tática; sem linha, 0',
          "the captain stored (+0x52) in the club and team row of the tactics table; no row, 0"),
         ('escolher_capitao', 'Tactics::set_captain', '0x897880', 'tactics.cpp', 'thiscall, ret 0xc',
          'grava o capitão (+0x52) na linha do clube e do time na tabela tática e devolve 1; sem linha, 0',
          "writes the captain (+0x52) in the club and team row of the tactics table and returns 1; no row, 0")])
CAMPOS = [('[0xde1d08] +0: a tabela de estatísticas das táticas (0x28 bytes: +0 a tática, +2 jogos, +4 vitórias, +6 empates, +8 derrotas, +0xa gols pró, +0xc contra, +0xe pontos, +0x10/+0x18/+0x20 as razões em double); ponteiro e valor; lê e grava (registrar_resultado_da_tatica)',
           '[0xde1d08] +0: the tactic statistics table (0x28 bytes: +0 the tactic, +2 games, +4 wins, +6 draws, +8 losses, +0xa goals for, +0xc against, +0xe points, +0x10/+0x18/+0x20 the double ratios); pointer and value; reads and writes (registrar_resultado_da_tatica)'),
          ('[0xde1d08] +0x601: a tabela tática dos clubes (0x91 bytes por linha: +0 a tática, +0x52 o capitão); ponteiro, valor e id; lê (registrar_resultado_da_tatica, capitao_guardado), grava o capitão (escolher_capitao)',
           "[0xde1d08] +0x601: the clubs' tactics table (0x91 bytes a row: +0 the tactic, +0x52 the captain); pointer, value and id; reads (registrar_resultado_da_tatica, capitao_guardado), writes the captain (escolher_capitao)"),
          ('fixture +0x1c / +0x20: o mandante e o visitante; ponteiro; lê (registrar_resultado_da_tatica)',
           'fixture +0x1c / +0x20: the home and away clubs; pointer; reads (registrar_resultado_da_tatica)'),
          ('registro da partida +0x19 / +0x1a: gols pró e contra pelo clube (0x7e4750); valor; lê (registrar_resultado_da_tatica)',
           'match record +0x19 / +0x1a: goals for and against by the club (0x7e4750); value; reads (registrar_resultado_da_tatica)')]
INTERESSA = ['CMRTE']
