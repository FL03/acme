pub trait Controller {
    type Actor;

    fn authenticate(&self, actor: Self::Actor);
}