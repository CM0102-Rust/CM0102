# 211ª leva — o fechamento das propostas de contrato da oferta, a fila da transferência livre e as negociações de troca do jogador; o jogador que sai da seleção, a faixa de horário do jogo e o teto da lista de observados; as ofertas pelo jogador, a reputação-alvo do clube e o jogo na notícia

**Rotinas:**
- `fechar_propostas_da_oferta()` = 0x8c6db0 (transfer_manager.cpp, stdcall): fecha (estado 4) as propostas de contrato ligadas à oferta: a do jogador-alvo e as dos jogadores da troca;
- `enfileirar_transferencia_livre()` = 0x8c16b0 (transfer_manager.cpp, thiscall): põe o jogador na fila da transferência livre (id, motivo e compensação), recusando o emprestado;
- `negociacoes_de_troca_do_jogador()` = 0x8c5fc0 (transfer_manager.cpp, thiscall): conta as negociações abertas do clube do jogador que o incluem na troca e, se pedido, lista os índices das ofertas;
- `tirar_jogador_da_selecao()` = 0x775ba0 (national_teams.cpp, thiscall): tira o jogador da reserva do nível dele na nação e da convocação, limpando os campos de seleção;
- `faixa_do_horario()` = 0x5af2f0 (fix_man.cpp, thiscall): a faixa de horário do jogo (1 fim de semana, 2 os outros) pela nação da competição e o dia;
- `teto_da_observacao()` = 0x84b230 (shortlist_manager.cpp, stdcall): o teto de preço da vaga na lista de observados: reputação do clube vezes um fator (ou -1);
- `ofertas_pelo_jogador()` = 0x8c5cf0 (transfer_manager.cpp, thiscall): conta as ofertas de transferência pelo jogador (sem as encerradas, salvo pedido) e pode listar os índices delas;
- `reputacao_alvo_do_clube()` = 0x83bf40 (shortlist_manager.cpp, thiscall): a reputação que o clube procura nas contratações, ajustada pelo técnico e pelo presidente, entre o piso do clube e 10000;
- `gravar_jogo_na_noticia()` = 0x4afeb0 (comp_util.cpp, cdecl): grava clubes, local, competição, placar e rodada do jogo em 18 dados seguidos da notícia e avança o cursor;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (9):** 8 acusadas. `s_faixa` escapou: na carreira o jogador que sai da seleção nunca está na faixa do nível dele com a nota do nível abaixo da da ficha, então o tamanho da faixa não diminui (o comparador cobre o registro da nação inteiro, onde fica o tamanho). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (034ff3eb3e55dfd3).

**Refazer**, sobre o commit da 210ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 211`
