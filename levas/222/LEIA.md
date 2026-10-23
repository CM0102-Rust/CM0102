# 222ª leva — o alcance da reputação, a reputação vista por alguém e se a pessoa é estrangeira; a idade da pessoa, se a nação tem transferência livre e se a pessoa é da UE; se a pessoa está na lista de transferências e os atributos de jogador de 1 a 20

**Rotinas:**
- `escopo_da_reputacao()` = 0x5408c0 (Database.cpp, cdecl): de que distância quem vê olha a pessoa (0 mesmo clube ou liga, 2 mesma nação, 3 nação do clube ou reputação alta, 4 mundo): escolhe a reputação;
- `reputacao_vista_por()` = 0x5407e0 (Database.cpp, cdecl): a reputação da pessoa vista por alguém: atual, nacional, mundial ou uma média, pelo alcance; de jogador ou de não jogador;
- `pessoa_e_estrangeira()` = 0x53d980 (Database.cpp, cdecl): se a pessoa é estrangeira na nação: nacionalidades, grupos (ilhas britânicas, Austrália, [0x9cf360]) e a regra de Portugal para contratos até 1998;
- `idade_da_pessoa()` = 0x53ab90 (Database.cpp, cdecl): a idade da pessoa em anos completos numa data, pelo nascimento;
- `nacao_com_transferencia_livre()` = 0x544670 (Database.cpp, cdecl): se a nação tem transferência livre: regras novas da FIFA, Brasil, ou Europa fora de Polônia, Turquia, Croácia e Rússia;
- `pessoa_e_comunitaria()` = 0x53d740 (Database.cpp, cdecl): se a pessoa conta como da UE para um clube da nação: marca de UE da nacionalidade, Suíça e as exceções da Alemanha, [0x9cf360] e Dinamarca;
- `pessoa_listada_para_transferencia()` = 0x544520 (Database.cpp, cdecl): se o vínculo da pessoa com o clube (ou o do contrato) está na lista de transferências, pelo clube ou a pedido;
- `atributo_calculado_do_jogador()` = 0x543540 (Database.cpp, cdecl): um atributo de jogador (1 a 31) de 1 a 20: o byte do registro ou, nos relativos, a nota de 2 × byte + habilidade atual;
- `atributo_da_pessoa()` = 0x5431e0 (Database.cpp, cdecl): um atributo de jogador da pessoa, pelos dados de jogador dela (0x543540);

**Como foi comparada:** comparação de verdade em cada rotina (o original e o Rust a partir do mesmo estado, com as regiões que a rotina toca).

**Resultado** (`cadeia.txt`):
- **Mutações (9):** 9 acusadas.
- **Jogo real comparando até 2002-01-01:** 0 divergências (963cea289f055ac3).
- **Carreiras Rust ×2 e original até 2002-06-01:** as três iguais (b28542870027e432).

**Refazer**, sobre o commit da 221ª:
1. `python integrar.py`
2. `cargo build --release` e copiar a DLL para `jogo_testes/`
3. `bash engenharia-reversa/cadeia.sh 222`

