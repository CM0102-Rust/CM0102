# Mutacoes (cqa_4). As tres nos registros de final da 0x519890 (a virtual +0x44 da FA Cup e da Copa da Liga): as
# finais da Copa da Liga (fevereiro de 2002) e da FA Cup (maio de 2002) caem antes de 2002-06-01. O registro de
# campeao (0x86a800) sobe mais a injury_word_0c do clube, do tecnico e do elenco da final; o de vice (0x86b0b0) e o de
# terceiro (0x86b710) sobem menos: trocar um pelo outro muda as reputacoes, que estao no banco e pesam nas
# transferencias, nas propostas e nas notas da diretoria ate o fim da temporada. Os contadores ficam colados em cada
# registro, depois das condicoes (jogo valido, copa sem fases e desfecho). Se as finais cairem depois da data da
# carreira (o contador "cqa_4 campeao" mostra), PODEM ESCAPAR.
MUTACOES = [
    ('cqa4_vice_como_campeao',
     'cup_record_final_result(0x86b0b0, clube, jogo, final_); // cqa4: o registro de vice',
     'cup_record_final_result(0x86a800, clube, jogo, final_); // mutacao: o vice com o registro de campeao',
     'cqa_4 vice'),
    ('cqa4_campeao_como_vice',
     'cup_record_final_result(0x86a800, clube, jogo, final_); // cqa4: o registro de campeão',
     'cup_record_final_result(0x86b0b0, clube, jogo, final_); // mutacao: o campeao com o registro de vice',
     'cqa_4 campeao'),
    ('cqa4_campeao_como_terceiro',
     'cup_record_final_result(0x86a800, clube, jogo, final_); // cqa4: o registro de campeão',
     'cup_record_final_result(0x86b710, clube, jogo, final_); // mutacao: o campeao com o registro de terceiro',
     'cqa_4 campeao'),
]
