# 264ª leva — a renda das partidas: a renda da partida nas finanças dos dois clubes, a renda do estádio e a multa da liga; os eventos raros das finanças: o impulso das finanças, a troca do presidente e os outros custos; as datas internacionais dos amistosos: a tabela das datas, as datas no ciclo da Copa e o torneio do clube humano; os amistosos da pré-temporada da IA: os amistosos dos clubes, o torneio acrescentado e o convite aberto do clube humano; a lesão nova (a vaga, a duração pelo fisioterapeuta, pela idade e pela propensão, a volta ao treino, a recaída e a aposentadoria), a notícia da lesão e a marca de ausente sem licença; a parte dos clubes anteriores na transferência (que a original credita ao comprador), o percentual da próxima venda pago ao clube da cláusula e a notícia dele; a permissão de trabalho concedida (o registro de permissão, o potencial, o salário, o jovem, os jogos pela seleção e o histórico de 16 jogos), o estatuto pelo motivo da recusa e a vaga da negociação; a reação do elenco à história de um jogador, o aviso aos rivais de posição e o rótulo das respostas à mídia; as suspensões internacionais cumpridas no jogo da seleção, o recurso da suspensão e o texto da vaga da suspensão; a lista encadeada do jogo: tirar o item pelo conteúdo, esvaziar e o item do cursor; os sorteios da copa: pela semente e separando as nações, e o começo da temporada da copa base; as nações-chave das competições: a do clube, a do estádio e a entrada da tabela pelo índice ativo; o calendário: o conflito de dois jogos no mesmo dia e o primeiro e o último ano com jogos de um clube

