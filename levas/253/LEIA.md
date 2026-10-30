# 253ª leva — a diretoria e o técnico: pensar em demitir o técnico, dar o ultimato e tirar o técnico do cargo; a federação e a diretoria: avaliar a seleção na competição, o clube em apuro financeiro e a faixa da confiança da diretoria; os registros de carreira: o acesso, o rebaixamento e a classificação para a fase seguinte; os registros de carreira: a transferência, a eliminação e a não classificação; as datas da transferência: a data de conclusão da oferta, o primeiro dia que as regras permitem e a janela fixa de transferências; o dinheiro da transferência: a compensação pela pessoa, pagar a transferência e guardar as cláusulas, e se o clube vendedor tem de aceitar; os contratos da transferência: a permissão de trabalho na mudança de clube, concluir a oferta de contrato aceita e oferecer contrato ao jogador da oferta; as ofertas de transferência: desfazer a oferta, fazer a oferta de empréstimo e concluir as transferências futuras do dia; os regens: a nota do doador de dados de não jogador, a ordem das aposentadorias e a ordem da fila de regens; as estatísticas dos jogadores e a aposentadoria marcada: o registro da categoria, criar o registro e se a pessoa está na lista de aposentadorias; a copa: o sorteio dos times da rodada, os confrontos no estádio compartilhado e a escala pela rodada; o lado do jogo, a ordem dos coeficientes das nações e o pedido de observação; a escalação guardada: limpar uma vaga e o batedor de bola parada

