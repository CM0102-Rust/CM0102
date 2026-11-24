# Mutacoes (sx19b_11), item de TELAS: a carreira automatica nao passa; servem para provar os comparadores numa sessao
# do Lucas jogando comparando (Procurar):
# - "School" ate 18 anos inclusive (o comparador acusa ao procurar "School" com algum jogador de 18 anos);
# - o nome so nos seis primeiros estilos (o comparador acusa ao procurar uma pessoa pelo estilo 7).
MUTACOES = [
    ('sx19b_11_escola_18',
     'return staff_pass(this, |p, _| staff_kind_fits(p, players) && read_i8(p + staff::AGE) < 18); // sx19b_11: school age',
     'return staff_pass(this, |p, _| staff_kind_fits(p, players) && read_i8(p + staff::AGE) <= 18); // mutation: 18 too'),
    ('sx19b_11_seis_estilos',
     'for style in 1..=7u32 { // sx19b_11: the seven name styles',
     'for style in 1..=6u32 { // mutation: six styles'),
]
