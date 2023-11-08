use crate::handlers::RestRoutes;
use crate::models::product::Product;

pub struct ProductsHandlers;

impl RestRoutes for ProductsHandlers {
    type Model = Product;
}
