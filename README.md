#Instruções para execução
----------------------------------------------------
###1 - Pré-requisitos
* Rustup e Cargo, instruções =>`https://www.rust-lang.org/tools/install`
* Red Panda Streams, instruções => `https://vectorized.io/redpanda`
    * Alternativa ao RedPanda, qualquer distribuição **Apache Kafka**.
* Docker Engine, instruções => `https://docs.docker.com/engine/install/`
    * Alternativa a docker, Podman, instruções => `https://podman.io/getting-started/installation`
* Postman, Instruções=> `https://www.postman.com/downloads/`

###2 - Preparos pré-execução



* RedPanda:

1. Criar tópico onde as mensagens serão entregues:

`rpk topic produce vivaz-agendamentos`

2. Registrar um consumer neste tópico, e manter o terminal neste processo para capturar as mensagens.

`rpk topic consume vivaz-agendamentos`

3. Executar o cluster do redis, usando o docker em outro terminal:

`docker run -e "IP=0.0.0.0" -p 7000-7005:7000-7005 grokzen/redis-cluster:latest`


###3 - Execução

1. Build

`cargo build --release`

2. Execute

`cargo run --release`

O servidore entrará em execução na porta 8080:


###4 - Preparação para os testes

1. Popular a base de dados:

`Execute o Passo 1 do postman.
   Ao executar este passo, o Redis será carrgado com uma base inicial de horários disponíveis para agendamento.`
   
2. Buscar horários disponíveis:

`Execute o passo 2 do postman.
  Ao executar você receberá como retorno uma lista de horários disponíveis.`

3. Agendar o Horário:

`Execute o passo 3 do postman.
   Ao executar este passo você está retirando o horário da lista de horários disponiveis,
   e colocando na lista de horários indisponisponiveis.`
   
4. Verificar se o horárioainda existe na lista:

`Execute o passo 4 do postman.
   Ao executar este passo você receberá o retorno com a lista de horários disponĩveis.
   Verifique se o horário se encontra na lista.`

5. Verificar se alguma mensagem foi consumida no terminal em que executou o rpk para consumir eventos.