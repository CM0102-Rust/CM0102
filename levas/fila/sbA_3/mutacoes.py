# Mutacoes (sbA_3): texto, acusadas pela comparacao com CM_TEXTOS=1 (o texto nao volta para a simulacao). Nenhuma abre
# caixa. Saem so quando uma noticia de financas da diretoria sai ate a data (injecao, patrocinio, acoes...): podem
# escapar se nenhuma sair na carreira ENG ate 2002-06-01.
MUTACOES = [
    ('sbA_3_limite_trocado',
     '                if second > limit() {',
     '                if second <= limit() { // mutation'),
    ('sbA_3_plc_sem_ligacao',
     '                fmt(buf, 0x9c87d0, g, g, name); // the club became a PLC\n                exe::text_add_link(out, name as *const u8, 0x7d5, club);',
     '                fmt(buf, 0x9c87d0, g, g, name); // mutation'),
    ('sbA_3_patrocinio_com_acoes',
     '            6 => { fmt(buf, 0x9c8e7c, a); }',
     '            6 => { fmt(buf, 0x9c8e08, a); } // mutation'),
]
