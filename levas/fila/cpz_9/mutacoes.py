# Mutacoes (cpz_9), todas no incidente de forma (0x41add0, pelo diario das pessoas 0x5ce2d0 na queda grande de
# match_condition, que acontece com jogadores de todos os clubes), no clube da IA com tecnico: o sorteio da multa em 20 (e nao
# 10) multa menos jogadores; sem o sorteio(5) do prazo, um sorteio a menos por multa; o prazo sempre 1 (aviso) muda o
# caminho da multa (0x5a5980: a moral, a reacao do elenco e os sorteios dela). As tres mudam a contagem dos sorteios.
# O ramo do incidente na IA nao tem contador de levas anteriores (a 0x41add0 nunca foi desviada): se o contador
# 'cpz_9 multa avaliada' vier zerado na base, o ramo nao roda e as tres escapam por isso.
MUTACOES = [
    ('cpz9_sorteio_da_multa_20', '    let b = cpz9_sortear(10); // cpz9: o sorteio da multa',
     '    let b = cpz9_sortear(20); // mutacao: o sorteio da multa em 20',
     'cpz_9 multa avaliada'),
    ('cpz9_sem_sorteio_do_prazo', '    let e = cpz9_sortear(5); // cpz9: o sorteio do prazo',
     '    let e = 2i32; // mutacao: sem o sorteio do prazo',
     'cpz_9 multa pela forma'),
    ('cpz9_prazo_sempre_aviso', '    let prazo = if c > e { 1 } else { 2 }; // cpz9: o prazo da multa',
     '    let prazo = if c > e { 1 } else { 1 }; // mutacao: o prazo sempre 1',
     'cpz_9 multa pela forma'),
]
