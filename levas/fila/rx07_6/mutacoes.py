# Mutações (rx07_6), para o comparador numa sessão do Lucas nas telas de contrato (a carreira automática não abre
# tela nenhuma):
# - o jogador-treinador pede treino 11, não 10 (get_offer_job lado a lado);
# - o jogador abaixo de 25 anos sem queixas não ganha o sim imediato (will_negotiate_offer lado a lado).
MUTACOES = [
    ('rx07_6_treinador_11',
     '        return if sheet != 0 && coaching_rating_reaches(sheet, 10) { 0xf } else { 0xb };',
     '        return if sheet != 0 && coaching_rating_reaches(sheet, 11) { 0xf } else { 0xb }; // mutation',
     'rx07_6 player offer job'),
    ('rx07_6_jovem_sem_sim',
     '    if age < 0x19 { // rx07_6: under 25',
     '    if age < 0x10 { // rx07_6: under 25 (mutation)',
     'rx07_6 young player will talk'),
]
