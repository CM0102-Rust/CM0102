# 267ª leva — o lance aceito pelo clube vendedor (a recusa em se mudar no empréstimo, os jogadores da troca caros demais e a oferta reagendada), a notícia da oferta feita e o empréstimo possível; a liga: o dia da liga (fases, fim da temporada, TV, prazo das inscrições), a criação dos jogos da temporada e a promoção e o rebaixamento; o calendário: o dia do calendário (reservas, notícias de jogo, amistosos de ontem, rótulos das rodadas), a lista dos jogos do dia e a carga dos jogos do ano; a copa e as fases: a árvore dos confrontos da copa (as vagas na rodada seguinte e os times da rodada, sorteados), o começo do grupo de uma fase e a fase em andamento; o treino do jogador posto em dia: o progresso das semanas, o programa do jogador e o nível da categoria; o progresso do treino guardado: tirar e devolver o progresso, o quanto o treino soma a um atributo e a aptidão para uma posição; as aposentadorias do dia: as anunciadas e as que vencem, a notícia da aposentadoria e a do jovem promovido; o fim da falta ao treino e da ausência sem licença e a notícia da falta ao treino; a troca com compensação, a ordem das vagas da lista de compras e o jogador sob oferta; os registros do campeão da liga e da copa e o registro do técnico humano zerado

