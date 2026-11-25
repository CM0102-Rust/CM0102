# O texto do progresso nas eliminatórias africanas e os destrutores escalares das competições reserva, sub-21 e B
# (wc_african_cup.cpp, record_utils.cpp; leitor rx11, rodada 6, 25/11/2026). Item de TELAS:
# - 0x910260 WcAfricanCup::write_progress_text thiscall(buf, tamanho, rotulo, rodada, situacao, competicao) ret 0x18
#   (virtual +0x38): tamanho < 30, caixa 0x307; rótulo 0x96: "Qualified For World Cup" (situação 1, o gênero +0x37 da
#   competição e a cadeia vazia 0xad9c64) ou "Final"; 0x407..0x40b e 0x472: situação 7 "Qualified For World Cup", 6
#   "Qualified For Final", outra "Second Round"; senão a rodada 10 "First Round". Caminho no jogo: a ficha de uma seleção
#   africana (o histórico das competições) durante as eliminatórias de 2006, ou a tela da competição. Prólogo mov eax,
#   [esp+8]; sub esp, 0x200 (10 bytes).
# - 0x7e6130 ReserveComp, 0x7e6320 Under21IntComp e 0x7e6350 BIntComp ::scalar_deleting_destructor thiscall(flags) ret
#   4: o destrutor da classe (0x7e6150, 0x7e6340, 0x7e6370, hooks do sx18_29) e, com o bit 0, operator delete 0x944c94.
#   Rodam quando o jogo é desmontado; caminho: com um jogo aberto, carregar outro save. Prólogo push esi; mov esi, ecx;
#   call rel32 (8 bytes, o call na posição 3): entram pelas RELATIVAS (INTEGRAR_ANTES.txt).
# Usa o qualifying_progress_sprintf e o compare_qualifying_progress_text do cx16_5 e o delete_through do cx03_3:
# integrar depois deles. Iguais no cm0102-1080.exe. Comparadores: o texto lado a lado num buffer local; os destrutores só
# a original (liberam o objeto).
SAVE = False
TELAS = True
ROTINAS = [('wc_african_cup_write_progress_text', 0x910260, [0x8b, 0x44, 0x24, 0x08, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('delete_reserve_comp', 0x7e6130, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_under21_int_comp', 0x7e6320, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00]),
           ('delete_b_int_comp', 0x7e6350, [0x56, 0x8b, 0xf1, 0xe8, 0x18, 0x00, 0x00, 0x00])]
DOCS = ('o texto do progresso nas eliminatórias africanas e os destrutores escalares das competições reserva, sub-21 e B',
        'the African qualifying progress text and the reserve, Under 21 and B competitions\' scalar deleting destructors',
        [('wc_african_cup_write_progress_text', 'WcAfricanCup::write_progress_text', '0x910260', 'wc_african_cup.cpp', 'thiscall, ret 0x18',
          'regra: o progresso de uma seleção nas eliminatórias africanas diz "Qualified For World Cup" ou "Final" no grupo final, "Qualified For World Cup", "Qualified For Final" ou "Second Round" nos grupos e "First Round" na primeira fase',
          'rule: a nation\'s African qualifying progress reads "Qualified For World Cup" or "Final" in the final group, "Qualified For World Cup", "Qualified For Final" or "Second Round" in the groups, and "First Round" in the first round'),
         ('delete_reserve_comp', 'ReserveComp::scalar_deleting_destructor', '0x7e6130', 'record_utils.cpp', 'thiscall, ret 4',
          'regra: apagar a competição reserva roda o destrutor dela e libera o objeto quando pedido',
          'rule: deleting the reserve competition runs its destructor and frees the object when asked'),
         ('delete_under21_int_comp', 'Under21IntComp::scalar_deleting_destructor', '0x7e6320', 'record_utils.cpp', 'thiscall, ret 4',
          'regra: apagar a competição internacional sub-21 roda o destrutor dela e libera o objeto quando pedido',
          'rule: deleting the Under 21 international competition runs its destructor and frees the object when asked'),
         ('delete_b_int_comp', 'BIntComp::scalar_deleting_destructor', '0x7e6350', 'record_utils.cpp', 'thiscall, ret 4',
          'regra: apagar a competição internacional B roda o destrutor dela e libera o objeto quando pedido',
          'rule: deleting the B international competition runs its destructor and frees the object when asked')])
CAMPOS = []
INTERESSA = []