**Rotinas:**
- `pensar_em_demitir_o_tecnico()` = 0x69aba0 (manager_manager.cpp, thiscall, ret 8): com a confiança da diretoria baixa e o técnico no cargo há 150 dias: o limite pela tabela da divisão, pelas séries e pela marca do técnico; sorteios do presidente seguram, dão o ultimato, deixam o técnico sair ou o demitem (a saída na lista, motivo 0x69b350), ou trocam pelo melhor candidato com a notícia da pressão;
- `dar_o_ultimato_da_diretoria()` = 0x6ac560 (manager_manager.cpp, thiscall, ret 4): o primeiro jogo do clube nas próximas 8 semanas (fora de hoje, ainda por jogar): a notícia do ultimato com o adversário e o clube na lista dos ultimatos; devolve 1, ou 0 sem jogo;
- `tirar_o_tecnico_do_cargo()` = 0x69b790 (manager_manager.cpp, thiscall, ret 0x10): o técnico sai: a data da saída, o elenco e a comissão, o contrato (fora da seleção), a notícia (fora do tipo 3), a confiança em 3500 (sem manter), o registro da troca, as multas e as queixas e marcas dos contratos do elenco;
- `avaliar_a_selecao_na_competicao()` = 0x6a5610 (manager_manager.cpp, thiscall, ret 8): a confiança da federação pela nota do técnico na competição (−3..3, virtual +0x74), a notícia da avaliação e, conforme o tempo no cargo, a reputação e sorteios, a saída do técnico ou a federação pensando em demiti-lo (0x69aba0);
- `clube_em_apuro_financeiro()` = 0x69eb50 (manager_manager.cpp, thiscall, ret 4): clube dirigível, fora da seleção, com a confiança da diretoria em +8 da linha abaixo de 3000 e saldo negativo;
- `faixa_da_confianca_da_diretoria()` = 0x690b70 (manager_manager.cpp, thiscall, ret 4): a faixa 0..6 da confiança da diretoria (limites 3000, 3500, 4275, 6500, 8500, 9500);
- `registrar_o_acesso()` = 0x869480 (staff_records.cpp, thiscall, ret 0xc): reputação do clube +500 (até 10000) e +0x7b; as notas do técnico pela importância da competição (× 10/61, com mínimo); os históricos do técnico, do presidente e do clube (tipo 8); os pontos do técnico; moral +15 e vínculo +75 do elenco e a renovação dos contratos que vencem;
- `registrar_o_rebaixamento()` = 0x86a1c0 (staff_records.cpp, thiscall, ret 8): reputação do clube −500 (mínimo 750); as notas do técnico caem pela importância da competição (1..10000); os históricos do técnico, do presidente e do clube (tipo 9); moral −15 do elenco, reputações × 0,9 (contratos 1 e 2, com a renovação dos que vencem) ou × 0,95 (3 a 5);
- `registrar_a_classificacao_na_fase()` = 0x86bdd0 (staff_records.cpp, thiscall, ret 0x14): o histórico do técnico (tipo 3, com a fase e a rodada base) e os pontos dele: importância² × peso no ano, no mês e na copa (seleção) ou na liga (clube);
- `registrar_a_transferencia()` = 0x86cd70 (staff_records.cpp, thiscall, ret 4): nos registros dos técnicos do comprador e do vendedor: a maior compra ou venda, as contas, os totais em 64 bits e os jogadores da troca; os históricos de compra (0x64), venda (0x65) e transferência (0x66) dos técnicos, da pessoa e dos jogadores da troca;
- `registrar_a_eliminacao()` = 0x86c000 (staff_records.cpp, thiscall, ret 0x14): o histórico do técnico (tipo 4, com a fase e a rodada base) e o fim das convocações da seleção na competição guardada;
- `registrar_a_nao_classificacao()` = 0x86c1d0 (staff_records.cpp, thiscall, ret 0x10): o histórico do técnico (tipo 7, com a fase) e o fim das convocações da seleção na competição guardada;
- `data_de_conclusao_da_oferta()` = 0x8cee70 (transfer_manager.cpp, thiscall, ret 8): pelo tipo de prazo da oferta (+0x30): hoje, o primeiro dia permitido (0x8cf0e0), o fim da temporada do vendedor mais 7 dias, o dia seguinte ao fim do contrato (nunca antes de hoje) ou o início da janela fixa; devolve o motivo;
- `primeiro_dia_permitido_da_oferta()` = 0x8cf0e0 (transfer_manager.cpp, thiscall, ret 8): anda dia a dia, de hoje (ou do fim do contrato, no tipo 4) até um ano, até as regras das nações do comprador e do vendedor permitirem (método +0xc); sem dia, devolve o primeiro motivo;
- `janela_fixa_de_transferencias()` = 0x8d0bb0 (transfer_manager.cpp, stdcall, ret 8): o início (hoje, se dentro) e o fim da janela atual ou da próxima: 1º a 31 de julho e 15 de dezembro a 15 de janeiro;
- `compensacao_pela_pessoa()` = 0x8d0780 (transfer_manager.cpp, stdcall, ret 0xc): comissão técnica de contrato tipo 2: o que falta do contrato (1000 a 1000000); regras novas: a média com o valor do jogador; entre clubes da nação [0x9cf4a8] ou, abaixo de 24 anos, entre nações britânicas: metade do valor (0x4d0c60);
- `pagar_transferencia_e_clausulas()` = 0x8ca8a0 (transfer_manager.cpp, thiscall, ret 4): soma o valor aos totais, o comprador paga e o vendedor recebe a primeira parcela, guarda as parcelas que faltam (lista +0x872) e as cláusulas de próxima venda e prêmios por jogo, com as marcas da pessoa;
- `clube_vendedor_deve_aceitar()` = 0x8cd240 (transfer_manager.cpp, stdcall, ret 0xc): 1 quando o valor cobre a multa de rescisão ativa (motivo 0x1b) ou o vendedor em crise recebe bem mais que vale (motivo 0x1c); sem vendedor, 1;
- `permissao_de_trabalho_na_mudanca()` = 0x8c7e20 (transfer_manager.cpp, thiscall, ret 8): precisando de permissão no clube novo: atualiza a que tem (nação, ano) ou dá uma nova (0x8c7070); sem precisar: tira a permissão que tiver e reordena o vetor;
- `concluir_oferta_de_contrato_aceita()` = 0x8c38b0 (transfer_manager.cpp, thiscall, ret 4): sem a permissão de trabalho, a notícia da oferta que caiu ou da permissão recusada e 0; senão assina (0x4cf720), o clube paga as luvas e 1;
- `oferecer_contrato_ao_jogador_da_oferta()` = 0x8c1e40 (transfer_manager.cpp, thiscall, ret 0xc): monta o contrato do clube (0x4cda70, fator 1) e acrescenta a oferta de contrato; sem contrato, retira a oferta (jogador principal) ou tira o jogador da troca somando o valor dele;
- `desfazer_oferta()` = 0x8c44e0 (transfer_manager.cpp, thiscall, ret 0x10): estado 0x10, fecha as propostas, solta os lances, posta a notícia, agenda a remoção, tira as marcas, a oferta de contrato e os contratos da troca;
- `fazer_oferta_de_emprestimo()` = 0x8bed60 (transfer_manager.cpp, thiscall, ret 0x18): monta a oferta de empréstimo e o contrato pelo atual (porcentagem do salário), acrescenta os dois e devolve a vaga da oferta; em DESLIGADAS desde 30/10/2026: com o desvio a carreira no modo Rust saía da referência be54a138 (bissecção das 38 rotinas da 253ª); a original monta a oferta e o contrato em variáveis locais que os construtores (0x8e5680, 0x85e410) não preenchem inteiras, e o resíduo de pilha desses bytes vai para a oferta guardada, onde a cópia põe zero; a cópia fica no código e não roda;
- `concluir_transferencias_futuras()` = 0x8cbbe0 (transfer_manager.cpp, thiscall): as transferências futuras com data chegada: desfaz as que as restrições seguram, conclui as outras (0x8c9390) e as tira do vetor;
- `nota_do_doador_de_dados()` = 0x7a7150 (player_regen.cpp, stdcall, ret 4): −10000 sem idade (31+) ou marcado; senão sorteio pela idade, a habilidade sobre a reputação do clube, sorteio pelo cubo de +0x2f, os atributos e os quadrados de +0x5b e +0x57, e o sorteio final (+75 sem +0xcf no clube; sem clube, + (+0x22)² e dobra contra um sorteio de 10000);
- `ordem_das_aposentadorias()` = 0x7ae2a0 (player_regen.cpp, cdecl): comparador do qsort da lista de aposentadorias: pela data que vale (a do anúncio +8 se ainda não passou, senão a da aposentadoria +0x10), ano e dia; no empate pelo id;
- `ordem_da_fila_de_regens()` = 0x7ae240 (player_regen.cpp, cdecl): comparador do qsort da fila de regens: pelo ano (+6), pelo dia (+4) e pelo id (+0);
- `registro_da_categoria()` = 0x7c3920 (player_stats.cpp, thiscall, ret 8): o registro de 0x20 bytes da categoria (1..6) do jogador, pelo índice no registro de 39 bytes do id dele; 0 sem índice;
- `criar_estatisticas()` = 0x7c5d80 (player_stats.cpp, thiscall, ret 8): o registro da categoria do jogador; sem índice, o primeiro registro solto (+0xe = 0xffff) ou um novo no fim (cresce de 100 em 100), zerado;
- `aposentadoria_marcada()` = 0x7ae350 (player_regen.cpp, thiscall, ret 4): 1 quando o id da pessoa está na lista de aposentadorias, senão 0;
- `copa_sortear_times_da_rodada()` = 0x51e220 (Cup.cpp, thiscall, ret 4): tira os times dos confrontos da rodada, embaralha e põe cada um num confronto sorteado (primeiro os vazios), sorteando o lado quando as duas vagas estão livres, e acerta o vínculo com a rodada seguinte;
- `copa_separar_estadio_compartilhado()` = 0x522610 (Cup.cpp, thiscall, ret 4): inverte o mandante do confronto seguinte cujo time divide o estádio com um time de um confronto anterior da rodada, e acerta o lado nos confrontos ligados;
- `escala_pela_rodada()` = 0x4bf850 (comp_util.cpp, cdecl): o valor da rodada entre dois limites: a média com uma rodada só, senão (alto − baixo) × rodada / (rodadas − 1); fora de 0..100, a caixa de erro;
- `lado_do_jogo_para_o_clube()` = 0x4afa80 (comp_util.cpp, cdecl): o lado do clube no jogo: 1 em casa, 2 fora, 3 neutro, pelo estádio do jogo e dos dois clubes (seleções: pela nação do estádio);
- `ordem_dos_coeficientes_das_nacoes()` = 0x4afba0 (comp_util.cpp, cdecl): a ordem do qsort dos coeficientes das nações: o maior antes;
- `pedido_de_observacao()` = 0x7f8d70 (scout_manager.cpp, thiscall, ret 0xc): se o clube (ou o do humano da vez) pediu a observação do jogador, com menos de 3 relatórios e, se pedido, com a data já vencida;
- `limpar_vaga_da_escalacao()` = 0x896e90 (tactics.cpp, thiscall, ret 0xc): a vaga da escalação do clube na tabela tática fica vazia (−1) e os espaços guardados do mesmo clube e time são soltos;
- `marcar_batedor_de_bola_parada()` = 0x897bb0 (tactics.cpp, thiscall, ret 0x14): grava o batedor de uma bola parada (primeira ou segunda escolha); sem pessoa, limpa e a segunda escolha sobe para a primeira;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (30):** 28 acusadas. `cpq1_tecnico_com_contrato` escapou: a 0x690890 (sair da comissão), chamada logo antes, já tira o contrato do técnico (call 0x4d0b50 em 0x69089e, sem condição): a segunda chamada não acha contrato. `cpq2_apuro_sem_saldo` escapou: nenhum clube da carreira chegou à pergunta com a confiança da diretoria abaixo de 3000 e o saldo positivo [inferência]; o comparador confere o valor e também não divergiu no comparando. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (2f65fd6b68d1027b).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (be54a1386f51073a).

