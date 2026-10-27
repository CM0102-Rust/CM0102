# A marca do regen: o bit 0x40 do registro de 79 bytes

Investigação de 27/10/2026, só leitura (desmontagem do `cm0102.exe` 3.9.68 e leitura de saves; o jogo não foi rodado), pedida para o filtro "Só regens" dos editores. Hoje eles decidem quem é regen comparando o save com a base `Data`. Isso falha com homônimos e com as 51.353 pessoas da base de fábrica de nascimento desconhecido, cujo dia é sorteado na criação da carreira. Marcação: [observado] = visto no binário ou no save; [inferência] = conclusão sem prova direta.

## Resposta curta

1. **Existe uma marca determinística.** É o **bit 0x40 do byte +0xb** do registro de 79 bytes da pessoa (tabela `[0xAE2D0C]`, gravada no `general.dat`). Só a rotina que cria o regen jogador (`PLAYER_REGEN::regenerate_as_youth_player` 0x7abac0) liga esse bit, e nada no jogo o desliga [observado].
2. **A marca sobrevive à gravação e à carga.** A tabela vai inteira para o `general.dat` e volta inteira na carga [observado no código e em três saves da mesma carreira].
3. **O dia é sorteado na criação da carreira**, não na carga do `.sav`: `database_player_initialisation` 0x535a80, chamada só pela `create_game` 0x828110. O ano não é sorteado: vem do +0x18 da base, que a rotina depois troca pela idade. Por isso o par nome + ano reconhece a pessoa da base (§3).
4. **Não registra.** O regen ocupa a vaga (o id) do aposentado da fila. A ficha de jogador e a nação vêm da primeira entrada de uma reserva de fichas, que é apagada ao ser usada. Nada guarda "no lugar de quem" (§4).

## 1. O bit 0x40

### Onde fica

| Lugar | Endereço |
|---|---|
| Memória | `[0xAE2D0C] + 79 × id + 0xb`, bit 0x40 (quantidade em `[0xAE2D10]` = `[0xAE2364]`) |
| Save | item `general.dat`: quantidade (i32) em **+0xfa3**, registros de 79 bytes a partir de **+0xfa7**, ou seja, o byte `0xfa7 + 79 × id + 0xb` |

[observado] A quantidade em +0xfa3 é igual ao número de pessoas do `staff.dat` (tamanho ÷ 110) nos 6 saves lidos (`Valencia.sav`, `FeriasTesteSave.sav`, `Lucas.sav`, `Roteiro.sav`, `TesteCMRTE.sav`, `TesteTelas.sav`). É a mesma tabela que `CAMPOS.md` chama de "índices de contrato por pessoa" e que `PARES_TRANSFERENCIA.md` lê em `0xfa7 + 79·p`. No mesmo byte, os bits 0x30 são o estado de aposentadoria (0x10 anunciada, 0x20 aposentado; `player_regen_get_retirement_status` 0x7a7eb0 devolve `(byte >> 4) & 3`). O nibble 0x0f e o bit 0x80 têm outros usos.

**A marca não está no `staff.dat`.** Um editor que só lê o registro de 110 bytes não a vê.

### Quem liga

Todo o `.text` foi varrido (capstone; cada acesso a `[0xAE2D0C]` e as escritas de byte, word e dword que cobrem +0xb). Só há duas escritas que ligam o bit [observado]:

**a) A criação do regen jogador**, 0x7abac0. Ela só segue se a pessoa está aposentada (`byte & 0x30 ≠ 0`) e sem clube. Senão limpa os bits 0x30 e sai (0x7abadc..0x7abaf1).

```
7abb2e  mov byte [esi+0x5f], 2          ; classificação: jogador
7abb38  mov [esi+0x61], eax              ; ficha de jogador da reserva
7abb4b  lea eax, [ecx+edx+0xb]           ; [0xAE2D0C] + 79·id + 0xb
7abb4f  mov cl, [ecx+edx+0xb]
7abb53  or  cl, 0x40                     ; <- a marca
7abb56  mov [eax], cl
```

Depois a mesma rotina faz `and 0x7f` (0x7abba2), `and 0xf0` (0x7abc94) e `and 0xcf` (0x7abe82). Nenhuma dessas máscaras apaga o 0x40.

