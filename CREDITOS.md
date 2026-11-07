*[English version](CREDITOS.en.md).*

# Créditos

Este projeto reescreve em Rust, rotina por rotina, o Championship Manager 01/02 (3.9.68). Ele se apoia no trabalho de muita gente da comunidade.

## O mapa de funções de maxevans60

O mapa [cm0102-function-map](https://github.com/maxevans60/cm0102-function-map), de **maxevans60**, é distribuído sob a licença [Creative Commons Atribuição 4.0 (CC BY 4.0)](https://creativecommons.org/licenses/by/4.0/deed.pt-br), com permissão extra para uso com IA.

O mapa dá nome, arquivo-fonte, convenção, propósito e confiança às 6.863 funções do `cm0102.exe` 3.9.68, e nome, tipo e leitores/escritores aos 4.428 globais. Foi feito para o "Bosman Edition", um porte recompilado do jogo.

Aqui, o mapa:
- é lido pelas ferramentas de `engenharia-reversa/` (o mapa fica fora do repositório, em `referencias/`);
- deu origem, com alterações (as nossas medições de cobertura e os nossos nomes), a [`docs/mapa_funcoes.csv`](docs/mapa_funcoes.csv), [`docs/PLACAR_ARQUIVOS.md`](docs/PLACAR_ARQUIVOS.md) e [`docs/MAPA_COMPETICOES.md`](docs/MAPA_COMPETICOES.md);
- dá os nomes originais das rotinas citados no código e na documentação.

Obrigado a maxevans60, e aos que ele credita: John Locke, Tapani, Saturn e o tópico Offset Index.

## Fórum e patches da comunidade

Os endereços, os patches e os tópicos antigos do fórum que serviram de pista estão listados, com os autores, em [`docs/ENDERECOS_COMUNIDADE.md`](docs/ENDERECOS_COMUNIDADE.md). Obrigado a todos eles e a quem mantém o arquivo dos tópicos antigos em nic.opik.net.

## O jogo

Championship Manager 01/02 é da Sports Interactive e da Eidos. O jogo foi liberado gratuitamente em 2008. Este repositório não contém nada do jogo (nem o executável, nem os dados); para jogar, é preciso a instalação original.
