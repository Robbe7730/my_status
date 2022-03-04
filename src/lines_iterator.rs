use crate::modules::Module;

pub struct StatusLineIterator {
    modules: Vec<Box<dyn Module>>
}

impl Iterator for StatusLineIterator {
    type Item = Vec<StatusBlock>;

    fn next(&mut self) -> Option<Vec<StatusBlock>> {
        Some(self.modules.iter().map(|module| {
            module.get_blocks()
        }).flatten().collect())
    }
}


