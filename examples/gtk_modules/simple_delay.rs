use {
    audio_module::{
        percent_string_converter, AudioModule, AudioProcessor, Command, CommandHandler,
        FloatParameter, Parameter, ParameterProvider,
    },
    num_traits::FromPrimitive,
    yanel_dsp::SimpleDelay,
};

#[derive(FromPrimitive)]
pub enum Parameters {
    Delay,
    Feedback,
    Dry,
    Wet,
}

pub struct SimpleDelayProcessor {
    simple_delay: SimpleDelay,
}

impl SimpleDelayProcessor {
    fn new(sample_rate: usize) -> Self {
        Self {
            simple_delay: SimpleDelay::init(sample_rate),
        }
    }
}

impl CommandHandler for SimpleDelayProcessor {
    fn handle_command(&mut self, command: Command) {
        match command {
            Command::SetParameter(id, value) => match Parameters::from_usize(id).unwrap() {
                Parameters::Delay => {
                    self.simple_delay.set_delay_in_secs(value);
                }
                Parameters::Feedback => {
                    self.simple_delay.set_feedback(value);
                }
                Parameters::Dry => {
                    self.simple_delay.set_dry(value);
                }
                Parameters::Wet => {
                    self.simple_delay.set_wet(value);
                }
            },
        }
    }
}

impl AudioProcessor for SimpleDelayProcessor {
    fn process_stereo(&mut self, input: &[f32], output: &mut [f32]) {
        debug_assert!(input.len() == output.len());

        for i in (0..input.len()).step_by(2) {
            let result = self.simple_delay.tick((input[i], input[i + 1]));

            output[i] = result.0 as f32;
            output[i + 1] = result.1 as f32;
        }
    }
}

pub struct SimpleDelayModule {}

impl AudioModule for SimpleDelayModule {
    type Processor = SimpleDelayProcessor;

    fn create_processor(sample_rate: usize) -> Self::Processor {
        SimpleDelayProcessor::new(sample_rate)
    }
}

impl ParameterProvider for SimpleDelayModule {
    fn parameter_count() -> usize {
        (0..usize::max_value())
            .take_while(|&x| Parameters::from_usize(x).is_some())
            .count()
    }

    fn parameter(id: usize) -> Box<dyn Parameter> {
        match Parameters::from_usize(id).unwrap() {
            Parameters::Delay => Box::new(
                FloatParameter::new("Delay")
                    .string_converter(percent_string_converter)
                    .default_user_value(0.5),
            ),
            Parameters::Feedback => Box::new(
                FloatParameter::new("Feedback")
                    .string_converter(percent_string_converter)
                    .default_user_value(0.5),
            ),
            Parameters::Dry => Box::new(
                FloatParameter::new("Dry")
                    .string_converter(percent_string_converter)
                    .default_user_value(0.0),
            ),
            Parameters::Wet => Box::new(
                FloatParameter::new("Wet")
                    .string_converter(percent_string_converter)
                    .default_user_value(1.0),
            ),
        }
    }
}
