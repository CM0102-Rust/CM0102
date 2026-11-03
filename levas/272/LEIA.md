# 272ª leva — a Liga dos Campeões: o sorteio dos grupos da primeira fase (parceiros da mesma nação, metades, potes, prêmio, fases e os perdedores para a Copa da UEFA), a ordem dos potes e as posições previstas; a Liga dos Campeões: o sorteio dos grupos da segunda fase (pares do mesmo grupo, metades, nações, prêmio, fases, terceiros para a Copa da UEFA) e a ordem dos classificados; o grupo de uma fase pelo id; a Liga dos Campeões: a tabela das rodadas das fases de grupos (as datas das duas fases, por metade); a liga: as expectativas da temporada e o rebaixamento de um clube; a competição: todas as fases completas, a lista dos clubes e o progresso do clube

**Rotinas:**
- `campeoes_sortear_grupos()` = 0x585290 (european_cup.cpp, thiscall, ret 0): os 32 clubes ordenados pelo pote, os parceiros da mesma nação em metades opostas, o sorteio dos 8 grupos pote a pote, a troca dos clubes da mesma nação no grupo, o coeficiente, o prêmio de 1.800.000 e a notícia, as 8 fases e os perdedores da terceira preliminar na Copa da UEFA;
- `campeoes_ordem_dos_potes()` = 0x586f60 (european_cup.cpp, cdecl): a comparação dos potes: a semente europeia do clube (+0x1df), e no empate a reputação maior primeiro;
- `campeoes_posicoes_previstas()` = 0x586900 (european_cup.cpp, thiscall, ret 0): os inscritos ordenados pela força e a posição prevista de cada um: o mata-mata da final para trás, a segunda e a primeira fase de grupos e as preliminares;
- `campeoes_sortear_segunda_fase()` = 0x585ab0 (european_cup.cpp, thiscall, ret 4): os 16 classificados ordenados, os pares do mesmo grupo, a escolha do grupo de cada um (ordem embaralhada, menos times, sem o mesmo grupo nem a mesma nação), o sorteio de novo até 20 vezes, o coeficiente, o prêmio de 1.200.000 e a notícia, as fases 8 a 11 e os terceiros na Copa da UEFA;
- `campeoes_ordem_da_segunda_fase()` = 0x5871e0 (european_cup.cpp, cdecl): a comparação dos classificados: mais clubes da mesma nação primeiro, depois o word +0x8e da nação maior, depois o vencedor do grupo antes do segundo;
- `grupo_da_fase()` = 0x4b01d0 (comp_util.cpp, cdecl): o grupo (a fase que contém) de uma fase ou rodada pelo id, pelas faixas fixas; −1 sem grupo;
- `campeoes_rodadas_da_fase()` = 0x583c50 (european_cup.cpp, thiscall, ret 0x10): nas fases de grupos, a tabela de 6 rodadas (as datas da primeira fase em setembro e outubro e da segunda de novembro a março, por metade) com o jogo 0 de cada rodada vazio; as preliminares e o mata-mata ficam com a original;
- `liga_expectativas_da_temporada()` = 0x68aa80 (league.cpp, thiscall, ret 0): as expectativas da temporada de cada clube da tabela, pelo gerenciador dos técnicos, com a notícia;
- `liga_rebaixar_clube()` = 0x6831a0 (league.cpp, thiscall, ret 0xc): o clube desce para a divisão (guardando a anterior), as cláusulas de rebaixamento e a saída do técnico humano quando a divisão não é jogada;
- `competicao_todas_completas()` = 0x48ce10 (comp.cpp, thiscall, ret 0): na competição do topo, se ela e todas as da cadeia de baixo estão completas;
- `competicao_lista_de_clubes()` = 0x48e1c0 (comp.cpp, thiscall, ret 8): o vetor dos clubes distintos de todas as fases (até 200) e a conta;
- `competicao_progresso_do_clube()` = 0x48cc70 (comp.cpp, thiscall, ret 0x10): o id da fase, a situação e a rodada a que o clube chegou, da última fase para a primeira;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (11):** 10 acusadas. `cqh6_lista_com_repetidos` escapou: nenhuma fase da carreira até 2002-06-01 tem clube repetido na lista, então tirar a checagem de repetidos não muda nada. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (8845ebc9).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 151bdf85, original 2aacab3a (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 271ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 272`

**Campos tocados:**
- liga dos campeões +4 id, +0xc as fases, +0x14 tabela (6 bytes: +0 clube), +0x30 fase atual, +0x3e times da preliminar (word), +0x40 ano, +0xa7 tabela da preliminar (6 bytes: +0 clube, +5 1 vencedor / 2 perdedor da terceira), +0xb1 (byte zerado sem os 32); valor e ponteiro; lê e grava (campeoes_sortear_grupos)
- copa da UEFA ([[0xadadfc] + 4 × [0x9cf6f0]]) +0x14 tabela (situação 5), +0x36 quantos (word), +0x3e times (word), +0xa7 tabela (+4/+5 zerados); valor e ponteiro; grava (campeoes_sortear_grupos)
- clube +0x53 nação, +0x80 reputação (word), +0x1db competição europeia (id), +0x1df semente europeia (char); valor, id e ponteiro; lê (campeoes_sortear_grupos, campeoes_ordem_dos_potes), grava +0x1db (campeoes_sortear_grupos)
- liga dos campeões +8 estatística (Comp_Stats), +0x14 inscritos (6 bytes: +0 clube), +0x36 quantos (word), +0x3c rodadas preliminares (word), +0xa3 tabela das preliminares (0x68 bytes, +0x1a confrontos); ponteiro e valor; lê (campeoes_posicoes_previstas)
- registro da estatística da competição +0x72 posição (byte); valor; grava pela 0x4a2540 (campeoes_posicoes_previstas)
- liga dos campeões +4 id, +0xc as fases (0..7 a primeira fase, 8..11 a segunda), +0x30 fase atual, +0x40 ano, +0x50 índice do órgão (char); valor e ponteiro; lê e grava (campeoes_sortear_segunda_fase)
- entrada do sorteio da segunda fase (7 bytes): +0 clube, +4 quantos da mesma nação, +5 grupo da primeira fase, +6 0 vencedor / 1 segundo; valor; grava (campeoes_sortear_segunda_fase), lê (campeoes_ordem_da_segunda_fase)
- copa da UEFA +0x14 tabela (situação 5), +0x36 quantos, +0x3e times, +0xa7 tabela; clube +0x53 nação, +0x1db competição europeia (id; −1 sem); nação +0x8e (word) [inferência: a força da nação no coeficiente]; valor, id e ponteiro; lê e grava (campeoes_sortear_segunda_fase), lê (campeoes_ordem_da_segunda_fase)
- liga dos campeões +8 estatística (Comp_Stats), +0x14 inscritos (6 bytes: +0 clube), +0x36 quantos (word), +0x3c rodadas preliminares (word), +0xa3 tabela das preliminares (0x68 bytes, +0x1a confrontos); ponteiro e valor; lê (campeoes_posicoes_previstas)
- registro da estatística da competição +0x72 posição (byte); valor; grava pela 0x4a2540 (campeoes_posicoes_previstas)
- liga dos campeões +0x40 ano; valor; lê (campeoes_rodadas_da_fase)
- tabela das rodadas (0x41 bytes por rodada): +0 dia do ano, +2 ano − base, +4 código, +0x3d valor; jogo 0 em +5 (+5 valor, +9 a, +0xa b, +0xb c); valor; grava pela 0x68a160 e 0x68a1c0 (campeoes_rodadas_da_fase)
- liga +0x1c a de cima (índice; < 0 no topo), +0x20 a de baixo (índice em [0xadadfc]), +0x3e clubes, +0xb1 tabela (linhas de 0x3b bytes, +0 clube); valor e ponteiro; lê (liga_expectativas_da_temporada, liga_cadeia_de_promocoes)
- clube +0x57 competição, +0x5b competição anterior; ponteiro; lê e grava (liga_rebaixar_clube)
- competição +0xc as fases, +0x1c a de cima (índice), +0x20 a de baixo (índice em [0xadadfc]), +0x30 última fase, +0x3a id, +0x3e times, +0x43 tipo (2 liga), +0xa3 rodadas (0x68 bytes: +7 rodada, +0x1a confrontos), +0xa7 times (6 bytes: +0 clube, +5 situação), +0xab rodada atual (char), +0xad confrontos (0x35 bytes: +0/+2 os times), +0xb1 tabela (0x3b bytes: +0 clube, +0x37 situação); valor e ponteiro; lê (competicao_todas_completas, competicao_lista_de_clubes, competicao_progresso_do_clube)

**Interessa a:** CMRTE, GPF2
