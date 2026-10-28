# A marca do regen: o bit 0x40 do registro de 79 bytes

Investigação de 27/10/2026, só leitura (desmontagem do `cm0102.exe` 3.9.68 e leitura de saves; o jogo não foi rodado), pedida para o filtro "Só regens" dos editores. Hoje eles decidem quem é regen comparando o save com a base `Data`. Isso falha com homônimos e com as 51.353 pessoas da base de fábrica de nascimento desconhecido, cujo dia é sorteado na criação da carreira. Marcação: [observado] = visto no binário ou no save; [inferência] = conclusão sem prova direta.

## Resposta curta

1. **Existe uma marca determinística.** É o **bit 0x40 do byte +0xb** do registro de 79 bytes da pessoa (tabela `[0xAE2D0C]`, gravada no `general.dat`). Só a rotina que cria o regen jogador (`PLAYER_REGEN::regenerate_as_youth_player` 0x7abac0) liga esse bit, e nada no jogo o desliga [observado].
2. **A marca sobrevive à gravação e à carga.** A tabela vai inteira para o `general.dat` e volta inteira na carga [observado no código e em três saves da mesma carreira].
3. **O dia é sorteado na criação da carreira**, não na carga do `.sav`: `database_player_initialisation` 0x535a80, chamada só pela `create_game` 0x828110. O ano não é sorteado: vem do +0x18 da base, que a rotina depois troca pela idade. Por isso o par nome + ano reconhece a pessoa da base (§3).
4. **Não registra.** O regen ocupa a vaga (o id) do aposentado da fila. A ficha de jogador e a nação vêm da primeira entrada de uma reserva de fichas, que é apagada ao ser usada. Nada guarda "no lugar de quem" (§4).
5. **Herda, mas de dois antecessores diferentes.** Do dono da ficha (B, um jogador que se aposentou) o regen herda o PA, as posições, os pés, a nação e o perfil dos atributos técnicos e ocultos; o CA é sorteado como fração desse PA. Do dono da vaga (A, em geral um não jogador aposentado) herda só o id, a personalidade do registro de pessoa e as preferências. A e B só coincidem quando a fila está curta (§8).

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

## 7. Quando o regen entrou (28/10/2026)

Pedido para a coluna "Entrou em" da lista de regens. Mesmo método: desmontagem e leitura de saves, sem rodar o jogo.

### O que a criação grava

A `regenerate_as_youth_player` 0x7abac0 grava **uma** data do dia: a entrada no clube [observado].

```
7abda8  mov ax, [0xae2c92]      ; ano atual
7abdb0  mov [esi+0x40], ax      ; entrada no clube: ano
7abdb4  mov cx, [0xae2c90]      ; dia do ano atual
7abdbd  mov [esi+0x3e], cx      ; entrada no clube: dia
7abdc1  mov [esi+0x39], 0       ; ainda sem clube
```

O resto da rotina não grava data do dia:

- O nascimento é `+0x12 = ano atual − idade` e `+0x10 = sorteio(365)` (0x7ac74e/0x7ac75d), com a idade sorteada entre **15 e 27**.
- O registro de 79 bytes recebe só `+8 = 0`, `+9 = 0x9c` e `+0xd = 0`, que não são datas.
- Quando há clube, `player_regen_sign_for_club` 0x7a6f10 põe o regen numa vaga do elenco direto (0x7a6f6a). Ela **não** chama `database_add_staff_to_club` e não mexe no +0x3e. Grava só o fim do contrato (+0x46, 0x7a7061: ano atual + 2, ou + 1..6 a partir dos 19 anos) e o contrato (0x4c7950).
- Logo, enquanto o regen não troca de clube, +0x3e/+0x40 é **o dia em que o jogo o criou** [observado].

### Quando a data muda

A data vai para o `staff.dat` do save (+0x3e/+0x40, i16 dia do ano e i16 ano) e volta na carga como qualquer campo da pessoa. Mas várias rotinas a regravam [observado]:

- `database_add_staff_to_club` 0x53bd10 (0x53be14), quando recebe uma data: entrada num clube;
- `database_remove_staff_from_club` 0x53c190 (0x53c24c): saída;
- `TRANSFER_MANAGER::free_transfer_staff` 0x8c1af0 (0x8c1d6c, grava o dia atual);
- `CONTRACT_MANAGER::sign_loan_contract` 0x4d01e0 (0x4d0431): empréstimo;
- `MANAGER_MANAGER` 0x690472, 0x69b804 e 0x69f5d4: técnicos.

