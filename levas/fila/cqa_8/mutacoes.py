# Mutacoes (cqa_8), todas no 0x4ca6d0 (19.414 chamadas ate 2002-06-01, a cada jogo com +0x40 e +0x41). As tres tiram
# sorteios do caminho de todo jogo: a fila do sorteio anda diferente e todos os sorteios seguintes mudam.
# - O jogador do vencedor sorteia(21 - +0x57) a moral: sem esse sorteio (e sem a moral), a fila anda menos.
# - Os reservas que entraram (e os starters de pouco jogo) sorteiam duas vezes contra os minutos: sem o primeiro.
# - Quem ficou de fora evolui (0x6d26b0 sorteia o ajuste da moral e mexe nas notas da ficha): sem a evolucao.
MUTACOES = [
    ('cqa8_vencedor_sem_sorteio', '                let r = sorteio(0x15 - cqa8_b(pessoa + 0x57) as i32); // cqa8: o sorteio do vencedor',
     '                let r = 4; // mutacao: sem o sorteio do vencedor',
     'cqa_8 winner morale'),
    ('cqa8_moral_sem_sorteio', '                    let r1 = sorteio(0x1e); // cqa8: o primeiro sorteio da moral',
     '                    let r1 = 15; // mutacao: sem o primeiro sorteio da moral',
     'cqa_8 tests the morale'),
    ('cqa8_fora_sem_evoluir', '            evoluir(0, pessoa, clube, outro, jogo, (nota - read_f64_at(0x967888)) as f32, 0, 0); // cqa8: a evolução de quem ficou de fora',
     '            let _ = (evoluir, nota); // mutacao: quem ficou de fora não evolui',
     'cqa_8 develops who was left out'),
]