**Rotinas:**
- `aceitar_lance()` = 0x8c0080 (transfer_manager.cpp, thiscall, ret 8): o vendedor aceita o lance: status 5; no empréstimo, interesse do alvo abaixo de 0,75 dá a recusa em se mudar; os jogadores da troca com salário desejado acima de 1,5 teto do vendedor saem, com o valor somado ao preço; a oferta vai à fatia seguinte;
- `noticia_da_oferta_feita()` = 0x8d3090 (transfer_manager.cpp, thiscall, ret 4): a notícia 0xfa0 da oferta feita (alvo, clubes, valores, troca, contrato, janela) aos técnicos humanos interessados;
- `pode_emprestar()` = 0x8d0040 (transfer_manager.cpp, thiscall, ret 0xc): o clube pode pegar o jogador emprestado: clube atual, ficha de contrato, cargo de jogador, se o clube chega a ele, a regra da nação do clube e a do clube atual mês a mês no período do empréstimo; o texto do motivo na mensagem;
- `liga_dia()` = 0x6847c0 (league.cpp, thiscall, ret 8): o dia da liga: a notícia da tabela no dia certo, o dia de cada fase, o evento da data +0xc9, o fim da temporada (fase seguinte, histórico, recordes), os jogos para a TV quatro semanas adiante, o prazo das inscrições 5/20/14 dias antes da primeira data e a data +0xd1;
- `liga_criar_tabela()` = 0x6835c0 (league.cpp, thiscall, ret 0): os jogos da temporada: a grade do rodízio (com a ordem dos clubes sorteada) ou a lista própria do ano, o mando pela paridade do turno e do ano, o estádio, a data e a fatia de cada rodada, e o calendário; depois as datas da TV;
- `liga_promover_e_rebaixar()` = 0x689c80 (league.cpp, thiscall (ecx repassado), ret 0x18): as cláusulas de não promoção, os rebaixados de cima descem (cláusulas de rebaixamento, técnico humano demitido numa competição sem registro) e os promovidos de baixo sobem, aos pares ou todos; a caixa se as contas diferem das esperadas;
- `calendario_dia()` = 0x5aaf00 (fix_man.cpp, thiscall, ret 0): nos dias 2 e 3 os jogos de reservas de daqui a 3 dias (e de amanhã no dia 3); a fila das notícias de jogo aos técnicos dos dois clubes; nos dias 5 e 6 os amistosos de ontem entre clubes sem técnico humano saem das listas; os rótulos das rodadas das copas da lista das conferências das pernas;
- `calendario_lista()` = 0x5abf10 (fix_man.cpp, thiscall, ret 0x10): a lista dos jogos do ano, dia, fatia e nação-chave, trocando o bloco do outro ano para o ano pedido ou criando-o;
- `calendario_carregar_ano()` = 0x5ac3b0 (fix_man.cpp, thiscall, ret 0): lê o fixtures_<ano>.tmp: cada jogo de 0x4f bytes ganha os ponteiros dos clubes, da competição e do estádio e entra na lista do dia, fatia e nação-chave;
- `copa_arvore_dos_confrontos()` = 0x518790 (Cup.cpp, thiscall, ret 0): aloca os confrontos de cada rodada, zera-os, liga cada confronto a uma vaga na rodada seguinte (pelo par ou sorteada, o lado pela moeda; sem vaga, duas rodadas adiante), põe os times de cada rodada nos confrontos dela (sorteados, o time da nação da copa como mandante) e numera o primeiro confronto de cada rodada;
- `competicao_primeira_fase_do_grupo()` = 0x48d2b0 (comp.cpp, thiscall, ret 4): da fase pedida para trás, o índice da primeira fase do mesmo grupo;
- `competicao_fase_atual()` = 0x48cec0 (comp.cpp, thiscall, ret 0): a primeira fase que não acabou (numa copa que não começou, o começo do grupo dela); todas acabadas, o começo do grupo da última;
- `atualizar_o_treino_do_jogador()` = 0x8b1750 (training_manager.cpp, thiscall): põe em dia as semanas de treino do jogador (até 30): o programa do clube de técnico humano, os sorteios do progresso de cada categoria pelo peso do programa e do treinador, os treinos de posição (a nota, o fim e a reclamação do jogador) e o progresso nos atributos;
- `programa_de_treino_do_jogador()` = 0x8b39f0 (training_manager.cpp, thiscall): o programa de treino do jogador: o do técnico humano do clube pela vaga da entrada (o de descanso se lesionado) ou o programa padrão da vaga;
- `nivel_da_categoria_do_programa()` = 0x8b4f50 (training_manager.cpp, thiscall): o nível de uma das cinco categorias do programa de treino;
- `tirar_ou_devolver_o_progresso()` = 0x8b3640 (training_manager.cpp, thiscall): tira dos atributos o progresso guardado na entrada do treino (guardando os cinco meios bytes) ou o devolve, em volta das normalizações dos atributos do jogador;
- `mudanca_do_atributo_pelo_treino()` = 0x8b37c0 (training_manager.cpp, thiscall): quanto o progresso do treino soma a um atributo: o atributo menos o mesmo atributo sem o progresso;
- `aptidao_para_a_posicao()` = 0x8b4770 (training_manager.cpp, thiscall): a aptidão do jogador para aprender uma posição ou um lado: a média da adaptabilidade com os atributos que a posição pede;
- `aposentadorias_do_dia()` = 0x7a59e0 (player_regen.cpp, thiscall): as aposentadorias pendentes do dia: desiste das do jovem com clube, de quem tem contrato pendente, está sob oferta ou mudou de clube há pouco; aposenta (notícia, contrato, seleção, elenco), pode fazer o jogador virar técnico (os atributos pela reputação e pela ficha), guarda a ficha no banco dos regens ou põe a vaga para o regen, e anuncia as futuras;
- `noticia_da_aposentadoria()` = 0x7a92e0 (player_regen.cpp, stdcall): a notícia 0xbba da aposentadoria para os técnicos (a todos, aos do país, da divisão, do clube ou aos clubes interessados, pela fama) e o seguro do clube quando a lesão aposenta um jogador de peso;
- `noticia_do_jovem_promovido()` = 0x7a8ba0 (player_regen.cpp, stdcall): a notícia 0xbce do jovem que chega ao clube, com o veredito do auxiliar ou de um treinador sobre a habilidade dele;
- `limpar_ausencia()` = 0x4196a0 (awol.cpp, thiscall): a ausência acaba: com o bit 1 da marca da pessoa, o bit troca e o registro dela sai da lista das ausências;
- `limpar_falta_ao_treino()` = 0x4197c0 (awol.cpp, thiscall): a falta ao treino acaba: com o bit 2 da marca da pessoa, o bit troca e o registro dela sai da lista das faltas;
- `noticia_da_falta_ao_treino()` = 0x41a630 (awol.cpp, thiscall): a notícia 0xbc9 da falta ao treino: o jogador, o clube, sorteio(3), se o clube é de humano e quem relata (o técnico do clube da IA, ou por sorteio um dos 5 treinadores ou o auxiliar); ao clube do jogador se ele é de humano, senão à nação com a reputação do jogador acima de 7250;
- `troca_com_compensacao()` = 0x84b6f0 (shortlist_manager.cpp, thiscall): o clube leva o jogador pagando a compensação: fora do período protegido da FIFA, se a taxa (0x8d0780) é menor que o valor dele; dentro, só com lei Bosman e a taxa até o valor, pela faixa do vencimento do contrato (0 não; 1-8: o +3 da vaga × 0,5 a 0,9 acima do preço; 9-14 sim);
- `ordem_das_vagas()` = 0x83b6c0 (shortlist_manager.cpp, cdecl): o critério do qsort das 40 vagas da lista de compras: a diferença dos índices (+0x23); iguais, 1;
- `marcar_sob_oferta()` = 0x84b580 (shortlist_manager.cpp, thiscall): o jogador sob oferta: a vaga dele nas listas dos clubes interessados (fora os de humano, o comprador ou os clubes das ofertas da troca) ganha o bit 0x20, que dobra o preço do lance da IA;
- `campeao_da_liga_nos_registros()` = 0x868c50 (staff_records.cpp, thiscall): o campeão da liga: a reputação do clube (até peso × 500), o +0x7b e o bloco +0x69, as notas e o registro do técnico (peso × 5000), os históricos (tipo 5), o moral do elenco pela média do jogador (acima do limiar: +20 e +100 no contrato com a renovação; senão +10 e +50) e o elogio da mídia;
- `campeao_da_copa_nos_registros()` = 0x86a800 (staff_records.cpp, thiscall): o campeão da copa: a reputação do clube, as notas do lado dele na partida, o registro do técnico (peso^4 × 0,5), os históricos (tipo 0), o moral do elenco (+15 e +100 no contrato, com a renovação) enquanto a reputação não passa de peso × 400, o fim das convocações e o elogio da mídia;
- `limpar_registro_do_humano()` = 0x86fc70 (staff_records.cpp, thiscall): o registro do técnico humano (um dos 16; senão a própria pessoa, como na original) zerado, com −1 nas vagas;

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (29):** 28 acusadas. `cqd10_fica_com_o_contrato` escapou: o aposentado sai do elenco de qualquer jeito (0x7aace0) e a vaga dele vai para o regen; o contrato que sobra é de uma pessoa fora do jogo, e nenhuma rotina da carreira até 2002-06-01 decidiu nada por ele (uns 18 aposentados de fato); a comparação rotina a rotina também não acusou [inferência]. (lacuna em `PENDENCIAS.md`)
- **Jogo real comparando até 2002-01-01:** 0 divergências (32ca2db6).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 f8972df1, original e3b7b045 (registrada; não barra a leva desde 31/10/2026).

