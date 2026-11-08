# A permissão de trabalho concedida, o estatuto pelo motivo da recusa e a vaga da negociação (transfer_manager.cpp):
# TRANSFER_MANAGER::is_work_permit_granted (0x8c7950, ~1,2 KB: pela permissão na lista de observação 0x84b180 e pela
# oferta de contrato aceita 0x8c38b0, já em Rust, e pelas permissões da oferta 0x8c7690, original, a cada oferta que
# vence; "cpl_3 work permit" 64 e "cpr_1 permit found" 131072 na base da 261ª),
# transfer_manager_get_squad_status_for_reason (0x8c5800, só da tela da oferta: passageira) e
# TRANSFER_MANAGER::get_negotiation_slot (0x8c6200, da resposta automática às notícias e das telas: passageira).
# Iguais no cm0102-1080.exe; sem SEH. A 0x8c7950 tem x87 só para a máscara do histórico (fld qword [0x968950] = 2,0,
# fild k, pow 0x945ff0, _ftol 0x945640): 2^k de 0 a 15 é exato, a cópia usa 1 << k.
# Convenções conferidas nos pontos de chamada da original (lição da 258ª):
# - quem chama a 0x8c7950: 0x8c7737 push 0; push 0; push oferta; push [clube+0x53]; push pessoa; mov ecx, edi ->
#   thiscall(this, pessoa, nação, oferta, registro, sorteio), ret 0x14 (as cópias Rust da 0x84b180 e da 0x8c38b0 já a
#   chamam assim).
# - quem chama a 0x8c5800: 0x8f0849 push ebp (motivo); push esi (pessoa); mov ecx, [0xde1f54] -> thiscall(this sem uso,
#   pessoa, motivo), ret 8.
# - quem chama a 0x8c6200: 0x78394f mov ecx, [0xde1f54]; push i -> thiscall(this, i), ret 4.
# - dentro da 0x8c7950: 0x4cf5a0 mov ecx, [0xae1050]; push pessoa -> thiscall(gerência, pessoa), ret 4 (aqui sem
#   gravar [0xae1080]); 0x946464 push 0x8e55c0; push 0xe; push conta; push vetor; push chave; add esp, 0x14 ->
#   cdecl(chave, vetor, conta, 0xe, comparação); 0x456740 push [c+0x31]; push [c+0x2d]; lea ecx, [oferta+0x2d] ->
#   thiscall(data, d0, d1), ret 8; 0x945a60 push agulha; push texto; add esp, 8 -> cdecl (strstr); 0x90cfc0 push n;
#   add esp, 4 -> cdecl(n); 0x54cfb0 mov ecx, 0xae2c90 -> thiscall(data), ret 0 (os push antes dela são da 0x58d710);
#   0x58d710 mov ecx, [[0xb74318]+0x1c]; push ano; push mês; push ano − 2; push mês; push nação ->
#   thiscall(fifa, nação, mês_de, ano_de, mês_até, ano_até), ret 0x14.
# - dentro da 0x8c5800: 0x4cf5a0 como acima (grava [0xae1080]); 0x4cf5f0 mov ecx, [0xae1050]; push clube; push pessoa
#   -> thiscall(gerência, pessoa, clube), ret 8.
# Comparadores: a 0x8c7950 lado a lado nas marcas da pessoa (8 bytes em [this+0x86e] + id × 8: o histórico dos jogos da
# seleção anda), no sorteio e na devolução; a 0x8c5800 lado a lado em [0xae1080] e no al; a 0x8c6200 pelo valor.
SAVE = False
ROTINAS = [('permissao_concedida', 0x8c7950, [0x83, 0xec, 0x14, 0x53, 0x55]),
           ('estatuto_pelo_motivo', 0x8c5800, [0x8b, 0x0d, 0x50, 0x10, 0xae, 0x00]),
           ('vaga_da_negociacao', 0x8c6200, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x51, 0x10])]
