# Mutacoes (cpy_11), na 0x8c39f0 (19414 chamadas ate 2002-06-01 na cobertura de 17/10), que roda no fim de cada
# partida. As permissoes de trabalho (+4 jogos, +6 faltas) sao lidas pela 0x8c7950 em toda avaliacao de permissao
# (jogos > 3 x faltas ou faltas < 4; 247570 chamadas): sem contar as faltas, jogadores reservas mantem a permissao que
# perderiam; sem contar os jogos, os starters com 4 faltas a perdem; as listas de observacao e as transferencias
# mudam. Os premios por jogos pagos (cpp_6 "premio por jogos" ~4 na base; mexem no saldo de dois clubes) deixam de sair
# quando os jogos do premio passam a ser os de final 3.
MUTACOES = [
    ('cpy11_falta_nao_conta', '    increment_permit_counter(r, 6); // cpy11: a falta conta na permissão',
     '    let _ = r; // mutacao: a falta não conta',
     'cpy_11 absence counted'),
    ('cpy11_jogo_nao_conta', '        increment_permit_counter(r, 4); // cpy11: o jogo conta na permissão',
     '        let _ = r; // mutacao: o jogo não conta',
     'cpy_11 match counted'),
    ('cpy11_premio_aos_3', '        if (jogos as i32 % 10 == 0 && jogos <= 0x32) || jogos == 1 { // cpy11: os jogos do prêmio',
     '        if (jogos as i32 % 10 == 3 && jogos <= 0x32) || jogos == 1 { // mutacao: os jogos do prêmio',
     'cpy_11 bonus paid'),
]
