pub trait DtoMapper<D, E> {

    fn map_to_domain_model(dto: D) -> E;

    fn map_to_dto(entity: E) -> D;
}