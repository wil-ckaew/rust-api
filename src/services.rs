use actix_web::{
    get,
    post,
    delete,
    patch,
    web::{
        Data,
        Json,
        scope,
        Query,
        Path,
        ServiceConfig
    },
    HttpResponse,
    Responder
};

use serde_json::json;

use crate::{
    model::{TaskModel, DocumentModel},
    schema::{CreateTaskSchema, CreateDocumentSchema, FilterOptions, UpdateTaskSchema, UpdateDocumentSchema},
    AppState
};
use sqlx::PgPool;
use uuid::Uuid;

// Endpoint de verificação de saúde
#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Health check: API is up and running smoothly.";

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }))
}

// Endpoint para criar uma tarefa
#[post("/task")]
async fn create_task(
    body: Json<CreateTaskSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO tasks (title, content)
        VALUES ($1, $2)
        RETURNING id, title, content, created_at
    "#;

    match sqlx::query_as::<_, TaskModel>(query)
        .bind(&body.title)
        .bind(&body.content)
        .fetch_one(&data.db)
        .await
    {
        Ok(task) => {
            let response = json!({
                "status": "success",
                "task": {
                    "id": task.id,
                    "title": task.title,
                    "content": task.content,
                    "created_at": task.created_at
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to create task: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

// Endpoint para criar um documento
#[post("/documents")]
async fn create_document(
    body: Json<CreateDocumentSchema>,
    data: Data<AppState>
) -> impl Responder {
    let query = r#"
        INSERT INTO documents (user_id, doc_type, filename)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, doc_type, filename, created_at
    "#;

    // O filename é gerado automaticamente; ajuste conforme necessário
    let filename = format!("document_{}.jpg", Uuid::new_v4());

    match sqlx::query_as::<_, DocumentModel>(query)
        .bind(&body.user_id)
        .bind(&body.doc_type)
        .bind(&filename)
        .fetch_one(&data.db)
        .await
    {
        Ok(document) => {
            let response = json!({
                "status": "success",
                "document": {
                    "id": document.id,
                    "user_id": document.user_id,
                    "doc_type": document.doc_type,
                    "filename": document.filename,
                    "created_at": document.created_at
                }
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to create document: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/tasks")]
pub async fn get_all_tasks(opts: Query<FilterOptions>, data: Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match
        sqlx
            ::query_as!(
                TaskModel,
                "SELECT * FROM tasks ORDER by id LIMIT $1 OFFSET $2",
                limit as i32,
                offset as i32
            )
            .fetch_all(&data.db)
            .await {
                Ok(task) => {
                    let task_note = json!({
                        "status": "success",
                        "task": task
                    });


                  return HttpResponse::Ok().json(task_note);
                }

                Err(error) => {

                    return HttpResponse::InternalServerError().json(
                        json!({
                            "status": "error",
                            "message": format!("{:?}", error)
                        })
                    )
                }
            }
}


#[get("/tasks/{id}")]
async fn get_task_by_id(path: Path<uuid::Uuid>, data: Data<AppState>) -> impl Responder {
  let task_id = path.into_inner();

  let query_result = sqlx
        ::query_as!(TaskModel, "SELECT * FROM tasks WHERE id = $1", task_id)
        .fetch_one(&data.db).await;

    match query_result {
        Ok(task) => {
            let task_note = json!({
                "status": "success",
                "task": task
            });


            return HttpResponse::Ok().json(task_note);
        }

        Err(error) => {

            return HttpResponse::InternalServerError().json(
                json!({
                    "status": "error",
                    "message": format!("{:?}", error)
                })
            )
        }
    }
}

// get documents 
#[get("/documents")]
pub async fn get_all_documents(
    opts: Query<FilterOptions>,
    data: Data<AppState>
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match sqlx::query_as!(
        DocumentModel,
        "SELECT * FROM documents ORDER BY id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    {
        Ok(documents) => {
            let response = json!({
                "status": "success",
                "documents": documents
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get documents: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[get("/documents/{id}")]
pub async fn get_document_by_id(
    path: Path<Uuid>,
    data: Data<AppState>
) -> impl Responder {
    let document_id = path.into_inner();

    match sqlx::query_as!(
        DocumentModel,
        "SELECT * FROM documents WHERE id = $1",
        document_id
    )
    .fetch_one(&data.db)
    .await
    {
        Ok(document) => {
            let response = json!({
                "status": "success",
                "document": document
            });
            HttpResponse::Ok().json(response)
        }
        Err(error) => {
            let response = json!({
                "status": "error",
                "message": format!("Failed to get document: {:?}", error)
            });
            HttpResponse::InternalServerError().json(response)
        }
    }
}

#[delete("/tasks/{id}")]
async fn delete_task_by_id(path: Path<uuid::Uuid>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id).execute(&data.db).await {
        Ok(_) => { HttpResponse::NoContent().finish() }
        Err(err) => {
            let message = format!("Internal server error: {:?}", err);
            return HttpResponse::NotFound().json(
                serde_json::json!({"status": "fail","message": message})
            );
        }
    }
}

#[delete("/documents/{id}")]
async fn delete_documents_by_id(path: Path<uuid::Uuid>, data: Data<AppState>) -> impl Responder {
    let documents_id = path.into_inner();

    match sqlx::query!("DELETE FROM documents WHERE id = $1", documents_id).execute(&data.db).await {
        Ok(_) => { HttpResponse::NoContent().finish() }
        Err(err) => {
            let message = format!("Internal server error: {:?}", err);
            return HttpResponse::NotFound().json(
                serde_json::json!({"status": "fail","message": message})
            );
        }
    }
}

// Endpoint para atualizar uma tarefa por ID
#[patch("/tasks/{id}")]
async fn update_task_by_id(
    path: Path<Uuid>,
    body: Json<UpdateTaskSchema>,
    data: Data<AppState>
) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query_as!(TaskModel, "SELECT * FROM tasks WHERE id = $1", task_id)
        .fetch_one(&data.db)
        .await
    {
        Ok(task) => {
            let update_result = sqlx::query_as!(
                TaskModel,
                "UPDATE tasks SET title = $1, content = $2 WHERE id = $3 RETURNING *",
                body.title.as_ref().unwrap_or(&task.title),
                body.content.as_ref().unwrap_or(&task.content),
                task_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_task) => {
                    let response = json!({
                        "status": "success",
                        "task": updated_task
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    let message = format!("Failed to update task: {:?}", update_error);
                    HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": message
                    }))
                }
            }
        }
        Err(fetch_error) => {
            let message = format!("Task not found: {:?}", fetch_error);
            HttpResponse::NotFound().json(json!({
                "status": "not found",
                "message": message
            }))
        }
    }
}

// Endpoint para atualizar um documento por ID
#[patch("/documents/{id}")]
async fn update_document_by_id(
    path: Path<Uuid>,
    body: Json<UpdateDocumentSchema>,
    data: Data<AppState>
) -> impl Responder {
    let document_id = path.into_inner();

    // Recuperar o documento existente
    let document_result = sqlx::query_as!(
        DocumentModel,
        "SELECT * FROM documents WHERE id = $1",
        document_id
    )
    .fetch_one(&data.db)
    .await;

    match document_result {
        Ok(document) => {
            // Atualizar o documento
            let update_result = sqlx::query_as!(
                DocumentModel,
                "UPDATE documents SET user_id = COALESCE($1, user_id), doc_type = COALESCE($2, doc_type) WHERE id = $3 RETURNING *",
                body.user_id.as_ref(),
                body.doc_type.as_ref(),
                document_id
            )
            .fetch_one(&data.db)
            .await;

            match update_result {
                Ok(updated_document) => {
                    let response = json!({
                        "status": "success",
                        "document": updated_document
                    });
                    HttpResponse::Ok().json(response)
                }
                Err(update_error) => {
                    // Mensagem de erro detalhada
                    let message = format!("Failed to update document: {:?}", update_error);
                    HttpResponse::InternalServerError().json(json!({
                        "status": "error",
                        "message": message
                    }))
                }
            }
        }
        Err(fetch_error) => {
            // Mensagem de erro detalhada
            let message = format!("Document not found: {:?}", fetch_error);
            HttpResponse::NotFound().json(json!({
                "status": "not found",
                "message": message
            }))
        }
    }
}



// Configuração das rotas
pub fn config(conf: &mut ServiceConfig) {
    conf.service(
        scope("/api")
            .service(health_checker)
            .service(create_task)
            .service(create_document) // Adiciona o serviço de documentos
            .service(get_all_tasks)
            .service(get_task_by_id)
            .service(get_all_documents)
            .service(get_document_by_id)
            .service(delete_task_by_id)
            .service(delete_documents_by_id)
            .service(update_task_by_id)
            .service(update_document_by_id) // Adiciona o serviço de atualização
    );
}
