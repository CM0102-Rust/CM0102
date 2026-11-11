# Mutacoes (mkb_2), na avaliacao do elenco pela IA (0x83cb40; roda a cada lista refeita no laco do dia, centenas de
# vezes ate 2002-06-01, e na criacao da carreira); as tres mudam quem e listado:
# - o elenco que permite a lista de transferencia cai de reputacao/500 + 20 para + 10: clubes com elenco menor tambem
#   listam os excedentes (contador "mkb_2 transfer listed");
# - a infelicidade que poe na lista de emprestimo passa de idade x 50/3 - 400 para - 200: mais jogadores de linha
#   infelizes emprestaveis (contador "mkb_2 loan listed");
# - o elenco que libera um contrato (fila de transferencia livre) cai de reputacao/500 + 25 para + 10 (contador
#   "mkb_2 free transfer").
MUTACOES = [
    ('mkb2_lista_com_elenco_menor',
     '> rep / 500 + 20 { // mkb2: the squad size that allows listing',
     '> rep / 500 + 10 { // mutation: a smaller squad allows listing',
     'mkb_2 transfer listed'),
    ('mkb2_emprestimo_menos_infeliz',
     'age_i * 50 / 3 - 400 // mkb2: the unhappiness that puts a player on the loan list',
     'age_i * 50 / 3 - 200 // mutation: less unhappy',
     'mkb_2 loan listed'),
    ('mkb2_libera_com_elenco_menor',
     '> rep / 500 + 25 // mkb2: the squad size that frees a contract',
     '> rep / 500 + 10 // mutation: a smaller squad frees contracts',
     'mkb_2 free transfer'),
]
