# Mutacoes (cx13_8), com a Escocia ate 2002-08-01 (o split da Premier em abril de 2002):
# - o texto do grupo do titulo com o clube seguinte (acusada pelo comparador, lado a lado no buffer);
# - o codigo atual do progresso + 1 (acusada pelo comparador, lado a lado nos 3 bytes; as expectativas da diretoria
#   leem o codigo).
# Os recordes de pessoal nao tem mutacao propria: o retorno e sempre 0 e o recorde so pesa na reputacao do tecnico
# (a carreira dificilmente acusa ate agosto); a prova deles e a comparacao (o retorno) e a leitura.
MUTACOES = [
    ('cx13_8_texto_do_clube_seguinte',
     'return championship_group_text(League(this), club_id, news); // cx13_8: the split text',
     'return championship_group_text(League(this), club_id + 1, news); // mutation: the next club',
     'cx13_8 scottish championship group text'),
    ('cx13_8_codigo_atual_mais_um',
     'e.set_current_position(current as u8); // cx13_8: the current code',
     'e.set_current_position(current as u8 + 1); // mutation: one more',
     'cx13_8 scottish progress'),
]
