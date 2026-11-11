# Fechamento do mapa do mercado, leitor D3: 0x862720 e 0x844780 (11/11/2026)

Rotinas: `staff_contract_calc_unset_default_wage` (0x862720, o salário padrão das listas iniciais e dos regens) e
`staff_interest_in_job_at_club` (0x844780, a vontade de quem tem ficha de comissão de aceitar um cargo). A seção D do
mapa (`docs/MAPA_MERCADO.md`) só as tinha pela estrutura. Tudo abaixo vem da desmontagem do `jogo/cm0102.exe`
(`desmonta.py`), com o endereço da instrução; as constantes lidas do exe. O pseudo-C do Ghidra das duas é enganoso: na
0x862720 ele perde o ajuste da regra da FIFA (0x86315d..0x8631d9) e diz `return 0` onde a rotina devolve o valor
(0x864320); na 0x844780 os `_ftol` aparecem sem argumento e somem os pesos da ambição (0x844a9f..0x844b25), as
reputações ponderadas e a confiança da diretoria. Para cada afirmação do mapa: o texto antigo, o novo e a prova. A
sessão principal troca [inferido] por [provado].

Cargos (`enums.rs`, `job_kind`): 5 técnico, 6 auxiliar, 7 técnico dos reservas, 8 treinador, 9 olheiro, 10
fisioterapeuta, 11 jogador, 12 jogador-técnico, 13 jogador-auxiliar, 15 jogador-treinador, 16 aposentado, 17
comentarista.

## 1. 0x862720 `staff_contract_calc_unset_default_wage` — item **mkd_6** (portado)

Convenção: stdcall(pessoa, clube, conferir, estatuto, cargo, piso), `ret 0x18` (0x8644e7), float em st0 (o f32 de
[esp+0x14]); a primeira instrução sobrescreve o ecx (os dois chamadores, thiscall, passam o contrato nele e ele não é
lido). Chamadores (`refs.py`): `set_default_wage` 0x8626f6 (pessoa, `club_job`, 1, −1, 11, 0) e
`create_initial_shortlists` 0x83a2c3 (pessoa, clube, 1, o nibble alto de +0x4f do contrato com o resto do edx, 11, 0),
este para os contratos com salário −1 (0x83a2a8). Igual no cm0102-1080.exe (0x1e30 bytes).

Constantes (lidas do exe): 750 = 0x96ff30, 25000 = 0x96ff34 (f32); 500 = 0x96d1a8, 10000 = 0x96d1b0, 275 = 0x96ff20,
5000 = 0x968db0, 250 = 0x96d210, 1000 = 0x96d2dc, 200 = 0x96d834, 2500 = 0x968dac (f32); 2,5e-8 = 0x96ff38, 1e-8 =
0x968eb8, 5e-9 = 0x96ff28, 1e-9 = 0x96ff18, 0,0001 = 0x96a5c0, 1,35 = 0x969588 (f64); as bases 50000 / 55000 / 65000 =
0x96ff04 / 0x96ff08 / 0x96ff0c, 40000 / 45000 / 52500 = 0x96fef0 / 0x96fef4 / 0x96fef8, 20000 = 0x96ff00, 30000 =
0x96fefc, 8500 = 0x96d4e0, 11000 = 0x96feec, 175 = 0x96fee8, 550 = 0x96fed8, 1,375 = 0x96fedc, 0,0125 = 0x96fee0,
0,0004 = 0x96fee4, 0,2 = 0x96b028 (f32); 0,00017 = 0x96fed0, 0,00012 = 0x96ff10, 6 = 0x969000, 1,125 = 0x96a5a8, 2750
= 0x96fec8, 3250 = 0x96fec0, 1500 = 0x96feb8, 350 = 0x96f870 (f64). Os imediatos 150000 (0x48127c00), 85000
(0x47a60400), 75000 (0x47927c00), 65000 (0x477de800), 50000 (0x47435000), 40000 (0x471c4000), 75 (0x42960000), 50
(0x42480000) e 100 (0x42c80000) não estão no `.rdata`.