**b) A linha `"ARSE"` do `player_setup.cfg`**, em `player_setup_apply_line` 0x7a3a10: o tipo 0x20 (0x7a324f..0x7a3261) faz `or [registro+0xb], bl` com `bl = 0x40` (0x7a3ad2) **em todas as pessoas** (laço 0x7a3c0c..0x7a3c22). A `create_game` lê o arquivo com a máscara 0x203b, que inclui o 0x20 (0x828344). Nem o `player_setup.cfg` de fábrica nem o da CLZ têm essa linha [observado]. Uma base modificada que a tenha marcaria todo mundo; o editor reconhece esse caso porque o bit aparece na maioria das pessoas.

### Quem não desliga

- **Criação da carreira:** cada registro começa com `+0xb = lixo & 0x80` (0x828548..0x828561), então o bit 0x40 começa **zerado** em todas as pessoas [observado].
- Todas as outras escritas no byte preservam o 0x40: `and 0x7f` (0x4c7675, 0x4cf444, 0x7abba2), `or 0x80` (0x4c910e, 0x4cf091…), `and 0xcf` (0x7a5ae8, 0x7a6358, 0x7ab362…), `and 0xef / or 0x20` (0x7a63d7), `and 0xdf / or 0x10` (0x7a491d, 0x7a657c), `and 0xf0` (0x5361b1, 0x7ab396), `+1`/`−1` só no nibble 0x0f (0x842728, 0x84402e) [observado].
- **Ressalva:** a varredura acha escritas a até ~20 instruções de uma leitura de `[0xAE2D0C]`. Um ponteiro para o registro guardado por mais tempo escaparia. Os saves (abaixo) não mostram nenhum caso desses.

### Quem lê

`club_screen_draw_info_tab` 0x4617bf, `write_coach_report_news_text` 0x488207, `player_search_assess_player_traits` 0x7bdc3a, `apply_obscene_gesture_ban` 0x7db007 e `staff_screens_draw_profile_tab` 0x87b117/0x87b854 testam o bit [observado]. Que o significado seja "jovem gerado pelo jogo" é **inferência**: a única origem normal do bit é o regen.

### Conferência nos saves

Scripts em `%TEMP%\marca` (só leitura). "Bate com a base" = mesmo primeiro, segundo e comum (em texto) e mesma data (dia do ano e ano). Na CLZ o ano é deslocado em +24.

| Save (base) | Pessoas | Com o bit | Classificação +0x5f dos marcados | Marcados que batem com a base | Só homônimos | Nome ausente da base |
|---|---|---|---|---|---|---|
| `clz_teste\Valencia.sav` (CLZ, 2028) | 62.487 | 3.857 | todos 2, todos com ficha de jogador | **0** | 419 | 3.438 |
| `jogo\TesteTelas.sav` (fábrica, 2002) | 158.727 | 492 | todos 2 | **0** | 31 | 461 |
| `jogo\TesteCMRTE.sav` / `Roteiro.sav` | 158.727 | 13 / 1 | todos 2 | — | — | — |
| `jogo\FeriasTesteSave.sav`, `Lucas.sav` | 54.148 / 42.314 | 0 | — | — | — | — |

- No `Valencia.sav`, 49.808 pessoas sem o bit batem nome + data com a base, e nenhuma marcada bate. Os 419 "só homônimos" são exatamente o caso que hoje engana os editores. O bit resolve sem consultar a base.
- Idades dos regens na criação (+0x18 do `TesteTelas`): 15 a 27, a maioria de 16 a 19, como a rotina sorteia.

## 2. A marca na gravação e na carga

- **Gravação:** `save_game_data` grava a tabela inteira: `zwrite([0xAE2D0C], 0x4f, [0xAE2D10])` (0x82e934..0x82e944) [observado].
- **Carga:** `load_game_data` 0x82a9c0 aloca 79 × N (0x82af53..0x82af6d). Na versão atual do save (`|[0xDD7110]|` = 0x3e047 ≥ 0x3e045), lê os 79 bytes de cada pessoa tal como estão (`zread(…, 0x4f, N)`, 0x82b634..0x82b643). Nenhum passo posterior mexe no bit. Só versões < 0x3e018 limpam o 0x80 (0x82b682). Saves de versões antigas convertem campo a campo e também copiam o 0x40 (0x82b05e..0x82b069) [observado].
- **Na prática:** `Roteiro.sav` → `TesteCMRTE.sav` → `TesteTelas.sav` (gravados em 15/10, 19/10 e 26/10, mesmas 158.727 pessoas) têm 1 → 13 → 492 marcados. Todo id marcado num save continua marcado no seguinte, com o mesmo nome e nascimento (1 de 1, 13 de 13) [observado]. Que sejam a mesma carreira é inferência pelos mesmos regens, de nome e data sorteados.

