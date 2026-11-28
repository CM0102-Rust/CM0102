# Mutacoes (r7_08_1). A carreira de teste nao passa pela rotina: o Data/player_setup.cfg do jogo_testes nao tem linha
# FUTURE_TRANSFER (ver o PRONTO.txt); acusadas na emulacao (scratchpad\r7_08\check8.py fut).
# - os anos sorteados de 2..5 passam a 3..6;
# - o salario limitado pela metade do teto do clube;
# - a transferencia futura marcada na data do fim do contrato em vez da data combinada.
MUTACOES = [
    ('r7_08_1_anos',
     "    let years = if (years as i8) < 1 { (exe::utils_random(4) as u8).wrapping_add(2) as i8 } else { years as i8 }; // r7_08_1: 2 to 5 years",
     "    let years = if (years as i8) < 1 { (exe::utils_random(4) as u8).wrapping_add(3) as i8 } else { years as i8 }; // r7_08_1 mutation",
     'r7_08_1 future transfer added'),
    ('r7_08_1_teto',
     "            terms.set_wage(cap); // r7_08_1: the wage within the club's cap",
     "            terms.set_wage(cap / 2); // r7_08_1 mutation",
     'r7_08_1 future transfer added'),
    ('r7_08_1_data',
     "    exe::transfer_manager_add_future_transfer(this, placed, date as *mut [u32; 2]);",
     "    exe::transfer_manager_add_future_transfer(this, placed, end as *mut [u32; 2]); // r7_08_1 mutation",
     'r7_08_1 future transfer added'),
]