| Afirmação do mapa (§1, §2.3) | Texto novo | Prova |
|---|---|---|
| Chamadores: `set_default_wage` (0x8626e0), `create_initial_shortlists` (0x83a1b0) | **[provado]**, com os argumentos: as duas passam conferir = 1 e cargo = 11; a 0x83a1b0 só para contratos com salário −1, com o estatuto do contrato | 0x8626e0..0x8626f6, 0x83a2a8..0x83a2c3 |
| "O salário padrão pela ambição, lealdade, pressão, a nação grande (0x549e50), a temporada (0x547770) e o teto (0x5968d0)" [inferido] | **corrigido [provado]**: não lê a pressão nem o profissionalismo; lê a lealdade (base e mercado), a ambição (mercado e bônus), a determinação e o ritmo de trabalho (o veterano), as internacionalizações, a idade, as três reputações, a habilidade e o potencial; usa a 0x549e50, a 0x547770 (com a data de hoje) e a 0x5968d0 com o dword do estatuto | abaixo |
| (novo) O contrato e o clube atual | **[provado]**: `get_contract` (gravado em [0xae1080]); o clube do contrato, nenhum para um empréstimo (tipo 5) | 0x862720..0x86276d |
| (novo) O nível | **[provado]**: com clube, reputação / 500 + 1 em aritmética de byte; com clube atual e clube de estatuto profissional 2, um a menos, entre 1 e 4; sem clube, a reputação mundial da ficha de jogador (ou de comissão) / 500, no mínimo 1 | 0x86279c..0x862838 |
| (novo) O cargo do cálculo | **[provado]**: o cargo no clube (com clube atual), senão o da seleção, senão o do clube, senão 11 com ficha de jogador, senão o da seleção ou 9; o argumento `cargo` só pesa nos cargos 6 e 8 (6 ou 11 oferecido) e no coach de 36 anos ou mais (13, 15) | 0x86283c..0x86288e, tabela 0x8644ec |
| (novo) Comissão (5 a 10) | **[provado]**: o nível sobe à reputação do clube atual / 500 + 1 (ou da seleção, fora do cargo 11); salário = nível × (reputação atual³ × k com ficha de comissão, senão (id / total de pessoas × 4 + 1) × m × 0,0001) × 0,25; técnico: k 2,5e-8, m 25000, mínimo 750, acima de 25000 vira 150000; auxiliar e técnico dos reservas: 1e-8 / 10000, mínimo 500; treinador: 5e-9 / 5000, 275..5000; olheiro: 1e-9 / 1000, 250..1000; fisioterapeuta: 200..1000. Em outro clube: o auxiliar vale 150000 (fora o 6/11 oferecido com o nível da reputação atual / 500 + 4: abaixo de 2500, × 1,35); técnico dos reservas e treinador valem 150000 quando a reputação do clube atual / 500 + 4 passa do nível; fora disso, acima de 25000 vira 150000 (cargo oferecido fora de 6 e 11) | 0x862895..0x862f06 |
| (novo) Jogador (11, 12, 13, 15) sem ficha | **[provado]**: grava "No player data..." em 0xde1f64 (sprintf com os dwords +4, +8 e o id) e devolve `rand()` × 4 / 32768 × 550 | 0x863660..0x8636b0 |
| (novo) Jogador: o estatuto −1 | **[provado]**: com clube: 1 (reputação atual acima da do clube, 24..33 anos), 2 (acima de 0,75 × a do clube, 21+), 4 (acima de 0,5 × e 17+, ou 25+), 5 (potencial × 50 acima de 0,75 × a do clube), 6; o byte do argumento é reescrito e vai inteiro para o `get_max_wage` | 0x862f5b..0x86302f |
| (novo) Jogador: a base | **[provado]**: cargo oferecido 13/15 com 36+: 10000; estatuto 5/6: 8500 (reputação atual < 2500) ou 10000 (< 6250); atacante ≥ 15: 65000 / 55000 (reputação > 6750, lealdade < 15 / ≥ 15) ou 50000; volante, meia ou meia-atacante ≥ 15: 52500 / 45000 (> 7250, lealdade < 14) ou 40000; goleiro 20: 25000 (mundial > 7750, lealdade < 3) ou 20000; líbero < 15 e zagueiro 0: 25000; senão 30000 (mundial > 7500, lealdade < 6) ou 25000. Com a regra da FIFA ([0xae233c] = 1), base acima de 11000 e clube de nação grande: × 1,15 / 1,125 / 1,1 (reputação > 6750, lealdade 1 / < 6 / outra) ou × 1,025 (> 4250), sem arredondar a f32 | 0x86303d..0x8631d9 |
| (novo) Jogador: o escore e o salário | **[provado]**: m = a maior das reputações (atual, nacional, mundial), piso 7250 (piso 1) ou 6750 (piso 2); s = (atual + nacional + mundial + 4m + 50 × (habilidade + potencial)) / 9 + ftol(id / total × 8 − 4); estrangeiro (região +0x76 diferente da do clube e da do clube atual): no mínimo 1500, senão × 1,05; no mínimo 1; salário = base × s² × 1e-8, no mínimo 175 (comparado antes de arredondar); × 0,75 ou × 0,5 pelo desenvolvimento da nação do clube atual (ou da pessoa); × reputação atual × 0,0004 (clube < 2500) ou / (nível × 500), entre 0,85 e 1,15 | 0x8631d9..0x8633b7 |
| (novo) Jogador: os tetos | **[provado]**: nível < 10: 2500 × nível; < 15: 2750 × nível; senão 3250 × nível, ou 10% acima (brando) de 3250 × nível (Inglaterra, Espanha, Itália com PLC) ou 5000 × nível (sem PLC); posição: 20% acima de 85000 (atacante 20), 75000 (meia ou meia-atacante 20), 65000 (volante 20), 50000 (zagueiro 20) ou 40000, × 1,1 com a FIFA, nação grande e atual > 7250 | 0x8633bb..0x8635a9 |
| (novo) Jogador sem clube | **[provado]**: sem `club_job`, fora o veterano (33+, internacional, ritmo de trabalho + determinação ≤ 30, id % 15 ≠ 0, clube ≤ 4500): × (1,375 − d × 0,0125), no mínimo 0,2, d = dias desde +0x3e (sem teto), + 60 na temporada (0x547770) | 0x8635ad..0x86365b |
| (novo) Com clube: a oferta pelo estatuto | **[provado]**: com ficha de jogador, r = (reputação do clube × 0,00017 (amador) / 0,00012 (> 2250) / 0,0001)⁶ (pow 0x945ff0), no máximo 1; pelo estatuto (tabela 0x864518) o salário sobe à média com 3 × reputação × r × (1: 0,6 / 1,25 / 1,5; 2: 0,3 / 0,6 / 1; 3: 0,25 e depois 0,3 / 0,6; 4: 0,1 / 0,15 / 0,3; 5: 0,05; 6: 0,02, para < 20 anos / < 24 ou fora de nação grande / outros); com a queixa 0x10 do contrato: × 0,67 / 0,67 / 0,85 / 2,5 / 3,5 / 7,5 (tabela 0x864530) | 0x862f12..0x8639b7 |
| (novo) O mercado da nação do clube | **[provado]**, igual ao passo 7 da 0x864550 no mapa: quando a pessoa trabalha lá (`club_job` da mesma nação e é de lá ou lealdade > 3 ou ambição < 15) ou é de lá sem clube; Japão piso nível² × 10 e 25% acima de 7500; Inglaterra, Itália, Espanha nada; EUA × 0,75, MLS 15% acima de 250, nativos 20% acima de 1250 (32+, ou < 24 com mundial < 3750) ou 2750, estrangeiros 25% acima de 3250; Coreia, Escócia, Alemanha/França/Holanda, Suécia, Dinamarca/Noruega/Polônia, Bélgica, Portugal/Croácia, Brasil/Argentina × o fator do padrão da liga (reputação / 50 / padrão × 0,1 × k, no máximo 1, quando reputação / 50 > padrão × 10) e os tetos de cada uma; País de Gales, Finlândia, Irlanda, Irlanda do Norte, Austrália só tetos; as outras × 0,3 e 25% acima de 15000 (comparado antes de arredondar) | 0x8639bb..0x864291 |
| (novo) Os limites finais | **[provado]**: jogador: no máximo o salário do contrato atual de estatuto 7; ao conferir, ou sem clube atual com reputação < 2250: no máximo o `get_max_wage` (sem finanças devolve o valor como está, sem os pisos); sem conferir e com clube atual, o craque ambicioso (> 10), pouco leal (< 6), famoso (> 6750) atacante/meia/meia-atacante 20, 21+: × 1,1 (meia-atacante) / 1,05 (meia) / 1,15 se salário × 1,25 < teto; pisos: clube de estatuto 1: 75 abaixo de 20 anos, senão 100; outros: 75 num clube > 2250, senão 50; sem clube: o valor sem pisos | 0x864295..0x8644e7 |
| §2.3 "0x862720 segue inferida" | **[provado]**, e as diferenças para a 0x864550 (mapa, item mkd_2): bases menores (sem os tipos 1..3 de 75000..105000, sem a subida dos jovens, sem o corte do mundial < 1750, atacante 50000..65000 em vez de 55000..82500); tetos por nível 3250 / 5000 em vez de 5250 / 7500; tetos por posição 40000..85000 em vez de 80000..150000; a razão pela reputação **atual** (a 0x864550 usa a mundial); o desenvolvimento sem o "1 com mundial > 4250"; o sem clube sem teto de dias nem os 20% acima de 30; comissão com o 150000 | comparação com o texto da 0x864550 no mapa |

