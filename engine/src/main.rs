/**
 * Main entry point for Rust engine
 * This would be the executable that runs the engine as a service
 */
use sentinel_engine::*;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize memory manager
    let memory_manager = Arc::new(MemoryManager::new("./storage")?);
    
    // Initialize task manager
    let task_manager = Arc::new(TaskManager::new(memory_manager.clone()));
    
    // Initialize scheduler
    let scheduler = Arc::new(Scheduler::new(task_manager.clone()));
    
    // Start scheduler loop in background
    let scheduler_clone = scheduler.clone();
    tokio::spawn(async move {
        if let Err(e) = scheduler_clone.start_scheduler_loop().await {
            eprintln!("Scheduler error: {}", e);
        }
    });
    
    // Initialize IPC layer
    let (ipc_layer, request_rx, response_tx) = IpcLayer::new(task_manager.clone());
    
    // Start IPC handler loop
    tokio::spawn(async move {
        let mut receiver = request_rx;
        while let Some(request) = receiver.recv().await {
            if let Err(e) = ipc_clone.handle_request(request, &response_tx).await {
                eprintln!("IPC handler error: {}", e);
            }
        }
    });
    
    println!("Sentinel Engine started");
    println!("Waiting for IPC requests...");
    
    // Keep main thread alive
    tokio::signal::ctrl_c().await?;
    println!("Shutting down...");
    
    Ok(())
}

