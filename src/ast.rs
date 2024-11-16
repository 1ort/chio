pub type TaskNumber = u32;
pub type TaskGroupEntryHeader = String;
pub type Version = String;

#[derive(Debug)]
pub struct TaskId {
    pub project: String,
    pub number: TaskNumber,
}

#[derive(Debug)]
pub struct Task {
    pub id: TaskId,
    pub description: String,
    pub sub_list: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct EventVersion {
    pub version: Version,
    pub tasks: Vec<Task>,
}

#[derive(Debug)]
pub struct VersionGroup {
    pub version: Version,
    pub task_groups: Vec<TaskGroup>, // Can not be empty
}
#[derive(Debug)]
pub struct TaskGroup {
    pub header: TaskGroupEntryHeader,
    pub entries: Vec<Task>, // Can not be empty
}

#[derive(Debug)]
pub struct ChangeLog {
    pub header: String,
    pub versions: Vec<VersionGroup>,
}
