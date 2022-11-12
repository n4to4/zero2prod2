use crate::email_client::EmailClient;
use sqlx::{PgPool, Postgres, Transaction};
use tracing::{field::display, Span};
use uuid::Uuid;

#[tracing::instrument(
    skip_all,
    fields(
        newsletter_issue_id=tracing::field::Empty,
        subscriber_email=tracing::field::Empty,
    ),
    err
)]
async fn try_execute_task(pool: &PgPool, email_client: &EmailClient) -> anyhow::Result<()> {
    if let Some((transaction, issue_id, email)) = dequeue_task(pool).await? {
        Span::current()
            .record("newsletter_issue_id", &display(issue_id))
            .record("subscriber_email", &display(&email));
        // TODO: send email
        delete_task(transaction, issue_id, &email).await?
    }
    Ok(())
}

type PgTransaction = Transaction<'static, Postgres>;

#[tracing::instrument(skip_all)]
async fn dequeue_task(pool: &PgPool) -> anyhow::Result<Option<(PgTransaction, Uuid, String)>> {
    let mut transaction = pool.begin().await?;
    let r = sqlx::query!(
        r#"
        select newsletter_issue_id, subscriber_email
        from issue_delivery_queue
        for update
        skip locked
        limit 1
        "#
    )
    .fetch_optional(&mut transaction)
    .await?;
    if let Some(r) = r {
        Ok(Some((
            transaction,
            r.newsletter_issue_id,
            r.subscriber_email,
        )))
    } else {
        Ok(None)
    }
}

#[tracing::instrument(skip_all)]
async fn delete_task(
    mut transaction: PgTransaction,
    issue_id: Uuid,
    email: &str,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        delete from issue_delivery_queue
        where
            newsletter_issue_id = $1 and
            subscriber_email = $2
        "#,
        issue_id,
        email,
    )
    .execute(&mut transaction)
    .await?;
    transaction.commit().await?;
    Ok(())
}
