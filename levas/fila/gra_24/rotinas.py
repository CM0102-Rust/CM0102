# A notícia do resultado de um mata-mata de copa (comp_util_write_cup_tie_result_news 0x4b0b80, comp_util.cpp,
# 0x4b0b80..0x4b441b, ~14,5 KB; leitor gra, rodada 4, 21/11/2026). A rotina inteira desviada num item só, partida em
# pedaços com nome (TieNews::say e os três blocos: sem fase seguinte nomeada, "próxima fase" sem nome, fase nomeada).
# A 0x4b2070 do mapa da comunidade é um bloco dentro dela (mesmo quadro de pilha), não uma rotina.
# Convenções conferidas na desmontagem:
# - a rotina: cdecl(clube, jogo, veredito, desfecho, próxima fase, Text), 6 argumentos (os chamadores empilham 6 e
#   limpam 0x18; o Ghidra perde o Text, lido em [esp+0x3e8] no fim);
# - os argumentos dos 125 formatos (0x66f4e0, cdecl) conferidos por script: cada push seguido até a chamada, com os
#   [esp+k] normalizados pelos pushes e os caminhos que pulam para o meio de um bloco seguidos de volta; os 125
#   formatos e o formato de cada um batem com o código (gênero do clube, do meio, do adversário, do estádio; nome do
#   clube, meio, gols na ordem certa, adversário, local);
# - 0x4bfa30 cdecl(comp, prefixo, rodada, 1, buf, 500); 0x4ba8f0 cdecl(código, 0) (o gênero pela rodada crua, não a
#   sem variante); 0x4b4850 cdecl(buf, 100, próxima fase (dword inteiro), 0); 0x944c9f (sprintf) cdecl;
#   0x66f4d0 cdecl, chamada duas vezes como no original; 0x729500 thiscall em [0xb6b7c0] (vaga, id da casa, id de
#   fora, dia, ano; palavras com lixo em cima, lidas como i16); 0x549ef0 thiscall(data local de 8 bytes, dia, ano);
#   Friendly::is_comp_current_on_date (0x5c29b0, já em Rust) thiscall em [0xae2c74] (comp, &data); 0x4ae660/0x4ae8a0
#   thiscall no Text.
# - o buffer da próxima fase começa com o byte de 0xad9c64 (o resto zerado), como o original.
# Comparador: compare_text_writer (texto, vínculos e gerador).
SAVE = False
ROTINAS = [('comp_util_write_cup_tie_result_news', 0x4b0b80, [0x6a, 0xff, 0x68, 0x0b, 0x69, 0x95, 0x00])]
DOCS = ('a notícia do resultado de um mata-mata de copa',
        'the news of a cup tie result',
        [('comp_util_write_cup_tie_result_news', 'comp_util_write_cup_tie_result_news', '0x4b0b80', 'comp_util.cpp', 'cdecl',
          'a frase da notícia do resultado de um mata-mata para um clube: venceu na rodada, ganhou a competição (na final, rodada 0x96 sem prefixo), passou para a fase seguinte (com ou sem o nome dela) ou perdeu; a forma pela marca de desempate do jogo (gols fora, cabeça de chave, prorrogação, pênaltis, disputa de 35 jardas, gol de ouro), senão pelo veredito do chamador (surpresa, excelente, bom, decepcionante, muito decepcionante) e pelo desfecho, com o placar (o agregado, senão o da prorrogação, senão o do tempo normal; nas derrotas comuns o do adversário primeiro), o nome da rodada (nas eliminatórias do Mundial da Oceania e da Ásia com clube de fora do continente, o rótulo do Mundial) e, em jogo único com estádio, o local ("estádio, cidade" nas línguas 0 e 1, "estádio (cidade)" nas outras); W.O. com marca negativa; depois os vínculos dos dois clubes, do placar (quando o registro de estatísticas é o do jogo) e da competição (quando ela está em curso na data do jogo)',
          'the news sentence of a cup tie result for one club: won in the round, won the competition (in the final, round 0x96 with no prefix), through to the next stage (named or not) or lost; the wording by the tie\'s settling mark (away goals, higher seed, extra time, penalties, the 35-yard shootout, the golden goal), else by the caller\'s verdict (shock, excellent, fine, disappointing, very disappointing) and the outcome, with the score (the aggregate, else after extra time, else at full time; the opponent\'s first in the ordinary defeats), the round name (in the World Cup Oceania and Asian qualifying with a club from outside the continent, the World Cup label) and, in a single match with a stadium, the venue ("stadium, city" in languages 0 and 1, "stadium (city)" in the others); a forfeit by a negative mark; then the links of both clubs, of the score (when the statistics entry is the fixture\'s) and of the competition (when it is current on the fixture\'s date)')])