**Campos tocados:**
- pessoa +0x39: clube do contrato; ponteiro; lê (escopo_da_reputacao)
- clube +0x53: nação; ponteiro; lê (escopo_da_reputacao)
- clube +0x80 [2]: reputação; valor; lê (escopo_da_reputacao: + reputação atual do jogador ≥ 14000)
- pessoa +0x1a / +0x1e: nacionalidade e segunda nacionalidade; ponteiro; lê (escopo_da_reputacao, pessoa_e_estrangeira)
- pessoa +0x00: id (os 16 últimos são os técnicos humanos); id; lê (escopo_da_reputacao)
- pessoa +0x61: dados de jogador; ponteiro; lê (escopo_da_reputacao, reputacao_vista_por)
- pessoa +0x69: dados de não jogador; ponteiro; lê (reputacao_vista_por)
- jogador +0x09 / +0x0b / +0x0d [2]: reputação atual, nacional e mundial; valor; lê (escopo_da_reputacao, reputacao_vista_por)
- não jogador +0x08 / +0x0a / +0x0c [2]: reputação atual, nacional e mundial; valor; lê (reputacao_vista_por)
- nação +0x00: id; id; lê (pessoa_e_estrangeira: comparado com [0x9cf2e4] Inglaterra, [0x9cf45c] Escócia, [0x9cf510] País de Gales, [0x9cf3e4] Irlanda do Norte, [0x9cf2dc] Irlanda, [0x9cf224] Austrália, [0x9cf3f4], [0x9cf360], [0x9cf4f0] e [0x9cf434] Portugal)
- contrato (0x4cf5a0) +0x25 / +0x27 [2]: dia do ano e ano do início; valor; lê (pessoa_e_estrangeira, regra de Portugal: até 1/1/1998)
- [0xae2364] quantidade de pessoas; valor; lê (escopo_da_reputacao)
- pessoa +0x10 / +0x12 [2]: nascimento (dia do ano e ano); valor; lê (idade_da_pessoa)
- pessoa +0x1a / +0x1e: nacionalidade e segunda nacionalidade; ponteiro; lê (pessoa_e_comunitaria)
- nação +0x7f: marca de UE (2 = UE); valor; lê (pessoa_e_comunitaria)
- nação +0x71: continente; ponteiro; lê (nacao_com_transferencia_livre, pessoa_e_comunitaria)
- nação +0x00: id; id; lê (comparado com [0x9cf260] Brasil, [0x9cf430] Polônia, [0x9cf4dc] Turquia, [0x9cf2ac] Croácia, [0x9cf448] Rússia, [0x9cf4ac] Suíça, [0x9cf314] Alemanha, [0x9cf360], [0x9cf2c0] Dinamarca, [0x9cf348], [0x9cf408] Noruega, [0x9cf3a0])
- continente +0x00: id (Europa = [0x9cfa10]); id; lê (nacao_com_transferencia_livre)
- [0xae233c] regras novas da FIFA (1 = transferência livre em toda parte); valor; lê (nacao_com_transferencia_livre)
- [0xae23a4] tabela de continentes (198 bytes); ponteiro; lê (pessoa_e_comunitaria)
- pessoa +0x61: dados de jogador; ponteiro; lê (atributo_da_pessoa)
- pessoa +0x58: atributo 10 (byte direto); valor; lê (atributo_calculado_do_jogador)
- jogador +0x05 [2]: habilidade atual; valor; lê (atributo_calculado_do_jogador)
- jogador +0x0f: goleiro (1..20; = 20 ou < 15 muda a fórmula); valor; lê (atributo_calculado_do_jogador)
- jogador +0x1b..+0x20, +0x28, +0x29, +0x2e, +0x2f, +0x36, +0x3c, +0x3d, +0x3f, +0x40, +0x44: atributos absolutos (aceleração, agilidade, ..., disposição); valor; lê (atributo_calculado_do_jogador)
- jogador +0x1e, +0x23, +0x24, +0x26, +0x27, +0x2a, +0x2b, +0x31, +0x32, +0x33, +0x37, +0x39, +0x3a, +0x3e, +0x43: atributos relativos (antecipação, ..., desarme, criatividade; nota com a habilidade atual); valor; lê (atributo_calculado_do_jogador)
- contrato (0x4cf5a0) +4: id do clube; id; lê (pessoa_listada_para_transferencia)
- vínculo pessoa-clube (0x4cf5f0) +0x4e: marcas (1 lista de transferências, 2 lista de empréstimos, 8 a pedido); valor; lê (pessoa_listada_para_transferencia)
- [0xae1080] último contrato consultado; ponteiro; grava (pessoa_listada_para_transferencia, sem clube)

**Interessa a:** CMScout, CMRTE
