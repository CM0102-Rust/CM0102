# Mapa da disciplina (suspensões)

Levantamento para o editor `CM0102-Editor/CMRTE` ganhar "remover suspensão". Só leitura: desmontagem e pseudocódigo (Ghidra) do `cm0102.exe` 3.9.68, o mapa da comunidade (`referencias/cm0102-function-map`, `discipline.cpp` e `ruling_body.cpp`), as cópias Rust da 176ª, 184ª e 191ª leva (`rust/src/motor.rs`) e a leitura do `discipline.dat` de uma **cópia** do `jogo/FeriasTesteSave.sav` (MD5 33fda465193b5e8e6d7ace93ecd44559, intocado) com as funções de leitura do `engenharia-reversa/prova_save.py`. O jogo não foi aberto.

Cada afirmação leva **[observado]** (lido na desmontagem, no código ou no arquivo) ou **[inferência]** (dedução sem prova direta). Endereços em VA. "Pessoa" = índice na tabela de pessoas (`[0xae23bc] + id * 0x6e`; o jogador é uma pessoa).

Correção da pista: o **construtor** do subsistema é 0x5554a0; 0x555dd0 é o `save` (grava o `discipline.dat`) e 0x555860 o `load` [observado]. O `MAPA_SAVE.md` §2 cita 0x555dd0 na tabela das rotinas de gravar, e ali está certo.

## 1. As estruturas

### O objeto `Discipline` (ponteiro global `[0xae28f0]`)

