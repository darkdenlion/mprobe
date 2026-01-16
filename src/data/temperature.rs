use sysinfo::Components;

pub struct TemperatureData {
    components: Components,
    pub sensors: Vec<SensorInfo>,
}

pub struct SensorInfo {
    pub label: String,
    pub temperature: f32,
    pub critical: Option<f32>,
}

impl Default for TemperatureData {
    fn default() -> Self {
        Self {
            components: Components::new_with_refreshed_list(),
            sensors: Vec::new(),
        }
    }
}

impl TemperatureData {
    pub fn update(&mut self) {
        self.components.refresh();
        self.sensors.clear();

        for component in self.components.iter() {
            let label = component.label().to_string();
            let temp = component.temperature();
            let critical = component.critical();

            // Filter out sensors with no meaningful data
            if temp > 0.0 {
                self.sensors.push(SensorInfo {
                    label,
                    temperature: temp,
                    critical,
                });
            }
        }

        // Sort by temperature (highest first) for relevance
        self.sensors.sort_by(|a, b| {
            b.temperature
                .partial_cmp(&a.temperature)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Limit to most relevant sensors
        self.sensors.truncate(8);
    }

    pub fn get_temp_color_index(temp: f32, critical: Option<f32>) -> usize {
        let threshold = critical.unwrap_or(85.0);
        let ratio = temp / threshold;

        match ratio {
            r if r < 0.5 => 0,  // Green - cool
            r if r < 0.7 => 1,  // Yellow - warm
            r if r < 0.85 => 2, // Orange - hot
            _ => 3,             // Red - critical
        }
    }
}