### Defeitos e curiosidades (0x862720)

1. **O 150000 é uma marca**, como o 250000 da 0x85fc50: o técnico acima de 25000 e o auxiliar / treinador de um clube
   mais forte valem 150000, não um salário **[provado]** 0x862bea, 0x862a80, 0x862bb4, 0x862d33.
2. O teste do zagueiro é `!= 0` (0x86310f): todo jogador tem nota de zagueiro, então o ramo "líbero < 15 e zagueiro 0"
   nunca vale e a base fica em 25000 / 30000 **[provado]**; o mesmo resultado na prática.
3. Sem clube e com nível 15 ou mais (mundial ≥ 7500), um jogador lê +0x53 do clube nulo (0x863403): falha de acesso
   **[provado]**; os dois chamadores passam clube (o regen tem clube) **[inferido]**.
4. Sem ficha de jogador num cargo de jogador, o sprintf usa `%s` com os dwords +4 e +8 da pessoa (0x863660)
   **[provado]**; o salário vem do `rand()` da CRT, fora do gerador do jogo.

### Pontos de extensão

- **0x862720 (mkd_6, Rust depois de integrar)**: o salário dos regens (`set_default_wage`) e das listas iniciais; as
  bases por posição, o escore s² e o mercado por nação estão numa função só. Para o Tweaks, um desvio na saída que
  escale o valor só para os regens muda o salário deles sem tocar no contrato desejado (0x85fc50).

