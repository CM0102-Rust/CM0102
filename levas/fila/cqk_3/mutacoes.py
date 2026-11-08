# Mutações (cqk_3), todas no Brasil (liga da carreira; a Alemanha e a Escócia não são):
# - o Brasil fecha a janela em julho em vez de novembro: sem contratações de julho a dezembro de 2001 (contador "cqk_3 brazil");
# - o Brasil abre a janela em fevereiro em vez de dezembro: sem contratações de novembro de 2001 a fevereiro de 2002
#   (contador "cqk_3 brazil");
# - o +0x13 do Brasil = 1 em vez de 3 [inferência: o limit de estrangeiros] (contador "cqk_3 brazil"; pode escapar se
#   o campo não limitar nenhuma contratação da IA até 2002-06-01).
MUTACOES = [
    ('cqk3_brasil_fecha_em_julho', '[0x04, 0x00, 0x05, 0x08, 0x0a, 0x00], // cqk3: o Brasil fecha em novembro', '[0x04, 0x00, 0x05, 0x08, 0x06, 0x00], // mutacao: julho', 'cqk_3 brazil'),
    ('cqk3_brasil_abre_em_fevereiro', '[0x04, 0x00, 0x03, 0x0b, 0x0b, 0x01], // cqk3: o Brasil abre em dezembro', '[0x04, 0x00, 0x03, 0x0b, 0x01, 0x01], // mutacao: fevereiro', 'cqk_3 brazil'),
    ('cqk3_brasil_limite_um', '*((this + 0x13) as *mut u8) = 3; // cqk3: o limit do Brasil', '*((this + 0x13) as *mut u8) = 1; // mutacao: 1', 'cqk_3 brazil'),
]
