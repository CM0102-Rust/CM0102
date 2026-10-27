# Obras do estádio (coberto, aquecimento, capacidade)

Levantamento para o editor saber o que significam os bytes de obra do estádio e se é seguro gravá-los. Ponto de partida: no Flamengo (Maracanã) o editor leu o aquecimento do gramado (estádio +0x4d) = 1, fora do 0/255 do banco, depois de a diretoria ter anunciado uma obra. Só leitura: desmontagem e pseudocódigo (Ghidra) do `cm0102.exe` 3.9.68, o mapa da comunidade (`referencias/cm0102-function-map`, `fin_prm.cpp`, `manager_manager.cpp`, `stadium.cpp`, `weather.cpp`), a varredura de deslocamentos +0x4c/+0x4d no `.text` inteiro, o `jogo/Data/stadium.dat` e o `stadium.dat` de uma **cópia** do `jogo/FeriasTesteSave.sav` (MD5 33fda465193b5e8e6d7ace93ecd44559, intocado), lido com as funções do `engenharia-reversa/prova_save.py`. O jogo não foi aberto.

Cada afirmação leva **[observado]** (lido na desmontagem, no código ou no arquivo) ou **[inferência]** (dedução sem prova direta). Endereços em VA.

## Resumo

- **Não existe estado "em obra" no estádio.** +0x4c (coberto) e +0x4d (aquecimento) são booleanos: 0 = não tem, **qualquer outro valor = tem**. O banco usa 255; o jogo, quando instala, grava **1**. O 1 do Maracanã quer dizer "aquecimento instalado pelo jogo", não "obra em andamento" [observado].
- O aquecimento é instalado **de uma vez** no fim da temporada da liga do clube (0x59cfa0), com cobrança de 1,5 milhão e a notícia "Undersoil heating installed", cujo corpo diz "The board have announced plans to install undersoil heating..." — por isso parece um anúncio de obra, mas o byte já está gravado quando a notícia sai [observado].
- O único "pendente" que existe é o **pedido de expansão aprovado pela diretoria** (reunião com a diretoria), guardado num bit do registro do clube no `MANAGER_MANAGER`, não no estádio; ele é consumido no mesmo fim de temporada [observado]. Não há contagem de dias em lugar nenhum [observado nas rotinas lidas].
- Para o editor: mostrar os dois bytes como sim/não (`!= 0`), aceitar 1 como "sim" e gravar 255 para "sim" e 0 para "não". Gravar a qualquer momento é coerente: não há obra em curso que sobrescreva depois (ver §4).

## 1. Os campos e os valores

Registro `TStadiums` (78 bytes, `stadium.dat`; tabela `[0xae23b0]`, `[0xae2358]` registros); o clube aponta o estádio em `clube+0x69` (ponteiro) e diz se é dono em `clube+0x6d` [observado].

| Desl. | Tipo | Campo | Valores | Fonte |
|---|---|---|---|---|
| +0x3c | `i32` | capacidade total | lugares; muda nas obras (§2) | [observado] |
| +0x40 | `i32` | lugares sentados | ≤ capacidade; muda nas obras de lugares | [observado] |
| +0x44 | `i32` | **teto de expansão** (capacidade máxima permitida) | a expansão pára em `+0x44 − +0x3c`; 0 ou menor que a capacidade = não dá para expandir | [observado] 0x59cfa0, 0x599e00 |
| +0x4c | `u8` | coberto | **0 = não; ≠0 = sim** (banco: 255; jogo grava 1) | [observado] |
| +0x4d | `u8` | aquecimento do gramado | **0 = não; ≠0 = sim** (banco: 255; jogo grava 1) | [observado] |

Contagens [observado]: no `jogo/Data/stadium.dat` (8917 registros) o coberto é 0 em 8872 e 255 em 45; o aquecimento é 0 em 8720 e 255 em 197; nenhum outro valor. No `stadium.dat` do save de teste também só há 0 e 255 (nenhuma obra ocorrida). O Maracanã no banco: 70096 / 70096 / teto 0 / 0 / 0; no save de teste o teto já é 825 (a origem desse 825 não foi procurada).

Quem **escreve** +0x4c/+0x4d no estádio, pela varredura do `.text` (instruções com deslocamento 0x4c/0x4d, separando as estruturas que não são o estádio) [observado]:

| Rotina | Escreve | Quando |
|---|---|---|
| 0x59cfa0 `FinanceTable::season_end_club_update` | +0x4d = 1; +0x4c = 1 | fim da temporada (§2) |
| 0x59ed70 `FinanceTable::process_club_finance_events` | +0x4d = 0 | crise financeira prolongada (§2) |

Fora isso, só a carga do banco e do save (cópia em bloco). Todos os leitores conferidos testam **zero/não zero** (`test cl, cl` ou `!= 0`), nenhum compara com 1 ou 255 [observado]: a tela do clube 0x460590 (textos "<%ld - capacity> all seater with under-soil heating", "...with under-soil heating" etc.), o tempo 0x923630 e 0x923970, as finanças 0x59b9c0, 0x59cfa0 e 0x59ed70. Os demais acessos +0x4c/+0x4d da varredura (motor da partida, telas da partida) são de outras estruturas ou não foram conferidos [inferência para os não conferidos].

