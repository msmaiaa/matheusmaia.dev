use prisma_client_rust::NewClientError;

use crate::prisma::PrismaClient;

pub async fn get_client() -> Result<PrismaClient, NewClientError> {
    crate::prisma::new_client().await
}
