/*
Observer patern is a behavioral design pattern that lets you define a subscription mechanism to notify multiple objects about any events that happen to the object they're observing.
*/

// The Subject owns some important state and notifies observers when the state changes.
struct Subject {
    // For the sake of simplicity, the Subject's state, essential to all subscribers, is stored in this variable.
    state: i32,
    observers: Vec<Box<dyn Observer>>,
}

impl Subject {
    // The subscription management methods.
    fn attach(&mut self, observer: Box<dyn Observer>) {
        self.observers.push(observer);
    }

    fn detach(&mut self, observer: &dyn Observer) {
        self.observers.retain(|o| !std::ptr::eq(o.as_ref(), observer));
    }

    // Trigger an update in each subscriber.
    fn notify(&self) {
        for observer in &self.observers {
            observer.update(&self);
        }
    }

    // Usually, the subscription logic is only a fraction of what a Subject can really do. Subjects commonly hold some important business logic, that triggers a notification method whenever something important is about to happen (or after it).
    fn some_business_logic(&mut self) {
        self.state = 1;
        self.notify();
        println!("Subject: My state has just changed to: {}", self.state);
    }
}
    
// The Observer interface declares the update method, used by subjects.
trait Observer {
    // Receive update from subject.
    fn update(&self, subject: &Subject);
}

// Concrete Observers react to the updates issued by the Subject they had been attached to.
struct ConcreteObserverA {
    name: String,
}

impl Observer for ConcreteObserverA {
    fn update(&self, subject: &Subject) {
        println!("ConcreteObserverA: Reacted to the event");
    }
}

struct ConcreteObserverB {
    name: String,
}

impl Observer for ConcreteObserverB {
    fn update(&self, subject: &Subject) {
        println!("ConcreteObserverB: Reacted to the event");
    }
}

fn main() {
    let mut subject = Subject {
        state: 0,
        observers: Vec::new(),
    };

    let observer_a = Box::new(ConcreteObserverA { name: "A".to_string() });
    subject.attach(observer_a);

    let observer_b = Box::new(ConcreteObserverB { name: "B".to_string() });
    subject.attach(observer_b);

    subject.some_business_logic();
    subject.some_business_logic();

    subject.detach(&ConcreteObserverB { name: "B".to_string() });

    subject.some_business_logic();
}