DOCS = ('a permissão de trabalho concedida (o registro de permissão, o potencial, o salário, o jovem, os jogos pela seleção e o histórico de 16 jogos), o estatuto pelo motivo da recusa e a vaga da negociação',
        'the work permit granted (the permit record, potential, wage, youth, caps and the 16-match history), the squad status for a refusal reason and the negotiation slot',
        [('permissao_concedida', 'TRANSFER_MANAGER::is_work_permit_granted', '0x8c7950', 'transfer_manager.cpp', 'thiscall, ret 0x14',
          'decide a permissão: pelo registro de permissão (jogos e recusas), não jogador pela reputação, as exceções de 2001, potencial acima de 8500, salário acima de 34999, jovem acima de 190, menos de 5 jogos pela seleção não, mais de 20 com nação forte sim, e o histórico de 16 jogos da seleção (mais de 11 ou 70%), que anda pelos jogos novos',
          'decides the permit: by the permit record (matches and refusals), non-player by reputation, the 2001 exceptions, potential above 8500, wage above 34999, youth above 190, under 5 caps no, over 20 with a strong nation yes, and the 16-match national team history (over 11 or 70%), which advances with new matches'),
         ('estatuto_pelo_motivo', 'transfer_manager_get_squad_status_for_reason', '0x8c5800', 'transfer_manager.cpp', 'thiscall (this sem uso), ret 8',
          'o estatuto no elenco pelo motivo da recusa (0x200: 1, 0x400: 2, 0x2000: 5) ou o do contrato com o clube (+0x4f >> 4); sem contrato, 0',
          "the squad status for the refusal reason (0x200: 1, 0x400: 2, 0x2000: 5) or the one of the contract with the club (+0x4f >> 4); no contract, 0"),
         ('vaga_da_negociacao', 'TRANSFER_MANAGER::get_negotiation_slot', '0x8c6200', 'transfer_manager.cpp', 'thiscall, ret 4',
          'o endereço da vaga i do vetor de negociações; fora da conta, o começo do vetor',
          'the address of slot i of the negotiation array; out of range, the start of the array')])
CAMPOS = [('[this+0x86e] + id × 8: +0 bit 0x20 tem registro de permissão, +3 o histórico de 16 jogos da seleção (u16), +5 a conta dos jogos da seleção vistos (u16); valor; lê e grava (permissao_concedida)',
           '[this+0x86e] + id × 8: +0 bit 0x20 has a permit record, +3 the 16-match national team history (u16), +5 the count of national team matches seen (u16); value; reads and writes (permissao_concedida)'),
          ('[this+0x93a] / [this+0x93e]: os registros de permissão (14 bytes: +0 id, +4 jogos, +6 recusas) e a conta; ponteiro e valor; lê (permissao_concedida)',
           '[this+0x93a] / [this+0x93e]: the permit records (14 bytes: +0 id, +4 matches, +6 refusals) and their count; pointer and value; reads (permissao_concedida)'),
          ('pessoa +4 / +0x18 / +0x1a / +0x22 / +0x61 / +0x69: o nome, a idade, a nação, os jogos pela seleção, a ficha e a ficha de não jogador; ponteiro e valor; lê (permissao_concedida)',
           'person +4 / +0x18 / +0x1a / +0x22 / +0x61 / +0x69: the name, the age, the nation, the caps, the profile and the non-player profile; pointer and value; reads (permissao_concedida)'),
          ('ficha +7 / +0xd: a habilidade e o potencial; valor; lê (permissao_concedida)',
           'profile +7 / +0xd: the ability and the potential; value; reads (permissao_concedida)'),
          ('ficha de não jogador +0xc: a reputação; valor; lê (permissao_concedida)',
           'non-player profile +0xc: the reputation; value; reads (permissao_concedida)'),
          ('nação +0x8e: a reputação; [[0xb74318]+4] + id × 0x6e + 2: os jogos da seleção; valor; lê (permissao_concedida)',
           'nation +0x8e: the reputation; [[0xb74318]+4] + id × 0x6e + 2: the national team matches; value; reads (permissao_concedida)'),
          ('oferta de contrato +4 / +0xc / +0x2d: o clube, o salário e a data; id e valor; lê (permissao_concedida)',
           'contract offer +4 / +0xc / +0x2d: the club, the wage and the date; id and value; reads (permissao_concedida)'),
          ('contrato +0x4f: o estatuto no elenco (>> 4); valor; lê (estatuto_pelo_motivo)',
           'contract +0x4f: the squad status (>> 4); value; reads (estatuto_pelo_motivo)'),
          ('[this+0xc] / [this+0x10]: o vetor de negociações e a conta; ponteiro e valor; lê (vaga_da_negociacao)',
           '[this+0xc] / [this+0x10]: the negotiation array and its count; pointer and value; reads (vaga_da_negociacao)'),
          ('[0xae1080]: o último contrato consultado; ponteiro; grava (estatuto_pelo_motivo)',
           '[0xae1080]: the last contract looked up; pointer; writes (estatuto_pelo_motivo)')]
INTERESSA = ['CMRTE', 'SGE']
