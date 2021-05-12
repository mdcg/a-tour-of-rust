// As structs genéricas também podem ter seus tipos parametrizados
// restritos por traits.

// struct MinhaStruct<T>
// where
//     T: MinhaTrait
// {
//     foo: T
//     ...
// }

// As structs genéricas têm seu tipo parametrizado em seus blocos de
// implementação:

// impl<T> MinhaStruct<T> {
//     ...
// }