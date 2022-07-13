use crate::{
    common_types::CreatePostPayload,
    prisma::{post, tag, user, PrismaClient},
};

pub struct PostRepository {
    client: std::sync::Arc<PrismaClient>,
}

impl PostRepository {
    pub fn new(client: std::sync::Arc<PrismaClient>) -> Self {
        PostRepository { client }
    }
    pub async fn create_post(
        &self,
        data: &CreatePostPayload,
        user_id: &i32,
    ) -> Result<post::Data, prisma_client_rust::Error> {
        let mut _params: Vec<post::SetParam> =
            vec![post::published::set(data.published.unwrap_or(false))];
        if let Some(tags) = &data.tags {
            _params.push(post::tags::link(
                tags.into_iter().map(|tag| tag::id::equals(*tag)).collect(),
            ))
        }

        self.client
            .post()
            .create(
                post::title::set(data.title.clone()),
                post::content::set(data.content.clone()),
                post::author::link(user::id::equals(*user_id)),
                _params,
            )
            .exec()
            .await
    }
}
