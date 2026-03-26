//! # Async Channel
//!
//! In this exercise, you will use `tokio::sync::mpsc` async channels to implement producer-consumer pattern.
//!
//! ## Concepts
//! - `tokio::sync::mpsc::channel` creates bounded async channels
//! - Async `send` and `recv`
//! - Channel closing mechanism (receiver returns None after all senders are dropped)

use tokio::sync::mpsc;

/// Async producer-consumer:
/// - Create a producer task that sends each element from items sequentially
/// - Create a consumer task that receives all elements and collects them into Vec for return
///
/// Hint: Set channel capacity to items.len().max(1)
pub async fn producer_consumer(items: Vec<String>) -> Vec<String> {
    let (tx, mut rx) = mpsc::channel(items.len().max(1));

    // Spawn producer task: iterate through items, send each one
    let _producer = tokio::spawn(async move {
        for item in items {
            tx.send(item).await.unwrap();
        }
        // tx is dropped here, closing the channel
    });

    // Spawn consumer task: loop recv until channel closes, collect results
    let consumer = tokio::spawn(async move {
        let mut results = Vec::new();
        while let Some(item) = rx.recv().await {
            results.push(item);
        }
        results
    });

    // Wait for consumer to complete and return results
    consumer.await.unwrap()
}

/// Fan‑in pattern: multiple producers, one consumer.
/// Create `n_producers` producers, each sending `"producer {id}: message"`.
/// Consumer collects all messages, sorts them, and returns.
pub async fn fan_in(n_producers: usize) -> Vec<String> {
    let (tx, mut rx) = mpsc::channel(n_producers);

    // Spawn n_producers producer tasks
    let mut handles = vec![];
    for id in 0..n_producers {
        let tx_clone = tx.clone();
        let handle = tokio::spawn(async move {
            let message = format!("producer {id}: message");
            tx_clone.send(message).await.unwrap();
        });
        handles.push(handle);
    }

    // Drop the original sender (important! otherwise channel won't close)
    drop(tx);

    // Wait for all producers to complete
    for handle in handles {
        handle.await.unwrap();
    }

    // Consumer loops receiving, collects and sorts
    let mut results = Vec::new();
    while let Some(message) = rx.recv().await {
        results.push(message);
    }
    results.sort();

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_producer_consumer() {
        let items = vec!["hello".into(), "async".into(), "world".into()];
        let result = producer_consumer(items.clone()).await;
        assert_eq!(result, items);
    }

    #[tokio::test]
    async fn test_producer_consumer_empty() {
        let result = producer_consumer(vec![]).await;
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn test_fan_in() {
        let result = fan_in(3).await;
        assert_eq!(
            result,
            vec![
                "producer 0: message",
                "producer 1: message",
                "producer 2: message",
            ]
        );
    }

    #[tokio::test]
    async fn test_fan_in_single() {
        let result = fan_in(1).await;
        assert_eq!(result, vec!["producer 0: message"]);
    }
}
