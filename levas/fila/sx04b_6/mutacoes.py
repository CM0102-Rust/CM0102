# Mutacoes (sx04b_6) pela prova do save (mutacoes.py --prova-save N): o .qst gravado no modo rust com a mutacao contra
# a base (a mesma DLL sem as rotinas da leva). Sem o 4o campo (contador): a base da prova do save roda com as rotinas
# desligadas.
# - o premio do mes grava 11 vagas e nao 12 (o bloco de cada premio do mes sai 11 bytes menor e o resto deslocado; o
#   zwrite devolve a quantidade pedida, sem caixa);
# - a carga das notas do mes esquece o mes lido (volta o -1 do construtor; no primeiro dia o mes fecha de novo e os
#   premios do mes saem cedo: a impressao da carga do FeriasTesteSave muda). Uma mutacao que pule uma leitura
#   desalinharia o resto do save (caixa esperando clique): evitada.
MUTACOES = [
    ('sx04b_6_onze_vagas',
     'if exe::zwrite(this + month_award_fields::SLOT_WINNER_ID, MONTH_AWARD_SLOT_SIZE as u32, 12, file) != 12 { // sx04b_6: the twelve slots',
     'if exe::zwrite(this + month_award_fields::SLOT_WINNER_ID, MONTH_AWARD_SLOT_SIZE as u32, 11, file) != 11 { // mutation: eleven slots'),
    ('sx04b_6_mes_esquecido',
     'covered!("sx04b_6 month ratings loaded");',
     'write_u8(this + month_ratings_fields::MONTH, 0xff); // mutation: the month forgotten'),
]
