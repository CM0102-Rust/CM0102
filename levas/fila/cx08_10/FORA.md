# cx08: o que ficou para a próxima rodada

Leitor cx08, rodada 5, 24/11/2026. Estas rotinas da lista `levas/reorganizacao/proximas/cx08.txt` não viraram item nesta rodada (o tempo da rodada acabou nos calendários e nas fases); nenhuma está portada. As notas abaixo vêm da desmontagem e poupam a releitura.

## Segunda Divisão argentina (arg_second.cpp), com a Argentina (`LIGAS = 'ENG KOR ARG ITA FRA ESP AUS'`, `ATE = '2002-08-01'`)

- **0x4093f0 `ArgSecond::init_relegation_averages`** (2096 bytes): o mesmo molde do `arg_prm_init_relegation_averages` (cx08_5), com mais linhas fixas e a divisão da Segunda; o `reset_relegation_averages` do cx08_5 serve à volta vazia. Extrair as linhas com `calls.py` (as gravações `STORE`).
- **0x409c20 `ArgSecond::update_relegation_averages`** (960 bytes): como o da Primeira, mas a terceira coluna recebe os pontos e jogos da tabela da própria liga e depois soma (`add`) os de cada fase 0..`+0x30`−1 (as zonas); depois rola, calcula e ordena como o cx08_5 (`average_seasons`, `drop_departed_clubs`, a conferir).
- **0x408630 `ArgSecond::create_promotion_cup_stage`** (736 bytes): rebaixa os dois piores promédios e monta a copa de acesso de 8 clubes (fase 3) com os vencedores das zonas e os melhores.
- **0x408910 `ArgSecond::create_promotion_playoffs`** (960 bytes): os dois confrontos da repescagem (fases 4 e 5) entre clubes da Segunda e os das linhas 16 e 17 dos promédios da Primeira.
- **0x408cd0 `ArgSecond::update_staff_records_for_status`** (480 bytes) e **0x408ec0 `ArgSecond::build_fate_news_text`** (1328 bytes): os textos de "Second Promotion", "Third Promotion", acesso e rebaixamento por promédio (o `relegated_by_average_text` do cx08_6 serve ao último) e os registros dos técnicos.

## Segunda Divisão belga (bel_second.cpp), com a Bélgica (`LIGAS = 'ENG BEL HOL GER DEN'`, `ATE = '2002-08-01'`)

- **0x420790 `BelSecond::update_staff_records_for_status`** (560 bytes) e **0x4209c0 `BelSecond::build_fate_news_text`** (736 bytes): os registros e os textos do grupo e da copa da repescagem; a temporada regular vai para a base Comp.

## Fora de vez (não são itens)

- **0x404480 `shared_return_true`** (`mov eax, 1; ret`): corpo compartilhado por slots 0x88/0x98/0x9c das vtables de várias classes (ArgPrm, ArgSecond, EuroChampQual e outras); desviá-lo não acrescenta nada.
- **0x4110b0 `AusNsl::get_table_layout_type`** (`mov al, 3; ret`, 3 bytes): prólogo de menos de 5 bytes.