| Desl. | Tipo | Campo | Fonte |
|---|---|---|---|
| +0x00 | `RulingBody**` | tabela de 43 ponteiros, um por **regra de disciplina** (entidade: FIFA, confederações, federações nacionais); nulo para a entidade que não foi criada | [observado] 0x55d420, 0x556240 |
| +0x04 | `BanRecord*` | vetor das **suspensões** (20 bytes cada) | [observado] 0x556940, 0x556d00 |
| +0x08 | `i16` | capacidade do vetor (cresce de 100 em 100, teto 32000) | [observado] 0x556940 |
| +0x0a | `i16` | quantidade usada (marca d'água: não diminui; vaga livre = `+0 == 0xff` e `+2 == 0xffff`, reaproveitada pela próxima) | [observado] 0x556940, 0x556d00 |
| +0x0c | `{i32 pessoa, i32 nação}*` | **índice por nação** das suspensões da FIFA (8 bytes cada, ordenado por nação e depois pessoa, comparador 0x55e5f0) | [observado] 0x556940, 0x556d00, 0x557120 |
| +0x10 | `i16` | capacidade do índice (de 20 em 20) | [observado] |
| +0x12 | `i16` | quantidade no índice | [observado] |
| +0x14 | `u8[N][29]` | **registro de disciplina por pessoa**, N = `[0xae2364]` (quantidade de pessoas) | [observado] 0x5554a0, 0x55cb20 |
| +0x18 | `FILE_LLIST*` | `ban_history.tmp` (histórico das suspensões cumpridas) | [observado] 0x5554a0 |
| +0x1c | lista (0xe bytes) | **suspensões guardadas** de jogador emprestado (nós de 10 bytes) | [observado] 0x5563f0, 0x556570, 0x55cb60 |

A lista do jogo (0x68d020 e vizinhas) tem `+0 i16 quantidade`, `+2 primeiro nó`, `+6 último nó`, `+0xa cursor`; cada nó tem `+0 ponteiro do dado`, `+4 i16 tamanho`, `+6 próximo`, `+0xa anterior`; `0x68d1b0` avança o cursor e devolve o dado, `0x68d1f0` remove o nó cujo dado é igual byte a byte [observado].

### O registro por pessoa (29 bytes, em `[D+0x14] + id * 29`)

| Desl. | Tipo | Campo | Fonte |
|---|---|---|---|
| +0 + 5k (k = 0..3) | `i8` | **vaga k**: a regra de disciplina (índice 0..42 da tabela +0x00); −1 = vaga livre. É o "tipo" da `disciplina_gravar_vaga` | [observado] 0x55cb20, 0x55cae0 |
| +1 + 5k | `i32` | o índice da pessoa no vetor de entradas daquela regra (`[corpo+0xb]`, 14 bytes por entrada). É o "valor" | [observado] 0x7e82e0 |
| +0x14 | `i32` | primeiro nó do histórico no `ban_history.tmp` (−1 = nenhum) | [observado] 0x556e90, 0x58e340 |
| +0x18 | `i32` | último nó do histórico (−1 = nenhum) | [observado] |
| +0x1c | `i8` | quantas vagas estão guardadas na lista +0x1c (empréstimo) | [observado] 0x5563f0, 0x55cb60 |

Pessoa sem nada: os 28 primeiros bytes 0xff e +0x1c = 0 [observado] 0x5554a0, 0x556830.

Qual vaga cada regra usa é fixo na regra (`corpo+4`): a FIFA (regra 0) usa a vaga 3; as confederações (regras 1 e 2) a vaga 2; as federações nacionais a vaga 0 (algumas a vaga 1) [observado nos construtores 0x7df180, 0x7dd400, 0x7e29e0, 0x7db100, 0x7dc3c0, 0x7e01b0; o "nacional = 0 ou 1" vale para os que deu para ler, inferência para os demais]. No save de teste: regra 0 sempre na vaga 3, 1 e 2 na vaga 2, 6 e 42 na vaga 0, 7 na vaga 1 [observado].

### A regra de disciplina (`RulingBody`, 0x23 bytes, `[[D] + 4 * regra]`)

| Desl. | Tipo | Campo | Fonte |
|---|---|---|---|
| +0x00 | vtable | 43 classes (0x96e9f4..0x96ef34, base 0x96f104): +0 temporada nova (0x7e7ed0; a FIFA 0x7df3c0), +4 o dia (0x7e7d50), +8 lance de jogador na partida (cria a suspensão; próprio de cada regra), +0x10 texto do âmbito, +0x14 "a um cartão da suspensão", +0x1c extra do save (`ret 4` em todas: nada a mais) | [observado] |
| +0x04 | `u8` | a vaga do registro da pessoa que esta regra usa | [observado] |
| +0x05 | `u8` | o índice desta regra (0..42) | [observado] |
| +0x07 | lista* | **pendentes**: suspensões decididas que só entram numa data (nós de 0x70 bytes) | [observado] 0x7e7d50, 0x7e91d0 |
| +0x0b | `Entry*` | vetor das entradas por pessoa (14 bytes cada) | [observado] |
| +0x0f | `i32` | capacidade | [observado] |
| +0x13 | `i32` | quantidade | [observado] |
| +0x17 | `i32` | id da nação da regra (−1 se não é nacional) | [observado] 0x556270, construtores |
| +0x1b | `i32` | id da confederação (−1 na FIFA) | [observado] |

A competição aponta a sua regra no byte **`+0x50`** (`[[0xadadfc] + 4 * id_competição] + 0x50`, 0xff = nenhuma) [observado] 0x556210, 0x55ca00, 0x55d270.

### A entrada da pessoa na regra (14 bytes, `[corpo+0xb] + índice * 14`)

| Desl. | Tipo | Campo | Fonte |
|---|---|---|---|
| +0x0 | `i32` | id da pessoa (−1 = entrada livre) | [observado] 0x7e82e0, 0x7e7ed0 |
| +0x4 | `u8` | cartões amarelos acumulados na temporada (a regra 0x96ec74 avisa "a um cartão" em 4, 7, 10, 13, 16...) | [observado] 0x7df910 |
| +0x5 | `u8` | outro contador da temporada (vermelhos ou suspensões) | [inferência]; zerado na temporada nova [observado] |
| +0x6 | `i16` | **índice da suspensão** no vetor `[D+4]`; −1 = sem suspensão nesta regra | [observado] 0x7e8590, 0x7e85b0 |
| +0x8 | `i16` | contador da temporada | [inferência]; zerado na temporada nova [observado] |
| +0xa | 4 bytes | marcas (bits 0..5 zerados na temporada nova) | [observado] 0x7e7ed0 |

### A suspensão (`BanRecord`, 20 bytes, `[D+4] + índice * 20`)

| Desl. | Tipo | Campo | Fonte |
|---|---|---|---|
| +0x00 | `i8` | jogos de suspensão; −1 = suspensão por tempo | [observado] 0x556e90, 0x7e85b0 |
| +0x01 | `i8` | jogos já cumpridos | [observado] |
| +0x02 | `i16` | dias de suspensão (−1 = suspensão por jogos) | [observado] |
| +0x04 | `TCMDate` (8) | data de início (+4 dia do ano, +6 ano, +8 bissexto) | [observado] 0x556c70, 0x7df789 |
| +0x0c | `i8[4]` | a regra que vale em cada vaga (índice = vaga do corpo; −1 = não vale); +0xf ≠ −1 = suspensão da FIFA, entra no índice por nação | [observado] 0x556940, 0x556d00 e no save |
| +0x10 | `u8` | motivo: 0 pontos, 1 amarelos, 2 vermelho(s), 3 conduta, 4 amarelos seguidos, 5 doping, 6 passaporte inválido, outro = comportamento antiprofissional | [observado] 0x55d0a0 |
| +0x11 | `i16` | quantidade do motivo (pontos, cartões) | [observado] 0x55d0a0 |
| +0x13 | `u8` | bits: 0 = somada a uma suspensão existente; 1 = vale em todas as competições (é conferida contra a FIFA em `is_player_banned`); 2 = ver nota | bit 0 e 1 [observado] 0x556940, 0x55d270; bit 2 [inferência] |

Livre: `+0 = 0xff` e `+2 = 0xffff` [observado] 0x556d00.

Está valendo hoje (`is_banned` 0x7e85b0): hoje (`[0xae2c90]` dia, `[0xae2c92]` ano) ≥ início **e** (por jogos: `cumpridos + a ≤ jogos`, com `a = 1` na conferência da escalação; por tempo: dias desde o início < `+2`) [observado]. Antes do início a suspensão já está no vetor, mas não impede o jogo [observado].

### O pendente (nó de 0x70 bytes na lista `corpo+7`)

`+0x00` um `BanRecord` inteiro; `+0x14` `TCMDate` em que entra; `+0x1c` id da pessoa; `+0x20..` dados da partida para a notícia; `+0x6f` marca [observado 0x7e7d50; os campos depois de +0x20, inferência]. No dia (`+4` da vtable, 0x7e7d50), cada pendente com data ≤ hoje vira suspensão (`add_ban` 0x556940) ou soma à que já existe, gera a notícia (0x557280, tipo 0x7d2) e sai da lista [observado].

O começo "em 14 dias" das pistas (0x7DD212, 0x7DF766, 0x7DB019) **não** passa pelo pendente: o lance cria a suspensão direto com `add_ban` e início = hoje + 14 (`0x54c5b0`), motivo 2, regra da própria federação na vaga 0 [observado em 0x7df755..0x7df7fb].

### O `discipline.dat` (no save)

Gravado por 0x555dd0 e lido por 0x555860 (item `discipline.dat` do contêiner, id 0x16/4), nesta ordem [observado]:

1. `N × 29` bytes: os registros por pessoa (N = `[0xae2364]`, que é o tamanho do `staff.dat` / 110 [observado no save: 54148]).
2. Para cada regra **não nula**, na ordem 0..42 (`0x7e7b80`): `i32 n`, `n × 14` entradas, `i16 p`, `p × 0x70` pendentes. O arquivo não diz quais regras existem: isso depende das nações carregadas (`[0xae23a8] + nação * 0x122 + 0x11c`, conferido em 0x55d420). Lendo de fora, basta tentar K = 3..43 regras e ficar com o K em que o resto fecha certinho (no save de teste só K = 10 fecha, com os ids todos < N) [observado].
3. `i16 b`, `b × 20` suspensões.
4. `i16 m`, `m × 8` índice por nação.
5. `i16 h`, `h × 10` guardadas: `+0 i32 pessoa`, `+4 i8 vaga`, `+5` a cópia dos 5 bytes da vaga [observado 0x5563f0].

No save de teste: 10 regras (0, 1, 2, 6, 7, quatro vazias e 42; entradas 88, 143, 106, 241, 1448, 0, 0, 0, 0, 107), 1 pendente, 85 suspensões (uma livre), 10 no índice por nação, 0 guardadas [observado]. Exemplo: a pessoa 51013 tem vaga 3 = (regra 0, entrada 36); a entrada 36 da FIFA tem `+6 = 33`; a suspensão 33 é de 1 jogo por vermelho, início no dia 113 de 2001, regras `[−1, −1, −1, 0]`, e o índice por nação tem `(51013, 3)` [observado].

## 2. Como o jogo encerra uma suspensão

Todas passam por **`Discipline::release_ban(pessoa, índice)` = 0x556d00** [observado]:

1. Para cada vaga j = 0..3 com `ban[0xc + j] ≠ −1`: a regra `R = [[D] + 4 * ban[0xc+j]]` recebe 0x7e8590 (`R->entrada(pessoa)`, achada pela vaga `R+4` do registro e conferida com `R+5`, e põe `entrada+6 = −1`).
2. Se j = 3 (FIFA): tira `(pessoa, nação)` do índice `[D+0xc]` (nação = `*(int*)[[0xae23bc] + pessoa*0x6e + 0x1a]`, ou a segunda nacionalidade em `+0x1e`); o último elemento vai para o buraco, `[D+0x12]` −1 e reordena (`qsort` 0x945333, comparador 0x55e5f0).
3. A suspensão vira livre: `+0 = 0xff`, `+2 = 0xffff`. `[D+0xa]` não muda.

O registro da pessoa (vaga e índice) e a entrada (cartões acumulados) **ficam** [observado].

Quem chama:

| Caminho | Rotina | O que faz antes | Fonte |
|---|---|---|---|
| cumprida | `update_ban_progress` 0x556e90 | conta o jogo (`+1`) ou vê o fim do prazo; quando cumpre, grava no histórico (`0x58e340`, `ban_history.tmp`, registro +0x14/+0x18) e solta | [observado] |
| recurso aceito | `apply_appeal_result` 0x557000 | **0x7e91d0** na regra (apaga os pendentes da pessoa nela) e reduz jogos/dias ou solta; sem histórico | [observado] |
| agressão ao árbitro | 0x55e300 | solta as atuais e cria 90 ou 180 dias da FIFA | [observado] |
| transferência | 0x556270 | solta as que não valem no clube novo e limpa a vaga | [observado] |
| pessoa removida | 0x556830 | solta tudo, apaga histórico e guardadas, registro = 0xff | [observado] |
| temporada nova | 0x7e7ed0 (`+0` da vtable) | entrada sem suspensão sai (vaga = −1); suspensão com início há ≥ 2 anos (4 na FIFA) é solta e a entrada sai; as mais novas ficam, com os contadores zerados; depois **compacta** o vetor (0x7e9050) e regrava o índice nas vagas | [observado] |

**Para o editor, repetir o recurso aceito** (é o caminho do próprio jogo para "tirar a suspensão" sem cumprir) [o caminho é observado; escolher ele, inferência]:

```
D = [0xae28f0]; reg = [D+0x14] + s*29
para k em 0..3:
    rs = (i8) reg[5k];  se rs == -1: continua
    R = [[D] + 4*rs];  se R == 0: continua
    idx = (i32) reg[5k+1];  e = [R+0xb] + idx*14
    se idx < 0 ou idx >= [R+0x13] ou [e] != s: continua
    remover da lista [R+7] os nós de 0x70 com [nó+0x1c] == s      (0x7e91d0)
    b = (i16) [e+6];  se b != -1: release_ban(s, b) como acima
```

Para tirar só a suspensão de uma competição: `rs = [[0xadadfc] + 4*id + 0x50]` e a vaga `[R+4]`; lembrar que a suspensão com bit 1 (`+0x13`) também vale pela FIFA (regra 0) [observado 0x55d270].

Cuidado: o índice da vaga muda na temporada nova (compactação) e no carregamento; ler sempre na hora [observado]. Jogador emprestado pode ter vagas na lista guardada (`reg+0x1c > 0`): ali a vaga do registro está 0xff, então `release_ban` não acha a entrada pela vaga; o editor tem de usar o nó guardado (`+5` regra, `+6` índice) e pôr `entrada+6 = −1` direto [inferência pelo código de 0x7e82e0 e 0x556570].

## 3. Outros lugares

| Lugar | Guarda a suspensão? | Precisa mexer? |
|---|---|---|
| `ban_history.tmp` | só as **cumpridas** (lista em arquivo por pessoa, `reg+0x14/+0x18`) | não [observado: só 0x556e90 grava] |
| contadores da entrada (`+4` amarelos) | não; mas, mantidos, o próximo amarelo pode gerar suspensão nova | opcional [inferência] |
| notícia da suspensão (tipo 0x7d2, botão "recorrer") | cópia dos campos para o texto | não: o recurso (0x557000) sai cedo se `entrada+6 == −1` [observado] |
| escalação do clube (`clube+0x1e0`, 20 vagas) | `is_player_banned` tira o suspenso dali | não [observado 0x55d270] |
| conferência da escalação 0x7260f0 | consulta `is_player_banned` (0x55d270) para a lista de suspensos; os códigos de 0x8cf5e0 (`[0xde1f54]`, `get_ineligibility_code`) são regras de inscrição/transferência, não disciplina | não [observado; o significado dos códigos 10/11, inferência] |
| registro de 79 bytes do `general.dat` | é a tabela de contratos por pessoa (`0xAE2D0C`, `CAMPOS.md`), não disciplina | não [observado] |
| `match_stats`, `player stats` (cartões na temporada) | estatística | não [inferência: nenhuma das rotinas de suspensão lê] |
| `staff` (0x6e) | só a nação (+0x1a/+0x1e) para o índice por nação | não |
| `fine.dat` (`[0xae2a3c]`) | multas, outro subsistema | não |

## 4. Ler na memória do jogo rodando

```
D      = [0xae28f0]                       ; Discipline*
N      = [0xae2364]                       ; pessoas
pessoa = [0xae23bc] + s*0x6e              ; [pessoa] == s
reg    = [D+0x14] + s*29
  vaga k: regra = (i8)[reg+5k], idx = (i32)[reg+5k+1]
R      = [[D] + 4*regra]                  ; RulingBody*, pode ser 0
e      = [R+0xb] + idx*14                 ; conferir [e] == s e idx < [R+0x13]
b      = (i16)[e+6]                       ; -1 = nada
ban    = [D+4] + b*20                     ; BanRecord
hoje   = (i16)[0xae2c90] dia, (i16)[0xae2c92] ano
regra da competição = (i8)[[[0xadadfc] + 4*id] + 0x50]
pendentes = lista [R+7]: [+2] primeiro nó; nó: [+0] dado (0x70), [+6] próximo
```

Tudo [observado]. Para listar todos os suspensos: percorrer `[D+4]` até `(i16)[D+0xa]` e pular os livres; o dono sai pelas entradas das regras listadas em `+0xc..+0xf` (a suspensão não guarda a pessoa) [observado].

## 5. Prova no jogo (feita em 25/10/2026: ver `CONFERENCIA_SUSPENSAO.md`, com `engenharia-reversa/remover_suspensao.py`)

Sem cliques daria assim: numa cópia do save, apagar uma suspensão no `discipline.dat` (passos da §2, reescrevendo o item no contêiner tipo 3 sem compressão), carregar com `-load` (`prova_save.py --sav`), deixar a carreira de férias correr um dia e conferir na memória (cadeia da §4) que a entrada continua com `+6 = −1` e o vetor fechado; e, para ver o efeito, que o jogador entra na escalação automática da próxima partida. A gravação de volta do item exige reescrever o índice do contêiner (tamanhos e posições): é trabalho de ferramenta, por isso ficou de fora [inferência].
