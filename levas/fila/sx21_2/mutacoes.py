# Mutacoes (sx21_2). O codigo de quem nao pode jogar roda em cada escalacao da carreira ENG; os dias ate o fim da
# janela, nas noticias de janela (todo dia). O destrutor nao roda na carreira: sem mutacao.
# - contrato do tipo 5 da 0 em vez de 3: escala quem o original barra (comparador e carreira);
# - o emprestado contra o proprio clube da 0 em vez de 11 (comparador; carreira quando acontece);
# - sem janela aberta, 364 dias (comparador).
MUTACOES = [
    ('sx21_2_tipo_5',
     'return 3; // sx21_2: contract type 5',
     'return 0; // mutation: type 5 may play',
     'sx21_2 contract type 5'),
    ('sx21_2_contra_o_dono',
     'return 0xb; // sx21_2: against the owning club',
     'return 0; // mutation: may play against his owner',
     'sx21_2 loan against his own club'),
    ('sx21_2_sem_janela_364',
     'return 0x16d; // sx21_2: no window open: 365',
     'return 0x16c; // mutation: 364',
     'sx21_2 days to the close'),
]
