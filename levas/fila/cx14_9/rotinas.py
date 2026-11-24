# A cadeia de acesso e rebaixamento da Allsvenskan, a subida dos clubes regionais quando a Segunda Divisao sueca nao e
# jogada e os codigos de progresso da Allsvenskan (swe_prm.cpp; leitor cx14, rodada 5, 24/11/2026). Simulacao com a
# Suecia (as ligas e a data dos itens mwe): a cadeia e a subida na virada sueca do fim de 2001 (1 chamada cada na
# cobertura do mundo, pelo swe_prm_start_new_season do mwe_14), o progresso no fim da temporada.
# - 0x8907f0 SwePrm::process_promotion_relegation_chain thiscall(a) ret 4: as posicoes finais (virtual +0xa4) da
#   Allsvenskan e da Superettan, League::promote_and_relegate 0x689c80 (this, de cima, de baixo, 1, 1, -1, -1; ret
#   0x18); com a Segunda Divisao (0x9cf7c8, so com a base NORMAL ou maior) as posicoes dela e as trocas da Superettan
#   com ela e com as cinco fases das secoes. Sem ela devolve o ponteiro da tabela das competicoes (o eax do original).
# - 0x890980 swe_prm_promote_regional_clubs_to_first_division thiscall() ret 0: malloc 0x120 (caixa 0x224); os clubes
#   com divisao e nacao (+0x53) das seis secoes (0x9cf5f4..0x9cf608), ate 72; 0x4afbd0 (cdecl(lista, n, 3)); os 3
#   primeiros para a Superettan (League::move_club_to_division 0x6830b0, thiscall(clube, registro, 1)); as linhas de
#   status 3 da Superettan para a secao Norte (League::relegate_club_to_division 0x6831a0); free. Com menos de 3
#   clubes o original move restos da lista (defeito; nao acontece com os dados do jogo).
# - 0x8916b0 SwePrm::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14: o melhor 11/12 -> 10, o pior
#   11/12 -> 13; a fase nao e lida.
# Prologos: mov eax, [0xadadfc] (5 bytes); sub esp, 0x208 (6); mov eax, [esp+4]; mov ecx, [ecx+8] (7). Iguais no
# cm0102-1080.exe (inteiras). Comparadores: a cadeia so a original; a subida so a original contra o plano (3 clubes na
# Superettan e os de status 3 na secao Norte); o progresso lado a lado.
SAVE = False
LIGAS = 'ENG SWE FIN NOR POR'
ATE = '2002-08-01'
ROTINAS = [('swe_prm_process_promotion_relegation_chain', 0x8907f0, [0xa1, 0xfc, 0xad, 0xad, 0x00]),
           ('swe_prm_promote_regional_clubs_to_first_division', 0x890980, [0x81, 0xec, 0x08, 0x02, 0x00, 0x00]),
           ('swe_prm_set_club_progress', 0x8916b0, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08])]
DOCS = ('a cadeia de acesso e rebaixamento da Allsvenskan, a subida dos clubes regionais e o progresso da Allsvenskan',
        'the Allsvenskan promotion and relegation chain, the regional clubs going up and the Allsvenskan progress',
        [('swe_prm_process_promotion_relegation_chain', 'SwePrm::process_promotion_relegation_chain', '0x8907f0', 'swe_prm.cpp', 'thiscall, ret 4',
          'regra: na virada sueca, as posições finais da Allsvenskan e da Superettan e a troca de clubes entre elas; com a Segunda Divisão (só com a base maior), também as posições dela e as trocas da Superettan com ela e com as suas cinco seções',
          'rule: at the Swedish turnover, the final positions of the Allsvenskan and the Superettan and the exchange of clubs between them; with the Second Division (only with the larger database), also its positions and the exchanges of the Superettan with it and with its five sections'),
         ('swe_prm_promote_regional_clubs_to_first_division', 'swe_prm_promote_regional_clubs_to_first_division', '0x890980', 'swe_prm.cpp', 'thiscall, ret 0',
          'regra: quando a Segunda Divisão sueca não é jogada, três clubes das seções regionais (sorteados entre os melhores) sobem para a Superettan e os rebaixados da Superettan descem para a seção Norte',
          'rule: when the Swedish Second Division is not played, three clubs of the regional sections (drawn among the best) go up into the Superettan and the Superettan\'s relegated clubs go down to the North section'),
         ('swe_prm_set_club_progress', 'SwePrm::set_club_progress', '0x8916b0', 'swe_prm.cpp', 'thiscall, ret 0x14',
          'regra: os códigos de progresso da Allsvenskan: o atual como veio; o melhor 11 ou 12 vira 10 e o pior 11 ou 12 vira 13 (os códigos do mata-mata)',
          'rule: the Allsvenskan progress codes: the current one as it came; a best code 11 or 12 becomes 10 and a worst code 11 or 12 becomes 13 (the play-off codes)')])
CAMPOS = []
INTERESSA = []
