use crate::handlers::RestRoutes;
use crate::models::category::Category;

pub struct CategoriesHandlers;

impl RestRoutes for CategoriesHandlers {
    type Model = Category;
}
