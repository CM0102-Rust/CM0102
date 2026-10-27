# 241ª leva — a tabela da liga: ordenar, a minitabela do confronto direto, liberar e conferir o fim; o fim de temporada da liga: os direitos de TV, as honras da temporada e o registro no histórico de competições; os auxiliares de competição: o tipo de confronto na liga, a força do clube e o colocado mais recente; competições: o amistoso marcado do clube e a lista de partidas da data; competições: a próxima partida pelo filtro, o estádio em uso no dia e a data do amistoso internacional

**Rotinas:**
- `ordenar_tabela_da_liga()` = 0x6827d0 (league.cpp, thiscall): ordena a tabela pelas regras (+0xc5): a regra 6 só ordena (0x683210); a 4 monta a minitabela e, sendo a primeira, ordena pelo confronto (0x683410) e passa bolha nos empates; grava a posição (+4) e o histórico de posições (+9); devolve 1;
- `mini_tabela_do_confronto()` = 0x682b30 (league.cpp, thiscall): minitabela dos clubes empatados em pontos com a soma dos confrontos diretos, ordenada pela 0x6834f0; a posição (+4) começa no número de clubes e desce a cada par separado;
- `liberar_tabela_da_liga()` = 0x687970 (league.cpp, thiscall): solta os vetores +5 e +9 das linhas e a tabela (nula: a da liga, e então zera +0x3e e +0xb1);
- `liga_completa()` = 0x689ad0 (league.cpp, thiscall): 1 quando todos os clubes jogaram as +0xa7 rodadas (e as fases, se pedidas); no fim da liga, os registros (0x86a070), os prêmios (0x682e10), o aviso (+0x94) e +0xdd;
- `pagar_direitos_de_tv()` = 0x683010 (league.cpp, thiscall): soma o valor à receita de TV de cada clube da tabela (com filtro, só os de +0x5b igual) e manda a notícia (0x48ea40);
- `registrar_honras_da_temporada()` = 0x684640 (league.cpp, thiscall): campeão, vice e terceiro no histórico (0x4afce0), com a seleção do país-sede como destaque quando a competição pede (+0x88, 0x5fa730, 0x53b3d0); o campeão que é desta divisão limpa o +0x5b dos clubes que apontam para ela;
- `acrescentar_ao_historico()` = 0x4afce0 (comp_util.cpp, cdecl): grava competição, ano e quatro valores na linha livre do histórico de competições (sem ela, cresce 20 linhas pela 0x527f60); devolve a linha;
- `tipo_de_confronto_na_liga()` = 0x4b02e0 (comp_util.cpp, cdecl): 1 = briga pelo título, 3 = briga na zona, 0 = nada, entre dois clubes da mesma liga, pelas notas ponderadas da estatística (0x4a4850) contra um terço dos clubes ou 0,2 × clubes + +0xbe (+ +0xbf);
- `forca_do_clube()` = 0x4c0720 (comp_util.cpp, cdecl): dinheiro (+0x65 / 400 ou saldo / 300) + 16 × reputação + as notas fora da partida (0x6e6340) dos 15 escalados (0x6e2000) do elenco; clube nulo: −10.000.000;
- `clube_colocado_mais_recente()` = 0x4afd80 (comp_util.cpp, cdecl): ordena o histórico de competições e devolve o campeão, o vice ou o terceiro (+0xa, +0xe, +0x12) da primeira linha da competição; não achou: 0;
- `amistoso_do_clube()` = 0x5c88f0 (friendly.cpp, thiscall, ret 8): o amistoso do clube (+0xc) na lista de amistosos (+0x34c8): com data, os da mesma data (0x4a7f20), o completo (+0x10 e +0x14) ou o segundo achado na hora, senão o último guardado; sem data, o primeiro incompleto com +8;
- `lista_da_data()` = 0x5ae6a0 (fix_man.cpp, stdcall, ret 0xc): a lista de partidas (0x5abf10 do gerenciador [0xae2a58]) do ano (+2) e dia (+0) da data, na fatia e no grupo dados;
- `proxima_partida()` = 0x5ac920 (fix_man.cpp, thiscall, ret 0x2c): a partida seguinte (ou anterior) ao cursor ano/dia/fatia, ele incluído, pelo filtro de competição (+0), fase (+0x34) e dois clubes (+0xc/+0x10), cada um -1 = qualquer; até 5 anos; grava o cursor e devolve a partida, ou 0;
- `estadio_em_uso()` = 0x5ae870 (fix_man.cpp, thiscall, ret 0x10): 1 se alguma partida (+0x18) das listas do dia (grupos 0 a 7 e o do estádio, 0x48df40) usa o estádio, na fatia dada ou nas 0 a 2 (fatia -1);
- `data_do_amistoso_internacional()` = 0x5cbdd0 (friendly.cpp, thiscall, ret 4): a data (+3) pela 0x54a130 com o dia (+1), o mês (+2), o ano dado e o dia da semana (+0); a metade alta dos argumentos vem do ano (o eax do chamador);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (15):** 11 acusadas. `cpa_ordem_posicao` escapou: ordenar_tabela_da_liga 0x6827d0: rotina desligada (DESLIGADAS em motor.rs): sem desvio em nenhum modo, a cópia não roda. `cpa_completa_rodadas` escapou: 0x689ad0: rotina desligada (DESLIGADAS em motor.rs): sem desvio em nenhum modo, a cópia não roda (desligada em 26/10/2026). `cpa_tv_valor` escapou: pagar_direitos_de_tv 0x683010 não é chamada até 2002-06-01 (o ramo "liga direitos de tv" não aparece na base); escapou também no jogo comparando e na carga pt (0 divergências). `cpa_confronto_titulo` escapou: tipo_de_confronto_na_liga 0x4b02e0: rotina desligada (DESLIGADAS em motor.rs): sem desvio em nenhum modo, a cópia não roda. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 240ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 241`

**Campos tocados:**
- League +0x3e clubes (curto), +0x3c turnos, +0xa7 total de rodadas (curto; −1 = calcular), +0xad coluna, +0xb1 tabela, +0xc5 regras, +0xa3 comparação própria, +0x44 fase, +0xdd completa, +0xe6, +0xa9, +0x2c/+0x30/+0xc fases; lê e grava (as quatro)
- linha da tabela (0x3b bytes): +0 clube, +4 posição, +5 confrontos (entradas de 14), +9 histórico de posições, +0x11 jogos, +0x19 pontos; lê e grava (ordenar_tabela_da_liga, mini_tabela_do_confronto)
- [0xb679fc] liga sendo ordenada, [0xb67a00] comparação própria; grava (ordenar_tabela_da_liga)
- [0xae2a38] tabela de finanças; finanças do clube (0x167 bytes) +0, +0xb8, +0x158 (saldo e receita de TV); grava (pagar_direitos_de_tv, pela 0x5987c0)
- TClub +0x5b: competição (divisão anterior); lê (pagar_direitos_de_tv) e zera (registrar_honras_da_temporada); TClub +0x57: divisão; lê
- [0xae23d4] / [0xae237c]: histórico de competições (26 bytes: +4 competição, +8 ano, +0xa/+0xe/+0x12/+0x16) e contagem; [0xae2c92] ano; lê e grava (acrescentar_ao_historico)
- TNation (0x122 bytes, [0xae23a8]) +0x8e; lê (registrar_honras_da_temporada)
- TClub +0x57 divisão; lê. Liga ([0xadadfc][id]) +8 estatística, +0x30 fases, +0x3e clubes, +0x43, +0xbe, +0xbf, virtual +0x98; lê. Entrada da estatística +4 jogos, +0x72..+0x75; lê (tipo_de_confronto_na_liga)
- TClub +0x65 dinheiro, +0x80 reputação; [0xae2a38] finanças (saldo de 64 bits); lê (forca_do_clube)
- [0xae23d4] / [0xae237c] histórico de competições (26 bytes); reordena e lê (clube_colocado_mais_recente)
- gerenciador de amistosos +0x34c8: lista de amistosos; amistoso +8, +0xc clube, +0x10, +0x14; lê (amistoso_do_clube)
- data: +0 dia, +2 ano (curtos); lê (lista_da_data)
- partida: +0 competição, +0xc/+0x10 clubes, +0x18 estádio (ponteiros), +0x34 fase (curto); lê (proxima_partida, estadio_em_uso)
- lista de partidas: +2 começo, +0xa cursor; grava o cursor (proxima_partida, estadio_em_uso)
- data internacional: +0 dia da semana, +1 dia, +2 mês (bytes), +3 data; grava a data (data_do_amistoso_internacional)

**Interessa a:** SGE, CMRTE
