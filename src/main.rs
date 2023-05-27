mod hls;
mod controller;
use std::{env, sync::{Mutex, Arc}, time::Duration};

use std::thread::sleep;

use tokio::sync::{mpsc, mpsc::Receiver};

use controller::{Controller};
use dotenv::dotenv;
use hls::ErrorHttpM3u8;

// Uma forma de organizar meu sistema de multithread
async fn multi_controller(thread: i64) {

    // Cada processo tera seu proprio canal de mensagens e será guardado
    // Motivo disso é para o programa so acabar quando todas as threads acabarem    
    let mut processes: Vec<Arc<Mutex<Receiver<String>>>> = Vec::new();
    // Esse counter era para contar quantidade feita mas é uma ideia abandonada
    let mut counter: i64 = 0;
    
    // Não sei se isso se encaixa como gambiarra
    // Mas foi uma ideia que tive para realizar essa operação
    for _ in 0..thread {
        // Criando os canais e guardando
        let (tx, mut rx) = mpsc::channel::<String>(100);
        processes.push(Arc::new(Mutex::new(rx)));
        
        // Chamando biblioteca tokio para criar essa parte for thread separada
        tokio::spawn(async move {
            // Pegando variavel do .env
            // Channel é o canal da twitch
            // Quantity é quantidade de anuncios
            // Timeout é apos realizar ação, ele esperar um tempo. Nome contra intuitivo  
            let channel = env::var("CHANNEL").unwrap();
            let quantity: i64 = env::var("QUANTITY").unwrap().parse().unwrap();
            let timeout: u64 = env::var("TIMEOUT").unwrap().parse().unwrap();
            match Controller::new( quantity,timeout,channel).await {
                Ok(controller) => {
                    // So para sinalizar que estar rodando  
                    println!("Running...");
                    // Para não precisar ter problemas e controle melhor, usei match
                    match controller.run(&mut counter).await {
                        Ok(_) => {
                            // Deu certo
                            if let Ok(_) = tx.send(String::from("Nao deu merda")).await {
                                println!("Tarefa terminada");
                                return;
                            }
                        },
                        Err(err) => {
                            // Code horse em ação kkkkk
                            // Sim, poderia criar tratativa melhores mas tava sem vontade 
                            // Tipo, se a live é +18, ou se foi problema de internet. Ou se live é VOD e outros
                            if let Ok(_) = tx.send(String::from("Deu merda")).await {
                                println!("Error: {}", err);
                                return;
                            }
                        },
                    } 
                },
                Err(err) => {
                    if let Ok(_) = tx.send(String::from("Deu merda")).await {
                        println!("Error: {}", err);
                        return;
                    }
                } 
            }
            // Nessa etapa so acontece quando o receptor usa .close()  
            if let Err(_) = tx.send(String::from("Deu merda")).await {
                println!("O receptor foi descartado");
                return;
            }
        });  
    }

    // Aqui era para ser o contador enquanto as threads mexiam no ponteiro e tals mas...
    // Apenas desistir, podia sim utilizar outras formas mas nao quis mesmo
    tokio::spawn(async move {
        let quantity: i64 = env::var("QUANTITY").unwrap().parse().unwrap();
        while counter < quantity {
            println!("Ads counter: {}", counter); 
            sleep(Duration::from_secs(2));
        }
    });

    // Apenas esperando os processos acabarem
    for process in processes {
        while let Some(_) = process.lock().unwrap().recv().await  {}
    }
}

#[tokio::main]
async fn main() -> Result<(), ErrorHttpM3u8> {
    dotenv().ok();
    let thread: i64 = env::var("THREAD").unwrap().parse().unwrap();

    // Uma forma de organizar a execução    
    multi_controller(thread).await;

    Ok(())

}