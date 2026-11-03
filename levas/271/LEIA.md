# 271ª leva — o contrato assinado e posto no vetor dos contratos (a entrada no clube, o empréstimo com a duração a partir de hoje, o fim mínimo, o lugar no vetor com os termos antigos no mesmo clube, a vaga livre ou o fim do vetor, o valor na pessoa e a troca de clube); o regen jovem: a pessoa aposentada refeita como jogador novo do banco de fichas (nação, nomes, idiomas, idade, habilidade, reputações, personalidade) e posta num clube; a saída da disputa; a formação lida do arquivo .pct (as táticas da IA e as do humano) e a troca da ordem dos bytes de palavras e dwords; os árbitros do jogo novo: os que ficam por nação, as vagas das competições e a parte de cada uma; a ordem das vagas pelo nível da competição; o amistoso cancelado, o jogo da seleção que sai junto e o torneio cancelado; a ordem dos árbitros por nação e qualidade, nos dois sentidos; as estatísticas do jogador arquivadas no histórico (a troca de clube, o empréstimo, a aposentadoria) e os jogos e gols pelo clube recontados; o player_setup.cfg: a pessoa pelo nome (no clube ou em todas), o texto entre aspas da linha e o tipo da linha pelo nome

**Rotinas:**
- `assinar_contrato()` = 0x4cf720 (contract_manager.cpp, thiscall, ret 8): põe a pessoa no clube do contrato (ou cancela sem vaga), trata o empréstimo (elenco cheio, cargo 11, duração a partir de hoje), garante 4 semanas de contrato, grava o contrato no vetor (o índice antigo, a vaga livre ou o fim, que cresce 100), o valor em +0x4e da pessoa, a folha e a troca de clube;
- `refazer_como_jovem()` = 0x7abac0 (player_regen.cpp, thiscall): refaz a pessoa aposentada da fila dos regens como jogador jovem: a ficha da frente do banco de fichas, a nação (ou outra pequena), a segunda nação, os nomes e os idiomas da nação, a idade e a habilidade (pela reputação da nação pequena ou pela posição e por sorteio), as reputações e seus tetos, o nascimento, os atributos de personalidade pela idade e, com chance, o melhor clube ou um clube da nação;
- `tirar_da_disputa()` = 0x56a090 (dispute.cpp, thiscall): zera a marca de disputa da pessoa e tira a entrada dela da lista das disputas;
- `ler_formacao_do_disco()` = 0x5b5d10 (formation.cpp, thiscall): lê a formação de um arquivo .pct (o identificador e a versão, a senha invertida, o criador, as posições gerais e extras, as casas por fase e faixa, as instruções do time e dos jogadores, a marcação), acerta as instruções das versões antigas, refaz as posições das 11 vagas pelas casas e monta o mapa das casas;
- `trocar_bytes_das_palavras()` = 0x43f2d0 (cash.cpp, cdecl): troca os dois bytes de cada palavra de um vetor;
- `trocar_bytes_dos_dwords()` = 0x43f300 (cash.cpp, cdecl): inverte os quatro bytes de cada dword de um vetor;
- `montar_os_arbitros()` = 0x79bf60 (officials_manager.cpp, thiscall): no jogo novo: revê os árbitros, ordena por nação e qualidade, fica com 6 por nação (61 nas de clubes de base), monta os blocos por nação com uma vaga por liga (o pedido pela competição), sorteia os que faltam e reparte os árbitros da nação pelas vagas;
- `ordem_das_vagas_de_competicao()` = 0x79d410 (officials_manager.cpp, cdecl): ordena as vagas de árbitros de uma nação pelo nível da competição (as inválidas por último ou primeiro), empate pelo id;
- `cqg10_amistoso_cancelado()` = 0x5c6570 (friendly.cpp, thiscall, ret 8): amistoso desta competição: a notícia 8 aos técnicos dos dois clubes que não cancelaram e a vaga do amistoso livre; de outra competição, o torneio dela é cancelado; jogo sem rodada sai do calendário (o de seleção pela 0x5cb9e0);
- `cqg10_jogo_da_selecao_cancelado()` = 0x5cb9e0 (friendly.cpp, thiscall, ret 4): pela marca do jogo, a seleção mandante (ou a principal da nação dele): o jogo dela na véspera sai do calendário; depois o próprio jogo sai;
- `cqg10_torneio_cancelado()` = 0x5c8260 (friendly.cpp, thiscall, ret 8): a notícia 0xb aos convidados (torneio que começa depois de uma semana atrás); o torneio sai dos pendentes; os jogos da competição criada voltam a ser amistosos ou saem; a competição é destruída e a vaga dela e o dono zeram;
- `ordem_dos_arbitros_decrescente()` = 0x79d4e0 (officials_manager.cpp, cdecl): ordena os árbitros pela nação e, nela, pela qualidade (+0x28 × 50 + +0x22) da maior para a menor; empate pelo índice, decrescente;
- `ordem_dos_arbitros_crescente()` = 0x79d480 (officials_manager.cpp, cdecl): ordena os árbitros pela nação e pela qualidade da menor para a maior; empate pelo índice, crescente;
- `arquivar_estatisticas()` = 0x7c5fe0 (player_stats.cpp, thiscall): na saída do jogador de um clube: as estatísticas de cada categoria da temporada vão para o arquivo de histórico (com o ano, o clube, o valor e o tipo da saída) e são zeradas; um nó vazio quando falta a última categoria; e os jogos e gols pelo clube recontados do histórico;
- `pessoa_pelo_nome()` = 0x7a50f0 (player_regen.cpp, cdecl): acha a pessoa de uma linha do player_setup.cfg pelos três nomes: em todas as pessoas ou, com o nome do clube, no elenco (se pedido) e no técnico dele;
- `texto_entre_aspas()` = 0x7a5090 (player_regen.cpp, cdecl): copia o próximo texto entre aspas da linha (pulando espaços e quebras antes dele) e devolve onde continuar;
- `tipo_da_linha()` = 0x7a3140 (player_regen.cpp, cdecl): o bit do tipo de uma linha do player_setup.cfg (lesão, técnico, aposentadoria, transferência, empréstimo, doping, conduta, passaporte, táticas, daltonismo, idioma, fuga);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (21):** 21 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (8845ebc9).
- **Carreiras Rust ×2 e original até 2002-06-01:** Rust ×2 151bdf85, original 932f1944 (registrada; não barra a leva desde 31/10/2026). Detector de lixo: igual.