## 2. As rotinas das obras

### Fim da temporada: 0x59cfa0 `FinanceTable::season_end_club_update`

Chamada por 0x59cdd0 `FinanceTable::end_season_for_nation` (por clube da liga, no fim da temporada da nação) [observado]. Valores: saldo = `i64` em `finança+0` (registro de 0x167 bytes por clube); toda obra soma o custo em dois acumuladores de despesa (`finança+0x8c` e `+0x12c`) e o tira do saldo [observado; o nome de cada acumulador é inferência]. Reputação = `clube+0x80` (`i16`); instalações de treino = `clube+0x7f`.

Ordem dentro da rotina [observado]:

1. Compra do estádio (quem não é dono e tem saldo > 35 milhões): `clube+0x6d = 1`, −20 milhões, notícia tipo 4 "Stadium purchase announced" (ou tipo 5 para o vendedor). Não toca +0x4c/+0x4d.
2. Treino pela reputação (sobe `clube+0x7f`, cobra).
3. **Pedido de expansão aprovado**: se o bit 0x40 do byte `[[0xb67a44]] + clube * 0x49 + 2` estiver ligado e o clube for de humano (0x600a50), marca "expandir à força" e **apaga o bit** (`& 0xbf`).
4. **Aquecimento**: se saldo > 17,5 milhões, reputação > 7500, capacidade > 25000 e `+0x4d == 0` → `+0x4d = 1`, −1,5 milhão, notícia tipo 6 (`0x59ffb0(clube, id do estádio, 6)`) e **sai da parte de obras** (pula a expansão).
5. **Cobertura**: senão, se saldo > 75 milhões, reputação > 9250, capacidade > 57500, treino = 20 e `+0x4c == 0` → `+0x4c = 1`, −7,5 milhões, **sem notícia**, e sai da parte de obras.
6. **Expansão**: exige saldo ≥ 5,25 milhões (o clube de humano também passa com saldo + reputação × 1000 ≥ isso; sem dinheiro e com o pedido marcado, sai a notícia de obra cancelada) e capacidade ≤ 125000. Com o pedido marcado, público médio alto ou capacidade < 2 × reputação (reputação > 4250) → ampliação; senão, troca de em pé por sentados com sorte:
   - estádio só de sentados (`clube+0x64 == 1`): até 16000 lugares em múltiplos de 500, limitado a `+0x44 − +0x3c` e a `max(+0x44/3, 5000)`; se ≥ 1000: `+0x3c += n`, `+0x40 += n`, custo `((n/1000 + 1) * 6000 + n) * 125`, notícia tipo 1 ("...plans to enlarge the stadium by N seats");
   - com arquibancada em pé: idem até 21000, `+0x3c += n` só, custo `(n + (n/1000 + 1) * 1000) * 50`, notícia tipo 2 ("...increase the standing area ... another N supporters");
   - troca de em pé por sentados: `n ≤ +0x3c − +0x40`, ≥ 500 → `+0x3c += n`, `+0x40 += n` como decompilado, custo `(n/1000 + 1) * 50000 + n * 75`, notícia tipo 3 ("...install N seats within the stadium");
   - se não deu para expandir e a capacidade < 30000: `+0x44 += 1500` (o teto sobe para a próxima vez); com o pedido marcado sai a notícia de obra cancelada (0x59fae0: "Stadium Expansion plans shelved"/"cancelled"/"Local council blocks planning permission").
7. Treino pedido (bit 0x80 do mesmo byte do passo 3): faz (0x59b630, notícia tipo 8) ou cancela, e apaga o bit.

Consequência [observado]: se no mesmo fim de temporada o aquecimento ou a cobertura forem instalados, a expansão aprovada pela diretoria **não acontece** e o bit já foi apagado — o pedido se perde sem notícia de cancelamento.

### A notícia: 0x59ffb0 e 0x5a0050

`0x59ffb0(clube, valor, tipo)` cria a notícia 0x177d; o texto (0x5a0050) escolhe pelo tipo: 1/2/3 expansão (título "Stadium expansion plans announced"), 4 compra, 5 venda, **6 "Undersoil heating installed"** (corpo "The board have announced plans to install undersoil heating in the {estádio}..."), **7 "Stadium Facilities reduced"** (corpo "The pitch's undersoil heating has failed..."), 8 treino [observado]. A redação "announced plans" é só texto: todas essas notícias saem **depois** de gravar o estádio e o saldo [observado].

### Crise: 0x59ed70 `FinanceTable::process_club_finance_events`

O contador de aperto financeiro `finança+0x166` sobe com o clube endividado; quando chega a 51 ou mais (e o clube está numa liga ativa) [observado]:

