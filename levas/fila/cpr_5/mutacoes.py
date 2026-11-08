# Mutacoes (cpr_5), todas nas diretorias do inicio da carreira (0x68f130, uma chamada, todos os clubes com tecnico; as
# outras duas rotinas nao tem efeito que a carreira automatica leia: a lista das apelacoes fica vazia e a 0x5fe700 so
# ve partidas assistidas vazias, +0x43 = 0). A expectativa vale 7000 em vez de 7500 (as quatro confiancas de cada
# diretoria descem: as demissoes e as vagas mudam). A confianca +0xa sem o primeiro sorteio (o sorteador anda uma vez a
# menos por clube: tudo o que sorteia depois muda). A confianca +6 baixa (abaixo de 5750) sobe sem sorteio (sorteio a
# menos nos clubes nesse ramo; ver o ramo).
MUTACOES = [
    ('cpr5_expectativa_7000', '        let h = ((f as f64 / g) * 7500.0) as f32;',
     '        let h = ((f as f64 / g) * 7000.0) as f32; // mutacao',
     'cpr_5 diretoria com tecnico'),
    ('cpr5_sem_sorteio_0xa', '        roll_confidence(l + 0xa, 0x6d6, h);',
     '        cpr5_pw(l + 0xa, match_pl_ftol((0x36b as f64 + h as f64) as f32 as f64 - cpr5_sortear(0x1f4) as f64) as i16); // mutacao',
     'cpr_5 diretoria com tecnico'),
    ('cpr5_baixa_sem_sorteio', '            cpr5_pw(l + 6, (cpr5_sortear(0x2ee) + 0x1676) as i16);',
     '            cpr5_pw(l + 6, 0x17ed); // mutacao: sem o sorteio',
     'cpr_5 confianca baixa'),
]
