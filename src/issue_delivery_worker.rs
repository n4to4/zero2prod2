use crate::domain::SubscriberEmail;
use crate::email_client::EmailClient;
use sqlx::{PgPool, Postgres, Transaction};
use std::time::Duration;
use tracing::{field::display, Span};
use uuid::Uuid;

struct NewsletterIssue {
    title: String,
    text_content: String,
    html_content: String,
}

enum ExecutionOutcome {
    TaskCompleted,
    EmptyQueue,
}

async fn worker_loop(pool: PgPool, email_client: EmailClient) -> anyhow::Result<()> {
    loop {
        match try_execute_task(&pool, &email_client).await {
            Ok(ExecutionOutcome::EmptyQueue) => {
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
            Err(_) => {
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            Ok(ExecutionOutcome::TaskCompleted) => {}
        }
    }
}

#[tracing::instrument(
    skip_all,
    fields(
        newsletter_issue_id=tracing::field::Empty,
        subscriber_email=tracing::field::Empty,
    ),
    err
)]
async fn try_execute_task(
    pool: &PgPool,
    email_client: &EmailClient,
) -> anyhow::Result<ExecutionOutcome> {
    let (transaction, issue_id, email) = match dequeue_task(pool).await? {
        Some(t) => t,
        None => return Ok(ExecutionOutcome::EmptyQueue),
    };
    Span::current()
        .record("newsletter_issue_id", &display(issue_id))
        .record("subscriber_email", &display(&email));
    match SubscriberEmail::parse(email.clone()) {
        Ok(email) => {
            let issue = get_issue(pool, issue_id).await?;
            if let Err(e) = email_client
                .send_email(
                    &email,
                    &issue.title,
                    &issue.text_content,
                    &issue.html_content,
                )
                .await
            {
                tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "Failed to deliver issue to a confirmed subscriber. \
                    Skipping.",
                );
            }
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "Skipping a confirmed subscriber. \
                Their stored contact details are invalid",
            )
        }
    }
    delete_task(transaction, issue_id, &email).await?;
    Ok(ExecutionOutcome::TaskCompleted)
}

#[tracing::instrument(skip_all)]
async fn get_issue(pool: &PgPool, issue_id: Uuid) -> anyhow::Result<NewsletterIssue> {
    let issue = sqlx::query_as!(
        NewsletterIssue,
        r#"
        select title, text_content, html_content
        from newsletter_issues
        where
            newsletter_issue_id = $1
        "#,
        issue_id
    )
    .fetch_one(pool)
    .await?;
    Ok(issue)
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
