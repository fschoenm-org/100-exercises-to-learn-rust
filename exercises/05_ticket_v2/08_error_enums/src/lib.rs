use crate::TicketNewError::{DescriptionEmpty, DescriptionTooLong, TitleEmpty, TitleTooLong};

#[derive(Debug)]
enum TicketNewError {
    TitleEmpty,
    TitleTooLong,
    DescriptionEmpty,
    DescriptionTooLong,
}

fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(), description.clone(), status.clone()) {
        Ok(ticket) => ticket,
        Err(error) => match error {
            TitleEmpty => panic!("Title cannot be empty"),
            TitleTooLong => panic!("Title cannot be longer than 50 bytes"),
            _ => Ticket::new(title, "Description not provided".into(), status).unwrap(),
        },
    }
}

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TitleEmpty);
        }
        if title.len() > 50 {
            return Err(TitleTooLong);
        }
        if description.is_empty() {
            return Err(DescriptionEmpty);
        }
        if description.len() > 500 {
            return Err(DescriptionTooLong);
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    use super::*;

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
