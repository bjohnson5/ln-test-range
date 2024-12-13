// Standard libraries
use std::collections::HashMap;
use std::vec::Vec;
use std::process::Child;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::io::{stdin, stdout, Read, Write};
use std::env;
use std::path::PathBuf;

// Extra dependencies
use ctrlc;
use simple_logger::SimpleLogger;
use log::LevelFilter;
use tokio::task::JoinSet;

// Blast libraries
use blast_core::Blast;

const NUM_LND: i32 = 45;
const NUM_LDK: i32 = 45;
const NUM_CLN: i32 = 10;

#[tokio::main]
async fn main() {
    let home = env::var("HOME").expect("HOME environment variable not set");
    let folder_path = PathBuf::from(home).join(".blast/");
    std::fs::create_dir_all(folder_path.display().to_string()).unwrap();

    SimpleLogger::new()
    .with_level(LevelFilter::Info)
    .init()
    .unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify a models directory");
        return;
    }

    println!("BLAST starting up...");

    // Set up a Ctrl+C signal handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    // Create the blast core object
    let mut blast = Blast::new(String::from(args[1].clone()));

    let mut m = HashMap::new();
    m.insert(String::from("blast_lnd"), NUM_LND);
    m.insert(String::from("blast_ldk"), NUM_LDK);
    m.insert(String::from("blast_cln"), NUM_CLN);
    let models = match blast.create_network("test", m, running.clone()).await {
        Ok(m) => m,
        Err(e) => {
            println!("Failed to start network: {}", e);
            return;
        }
    };

    // Connect peers
    println!("Connecting peers...");
    match connect(&mut blast).await {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to connect nodes: {}", e);
            stop(blast, models, None, running).await;
            return;
        }
    }

    // Fund the nodes
    println!("Funding nodes...");
    match fund(&mut blast).await {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to fund nodes: {}", e);
            stop(blast, models, None, running).await;
            return;
        }
    }

    // Open channels
    println!("Opening channels...");
    match open(&mut blast).await {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to open channels: {}", e);
            stop(blast, models, None, running).await;
            return;
        }
    }

    // Add payment activity
    println!("Adding payment activity...");
    match activity(&mut blast).await {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to add activity: {}", e);
            stop(blast, models, None, running).await;
            return;
        }
    }

    // Add events
    println!("Adding events...");
    match events(&mut blast).await {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to add events: {}", e);
            stop(blast, models, None, running).await;
            return;
        }
    }

    pause();

    // Finalize the simulation and make it ready to run
    match blast.finalize_simulation().await {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to finalize the simulation: {:?}", e);
            stop(blast, models, None, running).await;
            return;
        }
    }

    // Start the simulation
    let sim_tasks = match blast.start_simulation().await {
        Ok(j) => j,
        Err(e) => {
            println!("Failed to start the simulation: {:?}", e);
            stop(blast, models, None, running).await;
            return;
        }
    };

    pause();

    // Stop the simulation
    stop(blast, models, Some(sim_tasks), running).await;
    println!("BLAST shutting down...");
}

async fn stop(mut blast: Blast, models: Vec<Child>, sim_tasks: Option<JoinSet<()>>, running: Arc<AtomicBool>) {
    match sim_tasks {
        Some(mut t) => {
            // Stop the simulation
            blast.stop_simulation();
            while let Some(res) = t.join_next().await {
                if let Err(_) = res {
                    println!("Error waiting for simulation to stop");
                }
            }
        },
        None => {}
    }

    // Stop the nodes
    match blast.stop_network().await {
        Ok(_) => {},
        Err(e) => {
            println!("Failed to stop the network: {:?}", e);
        }
    }

    // Wait for the models to stop
    for mut child in models {
        let exit_status = match child.wait() {
            Ok(s) => Some(s),
            Err(e) => {
                println!("Failed to wait for child process: {:?}", e);
                None
            }
        };
        println!("Model process exited with status: {:?}", exit_status);
    }

    running.store(false, Ordering::SeqCst);
}

async fn connect(blast: &mut Blast) -> Result<(), String> {
    let _ = blast.connect_peer(blast_lnd_node(0), blast_cln_node(0)).await?;
    let _ = blast.connect_peer(blast_cln_node(1), blast_lnd_node(1)).await?;
    let _ = blast.connect_peer(blast_lnd_node(2), blast_ldk_node(0)).await?;
    let _ = blast.connect_peer(blast_cln_node(3), blast_ldk_node(1)).await?;
    let _ = blast.connect_peer(blast_ldk_node(0), blast_cln_node(2)).await?;
    let _ = blast.connect_peer(blast_ldk_node(1), blast_lnd_node(3)).await?;
    let _ = blast.connect_peer(blast_cln_node(7), blast_ldk_node(2)).await?;
    let _ = blast.connect_peer(blast_cln_node(8), blast_ldk_node(3)).await?;
    let _ = blast.connect_peer(blast_cln_node(9), blast_ldk_node(4)).await?;
    let _ = blast.connect_peer(blast_ldk_node(2), blast_ldk_node(5)).await?;
    let _ = blast.connect_peer(blast_ldk_node(3), blast_lnd_node(4)).await?;
    let _ = blast.connect_peer(blast_ldk_node(4), blast_lnd_node(4)).await?;
    let _ = blast.connect_peer(blast_ldk_node(5), blast_lnd_node(4)).await?;

    for i in 5..25 {
        let _ = blast.connect_peer(blast_lnd_node(4), blast_lnd_node(i)).await?;
    }

    for i in 25..45 {
        let _ = blast.connect_peer(blast_lnd_node(4), blast_ldk_node(i)).await?;
    }

    for i in 5..25 {
        let _ = blast.connect_peer(blast_lnd_node(i), blast_ldk_node(i)).await?;
        let _ = blast.connect_peer(blast_ldk_node(i), blast_cln_node(5)).await?;
    }

    for i in 25..45 {
        let _ = blast.connect_peer(blast_ldk_node(i), blast_lnd_node(i)).await?;
        let _ = blast.connect_peer(blast_lnd_node(i), blast_cln_node(6)).await?;
    }

    Ok(())
}

