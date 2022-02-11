use revolt_quark::models::user::PartialUser;
use revolt_quark::*;

#[async_std::main]
async fn main() {
    let db = DatabaseInfo::Dummy.connect().await.unwrap();

    let sus = PartialUser {
        username: Some("neat".into()),
        ..Default::default()
    };

    db.update_user("user id", &sus, vec![]).await.unwrap();
}