**Refazer**, sobre o commit da 266ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 267`

**Campos tocados:**
- oferta +0 lista, +4 troca (nós com +5 estatuto), +8 comprador, +0xc alvo, +0x11 tipo, +0x1b preço, +0x20, +0x24, +0x2c status, +0x2d, +0x2e, +0x2f, +0x30, +0x31; id e valor; lê e grava (aceitar_lance), lê (noticia_da_oferta_feita)
- gerência de transferências +0x86e registros de 8 bytes (bit 0 do +0, contagem +2), +0x856 a lista de observação; valor; grava (aceitar_lance)
- entrada da lista de observação +0 cargo, +0x17 índice da oferta; valor; lê e grava (aceitar_lance)
- regras de nação (as 28 em +0x8ac) +0xa nação, virtuais +0x20 e +0x24; pessoa +0x3d cargo; clube +0x38 nome, +0x52, +0x53 nação; ponteiro e valor; lê (pode_emprestar)
- contrato +0x2d/+0x2f fim, +0x35 tipo; [0xae1080] o último contrato; valor e ponteiro; lê (noticia_da_oferta_feita), grava [0xae1080] (as duas)
- liga +0x2c fases, +0x30 fase atual, +0xc as fases, +0x3c turnos, +0x3e times, +0x40 ano, +0x44 fase (0xff primeira), +0x45 clubes, +0x4c fim feito, +0x52 bit 2 inscrições, +0xa7 rodadas, +0xa9 datas, +0xad, +0xb1 tabela (0x3b bytes: +0 clube, +0x37 situação, +0x38 marca; +0x11 em andamento), +0xb5 parada, +0xba tabela das datas (0x41 bytes), +0xc9 data do evento, +0xd1 última data, +0xdd fim da temporada, +0xe6, +0xea lista própria; valor e ponteiro; lê e grava (liga_dia, liga_criar_tabela, liga_promover_e_rebaixar)
- partida (0x4f bytes) +0 competição, +8, +0xc/+0x10 ids, +0x14 competição, +0x1c/+0x20 clubes, +0x24/+0x26 última data, +0x28 ano, +0x2a dia, +0x2c ano da liga, +0x34 rodada, +0x3f fatia, +0x40/+0x41 marcas, +0x42 fase, +0x4e bit 3 última rodada; valor e ponteiro; grava (liga_criar_tabela)
- clube +0x57 competição, +0x5b competição anterior, +0x69 nação; ponteiro; lê e grava (liga_promover_e_rebaixar), lê (liga_criar_tabela)
- calendário (bloco do ano, 0x1184 bytes) +0..+0x1127 as listas de 366 dias × 3 fatias (vetor por nação-chave), +0x1128 a fila das notícias de jogo, +0x112c o bloco do outro ano, +0x1130 o ano, +0x1133 o nome do arquivo; ponteiro e valor; lê e grava (calendario_dia, calendario_lista, calendario_carregar_ano)
- partida (0x4f bytes) +0 competição (id → +0x14), +4 estádio (id → +0x18), +0xc/+0x10 clubes (ids → +0x1c/+0x20), +0x2a dia, +0x3a tipo, +0x3f fatia; id, ponteiro e valor; grava os ponteiros (calendario_carregar_ano), lê (calendario_dia)
- [0xae2a48] a lista das conferências das pernas (+4 fase, +5 rodada, +7/+0xb os clubes); valor; lê e esvazia (calendario_dia)
- [0x9cf684] a competição dos amistosos; id; lê (calendario_dia)
- copa +0x3c rodadas, +0x3e times, +0x45 clubes, +0xa3 rodadas (0x68 bytes: +7 tipo 0x8c, +0xd bits 0x200 pelo par e 0x400 vaga toda livre, +0x18, +0x1a confrontos, +0x1c times que entram, +0x1e primeiro time, +0x5b primeiro confronto), +0xa7 times (6 bytes: +0 clube, +5 marca), +0xab/+0xac, +0xad vetores dos confrontos, +0xb1 sem confrontos, +0x24 nação; valor e ponteiro; lê e grava (copa_arvore_dos_confrontos)
- confronto (0x35 bytes) +0 mandante, +2 visitante, +6 vaga na rodada de destino, +8 lado (1 mandante), +0xa/+0xc confrontos de origem, +0xe/+0x10 rodadas de origem, +0x12 rodada de destino, +0x14..+0x31, +0x32..+0x34; valor; grava (copa_arvore_dos_confrontos)
- competição +0xc fases, +0x30 última fase, +0x3a id (grupo pela 0x4b01d0), +0x43 copa, +0xab começou; valor e ponteiro; lê (competicao_primeira_fase_do_grupo, competicao_fase_atual)
- treino (TRAINING_MANAGER [0xde1f44]) +0x651d entradas de 6 bytes por ficha de jogador (+0 vaga, +1 semana, +3..+5 meios bytes do progresso, +5 marcas 0x10/0x20); ponteiro/valor; lê e grava (atualizar_o_treino_do_jogador)
- treino +0x688d semana do treino; valor; lê; +0x6521/+0x6525 lista do treino de posição (8 bytes: +0 id, +4 máscara, +6 nota, +7 taxa); ponteiro/valor; lê e grava
- treino + 0x9b × vaga programas (+4..+8 níveis, +0x15 tipo); os dos humanos em +0x4d8 + 0x60e × índice; valor; lê (programa_de_treino_do_jogador, nivel_da_categoria_do_programa)
- pessoa +0x39 clube, +0x58 e +0x5b (limites do progresso), +0x61 ficha (+0xf goleiro, +0x1b..+0x41 atributos pela 0x8b2be0); valor/ponteiro; lê
- contrato do clube +0x44 felicidade, +0x45 marca 0x8000; valor; lê e grava (a reclamação do treino de posição)
- [0xae1080] o último contrato consultado; ponteiro; grava; [[0xb63d28]] + 31 × id + 0x1e carga de treino; valor; grava
- treino (TRAINING_MANAGER [0xde1f44]) +0x651d entradas de 6 bytes por ficha (+3..+5 meios bytes do progresso); ponteiro/valor; lê (e grava pela 0x8b2be0)
- 0xde1f3c..0xde1f40 o progresso tirado (categorias 3, 2, 4, 0, 1); valor; grava e lê (tirar_ou_devolver_o_progresso)
- pessoa +0x56 adaptabilidade; valor; lê; ficha +0x27, +0x2a, +0x30, +0x32, +0x33, +0x35..+0x37, +0x39, +0x3b, +0x3e..+0x40 atributos; valor; lê (aptidao_para_a_posicao)
- regen (PLAYER_REGEN [0xb7434c]) +4/+8 aposentadorias pendentes (26 bytes: +0 pessoa, +4 clube, +8/+0xa anúncio, +0x10/+0x12 data, +0x18 motivo, +0x19 anunciada); ponteiro/valor; lê e grava (aposentadorias_do_dia)
- regen +0x10/+0x14 vagas para o regen (12 bytes: pessoa, dia, ano, bissexto 1); +0x1c/+0x20 reserva de fichas de não jogador; ponteiro/valor; lê e grava
- [0xae2d0c] + 79 × id +0xb marcas da aposentadoria (0x10 anunciada, 0x20 vaga, 0xf); valor; lê e grava
- pessoa +0x18 idade, +0x1a/+0x1e nações, +0x22 reputação, +0x39 clube, +0x3d cargo, +0x3e/+0x42 data de chegada, +0x52, +0x57, +0x5b, +0x5f, +0x61 ficha (+9, +0xb, +0xd, +0xf), +0x69 registro de não jogador (+8, +0xa, +0xc, +0x10, +0x11); valor/ponteiro; lê e grava
- clube +0x53 nação, +0x57 divisão, +0x7f força, +0x80 reputação, +0xd3 auxiliar, +0x19f treinadores; ponteiro/valor; lê (as notícias)
- [[0xde1f54] + 0x856] (+0xdc linhas de 9 bytes por clube, +8 = 100); ponteiro/valor; grava
- AWOL +0xc tabela de marcas (um byte por pessoa: bit 1 ausente, bit 2 faltou ao treino); valor; lê e grava (limpar_ausencia, limpar_falta_ao_treino)
- AWOL +0 lista das ausências, +4 lista das faltas (LLIST: +0 conta, +2 cabeça, +0xa cursor; registro +0 id da pessoa); ponteiro; lê e grava (limpar_ausencia, limpar_falta_ao_treino)
- pessoa +0 id, +4/+8/+0xc chaves (+0x33), +0x39 clube, +0x61 ficha de jogador (+0xb reputação); id/ponteiro/valor; lê (noticia_da_falta_ao_treino)
- clube +0xcf técnico, +0xd3 auxiliar, +0x19f os 5 treinadores, +0x53 nação; ponteiro; lê (noticia_da_falta_ao_treino)
- vaga da lista de compras (0x25 bytes) +3 verba, +0xb preço, +0x23 índice, +0x24 marcas (bit 0x20 sob oferta); valor; lê (troca_com_compensacao, ordem_das_vagas) e grava o bit (marcar_sob_oferta)
- contrato +0 id da pessoa, +0x49 byte que dispensa a conta; id/valor; lê (troca_com_compensacao)
- pessoa ([0xae23bc] + 110 × id) +0x52 valor; valor; lê (troca_com_compensacao)
- Shortlist_Manager +0xdc tabela dos clubes (9 bytes por clube; +0 as 40 vagas); ponteiro; lê (marcar_sob_oferta)
- clube +0x80 reputação; valor; lê e grava (campeao_da_liga_nos_registros, campeao_da_copa_nos_registros)
- clube +0x57, +0x73, +0x7b e o bloco +0x69 (+0x44); valor; lê e grava (campeao_da_liga_nos_registros)
- partida +0x188/+0x18a/+0x18c notas do lado (0x625 por lado), +0x190 escalação (20 × 0x49 bytes); valor; lê e grava (campeao_da_copa_nos_registros)
- registro de não jogador do técnico +8/+0xa/+0xc notas; valor; grava (campeao_da_liga_nos_registros)
- registro do técnico (0xd2 bytes, Staff_Records +8) +0x8a/+0x8c títulos, +0xb6/+0xba/+0xbe/+0xca pontos; valor; grava (as duas) e zera (limpar_registro_do_humano)
- contrato +0x44 moral, +0x4f situação (>> 4), +0x25 data; valor; lê e grava (as duas)
- competição +0x69 peso; objeto [0xadadfc] + 4 × id: +0x3c/+0x3e (limiar), +0x42 fase; valor; lê (as duas)

**Interessa a:** CMRTE, GPF2, SGE, CMScout
