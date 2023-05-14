#[derive(Debug)]
struct Task {
    id: u16,
    name: String,
    duration: u16,
}

impl Task {
    pub fn new(id: u16, name: String, duration: u16) -> Self {
        Self { id, name, duration }
    }
}

#[derive(Debug)]
struct Member {
    id: u16,
    name: String,
}

impl Member {
    pub fn new(id: u16, name: String) -> Self {
        Self { id, name }
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

    pub fn add_task(&mut self, name: String, duration: u16) {
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

struct TaskSchedule {
    task_id: u16,
    duration: u16,
    assigned_member: Option<u16>,
    start_time: Option<u16>,
    finish_time: Option<u16>,
}

impl TaskSchedule {
    pub fn from(task: &Task) -> Self {
        Self {
            task_id: task.id,
            duration: task.duration,
            assigned_member: None,
            start_time: None,
            finish_time: None,
        }
    }
}

struct MemberSchedule {
    member_id: u16,
    assigned_tasks: Vec<u16>,
    assignable_from: u16,
}

impl MemberSchedule {
    pub fn from(member: &Member) -> Self {
        Self {
            member_id: member.id,
            assigned_tasks: vec![],
            assignable_from: 0,
        }
    }
}

struct ProjectSchedule {
    task_schedules: Vec<TaskSchedule>,
    member_schedules: Vec<MemberSchedule>,
    duration: u16,
}

struct Scheduler {}

impl Scheduler {
    pub fn create_schedule(project: &Project) -> ProjectSchedule {
        let mut task_schedules: Vec<TaskSchedule> =
            project.tasks.iter().map(TaskSchedule::from).collect();
        let mut member_schedules: Vec<MemberSchedule> =
            project.members.iter().map(MemberSchedule::from).collect();
        // TODO: 偏りが生じる
        for task in task_schedules.iter_mut() {
            member_schedules.sort_by_key(|m| m.assignable_from);
            let assignee: &mut MemberSchedule = member_schedules.first_mut().unwrap();
            Self::assign(task, assignee);
        }
        ProjectSchedule {
            task_schedules,
            member_schedules,
            duration: project.duration,
        }
    }

    fn assign(task: &mut TaskSchedule, member: &mut MemberSchedule) {
        task.assigned_member = Some(member.member_id);
        member.assigned_tasks.push(task.task_id);

        let start_time: u16 = member.assignable_from;
        let finish_time: u16 = start_time + task.duration - 1;

        task.start_time = Some(start_time);
        task.finish_time = Some(finish_time);

        member.assignable_from = finish_time + 1;
    }
}

struct GanttChart {
    timelines: Vec<Vec<String>>,
}

impl GanttChart {
    pub fn from(schedule: &ProjectSchedule) -> GanttChart {
        let mut timelines: Vec<Vec<String>> = vec![];
        for member in schedule.member_schedules.iter() {
            let mut timeline: Vec<String> = vec![];
            let assigned_task = member.assigned_tasks.iter().map(|task_id| {
                schedule
                    .task_schedules
                    .iter()
                    .find(|t| &t.task_id == task_id)
            });
            for task in assigned_task {
                while timeline.len() < task.unwrap().start_time.unwrap() as usize {
                    timeline.push("-".to_string())
                }
                for _ in 0..task.unwrap().duration {
                    timeline.push(task.unwrap().task_id.to_string())
                }
            }
            while timeline.len() < schedule.duration as usize {
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

    let schedule = Scheduler::create_schedule(&project);

    let gantt_chart = GanttChart::from(&schedule);
    gantt_chart.display();
}
