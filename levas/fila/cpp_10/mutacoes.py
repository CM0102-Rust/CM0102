# Mutacoes. A compra grande a partir de 1000 x (reputacao + 200) (mais recordes vao a fila e mais tecnicos passam a
# gostar do jogador pela 0x547a10, que sorteia quando nao ha vaga: muda a carreira; comparando, o plano usa a mesma
# regra e a original nao: a conta da fila difere). O gostar sem forcar (sem vaga, nao sorteia: so o modo Rust e a
# carreira; PODE ESCAPAR se os tecnicos tiverem vaga livre, ver o ramo). O jogador de quem o clube gosta perdendo 0,2
# em vez de ganhar (comparando, o lado a lado acusa quando o escolhido muda; premio de fim de temporada: na carreira
# PODE ESCAPAR).
MUTACOES = [
    ('cpp10_compra_grande', 'valor >= (cpp10_w(clube + 0x80) as i32 + 300).wrapping_mul(1000)',
     'valor >= (cpp10_w(clube + 0x80) as i32 + 200).wrapping_mul(1000) // mutacao',
     'cpp_10 transferencia grande'),
    ('cpp10_gostar_forcado', 'gostar(cpp10_d(comprador + 0xbf), pessoa, 1); // forçado',
     'gostar(cpp10_d(comprador + 0xbf), pessoa, 0); // mutacao: sem forcar',
     'cpp_10 transferencia grande'),
    ('cpp10_favorito_do_clube', 'nota = nota + cpp10_q(0x968918); // o clube gosta dele',
     'nota = nota - cpp10_q(0x968918); // mutacao: perde 0,2',
     'cpp_10 favorito do clube'),
]
