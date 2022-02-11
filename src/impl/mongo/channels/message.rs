use futures::try_join;
use mongodb::options::FindOptions;

use crate::models::message::{Message, MessageSort, PartialMessage};
use crate::{AbstractMessage, Result};

use super::super::MongoDb;

static COL: &str = "messages";

#[async_trait]
impl AbstractMessage for MongoDb {
    async fn fetch_message(&self, id: &str) -> Result<Message> {
        self.find_one_by_id(COL, id).await
    }

    async fn insert_message(&self, message: &Message) -> Result<()> {
        self.insert_one(COL, message).await.map(|_| ())
    }

    async fn update_message(&self, id: &str, message: &PartialMessage) -> Result<()> {
        self.update_one_by_id(COL, id, message, vec![], None)
            .await
            .map(|_| ())
    }

    async fn delete_message(&self, id: &str) -> Result<()> {
        self.delete_one_by_id(COL, id).await.map(|_| ())
    }

    async fn fetch_messages(
        &self,
        channel: &str,
        limit: Option<i64>,
        before: Option<String>,
        after: Option<String>,
        sort: Option<MessageSort>,
        nearby: Option<String>,
    ) -> Result<Vec<Message>> {
        let limit = limit.unwrap_or(50);
        Ok(if let Some(nearby) = nearby {
            let (a, b) = try_join!(
                self.find_with_options::<_, Message>(
                    COL,
                    doc! {
                        "channel": channel,
                        "_id": {
                            "$gte": &nearby
                        }
                    },
                    FindOptions::builder()
                        .limit(limit / 2 + 1)
                        .sort(doc! {
                            "_id": 1_i32
                        })
                        .build(),
                ),
                self.find_with_options::<_, Message>(
                    COL,
                    doc! {
                        "channel": channel,
                        "_id": {
                            "$lt": &nearby
                        }
                    },
                    FindOptions::builder()
                        .limit(limit / 2)
                        .sort(doc! {
                            "_id": -1_i32
                        })
                        .build(),
                )
            )?;

            [a, b].concat()
        } else {
            let mut query = doc! { "channel": channel };
            if let Some(before) = before {
                query.insert("_id", doc! { "$lt": before });
            }

            if let Some(after) = after {
                query.insert("_id", doc! { "$gt": after });
            }

            let sort: i32 = if let MessageSort::Latest = sort.unwrap_or(MessageSort::Latest) {
                -1
            } else {
                1
            };

            self.find_with_options::<_, Message>(
                COL,
                query,
                FindOptions::builder()
                    .limit(limit)
                    .sort(doc! {
                        "_id": sort
                    })
                    .build(),
            )
            .await?
        })
    }

    async fn search_messages(
        &self,
        channel: &str,
        query: &str,
        limit: Option<i64>,
        before: Option<String>,
        after: Option<String>,
        sort: MessageSort,
    ) -> Result<Vec<Message>> {
        let limit = limit.unwrap_or(50);

        let mut filter = doc! {
            "channel": channel,
            "$text": {
                "$search": query
            }
        };

        if let Some(doc) = match (before, after) {
            (Some(before), Some(after)) => Some(doc! {
                "lt": before,
                "gt": after
            }),
            (Some(before), _) => Some(doc! {
                "lt": before
            }),
            (_, Some(after)) => Some(doc! {
                "gt": after
            }),
            _ => None,
        } {
            filter.insert("_id", doc);
        }

        self.find_with_options(
            COL,
            filter,
            FindOptions::builder()
                .projection(if let MessageSort::Relevance = &sort {
                    doc! {
                        "score": {
                            "$meta": "textScore"
                        }
                    }
                } else {
                    doc! {}
                })
                .limit(limit)
                .sort(match &sort {
                    MessageSort::Relevance => doc! {
                        "score": {
                            "$meta": "textScore"
                        }
                    },
                    MessageSort::Latest => doc! {
                        "_id": -1_i32
                    },
                    MessageSort::Oldest => doc! {
                        "_id": 1_i32
                    },
                })
                .build(),
        )
        .await
    }
}
