use futures::future::join_all;
async fn task_spawner(){
    let tasks = vec![
        task::spawn(my_task(Duration::from_secs(1))),
        task::spawn(my_task(Duration::from_secs(2))),
        task::spawn(my_task(Duration::from_secs(3))),
    ];
    // If we do not await these tasks and the function finishes, they will be dropped
    join_all(tasks).await;
}