## 2. 0x844780 `staff_interest_in_job_at_club` — item **mkd_7** (portado)

Convenção: thiscall(gerente de listas, pessoa, clube, conferir, cargo, &razões), `ret 0x14` (0x84680b), quadro SEH,
f64 em st0; o ecx só é lido para as listas dos clubes (+0xdc, 0x845809, 0x84600a). Chamadores (`refs.py`): 0x4ce92c
(`resolve_contract_offers`, conferir 0), 0x69fc3d (`is_staff_suitable_for_job` 0x69f810: cargo 5, conferir 0),
0x840682 (0x840084, as ofertas da IA: conferir 1), 0x860339 (`set_to_desired_contract` 0x85fc50) e 0x807ebc (filtro de
busca 0x807450, conferir 1). Igual no cm0102-1080.exe (0x21a0 bytes).

Constantes (lidas do exe): 0,025 = 0x967878, 1,4 = 0x968dc0, 0,02 = 0x968a98, 0,8 = 0x9689b0, 0,05 = 0x967888, 0,5 =
0x968968, 0 = 0x9689a0, 3,5 = 0x968e38, 900 = 0x96f958, 12 = 0x96d6a0, 0,33 = 0x968f78, 0,85 = 0x969520, 3 = 0x968930,
1/60 = 0x968958, 0,75 = 0x969040, 0,7 = 0x968e40, 0,1 = 0x967880, 0,0025 = 0x969028, 1,5 = 0x968940, 0,01 = 0x967898,
0,45 = 0x96a6a8, 0,65 = 0x96a5d0, 2,2 = 0x96d7d0, 10 = 0x9688b0, 1/0,85 = 0x96f950, 1/0,45 = 0x96f948, 0,4 =
0x968e00, 0,25 = 0x9689e0, 1,4 = 0x968dc0, 0,2857 = 0x96d5e0, 7000 = 0x96d470, 0,67 = 0x968f80, 0,0008 = 0x96a618,
5 = 0x9688a0, 1000 = 0x968908, 0,0001 = 0x96a5c0, 1,25 = 0x968928, 1,45 = 0x96f940, 0,6 = 0x968e48, 5000 = 0x9688e8,
1/15000 = 0x968d78, 12000 = 0x96f938, 200 = 0x96a648, 0,2 = 0x968918, 2,5 = 0x968e18, 0,000125 = 0x96a6b8, 0,0877 =
0x96f930, 1,1 = 0x9689c0, 100 = 0x9688a8, 150 = 0x968f50, 1/150 = 0x96d1d0, 0,04 = 0x968a90, 1/4500 = 0x96f928, 0,001
= 0x96a350, 1,2 = 0x968978, 0,9 = 0x9689d8, 1,75 = 0x968f98, 1,35 = 0x969588, 0,3 = 0x968e80, 0,225 = 0x96d6d8 (f64);
1 = 0x968e70 (f32). Imediatos: 2,25 (0x40020000), 1,5 (0x3ff80000), 3,5 (0x400c0000), 0,5 (0x3f000000, f32), 0,001
(0x3f50624d_d2f1a9fc), 0,9 (0x3feccccc_cccccccd). Nomes: "Dario Gradi" 0xa82620, "Pat Nevin" 0xa82614, "Kevin
Keegan" 0xa82604.

