pub mod user_mapper;

pub trait Mapper<D, E> {

    fn map_to_entity(dto: D) -> E;

    fn map_to_dto(entity: E) -> D;
}