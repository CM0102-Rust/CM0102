# Mutacoes (cx15_7): texto, acusado pela comparacao com CM_TEXTOS=1 (o texto nao volta para a simulacao) quando o clube
# humano recebe a noticia da inscricao europeia. Nenhuma abre caixa.
# - o passo 1 com os inscritos no lugar do maximo de vagas;
# - o corpo sem a ligacao da competicao.
MUTACOES = [
    ('cx15_7_maximo_trocado',
     '            lang_sprintf(0xde1f64, 0xa834dc, cg, cg, kg, kg, club_name, r.places as i32, comp_name);',
     '            lang_sprintf(0xde1f64, 0xa834dc, cg, cg, kg, kg, club_name, r.registered as i32, comp_name); // mutation',
     'cx15_7 qualifying phase request'),
    ('cx15_7_sem_ligacao_da_competicao',
     '    exe::text_add_link(text, comp_name as *const u8, 0x7d0, r.comp);\n    if r.made == 0 {',
     '    if r.made == 0 { // mutation'),
]
