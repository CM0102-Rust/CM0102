# As conversas de contrato e os ajudantes da oferta das telas de contrato (contract_manager.cpp; leitor rx07, rodada 6,
# 25/11/2026). Item de telas (TELAS): só as telas de contrato (contract_screens.cpp, já Rust, pelo endereço) os chamam;
# a carreira automática não conversa nem oferece contrato (contagem 0 em todas). Iguais no cm0102-1080.exe.
# Convenções conferidas na desmontagem e nos chamadores:
# - 0x4dc970 compare_talks_records cdecl(a, b) -> int: nulo primeiro; +0, +4, +8 como diferença com sinal. Usado pelo
#   qsort/bsearch das conversas (0x4c9f60, 0x4ca2a0 e o dia dos contratos, Rust, pelo endereço). Prólogo mov edx,
#   [esp+4]; push esi (5).
# - 0x4ca2a0 CONTRACT_MANAGER::find_talks_record thiscall(pessoa, técnico, clube) ret 0xc -> registro ou 0 (bsearch
#   0x946464 com a chave pessoa, clube, técnico; sem vetor, devolve eax = 0 do teste). Prólogo sub esp, 0x18; push esi;
#   mov esi, ecx (6). A Date local (0x54a110) não é lida.
# - 0x4c9f60 CONTRACT_MANAGER::add_talks_record thiscall(pessoa, técnico, clube, papel) ret 0x10: o registro de 21
#   bytes (+0 pessoa, +4 clube, +8 técnico, +0xc a data de hoje, +0x14 o papel); malloc(0x1a4) e realloc 0x945667 de 20
#   em 20, sem memória a caixa fatal (linhas 0x5b1 e 0x5bc: [0xb67a34] = 0, 0x5cc7d0, 0x5a8f60, exit(-1)); qsort
#   0x945333. Prólogo sub esp, 0x220 (6). A capacidade fica em +0x30.
# - 0x4d44e0 contract_manager_get_offer_job thiscall(pessoa) ret 4 -> al; non_player_attribute_rating 0x544410
#   cdecl(ficha, byte do atributo); staff_can_take_job 0x4cb990 stdcall(cargo, pessoa) ret 8 com o ecx do gerente
#   (Rust; a cópia chama a função Rust). Prólogo push esi; mov esi, [esp+8] (5).
# - 0x4d4660 CONTRACT_MANAGER::will_negotiate_offer thiscall(proposta) ret 4 -> 0/1;
#   staff_is_transfer_listed_or_requested 0x544520 cdecl(pessoa, 0); STAFF_CONTRACT::wants_to_leave 0x866f60
#   thiscall(contrato); Date::sub_days 0x54c8e0 thiscall(G_CURRENT_DATE, &tmp, 150) ret 8. Prólogo sub esp, 0x10; push
#   ebx; mov ebx, [esp+0x18] (8). Chamada por 0x4de5d0, 0x4e06e0 e 0x4e30f0 (contract_screens).
# - 0x4d4830 CONTRACT_MANAGER::prefill_offer_from_pending thiscall(proposta) ret 4 (o eax da volta não é lido):
#   STAFF_CONTRACT::calc_contract_value 0x866830 thiscall(proposta, pessoa, 0) -> int; o alvo guardado em f32; as
#   constantes 0.97 (0x9690a0), 1.03 (0x969098), 10.0f (0x968a78) e 0.5f (0x968f18); _ftol 0x945640. O cursor da lista
#   de propostas fica depois da proposta achada (a cópia anda com o get_next, como a original). Prólogo igual ao da
#   0x4d4660 (8). Chamada por 0x4de5d0.
# Comparadores: compare_talks_records, find_talks_record, get_offer_job e will_negotiate_offer lado a lado (só leem);
# add_talks_record só a original e o registro conferido depois; prefill_offer_from_pending lado a lado (a cópia numa
# cópia da proposta, 0x6f bytes).
# Caminho no jogo (para o Lucas): o perfil de um jogador de outro clube ou do próprio -> Contrato -> Oferecer contrato
# (a tela do papel no clube usa find_talks_record e get_offer_job; Definir grava a conversa); a renovação de um
# jogador do próprio clube (will_negotiate_offer); a oferta de contrato depois de uma proposta de transferência aceita
# com uma oferta pendente do mesmo clube (prefill_offer_from_pending).
TELAS = True
ROTINAS = [('compare_talks_records', 0x4dc970, [0x8b, 0x54, 0x24, 0x04, 0x56]),
           ('find_talks_record', 0x4ca2a0, [0x83, 0xec, 0x18, 0x56, 0x8b, 0xf1]),
           ('add_talks_record', 0x4c9f60, [0x81, 0xec, 0x20, 0x02, 0x00, 0x00]),
           ('get_offer_job', 0x4d44e0, [0x56, 0x8b, 0x74, 0x24, 0x08]),
           ('will_negotiate_offer', 0x4d4660, [0x83, 0xec, 0x10, 0x53, 0x8b, 0x5c, 0x24, 0x18]),
           ('prefill_offer_from_pending', 0x4d4830, [0x83, 0xec, 0x10, 0x53, 0x8b, 0x5c, 0x24, 0x18])]
