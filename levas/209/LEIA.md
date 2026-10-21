# 209ª leva — as telas da partida (14): a montagem e a resposta da tela da partida ao vivo e a montagem da tela de detalhes; as telas da partida (13): a barra da posse, o painel da visão geral da partida e a tabela da liga ao vivo

**Rotinas:**
- `montar_tela_da_partida()` = 0x718e90 (match_screens.cpp, cdecl): monta a tela da partida ao vivo: cabeçalhos dos times e placar, narração, relógio, controle, botões táticos, velocidades da narração, abas, o painel da aba, o estádio e o painel de narração;
- `responder_tela_da_partida()` = 0x71a240 (match_screens.cpp, cdecl): responde na tela da partida ao vivo: volume e velocidade da narração, o lance novo (relógio, narração, modos), assistir ou não, a tática de um time, a troca de aba e as respostas de cada aba;
- `montar_detalhes_da_partida()` = 0x71b630 (match_screens.cpp, cdecl): monta a tela de detalhes de uma partida guardada: lê o registro e a foto da partida nas estatísticas, os cabeçalhos dos times, as 7 abas e o painel da aba;
- `atualizar_barra_da_posse()` = 0x71d9e0 (match_screens.cpp, cdecl): redesenha a barra da posse da visão geral quando a posse muda (percentuais e a cor do clube com mais posse) e guarda o estado da partida na cópia do parâmetro 0x1d;
- `painel_da_visao_geral_da_partida()` = 0x71c270 (match_screens.cpp, cdecl): desenha a visão geral da partida: período, barra da posse e comentário, data, placar com intervalo/prorrogação/pênaltis, competição, autores dos gols e cartões, disputa de pênaltis, árbitro, público, tempo e partida abandonada;
- `tabela_da_liga_ao_vivo()` = 0x7280e0 (match_screens.cpp, cdecl): desenha a tabela da liga ao vivo (liga, topo, base) numa cópia da liga com os placares do dia aplicados (ou o aviso da data do sorteio): título, cabeçalhos, separadores, posição, seta de subida/descida, clube e colunas;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (0):** 0 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 208ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 209`
