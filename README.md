

# Twitch Client

Este repositório contém o código-fonte e os recursos relacionados a Twitch

## Estrutura do diretório

O repositório possui a seguinte estrutura de diretórios:

- **`src/`**: Este diretório contém o código-fonte main do Twitch Client.
- **`src/hls`**: Este diretório contém o código-fonte de como é tratado hls e erros.
- **`src/controller`**: Este diretório contém o código-fonte de como é feito uma especie de controlador e para ser usado com multithread.


## Arquivos

Aqui estão os principais arquivos do repositório:

- **`src/main.rs`**: Este arquivo contém o código principal do Twitch Client, incluindo a lógica principal.
###
- **`src/hls/errors.rs`**: Este arquivo contém o código para tratamento de error e uma forma de controlar melhor aplicação
- **`src/hls/mod.rs`**: Este arquivo contém o código que cria o modulo hls, ele ta mal feito mas não quis melhorar mais por preguiça e cansado do projeto
- **`src/hls/proxy.rs`**: Este arquivo contém o código que cria o client http, o nome é proxy mas por causa que em teoria seria para colocar proxy mas acabou que mudei de ideia e foi para criar o client middleware. Não mudei o nome mais por preguiça tambem
- **`src/hls/proxy.rs`**: Este arquivo contém o código que cria o client http, o nome é proxy mas por causa que em teoria seria para colocar proxy mas acabou que mudei de ideia e foi para criar o client middleware. Não mudei o nome mais por preguiça tambem
- **`src/hls/http_client.rs`**: Este arquivo contém o código que tem toda logica de requisição e tratamento da twitch e m3u8. Ele ta quase tudo que precisaria ver e o nucleo. Tem muita coisa que queria fazer mais acabei abandonando
- **`src/hls/utils.rs`**: Este arquivo contém o código que são funções para coisas mais simples como fosse ferramentas
###
- **`src/controller/controller.rs`**: Este arquivo contém o código que são funções como intermediario entre o http_client e quem quer utilizar ele.
- **`src/controller/controller.rs`**: Este arquivo contém o código o modulo do controller

## Como usar

Para usar o Twitch Client, siga as etapas abaixo:

1. Clone este repositório em sua máquina local:

   ```bash
   git clone https://github.com/JeanV0/twitch-client.git

2. Tenha rust e rust instalado. Após instalado, execute seguinte comando
	```bash
   cargo install

3. Após isso, execute modo desenvolvimento
	```bash
   cargo run