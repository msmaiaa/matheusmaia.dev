use poem_openapi::Object;

#[derive(serde::Deserialize, Object)]
pub struct CreatePostPayload {
    pub title: String,
    pub content: String,
    pub published: Option<bool>,
    pub tags: Option<Vec<u32>>,
}
