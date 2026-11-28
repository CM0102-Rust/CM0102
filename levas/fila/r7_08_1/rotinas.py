# A transferencia futura combinada pelo arquivo de configuracao dos jogadores (transfer_manager.cpp; leitor r7_08,
# rodada 7, 28/11/2026): estava adiada no levas/fila/rxs1_12/FORA.md, rxg_5/FORA.md e rxf_7/FORA.md por so rodar com uma
# linha FUTURE_TRANSFER no player_setup.cfg (o motivo era a falta do arquivo de teste e o tempo; nao e thunk, corpo
# vazio nem entrada falsa). Rotina inteira:
# - 0x8bf060 TRANSFER_MANAGER::add_config_future_transfer (928 bytes, ate o ret 0x18 em 0x8bf3f7; quadro SEH da oferta
#   local, destruida no fim por 0x8e5710).
# Simulacao (criacao do jogo): chamada so por 0x7a4cfc, a player_setup_apply_line (r7_07_1, que a chama pelo endereco
# do exe com a mesma assinatura). O Data/player_setup.cfg do jogo_testes NAO tem linha FUTURE_TRANSFER (184 INJURY,
# 49 INT_RETIREMENT, 56 LOAN, 1 MISCONDUCT): contagem 0 na cobertura do mundo. Ver o PRONTO.txt sobre as mutacoes.
# Prologo: push -1 / push 0x963b16 (7 bytes), sem relativas. Igual no cm0102-1080.exe (a rotina inteira, conferido
# byte a byte).
# Convencoes conferidas na desmontagem:
#   0x8bf060 thiscall([0xde1f54], pessoa, clube, &data, taxa, salario, anos) ret 0x18 (taxa -1 = o valor da pessoa;
#   salario -1 = o desejado limitado ao teto; anos lidos como byte, < 1 = random(4) + 2).
#   Dentro: 0x8e5680 / 0x8e5710 thiscall(oferta); 0x85e410 thiscall(contrato); 0x4cf5a0 thiscall([0xae1050], pessoa);
#   0x8c5cf0 thiscall(tm, pessoa, 0, 0); 0x944cff/0x90d130/0x944c9f/0x5e8290 (a caixa "Error", show_error_box);
#   0x53dba0 cdecl(pessoa, 0xde1f64, 0x7d0, 1); 0x8e5d60 thiscall(oferta, pessoa, clube, 0, taxa, taxa == 0, 0);
#   0x8be300 thiscall(tm, &oferta, 0); 0x85fc50 thiscall(contrato, pessoa, clube, vaga, 2, 0xb, 0, 0);
#   0x5a0590 thiscall([0xae2a38], clube) + 0x5968d0 thiscall(financas, pessoa, -1); 0x945640 _ftol; 0x90cfc0 cdecl(4);
#   0x417c10 Date::operator=; 0x54c5b0 thiscall(&data, &saida, 365 x anos) (so os 16 bits de baixo contam);
#   0x4cc490 thiscall([0xae1050], &contrato, vaga, serie, 0xb, 0, 0, 1); 0x8cb9a0 thiscall(tm, oferta da vaga, &data);
#   0x544670 cdecl(nacao); 0x54c8e0 thiscall(&data, &saida, 1).
# O original monta os 365 x anos em eax com os 16 bits de cima vindos do ponteiro devolvido pelo Date::operator= (lixo);
# o 0x54c5b0 so le si: a copia passa o valor limpo.
# Leitura de memoria: a pessoa, o contrato, a oferta e o contrato locais; nenhuma local lida antes de gravada.
# Comparador: so a original (acrescenta a oferta, a proposta de contrato e a transferencia futura); o plano conferido
# na saida pela contagem de ofertas da pessoa (0x8c5cf0): sem ofertas antes, alguma depois; com ofertas, a mesma.
SAVE = False
ROTINAS = [('add_config_future_transfer', 0x8bf060, [0x6a, 0xff, 0x68, 0x16, 0x3b, 0x96, 0x00])]
DOCS = ('a transferência futura combinada pelo player_setup.cfg na criação do jogo: a proposta, o contrato e o registro da transferência futura',
        'the future transfer the player_setup.cfg arranges on game creation: the bid, the contract and the future transfer record',
        [('add_config_future_transfer', 'TRANSFER_MANAGER::add_config_future_transfer', '0x8bf060', 'transfer_manager.cpp', 'thiscall, ret 0x18',
          'arma uma transferência futura de uma linha FUTURE_TRANSFER: taxa -1 vira o valor da pessoa; quem já tem proposta recebe duas caixas "Error" (linhas 0x6ce e 0x6d0) e nada mais. Senão, a proposta (com "livre" quando a taxa é 0; o momento com o nibble 3; cargo 11) entra no gerente; o contrato desejado (vaga da proposta, 2, cargo 11) fica com o salário dado ou, com -1, o desejado limitado ao teto do clube; a multa rescisória, se houver, vira o maior entre taxa + 250.000 e 1,5 × taxa; anos abaixo de 1 viram 2 a 5 (sorteio); começa na data e termina 365 × anos dias depois. A proposta de contrato vai ao gerente de contratos e a proposta vira transferência futura na data. Passe livre (taxa 0) de quem tem contrato num clube de nação que permite passe livre: o contrato atual fica "saindo pela Bosman", perde o bit 3 da situação de transferência e termina na véspera da data',
          'sets up a future transfer from a FUTURE_TRANSFER line: a fee of -1 becomes the person\'s value; someone who already has an offer gets two "Error" boxes (lines 0x6ce and 0x6d0) and nothing else. Otherwise the bid ("free" when the fee is 0; timing nibble 3; job 11) goes into the manager; the desired contract (the offer\'s slot, 2, job 11) gets the given wage or, with -1, the desired one within the club\'s cap; a release fee, if any, becomes the larger of fee + 250,000 and 1.5 × fee; years below 1 become 2 to 5 (drawn); it starts on the date and ends 365 × years days later. The contract offer goes to the contract manager and the bid becomes a future transfer on the date. A free transfer (fee 0) of someone under contract at a club whose nation allows free transfers: the current contract is marked "leaving on a Bosman", loses bit 3 of the transfer status and ends the day before the date')])
INTERESSA = ['CMRTE', 'SGE']
