# Mutacoes (sx09c_6), pela carreira ENG ate 2002-06-01 no modo Rust (os comparadores so rodam a original):
# - a queixa corta as reputacoes ate 40% em vez de 20%: muda a reputacao do jogador, o interesse dos clubes e as
#   transferencias. So roda se alguma queixa for aberta na carreira de ferias (pela multa contestada); se nenhuma for,
#   escapa (o ramo nao roda).
# O construtor e o destrutor ficam sem mutacao: com a lista vazia e as marcas zeradas, qualquer desvio que nao derrube
# o jogo nao muda nada sem disputa; a carreira no modo Rust, que cria o jogo do zero, os prova.
MUTACOES = [
    ('sx09c_queixa_corta_40',
     'let v = cut(person.player().current_reputation()); // sx09c_6: the reputations drop up to 20%',
     'let v = cut(cut(person.player().current_reputation())); // mutation',
     'sx09c_6 complaint filed'),
]
