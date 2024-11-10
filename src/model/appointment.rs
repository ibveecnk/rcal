use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Appointment {
    id: u64,
    title: String,
    description: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl Appointment {
    pub fn new(
        id: u64,
        title: String,
        description: String,
        start: DateTime<Utc>,
        end: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            start,
            end,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn start(&self) -> &DateTime<Utc> {
        &self.start
    }

    pub fn end(&self) -> &DateTime<Utc> {
        &self.end
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_new_appointment() {
        use super::Appointment;
        use chrono::Utc;

        let id = 1;
        let title = "Meeting".to_string();
        let description = "Discuss the project".to_string();
        let start = Utc::now();
        let end = start + chrono::Duration::hours(1);

        let appointment = Appointment::new(id, title.clone(), description.clone(), start, end);

        assert_eq!(appointment.id(), id);
        assert_eq!(appointment.title(), title);
        assert_eq!(appointment.description(), description);
        assert_eq!(appointment.start(), &start);
        assert_eq!(appointment.end(), &end);
    }
}
