# Os registros dos tecnicos, o texto da noticia do destino e as posicoes finais da segunda divisao turca categoria B
# (tur_second_b.cpp; leitor cx14, rodada 5, 24/11/2026). Simulacao com a Turquia (as ligas e a data dos itens mwd).
# - 0x9012d0 TurSecondB::update_staff_records_for_status thiscall(clube, status, fase, a4, a5, a6) ret 0x18: 46
#   chamadas na cobertura do mundo. O grupo do clube pela divisao reserva (+0x60) contra as chaves 0x9cf8e4..0x9cf8f4
#   (G1 = a propria tabela, G2..G5 = as fases 0..3); League::find_club_row 0x68ab70 (thiscall(clube, 0), ret 8);
#   Staff_Records 0x86bdd0 (clube, comp, 0x458, 0, 0x28), 0x869480 (clube, comp, 0x64) e 0x86a1c0 (clube, comp),
#   thiscall sobre [0xdd7f00]. As caixas das linhas 0x3d4, 0x400 e 0x42a (grupo nenhum) devolvem -1.
# - 0x9015f0 TurSecondB::build_fate_news_text thiscall(10 argumentos) ret 0x28: os pushes do texto comum conferidos um
#   a um (a7 sempre 0; na fase 4 com noticia a fase vai como a constante 4). 0 chamadas na cobertura do mundo (so
#   com clube humano na categoria B): so o comparador a prova, se rodar.
# - 0x901ba0 TurSecondB::record_final_positions thiscall() ret 0: os 2 primeiros de cada grupo da primeira fase (-1..3)
#   com 1 e 2; os clubes de cada grupo de classificacao (fases 5..9) com 3 em diante; sem conferir a divisao do clube.
# Prologos: mov al, [esp+0xc]; sub esp, 0x200 (10 bytes); mov eax, [esp+0xc]; push esi (5 bytes); push ecx; push ebx;
# push ebp; push esi; push edi (5 bytes). Iguais no cm0102-1080.exe (inteiras). Comparadores: os registros so a
# original (o plano: o resultado e o status da linha); o texto lado a lado (compare_text_writer; o texto comum so a
# original); as posicoes finais lado a lado (+0x5b..+0x5f de cada clube).
SAVE = False
LIGAS = 'ENG POL TUR GRE HRV RUS'
ATE = '2002-08-01'
ROTINAS = [('tur_second_b_update_staff_records_for_status', 0x9012d0, [0x8a, 0x44, 0x24, 0x0c, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00]),
           ('tur_second_b_build_fate_news_text', 0x9015f0, [0x8b, 0x44, 0x24, 0x0c, 0x56]),
           ('tur_second_b_record_final_positions', 0x901ba0, [0x51, 0x53, 0x55, 0x56, 0x57])]
DOCS = ('os registros dos técnicos, a notícia do destino e as posições finais da segunda divisão turca categoria B',
        'the staff records, the fate news text and the final positions of the Turkish second division category B',
        [('tur_second_b_update_staff_records_for_status', 'TurSecondB::update_staff_records_for_status', '0x9012d0', 'tur_second_b.cpp', 'thiscall, ret 0x18',
          'regra: na primeira fase o clube que passa (status 1) ganha o registro de classificado; no grupo de acesso, status 0 ou 5 é gravado na linha do clube no grupo da primeira fase, com o registro de acesso; nos grupos de classificação o status 3 (rebaixado) é gravado com o registro de rebaixamento; no mata-mata quem vence (status 1) fica com 0 na linha e ganha o registro de acesso',
          'rule: in the first phase a club that goes through (status 1) gets the qualified record; in the promotion group status 0 or 5 is written in the club\'s row of its first phase group, with the promotion record; in the classification groups status 3 (relegated) is written with the relegation record; in the play-off the winner (status 1) gets 0 in its row and the promotion record'),
         ('tur_second_b_build_fate_news_text', 'TurSecondB::build_fate_news_text', '0x9015f0', 'tur_second_b.cpp', 'thiscall, ret 0x28',
          'regra: a notícia do destino de um clube da categoria B: no grupo de acesso, sem notícia, "conquista o acesso"; nos grupos de classificação quem vai ao mata-mata recebe "se classificou para o mata-mata da competição" (com notícia) ou "vai ao mata-mata"; o resto, o texto comum',
          'rule: the fate news text of a category B club: in the promotion group, without a news item, "win promotion"; in the classification groups a club through to the play-off gets "have qualified for the playoff of the competition" (with a news item) or "qualify for playoff"; the rest, the common text'),
         ('tur_second_b_record_final_positions', 'TurSecondB::record_final_positions', '0x901ba0', 'tur_second_b.cpp', 'thiscall, ret 0',
          'regra: as posições finais da categoria B: os dois primeiros de cada grupo da primeira fase ficam em 1º e 2º e os clubes de cada grupo de classificação de 3º em diante, pela ordem da tabela; o grupo de acesso e o mata-mata não gravam posição',
          'rule: the final positions of category B: the first two of each first phase group get 1st and 2nd and the clubs of each classification group 3rd upwards, in table order; the promotion group and the play-off record no position')])
CAMPOS = []
INTERESSA = []