## 3. O sorteio do nascimento desconhecido

**Quando:** na criação da carreira. `database_player_initialisation` 0x535a80 tem um só chamador: `create_game` em 0x828894 [observado]. Carregar um `.sav` não sorteia nada: as datas vêm prontas no `staff.dat` do save.

**A base:** a de fábrica tem 51.353 pessoas com nascimento `(dia 31, ano 1900, +0x14 = 1)`. Nelas o +0x18 guarda o ano verdadeiro em 26.327 casos (> 1900) e 0 ou ≤ 1900 em 25.026. A base da CLZ tem 1.812 pessoas assim, com +0x14 = 0 [observado].

**O que a 0x535a80 faz** [observado]:

- **Pessoa com ficha de jogador** (0x536836..0x53689c): se a idade (byte do ano atual − byte do ano +0x12) for < 14 ou > 50, o que sempre vale para 1900, o nascimento passa a `date_set(dia = sorteio(27)+1, mês = sorteio(11), ano)`:
  - ano = +0x18 se +0x18 > 1900;
  - senão 1967 (se tem ficha de não jogador) ou 1980.
- **Só não jogador** (0x537c71..0x537cbe): se a idade for < 29 ou > 75, o ano = +0x18 se 1900 < +0x18 < 1967; senão 1940 + sorteio(26).
- `date_set` 0x54a130 grava o dia do ano em +0x10 e o ano em +0x12, e **recalcula +0x14** (bissexto). Some o "1" da base.
- No fim de cada pessoa, **+0x18 recebe a idade**. O ano que a base guardava ali some da memória e do save.

**O que sobra para reconhecer a pessoa da base** [observado]:

- O mês e o dia sorteados caem sempre entre o dia 1 e o 27 e entre janeiro e novembro. O sorteio nunca dá dezembro nem os dias 28 a 31.
- O ano não é sorteado quando a base tinha o +0x18.

No `TesteTelas.sav`, das 35.185 pessoas de nome único na base e nascimento desconhecido:

| Regra | Pessoas |
|---|---|
| ano = +0x18 da base | 16.785 |
| ano 1967 ou 1980 (jogador sem +0x18) | 7.932 |
| ano 1940..1965 (não jogador) | 10.293 |
| Total pelas regras | 35.010 (99,5%) |
| Dia ≤ 27 e mês ≤ novembro | todas, menos 5 |

O resto (175) não foi investigado [inferência: homônimo trocado pelo corte da base ou vaga já reaproveitada]. A rotina não deixa outra marca nessas pessoas.

## 4. "No lugar de quem"

- A fila de regens tem entradas de 12 bytes (id da pessoa, data), criadas por `queue_staff_for_regen` 0x7a87d0 quando um aposentado não segue como não jogador. Na data, `process_regen_queue` 0x7a65d0 chama `0x7abac0(pessoas + id × 0x6e)` (0x7a664f). O regen **é a mesma vaga, o mesmo id do aposentado**: a rotina sobrescreve nomes, nascimento, nação, ficha e clube [observado].
- A ficha de jogador e a nação vêm da **primeira entrada da reserva de fichas** (`this+0x3c`, 16 bytes: id da ficha, nação, segunda nação, e em 0x7a87d0 o id do dono). A entrada é apagada na hora (`remove_player_data_from_pool` 0x7a8470, chamada em 0x7abb1a). O dono da ficha pode ser outro aposentado [observado].
- Nada persistente liga o regen ao antecessor. O `retire.dat` guarda só as filas pendentes, e a entrada sai da fila quando o regen nasce (0x7a65d0) [observado].
- **Para o editor:** dentro de um save não há como saber de quem era a vaga. Entre dois saves da **mesma carreira**, o mesmo id com outro nome é a troca. Comparar com a base não serve: a criação compacta as tabelas (`database_trim_staff_tables` 0x524d70), então os ids do save não são os da base. Isso também derruba a pista "id acima da contagem da base": o regen tem o id de quem se aposentou (no `Valencia.sav`, de 19 a 61.158).

