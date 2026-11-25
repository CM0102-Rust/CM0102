# Mutações (rx07_7). Estas notícias só saem para um técnico humano: a carreira automática não as roda e o comparador
# roda só a original. As mutações mudam o que chega à caixa do Lucas no modo Rust (a prova dele numa carreira com a
# comparação; o texto da notícia, montado pelos escritores do rx07_8/rx07_9 com o comparador de texto, mostra a troca):
# - a notícia da volta do empréstimo vai sempre ao clube onde o jogador joga.
MUTACOES = [
    ('rx07_7_volta_sempre_ao_atual',
     '    let to = if club == read_usize(person + staff::CLUB_JOB) { main_club(person) } else { read_usize(person + staff::CLUB_JOB) };',
     '    let to = read_usize(person + staff::CLUB_JOB); // mutation',
     'rx07_7 loan recall news'),
]
