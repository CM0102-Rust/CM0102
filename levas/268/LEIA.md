# 268ª leva — a resposta da IA ao lance (o lance de empréstimo recusado de saída, o piso, as opções de dinheiro, troca e cláusula, o aceite, a contraproposta e a recusa) e a negociação de empréstimo copiada da oferta; a opção de troca da resposta da IA ao lance (os jogadores do comprador que o vendedor quer, pelo interesse e pela nota) e o fim do empréstimo agendado; os lances e os pedidos de empréstimo da IA pelas 40 vagas da lista de observação; a vontade de uma pessoa de ir para um clube (ou de ficar no dela), com os motivos, e o construtor do contrato; as regras de empréstimo da Inglaterra: pode emprestar (mesma divisão, cinco na temporada, dois ao mesmo tempo, a janela), a conta da temporada e a volta a zero em 20 de junho; o período do empréstimo na Inglaterra e as regras de empréstimo da Escócia, da Itália, da Irlanda e da Espanha

**Rotinas:**
- `resposta_da_ia_ao_lance()` = 0x8c8090 (transfer_manager.cpp, thiscall, ret 8): o vendedor do computador responde ao lance: o de empréstimo é recusado sem olhar mais nada; sem regra que o faça aceitar, o piso e as outras ofertas, o valor da troca, as opções de dinheiro, troca e cláusula; a opção × 0,85 abaixo do pedido aceita, a melhor × 0,75 abaixo vira contraproposta, senão recusa;
- `negociacao_de_emprestimo()` = 0x8bee50 (transfer_manager.cpp, thiscall, ret 0x14): só pela tela: a cópia da oferta sem as listas, com o contrato montado pelos termos novos do empréstimo, vira negociação; devolve o índice da cópia;
- `opcao_de_troca()` = 0x8c8860 (transfer_manager.cpp, thiscall, ret 0x18): monta a opção de troca: oferta nova pelo preço com atração 50; para o alvo e as 25 vagas da lista do vendedor, jogadores do comprador na mesma posição, livres de ofertas e trocas, com interesse de pelo menos 3,15 e nota acima de 0,9 da do candidato entram na troca (até 3): a atração sobe 20 ou 5 e o valor deles sai do preço;
- `agendar_fim_do_emprestimo()` = 0x4ccca0 (contract_manager.cpp, thiscall, ret 8): só pela tela: o contrato de empréstimo da pessoa acaba amanhã (tipo 7) e, com técnico humano no clube que interessa, a volta antecipada;
- `lances_do_clube()` = 0x84a020 (shortlist_manager.cpp, thiscall, ret 4): clube do computador com lista ativa: em cada vaga com alvo e sem oferta, as regras de cargo, contrato, chegada e oferta; na de compra, o valor da vaga (dobros pela marca 0x20 e pela liga, o substituto lesionado, a ficha e a oferta por ele) e o lance se sorteio(valor) passar de sorteio(2 × reputação + 1000), com a oferta de contrato e a renovação no vendedor; na de empréstimo, o empréstimo possível e o pedido (0x842740) se sorteio(+0xb) passar de sorteio(reputação / 10) ou de sorteio(2 × reputação);
- `cqe4_interesse()` = 0x846920 (shortlist_manager.cpp, thiscall, ret 0x18): a nota de vontade (devolvida no st0) e os bits de motivo: a base 2,75 (3,5 no próprio clube, o contrato acabando, a idade sem clube), a reputação dos clubes no empréstimo, gostar ou não do clube e do técnico, rivais, nações e distância, a moral, a situação no elenco e a pedida, a reputação comparada (estádio, divisão, competição, técnico, finanças), a fluência no idioma, o tempo sem clube e, pedido o salário, o salário padrão contra o teto do clube;
- `cqe4_construir_contrato()` = 0x85e410 (staff_contracts.cpp, thiscall, ret 0): o contrato vazio: as datas +0x25 e +0x2d padrão, +4 e +0x4a = -1, +8, +0x3b, +0x40, +0x45, +0x49 e +0x4e = 0;
- `cqe5_pode_emprestar()` = 0x579d60 (england_rules.cpp, thiscall, ret 0x10): recusa (0, com a mensagem e o motivo) entre clubes da mesma divisão inglesa (0x1d), com cinco empréstimos na temporada (0x1e), com mais de um jogador emprestado de outro clube no elenco (0x26) e fora da janela (9); senão 1;
- `cqe5_registrar_emprestimo()` = 0x579fd0 (england_rules.cpp, thiscall, ret 4): soma 1 na conta da temporada do clube que pega emprestado, quando ele é inglês;
- `cqe5_dia_das_regras()` = 0x57a020 (england_rules.cpp, thiscall, ret 4): em 20 de junho, zera a conta da temporada de cada clube da nação destas regras;
- `cqe6_periodo_inglaterra()` = 0x579c70 (england_rules.cpp, thiscall, ret 0x1c): depois de 19 de junho e com o jogador abaixo de 24 anos, o período da regra base; senão do mês seguinte a três meses depois;
- `cqe6_pode_emprestar_escocia()` = 0x7f6650 (scotland_rules.cpp, thiscall, ret 0x10): recusa (0x1d) entre dois clubes da mesma divisão escocesa;
- `cqe6_periodo_escocia()` = 0x7f6530 (scotland_rules.cpp, thiscall, ret 0x1c): começa no mês seguinte; abaixo de 22 anos acaba no mês do fim da temporada (neste ano ou no seguinte), senão três meses depois;
- `cqe6_pode_emprestar_italia()` = 0x661770 (italy_rules.cpp, thiscall, ret 0x10): recusa fora das duas primeiras janelas (9) e, nas cinco divisões listadas, com quatro jogadores emprestados de outros clubes (0x25);
- `cqe6_pode_emprestar_irlanda()` = 0x63c3d0 (ireland_rules.cpp, thiscall, ret 0x10): recusa com cinco empréstimos na temporada (0x1e) e entre clubes da mesma divisão (0x1d);
- `cqe6_pode_emprestar_espanha()` = 0x859630 (spain_rules.cpp, thiscall, ret 0x10): recusa fora da janela (9) e entre clubes da mesma divisão depois de cinco jogos do jogador na liga (0x1f);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (18):** 17 acusadas. `cqe1_preco_sempre_o_piso` escapou: o ramo do preço pelo k rodou (contador ≥ 256), mas em nenhuma das ofertas até 2002-06-01 a opção mais barata mudou a decisão: as que a IA aceitou continuaram acima do pedido e as recusadas abaixo, e só 8 lances chegaram à contraproposta; a comparação rotina a rotina também não acusou [inferência]. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (8845ebc9).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 151bdf85, original 117d6bb4 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 267ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 268`

**Campos tocados:**
- oferta +0 lista das ofertas pelo jogador, +4 troca, +8 comprador, +0xc alvo, +0x1f empréstimo, +0x20 índice, +0x2c status, +0x2e, +0x30 nibble baixo, +0x31 tipo (4 bits); id e valor; lê e grava +0x2c (resposta_da_ia_ao_lance), lê tudo (negociacao_de_emprestimo)
- pessoa +0x52 valor, +0x61 ficha de jogador; valor e ponteiro; lê (resposta_da_ia_ao_lance)
- contrato +4 clube, +0x35 tipo (6 bits); [0xae1080] o último contrato; valor e ponteiro; lê, grava [0xae1080] (resposta_da_ia_ao_lance)
- gerência de transferências +0 vetor das ofertas, +0x856 a lista de observação, +0x946 a lista dos contratos de negociação; ponteiro; lê (resposta_da_ia_ao_lance), lê e grava +0x946 (negociacao_de_emprestimo)
- oferta nova +4 troca (itens de 7 bytes: id, 0, estatuto), +0x1b preço, +0x20 índice; valor; grava (opcao_de_troca)
- vaga da lista de observação +0xf alvo, +0x24 estatuto (bits 1..4); registro de 9 bytes +0 vagas, +4 ativa; valor; lê (opcao_de_troca)
- contrato +0x35 tipo, +0x4f estatuto (4 bits altos); registro de 79 bytes +0xb bits 0x30; pessoa +0x39 clube, +0x61 ficha; valor e ponteiro; lê (opcao_de_troca)
- registro de 79 bytes +0 contrato, +4 contrato de empréstimo; contrato +0x2d fim, +0x35 tipo (bits 3..5 = 0, baixos 7); [0xae1080]; índice e valor; lê e grava (agendar_fim_do_emprestimo)
- vaga da lista de observação (0x25 bytes, 40 por clube em [this+0xdc] + 9 × id) +0 cargo, +1 posição, +7, +0xb valor, +0xf alvo, +0x13 substituto, +0x17 oferta, +0x24 bits (0 empréstimo, 5 dobro); valor e id; lê e grava +0x17 e +0x24 (lances_do_clube)
- registro de 9 bytes do clube em [this+0xdc] +0 vagas, +4 ativa, +8 dias; ponteiro e valor; lê (lances_do_clube)
- clube +0x53 nação, +0x57 liga, +0x64, +0x80 reputação, +0xd3 técnico, +0x19f/+0x1b3/+0x1cf as listas da comissão; valor e ponteiro; lê (lances_do_clube)
- pessoa +0x24/+0x28 empréstimo de saída, +0x39 clube, +0x61 ficha (+0x45 moral do jogador); registro de 79 bytes +0xb (bits 0x30 e 4 baixos); lesão +0x2c dias; contrato +0x4e bit 2, +0x4f estatuto; [0xae1080]; lê, grava [0xae1080] (lances_do_clube)
- pessoa +0x18 idade, +0x1a nação, +0x22 jogos pela seleção (byte), +0x39 clube favorito, +0x3e/+0x40 data (fora do clube), +0x56/+0x57/+0x59/+0x5a/+0x5b atributos de caráter, +0x61 ficha (+5, +7, +9/+0xb/+0xd reputações, +0xf posição, +0x45 moral do jogador (i8)), +0x69; valor e ponteiro; lê (cqe4_interesse)
- clube +0x53 nação (+0x71, +0x77 idioma, +0x7e, +0x85), +0x57 divisão (+0x69), +0x64, +0x69 estádio (+0x38 cidade: +0x23/+0x2b coordenadas, +0x33 importância), +0x73 público, +0x80 reputação, +0xb3/+0xb7/+0xbb rivais, +0xcf técnico (+0x1a nação, +0x3e/+0x40 data de chegada, +0x59, +0x69), +0x1db competição; valor e ponteiro; lê (cqe4_interesse)
- contrato +4 clube, +8, +0xc, +0x25/+0x2d datas, +0x2f ano do fim, +0x35 tipo, +0x3b, +0x3d pedida, +0x40, +0x44 felicidade no contrato (happiness, i8 ±100), +0x45 marcas (1, 0x10, 0x20, 0x100, 0x161a8), +0x46 bit 1, +0x49 pedida em aberto, +0x4a, +0x4e bit 8, +0x4f situação (>> 4); valor; lê (cqe4_interesse), grava (cqe4_construir_contrato)
- [0xae1080] o último contrato consultado; ponteiro; grava (cqe4_interesse); [Shortlist_Manager +0xdc] tabela de 9 bytes por clube (+5 reputação); valor; lê; [0xb67a44] reputações vistas pelo técnico (0x49 bytes por clube: +6, +8, +0xa); valor; lê
- [[0xde1f54] + 0x91c] + id do clube que pega emprestado × 16, byte +0 (no TRANSFER_MANAGER, transfer.dat no save): a conta dos empréstimos da temporada do clube; valor; lê (cqe5_pode_emprestar), soma 1 (cqe5_registrar_emprestimo), zera (cqe5_dia_das_regras)
- clube +0x38 nome, +0x52, +0x53 nação, +0x57 divisão, +0xd7 as 50 vagas do elenco; ponteiro e valor; lê (cqe5_pode_emprestar, cqe5_registrar_emprestimo, cqe5_dia_das_regras)
- oferta +8 o clube que pega (índice em [0xae23b4]); id; lê (cqe5_registrar_emprestimo); regras +0xa a nação; ponteiro; lê; [0x9cf5bc] a divisão inglesa, [0x9cf2e4] a Inglaterra, [0xae2364] o limite dos ids; id; lê; [0xae1080] o último contrato; ponteiro; grava (cqe5_pode_emprestar)
- jogador +0x18 idade, +0x61 ficha; valor e ponteiro; lê (cqe6_periodo_inglaterra, cqe6_periodo_escocia, cqe6_pode_emprestar_espanha)
- clube +0x38 nome, +0x52, +0x57 divisão, +0xd7 as 50 vagas do elenco; ponteiro e valor; lê (os can_loan_player)
- [[0xde1f54] + 0x91c] + id × 16, byte +0: a conta da temporada; valor; lê (cqe6_pode_emprestar_irlanda); regras +4 as janelas, +0xa a nação; ponteiro; lê; [0x9cf5dc] a divisão escocesa, [0x9cf578]..[0x9cf588] as divisões italianas; id; lê; [0xae1080] o último contrato; ponteiro; grava

**Interessa a:** CMRTE, GPF2