## 5. Outras pessoas geradas pelo jogo (sem o bit)

O bit marca só o regen **jogador**. O jogo fabrica mais três grupos, e nenhum tem marca determinística:

| Grupo | Rotina | Como aparece no save |
|---|---|---|
| Regen não jogador | `player_regen_regenerate_as_non_player` 0x7ab340: novos nomes e nascimento na vaga do aposentado | +0x5f = 1, sem bit |
| Pessoal extra da criação | `database_generate_extra_non_playing_staff` 0x527240 (0x527545…: +0x5f = 1) | não jogadores sem nome na base |
| Jogadores fabricados na criação | `database_trim_staff_tables` 0x524d70 (0x526385..0x52651f) | ver abaixo |

Os jogadores fabricados na criação:

- ocupam os ids logo **antes dos 16 técnicos humanos**: `N − 17 − i`;
- nomes −2, sorteados depois na 0x535a80;
- ano de nascimento **1982** (0x7be, em 0x52638c/0x5263b6; na CLZ aparece 2006 por causa do deslocamento de +24);
- cargo 0xb, +0x5e = 2, +0x5f = 2.

O total é 1% (opção de tamanho 1 ou 2: 2%) das pessoas mantidas (0x524dcf/0x524de1; ×0.01 ou ×0.02, `__ftol`). Medido nos saves [observado]:

| Save | Ids | Quantos | Nascidos em |
|---|---|---|---|
| `TesteTelas` | 157.140..158.710 | 1.571 | 1982 |
| `FeriasTesteSave` | 53.596..54.131 | 536 | 1982 |
| `Valencia` | 61.246..62.470 | 1.225 | 2006 |

No `TesteTelas` não há nenhum jogador sem o bit e sem nome na base fora desse bloco. No `Valencia` há 28, e pelos nomes parecem pessoas reais com outra grafia na base [inferência].

O `+0x5f` não é marca de regen: é a classificação (0 técnico humano, 1 não jogador, 2 jogador, 3 outro). A reserva de tipo 6 não chega aos saves (nenhum dos 6 tem 6) [observado].

## 6. Recomendação para os editores

**Certo (pelo código e conferido nos saves):**

1. **Regen jogador** ⇔ bit 0x40 em `general.dat[0xfa7 + 79 × id + 0xb]`.
   - Antes, conferir que `i32 general.dat[0xfa3]` = número de pessoas do `staff.dat` (tamanho ÷ 110).
   - Se o bit aparecer na maior parte das pessoas, a base usou `"ARSE"` no `player_setup.cfg`: não confiar no filtro e avisar.
   - Não precisa da pasta `Data`, de nomes nem de datas. Homônimos e nascimentos sorteados deixam de importar.
2. Ao vivo (memória do jogo): o mesmo bit em `[0xAE2D0C] + 79 × id + 0xb`.

**Heurística forte (opcional, para "gerados na criação"):** descer de `N − 17` enquanto a pessoa for jogador (+0x5f = 2) nascido no ano fabricado (1982 na base de fábrica; 1982 + deslocamento numa base com anos deslocados) ou marcado com o bit. O trecho é o bloco fabricado.
- O tamanho esperado é ~1% ou 2% das pessoas.
- Com os anos, aposentadorias e regens corroem o bloco: é heurística, não marca.

**Não usar como critério:**
- +0x5f (é a classificação);
- faixa de id (regens reaproveitam vagas, e a criação renumera);
- +0x14 (incoerente com o ano em ~24 mil pessoas da base também);
- comparação com a base por nome + data.

**Se o editor ainda quiser casar uma pessoa com a base** (para outros fins): use nome em texto + ano, e aceite o dia/mês sorteados quando a base tinha `31/1900`. O ano bate com o +0x18 da base quando ele é > 1900; senão é 1967/1980 (jogador) ou 1940..1965 (não jogador). O dia fica entre 1 e 27, e o mês entre janeiro e novembro.

**Não marcado:** regens não jogadores e o pessoal extra da criação. Para eles não há marca. O filtro "Só regens" deve dizer que cobre só os jogadores gerados.