**Rotinas:**
- `renda_da_partida()` = 0x59a5d0 (fin_prm.cpp, thiscall, ret 8): a TV pelo público (1,75 × público), a renda do estádio (+0x38 do resultado), os custos do jogo e os gastos do mandante (com sorteios), o dinheiro da TV da competição pela fase ou pelo tipo, divididos pelo tipo da nação; na liga do mandante, a TV 3/4 e 1/4, o líquido ao mandante e a manutenção do estádio; fora dela, metade de cada um para os dois;
- `renda_do_estadio()` = 0x59af40 (fin_prm.cpp, thiscall, ret 8): a renda do estádio: o preço pela reputação do mandante (/ 500; × 0,5 em amistoso, + 3 ou + 4 nas competições) × público, menos o desconto das finanças na liga do mandante; em campo neutro pelo +0x69 da competição; ajustada pelo tipo da nação, pela liga, por clubes e pela nação de 0x9cf314;
- `pagar_multa_da_liga()` = 0x599850 (fin_prm.cpp, thiscall, ret 4): soma a multa às multas do ano (+0x128) e do mês (+0x88) e a tira do saldo;
- `impulso_das_financas()` = 0x59e050 (fin_prm.cpp, thiscall, ret 8): fora de sete nações e das marcas +0x64 2 e 3, o divisor pela faixa de riqueza e pela reputação; com a ficha do presidente, três sorteios (contra o +0xf, a dívida e 750 × +0x20) dão (3 × (+0x20)² + 750) × reputação + 1 milhão mais o saldo; sem ela, dois sorteios dão (reputação + 1000) × 1000; um terço em três nações; o dinheiro, a notícia pelo tipo e o fim da crise;
- `troca_do_presidente()` = 0x59e940 (fin_prm.cpp, thiscall, ret 8): o divisor pela faixa e o fator pela reputação e pela liga; sorteios pela ficha do presidente e por 15000 / divisor; (reputação + 1000) × 1000 menos o saldo (ou só o que zera o saldo); com a ficha, o presidente novo (nação, nomes e ficha sorteados); a notícia (tipo 2) e o fim da crise;
- `pagar_outros_custos()` = 0x5999e0 (fin_prm.cpp, thiscall, ret 4): soma o valor aos outros custos do ano (+0x120) e do mês (+0x80) e o tira do saldo;
- `tabela_das_datas_internacionais()` = 0x5ca4f0 (friendly.cpp, thiscall): aloca a tabela (+0x34ec) e grava as 110 datas internacionais de 20 bytes (dia da semana, dia, mês, ano a somar ao da Copa, continente e competição de eliminatória), reduz a tabela a 110, resolve as datas (0x5cc510) e marca o último dia feito (+0x34f4 = 400);
- `resolver_datas_internacionais()` = 0x5cc510 (friendly.cpp, thiscall): dá a cada data a data do ciclo da Copa (o primeiro ano com (ano − 2) múltiplo de 4, mais o deslocamento, menos 4 se passar de 3 anos à frente) pela 0x54a130 e ordena a tabela pela data;
- `torneio_do_clube_humano()` = 0x5c6830 (friendly.cpp, thiscall): o torneio em uso (de 127, 0x69 bytes a partir de +0xb1) que o clube do técnico humano da vez organiza, ou 0;
- `amistosos_dos_clubes()` = 0x5c4770 (friendly.cpp, thiscall, ret 0xc): refaz o plano de amistosos (+0x34d8); para cada liga ativa na pré-temporada, os clubes da IA da nação (pela busca nos candidatos ordenados) ganham uma faixa pela reputação e um sorteio, e até sorteio(3) + 4 amistosos com adversários achados (0x5c4cb0) em datas livres (0x5c4380), a faixa caindo 20% a cada um; no fim, o plano em ordem de data;
- `acrescentar_torneio()` = 0x5c6e00 (friendly.cpp, thiscall, ret 0x28): preenche o primeiro dos 127 torneios livres com o anfitrião, o tipo, a data de início, os bytes e palavras das regras, o nome e os 4 convidados (marcados), ou 0 sem vaga;
- `convite_aberto_do_clube_humano()` = 0x5c68b0 (friendly.cpp, thiscall): o amistoso da lista (+0x34c8) que o clube do técnico humano da vez pediu e que ainda espera adversário, ou 0;
- `lesao_nova()` = 0x62d250 (injury.cpp, thiscall, ret 0x20): registra a lesão: troca a anterior só nos tipos 0x4d/0x4e/0x4f/0x51, acha ou cria a vaga, grava a partida, a duração (base + variáveis × (35 − melhor fisioterapeuta) / 20 com sorteios e os extras da idade e da propensão), a volta ao treino (percentual do tipo, até 60 dias antes do fim), a data, a notícia, a recorrência, a seleção, a recaída, a aposentadoria e o surto;
- `noticia_da_lesao()` = 0x62dca0 (injury.cpp, thiscall (this sem uso), ret 0xc): a notícia 2000 da lesão: a pessoa, o tipo, a data, os dias, o nascimento e a partida; de 100 dias, a todos com potencial acima de 8000 ou aos clubes interessados; até 100, ao clube humano, ao clube do contrato e à seleção;
- `ausente_sem_licenca()` = 0x418f60 (awol.cpp, thiscall, ret 8): o bit 0 da marca da pessoa; sem só_ler e sem contrato ou clube, a ausência acaba antes (0x4196a0);
- `parte_dos_clubes_anteriores()` = 0x8e4f20 (transfer_manager.cpp, thiscall, ret 4): jogador com mais de 22 anos: (valor da oferta / 20) / soma dos pesos × peso de cada um dos até cinco clubes anteriores entra nas vendas das finanças do comprador; a notícia 0xfc3 vai ao clube anterior quando a parte passa de 11 × reputação − 10000;
- `percentual_da_proxima_venda_pago()` = 0x8cb0f0 (transfer_manager.cpp, thiscall, ret 8): acha pela busca binária a cláusula da pessoa com o clube do contrato; a parte (percentual × valor da oferta com os da troca, ou o valor da pessoa) de 1000 para cima: notícia, totais de transferência, entra no clube da cláusula e sai do clube do contrato;
- `noticia_do_percentual_da_venda()` = 0x8db3a0 (transfer_manager.cpp, thiscall, ret 0x10): a notícia 0xfa7: a pessoa, o percentual, os dois clubes da cláusula, a parte e o comprador (ou o clube do contrato), aos técnicos do clube da cláusula e do vendedor;
- `permissao_concedida()` = 0x8c7950 (transfer_manager.cpp, thiscall, ret 0x14): decide a permissão: pelo registro de permissão (jogos e recusas), não jogador pela reputação, as exceções de 2001, potencial acima de 8500, salário acima de 34999, jovem acima de 190, menos de 5 jogos pela seleção não, mais de 20 com nação forte sim, e o histórico de 16 jogos da seleção (mais de 11 ou 70%), que anda pelos jogos novos;
- `estatuto_pelo_motivo()` = 0x8c5800 (transfer_manager.cpp, thiscall (this sem uso), ret 8): o estatuto no elenco pelo motivo da recusa (0x200: 1, 0x400: 2, 0x2000: 5) ou o do contrato com o clube (+0x4f >> 4); sem contrato, 0;
- `vaga_da_negociacao()` = 0x8c6200 (transfer_manager.cpp, thiscall, ret 4): o endereço da vaga i do vetor de negociações; fora da conta, o começo do vetor;
- `reacao_do_elenco_a_historia()` = 0x755340 (media.cpp, thiscall): depois de uma história sobre o jogador (o boato de transferência): a chance de desafeição pelos clubes interessados (clássico de Glasgow, confronto na liga, rival, mesma divisão), a rejeição pelo clube e, para cada pessoa do elenco, da comissão e da diretoria, a nota pela posição no elenco, ambição, lealdade e preferências; sorteada, a pessoa passa a não gostar do jogador e o contrato dela ganha a marca 1 ou as queixas 0x800;
- `aviso_ao_rival_de_posicao()` = 0x758010 (media.cpp, thiscall): com o jogador de reputação alta ligado ao clube e mais de dois do elenco na posição dele: os rivais de posição insatisfeitos (pela ambição e pela lealdade, por sorteio) ganham a marca 0x800 no contrato, e o primeiro dá a história 0x25;
- `rotulo_da_resposta_da_midia()` = 0x735160 (media.cpp, stdcall): o rótulo da opção de resposta a uma história da mídia ("Deny rumour", "Defend Player", "No Comment"...), pela tabela de 22 casos do tipo da história;
- `cumprir_suspensoes_internacionais()` = 0x557120 (discipline.cpp, thiscall): no jogo da seleção: as pessoas da nação dela no índice das nações (busca binária pela nação) cumprem um jogo da suspensão na regra 0, uma a uma a partir de uma lista com os ids (a contagem pode soltar a suspensão e reordenar o índice);
- `recurso_da_suspensao()` = 0x557000 (discipline.cpp, thiscall): o resultado do recurso contra a suspensão: o órgão tira as suspensões pendentes da pessoa; a suspensão por jogos perde jogos (ou fica no contador) e a por dias perde dias, e é solta quando acaba ou quando o recurso a anula;
- `texto_da_vaga_da_suspensao()` = 0x55d3c0 (discipline.cpp, thiscall): o texto da suspensão da vaga da pessoa, pelo órgão da regra dela; sem regra na vaga, 0;
- `tirar_da_lista()` = 0x68d1f0 (manager_manager.cpp, thiscall): o primeiro nó cujo item tem os mesmos bytes do dado sai da lista (cabeça, cauda, conta e cursor consertados) e é liberado com o item; 1 se tirou;
- `esvaziar_lista()` = 0x68d080 (manager_manager.cpp, thiscall): libera todos os nós e itens da lista e zera a conta, a cabeça, a cauda e o cursor;
- `item_do_cursor()` = 0x68d2e0 (manager_manager.cpp, thiscall): o item do nó do cursor, sem andar; sem cursor, 0;
- `copa_sortear_pela_semente()` = 0x51d2d0 (Cup.cpp, thiscall): o sorteio da rodada pela semente: os times saem dos confrontos, ordenados pela semente (a maior primeiro), e cada um vai a um confronto sorteado; na primeira volta só confrontos vazios (com as duas vagas livres, o lado sorteado), depois qualquer vaga livre, com a regra 4 separando os clubes da mesma nação até não haver jeito;
- `copa_sortear_separando_nacoes()` = 0x51d9f0 (Cup.cpp, thiscall): o sorteio da rodada que separa as nações: o nível de cada time vira quantos times da nação dele há na rodada (as nações maiores saem primeiro) e, da segunda volta em diante, um time não cai num confronto com outro da mesma nação, enquanto der;
- `copa_comeco_da_temporada()` = 0x518780 (Cup.cpp, thiscall): o começo da temporada da copa base: zera o byte +0xb1;
- `nacao_chave_do_clube()` = 0x48dee0 (comp.cpp, cdecl): o índice ativo da nação do clube entre as nações-chave ativas; fora delas, 0 para uma seleção e 3 para um clube (o tipo que a busca dos jogos e o conflito de datas usam);
- `nacao_chave_do_estadio()` = 0x48df40 (comp.cpp, cdecl): o índice ativo da nação da cidade do estádio entre as nações-chave ativas, ou 3;
- `entrada_da_nacao_chave_ativa()` = 0x48dcd0 (comp.cpp, cdecl): a primeira entrada da tabela das nações-chave com aquele índice ativo; nenhuma, a caixa de erro e 3;
- `conflito_de_jogos_no_mesmo_dia()` = 0x5ae380 (fix_man.cpp, thiscall): o jogo novo e o primeiro do mesmo dia: fica o de prioridade maior (a da competição, +21 com seleção mandante; empate pela reputação do mandante), desde que o outro ainda não tenha sido jogado; o que sai vai para o dia da semana alvo pela data do outro (os dias 0 e 5 da semana vão a 6, 1 e 3 a 2, o 2 a 1, 4 e 6 a 5) ou, com conflito ali, para outra data, com a notícia da remarcação; jogo fixo com conflito não muda (devolve 0);
- `primeiro_ano_com_jogos()` = 0x5aee40 (fix_man.cpp, thiscall): o primeiro ano com jogos do clube: 2001, ou 2002 pelo bit 0 da palavra dele em [0xae2a5c];
- `ultimo_ano_com_jogos()` = 0x5aee60 (fix_man.cpp, thiscall): o último ano com jogos do clube: a palavra dele em [0xae2a5c] sem o bit 0, ou o ano corrente;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (36):** 35 acusadas. `cqb6_so_acima_de_30` escapou: nenhuma venda até 2002-06-01 chegou a pagar a parte dos clubes anteriores (o ramo "cqb_6 parte paga" não rodou na base), então restringir aos jogadores acima de 30 anos não muda nada; a comparação rotina a rotina também não acusou. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (0bb74e04).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 6299f556, original cbf229f9 (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 263ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 264`

**Campos tocados:**
- finanças do clube (0x167 bytes) +0: o saldo (i64); valor; soma e tira (as três)
- finanças +0x158 / +0xb8: a TV do ano e do mês; +0x140 / +0xa0: a renda líquida das partidas do ano e do mês; +0x12c / +0x8c: a manutenção do estádio do ano e do mês; valor; soma (renda_da_partida)
- finanças +0x128 / +0x88: as multas do ano e do mês; valor; soma (pagar_multa_da_liga)
- finanças +0x160: o desconto do ingresso (float); valor; lê (renda_do_estadio)
- partida +0x14 a competição (+0x69 palavra), +0x18 o estádio (+0x3c capacidade), +0x1c / +0x20 os clubes, +0x3a o tipo, +0x3b a fase, +0x3c campo neutro, +0x40 o piso de custo; ponteiro e valor; lê (renda_da_partida, renda_do_estadio)
- resultado +0x34 o público, +0x38 a renda do estádio; valor; lê (renda_da_partida)
- clube +0x53 a nação (+0x7e o tipo), +0x57 a liga, +0x64 a marca (1: custos, gastos e preço pelas faixas), +0x69 / +0x6d o estádio próprio (+0x3c), +0x80 a reputação; ponteiro e valor; lê (renda_da_partida, renda_do_estadio)
- finanças do clube +0 o saldo (i64), +0xb4 / +0x154 os investimentos do mês e do ano, +0x165 o estado de crise; valor; lê e grava (impulso_das_financas, troca_do_presidente)
- finanças +0x120 / +0x80: os outros custos do ano e do mês; valor; soma (pagar_outros_custos)
- clube +0x53 a nação, +0x57 a liga (+0x69 palavra), +0x64 a marca, +0x80 a reputação, +0x82 a marca do impulso (tipo 7); ponteiro e valor; lê e grava +0x82 (impulso_das_financas, troca_do_presidente)
- clube +0xbf o presidente: +0x1a a nação, +4 / +8 / +0xc os nomes, +0x57 byte, +0x69 a ficha (+0xf, +0x16, +0x1d, +0x20); ponteiro e valor; lê e grava (impulso_das_financas: +0x20 cai 1; troca_do_presidente: tudo sorteado)
- [0xae2350] / [0xae23a8] a conta e a tabela das nações (0x122 bytes); valor e ponteiro; lê (troca_do_presidente)
- Friendly +0x34ec / +0x34f0: a tabela das datas internacionais (20 bytes: +0 dia da semana, +1 dia, +2 mês, +3 a data de 8 bytes, +0xb o ano a somar, +0xc continente, +0x10 competição) e a conta; ponteiro e valor; aloca, grava e ordena (tabela_das_datas_internacionais, resolver_datas_internacionais)
- Friendly +0x34f4: o último dia feito dos amistosos internacionais (palavra); valor; grava 400 (tabela_das_datas_internacionais)
- Friendly +0xb1: os 127 torneios (0x69 bytes: +0 o anfitrião, +0x29 em uso); ponteiro; lê (torneio_do_clube_humano)
- [0x9cfa08..0x9cfa1c] os continentes e [0x9cf76c..0x9cf784] as eliminatórias das datas; id; lê (tabela_das_datas_internacionais)
- [0xae2c92] o ano de hoje; valor; lê (resolver_datas_internacionais)
- staff +0x39: o clube do técnico humano da vez ([0xb74c26 + 0x300 × [0xb77c7a]]); ponteiro; lê (torneio_do_clube_humano)
- Friendly +0x34d8 / +0x34dc / +0x34e0 / +0x34e4: o plano dos amistosos (20 bytes), zerado, a conta e a capacidade; ponteiro e valor; libera, aloca e grava (amistosos_dos_clubes)
- [0xb63d60] as 35 ligas ativas (0x48 bytes: +0 a nação, +0x2a o início da temporada, +0x39 a data da pré-temporada); ponteiro e valor; lê (amistosos_dos_clubes)
- candidato do amistoso +0 o id do clube, +4 a conta de amistosos (byte), +5 a faixa, +6 a nação; valor; lê (amistosos_dos_clubes)
- clube +0x53 a nação, +0x57 a liga, +0x80 a reputação, +0x1db o id comparado com [0x9cf6f8]; ponteiro e valor; lê (amistosos_dos_clubes)
- Friendly +0xb1: os 127 torneios (0x69 bytes: +0 anfitrião, +4 a data, +0xc tipo, +0xd/+0xe bytes, +0xf/+0x11/+0x13 palavras, +0x15 os 4 convidados, +0x25 as marcas, +0x29/+0x2d/+0x65 zerados, +0x31 o nome); ponteiro e valor; grava (acrescentar_torneio)
- Friendly +0x34c8: a lista dos amistosos (+2 o começo, +0xa o cursor; amistoso +8 ainda sem data, +0xc o clube); ponteiro; lê e grava o cursor (convite_aberto_do_clube_humano)
- registro físico (31 bytes, [Injury+0]) +0xe / +0x12 / +0x13 / +0x14 / +0x18 / +0x1c / +0x1d: a lesão atual (vaga, −1 = nenhuma), o tipo e a conta da recorrência, o histórico, a marca e o último tipo; id e valor; lê e grava (lesao_nova)
- lesão (0x30 bytes, [Injury+4], conta [Injury+8]) +0 tipo (0xff = vaga livre), +1 registro da partida (+0x1a = 0xff sem partida), +0x20 data, +0x28 dias, +0x2a volta ao treino, +0x2c/+0x2e os dois contadores; valor; grava (lesao_nova), lê (noticia_da_lesao)
- tabela dos tipos de lesão (13 bytes por tipo em 0x9e7adb): +0 dias de base, +2 dias variáveis, +4 percentual da volta ao treino, +5 pode voltar; valor; lê (lesao_nova)
- ficha do jogador (+0x61) +0x2d propensão a lesões, +0x34 forma natural, +0xd potencial; valor; lê (lesao_nova, noticia_da_lesao)
- registro de 79 bytes ([0xae2d0c]) +0xa: sobe com a lesão até 100; valor; lê e grava (lesao_nova)
- clube +0x1cf: os três fisioterapeutas (com +0x69 a ficha de não jogador); ponteiro; lê (lesao_nova)
- lista de observação [[0xde1f54]+0x856] +0xdc + id do clube × 9 + 8: a marca 100 da lesão longa; valor; grava (lesao_nova)
- AWOL ([0xad9c6c]) +0xc + id: bit 0 ausente sem licença; valor; lê (ausente_sem_licenca)
- [0xae1080]: o último contrato consultado; ponteiro; grava (noticia_da_lesao, ausente_sem_licenca)
- oferta +4 / +8 / +0xc / +0x1b: a troca (LLIST de ids), o comprador, a pessoa e o valor; id e valor; lê (as três)
- pessoa +0x18: a idade; valor; lê (parte_dos_clubes_anteriores)
- pessoa +0x52: o valor dos jogadores da troca; valor; lê (percentual_da_proxima_venda_pago)
- registro de 79 bytes [0xae2d0c] +0xe + 5k / +0x12 + 5k (k = 0..4): os clubes anteriores (−1 = vaga vazia) e o peso de cada um; id e valor; lê (parte_dos_clubes_anteriores)
- clube +0x80: a reputação (o piso da notícia da parte); valor; lê (parte_dos_clubes_anteriores)
- finanças do clube (0x167 bytes) +0 saldo, +0x78/+0x118 vendas, +0x70/+0x110 compras; valores; grava pelas 0x599800/0x599890 (parte_dos_clubes_anteriores no comprador, percentual_da_proxima_venda_pago nos dois clubes)
- [this+0x880] / [this+0x888]: as cláusulas de percentual da próxima venda (0xd bytes: +0 percentual, +1 clube vendedor, +5 clube da cláusula, +9 pessoa) e a conta; ponteiro e valor; lê (percentual_da_proxima_venda_pago, noticia_do_percentual_da_venda)
- [0xae1080]: o último contrato consultado; ponteiro; grava (as três)
- [this+0x86e] + id × 8: +0 bit 0x20 tem registro de permissão, +3 o histórico de 16 jogos da seleção (u16), +5 a conta dos jogos da seleção vistos (u16); valor; lê e grava (permissao_concedida)
- [this+0x93a] / [this+0x93e]: os registros de permissão (14 bytes: +0 id, +4 jogos, +6 recusas) e a conta; ponteiro e valor; lê (permissao_concedida)
- pessoa +4 / +0x18 / +0x1a / +0x22 / +0x61 / +0x69: o nome, a idade, a nação, os jogos pela seleção, a ficha e a ficha de não jogador; ponteiro e valor; lê (permissao_concedida)
- ficha +7 / +0xd: a habilidade e o potencial; valor; lê (permissao_concedida)
- ficha de não jogador +0xc: a reputação; valor; lê (permissao_concedida)
- nação +0x8e: a reputação; [[0xb74318]+4] + id × 0x6e + 2: os jogos da seleção; valor; lê (permissao_concedida)
- oferta de contrato +4 / +0xc / +0x2d: o clube, o salário e a data; id e valor; lê (permissao_concedida)
- contrato +0x4f: o estatuto no elenco (>> 4); valor; lê (estatuto_pelo_motivo)
- [this+0xc] / [this+0x10]: o vetor de negociações e a conta; ponteiro e valor; lê (vaga_da_negociacao)
- [0xae1080]: o último contrato consultado; ponteiro; grava (estatuto_pelo_motivo)
- clube +0x9b..+0xb3 favoritos e rejeitados; ponteiro; lê e grava (reacao_do_elenco_a_historia, pela 0x547940)
- clube +0xb3/+0xb7/+0xbb rivais, +0x57 divisão, +0xbf, +0xcf técnico, +0xd3, +0xd7 elenco, +0x19f; ponteiro; lê
- pessoa +0x18 idade, +0x59/+0x5a/+0x5b/+0x5d atributos, +0x61 ficha (+0xb reputação), +0x65 preferências (+0x1c..+0x33), +0x69 ficha de técnico (+0x1d); valor/ponteiro; lê (e grava as preferências pela 0x547af0)
- contrato +0x3d felicidade, +0x40 marcas, +0x45 queixas, +0x4f tipo; valor; lê e grava (pelas 0x867560/0x867700)
- [0x9d0118]/[0x9d011c] Celtic e Rangers, [0x9cf764] divisão inferior, [0xadadfc] competições; id; lê
- [0x7356f4] tabela de bytes do tipo da história; valor; lê (rotulo_da_resposta_da_midia)
- disciplina +0xc índice das nações (8 bytes: +0 id da pessoa, +4 id da nação), +0x12 conta; ponteiro/valor; lê (cumprir_suspensoes_internacionais)
- disciplina +4 suspensões (20 bytes: +0 jogos, +1 contador, +2 dias); valor; lê e grava (recurso_da_suspensao)
- disciplina +0x14 registro de 29 bytes por pessoa (5 bytes por vaga: a regra); valor; lê (texto_da_vaga_da_suspensao)
- órgão: entrada de 14 bytes da pessoa (+6 índice da suspensão); valor; lê (0x7e82e0)
- clube (seleção) +0x53 nação; ponteiro; lê
- lista (14 bytes) +0 conta (palavra), +2 cabeça, +6 cauda, +0xa cursor; valor/ponteiro; lê e grava (tirar_da_lista, esvaziar_lista)
- nó (14 bytes) +0 item, +4 tamanho (palavra com sinal), +6 próximo, +0xa anterior; ponteiro/valor; lê e grava os elos
- copa +0xa3 rodadas (0x68 bytes: +0xb regra 4, +0xd regra 0x400, +0x18 vagas, +0x1a confrontos); ponteiro/valores; lê (copa_sortear_pela_semente, copa_sortear_separando_nacoes)
- copa +0xad confrontos por rodada (0x35 bytes: +0/+2 times, +0xe/+0x10 e +0xa/+0xc vínculos, +6/+8/+0x12 o confronto anterior); valores; lê e grava (copa_sortear_pela_semente, copa_sortear_separando_nacoes)
- copa +0xa7 times (6 bytes: +0 clube, +4 semente); ponteiro/valor; lê (copa_sortear_pela_semente, copa_sortear_separando_nacoes)
- clube +0x53 nação (+0 id); ponteiro; lê (copa_sortear_pela_semente, copa_sortear_separando_nacoes)
- [0xae2350] quantas nações; valor; lê (copa_sortear_separando_nacoes)
- copa +0xb1; valor; grava 0 (copa_comeco_da_temporada)
- [0xb63d60] tabela das 35 nações-chave (72 bytes: +0 nação, +0x14 índice ativo); ponteiro/valor; lê (as três)
- nação +0x11c: nação ativa (com ligas carregadas); valor; lê (nacao_chave_do_clube, nacao_chave_do_estadio)
- clube +0x53 nação; ponteiro; lê (nacao_chave_do_clube)
- estádio +0x38 cidade, cidade +0x1f nação; ponteiros; lê (nacao_chave_do_estadio)
- jogo (0x4f bytes) +0x14 competição, +0x1c mandante, +0x28 ano, +0x2a dia, +0x3a jogo fixo, +0x43 resultado (0xff sem jogo); ponteiros/valores; lê e grava a data na cópia (conflito_de_jogos_no_mesmo_dia)
- competição +0x69 prioridade; valor; lê (conflito_de_jogos_no_mesmo_dia)
- clube +0x80 reputação; valor; lê (conflito_de_jogos_no_mesmo_dia)
- lista (LLIST) +2 começo, +0xa cursor; ponteiros; lê e grava o cursor (conflito_de_jogos_no_mesmo_dia)
- [0xae2a5c] + 2 × id do clube: anos com jogos (bit 0 o primeiro, o resto o último × 2); valor; lê (primeiro_ano_com_jogos, ultimo_ano_com_jogos)
- [0xae2c92] ano corrente; valor; lê (ultimo_ano_com_jogos)

**Interessa a:** CMRTE, GPF2, SGE
