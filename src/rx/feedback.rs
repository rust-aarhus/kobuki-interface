use super::basic_sensor_data::BasicSensorData;
use super::docking_ir::DockingIr;
use super::feedback_decoder::FeedbackId;
use super::inertial_sensor::InertialSensor;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Feedback {
    pub basic_sensor_data: Option<BasicSensorData>,
    pub docking_ir: Option<DockingIr>,
    pub inertial_sensor: Option<InertialSensor>,
}

impl Feedback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_subpayload(&mut self, id: FeedbackId, data: &[u8]) -> Result<(), std::io::Error> {
        match id {
            FeedbackId::BasicSensorData => {
                self.basic_sensor_data = Some(BasicSensorData::try_from(data)?)
            }
            FeedbackId::DockingIR => self.docking_ir = Some(DockingIr::try_from(data)?),
            FeedbackId::InertialSensor => {
                self.inertial_sensor = Some(InertialSensor::try_from(data)?)
            }
            _ => {}
        }
        Ok(())
    }
}
