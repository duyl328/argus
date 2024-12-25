use std::time::{Duration, Instant};
use tokio::time::sleep;

async fn async_task(id: usize, delay_secs: u64) {
    sleep(Duration::from_secs(delay_secs)).await;
}

#[tokio::test]
async fn test_async_function() {
    let start = Instant::now();
    // 模拟异步逻辑
    let tasks = vec![
        async_task(1, 2),
        async_task(2, 1),
        async_task(3, 3),
        async_task(3, 3),
        async_task(3, 5),
        async_task(3, 3),
        async_task(3, 3),
        async_task(3, 3),
        async_task(3, 5),
        async_task(3, 3),
        async_task(3, 3),
        async_task(3, 4),
        async_task(3, 3),
    ];
    // 等待所有任务完成
    futures::future::join_all(tasks).await;

    let duration = start.elapsed();
    println!("Test duration: {:?}", duration);
}
