use router::Router;
use controllers::koo::*;

pub fn router_generator() -> Router {
    let mut router = Router::new();
    router.get("/", koo_show, "koo_show");
    router.post("/add", koo_add, "koo_add");
    router.get("/delete", koo_delete, "koo_delete");
    router
}
