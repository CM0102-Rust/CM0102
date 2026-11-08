# Mutacoes (cpz_2). A 0x576c50 e a virtual +0x44 da First, da Second e da Third: o anuncio da situacao (0x686670)
# chama-a quando a tabela (0x684cd0) decide o acesso (0), a vaga no mata-mata (1) ou o rebaixamento (3) de um clube,
# de marco a maio de 2002 (a cobertura da 260a tem "cpq_3 acesso" e "cpq_3 rebaixamento" 8 vezes). As tres mudam os
# registros que a carreira le: o peso do acesso (as notas e os pontos do tecnico, importancia x peso), o peso da vaga no
# mata-mata (os pontos do tecnico) e o rebaixamento inteiro (a injury_word_0c do clube -500, as notas do tecnico, o moral e
# as reputacoes do elenco, a renovacao dos contratos). Os contadores ficam em cada situacao, colados na chamada.
MUTACOES = [
    ('cpz2_acesso_pesa_metade',
     'acesso(eng_second_staff_records(), clube, comp, 0x64); // cpz2: o acesso pela tabela pesa 0x64',
     'acesso(eng_second_staff_records(), clube, comp, 0x32); // mutacao: o acesso pesa a metade',
     'cpz_2 acesso na tabela'),
    ('cpz2_vaga_no_mata_mata_pesa_o_dobro',
     'vaga(eng_second_staff_records(), clube, comp, 0xa0, 0x82, 0x1e); // cpz2: a vaga no mata-mata pesa 0x1e',
     'vaga(eng_second_staff_records(), clube, comp, 0xa0, 0x82, 0x3c); // mutacao: a vaga pesa o dobro',
     'cpz_2 vaga no mata-mata'),
    ('cpz2_sem_registro_do_rebaixamento',
     'cair(eng_second_staff_records(), clube, comp); // cpz2: o rebaixamento pela tabela',
     'let _ = (cair, clube, comp); // mutacao: sem o registro do rebaixamento',
     'cpz_2 rebaixamento na tabela'),
]
