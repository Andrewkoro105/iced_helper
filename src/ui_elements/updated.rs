use iced::Task;

pub trait Updated<Message> {
    fn update(&mut self, message: Message) {
        let _ = self.task_update(message);
    }
    
    fn task_update(&mut self, message: Message) -> Task<Message> {
        self.update(message);
        Task::none()
    }
}