**Refazer**, sobre o commit da 270ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 271`

**Campos tocados:**
- gerência de contratos +0 vetor dos contratos de 0x50 bytes, +4 capacidade, +8 conta; ponteiro e valor; lê e grava (assinar_contrato)
- contrato +0 pessoa (−1 vaga livre), +4 clube, +0xc valor [inferência], +0x25 início, +0x2d fim, +0x35 tipo (6 bits; 5 empréstimo, 2) e bits, +0x3a, +0x3b, +0x3d tipo pelo cargo, +0x3e, +0x3f, +0x44 felicidade no contrato, +0x45, +0x49, +0x4a, +0x4e, +0x4f nibble alto do cargo; id e valor; lê e grava (assinar_contrato)
- pessoa +0x1a nacionalidade, +0x1e segunda nacionalidade, +0x39 clube, +0x3d cargo, +0x3e data (+0x40 ano), +0x46 fim do contrato, +0x4e valor do contrato [inferência], +0x61 ficha; ponteiro e valor; lê, grava +0x1e, +0x46 e +0x4e (assinar_contrato)
- [0xae2d0c] registro de 79 bytes (+0 índice do contrato); [0x9cf434] e [0x9cf260] nações fixas; [0xae23a8] nações (0x122 bytes); índice; lê e grava (assinar_contrato)
- PLAYER_REGEN +0x3c o banco de fichas (entradas de 16 bytes: +0 id da ficha, +4 nação, +8 segunda nação); valor; lê (refazer_como_jovem)
- [0xae2d0c] + 0x4f × id: +8 = 0, +9 = 0x9c, +0xb bits 0x30 aposentado (zerados), 0x40 a marca do regen (docs/MARCA_DO_REGEN.md), +0xc bits 0/1, +0xd = 0; valor; lê e grava
- pessoa +4/+8/+0xc nomes, +0x10 dia e +0x12 ano do nascimento, +0x18 idade, +0x1a nação, +0x1e segunda nacionalidade, +0x22 jogos pela seleção (0), +0x23 (0), +0x24, +0x39 clube, +0x3e/+0x40 a data de entrada, +0x4e, +0x52, +0x56, +0x5f tipo (2 jogador), +0x61 ficha, +0x69 ficha de não jogador (0), +0x28/+0x3d = 11; valor/ponteiro; grava
- ficha do jogador (0x46 bytes em [0xae23c4]) +5 habilidade (CA), +7 PA, +9/+0xb/+0xd reputações [inferência: atual, local, mundial], +0xf/+0x10/+0x11/+0x12/+0x13/+0x14/+0x15/+0x19 posições, +0x1b, +0x1d, +0x1e, +0x21, +0x24, +0x2c, +0x2e, +0x2f, +0x34, +0x36, +0x3c, +0x3d, +0x44 atributos [inferência: os de personalidade e os mentais]; valor; grava
- nação +0x71 continente, +0x88 reputação, +0x11c a marca das nações com clubes de base [inferência]; valor; lê; [0x9cf488]/[0x9cf424] as chaves da Espanha e do País Basco; id; lê
- DISPUTE ([0xae2a30]) +0 a lista das disputas (LLIST, +0 conta, +2/+0xa cursor), +4 a tabela de marcas por id; valor; lê e grava (tirar_da_disputa)
- formação +0 o nome (do arquivo), +0x33 senha (50, invertida), +0x65 criador (200), +0x12d posições gerais (11 palavras), +0x143 ordem (11), +0x14e posições extras (11 palavras), +0x164 casas (11 vagas × 2 fases × 3 × 4 faixas, palavras coluna e linha), +0x584 instruções do time (0x2000, 0x4000, 0x20000), +0x5b4 marcação (11), +0x187b máscara, +0x187f/+0x1883/+0x1887 marcas, +0x188b instruções dos jogadores (11 × 8); valor; grava (ler_formacao_do_disco)
- árbitro (0x2b bytes em [0xae23e8], conta [0xae2390]): +0 índice, +0x16 nação, +0x22/+0x28 a qualidade na ordem; valor/ponteiro; lê e grava (montar_os_arbitros)
- OfficialsManager +4 conta, +8 marcas (calloc), +0xc blocos por nação (0x102 bytes: +0 nação, +4 árbitros, +8 primeiro índice, +0xc pedido, +0x10 competições, +0x12 15 vagas de 16 bytes: +0 início, +4 parte, +8 pedido, +0xc competição); valor; grava
- nação +0x8e a quantidade padrão de árbitros (1..5; 0 vira 1), +0x11c; valor; lê e grava; competição (0x6b bytes em [0xae23d0]) +0x5d nação, +0x69 nível; competição de clube ([0xadadfc][k]) +0x3e o pedido de árbitros [inferência]; valor; lê
- Friendly +4 a competição dos amistosos, +0xaf índice da primeira competição de torneio, +0xb1 os torneios (0x69 bytes), +0x34cc vagas dos amistosos, +0x34d0 lista dos torneios pendentes; ponteiro/valor; lê e grava (as três)
- torneio +0 dono, +4 começo (dia, ano), +0x15 os 4 convidados, +0x25 respostas (1 recusou), +0x64 índice da competição, +0x65 a competição criada; ponteiro/valor; lê e grava (cqg10_torneio_cancelado)
- jogo +0 chave da competição, +0x14 competição, +0x1c/+0x20 clubes, +0x28 ano, +0x2a dia, +0x42/+0x43 rodada, +0x4d marca (0x100/0x200 seleção); valor/ponteiro; lê e grava (as três)
- [0xadadfc] tabela das competições; ponteiro; grava 0 (cqg10_torneio_cancelado)
- árbitro +0 índice, +0x16 nação (+0 id), +0x22 nota, +0x28 atributo [inferência: os dois da qualidade na ordem]; valor/ponteiro; lê
- PlayerStats ([0xb74350]) +4 registros de 32 bytes das categorias (zerados), +0x30 registros de 39 bytes por ficha (+0..+0x14 as seis categorias, −1 sem, +0x18/+0x1c o primeiro e o último nó do histórico, +0x20 jogos e +0x22 gols pelo clube); valor; lê e grava (arquivar_estatisticas)
- nó do histórico (0x2b): +0 ano, +2 clube, +6 os 32 bytes da categoria (+6 jogos, +8 gols), +0x26 valor da transferência, +0x2a categoria × 8 + tipo (0 transferência, 1 fim, 2 empréstimo, 3 outro); valor; grava no arquivo
- pessoa +0x39 clube, +0x3e/+0x42 a data de entrada no clube, +0x61 ficha; [0xb63d8a] + 0x48 × linha da nação o começo (+0/+2) e o fim (+0xf/+0x11) da temporada; valor; lê
- pessoa ([0xae23bc], 0x6e bytes) +4 primeiro nome, +8 segundo nome, +0xc nome comum (registros de nome que começam pelo texto); ponteiro; lê (pessoa_pelo_nome)
- clube ([0xae23b4], 0x245 bytes) o nome (0x545880), +0xcf técnico, +0xd7 elenco (50); ponteiro; lê

**Interessa a:** CMRTE, SGE, CMScout, GPF2