Assim, depois de uma transferência, de um empréstimo, de uma dispensa ou da contratação de um regen que estava sem clube, o campo passa a ser a **entrada no clube atual**. A renovação de contrato não mexe nele (abaixo).

### Conferência nos saves

| Save | Regens | Ano de entrada (+0x40) | Sem clube |
|---|---|---|---|
| `Roteiro.sav` (hoje ≈ dia 213 de 2001) | 1 | 2001 (dia 204) | 1 |
| `TesteCMRTE.sav` (≈ dia 293 de 2001) | 13 | todos 2001, dias 204 a 290 | 4 |
| `TesteTelas.sav` (gravado de novo em 27/10 às 17:24, ≈ dia 50 de 2002) | 533 | 27 em 2001, 506 em 2002 | 207 |
| `Valencia.sav` (CLZ, dia 329 de 2028) | 3.857 | 431 em 2026, 1.514 em 2027, 1.912 em 2028 | 214 |

O `TesteTelas.sav` mudou durante a investigação: tinha 492 marcados na §1 e 533 nesta leitura.

- **Toda data de entrada cai depois do início da carreira** e não passa de "hoje" [observado]: 2001 em diante na base de fábrica, 2026 em diante na CLZ, que começa em 2025. Como o regen nasce na carreira e as regravações usam o dia corrente, isso vale por construção.
- **Sequência Roteiro → TesteCMRTE → TesteTelas** (os 13 regens em comum):
  - 2 mudaram de clube, os dois saindo de "sem clube": ids 86141 (de 253/2001 para 348/2001) e 107241 (de 277/2001 para 340/2001). **A entrada mudou nos 2.**
  - 11 não mudaram de clube, e a entrada continuou igual nos 11.
  - Um deles (id 32095) teve o fim de contrato trocado de 178/2003 para 160/2003, uma renegociação, e a entrada **não** mudou.
- O `Valencia.sav` tem 118 regens com entrada no dia 0 de 2028 (1º de janeiro), um pico que combina com transferências na virada do ano [inferência]. Esses já não mostram a data de criação.

### Deduzir a temporada sem a data

A idade de criação é sorteada entre 15 e 27, e o ano de nascimento é **exatamente** `ano da criação − idade` [observado; 0x7ac74e e os ramos do sorteio da idade]. Daí:

- **ano da criação ∈ [nascimento + 15, nascimento + 27]**;
- e também **≤ ano da entrada (+0x40)**, porque toda troca de clube só move a data para frente;
- e **≥ ano do início da carreira**.

Nos saves, `ano da entrada − ano de nascimento` deu 16 a 27; quase todos ficam entre 16 e 20 (`Valencia`: 3.741 de 3.857). Isso é distribuição, não regra: o nascimento sozinho não fixa a temporada [observado nos saves; usar a faixa mais provável é inferência].

Outras fontes possíveis, **não conferidas**: a primeira temporada do histórico do jogador (`PlayerStats`) e a notícia de promoção de juvenil (0xbce, 0x7a8ba0), que as notícias antigas apagam.

### Critério recomendado para a coluna

1. **"Entrou em" = +0x3e/+0x40** (dia do ano e ano), com o rótulo "entrada no clube atual (ou criação, se nunca trocou de clube)". É a data da criação exata enquanto o regen não trocou de clube, e o editor não tem como saber, por um save só, se trocou.
2. Para não mentir, mostrar ao lado a faixa da criação: **de max(início da carreira, nascimento + 15) a min(+0x40, nascimento + 27)**.
3. Se o editor guardar os saves da carreira (ou um cache por id + nome), use a **primeira** data vista para cada regen. Entre saves, a data só muda quando o clube muda; renovação não muda.

## 8. O que o regen herda (28/10/2026)

Pergunta do Lucas: "se o regen não herda nada do aposentado, não faz sentido vincular um ao outro". Mesmo método: desmontagem e pseudocódigo da 0x7abac0 e das rotinas da fila e da reserva, e leitura de saves, sem rodar o jogo. Scripts em `%TEMP%\marca` (`heranca.py`, `cruza_heranca.py`, `extra_heranca.py`, `b_depois.py`, `ficha_dividida.py`; só leitura).

