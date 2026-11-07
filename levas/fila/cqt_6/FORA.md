# cqt_6 sem PRONTO (06/11/2026)

As três rotinas da Copa dos Campeões do Brasil (0x426130 a tabela da preliminar, 0x427320 a preliminar, 0x4263c0 a
fase seguinte) rodam na carreira de 8 ligas (2001-167, 2001-183 e 2001-200; os contadores das mutações rodaram) e não
dão divergência na comparação até 2001-10-01 (o plano da fase seguinte foi relaxado: os grupos gravam a fase deles em
+0x30 depois; a comparação longa até 2002-03-15 acusou isso à toa). As carreiras Rust ×2 com o detector saem iguais.
Mas as mutações não mudam o resultado da carreira:

- os 3 da preliminar ficarem na lista dos inscritos (+0x14) escapou: os grupos não tiram os times dessa lista
  [inferência];
- a tabela da preliminar sem a ordem inicial (League::sort_table) escapou: a liga reordena depois;
- as linhas da preliminar em ordem inversa derrubam o jogo (EXCECAO na DLL em 2001-167): acusada só pela queda.

É a lição da 279ª–281ª (a Copa dos Campeões é torneio passageiro: o que se lê depois não depende dela dentro da
janela). Saídas: entrar como passageiras num item com rotina de efeito, ou mirar o campeão (a vaga da Libertadores),
que é decidido nas semifinais e na final (0x426720 e a final, ainda não portadas).
