#[derive(Debug)]
struct Task {
    id: u16,
    name: String,
    duration: u8,
    assigned_member: Option<u16>,
    start_time: Option<u16>,
    finish_time: Option<u16>,
}

impl Task {
    pub fn new(id: u16, name: String, duration: u8) -> Self {
        Self {
            id,
            name,
            duration,
            assigned_member: None,
            start_time: None,
            finish_time: None,
        }
    }
}

#[derive(Debug)]
struct Member {
    id: u16,
    name: String,
    assigned_tasks: Vec<u16>,
    assignable_from: u16,
}

impl Member {
    pub fn new(id: u16, name: String) -> Self {
        Self {
            id,
            name,
            assigned_tasks: vec![],
            assignable_from: 0,
        }
    }

    pub fn assign(&mut self, task: &Task) {
        self.assigned_tasks.push(task.id);
        self.assignable_from = self.assignable_from + task.duration as u16
    }
}

struct Project {
    tasks: Vec<Task>,
    members: Vec<Member>,
    duration: u16,
}

impl Project {
    pub fn new(duration: u16) -> Self {
        Self {
            tasks: vec![],
            members: vec![],
            duration,
        }
    }

    pub fn add_task(&mut self, name: String, duration: u8) {
        let id = self.tasks.len();
        let task = Task::new(id as u16, name, duration);
        self.tasks.push(task)
    }

    pub fn add_member(&mut self, name: String) {
        let id = self.members.len();
        let member = Member::new(id as u16, name);
        self.members.push(member)
    }
}

struct Scheduler {}

impl Scheduler {
    pub fn create_schedule(project: &mut Project) {
        // TODO: 偏りが生じる
        for task in project
            .tasks
            .iter_mut()
            .filter(|task| task.assigned_member.is_none())
        {
            project.members.sort_by_key(|m| m.assignable_from);
            let assignee: &mut Member = project.members.first_mut().unwrap();
            Self::assign(task, assignee);
        }
    }

    fn assign(task: &mut Task, member: &mut Member) {
        task.assigned_member = Some(member.id);
        task.start_time = Some(member.assignable_from as u16);
        member.assign(task);
        task.finish_time = Some(member.assignable_from as u16 - 1);
    }
}

struct GanttChart {
    timelines: Vec<Vec<String>>,
}

impl GanttChart {
    pub fn from(project: &Project) -> GanttChart {
        let mut timelines: Vec<Vec<String>> = vec![];
        for member in project.members.iter() {
            let mut timeline: Vec<String> = vec![];
            let assigned_task = member
                .assigned_tasks
                .iter()
                .map(|task_id| project.tasks.iter().find(|t| &t.id == task_id));
            for task in assigned_task {
                while timeline.len() < task.unwrap().start_time.unwrap() as usize {
                    timeline.push("-".to_string())
                }
                for _ in 0..task.unwrap().duration {
                    timeline.push(task.unwrap().id.to_string())
                }
            }
            while timeline.len() < project.duration as usize {
                timeline.push("-".to_string())
            }
            timelines.push(timeline);
        }
        GanttChart { timelines }
    }
    pub fn display(&self) {
        for timeline in self.timelines.iter() {
            println!("{:?}", timeline);
        }
    }
}

fn main() {
    let mut project = Project::new(10);
    project.add_task("new task1".to_string(), 3);
    project.add_task("new task2".to_string(), 2);
    project.add_task("new task3".to_string(), 1);
    project.add_task("new task4".to_string(), 1);
    project.add_task("new task5".to_string(), 3);

    project.add_member("member1".to_string());
    project.add_member("member2".to_string());
    project.add_member("member3".to_string());

    Scheduler::create_schedule(&mut project);

    let gantt_chart = GanttChart::from(&project);
    gantt_chart.display();
}
