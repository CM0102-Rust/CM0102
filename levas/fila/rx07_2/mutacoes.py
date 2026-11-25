# Mutações (rx07_2), carreira ENG até 2002-06-01 (o jogo é criado do zero em cada execução):
# - os fins de contrato vazios caem de 1 a 5 anos depois, não de 1 a 6: as renovações e as saídas no fim do contrato
#   mudam;
# - os fins de contrato antigos andam 51 semanas, não 52: os contratos que acabam em junho mudam de semana (o dia dos
#   contratos e os avisos de fim de contrato);
# - o cargo 15 não ganha contrato na criação.
MUTACOES = [
    ('rx07_2_fim_ate_5_anos',
     '        let years = exe::utils_random(6) as u32;',
     '        let years = exe::utils_random(5) as u32; // mutation',
     'rx07_2 initial contract'),
    ('rx07_2_anda_51_semanas',
     '        let n = exe::date_add_weeks(expires, &mut next, 0x34);',
     '        let n = exe::date_add_weeks(expires, &mut next, 0x33); // mutation',
     'rx07_2 contract end moved a year'),
    ('rx07_2_cargo_15_sem_contrato',
     '    matches!(job, 0 | 6..=13 | 15)',
     '    matches!(job, 0 | 6..=13) // mutation',
     'rx07_2 initial contract'),
]
