pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use super::Ticket;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 3 * size_of::<usize>());
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Data layout" section of the Rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.
        assert_eq!(size_of::<Ticket>(), 3 * 3 * size_of::<usize>());
    }
}