Há **dois** antecessores, e o "no lugar de" mistura os dois:

- **A, o dono da vaga:** a pessoa da cabeça da fila de regens. O regen fica com o id dela (§4).
- **B, o dono da ficha:** o jogador cuja ficha de jogador (`player.dat`, 70 bytes) é a primeira da reserva. O regen fica com essa ficha.

### Como a fila e a reserva se formam

`PLAYER_REGEN::process_retirements` 0x7a59e0 trata cada aposentado por um de três caminhos [observado]:

| Quem se aposenta | Fila (12 bytes, `this+0x10`) | Reserva de fichas (16 bytes, `this+0x3c`) |
|---|---|---|
| Jogador que não vira técnico | entra, por `queue_staff_for_regen` 0x7a87d0 (chamada em 0x7a64e2) | entra a ficha dele: id da ficha, nação (+0x1a), id da pessoa em +0xc (0x7a8ab0..0x7a8adc). O +8 (segunda nação) não é gravado aí. **O +0x61 dele não é zerado** |
| Jogador que vira técnico (já tem ficha de não jogador, ou ganha uma da reserva de não jogadores por sorteio com o profissionalismo +0x5b, 0x7a5cdb..0x7a5d98) | não entra | entra a ficha dele, por `add_player_data_to_pool` 0x7a8350 (chamada em 0x7a6335: ficha, nação, segunda nação). Depois `+0x61 = 0` e `+0x5f = 1`: ele segue vivo como não jogador |
| Não jogador | entra, por 0x7a87d0 (0x7a64c9), ou vira um não jogador novo na mesma vaga (`regenerate_as_non_player` 0x7ab340) | nada (a ficha de não jogador vai para a outra reserva, `this+0x1c`) |

- A data da entrada na fila é hoje, se a pessoa tem menos de 30 anos, ou hoje + 1 a 65 dias (0x7a6397..0x7a63b4) [observado].
- A fila é ordenada por data e depois por id (`player_regen_compare_queue_entries` 0x7ae240, no fim da 0x7a59e0). A reserva fica na ordem de chegada; só a 0x7a87d0 e a 0x7a8350 acrescentam nela [observado; `refs.py`].
- `process_regen_queue` 0x7a65d0 anda enquanto a data da cabeça da fila for anterior a hoje (0x7a660b) **e** a reserva não estiver vazia (0x7a6618). Para cada cabeça chama a 0x7abac0, que pega **sempre a entrada 0 da reserva** (0x7abb0e..0x7abb16) e a apaga (0x7abb1a). Não há nenhuma comparação entre a vaga e a ficha: nem nação, nem posição, nem nível, nem clube [observado].

Então:

- **B é sempre um jogador que se aposentou**: ou está ele mesmo na fila esperando a sua vez, ou virou técnico [observado no código e nos saves].
- **A pode ser qualquer aposentado da fila**, e com a fila acumulada é quase sempre um não jogador. Os não jogadores entram na fila sem ficha, então a reserva vira o gargalo: cada ficha nova vai para a vaga mais antiga [observado].
- **A = B só quando a fila está curta**, e a cabeça da fila é o próprio jogador cuja ficha está na frente da reserva [observado nos saves, abaixo].

Nos saves da mesma carreira (§1):

| Par de saves | Fila / reserva no primeiro | Regens novos | B = A | A era não jogador | B ativo (sem aposentadoria) no primeiro |
|---|---|---|---|---|---|
| `Roteiro` → `TesteCMRTE` | 2 / 2 (as duas fichas são das duas pessoas da fila) | 12 | 2 | 10 | 10 (os outros 2 são os B = A) |
| `TesteCMRTE` → `TesteTelas` | 847 / 0 | 520 | **0** | 518 | 520 (idades 25 a 50, 506 entre 30 e 39) |

No `TesteCMRTE`, a fila tem 847 vagas paradas e a reserva está vazia. No `Valencia.sav` são 26 vagas e reserva vazia [observado].

### O que vem de B (o dono da ficha)

A 0x7abac0 põe a ficha da reserva em +0x61 (0x7abb38) e **não a apaga nem a refaz**. Ela regrava só parte da ficha [observado: todas as escritas da rotina, e as das chamadas]:

