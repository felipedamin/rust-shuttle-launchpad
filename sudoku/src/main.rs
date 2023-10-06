use axum::{routing::post, Router, Json, response::{IntoResponse, Response}, http::StatusCode};

// docs: https://doc.rust-lang.org/reference/items/modules.html
// useful explanation about module system in rust: https://spin.atomicobject.com/2022/01/24/rust-module-system/
mod sudoku;
use sudoku::{Sudoku};

// another way is to use return a Result: Result<Json<Sudoku>, StatusCode>
// ex: return Ok(Json(sudoku)) or return Err(StatusCode::BAD_REQUEST)
async fn solve(Json(mut sudoku): Json<Sudoku>) -> Response {
    let solution_exists = Sudoku::solve(&mut sudoku);

    if solution_exists {
        return Json(sudoku).into_response();
    }
    return (
        StatusCode::BAD_REQUEST,
        "Something went wrong...",
    ).into_response();

}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/solve", post(solve));

    Ok(router.into())
}
