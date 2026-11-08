# Mutacoes. A 0x5c6b70 decide se o clube esta livre para o amistoso (430 chamadas ate 2002-06-01: excursoes e torneios
# da pre-temporada, respostas aos convites, a situacao das datas que a marcacao dos amistosos da maquina le): livre com
# um jogo na janela (so dois ou mais ocupam) e a janela sem a vespera mudam os amistosos marcados, os jogos e as rendas.
# A 0x522910 sem achar o clube (a 0x4ae300 da diretoria percorre as competicoes da nacao do fim para o comeco e fica
# com a primeira que tem o clube; se a copa vem depois da liga na lista, o ramo diz): a projecao semanal sai de outra
# competicao e a confianca da diretoria muda. A parceira (Inglaterra/Pais de Gales) so
# pesa com as duas nacoes ativas: sem mutacao.
MUTACOES = [
    ('cpr13_um_jogo_livre', 'if n as u16 != 0 {',
     'if n as u16 > 1 { // mutacao: um jogo nao ocupa',
     'cpr_13 busy'),
    ('cpr13_sem_vespera', 'let r = menos_dias(data, p0, 1); // a véspera',
     'let r = menos_dias(data, p0, 0); // mutacao: sem a vespera',
     'cpr_13 free club'),
    ('cpr13_copa_sem_o_clube', 'return 1; // está na copa',
     'return 0; // mutacao: nunca na copa',
     'cpr_13 cup has the club'),
]
