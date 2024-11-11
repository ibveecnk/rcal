use chrono::{DateTime, Local, Utc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Appointment {
    id: u64,
    title: String,
    description: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl Appointment {
    pub fn new_from_utc(
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

    pub fn new_from_local(
        id: u64,
        title: String,
        description: String,
        start: DateTime<Local>,
        end: DateTime<Local>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            start: start.to_utc(),
            end: end.to_utc(),
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
    use super::Appointment;
    use chrono::{Local, TimeDelta, Utc};

    #[test]
    fn test_new_appointment_utc() {
        let id = 1;
        let title = "Meeting".to_string();
        let description = "Discuss the project".to_string();

        let start = Utc::now();
        let end = start + chrono::Duration::hours(1);

        let appointment =
            Appointment::new_from_utc(id, title.clone(), description.clone(), start, end);

        assert_eq!(appointment.id(), id);
        assert_eq!(appointment.title(), title);
        assert_eq!(appointment.description(), description);
        assert_eq!(appointment.start(), &start);
        assert_eq!(appointment.end(), &end);
        assert_eq!(
            appointment.end().signed_duration_since(appointment.start),
            TimeDelta::hours(1)
        )
    }

    #[test]
    fn test_new_appointment_local() {
        let id = 1;
        let title = "Meeting".to_string();
        let description = "Discuss the project".to_string();

        let start = Local::now();
        let end = start + chrono::Duration::days(3);

        let appointment =
            Appointment::new_from_local(id, title.clone(), description.clone(), start, end);

        assert_eq!(appointment.id(), id);
        assert_eq!(appointment.title(), title);
        assert_eq!(appointment.description(), description);
        assert_eq!(appointment.start(), &start);
        assert_eq!(appointment.end(), &end);
        assert_eq!(
            appointment.end().signed_duration_since(appointment.start),
            TimeDelta::days(3)
        )
    }
}
