# As chaves fixas do exe: o que o código trata pelo nome

Levantamento de 30/10/2026, só leitura (desmontagem do `cm0102.exe` 3.9.68). Ferramenta: `engenharia-reversa/chaves.py`, que grava `docs/chaves_uso.csv` (uma linha por chave). Os nomes das globais vêm do mapa de maxevans60 (`referencias/cm0102-function-map`, CC BY 4.0; ver `ENDERECOS_COMUNIDADE.md`). Marcação: [observado] = visto no binário; [inferência] = conclusão sem prova direta.

## Resposta curta

- O exe não traz ids de nação, competição ou clube escritos como número. Ele traz **nomes**. Na carga, `set_key_ids` (0x601a40) põe −1 em cada global `g_key_*` e procura cada nome fixo nas tabelas do banco. Depois guarda na global a linha achada. As regras só comparam o id do registro com essa global [observado]. A carga e o que acontece quando um nome falta estão em `TESTE_CHAVES.md`.
- São **1.284 chaves**, usadas em **8.809 pontos** do código fora da carga. A carga confere 1.263 delas: quando uma fica −1, aparece a caixa "Unable to find the %s index" [observado].
- Desacoplar as competições é, na prática, trocar esses pontos por dados. A lista completa, com os arquivos de origem de cada uso, é o `chaves_uso.csv`.

## Por tipo

| Tipo | Chaves | Conferidas na carga | Usos fora da carga | Arquivos de origem |
|---|---|---|---|---|
| nação | 210 | 209 | 3.679 | 115 |
| competição de clubes (inclui as de seleções, que estão na mesma tabela) | 304 | 296 | 2.989 | 238 |
| clube | 313 | 308 | 1.332 | 25 |
| prêmio (`staff_comp`) | 417 | 415 | 458 | 35 |
| continente | 6 | 6 | 315 | 43 |
| cor | 34 | 29 | 36 | 2 |

Os arquivos de origem vêm do `docs/mapa_funcoes.csv`. Cerca de um quarto dos usos cai em rotinas sem arquivo conhecido ("?" no CSV).

## Onde pesa

- **Nações nas competições de seleções:** as eliminatórias da Euro (`euro_champ_qual.cpp`, 49 nações, 546 usos) e as europeias da Copa (`wc_europe_league.cpp`, 50 nações, 510 usos) montam grupos e potes nação por nação. As sul-americanas (`wc_south_american_league.cpp`, 180 usos) fazem o mesmo. A sede de cada torneio (`host_country.cpp`, 45 nações e 15 competições) também é fixa [observado: as referências; que sejam listas de grupos, potes e sedes é inferência pelo nome dos arquivos e pela contagem].
- **Nações espalhadas pela simulação:** contratos (`staff_contracts.cpp`), transferências e lista de observação (`transfer_manager.cpp`, `shortlist_manager.cpp`) e o Mundial (`world_cup.cpp`) perguntam por nações específicas. É aí que mora a troca da Irlanda do Norte pela Áustria citada em `CAMPOS.md`: a Irlanda do Norte (0x9cf3e4) tem 61 usos em 21 arquivos.
- **Competições de clubes:** cada liga reescreve as próprias regras pelo id-chave (`bra_nat_first.cpp`, `spa_first.cpp`, `swe_prm.cpp`, `ita_ser_a.cpp`, `por_prm.cpp`, com 57 a 71 usos cada). `Setup.cpp` (189) liga as competições às nações na montagem do jogo. As mais citadas são a Copa do Mundo (91 usos) e a "a lower division" (52 usos), a divisão inferior genérica.
- **Clubes:** quase tudo na carga do banco (`Database.cpp`, `index.cpp`). Fora dela, as regras de clubes nomeados na Argentina (`arg_prm.cpp`, `arg_second.cpp`), na MLS (`usa_mls.cpp`) e no sorteio da CONMEBOL (`conmebol_seeding.cpp`). Dos 313 clubes, 49 são times B ou II (reservas espanhóis e alemães).
- **Prêmios:** cada `*_awards.cpp` dá os seus pelo id-chave (o Brasil tem 58 usos, a Itália 40, a Inglaterra 32).
- **Continentes:** sobretudo os amistosos (`friendly.cpp`, 112 usos) e o Mundial de Clubes.

## Chaves que a carga não confere (21)

- 5 cores;
- Burma (Mianmar);
- os grupos A a D da copa da liga norte-irlandesa;
- as copas "unmatched" da Irlanda e da Turquia;
- a Football Writers' Cup turca;
- 2 prêmios belgas;
- 5 clubes: Godoy Cruz, Elgin City, Peterhead, C.P. Mérida e C.P. Mérida B.

Se uma delas faltar, não aparece aviso: a global fica −1 e as comparações só deixam de bater [observado: a ausência do `cmp [chave], edi` na carga; o efeito em jogo é inferência]. Os 6 contadores com nome `g_key_nation_*_loop_end`/`active_count` no mapa não são chaves, e a ferramenta os deixa de fora.

## Chaves sem uso fora da carga (40)

Exemplos:

- a Supercopa e a Recopa sul-americanas;
- as zonas Metropolitana e Interior da Argentina;
- as divisões croatas;
- os grupos da 2ª divisão turca;
- as copas regionais brasileiras;
- prêmios belgas, dinamarqueses e da 3ª divisão brasileira.

Essas chaves são procuradas e conferidas, mas nenhum código as lê depois. Se o nome não estiver no banco, a carga mostra a caixa à toa [observado].

## Para o desacoplamento e o editor

- Renomear uma competição, nação ou clube da lista quebra a chave na carga (`TESTE_CHAVES.md`). Trocar um país por outro exige mudar os pontos de uso, não só o nome. É por isso que esse tipo de troca hoje é feito por patch.
- O `chaves_uso.csv` diz, para cada chave, quantos pontos e quais arquivos precisam ir para dados. As eliminatórias (Euro e Copa) e o `host_country.cpp` concentram as nações. As regras de liga e os prêmios concentram as competições.
- No porte, a versão Rust pode ler a mesma tabela de nomes e devolver as linhas. Assim a tabela de nomes deixa de ser código e vira dado [inferência; proposta].
