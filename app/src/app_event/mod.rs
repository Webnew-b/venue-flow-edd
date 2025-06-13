
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct AppEventList {
    elements:Vec<AppEvent>
}

impl AppEventList {
    pub fn new() -> Self {
        Self {
            elements:vec![AppEvent::LogUseCase]
        }
    }
    pub fn custom_new(first:AppEvent) -> Self{
        Self {
            elements:vec![first]
        }
    }

    pub fn concat(self,new_vec:Vec<AppEvent>) -> Self {
        Self {
            elements: [self.elements.as_slice(), new_vec.as_slice()].concat()
        }
    }

    pub fn push(&mut self,item:AppEvent) {
        self.elements.push(item);
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn get_elements(self) -> Vec<AppEvent> {
        self.elements
    }
}

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum AppEvent {
    LogUseCase,
}