_F = 'contract_manager.cpp'
DOCS = ('as conversas de contrato e os ajudantes da oferta',
        'the contract talks and the offer helpers',
        [('compare_talks_records', 'compare_talks_records', '0x4dc970', _F, 'cdecl',
          'ordena os registros de conversa: o nulo primeiro, depois pela pessoa, pelo clube e pelo técnico',
          'orders the talks records: a null one first, then by person, club and manager'),
         ('find_talks_record', 'CONTRACT_MANAGER::find_talks_record', '0x4ca2a0', _F, 'thiscall, ret 0xc',
          'acha, por busca binária, o registro da conversa de um técnico de um clube com uma pessoa; 0 sem registro',
          "finds, by binary search, the record of a club manager's talks with a person; 0 without one"),
         ('add_talks_record', 'CONTRACT_MANAGER::add_talks_record', '0x4c9f60', _F, 'thiscall, ret 0x10',
          'registra que o técnico de um clube conversou com uma pessoa sobre um papel: sem registro dos três, o vetor cresce de 20 em 20 (sem memória, o jogo para), o registro novo leva os três, a data de hoje e o papel, e os registros são reordenados',
          'records that a club manager talked to a person about a role: without a record of the three, the array grows by 20 (out of memory, the game stops), the new record takes the three, today\'s date and the role, and the records are sorted again'),
         ('get_offer_job', 'contract_manager_get_offer_job', '0x4d44e0', _F, 'thiscall, ret 4',
          'o cargo que a oferta de contrato nomeia: jogador, 15 (jogador-treinador) com treino ou treino de goleiros 10 ou mais, senão 11; os outros: 9 sem ficha de não jogador, 6 com o cargo no clube ou na seleção 5 ou 12, o cargo no clube, o da seleção ou de novo o do clube quando ele pode assumi-lo, senão 8 com treino 15 ou mais, senão 9',
          'the job a contract offer names: a player, 15 (player-coach) with coaching or goalkeeper coaching of 10 or more, else 11; anyone else: 9 without a non-player sheet, 6 with a club or national job of 5 or 12, his club job, national job or club job again when he can take it, else 8 with coaching of 15 or more, else 9'),
         ('will_negotiate_offer', 'CONTRACT_MANAGER::will_negotiate_offer', '0x4d4660', _F, 'thiscall, ret 4',
          'se a pessoa aceita conversar sobre um contrato novo com o próprio clube: sim sem contrato principal, com a queixa 2, com proposta de outro clube ou para quem não é jogador; não com tipo de contrato 0, saindo pela Lei Bosman, listado ou pedindo transferência, ou com situação 7 no elenco; abaixo de 25 anos, sim sem marcas nem queixas; não se quer sair ou com o bit 8 da situação de transferência; acima de 23, pouco deixado de fora, infeliz (abaixo de -50) e com um técnico há mais de 150 dias no clube: não; senão sim',
          'whether the person will talk about a new contract with his own club: yes without a main contract, with grievance 2, with an offer from another club or for a non-player; no with contract type 0, leaving on a Bosman, listed or asking for a transfer, or with squad status 7; under 25, yes without marks or grievances; no when he wants to leave or with bit 8 of the transfer status; over 23, rarely left out, unhappy (under -50) and with a manager at the club for over 150 days: no; otherwise yes'),
         ('prefill_offer_from_pending', 'CONTRACT_MANAGER::prefill_offer_from_pending', '0x4d4830', _F, 'thiscall, ret 4',
          'uma oferta nova copia os bônus, as cláusulas, o início e o fim da oferta pendente (estado 3) da pessoa com o mesmo clube; depois o salário anda metade da diferença até o valor do contrato ficar entre 0,97 e 1,03 do anterior ou o passo ficar em 10 ou menos',
          'a new offer copies the bonuses, the clauses, the start and the end of the person\'s pending offer (state 3) with the same club; then the wage moves half the difference until the contract value is within 0.97 to 1.03 of the earlier one or the step is 10 or less')])
CAMPOS = []
INTERESSA = []