| Afirmação do mapa (§1) | Texto novo | Prova |
|---|---|---|
| "Chamadores: 0x4ce4e0, 0x85fc50, `is_staff_suitable_for_job` (0x69f810), 0x840084, filtros" | **[provado]**, com os sítios e os argumentos: 0x4ce92c, 0x860339, 0x69fc3d (cargo 5, conferir 0), 0x840682 (conferir 1), 0x807ebc (search_filters.cpp, conferir 1) | `refs.py`; 0x69fc2a..0x69fc3d, 0x84066f..0x840682, 0x807ea5..0x807ebc |
| (novo) Os filtros que dão 0 | **[provado]**: sem ficha de comissão, sem a nação do clube ou da pessoa; **Dario Gradi, Pat Nevin e Kevin Keegan com cargo em clube** (pelo nome formatado, 0x53dba0); cargo no clube ou na seleção 17; fora do cargo 5, os últimos 16 registros (id ≥ total − 16); técnico de seleção (cargo 5 na seleção); com cargo oferecido ≠ 0: técnico (5/12 no clube ou na seleção) com reputação atual > 4250 ou habilidade > 100 querendo outro cargo que não 5; fisioterapeuta (10) chamado para outro cargo, ou quem tem outro cargo chamado para fisioterapeuta; jogador (11) com clube chamado por outro clube (fora dos cargos 5, 12, 15); técnico cujo clube atual tem divisão de reputação maior ou igual à do novo, ou reputação acima da do novo − 750; clube abaixo de 2750 da nação do clube atual ou que o tem por estrangeiro (0x53d980) | 0x8447d2..0x844a84 |
| (novo) Os pesos | **[provado]**: w_estádio = 1,4 − ambição × 0,025 − profissionalismo × 0,025; w_divisão = ambição × 0,02 + 0,8; w_competição = ambição × 0,05 + 0,5; w_técnico = 0 para os cargos 5 e 12, senão 1; denominador = a soma + 1 (0,5 se ≤ 0) | 0x844a87..0x844b25 |
| (novo) O clube atual | **[provado]**: o `club_job`; sem ele, ou quando o clube oferecido é seleção (0x53b900), a seleção (fora o cargo 11 nela); clube atual sem nação: devolve 3,5 | 0x844b30..0x844b8c |
| (novo) A base | **[provado]**: 3,5 no próprio clube; senão 0 com o contrato atual de mesma língua e a queixa 0x20; 2,25 (cargos 5, 12) ou 1,5, mais, sem clube, (dias desde o mais tarde entre o início do jogo [0xdd6ec8] e a entrada no clube, até 500)² / (reputação atual + 900), até 12 | 0x844b97..0x844d22 |
| (novo) Simpatias, rivais, técnicos | **[provado]**: não gosta do clube × 0,33 (2); do atual × 2 (4); gosta do atual × 0,85 (atual amador e novo profissional) ou × 0,5 (8 com contrato sem Bosman); gosta do novo × 3 (0x10); rival ou confronto de liga (0x4b02e0): Celtic × Rangers 0,5 − lealdade × 0,025, Itália 1 − lealdade / 60, rival da mesma divisão 0,75 − lealdade × 0,025 (diferença < 1250) ou 0,8 − lealdade / 60, senão 0,85 − lealdade × 0,025 (0x20 abaixo de 0,7); sem rival, Croácia × Iugoslávia × 0,05 / 0,1 (2); técnico do novo gosta × 2 (0x40), não gosta × 0,33 (0x80), fora dos cargos 5 e 12; técnico do atual não gosta × 2 (0x100), gosta × 0,5 (0x200); com transferência futura: 0 (0x10000000) | 0x844d34..0x8450d3 |
| (novo) Moral e cargos | **[provado]**: moral do contrato atual: no próprio clube × (1 + moral × 0,0025), senão × (1 − moral × 0,0025) (0x400 acima de 1,5, 0x800 abaixo de 0,5); no próprio clube a rotina pula daqui ao fim. Pelo cargo atual (tabela 0x846810): presidente × 0,01; diretores × 0,01 (× 0,5 se oferecido técnico num clube ≥ 6000); técnico × 0,45 (0x4000000) ou × 0,65 sem clube; auxiliar × 1,5, reservas × 2,2, treinador × (ambição × 0,05 + 0,8) (0x8000000 se oferecido técnico); 16 (aposentado) × 10 / ambição; jogador e outros × 10 (0x8000000). Pelo oferecido (tabela 0x846850): 0 → 0; presidente × 10; diretores × 1/0,85; técnico × 1/0,45 (× 1,5 se já técnico); auxiliar × 0,5; reservas e treinador × 0,4; olheiro e fisioterapeuta × 0,4 (já um deles) ou 0,25, × 0,25 com ficha de jogador; jogador e outros × 0,1; técnico atual × 0,4 nos outros cargos. Técnico com a confiança da diretoria (+6 + +8 da linha de 0xb67a44 + 500 × (lealdade + determinação)) abaixo de 8000: × 1,4 | 0x845127..0x845438 |
| (novo) As reputações ponderadas | **[provado]**: para o clube novo, e = ftol(1,5 × (w_comp × competição + w_div × divisão + w_téc × técnico + w_est × estádio + listas) / den − 5000), com o estádio e a divisão da 0x846920, a competição = 600 × a reputação da continental (16 bits) ou 0,8 × divisão até 7000, o técnico = a reputação dele vista pela pessoa (0x5407e0) pesada pela confiança da diretoria (0,67 × +6 + 0,33 × +8), as listas = 0,4 × a reputação nas listas + 0,6 × a reputação pela faixa de riqueza (0,7 / 0,85 / 1,25 mín. 4250 / 1,5 mín. 5750), × 1,45 numa seleção da nação dele; clube não profissional: min(e/2, e − 5 × (internacionalizações² + 400)), estatuto 3 de novo min(e/2, e − 2000). Para o clube atual o mesmo com outras constantes: técnico pela reputação atual dele (div × 0,75 sem técnico), faixas 0,75 / 0,9 / 1,25 mín. 4750 / 1,5, seleção × 1,35, a regra da Intertoto. Sem clube atual: a média das três reputações da ficha | 0x84543c..0x845955, 0x845bd8..0x846122 |
| (novo) Idioma, estrangeiro, tempo | **[provado]**: × max(F × 0,1, (e + 5000)² / 15000 / (12000 − 200 F)) + 0,2 (F = fluência, 10 sem ficha de jogador) no clube novo (0x10000 abaixo de 0,6), ÷ o mesmo no atual; estrangeiro (sem clube, ou estrangeiro na nação do clube de nação diferente da do atual): × 0,85 / 0,75 / 0,65 / 0,5 / 0,01 por cargo quando o técnico do clube o tem por conterrâneo, senão × 0,25 / 0,3 / 0,225 / 0,2 / 0,001; sem clube × 2,5; com clube, 0 abaixo de (lealdade + 8) × 15 dias no clube, e t = min(dias² / 20 / (lealdade + 6), 400) × 0,0025 | 0x845955..0x845bd0, 0x846122..0x8461ce |
| (novo) A diferença das escalas | **[provado]**: divisão nula ou a divisão-chave 0x9cf764: × 0,1; g = 1 + (reputação do clube − a média da ficha) × 0,000125; y = (e_novo − e_atual) × (ambição × 0,0877 + 0,75) se sobe, × 1,5 se desce (cargos fora de 5/12), ou para técnico × min(max(1,1 − vontade × 0,01, 0,1), 1) com vontade > 10; y > 0 com t < 1: × ((150 − min(y × 0,02, 100)) × t + y × 0,02) / 150 (0x100000 abaixo de 0,6); oferta de técnico a quem não é, y < 0: y × max(1 − vontade × 0,04, 0,4) (treinador, auxiliar) ou max(1 − vontade × 0,1, 0,2); × max(1 + y / 4500, 0,001) (0x400000 / 0x800000 / 0x200000 pela ambição e as competições); desenvolvimento das nações: iguais, europeu indo para fora da Europa × 0,4; diferentes, × (dev_atual² × div_novo) / (max(div_atual, 500) × dev_novo²) entre 0,01 e 10, × 0,2 se de continente diferente e < 1 (0x2000000); × g (0x800000 abaixo de 0,6) | 0x8461d7..0x846628 |
| (novo) O salário | **[provado]**: ao conferir e com vontade ≥ 1,75: sem finanças, 0; q = max(3,5 / max(vontade, 0,1), 0,9); pedido = q × o salário padrão (0x864550 com contrato local, conferir 0, estatuto −1); acima do teto da comissão (0x597db0): vontade × teto / pedido e o pedido vira o teto; com contrato atual: razão = pedido / max(salário, 10); abaixo de 0,1 ou de 1, a vontade; acima de 10, × 10 / q; senão × razão / q | 0x84662e..0x8467ec |
| §2.1 "a vontade 0x846920; não-jogador 0x844780 em 0x840682" | **[provado]**: 0x840682 chama a 0x844780 com (pessoa, clube, 1, o cargo da vaga, 0) | 0x84066f..0x840682 |

