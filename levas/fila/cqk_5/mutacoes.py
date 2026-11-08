# Mutacoes (cqk_5):
# - a Primera sem premio no fim (+0xe1 = 2: a 0x682e10 nao paga nada quando a liga acaba, em maio de 2002);
# - a cota do fundo de premios da Primera vira a metade (provado pelo agente: a impressao muda ate 2002-06-01);
# - sem o registro do rebaixamento pela tabela da C2/A (0x86a1c0: a injury_word_0c do clube, as notas do tecnico, o moral e
#   as reputacoes do elenco), em abril e maio de 2002 (provado pelo agente).
# Ficaram de fora: a TV (a 0x683010 no inicio da carreira nao mudou a impressao nas provas da cqi_5 e da cqj_4) e as
# linhas trocadas (derrubam o jogo na Espanha: o confronto direto da 0x687430 le a linha seguinte ainda vazia).
MUTACOES = [
    ('cqk5_primera_sem_premio',
     '*((this + 0xe1) as *mut u8) = 1; // cqk5: o prêmio pela tabela no fim da liga',
     '*((this + 0xe1) as *mut u8) = 2; // mutacao: sem premio no fim',
     'cqk_5 primera fixtures'),
    ('cqk5_cota_da_primera_metade',
     'let f = premio(this, 0x3d090); // cqk5: a cota da Primera',
     'let f = premio(this, 0x1e848); // mutacao: a metade',
     'cqk_5 primera fixtures'),
    ('cqk5_sem_registro_do_rebaixamento',
     'cair(cqk5_registros(), clube, comp); // cqk5: o rebaixamento pela tabela da C2/A',
     'let _ = (clube, comp); // mutacao: sem o registro',
     'cqk_5 relegation in c2a'),
]
