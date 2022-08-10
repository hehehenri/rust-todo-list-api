use std::net::SocketAddr;
use sqlx::MySqlPool;

use crate::router;

pub async fn serve(pool: MySqlPool, addr: SocketAddr) -> Result<(), sqlx::Error> {
    let router = router::new(pool).unwrap();

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