- **Sorteados de novo:** CA (+5), as três reputações (+9, +0xb, +0xd), aceleração (+0x1b), agilidade (+0x1d), impulsão (+0x2e), forma física natural (+0x34), velocidade (+0x36), resistência (+0x3c), força (+0x3d) e índice de trabalho (+0x44).
- **Mexidos a partir do valor de B:** antecipação (+0x1e) e decisões (+0x24) perdem um sorteio de até 40, 20 ou 10 conforme a idade; jogos importantes (+0x2c) e liderança (+0x2f) perdem um sorteio de até 3 a 10; consistência (+0x21) só sobe, e só se for < 5 num regen de 26 a 28 anos.
- **Nunca escritos:** o PA (+7) só é lido. Nem as posições (+0xf..+0x1a), os pés (+0x30, +0x3b) e os outros atributos.
- No fim, `staff_normalise_player_physical_attributes` 0x538ec0 e `staff_normalise_player_attributes` 0x538bc0 rebalanceiam o vetor de atributos. Elas partem dos valores que já estão na ficha (`database_player_to_attribute_vector` 0x53a610 → `database_attribute_vector_to_player` 0x53aa10), ou seja, dos de B.

**O CA sai do PA de B.** Exemplo, no ramo da nação de reputação < 350 (0x7abea0..0x7abffa): CA = PA/4 + sorteio(0,55·PA) a partir dos 21 anos, ou PA/4 + sorteio(0,6·PA) até os 20. O outro ramo (0x7ac3dd..) também usa só frações do PA (0,25, 0,55, 0,6…) [observado].

**O sorteio depende de B, não de A** [observado]:

- a idade e as fórmulas do CA dependem da nação (de B), da reputação dela (+0x88), do PA e das posições da ficha (goleiro +0xf, defesa +0x11, atacante +0x15);
- nomes: da nação, por `database_pick_first_name_for_nation` 0x539c60, 0x53a1b0 e 0x53a5b0;
- clube: `pick_best_club_for_player` 0x7ad0e0 ou `pick_club_from_nation_list` 0x7ad590, pela nação e pela habilidade do regen.

**Nação:** a da entrada da reserva, ou seja, a de B (0x7abcd4). Se a reputação dela for < 100, há 1 chance em 4 de trocar por uma nação sorteada, também de reputação < 100 (0x7abcd7..0x7abd1d). A segunda nação vem do +8 da entrada:
- sempre, se for o País Basco;
- senão com 1 chance em 3;
- senão zero (0x7abd20..0x7abd6e).
- Se a nação for a Espanha, 1 vez em 100 a rotina pula esse passo e o +0x1e fica com a segunda nação de A (0x7abd2c..0x7abd3f).

Conferido nos 520 regens de `TesteCMRTE` → `TesteTelas`: B foi achado como dono anterior da ficha em todos [observado].

| Comparação regen × B | Iguais |
|---|---|
| PA | 520 de 520 |
| 12 posições | 520 |
| pés esquerdo e direito | 520 |
| nação | 520 (nação de A: só 14, coincidência) |
| agressividade, equilíbrio, bravura, deslealdade, talento, propensão a lesões | 520 cada |
| versatilidade, consistência, jogos importantes, mãos, liderança, escanteios, faltas, espírito de equipe, técnica | 259 a 454 cada |
| demais atributos | 10 a 63 cada |
| CA | 14 |

- Diferença média por atributo (+0x1b..+0x44): **4,3** entre o regen e B, contra **14,0** entre o regen e uma ficha qualquer [observado].
- No par `Roteiro` → `TesteCMRTE` vale o mesmo nos 12 regens: PA, posições, pés e nação iguais aos de B; CA diferente; diferença média 4,2 contra 15,6 [observado].

### O que vem de A (o dono da vaga)

Além do id, a 0x7abac0 e as chamadas dela **não escrevem** nestes campos do registro de pessoa (110 bytes) [observado]:

- a personalidade +0x56..+0x5d (adaptabilidade, ambição, determinação, lealdade, pressão, profissionalismo, espírito esportivo, temperamento). A determinação (+0x58) passa pelo rebalanceamento da 0x53aa10, mas parte do valor de A;
- as preferências (+0x65: clubes e pessoas favoritos e detestados);
- +0x60 (ClubValuation).

