@echo off
rem Abre o CM 01/02 sem CD, com a camada de video em Rust (jogo\ddraw.dll): nunca troca a resolucao do Windows.
rem Janela ou tela cheia: opcao "Screen Mode" nas definicoes do jogo. Em tela cheia o quadro 800x600 e escalado.
start "" /D "%~dp0jogo" "%~dp0jogo\cm0102-portatil.exe"