- com aquecimento (`+0x4d ≠ 0`) e reputação ≤ 4249: notícia tipo 7 e **`+0x4d = 0`**, treino cai;
- senão (capacidade ≥ 2751): fecha parte da arquibancada: `+0x3c −= x`, `+0x40 −= x` se couber, **`+0x44 += x`**, notícia 0x59f900 ("stand closed"), contador volta a 20.

A cobertura nunca é retirada pelo jogo [observado na varredura].

### O pedido à diretoria: 0x6a8810 `MANAGER_MANAGER::handle_board_request`

Opção 1 "Expand the Stadium" [observado]: recusa se o pedido já estiver aprovado (bit 0x40, resposta "already indicated that they are considering the expansion"), se o clube não for dono (`+0x6d == 0`), se a capacidade passar de 75000 ou se faltar dinheiro (2,5 / 5 / 7,5 milhões conforme a capacidade). Aprovando (confiança da diretoria e sorte), responde "plans afoot to expand the stadium" e **liga o bit 0x40** em `[[0xb67a44]] + clube * 0x49 + 2`. Nada no estádio muda nessa hora; a obra é o passo 3 → 6 do fim da temporada. O pedido nunca toca +0x4c/+0x4d.

Para o Maracanã (70096 lugares, teto 825 no save de teste): mesmo com o pedido aprovado, `+0x44 − +0x3c < 0` dá 0 lugares, a capacidade passa de 30000 (o teto não sobe) e sairia a notícia de obra cancelada [inferência a partir do código e dos valores do save de teste; o save do Lucas não foi lido].

### Quem lê (efeitos)

| Leitor | Uso | Fonte |
|---|---|---|
| 0x460590 tela do clube | texto da capacidade "with under-soil heating", "all seater", coberto | [observado] |
| 0x59b9c0 `FinanceTable::daily_update` | custo de manutenção: 12500 se coberto; +7500 (reputação < 4501) ou +12500 se aquecido | [observado]; a periodicidade desse ramo não foi conferida |
| 0x923630 `Weather::check_postponement` | coberto → nunca adia por tempo; aquecido → escapa dos adiamentos por gelo/neve | [observado] |
| 0x923970 `Weather::get_pitch_condition` | coberto → sem marca de gramado; aquecido melhora o estado do gramado | [observado] |
| 0x59cfa0, 0x59ed70 | condições de instalar/retirar (acima) | [observado] |

## 3. Se o editor gravar 0 ou 255

Não há obra em andamento guardada no estádio, então não há o que concluir, cancelar ou sobrescrever [observado: as únicas escritas são instantâneas, §1].

| Gravação | Efeito |
|---|---|
| 1 → 255 (ou 255 → 1) | nenhum: todo leitor só testa `≠ 0` [observado] |
| 1 → 0 (tirar o aquecimento instalado) | coerente; no próximo fim de temporada, se o clube ainda cumprir as condições, o jogo reinstala, cobra de novo 1,5 milhão e dá a notícia [observado] |
| 0 → 255 (dar aquecimento) | coerente; o jogo não cobra nem dá notícia, e não instala mais (já ≠ 0); entra a manutenção e o efeito no tempo; numa crise pode ser retirado (§2) [observado] |
| coberto 0 ↔ 255 | idem, sem retirada pelo jogo [observado] |
| editar capacidade/sentados/teto com expansão aprovada pendente | coerente: o fim da temporada calcula a partir dos valores do momento (pode resultar em 0 lugares e cancelar) [observado] |

Detalhe: o estádio é do estádio, não do clube; dois clubes que dividem o mesmo estádio (mesmo `clube+0x69`) veem a mesma mudança [observado no acesso por ponteiro].

## 4. Recomendação para o editor

1. Ler +0x4c e +0x4d como booleano (`≠ 0`). **Não marcar 1 como inválido**: é o valor que o próprio jogo grava [observado].
2. Ao gravar, usar 255 para "sim" e 0 para "não" (convenção do banco); se o usuário não mudou o campo, manter o byte como está (não "normalizar" 1 para 255 sem necessidade; é indiferente para o jogo, mas evita diferença boba no save) [inferência sobre a conveniência; a equivalência é observada].
3. Não é preciso travar os campos "durante a obra": não existe obra em curso no estádio. Dica de interface: com valor 1, mostrar "sim (instalado pelo jogo)".
4. Se o editor quiser mostrar "obra anunciada", o dado é o bit 0x40 (expansão aprovada) e o bit 0x80 (treino aprovado) do byte `[[0xb67a44]] + clube * 0x49 + 2`, válidos até o fim da temporada da liga do clube [observado no código]. Em que item do save esse registro fica não foi conferido [inferência: no item do `MANAGER_MANAGER`]. Mudar a capacidade enquanto o bit está ligado é seguro (§3).
5. Gravar a capacidade acima do teto (+0x44) não quebra nada lido aqui, mas impede expansões (o teto vira o limite); ao subir a capacidade, subir o teto junto é o que o próprio jogo faz em 0x599e00 [observado].