A adaptabilidade de A ainda pesa no sorteio dos idiomas do regen: o corte é (20 − adaptabilidade) × 10 (0x7abded) [observado].

Nos 520 regens:
- a personalidade sem a determinação é a de A em 514 [observado];
- a determinação, em 518 [observado];
- o índice das preferências, em 520 [observado];
- as 6 diferenças restantes não foram investigadas [inferência: mudanças depois da criação].

O resto de A é apagado ou trocado [observado]:

- gravados pela própria 0x7abac0: nomes, nascimento, idade, nação e segunda nação, jogos e gols pela seleção (+0x22/+0x23), cargo na seleção (+0x24), clube, salário, valor e a ficha de não jogador (+0x69 = 0). A rotina só segue se A estiver sem clube (0x7abaec);
- limpos pelas chamadas:

| O que | Rotina |
|---|---|
| estatísticas | `PlayerStats::reset_player` 0x7c6650 |
| histórico | 0x86fb80 |
| disciplina | 0x55e1d0, 0x556830 |
| lesões | 0x631f90, 0x62bfa0 |
| contratos | 0x4d0b50, 0x4d0bd0 |
| seleção | 0x775ba0 |
| névoa de guerra | 0x5afd00 |
| apelido | 0x5fff30 |

**O clube, a posição e a reputação de A não entram em nada**: A já está sem clube, e a ficha dele, se tinha uma, foi para a reserva e pode ir para outro regen [observado].

### A ficha dividida no save

Como a 0x7a87d0 não zera o +0x61 do jogador que entra na fila, **enquanto ele espera a vez dele, a mesma ficha é apontada por duas pessoas**: o aposentado na fila (B) e o regen que já a usa. Quando chega a vez de B, a 0x7abac0 troca o +0x61 dele pela ficha seguinte da reserva [observado].

| Save | Fichas com dois donos | O outro dono |
|---|---|---|
| `jogo\TesteTelas.sav` de 27/10 às 17:24 (SHA-256 `571a96aa…`; 533 regens, fila de 1.289) | 428 | aposentado (bits 0x30 = 2), sem clube, na fila |
| `TesteTelas-2026-10-27.sav` do Orquestrador, das 17:21 (SHA-256 `89067368…`; 529 regens, fila de 1.290) | 426 | idem |
| `Valencia.sav` | 24 | idem |

Os dois `TesteTelas` são gravações diferentes da mesma carreira; no mesmo arquivo, a contagem do CMRTE e a desta leitura batem (426). Em todos os saves, todas as fichas com dois donos são de um regen e de um aposentado assim [observado].

**Para o editor:** a tela de um aposentado na fila pode mostrar a ficha do regen. Editar essa ficha muda os dois.

### Conclusão

- **Existe uma relação real de habilidade, sim, mas com B, não com A.** O regen é a ficha de um jogador que se aposentou (ou virou técnico), com o mesmo PA, as mesmas posições, os mesmos pés e a mesma nação. O CA, a idade, os nomes, o físico e parte dos atributos mentais são sorteados de novo, e os demais atributos são rebalanceados a partir dos de B [observado no código e nos saves].
- **Com A a ligação é a vaga:** o id, a personalidade do registro de pessoa (com a determinação e a adaptabilidade, que pesa nos idiomas) e as preferências. Nada de habilidade, posição, nação ou clube [observado].
- **"Fulano é o regen de Beltrano" é verdade no sentido do B**, o dono da ficha: mesmo potencial, mesmo jogador "em forma de juvenil".
  - **No sentido do A** (o id, a vaga), é mito para a habilidade e só vale para a personalidade.
  - **Os dois só coincidem com a fila curta.** No `TesteCMRTE` → `TesteTelas`, com 847 vagas paradas, coincidiram 0 de 520 vezes [observado].
- **Para o editor ligar um regen ao "original":**
  - num save só, B é o aposentado na fila que aponta para a mesma ficha (enquanto ele espera);
  - entre dois saves da carreira, B é quem tinha a ficha no save anterior;
  - depois que B também vira regen, ou se o save anterior não existe, não sobra rastro [inferência pelo código: a entrada da reserva é apagada ao ser usada, e o +0x61 de B é trocado na vez dele].
