use vst::buffer::AudioBuffer;
use vst::plugin::{Info, Plugin};
use vst::plugin_main;

pub struct GainPlugin {
    gain: f32,
}

impl Default for GainPlugin {
    fn default() -> Self {
        GainPlugin { gain: 0.5 } // Ganancia inicial al 50%
    }
}

impl Plugin for GainPlugin {
    fn get_info(&self) -> Info {
        Info {
            name: "BasicGainPlugin".to_string(),
            vendor: "YourName".to_string(),
            unique_id: 243723073,
            version: 1,
            inputs: 2,
            outputs: 2,
            parameters: 1,
            ..Default::default()
        }
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
        let (inputs, outputs) = buffer.split();

        for (input_buffer, output_buffer) in inputs.iter().zip(outputs.iter_mut()) {
            for (input_sample, output_sample) in input_buffer.iter().zip(output_buffer.iter_mut()) {
                *output_sample = *input_sample * self.gain;
            }
        }
    }

    fn set_parameter(&mut self, index: i32, value: f32) {
        if index == 0 {
            self.gain = value;
        }
    }

    fn get_parameter(&self, index: i32) -> f32 {
        if index == 0 {
            self.gain
        } else {
            0.0
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        if index == 0 {
            "Gain".to_string()
        } else {
            "".to_string()
        }
    }
}
//MACRO VST
plugin_main!(GainPlugin);
