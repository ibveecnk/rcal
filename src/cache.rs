//! this module caches data from the sqlite database containing the
//! appointments. It is used to reduce the number of queries to the
//! database and to speed up the application.

use std::collections::HashMap;

use crate::model::Appointment;

/// The cache struct contains a hashmap that maps the id of an appointment
/// to the appointment itself.
pub struct Cache {
    appointments: HashMap<u64, Appointment>,
}

impl Cache {
    /// Creates a new cache.
    pub fn new() -> Self {
        Self {
            appointments: HashMap::new(),
        }
    }

    /// Inserts an appointment into the cache.
    pub fn load(&mut self) {
        todo!("Implement loading appointments from the database");
    }

    /// Inserts an appointment into the cache.
    pub fn insert(&mut self, appointment: Appointment) {
        self.appointments.insert(appointment.id(), appointment);
    }

    /// Returns an appointment from the cache.
    pub fn get(&self, id: u64) -> Option<&Appointment> {
        self.appointments.get(&id)
    }

    /// Returns all appointments from the cache.
    pub fn get_all(&self) -> Vec<&Appointment> {
        self.appointments.values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut cache = Cache::new();

        for i in 0..10 {
            let appointment = Appointment::new(
                i,
                "Meeting".to_string(),
                "Discuss the project".to_string(),
                chrono::Utc::now(),
                chrono::Utc::now(),
            );
            cache.insert(appointment.clone());
            assert_eq!(cache.get(i), Some(&appointment));
        }
    }

    #[test]
    fn test_get_all() {
        let mut cache = Cache::new();
        let appointment1 = Appointment::new(
            1,
            "Meeting".to_string(),
            "Discuss the project".to_string(),
            chrono::Utc::now(),
            chrono::Utc::now(),
        );
        let appointment2 = Appointment::new(
            2,
            "Lunch".to_string(),
            "Eat with colleagues".to_string(),
            chrono::Utc::now(),
            chrono::Utc::now(),
        );
        cache.insert(appointment1.clone());
        cache.insert(appointment2.clone());

        // The appointments are not guaranteed to be in the same order
        let mut cache_data = cache.get_all();
        cache_data.sort_by_key(|a| a.id());

        assert_eq!(cache_data, vec![&appointment1, &appointment2]);
    }
}
