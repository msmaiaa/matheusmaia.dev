use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    pub prisma: Arc<crate::prisma::PrismaClient>,
}

impl Context {
    pub fn new(prisma: Arc<crate::prisma::PrismaClient>) -> Self {
        Self { prisma }
    }
}
