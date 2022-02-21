use redis_kiss::{AsyncCommands, Conn};

use super::entry::PresenceEntry;

pub async fn __set_key(conn: &mut Conn, id: &str, data: Vec<PresenceEntry>) {
    let _: Option<()> = conn.set(id, bincode::serialize(&data).unwrap()).await.ok();
}

pub async fn __delete_key(conn: &mut Conn, id: &str) {
    let _: Option<()> = conn.del(id).await.ok();
}

pub async fn __get_key(conn: &mut Conn, id: &str) -> Option<Vec<PresenceEntry>> {
    conn.get::<_, Option<Vec<u8>>>(id)
        .await
        .unwrap()
        .map(|entry| bincode::deserialize(&entry[..]).unwrap())
}