### Defeitos e curiosidades (0x844780)

1. **Três nomes fixos**: Dario Gradi, Pat Nevin e Kevin Keegan nunca saem de um cargo em clube (vontade 0); a
   comparação é pelo nome formatado, então qualquer pessoa com esse nome também **[provado]** 0x8447fb..0x8448f5.
2. **Os últimos 16 registros** de pessoa (id ≥ [0xae2364] − 16) só aceitam o cargo de técnico **[provado]** 0x84491a;
   [inferido] são os registros sobrando dos regens.
3. **Cargo 0 só no próprio clube**: o cargo oferecido 0 dá 0 em outro clube (0x84532a), mas no próprio clube a rotina
   pula as tabelas de cargo e calcula a vontade **[provado]** 0x8451ca.
4. **Assimetria do técnico**: o clube novo sem técnico pesa 0 na escala; o atual sem técnico pesa 0,75 × a divisão
   **[provado]** 0x8455f4 contra 0x845ded; as faixas de riqueza, o fator de seleção (1,45 contra 1,35) e a regra da
   Intertoto (só no atual) também diferem **[provado]**.
5. **Sem finanças, 0**: ao conferir com vontade ≥ 1,75, um clube sem registro de finanças dá vontade 0, mesmo positiva
   **[provado]** 0x846671..0x846673.

### Pontos de extensão

- **0x844780 (mkd_7, Rust depois de integrar)**: o movimento da comissão (técnicos, auxiliares, olheiros) inteiro numa
  função: a base 1,5 / 2,25, as tabelas de cargo (× 0,01 a × 10), o tempo mínimo no clube ((lealdade + 8) × 15 dias), a
  confiança da diretoria (8000) e o portão do salário (1,75). Para o Tweaks: mudar quanto técnicos trocam de clube por
  grupos (reputação, idade) sem tocar na vontade dos jogadores (0x846920).
