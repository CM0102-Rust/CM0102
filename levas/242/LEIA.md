# 242ª leva — as seleções: o fim das convocações de uma competição e a poda dos registros de convocação; as seleções: pôr e tirar do elenco e dispensar os acima da idade do sub-21; as seleções: a competição atual de cada nação e a convocação do substituto; as seleções: as reservas de convocáveis de cada nação e as duas ordens delas; os amistosos: o adversário do amistoso e o da excursão; a tabela da liga: somar o resultado e os pontos do resultado; a tabela da liga: a ordem pelo confronto guardado, a ordem das datas, a linha do clube e se a liga tem o clube

**Rotinas:**
- `encerrar_convocacoes_da_competicao()` = 0x774f30 (national_teams.cpp, thiscall, ret 4): na lista de registros de convocação, os da competição (+4) terminam amanhã (+0x10, o dia da 0x54c5b0);
- `encerrar_convocacoes_da_nacao()` = 0x774f90 (national_teams.cpp, thiscall, ret 8): como a 0x774f30, só os registros da competição (+4) e da nação (+0);
- `podar_registros_de_convocacao()` = 0x7774b0 (national_teams.cpp, thiscall): em cada registro de clube (+0x20, 6 bytes), tira as entradas vencidas (0x456770) do começo, desce as outras (0x777590) e solta o vetor que ficou vazio;
- `por_no_elenco_da_selecao()` = 0x776250 (national_teams.cpp, thiscall, ret 0xc): a primeira das 50 vagas livres do time (+0xd7; tipo 0/1 a principal, senão o sub-21) recebe a pessoa, que ganha o time (+0x24), a data de hoje (+0x29), a marca 0xb (+0x28) e o elenco 4 ou 8 (0x540980);
- `tirar_do_elenco_da_selecao()` = 0x7762f0 (national_teams.cpp, thiscall, ret 0xc): tira a pessoa da tática do time (0x896c60) e, achando-a numa vaga, esvazia a vaga e limpa +0x24, +0x28 e os bits de seleção de +0x5e;
- `dispensar_acima_da_idade_do_sub21()` = 0x76d880 (national_teams.cpp, thiscall, ret 4): passada a data do registro da nação (+0x62), tira do sub-21 quem tem mais de 21 anos (+0x18) e zera o +0x6b do registro;
- `limpar_competicao_atual()` = 0x775000 (national_teams.cpp, thiscall, ret 4): a competição atual da nação (registro +4) vira −1 e, havendo a seguinte ([this+8][id]), passa a ser ela, com o aviso aos técnicos (0x6a3770);
- `liberar_competicoes_do_continente()` = 0x775070 (national_teams.cpp, thiscall, ret 8): nas nações do continente (+0x71), a atual e a seguinte que acabaram (+0x59 da competição nulo, ou a fase [0x9cfa10]) viram −1; atual vazia e seguinte: a seguinte sobe, com o aviso (0x6a3770);
- `convocar_substituto()` = 0x775f90 (national_teams.cpp, thiscall, ret 0x10): na reserva do nível, o primeiro apto da nação que joga numa posição de quem sai (0x5b7d20) é convocado (0x776250, saindo antes de outro elenco pela 0x7762f0) com a notícia (0x76fd10); ninguém: zera a vaga do tipo (+0x45);
- `montar_reservas_das_selecoes()` = 0x76abf0 (national_teams.cpp, thiscall): zera os 4 níveis de cada registro de nação e põe cada pessoa apta na reserva da nação (e do sub-21, até 21 anos e menos de 5 jogos), também pela segunda nacionalidade; trecho cheio: entra pela nota, e ordena (qsort com 0x778ed0);
- `ordem_das_reservas()` = 0x778ed0 (national_teams.cpp, cdecl): a ordem do qsort das reservas: nota (+9 da ficha) decrescente;
- `ordem_pelo_clube()` = 0x778ef0 (national_teams.cpp, cdecl): a ordem pelo clube (+0x39): sem clube depois, clubes pelo id, no mesmo clube o elenco 4 antes do 8 (+0x5e);
- `adversario_do_amistoso()` = 0x5c4cb0 (friendly.cpp, thiscall, ret 0x14): busca (bsearch) os candidatos da faixa (1 a 20, subindo até achar) e, dali, o primeiro que não é o clube, nem excluído, nem já adversário, nem fora pela data, nem de faixa cruzada, rival, de nação rival (meio a meio), da mesma cidade ou acima da faixa sorteada, que passa pela nação conforme a reputação do clube e é sorteado (1 em 5); senão 0;
- `adversario_da_excursao()` = 0x5c5040 (friendly.cpp, thiscall, ret 0x14): busca (bsearch) os candidatos da nação da excursão e, do primeiro dela, o que não passa da faixa máxima nem é fora pela data, o próprio clube, já adversário, rival, de nação rival (meio a meio), da mesma cidade ou acima da faixa sorteada, sorteado meio a meio; para no primeiro de outra nação; senão 0;
- `aplicar_resultado_na_tabela()` = 0x685f60 (league.cpp, thiscall, ret 0x18): acha as linhas do mandante e do visitante e soma o jogo em casa e fora (jogos, gols, vitória/empate/derrota, pela prorrogação e pelos pênaltis, marcando +0x3d/+0x3e), os pontos de vitória pela virtual +0x90 e os de empate +0xc3; refaz os totais, devolve os pontos das duas linhas e soma o confronto direto (0x6869a0);
- `pontos_do_resultado()` = 0x68aad0 (league.cpp, thiscall, ret 8): os pontos de vitória (+0xc2) a quem venceu, 0 a quem perdeu; empatado, pela prorrogação (+0xab bit 2), pelos pênaltis (bits 0x21: os de empate +0xc3 a quem venceu) ou os de empate;
- `ordem_pela_posicao_do_confronto()` = 0x683410 (league.cpp, cdecl): callback do qsort: a comparação própria, os pontos, o id dos clubes sem jogos, a posição do confronto guardada (+4) e o nome;
- `ordem_das_datas_da_liga()` = 0x685cf0 (league.cpp, cdecl): callback do bsearch: o ano (+2, sem sinal) e o dia (+0): −1, 0 ou 1;
- `linha_do_clube()` = 0x68ab70 (league.cpp, thiscall, ret 8): a linha do clube nas linhas dadas (nulas: a tabela da liga), ou 0;
- `liga_tem_o_clube()` = 0x68abd0 (league.cpp, thiscall, ret 4): 1 se o clube tem linha na tabela da liga;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (20):** 16 acusadas. `cpd_atual_aviso` escapou: a nação com competição seguinte não aparece até 2002-06-01 (o ramo "seleção a competição seguinte" não está na base); escapou também no jogo comparando e na carga pt (0 divergências). `cpd_continente_aviso` escapou: a seguinte que vira a atual no continente não aparece até 2002-06-01 (o ramo "seleção continente a seguinte vira a atual" não está na base); escapou também no jogo comparando e na carga pt (0 divergências). `cpe_datas_ano` escapou: ordem_das_datas_da_liga 0x685cf0: nas buscas da carreira o ano sempre empata, só o dia decide; escapou também no jogo comparando e na carga pt (0 divergências). `cpe_linha_do_clube` escapou: linha_do_clube 0x68ab70 (31 chamadas diretas): a linha achada não muda nada que as impressões e o comparador vejam até 2002-06-01; escapou também no jogo comparando e na carga pt (0 divergências). (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 241ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 242`

**Campos tocados:**
- registro de convocação: +0 nação, +4 competição, +0x10 dia do fim (curto); lê e grava (encerrar_convocacoes_*)
- gerenciador das seleções +0x20: registros de clube (6 bytes: +0 vetor de entradas de 9 bytes, data e marca; +4 marca; +5 quantas); lê e grava (podar_registros_de_convocacao)
- time (TClub) +0xd7: as 50 vagas da seleção; lê e grava (as três)
- pessoa (0x6e bytes) +0x18 idade, +0x24 time da seleção, +0x28 marca de convocado, +0x29 data, +0x5e bits de elenco; lê e grava (as três)
- registro da nação no gerenciador das seleções ([this+4] + 0x6e × id) +0x62 data, +0x6b; lê e grava (dispensar_acima_da_idade_do_sub21)
- registro da nação no gerenciador das seleções +4 competição atual, +0x3a datas e +0x45 vagas por tipo (20 bytes); [this+8] competição seguinte por nação; lê e grava
- TNation (0x122 bytes) +0x71 continente; competição ([0xae23d0], 107 bytes) +0x59; lê (liberar_competicoes_do_continente)
- registro da nação ([this+4] + 0x6e × id): +8/+0x1d começo das reservas, +0xd/+0x22 quantos, +0xe/+0x23 vagas, +0xf/+0x24 menor nota (4 níveis de 4 bytes); [this+0xc]/[this+0x10] reservas; lê e grava (montar_reservas_das_selecoes)
- pessoa (0x6e bytes) +0x18, +0x1a/+0x1e nacionalidades, +0x22 jogos, +0x24, +0x28, +0x39 clube, +0x3d, +0x5e, +0x61 ficha (+9 nota); lê
- candidato do amistoso (0x16 bytes): +0 id do clube, +4 faixa sorteada, +5 faixa, +6 nação, +0xa lista de adversários, +0xe/+0x10 faixas (curtos); lê e grava o cursor (+0xa) da lista (adversario_do_amistoso, adversario_da_excursao)
- clube (0x245 bytes): +0x53 nação, +0x57 cidade, +0x80 reputação, +0xb3/+0xb7/+0xbb rivais, +0x1db; nação +0x71, +0x75, +0x110/+0x114/+0x118 rivais; Friendly +0x34e8 ano; lê (as duas)
- linha da tabela (0x3b bytes): +0 clube, +5 confrontos, +0x11..+0x1a totais (jogos, V, E, D, gols pró, contra, pontos), +0x1f..+0x28 em casa, +0x2d..+0x36 fora; lê e grava (aplicar_resultado_na_tabela)
- partida: +0x1c/+0x20 clubes, +0x38 regras (2 prorrogação, 0x20/1 pênaltis), +0x43/+0x44 placar, +0x47/+0x48 prorrogação (0xff = sem), +0x49/+0x4a pênaltis; grava +0x3d/+0x3e (aplicar_resultado_na_tabela); League +0xab, +0xc2, +0xc3; lê (as duas)
- linha da tabela (0x3b bytes): +0 clube, +4 posição do confronto, +0x11 jogos e +0x19 pontos por coluna; data da liga: +0 dia, +2 ano; lê (ordem_pela_posicao_do_confronto, ordem_das_datas_da_liga, linha_do_clube, liga_tem_o_clube)

**Interessa a:** SGE, CMRTE
