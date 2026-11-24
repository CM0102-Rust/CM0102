# A Copa dos Campeoes do Brasil: os registros do pessoal, o progresso de um clube e a noticia do destino
# (bra_champ_cup.cpp; leitor cx01, rodada 5, 24/11/2026). Usa scale_by_round e os european_cup_add_* (european_cup.rs),
# african_nations_record_final_result (african_nations.rs) e set_bra_champ_cup_winner (conmebol_seeding.rs). Carreira de
# 8 ligas: os registros 12 vezes e o progresso 69 vezes ate 2002-06-01; a noticia so na comparacao com CM_TEXTOS=1.
# - 0x426900 BraChampCup::update_staff_records_for_status thiscall(clube, desfecho, fase, _, partida, extra) ret 0x18
#   (slot 0x44): Staff_Records (mov ecx, [0xdd7f00]) add_qualified_for_round_record 0x86bdd0 ret 0x14,
#   add_failed_to_qualify_record 0x86c1d0 ret 0x10, add_knocked_out_record 0x86c000 ret 0x14, add_cup_winners_record
#   0x86a800 e add_cup_runners_up_record 0x86b0b0 ret 0xc; comp_util_scale_by_round cdecl(0x32, 0x64, rodada, 2);
#   ConmebolSeeding::set_bra_champ_cup_winner 0x4c6960 (mov ecx, [0xae1000]; push clube; ret 4).
# - 0x426c20 BraChampCup::set_club_progress thiscall(clube, fase, atual, melhor, pior) ret 0x14 (slot 0x68):
#   Comp_Stats::find_club_entry (mov ecx, [+8]; push clube; ret 4; o retorno nao e testado); tres tabelas de salto
#   (0x426d4c, 0x426d5c, 0x426d6c) para os grupos.
# - 0x425fd0 BraChampCup::build_fate_news_text thiscall(clube, desfecho, fase, nome, partida, fase_seguinte, _, veredito,
#   corpo, texto) ret 0x28 (slot 0x48): comp_util_write_group_qualify_news 0x4b4590 cdecl(clube, fase ou nome, comp,
#   6/-2, corpo, texto); 0x4b0b80 e 0x4b4420 como no Comp::build_fate_news_text.
# Iguais no cm0102-1080.exe (as tres inteiras).
# Comparadores: os registros so a original (historico dos tecnicos e a vaga da CONMEBOL), devolvendo 0; o progresso
# lado a lado nos 3 bytes da entrada; a noticia por compare_text_writer.
# Defeito do original [inferencia] (0x425fd0): a linha de classificado passa o indice da fase (fase) onde a de eliminado
# passa o id do nome da fase.
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
SAVE = False
ROTINAS = [('bra_champ_cup_update_staff_records_for_status', 0x426900, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('bra_champ_cup_set_club_progress', 0x426c20, [0x8b, 0x44, 0x24, 0x04, 0x8b, 0x49, 0x08]),
           ('bra_champ_cup_build_fate_news_text', 0x425fd0, [0x8a, 0x44, 0x24, 0x0c, 0x56])]
DOCS = ('a Copa dos Campeões do Brasil: os registros do pessoal, o progresso de um clube e a notícia do destino',
        "the Brazilian Champions Cup: the staff records, a club's progress and the fate news",
        [('bra_champ_cup_update_staff_records_for_status', 'BraChampCup::update_staff_records_for_status', '0x426900', 'bra_champ_cup.cpp', 'thiscall, ret 0x18',
          'registra no histórico do técnico: na preliminar e nos grupos, classificado ou eliminado (peso 40); no mata-mata, eliminado ou classificado com o peso pela rodada (50 a 100), o campeão (que vira o campeão da Copa dos Campeões no sorteio da CONMEBOL) e o vice',
          "writes into the manager's history: in the preliminary and the groups, qualified or out (weight 40); in the knockout, out or through with the weight by round (50 to 100), the winner (who becomes the Champions Cup winner in the CONMEBOL seeding) and the runner-up"),
         ('bra_champ_cup_set_club_progress', 'BraChampCup::set_club_progress', '0x426c20', 'bra_champ_cup.cpp', 'thiscall, ret 0x14',
          'guarda as posições do clube nas estatísticas convertendo os códigos: na preliminar 1 e 2 viram 1 (o pior, 7) e 3 vira 9; nos grupos 1 e 2 viram 1 (o pior, 3), 3 vira 5 e 4 vira 7; do mata-mata em diante ficam como estão',
          "stores the club's positions in the statistics converting the codes: in the preliminary 1 and 2 become 1 (the worst, 7) and 3 becomes 9; in the groups 1 and 2 become 1 (the worst, 3), 3 becomes 5 and 4 becomes 7; from the knockout on they stay as they are"),
         ('bra_champ_cup_build_fate_news_text', 'BraChampCup::build_fate_news_text', '0x425fd0', 'bra_champ_cup.cpp', 'thiscall, ret 0x28',
          'escreve a notícia do destino: na preliminar e nos grupos, o classificado (a linha do grupo no corpo; "passa à fase de grupos" ou "passa à semifinal" na manchete) e o eliminado (a linha do grupo); no mata-mata, as linhas comuns da copa',
          'writes the fate news: in the preliminary and the groups, the qualified club (the group line in the body; "through to group phase" or "through to semi final" in the headline) and the one out (the group line); in the knockout, the shared cup lines')])
CAMPOS = []
INTERESSA = []
