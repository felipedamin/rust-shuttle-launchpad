use std::sync::Arc;
use axum::{routing::get, Router, extract::{State, Path}, response::{IntoResponse, Html}};
mod Podcast;

// tutorial followed from: https://www.shuttle.rs/launchpad/issues/2023-06-16-issue-02-Structs-and-Enums
type AppState = Arc<Vec<Podcast::Podcast>>;

async fn podcast(
    State(app_state): State<AppState>, Path(id): Path<usize>
) -> impl IntoResponse {
    let podcast = app_state.get(id);
    Html(match podcast {
        Some(podcast) => podcast.to_html(),
        None => "No podcast found".to_string(),
    })
}

async fn root(
    State(app_state): State<AppState>
) -> impl IntoResponse {
    let response = format!(
        r#"
        <html>
            <head>
                <title>My Podcasts</title>
            </head>
            <body>
                <h1>My Podcasts</h1>
                <ul>
                    {}
                </ul>
            </body>
        </html>
        "#,
        app_state
            .iter()
            .enumerate()
            .map(|(id, podcast)| {
                format!(r#"<li><a href="/{}">{}</a></li>"#, id, podcast.title)
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
    Html(response)
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let podcasts = Podcast::read_podcasts_from_xml("https://workingdraft.de/feed/").await?;
    let app_state = Arc::new(podcasts);
    
    let router = Router::new()
        .route("/", get(root))
        .route("/:id", get(podcast))
        .with_state(app_state);

    Ok(router.into())
}
