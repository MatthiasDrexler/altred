use waiter_di::Container;

pub fn get<T>() -> Container<T> {
    Container::<T>::new()
}
