# A mesma pessoa repetida nas vagas da comissão do clube (`club.dat` +415)

Investigação de 25/10/2026, só leitura e desmontagem (sem rodar o jogo), pedida para o aviso N6 do validador do CM3SaveEditor. [observado] = visto no binário ou no save; [inferência] = conclusão sem prova direta.

## As vagas

O registro de clube (581 bytes) tem em +415 (0x19f) 15 vagas seguidas: 5 treinadores (+415..+431), 7 olheiros (+435..+459) e 3 fisioterapeutas (+463..+471). No save, vaga vazia é **−1** e o resto é o id da pessoa: `save_database` 0x52dad0 grava ponteiro nulo como −1, e `resolve_database_indexes` 0x5315a0 volta id negativo para nulo [observado]. **0 não é vazio: é a pessoa de id 0.**

## Os casos nos saves

| Save | Repetições em +415 | Onde |
|---|---|---|
| `clz_teste\Valencia.sav` (o mesmo do CMExplorer, MD5 67e35978…) | 6, em 2 clubes | clube 894 (Real Avilés Industrial): `[10026, 1379, 10026, 10026, 10026]`; clube 2057 (Cusco FC): `[30189, 28530, 30189, 30189, 30189]` |
| `jogo\FeriasTesteSave.sav` | 0 | — |
| base CLZ (`clz_teste\Data\club.dat`) | 0 | — |

[observado] Só as vagas de treinador; olheiros e fisioterapeutas nunca. A pessoa repetida tem, no `staff.dat`, clube = o próprio clube e cargo 8 (treinador), tem registro de não jogador e não tem jogador. As 6 repetições aparecem em +423/+427/+431 (a primeira, em +415, não conta como repetição). Das 239 do N6 no Valencia, as outras 233 estão em outras listas: rivais (+179, 58) e preferências (175). Os exemplos do pedido (clube 498 com o id 97604, clube 2681 com o 132058) **não aparecem** neste save: ele tem 62.487 pessoas, e o clube 2681 está com as 15 vagas vazias.

## 1. Quem grava: a fila da regeneração (defeito)

`PLAYER_REGEN::process_regen_queue` 0x7a65d0 dá registro de não jogador a jogadores mais velhos de clubes sem técnico humano. Quando o clube conta menos de 5 treinadores (`database_count_club_coaches` 0x53c8d0) e a habilidade bate com a reputação, o jogador vira **jogador-treinador** (cargo 15) [observado, 0x7a67b3..0x7a67e8]:

```
7a67ca  mov eax,[esi+0x39]            ; o clube
7a67cd  mov ebp,[ecx+eax+0x19f]       ; vaga j de treinador
7a67db  test ebp,ebp / jne 7a67e4
7a67df  mov [eax],esi                 ; grava a pessoa
7a67e1  mov byte [esi+0x3d],bl        ; cargo 15
7a67e4  add ecx,4 / dec edx / jne 7a67ca   ; segue: não para na primeira
```

O laço não sai depois da primeira vaga vazia: a pessoa entra em **todas** as vagas vazias de treinador. Por isso o padrão `[X, Y, X, X, X]`: Y já estava na vaga 1 e X ocupou as outras 4. Depois, quando ele para de jogar, `PLAYER_REGEN::retire_from_playing_staff` 0x7aace0 troca o cargo 15 por 8 e tira a pessoa só do elenco (+0xd7). As cópias nas vagas de treinador ficam [observado no código]. Isso bate com o que está no save: cargo 8, sem jogador [inferência: a sequência não foi vista acontecendo].

É um defeito (falta um `break`), não uma pessoa com várias funções. As funções vão em campos diferentes (técnico +0xcf, auxiliar +0xd3, elenco +0xd7), e nenhuma outra rotina repete alguém de propósito [observado]. As outras rotinas que escrevem nessas vagas descartam as outras hipóteses:

- `database_add_staff_to_club` 0x53bd10 ocupa só a primeira vaga vazia.
- `database_generate_extra_non_playing_staff` 0x527240 e `database_merge_b_team_staff` 0x544970, na criação, põem uma pessoa nova por vaga.
- A carga (0x5315a0) e a gravação (0x52dad0) só convertem id ↔ ponteiro.
- A base da CLZ chega sem repetição.

Outra brecha, sem caso no save: a 0x53bd10 não confere se a pessoa já está na lista, e `database_club_has_room_for_job` 0x53b9a0 trata "já está aqui" como vaga. Por esse caminho também sairia repetição se alguém reincluísse uma pessoa que já está na lista [observado no código].

## 2. O efeito no jogo

- **Contagem:** a 0x53c8d0 conta cada vaga, então o clube 894 tem 5 treinadores em vez de 2. Na própria regeneração (`> 4`), o clube não recebe mais jogadores-treinadores [observado].
- **Vagas livres:** com as 5 casas ocupadas, a 0x53b9a0 diz que não há vaga para outro treinador e a 0x53bd10 falha. A contratação pela `CONTRACT_MANAGER::sign_contract` 0x4cf720 e o preenchimento de cargo vago 0x8cb6d0 não conseguem pôr treinador nesse clube [observado no código].
- **Saída:** `database_remove_staff_from_club` 0x53c190, usada em transferências e demissões, zera só a **primeira** ocorrência. Quando a pessoa sai, as outras ficam como treinador "fantasma", apontando para alguém de outro clube. `MANAGER_MANAGER::remove_staff_from_club` 0x690890 e a aposentadoria 0x7aaeb0 limpam todas as ocorrências [observado].
- **Treino:** `database_get_club_coaches` 0x53cc30 devolve a mesma pessoa várias vezes, mas só a distribuição de treinos dos clubes humanos (0x8b2880) usa essa lista. Os dois casos são de clubes do computador [observado; efeito no treino humano: inferência].
- **Salário:** sai do contrato, que é um por pessoa, e não das vagas. A repetição não multiplica o salário [inferência].

**Esvaziar as repetições é seguro.** O resultado é exatamente o estado de uma inclusão normal: a pessoa numa vaga, com clube e cargo 8 nela mesma. Tudo o que tira a pessoa do clube (0x53c190, 0x690890) continua achando a pessoa. O clube volta a ter vagas.

## 3. Recomendação para o validador

**Corrigir, sem perguntar**, só na faixa dos treinadores (+415..+431):

1. Manter a **primeira** ocorrência e gravar **−1** nas seguintes (nunca 0, que é a pessoa de id 0). É o mesmo que o validador original fazia.
2. Não mexer no `staff.dat`: o clube (+0x39) e o cargo (8, ou 15 se ainda for jogador) já estão certos.
3. Não precisa compactar: o jogo aceita buracos e ocupa a primeira vaga vazia.

Texto sugerido: *"clube N: o treinador X aparece em K vagas. É um defeito do jogo (a regeneração põe o jogador-treinador em todas as vagas vazias). As repetições foram esvaziadas e o clube volta a ter vagas de treinador."*

Repetição em olheiros ou fisioterapeutas não tem causa conhecida no jogo: corrigir do mesmo jeito (−1 nas seguintes), mas com o aviso genérico de lista repetida. Rivais e preferências são outro assunto e não foram investigados aqui.
