use waiter_di::Container;

pub(crate) fn get<T>() -> Container<T> {
    Container::<T>::new()
}
