# A noticia do ultimato do tecnico a diretoria (manager_manager.cpp; leitor rx08, rodada 6, 25/11/2026). TELAS: roda
# pelo tratador do ultimato (MANAGER_MANAGER::handle_manager_ultimatum 0x6ad610, ainda original), que o tecnico humano
# dispara pela tela da diretoria; a cobertura do mundo viu 0 chamadas.
# Caminho no jogo: Clube -> Diretoria (Board) -> dar um ultimato (pedir dinheiro, estadio ou treino sob ameaca de sair):
# as noticias da resposta da diretoria saem por esta rotina (o item 0x178f).
# - 0x6ad930 MANAGER_MANAGER::post_ultimatum_response_news thiscall(tecnico, clube, a, b, c, d) ret 0x18 (quadro SEH so
#   para o NewsItem local): tecnico ou clube nulo: caixa 0x5cd2; NewsItem::NewsItem 0x77bbe0 (0x178f, 0),
#   NewsItem::set_slot 0x785790 (k, valor) nas vagas 0..10: o id do tecnico, os ids (+0x33) dos registros de nome +4, +8
#   e +0xc, o id do clube, os quatro codigos (bytes com sinal), o estadio pequeno ([[clube+0x69]+0x3c] < o esperado pela
#   reputacao +0x80 e pela forca da liga +0x85 da nacao +0x53: > 14, 3 x rep + 5000; > 8, 2 x rep + 2500; senao 2 x rep +
#   1250; rep < 4250 no maximo 5000; semiprofissional (+0x64 = 3) metade por _ftol 0x945640 de x 0.5 [0x968968]) e o
#   treino fraco (+0x7f < 10 com rep > 4250, ou < 14 com rep > 5750); News::send_to_club_manager 0x7861e0 em [0xb7431c].
# Prologo mov eax, fs:[0] (6 bytes). Igual no cm0102-1080.exe (inteira).
# Comparador: so a original (manda a noticia); a prova e o Lucas jogando comparando.
TELAS = True
SAVE = False
ROTINAS = [('post_ultimatum_response_news', 0x6ad930, [0x64, 0xa1, 0x00, 0x00, 0x00, 0x00])]
DOCS = ('a notícia do ultimato do técnico à diretoria', "the news of a manager's ultimatum to the board",
        [('post_ultimatum_response_news', 'MANAGER_MANAGER::post_ultimatum_response_news', '0x6ad930', 'manager_manager.cpp', 'thiscall, ret 0x18',
          'regra: a notícia do ultimato (item 0x178f) cita o técnico, o clube e os quatro códigos da resposta, e diz se o estádio é pequeno para o clube (pela reputação e pela força da liga; semiprofissional pede metade) e se o centro de treinamento é fraco',
          "rule: the ultimatum news (item 0x178f) names the manager, the club and the four answer codes, and says whether the stadium is too small for the club (by its reputation and its league's strength; a part-time club needs half) and whether its training facilities are poor")])
CAMPOS = []
INTERESSA = []