async fn fund(blast: &mut Blast) -> Result<(), String> {
    for i in 0..NUM_LND {
        if i == 4 {
            continue;
        }
        let _ = blast.fund_node(blast_lnd_node(i), 1.0, false).await?;
    }

    for i in 0..NUM_LDK {
        let _ = blast.fund_node(blast_ldk_node(i), 1.0, false).await?;
    }

    for i in 0..NUM_CLN {
        let _ = blast.fund_node(blast_cln_node(i), 1.0, false).await?;
    }

    for _ in 0..39 {
        let _ = blast.fund_node(blast_lnd_node(4), 0.025, false).await?;
    }
    let _ = blast.fund_node(blast_lnd_node(4), 0.025, true).await?;

    Ok(())
}

async fn open(blast: &mut Blast)-> Result<(), String> {
    let mut id = 12;
    for i in 5..25 {
        let _ = blast.open_channel(blast_lnd_node(4), blast_lnd_node(i), 500000, 0, id, true).await?;
        id = id+1;
    }

    for i in 25..45 {
        let _ = blast.open_channel(blast_lnd_node(4), blast_ldk_node(i), 500000, 0, id, true).await?;
        id = id+1;
    }

    for i in 5..25 {
        let _ = blast.open_channel(blast_lnd_node(i), blast_ldk_node(i), 500000, 0, id, false).await?;
        let _ = blast.open_channel(blast_ldk_node(i), blast_cln_node(5), 500000, 0, id, false).await?;
        id = id+1;
    }

    for i in 25..45 {
        let _ = blast.open_channel(blast_ldk_node(i), blast_lnd_node(i), 500000, 0, id, false).await?;
        let _ = blast.open_channel(blast_lnd_node(i), blast_cln_node(6), 500000, 0, id, false).await?;
        id = id+1;
    }

    let _ = blast.open_channel(blast_lnd_node(0), blast_cln_node(0), 500000, 0, 0, false).await?;
    let _ = blast.open_channel(blast_cln_node(1), blast_lnd_node(1), 500000, 0, 1, false).await?;
    let _ = blast.open_channel(blast_lnd_node(2), blast_ldk_node(0), 500000, 0, 2, false).await?;
    let _ = blast.open_channel(blast_cln_node(3), blast_ldk_node(1), 500000, 0, 3, false).await?;
    let _ = blast.open_channel(blast_ldk_node(0), blast_cln_node(2), 500000, 0, 4, false).await?;
    let _ = blast.open_channel(blast_ldk_node(1), blast_lnd_node(3), 500000, 0, 5, false).await?;
    let _ = blast.open_channel(blast_cln_node(7), blast_ldk_node(2), 500000, 0, 6, false).await?;
    let _ = blast.open_channel(blast_cln_node(8), blast_ldk_node(3), 500000, 0, 7, false).await?;
    let _ = blast.open_channel(blast_cln_node(9), blast_ldk_node(4), 500000, 0, 8, false).await?;
    let _ = blast.open_channel(blast_ldk_node(2), blast_ldk_node(5), 500000, 0, 9, false).await?;
    let _ = blast.open_channel(blast_ldk_node(3), blast_lnd_node(4), 500000, 0, 10, false).await?;
    let _ = blast.open_channel(blast_ldk_node(4), blast_lnd_node(4), 500000, 0, 11, false).await?;
    let _ = blast.open_channel(blast_ldk_node(5), blast_lnd_node(4), 500000, 0, id, true).await?;

    Ok(())
}

async fn activity(blast: &mut Blast)-> Result<(), String> {
    blast.add_activity(&blast_lnd_node(0), &blast_cln_node(0), None, None, 1, 2000);
    blast.add_activity(&blast_cln_node(1), &blast_lnd_node(1), None, None, 1, 3000);
    blast.add_activity(&blast_lnd_node(2), &blast_cln_node(2), None, None, 1, 4000);
    blast.add_activity(&blast_cln_node(3), &blast_lnd_node(3), None, None, 1, 5000);
    blast.add_activity(&blast_lnd_node(4), &blast_cln_node(5), None, None, 5, 6000);
    blast.add_activity(&blast_lnd_node(4), &blast_cln_node(6), None, None, 5, 7000);
    blast.add_activity(&blast_cln_node(7), &blast_lnd_node(4), None, None, 10, 8000);
    blast.add_activity(&blast_cln_node(8), &blast_lnd_node(4), None, None, 10, 9000);
    blast.add_activity(&blast_cln_node(9), &blast_lnd_node(4), None, None, 10, 9500);

    Ok(())
}

async fn events(_blast: &mut Blast)-> Result<(), String> {
    //let mut good_close = Vec::new();
    //good_close.push(String::from("blast_lnd-0000"));
    //good_close.push(String::from("0"));
    //let _ = blast.add_event(10, "CloseChannel", Some(good_close.clone()))?;
    Ok(())
}

fn blast_lnd_node(id: i32) -> String {
    format!("{}{:04}", "blast_lnd-", id)
}

fn blast_ldk_node(id: i32) -> String {
    format!("{}{:04}", "blast_ldk-", id)
}

fn blast_cln_node(id: i32) -> String {
    format!("{}{:04}", "blast_cln-", id)
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}