**Refazer**, sobre o commit da 252ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 253`

**Campos tocados:**
- linha da diretoria [this] + id×73: +2 bit 2 (saída marcada), +6 confiança, +0x18 data da última notícia; valores; lê e grava (pensar_em_demitir_o_tecnico), grava +6 = 3500 (tirar_o_tecnico_do_cargo)
- MANAGER_MANAGER +4, +0x10: a lista das saídas (6 bytes) e a dos ultimatos (4 bytes, o id do clube); ponteiro; acrescenta (pensar_em_demitir_o_tecnico, dar_o_ultimato_da_diretoria)
- não jogador do técnico (+0x69) +0xa: a segurança no cargo; valor; lê e grava −250 com piso (pensar_em_demitir_o_tecnico)
- não jogador do presidente (+0xbf, +0x69) +0x1d: a paciência; valor; lê (pensar_em_demitir_o_tecnico)
- linha da tabela da divisão (0x4a4850) +4 posição, +0x14..+0x18 últimos resultados, +0x19/+0x41/+0x55/+0x2d séries, +0x74; valores; lê (pensar_em_demitir_o_tecnico)
- staff +0x29 / +0x3e: a data no cargo da seleção / do clube; valor; grava a de hoje (tirar_o_tecnico_do_cargo)
- contrato +0x40 (marcas 0x80, 0x100) e +0x45 (queixas 0x8000000, 8, 0x200, 0x400, 0x2000, 0x10000, 0x80000, 0x800000, 0x10000000); valores; apaga (tirar_o_tecnico_do_cargo)
- [0xdd6ec8] data a partir da qual a diretoria pensa em demitir; valor; lê (pensar_em_demitir_o_tecnico)
- linha da diretoria [this] + id×73: +6 confiança (da federação na seleção), +8 confiança financeira, +0xe; valores; lê e grava +6 (avaliar_a_selecao_na_competicao), lê +6 (faixa_da_confianca_da_diretoria) e +8 (clube_em_apuro_financeiro)
- MANAGER_MANAGER +4: a lista das saídas (6 bytes: 1, motivo 6 ou 0, id); ponteiro; acrescenta (avaliar_a_selecao_na_competicao)
- clube +0x80: reputação; valor; lê (avaliar_a_selecao_na_competicao)
- registro de finanças (0x5a0590 de [0xae2a38]) +0: saldo em 64 bits; valor; lê (clube_em_apuro_financeiro)
- clube +0x80: reputação; valor; lê e grava ±500 com limites (registrar_o_acesso, registrar_o_rebaixamento)
- clube +0x7b, +0x73; valores; lê, grava +0x7b (registrar_o_acesso)
- não jogador do técnico (+0xcf, +0x69) +8, +0xa, +0xc: notas; valores; lê e grava (registrar_o_acesso, registrar_o_rebaixamento)
- competição +0x69: importância; valor; lê (as três); objeto em [0xadadfc] +0x1c (registro de competição de clube, 0x6b bytes em [0xae23d0]) e +0x42; lê (registrar_o_acesso)
- registro de jogador (+0x61) +0x45 moral (0x544710), +9/+0xb/+0xd reputações; valores; lê e grava (registrar_o_acesso: moral; registrar_o_rebaixamento: moral e reputações)
- contrato (0x4cf5f0) +0x44 vínculo, +0x4f tipo (>> 4), +0x25 data; valores; lê, grava +0x44 (registrar_o_acesso), lê (registrar_o_rebaixamento)
- registro de pontos do técnico (0x86f5b0, 210 bytes) +0xb6 ano, +0xba liga, +0xbe copa, +0xca mês; valores; soma (registrar_o_acesso, registrar_a_classificacao_na_fase)
- Staff_Records +0 e +0xc (arquivo e listas do histórico das pessoas), +4 e +0x10 (dos clubes): entradas de 8 bytes de data + clube, tipo, competição; acrescenta (as três)
- registro de pontos do técnico (0x86f5b0, 210 bytes): +0..+0x24 maior compra (pessoa, três ids +0x33, data, valor +0x18, troca +0x1c, vendedor, comprador), +0x25..+0x49 maior venda (valor +0x3d), +0xa0/+0xa2 contas, +0xa4/+0xac totais em 64 bits; valores; lê e grava (registrar_a_transferencia)
- transferência +4 lista da troca (LLIST: cursor +0xa reposto), +8 comprador, +0xc pessoa, +0x11 tipo; ids e ponteiro; lê (registrar_a_transferencia)
- [0xae1080] o contrato atual; ponteiro; grava (registrar_a_transferencia)
- pessoa +0x52: valor; lê (registrar_a_transferencia); +0x39 clube contratado, +0x3d cargo; lê
- Staff_Records +0 e +0xc: arquivo e listas do histórico das pessoas; acrescenta (as três)
- oferta de transferência +0x30: o tipo de prazo (& 0xf: 0 hoje, 1 e 4 pelas regras, 2 fim da temporada, 3 fim do contrato, 5 janela fixa); valor; lê (data_de_conclusao_da_oferta, primeiro_dia_permitido_da_oferta)
- oferta +4: a lista de jogadores na troca (só se há vendedor a consultar); ponteiro; lê (primeiro_dia_permitido_da_oferta)
- oferta +8 / +0xc: o clube comprador e a pessoa; id; lê (as duas primeiras)
- contrato (0x4cf5a0) +4: o clube; +0x2d: o fim (dia, ano); +0x35 & 0x3f: o tipo (5 não conta); id e valor; lê
- [0xae1080] o último contrato achado; ponteiro; grava (data_de_conclusao_da_oferta, primeiro_dia_permitido_da_oferta)
- staff +0x61: a ficha de jogador (sem ela, hoje); ponteiro; lê (primeiro_dia_permitido_da_oferta)
- clube +0x53: a nação (regras de transferência das 28 em TRANSFER_MANAGER +0x8ac, +0xa a nação); ponteiro; lê
- [0xae2c90] a data de hoje (dia do ano, ano); valor; lê (as três)
- oferta de transferência +8 / +0xc: o comprador e a pessoa; id; lê (pagar_transferencia_e_clausulas, clube_vendedor_deve_aceitar)
- oferta +0x1b: o valor; +0x40: as parcelas; +4: a troca; valor e ponteiro; lê
- oferta +0x3f: a porcentagem da próxima venda; +0x35/+0x39 e +0x3a/+0x3e: os prêmios por jogo no time e pela seleção; valor; lê (pagar_transferencia_e_clausulas)
- TRANSFER_MANAGER +0x872: a lista das parcelas (13 bytes); +0x86e: o registro de 8 bytes da pessoa (marcas 8, 2, 4); ponteiro e valor; grava (pagar_transferencia_e_clausulas)
- finanças do clube (0x5a0590) +0x1c: soma ± 5/21 da parcela; +0x165: a crise (1); valor; grava e lê
- contrato (0x4cf5a0) +4 o clube; +0xc o salário; +0x21 a multa de rescisão; +0x2d o fim; +0x35 o tipo; id e valor; lê
- staff +0x61 / +0x69: as fichas de jogador e de comissão técnica; ponteiro; lê (compensacao_pela_pessoa)
- clube +0x53: a nação; +0x80: a reputação; ponteiro e valor; lê
- [0xae233c] as regras novas de transferência (1); valor; lê (compensacao_pela_pessoa)
- [0xae1080] o último contrato achado; ponteiro; grava (pagar_transferencia_e_clausulas, clube_vendedor_deve_aceitar)
- TRANSFER_MANAGER +0x93a / +0x93e: as permissões de trabalho (14 bytes: +0 pessoa, +4 e +6, +8 ano, +0xa nação) e a conta; ponteiro e valor; lê e grava (permissao_de_trabalho_na_mudanca)
- TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, bit 0x20 (tem permissão); valor; lê e grava (permissao_de_trabalho_na_mudanca)
- nação (registros de 0x122 bytes em [0xae23a8]) +0x76: a marca 0xe; valor; lê (permissao_de_trabalho_na_mudanca)
- oferta de contrato +0 / +4: a pessoa e o clube; +8: as luvas; +0x61: o cargo; id e valor; lê (concluir_oferta_de_contrato_aceita)
- oferta de transferência +0xc: o jogador principal; +0x11: o tipo (0xb); +0x1b: o valor; +0x20 / +0x24: a vaga e o valor da oferta de contrato; id e valor; lê e grava +0x1b (oferecer_contrato_ao_jogador_da_oferta)
- staff +0x52: o valor do jogador; valor; lê (oferecer_contrato_ao_jogador_da_oferta)
- clube +0x53: a nação; ponteiro; lê
- [0xae1080] o último contrato achado; ponteiro; grava (concluir_oferta_de_contrato_aceita)
- oferta de transferência +0x2c: o estado (0x13, 0x10, 0x14...); +0x2e: o passo; valor; lê e grava (desfazer_oferta, concluir_transferencias_futuras)
- oferta +0 / +4: as listas das ofertas rivais e dos jogadores da troca (LLIST, cursor +0xa); ponteiro; lê (desfazer_oferta)
- oferta +0x10: zerado ao desfazer; +0x20 / +0x24: a vaga e o valor da oferta de contrato; valor; grava e lê
- TRANSFER_MANAGER +0x86e: registro de 8 bytes da pessoa, bits 0x10 e 1; valor; grava (desfazer_oferta)
- TRANSFER_MANAGER +0x920 / +0x928: as transferências futuras (12 bytes: data, índice da oferta) e a conta; ponteiro e valor; lê e grava (concluir_transferencias_futuras)
- TRANSFER_MANAGER +0: as ofertas (ponteiros pelo índice); +0x856: a Shortlist_Manager; ponteiro; lê
- [0xae1080] o último contrato achado; ponteiro; grava (desfazer_oferta, concluir_transferencias_futuras)
- pessoa +0x18, +0x22, +0x56..+0x5d, +0x69: idade, um byte sem sinal, atributos (bytes com sinal) e a marca que exclui; valor; lê (nota_do_doador_de_dados)
- pessoa +0x39, +0x61: clube e jogador; ponteiro; lê (nota_do_doador_de_dados)
- jogador +0xb, +0xd, +0x2f: habilidade atual e potencial (com sinal) e um byte com sinal; valor; lê (nota_do_doador_de_dados)
- clube +0x80, +0xcf: reputação e um ponteiro que dá +75 quando falta; valor e ponteiro; lê (nota_do_doador_de_dados)
- [0xae2d0c] registro de 79 bytes por pessoa, +0xb bits 0x30; valor; lê (nota_do_doador_de_dados)
- aposentadoria (0x1a bytes) +0, +8, +0x10: id, data do anúncio e data da aposentadoria (dia com sinal, ano sem sinal); id e valor; lê (ordem_das_aposentadorias)
- [0xae2c90], [0xae2c92]: dia e ano do jogo; valor; lê (ordem_das_aposentadorias)
- fila de regens (0xc bytes) +0, +4, +6: id, dia e ano; id e valor; lê (ordem_da_fila_de_regens)
- PlayerStats +0x30: o índice por jogador (39 bytes por id; +0..+0x14 os índices das 6 categorias, −1 sem registro); ponteiro e valor; lê (registro_da_categoria) e grava (criar_estatisticas)
- PlayerStats +4, +8, +0xc, +0x38: os registros de 0x20 bytes, a capacidade, a conta e os soltos; ponteiro e valor; lê e grava (criar_estatisticas)
- registro de estatísticas +0xe: 0xffff no registro solto; valor; lê (criar_estatisticas)
- PLAYER_REGEN +4, +8: a lista de aposentadorias (0x1a bytes, +0 o id) e a conta; ponteiro e valor; lê (aposentadoria_marcada)
- Cup +0xa3: as rodadas (0x68 bytes: +0x18 vagas de times, +0x1a confrontos); lê (as três da copa)
- Cup +0xad: os confrontos por rodada (0x35 bytes: +0/+2 times, −1 sem time, −2 vaga livre; +0xe/+0xa e +0x10/+0xc o confronto ligado na rodada seguinte; +6 confronto de origem, +8 lado, +0x12 rodada de origem); lê e grava (copa_sortear_times_da_rodada, copa_separar_estadio_compartilhado)
- Cup +0xa7: os times (6 bytes: +0 clube, +4 nível); lê (copa_sortear_times_da_rodada, copa_separar_estadio_compartilhado)
- club +0x69: o estádio; ponteiro; lê (copa_separar_estadio_compartilhado: dois clubes com o mesmo estádio não nulo)
- fixture +0x18: o estádio do jogo (0 sem estádio); +0x1c/+0x20 mandante e visitante; ponteiros; lê (lado_do_jogo_para_o_clube)
- club +0x69: o estádio; club +0x53: a nação; estádio +0x38 → nação +0x1f; ponteiros; lê (lado_do_jogo_para_o_clube)
- vetor temporário da 0x5d5f20 (12 bytes: +0 nação, +4 coeficiente em double); lê (ordem_dos_coeficientes_das_nacoes)
- SCOUT_MANAGER +0x5c, +0x64: os pedidos de observação (17 bytes: +0 id do clube, +4 id do jogador, +8 data, +0x10 relatórios) e a conta; lê (pedido_de_observacao)
- [0xb74c26 + 0x300 × [0xb77c7a]] +0x39: o clube do humano da vez; [0xae2c90] hoje; lê (pedido_de_observacao)
- Tactics +0x601: a tabela tática (linhas de 0x91 bytes, linha da 0x8982b0): +2 as 20 vagas da escalação (id da pessoa ou −1), +0x5e os batedores (5 bolas paradas × 2 escolhas); id; lê e grava (limpar_vaga_da_escalacao, marcar_batedor_de_bola_parada)
- Tactics +0x30: 16 espaços guardados de 0x5c bytes (+0 clube, +4 time, +8 ocupado) e +0x5f0 a conta; lê e grava 0 / −1 (limpar_vaga_da_escalacao)

**Interessa a:** CMRTE, SGE, GPF2, CMScout
