#Instruções para execução
----------------------------------------------------
###1 - Pré-requisitos
* Rustup e Cargo, instruções =>`https://www.rust-lang.org/tools/install`
* Red Panda Streams, instruções => `https://vectorized.io/redpanda`
    * Alternativa ao RedPanda, qualquer distribuição **Apache Kafka**.
* Docker Engine, instruções => `https://docs.docker.com/engine/install/`
    * Alternativa a docker, Podman, instruções => `https://podman.io/getting-started/installation`


###2 - Preparos pré-execução



* RedPanda:

Criar tópico onde as mensagens serão entregues:

`rpk topic produce vivaz-agendamentos`


Executar o cluster do redis, usando o docker:
`docker run -e "IP=0.0.0.0" -p 7000-7005:7000-7005 grokzen/redis-cluster:latest`


###3 - Execução

1. Build

`cargo build --release`

2. Execute

`cargo run --release`

O servidore entrará em execução na porta 8080:


